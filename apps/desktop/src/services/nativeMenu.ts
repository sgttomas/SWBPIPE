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
