# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/KeyState/keystate-adapter-keycloak/releases/tag/v0.3.0) - 2026-08-18

### Added

- render importable realm-export format for round-trip validation
- scaffold modular Keycloak adapter (v0.2.0 realm extraction)

### Fixed

- run both release-pr and release in the release-plz pipeline

### Other

- update supported-version assertion to 26.5.5
- support Keycloak 26.5.5
- fix audit critical + make integration job order-safe
- fix cargo-deny license allowlist and git wildcard dep
- run integration tests against live Keycloak in the pipeline
- integration tests against live Keycloak 26.7 + compose stack
- align dev/release workflow with the develop/main branch model
- add CI quality gates and release automation
- Initial commit
