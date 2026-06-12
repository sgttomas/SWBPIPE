import "@testing-library/jest-dom/vitest";

// Pre-warm the wasm operation engine (DEC-020 / ADR-0001) so jsdom suites
// that exercise the edit loop start with the engine ready. An absent
// artifact fails the suite loudly here with the exact build command
// (WASM-ENGINE-ASSET-ABSENT → `npm run build:wasm --workspace apps/desktop`);
// no fallback engine exists.
import { loadWasmEngine } from "../services/wasmEngine/loadWasmEngine";

await loadWasmEngine();
