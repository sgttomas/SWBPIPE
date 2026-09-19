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

/**
 * The in-app View menu's commands of slice B3: the three views, the four
 * stages, the theme and the density. They are not in `NATIVE_MENU_COMMAND_IDS`:
 * the native menu is `src-tauri/**`, outside this lane, so the native runtime
 * reaches them by the toolbar, the rail and the palette.
 */
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
