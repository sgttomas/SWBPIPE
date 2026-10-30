import "@testing-library/jest-dom/vitest";
import { afterEach } from "vitest";

// Each case starts with an empty browser preview store, as a fresh page load
// does; a stored project left by an earlier case would otherwise make that
// case's first save a replacement (HELP_HUMAN ruling 2026-10-10). Imported
// lazily so a test file's own module mocks are in place first.
afterEach(async () => (await import("../services/projectService")).clearBrowserPreviewStore());
// Likewise each case starts as a fresh webview session for native result
// custody: its startup release has not run and no job has been released yet.
afterEach(async () => (await import("../services/previewService")).resetSessionResultCustodyForTests?.());

// Pre-warm the wasm operation engine (DEC-020 / ADR-0001) so jsdom suites
// that exercise the edit loop start with the engine ready. An absent
// artifact fails the suite loudly here with the exact build command
// (WASM-ENGINE-ASSET-ABSENT → `npm run build:wasm --workspace apps/desktop`);
// no fallback engine exists.
import { loadWasmEngine } from "../services/wasmEngine/loadWasmEngine";

await loadWasmEngine();
