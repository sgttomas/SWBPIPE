import { invoke } from "@tauri-apps/api/core";
import type { MechanicsResult, PreviewModel } from "../types";
import type { RulePackDocument } from "./rulePackService";

// Rule-check runner seam (Phase C4, TP-C4-CHECKGUI-001). Running a rule pack's
// checks against a solved model routes through the desktop (Tauri)
// `run_rule_checks` command backed by `core/rules/rule_check_runner`; browser
// preview mode has no backend and reports an explicit unavailable route
// instead of synthesizing a fallback evaluator. The runner emits ONLY the
// three automatic rule-check statuses; acceptance and professional judgment
// remain with the responsible engineer (PRD §12.5, §22.4; DEC-022 grammar
// v1.0.0; docs/claims_registry.md, DEC-081).

export const RULE_CHECK_BACKEND_DIAGNOSTIC =
  "RULE-CHECK-BACKEND-DESKTOP-ONLY: running rule checks (completeness gate, frozen-grammar " +
  "formula evaluation, and acceptability comparison) runs through the desktop (Tauri) " +
  "run_rule_checks command backed by core/rules/rule_check_runner; browser preview mode has no " +
  "backend evaluator and does not synthesize a fallback.";

// The three automatic rule-check statuses the runner may emit. The wider
// status vocabulary (MODEL_INCOMPLETE, MECHANICS_SOLVED, HUMAN_REVIEW_REQUIRED)
// and the external HUMAN_APPROVED_FOR_PROJECT record are not producible here.
export type RuleCheckStatus = "RULE_INPUTS_INCOMPLETE" | "USER_RULE_CHECKED" | "USER_RULE_FAILED";

export type RuleCheckComputedQuantity = {
  value: number;
  dimension: string;
  unit_ref: string;
};

export type RuleCheckBoundInput = {
  input_id: string;
  source_kind: string;
  supplied: boolean;
  value?: number;
  unit?: string;
  result_id?: string;
  note?: string;
};

export type RuleCheckRunFinding = {
  code: string;
  severity: string;
  subject_id: string;
  message: string;
};

export type RuleCheckOutcome = {
  check_id: string;
  status: RuleCheckStatus;
  computed_value?: RuleCheckComputedQuantity;
  limit_value?: RuleCheckComputedQuantity;
  acceptability_relation: string;
  bound_inputs: RuleCheckBoundInput[];
  completeness_findings: RuleCheckRunFinding[];
  evaluator_findings: RuleCheckRunFinding[];
  diagnostic_codes: string[];
};

export type RuleCheckRunResult = {
  document_kind: string;
  rule_pack_id: string;
  grammar_version: string;
  aggregate_status: RuleCheckStatus;
  checks: RuleCheckOutcome[];
  professional_boundary_notice: string;
};

// Caller-supplied binding selectors (the schema carries no selector tying a
// solver_result required input to a solved result row, so the binding is
// caller-supplied per the rule_check_runner contract).
export type SolverResultSelector = {
  input_id: string;
  result_id: string;
};

export type SuppliedValueBinding = {
  ref_id: string;
  value: number;
  unit: string;
  dimension: string;
};

export type RuleCheckUnavailable = {
  route: "unavailable_browser_preview";
  diagnostic: string;
};

export type RuleCheckRunRoute =
  | { route: "tauri_backend"; result: RuleCheckRunResult }
  | RuleCheckUnavailable;

function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function unavailable(): RuleCheckUnavailable {
  return { route: "unavailable_browser_preview", diagnostic: RULE_CHECK_BACKEND_DIAGNOSTIC };
}

export async function runRuleChecks(args: {
  rulePackDocument: RulePackDocument;
  model?: PreviewModel | null;
  solvedEnvelope?: MechanicsResult | null;
  solverResultBindings?: SolverResultSelector[];
  suppliedValueBindings?: SuppliedValueBinding[];
  projectId?: string | null;
}): Promise<RuleCheckRunRoute> {
  if (!isTauriRuntime()) return unavailable();
  // Prefer an already-solved envelope (so the backend does not re-solve); fall
  // back to the model so the backend solves it. Empty binding arrays are
  // omitted so the backend treats those inputs as unsupplied (never a silent
  // pass). `projectId` scopes the private-library lookup for
  // `private_library_value` inputs (resolved backend-side from the local store).
  const invokeArgs: Record<string, unknown> = { rulePackDocument: args.rulePackDocument };
  if (args.solvedEnvelope) invokeArgs.solvedEnvelope = args.solvedEnvelope;
  else if (args.model) invokeArgs.model = args.model;
  if (args.solverResultBindings && args.solverResultBindings.length > 0) {
    invokeArgs.solverResultBindings = args.solverResultBindings;
  }
  if (args.suppliedValueBindings && args.suppliedValueBindings.length > 0) {
    invokeArgs.suppliedValueBindings = args.suppliedValueBindings;
  }
  if (args.projectId) invokeArgs.projectId = args.projectId;
  const result = await invoke<RuleCheckRunResult>("run_rule_checks", invokeArgs);
  return { route: "tauri_backend", result };
}

// Load the bundled invented demonstration rule pack — invented non-engineering
// example content (DEL-06-05). Byte-parallel to the backend example fixture
// examples/rule_packs/invented_demo.yaml (the run_rule_checks command tests'
// `invented_demo_rule_pack`); it is a valid, runnable pack that yields a clean
// USER_RULE_CHECKED / USER_RULE_FAILED demonstration. It is NOT an engineering
// design basis and is not suitable for professional reliance.
export async function loadDemoRuleCheckPack(): Promise<RulePackDocument> {
  return (await import("../../../../fixtures/product_preview/invented_demo_rule_pack.json"))
    .default as RulePackDocument;
}

// --- Binding plan derivation (pure; unit-testable) -------------------------

export type SolverInputRequirement = {
  input_id: string;
  name: string;
  dimension: string;
  unit_ref: string;
  solver_result_ref?: SolverResultRef;
};

export type ValueInputRequirement = {
  ref_id: string;
  name: string;
  source_kind: string;
  dimension: string;
  unit_ref: string;
};

export type ValueSlotRequirement = {
  slot_id: string;
  slot_kind: string;
  dimension: string;
  unit_ref: string;
};

// Reference from a private_library_value input to a value in a saved private
// library record (resolved backend-side from the local store at run time; never
// embedded in the rule pack — IP boundary).
export type LibraryValueRef = {
  library_kind: string;
  library_id: string;
  record_id: string;
  slot_id: string;
};

// Reference from a solver_result input to a row in the current solved result
// envelope. Authored references are canonical: the run-panel selector is only
// used for older packs without this member.
export type SolverResultRef = {
  result_id: string;
};

export type LibraryInputRequirement = {
  input_id: string;
  name: string;
  library_value_ref?: LibraryValueRef;
};

// What a pack needs the user to bind before a check can produce pass/fail.
// `solverInputs` bind to a solved result row; `valueInputs` and `valueSlots`
// take a user-entered value + unit (dimension comes from the pack). Inputs
// sourced from a private library are surfaced separately: their resolution is
// deferred to the C3 rule-pack <-> library reference slice, so they are
// treated as unsupplied (never a silent pass).
export type RuleCheckBindingPlan = {
  solverInputs: SolverInputRequirement[];
  valueInputs: ValueInputRequirement[];
  valueSlots: ValueSlotRequirement[];
  libraryInputs: LibraryInputRequirement[];
};

function asArray(value: unknown): unknown[] {
  return Array.isArray(value) ? value : [];
}

function asString(value: unknown, fallback = ""): string {
  return typeof value === "string" ? value : fallback;
}

function quantityIntent(item: Record<string, unknown>): { dimension: string; unit_ref: string } {
  const intent = (item.quantity_intent ?? {}) as Record<string, unknown>;
  return {
    dimension: asString(intent.dimension, "TBD"),
    unit_ref: asString(intent.unit_ref, "TBD")
  };
}

function readLibraryValueRef(item: Record<string, unknown>): LibraryValueRef | undefined {
  const ref = item.library_value_ref;
  if (typeof ref !== "object" || ref === null) return undefined;
  const record = ref as Record<string, unknown>;
  const libraryKind = asString(record.library_kind);
  const libraryId = asString(record.library_id);
  const recordId = asString(record.record_id);
  const slotId = asString(record.slot_id);
  if (!libraryKind || !libraryId || !recordId || !slotId) return undefined;
  return { library_kind: libraryKind, library_id: libraryId, record_id: recordId, slot_id: slotId };
}

function readSolverResultRef(item: Record<string, unknown>): SolverResultRef | undefined {
  const ref = item.solver_result_ref;
  if (typeof ref !== "object" || ref === null) return undefined;
  const record = ref as Record<string, unknown>;
  const resultId = asString(record.result_id);
  if (!resultId) return undefined;
  return { result_id: resultId };
}

export function deriveRuleCheckBindingPlan(document: RulePackDocument): RuleCheckBindingPlan {
  const plan: RuleCheckBindingPlan = {
    solverInputs: [],
    valueInputs: [],
    valueSlots: [],
    libraryInputs: []
  };

  for (const raw of asArray(document.required_inputs)) {
    const item = raw as Record<string, unknown>;
    const inputId = asString(item.input_id);
    if (!inputId) continue;
    const sourceKind = asString(item.source_kind, "TBD");
    const name = asString(item.name, inputId);
    const { dimension, unit_ref } = quantityIntent(item);
    if (sourceKind === "solver_result") {
      plan.solverInputs.push({
        input_id: inputId,
        name,
        dimension,
        unit_ref,
        solver_result_ref: readSolverResultRef(item)
      });
    } else if (sourceKind === "private_library_value") {
      plan.libraryInputs.push({ input_id: inputId, name, library_value_ref: readLibraryValueRef(item) });
    } else {
      plan.valueInputs.push({ ref_id: inputId, name, source_kind: sourceKind, dimension, unit_ref });
    }
  }

  for (const raw of asArray(document.value_slots)) {
    const item = raw as Record<string, unknown>;
    const slotId = asString(item.slot_id);
    if (!slotId) continue;
    const { dimension, unit_ref } = quantityIntent(item);
    plan.valueSlots.push({
      slot_id: slotId,
      slot_kind: asString(item.slot_kind, "TBD"),
      dimension,
      unit_ref
    });
  }

  return plan;
}

export type SolverResultReferenceResolution = "resolves" | "result_missing" | "no_result_rows";

export function classifySolverResultReference(
  resultRows: MechanicsResult["results"] | null | undefined,
  resultId: string
): SolverResultReferenceResolution {
  if (!resultRows || resultRows.length === 0) return "no_result_rows";
  return resultRows.some((row) => row.id === resultId) ? "resolves" : "result_missing";
}
