# Rewrite README around a concise example-led structure

<!-- planwarden:data:start -->
{
  "kind": "task",
  "plan_status": "done",
  "title": "Rewrite README around a concise example-led structure",
  "goal": "Rewrite the README so it is concise, example-led, and accurate to the current CLI while following the same documentation standard the user referenced.",
  "facts": [
    "The repo currently has README.md, Cargo.toml, src/, tests/, a LICENSE file, and GitHub workflows at .github/workflows/ci.yml and .github/workflows/release.yml.",
    "Cargo.toml identifies the project as planwarden v0.3.1, a Rust CLI published at https://github.com/nclandrei/planwarden with Homebrew tap metadata for nclandrei/homebrew-tap.",
    "The current CLI help describes a schema-first flow with the commands review, schema, create, review-next, advance-review, next, set-status, approve, start, and complete.",
    "The existing README already documents install, quick start, workflow, commands, release automation, and license, but it is not organized as a badge-first, example-led document.",
    "The reference README fetched with gh api is badge-first, concise, example-led, installation-focused, and includes an inlined help block plus short focused sections for deeper capabilities."
  ],
  "constraints": [
    "Do not mention the reference project in README.md.",
    "Keep every command and installation path accurate to the current planwarden CLI and repository metadata.",
    "Preserve planwarden's schema-first review flow and section-by-section review contract in the documentation."
  ],
  "acceptance_criteria": [
    "README.md opens with a concise positioning statement that treats AI agents as the primary operator of the tool, while making the human's role in pointing the agent at the work clear, and includes real repository badges/links.",
    "README.md includes a concrete example-led workflow that reflects the actual planwarden contract and command names from the perspective of an AI agent using the tool.",
    "README.md includes installation and help/reference material that matches the current CLI without mentioning the reference project."
  ],
  "risks": [
    "A style-focused rewrite could accidentally flatten planwarden-specific guidance or drift from the exact CLI wording.",
    "Embedded help text can go stale unless it is copied from the current binary output."
  ],
  "concerns": {
    "rollback": {
      "applicable": false,
      "reason": "This is a documentation-only change with no runtime behavior; reverting means editing README.md again rather than shipping a rollback path.",
      "approach": null
    },
    "security": {
      "applicable": false,
      "reason": "The task changes documentation only and does not alter trust boundaries, secrets, or data exposure.",
      "approach": null
    },
    "authentication": {
      "applicable": false,
      "reason": "The task does not touch login, sessions, or identity flows.",
      "approach": null
    },
    "authorization": {
      "applicable": false,
      "reason": "The task does not touch roles, permissions, or data access checks.",
      "approach": null
    },
    "decoupling": {
      "applicable": false,
      "reason": "The task rewrites documentation and does not introduce code coupling across modules.",
      "approach": null
    },
    "tests": {
      "unit": {
        "applicable": false,
        "reason": "There is no code change that would justify unit tests.",
        "approach": null
      },
      "integration": {
        "applicable": false,
        "reason": "There is no integration behavior change to cover.",
        "approach": null
      },
      "regression": {
        "applicable": true,
        "reason": null,
        "approach": "Verify every command sequence and install/reference claim in the rewritten README against the current CLI help, Cargo metadata, workflow files, and GitHub release metadata."
      },
      "smoke": {
        "applicable": true,
        "reason": null,
        "approach": "Run the key CLI help commands used by the README and confirm linked release/workflow assets exist so the docs remain executable and trustworthy."
      }
    },
    "bugfix_red": {
      "applicable": false,
      "reason": "This is documentation work, not a bugfix.",
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
      "title": "Rewrite the README structure and content",
      "summary": "Replace README.md with a badge-first, concise, example-led document that treats AI agents as the primary user, while still helping humans understand how to point an agent at the tool.",
      "estimated_minutes": 60,
      "dependencies": [],
      "acceptance_criteria": [
        "The README starts with badges, a short description, and a concrete example section that frames the tool for AI-agent use.",
        "The README documents installation options supported by the repo and releases.",
        "The README preserves planwarden-specific workflow rules, including review-next and advance-review usage."
      ]
    }
  ]
}
<!-- planwarden:data:end -->

## Goal

Rewrite the README so it is concise, example-led, and accurate to the current CLI while following the same documentation standard the user referenced.

## Plan Status

- done

## Facts

- The repo currently has README.md, Cargo.toml, src/, tests/, a LICENSE file, and GitHub workflows at .github/workflows/ci.yml and .github/workflows/release.yml.
- Cargo.toml identifies the project as planwarden v0.3.1, a Rust CLI published at https://github.com/nclandrei/planwarden with Homebrew tap metadata for nclandrei/homebrew-tap.
- The current CLI help describes a schema-first flow with the commands review, schema, create, review-next, advance-review, next, set-status, approve, start, and complete.
- The existing README already documents install, quick start, workflow, commands, release automation, and license, but it is not organized as a badge-first, example-led document.
- The reference README fetched with gh api is badge-first, concise, example-led, installation-focused, and includes an inlined help block plus short focused sections for deeper capabilities.

## Constraints

- Do not mention the reference project in README.md.
- Keep every command and installation path accurate to the current planwarden CLI and repository metadata.
- Preserve planwarden's schema-first review flow and section-by-section review contract in the documentation.

## Acceptance Criteria

- README.md opens with a concise positioning statement that treats AI agents as the primary operator of the tool, while making the human's role in pointing the agent at the work clear, and includes real repository badges/links.
- README.md includes a concrete example-led workflow that reflects the actual planwarden contract and command names from the perspective of an AI agent using the tool.
- README.md includes installation and help/reference material that matches the current CLI without mentioning the reference project.

## Risks

- A style-focused rewrite could accidentally flatten planwarden-specific guidance or drift from the exact CLI wording.
- Embedded help text can go stale unless it is copied from the current binary output.

## Open Questions

- none

## Concerns

- Rollback: not applicable. This is a documentation-only change with no runtime behavior; reverting means editing README.md again rather than shipping a rollback path.
- Security: not applicable. The task changes documentation only and does not alter trust boundaries, secrets, or data exposure.
- Authentication: not applicable. The task does not touch login, sessions, or identity flows.
- Authorization: not applicable. The task does not touch roles, permissions, or data access checks.
- Decoupling: not applicable. The task rewrites documentation and does not introduce code coupling across modules.
- Unit Tests: not applicable. There is no code change that would justify unit tests.
- Integration Tests: not applicable. There is no integration behavior change to cover.
- Regression Tests: applicable. Verify every command sequence and install/reference claim in the rewritten README against the current CLI help, Cargo metadata, workflow files, and GitHub release metadata.
- Smoke Tests: applicable. Run the key CLI help commands used by the README and confirm linked release/workflow assets exist so the docs remain executable and trustworthy.
- Bugfix Red Proof: not applicable. This is documentation work, not a bugfix.

## Checklist

- [x] T1 Rewrite the README structure and content (60m)
  Summary: Replace README.md with a badge-first, concise, example-led document that treats AI agents as the primary user, while still helping humans understand how to point an agent at the tool.
  Dependencies: none
  Acceptance:
  - The README starts with badges, a short description, and a concrete example section that frames the tool for AI-agent use.
  - The README documents installation options supported by the repo and releases.
  - The README preserves planwarden-specific workflow rules, including review-next and advance-review usage.
