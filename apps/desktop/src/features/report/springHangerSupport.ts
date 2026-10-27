/** Which supports the report treats as spring hangers or constant-effort supports: the rule the
 * report's spring hanger evidence, review rows and provenance use (`ReportPanel.tsx`,
 * `renderableReportInput.ts`), kept here so other surfaces apply the same rule. A8's "Hangers"
 * filter uses it (DEL-07-05 Design `displacement-extremes.md` §2.1).
 *
 * The rule is the solver's (`core/product_physics/src/lib.rs`): `support_hanger_type` reads the
 * trimmed `hanger.hanger_type` when it is non-empty, otherwise the trimmed `family`; a support
 * counts when that reading is `variable_spring_hanger` or `spring_hanger`
 * (`is_variable_spring_hanger`) or `constant_effort_support` (`is_constant_effort_support`), the
 * same union the solver uses to emit its hanger review rows
 * (`append_spring_hanger_user_input_results`). An explicit hanger type therefore decides over the
 * family. The report still counts a support declared a hanger without a `hanger` record, and
 * reports its missing hanger data, as before. */

import type { PreviewModel } from "../../types";

type Support = PreviewModel["supports"][number];

const HANGER_TYPES: ReadonlySet<string> = new Set(["variable_spring_hanger", "spring_hanger", "constant_effort_support"]);

/** The solver's `support_hanger_type`: the trimmed explicit hanger type, else the trimmed family. */
function supportHangerType(support: Support): string | null {
  const explicit = support.hanger?.hanger_type;
  const trimmed = typeof explicit === "string" ? explicit.trim() : "";
  if (trimmed !== "") return trimmed;
  return typeof support.family === "string" ? support.family.trim() : null;
}

export function isSpringHangerSupport(support: Support): boolean {
  const hangerType = supportHangerType(support);
  return hangerType !== null && HANGER_TYPES.has(hangerType);
}
