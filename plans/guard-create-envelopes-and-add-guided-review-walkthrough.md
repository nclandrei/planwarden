# Guard create envelopes and add guided review walkthrough

<!-- planwarden:data:start -->
{
  "kind": "plan",
  "plan_status": "done",
  "title": "Guard create envelopes and add guided review walkthrough",
  "goal": "Require review envelopes to be ready before create writes a plan, and add a durable section-by-section review flow that agents must use before approval.",
  "facts": [
    "`create` currently accepts any review envelope that contains `normalized_plan` because `extract_plan_from_json` ignores the review decision and `create` only checks the normalized plan kind.",
    "`review_request` returns `needs_input` whenever there are unresolved questions or missing fields, but `create` still writes a draft plan from that envelope.",
    "Plan files render the embedded JSON block before the human-readable markdown sections, so a user who is only told the file path never sees the reviewable content in chat.",
    "`next` currently renders execution checklist chunks and open questions; there is no dedicated pre-approval walkthrough for Goal, Facts, Constraints, Risks, Concerns, and Checklist sections.",
    "`approve` currently enforces only lifecycle status, not whether the plan has been reviewed section by section."
  ],
  "constraints": [
    "Keep the markdown plan file as the durable source of truth.",
    "Preserve support for advanced/manual workflows that pass a bare normalized plan document directly to `create`.",
    "Do not break the existing execution `next` flow after a plan has been approved and started.",
    "Older plan files should continue to load with sensible defaults for any new review-tracking state."
  ],
  "acceptance_criteria": [
    "`planwarden create plan|task` rejects review response envelopes unless their decision is `ready`, while still accepting a bare normalized plan document.",
    "There is a durable CLI walkthrough for draft-plan review that surfaces one human section at a time and tracks progress in the plan file.",
    "A plan cannot be approved until its review walkthrough is complete, so agents cannot skip straight from file creation to approval.",
    "CLI help, README, AGENTS guidance, and automated tests cover the ready-only create flow and the new review walkthrough."
  ],
  "risks": [
    "If the review walkthrough is bolted onto `next`, execution-oriented consumers could break; the pre-approval flow should stay explicit.",
    "Persisting review state in plan files must remain backward-compatible so existing plans still parse.",
    "Approval gating could feel too rigid unless the CLI clearly tells the agent which review command to run next."
  ],
  "concerns": {
    "rollback": {
      "applicable": true,
      "reason": null,
      "approach": "Keep envelope validation and review-walkthrough logic isolated so the repo can revert to the previous create/approve behavior without rewriting plan files."
    },
    "security": {
      "applicable": false,
      "reason": "This work changes local CLI validation and markdown rendering only; it does not touch trust boundaries or sensitive data handling.",
      "approach": null
    },
    "authentication": {
      "applicable": false,
      "reason": "No login, session, or identity flows are involved.",
      "approach": null
    },
    "authorization": {
      "applicable": false,
      "reason": "No roles, permissions, or access-control paths are involved.",
      "approach": null
    },
    "decoupling": {
      "applicable": true,
      "reason": null,
      "approach": "Represent review-envelope parsing and draft-review state in dedicated helpers so create, review walkthrough, and execution chunk logic can evolve independently."
    },
    "tests": {
      "unit": {
        "applicable": true,
        "reason": null,
        "approach": "Add focused coverage for envelope parsing and review-section selection/defaulting."
      },
      "integration": {
        "applicable": true,
        "reason": null,
        "approach": "Exercise the CLI lifecycle from review/create through review walkthrough and approval gating."
      },
      "regression": {
        "applicable": true,
        "reason": null,
        "approach": "Capture the current bug where `needs_input` envelopes still create plan files, then keep that path failing after the fix."
      },
      "smoke": {
        "applicable": true,
        "reason": null,
        "approach": "Run the Rust test suite covering CLI help and lifecycle behavior after the flow change."
      }
    },
    "bugfix_red": {
      "applicable": true,
      "reason": null,
      "approach": "Start with a failing test that reproduces `create` accepting a `needs_input` envelope before tightening the create path."
    }
  },
  "open_questions": [],
  "review_state": {
    "completed_sections": []
  },
  "items": [
    {
      "id": "P1",
      "status": "done",
      "title": "Reject non-ready review envelopes",
      "summary": "Tighten create-time parsing so review envelopes must carry a `ready` decision, add red-first coverage for the current gap, and keep bare normalized-plan input working for advanced use.",
      "estimated_minutes": 45,
      "dependencies": [],
      "acceptance_criteria": [
        "Create fails with a clear error when given a `blocked` or `needs_input` review response envelope.",
        "Create still accepts a bare normalized plan document and a `ready` review envelope."
      ]
    },
    {
      "id": "P2",
      "status": "done",
      "title": "Add guided draft review",
      "summary": "Persist lightweight review progress in plan files, add CLI commands that show the next review section and advance review state, and require review completion before approval.",
      "estimated_minutes": 90,
      "dependencies": [
        "Reject non-ready review envelopes"
      ],
      "acceptance_criteria": [
        "Draft plans expose the next review section in a dedicated walkthrough command and track section completion durably.",
        "Approve fails with a clear next step until all review sections have been completed."
      ]
    },
    {
      "id": "P3",
      "status": "done",
      "title": "Document and verify the new flow",
      "summary": "Update agent-facing docs and CLI coverage so the documented flow is create, review section-by-section, approve, then execute with next/set-status.",
      "estimated_minutes": 60,
      "dependencies": [
        "Add guided draft review"
      ],
      "acceptance_criteria": [
        "README, AGENTS.md, and help text describe the review walkthrough before approval.",
        "Tests cover both the create-time decision guard and the section-by-section review lifecycle."
      ]
    }
  ]
}
<!-- planwarden:data:end -->

## Goal

Require review envelopes to be ready before create writes a plan, and add a durable section-by-section review flow that agents must use before approval.

## Plan Status

- done

## Facts

- `create` currently accepts any review envelope that contains `normalized_plan` because `extract_plan_from_json` ignores the review decision and `create` only checks the normalized plan kind.
- `review_request` returns `needs_input` whenever there are unresolved questions or missing fields, but `create` still writes a draft plan from that envelope.
- Plan files render the embedded JSON block before the human-readable markdown sections, so a user who is only told the file path never sees the reviewable content in chat.
- `next` currently renders execution checklist chunks and open questions; there is no dedicated pre-approval walkthrough for Goal, Facts, Constraints, Risks, Concerns, and Checklist sections.
- `approve` currently enforces only lifecycle status, not whether the plan has been reviewed section by section.

## Constraints

- Keep the markdown plan file as the durable source of truth.
- Preserve support for advanced/manual workflows that pass a bare normalized plan document directly to `create`.
- Do not break the existing execution `next` flow after a plan has been approved and started.
- Older plan files should continue to load with sensible defaults for any new review-tracking state.

## Acceptance Criteria

- `planwarden create plan|task` rejects review response envelopes unless their decision is `ready`, while still accepting a bare normalized plan document.
- There is a durable CLI walkthrough for draft-plan review that surfaces one human section at a time and tracks progress in the plan file.
- A plan cannot be approved until its review walkthrough is complete, so agents cannot skip straight from file creation to approval.
- CLI help, README, AGENTS guidance, and automated tests cover the ready-only create flow and the new review walkthrough.

## Risks

- If the review walkthrough is bolted onto `next`, execution-oriented consumers could break; the pre-approval flow should stay explicit.
- Persisting review state in plan files must remain backward-compatible so existing plans still parse.
- Approval gating could feel too rigid unless the CLI clearly tells the agent which review command to run next.

## Open Questions

- none

## Concerns

- Rollback: applicable. Keep envelope validation and review-walkthrough logic isolated so the repo can revert to the previous create/approve behavior without rewriting plan files.
- Security: not applicable. This work changes local CLI validation and markdown rendering only; it does not touch trust boundaries or sensitive data handling.
- Authentication: not applicable. No login, session, or identity flows are involved.
- Authorization: not applicable. No roles, permissions, or access-control paths are involved.
- Decoupling: applicable. Represent review-envelope parsing and draft-review state in dedicated helpers so create, review walkthrough, and execution chunk logic can evolve independently.
- Unit Tests: applicable. Add focused coverage for envelope parsing and review-section selection/defaulting.
- Integration Tests: applicable. Exercise the CLI lifecycle from review/create through review walkthrough and approval gating.
- Regression Tests: applicable. Capture the current bug where `needs_input` envelopes still create plan files, then keep that path failing after the fix.
- Smoke Tests: applicable. Run the Rust test suite covering CLI help and lifecycle behavior after the flow change.
- Bugfix Red Proof: applicable. Start with a failing test that reproduces `create` accepting a `needs_input` envelope before tightening the create path.

## Checklist

- [x] P1 Reject non-ready review envelopes (45m)
  Summary: Tighten create-time parsing so review envelopes must carry a `ready` decision, add red-first coverage for the current gap, and keep bare normalized-plan input working for advanced use.
  Dependencies: none
  Acceptance:
  - Create fails with a clear error when given a `blocked` or `needs_input` review response envelope.
  - Create still accepts a bare normalized plan document and a `ready` review envelope.
- [x] P2 Add guided draft review (90m)
  Summary: Persist lightweight review progress in plan files, add CLI commands that show the next review section and advance review state, and require review completion before approval.
  Dependencies: Reject non-ready review envelopes
  Acceptance:
  - Draft plans expose the next review section in a dedicated walkthrough command and track section completion durably.
  - Approve fails with a clear next step until all review sections have been completed.
- [x] P3 Document and verify the new flow (60m)
  Summary: Update agent-facing docs and CLI coverage so the documented flow is create, review section-by-section, approve, then execute with next/set-status.
  Dependencies: Add guided draft review
  Acceptance:
  - README, AGENTS.md, and help text describe the review walkthrough before approval.
  - Tests cover both the create-time decision guard and the section-by-section review lifecycle.
