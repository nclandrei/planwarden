use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

fn binary() -> Command {
    Command::cargo_bin("planwarden").expect("binary should build")
}

fn create_roadmap_plan(temp_dir: &Path, payload: impl Into<Vec<u8>>) -> PathBuf {
    let review_output = binary()
        .current_dir(temp_dir)
        .args(["review", "roadmap", "--compact"])
        .write_stdin(payload)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let create_output = binary()
        .current_dir(temp_dir)
        .args(["create", "roadmap", "--compact"])
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

fn advance_plan_to_in_progress(path: &Path) {
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
fn schema_review_roadmap_text_is_agent_facing() {
    binary()
        .args(["schema", "review", "roadmap"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Top-level fields"))
        .stdout(predicate::str::contains("Example payload"))
        .stdout(predicate::str::contains(
            "planwarden next <plan-file> --format text",
        ));
}

#[test]
fn top_level_help_describes_schema_first_flow() {
    binary()
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "planwarden schema review roadmap|task",
        ))
        .stdout(predicate::str::contains(
            "planwarden next <plan-file> --format text",
        ));
}

#[test]
fn review_help_points_to_schema_command() {
    binary()
        .args(["review", "roadmap", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("planwarden schema review roadmap"));
}

#[test]
fn create_help_points_to_next_chunk_flow() {
    binary()
        .args(["create", "roadmap", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "planwarden next <plan-file> --format text",
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
        .args(["review", "roadmap"])
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
        .args(["review", "roadmap", "--compact"])
        .write_stdin(include_str!("../examples/review-roadmap.json"))
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
fn create_without_output_uses_default_slugified_path() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let path = create_roadmap_plan(temp.path(), include_str!("../examples/review-roadmap.json"));

    assert_eq!(
        path.strip_prefix(temp.path())
            .expect("path should be relative to temp")
            .to_str()
            .expect("utf8 path"),
        "plans/roadmaps/add-billing-portal.md"
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
    let path = create_roadmap_plan(temp.path(), include_str!("../examples/review-roadmap.json"));
    advance_plan_to_in_progress(&path);

    binary()
        .args([
            "set-status",
            path.to_str().expect("utf8 path"),
            "R9",
            "done",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("item `R9` not found"));
}

#[test]
fn next_respects_limit_and_status_updates() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let mut payload: Value =
        serde_json::from_str(include_str!("../examples/review-roadmap.json")).expect("json");
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
        "dependencies": ["R1"],
        "acceptance_criteria": ["Second works."]
      }
    ]);

    let path = create_roadmap_plan(
        temp.path(),
        serde_json::to_vec(&payload).expect("payload should serialize"),
    );
    advance_plan_to_in_progress(&path);

    binary()
        .args([
            "set-status",
            path.to_str().expect("utf8 path"),
            "R1",
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
        .stdout(predicate::str::contains("\"focus\":{\"id\":\"R2\""))
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
          "dependencies": ["R1"],
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

    let path = create_roadmap_plan(temp.path(), payload);

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
        .stdout(predicate::str::contains("Next step: planwarden approve"))
        .stdout(predicate::str::contains("Next Chunk"))
        .stdout(predicate::str::contains("Up Next"))
        .stdout(predicate::str::contains("Open Questions"))
        .stdout(predicate::str::contains("unknown_1"));
}

#[test]
fn set_status_rejects_before_plan_starts() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let path = create_roadmap_plan(temp.path(), include_str!("../examples/review-roadmap.json"));

    binary()
        .args([
            "set-status",
            path.to_str().expect("utf8 path"),
            "R1",
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
    let path = create_roadmap_plan(temp.path(), include_str!("../examples/review-roadmap.json"));

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
        .stdout(predicate::str::contains("Next step: planwarden approve"))
        .stdout(predicate::str::contains("Execution has not started yet"));

    binary()
        .args(["start", path.to_str().expect("utf8 path")])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "cannot start plan while status is `draft`; expected `approved`",
        ));

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
            "R1",
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
            "R1",
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
    let path = create_roadmap_plan(temp.path(), include_str!("../examples/review-roadmap.json"));
    advance_plan_to_in_progress(&path);

    binary()
        .args(["complete", path.to_str().expect("utf8 path")])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "cannot complete plan while items remain incomplete: R1",
        ));
}
