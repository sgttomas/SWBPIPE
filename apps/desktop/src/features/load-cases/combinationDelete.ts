// Deleting combinations and their terms on the applier contract of #115 (DEL-05-02 Work
// `absolute-sum-srss-combinations.md` §10.5; owned by DEL-07-03). Each `before` value is
// built only from the stored record or term, never by coercing it to the flat display:
// - a record whose `terms` is an array of flat `{load_case, factor}` terms (empty included,
//   whatever the basis) keeps the engine's historical display
//   (`combination_delete_display`); every other record is its canonical JSON;
// - a flat term keeps `"<load_case> x <factor>"`; every other term is its canonical JSON.
// A combination still named by another is refused by the engine
// (`OP-COMBINATION-DELETE-REFERENCED`); the editor lists the referencing paths before the
// attempt and never deletes the referencers.
import type { PreviewModel } from "../../types";
import { trimLikeRust } from "../../services/rustTrim";
import { loadedWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import type { Combination, CombinationTerm } from "./combinationOperands";

export const DELETE_REFERENCED_CODE = "OP-COMBINATION-DELETE-REFERENCED";

/** Asynchronous canonical JSON (the wasm engine's RFC 8785 form), as `canonicalJsonString`. */
export type Canonicalize = (value: unknown) => Promise<string>;

export const NUMBER_DISPLAY_ENGINE_ABSENT =
  "The delete's before value needs the wasm operation engine's number display, which is not loaded; no delete is built.";

/** Thrown when a flat record's or term's historical display needs a factor's text and the
 * wasm engine is not loaded: no `before` is built, and nothing is mirrored. */
export class NumberDisplayUnavailable extends Error {
  constructor() { super(NUMBER_DISPLAY_ENGINE_ABSENT); }
}

/** A factor as the engine prints it: its own `display_number` through the wasm export
 * `display_number_text` (#118), never a desktop mirror (JavaScript and Rust break ties between
 * equally short digit strings differently: 1000000000000000.25 is 1000000000000000.3 there). */
function engineNumberText(value: number): string {
  const text = loadedWasmEngine()?.displayNumberText(value);
  if (text === undefined) throw new NumberDisplayUnavailable();
  return text;
}

// The applier trims with Rust's `str::trim` (Unicode White_Space: U+0085 is trimmed, U+FEFF
// is not), so every trim here is `trimLikeRust`, never `String.prototype.trim`.
const textOr = (value: unknown, fallback: string) => (typeof value === "string" ? trimLikeRust(value) : fallback);

/** A flat term's historical display, or null for any other stored term (engine
 * `combination_term_display`). Throws `NumberDisplayUnavailable` for a flat term while the
 * engine is not loaded. */
export function historicalTermDisplay(term: unknown): string | null {
  if (!term || typeof term !== "object" || Array.isArray(term)) return null;
  const record = term as Record<string, unknown>;
  const loadCase = typeof record.load_case === "string" ? trimLikeRust(record.load_case) : "";
  const factor = record.factor;
  if (!loadCase || typeof factor !== "number" || !Number.isFinite(factor)) return null;
  return `${loadCase} x ${engineNumberText(factor)}`;
}

/** The historical record display, or null when `terms` is absent or any term is not flat (engine `combination_delete_display`). */
export function historicalCombinationDisplay(combination: unknown): string | null {
  if (!combination || typeof combination !== "object" || Array.isArray(combination)) return null;
  const record = combination as Record<string, unknown>;
  if (typeof record.id !== "string" || !Array.isArray(record.terms)) return null;
  const items: string[] = [];
  for (const term of record.terms) {
    const display = historicalTermDisplay(term);
    if (display === null) return null;
    items.push(display);
  }
  return `${trimLikeRust(record.id)}; ${textOr(record.label, "TBD")}; basis=${textOr(record.basis, "TBD")}; terms=${items.length ? items.join("; ") : "none"}`;
}

export async function combinationDeleteBefore(combination: Combination, canonicalize: Canonicalize): Promise<string> {
  return historicalCombinationDisplay(combination) ?? canonicalize(combination);
}

export async function combinationTermDeleteBefore(term: CombinationTerm, canonicalize: Canonicalize): Promise<string> {
  return historicalTermDisplay(term) ?? canonicalize(term);
}

const UTF8 = new TextEncoder();
function byteOrder(left: string, right: string): number {
  const a = UTF8.encode(left);
  const b = UTF8.encode(right);
  for (let index = 0; index < Math.min(a.length, b.length); index += 1) if (a[index] !== b[index]) return a[index] - b[index];
  return a.length - b.length;
}

/** Every other combination's member that names `combinationId`, as the engine lists it
 * (`combination_operand_references`): `<id>.terms.<i>` (any factor, 0 included),
 * `<id>.minuend_id`, `<id>.subtrahend_id`, `<id>.operand_ids.<i>`; sorted and unique. */
export function combinationReferencePaths(model: PreviewModel, combinationId: string): string[] {
  const paths = new Set<string>();
  for (const combination of model.combinations ?? []) {
    const record = combination as unknown as Record<string, unknown>;
    const id = typeof record.id === "string" ? record.id : "combination:unknown";
    if (id === combinationId) continue;
    if (Array.isArray(record.terms)) {
      record.terms.forEach((term, index) => {
        if (term && typeof term === "object" && (term as Record<string, unknown>).combination === combinationId) paths.add(`${id}.terms.${index}`);
      });
    }
    for (const field of ["minuend_id", "subtrahend_id"]) if (record[field] === combinationId) paths.add(`${id}.${field}`);
    if (Array.isArray(record.operand_ids)) {
      record.operand_ids.forEach((operand, index) => { if (operand === combinationId) paths.add(`${id}.operand_ids.${index}`); });
    }
  }
  return [...paths].sort(byteOrder);
}
