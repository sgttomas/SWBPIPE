import { WORKSPACE_SECTIONS } from "./workspaceSections";
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
  | "insert.node"
  | "insert.pipe"
  | "insert.support"
  | "insert.component"
  | "insert.load"
  | "analyze.run"
  | "analyze.cancel"
  | "analyze.rule-checks";

export type MenuItemSpec =
  | { kind: "command"; id: MenuCommandId; label: string; disabled?: boolean; active?: boolean }
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
