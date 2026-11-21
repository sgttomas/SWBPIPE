import { useEffect, useRef, useState } from "react";

/** DEL-07-12: what an export row needs of the save that writes its file, and the file's suggested name.
 *
 * The native save is built with the export control and defines its outcome; the row adapts to it. The row says
 * the outcome at the row, in the result JSON row's form: one status line under the save. Until the save exists for
 * a row, the row keeps its present link and says nothing it cannot know: no words after a press of the link. */

/** What the row hands the save. */
export type ExportSaveRequest = {
  /** The save control's test id; it names the export route (`routeBindingForTestId`). */
  control: string;
  /** The suggested file name (`exportFileName`), which the person may change in the save panel. */
  suggestedName: string;
  /** The file as the control would write it: its data address, after the control's screening. */
  href: string;
};

/** The six outcomes of a save, as the row reads them. THE ONE PLACE WHERE THE SAVE'S OWN MEMBER NAMES ARE READ is
 * `rowSaveOutcome` below: when the save's exact outcome type lands, it maps that type to this one, and nothing else
 * in the rows changes. */
export type RowSaveOutcome =
  /** Saved: the file's name as the person confirmed it in the save panel, never its folder. */
  | { kind: "saved"; name: string }
  /** Saved, with its durability not confirmed. */
  | { kind: "saved-unconfirmed"; name: string }
  | { kind: "cancelled" }
  /** Refused: the reason in plain words, then its code. */
  | { kind: "refused"; reason: string; code: string }
  /** Failed: at which stage, and whether a temporary file may remain. */
  | { kind: "failed"; stage: string; temporaryMayRemain: boolean }
  /** Not known: never worded as nothing written. */
  | { kind: "unknown" };

/** The function a row calls, one call per press. */
export type SaveExport = (request: ExportSaveRequest) => Promise<unknown>;

/** The state a row's status line shows: nothing yet, a save in progress, or its outcome. */
export type RowSaveState = null | { kind: "saving" } | RowSaveOutcome;

/** Maps the save's own outcome to the row's. Placeholder until the save's type lands: it takes the row's shape. */
export function rowSaveOutcome(outcome: unknown): RowSaveOutcome {
  const value = outcome as RowSaveOutcome | null;
  return value && typeof value === "object" && typeof value.kind === "string" ? value : { kind: "unknown" };
}

/** The status line's words, in one place: one line for each outcome. The saved-unconfirmed, failed and unknown
 * lines are placeholders for the save's own words. */
export function rowSaveStatusText(state: RowSaveState): string {
  if (state === null) return "";
  switch (state.kind) {
    case "saving": return "Saving…";
    case "saved": return `Saved as ${state.name}.`;
    case "saved-unconfirmed": return `Saved as ${state.name}. The system did not confirm it is safely on disk.`;
    case "cancelled": return "Not saved: the save was cancelled.";
    case "refused": return `Not saved: ${state.reason} (${state.code})`;
    case "failed": return `Not saved: the write failed while ${state.stage}.${state.temporaryMayRemain ? " A temporary file may remain beside it." : ""}`;
    case "unknown": return "It is not known whether the file was saved.";
  }
}

/** A row's save: calls `save` once per press and keeps its outcome only while the file is the one it saved. An
 * outcome that arrives after the file changed (`file` is no longer the same) is ignored, and the line clears. */
export function useRowSave(save: SaveExport | null, file: unknown): { state: RowSaveState; run: (request: ExportSaveRequest) => void } {
  const [state, setState] = useState<{ file: unknown; value: RowSaveState }>({ file, value: null });
  const current = useRef(file);
  current.current = file;
  const mounted = useRef(true);
  useEffect(() => () => { mounted.current = false; }, []);
  const run = (request: ExportSaveRequest) => {
    if (!save) return;
    const saved = file;
    setState({ file: saved, value: { kind: "saving" } });
    save(request).then(
      (outcome) => { if (mounted.current && current.current === saved) setState({ file: saved, value: rowSaveOutcome(outcome) }); },
      () => { if (mounted.current && current.current === saved) setState({ file: saved, value: { kind: "unknown" } }); }
    );
  };
  return { state: state.file === file ? state.value : null, run };
}

const SLUG_LIMIT = 64;

function slug(text: string | null | undefined): string {
  return (text ?? "").toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "").slice(0, SLUG_LIMIT).replace(/-+$/, "");
}

/** The project's name as a file-name slug: its display name; where that leaves nothing, its id without the leading
 * "project:"; where that leaves nothing too, "project". Lower case; every run of characters outside a to z and 0
 * to 9 one hyphen; no hyphen at either end; at most 64 characters, cut at the limit with any trailing hyphen
 * removed. */
export function projectSlug(project: { id: string; name?: string | null }): string {
  return slug(project.name) || slug(project.id.replace(/^project:/, "")) || "project";
}

/** The suggested name of an exported file: the project's slug, a hyphen, the kind of file and its extension, both
 * from the row's fixed table (`EXPORT_FILE_KINDS`), never from the model. */
export function exportFileName(project: { id: string; name?: string | null }, file: { kind: string; extension: string }): string {
  return `${projectSlug(project)}-${file.kind}.${file.extension}`;
}

/** Each row's kinds of file and their extensions: the fixed table the names are built from. The result JSON, the
 * stress-neutral JSON and review geometry keep their present names until the native save and the review geometry
 * generator move with them; their entries here are the intended names. */
export const EXPORT_FILE_KINDS = Object.freeze({
  resultJson: { kind: "result", extension: "json" },
  stressNeutralJson: { kind: "stress-neutral", extension: "json" },
  stressNeutralCsv: { kind: "stress-neutral", extension: "csv" },
  pcfPackage: { kind: "pcf-package", extension: "json" },
  pcfText: { kind: "pcf", extension: "pcf" },
  reviewGeometry: { kind: "review-geometry", extension: "gltf.json" },
  localFeaHandoff: { kind: "local-fea-handoff", extension: "json" }
});

/** The local FEA handoff's name: where the handoff names a region, its slug after the kind, by the same rule. */
export function localFeaHandoffFileName(project: { id: string; name?: string | null }, regionId: string | null): string {
  const region = slug(regionId?.replace(/^local-region:/, ""));
  const file = EXPORT_FILE_KINDS.localFeaHandoff;
  return `${projectSlug(project)}-${file.kind}${region ? `-${region}` : ""}.${file.extension}`;
}
