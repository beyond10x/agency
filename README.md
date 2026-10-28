# agency

Agency is the service and cockpit from which an agentic engineer defines agents, gives them tools
and credentials, forms them into teams, feeds them work, and watches the runs that
[Loom](https://github.com/beyond10x/loom) executes for them. One Rust service with one HTTP API, one
event stream and a browser cockpit; applications such as conductor drive it through that API.

Status: specification first. The ESS system `agency` (domains shared, auth, agents, runs, tools,
teams, work and service; ess 0.74.0) is the contract; the service, the Loom driver and the cockpit
follow. An agent definition holds its `loom.agent/1` document by value; Loom owns that format.

| Path | Contents |
|---|---|
| `ess/` | The specification of the `agency` domain |
| `generated/` | What ESS derives from it: OpenAPI, AsyncAPI, JSON Schema, a synthesized Rust crate, TypeScript types, the conformance suite |
| `conformance/` | The counts each conformance target must hold, and its known failures by name |
| `crates/` | Hand-written Rust |
| `.engineering/` | The plan |

```console
task check
```

validates the specification and the plan, fails when anything generated from it has drifted, and
runs the conformance suite, holding its counts to `conformance/`.

Agent instructions are in [AGENTS.md](AGENTS.md); changes are listed in [CHANGELOG.md](CHANGELOG.md).

Licensed under Apache-2.0.
