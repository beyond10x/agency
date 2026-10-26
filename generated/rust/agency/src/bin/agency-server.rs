// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! Ephemeral generated server. Durable storage and production authentication remain ports.
use clap::Parser;

#[derive(Parser)]
struct Options {
#[arg(long, default_value = "127.0.0.1:8080")]
listen: String,
#[arg(long, default_value = "none", value_parser = ["none", "actor-header"])]
callers: String,
#[arg(long = "static")]
static_root: Option<std::path::PathBuf>,
}

fn main() -> std::process::ExitCode {
let options = Options::parse();
let _ = options;
eprintln!("unmet startup obligations:\nassigned value: Integer\nassigned value: agency.shared.Digest\nexternal branch answer: agency.teams.Report/not-one-level-up\nexternal branch answer: agency.teams.Send/foreign-instance\nexternal branch answer: agency.teams.Send/outside-team\nview query: agency.auth.EffectivePolicy\nview query: agency.runs.RunDetail\nview query: agency.teams.TeamDetail\nview query: agency.work.WorkItemDetail\nview query: agency.work.WorkQueue");
std::process::ExitCode::FAILURE
}
