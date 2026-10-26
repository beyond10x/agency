# AGENTS.md

Agency: the service and cockpit for agents, tools, teams, work and auth (what it is: [README.md](README.md)).
The ESS domain in `ess/` comes first; everything else is derived from it or held to it.

Running code is Rust with clap derive. Browser code is Vue 3 with TypeScript, built with Vite and
pnpm, and its types are generated from `ess/`.

## Layout

| Path | What it is | Who writes it |
|---|---|---|
| `ess/` | ESS source of the `agency` domain, listed in `ess/ess-inputs.yaml` | authored |
| `generated/` | ESS projections, the synthesized Rust crate, TypeScript types, the synthesized conformance suite | `task generate` |
| `conformance/` | Per-target baselines: count floors and ceilings, known failures by name with a reason | authored |
| `crates/` | Hand-written Rust | authored |
| `.engineering/` | AEP planning store; the plan lives here, not in chat or plan files | `aep` |

## Commands

| Command | Does |
|---|---|
| `task check` | The gate: validate `ess/` (`--strict-requires`) and the plan, fail on drift of anything under `generated/`, run the conformance suite, then fmt, clippy, test, build the generated crate |
| `task conformance` | Run `generated/conformance/suite.json` against the interpreted target and hold the report to `conformance/interpreted.baseline.json` |
| `cargo test -p agency-conformance` | Run the suite against the synthesized crate `generated/rust/agency` and hold it to `conformance/generated.baseline.json` (part of `task check`) |
| `task generate` | Rewrite `generated/` after a change to `ess/` |
| `task plan` | `aep plan artifact validate` |

## Rules

- Change `ess/` first, then `task generate`. Never edit `generated/` by hand; `task check` fails on drift.
- The conformance gate reads the report, never the runner's exit status. A failure is admitted only by
  name in `conformance/<target>.baseline.json`, with the ESS defect that causes it; never widen a
  ceiling or add a name to make the gate green. A quarantined scenario that passes must leave the
  list.
- `ess/ess-inputs.yaml` pins the ess release (`requires:`); move it, the CI `check` workflow and
  the regenerated `generated/` in one change.
- Agency holds policy input, records and custody pointers. It never decides a single tool call
  (metaharness does), never runs a model loop (Loom does), and never stores a secret value or a
  work item's body: a credential is a custody pointer, a work item is a reference plus state.
- Every commit and push is `b10x-bot[bot]` through `b10x-gates bot`; every other GitHub write goes
  through `b10x-gates api`. The repository is enrolled in the common Gates policy
  (`.github/workflows/shared-gates.yml`).
- Nothing from a customer or employer outside beyond10x enters this repository.

## Releases

Releases are bare-version tags (`0.1.0`, no `v`). `CHANGELOG.md` follows Keep a Changelog 1.1.0;
changes collect under `## [Unreleased]`. Never move or delete a pushed tag; fix forward.
