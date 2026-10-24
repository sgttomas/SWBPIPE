import { isRetainedRoute } from "../features/results/numericalResultQuality";
import { sourceContract } from "../features/results/resultReader";
import { numericalResultStanding, retainedSourceRecoveryInvocation } from "../features/results/resultStanding";
import { refuseSurfaceOutput } from "../features/results/outputPolicy";
import { loadReferenceDisclosure } from "../features/results/loadReferenceDisclosure";
import { isFreshSemanticResult, ruleBindingRefusal, retainedRowClassLabel, N_RP_UNVALIDATED, COMPONENT_BODY_REFERENCE_SECTION } from "../features/results/knownSemanticLimitations";
import { retainedPrecisionInvocation } from "../features/results/retainedPrecisionStanding";
import { invokeWithIngress } from "./nativeIngress";
import type { MechanicsResult, PreviewModel } from "../types";
import type { RulePackDocument } from "./rulePackService";
import { disposeDeliveredMechanicsResult, hasNativeMechanicsInvocation, runPreviewMechanics } from "./previewService";

// Rule-check runner seam (Phase C4, TP-C4-CHECKGUI-001). Running a rule pack's
// checks against a solved model routes through the desktop (Tauri)
// `run_rule_checks` command backed by `core/rules/rule_check_runner`; browser
// preview mode has no backend and reports an explicit unavailable route
// instead of synthesizing a fallback evaluator. The runner emits ONLY the
// three automatic rule-check statuses (PRD §12.5, §22.4; DEC-022 grammar
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
  // Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §4.4, §5.4): the disclosure line
  // for a qualified exact result carrying load/reference states, computed from that
  // source before a transient solve is disposed. UI text only; never part of the run
  // result, the record or a file.
  | { route: "tauri_backend"; result: RuleCheckRunResult; loadReferenceDisclosure?: string | null }
  | RuleCheckUnavailable;

/** Epoch 8 (design §4.4): an exact result that is not numerically eligible is refused
 * before the backend, after Rust's first standing finding. */
export const N_RULE_CHECK_NOT_NUMERICALLY_ELIGIBLE = "The result is not numerically eligible for rule checks. It remains readable here.";

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
  // Supply the model even with a solved envelope so the backend can verify
  // complete requested-case coverage without inferring requests from results. Empty binding arrays are
  // omitted so the backend treats those inputs as unsupplied (never a silent
  // pass). `projectId` scopes the private-library lookup for
  // `private_library_value` inputs (resolved backend-side from the local store).
  if (!args.model) throw new Error("RULE_NUMERICAL_CASE_COVERAGE_UNAVAILABLE: supply the actual current model.");
  // Without a supplied envelope this run solves for itself through the native
  // job path; that transient result is released from native custody once used.
  const transient = args.solvedEnvelope ? null : await runPreviewMechanics(args.model);
  try {
    return await checkSolvedSource(args, args.solvedEnvelope ?? transient!, args.model);
  } finally {
    if (transient) await disposeDeliveredMechanicsResult(transient);
  }
}

async function checkSolvedSource(
  args: Parameters<typeof runRuleChecks>[0],
  source: MechanicsResult,
  model: PreviewModel
): Promise<RuleCheckRunRoute> {
  if (!hasNativeMechanicsInvocation(source, model)) throw new Error("RULE_NATIVE_INVOCATION_REQUIRED: reference or saved data is not a fresh supported solve.");
  // T0R: only the static fresh-identity set is rule-eligible (epoch 8 adds pressure-2 and
  // pressure-source-1, as Rust's set holds them).
  if (!isFreshSemanticResult(source)) throw new Error("RULE_SOURCE_IDENTITY_NOT_FRESH: the result is not a fresh supported publication.");
  // Epoch 8 (design §4.4): for the exact routes only, the policy's rule-check entry and the
  // Rust reader's full standing (with the model's checks and, for pressure-source-1, the
  // registered invocation) govern the run on the Current and the transient path alike.
  // The retained and ungated routes keep the checks below, unchanged.
  const exactRoute = sourceContract(source) === "pressure" || sourceContract(source) === "pressure_source";
  if (exactRoute) {
    refuseSurfaceOutput(source, model, "rule-check");
    const standing = numericalResultStanding(source, model);
    if (!standing.eligible) throw new Error(`${standing.findings[0] ?? "RULE_NUMERICAL_NEEDS_RECOMPUTE"}: ${N_RULE_CHECK_NOT_NUMERICALLY_ELIGIBLE}`);
  }
  // U6d: a preview successor requires eligible standing, as the receipt routes do;
  // standing comes only from its registered reader validation (never numerical_quality).
  if (isRetainedRoute(sourceContract(source))) {
    const standing = numericalResultStanding(source, model);
    if (!standing.eligible) throw new Error(`${standing.findings[0]}: the retained-precision result is not numerically eligible for rule checks.`);
  }
  const invokeArgs: Record<string, unknown> = { rulePackDocument: args.rulePackDocument, solvedEnvelope: source };
  if (isRetainedRoute(sourceContract(source))) invokeArgs.sourceBlockInvocation = retainedPrecisionInvocation(source, model);
  // Without the registered invocation the backend's pressure-source-1 standing is never eligible.
  if (sourceContract(source) === "pressure_source") invokeArgs.sourceBlockInvocation = retainedSourceRecoveryInvocation(source, model);
  const disclosure = exactRoute ? loadReferenceDisclosure(source, "rule-check") : null;
  if (model) invokeArgs.model = model;
  if (args.solverResultBindings && args.solverResultBindings.length > 0) {
    invokeArgs.solverResultBindings = args.solverResultBindings;
  }
  if (args.suppliedValueBindings && args.suppliedValueBindings.length > 0) {
    invokeArgs.suppliedValueBindings = args.suppliedValueBindings;
  }
  if (args.projectId) invokeArgs.projectId = args.projectId;
  const result = await invokeWithIngress<RuleCheckRunResult>("run_rule_checks", invokeArgs);
  return { route: "tauri_backend", result, loadReferenceDisclosure: disclosure };
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

// --- Binding pre-check (T0R, mirrors result_export::semantic_contract::rule_binding_refusal) ---
// Display only: the backend enforces the refusal and reports RULE_INPUTS_INCOMPLETE
// with the reason. The UI shows the reason before invoking; nothing here passes a check.
export type RuleBindingPrecheckFinding = { input_id: string; result_id: string; reason: string; notice: string };

export function ruleBindingPrecheck(
  source: MechanicsResult | null | undefined,
  plan: RuleCheckBindingPlan | null | undefined,
  solverResultBindings: SolverResultSelector[] = []
): RuleBindingPrecheckFinding[] {
  if (!source || !plan) return [];
  const bindings: SolverResultSelector[] = [
    ...plan.solverInputs.flatMap((input) => input.solver_result_ref ? [{ input_id: input.input_id, result_id: input.solver_result_ref.result_id }] : []),
    ...solverResultBindings.filter((b) => !plan.solverInputs.some((input) => input.input_id === b.input_id && input.solver_result_ref))
  ];
  // An unreadable source has no bindable rows (B3: a binding to a retired id is a
  // missing result, refused as any other).
  try { sourceContract(source); } catch { return []; }
  const findings: RuleBindingPrecheckFinding[] = [];
  for (const binding of bindings) {
    if (!binding.result_id) continue;
    const row = source.results.find((candidate) => candidate.id === binding.result_id);
    if (row) {
      const reason = ruleBindingRefusal(source, row);
      // U6d: a successor refusal (the only kind since B4/D1 (b)) shows its class notice
      // (D2 4.9.9). Without a validated class (unregistered or refused) the row has no class label.
      // W-15 slice d: a component body's row names its reference section instead.
      const notice = reason === COMPONENT_BODY_REFERENCE_SECTION && row.component_body
        ? `stress of the reference pipe section of component body ${row.component_body.component_ref} (stiffness scale ${row.component_body.stiffness_scale}); not a stress of the component body`
        : retainedRowClassLabel(row, source) ?? N_RP_UNVALIDATED;
      if (reason) findings.push({ ...binding, reason, notice });
    }
  }
  return findings;
}
