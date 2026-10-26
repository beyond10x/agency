//! The conformance gate: a report's counts held to a committed baseline,
//! `conformance/<target>.baseline.json`. The same rules `agency-xtask conformance` applies to the
//! interpreted target (`crates/agency-xtask/src/conformance.rs`), copied rather than shared because
//! that crate is a binary.
//!
//! The verdict is read from the report document, never from whether the run returned: a run that
//! answered nothing also returns.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::Value;

/// What one target's run must keep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Baseline {
    /// The fewest scenarios the report may count.
    pub total_floor: u64,
    /// The fewest scenarios that may be answered, passed or failed.
    pub answered_floor: u64,
    /// The most scenarios that may be skipped, unsupported or in error.
    pub unanswered_ceiling: u64,
    /// The scenarios known to fail, each with the reason.
    pub quarantine: BTreeMap<String, String>,
}

/// The counts of one report and the scenarios it lists, by status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// The count of each status, and `total`.
    pub counts: BTreeMap<String, u64>,
    /// The scenarios of each status.
    pub outcomes: BTreeMap<String, Vec<String>>,
}

/// Reads a baseline file.
///
/// # Errors
///
/// When the file cannot be read, is not JSON, or misses a field.
pub fn read_baseline(path: &Path) -> Result<Baseline, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let value: Value =
        serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
    let number = |key: &str| {
        value
            .get(key)
            .and_then(Value::as_u64)
            .ok_or_else(|| format!("{}: `{key}` is not a whole number", path.display()))
    };
    let mut quarantine = BTreeMap::new();
    for entry in value
        .get("quarantine")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let scenario = entry.get("scenario").and_then(Value::as_str);
        let reason = entry.get("reason").and_then(Value::as_str);
        match (scenario, reason) {
            (Some(scenario), Some(reason)) => {
                quarantine.insert(scenario.to_owned(), reason.to_owned());
            }
            _ => {
                return Err(format!(
                    "{}: a quarantine entry needs `scenario` and `reason`",
                    path.display()
                ));
            }
        }
    }
    Ok(Baseline {
        total_floor: number("total_floor")?,
        answered_floor: number("answered_floor")?,
        unanswered_ceiling: number("unanswered_ceiling")?,
        quarantine,
    })
}

/// Reads the counts and outcomes of an `ess-conformance-report/2` document.
///
/// # Errors
///
/// When the text is not JSON or carries no counts.
pub fn parse_report(text: &str) -> Result<Report, String> {
    let value: Value = serde_json::from_str(text).map_err(|e| format!("parse report: {e}"))?;
    let summary = value.get("summary").unwrap_or(&value);
    let counts = summary
        .get("counts")
        .and_then(Value::as_object)
        .ok_or("the report has no counts")?
        .iter()
        .filter_map(|(key, count)| count.as_u64().map(|count| (key.clone(), count)))
        .collect();
    let outcomes = summary
        .get("outcomes")
        .and_then(Value::as_object)
        .map(|outcomes| {
            outcomes
                .iter()
                .map(|(status, names)| {
                    let names = names
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(Value::as_str)
                        .map(str::to_owned)
                        .collect();
                    (status.clone(), names)
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(Report { counts, outcomes })
}

/// Every way `report` breaks `baseline`; empty when it holds.
pub fn judge(baseline: &Baseline, report: &Report) -> Vec<String> {
    let count = |key: &str| report.counts.get(key).copied().unwrap_or(0);
    let names = |key: &str| report.outcomes.get(key).cloned().unwrap_or_default();
    let total = count("total");
    let answered = count("passed") + count("failed");
    let unanswered = count("skipped") + count("unsupported") + count("error");
    let mut problems = Vec::new();
    if total == 0 {
        problems.push("the suite ran no scenario".to_owned());
    }
    if count("passed") == 0 {
        problems.push("no scenario passed, so nothing was asserted".to_owned());
    }
    if total < baseline.total_floor {
        problems.push(format!(
            "total {total} is below the floor {}",
            baseline.total_floor
        ));
    }
    if answered < baseline.answered_floor {
        problems.push(format!(
            "answered {answered} is below the floor {}",
            baseline.answered_floor
        ));
    }
    if unanswered > baseline.unanswered_ceiling {
        problems.push(format!(
            "skipped, unsupported and error {unanswered} exceed the ceiling {}",
            baseline.unanswered_ceiling
        ));
    }
    let failed = names("failed");
    for scenario in failed.iter().chain(names("error").iter()) {
        if !baseline.quarantine.contains_key(scenario) {
            problems.push(format!("not a known failure: {scenario}"));
        }
    }
    for scenario in baseline.quarantine.keys() {
        if !failed.contains(scenario) {
            problems.push(format!(
                "quarantined but no longer failing, remove it: {scenario}"
            ));
        }
    }
    problems
}

/// One line of counts, by status.
pub fn counts_line(target: &str, report: &Report) -> String {
    let shown: Vec<String> = [
        "total",
        "passed",
        "failed",
        "skipped",
        "unsupported",
        "error",
    ]
    .iter()
    .map(|key| format!("{key} {}", report.counts.get(*key).copied().unwrap_or(0)))
    .collect();
    format!("conformance ({target}): {}", shown.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report(passed: u64, failed: &[&str], unsupported: &[&str]) -> Report {
        let total = passed + failed.len() as u64 + unsupported.len() as u64;
        let counts = [
            ("total", total),
            ("passed", passed),
            ("failed", failed.len() as u64),
            ("unsupported", unsupported.len() as u64),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_owned(), v))
        .collect();
        let owned = |names: &[&str]| names.iter().map(|n| (*n).to_owned()).collect();
        let outcomes = [
            ("failed".to_owned(), owned(failed)),
            ("unsupported".to_owned(), owned(unsupported)),
        ]
        .into_iter()
        .collect();
        Report { counts, outcomes }
    }

    fn baseline(quarantine: &[&str]) -> Baseline {
        Baseline {
            total_floor: 4,
            answered_floor: 3,
            unanswered_ceiling: 1,
            quarantine: quarantine
                .iter()
                .map(|n| ((*n).to_owned(), "known".to_owned()))
                .collect(),
        }
    }

    #[test]
    fn holds_when_every_failure_is_quarantined() {
        assert!(judge(&baseline(&["a"]), &report(2, &["a"], &["b"])).is_empty());
    }

    #[test]
    fn refuses_an_unknown_failure() {
        let problems = judge(&baseline(&["a"]), &report(2, &["a", "c"], &[]));
        assert!(
            problems.iter().any(|p| p == "not a known failure: c"),
            "{problems:?}"
        );
    }

    #[test]
    fn refuses_a_stale_quarantine_entry() {
        let problems = judge(&baseline(&["a", "z"]), &report(3, &["a"], &[]));
        assert!(problems.iter().any(|p| p.ends_with(": z")), "{problems:?}");
    }

    #[test]
    fn refuses_a_pass_that_became_unsupported() {
        let problems = judge(&baseline(&[]), &report(2, &[], &["b", "c"]));
        assert!(
            problems.iter().any(|p| p.starts_with("answered 2")),
            "{problems:?}"
        );
        assert!(
            problems
                .iter()
                .any(|p| p.starts_with("skipped, unsupported and error 2")),
            "{problems:?}"
        );
    }

    #[test]
    fn refuses_a_run_that_asserted_nothing() {
        let problems = judge(&baseline(&[]), &report(0, &[], &[]));
        assert!(
            problems.iter().any(|p| p == "the suite ran no scenario"),
            "{problems:?}"
        );
    }
}
