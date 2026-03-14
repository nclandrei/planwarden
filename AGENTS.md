# AGENTS.md

## Purpose

Use `planwarden` as a schema-first planning workflow for AI agents. The agent investigates and understands the work; `planwarden` enforces the planning contract and writes the durable plan file.

## Required Flow

1. Investigate the repo and the user request first.
2. Run `planwarden schema review plan|task`.
3. Build structured findings JSON from verified facts, constraints, unknowns, signals, slices, and concerns.
4. Run `planwarden review plan|task`.
5. If the decision is `blocked` or `needs_input`, resolve `missing`, `questions`, and `pushback` before continuing.
6. Run `planwarden create plan|task` once review is ready.
7. Do not paste the full plan file into chat unless the user explicitly asks for it.
8. Immediately run `planwarden next <plan-file> --format text` and show only that chunk in chat.
9. Keep lifecycle and checklist state accurate with `approve`, `start`, `set-status`, and `complete`.

## Planning Contract

- The plan file is the source of truth; chat stays small and chunked.
- The agent decides whether a concern is applicable and must justify `applicable = false`.
- `planwarden` enforces missing coverage, inconsistent concern waivers, oversized slices, and weak plans.
- Push back on weak rollback, decoupling, testing, authentication, authorization, security, or data-exposure handling.
- Bugfix work must prove red before green.
