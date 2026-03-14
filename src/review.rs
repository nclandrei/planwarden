use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanKind {
    Roadmap,
    Task,
}

impl PlanKind {
    fn id_prefix(self) -> &'static str {
        match self {
            Self::Roadmap => "R",
            Self::Task => "T",
        }
    }
}

impl From<PlanKind> for PlanDocumentKind {
    fn from(value: PlanKind) -> Self {
        match value {
            PlanKind::Roadmap => Self::Roadmap,
            PlanKind::Task => Self::Task,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewRequest {
    #[serde(default)]
    pub title: Option<String>,
    pub goal: String,
    #[serde(default)]
    pub facts: Vec<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,
    #[serde(default)]
    pub unknowns: Vec<String>,
    #[serde(default)]
    pub risks: Vec<String>,
    pub signals: ReviewSignals,
    #[serde(default)]
    pub proposed_slices: Vec<ProposedSlice>,
    pub concerns: ReviewConcerns,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewSignals {
    #[serde(default)]
    pub bugfix: bool,
    #[serde(default)]
    pub user_visible: bool,
    #[serde(default)]
    pub touches_authentication: bool,
    #[serde(default)]
    pub touches_authorization: bool,
    #[serde(default)]
    pub touches_sensitive_data: bool,
    #[serde(default)]
    pub touches_external_boundary: bool,
    #[serde(default)]
    pub touches_database_schema: bool,
    #[serde(default)]
    pub cross_cutting_change: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProposedSlice {
    pub title: String,
    pub summary: String,
    pub estimated_minutes: u32,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Concern {
    pub applicable: bool,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub approach: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TestConcerns {
    pub unit: Concern,
    pub integration: Concern,
    pub regression: Concern,
    pub smoke: Concern,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReviewConcerns {
    pub rollback: Concern,
    pub security: Concern,
    pub authentication: Concern,
    pub authorization: Concern,
    pub decoupling: Concern,
    pub tests: TestConcerns,
    pub bugfix_red: Concern,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecision {
    Blocked,
    NeedsInput,
    Ready,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Issue {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ReviewQuestion {
    pub code: String,
    pub prompt: String,
}

#[derive(Debug, Serialize)]
pub struct ReviewResponse {
    pub decision: ReviewDecision,
    pub missing: Vec<Issue>,
    pub questions: Vec<ReviewQuestion>,
    pub pushback: Vec<Issue>,
    pub normalized_plan: NormalizedPlan,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlanDocumentKind {
    Roadmap,
    Task,
}

impl PlanDocumentKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Roadmap => "roadmap",
            Self::Task => "task",
        }
    }

    pub fn directory(&self) -> &'static str {
        match self {
            Self::Roadmap => "roadmaps",
            Self::Task => "tasks",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NormalizedPlan {
    pub kind: PlanDocumentKind,
    pub plan_status: PlanLifecycleStatus,
    pub title: String,
    pub goal: String,
    pub facts: Vec<String>,
    pub constraints: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub risks: Vec<String>,
    pub concerns: ReviewConcerns,
    #[serde(default)]
    pub open_questions: Vec<ReviewQuestion>,
    pub items: Vec<NormalizedPlanItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NormalizedPlanItem {
    pub id: String,
    pub status: PlanItemStatus,
    pub title: String,
    pub summary: String,
    pub estimated_minutes: u32,
    pub dependencies: Vec<String>,
    pub acceptance_criteria: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlanItemStatus {
    Todo,
    InProgress,
    Done,
}

impl PlanItemStatus {
    pub fn checkbox(self) -> &'static str {
        match self {
            Self::Todo => "[ ]",
            Self::InProgress => "[-]",
            Self::Done => "[x]",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlanLifecycleStatus {
    Draft,
    Approved,
    InProgress,
    Done,
}

impl PlanLifecycleStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Approved => "approved",
            Self::InProgress => "in_progress",
            Self::Done => "done",
        }
    }
}

pub fn review_request(kind: PlanKind, request: ReviewRequest) -> ReviewResponse {
    let mut missing = Vec::new();
    let mut pushback = Vec::new();
    let mut questions = Vec::new();

    if request.goal.trim().is_empty() {
        missing.push(issue("goal_missing", "Goal is required.", Some("goal")));
    }

    if request.acceptance_criteria.is_empty() {
        missing.push(issue(
            "acceptance_criteria_missing",
            "At least one top-level acceptance criterion is required.",
            Some("acceptance_criteria"),
        ));
    }

    if request.proposed_slices.is_empty() {
        missing.push(issue(
            "proposed_slices_missing",
            "At least one proposed slice is required.",
            Some("proposed_slices"),
        ));
    }

    validate_concern(
        "rollback",
        "concerns.rollback",
        &request.concerns.rollback,
        &mut missing,
    );
    validate_concern(
        "security",
        "concerns.security",
        &request.concerns.security,
        &mut missing,
    );
    validate_concern(
        "authentication",
        "concerns.authentication",
        &request.concerns.authentication,
        &mut missing,
    );
    validate_concern(
        "authorization",
        "concerns.authorization",
        &request.concerns.authorization,
        &mut missing,
    );
    validate_concern(
        "decoupling",
        "concerns.decoupling",
        &request.concerns.decoupling,
        &mut missing,
    );
    validate_concern(
        "unit_tests",
        "concerns.tests.unit",
        &request.concerns.tests.unit,
        &mut missing,
    );
    validate_concern(
        "integration_tests",
        "concerns.tests.integration",
        &request.concerns.tests.integration,
        &mut missing,
    );
    validate_concern(
        "regression_tests",
        "concerns.tests.regression",
        &request.concerns.tests.regression,
        &mut missing,
    );
    validate_concern(
        "smoke_tests",
        "concerns.tests.smoke",
        &request.concerns.tests.smoke,
        &mut missing,
    );
    validate_concern(
        "bugfix_red",
        "concerns.bugfix_red",
        &request.concerns.bugfix_red,
        &mut missing,
    );

    if request.signals.touches_database_schema && !request.concerns.rollback.applicable {
        pushback.push(issue(
            "rollback_inconsistent",
            "Rollback cannot be marked not applicable when the change touches database schema.",
            Some("concerns.rollback"),
        ));
    }

    if request.signals.touches_authentication && !request.concerns.authentication.applicable {
        pushback.push(issue(
            "authentication_inconsistent",
            "Authentication review cannot be marked not applicable when authentication is affected.",
            Some("concerns.authentication"),
        ));
    }

    if request.signals.touches_authorization && !request.concerns.authorization.applicable {
        pushback.push(issue(
            "authorization_inconsistent",
            "Authorization review cannot be marked not applicable when permissions are affected.",
            Some("concerns.authorization"),
        ));
    }

    if (request.signals.touches_sensitive_data || request.signals.touches_external_boundary)
        && !request.concerns.security.applicable
    {
        pushback.push(issue(
            "security_inconsistent",
            "Security review cannot be marked not applicable when the change touches sensitive data or an external boundary.",
            Some("concerns.security"),
        ));
    }

    if request.signals.cross_cutting_change && !request.concerns.decoupling.applicable {
        pushback.push(issue(
            "decoupling_inconsistent",
            "Decoupling cannot be marked not applicable for a cross-cutting change.",
            Some("concerns.decoupling"),
        ));
    }

    if request.signals.bugfix && !request.concerns.bugfix_red.applicable {
        pushback.push(issue(
            "bugfix_red_inconsistent",
            "Bugfix work must include explicit red-proof handling.",
            Some("concerns.bugfix_red"),
        ));
    }

    if request.signals.user_visible
        && !request.concerns.tests.regression.applicable
        && !request.concerns.tests.smoke.applicable
    {
        pushback.push(issue(
            "user_visible_test_gap",
            "User-visible work must include regression or smoke coverage, or both.",
            Some("concerns.tests"),
        ));
    }

    for (index, slice) in request.proposed_slices.iter().enumerate() {
        let slice_number = index + 1;
        if slice.title.trim().is_empty() {
            missing.push(issue(
                format!("slice_{slice_number}_title_missing"),
                format!("Slice {slice_number} is missing a title."),
                Some(format!("proposed_slices[{index}].title")),
            ));
        }

        if slice.summary.trim().is_empty() {
            missing.push(issue(
                format!("slice_{slice_number}_summary_missing"),
                format!("Slice {slice_number} is missing a summary."),
                Some(format!("proposed_slices[{index}].summary")),
            ));
        }

        if slice.acceptance_criteria.is_empty() {
            missing.push(issue(
                format!("slice_{slice_number}_acceptance_missing"),
                format!("Slice {slice_number} needs at least one acceptance criterion."),
                Some(format!("proposed_slices[{index}].acceptance_criteria")),
            ));
        }

        if slice.estimated_minutes > 90 {
            pushback.push(issue(
                format!("slice_{slice_number}_too_large"),
                format!(
                    "Slice {slice_number} is estimated at {} minutes; split it into a smaller chunk.",
                    slice.estimated_minutes
                ),
                Some(format!("proposed_slices[{index}].estimated_minutes")),
            ));
        }
    }

    for (index, unknown) in request.unknowns.iter().enumerate() {
        if !unknown.trim().is_empty() {
            questions.push(ReviewQuestion {
                code: format!("unknown_{}", index + 1),
                prompt: format!("Resolve this before finalizing the plan: {unknown}"),
            });
        }
    }

    let normalized_plan = normalize_plan(kind, &request, &questions);

    let decision = if !pushback.is_empty() {
        ReviewDecision::Blocked
    } else if !missing.is_empty() || !questions.is_empty() {
        ReviewDecision::NeedsInput
    } else {
        ReviewDecision::Ready
    };

    ReviewResponse {
        decision,
        missing,
        questions,
        pushback,
        normalized_plan,
    }
}

fn normalize_plan(
    kind: PlanKind,
    request: &ReviewRequest,
    questions: &[ReviewQuestion],
) -> NormalizedPlan {
    let title = request
        .title
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(&request.goal)
        .to_string();

    let items = request
        .proposed_slices
        .iter()
        .enumerate()
        .map(|(index, slice)| NormalizedPlanItem {
            id: format!("{}{}", kind.id_prefix(), index + 1),
            status: PlanItemStatus::Todo,
            title: slice.title.clone(),
            summary: slice.summary.clone(),
            estimated_minutes: slice.estimated_minutes,
            dependencies: slice.dependencies.clone(),
            acceptance_criteria: slice.acceptance_criteria.clone(),
        })
        .collect();

    NormalizedPlan {
        kind: kind.into(),
        plan_status: PlanLifecycleStatus::Draft,
        title,
        goal: request.goal.clone(),
        facts: request.facts.clone(),
        constraints: request.constraints.clone(),
        acceptance_criteria: request.acceptance_criteria.clone(),
        risks: request.risks.clone(),
        concerns: request.concerns.clone(),
        open_questions: questions.to_vec(),
        items,
    }
}

fn validate_concern(name: &str, field: &str, concern: &Concern, missing: &mut Vec<Issue>) {
    if concern.applicable {
        if blank(&concern.approach) {
            missing.push(issue(
                format!("{name}_approach_missing"),
                format!("{name} is marked applicable but has no approach."),
                Some(field),
            ));
        }
    } else if blank(&concern.reason) {
        missing.push(issue(
            format!("{name}_reason_missing"),
            format!("{name} is marked not applicable but has no justification."),
            Some(field),
        ));
    }
}

fn blank(value: &Option<String>) -> bool {
    value
        .as_deref()
        .map(|item| item.trim().is_empty())
        .unwrap_or(true)
}

fn issue<C, M, F>(code: C, message: M, field: Option<F>) -> Issue
where
    C: Into<String>,
    M: Into<String>,
    F: Into<String>,
{
    Issue {
        code: code.into(),
        message: message.into(),
        field: field.map(Into::into),
    }
}

#[cfg(test)]
mod tests {
    use super::{PlanKind, ReviewDecision, ReviewRequest, review_request};

    fn ready_request() -> ReviewRequest {
        serde_json::from_str(
            r#"
            {
              "title": "Add billing portal",
              "goal": "Add a billing portal entry point under settings.",
              "facts": ["Stripe integration already exists."],
              "constraints": ["Must be rollbackable."],
              "acceptance_criteria": ["Users can open the billing portal from settings."],
              "unknowns": [],
              "risks": ["Incorrect tenant mapping could expose the wrong portal session."],
              "signals": {
                "bugfix": false,
                "user_visible": true,
                "touches_authentication": false,
                "touches_authorization": true,
                "touches_sensitive_data": true,
                "touches_external_boundary": true,
                "touches_database_schema": false,
                "cross_cutting_change": true
              },
              "proposed_slices": [
                {
                  "title": "Wire settings action",
                  "summary": "Add the settings action that creates a portal session through the existing backend endpoint.",
                  "estimated_minutes": 45,
                  "dependencies": [],
                  "acceptance_criteria": ["The settings page shows a billing portal action for eligible users."]
                }
              ],
              "concerns": {
                "rollback": {
                  "applicable": true,
                  "approach": "Guard the new entry point behind a feature flag that can be disabled."
                },
                "security": {
                  "applicable": true,
                  "approach": "Reuse the existing server-side portal session creation path and keep Stripe keys server-only."
                },
                "authentication": {
                  "applicable": false,
                  "reason": "The change reuses the existing authenticated session model without altering login flows."
                },
                "authorization": {
                  "applicable": true,
                  "approach": "Only render the action for owners and enforce the same role check on the server."
                },
                "decoupling": {
                  "applicable": true,
                  "approach": "Keep billing wiring inside the settings and billing modules without spreading Stripe calls into unrelated views."
                },
                "tests": {
                  "unit": {
                    "applicable": true,
                    "approach": "Cover the visibility predicate for billing portal access."
                  },
                  "integration": {
                    "applicable": true,
                    "approach": "Add an integration test for portal session creation and redirect handling."
                  },
                  "regression": {
                    "applicable": true,
                    "approach": "Add a regression test for owner visibility on the settings page."
                  },
                  "smoke": {
                    "applicable": true,
                    "approach": "Exercise the happy path in a smoke check against the settings flow."
                  }
                },
                "bugfix_red": {
                  "applicable": false,
                  "reason": "This is feature work, not a bug fix."
                }
              }
            }
            "#,
        )
        .expect("request should deserialize")
    }

    #[test]
    fn review_ready_when_contract_is_satisfied() {
        let response = review_request(PlanKind::Roadmap, ready_request());

        assert_eq!(response.decision, ReviewDecision::Ready);
        assert!(response.missing.is_empty());
        assert!(response.questions.is_empty());
        assert!(response.pushback.is_empty());
        assert_eq!(response.normalized_plan.items[0].id, "R1");
        assert_eq!(response.normalized_plan.kind.label(), "roadmap");
    }

    #[test]
    fn review_needs_input_for_missing_approach_and_unknowns() {
        let mut request = ready_request();
        request.unknowns =
            vec!["Confirm whether billing is owner-only or admin-accessible.".into()];
        request.concerns.security.approach = None;

        let response = review_request(PlanKind::Task, request);

        assert_eq!(response.decision, ReviewDecision::NeedsInput);
        assert_eq!(response.questions.len(), 1);
        assert_eq!(response.normalized_plan.open_questions.len(), 1);
        assert!(
            response
                .missing
                .iter()
                .any(|issue| issue.code == "security_approach_missing")
        );
    }

    #[test]
    fn review_blocks_inconsistent_auth_and_oversized_slice() {
        let mut request = ready_request();
        request.signals.touches_authorization = true;
        request.concerns.authorization.applicable = false;
        request.concerns.authorization.reason = Some("Not needed.".into());
        request.proposed_slices[0].estimated_minutes = 180;

        let response = review_request(PlanKind::Task, request);

        assert_eq!(response.decision, ReviewDecision::Blocked);
        assert!(
            response
                .pushback
                .iter()
                .any(|issue| issue.code == "authorization_inconsistent")
        );
        assert!(
            response
                .pushback
                .iter()
                .any(|issue| issue.code == "slice_1_too_large")
        );
    }

    #[test]
    fn review_blocks_bugfix_without_red_proof() {
        let mut request = ready_request();
        request.signals.bugfix = true;
        request.concerns.bugfix_red.applicable = false;
        request.concerns.bugfix_red.reason = Some("We can skip this.".into());

        let response = review_request(PlanKind::Task, request);

        assert_eq!(response.decision, ReviewDecision::Blocked);
        assert!(
            response
                .pushback
                .iter()
                .any(|issue| issue.code == "bugfix_red_inconsistent")
        );
    }

    #[test]
    fn review_request_requires_declared_signals_and_concerns() {
        let invalid = serde_json::from_str::<ReviewRequest>(
            r#"
            {
              "goal": "Missing required structures."
            }
            "#,
        );

        assert!(invalid.is_err());
    }

    #[test]
    fn review_request_rejects_unknown_fields() {
        let invalid = serde_json::from_str::<ReviewRequest>(
            r#"
            {
              "goal": "Test unknown field handling.",
              "facts": [],
              "constraints": [],
              "acceptance_criteria": ["It works."],
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
                "summary": "Do one thing.",
                "estimated_minutes": 30,
                "acceptance_criteria": ["Still works."]
              }],
              "concerns": {
                "rollback": {"applicable": true, "approach": "Revert the commit."},
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
              "surprise": true
            }
            "#,
        );

        assert!(invalid.is_err());
    }
}
