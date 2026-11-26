# Red Hat fork notes

This directory is a vendored fork of the upstream [`google-cloud-auth`](
https://crates.io/crates/google-cloud-auth) crate, version **1.8.0**, patched
in via `[patch.crates-io]` in the repo root `Cargo.toml`. It follows the same
convention as the other crates under `patch/` (`openssl`, `hyper`,
`rustls-webpki-0.102`): a full copy of the upstream crate source with a small,
targeted set of changes layered on top.

## Why this fork exists

Jira: [LOG-10744](https://redhat.atlassian.net/browse/LOG-10744)

check-payload (the FIPS compliance scanner used by Konflux/Conforma) flags
`/usr/bin/vector` in the `ocp-logging` build:

```
rust binary uses bundled crypto that is not an attested FIPS-certified module (aws-lc)
```

Upstream `google-cloud-auth` pulls in `aws-lc-rs` (and therefore `aws-lc-sys`,
the compiled AWS-LC C library that produces the `aws_lc_`/`AWSLC_` symbols
check-payload detects) in two places:

1. Its `default-idtoken-backend` feature directly depends on `aws-lc-rs`,
   used by `ServiceAccountKey::signer()` to sign service-account JWT
   assertions (this path is also used for GCP Workload Identity Federation).
2. Its `reqwest` dependency is pinned to the `rustls-no-provider` feature
   (no `native-tls`/system-OpenSSL option), so the HTTP transport also needs
   *some* rustls crypto provider — `default-rustls-provider` picks
   `aws-lc-rs` for that too.

check-payload's own policy (`fips_certified_modules` in
[openshift/check-payload](https://github.com/openshift/check-payload)'s
`config.toml`) only attests **system OpenSSL** (and Go's own FIPS 140 module)
for Rust binaries — no bundled Rust crypto crate is accepted, not even the
real CMVP-validated AWS-LC-FIPS build. So the only compliant fix is removing
`aws-lc-rs` from the linked binary entirely, the same approach this fork
already takes for `ring` (see `patch/rustls-webpki-0.102`, and
[ViaQ/vector#327](https://github.com/ViaQ/vector/pull/327) /
[LOG-10689](https://redhat.atlassian.net/browse/LOG-10689) for the
`v0.47.0-rh` equivalent).

This is unrelated to [ViaQ/vector#323](https://github.com/ViaQ/vector/pull/323)
/ LOG-10499/10500/10506 (the `rustls-webpki` 0.103.16 CVE fix for distrusted
emSign SPKIs, which also touches the GCP OAuth path) — this fork must not
regress that fix.

## What changed vs. upstream 1.8.0

Diff against the pristine crate (`cargo:google-cloud-auth@1.8.0`), file by
file:

### `Cargo.toml`

- Added an empty `[workspace]` table at the top. This keeps the crate out of
  the parent repo's Cargo workspace (it's only ever consumed via
  `[patch.crates-io]`, never as a workspace member) while still letting
  `cargo test` run standalone from this directory — see "How to verify"
  below.
- `default-idtoken-backend` feature body changed from
  `["dep:aws-lc-rs", "jsonwebtoken?/aws_lc_rs"]` to `[]`. There's no longer a
  bundled-crypto backend to select: `ServiceAccountKey::signer()` doesn't use
  `aws-lc-rs` anymore (see below). The `jsonwebtoken?/aws_lc_rs` wiring was
  already inert for us — `ocp-logging` never enables the `idtoken`/
  `jsonwebtoken` feature.
- `default-rustls-provider` feature body changed from
  `["reqwest/default-tls", "rustls/aws_lc_rs"]` to `["reqwest/native-tls"]`.
  The HTTP transport now uses `native-tls` (system OpenSSL on Linux) instead
  of rustls, so it needs no rustls crypto provider (ring or aws-lc-rs) at
  all.
- Removed the `[[test]] name = "crypto_provider"` entry (the test file is
  deleted too, see below).
- Removed the `[dependencies.aws-lc-rs]` block entirely.
- `[dependencies.reqwest]` features: replaced `"rustls-no-provider"` with
  `"native-tls"`.
- Added `[dependencies.openssl]` (`version = "0.10"`, `default-features =
  false`) — this resolves through this fork's own `patch/openssl` (FIPS/TLS
  profile-aware) via the parent repo's `[patch.crates-io]`, same as every
  other OpenSSL usage in this project. Backs the new signer (see below).
- `rustls` and `rustls-pki-types` dependencies are **unchanged** — still
  needed for `rustls::sign::Signer`/`SignatureScheme` (trait/type definitions
  only, no crypto provider feature enabled) and for PEM/PKCS8 parsing,
  respectively.

### `src/lib.rs`

- Updated the crate-level doc comment's `# Features` section to describe the
  fork's reality instead of upstream's (removed the `[aws-lc-rs]`/`[ring]`
  doc-links and the now-inaccurate `default-idtoken-backend` description).
  No functional change.

### `src/credentials/service_account.rs`

This is the only file with functional changes, all inside
`ServiceAccountKey::signer()` and its surrounding code:

- **New `OpenSslRsaSigner` struct** implementing `rustls::sign::Signer`
  (`fn sign(&self, message: &[u8]) -> Result<Vec<u8>, rustls::Error>` and
  `fn scheme(&self) -> rustls::SignatureScheme`), backed by
  `openssl::sign::Signer` doing RSA-PKCS1-SHA256 over a `PKey<Private>`. This
  replaces `rustls::crypto::aws_lc_rs::default_provider().key_provider`.
- **`ServiceAccountKey::signer()` rewritten**: still uses
  `rustls_pki_types::PrivateKeyDer::from_pem_slice` to parse the PEM envelope
  and reject non-PKCS8 keys (preserving upstream's exact
  `"expected key to be in form of PKCS8, found {:?}"` behavior/error — this
  step involves no crypto backend, just PEM/DER structure), then hands the
  PKCS8 DER bytes to `openssl::pkey::PKey::private_key_from_der(...)` for
  the actual key material and validation, wrapped in `OpenSslRsaSigner`.
  Upstream's `CryptoProvider::get_default()` fallback-to-app-installed-
  provider logic and the `#[cfg(feature = "default-rustls-provider")]`
  branching are gone — this fork always uses OpenSSL.
- Error wrapping uses `CredentialsError::new(is_transient, message, source)`
  (not `non_retryable_from_str`/`non_retryable` alone) so that **both** the
  human-readable "Failed to parse service account private key PEM" message
  *and* the typed source error (`rustls_pki_types::pem::Error` or
  `openssl::error::ErrorStack`, depending on which parse step failed) survive
  — upstream's tests check both, in different tests.
- **Test fix**: `get_service_account_headers_invalid_key_failure` downcasts
  `.source()` to `openssl::error::ErrorStack` instead of `rustls::Error`
  (that's genuinely a different error type now, for a semantically
  equivalent "the key is garbage" failure).
- **New test**: `signer_produces_verifiable_pkcs1_sha256_signature` — signs
  with `OpenSslRsaSigner` and verifies the signature using an independent,
  pure-Rust RSA implementation (`rsa::pkcs1v15::VerifyingKey`, already a
  dev-dependency here), not OpenSSL verifying its own output. This is the
  strongest evidence that the new signer produces standards-correct
  RSA-PKCS1-SHA256 signatures a real verifier (e.g. Google's JWKS endpoint)
  would accept.

### `tests/crypto_provider.rs` — deleted

This integration test only covered the now-removed
"application installs its own `rustls::CryptoProvider`" fallback mechanism.
Since this fork always signs via OpenSSL regardless of any installed rustls
provider, the behavior it tested no longer exists.

## How to verify

This crate isn't a member of the parent repo's Cargo workspace, so
`cargo test --workspace` at the repo root never touches it. To validate
changes here, run its test suite standalone:

```sh
cd patch/google-cloud-auth
cargo test --lib -- --test-threads=1
```

At the time of this fork, this passes 285/288 tests; the 3 failures
(`credentials::external_account_sources::aws_sourced::tests::*`, AWS IMDS
mocking) are **pre-existing upstream test flakiness**, reproduced identically
against the unmodified crate from crates.io — not caused by this fork.

For the actual FIPS evidence, build the real `ocp-logging` binary and either:

- `cargo tree -e normal,build -i aws-lc-rs` / `-i aws-lc-sys` from the repo
  root — both should report "did not match any packages".
- Run `check-payload scan local` against a built image (or an unpacked image
  tree with only `/usr/bin/vector` swapped for an A/B comparison) — see
  LOG-10744 for the exact methodology used.

## Updating to a newer upstream `google-cloud-auth` version

1. Diff this directory against the new upstream release to see what else
   changed upstream in the meantime (`diff -ru <new-upstream-src>
   patch/google-cloud-auth`, after mentally subtracting the changes listed
   above).
2. Re-apply the same conceptual changes against the new version:
   - Re-check whether `ServiceAccountKey::signer()` (or wherever the
     equivalent JWT-signing code lives now) still has the same
     `rustls::crypto::aws_lc_rs` shape. Re-port the `OpenSslRsaSigner`
     approach if so; if upstream changed its signing abstraction, you'll
     need to redesign this part.
   - Re-check `default-rustls-provider`/`default-idtoken-backend` (or
     whatever upstream renames them to) for any newly-introduced bundled
     crypto dependency, and re-verify `jsonwebtoken`/`idtoken` is still
     disabled by default in our build (if `ocp-logging` ever needs GCP
     ID-token *verification*, not just signing, that feature would need its
     own non-bundled-crypto backend — not handled here).
   - Re-check `reqwest`'s feature set still supports `native-tls` cleanly at
     the new version.
3. Re-run the verification steps above, including the standalone test suite
   and a real `cargo tree`/check-payload check from the repo root.
4. Update this file's "version 1.8.0" references and the diff summary above.
