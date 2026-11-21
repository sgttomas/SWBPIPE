import { WORKSPACE_SECTIONS } from "./workspaceSections";
import type { ShellStage, ShellView } from "./shellLayout";
import type { UiDensityPreference, UiThemePreference } from "./uiPreferences";
import type { WorkspaceSectionId } from "./workspaceSections";

// CAD-shell menu model (TP-R3UX-CADSHELL). The in-DOM menu bar is the tested
// source of truth; the native macOS menu (Tauri) emits these same command ids.
export type MenuId = "file" | "edit" | "view" | "insert" | "analyze";

export type MenuCommandId =
  | "file.new-local"
  | "file.new-blank"
  | "file.open-model"
  | "file.import-mbf"
  | "file.open-local"
  | "file.list-local"
  | "file.save-local"
  | "file.save-model-document"
  | "file.save-handoff"
  | "file.export-mbf"
  | "file.save-report-package"
  | "file.save-shareable-report-package"
  | "edit.undo"
  | "edit.redo"
  | "view.tree"
  | "view.inspector"
  | "view.issues"
  | "view.audit"
  | "view.close-panels"
  // A6 key results: the browser menu's route. The native menu has no item for it yet, so it is
  // not in NATIVE_MENU_COMMAND_IDS, and App handles it before the session's runMenuCommand.
  | "view.key-results"
  | `view.section.${WorkspaceSectionId}`
  | ShellMenuCommandId
  | "insert.node"
  | "insert.pipe"
  | "insert.support"
  | "insert.component"
  | "insert.load"
  | "analyze.run"
  | "analyze.cancel"
  | "analyze.rule-checks"
  | "analyze.check-model";

/** Shell commands share IDs across native menu, browser menu and palette. */
export type ShellMenuCommandId =
  | `view.view.${ShellView}`
  | `view.stage.${ShellStage}`
  | `view.theme.${UiThemePreference}`
  | `view.density.${UiDensityPreference}`;

export type MenuItemSpec =
  | { kind: "command"; id: MenuCommandId; label: string; disabled?: boolean; active?: boolean; reason?: string | null;
      /** The item's keyboard equivalent as shown, with its `aria-keyshortcuts` form. */
      shortcut?: { label: string; aria: string } }
  | { kind: "separator" };

/**
 * The workspace sections both View menus list (the in-app one here, the native one in
 * `src-tauri/src/lib.rs`, NATIVE_VIEW_SECTIONS). Review changes and Load Cases are not
 * listed: the Model stage's Review changes tab and the Loads stage reach them in every
 * state. Results, Evidence and Report stay, because their stages are disabled until a
 * run (or a current solved run) exists while the sections still hold what needs none:
 * the weight summary (DEL-07-05 REQ-022), the boundary panels, and a held result's
 * report (DEL-08-01 REQ-017). Their `view.section.*` commands are all still accepted.
 */
export const VIEW_MENU_SECTIONS: ReadonlySet<WorkspaceSectionId> = new Set<WorkspaceSectionId>([
  "libraries",
  "rule-packs",
  "solve",
  "results",
  "report",
  "project",
  "exports",
  "evidence"
]);

// Every id a native menu event may carry, and so every id the command sink accepts from
// it. It keeps `view.tree` and every `view.section.*` the menus no longer list: the
// scripted-run build and the tests still dispatch them.
const NATIVE_MENU_COMMAND_IDS: ReadonlySet<string> = new Set([
  "file.new-local",
  "file.new-blank",
  "file.open-model",
  "file.import-mbf",
  "file.open-local",
  "file.list-local",
  "file.save-local",
  "file.save-model-document",
  "file.save-handoff",
  "file.export-mbf",
  "file.save-report-package",
  "file.save-shareable-report-package",
  "edit.undo",
  "edit.redo",
  "view.tree",
  "view.inspector",
  "view.issues",
  "view.audit",
  "view.close-panels",
  ...["table", "model", "both"].map((view) => `view.view.${view}`),
  ...["model", "loads", "results", "review"].map((stage) => `view.stage.${stage}`),
  ...["light", "dark", "system"].map((theme) => `view.theme.${theme}`),
  ...["comfortable", "compact"].map((density) => `view.density.${density}`),
  ...WORKSPACE_SECTIONS.map((section) => `view.section.${section.id}`),
  "insert.node",
  "insert.pipe",
  "insert.support",
  "insert.component",
  "insert.load",
  "analyze.run",
  "analyze.cancel",
  "analyze.rule-checks",
  "analyze.check-model"
]);

export function isMenuCommandId(value: string): value is MenuCommandId {
  return NATIVE_MENU_COMMAND_IDS.has(value);
}
