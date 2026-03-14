# Rename roadmap to plan

<!-- planwarden:data:start -->
{
  "kind": "task",
  "plan_status": "done",
  "title": "Rename roadmap to plan",
  "goal": "Replace the roadmap plan kind with plan across the CLI, generated files, examples, docs, and tests so agents use plan for multi-slice work and task for a single slice.",
  "facts": [
    "The CLI currently exposes roadmap and task subcommands for schema, review, and create.",
    "The schema text says roadmap is a big-picture plan while task is a single execution slice.",
    "Normalized roadmap items currently use the R prefix and default to the plans/roadmaps directory.",
    "README, AGENTS.md, the smoke script, examples, and CLI tests all reference roadmap as the primary non-task plan kind."
  ],
  "constraints": [
    "Remove roadmap entirely rather than keeping a compatibility alias.",
    "Preserve task as the single-slice plan kind.",
    "Use plan as the canonical multi-slice plan term in commands, help text, examples, and generated files.",
    "Default plan file output should be plans/<slug>.md rather than plans/plans/<slug>.md."
  ],
  "acceptance_criteria": [
    "The CLI accepts plan and task subcommands and no longer advertises or accepts roadmap.",
    "Normalized plan files and item IDs use plan terminology consistently, including default output paths and checklist IDs.",
    "Repo docs, examples, and bundled plans use plan terminology consistently.",
    "Targeted tests cover the rename and pass."
  ],
  "risks": [
    "Missing a roadmap reference would leave the CLI or docs internally inconsistent.",
    "Changing IDs and default paths could silently break tests or bundled fixtures if not updated together."
  ],
  "concerns": {
    "rollback": {
      "applicable": true,
      "reason": null,
      "approach": "Keep the rename scoped to terminology and file layout so a revert can restore the previous command names and paths in one change set."
    },
    "security": {
      "applicable": false,
      "reason": "This is a local CLI terminology and file-layout change with no new trust boundary or data exposure.",
      "approach": null
    },
    "authentication": {
      "applicable": false,
      "reason": "The CLI has no authentication flow and this change does not add one.",
      "approach": null
    },
    "authorization": {
      "applicable": false,
      "reason": "The CLI has no permission model and this rename does not affect access control.",
      "approach": null
    },
    "decoupling": {
      "applicable": true,
      "reason": null,
      "approach": "Update the plan-kind abstraction and file-path helpers centrally so docs and tests follow the same terminology without special cases spread through the codebase."
    },
    "tests": {
      "unit": {
        "applicable": true,
        "reason": null,
        "approach": "Update unit-style assertions around schema text, normalized plan kinds, item IDs, and default output paths."
      },
      "integration": {
        "applicable": true,
        "reason": null,
        "approach": "Exercise the CLI end-to-end through review/create/next flows using the renamed plan command set."
      },
      "regression": {
        "applicable": true,
        "reason": null,
        "approach": "Cover the user-visible help and generated-path changes so the terminology regression is caught automatically."
      },
      "smoke": {
        "applicable": true,
        "reason": null,
        "approach": "Update the installed-binary smoke script to invoke schema review plan and confirm the new command surface exists."
      }
    },
    "bugfix_red": {
      "applicable": false,
      "reason": "This is a terminology and UX change rather than a bugfix.",
      "approach": null
    }
  },
  "open_questions": [],
  "items": [
    {
      "id": "T1",
      "status": "done",
      "title": "Rename the core plan kind",
      "summary": "Replace roadmap with plan in the CLI enums, schema text, review normalization, ID prefixes, and default output path behavior.",
      "estimated_minutes": 35,
      "dependencies": [],
      "acceptance_criteria": [
        "Schema, review, and create expose plan instead of roadmap.",
        "Generated multi-slice plans use plan labels and P-prefixed item IDs.",
        "Default plan output resolves under plans/<slug>.md."
      ]
    },
    {
      "id": "T2",
      "status": "done",
      "title": "Update fixtures and docs",
      "summary": "Rename examples, embedded plan fixtures, AGENTS guidance, README examples, and helper scripts to use plan terminology and paths.",
      "estimated_minutes": 25,
      "dependencies": [
        "T1"
      ],
      "acceptance_criteria": [
        "Repository-facing documentation uses plan|task rather than roadmap|task.",
        "Example payloads and bundled markdown plans use the renamed paths and labels."
      ]
    },
    {
      "id": "T3",
      "status": "done",
      "title": "Refresh verification",
      "summary": "Update CLI and plan-file tests to assert the new plan terminology, paths, and IDs, then run targeted verification.",
      "estimated_minutes": 30,
      "dependencies": [
        "T1",
        "T2"
      ],
      "acceptance_criteria": [
        "Targeted automated tests pass with plan terminology.",
        "Installed-binary smoke coverage checks the new plan command path."
      ]
    }
  ]
}
<!-- planwarden:data:end -->

## Goal

Replace the roadmap plan kind with plan across the CLI, generated files, examples, docs, and tests so agents use plan for multi-slice work and task for a single slice.

## Plan Status

- done

## Facts

- The CLI currently exposes roadmap and task subcommands for schema, review, and create.
- The schema text says roadmap is a big-picture plan while task is a single execution slice.
- Normalized roadmap items currently use the R prefix and default to the plans/roadmaps directory.
- README, AGENTS.md, the smoke script, examples, and CLI tests all reference roadmap as the primary non-task plan kind.

## Constraints

- Remove roadmap entirely rather than keeping a compatibility alias.
- Preserve task as the single-slice plan kind.
- Use plan as the canonical multi-slice plan term in commands, help text, examples, and generated files.
- Default plan file output should be plans/<slug>.md rather than plans/plans/<slug>.md.

## Acceptance Criteria

- The CLI accepts plan and task subcommands and no longer advertises or accepts roadmap.
- Normalized plan files and item IDs use plan terminology consistently, including default output paths and checklist IDs.
- Repo docs, examples, and bundled plans use plan terminology consistently.
- Targeted tests cover the rename and pass.

## Risks

- Missing a roadmap reference would leave the CLI or docs internally inconsistent.
- Changing IDs and default paths could silently break tests or bundled fixtures if not updated together.

## Open Questions

- none

## Concerns

- Rollback: applicable. Keep the rename scoped to terminology and file layout so a revert can restore the previous command names and paths in one change set.
- Security: not applicable. This is a local CLI terminology and file-layout change with no new trust boundary or data exposure.
- Authentication: not applicable. The CLI has no authentication flow and this change does not add one.
- Authorization: not applicable. The CLI has no permission model and this rename does not affect access control.
- Decoupling: applicable. Update the plan-kind abstraction and file-path helpers centrally so docs and tests follow the same terminology without special cases spread through the codebase.
- Unit Tests: applicable. Update unit-style assertions around schema text, normalized plan kinds, item IDs, and default output paths.
- Integration Tests: applicable. Exercise the CLI end-to-end through review/create/next flows using the renamed plan command set.
- Regression Tests: applicable. Cover the user-visible help and generated-path changes so the terminology regression is caught automatically.
- Smoke Tests: applicable. Update the installed-binary smoke script to invoke schema review plan and confirm the new command surface exists.
- Bugfix Red Proof: not applicable. This is a terminology and UX change rather than a bugfix.

## Checklist

- [x] T1 Rename the core plan kind (35m)
  Summary: Replace roadmap with plan in the CLI enums, schema text, review normalization, ID prefixes, and default output path behavior.
  Dependencies: none
  Acceptance:
  - Schema, review, and create expose plan instead of roadmap.
  - Generated multi-slice plans use plan labels and P-prefixed item IDs.
  - Default plan output resolves under plans/<slug>.md.
- [x] T2 Update fixtures and docs (25m)
  Summary: Rename examples, embedded plan fixtures, AGENTS guidance, README examples, and helper scripts to use plan terminology and paths.
  Dependencies: T1
  Acceptance:
  - Repository-facing documentation uses plan|task rather than roadmap|task.
  - Example payloads and bundled markdown plans use the renamed paths and labels.
- [x] T3 Refresh verification (30m)
  Summary: Update CLI and plan-file tests to assert the new plan terminology, paths, and IDs, then run targeted verification.
  Dependencies: T1, T2
  Acceptance:
  - Targeted automated tests pass with plan terminology.
  - Installed-binary smoke coverage checks the new plan command path.
