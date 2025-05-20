# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.6](https://github.com/jdrouet/no-proxy/compare/v0.3.5...v0.3.6) - 2025-03-07

### Other

- replace cidr-utils with cidre crate

## [0.3.5](https://github.com/jdrouet/no-proxy/compare/v0.3.4...v0.3.5) - 2024-10-04

### Fixed

- *(test)* update tests to be valid
- *(deps)* update for cidr-utils changes

### Other

- add format and check test ([#8](https://github.com/jdrouet/no-proxy/pull/8))
- remove default features for async-graphql
- avoid memory alloc
- avoid allocation when not needed
- use any to go through possible match
- *(deps)* bump async-graphql and cidr-utils
- create release-plz configuration
