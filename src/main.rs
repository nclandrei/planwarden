use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use planwarden::plan_file::{extract_plan_from_json, next_chunk, set_status, write_plan_file};
use planwarden::review::{PlanItemStatus, PlanKind, ReviewRequest, review_request};

#[derive(Debug, Parser)]
#[command(name = "planwarden")]
#[command(about = "A planning enforcer for AI agents.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Review {
        #[command(subcommand)]
        kind: ReviewCommand,
    },
    Create {
        #[command(subcommand)]
        kind: CreateCommand,
    },
    Next(NextArgs),
    SetStatus(SetStatusArgs),
}

#[derive(Debug, Subcommand)]
enum ReviewCommand {
    Roadmap(InputArgs),
    Task(InputArgs),
}

#[derive(Debug, Subcommand)]
enum CreateCommand {
    Roadmap(CreateArgs),
    Task(CreateArgs),
}

#[derive(Debug, Args)]
struct InputArgs {
    #[arg(long, short)]
    input: Option<PathBuf>,
    #[arg(long)]
    compact: bool,
}

#[derive(Debug, Args)]
struct CreateArgs {
    #[arg(long, short)]
    input: Option<PathBuf>,
    #[arg(long, short)]
    output: Option<PathBuf>,
    #[arg(long)]
    compact: bool,
}

#[derive(Debug, Args)]
struct NextArgs {
    plan_file: PathBuf,
    #[arg(long, default_value_t = 3)]
    limit: usize,
    #[arg(long)]
    compact: bool,
}

#[derive(Debug, Args)]
struct SetStatusArgs {
    plan_file: PathBuf,
    item_id: String,
    #[arg(value_enum)]
    status: CliStatus,
    #[arg(long)]
    compact: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum CliStatus {
    Todo,
    InProgress,
    Done,
}

impl From<CliStatus> for PlanItemStatus {
    fn from(value: CliStatus) -> Self {
        match value {
            CliStatus::Todo => Self::Todo,
            CliStatus::InProgress => Self::InProgress,
            CliStatus::Done => Self::Done,
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Review { kind } => {
            let (plan_kind, args) = match kind {
                ReviewCommand::Roadmap(args) => (PlanKind::Roadmap, args),
                ReviewCommand::Task(args) => (PlanKind::Task, args),
            };
            let input = read_input(args.input)?;
            let request: ReviewRequest =
                serde_json::from_str(&input).context("failed to parse review request JSON")?;
            let response = review_request(plan_kind, request);
            print_json(&response, args.compact)?;
        }
        Command::Create { kind } => {
            let (expected_kind, args) = match kind {
                CreateCommand::Roadmap(args) => ("roadmap", args),
                CreateCommand::Task(args) => ("task", args),
            };
            let input = read_input(args.input)?;
            let plan = extract_plan_from_json(&input)?;
            if plan.kind.label() != expected_kind {
                anyhow::bail!(
                    "plan kind mismatch: create {expected_kind} received {}",
                    plan.kind.label()
                );
            }
            let response = write_plan_file(&plan, args.output.as_deref())?;
            print_json(&response, args.compact)?;
        }
        Command::Next(args) => {
            let response = next_chunk(&args.plan_file, args.limit)?;
            print_json(&response, args.compact)?;
        }
        Command::SetStatus(args) => {
            let response = set_status(&args.plan_file, &args.item_id, args.status.into())?;
            print_json(&response, args.compact)?;
        }
    }

    Ok(())
}

fn read_input(path: Option<PathBuf>) -> Result<String> {
    match path {
        Some(path) => fs::read_to_string(&path)
            .with_context(|| format!("failed to read input from {}", path.display())),
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("failed to read JSON input from stdin")?;
            Ok(buffer)
        }
    }
}

fn print_json<T: serde::Serialize>(value: &T, compact: bool) -> Result<()> {
    let output = if compact {
        serde_json::to_string(value)?
    } else {
        serde_json::to_string_pretty(value)?
    };
    println!("{output}");
    Ok(())
}
