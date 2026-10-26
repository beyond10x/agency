// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! Shared — `agency.shared`.
//!
//! Value types shared by the Agency sub-domains.
//!
//! Everything this bounded context declares that the synthesis plan marks generated.

/// Digest — `agency.shared.Digest`: a distinct wrapper around `String`.
///
/// A content digest, as a sealed definition, policy or frame carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Digest(pub String);

/// RunBudget — `agency.shared.RunBudget`.
///
/// The token and deadline limits a run is held to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunBudget {
    /// `max_tokens` — `Optional<Integer>`.
    pub max_tokens: Option<i64>,
    /// `deadline` — `Optional<Timestamp>`.
    pub deadline: Option<crate::primitives::Timestamp>,
}

/// Verb — `agency.shared.Verb`: a distinct wrapper around `String`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verb(pub String);
