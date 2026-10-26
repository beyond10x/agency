---
format: aep.planning-md/3
id: epic:agency-tools-policy
kind: epic
status: proposed
title: M3 Tools and policy
summary: RegisterTool from a tool-contract/1 ref, BindTool, Policy to EffectivePolicy to a sealed Frame; Ask decisions surface as approvals.
revision: 2
transitions:
- {from: "draft", to: "proposed", at: "2026-10-26T14:54:20Z", actor: "human:timo", revision: 2}
---
## Outcome

Tools registered from `tool-contract/1` references, bindings with scope and expiry, policy rules resolved into a digested `EffectivePolicy` handed to Loom as the run's authority input, an approval inbox for `Ask` decisions, and a `ToolCallLog` of metaharness decisions.

## Acceptance

A `Deny` and a `Replace` decision on a real run are observed in `ToolCallLog`.

## Rule

Agency never decides a single tool call; metaharness does (ADR 0104).
