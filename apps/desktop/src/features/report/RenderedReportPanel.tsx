import { ReportComposer } from "./ReportComposer";
import type { ReportComposition } from "./reportComposition";
import { useEffect, useMemo, useRef, useState } from "react";
import { LoadReferenceOutputGate } from "../results/LoadReferenceOutputGate";
import { surfaceOutputRefusal } from "../results/outputPolicy";
import { FileCheck, Printer } from "lucide-react";
import type {
  AnalysisRunEnvelope,
  LocalProjectSummary,
  MechanicsResult,
  PreviewModel
} from "../../types";
import {
  renderCalculationReport,
  type RenderReportRoute
} from "../../services/reportRenderService";
import {
  buildUnitDisplaySummary,
  type ReportExportContext,
  type SessionRuleCheckRun
} from "./renderableReportInput";
import { controlLocalPrivateRenderedHtml } from "./reportRedactionProjector";
import { provenanceReasonEntity } from "./provenanceReasonEntities";
import { assembleRenderedReport } from "./renderedReportAssembly";
import type { NumericalHealthView } from "../results/numericalHealth";
import { controlledRouteEvidence, type ControlledRouteEvidence, type ControlledRouteEvidenceWithPayload } from "./controlledRouteEvidence";
import type { ReportPackageSaveReceipt, ReportPackageSaveRoute } from "../../services/reportPackageSaveService";
import { REPORT_PACKAGE_USER_RELEASE_ROUTE_ID } from "../redaction-controls/redactionExportControls";
import type { CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import {
  controlRenderedReportHtml,
  isNativeRenderedReportRuntime,
  printOutcomeText,
  printRenderedReport,
  renderedReportRouteOpen,
  retireRenderedReport,
  saveOutcomeText,
  saveRenderedReportHtml
} from "../../services/renderedReportRoutes";
import type { ReportRenderRequest, ReportSourceMark, ReportSourceSequence } from "../workspace/reportSourceSequence";
import { currentLibraryRefFindings, NO_LIBRARY_REF_FINDINGS, type LibraryRefFindingsState } from "../library/libraryRefFindings";
import {
  notSentSettlement,
  printSettlement,
  saveSettlement,
  type ReportOutputBasis,
  type ReportOutputOutcomes
} from "./reportOutputOutcomes";

// DEC-021 (A7): rendered FR-016 calculation report. The Rust renderer crate
// composes, gates, and hashes the document; this panel only requests a
// render, shows the canonical SHA-256 and gate state, and offers save/print.
// Save and print are refused while the renderer reports export_blocked.
// Printed/PDF output is the derived view naming the canonical hash — it is
// never hash-bound evidence.
//
// DEL-12-02 REXC-REQ-006: the panel owns one explicit local-private selection.
// Unselected, the render runs in the public-report context (private inputs
// stay withheld). Selected, the render input, the preview, the canonical-HTML
// save and the print view are each controlled in the local-private context
// under that same wrapper-owned intent, the warning evidence is shown, and the
// rendered document records its local-private status. A render is shown only
// while the selection that produced it is still current; clearing the
// selection discards it.
//
// W-11 slice (b) (DEL-08-01 Design `exact-route-report-sections.md` §3.1): the panel's
// check is the surface's own (`surfaceOutputRefusal` on `rendered-report`), which refuses
// exactly as the route-level check did while the cell refuses an exact result (ruling
// 10A). Behind it, an exact source is built with its Export Result JSON admission, which
// is re-checked immediately before IPC, and the render resubmits the texts the native
// host binds (`renderedReportAssembly.ts`).
//
// DEL-08-01 REQ-022 (native routes): in the desktop application the save and the
// print go through native commands (`renderedReportRoutes.ts`): the canonical HTML
// is saved through the system save chooser and printed from a window of its own,
// each bound natively to this render's hash. Outside the desktop the save stays a
// controlled download link and the print a print iframe.
//
// DEL-08-01 REQ-022 (currency): each render, save and print is prepared from the
// report source number this panel was rendered with (`reportSourceSequence`, committed
// with its other props). It waits for the desktop application's native process to
// register that number, re-checks that it is still the latest, and names it in the
// request; the native side refuses a retired or unregistered number. The panel's own
// export-context and composition changes advance the number. Every save and print is
// started and settled through the session's report output outcomes (`reportOutputs`),
// shown outside this panel, so its outcome survives this panel and its render.
//
// Each render request carries an ordinal the source owner allocates when its preparation
// begins (`nextRenderRequest`, before any await), and the panel keeps that request's
// identity (source and ordinal) from before it is sent. A retirement names that exact
// request: on unmount the panel retires the render it holds and every request it sent whose
// answer is outstanding, and it retires at once a request whose answer arrives after the
// panel has discarded it or whose outcome is not known. It never retires globally.

/** Whether a saved package records the user's per-report user release. Only a saved outcome
 * wrote a package, so no other outcome shows a user release, whatever its members say. The
 * binding stays in the report and its package; the status carries none (REQ-019). */
function recordsUserRelease(receipt: ReportPackageSaveReceipt): boolean {
  return receipt.outcome === "saved" && (receipt as { user_release_recorded?: unknown }).user_release_recorded === true;
}

/** A cancelled save wrote nothing, and the panel says so (DEL-08-01 AC-010 and REQ-019 for
 * the shareable save, which also made no user release). */
const SHAREABLE_SAVE_CANCELLED =
  "Not saved: the save was cancelled. No shareable report package was written and no user release was made.";
const LOCAL_PRIVATE_SAVE_CANCELLED = "Not saved: the save was cancelled. No report package was written.";

/** The sentence a cancelled receipt leads with, by its route; null for a saved one. */
function cancelledStatement(receipt: ReportPackageSaveReceipt): string | null {
  if (receipt.outcome !== "cancelled") return null;
  return receipt.redaction_route_id === REPORT_PACKAGE_USER_RELEASE_ROUTE_ID ? SHAREABLE_SAVE_CANCELLED : LOCAL_PRIVATE_SAVE_CANCELLED;
}

/** The first line of a receipt. A saved outcome's is unchanged (the native smoke pins its
 * prefix). A cancelled save names only the file the person selected, if any, never the
 * suggested name of a file that was not written. */
function receiptOutcomeLine(receipt: ReportPackageSaveReceipt): string {
  const head = `outcome=${receipt.outcome}; code=${receipt.code}`;
  if (receipt.outcome !== "saved") {
    return receipt.selected_basename ? `${head}; file=${receipt.selected_basename}` : head;
  }
  return `${head}; file=${receipt.selected_basename || receipt.container_file_name}`;
}

const LOCAL_PRIVATE_REPORT_WARNING =
  "Local private report: this rendering contains private project values for your own local review. It is not for sharing or publication.";

export function RenderedReportPanel({
  model,
  result,
  analysisRun,
  projectSummary,
  inputManifest = null,
  ruleCheckRun = null,
  packagePrivateIntent = false,
  packageBusy = false,
  packageRedaction = null,
  packageRoute = null,
  packageNeedsCurrent = false,
  numericalHealth = null,
  reportComposition: sessionComposition,
  onReportCompositionChange,
  onPackagePrivateIntentChange,
  onSaveReportPackage,
  onSaveShareableReportPackage,
  reportSource = null,
  reportSourceSequence = 0,
  reportOutputs = null,
  libraryRefFindingsState = NO_LIBRARY_REF_FINDINGS
}: {
  /** DEL-08-01 REQ-022 (F1): the session's library-reference findings publication, committed
   * with this panel's other props and source number; the report takes the findings that apply
   * to `model` from it. */
  libraryRefFindingsState?: LibraryRefFindingsState;
  /** DEL-08-01 REQ-022: the session's report source sequence, and the number this
   * panel's other props were committed with. */
  reportSource?: ReportSourceSequence | null;
  reportSourceSequence?: number;
  /** The session's report output outcomes (outside the dormant Report section). */
  reportOutputs?: ReportOutputOutcomes | null;
  reportComposition?: ReportComposition | null;
  onReportCompositionChange?: (value: ReportComposition | null) => void;
  model: PreviewModel;
  result: MechanicsResult | null;
  /** DEL-07-05 numerical health of `result` (the report's per-load-case line). */
  numericalHealth?: NumericalHealthView | null;
  analysisRun: AnalysisRunEnvelope | null;
  projectSummary: LocalProjectSummary | null;
  inputManifest?: CurrentSessionInputManifestEvidence | null;
  ruleCheckRun?: SessionRuleCheckRun | null;
  packagePrivateIntent?: boolean;
  packageBusy?: boolean;
  /** The package control, of which the panel reads the summary and the blocked flag. */
  packageRedaction?: ControlledRouteEvidence | null;
  packageRoute?: ReportPackageSaveRoute | null;
  /** DEL-08-01 REQ-024: the panel's result is shown for its rendered report only (a solved
   * exact-route result that is not numerically eligible). The report package takes only a
   * Current result, so its save buttons stay disabled for it, as the File menu's are. */
  packageNeedsCurrent?: boolean;
  onPackagePrivateIntentChange?: (value: boolean) => void;
  onSaveReportPackage?: () => void;
  /** W-18b (DEL-08-01 Design `report-user-release.md` §2.2): opens the user-release dialog. */
  onSaveShareableReportPackage?: () => void;
}) {
  const [panelComposition, setPanelComposition] = useState<ReportComposition | null>(null);
  const packageSaveTitle = !result || !analysisRun
    ? "Solve first: report-package save needs the current mechanics result and analysis-run record."
    : packageNeedsCurrent
      ? "Not available for this result: report-package save needs a numerically eligible result. The rendered report states this result's numerical standing."
      : undefined;
  const reportComposition = sessionComposition === undefined ? panelComposition : sessionComposition;
  const changeComposition = onReportCompositionChange ?? setPanelComposition;
  const [localPrivateIntent, setLocalPrivateIntent] = useState(false);
  const currentContext: ReportExportContext = localPrivateIntent ? "local_private" : "public_report";
  const [renderState, setRenderState] = useState<{
    context: ReportExportContext;
    route: RenderReportRoute;
    /** The registered report source the render was made for. */
    source: ReportSourceMark;
  } | null>(null);
  const [rendering, setRendering] = useState(false);
  const [renderError, setRenderError] = useState<string | null>(null);
  // The browser print frame: one mount per press (its `key`), cleared when the
  // print ends (`afterprint`) or the render is discarded, so the same report can
  // be printed again and a changed one printed afresh.
  const [printRequest, setPrintRequest] = useState<number | null>(null);
  const printPresses = useRef(0);
  // The render control's summary, blocked flag and payload (the provenance reason reader's
  // input), never its per-value decisions and findings, which the panel does not read.
  const [redactionState, setRedactionState] = useState<{
    context: ReportExportContext;
    controlled: ControlledRouteEvidenceWithPayload;
  } | null>(null);
  const renderGeneration = useRef(0);
  const nativeRoutes = isNativeRenderedReportRuntime();
  // The last accepted native render, retained until a replacement is accepted or unmount
  // retires it. Clearing the preview or starting preparation does not prove native custody
  // changed: preparation or entry can fail, and a source advance can still be in transit.
  // Requests sent with answers outstanding are tracked separately by their exact identity.
  const heldRequest = useRef<ReportRenderRequest | null>(null);
  const outstandingRequests = useRef(new Set<ReportRenderRequest>());
  const routeInFlight = useRef(false);
  const [routeBusy, setRouteBusy] = useState<"save" | "print" | null>(null);
  const [routeStatus, setRouteStatus] = useState<{ generation: number; save?: string; print?: string } | null>(null);
  // The native side retires a held render when its source advances (a registered
  // source change) or another render request enters; here, on unmount, the panel
  // retires the exact requests it holds or has outstanding.
  function retireOnUnmount() {
    const requests = [...outstandingRequests.current, ...(heldRequest.current ? [heldRequest.current] : [])];
    outstandingRequests.current.clear();
    heldRequest.current = null;
    for (const request of requests) retireRenderedReport(request);
  }

  // A render (or its redaction evidence) is shown only under the selection
  // that produced it: a public render never reuses a private render's output.
  const route = renderState?.context === currentContext ? renderState.route : null;
  const redaction = redactionState?.context === currentContext ? redactionState.controlled : null;
  const canRender = Boolean(result && analysisRun) && !rendering && (!reportComposition || localPrivateIntent);
  const outcome = route?.route === "tauri_renderer" ? route.outcome : null;
  const localPrivateOutcome = outcome !== null && renderState?.context === "local_private";
  const localPrivateSinks = useMemo(() => {
    if (!localPrivateOutcome || !outcome || outcome.export_blocked) return null;
    const html = controlLocalPrivateRenderedHtml(outcome.html, localPrivateIntent);
    const print = controlLocalPrivateRenderedHtml(outcome.derived_print_html, localPrivateIntent);
    if (
      html.blocked ||
      print.blocked ||
      html.payload !== outcome.html ||
      print.payload !== outcome.derived_print_html
    ) {
      return null;
    }
    return { html: html.payload as string, print: print.payload as string };
  }, [localPrivateIntent, localPrivateOutcome, outcome]);
  // Public renders keep their existing sinks; local-private renders expose the
  // document only through the local-private controlled copies.
  const previewHtml = outcome && !outcome.export_blocked
    ? localPrivateOutcome ? localPrivateSinks?.html ?? null : outcome.html
    : null;
  const printHtml = outcome && !outcome.export_blocked
    ? localPrivateOutcome ? localPrivateSinks?.print ?? null : outcome.derived_print_html
    : null;
  // The findings applicable to this model, from the publication committed with it: a stable
  // snapshot for the whole preparation, never a reread of the store after an await.
  const libraryRefFindings = useMemo(() => currentLibraryRefFindings(model, libraryRefFindingsState), [model, libraryRefFindingsState]);
  const libraryRefFindingsKey = useMemo(() => JSON.stringify(libraryRefFindings), [libraryRefFindings]);
  const sessionRuleCheckRun =
    ruleCheckRun &&
    analysisRun?.analysis_run.analysis_status.includes(ruleCheckRun.result.aggregate_status)
      ? ruleCheckRun
      : null;

  // Pending render work belongs to the panel instance and the exact source it
  // was started from. Unmounting (e.g. a project reopen replaces the panel) or
  // replacing any source invalidates it before it can reach renderer IPC or
  // write state; a source replacement also retires the shown render.
  useEffect(() => () => {
    renderGeneration.current += 1;
    retireOnUnmount();
  }, []);
  const sourceRef = useRef({ model, result, analysisRun, projectSummary, inputManifest, ruleCheckRun, reportComposition, libraryRefFindingsKey });
  useEffect(() => {
    const previous = sourceRef.current;
    const next = { model, result, analysisRun, projectSummary, inputManifest, ruleCheckRun, reportComposition, libraryRefFindingsKey };
    sourceRef.current = next;
    if ((Object.keys(next) as Array<keyof typeof next>).every((key) => previous[key] === next[key])) return;
    renderGeneration.current += 1;
    setRenderState(null);
    setRedactionState(null);
    setPrintRequest(null);
    setRouteStatus(null);
    setRenderError(null);
    setRendering(false);
  }, [model, result, analysisRun, projectSummary, inputManifest, ruleCheckRun, reportComposition, libraryRefFindingsKey]);

  function onLocalPrivateIntentChange(value: boolean) {
    renderGeneration.current += 1;
    // The export context is report-bound: its change advances the source sequence.
    reportSource?.advance();
    setLocalPrivateIntent(value);
    setRenderState(null);
    setRedactionState(null);
    setPrintRequest(null);
    setRouteStatus(null);
    setRenderError(null);
    setRendering(false);
  }
  function onCompositionChange(value: ReportComposition | null) {
    renderGeneration.current += 1;
    // The session's composition cell advances the sequence itself; the panel's own does here.
    if (!onReportCompositionChange) reportSource?.advance();
    changeComposition(value);
    setRenderState(null);
    setRedactionState(null);
    setPrintRequest(null);
    setRouteStatus(null);
    setRenderError(null);
    setRendering(false);
  }
  const unitBasis = result ? buildUnitDisplaySummary(model, result) : null;
  const unitBasisText = unitBasis
    ? [
        `unit_system=${DEC018_UNIT_SYSTEM_REF}`,
        `model=${formatModelUnits(unitBasis.model_units)}`,
        `results=${unitBasis.result_units.join(",") || "none"}`,
        `conversion=${String(unitBasis.conversion_performed)}`,
        "source=renderable_report_input"
      ].join("; ")
    : null;

  async function onRender() {
    if ((reportComposition && !localPrivateIntent) || !result || !analysisRun || surfaceOutputRefusal(result, model, "rendered-report")) return;
    const generation = ++renderGeneration.current;
    const intent = localPrivateIntent;
    const context: ReportExportContext = intent ? "local_private" : "public_report";
    const isCurrent = () => renderGeneration.current === generation;
    // The basis's number, committed with the props this render reads, and this render
    // request's ordinal, allocated as its preparation begins (before any await).
    const viewSequence = reportSourceSequence;
    const requestOrdinal = reportSource ? reportSource.nextRenderRequest() : 0;
    setRendering(true);
    setRenderError(null);
    setPrintRequest(null);
    setRouteStatus(null);
    setRenderState(null);
    setRedactionState(null);
    try {
      const assembly = await assembleRenderedReport({
        model,
        result,
        analysisRun,
        projectSummary,
        inputManifest,
        ruleCheckRun: sessionRuleCheckRun,
        numericalHealth,
        reportComposition,
        context,
        intent,
        generatedAt: new Date().toISOString(),
        libraryRefFindings
      });
      if (!isCurrent()) return;
      const { controlled, exactSubmission } = assembly;
      setRedactionState({ context, controlled: controlledRouteEvidence(controlled, "payload") });
      if (controlled.blocked || controlled.payload === null) return;
      if (!isCurrent()) return;
      // An exact source: the admission's live re-check, immediately before IPC (and
      // before each re-send); a retired, replaced or edited source sends nothing.
      const submission = exactSubmission ? exactSubmission() : null;
      const stillCurrent = exactSubmission
        ? () => {
            if (!isCurrent()) return false;
            try { exactSubmission(); return true; } catch { return false; }
          }
        : isCurrent;
      // Output preparation waits for the native side to register this basis's number
      // and re-checks that it is still the latest; editing never waits for it.
      const source = reportSource ? await reportSource.registeredFor(viewSequence) : { incarnation: 0, sequence: viewSequence };
      if (!isCurrent()) return;
      // The request's identity is kept before it is sent, so an unmount while it is
      // outstanding retires exactly it.
      const request: ReportRenderRequest | null = isNativeRenderedReportRuntime() ? { source, request: requestOrdinal } : null;
      if (request) outstandingRequests.current.add(request);
      let next: RenderReportRoute;
      try {
        next = await renderCalculationReport(controlled, { stillCurrent, exactSubmission: submission, source, request: requestOrdinal });
      } catch (error) {
        // Its outcome is not known to the panel: retire exactly this request.
        if (request && outstandingRequests.current.delete(request)) retireRenderedReport(request);
        throw error;
      }
      if (request && outstandingRequests.current.delete(request)) {
        if (isCurrent() && next.route === "tauri_renderer") heldRequest.current = request;
        // Discarded while it was outstanding, or never rendered: retire exactly this request.
        else retireRenderedReport(request);
      }
      if (!isCurrent()) return;
      setRenderState({ context, route: next, source });
    } catch (error) {
      if (!isCurrent()) return;
      setRenderState(null);
      setRenderError(error instanceof Error ? error.message : String(error));
    } finally {
      if (isCurrent()) setRendering(false);
    }
  }

  // The native routes' document: the canonical HTML of the shown render, which is
  // the local-private controlled copy (checked equal) under that context.
  const routeHtml = outcome && !outcome.export_blocked && previewHtml !== null ? previewHtml : null;
  const routeContext = renderState?.context ?? null;
  const routeControl = useMemo(
    () => (nativeRoutes && routeHtml !== null && routeContext !== null
      ? controlRenderedReportHtml(routeHtml, routeContext, routeContext === "local_private" && localPrivateIntent)
      : null),
    [nativeRoutes, routeHtml, routeContext, localPrivateIntent]
  );
  // The save is gated by the HTML route's control under the render's context, as
  // its download link was; the print by the same control for a local-private
  // render and by the render's own public gate for a public one, as its iframe was.
  const routeOpen = routeControl !== null && routeHtml !== null && renderedReportRouteOpen(routeControl, routeHtml);
  const printOpen = routeContext === "local_private" ? routeOpen : routeHtml !== null && printHtml !== null;
  const shownRouteStatus = routeStatus?.generation === renderGeneration.current ? routeStatus : null;

  async function onNativeRoute(kind: "save" | "print") {
    const open = kind === "save" ? routeOpen : printOpen;
    if (routeInFlight.current || !open || routeControl === null || routeHtml === null || outcome === null || routeContext === null || renderState === null) return;
    routeInFlight.current = true;
    const generation = renderGeneration.current;
    const stillCurrent = () => renderGeneration.current === generation;
    const renderedFor = renderState.source;
    const viewSequence = reportSourceSequence;
    const basis: ReportOutputBasis = {
      sequence: renderedFor.sequence,
      model: model.project.name ?? model.project.id,
      run: analysisRun?.analysis_run.run_id ?? result?.run_id ?? null
    };
    // The session's outcome entry: started now, settled on every return or rejection,
    // whether or not this panel is still mounted.
    const operation = reportOutputs?.begin(kind, basis) ?? null;
    const settle = (settlement: Parameters<ReportOutputOutcomes["settle"]>[1]) => {
      if (operation !== null) reportOutputs?.settle(operation, settlement);
    };
    setRouteBusy(kind);
    setRouteStatus((previous) => ({
      ...(previous?.generation === generation ? previous : {}),
      generation,
      [kind]: kind === "save" ? "Choose where to save the canonical HTML…" : "Opening the print window…"
    }));
    const show = (text: string) => {
      if (stillCurrent()) setRouteStatus((previous) => ({ ...(previous?.generation === generation ? previous : {}), generation, [kind]: text }));
    };
    try {
      let source: ReportSourceMark;
      try {
        if (viewSequence !== renderedFor.sequence) throw new Error("REPORT-SOURCE-STALE-VIEW: the workspace changed after this report was rendered; render it again.");
        source = reportSource ? await reportSource.registeredFor(viewSequence) : renderedFor;
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        const code = message.split(":")[0] || "REPORT-SOURCE-UNREGISTERED";
        settle(notSentSettlement(code, `${message} Nothing was sent.`));
        show(`${kind === "save" ? "Not saved" : "Not printed"}: ${message} Nothing was sent.`);
        return;
      }
      const request = {
        // The render is named by its hash; the desktop holds its document.
        // The HTML stays here: the service checks the controlled export against it.
        sha256Hex: outcome.sha256_hex,
        html: routeHtml,
        context: routeContext,
        explicitLocalPrivateIntent: routeContext === "local_private" && localPrivateIntent,
        controlled: routeControl,
        source
      };
      if (kind === "save") {
        const outcome = await saveRenderedReportHtml(request, stillCurrent);
        settle(saveSettlement(outcome));
        show(saveOutcomeText(outcome));
      } else {
        const outcome = await printRenderedReport({ ...request, controlled: routeContext === "local_private" ? routeControl : null }, stillCurrent);
        settle(printSettlement(outcome));
        show(printOutcomeText(outcome));
      }
    } catch (error) {
      // Not reached by the route services, which return every outcome; settled anyway.
      settle({ state: "unknown", code: null, detail: error instanceof Error ? error.message : String(error), basename: null });
      throw error;
    } finally {
      routeInFlight.current = false;
      setRouteBusy(null);
    }
  }

  return (
    <section
      className="panel rendered-report-panel"
      aria-label="Rendered calculation report"
      data-testid="rendered-report-panel"
    >
      <div className="panel-title">
        <FileCheck size={16} />
        Rendered Report (FR-016)
      </div>
      <ReportComposer composition={reportComposition} result={result} onChange={onCompositionChange} />
      {reportComposition && !localPrivateIntent ? <p role="status">Select local private rendering to render this composition.</p> : null}
      <div className="report-actions">
        <LoadReferenceOutputGate result={result} testIdPrefix="rendered-report">
        <button
          type="button"
          data-testid="rendered-report-render"
          disabled={!canRender}
          title={
            canRender
              ? undefined
              : rendering
                ? "A report render is already in progress."
                : "Disabled: rendering needs a completed mechanics preview and its analysis-run record in this session. Open the Solve section and run the mechanics preview first."
          }
          onClick={() => void onRender()}
        >
          {rendering ? "Rendering…" : "Render report"}
        </button>
        </LoadReferenceOutputGate>
        <label>
          <input
            type="checkbox"
            data-testid="rendered-report-local-private-intent"
            checked={localPrivateIntent}
            onChange={(event) => onLocalPrivateIntentChange(event.currentTarget.checked)}
          />
          Include known private values in this local rendered report (your own review; not for sharing)
        </label>
        {!result || !analysisRun ? (
          <span data-testid="rendered-report-precondition">
            Solve first: rendering needs a mechanics result and analysis-run record.
          </span>
        ) : null}
      </div>
      {onSaveReportPackage ? (
        <div className="report-actions" data-testid="report-package-controls">
          <label>
            <input
              type="checkbox"
              data-testid="report-package-private-intent"
              checked={packagePrivateIntent}
              disabled={packageBusy}
              onChange={(event) => onPackagePrivateIntentChange?.(event.currentTarget.checked)}
            />
            Include known private values in this local package
          </label>
          <button
            type="button"
            data-testid="report-package-save"
            disabled={!result || !analysisRun || packageBusy || packageNeedsCurrent}
            title={packageSaveTitle}
            onClick={onSaveReportPackage}
          >
            {packageBusy ? "Saving report package…" : "Save Report Package…"}
          </button>
          {onSaveShareableReportPackage ? (
            <button
              type="button"
              data-testid="report-package-user-release"
              disabled={!result || !analysisRun || packageBusy || packageNeedsCurrent}
              title={packageSaveTitle}
              onClick={onSaveShareableReportPackage}
            >
              Save Shareable Report Package…
            </button>
          ) : null}
        </div>
      ) : null}
      {packageRedaction ? (
        <p data-testid="report-package-redaction-summary">
          route={packageRedaction.summary.route_id}; decisions={packageRedaction.summary.decision_count}; findings=
          {packageRedaction.summary.finding_count}; blocking={packageRedaction.summary.blocking_count}; blocked=
          {String(packageRedaction.blocked)}
        </p>
      ) : null}
      {packageRoute?.route === "unavailable_browser" ? (
        <p data-testid="report-package-save-status">{packageRoute.diagnostic}</p>
      ) : null}
      {packageRoute?.route === "redaction_blocked" ? (
        <p data-testid="report-package-save-status">{packageRoute.diagnostic}</p>
      ) : null}
      {packageRoute?.route === "not_requested" ? (
        <p role="status" data-testid="report-package-save-status">{packageRoute.diagnostic}</p>
      ) : null}
      {packageRoute?.route === "tauri_report_package_save" ? (
        <div className="report-list" data-testid="report-package-save-status">
          {cancelledStatement(packageRoute.receipt) ? (
            <span data-testid="report-package-save-cancelled">{cancelledStatement(packageRoute.receipt)}</span>
          ) : null}
          <span>{receiptOutcomeLine(packageRoute.receipt)}</span>
          <span>
            bytes={packageRoute.receipt.byte_count}; container_sha256={packageRoute.receipt.container_sha256_hex}; package_identity=
            {packageRoute.receipt.package_identity_sha256_hex}
          </span>
          <span>
            replacement={String(packageRoute.receipt.replaced_existing)}; durability={packageRoute.receipt.durability}; redaction_route=
            {packageRoute.receipt.redaction_route_id}
          </span>
          <span>
            path_containment={packageRoute.receipt.path_containment}; limitation={packageRoute.receipt.limitation}
          </span>
          {recordsUserRelease(packageRoute.receipt) ? (
            <span data-testid="report-package-user-release-status">user_release_recorded=true</span>
          ) : null}
        </div>
      ) : null}
      {unitBasisText ? (
        <div className="report-list">
          <span data-testid="rendered-report-unit-basis">{unitBasisText}</span>
        </div>
      ) : null}
      {renderError ? (
        <p data-testid="rendered-report-error">Render failed: {renderError}</p>
      ) : null}
      {redaction ? (
        <p data-testid="rendered-report-redaction-summary">
          route={redaction.summary.route_id}; context={redactionState?.context}; explicit_intent=
          {String(redaction.summary.local_first?.explicit_local_private_intent === true)}; decisions=
          {redaction.summary.decision_count}; findings={redaction.summary.finding_count}; warnings=
          {redaction.summary.warning_count}; blocking={redaction.summary.blocking_count}; blocked=
          {String(redaction.blocked)}
        </p>
      ) : null}
      {redaction && redactionState?.context === "local_private" && !redaction.blocked ? (
        <p role="note" data-testid="rendered-report-local-private-warning">
          {LOCAL_PRIVATE_REPORT_WARNING}
        </p>
      ) : null}
      {route?.route === "unavailable_browser_preview" ? (
        <p data-testid="rendered-report-route">{route.diagnostic}</p>
      ) : null}
      {route?.route === "redaction_blocked" ? (
        <p data-testid="rendered-report-route">{route.diagnostic}</p>
      ) : null}
      {outcome ? (
        <>
          <div className="report-list" data-export-context={renderState?.context}>
            <span data-testid="rendered-report-route">route=tauri_renderer</span>
            <span data-testid="rendered-report-hash">
              Canonical HTML SHA-256: {outcome.sha256_hex}
            </span>
            <span data-testid="rendered-report-gate">
              {outcome.export_blocked
                ? `EXPORT BLOCKED (${outcome.blocking_reasons.length} reason${
                    outcome.blocking_reasons.length === 1 ? "" : "s"
                  })`
                : "export gates passed"}
            </span>
          </div>
          {outcome.export_blocked ? (
            <ul data-testid="rendered-report-blocking-reasons">
              {outcome.blocking_reasons.map((reason) => {
                // A refused record is named by position; resolve it locally from the sent input.
                const entity = provenanceReasonEntity(reason, redaction?.payload ?? null);
                return <li key={reason}>{reason}{entity ? ` (${entity})` : ""}</li>;
              })}
            </ul>
          ) : previewHtml !== null && printHtml !== null ? (
            <div className="report-actions">
              {nativeRoutes ? (
                <div
                  className="controlled-export-control"
                  data-route-id="DREP-HTML-SAVE-005"
                  data-local-first-blocked={String(!routeOpen)}
                >
                  <span data-testid={`${localPrivateOutcome ? "rendered-report-save-local-private" : "rendered-report-save"}-redaction-summary`}>
                    decisions={routeControl?.summary.decision_count ?? 0}; findings={routeControl?.summary.finding_count ?? 0}; blocked=
                    {String(!routeOpen)}
                  </span>
                  <button
                    type="button"
                    data-testid={localPrivateOutcome ? "rendered-report-save-local-private" : "rendered-report-save"}
                    disabled={!routeOpen || routeBusy !== null}
                    onClick={() => void onNativeRoute("save")}
                  >
                    {localPrivateOutcome ? "Save canonical HTML (local private)…" : "Save canonical HTML…"}
                  </button>
                </div>
              ) : localPrivateOutcome ? (
                <ControlledExportLink
                  data-testid="rendered-report-save-local-private"
                  explicitLocalPrivateIntent={localPrivateIntent}
                  download={`openpipestress-local-private-report-${outcome.sha256_hex.slice(0, 12)}.html`}
                  href={`data:text/html;charset=utf-8,${encodeURIComponent(previewHtml)}`}
                >
                  Save canonical HTML (local private)
                </ControlledExportLink>
              ) : (
                <ControlledExportLink
                  data-testid="rendered-report-save"
                  download={`openpipestress-report-${outcome.sha256_hex.slice(0, 12)}.html`}
                  href={`data:text/html;charset=utf-8,${encodeURIComponent(outcome.html)}`}
                >
                  Save canonical HTML
                </ControlledExportLink>
              )}
              <button
                type="button"
                data-testid="rendered-report-print"
                disabled={nativeRoutes && (!printOpen || routeBusy !== null)}
                onClick={() => (nativeRoutes ? void onNativeRoute("print") : setPrintRequest(++printPresses.current))}
              >
                <Printer size={14} aria-hidden="true" />
                Print / PDF (derived view)
              </button>
              <p data-testid="rendered-report-list-notice">
                HTML saves and printed reports are not listed in the project. Save a report package to keep it in the project’s report list.
              </p>
            </div>
          ) : (
            <p data-testid="rendered-report-sink-blocked">
              The rendered document is withheld: its local-private sink control did not pass.
            </p>
          )}
          {nativeRoutes && !outcome.export_blocked ? (
            <div className="report-list">
              <span role="status" data-testid="rendered-report-save-status">{shownRouteStatus?.save ?? ""}</span>
              <span role="status" data-testid="rendered-report-print-status">{shownRouteStatus?.print ?? ""}</span>
            </div>
          ) : null}
          {previewHtml !== null ? (
            <iframe
              title="Rendered calculation report preview"
              data-testid="rendered-report-preview"
              sandbox=""
              srcDoc={previewHtml}
              style={{ width: "100%", height: "20rem", border: "1px solid #888" }}
            />
          ) : null}
          {!nativeRoutes && printRequest !== null && printHtml !== null ? (
            // Sandboxed with only the two flags a print needs: `allow-same-origin`
            // so the panel may call print() on the frame's window at all (print is
            // not reachable across origins), and `allow-modals` because a document
            // whose sandbox sets the sandboxed modals flag ignores print(). With no
            // `allow-scripts` nothing in the document runs, and the pair is safe
            // only without it; no forms, popups, navigation or downloads.
            <iframe
              key={printRequest}
              title="Derived print view"
              data-testid="rendered-report-print-frame"
              sandbox="allow-same-origin allow-modals"
              srcDoc={printHtml}
              style={{ position: "absolute", width: 0, height: 0, border: 0 }}
              onLoad={(event) => {
                const frameWindow = event.currentTarget.contentWindow;
                if (!frameWindow) return;
                const press = printRequest;
                frameWindow.addEventListener(
                  "afterprint",
                  () => setPrintRequest((current) => (current === press ? null : current)),
                  { once: true }
                );
                frameWindow.print();
              }}
            />
          ) : null}
        </>
      ) : null}
    </section>
  );
}

const DEC018_UNIT_SYSTEM_REF = "unit-system:dec-018-si-dual-display";

function formatModelUnits(modelUnits: Record<string, string>): string {
  return (
    Object.entries(modelUnits)
      .map(([dimension, unit]) => `${dimension}=${unit}`)
      .join(",") || "none"
  );
}
import { ControlledExportLink } from "../redaction-controls/ControlledExportLink";
