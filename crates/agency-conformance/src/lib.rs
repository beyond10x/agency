//! The Rust ESS conformance target for the synthesized crate `generated/rust/agency`.
//!
//! [`AgencyTarget`] is the `ess_conformance::ConformanceTarget` that the committed suite
//! `generated/conformance/suite.json` runs against. [`run_suite`] runs a suite against it and
//! returns the `ess-conformance-report/2` document, which [`gate`] holds to a committed baseline.
//!
//! Every command and every view goes through `agency::server::agency::handle`, the transport-free
//! entry point that runs the same code the generated HTTP routes run: the generated grant check
//! (`admit`, `admit_read`), the generated input decoder, the generated behaviour over its ports,
//! the generated query, and the generated rendering of the answer. The ports are the generated
//! in-memory storage behind [`ports::TargetPorts`], which also answers the context the
//! specification leaves to an implementation and the five view queries `PLAN.md` lists as
//! obligations.
//!
//! This crate translates values and records what the generated answer says was published. It
//! decides no outcome: each outcome, error and event is the one the generated answer names.
//! What the generated crate cannot do is answered unsupported, with the reason:
//!
//! * a command the system issues itself (`await_outcome`): the generated crate holds no issuer, so
//!   nothing issues `RecordSpecCheck` or the run read-backs, and nothing can be held or observed;
//! * redelivering an event: no binding reacts to anything the specification publishes
//!   (`System::pump`);
//! * a caller's attributes: the specification declares none, and the generated surface reads none.
//!
//! Each scenario starts from a fresh system, empty storage, an empty event log and no forced
//! external branch, so no observation of one scenario can satisfy another.

pub mod codec;
pub mod gate;
pub mod ports;

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use std::str::FromStr;

use agency::actor::{Actor, Caller};
use agency::behaviour::Generated;
use agency::json::Value;
use agency::ports::agency::Agency;
use agency::server::agency::handle;
use agency::server::entry::Refused;
use agency::system::System;
use ess_conformance::scenario::{ActorRef, CommandRef, ErrorRef, EventRef, OutcomeRef};
use ess_conformance::target::{
    AwaitAnswer, AwaitRequest, ConformanceTarget, DeclaredErrorValue, EventObservationRequest,
    ExternalOutcomeControl, ImplementationIdentity, ObservedEvent, RedeliveryRequest,
    ScenarioContext, SemanticCommandRequest, SemanticCommandResult, SemanticViewRequest,
    SemanticViewResult, TargetError, ViewRow,
};
use ess_conformance::{AdmittedSuite, CountReport, Runner};
use ess_primitives::consistency::ConsistencyToken;
use ess_primitives::ids::CorrelationId;

use crate::ports::{Control, TargetPorts};

/// The name this implementation is reported under.
pub const IMPLEMENTATION: &str = "agency-generated";

/// The generated system, over the target's ports.
type AgencySystem = System<Generated<TargetPorts>>;

/// One scenario's state.
struct Live {
    system: AgencySystem,
    control: Rc<RefCell<Control>>,
    /// Every event the generated answers reported published in this scenario, in order.
    log: Vec<ObservedEvent>,
    /// The invocations answered, which consistency tokens are minted from.
    sequence: u64,
}

impl Live {
    fn open() -> Self {
        let control = Rc::new(RefCell::new(Control::default()));
        let ports = TargetPorts::new(Rc::clone(&control));
        Self {
            system: System::new(Agency::new(Generated::new(ports))),
            control,
            log: Vec::new(),
            sequence: 0,
        }
    }

    fn token(&mut self) -> ConsistencyToken {
        self.sequence += 1;
        ConsistencyToken::new(format!("seq:{}", self.sequence))
            .expect("`seq:` and decimal digits is a well-formed token")
    }

    /// Records `event` as published, returning it for the command's own result.
    fn publish(&mut self, event: ObservedEvent, correlation: &CorrelationId) -> ObservedEvent {
        let sequence = u64::try_from(self.log.len()).expect("the log fits") + 1;
        let event = event.in_activity(correlation.clone()).at(sequence);
        self.log.push(event.clone());
        event
    }

    /// The generated answer of `command`, `{outcome, published, error?}`, as a command result.
    fn took(
        &mut self,
        command: &str,
        answer: &Value,
        correlation: &CorrelationId,
    ) -> Result<SemanticCommandResult, String> {
        let Value::Object(members) = answer else {
            return Err(format!("the answer is {}", answer.describes()));
        };
        if let Some((name, _)) = members
            .iter()
            .find(|(name, _)| !matches!(name.as_str(), "outcome" | "published" | "error"))
        {
            return Err(format!(
                "the answer carries `{name}`, which this target does not read"
            ));
        }
        let outcome = text(answer, "outcome")?;
        let mut result = SemanticCommandResult::took(OutcomeRef::new(
            parse::<CommandRef>(command)?,
            parse(outcome)?,
        ));
        if answer.member("error").is_some() {
            let error = parse::<ErrorRef>(text(answer, "error")?)?;
            result = result.with_error(DeclaredErrorValue::new(error));
        }
        let Some(Value::Array(published)) = answer.member("published") else {
            return Err("the answer carries no `published` list".to_owned());
        };
        for occurrence in published {
            let mut event = ObservedEvent::new(parse::<EventRef>(text(occurrence, "event")?)?);
            let payload = occurrence
                .member("payload")
                .ok_or("a published event carries no `payload`")?;
            event.payload = codec::members(payload)?;
            result = result.emitting(self.publish(event, correlation));
        }
        Ok(result)
    }
}

/// The text member `name` of `value`.
fn text<'a>(value: &'a Value, name: &str) -> Result<&'a str, String> {
    match value.member(name) {
        Some(Value::Text(text)) => Ok(text),
        Some(other) => Err(format!("`{name}` is {}", other.describes())),
        None => Err(format!("no `{name}`")),
    }
}

fn parse<T: FromStr>(text: &str) -> Result<T, String>
where
    T::Err: Display,
{
    text.parse()
        .map_err(|error| format!("`{text}` is not a well-formed name: {error}"))
}

/// The generated caller for the declared actor `actor`.
fn caller(actor: &ActorRef, operation: &str) -> Result<Caller, TargetError> {
    let name = actor.to_string();
    Actor::ALL
        .iter()
        .find(|declared| declared.name() == name)
        .map(|&actor| Caller { actor })
        .ok_or_else(|| {
            TargetError::unavailable(
                operation.to_owned(),
                format!("the generated crate declares no actor `{name}`"),
            )
        })
}

/// Who a read is sent as.
enum Reader<'a> {
    /// The harness, reading on its own authority to check what a command did: as the first actor
    /// the generated grant (`agency::actor::may_read`) lets read the view.
    Harness,
    /// The actor a `ReadAs` step named.
    Actor(&'a ActorRef),
    /// No actor, after a `ReadAs` step naming none.
    Nobody,
}

/// The conformance target over the generated crate.
#[derive(Default)]
pub struct AgencyTarget {
    live: RefCell<Option<Live>>,
}

impl AgencyTarget {
    /// A target with no scenario open.
    pub fn new() -> Self {
        Self::default()
    }

    fn read(
        &self,
        request: SemanticViewRequest,
        reader: Reader<'_>,
    ) -> Result<SemanticViewResult, TargetError> {
        let view = request.view.to_string();
        let operation = format!("reading `{view}`");
        let caller = match reader {
            Reader::Harness => Some(
                Actor::ALL
                    .iter()
                    .find(|actor| agency::actor::may_read(**actor).contains(&view.as_str()))
                    .map(|&actor| Caller { actor })
                    .ok_or_else(|| {
                        TargetError::unsupported(
                            operation.clone(),
                            "no actor's generated grant may read this view, so the harness has \
                             no caller to read it as",
                        )
                    })?,
            ),
            Reader::Actor(actor) => Some(caller(actor, &operation)?),
            Reader::Nobody => None,
        };
        let mut slot = self.live.borrow_mut();
        let live = current(&mut slot, &operation)?;
        let answer = match handle(
            &mut live.system,
            caller.as_ref(),
            &view,
            codec::object(&request.params),
        ) {
            Ok(answer) => answer,
            Err(Refused::NotGranted(actor)) => return Err(TargetError::not_granted(actor)),
            Err(Refused::Unmet(detail)) => {
                return Err(TargetError::unsupported(
                    operation,
                    format!("the generated query reports an unmet obligation: {detail}"),
                ));
            }
            Err(refused) => return Err(TargetError::unavailable(operation, refused.to_string())),
        };
        let rows = match answer.member("rows") {
            Some(Value::Array(rows)) => rows
                .iter()
                .map(codec::members)
                .collect::<Result<Vec<ViewRow>, _>>()
                .map_err(|detail| TargetError::unavailable(operation.clone(), detail))?,
            _ => {
                return Err(TargetError::unavailable(
                    operation,
                    "the generated answer carries no `rows` list",
                ));
            }
        };
        Ok(SemanticViewResult::of(rows))
    }
}

impl std::fmt::Debug for AgencyTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AgencyTarget").finish_non_exhaustive()
    }
}

/// The scenario's state, or the error a call outside a scenario gets.
fn current<'a>(slot: &'a mut Option<Live>, operation: &str) -> Result<&'a mut Live, TargetError> {
    slot.as_mut().ok_or_else(|| {
        TargetError::unavailable(operation.to_owned(), "no scenario is open".to_owned())
    })
}

impl ConformanceTarget for AgencyTarget {
    fn identity(&self) -> Result<ImplementationIdentity, TargetError> {
        Ok(ImplementationIdentity::new(
            IMPLEMENTATION,
            env!("CARGO_PKG_VERSION"),
        ))
    }

    fn begin_scenario(&self, _scenario: &ScenarioContext) -> Result<(), TargetError> {
        *self.live.borrow_mut() = Some(Live::open());
        Ok(())
    }

    fn execute_command(
        &self,
        request: SemanticCommandRequest,
    ) -> Result<SemanticCommandResult, TargetError> {
        let command = request.command.to_string();
        let operation = format!("invoking `{command}`");
        if request.caller.is_some() {
            return Err(TargetError::unsupported(
                format!("sending `{command}` with caller attributes"),
                "the specification declares no caller attribute, and the generated surface reads \
                 none",
            ));
        }
        let caller = request
            .actor
            .as_ref()
            .map(|actor| caller(actor, &operation))
            .transpose()?;
        let mut slot = self.live.borrow_mut();
        let live = current(&mut slot, &operation)?;
        let answered = handle(
            &mut live.system,
            caller.as_ref(),
            &command,
            codec::object(&request.input),
        );
        live.control.borrow_mut().invoked(&command);
        let answer = match answered {
            Ok(answer) => answer,
            Err(Refused::NotGranted(actor)) => return Err(TargetError::not_granted(actor)),
            // The generated decoder refused the input: no declared outcome answers it.
            Err(Refused::Input(_)) => return Ok(SemanticCommandResult::undeclared()),
            Err(Refused::Unmet(detail)) => {
                return Err(TargetError::unsupported(
                    operation,
                    format!("the generated behaviour reports an unmet obligation: {detail}"),
                ));
            }
            Err(refused) => return Err(TargetError::unavailable(operation, refused.to_string())),
        };
        let result = live
            .took(&command, &answer, &request.correlation)
            .map_err(|detail| {
                TargetError::unavailable(format!("reading the answer of `{command}`"), detail)
            })?;
        Ok(result.with_consistency(live.token()))
    }

    fn query_view(&self, request: SemanticViewRequest) -> Result<SemanticViewResult, TargetError> {
        self.read(request, Reader::Harness)
    }

    fn query_view_as(
        &self,
        request: SemanticViewRequest,
        reader: &ActorRef,
    ) -> Result<SemanticViewResult, TargetError> {
        self.read(request, Reader::Actor(reader))
    }

    fn query_view_anonymous(
        &self,
        request: SemanticViewRequest,
    ) -> Result<SemanticViewResult, TargetError> {
        self.read(request, Reader::Nobody)
    }

    fn observe_events(
        &self,
        request: EventObservationRequest,
    ) -> Result<Vec<ObservedEvent>, TargetError> {
        let slot = self.live.borrow();
        Ok(slot
            .iter()
            .flat_map(|live| live.log.iter())
            .filter(|published| published.event == request.event)
            .cloned()
            .collect())
    }

    fn configure_external_outcome(
        &self,
        request: ExternalOutcomeControl,
    ) -> Result<(), TargetError> {
        let mut slot = self.live.borrow_mut();
        let live = current(&mut slot, &format!("forcing `{}`", request.force))?;
        live.control.borrow_mut().force(
            request.force.command.to_string(),
            request.force.outcome.to_string(),
        );
        Ok(())
    }

    fn redeliver_event(&self, request: RedeliveryRequest) -> Result<(), TargetError> {
        Err(TargetError::unsupported(
            format!("redelivering `{}`", request.event),
            "no binding reacts to anything the specification publishes, so the generated system \
             has nothing to deliver it to",
        ))
    }

    fn await_outcome(&self, request: AwaitRequest) -> Result<AwaitAnswer, TargetError> {
        Err(TargetError::unsupported(
            format!("awaiting `{}` on an instance", request.command),
            "generated/rust/agency holds no issuer of a system-issued command: nothing in it issues \
             the command, so there is no step to hold or observe, and issuing it from this target \
             would be the target's decision rather than the system's",
        ))
    }

    fn end_scenario(&self, _scenario: &ScenarioContext) -> Result<(), TargetError> {
        *self.live.borrow_mut() = None;
        Ok(())
    }
}

/// One run of a suite against [`AgencyTarget`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Executed {
    /// `ess-conformance-report/2`, canonical: the document the verdict is read from.
    pub report: String,
    /// The runner's per-scenario results with their diagnostics, for a reader of a red run.
    pub diagnostics: String,
}

/// Admit `suite` (the JSON `ess verify conform synthesize` writes), run it against an
/// [`AgencyTarget`], and return the report.
///
/// # Errors
///
/// The admission failure when the bytes are not an admissible suite, and the report failure when
/// the run and the suite disagree.
pub fn run_suite(suite: &str) -> Result<Executed, String> {
    let admitted = AdmittedSuite::from_json(suite).map_err(|error| error.to_string())?;
    let target = AgencyTarget::new();
    let executed = Runner::for_suite(admitted.suite()).run_admitted(&admitted, &target);
    let report = CountReport::from_run(&executed, &admitted)
        .and_then(|report| report.to_canonical_json())
        .map_err(|error| error.to_string())?;
    let diagnostics =
        serde_json::to_string_pretty(&*executed).map_err(|error| error.to_string())?;
    Ok(Executed {
        report,
        diagnostics,
    })
}
