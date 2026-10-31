// The operands of a combination as the desktop's views show them (model document
// 0.9.0 §2.4). A term names a load case or a combination; a subtraction or a
// range envelope names its operands in its own members and may omit `terms`.
// Nothing here fills a default in: an absent `terms` is shown as absent, never
// as an empty sum.
import type { PreviewModel } from "../../types";

export type Combination = NonNullable<PreviewModel["combinations"]>[number];
export type CombinationTerm = NonNullable<Combination["terms"]>[number];

/** A load-case term shows its load case id, as the engine's displays do; a
 * combination term names itself as a combination. */
export function combinationTermOperand(term: CombinationTerm): string {
  return term.load_case ?? `combination ${term.combination}`;
}

export function combinationTermText(term: CombinationTerm): string {
  return `${combinationTermOperand(term)} x ${String(term.factor)}`;
}

/** The stored terms joined by "; ", or null when the record carries no `terms`. */
export function combinationTermsText(combination: Combination): string | null {
  return combination.terms ? combination.terms.map(combinationTermText).join("; ") : null;
}
