# Add structured review approval prompts

<!-- planwarden:data:start -->
{
  "kind": "plan",
  "plan_status": "done",
  "title": "Add structured review approval prompts",
  "goal": "Extend `planwarden review-next` with structured approval and prompt metadata so host agents can ask for section approval consistently, with host-aware naming hints informed by current Codex and Claude Code docs.",
  "facts": [
    "`ReviewNextResponse` currently returns `focus`, `up_next`, `remaining_sections`, and `next_action`, but there is no structured JSON field that tells a host exactly what approval question to ask before advancing review.",
    "The text renderer already tells the agent to ask for approval or concerns before calling `advance-review`, so the remaining gap is machine-readable metadata for host integrations.",
    "Current CLI tests cover text output and lifecycle gating, but they do not assert JSON fields for review-time approval prompts or host tool hints.",
    "OpenAI's current Codex docs describe confirmation before side-effecting actions and show app tools named like `Calendar.create_event`, but they do not expose a public first-party `ask user` tool name in those docs.",
    "Current Claude Code docs describe ask-style permission flows with `permissionDecision: \"ask\"`, allow a `--permission-prompt-tool`, and expose MCP prompts as slash commands named `/mcp__<serverName>__<promptName>`.",
    "In this Codex environment, the available structured user-input tool is named `request_user_input`, while repo-local Claude-oriented skill metadata refers to an `AskUserQuestion` capability, so host hints must distinguish documented surfaces from runtime-specific tool names."
  ],
  "constraints": [
    "Keep `review-next` backwards-compatible for existing JSON consumers by adding fields rather than renaming existing ones.",
    "Do not invent undocumented public tool names for Codex or Claude Code; when a runtime-specific name is helpful, label it as a host hint instead of a docs-backed canonical name.",
    "Preserve the current text workflow and approval gating behavior.",
    "Keep the response host-agnostic enough that another agent runtime can ignore the hints and still use the generic prompt and options."
  ],
  "acceptance_criteria": [
    "`review-next --format json` includes structured approval metadata for the current section: a prompt, clear options, the command to run after approval, and a rule that concerns block advancement until resolved.",
    "The response includes host-aware hints that separate docs-backed naming surfaces from runtime-specific tool names for Codex and Claude Code.",
    "Tests cover the new JSON structure, and docs/help explain the purpose of the new fields."
  ],
  "risks": [
    "If the response overfits one host, other agents may treat the fields as canonical and misuse them.",
    "If docs-backed names and runtime-specific names are mixed together without labeling, the metadata will be misleading instead of helpful."
  ],
  "concerns": {
    "rollback": {
      "applicable": true,
      "reason": null,
      "approach": "Keep the new fields additive and isolated to review-next response construction so they can be removed without rewriting plan files or lifecycle logic."
    },
    "security": {
      "applicable": false,
      "reason": "This changes local response metadata and docs only; it does not touch secrets, trust boundaries, or privileged actions.",
      "approach": null
    },
    "authentication": {
      "applicable": false,
      "reason": "No login, session, or identity flow is involved.",
      "approach": null
    },
    "authorization": {
      "applicable": false,
      "reason": "No permissions model is being changed.",
      "approach": null
    },
    "decoupling": {
      "applicable": true,
      "reason": null,
      "approach": "Model approval metadata in dedicated response structs so host integration hints remain separate from review state and text rendering."
    },
    "tests": {
      "unit": {
        "applicable": true,
        "reason": null,
        "approach": "Add focused tests for review-next response construction and field contents."
      },
      "integration": {
        "applicable": true,
        "reason": null,
        "approach": "Exercise the CLI JSON output to confirm consumers receive the new approval fields."
      },
      "regression": {
        "applicable": true,
        "reason": null,
        "approach": "Start with failing JSON assertions so the suite proves the missing machine-readable approval contract first."
      },
      "smoke": {
        "applicable": true,
        "reason": null,
        "approach": "Run the full Rust test suite after the response-shape change."
      }
    },
    "bugfix_red": {
      "applicable": true,
      "reason": null,
      "approach": "Add failing tests for the missing JSON approval metadata before implementing the new fields."
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
      "title": "Add structured approval metadata to review-next",
      "summary": "Extend the review-next response model with machine-readable approval prompts, options, and advance behavior so hosts do not need to scrape the text renderer.",
      "estimated_minutes": 45,
      "dependencies": [],
      "acceptance_criteria": [
        "The JSON response contains an approval block when a review section is active.",
        "The block includes the current prompt, available response options, and the command to run after approval."
      ]
    },
    {
      "id": "P2",
      "status": "done",
      "title": "Add host-aware tool naming hints",
      "summary": "Include structured Codex and Claude Code hint fields that distinguish public docs terminology from runtime-specific tool names available in actual hosts.",
      "estimated_minutes": 45,
      "dependencies": [
        "P1"
      ],
      "acceptance_criteria": [
        "Codex metadata distinguishes docs-backed approval surfaces from the runtime hint for this environment.",
        "Claude Code metadata captures docs-backed ask/prompt surfaces without pretending undocumented names are canonical."
      ]
    },
    {
      "id": "P3",
      "status": "done",
      "title": "Document and test the review-next JSON contract",
      "summary": "Add regression coverage and concise docs so integrators know how to consume the new review-next approval fields.",
      "estimated_minutes": 35,
      "dependencies": [
        "P1",
        "P2"
      ],
      "acceptance_criteria": [
        "CLI or unit tests assert the new approval fields and host hints.",
        "Agent-facing docs mention that hosts can use the JSON approval block instead of scraping text."
      ]
    }
  ]
}
<!-- planwarden:data:end -->

## Goal

Extend `planwarden review-next` with structured approval and prompt metadata so host agents can ask for section approval consistently, with host-aware naming hints informed by current Codex and Claude Code docs.

## Plan Status

- done

## Facts

- `ReviewNextResponse` currently returns `focus`, `up_next`, `remaining_sections`, and `next_action`, but there is no structured JSON field that tells a host exactly what approval question to ask before advancing review.
- The text renderer already tells the agent to ask for approval or concerns before calling `advance-review`, so the remaining gap is machine-readable metadata for host integrations.
- Current CLI tests cover text output and lifecycle gating, but they do not assert JSON fields for review-time approval prompts or host tool hints.
- OpenAI's current Codex docs describe confirmation before side-effecting actions and show app tools named like `Calendar.create_event`, but they do not expose a public first-party `ask user` tool name in those docs.
- Current Claude Code docs describe ask-style permission flows with `permissionDecision: "ask"`, allow a `--permission-prompt-tool`, and expose MCP prompts as slash commands named `/mcp__<serverName>__<promptName>`.
- In this Codex environment, the available structured user-input tool is named `request_user_input`, while repo-local Claude-oriented skill metadata refers to an `AskUserQuestion` capability, so host hints must distinguish documented surfaces from runtime-specific tool names.

## Constraints

- Keep `review-next` backwards-compatible for existing JSON consumers by adding fields rather than renaming existing ones.
- Do not invent undocumented public tool names for Codex or Claude Code; when a runtime-specific name is helpful, label it as a host hint instead of a docs-backed canonical name.
- Preserve the current text workflow and approval gating behavior.
- Keep the response host-agnostic enough that another agent runtime can ignore the hints and still use the generic prompt and options.

## Acceptance Criteria

- `review-next --format json` includes structured approval metadata for the current section: a prompt, clear options, the command to run after approval, and a rule that concerns block advancement until resolved.
- The response includes host-aware hints that separate docs-backed naming surfaces from runtime-specific tool names for Codex and Claude Code.
- Tests cover the new JSON structure, and docs/help explain the purpose of the new fields.

## Risks

- If the response overfits one host, other agents may treat the fields as canonical and misuse them.
- If docs-backed names and runtime-specific names are mixed together without labeling, the metadata will be misleading instead of helpful.

## Open Questions

- none

## Concerns

- Rollback: applicable. Keep the new fields additive and isolated to review-next response construction so they can be removed without rewriting plan files or lifecycle logic.
- Security: not applicable. This changes local response metadata and docs only; it does not touch secrets, trust boundaries, or privileged actions.
- Authentication: not applicable. No login, session, or identity flow is involved.
- Authorization: not applicable. No permissions model is being changed.
- Decoupling: applicable. Model approval metadata in dedicated response structs so host integration hints remain separate from review state and text rendering.
- Unit Tests: applicable. Add focused tests for review-next response construction and field contents.
- Integration Tests: applicable. Exercise the CLI JSON output to confirm consumers receive the new approval fields.
- Regression Tests: applicable. Start with failing JSON assertions so the suite proves the missing machine-readable approval contract first.
- Smoke Tests: applicable. Run the full Rust test suite after the response-shape change.
- Bugfix Red Proof: applicable. Add failing tests for the missing JSON approval metadata before implementing the new fields.

## Checklist

- [x] P1 Add structured approval metadata to review-next (45m)
  Summary: Extend the review-next response model with machine-readable approval prompts, options, and advance behavior so hosts do not need to scrape the text renderer.
  Dependencies: none
  Acceptance:
  - The JSON response contains an approval block when a review section is active.
  - The block includes the current prompt, available response options, and the command to run after approval.
- [x] P2 Add host-aware tool naming hints (45m)
  Summary: Include structured Codex and Claude Code hint fields that distinguish public docs terminology from runtime-specific tool names available in actual hosts.
  Dependencies: P1
  Acceptance:
  - Codex metadata distinguishes docs-backed approval surfaces from the runtime hint for this environment.
  - Claude Code metadata captures docs-backed ask/prompt surfaces without pretending undocumented names are canonical.
- [x] P3 Document and test the review-next JSON contract (35m)
  Summary: Add regression coverage and concise docs so integrators know how to consume the new review-next approval fields.
  Dependencies: P1, P2
  Acceptance:
  - CLI or unit tests assert the new approval fields and host hints.
  - Agent-facing docs mention that hosts can use the JSON approval block instead of scraping text.
