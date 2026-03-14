# Land plan rename and cut release

<!-- planwarden:data:start -->
{
  "kind": "task",
  "plan_status": "in_progress",
  "title": "Land plan rename and cut release",
  "goal": "Bump planwarden for the plan rename, land the current change on main with jj, and kick off a new automated release from trunk.",
  "facts": [
    "This checkout is a real jj repo rooted at /Users/anicolae/code/planwarden.",
    "The trunk bookmark is main and currently points to commit 5b7a22ab.",
    "origin/main also points to commit 5b7a22ab, so local trunk is in sync before landing.",
    "The current working copy @ contains the rename from roadmap to plan and @- is the existing main commit.",
    "The installed release workflow triggers on pushes to main and uses Cargo.toml version to decide the release tag.",
    "GitHub release v0.1.0 already exists, and Cargo.toml is still version 0.1.0.",
    "This jj installation does not expose a jj land subcommand, so landing must use the manual bookmark-move flow."
  ],
  "constraints": [
    "Do not keep roadmap as a compatibility alias.",
    "Because v0.1.0 is already published, Cargo.toml must be bumped before landing or the release workflow will skip.",
    "Use jj's manual land flow rather than assuming a built-in land command exists.",
    "Verify trunk locally and remotely after landing."
  ],
  "acceptance_criteria": [
    "The version is bumped to a new unreleased tag appropriate for the CLI rename.",
    "The current change is landed onto main and pushed to origin/main.",
    "A new release workflow run is started from the landed main revision.",
    "Local verification still passes after the version bump."
  ],
  "risks": [
    "Using the old version number would cause the release workflow to no-op.",
    "Moving the wrong bookmark target would push an unintended revision to main.",
    "If the GitHub release workflow fails after landing, release publication may require follow-up."
  ],
  "concerns": {
    "rollback": {
      "applicable": true,
      "reason": null,
      "approach": "If release verification fails before landing, stop before moving main; if a bad trunk move occurs, move the main bookmark back to the prior commit and push the correction immediately."
    },
    "security": {
      "applicable": true,
      "reason": null,
      "approach": "Use existing authenticated git and GitHub CLI sessions without changing secrets, tokens, or workflow permissions."
    },
    "authentication": {
      "applicable": false,
      "reason": "This work does not change any product login or session behavior.",
      "approach": null
    },
    "authorization": {
      "applicable": false,
      "reason": "This release flow does not alter roles, permissions, or data access rules in the product.",
      "approach": null
    },
    "decoupling": {
      "applicable": true,
      "reason": null,
      "approach": "Keep the release-specific version bump minimal and separate from the already-verified CLI rename so the landed diff remains easy to reason about."
    },
    "tests": {
      "unit": {
        "applicable": true,
        "reason": null,
        "approach": "Run the existing cargo test suite after the version bump to ensure the code and docs still match the renamed command surface."
      },
      "integration": {
        "applicable": true,
        "reason": null,
        "approach": "Build the binary and verify the release smoke script against the freshly built executable before landing."
      },
      "regression": {
        "applicable": true,
        "reason": null,
        "approach": "Keep the renamed CLI help and docs under regression coverage by rerunning the verified test suite before publishing."
      },
      "smoke": {
        "applicable": true,
        "reason": null,
        "approach": "Confirm the release workflow is actually triggered for the new version after pushing main."
      }
    },
    "bugfix_red": {
      "applicable": false,
      "reason": "This is a feature/release flow rather than a bugfix.",
      "approach": null
    }
  },
  "open_questions": [],
  "items": [
    {
      "id": "T1",
      "status": "done",
      "title": "Bump version and verify",
      "summary": "Update Cargo.toml to the next unreleased version for the CLI rename and rerun the targeted verification needed before publishing.",
      "estimated_minutes": 20,
      "dependencies": [],
      "acceptance_criteria": [
        "Cargo.toml reflects the new release version.",
        "Relevant build and test verification pass with the bumped version."
      ]
    },
    {
      "id": "T2",
      "status": "in_progress",
      "title": "Land the change on trunk",
      "summary": "Commit the current working copy, move the main bookmark to the intended revision, push it, and confirm local and remote main match.",
      "estimated_minutes": 20,
      "dependencies": [
        "T1"
      ],
      "acceptance_criteria": [
        "main points at the landed rename-and-version-bump commit locally.",
        "origin/main points at the same commit after push."
      ]
    },
    {
      "id": "T3",
      "status": "todo",
      "title": "Trigger and verify release kickoff",
      "summary": "Confirm the release automation is running for the new version and that the release tag is being prepared from the landed main commit.",
      "estimated_minutes": 20,
      "dependencies": [
        "T2"
      ],
      "acceptance_criteria": [
        "A new release workflow run exists for the landed main revision.",
        "Release metadata references the bumped version tag."
      ]
    }
  ]
}
<!-- planwarden:data:end -->

## Goal

Bump planwarden for the plan rename, land the current change on main with jj, and kick off a new automated release from trunk.

## Plan Status

- in_progress

## Facts

- This checkout is a real jj repo rooted at /Users/anicolae/code/planwarden.
- The trunk bookmark is main and currently points to commit 5b7a22ab.
- origin/main also points to commit 5b7a22ab, so local trunk is in sync before landing.
- The current working copy @ contains the rename from roadmap to plan and @- is the existing main commit.
- The installed release workflow triggers on pushes to main and uses Cargo.toml version to decide the release tag.
- GitHub release v0.1.0 already exists, and Cargo.toml is still version 0.1.0.
- This jj installation does not expose a jj land subcommand, so landing must use the manual bookmark-move flow.

## Constraints

- Do not keep roadmap as a compatibility alias.
- Because v0.1.0 is already published, Cargo.toml must be bumped before landing or the release workflow will skip.
- Use jj's manual land flow rather than assuming a built-in land command exists.
- Verify trunk locally and remotely after landing.

## Acceptance Criteria

- The version is bumped to a new unreleased tag appropriate for the CLI rename.
- The current change is landed onto main and pushed to origin/main.
- A new release workflow run is started from the landed main revision.
- Local verification still passes after the version bump.

## Risks

- Using the old version number would cause the release workflow to no-op.
- Moving the wrong bookmark target would push an unintended revision to main.
- If the GitHub release workflow fails after landing, release publication may require follow-up.

## Open Questions

- none

## Concerns

- Rollback: applicable. If release verification fails before landing, stop before moving main; if a bad trunk move occurs, move the main bookmark back to the prior commit and push the correction immediately.
- Security: applicable. Use existing authenticated git and GitHub CLI sessions without changing secrets, tokens, or workflow permissions.
- Authentication: not applicable. This work does not change any product login or session behavior.
- Authorization: not applicable. This release flow does not alter roles, permissions, or data access rules in the product.
- Decoupling: applicable. Keep the release-specific version bump minimal and separate from the already-verified CLI rename so the landed diff remains easy to reason about.
- Unit Tests: applicable. Run the existing cargo test suite after the version bump to ensure the code and docs still match the renamed command surface.
- Integration Tests: applicable. Build the binary and verify the release smoke script against the freshly built executable before landing.
- Regression Tests: applicable. Keep the renamed CLI help and docs under regression coverage by rerunning the verified test suite before publishing.
- Smoke Tests: applicable. Confirm the release workflow is actually triggered for the new version after pushing main.
- Bugfix Red Proof: not applicable. This is a feature/release flow rather than a bugfix.

## Checklist

- [x] T1 Bump version and verify (20m)
  Summary: Update Cargo.toml to the next unreleased version for the CLI rename and rerun the targeted verification needed before publishing.
  Dependencies: none
  Acceptance:
  - Cargo.toml reflects the new release version.
  - Relevant build and test verification pass with the bumped version.
- [-] T2 Land the change on trunk (20m)
  Summary: Commit the current working copy, move the main bookmark to the intended revision, push it, and confirm local and remote main match.
  Dependencies: T1
  Acceptance:
  - main points at the landed rename-and-version-bump commit locally.
  - origin/main points at the same commit after push.
- [ ] T3 Trigger and verify release kickoff (20m)
  Summary: Confirm the release automation is running for the new version and that the release tag is being prepared from the landed main commit.
  Dependencies: T2
  Acceptance:
  - A new release workflow run exists for the landed main revision.
  - Release metadata references the bumped version tag.
