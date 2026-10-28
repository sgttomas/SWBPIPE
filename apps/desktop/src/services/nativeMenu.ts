import { invoke } from "@tauri-apps/api/core";

// Native macOS menu bridge (TP-R3UX-CADSHELL).
//
// The Tauri shell owns an OS-level menu bar (File/Edit/View/Insert/Analyze).
// Each menu click emits a `native-menu-command` event carrying the command id;
// the React app listens here and dispatches it through the same command sink
// that drives the in-DOM menu bar. In the browser/Playwright preview there is
// no Tauri runtime and no OS menu bar, so this is inert — the app stays fully
// navigable and testable through the in-DOM menu.

const NATIVE_MENU_EVENT = "native-menu-command";

export function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

/**
 * Subscribe to native menu commands. Resolves to an unlisten function.
 * Returns a no-op disposer when not running inside the Tauri shell, and never
 * imports the Tauri event module in that case.
 */
export async function listenToNativeMenu(
  onCommand: (command: string) => void
): Promise<() => void> {
  if (!isTauriRuntime()) {
    return () => {};
  }
  const { listen } = await import("@tauri-apps/api/event");
  const unlisten = await listen<string>(NATIVE_MENU_EVENT, (event) => {
    onCommand(event.payload);
  });
  return unlisten;
}

/** Presentation state only; native commands still use the one session sink. */
export interface NativeShellState {
  projectName: string | null;
  modelEdited: boolean;
  projectBusy: boolean;
  stage: string;
  view: string;
  theme: string;
  density: string;
  resultsStageEnabled: boolean;
  reviewStageEnabled: boolean;
  inspectorOpen: boolean;
  canUndo: boolean;
  canRedo: boolean;
  canRun: boolean;
  canCancel: boolean;
}

export async function syncNativeShellState(state: NativeShellState): Promise<void> {
  if (!isTauriRuntime()) return;
  await invoke("sync_native_shell_state", { state });
}

/** A window close or quit the native host holds while the working model has
 * unsaved edits (`src-tauri/src/close_guard.rs`), dispatched to the webview as
 * the `openpipestress-native-close-request` DOM event. */
export const NATIVE_CLOSE_REQUEST_DOM_EVENT = "openpipestress-native-close-request";
export type NativeCloseRequest = { requestId: number; kind: "window" | "quit" };

export function nativeCloseRequestFrom(detail: unknown): NativeCloseRequest | null {
  if (typeof detail !== "object" || detail === null) return null;
  const { requestId, kind } = detail as Record<string, unknown>;
  if (typeof requestId !== "number" || !Number.isSafeInteger(requestId) || (kind !== "window" && kind !== "quit")) return null;
  return { requestId, kind };
}

/** Tells the host the request arrived; an unacknowledged request proceeds after the host's deadline. */
export async function acknowledgeNativeCloseRequest(requestId: number): Promise<void> {
  if (!isTauriRuntime()) return;
  await invoke("acknowledge_close_request", { requestId });
}

/** The user's decision: `proceed` closes or quits; otherwise the app stays open. */
export async function resolveNativeCloseRequest(requestId: number, proceed: boolean): Promise<void> {
  if (!isTauriRuntime()) return;
  await invoke("resolve_close_request", { requestId, proceed });
}
