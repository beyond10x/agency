---
format: aep.planning-md/3
id: epic:agency-cockpit
kind: epic
status: draft
title: M4 First usable cockpit
summary: Vue 3 + TypeScript cockpit (Board, Agents, Work, Teams, Run) on generated types, embedded in the binary.
revision: 1
---
## Outcome

A Vue 3, TypeScript, Vite and pnpm cockpit with Board, Agents, Work, Teams and Run screens, using TypeScript types generated from `ess/` and drift-checked, built into a bundle embedded in the `agency` binary.

## Acceptance

Timo runs one team on one queue from the browser.

## Rule

The server stays Rust; shared types come from `ess/`, never hand-written.
