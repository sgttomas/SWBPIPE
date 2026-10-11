import { useEffect, useRef } from "react";
import { storedProjectDescription, type ReplaceProjectPromptState } from "./storedProjectConflict";

/** Asks before a save would replace a stored project. Cancel holds focus, Escape
 * cancels, and nothing is replaced unless Replace is chosen. */
export function ReplaceProjectPrompt({ prompt }: { prompt: ReplaceProjectPromptState | null }) {
  const cancel = useRef<HTMLButtonElement>(null);
  useEffect(() => { cancel.current?.focus(); }, [prompt]);
  if (!prompt) return null;
  return (
    <div className="replace-project-backdrop" data-testid="replace-project-backdrop">
      <section
        role="alertdialog"
        aria-modal="true"
        aria-labelledby="replace-project-title"
        aria-describedby="replace-project-detail"
        className="replace-project-prompt panel"
        data-testid="replace-project-prompt"
        onKeyDown={(event) => {
          if (event.key === "Escape") {
            event.preventDefault();
            event.stopPropagation();
            prompt.choose("cancel");
          }
        }}
      >
        <h2 id="replace-project-title">A project with this id is already stored</h2>
        <p id="replace-project-detail" data-testid="replace-project-detail">
          {prompt.kind === "create" ? "Creating" : "Saving"} this project would replace the stored project{" "}
          {storedProjectDescription(prompt.existing)}.
        </p>
        <div className="report-actions">
          <button type="button" ref={cancel} data-testid="replace-project-cancel" onClick={() => prompt.choose("cancel")}>
            Cancel
          </button>
          <button type="button" data-testid="replace-project-save-as-new" onClick={() => prompt.choose("save-as-new")}>
            Save as a new project
          </button>
          <button type="button" data-testid="replace-project-replace" onClick={() => prompt.choose("replace")}>
            Replace "{prompt.existing.project_name}"
          </button>
        </div>
      </section>
    </div>
  );
}
