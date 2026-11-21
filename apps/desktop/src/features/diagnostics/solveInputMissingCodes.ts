// The diagnostic codes that mean a solve input is missing (DEL-07-06 AC-007, Missing
// data): a value the person must enter, on the entity the finding's affected
// references name. The list is `fixtures/diagnostics/solve_input_missing_codes.json`,
// which the engine reads too (`open_pipe_stress_product_physics::solve_input_missing_codes`);
// a reader decides by a finding's code, never by its message text.
import list from "../../../../../fixtures/diagnostics/solve_input_missing_codes.json";
import type { Diagnostic } from "../../types";

export const SOLVE_INPUT_MISSING_CODES: ReadonlySet<string> = new Set(list.codes);

/** Whether `diagnostic` states that a solve input is missing. */
export function isSolveInputMissing(diagnostic: Pick<Diagnostic, "code">): boolean {
  return SOLVE_INPUT_MISSING_CODES.has(diagnostic.code);
}
