---
format: aep.planning-md/3
id: story:agency-program-agents
kind: story
status: draft
title: 'An agent definition describes a program agent: the Command variant of loom.agent/1, held by value'
summary: The definition's loom.agent/1 document carries a Command agent (image, entrypoint, workspace, takt's run configuration), checked by Loom (conductor design § 3.2 G1).
tags:
- conductor-gap:G1
relations:
- decomposes: epic:agency-spec
revision: 1
---
## Outcome

An `AgentDefinition` describes a program agent: its `loom.agent/1` document carries the `Command`
variant (an image, an entrypoint, a workspace and the program's run configuration, such as takt's),
held by value, checked by Loom, and published like any other definition. Agency models the
variant inside the document, not beside it.

## Why

conductor `docs/design/conductor-on-agency.md` § 3.2 G1 (conductor main `9fa423a6`): a takt
delivery is a `Command` agent, and a definition that assumes a metaharness kind and an in-process
session cannot describe it. ADR 0108: Loom owns `loom.agent/1`; Agency holds definitions by value.

## Acceptance

Conformance scenarios (`ess/`, domain `agency.agents`) on the service target:

- `agency.agents.CreateAgentDefinition/outcome/created`
- `agency.agents.RecordSpecCheck/outcome/checked` and `agency.agents.RecordSpecCheck/outcome/refused`
- `agency.agents/authored/publish-and-provision`
- a new authored scenario `agency.agents/authored/program-agent`: a definition whose document is
  a `Command` agent with takt's run configuration in it is created, checked and published.

## Scope

- agency: `ess/domains/agents.yaml` (`agent_spec` is the document as `Json`, `RecordSpecCheck`
  records Loom's verdict and exit code; specified in M0), `ess/scenarios/`, the service's
  definition routes.
- loom (not filed here): takt's configuration expressible in `loom.agent/1` instead of opaque
  config files (trial gap "Command variant takes opaque config files instead of a mandatory
  loom.agent/1 spec.agent"; conductor design § 6 D3), and `b10x-loom agent check` accepting the
  `Command` variant.
- takt (not filed here): reads its run configuration from the document Loom mounts.

## Not in scope

Running the agent (`story:agency-deployed-runs`).
