use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::review::{Concern, NormalizedPlan, NormalizedPlanItem, PlanItemStatus, ReviewQuestion};

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
    pub progress: ProgressSummary,
    pub focus: Option<ChunkItem>,
    pub up_next: Vec<ChunkItem>,
    pub open_questions: Vec<ReviewQuestion>,
    pub remaining_items: usize,
}

#[derive(Debug, Serialize)]
pub struct StatusUpdateResponse {
    pub path: String,
    pub item: ChunkItem,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct ProgressSummary {
    pub total: usize,
    pub todo: usize,
    pub in_progress: usize,
    pub done: usize,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct ChunkItem {
    pub id: String,
    pub status: PlanItemStatus,
    pub title: String,
    pub summary: String,
    pub estimated_minutes: u32,
    pub dependencies: Vec<String>,
    pub blocked_by: Vec<String>,
    pub acceptance_criteria: Vec<String>,
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
    let progress = compute_progress(&plan);
    let (focus, up_next, remaining_items) = select_chunk_items(&plan, limit.max(1));

    Ok(NextChunkResponse {
        path: path.display().to_string(),
        title: plan.title,
        progress,
        focus,
        up_next,
        open_questions: plan.open_questions.into_iter().take(limit.max(1)).collect(),
        remaining_items,
    })
}

pub fn set_status(
    path: &Path,
    item_id: &str,
    status: PlanItemStatus,
) -> Result<StatusUpdateResponse> {
    let mut plan = load_plan_file(path)?;
    let position = plan
        .items
        .iter()
        .position(|item| item.id == item_id)
        .with_context(|| format!("item `{item_id}` not found in {}", path.display()))?;
    plan.items[position].status = status;
    let updated_item = chunk_item(&plan, &plan.items[position])?;

    let markdown = render_markdown(&plan)?;
    fs::write(path, markdown)
        .with_context(|| format!("failed to update plan file {}", path.display()))?;

    Ok(StatusUpdateResponse {
        path: path.display().to_string(),
        item: updated_item,
    })
}

pub fn render_next_chunk_text(response: &NextChunkResponse) -> String {
    let mut output = String::new();
    let _ = writeln!(&mut output, "{}", response.title);
    let _ = writeln!(
        &mut output,
        "Progress: {}/{} done, {} in progress, {} todo",
        response.progress.done,
        response.progress.total,
        response.progress.in_progress,
        response.progress.todo
    );
    let _ = writeln!(&mut output);

    if let Some(focus) = &response.focus {
        let _ = writeln!(&mut output, "Focus");
        let _ = writeln!(
            &mut output,
            "{} {} {} ({}m)",
            focus.status.checkbox(),
            focus.id,
            focus.title,
            focus.estimated_minutes
        );
        let _ = writeln!(&mut output, "Summary: {}", focus.summary);
        if !focus.blocked_by.is_empty() {
            let _ = writeln!(&mut output, "Blocked by: {}", focus.blocked_by.join(", "));
        }
        let _ = writeln!(&mut output, "Acceptance:");
        for acceptance in &focus.acceptance_criteria {
            let _ = writeln!(&mut output, "- {}", acceptance);
        }
        let _ = writeln!(&mut output);
    } else {
        let _ = writeln!(&mut output, "Focus");
        let _ = writeln!(&mut output, "No incomplete items remain.");
        let _ = writeln!(&mut output);
    }

    if !response.up_next.is_empty() {
        let _ = writeln!(&mut output, "Up Next");
        for item in &response.up_next {
            let _ = writeln!(
                &mut output,
                "{} {} {} ({}m)",
                item.status.checkbox(),
                item.id,
                item.title,
                item.estimated_minutes
            );
            if !item.blocked_by.is_empty() {
                let _ = writeln!(&mut output, "Blocked by: {}", item.blocked_by.join(", "));
            }
        }
        let _ = writeln!(&mut output);
    }

    if !response.open_questions.is_empty() {
        let _ = writeln!(&mut output, "Open Questions");
        for question in &response.open_questions {
            let _ = writeln!(&mut output, "- {}: {}", question.code, question.prompt);
        }
        let _ = writeln!(&mut output);
    }

    if response.remaining_items > 0 {
        let _ = writeln!(
            &mut output,
            "Remaining after this chunk: {} item(s)",
            response.remaining_items
        );
    }

    output.trim_end().to_string()
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

fn compute_progress(plan: &NormalizedPlan) -> ProgressSummary {
    let mut summary = ProgressSummary {
        total: plan.items.len(),
        todo: 0,
        in_progress: 0,
        done: 0,
    };

    for item in &plan.items {
        match item.status {
            PlanItemStatus::Todo => summary.todo += 1,
            PlanItemStatus::InProgress => summary.in_progress += 1,
            PlanItemStatus::Done => summary.done += 1,
        }
    }

    summary
}

fn select_chunk_items(
    plan: &NormalizedPlan,
    limit: usize,
) -> (Option<ChunkItem>, Vec<ChunkItem>, usize) {
    let items = actionable_items(plan);
    if items.is_empty() {
        return (None, Vec::new(), 0);
    }

    let focus = items[0].clone();
    let up_next = items
        .into_iter()
        .skip(1)
        .take(limit.saturating_sub(1))
        .collect::<Vec<_>>();
    let incomplete_count = plan
        .items
        .iter()
        .filter(|item| item.status != PlanItemStatus::Done)
        .count();
    let shown_count = 1 + up_next.len();
    let remaining_items = incomplete_count.saturating_sub(shown_count);

    (Some(focus), up_next, remaining_items)
}

fn actionable_items(plan: &NormalizedPlan) -> Vec<ChunkItem> {
    let done_ids = done_ids(plan);
    let in_progress = plan
        .items
        .iter()
        .filter(|item| item.status == PlanItemStatus::InProgress)
        .map(|item| chunk_item_from_done_ids(item, &done_ids))
        .collect::<Vec<_>>();

    let mut todo = plan
        .items
        .iter()
        .filter(|item| item.status == PlanItemStatus::Todo)
        .map(|item| chunk_item_from_done_ids(item, &done_ids))
        .collect::<Vec<_>>();

    if !in_progress.is_empty() {
        let mut items = in_progress;
        items.extend(todo);
        return items;
    }

    todo.sort_by_key(|item| (!item.blocked_by.is_empty(), item.id.clone()));
    todo
}

fn chunk_item(plan: &NormalizedPlan, item: &NormalizedPlanItem) -> Result<ChunkItem> {
    let done_ids = done_ids(plan);
    Ok(chunk_item_from_done_ids(item, &done_ids))
}

fn done_ids(plan: &NormalizedPlan) -> Vec<&str> {
    plan.items
        .iter()
        .filter(|item| item.status == PlanItemStatus::Done)
        .map(|item| item.id.as_str())
        .collect()
}

fn chunk_item_from_done_ids(item: &NormalizedPlanItem, done_ids: &[&str]) -> ChunkItem {
    let blocked_by = item
        .dependencies
        .iter()
        .filter(|dependency| !done_ids.contains(&dependency.as_str()))
        .cloned()
        .collect();

    ChunkItem {
        id: item.id.clone(),
        status: item.status,
        title: item.title.clone(),
        summary: item.summary.clone(),
        estimated_minutes: item.estimated_minutes,
        dependencies: item.dependencies.clone(),
        blocked_by,
        acceptance_criteria: item.acceptance_criteria.clone(),
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

    writeln!(&mut markdown, "## Open Questions")?;
    writeln!(&mut markdown)?;
    if plan.open_questions.is_empty() {
        writeln!(&mut markdown, "- none")?;
    } else {
        for question in &plan.open_questions {
            writeln!(&mut markdown, "- {}: {}", question.code, question.prompt)?;
        }
    }
    writeln!(&mut markdown)?;

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
        ReviewQuestion, ReviewRequest, review_request,
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

        assert_eq!(chunk.progress.done, 1);
        assert_eq!(chunk.focus.expect("focus item should exist").id, "R1");
        assert!(chunk.up_next.is_empty());
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
        assert!(updated.item.blocked_by.is_empty());
        assert_eq!(loaded.items[0].status, PlanItemStatus::InProgress);
    }

    #[test]
    fn load_plan_file_rejects_missing_markers() {
        let temp = tempfile::tempdir().expect("tempdir should be created");
        let output = temp.path().join("broken.md");
        std::fs::write(&output, "# Broken\n\nNo embedded data here.\n")
            .expect("broken file should write");

        let error = load_plan_file(&output).expect_err("missing markers should fail");
        assert!(
            error
                .to_string()
                .contains("plan file is missing the planwarden data start marker")
        );
    }

    #[test]
    fn write_plan_file_uses_slugified_default_path() {
        let temp = tempfile::tempdir().expect("tempdir should be created");
        let original_dir = std::env::current_dir().expect("cwd should exist");
        std::env::set_current_dir(temp.path()).expect("should enter temp dir");

        let mut plan = sample_plan();
        plan.title = "Billing Portal: MVP / Phase 1".into();

        let created = write_plan_file(&plan, None).expect("plan should write");

        std::env::set_current_dir(original_dir).expect("cwd should restore");
        assert!(
            created
                .path
                .ends_with("plans/roadmaps/billing-portal-mvp-phase-1.md")
        );
        assert!(
            temp.path()
                .join("plans/roadmaps/billing-portal-mvp-phase-1.md")
                .exists()
        );
    }

    #[test]
    fn next_chunk_prioritizes_in_progress_and_preserves_open_questions() {
        let temp = tempfile::tempdir().expect("tempdir should be created");
        let output = temp.path().join("roadmap.md");
        let mut plan = sample_plan();
        plan.open_questions = vec![ReviewQuestion {
            code: "unknown_1".into(),
            prompt: "Clarify the owner vs admin access model.".into(),
        }];
        plan.items[0].status = PlanItemStatus::InProgress;
        plan.items.push(NormalizedPlanItem {
            id: "R2".into(),
            status: PlanItemStatus::Todo,
            title: "Second slice".into(),
            summary: "Do the next thing.".into(),
            estimated_minutes: 30,
            dependencies: vec!["R1".into()],
            acceptance_criteria: vec!["Still works.".into()],
        });

        write_plan_file(&plan, Some(&output)).expect("plan should write");
        let chunk = next_chunk(&output, 3).expect("chunk should load");

        assert_eq!(chunk.progress.in_progress, 1);
        assert_eq!(chunk.focus.expect("focus item should exist").id, "R1");
        assert_eq!(chunk.up_next.len(), 1);
        assert_eq!(chunk.open_questions.len(), 1);
        assert_eq!(chunk.open_questions[0].code, "unknown_1");
    }

    #[test]
    fn next_chunk_surfaces_blocked_dependencies_when_nothing_is_in_progress() {
        let temp = tempfile::tempdir().expect("tempdir should be created");
        let output = temp.path().join("roadmap.md");
        let mut plan = sample_plan();
        plan.items.push(NormalizedPlanItem {
            id: "R2".into(),
            status: PlanItemStatus::Todo,
            title: "Blocked slice".into(),
            summary: "Cannot start yet.".into(),
            estimated_minutes: 30,
            dependencies: vec!["R3".into()],
            acceptance_criteria: vec!["Eventually works.".into()],
        });
        plan.items.push(NormalizedPlanItem {
            id: "R3".into(),
            status: PlanItemStatus::Todo,
            title: "Dependency slice".into(),
            summary: "Must happen first.".into(),
            estimated_minutes: 30,
            dependencies: Vec::new(),
            acceptance_criteria: vec!["Dependency works.".into()],
        });

        write_plan_file(&plan, Some(&output)).expect("plan should write");
        let chunk = next_chunk(&output, 3).expect("chunk should load");

        assert_eq!(chunk.focus.expect("focus item should exist").id, "R1");
        assert_eq!(chunk.up_next[0].id, "R3");
        assert_eq!(chunk.up_next[1].id, "R2");
        assert_eq!(chunk.up_next[1].blocked_by, vec!["R3".to_string()]);
    }
}
