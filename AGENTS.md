# AGENTS.md — Red Hat Vector Fork Context

This file provides context for AI agents and developers working in this repository. It covers only what is specific to this Red Hat fork. For generic Vector development commands, project structure, and Rust conventions, see the [upstream AGENTS.md](https://github.com/vectordotdev/vector/blob/master/AGENTS.md).

## What This Repo Is

A fork of [vectordotdev/vector](https://github.com/vectordotdev/vector) maintained by Red Hat for OpenShift Logging. Vector runs as a DaemonSet on every OpenShift node, collecting application and infrastructure logs. It is deployed and configured by the [cluster-logging-operator](https://github.com/openshift/cluster-logging-operator) — Vector is never run standalone in production.

**Upstream remote:** `vectordev` (git@github.com:vectordotdev/vector.git)
**Fork origin:** `origin` (git@github.com:ViaQ/vector.git)

## Critical: Branch Selection

**Never commit code to `rh-main`.** It contains only documentation.

| Branch       | Vector Version | Rust | Use                              |
|--------------|----------------|------|----------------------------------|
| `v0.47.0-rh` | v0.47.0        | 1.85 | Stable — OpenShift 6.2–6.5       |
| `v0.54.0-rh` | v0.54.0        | 1.92 | Next — future OpenShift releases |

Always check out the appropriate version branch before making changes:

```bash
git checkout v0.47.0-rh   # stable
git checkout v0.54.0-rh   # next
```

When a fix applies to both branches, submit separate PRs per branch.

## Build Quick Reference

The Makefile defaults to `FEATURES=ocp-logging` on Linux (not upstreams `default` feature set).

```bash
make build          # cargo build --release --no-default-features --features ocp-logging
make test           # cargo nextest run ... --features ocp-logging --test-threads 1
make fmt            # rustfmt
make check-clippy   # clippy lints
```

Key differences from upstream builds:
- **`--features ocp-logging`** — curated subset of Vector, not the full feature set
- **`--test-threads 1`** — single-threaded tests to avoid OOM in CI
- **`lld` linker** — configured in `.cargo/config.toml` (GNU ld OOMs during linking)
- **`CFLAGS=-g0 -O3`** — set in the build target for release optimization

## `ocp-logging` Feature Scope

This is the curated subset of Vector components enabled for OpenShift. Only these are compiled into the Red Hat binary.

**Sources:** stdin, file_descriptor, file, journald, kubernetes_logs, prometheus, internal_metrics, demo_logs, http_server, syslog, opentelemetry

**Transforms:** route, dedupe, filter, remap, log_to_metric, lua, throttle, reduce, detect_exceptions

**Sinks (v0.47.0-rh):** aws_cloudwatch_logs, aws_s3, azure_monitor_logs, elasticsearch, file, kafka, loki, console, prometheus, gcp, splunk_hec, http, socket, opentelemetry

**Sinks (v0.54.0-rh adds):** azure_logs_ingestion

**Also enabled:** api, api-client, unix

Components not in this list are **not available** in the Red Hat binary even though they exist in the source tree.

## Fork-Specific Files

These files exist in this fork but not in upstream:

| Path              | Purpose                                                                                                  |
|-------------------|----------------------------------------------------------------------------------------------------------|
| `patch/openssl/`  | Forked openssl crate — enables FIPS compliance and TLS security profile enforcement (LOG-2552, LOG-3398) |
| `patch/hyper/`    | Forked hyper crate — HTTP behavior modifications                                                         |
| `thirdparty/`     | Bundled binaries: `protoc`, `cargo-nextest`, `cargo-deny` (no external downloads in CI)                  |
| `Dockerfile`      | Developer/local build image (UBI 9 + rustup)                                                             |
| `Dockerfile.art`  | ART production build (UBI 9 minimal + RPM rust-toolset, `make build-offline`)                            |
| `Dockerfile.unit` | CI unit test runner                                                                                      |
| `OWNERS`          | Kubernetes-style code review assignments                                                                 |
| `CLAUDE.md`       | Stub pointing to this file                                                                               |

## Patched Crates (`[patch.crates-io]`)

**Both branches:**
- `openssl = { path = "patch/openssl" }` — Red Hat's forked openssl crate for FIPS and TLS security profiles
- `hyper = { path = "patch/hyper" }` — HTTP behavior patches
- `ntapi` — pinned revision for Windows alignment bug (inherited from upstream)

**v0.47.0-rh only** (removed in v0.54.0-rh):
- `tokio-util` — vectordotdev fork for framed-read-continue-on-error
- `nix` — vectordotdev fork for memfd gnu/musl support

When modifying TLS or HTTP behavior, check `patch/openssl/` and `patch/hyper/` — the standard crate.io versions are not used.

## Jira Conventions

Issues are tracked in [Red Hat JIRA](https://issues.redhat.com) under the **LOG** project.

- Prefix commit messages with the Jira ticket: `LOG-XXXX: description`
- Reference tickets in PR descriptions
- Use conventional commit format for PR titles: `fix(scope): description`

## Code Review (OWNERS)

```
approvers: jcantrill, alanconway, xperimental
reviewers: jcantrill, Clee2691, syedriko, vparfonov, cahartma
```

Dockerfile changes get the `midstream/Dockerfile` label automatically.

## Upstream References

For generic Vector development, use these upstream docs (they apply to this fork's Rust code):

| Topic                                                      | Upstream Document                                                                                     |
|------------------------------------------------------------|-------------------------------------------------------------------------------------------------------|
| Dev commands, project structure, common patterns           | [AGENTS.md](https://github.com/vectordotdev/vector/blob/master/AGENTS.md)                             |
| Contribution workflow, PR format                           | [CONTRIBUTING.md](https://github.com/vectordotdev/vector/blob/master/CONTRIBUTING.md)                 |
| Internal engine architecture (topology, fanout, buffering) | [docs/ARCHITECTURE.md](https://github.com/vectordotdev/vector/blob/master/docs/ARCHITECTURE.md)       |
| Rust coding conventions (agent-oriented)                   | [docs/RUST_STYLE.md](https://github.com/vectordotdev/vector/blob/master/docs/RUST_STYLE.md)           |
| Code style (logging, metrics, error handling, concurrency) | [STYLE.md](https://github.com/vectordotdev/vector/blob/master/STYLE.md)                               |
| Developer setup, testing, benchmarking                     | [docs/DEVELOPING.md](https://github.com/vectordotdev/vector/blob/master/docs/DEVELOPING.md)           |
| Component specification                                    | [docs/specs/component.md](https://github.com/vectordotdev/vector/blob/master/docs/specs/component.md) |
