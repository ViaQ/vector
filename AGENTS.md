# AGENTS.md

This file provides guidance to AI models and human developers working with code in this repository.

## Project Overview

This repository is a Red Hat fork of the upstream Vector project, specifically maintained for OpenShift Logging. It serves as a log collector and forwarder that runs as a daemon set on each OpenShift cluster node to gather application and node logs.

**Important**: The `rh-main` branch is intentionally minimal and contains only documentation. All active development occurs in version-specific branches.

## Branch Structure

The actual implementation code resides in version-specific branches, not in `rh-main`. Current active branches include:

- `v0.47.0-rh` - OpenShift 6.4 (Vector v0.47.0)
- `v0.37.1-rh` - OpenShift 6.0-6.3 (Vector v0.37.1)
- `release-5.9` - OpenShift 5.9 (Vector v0.34.1)
- `release-5.8` - OpenShift 5.8 (Vector v0.28.1)

## Development Workflow

When working on this project:

1. **Branch Selection**: Switch to the appropriate version branch for your target OpenShift release
2. **Build System**: This is a Rust project using Cargo as the build system
3. **Integration**: The project is designed to be deployed and configured by the [cluster-logging-operator](https://github.com/openshift/cluster-logging-operator)
4. **Runtime**: Runs with configuration and `run.sh` files provided by the operator

## Red Hat Specific Modifications

This fork includes several patches not found in upstream Vector:

- **OpenSSL Integration** (LOG-2552): Replaces Ring with OpenSSL for Red Hat compliance
- **TLS Security Profile** (LOG-3398): Applies OpenShift TLS security settings
- **Syslog Sink** (LOG-2288): Custom syslog output capability
- **File Rotation** (LOG-3949): Enhanced file rotation controls
- **AWS Credentials** (LOG-6789): Fixes for AWS credentials file authentication
- **CloudWatch Limits** (LOG-7013): Updated to match current AWS event size limits

## Issue Reporting

- File issues at [Red Hat JIRA](https://issues.redhat.com)
- Include [must-gather](https://github.com/openshift/cluster-logging-operator/tree/master/must-gather) output for debugging
- Reference the specific OpenShift Logging release version

## Development Notes

- This project is tightly integrated with the OpenShift ecosystem
- Testing and deployment should consider the cluster-logging-operator context
- Changes should maintain compatibility with Red Hat's OpenShift Logging architecture
- Consider upstream contribution potential when making modifications
