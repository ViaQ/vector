# Vector — Red Hat OpenShift Logging

This is a fork of [Vector](https://github.com/vectordotdev/vector) maintained for Red Hat OpenShift Logging. Vector is a high-performance observability data pipeline written in Rust. This fork carries patches for OpenShift integration, FIPS-compliant TLS (OpenSSL), and reliability improvements.

Vector runs as a DaemonSet on each OpenShift node, collecting application and infrastructure logs. It is deployed and configured by the [cluster-logging-operator](https://github.com/openshift/cluster-logging-operator). This image is intended to run with the configuration and `run.sh` files provided by the operator.

## Documentation

| Document | Purpose |
|----------|---------|
| [AGENTS.md](AGENTS.md) | AI agent and developer quick reference — fork-specific context, build commands, patched crates |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to submit changes — branching, PRs, code review, Jira conventions |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Fork architecture — deployment model, `ocp-logging` feature scope, build system, design decisions |
| [Upstream AGENTS.md](https://github.com/vectordotdev/vector/blob/master/AGENTS.md) | Generic Vector development commands, project structure, Rust conventions |

## Quick Start (building from source)

```bash
git checkout v0.47.0-rh          # or v0.54.0-rh
make build                       # builds with --features ocp-logging
make test                        # runs tests with --test-threads 1
```

## Release Branches

The `rh-main` branch contains only documentation. All development happens on version-specific branches:

| Release | Branch      | Vector Version | Status |
|---------|-------------|----------------|--------|
| next    | v0.54.0-rh  | v0.54.0        | Development |
| 6.5     | v0.47.0-rh  | v0.47.0        | Current |
| 6.4     | v0.47.0-rh  | v0.47.0        | Current |
| 6.3     | v0.47.0-rh  | v0.47.0        | EOL     |
| 6.2     | v0.47.0-rh  | v0.47.0        | Current |
| 6.1     | v0.37.1-rh  | v0.37.1        | EOL     |
| 6.0     | v0.37.1-rh  | v0.37.1        | Current |
| 5.9     | release-5.9 | v0.34.1        | EOL |
| 5.8     | release-5.8 | v0.28.1        | EOL |
| 5.7     | release-5.7 | v0.21.0        | EOL |
| 5.6     | release-5.6 | v0.21.0        | EOL |

This project varies from the upstream with the following features:

| Issue    | Description  | Release Added | Upstream Contribution                                            |
|----------|--------------|---------------|------------------------------------------------------------------|
| LOG-2552 | [Replace Ring with OpenSSL](https://github.com/ViaQ/vector/pull/61) | 5.5           | N/A - Patch is Red Hat only                                      |
| LOG-3398 | [Apply TLSSecurityProfile settings to TLS listeners in log collectors](https://github.com/ViaQ/vector/pull/129) | 5.6           | N/A - Relies upon OpenSSL patch                                  |
| LOG-2288 | [Add syslog sink](https://github.com/ViaQ/vector/pull/133) | 5.7           | [Accepted](https://github.com/vectordotdev/vector/pull/23777)        |
| LOG-3949 | [Add support for file rotate_wait_secs](https://github.com/ViaQ/vector/pull/154) | 5.8           | [Accepted](https://github.com/vectordotdev/vector/pull/18904)    |
| LOG-4739 | [Add support for include_paths_glob_pattern](https://github.com/ViaQ/vector/pull/167) | 5.9           | [Accepted](https://github.com/vectordotdev/vector/pull/19521)    |
| LOG-6155 | [Allow config of message_key for multiline exception transform](https://github.com/ViaQ/vector/pull/183) | 6.2           | N/A - Transform is Red Hat only                                  |
| LOG-6789 | [Resolve error when using AWS credentials file authentication](https://github.com/ViaQ/vector/pull/197) | 6.2           | [Accepted](https://github.com/vectordotdev/vector/pull/22831)    |
| LOG-7013 | [Update cloudwatch logs max event size to match new AWS limit](https://github.com/ViaQ/vector/pull/201) | 6.2           | [Accepted](https://github.com/vectordotdev/vector/pull/22886)    |
| LOG-7417 | [Fix loki event timestamp out of range panic ](https://github.com/ViaQ/vector/pull/217) | 0.37.1-rh     | [Cherry-pick](https://github.com/vectordotdev/vector/pull/20780) |

## Issues

Any issues can be filed at [Red Hat JIRA](https://issues.redhat.com). Please
include as many details as possible in order to assist in issue resolution along with attaching the output
from the [must gather](https://github.com/openshift/cluster-logging-operator/tree/master/must-gather) associated with the release.
