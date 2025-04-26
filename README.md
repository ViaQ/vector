# Vector

This repo is a fork of [vector](https://github.com/vectordotdev/vector) and contains patches carried by Red Hat OpenShift Logging. This is a log collector and forwarder that resides on each OpenShift node to gather application and node logs. Please refer to the [cluster-logging-operator](https://github.com/openshift/cluster-logging-operator) for details regarding the operator that deploys and configures this image. This image is intended to be run in conjunction with the configuration and `run.sh` files provided by the operator. Experiences with the image outside that context may vary.

The `rh-main` branch is empty except for this file.  The branches used by various Red Hat releases are summarized here:

| Release | Branch | Vector Version | Status |
| --------|--------|----------------|--------|
| 6.2 | v0.37.1-rh | v0.37.1 | Current |
| 6.1 | release-6.1 | v0.37.1 | Current |
| 6.0 | release-6.0 | v0.37.1 | Current |
| 5.9 | release-5.9 | v0.34.1 | Current |
| 5.8 | release-5.8 | v0.28.1 | Current |
| 5.7 | release-5.7 | v0.21.0 | EOL |
| 5.6 | release-5.6 | v0.21.0 | EOL |

This project varies from the upstream with the following features:

| Issue | Description  | Release Added | Upstream Contribution |
|-------|--------------|---------------|-----------------------|
| LOG-2552 | [Replace Ring with OpenSSL](https://github.com/ViaQ/vector/pull/61) | 5.5 | N/A - Patch is Red Hat only |
| LOG-3398 | [Apply TLSSecurityProfile settings to TLS listeners in log collectors](https://github.com/ViaQ/vector/pull/129) | 5.6 | N/A - Relies upon OpenSSL patch |
| LOG-2288 | [Add syslog sink](https://github.com/ViaQ/vector/pull/133) | 5.7 | [Open](https://github.com/vectordotdev/vector/pull/21307) |
| LOG-3949 | [Add support for file rotate_wait_secs](https://github.com/ViaQ/vector/pull/154) | 5.8 | [Accepted](https://github.com/vectordotdev/vector/pull/18904) |
| LOG-4739 | [Add support for include_paths_glob_pattern](https://github.com/ViaQ/vector/pull/167) | 5.9 | [Accepted](https://github.com/vectordotdev/vector/pull/19521) |
| LOG-6155 | [Allow config of message_key for multiline exception transform](https://github.com/ViaQ/vector/pull/183) | 6.2 | N/A - Transform is Red Hat only |
| LOG-6789 | [Resolve error when using AWS credentials file authentication](https://github.com/ViaQ/vector/pull/197) | 6.2 | [Accepted](https://github.com/vectordotdev/vector/pull/22831) |
| LOG-7013 | [Update cloudwatch logs max event size to match new AWS limit](https://github.com/ViaQ/vector/pull/201) | 6.2 | [Accepted](https://github.com/vectordotdev/vector/pull/22886) |

## Issues

Any issues can be filed at [Red Hat JIRA](https://issues.redhat.com). Please
include as many details as possible in order to assist in issue resolution along with attaching the output
from the [must gather](https://github.com/openshift/cluster-logging-operator/tree/master/must-gather) associated with the release.
