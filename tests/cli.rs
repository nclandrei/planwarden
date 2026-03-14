use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

fn binary() -> Command {
    Command::cargo_bin("planwarden").expect("binary should build")
}

fn create_plan(temp_dir: &Path, payload: impl Into<Vec<u8>>) -> PathBuf {
    let review_output = binary()
        .current_dir(temp_dir)
        .args(["review", "plan", "--compact"])
        .write_stdin(payload)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let create_output = binary()
        .current_dir(temp_dir)
        .args(["create", "plan", "--compact"])
        .write_stdin(review_output)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let response: Value =
        serde_json::from_slice(&create_output).expect("create output should be valid JSON");
    temp_dir.join(
        response["path"]
            .as_str()
            .expect("create output should include a path"),
    )
}

fn complete_plan_review(path: &Path) {
    loop {
        let output = binary()
            .args([
                "review-next",
                path.to_str().expect("utf8 path"),
                "--format",
                "json",
                "--compact",
            ])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();
        let response: Value =
            serde_json::from_slice(&output).expect("review-next output should be valid JSON");
        if response["focus"].is_null() {
            break;
        }

        binary()
            .args([
                "advance-review",
                path.to_str().expect("utf8 path"),
                "--compact",
            ])
            .assert()
            .success();
    }
}

fn advance_plan_to_in_progress(path: &Path) {
    complete_plan_review(path);
    binary()
        .args(["approve", path.to_str().expect("utf8 path"), "--compact"])
        .assert()
        .success();
    binary()
        .args(["start", path.to_str().expect("utf8 path"), "--compact"])
        .assert()
        .success();
}

#[test]
fn schema_review_plan_text_is_agent_facing() {
    binary()
        .args(["schema", "review", "plan"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Top-level fields"))
        .stdout(predicate::str::contains("Example payload"))
        .stdout(predicate::str::contains(
            "planwarden review-next <plan-file> --format text",
        ));
}

#[test]
fn top_level_help_describes_schema_first_flow() {
    binary()
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "planwarden schema review plan|task",
        ))
        .stdout(predicate::str::contains(
            "planwarden review-next <plan-file> --format text",
        ))
        .stdout(predicate::str::contains(
            "planwarden next <plan-file> --format text",
        ));
}

#[test]
fn review_help_points_to_schema_command() {
    binary()
        .args(["review", "plan", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("planwarden schema review plan"));
}

#[test]
fn create_help_points_to_next_chunk_flow() {
    binary()
        .args(["create", "plan", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "planwarden review-next <plan-file> --format text",
        ));
}

#[test]
fn review_rejects_unknown_input_fields() {
    let payload = r#"
    {
      "goal": "Test invalid input",
      "facts": [],
      "constraints": [],
      "acceptance_criteria": ["It works"],
      "unknowns": [],
      "risks": [],
      "signals": {
        "bugfix": false,
        "user_visible": false,
        "touches_authentication": false,
        "touches_authorization": false,
        "touches_sensitive_data": false,
        "touches_external_boundary": false,
        "touches_database_schema": false,
        "cross_cutting_change": false
      },
      "proposed_slices": [{
        "title": "One slice",
        "summary": "Do one thing",
        "estimated_minutes": 30,
        "acceptance_criteria": ["It still works"]
      }],
      "concerns": {
        "rollback": {"applicable": true, "approach": "Revert it."},
        "security": {"applicable": false, "reason": "No boundary changes."},
        "authentication": {"applicable": false, "reason": "No auth changes."},
        "authorization": {"applicable": false, "reason": "No permission changes."},
        "decoupling": {"applicable": true, "approach": "Keep it isolated."},
        "tests": {
          "unit": {"applicable": true, "approach": "Unit test it."},
          "integration": {"applicable": false, "reason": "No integration boundary."},
          "regression": {"applicable": false, "reason": "No user-visible change."},
          "smoke": {"applicable": false, "reason": "No smoke needed."}
        },
        "bugfix_red": {"applicable": false, "reason": "Not a bug fix."}
      },
      "extra_field": true
    }
    "#;

    binary()
        .args(["review", "plan"])
        .write_stdin(payload)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "failed to parse review request JSON",
        ))
        .stderr(predicate::str::contains("extra_field"));
}

#[test]
fn create_rejects_kind_mismatch() {
    let review_output = binary()
        .args(["review", "plan", "--compact"])
        .write_stdin(include_str!("../examples/review-plan.json"))
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    binary()
        .args(["create", "task"])
        .write_stdin(review_output)
        .assert()
        .failure()
        .stderr(predicate::str::contains("plan kind mismatch"));
}

#[test]
fn create_rejects_non_ready_review_envelopes() {
    let mut payload: Value =
        serde_json::from_str(include_str!("../examples/review-plan.json")).expect("json");
    payload["unknowns"] = serde_json::json!(["Pick the rollout order."]);

    let review_output = binary()
        .args(["review", "plan", "--compact"])
        .write_stdin(payload.to_string())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    binary()
        .args(["create", "plan"])
        .write_stdin(review_output)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "review response decision must be `ready` before create can write a plan file",
        ));
}

#[test]
fn review_next_text_output_is_chunked_by_section() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let path = create_plan(temp.path(), include_str!("../examples/review-plan.json"));

    binary()
        .args([
            "review-next",
            path.to_str().expect("utf8 path"),
            "--format",
            "text",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Plan Status: draft"))
        .stdout(predicate::str::contains(
            "Next step: planwarden advance-review",
        ))
        .stdout(predicate::str::contains(
            "Review Progress: 0/7 section(s) done",
        ))
        .stdout(predicate::str::contains("Review Now"))
        .stdout(predicate::str::contains("Goal"))
        .stdout(predicate::str::contains("Up Next Review"))
        .stdout(predicate::str::contains("Facts"));
}

#[test]
fn create_without_output_uses_default_slugified_path() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let path = create_plan(temp.path(), include_str!("../examples/review-plan.json"));

    assert_eq!(
        path.strip_prefix(temp.path())
            .expect("path should be relative to temp")
            .to_str()
            .expect("utf8 path"),
        "plans/add-billing-portal.md"
    );
    assert!(path.exists());
}

#[test]
fn next_rejects_malformed_plan_files() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let plan_path = temp.path().join("broken.md");
    fs::write(&plan_path, "# Broken\n\nNo markers here.\n").expect("broken file should write");

    binary()
        .args(["next", plan_path.to_str().expect("utf8 path")])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "plan file is missing the planwarden data start marker",
        ));
}

#[test]
fn set_status_rejects_unknown_item_ids() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let path = create_plan(temp.path(), include_str!("../examples/review-plan.json"));
    advance_plan_to_in_progress(&path);

    binary()
        .args([
            "set-status",
            path.to_str().expect("utf8 path"),
            "P9",
            "done",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("item `P9` not found"));
}

#[test]
fn next_respects_limit_and_status_updates() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let mut payload: Value =
        serde_json::from_str(include_str!("../examples/review-plan.json")).expect("json");
    payload["proposed_slices"] = serde_json::json!([
      {
        "title": "First slice",
        "summary": "Do the first thing.",
        "estimated_minutes": 30,
        "dependencies": [],
        "acceptance_criteria": ["First works."]
      },
      {
        "title": "Second slice",
        "summary": "Do the second thing.",
        "estimated_minutes": 30,
        "dependencies": ["P1"],
        "acceptance_criteria": ["Second works."]
      }
    ]);

    let path = create_plan(
        temp.path(),
        serde_json::to_vec(&payload).expect("payload should serialize"),
    );
    advance_plan_to_in_progress(&path);

    binary()
        .args([
            "set-status",
            path.to_str().expect("utf8 path"),
            "P1",
            "done",
        ])
        .assert()
        .success();

    binary()
        .args([
            "next",
            path.to_str().expect("utf8 path"),
            "--limit",
            "1",
            "--compact",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"focus\":{\"id\":\"P2\""))
        .stdout(predicate::str::contains("\"remaining_items\":0"));
}

#[test]
fn next_text_output_is_chunked_and_includes_questions() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let payload = r#"
    {
      "title": "Chunk demo",
      "goal": "Demonstrate chunked output.",
      "facts": ["Repo already has the core primitives."],
      "constraints": ["Keep output short."],
      "acceptance_criteria": ["The next command shows focus, up next, and questions."],
      "unknowns": ["Should owners and admins both approve the plan?"],
      "risks": ["Chunk output could regress into a wall of text."],
      "signals": {
        "bugfix": false,
        "user_visible": true,
        "touches_authentication": false,
        "touches_authorization": true,
        "touches_sensitive_data": false,
        "touches_external_boundary": false,
        "touches_database_schema": false,
        "cross_cutting_change": false
      },
      "proposed_slices": [
        {
          "title": "Focus slice",
          "summary": "Do the current thing.",
          "estimated_minutes": 30,
          "dependencies": [],
          "acceptance_criteria": ["Current thing works."]
        },
        {
          "title": "Follow-up slice",
          "summary": "Do the next thing.",
          "estimated_minutes": 30,
          "dependencies": ["P1"],
          "acceptance_criteria": ["Next thing works."]
        }
      ],
      "concerns": {
        "rollback": {"applicable": true, "approach": "Revert the changes."},
        "security": {"applicable": false, "reason": "No security boundary changes."},
        "authentication": {"applicable": false, "reason": "No auth changes."},
        "authorization": {"applicable": true, "approach": "Keep plan access limited to the right roles."},
        "decoupling": {"applicable": true, "approach": "Keep planning output isolated from execution details."},
        "tests": {
          "unit": {"applicable": true, "approach": "Test chunk selection."},
          "integration": {"applicable": true, "approach": "Test the CLI flow end to end."},
          "regression": {"applicable": true, "approach": "Protect chunk rendering."},
          "smoke": {"applicable": true, "approach": "Run a real CLI round-trip."}
        },
        "bugfix_red": {"applicable": false, "reason": "This is not a bug fix."}
      }
    }
    "#;

    let review_output = binary()
        .current_dir(temp.path())
        .args(["review", "plan", "--compact"])
        .write_stdin(payload)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let response: Value =
        serde_json::from_slice(&review_output).expect("review output should be valid JSON");
    let path = temp.path().join("plans/chunk-demo.md");
    binary()
        .current_dir(temp.path())
        .args([
            "create",
            "plan",
            "--compact",
            "--output",
            path.to_str().expect("utf8 path"),
        ])
        .write_stdin(
            serde_json::to_vec(&response["normalized_plan"])
                .expect("normalized plan should serialize"),
        )
        .assert()
        .success();

    binary()
        .args([
            "next",
            path.to_str().expect("utf8 path"),
            "--format",
            "text",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Plan Status: draft"))
        .stdout(predicate::str::contains(
            "Next step: planwarden review-next",
        ))
        .stdout(predicate::str::contains("Next Chunk"))
        .stdout(predicate::str::contains("Up Next"))
        .stdout(predicate::str::contains("Open Questions"))
        .stdout(predicate::str::contains("unknown_1"));
}

#[test]
fn set_status_rejects_before_plan_starts() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let path = create_plan(temp.path(), include_str!("../examples/review-plan.json"));

    binary()
        .args([
            "set-status",
            path.to_str().expect("utf8 path"),
            "P1",
            "in-progress",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "cannot update item status while plan is `draft`",
        ));
}

#[test]
fn lifecycle_commands_gate_execution_and_completion() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let path = create_plan(temp.path(), include_str!("../examples/review-plan.json"));

    binary()
        .args([
            "next",
            path.to_str().expect("utf8 path"),
            "--format",
            "text",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Plan Status: draft"))
        .stdout(predicate::str::contains(
            "Next step: planwarden review-next",
        ))
        .stdout(predicate::str::contains("Execution has not started yet"));

    binary()
        .args(["start", path.to_str().expect("utf8 path")])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "cannot start plan while status is `draft`; expected `approved`",
        ));

    binary()
        .args(["approve", path.to_str().expect("utf8 path")])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "cannot approve plan before review is complete",
        ))
        .stderr(predicate::str::contains("planwarden review-next"))
        .stderr(predicate::str::contains("planwarden advance-review"));

    complete_plan_review(&path);

    binary()
        .args(["approve", path.to_str().expect("utf8 path"), "--compact"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"plan_status\":\"approved\""));

    binary()
        .args([
            "next",
            path.to_str().expect("utf8 path"),
            "--format",
            "text",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Plan Status: approved"))
        .stdout(predicate::str::contains("Next step: planwarden start"));

    binary()
        .args([
            "set-status",
            path.to_str().expect("utf8 path"),
            "P1",
            "done",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "cannot update item status while plan is `approved`",
        ));

    binary()
        .args(["start", path.to_str().expect("utf8 path"), "--compact"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"plan_status\":\"in_progress\""));

    binary()
        .args([
            "set-status",
            path.to_str().expect("utf8 path"),
            "P1",
            "done",
        ])
        .assert()
        .success();

    binary()
        .args(["next", path.to_str().expect("utf8 path"), "--compact"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"plan_status\":\"in_progress\""))
        .stdout(predicate::str::contains(
            "\"next_action\":\"planwarden complete",
        ));

    binary()
        .args(["complete", path.to_str().expect("utf8 path"), "--compact"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"plan_status\":\"done\""));

    binary()
        .args([
            "next",
            path.to_str().expect("utf8 path"),
            "--format",
            "text",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Plan Status: done"))
        .stdout(predicate::str::contains("Plan is complete."));
}

#[test]
fn complete_rejects_incomplete_plans() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let path = create_plan(temp.path(), include_str!("../examples/review-plan.json"));
    advance_plan_to_in_progress(&path);

    binary()
        .args(["complete", path.to_str().expect("utf8 path")])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "cannot complete plan while items remain incomplete: P1",
        ));
}
