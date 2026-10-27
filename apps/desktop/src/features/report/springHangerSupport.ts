/** Which supports the report treats as spring hangers or constant-effort supports: the rule the
 * report's spring hanger evidence, review rows and provenance use (`ReportPanel.tsx`,
 * `renderableReportInput.ts`), kept here so other surfaces apply the same rule. A8's "Hangers"
 * filter uses it (DEL-07-05 Design `displacement-extremes.md` §2.1).
 *
 * A support counts when its `family` or its `hanger.hanger_type` is `variable_spring_hanger` or
 * `constant_effort_support`. The schema's third hanger spelling, `spring_hanger`, does not count
 * under this rule; whether it should is open (Design §10). */

import type { PreviewModel } from "../../types";

export function isSpringHangerSupport(support: PreviewModel["supports"][number]): boolean {
  return (
    support.family === "variable_spring_hanger" ||
    support.family === "constant_effort_support" ||
    support.hanger?.hanger_type === "variable_spring_hanger" ||
    support.hanger?.hanger_type === "constant_effort_support"
  );
}
