---
format: aep.planning-md/3
id: epic:agency-capacity
kind: epic
status: proposed
title: 'M4 Capacity: credential headroom and storage per run'
summary: Several subscription credentials chosen and rotated by headroom, and storage counted in a workforce's capacity (conductor design § 3.2 G7, G8).
revision: 2
transitions:
- {from: "draft", to: "proposed", at: "2026-10-26T14:54:20Z", actor: "human:timo", revision: 2}
---
## Outcome

Unattended runs that last: a run stopped on a credential's usage limit resumes on another
credential with headroom, and a workforce admits a run only while the storage its workspace and
build output need is free. Source: conductor `docs/design/conductor-on-agency.md` § 3.2, gaps G7
and G8 (conductor main `9fa423a6`).

## Acceptance

The stories under this epic pass their named conformance scenarios on the service target, and one
run stopped on a usage limit resumes on a second credential, traced end to end.

## Rule

Agency chooses which credential and how much storage a run gets; it never holds a secret value
(custody stays in Connectors) and never sets a pod's limits itself (Loom does, from the
`LoomAgent` Agency writes).
