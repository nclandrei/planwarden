# Tighten guided review presentation contract

<!-- planwarden:data:start -->
{
  "kind": "plan",
  "plan_status": "done",
  "title": "Tighten guided review presentation contract",
  "goal": "Make plan review presentation explicitly section-gated so agents present only one section at a time, ask the user for approval or concerns, and avoid dumping or summarizing the whole plan before advancing.",
  "facts": [
    "`review-next` and `advance-review` already exist, and `approve` already refuses draft plans whose review walkthrough is incomplete.",
    "`render_review_next_text` currently shows the focus section and a generic `Next step: planwarden advance-review ...`, but it does not explicitly instruct the agent to ask the user for approval before advancing.",
    "The schema text, CLI help, and README all say to show only the current review section in chat, but they do not clearly state that the agent must pause for user feedback and must not summarize the whole plan.",
    "The current test suite verifies chunked review/execution output and approval gating, but it does not assert for presentation-specific wording about user approval, concerns, or discussing edits before advancing."
  ],
  "constraints": [
    "Keep the markdown plan file as the durable source of truth.",
    "Do not break the existing `review-next`/`advance-review`/`approve` lifecycle or the execution-time `next` flow.",
    "Avoid coupling the CLI to a single host tool name because some agents may have `askuserquestion`-style tooling while others only have plain chat.",
    "Keep the change small and agent-facing: strengthen the presentation contract without requiring a new storage model for per-section comments."
  ],
  "acceptance_criteria": [
    "`review-next --format text` explicitly tells the agent to present only the current section, ask the user whether it is correct and approved, collect concerns first, and only then run `advance-review`.",
    "Schema/help/documentation make the same contract explicit, including that the agent must not dump the full plan or skip ahead before the current section is accepted.",
    "Automated tests fail red without the new wording and pass green after the change."
  ],
  "risks": [
    "If the output names a specific host tool such as `askuserquestion`, the CLI becomes less portable across agent environments.",
    "If the text is too vague, agents will keep paraphrasing and may still dump a whole-plan summary instead of the current section."
  ],
  "concerns": {
    "rollback": {
      "applicable": true,
      "reason": null,
      "approach": "Keep the change isolated to review text rendering, agent-facing guidance, and tests so it can be reverted without touching plan file structure or lifecycle state."
    },
    "security": {
      "applicable": false,
      "reason": "This changes local CLI messaging and docs only; it does not alter trust boundaries, secrets, or data handling.",
      "approach": null
    },
    "authentication": {
      "applicable": false,
      "reason": "No login, session, or identity flow is involved.",
      "approach": null
    },
    "authorization": {
      "applicable": false,
      "reason": "No permissions or access-control behavior is changing.",
      "approach": null
    },
    "decoupling": {
      "applicable": true,
      "reason": null,
      "approach": "Keep presentation instructions in review-specific rendering/helpers and docs so execution chunking stays separate."
    },
    "tests": {
      "unit": {
        "applicable": true,
        "reason": null,
        "approach": "Add focused assertions around the rendered review-next text contract."
      },
      "integration": {
        "applicable": true,
        "reason": null,
        "approach": "Exercise the CLI surfaces that teach the agent the review flow, including help/schema output."
      },
      "regression": {
        "applicable": true,
        "reason": null,
        "approach": "Capture the current weak wording first so the suite fails before the text is strengthened."
      },
      "smoke": {
        "applicable": true,
        "reason": null,
        "approach": "Run the Rust test suite after the change to ensure the CLI flow still behaves correctly."
      }
    },
    "bugfix_red": {
      "applicable": true,
      "reason": null,
      "approach": "Start by adding failing expectations for the missing approval/pause wording before changing the implementation."
    }
  },
  "open_questions": [],
  "review_state": {
    "completed_sections": [
      "goal",
      "facts",
      "constraints",
      "acceptance_criteria",
      "risks",
      "concerns",
      "checklist"
    ]
  },
  "items": [
    {
      "id": "P1",
      "status": "done",
      "title": "Strengthen review-next presentation text",
      "summary": "Update the guided review text output so it explicitly tells the agent to show only the current section, ask for approval or concerns, and wait before advancing.",
      "estimated_minutes": 35,
      "dependencies": [],
      "acceptance_criteria": [
        "The text output contains explicit instructions to pause on the current section and gather approval or concerns before advancing.",
        "The output tells the agent not to dump or summarize the full plan during review."
      ]
    },
    {
      "id": "P2",
      "status": "done",
      "title": "Align schema and help with the stronger contract",
      "summary": "Update schema notes, CLI help text, and README/AGENTS guidance so the same per-section approval flow is described everywhere the agent learns the workflow.",
      "estimated_minutes": 35,
      "dependencies": [
        "P1"
      ],
      "acceptance_criteria": [
        "Agent-facing contract text references one-section-at-a-time presentation and explicit user approval or concerns.",
        "The guidance remains host-agnostic by allowing either a question tool or plain chat."
      ]
    },
    {
      "id": "P3",
      "status": "done",
      "title": "Prove the presentation contract with tests",
      "summary": "Add or update CLI and unit tests so the regression is caught if the review output or help text stops instructing agents to pause for user review.",
      "estimated_minutes": 30,
      "dependencies": [
        "P1",
        "P2"
      ],
      "acceptance_criteria": [
        "Tests cover the new wording in `review-next` text output.",
        "Tests cover at least one schema/help/documentation surface that communicates the same contract."
      ]
    }
  ]
}
<!-- planwarden:data:end -->

## Goal

Make plan review presentation explicitly section-gated so agents present only one section at a time, ask the user for approval or concerns, and avoid dumping or summarizing the whole plan before advancing.

## Plan Status

- done

## Facts

- `review-next` and `advance-review` already exist, and `approve` already refuses draft plans whose review walkthrough is incomplete.
- `render_review_next_text` currently shows the focus section and a generic `Next step: planwarden advance-review ...`, but it does not explicitly instruct the agent to ask the user for approval before advancing.
- The schema text, CLI help, and README all say to show only the current review section in chat, but they do not clearly state that the agent must pause for user feedback and must not summarize the whole plan.
- The current test suite verifies chunked review/execution output and approval gating, but it does not assert for presentation-specific wording about user approval, concerns, or discussing edits before advancing.

## Constraints

- Keep the markdown plan file as the durable source of truth.
- Do not break the existing `review-next`/`advance-review`/`approve` lifecycle or the execution-time `next` flow.
- Avoid coupling the CLI to a single host tool name because some agents may have `askuserquestion`-style tooling while others only have plain chat.
- Keep the change small and agent-facing: strengthen the presentation contract without requiring a new storage model for per-section comments.

## Acceptance Criteria

- `review-next --format text` explicitly tells the agent to present only the current section, ask the user whether it is correct and approved, collect concerns first, and only then run `advance-review`.
- Schema/help/documentation make the same contract explicit, including that the agent must not dump the full plan or skip ahead before the current section is accepted.
- Automated tests fail red without the new wording and pass green after the change.

## Risks

- If the output names a specific host tool such as `askuserquestion`, the CLI becomes less portable across agent environments.
- If the text is too vague, agents will keep paraphrasing and may still dump a whole-plan summary instead of the current section.

## Open Questions

- none

## Concerns

- Rollback: applicable. Keep the change isolated to review text rendering, agent-facing guidance, and tests so it can be reverted without touching plan file structure or lifecycle state.
- Security: not applicable. This changes local CLI messaging and docs only; it does not alter trust boundaries, secrets, or data handling.
- Authentication: not applicable. No login, session, or identity flow is involved.
- Authorization: not applicable. No permissions or access-control behavior is changing.
- Decoupling: applicable. Keep presentation instructions in review-specific rendering/helpers and docs so execution chunking stays separate.
- Unit Tests: applicable. Add focused assertions around the rendered review-next text contract.
- Integration Tests: applicable. Exercise the CLI surfaces that teach the agent the review flow, including help/schema output.
- Regression Tests: applicable. Capture the current weak wording first so the suite fails before the text is strengthened.
- Smoke Tests: applicable. Run the Rust test suite after the change to ensure the CLI flow still behaves correctly.
- Bugfix Red Proof: applicable. Start by adding failing expectations for the missing approval/pause wording before changing the implementation.

## Checklist

- [x] P1 Strengthen review-next presentation text (35m)
  Summary: Update the guided review text output so it explicitly tells the agent to show only the current section, ask for approval or concerns, and wait before advancing.
  Dependencies: none
  Acceptance:
  - The text output contains explicit instructions to pause on the current section and gather approval or concerns before advancing.
  - The output tells the agent not to dump or summarize the full plan during review.
- [x] P2 Align schema and help with the stronger contract (35m)
  Summary: Update schema notes, CLI help text, and README/AGENTS guidance so the same per-section approval flow is described everywhere the agent learns the workflow.
  Dependencies: P1
  Acceptance:
  - Agent-facing contract text references one-section-at-a-time presentation and explicit user approval or concerns.
  - The guidance remains host-agnostic by allowing either a question tool or plain chat.
- [x] P3 Prove the presentation contract with tests (30m)
  Summary: Add or update CLI and unit tests so the regression is caught if the review output or help text stops instructing agents to pause for user review.
  Dependencies: P1, P2
  Acceptance:
  - Tests cover the new wording in `review-next` text output.
  - Tests cover at least one schema/help/documentation surface that communicates the same contract.
