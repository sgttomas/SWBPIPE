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
import type { ControlledRouteExport } from "../redaction-controls/redactionExportControls";
import type { ReportPackageSaveRoute } from "../../services/reportPackageSaveService";
import type { CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";

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
  numericalHealth = null,
  onPackagePrivateIntentChange,
  onSaveReportPackage
}: {
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
  packageRedaction?: ControlledRouteExport | null;
  packageRoute?: ReportPackageSaveRoute | null;
  onPackagePrivateIntentChange?: (value: boolean) => void;
  onSaveReportPackage?: () => void;
}) {
  const [localPrivateIntent, setLocalPrivateIntent] = useState(false);
  const currentContext: ReportExportContext = localPrivateIntent ? "local_private" : "public_report";
  const [renderState, setRenderState] = useState<{
    context: ReportExportContext;
    route: RenderReportRoute;
  } | null>(null);
  const [rendering, setRendering] = useState(false);
  const [renderError, setRenderError] = useState<string | null>(null);
  const [printRequested, setPrintRequested] = useState(false);
  const [redactionState, setRedactionState] = useState<{
    context: ReportExportContext;
    controlled: ControlledRouteExport;
  } | null>(null);
  const renderGeneration = useRef(0);

  // A render (or its redaction evidence) is shown only under the selection
  // that produced it: a public render never reuses a private render's output.
  const route = renderState?.context === currentContext ? renderState.route : null;
  const redaction = redactionState?.context === currentContext ? redactionState.controlled : null;
  const canRender = Boolean(result && analysisRun) && !rendering;
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
  }, []);
  const sourceRef = useRef({ model, result, analysisRun, projectSummary, inputManifest, ruleCheckRun });
  useEffect(() => {
    const previous = sourceRef.current;
    const next = { model, result, analysisRun, projectSummary, inputManifest, ruleCheckRun };
    sourceRef.current = next;
    if ((Object.keys(next) as Array<keyof typeof next>).every((key) => previous[key] === next[key])) return;
    renderGeneration.current += 1;
    setRenderState(null);
    setRedactionState(null);
    setPrintRequested(false);
    setRenderError(null);
    setRendering(false);
  }, [model, result, analysisRun, projectSummary, inputManifest, ruleCheckRun]);

  function onLocalPrivateIntentChange(value: boolean) {
    renderGeneration.current += 1;
    setLocalPrivateIntent(value);
    setRenderState(null);
    setRedactionState(null);
    setPrintRequested(false);
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
    if (!result || !analysisRun || surfaceOutputRefusal(result, model, "rendered-report")) return;
    const generation = ++renderGeneration.current;
    const intent = localPrivateIntent;
    const context: ReportExportContext = intent ? "local_private" : "public_report";
    const isCurrent = () => renderGeneration.current === generation;
    setRendering(true);
    setRenderError(null);
    setPrintRequested(false);
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
        context,
        intent,
        generatedAt: new Date().toISOString()
      });
      if (!isCurrent()) return;
      const { controlled, exactSubmission } = assembly;
      setRedactionState({ context, controlled });
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
      const next = await renderCalculationReport(controlled, { stillCurrent, exactSubmission: submission });
      if (!isCurrent()) return;
      setRenderState({ context, route: next });
    } catch (error) {
      if (!isCurrent()) return;
      setRenderState(null);
      setRenderError(error instanceof Error ? error.message : String(error));
    } finally {
      if (isCurrent()) setRendering(false);
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
            disabled={!result || !analysisRun || packageBusy}
            title={!result || !analysisRun ? "Solve first: report-package save needs the current mechanics result and analysis-run record." : undefined}
            onClick={onSaveReportPackage}
          >
            {packageBusy ? "Saving report package…" : "Save Report Package…"}
          </button>
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
      {packageRoute?.route === "tauri_report_package_save" ? (
        <div className="report-list" data-testid="report-package-save-status">
          <span>
            outcome={packageRoute.receipt.outcome}; code={packageRoute.receipt.code}; file=
            {packageRoute.receipt.selected_basename || packageRoute.receipt.container_file_name}
          </span>
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
              {localPrivateOutcome ? (
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
                onClick={() => setPrintRequested(true)}
              >
                <Printer size={14} aria-hidden="true" />
                Print / PDF (derived view)
              </button>
            </div>
          ) : (
            <p data-testid="rendered-report-sink-blocked">
              The rendered document is withheld: its local-private sink control did not pass.
            </p>
          )}
          {previewHtml !== null ? (
            <iframe
              title="Rendered calculation report preview"
              data-testid="rendered-report-preview"
              sandbox=""
              srcDoc={previewHtml}
              style={{ width: "100%", height: "20rem", border: "1px solid #888" }}
            />
          ) : null}
          {printRequested && printHtml !== null ? (
            <iframe
              title="Derived print view"
              data-testid="rendered-report-print-frame"
              srcDoc={printHtml}
              style={{ position: "absolute", width: 0, height: 0, border: 0 }}
              onLoad={(event) => {
                event.currentTarget.contentWindow?.print();
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
