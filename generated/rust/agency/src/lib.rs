// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! Semantic types synthesised from the `agency` specification, v1.
//!
//! The deployable service and cockpit from which an agentic engineer defines agents, gives them tools and credentials, forms them into teams, feeds them work, and watches the runs Loom executes for them (docs/design/agency.md §1).
//!
//! Generated, not written: the specification is the source of truth, and the door to changing
//! anything here is `ess synthesize`. What is deliberately absent — behaviour, queries,
//! escalations — is listed with reasons in the `PLAN.md` beside this workspace, and every entry
//! there is owed through a typed seam in an `obligations` module here.

// `deny`, not the source workspace's lint set: this crate must hold on its own, and an undocumented
// public item here is an emitter defect worth failing the gate over.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod actor;
pub mod agents;
pub mod auth;
pub mod behaviour;
pub mod json;
pub mod obligation;
pub mod primitives;
pub mod runs;
pub mod service;
pub mod shared;
pub mod teams;
pub mod tools;
pub mod work;

pub mod ports;
pub mod system;
#[cfg(feature = "server")]
pub mod server;
