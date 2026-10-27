/** Which supports the report treats as spring hangers or constant-effort supports: the rule the
 * report's spring hanger evidence, review rows, provenance and summary use (`ReportPanel.tsx`,
 * `renderableReportInput.ts`), kept here so other surfaces apply the same rule. A8's "Hangers"
 * filter uses it (DEL-07-05 Design `displacement-extremes.md` §2.1).
 *
 * The rule is the solver's (`core/product_physics/src/lib.rs`): `support_hanger_type` reads the
 * trimmed `hanger.hanger_type` when it is non-empty, otherwise the trimmed `family`; a support
 * counts when that reading is `variable_spring_hanger` or `spring_hanger`
 * (`is_variable_spring_hanger`) or `constant_effort_support` (`is_constant_effort_support`), the
 * same union the solver uses to emit its hanger review rows
 * (`append_spring_hanger_user_input_results`). An explicit hanger type therefore decides over the
 * family. Trimming is Rust's `str::trim` (`trimLikeRust`), not JavaScript's `String.prototype.trim`.
 * The report still counts a support declared a hanger without a `hanger` record, and reports its
 * missing hanger data, as before. */

import type { PreviewModel } from "../../types";

type Support = PreviewModel["supports"][number];

/** The characters Rust's `str::trim` removes from each end: those for which `char::is_whitespace`
 * holds, the Unicode `White_Space` property (Rust std `char::is_whitespace`; Unicode
 * `PropList.txt`, `White_Space`). All lie in the BMP. It differs from JavaScript's `trim`, which
 * also removes U+FEFF and keeps U+0085; neither removes U+001C–U+001F. Checked against rustc 1.97.1
 * (Unicode 17.0.0) over every code point. */
export const RUST_WHITESPACE: ReadonlySet<number> = new Set([
  0x0009, 0x000a, 0x000b, 0x000c, 0x000d, 0x0020, 0x0085, 0x00a0, 0x1680,
  0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007, 0x2008, 0x2009, 0x200a,
  0x2028, 0x2029, 0x202f, 0x205f, 0x3000
]);

/** Rust's `str::trim`. Every member of `RUST_WHITESPACE` is a single UTF-16 code unit and no
 * surrogate is a member, so scanning code units equals scanning Rust's `char`s. */
export function trimLikeRust(value: string): string {
  let start = 0;
  let end = value.length;
  while (start < end && RUST_WHITESPACE.has(value.charCodeAt(start))) start += 1;
  while (end > start && RUST_WHITESPACE.has(value.charCodeAt(end - 1))) end -= 1;
  return value.slice(start, end);
}

const VARIABLE_TYPES: ReadonlySet<string> = new Set(["variable_spring_hanger", "spring_hanger"]);
const CONSTANT_EFFORT_TYPE = "constant_effort_support";

/** The solver's `support_hanger_type`: the trimmed explicit hanger type, else the trimmed family. */
function supportHangerType(support: Pick<Support, "family" | "hanger">): string | null {
  const explicit = support.hanger?.hanger_type;
  const trimmed = typeof explicit === "string" ? trimLikeRust(explicit) : "";
  if (trimmed !== "") return trimmed;
  return typeof support.family === "string" ? trimLikeRust(support.family) : null;
}

/** The solver's subtype: `variable` for `is_variable_spring_hanger`, `constant_effort` for
 * `is_constant_effort_support`, otherwise null. */
export function springHangerKind(support: Pick<Support, "family" | "hanger">): "variable" | "constant_effort" | null {
  const hangerType = supportHangerType(support);
  if (hangerType === null) return null;
  if (VARIABLE_TYPES.has(hangerType)) return "variable";
  return hangerType === CONSTANT_EFFORT_TYPE ? "constant_effort" : null;
}

export function isSpringHangerSupport(support: Pick<Support, "family" | "hanger">): boolean {
  return springHangerKind(support) !== null;
}
