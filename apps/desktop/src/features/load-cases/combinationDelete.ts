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
import type { Combination, CombinationTerm } from "./combinationOperands";

export const DELETE_REFERENCED_CODE = "OP-COMBINATION-DELETE-REFERENCED";

/** Asynchronous canonical JSON (the wasm engine's RFC 8785 form), as `canonicalJsonString`. */
export type Canonicalize = (value: unknown) => Promise<string>;

/** The engine's `display_number`: an integral value below 1e15 in magnitude as an integer,
 * otherwise Rust's `{}` for f64 (the shortest round-trip digits, never an exponent). */
export function engineDisplayNumber(value: number): string {
  if (Number.isInteger(value) && Math.abs(value) < 1e15) return String(Math.trunc(value) + 0);
  return plainDecimal(value);
}

function plainDecimal(value: number): string {
  const sign = value < 0 ? "-" : "";
  const text = String(Math.abs(value));
  const [mantissa, exponentText] = text.split("e");
  const exponent = exponentText === undefined ? 0 : Number(exponentText);
  const pointAt = mantissa.indexOf(".");
  const digits = mantissa.replace(".", "");
  const point = (pointAt === -1 ? mantissa.length : pointAt) + exponent;
  let body: string;
  if (point <= 0) body = `0.${"0".repeat(-point)}${digits}`;
  else if (point >= digits.length) body = `${digits}${"0".repeat(point - digits.length)}`;
  else body = `${digits.slice(0, point)}.${digits.slice(point)}`;
  body = body.replace(/^0+(?=\d)/, "");
  return `${sign}${body}`;
}

// The applier trims with Rust's `str::trim` (Unicode White_Space: U+0085 is trimmed, U+FEFF
// is not), so every trim here is `trimLikeRust`, never `String.prototype.trim`.
const textOr = (value: unknown, fallback: string) => (typeof value === "string" ? trimLikeRust(value) : fallback);

/** A flat term's historical display, or null for any other stored term (engine `combination_term_display`). */
export function historicalTermDisplay(term: unknown): string | null {
  if (!term || typeof term !== "object" || Array.isArray(term)) return null;
  const record = term as Record<string, unknown>;
  const loadCase = typeof record.load_case === "string" ? trimLikeRust(record.load_case) : "";
  const factor = record.factor;
  if (!loadCase || typeof factor !== "number" || !Number.isFinite(factor)) return null;
  return `${loadCase} x ${engineDisplayNumber(factor)}`;
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
