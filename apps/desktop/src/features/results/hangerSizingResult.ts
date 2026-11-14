/** TypeScript types for the hanger-sizing result document, schema 1.0.0 (DEL-04-11 Design
 * `hanger-sizing.md` §8.3; `schemas/hanger_sizing_result.schema.json`).
 *
 * Types only. A value typed as `HangerSizingDocument` has not been validated by holding the
 * type: closure, binding and arithmetic are the readers' standings, and the desktop
 * shows them only as supplied evidence bound to the exact document (`hangerSizingView.ts`).
 * Nullable members are typed as nullable, exactly where the schema admits null. */

export const HANGER_SIZING_DOCUMENT_KIND = "openpipestress.hanger_sizing.result" as const;
export const HANGER_SIZING_SCHEMA_VERSION = "1.0.0" as const;
export const HANGER_SIZING_SEMANTIC_CONTRACT_ID = "openpipestress.result_semantics/hanger-sizing-1" as const;
export const HANGER_SIZING_CANONICAL_PROFILE = "openpipestress_jcs_ijson_v1" as const;

export type HangerSizingFamilyPolicy = "variable" | "constant" | "variable_else_constant";
export type HangerSizingOutcome = "selected" | "selected_by_fallback" | "no_record_fits";
export type HangerSizingDof = "UX" | "UY" | "UZ";
export type HangerSizingStateCode = "inactive" | "active" | "sticking" | "sliding";
export type HangerSizingLoadCategory =
  | "weight" | "distributed_force" | "pressure" | "thermal" | "hydrotest" | "wind" | "seismic"
  | "occasional" | "concentrated_force" | "concentrated_moment" | "imposed_displacement";
export type HangerSizingDisclosureCode = "HANGER_SIZING_SUPPORT_STATE_DIFFERS" | "HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS";
export type HangerSizingRefusalCode =
  | "HANGER_SIZING_CANDIDATES_REQUIRED"
  | "HANGER_SIZING_CANDIDATE_DOF_SHARED"
  | "HANGER_SIZING_CANDIDATE_DOF_UNSUPPORTED"
  | "HANGER_SIZING_CANDIDATE_DUPLICATE"
  | "HANGER_SIZING_CANDIDATE_FAMILY_UNSUPPORTED"
  | "HANGER_SIZING_CANDIDATE_MOVED"
  | "HANGER_SIZING_CANDIDATE_REFERENCED"
  | "HANGER_SIZING_CANDIDATE_SKEW_UNSUPPORTED"
  | "HANGER_SIZING_CANDIDATE_UNKNOWN"
  | "HANGER_SIZING_CASE_ROLES_OVERLAP"
  | "HANGER_SIZING_CASE_UNKNOWN"
  | "HANGER_SIZING_CHECK_OPERATING_CASES_REQUIRED"
  | "HANGER_SIZING_DERIVED_DOCUMENT_NOT_HASHABLE"
  | "HANGER_SIZING_DERIVED_SOLVE_FAILED"
  | "HANGER_SIZING_DESIGN_OPERATING_CASE_REQUIRED"
  | "HANGER_SIZING_FAMILY_POLICY_REQUIRED"
  | "HANGER_SIZING_ID_COLLISION"
  | "HANGER_SIZING_INPUT_NOT_ADMITTED"
  | "HANGER_SIZING_INPUT_SHAPE_INVALID"
  | "HANGER_SIZING_INPUT_TOO_LARGE"
  | "HANGER_SIZING_LIBRARY_NOT_ADMITTED"
  | "HANGER_SIZING_LIBRARY_RECORD_DUPLICATE"
  | "HANGER_SIZING_LIBRARY_REQUIRED"
  | "HANGER_SIZING_LIBRARY_UNAVAILABLE"
  | "HANGER_SIZING_LOAD_DESIGN_REQUIRED"
  | "HANGER_SIZING_LOAD_NOT_POSITIVE"
  | "HANGER_SIZING_REQUEST_INVALID"
  | "HANGER_SIZING_RESULT_NOT_PUBLISHABLE"
  | "HANGER_SIZING_SELECTION_RULE_REQUIRED"
  | "HANGER_SIZING_SOLVER_MODE_REQUIRED"
  | "HANGER_SIZING_TEST_CASE_UNSUPPORTED"
  | "HANGER_SIZING_TRAVEL_ALLOWANCE_INVALID"
  | "HANGER_SIZING_TRAVEL_ALLOWANCE_REQUIRED"
  | "HANGER_SIZING_VARIATION_LIMIT_INVALID"
  | "HANGER_SIZING_VARIATION_LIMIT_REQUIRED"
  | "HANGER_SIZING_VARIATION_REFERENCE_REQUIRED"
  | "HANGER_SIZING_WEIGHT_CASE_REQUIRED"
  | "HANGER_SIZING_WEIGHT_SUPPORT_STATE_REQUIRED"
  | "HANGER_SIZING_WEIGHT_SUPPORT_STATE_UNSUPPORTED";

/** `[min, max]` (or `[lo, hi]`), each a number. */
export type HangerSizingPair = readonly [number, number];

export type HangerSizingLimitations = readonly [string, string, string, string];
export type HangerSizingBasisStatement = readonly [string, string];

/** The route the derived solves run on: the pressure-free route (a model with no `pressure_contract`) or the
 * exact pressure route (`3.0.0/exact_pressure_v3`). */
export type HangerSizingRoute = "pressure_free" | "exact_pressure";

export type HangerSizingIdentity = {
  readonly document_kind: typeof HANGER_SIZING_DOCUMENT_KIND;
  readonly schema_version: typeof HANGER_SIZING_SCHEMA_VERSION;
  readonly semantic_contract_id: typeof HANGER_SIZING_SEMANTIC_CONTRACT_ID;
  /** A sized document states the route its solves ran on; a refused one states its solve request's route, or
   * null when no model was read (`HangerSizingRefusedDocument`). */
  readonly route: HangerSizingRoute | null;
  readonly limitations: HangerSizingLimitations;
  readonly basis_statement: HangerSizingBasisStatement;
};

export type HangerSizingRequest = {
  readonly weight_case: string;
  readonly weight_support_state: "own_solve";
  readonly design_operating_case: string;
  readonly check_operating_cases: readonly string[];
  readonly candidates: readonly { readonly support: string; readonly family_policy: HangerSizingFamilyPolicy }[];
  readonly method: {
    readonly load_design: "hot" | "cold";
    readonly variation_reference: "operating_load" | "design_load";
    readonly variation_limit: number;
    readonly travel_allowance_m: number;
    readonly selection_rule: "smallest_capacity" | "least_variation";
  };
  readonly solver_mode: "sparse_interactive" | "dense_scrutiny";
};

export type HangerSizingSizedInputs = {
  readonly canonical_profile: typeof HANGER_SIZING_CANONICAL_PROFILE;
  readonly sizing_request: HangerSizingRequest;
  readonly sizing_request_sha256: string;
  readonly solve_request: { readonly sha256: string };
  readonly library: { readonly library_id: string; readonly sha256: string };
};

/** `sizing_request` and `sizing_request_sha256` are null together (schema `oneOf`). */
export type HangerSizingRefusedInputs = {
  readonly canonical_profile: typeof HANGER_SIZING_CANONICAL_PROFILE;
  readonly solve_request: { readonly sha256: string | null };
  readonly library: { readonly library_id: string | null; readonly sha256: string | null };
} & (
  | { readonly sizing_request: HangerSizingRequest; readonly sizing_request_sha256: string }
  | { readonly sizing_request: null; readonly sizing_request_sha256: null }
);

export type HangerSizingCase = {
  readonly case: string;
  readonly role: "weight" | "operating";
  readonly load_categories: readonly HangerSizingLoadCategory[];
  readonly uses_condition_set: boolean;
};

export type HangerSizingVariableCounts = {
  readonly E1: number; readonly E2: number; readonly V0: number; readonly L: number; readonly T: number; readonly V: number;
};
export type HangerSizingConstantCounts = { readonly E1: number; readonly L: number; readonly T: number };

export type HangerSizingPosition = { readonly case: string; readonly u_m: number };

export type HangerSizingVariableSelection = {
  readonly family: "variable_spring";
  readonly hanger_id: string;
  readonly record_sha256: string;
  readonly k_n_per_m: number;
  readonly f_cold_n: number;
  readonly f_cold_condition_n: number;
  readonly envelope_n: HangerSizingPair;
  readonly operating: readonly { readonly case: string; readonly f_n: number; readonly variation: number }[];
};

export type HangerSizingConstantSelection = {
  readonly family: "constant_effort";
  readonly hanger_id: string;
  readonly record_sha256: string;
  readonly load_n: number;
};

export type HangerSizingSpringAssumption = { readonly role: "spring"; readonly k_n_per_m: number; readonly f_cold_n: number };
export type HangerSizingForceAssumption = { readonly role: "force"; readonly force_n: number };

export type HangerSizingVariableVerification = {
  readonly family: "variable_spring";
  readonly u_cold_m: number;
  readonly operating: readonly { readonly case: string; readonly u_m: number; readonly f_n: number; readonly variation: number | null }[];
  readonly span_m: HangerSizingPair;
  readonly required_travel_m: number;
  readonly t_pass: boolean;
  readonly f_cold_condition_n: number;
  readonly departure_n: number;
  readonly failed_tests: readonly ("V0" | "L" | "T" | "V")[];
};

export type HangerSizingConstantVerification = {
  readonly family: "constant_effort";
  readonly u_cold_m: number;
  readonly operating: readonly HangerSizingPosition[];
  readonly span_m: HangerSizingPair;
  readonly required_travel_m: number;
  readonly t_pass: boolean;
  readonly failed_tests: readonly "T"[];
};

type HangerSizingCandidateCommon = {
  readonly support: string;
  readonly node: string;
  readonly dof: HangerSizingDof;
  readonly family_policy: HangerSizingFamilyPolicy;
  readonly w_n: number;
  readonly u_cold_m: number;
  readonly operating: readonly HangerSizingPosition[];
  readonly span_m: HangerSizingPair;
  readonly required_travel_m: number;
  /** Each family is null when it was not tested (not the same as zero counts). */
  readonly failure_counts: {
    readonly variable: HangerSizingVariableCounts | null;
    readonly constant: HangerSizingConstantCounts | null;
  };
};

/** The three candidate variants of the schema's `Candidate.oneOf`. */
export type HangerSizingNoFitCandidate = HangerSizingCandidateCommon & {
  readonly outcome: "no_record_fits";
  readonly selected: null;
  readonly governing: null;
  readonly verification_assumption: HangerSizingForceAssumption;
  readonly verification: null;
};
export type HangerSizingVariableCandidate = HangerSizingCandidateCommon & {
  readonly outcome: "selected" | "selected_by_fallback";
  readonly selected: HangerSizingVariableSelection;
  readonly governing: { readonly travel_case: string; readonly variation_case: string };
  readonly verification_assumption: HangerSizingSpringAssumption;
  readonly verification: HangerSizingVariableVerification;
};
export type HangerSizingConstantCandidate = HangerSizingCandidateCommon & {
  readonly outcome: "selected" | "selected_by_fallback";
  readonly selected: HangerSizingConstantSelection;
  readonly governing: { readonly travel_case: string; readonly variation_case: null };
  readonly verification_assumption: HangerSizingForceAssumption;
  readonly verification: HangerSizingConstantVerification;
};
export type HangerSizingCandidate = HangerSizingNoFitCandidate | HangerSizingVariableCandidate | HangerSizingConstantCandidate;

export type HangerSizingSolve = {
  readonly label: string;
  readonly case: string;
  readonly derived_document_sha256: string;
  /** Nonlinear support id to state; empty when the model has none. */
  readonly states: Readonly<Record<string, HangerSizingStateCode>>;
};

export type HangerSizingDisclosure = {
  readonly code: HangerSizingDisclosureCode;
  readonly names: readonly string[];
  readonly message: string;
};

export type HangerSizingRefusal = {
  readonly code: HangerSizingRefusalCode;
  readonly names: readonly string[];
  readonly message: string;
};

export type HangerSizingResult = {
  readonly verification_standing: "unconditional" | "conditional";
  readonly cases: readonly HangerSizingCase[];
  readonly candidates: readonly HangerSizingCandidate[];
  readonly solves: readonly HangerSizingSolve[];
  readonly disclosures: readonly HangerSizingDisclosure[];
};

export type HangerSizingSizedDocument = HangerSizingIdentity & {
  readonly route: HangerSizingRoute;
  readonly status: "sized";
  readonly inputs: HangerSizingSizedInputs;
  readonly result: HangerSizingResult;
};

export type HangerSizingRefusedDocument = HangerSizingIdentity & {
  readonly status: "refused";
  readonly inputs: HangerSizingRefusedInputs;
  readonly refusals: readonly HangerSizingRefusal[];
};

export type HangerSizingDocument = HangerSizingSizedDocument | HangerSizingRefusedDocument;
