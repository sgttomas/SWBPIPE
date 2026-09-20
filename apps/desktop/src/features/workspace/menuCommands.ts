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
  | "file.open-local"
  | "file.list-local"
  | "file.save-local"
  | "file.save-report-package"
  | "edit.undo"
  | "edit.redo"
  | "view.tree"
  | "view.inspector"
  | "view.issues"
  | "view.audit"
  | "view.close-panels"
  | `view.section.${WorkspaceSectionId}`
  | ShellMenuCommandId
  | "insert.node"
  | "insert.pipe"
  | "insert.support"
  | "insert.component"
  | "insert.load"
  | "analyze.run"
  | "analyze.cancel"
  | "analyze.rule-checks";

/** Shell commands share IDs across native menu, browser menu and palette. */
export type ShellMenuCommandId =
  | `view.view.${ShellView}`
  | `view.stage.${ShellStage}`
  | `view.theme.${UiThemePreference}`
  | `view.density.${UiDensityPreference}`;

export type MenuItemSpec =
  | { kind: "command"; id: MenuCommandId; label: string; disabled?: boolean; active?: boolean; reason?: string | null }
  | { kind: "separator" };

const NATIVE_MENU_COMMAND_IDS: ReadonlySet<string> = new Set([
  "file.new-local",
  "file.new-blank",
  "file.open-local",
  "file.list-local",
  "file.save-local",
  "file.save-report-package",
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
  "analyze.rule-checks"
]);

export function isMenuCommandId(value: string): value is MenuCommandId {
  return NATIVE_MENU_COMMAND_IDS.has(value);
}
