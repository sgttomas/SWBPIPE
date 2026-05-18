//! Viewport editor command-intent support.
//!
//! This crate models bounded in-memory viewport state for the first
//! centerline-editor slice. It prepares deterministic application-service
//! command intents and view primitives for callers such as a future
//! Tauri/React/Three.js GUI. It does not render Three.js scenes, parse project
//! files, mutate persisted project payloads directly, access host resources,
//! provide protected engineering data, or make professional/code-compliance
//! claims.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrimitiveType {
    Node,
    PipeRun,
    BendArc,
    BranchSymbol,
    ValveSymbol,
    FlangeSymbol,
    ReducerSymbol,
    ExpansionJointSymbol,
    SupportSymbol,
    Label,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandType {
    CreateNode,
    ConnectPipeRun,
    InsertBend,
    InsertBranchSymbol,
    InsertComponentSymbol,
    SelectEntities,
    ClearSelection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationState {
    Valid,
    BlockedByMissingData,
    BlockedByUnitMismatch,
    PendingServiceValidation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticClass {
    SolveBlocking,
    RuleCheckBlocking,
    ProvenanceWarning,
    AssumptionWarning,
    NonlinearWarning,
    IpBoundaryWarning,
    UnitWarning,
    GuiStateWarning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Select,
    CreateNode,
    ConnectPipeRun,
    InsertBend,
    InsertBranchSymbol,
    InsertComponentSymbol,
    PanOrbit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    pub ref_type: String,
    pub ref_id: String,
}

impl Reference {
    pub fn new(ref_type: impl Into<String>, ref_id: impl Into<String>) -> Self {
        Self {
            ref_type: ref_type.into(),
            ref_id: ref_id.into(),
        }
    }

    fn is_complete(&self) -> bool {
        !self.ref_type.trim().is_empty() && !self.ref_id.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    pub source_name: String,
    pub source_location: String,
    pub source_license: String,
    pub contributor: String,
    pub contributor_certification: String,
    pub redistribution_status: String,
    pub review_status: String,
    pub privacy_classification: String,
}

impl Provenance {
    fn is_complete(&self) -> bool {
        !self.source_name.trim().is_empty()
            && !self.source_location.trim().is_empty()
            && !self.source_license.trim().is_empty()
            && !self.contributor.trim().is_empty()
            && !self.contributor_certification.trim().is_empty()
            && !self.redistribution_status.trim().is_empty()
            && !self.review_status.trim().is_empty()
            && !self.privacy_classification.trim().is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfessionalBoundary {
    pub human_review_required: bool,
    pub software_makes_compliance_claim: bool,
    pub software_makes_certification_claim: bool,
    pub software_makes_sealing_claim: bool,
    pub software_makes_approval_claim: bool,
    pub software_makes_authentication_claim: bool,
}

impl ProfessionalBoundary {
    pub fn project_default() -> Self {
        Self {
            human_review_required: true,
            software_makes_compliance_claim: false,
            software_makes_certification_claim: false,
            software_makes_sealing_claim: false,
            software_makes_approval_claim: false,
            software_makes_authentication_claim: false,
        }
    }

    fn preserves_boundary(self) -> bool {
        self.human_review_required
            && !self.software_makes_compliance_claim
            && !self.software_makes_certification_claim
            && !self.software_makes_sealing_claim
            && !self.software_makes_approval_claim
            && !self.software_makes_authentication_claim
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewportStatus {
    pub frontend_app_shell: String,
    pub viewport_renderer: String,
    pub durable_state_mutation: String,
    pub transient_state_policy: String,
    pub command_transport: String,
    pub dependency_versions: String,
    pub professional_boundary: ProfessionalBoundary,
}

impl ViewportStatus {
    pub fn bounded_contract() -> Self {
        Self {
            frontend_app_shell: "TBD".to_string(),
            viewport_renderer: "Three_js_runtime_integration_TBD".to_string(),
            durable_state_mutation: "application_service_command_intents_only".to_string(),
            transient_state_policy:
                "camera_hover_selection_drag_snap_are_not_persisted_project_payload".to_string(),
            command_transport: "TBD".to_string(),
            dependency_versions: "TBD".to_string(),
            professional_boundary: ProfessionalBoundary::project_default(),
        }
    }

    pub fn is_bounded(&self) -> bool {
        self.frontend_app_shell == "TBD"
            && self.viewport_renderer == "Three_js_runtime_integration_TBD"
            && self.durable_state_mutation == "application_service_command_intents_only"
            && self
                .transient_state_policy
                .contains("not_persisted_project_payload")
            && self.command_transport == "TBD"
            && self.dependency_versions == "TBD"
            && self.professional_boundary.preserves_boundary()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub diagnostic_id: String,
    pub code: String,
    pub class: DiagnosticClass,
    pub severity: DiagnosticSeverity,
    pub affected_ref: Reference,
    pub message: String,
    pub remediation: String,
    pub provenance: Provenance,
}

impl Diagnostic {
    fn gui_warning(
        id: impl Into<String>,
        code: impl Into<String>,
        affected_ref: Reference,
        message: impl Into<String>,
    ) -> Self {
        Self {
            diagnostic_id: id.into(),
            code: code.into(),
            class: DiagnosticClass::GuiStateWarning,
            severity: DiagnosticSeverity::Warning,
            affected_ref,
            message: message.into(),
            remediation: "Route the command intent through application-service validation before durable mutation.".to_string(),
            provenance: invented_provenance(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ViewPrimitive {
    pub primitive_id: String,
    pub primitive_type: PrimitiveType,
    pub source_ref: Reference,
    pub points: Vec<Point3>,
    pub unit: String,
    pub diagnostic_refs: Vec<Reference>,
    pub provenance: Provenance,
}

impl ViewPrimitive {
    fn ordering_key(&self) -> (PrimitiveType, &str) {
        (self.primitive_type, self.primitive_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandIntent {
    pub intent_id: String,
    pub command_type: CommandType,
    pub target_ref: Reference,
    pub payload_refs: Vec<Reference>,
    pub unit_policy: String,
    pub reversible: bool,
    pub validation_state: ValidationState,
    pub diagnostic_refs: Vec<Reference>,
    pub provenance: Provenance,
}

impl CommandIntent {
    fn ordering_key(&self) -> (CommandType, &str) {
        (self.command_type, self.intent_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ViewportSession {
    pub session_id: String,
    pub model_ref: Reference,
    pub active_tool: Tool,
    pub view_primitives: Vec<ViewPrimitive>,
    pub selected_refs: Vec<Reference>,
    pub command_intents: Vec<CommandIntent>,
    pub diagnostics: Vec<Diagnostic>,
    pub provenance: Provenance,
}

impl ViewportSession {
    pub fn new(session_id: impl Into<String>, model_ref: Reference) -> Self {
        Self {
            session_id: session_id.into(),
            model_ref,
            active_tool: Tool::Select,
            view_primitives: Vec::new(),
            selected_refs: Vec::new(),
            command_intents: Vec::new(),
            diagnostics: Vec::new(),
            provenance: invented_provenance(),
        }
    }

    pub fn add_node_preview(&mut self, node_id: impl Into<String>, point: Point3, unit: &str) {
        let node_id = node_id.into();
        let source_ref = Reference::new("node", node_id.clone());
        self.view_primitives.push(ViewPrimitive {
            primitive_id: format!("VP-{}", node_id),
            primitive_type: PrimitiveType::Node,
            source_ref,
            points: vec![point],
            unit: unit.to_string(),
            diagnostic_refs: Vec::new(),
            provenance: invented_provenance(),
        });
    }

    pub fn create_node_intent(
        &mut self,
        intent_id: impl Into<String>,
        node_ref: Reference,
        point: Point3,
        unit: &str,
    ) -> CommandIntent {
        self.active_tool = Tool::CreateNode;
        let (validation_state, diagnostic_refs) =
            self.validate_point_intent(&node_ref, point, unit);
        let intent = CommandIntent {
            intent_id: intent_id.into(),
            command_type: CommandType::CreateNode,
            target_ref: self.model_ref.clone(),
            payload_refs: vec![node_ref],
            unit_policy: "unit_aware_domain_validation_required".to_string(),
            reversible: true,
            validation_state,
            diagnostic_refs,
            provenance: invented_provenance(),
        };
        self.command_intents.push(intent.clone());
        self.sort_outputs();
        intent
    }

    pub fn connect_pipe_run_intent(
        &mut self,
        intent_id: impl Into<String>,
        pipe_ref: Reference,
        start_ref: Reference,
        end_ref: Reference,
    ) -> CommandIntent {
        self.active_tool = Tool::ConnectPipeRun;
        let mut diagnostic_refs = Vec::new();
        let validation_state = if start_ref.is_complete()
            && end_ref.is_complete()
            && start_ref != end_ref
            && pipe_ref.is_complete()
        {
            ValidationState::PendingServiceValidation
        } else {
            let diagnostic_id = format!("VD-{:04}", self.diagnostics.len() + 1);
            diagnostic_refs.push(Reference::new("diagnostic", diagnostic_id.clone()));
            self.diagnostics.push(Diagnostic::gui_warning(
                diagnostic_id,
                "VIEWPORT_COMMAND_REQUIRES_SERVICE_VALIDATION",
                pipe_ref.clone(),
                "Pipe-run preview requires complete distinct endpoint refs and service validation.",
            ));
            ValidationState::BlockedByMissingData
        };

        let intent = CommandIntent {
            intent_id: intent_id.into(),
            command_type: CommandType::ConnectPipeRun,
            target_ref: pipe_ref,
            payload_refs: vec![start_ref, end_ref],
            unit_policy: "unit_aware_domain_validation_required".to_string(),
            reversible: true,
            validation_state,
            diagnostic_refs,
            provenance: invented_provenance(),
        };
        self.command_intents.push(intent.clone());
        self.sort_outputs();
        intent
    }

    pub fn insert_component_symbol_intent(
        &mut self,
        intent_id: impl Into<String>,
        component_ref: Reference,
        primitive_type: PrimitiveType,
    ) -> CommandIntent {
        self.active_tool = Tool::InsertComponentSymbol;
        let allowed_symbol = matches!(
            primitive_type,
            PrimitiveType::BendArc
                | PrimitiveType::BranchSymbol
                | PrimitiveType::ValveSymbol
                | PrimitiveType::FlangeSymbol
                | PrimitiveType::ReducerSymbol
                | PrimitiveType::ExpansionJointSymbol
                | PrimitiveType::SupportSymbol
        );
        let mut diagnostic_refs = Vec::new();
        let validation_state = if allowed_symbol && component_ref.is_complete() {
            ValidationState::PendingServiceValidation
        } else {
            let diagnostic_id = format!("VD-{:04}", self.diagnostics.len() + 1);
            diagnostic_refs.push(Reference::new("diagnostic", diagnostic_id.clone()));
            self.diagnostics.push(Diagnostic::gui_warning(
                diagnostic_id,
                "VIEWPORT_COMPONENT_DATA_MISSING",
                component_ref.clone(),
                "Component symbol intent requires a supported symbolic primitive and complete component reference.",
            ));
            ValidationState::BlockedByMissingData
        };

        let intent = CommandIntent {
            intent_id: intent_id.into(),
            command_type: match primitive_type {
                PrimitiveType::BendArc => CommandType::InsertBend,
                PrimitiveType::BranchSymbol => CommandType::InsertBranchSymbol,
                _ => CommandType::InsertComponentSymbol,
            },
            target_ref: component_ref,
            payload_refs: Vec::new(),
            unit_policy: "unit_aware_domain_validation_required".to_string(),
            reversible: true,
            validation_state,
            diagnostic_refs,
            provenance: invented_provenance(),
        };
        self.command_intents.push(intent.clone());
        self.sort_outputs();
        intent
    }

    pub fn select(&mut self, refs: Vec<Reference>) -> CommandIntent {
        self.active_tool = Tool::Select;
        self.selected_refs = refs
            .into_iter()
            .filter(|reference| reference.is_complete())
            .collect();
        self.selected_refs.sort_by(|left, right| {
            (&left.ref_type, &left.ref_id).cmp(&(&right.ref_type, &right.ref_id))
        });
        let intent = CommandIntent {
            intent_id: format!("VCI-{:04}", self.command_intents.len() + 1),
            command_type: CommandType::SelectEntities,
            target_ref: self.model_ref.clone(),
            payload_refs: self.selected_refs.clone(),
            unit_policy: "unit_aware_domain_validation_required".to_string(),
            reversible: false,
            validation_state: ValidationState::Valid,
            diagnostic_refs: Vec::new(),
            provenance: invented_provenance(),
        };
        self.command_intents.push(intent.clone());
        self.sort_outputs();
        intent
    }

    pub fn is_valid_contract_state(&self) -> bool {
        !self.session_id.trim().is_empty()
            && self.model_ref.is_complete()
            && self.provenance.is_complete()
            && self
                .command_intents
                .iter()
                .all(|intent| intent.unit_policy == "unit_aware_domain_validation_required")
            && self.diagnostics.iter().all(|diagnostic| {
                diagnostic.provenance.is_complete() && diagnostic.affected_ref.is_complete()
            })
            && self
                .view_primitives
                .iter()
                .all(|primitive| primitive.provenance.is_complete())
    }

    fn validate_point_intent(
        &mut self,
        affected_ref: &Reference,
        point: Point3,
        unit: &str,
    ) -> (ValidationState, Vec<Reference>) {
        if point.is_finite() && !unit.trim().is_empty() && unit != "TBD" {
            return (ValidationState::PendingServiceValidation, Vec::new());
        }

        let diagnostic_id = format!("VD-{:04}", self.diagnostics.len() + 1);
        self.diagnostics.push(Diagnostic::gui_warning(
            diagnostic_id.clone(),
            "VIEWPORT_UNIT_MISMATCH",
            affected_ref.clone(),
            "Viewport coordinate intent requires finite coordinates and explicit length unit.",
        ));
        (
            ValidationState::BlockedByUnitMismatch,
            vec![Reference::new("diagnostic", diagnostic_id)],
        )
    }

    fn sort_outputs(&mut self) {
        self.view_primitives
            .sort_by(|left, right| left.ordering_key().cmp(&right.ordering_key()));
        self.command_intents
            .sort_by(|left, right| left.ordering_key().cmp(&right.ordering_key()));
        self.diagnostics
            .sort_by(|left, right| left.diagnostic_id.cmp(&right.diagnostic_id));
    }
}

pub fn invented_provenance() -> Provenance {
    Provenance {
        source_name: "OpenPipeStress invented GUI fixture".to_string(),
        source_location: "core/gui/viewport_editor".to_string(),
        source_license: "project_fixture".to_string(),
        contributor: "OpenPipeStress DEV-001".to_string(),
        contributor_certification:
            "Invented non-engineering interaction data; not copied from protected standards, vendor catalogs, or private projects."
                .to_string(),
        redistribution_status: "invented_non_engineering_example".to_string(),
        review_status: "accepted".to_string(),
        privacy_classification: "invented_public_example".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_status_preserves_bounded_contract() {
        let status = ViewportStatus::bounded_contract();
        assert!(status.is_bounded());
        assert!(status.professional_boundary.human_review_required);
        assert!(!status.professional_boundary.software_makes_compliance_claim);
        assert_eq!(status.frontend_app_shell, "TBD");
    }

    #[test]
    fn node_intent_requires_unit_aware_service_validation() {
        let mut session = ViewportSession::new("session-001", Reference::new("model", "M-001"));
        let intent = session.create_node_intent(
            "VCI-0001",
            Reference::new("node", "N-001"),
            Point3::new(1.0, 2.0, 3.0),
            "m",
        );

        assert_eq!(intent.command_type, CommandType::CreateNode);
        assert_eq!(intent.unit_policy, "unit_aware_domain_validation_required");
        assert_eq!(
            intent.validation_state,
            ValidationState::PendingServiceValidation
        );
        assert!(intent.reversible);
        assert!(session.is_valid_contract_state());
    }

    #[test]
    fn invalid_point_becomes_diagnostic_not_silent_default() {
        let mut session = ViewportSession::new("session-001", Reference::new("model", "M-001"));
        let intent = session.create_node_intent(
            "VCI-0001",
            Reference::new("node", "N-001"),
            Point3::new(f64::NAN, 2.0, 3.0),
            "TBD",
        );

        assert_eq!(
            intent.validation_state,
            ValidationState::BlockedByUnitMismatch
        );
        assert_eq!(session.diagnostics.len(), 1);
        assert_eq!(session.diagnostics[0].code, "VIEWPORT_UNIT_MISMATCH");
        assert_eq!(session.diagnostics[0].severity, DiagnosticSeverity::Warning);
    }

    #[test]
    fn pipe_run_intent_keeps_durable_mutation_as_command_intent() {
        let mut session = ViewportSession::new("session-001", Reference::new("model", "M-001"));
        let intent = session.connect_pipe_run_intent(
            "VCI-0002",
            Reference::new("element", "P-001"),
            Reference::new("node", "N-001"),
            Reference::new("node", "N-002"),
        );

        assert_eq!(intent.command_type, CommandType::ConnectPipeRun);
        assert_eq!(intent.target_ref, Reference::new("element", "P-001"));
        assert_eq!(intent.payload_refs.len(), 2);
        assert_eq!(
            intent.validation_state,
            ValidationState::PendingServiceValidation
        );
        assert!(session.is_valid_contract_state());
    }

    #[test]
    fn component_symbol_intents_are_limited_to_simple_symbols() {
        let mut session = ViewportSession::new("session-001", Reference::new("model", "M-001"));
        let bend = session.insert_component_symbol_intent(
            "VCI-0003",
            Reference::new("component", "BEND-001"),
            PrimitiveType::BendArc,
        );
        let invalid = session.insert_component_symbol_intent(
            "VCI-0004",
            Reference::new("component", ""),
            PrimitiveType::PipeRun,
        );

        assert_eq!(bend.command_type, CommandType::InsertBend);
        assert_eq!(
            bend.validation_state,
            ValidationState::PendingServiceValidation
        );
        assert_eq!(
            invalid.validation_state,
            ValidationState::BlockedByMissingData
        );
        assert_eq!(
            session.diagnostics[0].code,
            "VIEWPORT_COMPONENT_DATA_MISSING"
        );
    }

    #[test]
    fn selection_is_transient_and_deterministically_ordered() {
        let mut session = ViewportSession::new("session-001", Reference::new("model", "M-001"));
        let intent = session.select(vec![
            Reference::new("node", "N-002"),
            Reference::new("element", "P-001"),
            Reference::new("node", "N-001"),
        ]);

        assert_eq!(intent.command_type, CommandType::SelectEntities);
        assert!(!intent.reversible);
        assert_eq!(
            session.selected_refs,
            vec![
                Reference::new("element", "P-001"),
                Reference::new("node", "N-001"),
                Reference::new("node", "N-002"),
            ]
        );
    }
}
