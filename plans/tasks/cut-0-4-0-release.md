# Cut 0.4.0 release

<!-- planwarden:data:start -->
{
  "kind": "task",
  "plan_status": "done",
  "title": "Cut 0.4.0 release",
  "goal": "Bump `planwarden` to `0.4.0`, land the change on `main`, and manually dispatch the existing release workflow for `v0.4.0`.",
  "facts": [
    "`Cargo.toml` currently declares `version = \"0.3.1\"`.",
    "`origin` already has tag `v0.3.1`, and GitHub shows that release as published on 2026-03-15.",
    "The release workflow runs on pushes to `main` and also supports `workflow_dispatch`.",
    "On normal pushes, the workflow skips automatic release if the version in `Cargo.toml` is already published, so a new version is required.",
    "`gh auth status` shows an active GitHub login with `repo` and `workflow` scopes, so manual dispatch is available from this checkout."
  ],
  "constraints": [
    "Use a new release version rather than reusing `0.3.1`.",
    "Keep the release change minimal: version bump, verification, land, and workflow dispatch.",
    "Use the repo's existing release workflow instead of inventing a separate local publish path."
  ],
  "acceptance_criteria": [
    "`Cargo.toml` declares `version = \"0.4.0\"`.",
    "The release change is landed on `main` and the target revision is confirmed locally and remotely.",
    "A manual run of the existing `Release` workflow is dispatched for `main` after the land."
  ],
  "risks": [
    "Pushing to `main` will also trigger the release workflow automatically, so the manual dispatch needs to target the same versioned commit deliberately.",
    "If release credentials or downstream publish steps fail in GitHub Actions, the land can succeed while the external release remains incomplete."
  ],
  "concerns": {
    "rollback": {
      "applicable": true,
      "reason": null,
      "approach": "Keep the change isolated to release metadata so any correction is a follow-up version change rather than a code rollback."
    },
    "security": {
      "applicable": true,
      "reason": null,
      "approach": "Use the existing GitHub workflow and current authenticated tooling without changing secrets, tokens, or publish configuration."
    },
    "authentication": {
      "applicable": false,
      "reason": "The release flow does not alter user login or session behavior.",
      "approach": null
    },
    "authorization": {
      "applicable": false,
      "reason": "The change does not alter application permissions or access-control logic.",
      "approach": null
    },
    "decoupling": {
      "applicable": true,
      "reason": null,
      "approach": "Limit the change to version metadata and release orchestration so runtime behavior stays untouched."
    },
    "tests": {
      "unit": {
        "applicable": false,
        "reason": "This release task does not add new runtime logic beyond version metadata.",
        "approach": null
      },
      "integration": {
        "applicable": false,
        "reason": "The existing CLI behavior was already verified; this task is release orchestration.",
        "approach": null
      },
      "regression": {
        "applicable": true,
        "reason": null,
        "approach": "Re-run the current test suite after the version bump to make sure the landed release commit is still green."
      },
      "smoke": {
        "applicable": true,
        "reason": null,
        "approach": "Verify jj land state and confirm the GitHub Actions workflow run is created for the new release."
      }
    },
    "bugfix_red": {
      "applicable": false,
      "reason": "This is a release task, not a bug fix.",
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
      "title": "Bump, land, and dispatch the 0.4.0 release",
      "summary": "Update the crate version, verify the repo, land the change to `main` with jj, and manually trigger the existing GitHub release workflow for the new version.",
      "dependencies": [],
      "acceptance_criteria": [
        "The repo version is updated to `0.4.0` and local verification passes.",
        "`main` points at the release commit with a clean jj working copy after landing.",
        "A `Release` workflow run is created for the landed `main` revision."
      ]
    }
  ]
}
<!-- planwarden:data:end -->

## Goal

Bump `planwarden` to `0.4.0`, land the change on `main`, and manually dispatch the existing release workflow for `v0.4.0`.

## Plan Status

- done

## Facts

- `Cargo.toml` currently declares `version = "0.3.1"`.
- `origin` already has tag `v0.3.1`, and GitHub shows that release as published on 2026-03-15.
- The release workflow runs on pushes to `main` and also supports `workflow_dispatch`.
- On normal pushes, the workflow skips automatic release if the version in `Cargo.toml` is already published, so a new version is required.
- `gh auth status` shows an active GitHub login with `repo` and `workflow` scopes, so manual dispatch is available from this checkout.

## Constraints

- Use a new release version rather than reusing `0.3.1`.
- Keep the release change minimal: version bump, verification, land, and workflow dispatch.
- Use the repo's existing release workflow instead of inventing a separate local publish path.

## Acceptance Criteria

- `Cargo.toml` declares `version = "0.4.0"`.
- The release change is landed on `main` and the target revision is confirmed locally and remotely.
- A manual run of the existing `Release` workflow is dispatched for `main` after the land.

## Risks

- Pushing to `main` will also trigger the release workflow automatically, so the manual dispatch needs to target the same versioned commit deliberately.
- If release credentials or downstream publish steps fail in GitHub Actions, the land can succeed while the external release remains incomplete.

## Open Questions

- none

## Concerns

- Rollback: applicable. Keep the change isolated to release metadata so any correction is a follow-up version change rather than a code rollback.
- Security: applicable. Use the existing GitHub workflow and current authenticated tooling without changing secrets, tokens, or publish configuration.
- Authentication: not applicable. The release flow does not alter user login or session behavior.
- Authorization: not applicable. The change does not alter application permissions or access-control logic.
- Decoupling: applicable. Limit the change to version metadata and release orchestration so runtime behavior stays untouched.
- Unit Tests: not applicable. This release task does not add new runtime logic beyond version metadata.
- Integration Tests: not applicable. The existing CLI behavior was already verified; this task is release orchestration.
- Regression Tests: applicable. Re-run the current test suite after the version bump to make sure the landed release commit is still green.
- Smoke Tests: applicable. Verify jj land state and confirm the GitHub Actions workflow run is created for the new release.
- Bugfix Red Proof: not applicable. This is a release task, not a bug fix.

## Checklist

- [x] T1 Bump, land, and dispatch the 0.4.0 release
  Summary: Update the crate version, verify the repo, land the change to `main` with jj, and manually trigger the existing GitHub release workflow for the new version.
  Dependencies: none
  Acceptance:
  - The repo version is updated to `0.4.0` and local verification passes.
  - `main` points at the release commit with a clean jj working copy after landing.
  - A `Release` workflow run is created for the landed `main` revision.
