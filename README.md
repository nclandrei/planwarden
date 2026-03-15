# planwarden

[![Crates.io](https://img.shields.io/crates/v/planwarden.svg)](https://crates.io/crates/planwarden)
[![Releases](https://img.shields.io/github/v/release/nclandrei/planwarden?label=releases)](https://github.com/nclandrei/planwarden/releases)
[![Tests](https://github.com/nclandrei/planwarden/actions/workflows/ci.yml/badge.svg)](https://github.com/nclandrei/planwarden/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/nclandrei/planwarden/blob/main/LICENSE)

Make AI agents investigate first, plan against a contract, and review work one section at a time.

`planwarden` is a CLI for AI agents. A human points an agent at a repo and a request; the agent uses `planwarden` to turn verified findings into a durable markdown plan file, walk review section by section, and execute from the file instead of improvising in chat.

## Example

A human asks an agent to make a change. The agent can use `planwarden` like this:

```bash
# Inspect the contract after investigating the repo.
planwarden schema review task

# Validate a structured findings payload.
planwarden review task --input findings.json > review.json

# Write the durable task file.
planwarden create task --input review.json

# Show only the next section for human review.
planwarden review-next <task-file> --format text
planwarden advance-review <task-file>

# Once review is complete, execute from the task file.
planwarden approve <task-file>
planwarden start <task-file>
planwarden next <task-file> --format text
planwarden set-status <task-file> T1 in-progress
planwarden set-status <task-file> T1 done
planwarden complete <task-file>
```

This flow keeps the agent honest: investigate first, ask for the schema, send structured findings, write the durable file, review it in chunks, then execute from that file.

## Installation

Install with Homebrew:

```bash
brew install nclandrei/tap/planwarden
```

Or install from crates.io:

```bash
cargo install planwarden --locked
```

Prebuilt binaries are available on the [releases page](https://github.com/nclandrei/planwarden/releases).

## Help

```text
A planning enforcer for AI agents. Investigate first, ask `schema` for the contract, send structured findings to `review`, write the durable plan with `create`, and show only the current chunk with `next`.

Usage: planwarden <COMMAND>

Commands:
  review          Validate planning input and return decision/missing/questions/pushback.
  schema          Show the review contract so an agent knows what JSON to send after investigating.
  create          Write a durable markdown plan file from normalized review output.
  review-next     Show the next review section and structured approval metadata for a draft or approved plan.
  advance-review  Mark the current review section as complete.
  next            Show only the current plan chunk instead of the whole plan file.
  set-status      Update one checklist item to todo, in_progress, or done.
  approve         Mark a draft plan as approved.
  start           Move an approved plan into execution.
  complete        Mark an in-progress plan as done once every item is complete.
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

Agent flow:
  1. Investigate the repo and request first.
  2. Run `planwarden schema review plan|task`.
  3. Run `planwarden review plan|task` with structured findings.
  4. Resolve any `missing`, `questions`, and `pushback` before proceeding.
  5. Run `planwarden create plan|task`.
  6. Review the draft one section at a time with `planwarden review-next <plan-file> --format text`. Present only that section, ask the user for approval or concerns, discuss or revise if needed, and only then run `planwarden advance-review <plan-file>`. Do not dump the full plan while reviewing.
  7. Approve and start the plan, then use `planwarden next <plan-file> --format text` for execution chunks.
```

## Review Contract

`planwarden review plan` and `planwarden review task` accept structured JSON, not free-form planning prose. Ask for the exact contract first:

```bash
planwarden schema review plan
planwarden schema review task
```

The repository includes an example payload at [`examples/review-plan.json`](examples/review-plan.json).

`create` writes markdown files under `plans/` or `plans/tasks/`. Those files become the source of truth for review and execution.

## What It Enforces

- The agent investigates before it plans.
- The plan file is the source of truth; chat stays chunked.
- During review, the agent shows only the current section and asks for approval or concerns.
- Bugfix work must prove red before green.
- `review` pushes back on oversized slices, missing coverage, and inconsistent concern waivers.

## Host Integrations

`planwarden review-next <plan-file> --format json` returns structured approval metadata for hosts. That response includes an `approval` block with the prompt, response options, and the command to advance review, so hosts do not need to scrape the text renderer.

## License

MIT
