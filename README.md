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
planwarden schema review roadmap
planwarden review roadmap --input findings.json
planwarden create roadmap --input review.json
planwarden next plans/roadmaps/my-plan.md --format text
planwarden approve plans/roadmaps/my-plan.md
planwarden start plans/roadmaps/my-plan.md
planwarden set-status plans/roadmaps/my-plan.md R1 in-progress
planwarden complete plans/roadmaps/my-plan.md
```

## Core Flow

1. The agent investigates the repo and request.
2. The agent asks `planwarden schema review roadmap|task` what structured input it needs.
3. The agent sends findings to `planwarden review roadmap|task`.
4. `planwarden` returns `decision`, `missing`, `questions`, `pushback`, and `normalized_plan`.
5. Once the result is good enough, the agent writes the plan with `planwarden create`.
6. The agent only shows the next chunk with `planwarden next`.
7. The plan moves through `draft -> approved -> in_progress -> done`.

## Commands

- `planwarden schema review roadmap|task`
- `planwarden review roadmap|task`
- `planwarden create roadmap|task`
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
