# planwarden

`planwarden` is a CLI planning enforcer for AI agents. The agent explores the repo and gathers facts; `planwarden` validates the planning contract, pushes back on weak slices, writes a durable plan file, walks the user through review one section at a time, and only surfaces execution chunks instead of a wall of text.

## Install

```bash
# Homebrew (recommended)
brew install nclandrei/tap/planwarden

# crates.io
cargo install planwarden --locked
```

## Quick Start

```bash
# 1. Investigate first, then ask for the contract.
planwarden schema review plan

# 2. Send structured findings to review.
planwarden review plan --input findings.json

# 3. Write the full plan file once review is ready.
planwarden create plan --input review.json

# 4. Review the draft section by section in chat.
planwarden review-next plans/my-plan.md --format text
planwarden advance-review plans/my-plan.md

# 5. Approve, start, and execute the plan.
planwarden approve plans/my-plan.md
planwarden start plans/my-plan.md
planwarden next plans/my-plan.md --format text
planwarden set-status plans/my-plan.md P1 in-progress
planwarden complete plans/my-plan.md
```

## Agent Workflow

1. The agent investigates the repo and request.
2. The agent runs `planwarden schema review plan|task` to see the contract.
3. The agent sends structured findings to `planwarden review plan|task`.
4. `planwarden` returns `decision`, `missing`, `questions`, `pushback`, and `normalized_plan`.
5. The agent resolves any gaps before creating the plan file.
6. The agent writes the full plan with `planwarden create`.
7. The agent immediately runs `planwarden review-next <plan-file> --format text`, shows only that section in chat, asks the user to approve it or raise concerns, and only then advances review.
8. The agent discusses or revises the plan if the user raises concerns, keeps review section-by-section without dumping the whole plan, and repeats until every section is done.
9. The agent approves and starts the plan only after review is complete, then uses `planwarden next <plan-file> --format text` for execution chunks.
10. The plan moves through `draft -> approved -> in_progress -> done`.

## Working Contract

- The plan file is the source of truth; chat should stay chunked.
- During review, present only the current section, ask the user for approval or concerns, and do not dump or summarize the full plan.
- The agent decides whether a concern applies and must justify `applicable = false`.
- `planwarden` enforces consistency, slice size, and required coverage.
- Bugfix work must prove red before green.

## Commands

- `planwarden schema review plan|task`
- `planwarden review plan|task`
- `planwarden create plan|task`
- `planwarden review-next <plan-file>`
- `planwarden advance-review <plan-file>`
- `planwarden next <plan-file>`
- `planwarden approve <plan-file>`
- `planwarden start <plan-file>`
- `planwarden set-status <plan-file> <item-id> <todo|in-progress|done>`
- `planwarden complete <plan-file>`

## Release Automation

Pushing a new version to `main` can be automated the same way as `distill`:

- build release artifacts with `cargo-dist`
- publish the crate to crates.io
- update the Homebrew formula in `nclandrei/homebrew-tap`
- publish the GitHub release

Required GitHub Actions secrets:

- `CARGO_REGISTRY_TOKEN`: crates.io publish token for `planwarden`
- `HOMEBREW_TAP_TOKEN`: GitHub token with push access to `nclandrei/homebrew-tap`

## License

MIT
