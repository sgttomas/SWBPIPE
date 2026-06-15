import { invoke } from "@tauri-apps/api/core";

// Rule-pack backend seam (Phase C2, TP-MAC-147). Validation, checksum
// computation, and local persistence route through the desktop (Tauri)
// commands backed by core/rules; browser preview mode keeps drafts in
// memory only and reports an explicit unavailable route instead of
// synthesizing a TypeScript fallback engine. Private rule packs persist in
// the local SQLite store only and are never transmitted or committed
// (OPS-K-PRIV-1, PRD §17.3).

export const RULE_PACK_BACKEND_DIAGNOSTIC =
  "RULE-PACK-BACKEND-DESKTOP-ONLY: rule-pack validation, checksum computation, and local " +
  "persistence run through the desktop (Tauri) backend commands backed by core/rules; " +
  "browser preview mode keeps rule-pack drafts in memory only and does not synthesize a " +
  "fallback engine.";

export type RulePackDocument = Record<string, unknown>;

export type RulePackValidationFinding = {
  code: string;
  subject: string;
  message: string;
  blocking: boolean;
};

export type RulePackFormulaValidation = {
  formula_id: string;
  declaration_form: string;
  decode_status: string;
  findings: RulePackValidationFinding[];
};

export type RulePackComputedChecksum = {
  algorithm: string;
  canonicalization: string;
  payload_scope: string;
  payload_ref: string;
  payload_excludes: string[];
  basis: string;
  grammar_version: string;
  grammar_version_bound: boolean;
  value: string;
};

export type RulePackChecksumValidation = {
  declared_value: string | null;
  computed: RulePackComputedChecksum | null;
  match_status: string;
};

export type RulePackDocumentValidation = {
  document_kind: string;
  rule_pack_id: string;
  rule_pack_name: string;
  rule_pack_version: string;
  schema_version: string;
  grammar_version: string;
  grammar_version_supported: boolean;
  supported_grammar_versions: string[];
  evaluator_grammar_version: string;
  document_findings: RulePackValidationFinding[];
  lifecycle_findings: RulePackValidationFinding[];
  lifecycle_blocked: boolean;
  formulas: RulePackFormulaValidation[];
  checksum: RulePackChecksumValidation;
  publicly_exportable: boolean;
  professional_boundary_notice: string;
  has_blocking_findings: boolean;
};

export type LocalRulePackIndexEntry = {
  project_id: string;
  rule_pack_id: string;
  rule_pack_name: string;
  rule_pack_version: string;
  lifecycle_status: string;
  privacy_class: string;
  storage_mode: string;
  created_at_unix: number;
  updated_at_unix: number;
};

export type LocalRulePackEnvelope = {
  project_id: string;
  rule_pack_id: string;
  storage_mode: string;
  created_at_unix: number;
  updated_at_unix: number;
  document: RulePackDocument;
  validation: RulePackDocumentValidation;
  message: string;
};

export type LocalRulePackDeleteReceipt = {
  project_id: string;
  rule_pack_id: string;
  deleted: boolean;
  message: string;
};

export type RulePackUnavailable = {
  route: "unavailable_browser_preview";
  diagnostic: string;
};

export type RulePackValidationRoute =
  | { route: "tauri_backend"; validation: RulePackDocumentValidation }
  | RulePackUnavailable;

export type RulePackChecksumRoute =
  | { route: "tauri_backend"; computed: RulePackComputedChecksum }
  | RulePackUnavailable;

export type RulePackSaveRoute =
  | { route: "tauri_backend"; envelope: LocalRulePackEnvelope }
  | RulePackUnavailable;

export type RulePackOpenRoute =
  | { route: "tauri_backend"; envelope: LocalRulePackEnvelope | null }
  | RulePackUnavailable;

export type RulePackListRoute =
  | { route: "tauri_backend"; entries: LocalRulePackIndexEntry[] }
  | RulePackUnavailable;

export type RulePackDeleteRoute =
  | { route: "tauri_backend"; receipt: LocalRulePackDeleteReceipt }
  | RulePackUnavailable;

function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function unavailable(): RulePackUnavailable {
  return { route: "unavailable_browser_preview", diagnostic: RULE_PACK_BACKEND_DIAGNOSTIC };
}

export async function validateRulePack(
  document: RulePackDocument,
  publicExportRequested = false
): Promise<RulePackValidationRoute> {
  if (!isTauriRuntime()) return unavailable();
  const validation = await invoke<RulePackDocumentValidation>("validate_rule_pack", {
    document,
    publicExportRequested
  });
  return { route: "tauri_backend", validation };
}

export async function computeRulePackChecksum(
  document: RulePackDocument
): Promise<RulePackChecksumRoute> {
  if (!isTauriRuntime()) return unavailable();
  const computed = await invoke<RulePackComputedChecksum>("compute_rule_pack_document_checksum", {
    document
  });
  return { route: "tauri_backend", computed };
}

export async function saveLocalRulePack(
  projectId: string,
  document: RulePackDocument
): Promise<RulePackSaveRoute> {
  if (!isTauriRuntime()) return unavailable();
  const envelope = await invoke<LocalRulePackEnvelope>("save_local_rule_pack", {
    projectId,
    document
  });
  return { route: "tauri_backend", envelope };
}

export async function openLocalRulePack(
  projectId: string,
  rulePackId: string
): Promise<RulePackOpenRoute> {
  if (!isTauriRuntime()) return unavailable();
  const envelope = await invoke<LocalRulePackEnvelope | null>("open_local_rule_pack", {
    projectId,
    rulePackId
  });
  return { route: "tauri_backend", envelope };
}

export async function listLocalRulePacks(projectId: string | null): Promise<RulePackListRoute> {
  if (!isTauriRuntime()) return unavailable();
  const entries = await invoke<LocalRulePackIndexEntry[]>("list_local_rule_packs", {
    projectId
  });
  return { route: "tauri_backend", entries };
}

export async function deleteLocalRulePack(
  projectId: string,
  rulePackId: string
): Promise<RulePackDeleteRoute> {
  if (!isTauriRuntime()) return unavailable();
  const receipt = await invoke<LocalRulePackDeleteReceipt>("delete_local_rule_pack", {
    projectId,
    rulePackId
  });
  return { route: "tauri_backend", receipt };
}

// Placeholder provenance for draft authoring surfaces (the document template
// and the structured declaration form builders). Private by default; every
// field is either invented non-engineering wording or TBD — never a copied
// source, standards citation, or code value (PRD §12.4, IP boundary). Returns
// a fresh object each call so callers can embed it without aliasing.
export function draftPlaceholderProvenance(): Record<string, unknown> {
  return {
    source_name: "User private basis (draft placeholder)",
    source_type: "user_private_basis",
    source_location: "TBD",
    source_license: "TBD",
    contributor: "TBD",
    contributor_certification: "TBD",
    redistribution_status: "private_only",
    protected_content_review: "required_before_public_use",
    review_status: "pending"
  };
}

// Draft template for "New draft rule pack". Private by default (PRD §12.4:
// "The application shall mark such rule packs as private by default");
// every placeholder is either TBD or invented non-engineering wording. The
// editor never prefils code equations or standards values (PRD §14.5).
export function buildDraftRulePackDocument(): RulePackDocument {
  const provenance = draftPlaceholderProvenance();
  return {
    schema_version: "0.4.0",
    rule_pack_kind: "open_pipe_stress_rule_pack",
    grammar_version: "1.0.0",
    metadata: {
      rule_pack_id: "private_draft_rule_pack",
      name: "Private Draft Rule Pack",
      schema_version: "0.4.0",
      rule_pack_version: "0.1.0",
      lifecycle_status: "draft",
      source_notice:
        "User-supplied private rule-pack draft. Not an engineering design basis, not a " +
        "standards interpretation, and not suitable for professional reliance.",
      status: "active"
    },
    classification: {
      privacy_class: "private_user_data",
      redistribution_status: "private_only",
      public_repository_policy: "private_user_data_not_committed",
      protected_content_review_required: true,
      private_values_allowed: true
    },
    required_inputs: [
      {
        input_id: "user_required_input_1",
        name: "User-defined required input (placeholder)",
        source_kind: "user_supplied_rule_value",
        quantity_intent: {
          dimension: "TBD",
          unit_ref: "TBD",
          unit_required: true,
          dimension_check_required: true
        },
        required_for: "rule_check",
        completeness_status: "missing_required_value",
        missing_value_diagnostic: "RULE_INPUT_MISSING",
        provenance,
        redistribution_status: "private_only",
        provenance_required: true,
        redistribution_status_required: true
      }
    ],
    formula_declarations: [
      {
        formula_id: "user_formula_1",
        formula_kind: "derived_quantity",
        declaration_form: "declarative_ast",
        expression_language: "open_pipe_stress_declared_expression",
        declaration_payload: {
          payload_kind: "declarative_ast",
          payload_summary:
            "Placeholder declarative-AST formula (DEC-022 grammar v1.0.0); replace with the " +
            "user-defined expression.",
          grammar_status: "frozen_open_pipe_stress_declared_expression",
          expression_ast: {
            node: "variable_ref",
            variable_id: "user_required_input_1"
          },
          arbitrary_code_execution_allowed: false,
          protected_content_policy: "private_user_owned_reference_only"
        },
        arbitrary_code_execution_allowed: false,
        protected_content_policy: "private_user_owned_reference_only",
        input_refs: [{ ref_id: "user_required_input_1", ref_type: "required_input" }],
        output_dimension: {
          dimension: "TBD",
          unit_ref: "TBD",
          unit_required: true,
          dimension_check_required: true
        },
        provenance,
        completeness_status: "review_pending",
        review_status: "pending"
      }
    ],
    value_slots: [
      {
        slot_id: "user_limit_slot_1",
        slot_kind: "TBD",
        quantity_intent: {
          dimension: "TBD",
          unit_ref: "TBD",
          unit_required: true,
          dimension_check_required: true
        },
        value_status: "not_provided",
        required_for: "rule_check",
        missing_value_diagnostic: "RULE_INPUT_MISSING",
        provenance,
        redistribution_status: "private_only",
        provenance_required: true,
        redistribution_status_required: true,
        review_status: "pending",
        completeness_status: "missing_required_value"
      }
    ],
    check_definitions: [
      {
        check_id: "user_check_1",
        name: "User-defined check (placeholder)",
        description: "Replace with the user-defined acceptability check.",
        required_input_refs: [{ ref_id: "user_required_input_1", ref_type: "required_input" }],
        value_slot_refs: [{ ref_id: "user_limit_slot_1", ref_type: "value_slot" }],
        formula_ref: { ref_id: "user_formula_1", ref_type: "formula" },
        acceptability_basis: "user_supplied_rule_pack",
        acceptability_relation: "less_than_or_equal",
        result_statuses: [
          "RULE_INPUTS_INCOMPLETE",
          "USER_RULE_CHECKED",
          "USER_RULE_FAILED",
          "HUMAN_REVIEW_REQUIRED"
        ],
        diagnostic_policy: {
          rule_check_blocking: "RULE_CHECK_BLOCKING",
          missing_input: "RULE_INPUT_MISSING",
          unit_mismatch: "RULE_UNIT_MISMATCH",
          provenance_gap: "RULE_PROVENANCE_WARNING",
          redistribution_gap: "RULE_REDISTRIBUTION_WARNING",
          checksum_mismatch: "RULE_CHECKSUM_MISMATCH",
          protected_content_suspected: "RULE_PROTECTED_CONTENT_WARNING",
          evaluator_error: "RULE_EVALUATOR_ERROR"
        },
        provenance
      }
    ],
    diagnostics: [
      {
        diagnostic_id: "user_draft_notice",
        code: "RULE_PROFESSIONAL_BOUNDARY_NOTICE",
        diagnostic_class: "professional_boundary_notice",
        severity: "info",
        blocks_rule_check: false,
        affected_ref: { ref_id: "private_draft_rule_pack", ref_type: "external_record" },
        message: "Private user rule-pack draft; software computes user-defined checks only.",
        remediation:
          "Complete provenance, values, and review before relying on any rule-check output; " +
          "professional acceptance remains a human record outside this software.",
        provenance
      }
    ],
    checksums: {
      rule_pack_checksum: {
        algorithm: "TBD",
        canonicalization: "TBD",
        payload_ref: { ref_id: "private_draft_rule_pack", ref_type: "checksum" },
        payload_scope: "rule_pack_payload",
        verification_status: "checksum_lifecycle_pending",
        value: "TBD until computed in the editor"
      },
      payload_checksums: [],
      checksum_lifecycle_status: "TBD",
      hash_basis: "TBD"
    },
    provenance,
    professional_boundary: {
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false,
      human_review_required: true,
      human_acceptance_record_software_generated: false,
      external_human_acceptance_ref_allowed: true,
      hash_bound_human_acceptance_required: true,
      reliance_notice:
        "Rule-check output is a software computation over user-supplied data; any project " +
        "reliance requires separate human review bound to current model, report, and " +
        "rule-pack hashes."
    },
    open_decisions: [
      {
        decision_id: "OD-USER-DRAFT-001",
        topic: "TBD",
        status: "open",
        owner: "user",
        notes: "Draft placeholder; record user decisions for this private rule pack here."
      }
    ]
  };
}

// Stamp a computed checksum into a draft document (returns a new object;
// the draft's other members are untouched).
export function stampChecksumIntoDocument(
  document: RulePackDocument,
  computed: RulePackComputedChecksum
): RulePackDocument {
  const rulePackId =
    typeof (document.metadata as Record<string, unknown> | undefined)?.rule_pack_id === "string"
      ? ((document.metadata as Record<string, unknown>).rule_pack_id as string)
      : "rule_pack";
  return {
    ...document,
    checksums: {
      rule_pack_checksum: {
        algorithm: computed.algorithm,
        canonicalization: computed.canonicalization,
        payload_ref: { ref_id: rulePackId, ref_type: "checksum" },
        payload_scope: computed.payload_scope,
        payload_excludes: computed.payload_excludes,
        verification_status: "verified",
        value: computed.value
      },
      payload_checksums: [],
      checksum_lifecycle_status: "verified",
      hash_basis: "Canonical JSON/JCS-compatible"
    }
  };
}
