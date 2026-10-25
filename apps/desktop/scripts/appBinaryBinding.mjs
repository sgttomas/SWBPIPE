// The native smoke's frontend and app builds and the app-binary binding
// (DEL-07-11 VER-003).
//
// The app embeds `dist/` (and through it the WebAssembly engines) when cargo
// builds it, so neither a current `dist/` nor a successful cargo run establishes
// what the binary contains. `native-smoke/run.mjs` calls `prepareSmokeApp`,
// which builds what is needed and writes a binding record
// (`target/native-smoke/app-binding.json` under this app, git-ignored) after
// each app build whose inputs held still. The record names:
//
//   - `engines`: the engine stamps (`engineSourceStamp.mjs`) when cargo ran;
//   - `frontend`: a hash of the frontend inputs `dist/` was built from: every
//     file under `src/`, the app's `index.html`, `package.json`, Vite, TS,
//     PostCSS and `.env` files, `public/` other than the engine folders (they
//     carry their own stamps), the npm workspace's `package.json` and
//     `package-lock.json`, and each file outside `src/` that a source names in a
//     relative `import`/`from`/`import(...)` specifier (the project fixtures);
//   - `dist`: a digest of `dist/` as cargo embedded it;
//   - `app_sources`: a hash, with the engine stamp's toolchain context, of the
//     app crate's closure (`src-tauri` and its path dependencies, by cargo
//     metadata: manifests, build scripts, `src/`, the lock file), every file at
//     the top of `src-tauri/` (`tauri.conf.json`, `build.rs`, ...), `icons/` and
//     `capabilities/`, and the compile reads the binary's dep-info lists outside
//     those, `dist/` and the target directory; those reads are recorded;
//   - `build`: the cargo command and the smoke's TAURI_CONFIG;
//   - `binary`: the path cargo reported for the executable, its size and SHA-256.
//
// Cargo runs with CARGO_TARGET_DIR set to `src-tauri/target`, the crate's own
// directory in this checkout, and `--message-format=json-render-diagnostics`; the
// executable is the one cargo's `compiler-artifact` message reports, which must
// lie in that directory, and nothing else is bound or launched. When the app
// sources are not the ones a valid record names, `cargo clean -p` removes the
// closure's own packages from that directory first, so cargo's mtime-based
// freshness cannot keep an old binary. `dist/` changing while cargo builds, or
// frontend inputs changing while Vite builds, aborts the run by name.
//
// Under `--skip-build` the frontend is rebuilt (and with it the app) when an
// engine was rebuilt, a `dist/` engine copy's stamp differs, the record is
// missing or unreadable, or the frontend hash or the `dist/` digest differs; the
// app alone when the engine stamps, app sources, build or binary differ. The log
// names which applies.
import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, readdirSync, readFileSync, realpathSync, rmSync, statSync, writeFileSync } from "node:fs";
import path from "node:path";
import {
  compileReadsFromDepInfo,
  crateClosure,
  decideEngine,
  digestInputs,
  hashDigests,
  inputDrift,
  linkBeforeParent,
  listTreeFiles,
  projectRelative,
  readCrateManifest,
  readStampText
} from "./engineSourceStamp.mjs";

export const BINDING_FORMAT = "openpipestress.native-smoke-app-binding.v2";
export const MAX_STABLE_APP_BUILD_ATTEMPTS = 3;
const FRONTEND_ROOT_FILE = /^(index\.html|package\.json|vite\.config\.[cm]?[jt]s|tsconfig(\..+)?\.json|postcss\.config\..+|tailwind\.config\..+|\.env(\..+)?|\.browserslistrc)$/;
const FRONTEND_SOURCE = /\.(m?[jt]sx?|css)$/;
// `from "./x"`, `import "./x"`, `import("./x")`, `@import "./x"`, with any `?query`.
const RELATIVE_SPECIFIER = /(?:\bfrom|\bimport)\s*\(?\s*["'](\.{1,2}\/[^"'?#]+)/g;
const RESOLVE_SUFFIXES = ["", ".ts", ".tsx", ".js", ".jsx", ".mjs", "/index.ts", "/index.tsx", "/index.js"];

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");
const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);
const sorted = (map) => Object.fromEntries(Object.entries(map).sort());
const toPosix = (value) => value.split(path.sep).join("/");

export const bindingPathFor = (desktopRoot) => path.join(desktopRoot, "target", "native-smoke", "app-binding.json");
export const appTargetDirectory = (tauriDir) => path.join(tauriDir, "target");

// The file's size and SHA-256, or null when it is not a file.
export function fileIdentity(file) {
  if (!file || !existsSync(file) || !statSync(file).isFile()) return null;
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

// Project-relative files outside `src/` that the frontend sources name by a
// relative specifier, by the path as written (so that a symbolic link on the way
// is part of its identity). One that leaves the project, names no file or has a
// ".." after a symbolic link is refused by name.
export function frontendImportsOutsideSource(projectRoot, sourceFiles, sourcePrefix) {
  const found = new Set();
  const isFile = (candidate) => existsSync(candidate) && statSync(candidate).isFile();
  for (const relative of sourceFiles.filter((file) => FRONTEND_SOURCE.test(file))) {
    const absolute = path.join(projectRoot, relative);
    for (const match of readFileSync(absolute, "utf8").matchAll(RELATIVE_SPECIFIER)) {
      const written = `${path.dirname(absolute)}/${match[1]}`;
      const base = path.resolve(written);
      const shown = toPosix(path.relative(projectRoot, base));
      if (shown.startsWith(sourcePrefix)) continue;
      const refuse = (why) => new Error(`native smoke: ${relative} imports ${match[1]}, ${why}; the frontend binding cannot hash it`);
      if (shown.startsWith("..") || path.isAbsolute(shown)) throw refuse("outside the project");
      if (linkBeforeParent(written)) throw refuse('with a ".." after a symbolic link');
      const suffix = RESOLVE_SUFFIXES.find((candidate) => isFile(`${base}${candidate}`));
      if (suffix === undefined) throw refuse("which names no file");
      found.add(`${shown}${suffix}`);
    }
  }
  return [...found].sort();
}

// The frontend inputs (project-relative). `engineFolders` are the `public/`
// folders with their own stamps.
export function frontendInputs(projectRoot, desktopRoot, engineFolders) {
  const desktop = projectRelative(projectRoot, desktopRoot);
  const inputs = new Set(["package.json", "package-lock.json"]);
  for (const entry of readdirSync(desktopRoot, { withFileTypes: true })) {
    if (!entry.isDirectory() && FRONTEND_ROOT_FILE.test(entry.name)) inputs.add(`${desktop}/${entry.name}`);
  }
  const source = listTreeFiles(projectRoot, `${desktop}/src`);
  for (const file of source) inputs.add(file);
  for (const file of listTreeFiles(projectRoot, `${desktop}/public`)) {
    if (!engineFolders.some((folder) => file.startsWith(`${desktop}/public/${folder}/`))) inputs.add(file);
  }
  for (const file of frontendImportsOutsideSource(projectRoot, source, `${desktop}/src/`)) inputs.add(file);
  return [...inputs].sort();
}

// The app's input set: the static inputs (crate closure, top-level files,
// icons, capabilities) and the compile reads not among them.
export function appInputs({ projectRoot, tauriDir, compileReads = [], readManifest = readCrateManifest }) {
  const closure = crateClosure(projectRoot, tauriDir, readManifest);
  const crate = projectRelative(projectRoot, tauriDir);
  const inputs = new Set(closure.inputs);
  for (const entry of readdirSync(tauriDir, { withFileTypes: true })) {
    if (!entry.isDirectory()) inputs.add(`${crate}/${entry.name}`);
  }
  for (const folder of ["icons", "capabilities"]) for (const file of listTreeFiles(projectRoot, `${crate}/${folder}`)) inputs.add(file);
  const staticInputs = [...inputs].sort();
  const recorded = [...new Set(compileReads)].filter((relative) => !inputs.has(relative)).sort();
  return { staticInputs, recorded, all: [...staticInputs, ...recorded], packages: closure.packages };
}

// The executable cargo's `compiler-artifact` message reports for the `binName`
// binary of the package at `manifestPath`, from `--message-format=json*` output.
export function appExecutableFromCargoMessages(stdout, manifestPath, binName) {
  const wanted = existsSync(manifestPath) ? realpathSync(manifestPath) : path.resolve(manifestPath);
  let executable = null;
  for (const line of stdout.split("\n")) {
    if (!line.startsWith("{")) continue;
    let message;
    try {
      message = JSON.parse(line);
    } catch {
      continue;
    }
    if (message.reason !== "compiler-artifact" || typeof message.executable !== "string") continue;
    if (message.target?.name !== binName || !(message.target?.kind ?? []).includes("bin")) continue;
    const reported = typeof message.manifest_path === "string" && existsSync(message.manifest_path) ? realpathSync(message.manifest_path) : message.manifest_path;
    if (reported === wanted) executable = message.executable;
  }
  if (executable === null) {
    throw new Error(`native smoke: cargo reported no compiler-artifact executable for ${binName} (${manifestPath}); nothing is bound or launched`);
  }
  return executable;
}

// The record of one app build.
export function bindingRecord({ engines, frontend, dist, app_sources, build, binary }) {
  return { format: BINDING_FORMAT, engines: sorted(engines), frontend, dist, app_sources, build, binary };
}

// A binding record's text (null when absent) as a record, or the reason it is not one.
export function parseBinding(text) {
  if (text === null || text === undefined) return { record: null, problem: "no app binding record (the app was not built by the native smoke since its last build)" };
  let record;
  try {
    record = JSON.parse(text);
  } catch {
    return { record: null, problem: "app binding record unreadable" };
  }
  if (!record || typeof record !== "object" || record.format !== BINDING_FORMAT) return { record: null, problem: `app binding record is not ${BINDING_FORMAT}` };
  if (typeof record.binary?.path !== "string" || !Array.isArray(record.app_sources?.compile_reads)) return { record: null, problem: "app binding record unreadable" };
  return { record, problem: null };
}

// What the native smoke builds. `staleEngines` are engines build:wasm rebuilt;
// `distStale` engines whose dist/ copy differs from the engine stamp
// ({ name, reason }); `binding` a parseBinding result; `current` the state now
// ({ engines, frontend, dist, app_sources: hash, build, binary: identity of the
// record's binary path }).
export function planAppBuild({ skipBuild, staleEngines = [], distStale = [], binding = { record: null, problem: null }, current = null }) {
  if (!skipBuild) return { frontend: true, app: true, log: null };
  const both = (reasons) => ({ frontend: true, app: true, log: `native smoke: --skip-build overridden (${reasons.join("; ")}); the app embeds dist/, so the frontend and app are rebuilt` });
  if (staleEngines.length > 0 || distStale.length > 0) {
    return both([...staleEngines.map((name) => `${name} rebuilt`), ...distStale.map((engine) => `dist/${engine.name}: ${engine.reason}`)]);
  }
  const { record, problem } = binding;
  if (problem) return both([problem]);
  if (record.frontend !== current.frontend) return both(["the frontend sources differ from those dist/ was built from"]);
  if (record.dist !== current.dist) return both(["dist/ differs from the copy the app binary embedded"]);
  const appReason =
    (!same(record.engines, sorted(current.engines)) && "the engine stamps differ from those the app binary embedded") ||
    (record.app_sources.hash !== current.app_sources && "the app's Rust sources differ from those the binary was built from") ||
    (!same(record.build, current.build) && "the app build command differs from the binary's") ||
    (current.binary === null && "app binary missing") ||
    (!same({ size: record.binary.size, sha256: record.binary.sha256 }, current.binary) && "the app binary changed since its binding was written");
  if (appReason) return { frontend: false, app: true, log: `native smoke: --skip-build overridden for the app (${appReason}); cargo builds it from the current dist/` };
  return { frontend: false, app: false, log: "native smoke: --skip-build applies: engines, their dist/ copies, the frontend and app sources and the app binary's binding are current; frontend and app builds skipped" };
}

// Build what the smoke needs and bind the app binary. Returns the executable to
// launch and its identity; throws a "native smoke: ..." error when nothing may
// be launched. `runFrontend()` runs the frontend build; `runCargo(command, env)`
// runs a cargo command in `tauriDir` and returns its standard output.
export function prepareSmokeApp({
  projectRoot,
  desktopRoot,
  tauriDir,
  binName,
  skipBuild,
  staleEngines = [],
  engineFolders,
  engineHashes,
  build,
  context = [],
  runFrontend,
  runCargo,
  log = console.log,
  readManifest: readManifestUncached = readCrateManifest
}) {
  // Each crate's manifest facts once per run; its files are listed and digested each time.
  const manifests = new Map();
  const readManifest = (manifest) => {
    if (!manifests.has(manifest)) manifests.set(manifest, readManifestUncached(manifest));
    return manifests.get(manifest);
  };
  const distDirectory = path.join(desktopRoot, "dist");
  const bindingPath = bindingPathFor(desktopRoot);
  const targetDirectory = appTargetDirectory(tauriDir);
  const binding = parseBinding(existsSync(bindingPath) ? readFileSync(bindingPath, "utf8") : null);
  // The compile reads known before cargo runs: the record's, and those of the
  // last build's dep-info in the app's own target directory.
  const lastDepInfo = path.join(targetDirectory, "debug", `${binName}.d`);
  let lastReads = [];
  try {
    if (existsSync(lastDepInfo)) lastReads = compileReadsFromDepInfo(projectRoot, readFileSync(lastDepInfo, "utf8"), [targetDirectory, distDirectory]);
  } catch {
    lastReads = [];
  }
  const recordReads = binding.record?.app_sources.compile_reads ?? [];
  const knownReads = [...new Set([...recordReads, ...lastReads])].sort();
  const appHashFor = (digests) => hashDigests(digests, context).hash;
  const frontendHash = () => hashDigests(digestInputs(projectRoot, frontendInputs(projectRoot, desktopRoot, engineFolders))).hash;
  const appNow = appInputs({ projectRoot, tauriDir, compileReads: recordReads, readManifest });
  const appSourcesNow = appHashFor(digestInputs(projectRoot, appNow.all));

  let current = null;
  let distStale = [];
  if (skipBuild) {
    const engines = engineHashes();
    distStale = engineFolders
      .map((name) => {
        const copy = path.join(distDirectory, name);
        const folderPresent = existsSync(copy);
        return { name, ...decideEngine({ folderPresent, stampText: folderPresent ? readStampText(copy) : null, currentHash: engines[name] }) };
      })
      .filter((engine) => engine.action === "rebuild");
    current = { engines, frontend: frontendHash(), dist: treeDigest(distDirectory), app_sources: appSourcesNow, build, binary: fileIdentity(binding.record?.binary.path) };
  }
  const plan = planAppBuild({ skipBuild, staleEngines, distStale, binding, current });
  if (plan.log) log(plan.log);

  let frontend = current?.frontend;
  if (plan.frontend) {
    const beforeInputs = frontendInputs(projectRoot, desktopRoot, engineFolders);
    const beforeDigests = digestInputs(projectRoot, beforeInputs);
    runFrontend();
    const afterInputs = frontendInputs(projectRoot, desktopRoot, engineFolders);
    const afterDigests = digestInputs(projectRoot, afterInputs);
    const drift = inputDrift({ beforeStatic: beforeInputs, beforeDigests, after: { staticInputs: afterInputs, recorded: [], all: afterInputs }, afterDigests });
    if (drift.length > 0) throw new Error(`native smoke: the frontend inputs moved while it built (${drift.join("; ")}); nothing is bound or launched`);
    frontend = hashDigests(afterDigests).hash;
  }
  if (!plan.app) return { binary: binding.record.binary.path, identity: current.binary };

  // No record survives a failed or interrupted build.
  rmSync(bindingPath, { force: true });
  const engines = engineHashes();
  const env = { CARGO_TARGET_DIR: targetDirectory, TAURI_CONFIG: build.tauri_config };
  // Sources other than a valid record's: clean the closure's own packages, so
  // that cargo's mtime-based freshness cannot keep an old binary.
  let clean = binding.problem ?? (binding.record.app_sources.hash !== appSourcesNow ? "the app's Rust sources differ from the binding record" : null);
  let reads = knownReads;
  let drift = [];
  for (let attempt = 1; attempt <= MAX_STABLE_APP_BUILD_ATTEMPTS; attempt += 1) {
    const before = appInputs({ projectRoot, tauriDir, compileReads: reads, readManifest });
    if (clean !== null) {
      if (lstatSync(targetDirectory, { throwIfNoEntry: false })?.isSymbolicLink()) {
        throw new Error(`native smoke: ${targetDirectory} is a symbolic link; the smoke cleans only the app's own target directory`);
      }
      log(`native smoke: cleaning the app's own packages in ${path.relative(projectRoot, targetDirectory)} (${clean})`);
      runCargo(["cargo", "clean", "--offline", ...before.packages.flatMap((name) => ["-p", name])], { CARGO_TARGET_DIR: targetDirectory });
    }
    const beforeDigests = digestInputs(projectRoot, before.all);
    const distBefore = treeDigest(distDirectory);
    const binary = appExecutableFromCargoMessages(runCargo(build.command, env), path.join(tauriDir, "Cargo.toml"), binName);
    if (path.relative(targetDirectory, binary).startsWith("..") || path.isAbsolute(path.relative(targetDirectory, binary))) {
      throw new Error(`native smoke: cargo reported the app executable at ${binary}, outside ${targetDirectory}; nothing is bound or launched`);
    }
    if (treeDigest(distDirectory) !== distBefore) {
      throw new Error("native smoke: dist/ changed while cargo built the app, so what the binary embeds is unknown; nothing is bound or launched");
    }
    let afterReads;
    try {
      afterReads = compileReadsFromDepInfo(projectRoot, readFileSync(`${binary}.d`, "utf8"), [targetDirectory, distDirectory]);
    } catch (error) {
      throw new Error(`native smoke: could not read the app's compile reads from ${binary}.d: ${error.message}`);
    }
    const after = appInputs({ projectRoot, tauriDir, compileReads: afterReads, readManifest });
    const afterDigests = digestInputs(projectRoot, after.all);
    drift = inputDrift({ beforeStatic: before.staticInputs, beforeDigests, after, afterDigests });
    if (drift.length === 0) {
      const identity = fileIdentity(binary);
      if (identity === null) throw new Error(`native smoke: cargo reported the app executable ${binary}, which is not a file; nothing is bound or launched`);
      const app_sources = { hash: appHashFor(afterDigests), compile_reads: after.recorded };
      mkdirSync(path.dirname(bindingPath), { recursive: true });
      writeFileSync(bindingPath, `${JSON.stringify(bindingRecord({ engines, frontend, dist: distBefore, app_sources, build, binary: { path: binary, ...identity } }), null, 2)}\n`);
      log(`native smoke: app binding written: ${path.relative(projectRoot, bindingPath)} (binary ${binary}; app sources ${app_sources.hash}; dist ${distBefore})`);
      return { binary, identity };
    }
    log(`native smoke: app build not bound (${drift.join("; ")}); cargo builds it again`);
    reads = [...new Set([...reads, ...afterReads])];
    clean = "the last app build was not bound";
  }
  throw new Error(`native smoke: the app's inputs did not hold still across ${MAX_STABLE_APP_BUILD_ATTEMPTS} builds (${drift.join("; ")}); nothing is bound or launched`);
}
