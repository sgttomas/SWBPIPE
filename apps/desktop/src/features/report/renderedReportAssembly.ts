/** The rendered report panel's build step (DEC-021 A7; W-11 slice (b), DEL-08-01 Design
 * `exact-route-report-sections.md` §3.1, §3.6). The panel calls it only after its
 * output-policy cell admits the result (`surfaceOutputRefusal` on `rendered-report`),
 * which refuses every exact-route result until the owner opens it (ruling 10A). For an
 * exact source it first takes the Export Result JSON admission (`admitExactReportSource`),
 * builds the input with it, and returns the pre-IPC re-check that yields the texts the
 * native render re-reads: the delivered result bytes, the intake model text and the
 * admission's input manifest text. A pressure-free source takes no admission, so its
 * path is unchanged. Tests reach the exact path below the gate through the test-only
 * name `assembleExactReportBelowOutputGate`; the native render still refuses every
 * exact identity at its step 0. Standard claim fence applies (claims taxonomy per
 * DEC-081). */
import type { AnalysisRunEnvelope, LocalProjectSummary, MechanicsResult, PreviewModel } from "../../types";
import { canonicalJsonString } from "../../services/hashService";
import type { CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import type { NumericalHealthView } from "../results/numericalHealth";
import type { ControlledRouteExport } from "../redaction-controls/redactionExportControls";
import type { ExactRenderSubmission } from "../../services/reportRenderService";
import { admitExactReportSource, exactSourceSubmission, type ExactReportAdmission } from "./exactReportAdmission";
import { isExactReportSource } from "./exactReportSections";
import {
  buildRenderableReportInput,
  type ReportExportContext,
  type SessionRuleCheckRun
} from "./renderableReportInput";
import { controlReportRendererInput } from "./reportRedactionProjector";
import { currentLibraryRefFindings } from "../library/libraryRefFindings";

export type RenderedReportAssemblyInputs = {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  projectSummary: LocalProjectSummary | null;
  inputManifest: CurrentSessionInputManifestEvidence | null;
  ruleCheckRun: SessionRuleCheckRun | null;
  numericalHealth: NumericalHealthView | null;
  context: ReportExportContext;
  intent: boolean;
  generatedAt: string | null;
};

export type RenderedReportAssembly = {
  input: Awaited<ReturnType<typeof buildRenderableReportInput>>;
  controlled: ControlledRouteExport;
  admission: ExactReportAdmission | null;
  /** Null for a pressure-free source. For an exact source: the synchronous live
   * re-check of the admission, run immediately before IPC, returning the texts the
   * native render re-reads; it throws once the admission no longer holds. */
  exactSubmission: (() => ExactRenderSubmission) | null;
};

export async function assembleRenderedReport(inputs: RenderedReportAssemblyInputs): Promise<RenderedReportAssembly> {
  const { model, result, analysisRun, projectSummary, inputManifest, ruleCheckRun, numericalHealth, context, intent, generatedAt } = inputs;
  const exact = isExactReportSource(result);
  const admission = exact
    ? await admitExactReportSource({ model, result, analysisRun, inputManifest: inputManifest as CurrentSessionInputManifestEvidence })
    : null;
  const input = await buildRenderableReportInput({
    model,
    result,
    analysisRun,
    projectSummary,
    inputManifest,
    ruleCheckRun,
    renderContext: { exportContext: context, generatedAt },
    numericalHealth,
    exactAdmission: admission,
    libraryRefFindings: currentLibraryRefFindings(model)
  });
  const controlled = controlReportRendererInput(input, intent);
  let exactSubmission: RenderedReportAssembly["exactSubmission"] = null;
  if (admission) {
    const admitted = { model, result, analysisRun, inputManifest: inputManifest as CurrentSessionInputManifestEvidence };
    const inputManifestText = await canonicalJsonString(admitted.inputManifest.manifest);
    exactSubmission = () => ({ ...exactSourceSubmission(admission, admitted), inputManifestText });
    // After the last await: the admission still holds.
    exactSubmission();
  }
  return { input, controlled, admission, exactSubmission };
}

/** Test-only name of the panel's build step for an exact source, which production
 * reaches only after the panel's cell admits the result (never before opening). */
export const assembleExactReportBelowOutputGate = assembleRenderedReport;
