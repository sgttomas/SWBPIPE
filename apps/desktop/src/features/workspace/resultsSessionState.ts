import { useMemo, useRef, useState } from "react";
import type { CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import { buildPreviewComparison } from "../../services/previewService";
import type { PreviewSolverMode } from "../../services/previewService";
import type { ReportPackageSaveRoute } from "../../services/reportPackageSaveService";
import type { RuleCheckStatus } from "../../services/ruleCheckService";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  MechanicsResult,
  SelectedReviewTarget,
  SolveJobAuditState
} from "../../types";
import type { ControlledRouteExport } from "../redaction-controls/redactionExportControls";
import type { HistoricalRunContext } from "../results/HistoricalRunContext";
import { RuleRevisionGenerationGate, SolveRunGenerationGate } from "./solveGates";
import { initialSolveJob } from "./solveJobAudit";
import type { SolveProofEvidence } from "./solveProof";

/**
 * The results cells of the workspace session: the result, the historical run,
 * the analysis run and the input manifest; the two basis refs with their
 * render-time bumps; the rule-check aggregate; the proposal and the selected
 * review target, which are review context cleared and saved with the results;
 * the solve job, the solve proof, the running flag and the solver mode; the
 * report-package cells and refs; the two generation gates; the current-result
 * and current-manifest refs with their render-time assignments; the active
 * solve job, the cancellation tombstones and the comparison. It declares no
 * effect and no handler.
 *
 * Called only by the session, which is `AppSession` in `App.tsx`. Apart from
 * the two run options (`setSolverMode`, `setReportPackagePrivateIntent`), its
 * setters are not for components: a result is published or cleared only by the
 * session's solve, rule-check, operation, undo, redo and project handlers.
 */
export function useResultsSessionState() {
  const [result, setResult] = useState<MechanicsResult | null>(null);
  const [historicalRun, setHistoricalRun] = useState<HistoricalRunContext | null>(null);
  const currentSolvedResult = result?.status.mechanics === "MECHANICS_SOLVED" ? result : null;
  const [analysisRun, setAnalysisRun] = useState<AnalysisRunEnvelope | null>(null);
  const resultBasisRef = useRef<{ value: MechanicsResult | null; sequence: number }>({ value: null, sequence: 0 });
  if (resultBasisRef.current.value !== result) resultBasisRef.current = { value: result, sequence: resultBasisRef.current.sequence + 1 };
  const analysisBasisRef = useRef<{ value: AnalysisRunEnvelope | null; sequence: number }>({ value: null, sequence: 0 });
  if (analysisBasisRef.current.value !== analysisRun) analysisBasisRef.current = { value: analysisRun, sequence: analysisBasisRef.current.sequence + 1 };
  const [inputManifest, setInputManifest] =
    useState<CurrentSessionInputManifestEvidence | null>(null);
  // Worst-of rule-check aggregate from the GUI run panel, lifted so it can be
  // recorded in the app-held analysis-run envelope (TP-C4-APPAGG-001).
  const [ruleCheckAggregate, setRuleCheckAggregate] = useState<RuleCheckStatus | null>(null);
  const [proposal, setProposal] = useState<AgentProposal | null>(null);
  const [selectedReviewTarget, setSelectedReviewTarget] = useState<SelectedReviewTarget | null>(null);
  const [solveJob, setSolveJob] = useState<SolveJobAuditState>(() => initialSolveJob());
  const [solveProof, setSolveProof] = useState<SolveProofEvidence | null>(null);
  const [running, setRunning] = useState(false);
  const [solverMode, setSolverMode] = useState<PreviewSolverMode>("sparse_interactive");
  const [reportPackagePrivateIntent, setReportPackagePrivateIntent] = useState(false);
  const [reportPackageBusy, setReportPackageBusy] = useState(false);
  const [reportPackageRedaction, setReportPackageRedaction] = useState<ControlledRouteExport | null>(null);
  const [reportPackageRoute, setReportPackageRoute] = useState<ReportPackageSaveRoute | null>(null);
  const reportPackageRequestGenerationRef = useRef(0);
  const reportPackageBusyGenerationRef = useRef<number | null>(null);
  const solveRunGate = useRef(new SolveRunGenerationGate());
  const ruleRevisionGate = useRef(new RuleRevisionGenerationGate());
  const currentSolvedResultRef = useRef<MechanicsResult | null>(null);
  const currentInputManifestRef = useRef<CurrentSessionInputManifestEvidence | null>(null);
  currentSolvedResultRef.current = currentSolvedResult;
  currentInputManifestRef.current = inputManifest;
  const activeSolveJob = useRef<{
    generation: number;
    job: SolveJobAuditState;
    cancellation_dispatched: boolean;
  } | null>(null);
  const solveCancellationTombstones = useRef(new Map<number, {
    requested: boolean;
    dispatched: boolean;
  }>());
  const comparison = useMemo(
    () => (currentSolvedResult && analysisRun ? buildPreviewComparison({ result: currentSolvedResult, analysisRun }) : null),
    [analysisRun, currentSolvedResult]
  );
  return {
    result, setResult,
    historicalRun, setHistoricalRun,
    currentSolvedResult,
    analysisRun, setAnalysisRun,
    resultBasisRef,
    analysisBasisRef,
    inputManifest, setInputManifest,
    ruleCheckAggregate, setRuleCheckAggregate,
    proposal, setProposal,
    selectedReviewTarget, setSelectedReviewTarget,
    solveJob, setSolveJob,
    solveProof, setSolveProof,
    running, setRunning,
    solverMode, setSolverMode,
    reportPackagePrivateIntent, setReportPackagePrivateIntent,
    reportPackageBusy, setReportPackageBusy,
    reportPackageRedaction, setReportPackageRedaction,
    reportPackageRoute, setReportPackageRoute,
    reportPackageRequestGenerationRef,
    reportPackageBusyGenerationRef,
    solveRunGate,
    ruleRevisionGate,
    currentSolvedResultRef,
    currentInputManifestRef,
    activeSolveJob,
    solveCancellationTombstones,
    comparison
  };
}
