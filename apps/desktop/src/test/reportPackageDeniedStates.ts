/** DEL-08-01 REQ-024, "Package and user release": the three states a report package is denied
 * for, each from committed producer bytes, as the session test
 * (`reportPackageAdmission.session.test.tsx`) and the App test (`renderedReportGate.app.test.tsx`)
 * replay them: a pressure-free result that is not numerically eligible, an unsolved result and
 * an unreadable result. Each names the job's final state and the refusal the four package
 * commands leave. Every value is invented. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { nativeMechanicsReplayPair } from "./nativeMechanicsReplay";
import type { PreviewModel } from "../types";

const root = resolve(__dirname, "../../../../");
const text = (path: string) => readFileSync(resolve(root, path), "utf8");
const WITNESS = "fixtures/results/invented/agent_solve_witness";

/** A pressure-free Euler–Bernoulli result, current but for its numerical health (it has no
 * rendered report, so the sentence gives no advice). */
export const REPORT_PACKAGE_NOT_ELIGIBLE = "Report package not saved: this result's numerical health does not allow one. Nothing was written.";
/** A solve output the rendered report does not take, on no ground the session can name. */
export const REPORT_PACKAGE_NO_REPORT = "Report package not saved: no report is available for this solve. Nothing was written.";
export const REPORT_PACKAGE_SOLVE_FIRST = "Report package not saved: the model has no current result. Solve it, then save. Nothing was written.";

export type DeniedState = { model: PreviewModel; bytes: string; state: "completed" | "failed" };

/** Each state's name, its maker and its refusal. */
export const REPORT_PACKAGE_DENIED_STATES: [string, () => DeniedState, string][] = [
  ["a pressure-free result that is not numerically eligible", () => {
    // The pressure-free pair, its one case marked sensitive by the solver's own quality fields.
    const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    const source = JSON.parse(bytes);
    source.numerical_quality.status = "sensitive";
    source.numerical_quality.cases[0].solve_quality = "sensitive";
    return { model, bytes: JSON.stringify(source), state: "completed" };
  }, REPORT_PACKAGE_NOT_ELIGIBLE],
  ["an unsolved result", () => {
    // The producer's blocked document (MODEL_INCOMPLETE), its exact bytes.
    const request = JSON.parse(text(`${WITNESS}/blocked_retained_evaluation.request.json`));
    return { model: request.model as PreviewModel, bytes: text(`${WITNESS}/blocked_retained_evaluation-sparse_interactive.raw.json`), state: "completed" };
  }, REPORT_PACKAGE_NO_REPORT],
  ["an unreadable result", () => {
    // The exact pair with forged numerical evidence: the reader refuses it, and the session holds no result.
    const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
    const source = JSON.parse(bytes);
    source.contract_evidence.exact_cases[0].pipe_sections[0].nominal_section.As_m2 *= 1.01;
    return { model, bytes: JSON.stringify(source), state: "failed" };
  }, REPORT_PACKAGE_SOLVE_FIRST]
];
