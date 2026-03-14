use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::review::{Concern, NormalizedPlan, NormalizedPlanItem, PlanItemStatus};

const START_MARKER: &str = "<!-- planwarden:data:start -->";
const END_MARKER: &str = "<!-- planwarden:data:end -->";

#[derive(Debug, Serialize)]
pub struct CreatePlanResponse {
    pub path: String,
    pub title: String,
    pub item_count: usize,
}

#[derive(Debug, Serialize)]
pub struct NextChunkResponse {
    pub path: String,
    pub title: String,
    pub items: Vec<NormalizedPlanItem>,
}

#[derive(Debug, Serialize)]
pub struct StatusUpdateResponse {
    pub path: String,
    pub item: NormalizedPlanItem,
}

#[derive(Debug, Deserialize)]
struct ReviewEnvelope {
    normalized_plan: NormalizedPlan,
}

pub fn extract_plan_from_json(raw: &str) -> Result<NormalizedPlan> {
    if let Ok(plan) = serde_json::from_str::<NormalizedPlan>(raw) {
        return Ok(plan);
    }

    if let Ok(envelope) = serde_json::from_str::<ReviewEnvelope>(raw) {
        return Ok(envelope.normalized_plan);
    }

    let value: Value = serde_json::from_str(raw).context("failed to parse JSON input")?;
    bail!(
        "input must be either a normalized plan document or a review response containing `normalized_plan`; got keys: {}",
        value
            .as_object()
            .map(|object| {
                let mut keys = object.keys().cloned().collect::<Vec<_>>();
                keys.sort();
                keys.join(", ")
            })
            .unwrap_or_else(|| "<non-object>".to_string())
    );
}

pub fn write_plan_file(plan: &NormalizedPlan, output: Option<&Path>) -> Result<CreatePlanResponse> {
    let path = output
        .map(Path::to_path_buf)
        .unwrap_or_else(|| default_plan_path(plan));
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }

    let markdown = render_markdown(plan)?;
    fs::write(&path, markdown)
        .with_context(|| format!("failed to write plan file to {}", path.display()))?;

    Ok(CreatePlanResponse {
        path: path.display().to_string(),
        title: plan.title.clone(),
        item_count: plan.items.len(),
    })
}

pub fn load_plan_file(path: &Path) -> Result<NormalizedPlan> {
    let markdown = fs::read_to_string(path)
        .with_context(|| format!("failed to read plan file {}", path.display()))?;
    extract_plan_from_markdown(&markdown)
}

pub fn next_chunk(path: &Path, limit: usize) -> Result<NextChunkResponse> {
    let plan = load_plan_file(path)?;
    let items = plan
        .items
        .into_iter()
        .filter(|item| item.status != PlanItemStatus::Done)
        .take(limit.max(1))
        .collect();

    Ok(NextChunkResponse {
        path: path.display().to_string(),
        title: plan.title,
        items,
    })
}

pub fn set_status(
    path: &Path,
    item_id: &str,
    status: PlanItemStatus,
) -> Result<StatusUpdateResponse> {
    let mut plan = load_plan_file(path)?;
    let item = plan
        .items
        .iter_mut()
        .find(|item| item.id == item_id)
        .with_context(|| format!("item `{item_id}` not found in {}", path.display()))?;
    item.status = status;
    let updated_item = item.clone();

    let markdown = render_markdown(&plan)?;
    fs::write(path, markdown)
        .with_context(|| format!("failed to update plan file {}", path.display()))?;

    Ok(StatusUpdateResponse {
        path: path.display().to_string(),
        item: updated_item,
    })
}

fn default_plan_path(plan: &NormalizedPlan) -> PathBuf {
    PathBuf::from("plans")
        .join(plan.kind.directory())
        .join(format!("{}.md", slugify(&plan.title)))
}

fn slugify(title: &str) -> String {
    let mut slug = String::new();
    let mut last_dash = false;

    for character in title.chars() {
        let lowered = character.to_ascii_lowercase();
        if lowered.is_ascii_alphanumeric() {
            slug.push(lowered);
            last_dash = false;
        } else if !last_dash {
            slug.push('-');
            last_dash = true;
        }
    }

    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        "plan".to_string()
    } else {
        slug
    }
}

fn render_markdown(plan: &NormalizedPlan) -> Result<String> {
    let data = serde_json::to_string_pretty(plan).context("failed to serialize plan data")?;
    let mut markdown = String::new();

    writeln!(&mut markdown, "# {}", plan.title)?;
    writeln!(&mut markdown)?;
    writeln!(&mut markdown, "{START_MARKER}")?;
    writeln!(&mut markdown, "{data}")?;
    writeln!(&mut markdown, "{END_MARKER}")?;
    writeln!(&mut markdown)?;
    writeln!(&mut markdown, "## Goal")?;
    writeln!(&mut markdown)?;
    writeln!(&mut markdown, "{}", plan.goal)?;
    writeln!(&mut markdown)?;

    render_list_section(&mut markdown, "Facts", &plan.facts)?;
    render_list_section(&mut markdown, "Constraints", &plan.constraints)?;
    render_list_section(
        &mut markdown,
        "Acceptance Criteria",
        &plan.acceptance_criteria,
    )?;
    render_list_section(&mut markdown, "Risks", &plan.risks)?;

    writeln!(&mut markdown, "## Concerns")?;
    writeln!(&mut markdown)?;
    render_concern(&mut markdown, "Rollback", &plan.concerns.rollback)?;
    render_concern(&mut markdown, "Security", &plan.concerns.security)?;
    render_concern(
        &mut markdown,
        "Authentication",
        &plan.concerns.authentication,
    )?;
    render_concern(&mut markdown, "Authorization", &plan.concerns.authorization)?;
    render_concern(&mut markdown, "Decoupling", &plan.concerns.decoupling)?;
    render_concern(&mut markdown, "Unit Tests", &plan.concerns.tests.unit)?;
    render_concern(
        &mut markdown,
        "Integration Tests",
        &plan.concerns.tests.integration,
    )?;
    render_concern(
        &mut markdown,
        "Regression Tests",
        &plan.concerns.tests.regression,
    )?;
    render_concern(&mut markdown, "Smoke Tests", &plan.concerns.tests.smoke)?;
    render_concern(&mut markdown, "Bugfix Red Proof", &plan.concerns.bugfix_red)?;
    writeln!(&mut markdown)?;

    writeln!(&mut markdown, "## Checklist")?;
    writeln!(&mut markdown)?;

    for item in &plan.items {
        writeln!(
            &mut markdown,
            "- {} {} {} ({}m)",
            item.status.checkbox(),
            item.id,
            item.title,
            item.estimated_minutes
        )?;
        writeln!(&mut markdown, "  Summary: {}", item.summary)?;
        if item.dependencies.is_empty() {
            writeln!(&mut markdown, "  Dependencies: none")?;
        } else {
            writeln!(
                &mut markdown,
                "  Dependencies: {}",
                item.dependencies.join(", ")
            )?;
        }
        writeln!(&mut markdown, "  Acceptance:")?;
        for acceptance in &item.acceptance_criteria {
            writeln!(&mut markdown, "  - {}", acceptance)?;
        }
    }

    Ok(markdown)
}

fn render_list_section(markdown: &mut String, heading: &str, items: &[String]) -> Result<()> {
    writeln!(markdown, "## {heading}")?;
    writeln!(markdown)?;
    if items.is_empty() {
        writeln!(markdown, "- none")?;
    } else {
        for item in items {
            writeln!(markdown, "- {item}")?;
        }
    }
    writeln!(markdown)?;
    Ok(())
}

fn render_concern(markdown: &mut String, label: &str, concern: &Concern) -> Result<()> {
    let detail = if concern.applicable {
        concern.approach.as_deref().unwrap_or("missing approach")
    } else {
        concern.reason.as_deref().unwrap_or("missing reason")
    };
    let state = if concern.applicable {
        "applicable"
    } else {
        "not applicable"
    };
    writeln!(markdown, "- {label}: {state}. {detail}")?;
    Ok(())
}

fn extract_plan_from_markdown(markdown: &str) -> Result<NormalizedPlan> {
    let start = markdown
        .find(START_MARKER)
        .with_context(|| "plan file is missing the planwarden data start marker")?;
    let end = markdown
        .find(END_MARKER)
        .with_context(|| "plan file is missing the planwarden data end marker")?;

    if end <= start {
        bail!("plan file data markers are out of order");
    }

    let json = markdown[start + START_MARKER.len()..end].trim();
    serde_json::from_str(json).context("failed to parse embedded plan data")
}

#[cfg(test)]
mod tests {
    use super::{extract_plan_from_json, load_plan_file, next_chunk, set_status, write_plan_file};
    use crate::review::{
        NormalizedPlan, NormalizedPlanItem, PlanDocumentKind, PlanItemStatus, PlanKind,
        ReviewRequest, review_request,
    };

    fn sample_plan() -> NormalizedPlan {
        let request: ReviewRequest =
            serde_json::from_str(include_str!("../examples/review-roadmap.json"))
                .expect("example request should parse");
        review_request(PlanKind::Roadmap, request).normalized_plan
    }

    #[test]
    fn create_accepts_review_response_payload() {
        let request: ReviewRequest =
            serde_json::from_str(include_str!("../examples/review-roadmap.json"))
                .expect("example request should parse");
        let response = review_request(PlanKind::Roadmap, request);
        let raw = serde_json::to_string(&response).expect("response should serialize");

        let plan = extract_plan_from_json(&raw).expect("plan should extract");

        assert_eq!(plan.kind, PlanDocumentKind::Roadmap);
        assert_eq!(plan.items.len(), 1);
    }

    #[test]
    fn written_plan_round_trips_from_markdown() {
        let temp = tempfile::tempdir().expect("tempdir should be created");
        let output = temp.path().join("roadmap.md");
        let plan = sample_plan();

        write_plan_file(&plan, Some(&output)).expect("plan should write");
        let loaded = load_plan_file(&output).expect("plan should reload");

        assert_eq!(loaded.title, plan.title);
        assert_eq!(loaded.items, plan.items);
    }

    #[test]
    fn next_chunk_skips_done_items() {
        let temp = tempfile::tempdir().expect("tempdir should be created");
        let output = temp.path().join("roadmap.md");
        let mut plan = sample_plan();
        plan.items.push(NormalizedPlanItem {
            id: "R2".into(),
            status: PlanItemStatus::Done,
            title: "Already done".into(),
            summary: "Completed slice".into(),
            estimated_minutes: 30,
            dependencies: Vec::new(),
            acceptance_criteria: vec!["It exists.".into()],
        });

        write_plan_file(&plan, Some(&output)).expect("plan should write");
        let chunk = next_chunk(&output, 5).expect("chunk should load");

        assert_eq!(chunk.items.len(), 1);
        assert_eq!(chunk.items[0].id, "R1");
    }

    #[test]
    fn set_status_updates_embedded_plan_data() {
        let temp = tempfile::tempdir().expect("tempdir should be created");
        let output = temp.path().join("roadmap.md");
        let plan = sample_plan();

        write_plan_file(&plan, Some(&output)).expect("plan should write");
        let updated =
            set_status(&output, "R1", PlanItemStatus::InProgress).expect("status should update");
        let loaded = load_plan_file(&output).expect("plan should reload");

        assert_eq!(updated.item.status, PlanItemStatus::InProgress);
        assert_eq!(loaded.items[0].status, PlanItemStatus::InProgress);
    }
}
