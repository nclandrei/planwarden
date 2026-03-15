use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use planwarden::plan_file::{
    advance_review, approve_plan, complete_plan, extract_plan_from_json, next_chunk,
    render_next_chunk_text, render_review_next_text, review_next, set_status, start_plan,
    write_plan_file,
};
use planwarden::review::{PlanItemStatus, PlanKind, ReviewRequest, review_request};
use planwarden::schema::{render_review_schema_text, review_schema};

const REVIEW_PLAN_AFTER_HELP: &str =
    "Run `planwarden schema review plan` to inspect the contract before building the JSON payload.";
const REVIEW_TASK_AFTER_HELP: &str =
    "Run `planwarden schema review task` to inspect the contract before building the JSON payload.";
const CREATE_AFTER_HELP: &str = "Input can be either the full `review` response JSON or only the `normalized_plan` object. Review response envelopes must have decision `ready`. After create, run `planwarden review-next <plan-file> --format text`, present only the current section, ask the user for approval or concerns, and only then run `planwarden advance-review <plan-file>`. Do not dump the full plan into chat.";
const CLI_AFTER_HELP: &str = "Agent flow:\n  1. Investigate the repo and request first.\n  2. Run `planwarden schema review plan|task`.\n  3. Run `planwarden review plan|task` with structured findings.\n  4. Resolve any `missing`, `questions`, and `pushback` before proceeding.\n  5. Run `planwarden create plan|task`.\n  6. Review the draft one section at a time with `planwarden review-next <plan-file> --format text`. Present only that section, ask the user for approval or concerns, discuss or revise if needed, and only then run `planwarden advance-review <plan-file>`. Do not dump the full plan while reviewing.\n  7. Approve and start the plan, then use `planwarden next <plan-file> --format text` for execution chunks.";

#[derive(Debug, Parser)]
#[command(name = "planwarden")]
#[command(
    about = "A planning enforcer for AI agents.",
    long_about = "A planning enforcer for AI agents. Investigate first, ask `schema` for the contract, send structured findings to `review`, write the durable plan with `create`, and show only the current chunk with `next`.",
    after_help = CLI_AFTER_HELP
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Validate planning input and return decision/missing/questions/pushback.")]
    Review {
        #[command(subcommand)]
        kind: ReviewCommand,
    },
    #[command(
        about = "Show the review contract so an agent knows what JSON to send after investigating."
    )]
    Schema {
        #[command(subcommand)]
        kind: SchemaCommand,
    },
    #[command(about = "Write a durable markdown plan file from normalized review output.")]
    Create {
        #[command(subcommand)]
        kind: CreateCommand,
    },
    #[command(
        about = "Show the next review section and structured approval metadata for a draft or approved plan."
    )]
    ReviewNext(ReviewNextArgs),
    #[command(about = "Mark the current review section as complete.")]
    AdvanceReview(PlanFileArgs),
    #[command(about = "Show only the current plan chunk instead of the whole plan file.")]
    Next(NextArgs),
    #[command(about = "Update one checklist item to todo, in_progress, or done.")]
    SetStatus(SetStatusArgs),
    #[command(about = "Mark a draft plan as approved.")]
    Approve(PlanFileArgs),
    #[command(about = "Move an approved plan into execution.")]
    Start(PlanFileArgs),
    #[command(about = "Mark an in-progress plan as done once every item is complete.")]
    Complete(PlanFileArgs),
}

#[derive(Debug, Subcommand)]
enum ReviewCommand {
    #[command(about = "Validate a multi-slice plan request.")]
    #[command(after_long_help = REVIEW_PLAN_AFTER_HELP)]
    Plan(InputArgs),
    #[command(about = "Validate a single execution-slice task request.")]
    #[command(after_long_help = REVIEW_TASK_AFTER_HELP)]
    Task(InputArgs),
}

#[derive(Debug, Subcommand)]
enum SchemaCommand {
    #[command(about = "Show the input contract for `planwarden review`.")]
    Review {
        #[command(subcommand)]
        kind: SchemaReviewCommand,
    },
}

#[derive(Debug, Subcommand)]
enum SchemaReviewCommand {
    #[command(about = "Show the plan review contract.")]
    Plan(SchemaArgs),
    #[command(about = "Show the task review contract.")]
    Task(SchemaArgs),
}

#[derive(Debug, Subcommand)]
enum CreateCommand {
    #[command(about = "Write a plan markdown file from review output.")]
    #[command(after_long_help = CREATE_AFTER_HELP)]
    Plan(CreateArgs),
    #[command(about = "Write a task markdown file from review output.")]
    #[command(after_long_help = CREATE_AFTER_HELP)]
    Task(CreateArgs),
}

#[derive(Debug, Args)]
struct InputArgs {
    /// Read review request JSON from a file instead of stdin.
    #[arg(long, short)]
    input: Option<PathBuf>,
    /// Emit compact JSON instead of pretty-printed JSON.
    #[arg(long)]
    compact: bool,
}

#[derive(Debug, Args)]
struct CreateArgs {
    /// Read review response JSON or normalized plan JSON from a file instead of stdin.
    #[arg(long, short)]
    input: Option<PathBuf>,
    /// Write the markdown plan to this path instead of the default plans/ path.
    #[arg(long, short)]
    output: Option<PathBuf>,
    /// Emit compact JSON instead of pretty-printed JSON.
    #[arg(long)]
    compact: bool,
}

#[derive(Debug, Args)]
struct SchemaArgs {
    /// Choose human-readable text or machine-readable JSON output.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}

#[derive(Debug, Args)]
struct NextArgs {
    /// Path to a markdown plan file created by Planwarden.
    plan_file: PathBuf,
    /// Maximum number of incomplete items to return.
    #[arg(long, default_value_t = 3)]
    limit: usize,
    /// Choose human-readable text or machine-readable JSON output.
    #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
    format: OutputFormat,
    /// Emit compact JSON instead of pretty-printed JSON.
    #[arg(long)]
    compact: bool,
}

#[derive(Debug, Args)]
struct ReviewNextArgs {
    /// Path to a markdown plan file created by Planwarden.
    plan_file: PathBuf,
    /// Maximum number of remaining review sections to preview.
    #[arg(long, default_value_t = 3)]
    limit: usize,
    /// Choose human-readable text or machine-readable JSON output.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
    /// Emit compact JSON instead of pretty-printed JSON.
    #[arg(long)]
    compact: bool,
}

#[derive(Debug, Args)]
struct SetStatusArgs {
    /// Path to a markdown plan file created by Planwarden.
    plan_file: PathBuf,
    /// The checklist item ID to update, such as P1 or T2.
    item_id: String,
    #[arg(value_enum)]
    status: CliStatus,
    /// Emit compact JSON instead of pretty-printed JSON.
    #[arg(long)]
    compact: bool,
}

#[derive(Debug, Args)]
struct PlanFileArgs {
    /// Path to a markdown plan file created by Planwarden.
    plan_file: PathBuf,
    /// Emit compact JSON instead of pretty-printed JSON.
    #[arg(long)]
    compact: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum CliStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
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
                ReviewCommand::Plan(args) => (PlanKind::Plan, args),
                ReviewCommand::Task(args) => (PlanKind::Task, args),
            };
            let input = read_input(args.input)?;
            let request: ReviewRequest =
                serde_json::from_str(&input).context("failed to parse review request JSON")?;
            let response = review_request(plan_kind, request);
            print_json(&response, args.compact)?;
        }
        Command::Schema { kind } => match kind {
            SchemaCommand::Review { kind } => {
                let (plan_kind, args) = match kind {
                    SchemaReviewCommand::Plan(args) => (PlanKind::Plan, args),
                    SchemaReviewCommand::Task(args) => (PlanKind::Task, args),
                };
                let schema = review_schema(plan_kind);
                match args.format {
                    OutputFormat::Text => println!("{}", render_review_schema_text(&schema)),
                    OutputFormat::Json => print_json(&schema, false)?,
                }
            }
        },
        Command::Create { kind } => {
            let (expected_kind, args) = match kind {
                CreateCommand::Plan(args) => ("plan", args),
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
        Command::ReviewNext(args) => {
            let response = review_next(&args.plan_file, args.limit)?;
            match args.format {
                OutputFormat::Text => println!("{}", render_review_next_text(&response)),
                OutputFormat::Json => print_json(&response, args.compact)?,
            }
        }
        Command::AdvanceReview(args) => {
            let response = advance_review(&args.plan_file)?;
            print_json(&response, args.compact)?;
        }
        Command::Next(args) => {
            let response = next_chunk(&args.plan_file, args.limit)?;
            match args.format {
                OutputFormat::Text => println!("{}", render_next_chunk_text(&response)),
                OutputFormat::Json => print_json(&response, args.compact)?,
            }
        }
        Command::SetStatus(args) => {
            let response = set_status(&args.plan_file, &args.item_id, args.status.into())?;
            print_json(&response, args.compact)?;
        }
        Command::Approve(args) => {
            let response = approve_plan(&args.plan_file)?;
            print_json(&response, args.compact)?;
        }
        Command::Start(args) => {
            let response = start_plan(&args.plan_file)?;
            print_json(&response, args.compact)?;
        }
        Command::Complete(args) => {
            let response = complete_plan(&args.plan_file)?;
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
