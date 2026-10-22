---
format: aep.planning-md/3
id: approval-record:design-answers
kind: approval-record
status: draft
title: Timo's answers to the design's ten open questions
summary: All ten recommendations of docs/design/agency.md §8 taken; M0 approved
relations:
- decides: epic:agency-spec
revision: 1
---
## Decision

Timo, 2026-10-22, relayed by b10x-chef-orchestrator: answered "A" = all ten recommendations of
`docs/design/agency.md` §8.

| Q | Taken |
|---|---|
| Q1 | Name `agency` |
| Q2 | New public repository `beyond10x/agency`; control-plane is not archived ("Do not archive control-plane") |
| Q3 | Work items held as reference + state; the body stays in the source system |
| Q4 | conductor's team broker moves to Agency; conductor becomes a client |
| Q5 | Single local operator until M5; Mandate login at M5 |
| Q6 | Codex runs use `ApiKey` copied into the scratch home as the fallback, custody marked on the Run |
| Q7 | No takt link |
| Q8 | Agency links `loom-sdk` directly |
| Q9 | Swarm in Agency is a record plus the stop lever; the swarm product runs the loop |
| Q10 | ADR 0105 is amended by the atlas session, not here |

## Scope approved

Orchestrator: "(1) create public beyond10x/agency through the bot route the way beyond10x/a2a was
created, AGENTS.md + README per the docs skill, Taskfile.yml, AEP store with the 7 draft epics of §7,
enrolled in gates-policy like a2a (gates-policy 3d9ccf2); (2) M0: the ESS domain agency (14 entities
of §2) validated, compiled and projected, conformance suite synthesized, task check green, through
bot PRs."

§2 lists 15 entities (3 agents, 2 tools, 3 teams, 2 work, 5 auth); all 15 are in M0.
