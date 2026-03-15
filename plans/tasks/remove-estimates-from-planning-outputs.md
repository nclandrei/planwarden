# Remove estimates from planning outputs

<!-- planwarden:data:start -->
{
  "kind": "task",
  "plan_status": "done",
  "title": "Remove estimates from planning outputs",
  "goal": "Remove estimate and duration concepts from plan review payloads, normalized plan data, and rendered plan output.",
  "facts": [
    "`estimated_minutes` is currently required in the review schema text, example payload, and `ProposedSlice` request model.",
    "`NormalizedPlanItem` and `ChunkItem` both persist `estimated_minutes`, and checklist or chunk renderers print values like `(30m)`.",
    "Review pushback currently blocks slices over 90 minutes, so estimate size is part of validation rather than display-only metadata.",
    "Plan markdown stores the normalized plan JSON in an embedded data block, and those structs currently deserialize without `deny_unknown_fields`, which gives room for legacy file compatibility if needed."
  ],
  "constraints": [
    "Keep the schema-first review/create/review-next workflow intact.",
    "Do not regress plan-file round-tripping, review-next, next-chunk, or lifecycle commands.",
    "New plan output should stop showing estimate or duration language in checklist and chunk text."
  ],
  "acceptance_criteria": [
    "Review schema and example payloads no longer require or advertise estimate or duration fields for proposed slices.",
    "Review normalization, stored plan items, and rendered markdown/text outputs no longer carry estimate or duration data.",
    "Automated tests cover the updated contract and the main CLI flows still pass."
  ],
  "risks": [
    "Removing a required request field can break older review payloads unless compatibility is handled deliberately.",
    "Stored markdown plans and text renderers share the same models, so partial removal could leave stale `(Xm)` output or parse failures."
  ],
  "concerns": {
    "rollback": {
      "applicable": true,
      "reason": null,
      "approach": "Keep the change scoped to planning models and renderers so reverting restores the previous contract without data migrations."
    },
    "security": {
      "applicable": false,
      "reason": "This change only removes scheduling metadata from local planning payloads and output.",
      "approach": null
    },
    "authentication": {
      "applicable": false,
      "reason": "The change does not affect login, sessions, or identity flows.",
      "approach": null
    },
    "authorization": {
      "applicable": false,
      "reason": "The change does not affect role checks or access control.",
      "approach": null
    },
    "decoupling": {
      "applicable": true,
      "reason": null,
      "approach": "Update shared plan models and renderers in one pass so schema, storage, and text output stay aligned instead of forking per surface."
    },
    "tests": {
      "unit": {
        "applicable": true,
        "reason": null,
        "approach": "Update review and plan-file unit tests to assert the new slice shape and rendered text."
      },
      "integration": {
        "applicable": true,
        "reason": null,
        "approach": "Run CLI integration tests that exercise review, create, review-next, next, and lifecycle flows with the new payload shape."
      },
      "regression": {
        "applicable": true,
        "reason": null,
        "approach": "Cover checklist and chunk rendering so estimate text does not reappear."
      },
      "smoke": {
        "applicable": true,
        "reason": null,
        "approach": "Run the targeted test suite against the compiled CLI after the model change."
      }
    },
    "bugfix_red": {
      "applicable": false,
      "reason": "This is a contract simplification, not a bug fix.",
      "approach": null
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
      "id": "T1",
      "status": "done",
      "title": "Remove estimate fields from the planning contract and renderers",
      "summary": "Update the review schema, normalization path, plan-file rendering, and regression tests so plan slices no longer model or display estimated duration.",
      "dependencies": [],
      "acceptance_criteria": [
        "The CLI schema and example payloads describe slices without estimate fields.",
        "Generated checklist and next-chunk output no longer includes `(Xm)` annotations.",
        "Tests verify the new contract and pass for the touched flows."
      ]
    }
  ]
}
<!-- planwarden:data:end -->

## Goal

Remove estimate and duration concepts from plan review payloads, normalized plan data, and rendered plan output.

## Plan Status

- done

## Facts

- `estimated_minutes` is currently required in the review schema text, example payload, and `ProposedSlice` request model.
- `NormalizedPlanItem` and `ChunkItem` both persist `estimated_minutes`, and checklist or chunk renderers print values like `(30m)`.
- Review pushback currently blocks slices over 90 minutes, so estimate size is part of validation rather than display-only metadata.
- Plan markdown stores the normalized plan JSON in an embedded data block, and those structs currently deserialize without `deny_unknown_fields`, which gives room for legacy file compatibility if needed.

## Constraints

- Keep the schema-first review/create/review-next workflow intact.
- Do not regress plan-file round-tripping, review-next, next-chunk, or lifecycle commands.
- New plan output should stop showing estimate or duration language in checklist and chunk text.

## Acceptance Criteria

- Review schema and example payloads no longer require or advertise estimate or duration fields for proposed slices.
- Review normalization, stored plan items, and rendered markdown/text outputs no longer carry estimate or duration data.
- Automated tests cover the updated contract and the main CLI flows still pass.

## Risks

- Removing a required request field can break older review payloads unless compatibility is handled deliberately.
- Stored markdown plans and text renderers share the same models, so partial removal could leave stale `(Xm)` output or parse failures.

## Open Questions

- none

## Concerns

- Rollback: applicable. Keep the change scoped to planning models and renderers so reverting restores the previous contract without data migrations.
- Security: not applicable. This change only removes scheduling metadata from local planning payloads and output.
- Authentication: not applicable. The change does not affect login, sessions, or identity flows.
- Authorization: not applicable. The change does not affect role checks or access control.
- Decoupling: applicable. Update shared plan models and renderers in one pass so schema, storage, and text output stay aligned instead of forking per surface.
- Unit Tests: applicable. Update review and plan-file unit tests to assert the new slice shape and rendered text.
- Integration Tests: applicable. Run CLI integration tests that exercise review, create, review-next, next, and lifecycle flows with the new payload shape.
- Regression Tests: applicable. Cover checklist and chunk rendering so estimate text does not reappear.
- Smoke Tests: applicable. Run the targeted test suite against the compiled CLI after the model change.
- Bugfix Red Proof: not applicable. This is a contract simplification, not a bug fix.

## Checklist

- [x] T1 Remove estimate fields from the planning contract and renderers
  Summary: Update the review schema, normalization path, plan-file rendering, and regression tests so plan slices no longer model or display estimated duration.
  Dependencies: none
  Acceptance:
  - The CLI schema and example payloads describe slices without estimate fields.
  - Generated checklist and next-chunk output no longer includes `(Xm)` annotations.
  - Tests verify the new contract and pass for the touched flows.
