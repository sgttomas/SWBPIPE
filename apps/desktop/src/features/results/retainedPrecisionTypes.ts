/** The retained-precision reader's vocabulary on the desktop, with no validation: its outcome types,
 * its refusal, the W1 row token and the binary64 word codec. The reader is the Rust one, through
 * `retainedPrecisionReader.ts` (R1, b7-plan Unit R). */

/** The W1 recovery method a retained row carries. */
export const RETAINED_METHOD = "contribution_preserving_multiprecision_v1";

/** The reader's refusal: its gate, its bare code (the error message) and any detail it carries. */
export class RetainedPrecisionError extends Error {
  constructor(readonly gate: string, readonly code: string, readonly detail: string | null = null) { super(code); }
}

export type AccuracyClass = "relative_verified" | "absolute_verified" | "input_derived" | "non_quantity" | "not_covered";
export type RowClassification = Readonly<{
  result_id: string;
  basis_ref: Readonly<{ ref_type: string; ref_id: string }>;
  normalized_bits: string;
  scale_bits: string | null;
  bound_bits: string | null;
  class: AccuracyClass;
}>;
export type RetainedPrecisionValidation = Readonly<{
  invocation_bound: boolean;
  numerical_eligible: boolean;
  standing: "eligible" | "needs_recompute";
  publication_sha256: string;
  classifications: readonly RowClassification[];
}>;

/** A binary64 value's word, as 16 lowercase hex digits. */
export function binary64Bits(value: number): string {
  const view = new DataView(new ArrayBuffer(8)); view.setFloat64(0, value, false);
  return view.getBigUint64(0, false).toString(16).padStart(16, "0");
}
/** The finite binary64 value of a 16-hex-digit word. */
export function decodeBinary64(value: string): number {
  if (typeof value !== "string" || !/^[0-9a-f]{16}$/.test(value)) throw new Error("binary64 encoding");
  const view = new DataView(new ArrayBuffer(8)); view.setBigUint64(0, BigInt(`0x${value}`), false);
  const result = view.getFloat64(0, false);
  if (!Number.isFinite(result)) throw new Error("nonfinite binary64");
  return result;
}
