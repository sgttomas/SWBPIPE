// The native smoke's app-binary binding (DEL-07-11 VER-003).
//
// The app embeds `dist/` (and through it the WebAssembly engines) when cargo
// builds it, so a current `dist/` does not establish a current binary: a cargo
// build that failed or never ran after `build:wasm` and `npm run build` leaves
// the old binary with its old engines. After each successful app build,
// `native-smoke/run.mjs` writes a binding record next to the binary
// (`<binary>.native-smoke-binding.json`, inside cargo's git-ignored target
// directory) naming the engine stamps and the `dist/` digest the build
// embedded, the build command and the binary's size and SHA-256. `--skip-build`
// skips the app build only when that record matches all of them now.
import { createHash } from "node:crypto";
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import path from "node:path";

export const BINDING_FORMAT = "openpipestress.native-smoke-app-binding.v1";

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

export const bindingPathFor = (binary) => `${binary}.native-smoke-binding.json`;

// The binary's size and SHA-256, or null when it is not a file.
export function fileIdentity(file) {
  if (!existsSync(file) || !statSync(file).isFile()) return null;
  const bytes = readFileSync(file);
  return { size: bytes.length, sha256: `sha256:${sha256(bytes)}` };
}

// A SHA-256 over every file under `directory` (relative POSIX path and bytes,
// sorted; symbolic links followed), or null when it is not a directory.
export function treeDigest(directory) {
  if (!existsSync(directory) || !statSync(directory).isDirectory()) return null;
  const files = [];
  const visit = (absolute, relative) => {
    for (const entry of readdirSync(absolute, { withFileTypes: true })) {
      const childAbsolute = path.join(absolute, entry.name);
      const childRelative = relative ? `${relative}/${entry.name}` : entry.name;
      const stat = statSync(childAbsolute, { throwIfNoEntry: false });
      if (!stat) continue;
      if (stat.isDirectory()) visit(childAbsolute, childRelative);
      else if (stat.isFile()) files.push([childRelative, childAbsolute]);
    }
  };
  visit(directory, "");
  files.sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0));
  const hash = createHash("sha256");
  for (const [relative, absolute] of files) {
    const bytes = readFileSync(absolute);
    hash.update(`file ${JSON.stringify(relative)} ${bytes.length} ${sha256(bytes)}\n`);
  }
  return `sha256:${hash.digest("hex")}`;
}

// The record of one successful app build. `engines` maps engine name to stamp
// hash; `dist` is the `dist/` digest; `build` the command and its inputs that
// are not files (features, TAURI_CONFIG); `binary` the binary's identity.
export function bindingRecord({ engines, dist, build, binary }) {
  return { format: BINDING_FORMAT, engines: Object.fromEntries(Object.entries(engines).sort()), dist, build, binary };
}

// Whether a binding record's text (null when absent) describes the current
// state: { match, reason }.
export function checkBinding(bindingText, current) {
  if (bindingText === null || bindingText === undefined) return { match: false, reason: "no app binding record (the app was not built by the native smoke since its last build)" };
  let record;
  try {
    record = JSON.parse(bindingText);
  } catch {
    return { match: false, reason: "app binding record unreadable" };
  }
  if (!record || typeof record !== "object" || record.format !== BINDING_FORMAT) return { match: false, reason: `app binding record is not ${BINDING_FORMAT}` };
  if (current.binary === null) return { match: false, reason: "app binary missing" };
  const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);
  if (!same(record.engines, bindingRecord(current).engines)) return { match: false, reason: "the engine stamps differ from those the app binary embedded" };
  if (record.dist !== current.dist) return { match: false, reason: "dist/ differs from the copy the app binary embedded" };
  if (!same(record.build, current.build)) return { match: false, reason: "the app build command differs from the binary's" };
  if (!same(record.binary, current.binary)) return { match: false, reason: "the app binary changed since its binding was written" };
  return { match: true, reason: null };
}

// What the native smoke builds. `staleEngines` are engines build:wasm rebuilt;
// `distStale` engines whose dist/ copy differs from the engine stamp
// ({ name, reason }); `binding` the checkBinding result (consulted only under
// --skip-build when nothing else forces a build).
export function planAppBuild({ skipBuild, staleEngines, distStale, binding }) {
  if (!skipBuild) return { frontend: true, app: true, log: null };
  if (staleEngines.length > 0 || distStale.length > 0) {
    const reasons = [...staleEngines.map((name) => `${name} rebuilt`), ...distStale.map((engine) => `dist/${engine.name}: ${engine.reason}`)];
    return { frontend: true, app: true, log: `native smoke: --skip-build overridden (${reasons.join("; ")}); the app embeds dist/, so the frontend and app are rebuilt` };
  }
  if (!binding.match) {
    return { frontend: false, app: true, log: `native smoke: --skip-build overridden for the app (${binding.reason}); cargo builds it from the current dist/` };
  }
  return { frontend: false, app: false, log: "native smoke: --skip-build applies: engines, their dist/ copies and the app binary's binding are current; frontend and app builds skipped" };
}
