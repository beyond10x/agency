use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use clap::{Parser, Subcommand};

mod conformance;

const SPECIFICATION: &str = "ess";
/// `.ess-output/` carries a per-run anchor; `Cargo.lock` is held by `--locked`, not by synthesis.
const IGNORED: [&str; 2] = [".ess-output", "Cargo.lock"];

/// One directory under `generated/` that a single `ess` invocation writes in full.
struct Artifact {
    dir: &'static str,
    args: fn(&Path) -> Vec<String>,
}

const ARTIFACTS: [Artifact; 3] = [
    Artifact {
        dir: "generated/rust/agency",
        args: |out| {
            let mut args = strings(&["generate", "synthesize", "--path", SPECIFICATION]);
            args.extend(strings(&["--target", "rust", "--layout", "crate", "--out"]));
            args.push(out.display().to_string());
            args
        },
    },
    Artifact {
        dir: "generated/ts",
        args: |out| {
            let mut args = strings(&["generate", "types", "--path", SPECIFICATION]);
            args.extend(strings(&[
                "--all-types",
                "--all-events",
                "--all-commands",
                "--all-views",
            ]));
            args.extend(strings(&["--target", "typescript", "--out"]));
            args.push(out.display().to_string());
            args
        },
    },
    Artifact {
        dir: "generated/conformance",
        args: |out| {
            let mut args = strings(&["verify", "conform", "synthesize", "--path", SPECIFICATION]);
            args.extend(strings(&["--scenarios", SPECIFICATION, "--out"]));
            args.push(out.join("suite.json").display().to_string());
            args
        },
    },
];

#[derive(Parser)]
#[command(about = "Repository checks for agency")]
struct Cli {
    #[command(subcommand)]
    command: Task,
}

#[derive(Subcommand)]
enum Task {
    /// Rewrite the synthesized crate, the TypeScript types and the conformance suite from ess/.
    Generate,
    /// Fail when any of them differs from a fresh generation from ess/.
    Drift,
    /// Run the committed conformance suite against a target and hold its counts to
    /// `conformance/<target>.baseline.json`.
    Conformance {
        /// The `ess verify conform run` target.
        #[arg(long, default_value = "interpreted")]
        target: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let root = repository_root();
    let result = match cli.command {
        Task::Generate => ARTIFACTS.iter().try_for_each(|a| generate(&root, a)),
        Task::Drift => drift(&root),
        Task::Conformance { target } => conformance::run(&root, &scratch(&root), &target),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/agency-xtask sits two levels below the repository root")
        .to_path_buf()
}

fn generate(root: &Path, artifact: &Artifact) -> Result<(), String> {
    let dir = root.join(artifact.dir);
    let lock = dir.join("Cargo.lock");
    let held = fs::read(&lock).ok();
    run(root, artifact, &dir)?;
    match held {
        Some(bytes) => {
            fs::write(&lock, bytes).map_err(|e| format!("write {}: {e}", lock.display()))
        }
        None => Ok(()),
    }
}

fn run(root: &Path, artifact: &Artifact, out: &Path) -> Result<(), String> {
    if out.exists() {
        fs::remove_dir_all(out).map_err(|e| format!("remove {}: {e}", out.display()))?;
    }
    fs::create_dir_all(out).map_err(|e| format!("create {}: {e}", out.display()))?;
    let status = Command::new("ess")
        .current_dir(root)
        .args((artifact.args)(out))
        .status()
        .map_err(|e| format!("run ess: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("ess for {} exited with {status}", artifact.dir))
    }
}

fn scratch(root: &Path) -> PathBuf {
    std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target"))
        .join("agency-xtask")
}

fn drift(root: &Path) -> Result<(), String> {
    let scratch = scratch(root).join("drift");
    let mut differences = Vec::new();
    for artifact in &ARTIFACTS {
        let fresh_dir = scratch.join(artifact.dir);
        run(root, artifact, &fresh_dir)?;
        let committed = root.join(artifact.dir);
        let fresh = files(&fresh_dir)?;
        let held = files(&committed)?;
        for path in fresh.union(&held) {
            let shown = Path::new(artifact.dir).join(path);
            match (fresh.contains(path), held.contains(path)) {
                (true, false) => differences.push(format!("missing: {}", shown.display())),
                (false, true) => differences.push(format!("not generated: {}", shown.display())),
                _ => {
                    let a = fs::read(fresh_dir.join(path)).map_err(|e| e.to_string())?;
                    let b = fs::read(committed.join(path)).map_err(|e| e.to_string())?;
                    if a != b {
                        differences.push(format!("edited: {}", shown.display()));
                    }
                }
            }
        }
    }
    if differences.is_empty() {
        println!("generated/: no drift from {SPECIFICATION}/");
        Ok(())
    } else {
        Err(format!(
            "generated/ drifted from {SPECIFICATION}/ (run `task generate`):\n  {}",
            differences.join("\n  ")
        ))
    }
}

fn files(dir: &Path) -> Result<BTreeSet<PathBuf>, String> {
    let mut found = BTreeSet::new();
    if !dir.exists() {
        return Ok(found);
    }
    let mut pending = vec![dir.to_path_buf()];
    while let Some(current) = pending.pop() {
        let entries =
            fs::read_dir(&current).map_err(|e| format!("read {}: {e}", current.display()))?;
        for entry in entries {
            let path = entry.map_err(|e| e.to_string())?.path();
            let relative = path.strip_prefix(dir).map_err(|e| e.to_string())?;
            if IGNORED.iter().any(|ignored| relative == Path::new(ignored)) {
                continue;
            }
            if path.is_dir() {
                pending.push(path);
            } else {
                found.insert(relative.to_path_buf());
            }
        }
    }
    Ok(found)
}
