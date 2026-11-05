// Model document 0.9.0 S3b (DEL-02-01 Work `model-document-0.9.0.md` §2.2, §4.2), owned by
// DEL-07-03: the combination editor's "Operating minus sustained difference" template. It is
// offered only on a result-state subtraction. The person chooses the operating case (minuend)
// and the sustained case (subtrahend); the template then writes exactly those two and
// `difference_basis` through the existing `create_combination` route. It composes nothing
// (no implicit W+P+T), carries no code or range meaning and is never called "expansion": that
// naming belongs to the user's rule pack. `subtrahend_support_state` is never written
// (`own` behaves as absence; `minuend_final` is refused and not offered); a stored value is
// shown as retained. The solve's gates (G-1, G-2) are the solve's: nothing is checked here.
import type { PreviewModel } from "../../types";

export const OPERATING_DIFFERENCE_BASIS = "operating_minus_sustained";
export const OPERATING_DIFFERENCE_TEMPLATE_LABEL = "Operating minus sustained difference";
export const OPERATING_DIFFERENCE_EXACT_REFUSAL = {
  code: "LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED",
  text: "A combination's difference_basis is refused on a document with a pressure contract; the exact route's operating comparison is not built."
} as const;
export const OPERATING_DIFFERENCE_SOLVE_NOTE =
  "At solve, the engine checks that the sustained case's loads are contained in the operating case's and that both cases share one elastic (modulus) basis; where not, it withholds the combination with a warning.";

type Combination = NonNullable<PreviewModel["combinations"]>[number];

/** The declared-comparison members a stored combination carries, as authored. */
export function combinationDeclaredMembersText(combination: Combination): string {
  const parts: string[] = [];
  if (combination.difference_basis !== undefined) parts.push(`; difference_basis=${combination.difference_basis}`);
  if (combination.subtrahend_support_state !== undefined) {
    parts.push(`; subtrahend_support_state=${combination.subtrahend_support_state} (retained as authored; not written by this editor)`);
  }
  return parts.join("");
}
