/** T6S (I74 PLAN 1.2-1.3; RR decisions 4 and 5; D-U6-2): the export-side
 * disclosure of the F2a preview successor's verified-accuracy classes.
 *
 * The reason codes and messages are Rust `derivative::class_disclosure`'s, byte for
 * byte (`RE/src/derivative.rs`), including the receipt's absolute bound b printed as
 * Rust's `{:e}` prints an f64 (CQ-1). The result-JSON derivative discloses each
 * `absolute_verified` or `not_covered` row with them, and the stress-neutral package
 * withholds the row's unit-preservation witness with a finding carrying the same
 * message (S-d). One text per language (PLAN 4.3): S-I2 changes Rust's constant,
 * this module and the stress-neutral finding together.
 *
 * Classes come only from the accepted reader, run on the statement without an
 * invocation, as Rust `semantic_contract::retained_row_classes` does. Output code
 * never parses the receipt; it copies it whole. Text and disclosure only: nothing
 * here changes standing, and nothing claims a stop-rule bound, an extrema
 * enclosure or producer origin (D-U7-6).
 */
import { decodeBinary64, validateRetainedPrecision, type RowClassification } from "./retainedPrecision";
import { classificationSummary } from "./retainedPrecisionStanding";
import { isRetainedIdentity, isRetainedRoute, sourceContract } from "./numericalResultQuality";
import type { MechanicsResult, PreviewModel } from "../../types";

/** D-U6-2's two `RowDisclosure` reason codes (Rust `RETAINED_ABSOLUTE_VERIFIED`, `RETAINED_NOT_COVERED`). */
export const RETAINED_ABSOLUTE_VERIFIED = "retained_precision_absolute_verified";
export const RETAINED_NOT_COVERED = "retained_precision_not_covered";
export type RetainedDisclosureCode = typeof RETAINED_ABSOLUTE_VERIFIED | typeof RETAINED_NOT_COVERED;

/** Rust `derivative::si_unit`: the SI unit the reader normalizes a row's unit to,
 * and so the unit the receipt's absolute bound b is published in. */
const SI_UNIT: Readonly<Record<string, string>> = Object.freeze({ m: "m", mm: "m", rad: "rad", N: "N", kN: "N", "N*m": "N*m", "kN*m": "N*m", Pa: "Pa", MPa: "Pa" });
export function retainedSiUnit(unit: string): string | null {
  return Object.hasOwn(SI_UNIT, unit) ? SI_UNIT[unit] : null;
}

/** An f64 as Rust's `{:e}` prints it (CQ-1): the shortest round-trip digits,
 * closest to the value, one digit before the point, and an exponent with no `+`.
 * JavaScript's `toExponential()` without an argument gives the same shortest digits,
 * except when the value lies exactly halfway between two shortest candidates: V8
 * then takes the even one and Rust the larger magnitude (RV101 SF-1; REPAIR_01).
 * So, with n the shortest form's digit count, `toExponential(n - 1)` gives the n-digit
 * decimal nearest the value, which ECMAScript resolves on a tie to the larger
 * candidate, as Rust does. Printed when it round-trips, it differs from the shortest
 * form only on such a tie, which binary64 admits only at 16 or 17 digits. When it
 * does not round-trip (at a power of two, whose round-trip interval is narrower
 * below), the shortest form stands. The `e+` exponent is normalized to `e`,
 * and the sign of a negative zero, which `toExponential` drops, is kept. */
export function rustLowerExp(value: number): string {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Error("RETAINED_PRECISION_BOUND_NOT_FINITE");
  const sign = value < 0 || Object.is(value, -0) ? "-" : "";
  const magnitude = Math.abs(value);
  const shortest = magnitude.toExponential();
  const nearest = magnitude.toExponential(shortest.split("e")[0].replace(".", "").length - 1);
  return `${sign}${(Number(nearest) === magnitude ? nearest : shortest).replace("e+", "e")}`;
}

/** Rust `not_covered_message`. */
export function retainedNotCoveredMessage(kind: string): string {
  return `${kind}: ${RETAINED_NOT_COVERED}; no verified accuracy for this quantity kind; source value/unit and annotation retained; withheld from rule binding and reliance`;
}
/** Rust `class_disclosure`: the reason code and message of one validated class of a
 * row with this `kind` and source `unit`, or null for a class that is not disclosed.
 * An `absolute_verified` row names the receipt's bound in the SI unit; were the unit
 * one the reader does not normalize, the row is still withheld, with no bound claimed. */
export function retainedClassDisclosure(kind: string, unit: string, classified: RowClassification | null | undefined): { code: RetainedDisclosureCode; message: string } | null {
  if (!classified) return null;
  if (classified.class === "absolute_verified") {
    const si = retainedSiUnit(unit);
    if (si === null) return { code: RETAINED_NOT_COVERED, message: retainedNotCoveredMessage(kind) };
    const bits = classified.bound_bits;
    if (typeof bits !== "string" || !/^[0-9a-f]{16}$/.test(bits)) throw new Error("RETAINED_PRECISION_BOUND_MISSING");
    return {
      code: RETAINED_ABSOLUTE_VERIFIED,
      message: `${kind}: ${RETAINED_ABSOLUTE_VERIFIED}; verified only to the receipt's absolute bound b = ${rustLowerExp(decodeBinary64(bits))} ${si} (binary64 ${bits}), below the relative accuracy floor; source value/unit and annotation retained; withheld from rule binding and reliance`,
    };
  }
  if (classified.class === "not_covered") return { code: RETAINED_NOT_COVERED, message: retainedNotCoveredMessage(kind) };
  return null;
}

/** Rust `retained_row_classes`: validated row classes by result id, from the accepted
 * reader run on these bytes without an invocation; null for any other identity. A
 * statement the reader refuses throws the reader's code. */
export async function retainedRowClassesFromReader(source: MechanicsResult): Promise<ReadonlyMap<string, RowClassification> | null> {
  if (!isRetainedIdentity(source?.producer?.semantic_contract_id)) return null;
  const validation = await validateRetainedPrecision(source);
  return new Map(validation.classifications.map(row => [row.result_id, row]));
}

/** D2 4.9.9's UI summary for the two export panels: one text-only line, per case,
 * of the quantities verified only to an absolute bound and those uncovered, from the
 * carriers' `classificationSummary(result, model)`. Null unless the result is a
 * successor with a validated registration. It changes no standing. */
export function retainedPrecisionSummaryLine(result: MechanicsResult | null | undefined, model: PreviewModel | null | undefined): string | null {
  if (!result) return null;
  try { if (!isRetainedRoute(sourceContract(result))) return null; } catch { return null; }
  const summary = classificationSummary(result, model);
  if (!summary.length) return null;
  return `Retained precision, per case: ${summary.map(c => `${String(c.case_id)}: ${c.absolute_verified} verified only to an absolute bound; ${c.not_covered} uncovered`).join(". ")}.`;
}
