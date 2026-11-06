import { FileText } from "lucide-react";
import type { SessionAnalysisRun, SessionResult } from "../../services/nativeResult";
import type { PreviewModel } from "../../types";
import { LoadReferenceOutputGate } from "../results/LoadReferenceOutputGate";

/** The Exports section's Model batch file panel (DEL-17-04 Design `desktop-mbf-export-import.md` §3.1, slice 4b.2b).
 * Its one action opens the same Export MBF… dialog as File > Export MBF…, where the native writer reviews the session's
 * model and the save writes the model batch file and its identity file through the native chooser. The panel builds no
 * package or MBF text of its own: the TypeScript preview builder, its download links and its project-id file names are
 * retired (A-6). The panel stays behind its output gate (`caepipe-mbf` in `OUTPUT_SURFACES`), unchanged by this slice,
 * and the File menu's item applies the same gate. */
export function CaepipeMbfExportPanel({
  model,
  result,
  onExportMbf
}: {
  model: PreviewModel;
  result: SessionResult | null;
  analysisRun?: SessionAnalysisRun | null;
  /** Opens Export MBF… (absent where no session can open it: the panel then offers nothing). */
  onExportMbf?: () => void;
}) {
  return (
    <section className="panel caepipe-mbf-panel" aria-label="Model batch file (.mbf) export" data-testid="caepipe-mbf-panel">
      <div className="panel-title">
        <FileText size={16} aria-hidden="true" />
        Model batch file (.mbf)
      </div>
      <p className="muted" data-testid="caepipe-mbf-description">
        Writes the model of "{model.project.name}" as a model batch file and its identity file in a folder you choose,
        after a review of what the file holds and leaves out.
      </p>
      <div className="report-actions">
        <LoadReferenceOutputGate result={result} testIdPrefix="caepipe-mbf">
          {onExportMbf ? (
            <button type="button" data-testid="caepipe-mbf-export" onClick={() => onExportMbf()}>
              <FileText size={14} aria-hidden="true" />
              Export MBF…
            </button>
          ) : null}
        </LoadReferenceOutputGate>
      </div>
    </section>
  );
}
