# planwarden

`planwarden` is a CLI planning enforcer for AI agents. The agent explores the repo and gathers facts; `planwarden` validates the planning contract, pushes back on weak slices, writes a durable plan file, and only surfaces the next chunk instead of a wall of text.

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

# 4. Show only the current chunk in chat.
planwarden next plans/my-plan.md --format text

# 5. Move the plan through its lifecycle.
planwarden approve plans/my-plan.md
planwarden start plans/my-plan.md
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
7. The agent immediately runs `planwarden next <plan-file> --format text` and shows only that chunk in chat unless the user explicitly asks for the full plan.
8. The plan moves through `draft -> approved -> in_progress -> done`.

## Working Contract

- The plan file is the source of truth; chat should stay chunked.
- The agent decides whether a concern applies and must justify `applicable = false`.
- `planwarden` enforces consistency, slice size, and required coverage.
- Bugfix work must prove red before green.

## Commands

- `planwarden schema review plan|task`
- `planwarden review plan|task`
- `planwarden create plan|task`
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
