/** Row labels over the Rust reader's row semantics (`resultReader.ts`,
 * `reader_verdict::row_semantics`): a row's family, dimension and category as the
 * desktop shows them. R2 (b7-plan Unit R): the signature comes from the Rust reader of
 * the source's table; nothing here reads a table. */
import { readerRowSemantics, type RowSemantics } from "./resultReader";
import type { MechanicsResult } from "../../types";
export type SourceRow = MechanicsResult["results"][number];
export type RowLabels = Readonly<{
  /** The row's semantics; throws the reader's refusal (an unsupported source or a contradictory row). */
  semantics: (row: SourceRow) => RowSemantics;
  /** A physical quantity's family, else `other`. */
  family: (row: SourceRow) => string;
  /** The derivative target dimension, or null. */
  dimension: (row: SourceRow) => string | null;
  /** The table category; `unknown` without a signature, `integrity_failure` when refused. */
  category: (row: SourceRow) => string;
}>;
/** The labels of one source's rows (the legacy table without a source). Read every row
 * of a source through one call. */
export function semanticLabels(source?: MechanicsResult | null): RowLabels {
  const semantics = readerRowSemantics(source);
  const signature = (row: SourceRow) => semantics(row).signature;
  return {
    semantics,
    family: row => { try { const s = signature(row); return s?.category === "physical_quantity" ? s.family ?? "other" : "other"; } catch { return "other"; } },
    dimension: row => { try { return signature(row)?.derivative_target_dimension ?? null; } catch { return null; } },
    category: row => { try { return signature(row)?.category ?? "unknown"; } catch { return "integrity_failure"; } },
  };
}
export function semanticFamily(row: SourceRow, source?: MechanicsResult): string {
  return semanticLabels(source).family(row);
}
export function semanticDimension(row: SourceRow, source?: MechanicsResult): string | null {
  return semanticLabels(source).dimension(row);
}
export function semanticCategory(row: SourceRow, source?: MechanicsResult): string {
  return semanticLabels(source).category(row);
}
