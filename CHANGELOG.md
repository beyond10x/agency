# Changelog

All notable changes to this repository are recorded here, in the format of
[Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/). Releases are bare-version tags such as
`0.1.0`. Changes collect under `[Unreleased]` until the next release.

## [Unreleased]

### Added

- Repository, plan and gates.
- The ESS specification `ess/` (ess 0.69.2, `ess/31`): domains shared, auth, agents, runs, tools,
  teams, work and service, with five authored scenarios. Agent definitions hold their
  `loom.agent/1` document by value; run workspaces, run-level approvals and application sessions
  are specified.
- `generated/`: OpenAPI, AsyncAPI and JSON Schema projections, a synthesized Rust crate, TypeScript
  types and a 360-scenario conformance suite, held to `ess/` by `task check`.
- `task conformance`: the suite against the interpreted target, held to
  `conformance/interpreted.baseline.json` (`crates/agency-xtask`).
- `crates/agency-conformance`: a Rust conformance target over the synthesized crate, run by
  `cargo test` and held to `conformance/generated.baseline.json`.
- CI `check` workflow running `task check` with pinned task, ess and aep.
- Stories for the eight Agency gaps of conductor's design (G1 to G8) and `epic:agency-capacity`.

### Changed

- ESS pinned to 0.75.0 (`ess/ess-inputs.yaml`, the `ess-*` crates, the `check` workflow), and
  `generated/` regenerated: the synthesized crate's server gains `dispatch_with_events`,
  `handle_with_events` and per-event wire decoders; the suite and projections are unchanged.

[Unreleased]: https://github.com/beyond10x/agency/commits/main
