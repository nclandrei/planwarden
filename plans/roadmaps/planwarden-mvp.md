# Planwarden MVP

## Purpose

Planwarden is a CLI planning enforcer for AI agents.

- The agent explores the repo, understands the request, and gathers facts.
- Planwarden does not "detect" the whole world on its own.
- Planwarden validates the agent's input, forces missing concerns to be addressed, and writes durable plan files.

## Core Contract

- The agent submits structured findings to `planwarden review`.
- Planwarden returns structured feedback:
  - `decision`: `blocked | needs_input | ready`
  - `missing`: required coverage that is absent
  - `questions`: targeted questions the agent must ask next
  - `pushback`: objections when the proposed plan is weak or unsafe
  - `normalized_plan`: cleaned plan data when input is good enough
- The agent resolves open issues and only then calls `planwarden create`.

## Applicability Rule

- The agent decides whether a concern applies.
- The agent must justify non-applicability.
- Planwarden checks the justification for consistency against the rest of the input.

Examples:

- `security_review.applicable = false`
- `security_review.reason = "Pure local refactor with no data-flow or boundary changes"`

## MVP Scope

- [x] Seed the repository with the initial roadmap and decision record.
- [x] Define the JSON contract for `review roadmap` and `review task`.
- [x] Implement `planwarden review` with rule-based enforcement and machine-readable output.
- [x] Implement `planwarden create` to write roadmap and task plan files from normalized input.
- [x] Implement `planwarden next` to show only the next chunk instead of the whole plan.
- [x] Implement `planwarden set-status` for `todo`, `in_progress`, and `done`.
- [x] Add initial rules for rollback, tests, security, auth, authz, decoupling, and unresolved unknowns.
- [x] Add bugfix enforcement: no final plan without explicit red-proof metadata.
- [x] Add tests for JSON parsing, rule evaluation, and markdown rendering.
- [ ] Package the CLI for personal Homebrew install.

## First Command Set

- `planwarden review roadmap`
- `planwarden review task`
- `planwarden create roadmap`
- `planwarden create task`
- `planwarden next <plan-file>`
- `planwarden set-status <plan-file> <item-id> <todo|in_progress|done>`

## Initial Input Shape

The agent should pass structured findings, not free-form prose.

- goal
- facts
- constraints
- unknowns
- risks
- proposed_slices
- rollback
- tests
- security
- auth
- authz
- bugfix_red_proof

## Rule Philosophy

- `missing` means required coverage is absent.
- `questions` means there is a real unresolved decision.
- `pushback` means the plan is present but weak, unsafe, oversized, or poorly scoped.

## Non-Goals For V1

- No automatic context clearing or session spawning.
- No repo-wide intelligence engine inside the CLI.
- No hook integration on day one.
- No `showboat` integration on day one.
- No UI beyond stdout and plan files.

## Default Implementation Choice

- Start with Rust for the MVP.
- Revisit Zig only if there is a concrete distribution or binary-size reason later.

## Handoff Pattern

When a plan is ready, the agent should recommend:

1. Start a fresh session or thread.
2. Point the next agent at this plan file.
3. Execute only the next unchecked item.
4. Update the file as work progresses.
