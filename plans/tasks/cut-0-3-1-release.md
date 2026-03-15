# Cut 0.3.1 release

<!-- planwarden:data:start -->
{
  "kind": "task",
  "plan_status": "done",
  "title": "Cut 0.3.1 release",
  "goal": "Bump `planwarden` from 0.3.0 to 0.3.1 on `main` so the existing GitHub release workflow publishes the unreleased review-flow changes.",
  "facts": [
    "`main` currently includes two unreleased commits after `v0.3.0`: `399a3b0` and `d59bb43b`.",
    "`Cargo.toml` still declares `version = \"0.3.0\"`.",
    "The latest git tag is `v0.3.0`, and `git log` shows it at `6e8bab7` rather than current `HEAD`.",
    "The release workflow on pushes to `main` derives `release_version` from `Cargo.toml`, skips automatic release if that version already has a published GitHub release, and otherwise creates/pushes the corresponding tag.",
    "The repo is currently clean except for the empty mutable jj working commit."
  ],
  "constraints": [
    "Use the existing release automation rather than inventing a parallel manual publish path.",
    "Do not reuse `0.3.0`; a new version is required for the workflow to release these commits.",
    "Keep the change minimal and release-focused: version bump, verification, land to `main`.",
    "Avoid creating a tag locally by hand if the GitHub Actions workflow is already designed to create and push it from `main`."
  ],
  "acceptance_criteria": [
    "`Cargo.toml` is bumped to `0.3.1`.",
    "Local verification passes after the version bump.",
    "The version-bump commit is landed on `main`, which should trigger the repo's release workflow for `v0.3.1`."
  ],
  "risks": [
    "If the required GitHub Actions secrets are missing or broken, landing the version bump will not complete the full external release.",
    "If any other file needs a synchronized version update and we miss it, the release could become inconsistent."
  ],
  "concerns": {
    "rollback": {
      "applicable": true,
      "reason": null,
      "approach": "Keep the change isolated to version metadata so reverting the release trigger is just a follow-up version correction commit if needed."
    },
    "security": {
      "applicable": true,
      "reason": null,
      "approach": "Use the existing trusted GitHub Actions workflow and avoid changing publish secrets, token scopes, or release scripts as part of this version bump."
    },
    "authentication": {
      "applicable": false,
      "reason": "No login, session, or identity flow is involved.",
      "approach": null
    },
    "authorization": {
      "applicable": false,
      "reason": "No permissions logic is changing.",
      "approach": null
    },
    "decoupling": {
      "applicable": true,
      "reason": null,
      "approach": "Avoid touching runtime behavior so the release trigger remains a small metadata-only change on top of already-tested code."
    },
    "tests": {
      "unit": {
        "applicable": false,
        "reason": "The code paths were already covered; this release task is a metadata bump.",
        "approach": null
      },
      "integration": {
        "applicable": false,
        "reason": "No new runtime behavior is introduced by the version bump itself.",
        "approach": null
      },
      "regression": {
        "applicable": false,
        "reason": "This is a release trigger task, not a behavior change requiring new regression coverage.",
        "approach": null
      },
      "smoke": {
        "applicable": true,
        "reason": null,
        "approach": "Run the existing Rust test suite after the version bump before landing it."
      }
    },
    "bugfix_red": {
      "applicable": false,
      "reason": "This is not a bugfix implementation task.",
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
      "title": "Bump release version",
      "summary": "Update the crate version to 0.3.1 so the release workflow sees a new version instead of skipping 0.3.0 as already published.",
      "estimated_minutes": 15,
      "dependencies": [],
      "acceptance_criteria": [
        "`Cargo.toml` declares `version = \"0.3.1\"`.",
        "No unrelated release mechanics are changed."
      ]
    },
    {
      "id": "T2",
      "status": "done",
      "title": "Verify and land release trigger",
      "summary": "Run the test suite, land the version bump on `main`, and confirm the repository is positioned for the GitHub release workflow to publish `v0.3.1`.",
      "estimated_minutes": 20,
      "dependencies": [
        "T1"
      ],
      "acceptance_criteria": [
        "Local verification passes after the bump.",
        "`main` points at the version-bump commit with a clean working copy."
      ]
    }
  ]
}
<!-- planwarden:data:end -->

## Goal

Bump `planwarden` from 0.3.0 to 0.3.1 on `main` so the existing GitHub release workflow publishes the unreleased review-flow changes.

## Plan Status

- done

## Facts

- `main` currently includes two unreleased commits after `v0.3.0`: `399a3b0` and `d59bb43b`.
- `Cargo.toml` still declares `version = "0.3.0"`.
- The latest git tag is `v0.3.0`, and `git log` shows it at `6e8bab7` rather than current `HEAD`.
- The release workflow on pushes to `main` derives `release_version` from `Cargo.toml`, skips automatic release if that version already has a published GitHub release, and otherwise creates/pushes the corresponding tag.
- The repo is currently clean except for the empty mutable jj working commit.

## Constraints

- Use the existing release automation rather than inventing a parallel manual publish path.
- Do not reuse `0.3.0`; a new version is required for the workflow to release these commits.
- Keep the change minimal and release-focused: version bump, verification, land to `main`.
- Avoid creating a tag locally by hand if the GitHub Actions workflow is already designed to create and push it from `main`.

## Acceptance Criteria

- `Cargo.toml` is bumped to `0.3.1`.
- Local verification passes after the version bump.
- The version-bump commit is landed on `main`, which should trigger the repo's release workflow for `v0.3.1`.

## Risks

- If the required GitHub Actions secrets are missing or broken, landing the version bump will not complete the full external release.
- If any other file needs a synchronized version update and we miss it, the release could become inconsistent.

## Open Questions

- none

## Concerns

- Rollback: applicable. Keep the change isolated to version metadata so reverting the release trigger is just a follow-up version correction commit if needed.
- Security: applicable. Use the existing trusted GitHub Actions workflow and avoid changing publish secrets, token scopes, or release scripts as part of this version bump.
- Authentication: not applicable. No login, session, or identity flow is involved.
- Authorization: not applicable. No permissions logic is changing.
- Decoupling: applicable. Avoid touching runtime behavior so the release trigger remains a small metadata-only change on top of already-tested code.
- Unit Tests: not applicable. The code paths were already covered; this release task is a metadata bump.
- Integration Tests: not applicable. No new runtime behavior is introduced by the version bump itself.
- Regression Tests: not applicable. This is a release trigger task, not a behavior change requiring new regression coverage.
- Smoke Tests: applicable. Run the existing Rust test suite after the version bump before landing it.
- Bugfix Red Proof: not applicable. This is not a bugfix implementation task.

## Checklist

- [x] T1 Bump release version (15m)
  Summary: Update the crate version to 0.3.1 so the release workflow sees a new version instead of skipping 0.3.0 as already published.
  Dependencies: none
  Acceptance:
  - `Cargo.toml` declares `version = "0.3.1"`.
  - No unrelated release mechanics are changed.
- [x] T2 Verify and land release trigger (20m)
  Summary: Run the test suite, land the version bump on `main`, and confirm the repository is positioned for the GitHub release workflow to publish `v0.3.1`.
  Dependencies: T1
  Acceptance:
  - Local verification passes after the bump.
  - `main` points at the version-bump commit with a clean working copy.
