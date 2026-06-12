import react from "@vitejs/plugin-react";
import { existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig, type Plugin, type UserConfig } from "vite";

const desktopRoot = path.dirname(fileURLToPath(import.meta.url));

// TP-APP-R2-WASMPKG-001 build-time guard: the wasm operation engine is the
// sole browser-mode engine (DEC-020 / ADR-0001), so a production bundle
// without `public/wasm-engine/` assets is broken by construction — the
// packaged tauri:// asset protocol 404s onto index.html and the runtime dies
// with WASM-ENGINE-ASSET-ABSENT on first use. Fail the BUILD loudly instead
// of shipping that.
const WASM_ENGINE_ASSET_FILENAMES = [
  "open_pipe_stress_operation_applier.js",
  "open_pipe_stress_operation_applier_bg.wasm"
];

function missingWasmEngineAssets(dir: string): string[] {
  return WASM_ENGINE_ASSET_FILENAMES.filter((name) => !existsSync(path.join(dir, "wasm-engine", name)));
}

function wasmEngineGuardError(stage: string, dir: string, missing: string[]): Error {
  return new Error(
    `WASM-ENGINE-ASSET-ABSENT: ${stage}: missing ${missing.join(", ")} under ${dir}/wasm-engine/ — ` +
      "the wasm operation engine is required in browser mode (DEC-020 / ADR-0001) and no fallback engine " +
      "exists. Build it with `npm run build:wasm --workspace apps/desktop`, then rebuild."
  );
}

function wasmEngineDistGuard(): Plugin {
  return {
    name: "wasm-engine-dist-guard",
    apply: "build",
    buildStart() {
      const publicDir = path.join(desktopRoot, "public");
      const missing = missingWasmEngineAssets(publicDir);
      if (missing.length > 0) {
        throw wasmEngineGuardError("production build input check failed", publicDir, missing);
      }
    },
    closeBundle() {
      const distDir = path.join(desktopRoot, "dist");
      const missing = missingWasmEngineAssets(distDir);
      if (missing.length > 0) {
        throw wasmEngineGuardError("production dist output check failed", distDir, missing);
      }
    }
  };
}

export default defineConfig({
  plugins: [react(), wasmEngineDistGuard()],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: false
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (!id.includes("node_modules")) return undefined;
          if (id.includes("/three/")) return "vendor-three";
          if (id.includes("/react/") || id.includes("/react-dom/")) return "vendor-react";
          if (id.includes("/lucide-react/")) return "vendor-icons";
          if (id.includes("/@tauri-apps/")) return "vendor-tauri";
          return "vendor";
        }
      }
    }
  },
  test: {
    environment: "jsdom",
    globals: true,
    include: ["src/**/*.test.{ts,tsx}"],
    setupFiles: "./src/test/setup.ts"
  }
} as UserConfig & { test: Record<string, unknown> });
