export type Vec3 = { x: number; y: number; z: number };

export type QuantityValue = { value: number; unit: string };

export type PreviewComponent = {
  id: string;
  label: string;
  kind: string;
  node: string;
  provenance: string;
  geometry?: {
    bend_radius?: QuantityValue;
    bend_angle?: QuantityValue;
    bend_plane_orientation?: string;
    /** User-entered pipe reference naming the realized curved-bend span
     * (DEC-070). Starts absent; never defaulted or inferred. */
    bend_pipe_ref?: string;
    bend_geometry_source_reference?: string;
    branch_header_pipe_ref?: string;
    branch_branch_pipe_ref?: string;
    branch_run_size?: QuantityValue;
    branch_header_size?: QuantityValue;
    branch_connection_angle?: QuantityValue;
    branch_connection_type?: string;
    branch_reinforcement_area?: QuantityValue;
    branch_reinforcement_reference?: string;
    branch_geometry_source_reference?: string;
    rigid_pipe_ref?: string;
    rigid_body_length?: QuantityValue;
    end_a_size?: QuantityValue;
    end_b_size?: QuantityValue;
    weight?: QuantityValue;
    center_of_gravity?: { x: number; y: number; z: number; unit: string };
    connection_end_a_reference?: string;
    connection_end_b_reference?: string;
    stiffness_behavior_reference?: string;
    rigid_component_source_reference?: string;
    expansion_joint_pipe_ref?: string;
    effective_area?: QuantityValue;
    movement_limit?: QuantityValue;
    hardware_reference?: string;
    manufacturer_reference?: string;
    pressure_thrust_reference?: string;
    expansion_joint_source_reference?: string;
  };
  modifiers?: {
    sif_user_value?: QuantityValue;
    branch_header_sif_user_value?: QuantityValue;
    branch_branch_sif_user_value?: QuantityValue;
    flexibility_factor_user_value?: QuantityValue;
    stiffness_scaling_user_value?: QuantityValue;
    linear_stiffness_user_value?: QuantityValue;
    rotational_stiffness_user_value?: QuantityValue;
    axial_stiffness_user_value?: QuantityValue;
    lateral_stiffness_user_value?: QuantityValue;
    angular_stiffness_user_value?: QuantityValue;
    torsional_stiffness_user_value?: QuantityValue;
    source_reference?: string;
  };
  mechanics_interface?: {
    solver_consumption?: string;
    rule_check_consumption?: string;
  };
  completeness?: Array<{
    finding_id?: string;
    status: string;
    diagnostic_code: string;
    missing_field_kinds?: string[];
  }>;
};

/** User-entered seismic static-equivalent generation inputs, mirroring the
 * preview-engine input surface (`core/product_physics`
 * `SeismicGenerationInput`). All slots optional and user-entered; absence is
 * absence, not zero. */
export type SeismicEquivalentStaticGenerationInput = {
  gravity_acceleration?: QuantityValue;
  g_factor_x?: QuantityValue;
  g_factor_y?: QuantityValue;
  g_factor_z?: QuantityValue;
};

/** User-entered wind static-equivalent generation inputs, mirroring the
 * preview-engine input surface (`core/product_physics`
 * `WindGenerationInput`). `direction` is a global axis token
 * (`global_x` | `global_y` | `global_z`); `exposed_pipe_refs` are
 * user-marked exposed spans by pipe id. */
export type WindEquivalentStaticGenerationInput = {
  pressure?: QuantityValue;
  shape_factor?: QuantityValue;
  direction?: string;
  exposed_pipe_refs?: string[];
  exposed_spans?: Array<{ pipe_ref: string; start_fraction: QuantityValue; end_fraction: QuantityValue }>;
};

/** User-entered static-equivalent occasional-load generation inputs
 * (DEC-068 item 2), mirroring `core/product_physics`
 * `EquivalentStaticGenerationInput`. */
export type EquivalentStaticGenerationInput = {
  seismic?: SeismicEquivalentStaticGenerationInput;
  wind?: WindEquivalentStaticGenerationInput;
  provenance?: string;
};

export type PressureRegionInput = {
  id: string;
  member_pipe_ids: string[];
  pressure_basis: string;
  pressure: QuantityValue;
  terminals: Array<{
    node_ref: string;
    closure_transfer: "transfers_to_wall" | "separately_supported_or_compensated";
    provenance: string;
  }>;
  provenance: string;
};

/* ---------------------------------------------------------------------------
 * Load/reference-state namespace, model 0.4.0 (T1). Shapes of record:
 * core/product_physics/src/case_state/input.rs (closed, tagged DTOs) and
 * schemas/load_reference_state.schema.json. Every union is tagged by its
 * `kind` (or `definition`) discriminant. A key that Rust reads as
 * `Authored<T>` or `Option<T>` is `?: T | null` here: absent and explicit null
 * are different authored facts and both are carried unchanged. The desktop
 * never projects these records: models and results pass through session state,
 * persistence, IPC and the browser-fixture path as whole JSON values.
 * ------------------------------------------------------------------------ */
/** Closed quantity: exactly `value` and `unit` in this namespace. */
export type LoadStateQuantity = { value: number; unit: string };
export type GeometryRefInput = { kind: "authored_model_geometry" };
export type ReferenceBasisInput =
  | { kind: "temperature_reference"; installation_temperature: LoadStateQuantity }
  | { kind: "direct_strain_reference" };
/** Absence of fit is the explicit `none` selection, never an inferred default. */
export type FitReferenceInput =
  | { kind: "none" }
  | { kind: "natural_length_change"; length_change: LoadStateQuantity }
  | { kind: "fit_strain"; strain: LoadStateQuantity };
export type MemberReferenceInput = {
  pipe_ref: string;
  basis: ReferenceBasisInput;
  fit: FitReferenceInput;
  provenance: string;
};
/** Model-level reference configuration: the stress-free installed state. */
export type ReferenceConfigurationInput = {
  id: string;
  label?: string | null;
  geometry_ref: GeometryRefInput;
  member_references: MemberReferenceInput[];
  provenance: string;
};
export type CoefficientPointInput = { temperature: LoadStateQuantity; coefficient: LoadStateQuantity };
export type DilationPointInput = { temperature: LoadStateQuantity; dilation: LoadStateQuantity };
export type SecantDataInput =
  | { kind: "constant"; coefficient: LoadStateQuantity }
  | { kind: "table"; interpolation: "linear_coefficient"; points: CoefficientPointInput[] };
export type DilationDataInput = { kind: "table"; interpolation: "linear_dilation"; points: DilationPointInput[] };
export type CoefficientTableInput = { kind: "table"; interpolation: "linear_coefficient"; points: CoefficientPointInput[] };
/** User-owned expansion definition carried by its material record. */
export type ExpansionLawInput =
  | { definition: "engineering_secant"; id: string; datum_temperature: LoadStateQuantity; data: SecantDataInput; provenance: string }
  | { definition: "engineering_dilation"; id: string; datum_temperature: LoadStateQuantity; data: DilationDataInput; provenance: string }
  | { definition: "differential_per_datum_length"; id: string; datum_temperature: LoadStateQuantity; data: CoefficientTableInput; provenance: string }
  | { definition: "logarithmic_per_current_length"; id: string; datum_temperature: LoadStateQuantity; data: CoefficientTableInput; provenance: string };
export type MaterialSelectionInput =
  | { kind: "explicit_base_properties"; material_ref: string; applicability_reference: string }
  | { kind: "exact_point"; material_ref: string; point_ref: string }
  | { kind: "temperature_interpolation"; material_ref: string; temperature: LoadStateQuantity; interpolation: "piecewise_linear"; extrapolation: "forbidden" };
export type ThermalStateInput =
  | { kind: "unchanged_reference"; provenance: string }
  | { kind: "explicit_interval_strain"; strain: LoadStateQuantity; interval_reference: string; provenance: string }
  | { kind: "constant_alpha_interval"; coefficient: LoadStateQuantity; temperature_change: LoadStateQuantity; coefficient_meaning: "engineering_interval"; provenance: string }
  | { kind: "free_length_state"; expansion_law_ref: string };
export type AnalysisBasisOverrideInput = { reason: string; provenance: string };
export type ElementStateInput = {
  pipe_ref: string;
  /** Actual physical temperature; never inferred from a property point. */
  operating_temperature?: LoadStateQuantity | null;
  material_selection: MaterialSelectionInput;
  thermal_state: ThermalStateInput;
  analysis_basis_override?: AnalysisBasisOverrideInput | null;
  /** Reviewed M21 mass-state seam; parsed so it can be refused explicitly. */
  mass_state_ref?: string | null;
};
export type PositionSourceInput =
  | { kind: "entered"; value: LoadStateQuantity }
  | { kind: "predecessor_value"; case_ref: string; state_hash: string; support_ref: string; dof: string };
export type LockedComponentInput = { dof: string; position_source: PositionSourceInput };
export type ParticipationInput =
  | { kind: "active_model_device" }
  | { kind: "inactive" }
  | { kind: "locked_equivalent_support"; components: LockedComponentInput[] };
export type MotionInput = { dof: string; value: LoadStateQuantity; meaning: "absolute_reference_displacement" };
export type DeviceReferenceInput =
  | { kind: "force_at_reference"; reference_position: LoadStateQuantity; force: LoadStateQuantity }
  | { kind: "unloaded_reference"; reference_position: LoadStateQuantity };
export type SupportStateInput = {
  support_ref: string;
  participation: ParticipationInput;
  boundary_motion?: MotionInput[] | null;
  base_motion?: MotionInput[] | null;
  device_reference?: DeviceReferenceInput | null;
};
/** Complete inclusion entry for one stored ordinary primitive. */
export type LoadSourceInput = { source_ref: string; factor: number };
export type HistoryInput = { kind: "independent_equilibrium" };
/** Case-owned resolved-state request (`openpipestress.load_reference_state/1.0.0`). */
export type AnalysisStateInput = {
  contract: string;
  reference_configuration_ref: string;
  element_states: ElementStateInput[];
  support_states: SupportStateInput[];
  load_sources: LoadSourceInput[];
  history: HistoryInput;
  provenance: string;
};

/* Published result evidence: `contract_evidence` of load-reference-1 and
 * load-reference-source-1 (schemas/load_reference_state.schema.json
 * LoadReferenceContractEvidence and LoadReferenceStateRecord). Readers:
 * features/results/loadReferenceEvidence.ts, loadReferenceSourceEvidence.ts. */
export type LoadReferenceConsumedMaterialPoint = { point_id: string; temperature_k: number | null; E_pa: number; nu: number; retained_G_ignored: boolean };
export type LoadReferenceLawSegment = { use: "interpolation_sample" | "integration_interval"; lower_index: number; upper_index: number; start_k: number; end_k: number };
export type LoadReferenceMember = {
  pipe_id: string;
  material_id: string;
  material_selection_kind: "explicit_base_properties" | "exact_point" | "temperature_interpolation";
  consumed_material_points: LoadReferenceConsumedMaterialPoint[];
  interpolation_fraction: number | null;
  applicability_reference: string | null;
  analysis_basis_override: AnalysisBasisOverrideInput | null;
  selected_E_pa: number;
  selected_nu: number;
  derived_G_pa: number;
  G_basis: string;
  retained_G_ignored: boolean;
  operating_temperature_k: number | null;
  material_selection_temperature_k: number | null;
  reference_basis: "temperature_reference" | "direct_strain_reference";
  installation_temperature_k: number | null;
  thermal_definition: string;
  expansion_law_id: string | null;
  coefficient_datum_k: number | null;
  consumed_law_point_indices: number[];
  consumed_law_segments: LoadReferenceLawSegment[];
  consulted_law_point_indices: number[];
  consulted_law_segments: LoadReferenceLawSegment[];
  installation_datum_stretch: number | null;
  operating_datum_stretch: number | null;
  thermal_strain: number;
  thermal_stretch: number;
  fit_strain: number;
  fit_stretch: number;
  total_eigenstrain: number;
  eigenstrain_composition: string;
  fit_kind: "none" | "natural_length_change" | "fit_strain";
  fit_input: { length_change_m: number } | { strain: number } | null;
  reference_length_m: number;
};
export type LoadReferenceSupportComponent = {
  support_id: string; node_id: string; dof: "UX" | "UY" | "UZ" | "RX" | "RY" | "RZ"; global_dof: number;
  law_kind: "rigid_prescribed"; prescribed_value: number; unit: "m" | "rad";
  meaning: "absolute_reference_displacement"; physical_state_source: "support_state.boundary_motion";
};
export type LoadReferenceContribution =
  | { source_id: string; owner_kind: "stored_primitive"; classification: "ordinary_applied"; factor: number; category: string; dimension: string; authored_normalized_magnitude: number; applied_magnitude: number }
  | { source_id: string; owner_kind: "resolved_member_state"; classification: "eigenstrain"; consumed_input_refs: string[]; value: number }
  | { source_id: string; owner_kind: "support_state"; classification: "prescribed_boundary"; value: number }
  | { source_id: string; owner_kind: "pressure_region"; classification: "pressure_eigen_and_closure"; factor: null };
export type LoadReferenceExcludedSource = { source_id: string; owner_kind: "stored_primitive"; classification: "excluded"; category: string; reason: string };
export type LoadReferenceStateRecord = {
  load_case_id: string;
  contract: "openpipestress.load_reference_state/1.0.0";
  profile: "resolved_straight_load_state_v1";
  reference_configuration_id: string;
  provenance: string;
  reference_geometry: { kind: "authored_model_geometry"; projection_sha256: string };
  history: { kind: "independent_equilibrium" };
  solve: { requested_mode: "sparse_interactive" | "dense_scrutiny"; recovery_method: string; boundary: string; eigenload: string };
  source_recovery: { status: "not_joined"; code: "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED" } | { status: "selected"; method: "retained_source_blocks_exact_v1" };
  members: LoadReferenceMember[];
  support_components: LoadReferenceSupportComponent[];
  contributions: LoadReferenceContribution[];
  excluded_sources: LoadReferenceExcludedSource[];
};
/** Closed evidence namespace; `pressure` and `exact_cases` are the inherited
 * physics-1 statements with the resolved member pair. */
export type LoadReferenceContractEvidence = {
  pressure: Array<Record<string, unknown>>;
  connector: [];
  exact_cases: Array<Record<string, unknown>>;
  load_reference_states: LoadReferenceStateRecord[];
};

export type PreviewModel = {
  pressure_contract?: { version: string; mode: string };
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
    shear_modulus?: { value: number; unit: string };
    constitutive_basis?: string;
    poisson_ratio?: QuantityValue;
    thermal_expansion_coefficient?: { value: number; unit: string };
    temperature_points?: Array<{
      id: string;
      temperature?: QuantityValue;
      elastic_modulus?: QuantityValue;
      shear_modulus?: QuantityValue;
      poisson_ratio?: QuantityValue;
      thermal_expansion_coefficient?: QuantityValue;
      provenance?: string;
    }>;
    /** Model 0.4.0: user-owned expansion laws of this material. */
    expansion_laws?: ExpansionLawInput[] | null;
    provenance: string;
  }>;
  sections?: Array<{
    id: string;
    name: string;
    section_type: "pipe" | "rigid" | "custom" | "TBD" | string;
    properties: {
      outside_diameter?: { value: number; unit: string };
      wall_thickness?: { value: number; unit: string };
      [key: string]: { value: number; unit: string } | undefined;
    };
    provenance: string | Record<string, unknown>;
  }>;
  nodes: Array<{
    id: string;
    label: string;
    position: Vec3;
    provenance: string;
  }>;
  pipe_segments: Array<{
    id: string;
    label: string;
    from: string;
    to: string;
    /** Per-span section quantities. Besides `outside_diameter` and
     * `wall_thickness`, the record carries optional user-entered slots from
     * the preview-engine input surface (`core/product_physics`
     * `PipeSectionInput`), including `mill_tolerance` — the user-entered
     * absolute mill-tolerance thickness reduction (length). Absence means no
     * reduction; absence is not a default value of zero. */
    section: Record<string, { value: number; unit: string }>;
    section_ref?: string;
    material: string;
    y_reference?: Vec3;
    provenance: string;
  }>;
  supports: Array<{
    boundary_association?: {
      boundary_id: string;
      kind: "equipment" | "equipment_nozzle";
      equipment_reference: string;
      nozzle_reference?: string;
      coordinate_system: "global";
    };
    nonlinear?: {
      behavior: string;
      dof: string;
      initial_state?: string;
      active_when?: string;
      contact_when?: string;
      closes_when?: string;
      gap?: QuantityValue;
      friction_coefficient?: QuantityValue;
      normal_reaction?: QuantityValue;
      normal_reaction_source?: { support_ref: string; dof: string };
    };
    id: string;
    label: string;
    node: string;
    family?: string;
    restraints: string[];
    stiffness?: {
      dof: string;
      value: QuantityValue;
    };
    hanger?: {
      hanger_type?: string;
      stiffness?: {
        dof: string;
        value: QuantityValue;
      };
      installed_load?: QuantityValue;
      cold_load?: QuantityValue;
      hot_load?: QuantityValue;
      constant_load?: QuantityValue;
      travel_range?: QuantityValue;
      movement_limit?: QuantityValue;
      manufacturer_reference?: string;
      source_reference?: string;
      load_side_review_reference?: string;
      mechanics_consumption?: string;
    };
    properties?: {
      linear_stiffness?: { value: number; unit: string };
      [key: string]: { value: number; unit: string } | undefined;
    };
    provenance: string;
  }>;
  components: PreviewComponent[];
  load_cases: Array<{
    id: string;
    label: string;
    kind: string;
    status: string;
    provenance: string;
    primitive_loads?: Array<Record<string, unknown>>;
    pressure_regions?: PressureRegionInput[];
    /** Optional exact user-entered modulus basis (DEC-068 item 1). */
    modulus_basis_ref?: string;
    /** Optional DEC-077 solve temperature for bounded E/alpha interpolation. */
    modulus_basis_temperature?: QuantityValue;
    /** Optional user-entered static-equivalent occasional-load generation
     * inputs (DEC-068 item 2). Starts absent; every value is user-entered;
     * no code coefficient, catalog value, or physical-constant default. */
    equivalent_static?: EquivalentStaticGenerationInput;
    /** Model 0.4.0: the case-owned resolved-state request. */
    analysis_state?: AnalysisStateInput | null;
  }>;
  combinations?: Array<{
    id: string;
    label: string;
    /** Closed basis set mirroring core/loads/load_case_algebra vocabulary:
     * "mechanics" | "result_state_subtraction" | "range_envelope". */
    basis: string;
    terms: Array<{ load_case: string; factor: number }>;
    minuend_id?: string;
    subtrahend_id?: string;
    operand_ids?: string[];
    mode?: string;
    provenance: string;
  }>;
  diagnostics: Diagnostic[];
  /** Model 0.4.0: reference configurations (stress-free installed states). */
  reference_configurations?: ReferenceConfigurationInput[] | null;
};

export type Diagnostic = {
  id?: string;
  code: string;
  diagnostic_class?: string;
  class?: string;
  severity: "info" | "warning" | "error" | "blocking";
  message: string;
  remediation?: string;
  source?: string;
  affected_refs?: string[];
  provenance?: string | Record<string, unknown>;
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

export type NumericalQualityStatus = "not_assessed" | "checks_passed" | "sensitive" | "unresolved" | "failed";
export type NumericalQuality = {
  value_representation: "finite_binary64";
  publication_quantization: "none";
  integrity_policy: "M03-INTEGRITY-v1";
  status: NumericalQualityStatus;
  cases: Array<{
    basis_ref: ResultBasisRef;
    structural_status: "passive_model_basis" | "physical_mechanism_witnessed" | "negative_energy_witnessed" | "numerically_unresolved";
    solve_quality: NumericalQualityStatus;
    model_matrix_fidelity: "represented_equations_retained" | "assembly_loss_detected" | "assembly_uncertainty" | "not_assessed";
    accuracy_evidence: "not_claimed" | "reference_verified" | "unresolved";
    evidence_refs: string[];
  }>;
};
export type MechanicsResult = {
  contract_evidence?: Record<string, unknown>;
  /** Preserved producer receipt; eligibility requires independently captured invocation. */
  source_block_recovery?: unknown;
  producer?: { component_name: string; component_version: string; semantic_contract_id: string };
  numerical_quality?: NumericalQuality;
  formulation_basis?: { profile_id: string; limitations: string[] };
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
    max_displacement?: {
      value: number;
      unit: string;
      location_ref: string;
      result_ref: string;
    } | null;
    max_open_formula_stress?: {
      value: number;
      unit: string;
      location_ref: string;
      result_ref: string;
    } | null;
  };
  results: Array<{
    id: string;
    kind: string;
    value: number;
    unit: string;
    dimension?: CanonicalResultDimension;
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

export type CanonicalResultDimension =
  | "dimensionless"
  | "length"
  | "angle"
  | "force"
  | "moment"
  | "stress"
  | "area"
  | "section_modulus"
  | "second_moment_area"
  | "ratio"
  | "time"
  | "temperature"
  | "pressure"
  | "linear_stiffness"
  | "rotational_stiffness";

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
  diagnostic_class: string;
  severity: Diagnostic["severity"];
  source: string;
  message: string;
  remediation: string;
  provenance: string;
  affected_refs: string[];
  linked_results: Array<{
    id: string;
    kind: string;
    entity_ref: string;
    value_label: string;
    unit: string;
    unit_source: "result_envelope";
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
    /** Retained method evidence, never a live invocation credential. */
    source_block_recovery?: unknown;
    contract_evidence?: Record<string, unknown>;
    run_id: string;
    run_name: string;
    run_kind: string;
    created_at?: string | null;
    model_state_ref: ObjectRef;
    solver_version?: { solver_name: string; solver_version: string; build_ref: ObjectRef };
    settings_ref?: ObjectRef;
    unit_system_ref?: ObjectRef;
    load_basis_refs: ObjectRef[];
    diagnostics?: Array<{ source_annotation: Record<string, unknown> }>;
    rule_pack_refs?: ObjectRef[];
    library_refs?: ObjectRef[];
    result_refs: Array<{
      result_ref: ObjectRef;
      result_family: string | null;
      source_dimension: CanonicalResultDimension | null;
      source_row_index?: number;
      category?: string;
      semantic_contract?: { id: string; sha256: string; signature_id: string | null };
      interpretation?: { status: string; findings: string[] };
      source_annotation?: Record<string, unknown>;
      provenance?: Record<string, unknown>;
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
      input_manifest_hashes: Array<{
        algorithm: "sha256";
        canonicalization: "rfc8785_jcs";
        payload_ref: ObjectRef;
        payload_scope: "input_manifest";
        value: string;
      }>;
      determinism_notes: string[];
      unresolved_tbd: string[];
      semantic_contract?: { id: string; sha256: string };
    };
    immutability_policy: {
      run_record_is_read_only: boolean;
      mutation_policy: string;
      new_run_required_for_change?: boolean;
      new_mechanics_run_required_for_record_revision?: boolean;
      record_revision_identity?: "analysis_run_record_sha256";
      hash_invalidates_external_acceptance: boolean;
    };
    professional_boundary: Record<string, boolean>;
    provenance?: Record<string, unknown>;
  };
};

export type SolveJobEvent = {
  event_id: string;
  state: "not_started" | "queued" | "running" | "cancelling" | "cancelled" | "completed" | "failed";
  message: string;
  result_available: boolean;
  diagnostic_count: number;
  result_row_count: number;
  analysis_status: string[];
};

export type SolveJobAuditState = {
  job_id: string;
  state: SolveJobEvent["state"];
  progress_basis: string;
  percentages_synthesized: false;
  backend_percent_stream_available: false;
  cancellation_requested: boolean;
  cancellation_status: string;
  backend_job_seam: SolveJobBackendSeam;
  backend_job_id: string | null;
  backend_cancellation_token: string;
  events: SolveJobEvent[];
  error_message: string | null;
};

export type SolveJobBackendSeam =
  | "tauri_backend_job"
  | "browser_fixture_no_backend_job"
  | "no_job_started"
  | "restored_persisted_run_no_new_solve";

export type PreviewComparison = {
  schema_version: string;
  document_kind: "openpipestress.technical_preview.comparison";
  deliverable_id: "DEL-14-04";
  package_id: "PKG-14";
  scope_items: string[];
  objectives: string[];
  comparison_id: string;
  comparison_kind: "single_run_load_basis_review";
  left: ComparisonParticipant;
  right: ComparisonParticipant;
  summary: {
    comparable_result_pairs: number;
    unmatched_left_results: number;
    unmatched_right_results: number;
    mapping_basis: string;
    tolerance_status: "not_tolerance_checked";
    tolerance_profile_ref: "TBD";
  };
  unit_policy_evidence: ComparisonUnitPolicyEvidence;
  result_deltas: ComparisonDelta[];
  diagnostics: Diagnostic[];
  professional_boundary: Record<string, boolean>;
};

export type ComparisonUnitPolicyEvidence = {
  evidence_id: string;
  unit_system_ref: ObjectRef;
  storage_convention: "entered_units_preserved";
  comparison_unit_policy: string;
  matching_policy: string;
  matched_result_units: string[];
  unmatched_left_result_count: number;
  unmatched_right_result_count: number;
  conversion_policy: string;
  conversion_performed: false;
  tolerance_profile_ref: "TBD";
  tolerance_status: "not_tolerance_checked";
  decision_basis_refs: ObjectRef[];
  protected_content_included: false;
  private_payload_included: false;
};

export type ComparisonParticipant = {
  label: string;
  basis_ref: ObjectRef;
  model_state_ref: ObjectRef;
  analysis_run_ref: ObjectRef;
  result_count: number;
};

export type ComparisonDelta = {
  mapping_id: string;
  left_result_id: string;
  right_result_id: string;
  entity_ref: string;
  result_family: string;
  component: string;
  location: string;
  unit: string;
  left_value: number;
  right_value: number;
  raw_delta: number;
  absolute_delta: number;
  classification: "not_tolerance_checked";
  classification_basis: string;
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
    changes: Array<{
      change_id: string;
      change_kind: string;
      target_ref: string;
      before: string;
      after: string;
    }>;
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

export type EditorOperationObjectType =
  | "Model"
  | "Material"
  | "Section"
  | "Node"
  | "Element"
  | "Component"
  | "Support"
  | "Load"
  | "Combination";

export type EditorOperationSource = {
  source_ref: string;
  source_channel: "local_desktop_preview" | string;
  source_role: "gui_editor" | "viewport_editor" | string;
};

export type EditorOperationIntent = {
  queue_id?: string;
  operation_id: string;
  operation_kind: "create" | "connect" | "delete" | "insert" | "modify";
  operation_status: "proposed";
  author_type: "user" | "agent";
  source?: EditorOperationSource;
  target: {
    object_type: EditorOperationObjectType;
    ref: string;
  };
  change: {
    change_id: string;
    change_kind:
      | "set_field"
      | "update_load"
      | "update_support"
      | "create_node"
      | "delete_node"
      | "connect_pipe_run"
      | "delete_pipe_run"
      | "create_section"
      | "create_material"
      | "create_support"
      | "create_load_case"
      | "delete_load_case"
      | "create_primitive_load"
      | "delete_primitive_load"
      | "create_combination"
      | "delete_combination"
      | "create_combination_term"
      | "delete_support"
      | "delete_material"
      | "delete_section"
      | "delete_component"
      | "assign_section"
      | "detach_section"
      | "split_pipe_run"
      | "transform_pipe_run"
      | "delete_combination_term"
      | "insert_component_symbol";
    field_label: string;
    field_path: string;
    before: string;
    after: string;
    unit: string;
    dimension: string;
    source_note: string;
  };
  validation: {
    schema_validation: string;
    constraint_validation: string;
    unit_validation: string;
    diff_preview_status: string;
    application_status: string;
  };
  audit_boundary: {
    mutation_route: "structured_operations_only";
    direct_model_mutation_allowed: false;
    requires_user_acceptance: true;
    mutates_accepted_model_state: false;
  };
  professional_boundary: {
    human_review_required: true;
    software_makes_compliance_claim: false;
    software_makes_certification_claim: false;
    software_makes_sealing_claim: false;
    software_makes_approval_claim: false;
    software_makes_authentication_claim: false;
  };
  rationale: string;
};

export type OperationOutcomeDiagnostic = {
  id: string;
  code: string;
  severity: "blocking" | "warning" | "info" | string;
  message: string;
  remediation: string;
  affected_refs: string[];
  source: string;
};

export type OperationDiffPreviewRow = {
  entity_ref: string;
  object_type: string;
  field_path: string;
  before: string;
  after: string;
  unit: string;
  dimension: string;
  change_kind: string;
};

export type OperationValidationStates = {
  schema_validation: string;
  reference_validation: string;
  unit_validation: string;
  before_state_validation: string;
  diff_preview_status: string;
  application_status: string;
};

export type OperationModelBasisEvidence = {
  claimed_model_hash: string;
  claimed_hash_canonicalization: string;
  backend_model_hash: string;
  backend_canonicalization: string;
  binding_status: string;
};

export type OperationAcceptanceRecord = {
  acceptance_basis: string;
  acceptance_is_professional_approval: boolean;
  persistence_status: string;
};

export type OperationOutcome = {
  schema_version: string;
  document_kind: string;
  deliverable_refs: string[];
  mode: "validate_only" | "apply";
  application_route: "tauri_backend_apply" | "local_wasm_engine";
  operation_id: string;
  change_id: string;
  operation_kind: string;
  change_kind: string;
  target_object_type: string;
  target_ref: string;
  validation: OperationValidationStates;
  diff_preview: OperationDiffPreviewRow[];
  diagnostics: OperationOutcomeDiagnostic[];
  model_basis: OperationModelBasisEvidence;
  input_model_unchanged: boolean;
  applied_model: PreviewModel | null;
  applied_model_backend_hash: string | null;
  acceptance: OperationAcceptanceRecord;
  audit_boundary: Record<string, unknown>;
  professional_boundary: Record<string, boolean>;
};

export type AppliedOperationReceipt = {
  receipt_id: string;
  sequence: number;
  operation_id: string;
  change_id: string;
  target_object_type: string;
  target_ref: string;
  field_path: string;
  before: string;
  after: string;
  application_route: OperationOutcome["application_route"];
  applied_model_hash: string;
  acceptance: OperationAcceptanceRecord;
  diagnostics: OperationOutcomeDiagnostic[];
  professional_boundary: Record<string, boolean>;
};

export type EntityRef = {
  id: string;
  type:
    | "project"
    | "material"
    | "section"
    | "node"
    | "pipe"
    | "support"
    | "component"
    | "load"
    | "combination"
    | "diagnostic";
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
  migration_framework: string;
  migration_status: string;
  store_schema_version: number;
  store_schema_target_version: number;
  migrations_applied_on_open: string[];
};

export type LocalProjectSummary = {
  project_id: string;
  project_name: string;
  database_path: string;
  storage_mode: "local_sqlite" | string;
  migration_status: string;
  migration_framework: string;
  store_schema_version: number;
  store_schema_target_version: number;
  migrations_applied_on_open: string[];
  fts_indexed: boolean;
  copied_external_files: boolean;
  editor_intent_count: number;
  proposal_count: number;
  selected_review_target_count: number;
  selected_review_target_ref: string;
  persisted_mechanics_result_count: number;
  persisted_analysis_run_count: number;
  persisted_analysis_run_ref: string;
  persisted_model_hash_count: number;
  persisted_model_hash_ref: string;
  persisted_project_envelope_hash_count: number;
  persisted_project_envelope_hash_ref: string;
  unit_round_trip_status: string;
  unit_round_trip_checked_ref_count: number;
  unit_round_trip_signature: string;
  message: string;
};

export type LocalProjectIndexEntry = {
  project_id: string;
  project_name: string;
  storage_mode: "local_sqlite" | "browser_memory_preview" | string;
  created_at_unix: number;
  updated_at_unix: number;
};

export type ModelHashEvidence = {
  algorithm: "sha256";
  canonicalization: "rfc8785_jcs";
  payload_scope: "model_payload";
  payload_ref: string;
  value: string;
  hash_status: "computed_local_preview";
};

export type ModelHashIntegrityEvidence = {
  integrity_status: "verified_match" | "mismatch_review_required" | "not_persisted" | "hash_recompute_unavailable";
  persisted_value: string;
  recomputed_value: string;
  payload_ref: string;
  verification_basis: "recomputed_on_open_from_restored_model" | "recomputed_at_save_from_returned_model" | "recomputed_at_create_from_returned_model";
  verification_source?: "open" | "save" | "create";
  observed_at?: string;
  observation_scope?: "persisted_snapshot_not_current_local_model";
  claim_standing?: "canonical_payload" | "retained_historical_carrier";
};

export type ProjectEnvelopeHashEvidence = {
  algorithm: "sha256";
  canonicalization: "rfc8785_jcs";
  payload_scope: "project_envelope_payload";
  payload_excludes: "storage_summary_and_envelope_hash_carrier_fields";
  payload_ref: string;
  value: string;
  hash_status: "computed_local_preview";
};

export type ProjectEnvelopeHashIntegrityEvidence = {
  integrity_status: "verified_match" | "mismatch_review_required" | "not_persisted" | "hash_recompute_unavailable";
  persisted_value: string;
  recomputed_value: string;
  payload_ref: string;
  verification_basis: "recomputed_on_open_from_restored_envelope_payload" | "recomputed_at_save_from_returned_envelope_payload" | "recomputed_at_create_from_returned_envelope_payload";
  verification_source?: "open" | "save" | "create";
  observed_at?: string;
  observation_scope?: "persisted_snapshot_not_current_local_model";
};

export type PackageHashEvidence = {
  algorithm: "sha256";
  canonicalization: "rfc8785_jcs";
  payload_scope: "package_review_payload";
  payload_excludes:
    | "validation_report_package_hash_fields"
    | "manifest_and_validation_report_package_hash_carrier_fields";
  payload_ref: string;
  value: string;
  hash_status: "computed_local_preview";
};

export type ModelDocumentMigrationStatus = {
  status: "current" | "migrated" | "newer_than_supported" | "unsupported_schema" | "failed" | string;
  source_schema_version: string;
  target_schema_version: string;
  migration_framework: string;
  db_migration_status: string;
  product_schema_migration_status: string;
  applied_migration_ids: string[];
  persistence_state: string;
  detail: string;
};

export type ModelMigrationLedgerRecord = {
  record_kind: string;
  recorded_at_unix: number;
  source_schema_version: string;
  target_schema_version: string;
  applied_migration_ids: string[];
  migration_framework: string;
  pre_migration_model_hash: string;
  post_migration_model_hash: string;
  trigger: string;
  destructive_rewrite: boolean;
  professional_boundary: Record<string, boolean>;
  hash_evidence?: {
    schema: "model_migration_hash_evidence_v1";
    source_payload_basis: "incoming_pre_migration_model" | "stored_pre_open_migration_model";
    received: {
      model_hash: unknown;
      project_envelope_hash: unknown;
    };
    prior_stored: null | {
      model_hash: unknown;
      project_envelope_hash: unknown;
    };
    computed: {
      pre_migration_model_hash: string;
      post_migration_model_hash: string;
      post_migration_project_envelope_hash: string;
    };
    received_claim_verification: "not_asserted";
  };
};

export type LocalProjectEnvelope = {
  summary: LocalProjectSummary;
  model: PreviewModel;
  editor_intents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  selected_review_target: SelectedReviewTarget | null;
  mechanics_result: MechanicsResult | null;
  analysis_run: AnalysisRunEnvelope | null;
  model_hash: ModelHashEvidence | null;
  project_envelope_hash: ProjectEnvelopeHashEvidence | null;
  model_document_migration: ModelDocumentMigrationStatus | null;
  model_migration_ledger: ModelMigrationLedgerRecord[];
};
