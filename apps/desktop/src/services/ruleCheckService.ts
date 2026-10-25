import { isRetainedRoute } from "../features/results/numericalResultQuality";
import { sourceContract } from "../features/results/resultReader";
import { numericalResultStanding, retainedSourceRecoveryInvocation } from "../features/results/resultStanding";
import { refuseSurfaceOutput } from "../features/results/outputPolicy";
import { loadReferenceDisclosure } from "../features/results/loadReferenceDisclosure";
import { isFreshSemanticResult, ruleBindingRefusal, retainedRowClassLabel, N_RP_UNVALIDATED, COMPONENT_BODY_REFERENCE_SECTION } from "../features/results/knownSemanticLimitations";
import { retainedPrecisionInvocation } from "../features/results/retainedPrecisionStanding";
import { invokeWithIngress } from "./nativeIngress";
import type { MechanicsResult, PreviewModel } from "../types";
import type { RulePackDocument, RulePackDocumentValidation } from "./rulePackService";
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

// A model load case or combination (rule-pack format 0.5.0; DEL-06-01 Design
// `rule-pack-0.5.0.md` §2.3): as a pack's required load case names it, as an
// envelope row's basis_ref carries it and as each outcome reports it.
export type RuleCheckBasisRef = {
  ref_type: "load_case" | "combination";
  ref_id: string;
};

// One outcome per check and basis (design §2.7): `category_ref` is the check's
// stress category id; `basis_ref` the load case or combination this outcome was
// evaluated on (null only for a check the pack gives no basis, which validation
// blocks before any run). Outcome identity is (check_id, basis_ref.ref_type,
// basis_ref.ref_id): a check_id repeats once per basis. The run command always
// sends both; they are optional in this type only so outcome literals typed
// before format 0.5.0 (the report's own test inputs; the report change is
// deferred, design §6) still compile. A missing basis reads as none.
export type RuleCheckOutcome = {
  check_id: string;
  category_ref?: string;
  basis_ref?: RuleCheckBasisRef | null;
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
  // Present only when the run was refused because the pack's validation has
  // blocking findings (checks is then empty); the findings name the refusal,
  // for example RULE_PACK_SCHEMA_VERSION_UNSUPPORTED for a 0.4.0 pack.
  rule_pack_validation?: Omit<RulePackDocumentValidation, "unit_check">;
};

/** The serialized outcome identity (design §2.7): (check_id, basis_ref.ref_type,
 * basis_ref.ref_id). Keys and lookups use it, never check_id alone. */
export function ruleCheckOutcomeIdentity(outcome: Pick<RuleCheckOutcome, "check_id" | "basis_ref">): string {
  return JSON.stringify([outcome.check_id, outcome.basis_ref?.ref_type ?? null, outcome.basis_ref?.ref_id ?? null]);
}

/** The outcome's test id, built from the same identity. */
export function ruleCheckOutcomeTestId(outcome: Pick<RuleCheckOutcome, "check_id" | "basis_ref">): string {
  const basis = outcome.basis_ref ? `${outcome.basis_ref.ref_type}--${outcome.basis_ref.ref_id}` : "no-basis";
  return `rule-check-outcome-${outcome.check_id}--${basis}`;
}

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
  suppliedValueBindings?: SuppliedValueBinding[];
  projectId?: string | null;
}): Promise<RuleCheckRunRoute> {
  if (!isTauriRuntime()) return unavailable();
  // Supply the model even with a solved envelope so the backend can verify
  // complete requested-case coverage without inferring requests from results, and
  // so it can check the pack's required load cases against the model. Empty
  // binding arrays are omitted so the backend treats those inputs as unsupplied
  // (never a silent pass). Solver inputs bind only through each pack's own
  // solver_result_ref selector (format 0.5.0, design §2.5): no caller-supplied
  // solver binding is sent. `projectId` scopes the private-library lookup for
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
// `invented_demo_rule_pack`). In format 0.5.0 it demonstrates the refusal path:
// its solver input's selector names an invented kind no producer publishes and
// its limit slot stays unsupplied, so every run is RULE_INPUTS_INCOMPLETE naming
// both inputs missing (DEL-06-01 Design `rule-pack-0.5.0.md` §4). It is NOT an
// engineering design basis and is not suitable for professional reliance.
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
  // The pack's row selector; absent (or unusable) when the pack carries none,
  // which validation refuses as RULE_SOLVER_RESULT_REF_MISSING.
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

// The row selector of a solver_result input (rule-pack format 0.5.0; DEL-06-01
// Design `rule-pack-0.5.0.md` §2.5). On each basis of a check the input binds
// the one envelope row whose basis_ref is the basis and whose kind, entity_ref,
// metadata.location, metadata.component, section basis and tie rod equal these
// members; an absent optional member matches only a row without it. It is the
// one binding route: the run command refuses caller-supplied selectors.
export type SolverResultRef = {
  result_kind: string;
  entity_ref: string;
  location?: string;
  component?: string;
  section_basis?: string;
  tie_rod?: string;
};

export type LibraryInputRequirement = {
  input_id: string;
  name: string;
  library_value_ref?: LibraryValueRef;
};

// A required load case of the pack (design §2.3): the model load case or
// combination its checks are evaluated on. `basis_ref` is null when the pack's
// item is malformed (validation refuses it).
export type RequiredLoadCaseRequirement = {
  required_load_case_id: string;
  name: string;
  basis_ref: RuleCheckBasisRef | null;
};

// A user-defined stress category (design §2.3): the user's own name.
export type StressCategoryRequirement = {
  category_id: string;
  name: string;
};

// What a pack needs the user to bind before a check can produce pass/fail.
// `solverInputs` bind to a solved result row through the pack's selector, on
// each of the pack's required load cases; `valueInputs` and `valueSlots` take a
// user-entered value + unit (dimension comes from the pack). Inputs sourced
// from a private library are surfaced separately and resolved backend-side.
// A check's category and bases (design §2.4): the required load cases its
// category's one mapping lists, in the mapping's order. `bases` is empty when
// the pack does not give the check a basis list (validation refuses it).
export type CheckBasisRequirement = {
  check_id: string;
  category_ref: string;
  bases: RequiredLoadCaseRequirement[];
  /** The required inputs the check resolves (its `required_input_refs[].ref_id`),
   * the inputs the runner binds for it on each of its bases. */
  input_refs: string[];
};

export type RuleCheckBindingPlan = {
  solverInputs: SolverInputRequirement[];
  valueInputs: ValueInputRequirement[];
  valueSlots: ValueSlotRequirement[];
  libraryInputs: LibraryInputRequirement[];
  requiredLoadCases: RequiredLoadCaseRequirement[];
  stressCategories: StressCategoryRequirement[];
  checks: CheckBasisRequirement[];
};

function asArray(value: unknown): unknown[] {
  return Array.isArray(value) ? value : [];
}

function asString(value: unknown, fallback = ""): string {
  return typeof value === "string" ? value : fallback;
}

function asRecord(value: unknown): Record<string, unknown> | null {
  return typeof value === "object" && value !== null && !Array.isArray(value) ? (value as Record<string, unknown>) : null;
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

const SELECTOR_OPTIONAL = ["location", "component", "section_basis", "tie_rod"] as const;
const SELECTOR_MEMBERS: readonly string[] = ["result_kind", "entity_ref", ...SELECTOR_OPTIONAL];
/** The section-basis tokens a selector may name (rule_pack_document SECTION_BASES). */
export const SOLVER_SELECTOR_SECTION_BASES = ["nominal_section", "reduced_section"] as const;

/** Read a 0.5.0 selector as the validator does (rule_pack_document
 * `parse_solver_result_ref`): undefined when it is not an object, a required
 * member is absent or empty, an optional member is present but not a non-empty
 * string, the section basis is outside the two tokens, or it carries any other
 * member (the 0.4.0 result_id among them). */
export function parseSolverResultRef(value: unknown): SolverResultRef | undefined {
  const record = asRecord(value);
  if (!record) return undefined;
  if (Object.keys(record).some((key) => !SELECTOR_MEMBERS.includes(key))) return undefined;
  const resultKind = asString(record.result_kind);
  const entityRef = asString(record.entity_ref);
  if (!resultKind || !entityRef) return undefined;
  const selector: SolverResultRef = { result_kind: resultKind, entity_ref: entityRef };
  for (const key of SELECTOR_OPTIONAL) {
    if (!(key in record)) continue;
    const member = record[key];
    if (typeof member !== "string" || member.length === 0) return undefined;
    selector[key] = member;
  }
  if (selector.section_basis !== undefined && !(SOLVER_SELECTOR_SECTION_BASES as readonly string[]).includes(selector.section_basis)) {
    return undefined;
  }
  return selector;
}

/** A well-formed basis reference (rule_pack_document `BasisRef::from_value`). */
export function readBasisRef(value: unknown): RuleCheckBasisRef | null {
  const record = asRecord(value);
  if (!record) return null;
  const refType = record.ref_type;
  const refId = record.ref_id;
  if ((refType !== "load_case" && refType !== "combination") || typeof refId !== "string" || refId.trim() === "") return null;
  return { ref_type: refType, ref_id: refId };
}

export function deriveRuleCheckBindingPlan(document: RulePackDocument): RuleCheckBindingPlan {
  const plan: RuleCheckBindingPlan = {
    solverInputs: [],
    valueInputs: [],
    valueSlots: [],
    libraryInputs: [],
    requiredLoadCases: [],
    stressCategories: [],
    checks: []
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
        solver_result_ref: parseSolverResultRef(item.solver_result_ref)
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

  for (const raw of asArray(document.required_load_cases)) {
    const item = asRecord(raw);
    const id = asString(item?.required_load_case_id);
    if (!item || !id) continue;
    plan.requiredLoadCases.push({ required_load_case_id: id, name: asString(item.name), basis_ref: readBasisRef(item.basis_ref) });
  }

  for (const raw of asArray(document.stress_categories)) {
    const item = asRecord(raw);
    const id = asString(item?.category_id);
    if (!item || !id) continue;
    plan.stressCategories.push({ category_id: id, name: asString(item.name) });
  }

  // Each check's bases, as rule_pack_document `plan_check_bases` plans them:
  // the category's one mapping, its required load cases in order.
  const refId = (value: unknown, refType: string) => {
    const reference = asRecord(value);
    return reference?.ref_type === refType ? asString(reference.ref_id) : "";
  };
  const mappings = asArray(document.load_case_mappings).map(asRecord).filter((item): item is Record<string, unknown> => item !== null);
  for (const raw of asArray(document.check_definitions)) {
    const item = asRecord(raw);
    const checkId = asString(item?.check_id);
    if (!item || !checkId) continue;
    const category = refId(item.category_ref, "stress_category");
    const categoryMappings = category ? mappings.filter((mapping) => refId(mapping.category_ref, "stress_category") === category) : [];
    const bases = categoryMappings.length === 1
      ? asArray(categoryMappings[0].required_load_case_refs).flatMap((reference) => {
          const id = refId(reference, "required_load_case");
          const declared = plan.requiredLoadCases.filter((loadCase) => loadCase.required_load_case_id === id);
          return declared.length === 1 ? declared : [];
        })
      : [];
    const inputRefs = asArray(item.required_input_refs).map((reference) => asString(asRecord(reference)?.ref_id)).filter((id) => id !== "");
    plan.checks.push({ check_id: checkId, category_ref: category, bases, input_refs: inputRefs });
  }

  return plan;
}

/** The bases on which the run binds `inputId`: the bases of every check that
 * resolves it (`required_input_refs`), in check order and then mapping order,
 * each once. The runner reads an input's row only for those checks on those
 * bases (rule_check_runner `run_one_check`), so a basis no such check has is
 * never one the input must resolve on. Empty when no check uses the input. */
export function solverInputBases(plan: RuleCheckBindingPlan, inputId: string): RequiredLoadCaseRequirement[] {
  const bases: RequiredLoadCaseRequirement[] = [];
  for (const check of plan.checks) {
    if (!check.input_refs.includes(inputId)) continue;
    for (const basis of check.bases) {
      if (!basis.basis_ref || bases.some((seen) => seen.basis_ref && sameBasis(seen.basis_ref, basis.basis_ref!))) continue;
      bases.push(basis);
    }
  }
  return bases;
}

// --- Row selection (display only; mirrors the run command) ------------------
// The desktop run command (`src-tauri/src/rule_solver_selection.rs`
// `row_matches`) decides which row an input binds; these functions repeat its
// rule so the panels can show, before a run, which row each selector names on
// each basis. Nothing here binds a value or passes a check.

type ResultRow = MechanicsResult["results"][number];

const SECTION_BASIS_TOKEN_SEPARATOR = "; section_basis=";
/** connector_evidence.rs TIE_ROD_KIND / joint.rs TIE_ROD_FORCE_KIND. */
export const TIE_ROD_KIND = "connector_tie_rod_force_v1";

type RowSectionBasis = { kind: "absent" } | { kind: "named"; basis: string } | { kind: "malformed" };

// result_export::section_basis::split_token, as rule_solver_selection.rs reads it.
function rowSectionBasis(row: ResultRow): RowSectionBasis {
  const token = (row.metadata as Record<string, unknown> | undefined)?.basis;
  if (typeof token !== "string" || !token.includes("section_basis=")) return { kind: "absent" };
  const at = token.lastIndexOf(SECTION_BASIS_TOKEN_SEPARATOR);
  if (at < 0) return { kind: "malformed" };
  const recovery = token.slice(0, at);
  const basis = token.slice(at + SECTION_BASIS_TOKEN_SEPARATOR.length);
  if (!(SOLVER_SELECTOR_SECTION_BASES as readonly string[]).includes(basis) || recovery === "" || recovery.includes(SECTION_BASIS_TOKEN_SEPARATOR)) {
    return { kind: "malformed" };
  }
  return { kind: "named", basis };
}

function metadataString(row: ResultRow, key: string): string | undefined {
  const value = (row.metadata as Record<string, unknown> | undefined)?.[key];
  return typeof value === "string" ? value : undefined;
}

// The rod-row id convention (connector_evidence.rs tie_rod_row_tail and
// is_tie_rod_row_id, restated in rule_solver_selection.rs).
const stableSuffix = (id: string) => id.split(":").join("-");
const tieRodRowTail = (component: string, rod: string) =>
  `connector:${stableSuffix(component)}:tie-rod-force:tie_rod:${stableSuffix(rod)}:t_r`;
const isTieRodRowId = (id: string, caseId: string, tail: string) =>
  id === `result:${tail}` || id === `result:loadcase:${stableSuffix(caseId)}:${tail}`;

function sameBasis(left: RuleCheckBasisRef | null, right: RuleCheckBasisRef): boolean {
  return left !== null && left.ref_type === right.ref_type && left.ref_id === right.ref_id;
}

/** The solved result a selection reads: its rows and its connector evidence. */
export type SolverSelectionSource = Pick<MechanicsResult, "results" | "contract_evidence">;

/** The exact rod id a tie-rod row on `basis` belongs to (rule_solver_selection.rs
 * `row_tie_rod`): the one `tie_rods[].id` of the row's one connector record whose
 * encoded row id the row carries. The row id encodes a rod id lossily (`:` to
 * `-`), so the rod comes from the record, never from the row id: undefined when
 * the result has no record or several for the connector, or no rod or several
 * of the record encode to the row's id. Such a row matches no `tie_rod` selector. */
export function rowTieRod(source: Pick<MechanicsResult, "contract_evidence"> | null | undefined, row: ResultRow, basis: RuleCheckBasisRef): string | undefined {
  if (!row.entity_ref || typeof row.id !== "string") return undefined;
  const records = asArray((source?.contract_evidence as Record<string, unknown> | undefined)?.connector)
    .map(asRecord)
    .filter((record) => record?.component_id === row.entity_ref);
  if (records.length !== 1) return undefined;
  const bound = asArray(records[0]?.tie_rods)
    .map((entry) => asRecord(entry)?.id)
    .filter((id): id is string => typeof id === "string" && isTieRodRowId(row.id, basis.ref_id, tieRodRowTail(row.entity_ref, id)));
  return bound.length === 1 ? bound[0] : undefined;
}

/** Whether `row` of `source` is the selector's row on `basis` (design §2.5). A
 * selector's `tie_rod` must equal, exactly, the rod the row's connector record
 * binds it to (`rowTieRod`). */
export function solverSelectorMatchesRow(source: Pick<MechanicsResult, "contract_evidence"> | null | undefined, selector: SolverResultRef, basis: RuleCheckBasisRef, row: ResultRow): boolean {
  if (!sameBasis(readBasisRef(row.basis_ref), basis)) return false;
  if (row.kind !== selector.result_kind || row.entity_ref !== selector.entity_ref) return false;
  if (metadataString(row, "location") !== selector.location || metadataString(row, "component") !== selector.component) return false;
  const section = rowSectionBasis(row);
  const sectionMatches = section.kind === "malformed"
    ? false
    : section.kind === "absent" ? selector.section_basis === undefined : selector.section_basis === section.basis;
  if (!sectionMatches) return false;
  const rodRow = row.kind === TIE_ROD_KIND;
  if (selector.tie_rod === undefined) return !rodRow;
  return rodRow && rowTieRod(source, row, basis) === selector.tie_rod;
}

/** Every row of `source` the selector matches on `basis`, in envelope order. */
export function selectSolverRows(source: SolverSelectionSource | null | undefined, selector: SolverResultRef, basis: RuleCheckBasisRef): ResultRow[] {
  return (source?.results ?? []).filter((row) => solverSelectorMatchesRow(source, selector, basis, row));
}

/** The selector that names `row` within its basis (the authoring offer from a
 * current solved result), or undefined for a row without basis_ref, with a
 * malformed section-basis token, or a rod row no single rod of the result's
 * connector evidence binds (`rowTieRod`). */
export function solverSelectorForRow(source: Pick<MechanicsResult, "contract_evidence">, row: ResultRow): SolverResultRef | undefined {
  const basis = readBasisRef(row.basis_ref);
  if (!basis || !row.kind || !row.entity_ref) return undefined;
  const section = rowSectionBasis(row);
  if (section.kind === "malformed") return undefined;
  const selector: SolverResultRef = { result_kind: row.kind, entity_ref: row.entity_ref };
  const location = metadataString(row, "location");
  const component = metadataString(row, "component");
  if (location !== undefined) selector.location = location;
  if (component !== undefined) selector.component = component;
  if (section.kind === "named") selector.section_basis = section.basis;
  if (row.kind === TIE_ROD_KIND) {
    const rod = rowTieRod(source, row, basis);
    if (!rod) return undefined;
    selector.tie_rod = rod;
  }
  return selector;
}

/** A one-line reading of a selector: its members in schema order. */
export function describeSolverSelector(selector: SolverResultRef): string {
  return [
    `result_kind=${selector.result_kind}`,
    `entity_ref=${selector.entity_ref}`,
    ...SELECTOR_OPTIONAL.flatMap((key) => (selector[key] !== undefined ? [`${key}=${selector[key]}`] : []))
  ].join("; ");
}

export type SolverSelectorBasisStatus = "resolves" | "row_missing" | "row_ambiguous" | "basis_unsolved";

export type SolverSelectorBasisResolution = {
  required_load_case_id: string;
  name: string;
  basis_ref: RuleCheckBasisRef;
  status: SolverSelectorBasisStatus;
  rows: ResultRow[];
};

export type SolverSelectorResolution =
  | { status: "no_result_rows" }
  | { status: "resolves" | "incomplete"; bases: SolverSelectorBasisResolution[] };

/** Which row the selector names on each of the given required load cases (an
 * input's bases, `solverInputBases`) in the current solved result: one row
 * (resolves), none (row_missing, or basis_unsolved when the result publishes
 * nothing on that basis) or several (row_ambiguous). Only `resolves` on every
 * basis, with at least one basis, reads `resolves`. */
export function resolveSolverSelector(
  source: SolverSelectionSource | null | undefined,
  selector: SolverResultRef,
  requiredLoadCases: readonly RequiredLoadCaseRequirement[]
): SolverSelectorResolution {
  const resultRows = source?.results;
  if (!resultRows || resultRows.length === 0) return { status: "no_result_rows" };
  const bases: SolverSelectorBasisResolution[] = [];
  for (const loadCase of requiredLoadCases) {
    const basis = loadCase.basis_ref;
    if (!basis) continue;
    const rows = selectSolverRows(source, selector, basis);
    const solved = resultRows.some((row) => sameBasis(readBasisRef(row.basis_ref), basis));
    const status: SolverSelectorBasisStatus = rows.length === 1
      ? "resolves"
      : rows.length > 1 ? "row_ambiguous" : solved ? "row_missing" : "basis_unsolved";
    bases.push({ required_load_case_id: loadCase.required_load_case_id, name: loadCase.name, basis_ref: basis, status, rows });
  }
  return { status: bases.length > 0 && bases.every((entry) => entry.status === "resolves") ? "resolves" : "incomplete", bases };
}

// --- Binding pre-check (T0R, mirrors result_export::semantic_contract::rule_binding_refusal) ---
// Display only: the backend enforces the refusal and reports RULE_INPUTS_INCOMPLETE
// with the reason, on that input and basis. The UI shows the reason before
// invoking; nothing here passes a check.
export type RuleBindingPrecheckFinding = {
  input_id: string;
  basis_ref: RuleCheckBasisRef;
  result_id: string;
  reason: string;
  notice: string;
};

export function ruleBindingPrecheck(
  source: MechanicsResult | null | undefined,
  plan: RuleCheckBindingPlan | null | undefined
): RuleBindingPrecheckFinding[] {
  if (!source || !plan) return [];
  // An unreadable source has no bindable rows.
  try { sourceContract(source); } catch { return []; }
  const findings: RuleBindingPrecheckFinding[] = [];
  for (const input of plan.solverInputs) {
    const selector = input.solver_result_ref;
    if (!selector) continue;
    // Only the bases of the checks that use the input: the run binds it nowhere else.
    for (const { basis_ref: basis } of solverInputBases(plan, input.input_id)) {
      if (!basis) continue;
      // Only a row the selector names alone is bound; a missing or ambiguous
      // selection is refused by the run command on its own terms.
      const rows = selectSolverRows(source, selector, basis);
      if (rows.length !== 1) continue;
      const row = rows[0];
      const reason = ruleBindingRefusal(source, row);
      if (!reason) continue;
      // U6d: a successor refusal (the only kind since B4/D1 (b)) shows its class notice
      // (D2 4.9.9). Without a validated class (unregistered or refused) the row has no class label.
      // W-15 slice d: a component body's row names its reference section instead.
      const notice = reason === COMPONENT_BODY_REFERENCE_SECTION && row.component_body
        ? `stress of the reference pipe section of component body ${row.component_body.component_ref} (stiffness scale ${row.component_body.stiffness_scale}); not a stress of the component body`
        : retainedRowClassLabel(row, source) ?? N_RP_UNVALIDATED;
      findings.push({ input_id: input.input_id, basis_ref: basis, result_id: row.id, reason, notice });
    }
  }
  return findings;
}
