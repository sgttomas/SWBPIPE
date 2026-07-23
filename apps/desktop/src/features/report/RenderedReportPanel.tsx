import { useState } from "react";
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
import { buildRenderableReportInput, buildUnitDisplaySummary } from "./renderableReportInput";
import { controlReportRendererInput } from "./reportRedactionProjector";
import type { ControlledRouteExport } from "../redaction-controls/redactionExportControls";
import type { ReportPackageSaveRoute } from "../../services/reportPackageSaveService";

// DEC-021 (A7): rendered FR-016 calculation report. The Rust renderer crate
// composes, gates, and hashes the document; this panel only requests a
// render, shows the canonical SHA-256 and gate state, and offers save/print.
// Save and print are refused while the renderer reports export_blocked.
// Printed/PDF output is the derived view naming the canonical hash — it is
// never hash-bound evidence.

export function RenderedReportPanel({
  model,
  result,
  analysisRun,
  projectSummary,
  packagePrivateIntent = false,
  packageBusy = false,
  packageRedaction = null,
  packageRoute = null,
  onPackagePrivateIntentChange,
  onSaveReportPackage
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  projectSummary: LocalProjectSummary | null;
  packagePrivateIntent?: boolean;
  packageBusy?: boolean;
  packageRedaction?: ControlledRouteExport | null;
  packageRoute?: ReportPackageSaveRoute | null;
  onPackagePrivateIntentChange?: (value: boolean) => void;
  onSaveReportPackage?: () => void;
}) {
  const [route, setRoute] = useState<RenderReportRoute | null>(null);
  const [rendering, setRendering] = useState(false);
  const [renderError, setRenderError] = useState<string | null>(null);
  const [printRequested, setPrintRequested] = useState(false);
  const [redaction, setRedaction] = useState<ControlledRouteExport | null>(null);

  const canRender = Boolean(result && analysisRun) && !rendering;
  const outcome = route?.route === "tauri_renderer" ? route.outcome : null;
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
    if (!result || !analysisRun) return;
    setRendering(true);
    setRenderError(null);
    setPrintRequested(false);
    try {
      const input = await buildRenderableReportInput({
        model,
        result,
        analysisRun,
        projectSummary
      });
      const controlled = controlReportRendererInput(input);
      setRedaction(controlled);
      if (controlled.blocked || controlled.payload === null) {
        setRoute(null);
        return;
      }
      setRoute(await renderCalculationReport(controlled));
    } catch (error) {
      setRoute(null);
      setRenderError(error instanceof Error ? error.message : String(error));
    } finally {
      setRendering(false);
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
          decisions={redaction.summary.decision_count}; findings={redaction.summary.finding_count}; blocked=
          {String(redaction.blocked)}
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
          <div className="report-list">
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
              {outcome.blocking_reasons.map((reason) => (
                <li key={reason}>{reason}</li>
              ))}
            </ul>
          ) : (
            <div className="report-actions">
        <ControlledExportLink
                data-testid="rendered-report-save"
                download={`openpipestress-report-${outcome.sha256_hex.slice(0, 12)}.html`}
                href={`data:text/html;charset=utf-8,${encodeURIComponent(outcome.html)}`}
              >
                Save canonical HTML
        </ControlledExportLink>
              <button
                type="button"
                data-testid="rendered-report-print"
                onClick={() => setPrintRequested(true)}
              >
                <Printer size={14} aria-hidden="true" />
                Print / PDF (derived view)
              </button>
            </div>
          )}
          {!outcome.export_blocked ? (
            <iframe
              title="Rendered calculation report preview"
              data-testid="rendered-report-preview"
              sandbox=""
              srcDoc={outcome.html}
              style={{ width: "100%", height: "20rem", border: "1px solid #888" }}
            />
          ) : null}
          {printRequested && !outcome.export_blocked ? (
            <iframe
              title="Derived print view"
              data-testid="rendered-report-print-frame"
              srcDoc={outcome.derived_print_html}
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
