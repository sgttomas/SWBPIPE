// RT-A4 on the desktop (owned by
// DEL-07-03): the combination editor's operand rules. Each term names a typed operand, a load
// case (`{"load_case": id, "factor": f}`) or a combination (`{"combination": id, "factor": f}`).
// A bare reference (minuend_id, subtrahend_id, operand_ids) resolves to the load case when one
// has the id and to a combination only otherwise (r1 F3); the editor shows that resolution
// and never rewrites a reference. It prevents the obvious refusals in place (self-reference, a
// choice that would close a cycle, a repeated operand) with the reason shown, and leaves every
// other judgement to the operation engine, whose refusals are reported by name.
import type { EditorOperationIntent, PreviewModel } from "../../types";
import type { Combination } from "./combinationOperands";

export type OperandKind = "load_case" | "combination";

/** The bases whose expression is `terms`: at least one term, finite factors, no repeated operand. */
export const TERMS_BASES = ["mechanics", "absolute_sum", "srss"] as const;
export const UNSIGNED_BASES = ["absolute_sum", "srss"] as const;

export function isTermsBasis(basis: string): boolean {
  return (TERMS_BASES as readonly string[]).includes(basis);
}

export function isUnsignedBasis(basis: string): boolean {
  return (UNSIGNED_BASES as readonly string[]).includes(basis);
}

export const OPERAND_CYCLE_CODE = "LOAD_COMBINATION_OPERAND_CYCLE";
export const OPERAND_UNKNOWN_CODE = "LOAD_COMBINATION_OPERAND_UNKNOWN";
export const DIFFERENCE_BASIS_OPERAND_CODE = "LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE";

const hasLoadCase = (model: PreviewModel, id: string) => model.load_cases.some((loadCase) => loadCase.id === id);
const findCombination = (model: PreviewModel, id: string) => (model.combinations ?? []).find((combination) => combination.id === id);

export type ResolvedReference = {
  id: string;
  kind: OperandKind | "unknown";
  /** A combination also has this id; the bare reference names the load case. */
  shadowsCombination: boolean;
};

/** (r1 F3): a bare reference names the load case with the id, else the combination. */
export function resolveReference(model: PreviewModel, id: string): ResolvedReference {
  const loadCase = hasLoadCase(model, id);
  const combination = Boolean(findCombination(model, id));
  return {
    id,
    kind: loadCase ? "load_case" : combination ? "combination" : "unknown",
    shadowsCombination: loadCase && combination
  };
}

export function resolvedReferenceText(model: PreviewModel, id: string): string {
  if (!id) return "not chosen";
  const resolved = resolveReference(model, id);
  if (resolved.kind === "unknown") return `${id}: no load case or combination has this id (the engine refuses it, ${OPERAND_UNKNOWN_CODE})`;
  const kind = resolved.kind === "load_case" ? "load case" : "combination";
  return `${id} resolves to the ${kind}${resolved.shadowsCombination ? "; a combination also has this id and is not named by it" : ""}`;
}

/** The combinations a stored combination names: its combination terms, and its bare references that resolve to a combination. */
export function namedCombinations(model: PreviewModel, combination: Combination): string[] {
  const named: string[] = [];
  for (const term of combination.terms ?? []) {
    if (typeof term.combination === "string") named.push(term.combination);
  }
  for (const ref of [combination.minuend_id, combination.subtrahend_id, ...(combination.operand_ids ?? [])]) {
    if (typeof ref === "string" && resolveReference(model, ref).kind === "combination") named.push(ref);
  }
  return named;
}

/** A path of combination ids from `fromId` to `toId` along named operands, or null. */
export function operandPath(model: PreviewModel, fromId: string, toId: string): string[] | null {
  const parent = new Map<string, string | null>([[fromId, null]]);
  const queue = [fromId];
  while (queue.length) {
    const current = queue.shift()!;
    if (current === toId) {
      const path: string[] = [];
      for (let step: string | null = current; step !== null; step = parent.get(step) ?? null) path.unshift(step);
      return path;
    }
    const combination = findCombination(model, current);
    if (!combination) continue;
    for (const next of namedCombinations(model, combination)) {
      if (!parent.has(next)) {
        parent.set(next, current);
        queue.push(next);
      }
    }
  }
  return null;
}

/** Why naming combination `operandId` from combination `targetId` is refused, or null. */
export function combinationOperandRefusal(model: PreviewModel, targetId: string, operandId: string): string | null {
  if (!operandId) return null;
  if (operandId === targetId) return `${operandId} would name itself (self-reference; the engine refuses it as ${OPERAND_CYCLE_CODE})`;
  const path = operandPath(model, operandId, targetId);
  if (path) return `${operandId} would close a cycle (${[...path, operandId].join(" → ")}; the engine refuses it as ${OPERAND_CYCLE_CODE})`;
  return null;
}

/** Combinations a bare reference cannot name, because a load case has the same id. */
export function shadowedCombinationIds(model: PreviewModel): string[] {
  return (model.combinations ?? []).filter((combination) => hasLoadCase(model, combination.id)).map((combination) => combination.id);
}

export type TermDraft = { kind: OperandKind; ref: string; factor: string };

export function parseFiniteFactor(raw: string): number | null {
  const trimmed = raw.trim();
  if (!/^[+-]?(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?$/.test(trimmed)) return null;
  const parsed = Number.parseFloat(trimmed);
  return Number.isFinite(parsed) ? parsed : null;
}

/** The stored term a draft writes, exactly: `{"load_case": id, "factor": f}` or `{"combination": id, "factor": f}`. */
export function termPayload(term: TermDraft): { load_case: string; factor: number } | { combination: string; factor: number } {
  const factor = parseFiniteFactor(term.factor) ?? 0;
  return term.kind === "combination" ? { combination: term.ref, factor } : { load_case: term.ref, factor };
}

export function termDraftText(term: TermDraft): string {
  return `${term.kind === "combination" ? `combination ${term.ref || "TBD"}` : term.ref || "TBD"} x ${term.factor.trim() || "TBD"}`;
}

/** Why a term list cannot be queued for combination `targetId`, or null when it can. */
export function termsDraftProblem(model: PreviewModel, terms: readonly TermDraft[], targetId: string): string | null {
  if (terms.length === 0) return "add at least one term";
  const seen = new Set<string>();
  for (const [index, term] of terms.entries()) {
    const place = `term ${index + 1}`;
    if (!term.ref) return `${place}: not chosen: its ${term.kind === "combination" ? "combination" : "load case"}`;
    if (term.kind === "load_case" ? !hasLoadCase(model, term.ref) : !findCombination(model, term.ref)) {
      return `${place}: ${term.ref} is not a ${term.kind === "combination" ? "combination" : "load case"} of this model`;
    }
    if (term.kind === "combination") {
      const refusal = combinationOperandRefusal(model, targetId, term.ref);
      if (refusal) return `${place}: ${refusal}`;
    }
    if (!term.factor.trim()) return `${place}: not entered: its factor`;
    if (parseFiniteFactor(term.factor) === null) return `${place}: enter a finite factor`;
    const key = `${term.kind}\u0000${term.ref}`;
    if (seen.has(key)) return `${place}: repeats ${term.kind === "combination" ? "combination" : "load case"} ${term.ref}; one factor per operand`;
    seen.add(key);
  }
  return null;
}

/** Move the term at `index` by `offset` (−1 up, +1 down); out-of-range moves leave the list as it is. */
export function moveTerm<T>(terms: readonly T[], index: number, offset: -1 | 1): T[] {
  const target = index + offset;
  if (index < 0 || index >= terms.length || target < 0 || target >= terms.length) return [...terms];
  const next = [...terms];
  [next[index], next[target]] = [next[target], next[index]];
  return next;
}

/** Stored members the editor does not edit; each is retained as authored by the field-level edits. */
export function retainedCombinationMembers(combination: Combination): string[] {
  const edited = new Set(["id", "label", "basis", "terms", "provenance"]);
  return Object.keys(combination).filter((key) => !edited.has(key));
}

/** A support the solve may consume as a fixed-load source: the constant-effort hanger type or family, not nonlinear. The solve decides whether it consumes it. */
export function constantEffortSupportIds(model: PreviewModel): string[] {
  return model.supports
    .filter((support) => {
      const record = support as unknown as { family?: unknown; nonlinear?: unknown; hanger?: { hanger_type?: unknown } | null };
      if (record.nonlinear !== undefined && record.nonlinear !== null) return false;
      const hangerType = typeof record.hanger?.hanger_type === "string" && record.hanger.hanger_type.trim() ? record.hanger.hanger_type.trim() : record.family;
      return hangerType === "constant_effort_support";
    })
    .map((support) => support.id);
}

export function nonlinearSupportIds(model: PreviewModel): string[] {
  return model.supports
    .filter((support) => {
      const nonlinear = (support as unknown as { nonlinear?: unknown }).nonlinear;
      return nonlinear !== undefined && nonlinear !== null;
    })
    .map((support) => support.id);
}

export const UNSIGNED_GATE_TEXT =
  "§5.1: where the model has a fixed-load source (a constant-effort support the solve consumes; on a document with a pressure contract, a connector's or tie rod's nonzero installed reference), the solve withholds an absolute sum or SRSS unless every operand's fixed-load count κ is exactly 0 (CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND, INSTALLED_REFERENCE_UNSIGNED_COMBINATION_OPERAND). A load case has κ = 1; a load case minus a load case has κ = 0.";
export const UNSIGNED_ROW_TEXT =
  "§4, §5.2: an absolute sum or SRSS is a row operation and claims no state; its rows are unsigned and read \"not an equilibrium state\". In a model with nonlinear supports it is formed from rows with each operand's standing.";
export const NONLINEAR_MECHANICS_TEXT =
  "§5.2 (OQ-1): in a model with nonlinear supports the solve withholds a mechanics combination (NONLINEAR_COMBINATION_REQUIRES_SOLVE) unless one term names a solved load case at factor exactly 1 and every other term (at least one) names a combination whose κ is exactly 0; such a row is a solved state plus derived differences or bounds, not an equilibrium state.";

const DIMENSIONLESS_UNIT_VALIDATION_STATUS = "not_required_dimensionless";

const OPERATION_ID_SAFE = /^[A-Za-z0-9_.:-]$/u;
const UTF8 = new TextEncoder();

/** An id as it appears in the load manager's `operation_id` and `change_id`: a reversible
 * escape, so distinct raw ids always give distinct tokens. A character in [A-Za-z0-9_.:-] is
 * kept; any other (`~` included) is written as `~XX` per UTF-8 byte, uppercase hex. An id of
 * safe characters only is its own token, so existing operation ids are unchanged. Target ids
 * and payload values are never encoded. */
export function operationIdToken(value: string): string {
  let token = "";
  for (const character of value) {
    if (OPERATION_ID_SAFE.test(character)) token += character;
    else for (const byte of UTF8.encode(character)) token += `~${byte.toString(16).toUpperCase().padStart(2, "0")}`;
  }
  return token;
}

/** The `create_combination` intent of the Load Cases manager for an explicit payload; the operation engine judges it. */
export function createCombinationIntent({
  model,
  payload,
  rationale,
  sourceNote
}: {
  model: PreviewModel;
  payload: Record<string, unknown> & { id: string };
  rationale: string;
  sourceNote: string;
}): EditorOperationIntent {
  const combinationId = payload.id;
  const operationToken = `create-${operationIdToken(combinationId)}`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "create",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Combination",
      ref: combinationId
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "create_combination",
      field_label: "Combination",
      field_path: "combinations",
      before: "not_present",
      after: JSON.stringify(payload),
      unit: "none",
      dimension: "dimensionless",
      source_note: sourceNote
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: DIMENSIONLESS_UNIT_VALIDATION_STATUS,
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: rationale.trim() || `combination creation intent for ${model.project.id}`
  };
}
