export type Vec3 = { x: number; y: number; z: number };

export type PreviewModel = {
  schema_version: string;
  document_kind: string;
  data_boundary: Record<string, string>;
  project: {
    id: string;
    name: string;
    description: string;
    units: Record<string, string>;
  };
  analysis_status: {
    mechanics: string;
    rule_check: string;
    professional_acceptance: string;
  };
  materials?: Array<{
    id: string;
    label: string;
    elastic_modulus: { value: number; unit: string };
    shear_modulus: { value: number; unit: string };
    thermal_expansion_coefficient?: { value: number; unit: string };
    provenance: string;
  }>;
  nodes: Array<{ id: string; label: string; position: Vec3; provenance: string }>;
  pipe_segments: Array<{
    id: string;
    label: string;
    from: string;
    to: string;
    section: Record<string, { value: number; unit: string }>;
    material: string;
    provenance: string;
  }>;
  supports: Array<{ id: string; label: string; node: string; restraints: string[]; provenance: string }>;
  components: Array<{ id: string; label: string; kind: string; node: string; provenance: string }>;
  load_cases: Array<{
    id: string;
    label: string;
    kind: string;
    status: string;
    provenance: string;
    primitive_loads?: Array<Record<string, unknown>>;
  }>;
  combinations?: Array<{
    id: string;
    label: string;
    basis: string;
    terms: Array<{ load_case: string; factor: number }>;
    provenance: string;
  }>;
  diagnostics: Diagnostic[];
};

export type Diagnostic = {
  id?: string;
  code: string;
  severity: "info" | "warning" | "error" | "blocking";
  message: string;
  source?: string;
  affected_refs?: string[];
};

export type KnowledgeRecord = {
  id: string;
  kind: string;
  title: string;
  summary: string;
  affected_refs: string[];
  provenance: string;
  status: string;
};

export type DesignKnowledge = {
  schema_version: string;
  document_kind: string;
  knowledge_set_id: string;
  model_ref: string;
  records: KnowledgeRecord[];
  diagnostics: Diagnostic[];
};

export type MechanicsResult = {
  schema_version: string;
  document_kind: string;
  run_id: string;
  model_ref: string;
  status: {
    mechanics: string;
    rule_check: string;
    professional_acceptance: string;
  };
  summary: Record<string, unknown> & {
    max_displacement?: { value: number; unit: string; location_ref: string; result_ref: string } | null;
    max_open_formula_stress?: { value: number; unit: string; location_ref: string; result_ref: string } | null;
  };
  results: Array<{
    id: string;
    kind: string;
    value: number;
    unit: string;
    entity_ref: string;
    basis_ref?: ResultBasisRef;
    source_result_refs?: string[];
    metadata?: {
      component: string;
      coordinate_system: string;
      location: string;
      basis: string;
      sign_convention: string;
    };
  }>;
  diagnostics: Diagnostic[];
};

export type ResultBasisRef = {
  ref_type: "load_case" | "combination" | string;
  ref_id: string;
};

export type SelectedReviewTarget =
  | { target_type: "result"; id: string }
  | { target_type: "diagnostic"; id: string }
  | { target_type: "model_entity"; id: string };

export type ResultInterpretation = {
  result_id: string;
  family: string;
  entity_ref: string;
  value_label: string;
  component: string;
  coordinate_system: string;
  location: string;
  recovery_basis: string;
  sign_convention: string;
  source_result_refs: string[];
  linked_diagnostics: Diagnostic[];
  linked_knowledge: KnowledgeRecord[];
  source_run: {
    run_id: string;
    model_ref: string;
    audit_ref: string;
    result_hash_count: number;
    envelope_hash_available: boolean;
  };
  endpoint_pair?: {
    entity_ref: string;
    component: string;
    coordinate_system: string;
    recovery_basis: string;
    values: Array<{
      result_id: string;
      location: string;
      value_label: string;
      sign_convention: string;
    }>;
  };
  professional_boundary: string;
};

export type DiagnosticInterpretation = {
  diagnostic_id: string;
  code: string;
  severity: Diagnostic["severity"];
  source: string;
  message: string;
  affected_refs: string[];
  linked_results: Array<{
    id: string;
    kind: string;
    entity_ref: string;
    value_label: string;
  }>;
  linked_knowledge: KnowledgeRecord[];
  review_explanation: string;
  professional_boundary: string;
};

export type MechanicsGap = {
  id: string;
  capability: string;
  status: "implemented" | "deferred" | "not_implemented" | "requires_private_inputs";
  review_note: string;
};

export type ObjectRef = {
  object_type: string;
  ref: string;
};

export type AnalysisRunEnvelope = {
  schema_version: string;
  deliverable_id: "DEL-14-02";
  package_id: "PKG-14";
  scope_item: string;
  objectives: string[];
  run_contract_status: Record<string, string>;
  analysis_run: {
    run_id: string;
    run_name: string;
    run_kind: string;
    model_state_ref: ObjectRef;
    load_basis_refs: ObjectRef[];
    result_refs: Array<{
      result_ref: ObjectRef;
      result_family: string;
      hash_refs: Array<{
        algorithm: "sha256";
        canonicalization: string;
        payload_ref: ObjectRef;
        payload_scope: string;
        value: string;
      }>;
      privacy_classification: string;
    }>;
    hashes: Array<{
      algorithm: "sha256";
      canonicalization: string;
      payload_ref: ObjectRef;
      payload_scope: string;
      value: string;
    }>;
    analysis_status: string[];
    reproducibility: {
      input_manifest_refs: ObjectRef[];
      determinism_notes: string[];
      unresolved_tbd: string[];
    };
    immutability_policy: {
      run_record_is_read_only: boolean;
      mutation_policy: string;
      new_run_required_for_change: boolean;
      hash_invalidates_external_acceptance: boolean;
    };
    professional_boundary: Record<string, boolean>;
  };
};

export type AgentProposal = {
  schema_version: string;
  document_kind: string;
  proposal_id: string;
  prompt: string;
  operation: {
    operation_id: string;
    operation_kind: string;
    operation_status: string;
    affected_entity_ids: string[];
    changes: Array<{ change_id: string; change_kind: string; target_ref: string; before: string; after: string }>;
  };
  rationale: string;
  assumptions: string[];
  validation: Record<string, string>;
  audit_boundary: {
    requires_user_acceptance: boolean;
    mutates_accepted_model_state: boolean;
    acceptance_recorded_as_review_only: boolean;
  };
  professional_boundary: Record<string, boolean>;
};

export type EntityRef = {
  id: string;
  type: "project" | "node" | "pipe" | "support" | "component" | "load" | "diagnostic";
};

export type LocalStorageCapability = {
  engine: "SQLite" | string;
  bundled: boolean;
  fts5_available: boolean;
  network_required: boolean;
  daemon_required: boolean;
  telemetry_enabled: boolean;
  path_policy: string;
  large_file_policy: string;
  database_path: string;
  compile_options: string[];
};

export type LocalProjectSummary = {
  project_id: string;
  project_name: string;
  database_path: string;
  storage_mode: "local_sqlite" | string;
  migration_status: "current" | string;
  fts_indexed: boolean;
  copied_external_files: boolean;
  message: string;
};

export type LocalProjectEnvelope = {
  summary: LocalProjectSummary;
  model: PreviewModel;
};
