# Vector

This repo is a fork of [vector](https://github.com/vectordotdev/vector) and contains patches carried by Red Hat OpenShift Logging. This is a log collector and forwarder that resides on each OpenShift node to gather application and node logs. Please refer to the [cluster-logging-operator](https://github.com/openshift/cluster-logging-operator) for details regarding the operator that deploys and configures this image.  This image is intended to be run in conjunction with the configuration and `run.sh` files provided by the operator.  Experiences with the image outside that context may vary.

The `main` branch is empty except for this file.  The branches used by various Red Hat releases are summarized here:

| Release | Branch | Vector Version | Status |
| --------|--------|-----------------|--------|
| 6.4 | v0.40.0-rh | v0.40.0 | Pending |
| 6.3 | v0.37.1-rh | v0.37.1 | Current |
| 6.2 | v0.37.1-rh | v0.37.1 | Current |
| 6.1 | v0.37.1-rh | v0.37.1 | Current |
| 6.0 | v0.37.1-rh | v0.37.1 | Current |
| 5.9 | release-5.9| v0.33.0 | Current |
| 5.8 | release-5.8| v0.28.1 | Current |
| 5.7 | release-5.7| v0.20.1 | EOL     |
| 5.6 | release-5.6| v1.20.1 | EOL     |


## Issues

Any issues can be filed at [Red Hat JIRA](https://issues.redhat.com).  Please
include as many details as possible in order to assist in issue resolution along with attaching the output 
from the [must gather](https://github.com/openshift/cluster-logging-operator/tree/master/must-gather) associated with the release.
