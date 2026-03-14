use serde::Serialize;

use crate::review::PlanKind;

#[derive(Debug, Serialize)]
pub struct ReviewSchema {
    pub command: String,
    pub summary: String,
    pub notes: Vec<String>,
    pub fields: Vec<FieldSpec>,
    pub signals: Vec<FieldSpec>,
    pub concern_rule: ConcernRuleSpec,
    pub example_path: String,
}

#[derive(Debug, Serialize)]
pub struct FieldSpec {
    pub name: String,
    pub required: bool,
    pub kind: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct ConcernRuleSpec {
    pub shape: String,
    pub required_concerns: Vec<String>,
    pub rules: Vec<String>,
}

pub fn review_schema(kind: PlanKind) -> ReviewSchema {
    let label = match kind {
        PlanKind::Roadmap => "roadmap",
        PlanKind::Task => "task",
    };
    let summary = match kind {
        PlanKind::Roadmap => {
            "Validate a big-picture plan and normalize it into a durable roadmap file."
        }
        PlanKind::Task => "Validate one execution slice and normalize it into a durable task plan.",
    };

    ReviewSchema {
        command: format!("planwarden review {label}"),
        summary: summary.to_string(),
        notes: vec![
            "The agent is expected to investigate first, then send structured findings instead of free-form prose.".to_string(),
            "Roadmap and task currently share the same payload shape; the difference is scope and the resulting item IDs.".to_string(),
            "If a concern does not apply, the agent must say so explicitly and justify it.".to_string(),
        ],
        fields: vec![
            field("title", false, "string", "Optional display title; defaults to `goal`."),
            field("goal", true, "string", "One clear outcome statement."),
            field("facts", false, "string[]", "Concrete repo findings the agent already verified."),
            field("constraints", false, "string[]", "Hard limits or non-negotiables."),
            field(
                "acceptance_criteria",
                true,
                "string[]",
                "Top-level success conditions for the whole plan.",
            ),
            field(
                "unknowns",
                false,
                "string[]",
                "Real unresolved decisions that should turn into follow-up questions.",
            ),
            field("risks", false, "string[]", "Material implementation or rollout risks."),
            field(
                "proposed_slices",
                true,
                "slice[]",
                "At least one execution slice with title, summary, estimated_minutes, dependencies, and acceptance_criteria.",
            ),
            field(
                "concerns",
                true,
                "object",
                "Applicability plus approach/reason for rollback, security, auth, authz, decoupling, tests, and bugfix red proof.",
            ),
        ],
        signals: vec![
            field("bugfix", true, "boolean", "Set true for bugfix/debugging work."),
            field("user_visible", true, "boolean", "Set true when behavior changes in a user-facing surface."),
            field(
                "touches_authentication",
                true,
                "boolean",
                "Set true when login/session mechanics are affected.",
            ),
            field(
                "touches_authorization",
                true,
                "boolean",
                "Set true when roles/permissions/data access checks are affected.",
            ),
            field(
                "touches_sensitive_data",
                true,
                "boolean",
                "Set true when the work touches secrets, PII, tenant boundaries, or restricted records.",
            ),
            field(
                "touches_external_boundary",
                true,
                "boolean",
                "Set true when external APIs, webhooks, queues, or other trust boundaries are involved.",
            ),
            field(
                "touches_database_schema",
                true,
                "boolean",
                "Set true when migrations or schema changes are involved.",
            ),
            field(
                "cross_cutting_change",
                true,
                "boolean",
                "Set true when the work reaches across multiple modules or layers.",
            ),
        ],
        concern_rule: ConcernRuleSpec {
            shape: "Each concern is `{ applicable: boolean, reason?: string, approach?: string }`.".to_string(),
            required_concerns: vec![
                "rollback".to_string(),
                "security".to_string(),
                "authentication".to_string(),
                "authorization".to_string(),
                "decoupling".to_string(),
                "tests.unit".to_string(),
                "tests.integration".to_string(),
                "tests.regression".to_string(),
                "tests.smoke".to_string(),
                "bugfix_red".to_string(),
            ],
            rules: vec![
                "If `applicable` is true, provide `approach`.".to_string(),
                "If `applicable` is false, provide `reason`.".to_string(),
                "Bugfix work must keep `bugfix_red.applicable = true`; otherwise review blocks.".to_string(),
                "User-visible work should include regression or smoke coverage; otherwise review pushes back.".to_string(),
                "Slices over 90 minutes are pushed back as too large.".to_string(),
                "Signals and concerns must agree. For example, touching authorization cannot mark authorization review as not applicable.".to_string(),
            ],
        },
        example_path: "examples/review-roadmap.json".to_string(),
    }
}

pub fn render_review_schema_text(schema: &ReviewSchema) -> String {
    let mut output = String::new();
    output.push_str(&format!("{}\n", schema.command));
    output.push_str(&format!("Purpose: {}\n\n", schema.summary));

    output.push_str("Top-level fields:\n");
    for field in &schema.fields {
        let required = if field.required {
            "required"
        } else {
            "optional"
        };
        output.push_str(&format!(
            "- {} ({}, {}): {}\n",
            field.name, field.kind, required, field.description
        ));
    }

    output.push_str("\nSignals:\n");
    for field in &schema.signals {
        output.push_str(&format!("- {}: {}\n", field.name, field.description));
    }

    output.push_str("\nConcern rule:\n");
    output.push_str(&format!("- {}\n", schema.concern_rule.shape));
    for rule in &schema.concern_rule.rules {
        output.push_str(&format!("- {}\n", rule));
    }

    output.push_str("\nNotes:\n");
    for note in &schema.notes {
        output.push_str(&format!("- {}\n", note));
    }

    output.push_str(&format!("\nExample payload: {}\n", schema.example_path));
    output
}

fn field(name: &str, required: bool, kind: &str, description: &str) -> FieldSpec {
    FieldSpec {
        name: name.to_string(),
        required,
        kind: kind.to_string(),
        description: description.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{render_review_schema_text, review_schema};
    use crate::review::PlanKind;

    #[test]
    fn roadmap_schema_mentions_required_fields() {
        let schema = review_schema(PlanKind::Roadmap);
        assert_eq!(schema.command, "planwarden review roadmap");
        assert!(
            schema
                .fields
                .iter()
                .any(|field| field.name == "goal" && field.required)
        );
        assert!(
            schema
                .concern_rule
                .rules
                .iter()
                .any(|rule| rule.contains("90 minutes"))
        );
    }

    #[test]
    fn schema_text_is_agent_facing() {
        let output = render_review_schema_text(&review_schema(PlanKind::Task));
        assert!(output.contains("planwarden review task"));
        assert!(output.contains("Top-level fields"));
        assert!(output.contains("Example payload"));
    }
}
