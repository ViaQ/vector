# Architecture — Red Hat Vector Fork

This document covers what is architecturally specific to this fork. For Vector's internal engine architecture (topology, fanout, buffering, component construction), see the upstream [docs/ARCHITECTURE.md](https://github.com/vectordotdev/vector/blob/master/docs/ARCHITECTURE.md).

## Deployment Model

Vector runs as a **DaemonSet** on every OpenShift node, managed by the [cluster-logging-operator](https://github.com/openshift/cluster-logging-operator). It is not deployed standalone.

- Configuration is injected via ConfigMap by the operator
- The operator provides a `run.sh` entrypoint script
- Vector collects container logs from `/var/log/containers/` (file source) and infrastructure logs from journald
- Pod metadata is extracted from file paths and enriched by transforms

## `ocp-logging` Feature

The fork defines a custom Cargo feature `ocp-logging` in `Cargo.toml` that compiles only the Vector components needed for OpenShift. The Makefile sets `FEATURES=ocp-logging` by default on Linux.

### Enabled Components

**Sources:** stdin, file_descriptor, file, journald, kubernetes_logs, prometheus, internal_metrics, demo_logs, http_server, syslog, opentelemetry

**Transforms:** route, dedupe, filter, remap, log_to_metric, lua, throttle, reduce, detect_exceptions

**Sinks:**

| Sink                 | v0.47.0-rh | v0.54.0-rh |
|----------------------|:----------:|:----------:|
| aws_cloudwatch_logs  |    yes     |    yes     |
| aws_s3               |    yes     |    yes     |
| azure_logs_ingestion |     —      |    yes     |
| azure_monitor_logs   |    yes     |    yes     |
| elasticsearch        |    yes     |    yes     |
| file                 |    yes     |    yes     |
| kafka                |    yes     |    yes     |
| loki                 |    yes     |    yes     |
| console              |    yes     |    yes     |
| prometheus           |    yes     |    yes     |
| gcp                  |    yes     |    yes     |
| splunk_hec           |    yes     |    yes     |
| http                 |    yes     |    yes     |
| socket               |    yes     |    yes     |
| opentelemetry        |    yes     |    yes     |

Components not in `ocp-logging` exist in the source tree but are not compiled into the Red Hat binary.

## Patched Dependencies

The fork carries local patches for two crates via `[patch.crates-io]` in `Cargo.toml`:

### `patch/openssl/`

Forked from the `openssl` crate. Two purposes:

1. **FIPS compliance** (LOG-2552): Vector uses OpenSSL instead of upstream's Ring for all TLS operations, enabling FIPS-validated cryptography.
2. **TLS security profiles** (LOG-3398): Enforces OpenShift's `TLSSecurityProfile` settings (cipher suites, TLS versions) at the Vector level, so cluster administrators control the security posture globally.

### `patch/hyper/`

Forked from the `hyper` crate. Contains HTTP behavior modifications for reliability in OpenShift environments.

### Other patches (v0.47.0-rh only)

- `tokio-util` — vectordotdev fork for framed-read-continue-on-error behavior
- `nix` — vectordotdev fork for memfd gnu/musl support

Both were dropped in v0.54.0-rh (fixed upstream).

## Build System

### Dockerfiles

| File              | Base               | Rust               | Purpose                                                                                                                             |
|-------------------|--------------------|--------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| `Dockerfile`      | `ubi9/ubi`         | rustup             | Local/developer builds                                                                                                              |
| `Dockerfile.art`  | `ubi9/ubi-minimal` | RPM `rust-toolset` | ART production image (shipped to customers). Uses `make build-offline` for air-gapped builds                                        |
| `Dockerfile.unit` | `ubi9/ubi`         | rustup             | CI unit test runner. Selective COPY for build cache efficiency                                                                      |

### Build constraints

- **`lld` linker**: Configured in `.cargo/config.toml`. GNU ld runs out of memory during linking.
- **`cargo-nextest` and `cargo-deny` bundled**: In `thirdparty/` for CI reproducibility.
- **Single-threaded tests**: `--test-threads 1` in the Makefile `test` target to limit memory usage.

### CI/CD

No in-repo CI pipelines. Builds happen in Red Hat's internal infrastructure:
- `Dockerfile.art` → production image via ART (Automated Release Tooling)

## Key Design Decisions

### OpenSSL over Ring (LOG-2552)

**Decision:** Replace Ring with OpenSSL for all cryptographic operations.
**Why:** FIPS compliance requires a validated crypto library. Ring is not FIPS-validated. OpenSSL is.
**Trade-off:** Adds the `patch/openssl/` maintenance burden and prevents trivial upstream merges of TLS-related code.

### TLS Security Profile Enforcement (LOG-3398)

**Decision:** All TLS connections respect OpenShift's `TLSSecurityProfile` (cipher suites, min/max TLS version).
**Why:** OpenShift cluster administrators expect uniform TLS policy enforcement across all components.
**Depends on:** The OpenSSL patch above — upstreams Ring-based TLS does not support security profile configuration.

### `detect_exceptions` Transform (LOG-6155)

**Decision:** A Red Hat-specific transform for multiline exception detection (Java stack traces, Python tracebacks, etc.) with configurable `message_key`.
**Why:** Upstream's multiline handling is less flexible. OpenShift workloads heavily use Java, and stack traces must be grouped into single log events.

## Differences Between Version Branches

| Aspect                           | v0.47.0-rh     | v0.54.0-rh               |
|----------------------------------|----------------|--------------------------|
| Upstream base                    | Vector v0.47.0 | Vector v0.54.0           |
| Rust edition                     | 2021           | 2024                     |
| MSRV (rust-toolchain.toml)       | 1.85           | 1.92                     |
| `azure_logs_ingestion` sink      | —              | enabled                  |
| `tokio-util` / `nix` patches     | present        | removed (fixed upstream) |
| GCP Workload Identity Federation | —              | LOG-9171                 |
| TLS curve configuration          | —              | LOG-8968                 |

## Upstream Sync Strategy

1. **Version-based merges:** Each branch is based on a specific upstream tag (e.g., v0.47.0). Upstream changes are incorporated by creating a new branch from the next upstream tag and replaying Red Hat patches.
2. **Patch minimization:** Features are contributed upstream when possible. Once accepted, the fork patch is simplified or removed on the next branch.
3. **Cherry-picks:** Critical fixes from upstream are cherry-picked into active branches between version merges.
4. **Conflict zones:** `Cargo.toml` (feature definitions, patch section) and TLS-related code are the primary conflict areas during merges.
