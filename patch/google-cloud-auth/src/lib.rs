// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Google Cloud Client Libraries for Rust - Authentication Components
//!
//! This crate contains types and functions used to authenticate applications
//! on Google Cloud.  The SDK clients consume an implementation of
//! [credentials::Credentials] and use these credentials to authenticate RPCs
//! issued by the application.
//!
//! [Authentication methods at Google] is a good introduction on the topic of
//! authentication for Google Cloud services and other Google products. The
//! guide also describes the common terminology used with authentication, such
//! as [Principals], [Tokens], and [Credentials].
//!
//! # Features
//!
//! Red Hat fork note: this crate is vendored under `patch/google-cloud-auth`
//! to avoid bundled, FIPS-non-attestable crypto (ring and aws-lc-rs) in the
//! `ocp-logging` build (LOG-10744). `default-rustls-provider` now pulls in
//! `reqwest`'s `native-tls` feature (system OpenSSL) instead of rustls, and
//! `ServiceAccountKey`'s JWT signing uses this fork's patched `openssl` crate
//! instead of `rustls::crypto::aws_lc_rs`. `default-idtoken-backend` is a
//! no-op here; the `idtoken`/`jsonwebtoken` feature (unused by `ocp-logging`)
//! would still need its own non-bundled-crypto backend if ever enabled.
//!
//! - `idtoken`: disabled by default, this feature enables support to create and
//!   verify [OIDC ID Tokens].
//!
//! [jsonwebtoken]: https://crates.io/crates/jsonwebtoken
//! [oidc id tokens]: https://cloud.google.com/docs/authentication/token-types#identity-tokens
//! [Authentication methods at Google]: https://cloud.google.com/docs/authentication
//! [Principals]: https://cloud.google.com/docs/authentication#principal
//! [Tokens]: https://cloud.google.com/docs/authentication#token
//! [Credentials]: https://cloud.google.com/docs/authentication#credentials

pub(crate) mod access_boundary;
pub mod build_errors;
pub(crate) mod constants;
pub mod credentials;
pub mod errors;
pub(crate) mod headers_util;
pub(crate) mod mds;
pub(crate) mod retry;
pub mod signer;
pub(crate) mod token;
pub(crate) mod token_cache;

/// A `Result` alias where the `Err` case is [BuildCredentialsError].
pub(crate) type BuildResult<T> = std::result::Result<T, build_errors::Error>;

/// A `Result` alias where the `Err` case is [CredentialsError][errors::CredentialsError].
pub(crate) type Result<T> = std::result::Result<T, errors::CredentialsError>;
