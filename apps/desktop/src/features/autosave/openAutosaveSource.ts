// Open autosave (DEL-02-05 Design `autosave-and-backup.md` §3): after the
// unsaved-changes guard, the record is read verified (`read_project_autosave`,
// every check of §5) and given the open path's webview checks (the shape check
// and the validator, `checkNativeOutcome`). The session then adopts it exactly
// as Open Model Document adopts a file, with no stored base: a later Save Local
// Project onto a stored id asks through the replace prompt.
import { checkNativeOutcome, type ModelDocumentSource } from "../../services/modelDocumentFile";
import { readProjectAutosave } from "./autosaveService";

export function openAutosaveSource(fileName: string, writtenAt: string): ModelDocumentSource {
  return {
    action: "Opening an autosave",
    read: async () => {
      const read = await readProjectAutosave(fileName);
      if (read.outcome === "refused") return { outcome: "refused", file_name: fileName, diagnostics: read.diagnostics };
      // The native read admitted only a current model document.
      return checkNativeOutcome({ outcome: "opened", file_name: fileName, byte_count: read.byte_count, schema_status: "current", document: read.document, warnings: [] });
    },
    opened: (opened) => `Opened the autosave of ${opened.document.project.name} written ${writtenAt} (${opened.byte_count} bytes). No stored project was changed.`,
    operation: "open_autosave"
  };
}
