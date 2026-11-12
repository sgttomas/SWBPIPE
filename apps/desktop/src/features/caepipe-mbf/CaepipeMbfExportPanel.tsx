import { FileText } from "lucide-react";
import type { PreviewModel } from "../../types";

/** The Exports section's Model batch file panel (DEL-17-04 Design `desktop-mbf-export-import.md` §3.1, slice 4b.2b).
 * Its one action opens the same Export MBF… dialog as File > Export MBF…, where the native writer reviews the session's
 * model and the save writes the model batch file and its identity file through the native chooser. The panel builds no
 * package or MBF text of its own: the TypeScript preview builder, its download links and its project-id file names are
 * retired (A-6). The export carries no result data, so the panel takes no result and has no result-output gate
 * (`caepipe-mbf` is not in `OUTPUT_SURFACES` since 4b.2b's follow-up): whatever result the session holds, or none,
 * the action is offered, and the export's own admission and checks decide. */
export function CaepipeMbfExportPanel({
  model,
  onExportMbf
}: {
  model: PreviewModel;
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
        {onExportMbf ? (
          <button type="button" data-testid="caepipe-mbf-export" onClick={() => onExportMbf()}>
            <FileText size={14} aria-hidden="true" />
            Export MBF…
          </button>
        ) : null}
      </div>
    </section>
  );
}
