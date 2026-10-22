# agency

Agency is the service and cockpit from which an agentic engineer defines agents, gives them tools
and credentials, forms them into teams, feeds them work, and watches the runs that
[Loom](https://github.com/beyond10x/loom) executes for them. One Rust service with one HTTP API, one
event stream and a browser cockpit; applications such as conductor drive it through that API.

Status: specification first. The ESS domain `agency` (agents, tools, teams, work, auth) is the
contract; the service, the Loom driver and the cockpit follow.

| Path | Contents |
|---|---|
| `ess/` | The specification of the `agency` domain |
| `generated/` | What ESS derives from it: OpenAPI, AsyncAPI, JSON Schema, Rust and TypeScript types, the conformance suite |
| `crates/` | Hand-written Rust |
| `.engineering/` | The plan |

```console
task check
```

validates the specification and fails when anything generated from it has drifted.

Agent instructions are in [AGENTS.md](AGENTS.md); changes are listed in [CHANGELOG.md](CHANGELOG.md).

Licensed under Apache-2.0.
