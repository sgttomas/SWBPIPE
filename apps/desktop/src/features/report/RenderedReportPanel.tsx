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
  projectSummary
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  projectSummary: LocalProjectSummary | null;
}) {
  const [route, setRoute] = useState<RenderReportRoute | null>(null);
  const [rendering, setRendering] = useState(false);
  const [renderError, setRenderError] = useState<string | null>(null);
  const [printRequested, setPrintRequested] = useState(false);

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
      setRoute(await renderCalculationReport(input));
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
      {unitBasisText ? (
        <div className="report-list">
          <span data-testid="rendered-report-unit-basis">{unitBasisText}</span>
        </div>
      ) : null}
      {renderError ? (
        <p data-testid="rendered-report-error">Render failed: {renderError}</p>
      ) : null}
      {route?.route === "unavailable_browser_preview" ? (
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
              <a
                data-testid="rendered-report-save"
                download={`openpipestress-report-${outcome.sha256_hex.slice(0, 12)}.html`}
                href={`data:text/html;charset=utf-8,${encodeURIComponent(outcome.html)}`}
              >
                Save canonical HTML
              </a>
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
          <iframe
            title="Rendered calculation report preview"
            data-testid="rendered-report-preview"
            sandbox=""
            srcDoc={outcome.html}
            style={{ width: "100%", height: "20rem", border: "1px solid #888" }}
          />
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
