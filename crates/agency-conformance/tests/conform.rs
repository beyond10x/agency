//! `cargo test` holds the synthesized crate `generated/rust/agency` to the committed suite
//! `generated/conformance/suite.json`: it runs the suite against `AgencyTarget`, writes the
//! `ess-conformance-report/2` document and the runner's diagnostics under `CARGO_TARGET_TMPDIR`,
//! and holds the report's counts to `conformance/generated.baseline.json` by the rules of
//! `agency_conformance::gate`.

use std::fs;
use std::path::{Path, PathBuf};

use agency_conformance::{gate, run_suite};
use serde_json::Value;

const SUITE: &str = "generated/conformance/suite.json";
const BASELINE: &str = "conformance/generated.baseline.json";
const REPORT_FORMAT: &str = "ess-conformance-report/2";

/// The repository root, read when the test runs: a test binary is reused across worktrees that
/// share one `CARGO_TARGET_DIR`.
fn root() -> PathBuf {
    let manifest = std::env::var_os("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|| panic!("CARGO_MANIFEST_DIR is unset: run this test through cargo"));
    let root = PathBuf::from(manifest).join("../..");
    root.canonicalize().unwrap_or(root)
}

#[test]
fn generated_target_holds_its_baseline() {
    let root = root();
    let suite_path = root.join(SUITE);
    let suite = fs::read_to_string(&suite_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", suite_path.display()));
    let scenarios = serde_json::from_str::<Value>(&suite).expect("the suite is JSON")["scenarios"]
        .as_object()
        .map(serde_json::Map::len)
        .expect("the suite carries an object of scenarios");

    let executed =
        run_suite(&suite).unwrap_or_else(|error| panic!("the suite did not run: {error}"));
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("agency-conformance");
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    let report_path = dir.join("report.json");
    let diagnostics_path = dir.join("diagnostics.json");
    fs::write(&report_path, &executed.report).expect("write report.json");
    fs::write(&diagnostics_path, &executed.diagnostics).expect("write diagnostics.json");

    let document: Value = serde_json::from_str(&executed.report).expect("the report is JSON");
    assert_eq!(
        document["format"],
        REPORT_FORMAT,
        "report: {}",
        report_path.display()
    );
    assert_eq!(
        document["counts"]["total"].as_u64(),
        u64::try_from(scenarios).ok(),
        "the report does not cover every scenario of the suite: {}",
        report_path.display()
    );

    let baseline =
        gate::read_baseline(&root.join(BASELINE)).unwrap_or_else(|error| panic!("{error}"));
    let report = gate::parse_report(&executed.report).unwrap_or_else(|error| panic!("{error}"));
    println!("{}", gate::counts_line("generated", &report));
    let problems = gate::judge(&baseline, &report);
    assert!(
        problems.is_empty(),
        "conformance (generated) broke its baseline:\n  {}\nreport: {}\ndiagnostics: {}",
        problems.join("\n  "),
        report_path.display(),
        diagnostics_path.display()
    );
}
