#!/usr/bin/env node
// orbit_probe.mjs: the B-CANVAS lane's guidance probe for per-frame cost while the model is orbited.
// Additions of slice C2 (C2-PROBE): run --select, the hover subcommand (cost per frame and per pointer move with
// no button pressed), and the pairs subcommand (a simplified count of halo-and-ground pixel pairs).
//
// Guidance only. The lane uses this to see whether a slice that adds work per frame is worth keeping.
// It decides nothing about D-72: the qualification runs are ROOT's, by a separate runner and a separate
// instrument. This file imports nothing from that instrument and carries none of its tolerances, limits
// or oracles. It reads the two benchmark fixture files as data. It never builds and changes no product
// file. Interface, method and limits: README.md beside this file.

import { createHash } from "node:crypto";
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, rmSync, statSync, writeFileSync } from "node:fs";
import { createServer } from "node:http";
import { createRequire } from "node:module";
import os from "node:os";
import path from "node:path";
import { performance } from "node:perf_hooks";
import { setTimeout as sleep } from "node:timers/promises";
import { fileURLToPath, pathToFileURL } from "node:url";
import v8 from "node:v8";
import vm from "node:vm";
import zlib from "node:zlib";

const TOOL = "b-canvas-orbit-probe";
const FORMAT_VERSION = 1;
// The two reporting thresholds the lane's brief names. The tool reports the fraction of intervals over
// each one. It never judges a run against them.
const LIMIT_A_MS = 16.7;
const LIMIT_B_MS = 33.3;
const DEFAULT_PORT = 5185;
const RESERVED_PORTS = new Set([5174, 5175, 5176, 5177, 5183, 5184, 5186]);
const MOVE_PERIOD_MS = 4;
const TAIL_MS = 150;
const IDLE_MS = 700;
const IDLE_WINDOW = Object.freeze({ fromMs: 100, toMs: 600 });
const RECORDER_CAPACITY = 400000;
const MAX_LONG_INTERVALS = 1500;
const MAX_CONSOLE_ENTRIES = 20;
// A figure of eight (frequency ratio 1:2) that starts at the press point and closes every xPeriodMs.
const ORBIT_PATH = Object.freeze({
  shape: "figure-of-eight, x = ax*W*sin(2*pi*t/Tx), y = ay*H*sin(2*pi*t/Ty), offsets from the press point",
  xAmplitudeOfCanvasWidth: 0.28,
  yAmplitudeOfCanvasHeight: 0.12,
  xPeriodMs: 5000,
  yPeriodMs: 2500
});
// hover: a figure of eight about the canvas centre, no button pressed. It crosses the fitted model and stays out of the corners.
const HOVER_PATH = Object.freeze({
  shape: "figure-of-eight, x = ax*W*sin(2*pi*t/Tx), y = ay*H*sin(2*pi*t/Ty), offsets from the canvas centre, no button pressed",
  xAmplitudeOfCanvasWidth: 0.36,
  yAmplitudeOfCanvasHeight: 0.3,
  xPeriodMs: 5000,
  yPeriodMs: 2500
});
const POINTER_EVENT_CAPACITY = 200000;
// pairs: the values the lane's brief gives for its simplified count. Reporting rule of this guidance tool only.
const PAIRS_RULE = Object.freeze({ regionCssPx: 48, channelTolerance: 48, pairDistanceCssPx: 2, minimumContrast: 3, minimumPairs: 4 });
const TIMEOUTS = Object.freeze({ navigationMs: 60000, modelMs: 240000, geometryMs: 240000, controlMs: 30000, settleMs: 30000 });
const SETTLE = Object.freeze({ quietMs: 500, pollMs: 100, busyLimit: 0.2 });
const UNCAPPED_ARGS = Object.freeze(["--disable-frame-rate-limit", "--disable-gpu-vsync"]);
const PACING_CALIBRATION_MS = 400;
const UNCAPPED_CALIBRATION_P50_LIMIT_MS = 4;
const FIXTURE_ROUTE = /(?:\/assets\/invented_preview_model-[^/]+\.js|\/fixtures\/product_preview\/invented_preview_model\.json)(?:\?.*)?$/;
const GUIDANCE_NOTE = "Guidance for the B-CANVAS lane only. This output decides nothing about D-72; the qualification runs are ROOT's, by a separate runner and a separate instrument.";

const TOOL_DIR = path.dirname(fileURLToPath(import.meta.url));
const WORKING_ROOT = findUp(TOOL_DIR, (dir) => existsSync(path.join(dir, "software-workflow.json")));
const REPO_ROOT = WORKING_ROOT ? findUp(WORKING_ROOT, (dir) => existsSync(path.join(dir, ".git"))) : null;
const ACTIVE = { server: null, browser: null, launches: 0 };

class UsageError extends Error {}

function findUp(start, test) {
  let dir = start;
  for (;;) {
    if (test(dir)) return dir;
    const parent = path.dirname(dir);
    if (parent === dir) return null;
    dir = parent;
  }
}

// ---------------------------------------------------------------------------------------------------
// Arithmetic. frameStats also runs inside the page, so it refers to nothing outside itself.
// ---------------------------------------------------------------------------------------------------

/** Nearest-rank percentile of a list of numbers: the value at position ceil(p/100 * n) of the sorted list. */
function percentileNearestRank(values, p) {
  if (!values.length) return null;
  const sorted = Array.from(values).sort((a, b) => a - b);
  return sorted[Math.min(sorted.length - 1, Math.max(0, Math.ceil((p / 100) * sorted.length) - 1))];
}

/** Ordinary median: the middle value, or the mean of the two middle values. */
function median(values) {
  if (!values.length) return null;
  const sorted = Array.from(values).sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
}

function mean(values) {
  return values.length ? values.reduce((sum, value) => sum + value, 0) / values.length : null;
}

function range(values) {
  return values.length ? [Math.min(...values), Math.max(...values)] : null;
}

/**
 * Interval statistics over the frames whose timestamp lies in [startMs, endMs].
 * ts holds ascending requestAnimationFrame timestamps in ts[0..n). Percentiles are nearest-rank.
 * refreshMs, when given, is the idle frame period; an interval over 1.5 times it is a missed refresh.
 */
function frameStats(ts, n, startMs, endMs, limitAMs, limitBMs, maxLong, refreshMs) {
  let first = -1;
  let last = -1;
  for (let i = 0; i < n; i++) {
    const t = ts[i];
    if (t < startMs) continue;
    if (t > endMs) break;
    if (first < 0) first = i;
    last = i;
  }
  const frames = first < 0 ? 0 : last - first + 1;
  const count = Math.max(0, frames - 1);
  const round3 = (value) => Math.round(value * 1000) / 1000;
  const out = {
    frames, intervals: count, spanMs: null, meanMs: null, p50Ms: null, p95Ms: null, p99Ms: null, maxMs: null,
    fractionOver16_7Ms: null, fractionOver33_3Ms: null, fractionMissedRefresh: null,
    longIntervalLimitMs: limitBMs, longIntervals: [], longIntervalsTruncated: false
  };
  if (count === 0) return out;
  const intervals = new Float64Array(count);
  let sum = 0;
  let overA = 0;
  let overB = 0;
  let missed = 0;
  let max = 0;
  for (let i = 0; i < count; i++) {
    const d = ts[first + i + 1] - ts[first + i];
    intervals[i] = d;
    sum += d;
    if (d > max) max = d;
    if (d > limitAMs) overA += 1;
    if (d > limitBMs) {
      overB += 1;
      if (maxLong > 0) {
        if (out.longIntervals.length < maxLong) out.longIntervals.push([Math.round((ts[first + i + 1] - startMs) * 10) / 10, Math.round(d * 10) / 10]);
        else out.longIntervalsTruncated = true;
      }
    }
    if (refreshMs && d > 1.5 * refreshMs) missed += 1;
  }
  const sorted = Float64Array.from(intervals).sort();
  const rank = (p) => sorted[Math.min(count - 1, Math.max(0, Math.ceil((p / 100) * count) - 1))];
  out.spanMs = round3(ts[last] - ts[first]);
  out.meanMs = round3(sum / count);
  out.p50Ms = round3(rank(50));
  out.p95Ms = round3(rank(95));
  out.p99Ms = round3(rank(99));
  out.maxMs = round3(max);
  out.fractionOver16_7Ms = round3(overA / count);
  out.fractionOver33_3Ms = round3(overB / count);
  out.fractionMissedRefresh = refreshMs ? round3(missed / count) : null;
  return out;
}

function pathOffset(elapsedMs, canvas) {
  return {
    x: ORBIT_PATH.xAmplitudeOfCanvasWidth * canvas.width * Math.sin((2 * Math.PI * elapsedMs) / ORBIT_PATH.xPeriodMs),
    y: ORBIT_PATH.yAmplitudeOfCanvasHeight * canvas.height * Math.sin((2 * Math.PI * elapsedMs) / ORBIT_PATH.yPeriodMs)
  };
}

function hoverOffset(elapsedMs, canvas) {
  return {
    x: HOVER_PATH.xAmplitudeOfCanvasWidth * canvas.width * Math.sin((2 * Math.PI * elapsedMs) / HOVER_PATH.xPeriodMs),
    y: HOVER_PATH.yAmplitudeOfCanvasHeight * canvas.height * Math.sin((2 * Math.PI * elapsedMs) / HOVER_PATH.yPeriodMs)
  };
}

function round(value, digits = 3) {
  if (value === null || value === undefined || !Number.isFinite(value)) return null;
  const scale = 10 ** digits;
  return Math.round(value * scale) / scale;
}

// ---------------------------------------------------------------------------------------------------
// Paths: nothing the tool writes or prints may carry a machine path.
// ---------------------------------------------------------------------------------------------------

function isInside(root, target) {
  const relative = path.relative(root, target);
  return relative === "" || (!relative.startsWith("..") && !path.isAbsolute(relative));
}

function reducePath(value) {
  const absolute = path.resolve(value);
  const posix = (relative) => relative.split(path.sep).join("/");
  if (WORKING_ROOT && isInside(WORKING_ROOT, absolute)) return `{WORKING_ROOT}/${posix(path.relative(WORKING_ROOT, absolute))}`;
  if (REPO_ROOT && isInside(REPO_ROOT, absolute)) return `{REPO_ROOT}/${posix(path.relative(REPO_ROOT, absolute))}`;
  return `{OUTSIDE_REPOSITORY}/${path.basename(absolute)}`;
}

function buildScrubber(extraRoots = []) {
  const roots = [];
  const add = (label, value) => {
    if (!value) return;
    const variants = new Set([value]);
    try { variants.add(realpathSync(value)); } catch { /* the path may not exist */ }
    for (const variant of variants) if (variant && variant !== path.sep) roots.push([variant, label]);
  };
  for (const [label, value] of extraRoots) add(label, value);
  add("{WORKING_ROOT}", WORKING_ROOT);
  add("{REPO_ROOT}", REPO_ROOT);
  add("{HOME}", os.homedir());
  add("{TMP}", os.tmpdir());
  roots.sort((a, b) => b[0].length - a[0].length);
  const generic = /(?:\/(?:Users|home|private|var\/folders|tmp)\/|[A-Za-z]:\\)[^\s"'`)\]}>,;]*/g;
  return (text) => {
    let out = String(text);
    for (const [root, label] of roots) out = out.split(root).join(label);
    return out.replace(generic, "{SCRUBBED_PATH}");
  };
}

function scrubDeep(value, scrub) {
  if (typeof value === "string") return scrub(value);
  if (Array.isArray(value)) return value.map((entry) => scrubDeep(entry, scrub));
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, entry]) => [key, scrubDeep(entry, scrub)]));
  return value;
}

function writeOutput(outPath, document, scrub) {
  const text = `${JSON.stringify(scrubDeep(document, scrub), null, 2)}\n`;
  mkdirSync(path.dirname(path.resolve(outPath)), { recursive: true });
  writeFileSync(outPath, text);
  return text;
}

// ---------------------------------------------------------------------------------------------------
// Arguments.
// ---------------------------------------------------------------------------------------------------

const oneOf = (...allowed) => (value, name) => {
  if (!allowed.includes(value)) throw new UsageError(`--${name} must be one of: ${allowed.join(", ")}`);
  return value;
};
const integer = (min, max) => (value, name) => {
  const parsed = Number(value);
  if (!Number.isInteger(parsed) || parsed < min || parsed > max) throw new UsageError(`--${name} must be a whole number from ${min} to ${max}`);
  return parsed;
};
const parsePort = (value, name) => {
  const port = integer(1024, 65535)(value, name);
  if (RESERVED_PORTS.has(port)) throw new UsageError(`--${name} ${port} belongs to another lane or configuration on this host; use ${DEFAULT_PORT}`);
  return port;
};
const parseWindow = (value, name) => {
  const match = /^(\d{3,5})x(\d{3,5})$/.exec(value);
  if (!match) throw new UsageError(`--${name} must look like 1440x900`);
  return { width: Number(match[1]), height: Number(match[2]) };
};
const parseQuery = (value) => String(value).replace(/^\?/, "");

/**
 * "<name>[@<distDir>]=<query>;...". The name runs to the first "@" or "=", whichever comes first; after "@" the
 * directory runs to the next "="; the rest is the query, which may be empty. A directory path that contains "="
 * or ";" is not supported: it is cut short here and then fails the index.html check with a message that says so.
 */
function parseVariants(value) {
  const variants = String(value).split(";").map((part) => part.trim()).filter((part) => part.length > 0).map((part) => {
    const at = part.indexOf("@");
    const equals = part.indexOf("=");
    let name;
    let dist = null;
    let query;
    if (at >= 0 && (equals < 0 || at < equals)) {
      name = part.slice(0, at);
      const rest = part.slice(at + 1);
      const restEquals = rest.indexOf("=");
      dist = restEquals < 0 ? rest : rest.slice(0, restEquals);
      query = restEquals < 0 ? "" : rest.slice(restEquals + 1);
      if (!dist) throw new UsageError(`variant "${name}": "@" must be followed by a build directory`);
    } else {
      name = equals < 0 ? part : part.slice(0, equals);
      query = equals < 0 ? "" : part.slice(equals + 1);
    }
    if (!/^[A-Za-z0-9_.-]+$/.test(name)) throw new UsageError(`variant name "${name}" must use letters, digits, dot, dash or underscore`);
    return { name, dist, query: parseQuery(query) };
  });
  if (!variants.length) throw new UsageError("--variants needs at least one name=query entry");
  if (new Set(variants.map((variant) => variant.name)).size !== variants.length) throw new UsageError("variant names must differ");
  return variants;
}

function interleave(variants, pairs) {
  const order = [];
  for (let pair = 1; pair <= pairs; pair++) for (const variant of variants) order.push({ pair, variant: variant.name, query: variant.query, dist: variant.dist ?? null });
  return order;
}

const RUN_SPEC = {
    dist: {}, pipes: { required: true, parse: (value, name) => Number(oneOf("1000", "10000")(value, name)) },
    mode: { required: true, parse: oneOf("schematic", "actual-od") }, labels: { required: true, parse: oneOf("on", "off") },
    theme: { required: true, parse: oneOf("light", "dark") }, dpr: { required: true, parse: (value, name) => Number(oneOf("1", "2")(value, name)) },
    window: { required: true, parse: parseWindow }, pacing: { required: true, parse: oneOf("vsync", "uncapped") },
    variants: { required: true, parse: parseVariants }, pairs: { required: true, parse: integer(1, 50) },
    "warmup-ms": { default: "2000", parse: integer(0, 600000) }, "measure-ms": { default: "10000", parse: integer(100, 600000) },
    select: { default: "none", parse: oneOf("none", "one", "hundred", "all") },
    port: { default: String(DEFAULT_PORT), parse: parsePort }, out: { required: true }
};
const SPECS = {
  run: RUN_SPEC,
  hover: RUN_SPEC,
  pairs: {
    dist: { required: true }, pipes: { required: true, parse: (value, name) => Number(oneOf("1000", "10000")(value, name)) },
    mode: { required: true, parse: oneOf("schematic", "actual-od") }, theme: { required: true, parse: oneOf("light", "dark") },
    dpr: { default: "1", parse: (value, name) => Number(oneOf("1", "2")(value, name)) }, window: { default: "1440x900", parse: parseWindow },
    query: { default: "", parse: parseQuery }, samples: { default: "1-200", parse: (value, name) => parseSampleRange(value, name) }, note: {},
    port: { default: String(DEFAULT_PORT), parse: parsePort }, out: { required: true }
  },
  resources: {
    dist: { required: true }, pipes: { required: true, parse: (value, name) => Number(oneOf("1000", "10000")(value, name)) },
    query: { default: "", parse: parseQuery }, dpr: { default: "2", parse: (value, name) => Number(oneOf("1", "2")(value, name)) },
    window: { default: "1440x900", parse: parseWindow }, port: { default: String(DEFAULT_PORT), parse: parsePort }, out: { required: true }
  },
  "self-test": { dist: { required: true }, port: { default: String(DEFAULT_PORT), parse: parsePort } },
  summarize: {}
};

function parseFlags(argv, spec) {
  const raw = {};
  const positionals = [];
  for (let i = 0; i < argv.length; i++) {
    const token = argv[i];
    if (!token.startsWith("--")) { positionals.push(token); continue; }
    let name = token.slice(2);
    let value;
    const at = name.indexOf("=");
    if (at >= 0) { value = name.slice(at + 1); name = name.slice(0, at); }
    if (!(name in spec)) throw new UsageError(`unknown option --${name}`);
    if (value === undefined) {
      value = argv[i + 1];
      if (value === undefined) throw new UsageError(`--${name} needs a value`);
      i += 1;
    }
    raw[name] = value;
  }
  const options = {};
  for (const [name, rule] of Object.entries(spec)) {
    const given = raw[name] ?? rule.default;
    if (given === undefined) {
      if (rule.required) throw new UsageError(`--${name} is required`);
      continue;
    }
    options[name] = rule.parse ? rule.parse(given, name) : given;
  }
  return { options, positionals };
}

function recordedArguments(subcommand, options) {
  const shown = { subcommand };
  for (const [name, value] of Object.entries(options)) {
    if (name === "dist" || name === "out") shown[name] = reducePath(value);
    else if (name === "window") shown[name] = `${value.width}x${value.height}`;
    else if (name === "samples") shown[name] = `${value.from}-${value.to}`;
    else if (name === "variants") shown[name] = value.map((variant) => `${variant.name}${variant.dist ? `@${reducePath(variant.dist)}` : ""}=${variant.query}`).join(";");
    else shown[name] = value;
  }
  const argv = [subcommand];
  for (const [name, value] of Object.entries(shown)) if (name !== "subcommand") argv.push(`--${name}`, String(value));
  return { ...shown, argv };
}

const USAGE = `orbit_probe.mjs: guidance probe for per-frame cost while orbiting (lane B-CANVAS). It decides nothing about D-72.

  node orbit_probe.mjs run --dist <dir> --pipes <1000|10000> --mode <schematic|actual-od>
       --labels <on|off> --theme <light|dark> --dpr <1|2> --window <WxH> --pacing <vsync|uncapped>
       --variants "<name>=<query>;<name>=<query>" --pairs <n>
       [--select <none|one|hundred|all>] [--warmup-ms 2000] [--measure-ms 10000] [--port 5185] --out <file.json>
     --select selects pipes through the model tree with real input before the orbit and fails the run when the
     diagnostics surface reports another count. A variant may name its own build: "<name>@<distDir>=<query>". --dist is then needed only by the
     variants that name none. A directory path that contains "=" or ";" is not supported.
  node orbit_probe.mjs hover <the options of run>
     No orbit: the pointer sweeps the canvas with no button pressed, along a closed path that depends on elapsed time alone.
  node orbit_probe.mjs pairs --dist <dir> --pipes <1000|10000> --mode <schematic|actual-od> --theme <light|dark>
       [--dpr 1] [--window 1440x900] [--query <query>] [--samples 1-200] [--note <text>] [--port 5185] --out <file.json>
     Picks each point sample with real input and counts halo-and-ground pixel pairs (a simplified rule; see README).
  node orbit_probe.mjs resources --dist <dir> --pipes <1000|10000> [--query <query>] [--dpr 2]
       [--window <WxH>] [--port 5185] --out <file.json>
  node orbit_probe.mjs summarize <file.json> [<file.json> ...]
  node orbit_probe.mjs self-test --dist <dir> [--port 5185]

Every browser and server on this host goes through the run's lock script: sh <run>/tools/with_e2e_lock.sh node orbit_probe.mjs ...
`;

// ---------------------------------------------------------------------------------------------------
// Inputs: the built product, the fixture, the browser library.
// ---------------------------------------------------------------------------------------------------

function requireWorkingRoot() {
  if (!WORKING_ROOT) throw new Error("cannot find the working root: no directory above this tool holds software-workflow.json");
  return WORKING_ROOT;
}

function resolveDist(value) {
  const dist = path.resolve(value);
  if (!existsSync(path.join(dist, "index.html"))) throw new UsageError("--dist must name a built dist directory that holds index.html");
  return dist;
}

/** Every variant's build directory and identity, settled before any server or browser starts. */
function resolveVariantBuilds(options) {
  return options.variants.map((variant) => {
    const given = variant.dist ?? options.dist;
    if (!given) throw new UsageError(`variant "${variant.name}" has no build: give --dist, or name one as ${variant.name}@<distDir>=<query>`);
    const dist = path.resolve(given);
    if (!existsSync(path.join(dist, "index.html"))) {
      throw new UsageError(`variant "${variant.name}": ${reducePath(dist)} is not a built dist directory that holds index.html (a directory path that contains "=" or ";" is not supported)`);
    }
    return { name: variant.name, query: variant.query, dist, ownBuild: Boolean(variant.dist), buildIdentity: buildIdentity(dist) };
  });
}

function sameBuildIdentity(a, b) {
  return a.indexHtmlSha256 === b.indexHtmlSha256 && JSON.stringify(a.assets) === JSON.stringify(b.assets);
}

function buildIdentity(dist) {
  const assetsDir = path.join(dist, "assets");
  const assets = [];
  const walk = (dir) => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = path.join(dir, entry.name);
      if (entry.isDirectory()) walk(full);
      else if (entry.isFile()) assets.push({ name: path.relative(assetsDir, full).split(path.sep).join("/"), bytes: statSync(full).size });
    }
  };
  if (existsSync(assetsDir)) walk(assetsDir);
  assets.sort((a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0));
  return {
    indexHtmlSha256: createHash("sha256").update(readFileSync(path.join(dist, "index.html"))).digest("hex"),
    assetCount: assets.length,
    assetBytes: assets.reduce((sum, asset) => sum + asset.bytes, 0),
    assets
  };
}

function loadFixture(pipes) {
  const relative = `apps/desktop/e2e/ui-foundation/fixtures/ui-foundation-${pipes}.model.json`;
  const file = path.join(requireWorkingRoot(), ...relative.split("/"));
  if (!existsSync(file)) throw new Error(`fixture file is absent: {WORKING_ROOT}/${relative}`);
  const bytes = readFileSync(file, "utf8");
  const model = JSON.parse(bytes);
  return {
    identity: {
      file: `{WORKING_ROOT}/${relative}`,
      sha256: createHash("sha256").update(bytes).digest("hex"),
      projectId: model.project.id,
      pipeSegments: model.pipe_segments.length,
      nodes: model.nodes.length
    },
    projectId: model.project.id,
    // The product loads its start-up model as a module with a default export; this is that module for the fixture.
    moduleBody: `const model=${bytes.trim()};\nexport { model as default };\n`
  };
}

function loadPlaywright() {
  const require = createRequire(path.join(requireWorkingRoot(), "package.json"));
  const library = require("playwright");
  let version = null;
  try { version = require("playwright/package.json").version; } catch {
    try { version = JSON.parse(readFileSync(path.join(path.dirname(require.resolve("playwright")), "package.json"), "utf8")).version; } catch { /* unknown */ }
  }
  return { chromium: library.chromium, version };
}

function hostIdentity(playwrightVersion) {
  const cpus = os.cpus();
  return { platform: process.platform, arch: process.arch, osRelease: os.release(), cpuModel: cpus[0]?.model ?? null, cpuCount: cpus.length, node: process.version, playwright: playwrightVersion };
}

// ---------------------------------------------------------------------------------------------------
// The static server: serves one dist directory on 127.0.0.1, never caches, never builds.
// ---------------------------------------------------------------------------------------------------

const CONTENT_TYPES = {
  ".html": "text/html; charset=utf-8", ".js": "text/javascript; charset=utf-8", ".mjs": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8", ".json": "application/json; charset=utf-8", ".map": "application/json; charset=utf-8",
  ".wasm": "application/wasm", ".svg": "image/svg+xml", ".png": "image/png", ".jpg": "image/jpeg", ".jpeg": "image/jpeg",
  ".gif": "image/gif", ".webp": "image/webp", ".ico": "image/x-icon", ".woff": "font/woff", ".woff2": "font/woff2",
  ".ttf": "font/ttf", ".txt": "text/plain; charset=utf-8", ".ts": "text/plain; charset=utf-8"
};

function startStaticServer(dist, port) {
  const root = path.resolve(dist);
  const served = { requests: 0, notFound: 0, indexFallbacks: 0 };
  const server = createServer((request, response) => {
    served.requests += 1;
    const send = (status, headers, body) => {
      if (status === 404) served.notFound += 1;
      response.writeHead(status, { "Cache-Control": "no-store", "X-Content-Type-Options": "nosniff", ...headers });
      response.end(request.method === "HEAD" ? undefined : body);
    };
    if (request.method !== "GET" && request.method !== "HEAD") return send(405, { "Content-Type": "text/plain; charset=utf-8" }, "method not allowed");
    let pathname;
    try { pathname = decodeURIComponent(new URL(request.url, "http://127.0.0.1").pathname); } catch { return send(400, { "Content-Type": "text/plain; charset=utf-8" }, "bad request"); }
    let file = path.resolve(root, `.${path.posix.normalize(`/${pathname}`)}`);
    if (!isInside(root, file)) return send(403, { "Content-Type": "text/plain; charset=utf-8" }, "forbidden");
    let found = existsSync(file) && statSync(file).isFile();
    if (!found) {
      // Single-page fallback: a path with no file extension gets the application's index.
      if (path.extname(pathname) !== "") return send(404, { "Content-Type": "text/plain; charset=utf-8" }, "not found");
      file = path.join(root, "index.html");
      found = true;
      served.indexFallbacks += 1;
    }
    const body = readFileSync(file);
    return send(200, { "Content-Type": CONTENT_TYPES[path.extname(file).toLowerCase()] ?? "application/octet-stream", "Content-Length": String(body.length) }, body);
  });
  return new Promise((resolve, reject) => {
    server.once("error", (error) => reject(error.code === "EADDRINUSE" ? new Error(`port ${port} on 127.0.0.1 is in use; nothing was started`) : error));
    server.listen(port, "127.0.0.1", () => {
      const handle = {
        origin: `http://127.0.0.1:${port}`,
        served,
        close: () => new Promise((done) => { server.close(() => done()); server.closeAllConnections?.(); })
      };
      ACTIVE.server = handle;
      resolve(handle);
    });
  });
}

// ---------------------------------------------------------------------------------------------------
// The browser.
// ---------------------------------------------------------------------------------------------------

async function launchBrowser(chromium, pacing) {
  const args = pacing === "uncapped" ? [...UNCAPPED_ARGS] : [];
  const configured = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH;
  const attempts = configured && existsSync(configured)
    ? [{ source: "executable named by PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH", options: { executablePath: configured } }]
    : [{ source: "playwright channel chrome", options: { channel: "chrome" } }, { source: "playwright bundled chromium", options: {} }];
  const failures = [];
  for (const attempt of attempts) {
    try {
      ACTIVE.launches += 1;
      const browser = await chromium.launch({ headless: true, args, ...attempt.options });
      ACTIVE.browser = browser;
      return { browser, source: attempt.source, addedArguments: args };
    } catch (error) {
      failures.push(`${attempt.source}: ${String(error?.message ?? error).split("\n")[0]}`);
    }
  }
  throw new Error(`no browser could be launched (${failures.join("; ")})`);
}

async function closeBrowser(browser) {
  try { await browser?.close(); } catch { /* already gone */ }
  if (ACTIVE.browser === browser) ACTIVE.browser = null;
}

// Runs in a blank page of the same browser, so the product's own context is never touched.
function inPageWebglIdentity() {
  const canvas = document.createElement("canvas");
  const gl = canvas.getContext("webgl2") ?? canvas.getContext("webgl");
  if (!gl) return { status: "unavailable" };
  const extension = gl.getExtension("WEBGL_debug_renderer_info");
  const identity = {
    status: "available",
    version: gl.getParameter(gl.VERSION),
    vendor: extension ? gl.getParameter(extension.UNMASKED_VENDOR_WEBGL) : gl.getParameter(gl.VENDOR),
    renderer: extension ? gl.getParameter(extension.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER)
  };
  gl.getExtension("WEBGL_lose_context")?.loseContext();
  return identity;
}

// Also in the blank page: a small square is moved every frame, so every frame has something to present, and the
// requestAnimationFrame interval is timed. Under the display's pacing the p50 is the refresh period; with the frame
// rate limit off it is far below it. An idle page cannot show this: with nothing to present it is paced either way.
function inPagePacingCalibration(durationMs) {
  return new Promise((resolve) => {
    const square = document.createElement("div");
    square.style.cssText = "position:fixed;left:0;top:0;width:8px;height:8px;background:#000;will-change:transform";
    document.body.appendChild(square);
    const ts = new Float64Array(20000);
    let n = 0;
    const started = performance.now();
    const tick = (time) => {
      ts[n++] = time;
      square.style.transform = `translateX(${n % 64}px)`;
      if (performance.now() - started < durationMs && n < ts.length) { requestAnimationFrame(tick); return; }
      const intervals = [];
      for (let i = Math.min(5, n - 1); i < n - 1; i++) intervals.push(ts[i + 1] - ts[i]);
      intervals.sort((a, b) => a - b);
      square.remove();
      resolve({ intervals: intervals.length, p50Ms: intervals.length ? Math.round(intervals[Math.max(0, Math.ceil(intervals.length / 2) - 1)] * 1000) / 1000 : null });
    };
    requestAnimationFrame(tick);
  });
}

async function probeBrowser(context) {
  const page = await context.newPage();
  try {
    const webgl = await page.evaluate(inPageWebglIdentity);
    const pacingCalibration = await page.evaluate(inPagePacingCalibration, PACING_CALIBRATION_MS);
    return { webgl, pacingCalibration };
  } finally { await page.close(); }
}

function attachConsole(page) {
  const log = { errorCount: 0, warningCount: 0, pageErrorCount: 0, errors: [], warnings: [], pageErrors: [] };
  const keep = (list, text) => { if (list.length < MAX_CONSOLE_ENTRIES) list.push(String(text).slice(0, 400)); };
  page.on("console", (message) => {
    if (message.type() === "error") { log.errorCount += 1; keep(log.errors, message.text()); }
    else if (message.type() === "warning") { log.warningCount += 1; keep(log.warnings, message.text()); }
  });
  page.on("pageerror", (error) => { log.pageErrorCount += 1; keep(log.pageErrors, error?.message ?? error); });
  return log;
}

// ---------------------------------------------------------------------------------------------------
// Code that runs inside the product page. Each function is self-contained.
// ---------------------------------------------------------------------------------------------------

function inPageReadDiagnostics() {
  const surface = globalThis.__openPipeStressUiDiagnosticsV1;
  if (!surface || typeof surface.readCurrent !== "function") return null;
  const snapshot = surface.readCurrent();
  const viewport = snapshot.viewport && snapshot.viewport.status !== "unavailable" ? snapshot.viewport : null;
  const plain = (value) => (value === undefined ? null : JSON.parse(JSON.stringify(value)));
  return {
    pageNowMs: performance.now(),
    schema: snapshot.schema,
    model: {
      projectId: snapshot.model.projectId, generation: snapshot.model.generation, indexGeneration: snapshot.model.indexGeneration,
      identityHash: snapshot.model.identityHash, assignmentStatus: snapshot.model.assignment?.status ?? null
    },
    viewport: viewport ? {
      canvas: { cssWidth: viewport.canvas.cssWidth, cssHeight: viewport.canvas.cssHeight, bufferWidth: viewport.canvas.bufferWidth, bufferHeight: viewport.canvas.bufferHeight, pixelRatio: viewport.canvas.dpr },
      cameraSequence: viewport.camera.sequence,
      submissionSequence: viewport.mainRender.submissionSequence,
      // The count and the primary only: a selection of every pipe is never written out ref by ref.
      selection: viewport.selection ? { count: viewport.selection.orderedRefs.length, primaryRef: plain(viewport.selection.primaryRef), inputKind: viewport.selection.inputKind } : null,
      labels: plain(viewport.labels),
      geometry: plain(viewport.geometry),
      rendererInfo: plain(viewport.resources.rendererInfo),
      ownedPendingRafCount: viewport.resources.ownedPendingRafCount,
      owned: plain(viewport.resources.owned),
      context: plain(viewport.resources.context)
    } : null
  };
}

function inPageSettleState() {
  const snapshot = globalThis.__openPipeStressUiDiagnosticsV1?.readCurrent?.();
  const viewport = snapshot && snapshot.viewport && snapshot.viewport.status !== "unavailable" ? snapshot.viewport : null;
  return viewport ? { pendingRaf: viewport.resources.ownedPendingRafCount, submissionSequence: viewport.mainRender.submissionSequence, cameraSequence: viewport.camera.sequence } : null;
}

function inPageModelReady(projectId) {
  const snapshot = globalThis.__openPipeStressUiDiagnosticsV1?.readCurrent?.();
  return Boolean(snapshot && snapshot.model.projectId === projectId && snapshot.model.assignment?.status === "committed" &&
    snapshot.viewport && snapshot.viewport.status !== "unavailable" && snapshot.viewport.mainRender.submissionSequence > 0);
}

function inPageGeometryReady(mode) {
  const geometry = globalThis.__openPipeStressUiDiagnosticsV1?.readCurrent?.()?.viewport?.geometry;
  return Boolean(geometry && geometry.mode === mode && (mode === "schematic" || (geometry.odStatus === "available" && geometry.odGeneration > 0)));
}

function inPageCanvasGeometry() {
  const hosts = document.querySelectorAll('[data-testid="viewport-canvas"]');
  if (hosts.length !== 1) throw new Error(`expected one viewport host, found ${hosts.length}`);
  const canvases = hosts[0].querySelectorAll("canvas");
  if (canvases.length !== 1) throw new Error(`expected one canvas in the viewport host, found ${canvases.length}`);
  const rect = canvases[0].getBoundingClientRect();
  return {
    canvasRect: { left: rect.left, top: rect.top, width: rect.width, height: rect.height },
    page: { innerWidth, innerHeight, devicePixelRatio, visibilityState: document.visibilityState }
  };
}

/** True when a pointer event at the point would go to the viewport's canvas itself and not to a label or an overlay. */
function inPageCanvasReceives(point) {
  const canvas = document.querySelector('[data-testid="viewport-canvas"] canvas');
  return Boolean(canvas) && document.elementFromPoint(point.x, point.y) === canvas;
}

function inPageCollect(request) {
  const recorder = globalThis.__bCanvasOrbitProbe;
  if (!recorder) return { error: "the frame recorder is absent" };
  const endDiagnostics = recorder.readDiagnostics();
  recorder.running = false;
  const n = recorder.n;
  const idle = recorder.frameStats(recorder.ts, n, recorder.startedAt + request.idleFromMs, recorder.startedAt + request.idleToMs, request.limitAMs, request.limitBMs, 0, null);
  // A missed refresh means something only under the display's pacing, where the idle interval is the refresh period.
  const refreshMs = request.paced ? idle.p50Ms : null;
  const warmup = recorder.frameStats(recorder.ts, n, request.pageT0, request.pageT0 + request.warmupMs, request.limitAMs, request.limitBMs, 0, refreshMs);
  const measure = recorder.frameStats(recorder.ts, n, request.pageT0 + request.warmupMs, request.pageT0 + request.warmupMs + request.measureMs, request.limitAMs, request.limitBMs, request.maxLong, refreshMs);
  let framesInSpan = 0;
  const spanToMs = endDiagnostics ? endDiagnostics.pageNowMs : performance.now();
  for (let i = 0; i < n; i++) if (recorder.ts[i] >= request.spanFromMs && recorder.ts[i] <= spanToMs) framesInSpan += 1;
  return { endDiagnostics, idle, warmup, measure, framesInSpan, recorded: n, capacity: recorder.ts.length, overflow: recorder.overflow };
}

function inPageWatchSample() {
  const watch = globalThis.__bCanvasOrbitProbeWatch;
  if (!watch) return null;
  const snapshot = globalThis.__openPipeStressUiDiagnosticsV1?.readCurrent?.();
  const viewport = snapshot && snapshot.viewport && snapshot.viewport.status !== "unavailable" ? snapshot.viewport : null;
  return {
    pageNowMs: performance.now(), pendingPageRaf: watch.pending.size, requested: watch.requested, fired: watch.fired, cancelled: watch.cancelled,
    draws: watch.draws, clears: watch.clears,
    productOwnedPendingRaf: viewport ? viewport.resources.ownedPendingRafCount : null,
    productSubmissionSequence: viewport ? viewport.mainRender.submissionSequence : null,
    cameraSequence: viewport ? viewport.camera.sequence : null
  };
}

// Started by an expression, not an init script: the measured page carries nothing of the tool until the viewport has settled.
// One function object is rescheduled every frame and timestamps go into one preallocated array, so a frame allocates nothing here.
const RECORDER_SOURCE = `(() => {
  const capacity = ${RECORDER_CAPACITY};
  const recorder = { ts: new Float64Array(capacity), n: 0, running: true, overflow: false, startedAt: performance.now(),
    frameStats: ${frameStats.toString()},
    readDiagnostics: ${inPageReadDiagnostics.toString()} };
  const tick = (time) => {
    if (!recorder.running) return;
    if (recorder.n < capacity) recorder.ts[recorder.n++] = time; else recorder.overflow = true;
    requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
  Object.defineProperty(globalThis, "__bCanvasOrbitProbe", { value: recorder, configurable: true });
  return recorder.startedAt;
})()`;

// Used by "resources" only, where nothing is timed. Installed before any product script runs.
const WATCH_INIT_SOURCE = `(() => {
  if (globalThis.__bCanvasOrbitProbeWatch) return;
  const watch = { pending: new Set(), requested: 0, fired: 0, cancelled: 0, draws: 0, clears: 0 };
  const nativeRequest = window.requestAnimationFrame.bind(window);
  const nativeCancel = window.cancelAnimationFrame.bind(window);
  window.requestAnimationFrame = function (callback) {
    let id = 0;
    id = nativeRequest(function (time) { watch.pending.delete(id); watch.fired += 1; return callback(time); });
    watch.pending.add(id);
    watch.requested += 1;
    return id;
  };
  window.cancelAnimationFrame = function (id) { if (watch.pending.delete(id)) watch.cancelled += 1; return nativeCancel(id); };
  const drawNames = ["drawArrays", "drawElements", "drawArraysInstanced", "drawElementsInstanced", "drawRangeElements"];
  for (const constructor of [globalThis.WebGL2RenderingContext, globalThis.WebGLRenderingContext]) {
    if (!constructor) continue;
    for (const name of drawNames) {
      const native = constructor.prototype[name];
      if (typeof native !== "function") continue;
      constructor.prototype[name] = function (...args) { watch.draws += 1; return native.apply(this, args); };
    }
    const nativeClear = constructor.prototype.clear;
    if (typeof nativeClear === "function") constructor.prototype.clear = function (...args) { watch.clears += 1; return nativeClear.apply(this, args); };
  }
  Object.defineProperty(globalThis, "__bCanvasOrbitProbeWatch", { value: watch });
})();`;

// ---------------------------------------------------------------------------------------------------
// Driving the product through its own controls.
// ---------------------------------------------------------------------------------------------------

// The pointer is driven from this process on a 4 ms timer. A large collection here in mid-orbit would hold moves back,
// so this process collects its garbage (the parsed fixture above all) just before each orbit.
let collectGarbage = null;
function collectOwnGarbage() {
  try {
    if (collectGarbage === null) {
      v8.setFlagsFromString("--expose-gc");
      collectGarbage = vm.runInNewContext("gc");
    }
    collectGarbage();
    return true;
  } catch { collectGarbage = () => {}; return false; }
}

function metricsMap(result) {
  const map = {};
  for (const metric of result.metrics) map[metric.name] = metric.value;
  return map;
}

async function openProduct(context, origin, query, fixture, options = {}) {
  if (options.initScript) await context.addInitScript(options.initScript);
  const page = await context.newPage();
  const consoleLog = attachConsole(page);
  // The product opens its start-up model by importing one module. Answer that one request with the fixture,
  // so the product's own load path opens it. Every other request is served from the dist directory unchanged.
  await page.route(FIXTURE_ROUTE, (route) => route.fulfill({ status: 200, contentType: "application/javascript", body: fixture.moduleBody }));
  const cdp = await context.newCDPSession(page);
  await cdp.send("Performance.enable");
  const started = performance.now();
  await page.goto(`${origin}/${query ? `?${query}` : ""}`, { waitUntil: "domcontentloaded", timeout: TIMEOUTS.navigationMs });
  await page.waitForFunction(inPageModelReady, fixture.projectId, { timeout: TIMEOUTS.modelMs, polling: 100 });
  return { page, cdp, consoleLog, modelReadyMs: round(performance.now() - started, 0) };
}

async function setTheme(page, theme) {
  await page.selectOption('select[aria-label="Appearance theme"]', theme, { timeout: TIMEOUTS.controlMs });
  await page.waitForFunction((expected) => document.querySelector('[data-testid="desktop-preview-shell"]')?.getAttribute("data-theme") === expected, theme, { timeout: TIMEOUTS.controlMs, polling: 50 });
}

async function setGeometry(page, mode) {
  await page.click(`[data-testid="${mode === "schematic" ? "viewport-geometry-schematic" : "viewport-geometry-actual-od"}"]`, { timeout: TIMEOUTS.controlMs });
  await page.waitForFunction(inPageGeometryReady, mode, { timeout: TIMEOUTS.geometryMs, polling: 50 });
}

async function readToggle(page, testId) {
  return (await page.getAttribute(`[data-testid="${testId}"]`, "aria-pressed", { timeout: TIMEOUTS.controlMs })) === "true";
}

async function setToggle(page, testId, enabled) {
  if ((await readToggle(page, testId)) !== enabled) await page.click(`[data-testid="${testId}"]`, { timeout: TIMEOUTS.controlMs });
  await page.waitForFunction(({ id, pressed }) => document.querySelector(`[data-testid="${id}"]`)?.getAttribute("aria-pressed") === pressed, { id: testId, pressed: String(enabled) }, { timeout: TIMEOUTS.controlMs, polling: 50 });
}

async function isometricThenFit(page) {
  await page.click('[data-testid="viewport-view-isometric"]', { timeout: TIMEOUTS.controlMs });
  await page.click('[data-testid="viewport-fit-model"]', { timeout: TIMEOUTS.controlMs });
}

/**
 * Settled, as the tool means it before and after an orbit: for quietMs on end the product reports no pending
 * animation frame of its own, its main-render submission count and camera sequence do not move, and the main
 * thread is mostly idle. Read from the product's diagnostics surface and the DevTools Performance domain.
 */
async function waitSettled(page, cdp, timeoutMs = TIMEOUTS.settleMs) {
  const started = performance.now();
  let previous = null;
  let quietSince = null;
  let polls = 0;
  let busy = null;
  for (;;) {
    const [state, metrics] = await Promise.all([page.evaluate(inPageSettleState), cdp.send("Performance.getMetrics")]);
    const now = performance.now();
    const map = metricsMap(metrics);
    polls += 1;
    let quiet = false;
    if (previous && state && previous.state && state.pendingRaf === 0 &&
        state.submissionSequence === previous.state.submissionSequence && state.cameraSequence === previous.state.cameraSequence) {
      busy = (map.TaskDuration - previous.map.TaskDuration) / Math.max(1e-6, map.Timestamp - previous.map.Timestamp);
      quiet = busy < SETTLE.busyLimit;
    }
    if (quiet) {
      if (quietSince === null) quietSince = previous.at;
      if (now - quietSince >= SETTLE.quietMs) return { settled: true, waitedMs: round(now - started, 0), polls, mainThreadBusyFraction: round(busy, 3) };
    } else quietSince = null;
    if (now - started > timeoutMs) return { settled: false, waitedMs: round(now - started, 0), polls, mainThreadBusyFraction: round(busy, 3), lastState: state };
    previous = { state, map, at: now };
    await sleep(SETTLE.pollMs);
  }
}

/**
 * Candidate press points, outside in: eight points on each of five rectangles inset from the canvas edge.
 * Open space is likeliest near the edge after Fit. A candidate is dropped when the orbit path from it would
 * leave the page.
 */
function pressCandidates(geometry) {
  const { canvasRect: rect, page } = geometry;
  const reachX = ORBIT_PATH.xAmplitudeOfCanvasWidth * rect.width + 2;
  const reachY = ORBIT_PATH.yAmplitudeOfCanvasHeight * rect.height + 2;
  const candidates = [];
  for (const inset of [0.08, 0.16, 0.24, 0.32, 0.4]) {
    const far = 1 - inset;
    for (const [fx, fy] of [[inset, inset], [far, far], [far, inset], [inset, far], [0.5, inset], [0.5, far], [inset, 0.5], [far, 0.5]]) {
      const x = Math.round(rect.left + fx * rect.width);
      const y = Math.round(rect.top + fy * rect.height);
      if (x - reachX < 0 || x + reachX > page.innerWidth || y - reachY < 0 || y + reachY > page.innerHeight) continue;
      candidates.push({ x, y, inset });
    }
  }
  candidates.push({ x: Math.round(rect.left + rect.width / 2), y: Math.round(rect.top + rect.height / 2), inset: 0.5 });
  return candidates;
}

/**
 * The product gives the hovered entity a label, and a label takes the pointer. So a press point must still be
 * received by the canvas after the pointer has rested on it. Leaves the pointer resting on the point it returns.
 */
async function findPressPoint(page, cdp) {
  const geometry = await page.evaluate(inPageCanvasGeometry);
  const candidates = pressCandidates(geometry);
  for (const [index, candidate] of candidates.entries()) {
    if (!(await page.evaluate(inPageCanvasReceives, candidate))) continue;
    await page.mouse.move(candidate.x, candidate.y);
    const settle = await waitSettled(page, cdp);
    if (await page.evaluate(inPageCanvasReceives, candidate)) return { ...candidate, attempt: index + 1, candidates: candidates.length, settle, ...geometry };
  }
  throw new Error(`none of ${candidates.length} candidate press points is received by the canvas once the pointer rests on it`);
}

/** hover: the sweep starts at the canvas centre. The pointer rests there until the viewport has settled. */
async function restAtCanvasCentre(page, cdp) {
  const geometry = await page.evaluate(inPageCanvasGeometry);
  const x = Math.round(geometry.canvasRect.left + geometry.canvasRect.width / 2);
  const y = Math.round(geometry.canvasRect.top + geometry.canvasRect.height / 2);
  await page.mouse.move(x, y);
  const settle = await waitSettled(page, cdp);
  return { x, y, settle, receivedByCanvas: await page.evaluate(inPageCanvasReceives, { x, y }), ...geometry };
}

/**
 * Real pointer input: the button is already down at the press point. Moves are sent on a timer and never
 * awaited one by one, so a slow frame cannot slow the pointer. Position depends on elapsed time alone.
 */
async function driveOrbit(page, press, canvas, totalMs, marks = [], offsetOf = pathOffset) {
  const pending = new Set();
  const stats = { movesSent: 0, movesAcknowledged: 0, movesFailed: 0, meanSendGapMs: null, maxSendGapMs: 0, sendGapsOver8Ms: 0,
    sendGapCounts: { upTo5Ms: 0, upTo8Ms: 0, upTo12Ms: 0, upTo16Ms: 0, over16Ms: 0 }, maxInFlight: 0 };
  const due = marks.map((mark) => ({ ...mark, done: false }));
  let gapSum = 0;
  let lastSend = null;
  const started = performance.now();
  for (;;) {
    const now = performance.now();
    const elapsed = now - started;
    if (elapsed >= totalMs) break;
    for (const mark of due) if (!mark.done && elapsed >= mark.atMs) { mark.done = true; mark.run(elapsed); }
    const offset = offsetOf(elapsed, canvas);
    const move = page.mouse.move(press.x + offset.x, press.y + offset.y).then(() => { stats.movesAcknowledged += 1; }, () => { stats.movesFailed += 1; });
    const tracked = move.finally(() => pending.delete(tracked));
    pending.add(tracked);
    stats.movesSent += 1;
    if (pending.size > stats.maxInFlight) stats.maxInFlight = pending.size;
    if (lastSend !== null) {
      const gap = now - lastSend;
      gapSum += gap;
      if (gap > stats.maxSendGapMs) stats.maxSendGapMs = gap;
      if (gap > 8) stats.sendGapsOver8Ms += 1;
      stats.sendGapCounts[gap <= 5 ? "upTo5Ms" : gap <= 8 ? "upTo8Ms" : gap <= 12 ? "upTo12Ms" : gap <= 16 ? "upTo16Ms" : "over16Ms"] += 1;
    }
    lastSend = now;
    await sleep(MOVE_PERIOD_MS);
  }
  for (const mark of due) if (!mark.done) { mark.done = true; mark.run(performance.now() - started); }
  stats.meanSendGapMs = stats.movesSent > 1 ? round(gapSum / (stats.movesSent - 1), 3) : null;
  stats.maxSendGapMs = round(stats.maxSendGapMs, 3);
  return { stats, drain: () => Promise.allSettled([...pending]) };
}

// ---------------------------------------------------------------------------------------------------
// run
// ---------------------------------------------------------------------------------------------------

async function measureOneRun(chromium, origin, fixture, options, item, index, kind = "orbit") {
  const hover = kind === "hover";
  const record = { index, pair: item.pair, variant: item.variant, query: item.query, startedUtc: new Date().toISOString(), loadAverageBefore: os.loadavg().map((value) => round(value, 2)), error: null };
  let browser = null;
  try {
    const launched = await launchBrowser(chromium, options.pacing);
    browser = launched.browser;
    record.browser = { source: launched.source, version: browser.version(), addedArguments: launched.addedArguments };
    const context = await browser.newContext({ viewport: { width: options.window.width, height: options.window.height }, deviceScaleFactor: options.dpr });
    const probed = await probeBrowser(context);
    record.browser.webgl = probed.webgl;
    const { page, cdp, consoleLog, modelReadyMs } = await openProduct(context, origin, item.query, fixture);
    record.console = consoleLog;
    record.modelReadyMs = modelReadyMs;

    await setTheme(page, options.theme);
    await setGeometry(page, options.mode);
    await setToggle(page, "toggle-viewport-labels", options.labels === "on");
    await isometricThenFit(page);
    await waitSettled(page, cdp);
    // The selection comes before the press-point search, so that the search sees whatever the selection put on the canvas.
    record.selection = await selectPipesThroughTree(page, cdp, options.select ?? "none", fixture);
    const press = hover ? await restAtCanvasCentre(page, cdp) : await findPressPoint(page, cdp);
    const canvas = { width: press.canvasRect.width, height: press.canvasRect.height };
    record.page = { ...press.page, canvasCssWidth: round(canvas.width, 2), canvasCssHeight: round(canvas.height, 2), canvasCssArea: round(canvas.width * canvas.height, 0),
      ...(hover ? { sweepCentre: { x: press.x, y: press.y, receivedByCanvasAtRest: press.receivedByCanvas } } : { pressPoint: { x: press.x, y: press.y, attempt: press.attempt, candidates: press.candidates } }) };
    record.settleBeforeOrbit = press.settle;

    collectOwnGarbage();
    await page.evaluate(RECORDER_SOURCE);
    await sleep(IDLE_MS);
    if (hover) await page.evaluate(POINTER_RECORDER_SOURCE);
    const before = await page.evaluate(inPageReadDiagnostics);
    if (!hover) await page.mouse.down({ button: "left" });
    const pageT0 = await page.evaluate(() => performance.now());
    const samples = {};
    const totalMs = options["warmup-ms"] + options["measure-ms"];
    const sample = (name) => (elapsed) => { samples[name] = { elapsedMs: elapsed, result: cdp.send("Performance.getMetrics").then(metricsMap, () => null) }; };
    const orbit = await driveOrbit(page, press, canvas, totalMs + TAIL_MS, [{ atMs: options["warmup-ms"], run: sample("start") }, { atMs: totalMs, run: sample("end") }], hover ? hoverOffset : pathOffset);
    const pointerEvents = hover ? await page.evaluate(inPageCollectPointer, { fromMs: pageT0 + options["warmup-ms"], toMs: pageT0 + totalMs }) : null;
    const collected = await page.evaluate(inPageCollect, {
      idleFromMs: IDLE_WINDOW.fromMs, idleToMs: IDLE_WINDOW.toMs, pageT0, warmupMs: options["warmup-ms"], measureMs: options["measure-ms"],
      limitAMs: LIMIT_A_MS, limitBMs: LIMIT_B_MS, maxLong: MAX_LONG_INTERVALS, spanFromMs: before ? before.pageNowMs : pageT0,
      paced: options.pacing === "vsync"
    });
    await orbit.drain();
    if (!hover) await page.mouse.up({ button: "left" });
    if (collected.error) throw new Error(collected.error);
    record.settleAfterOrbit = await waitSettled(page, cdp);
    const after = await page.evaluate(inPageReadDiagnostics);

    const calibrationP50 = probed.pacingCalibration.p50Ms;
    record.pacing = {
      requested: options.pacing,
      calibrationIntervalP50Ms: calibrationP50,
      calibrationIntervals: probed.pacingCalibration.intervals,
      idleProductPageIntervalP50Ms: collected.idle.p50Ms,
      uncappedTookEffect: options.pacing === "uncapped" ? (calibrationP50 !== null && calibrationP50 < UNCAPPED_CALIBRATION_P50_LIMIT_MS) : null,
      rule: `a blank page of the same browser presents a moving square every frame for ${PACING_CALIBRATION_MS} ms before the product opens; uncapped is reported as having taken effect when that interval p50 is under ${UNCAPPED_CALIBRATION_P50_LIMIT_MS} ms`
    };
    record.frames = { source: "requestAnimationFrame timestamps recorded in the page; percentiles are nearest-rank", warmup: collected.warmup, measure: collected.measure, recorded: collected.recorded, capacity: collected.capacity, overflow: collected.overflow };
    const startMetrics = await samples.start?.result;
    const endMetrics = await samples.end?.result;
    record.mainThread = mainThreadCost(startMetrics, endMetrics, collected.measure.meanMs, samples, pointerEvents ? pointerEvents.eventsInWindow : null);
    record.pointer = { ...orbit.stats, movePeriodMs: MOVE_PERIOD_MS };
    const end = collected.endDiagnostics;
    record[hover ? "hover" : "orbit"] = {
      cameraSequenceDelta: before?.viewport && end?.viewport ? end.viewport.cameraSequence - before.viewport.cameraSequence : null,
      productSubmissionsInSpan: before?.viewport && end?.viewport ? end.viewport.submissionSequence - before.viewport.submissionSequence : null,
      framesInSpan: collected.framesInSpan,
      modelGenerationUnchanged: Boolean(before && after && before.model.generation === after.model.generation),
      geometryUnchanged: Boolean(before?.viewport && after?.viewport && JSON.stringify(before.viewport.geometry) === JSON.stringify(after.viewport.geometry)),
      selectionUnchanged: Boolean(before?.viewport && after?.viewport && JSON.stringify(before.viewport.selection) === JSON.stringify(after.viewport.selection))
    };
    if (hover) {
      record.hover.pointerEvents = pointerEvents;
      record.hover.hoveredEntities = { status: "not exposed", note: "the diagnostics surface carries no hovered entity, so distinct hovered entities cannot be counted; product main-render submissions in the span are recorded instead" };
      if (options.labels === "off" && pointerEvents && pointerEvents.fractionReceivedByCanvas !== null && pointerEvents.fractionReceivedByCanvas < 0.9) throw new Error(`hover: with labels off only ${pointerEvents.fractionReceivedByCanvas} of the pointer moves in the window reached the canvas; the sweep did not measure the canvas`);
    }
    record.diagnostics = { beforeOrbit: stripClock(before), endOfWindow: stripClock(end), afterOrbit: stripClock(after) };
  } catch (error) {
    record.error = String(error?.message ?? error).split("\n").slice(0, 3).join(" | ");
  } finally {
    await closeBrowser(browser);
    record.loadAverageAfter = os.loadavg().map((value) => round(value, 2));
    record.endedUtc = new Date().toISOString();
  }
  return record;
}

function stripClock(diagnostics) {
  if (!diagnostics) return null;
  const { pageNowMs: _pageNowMs, ...rest } = diagnostics;
  return rest;
}

function mainThreadCost(start, end, meanIntervalMs, samples, pointerEventsInWindow = null) {
  if (!start || !end) return { status: "unavailable" };
  const windowMs = (end.Timestamp - start.Timestamp) * 1000;
  const ms = (name) => (end[name] - start[name]) * 1000;
  const frames = meanIntervalMs ? windowMs / meanIntervalMs : null;
  const perFrame = (value) => (frames ? round(value / frames, 3) : null);
  const task = ms("TaskDuration");
  const script = ms("ScriptDuration");
  const layout = ms("LayoutDuration");
  const style = ms("RecalcStyleDuration");
  return {
    status: "measured",
    source: "DevTools Performance.getMetrics deltas between the start and the end of the measurement window",
    windowMs: round(windowMs, 1),
    sampledAtElapsedMs: [round(samples.start.elapsedMs, 1), round(samples.end.elapsedMs, 1)],
    framesInWindowByMeanInterval: round(frames, 1),
    taskMs: round(task, 1), scriptMs: round(script, 1), layoutMs: round(layout, 1), styleMs: round(style, 1),
    taskMsPerFrame: perFrame(task), scriptMsPerFrame: perFrame(script), layoutMsPerFrame: perFrame(layout), styleMsPerFrame: perFrame(style),
    busyFraction: round(task / windowMs, 3),
    layoutCount: end.LayoutCount - start.LayoutCount, styleRecalcCount: end.RecalcStyleCount - start.RecalcStyleCount,
    jsHeapUsedBytes: [start.JSHeapUsedSize, end.JSHeapUsedSize],
    // hover only: the same totals over the pointermove events the page dispatched in the window (the browser may merge
    // moves that arrive within one frame, so this is per dispatched event and not per move the tool sent).
    ...(pointerEventsInWindow === null ? {} : {
      pointerEventsInWindow,
      taskMsPerPointerEvent: pointerEventsInWindow ? round(task / pointerEventsInWindow, 3) : null,
      scriptMsPerPointerEvent: pointerEventsInWindow ? round(script / pointerEventsInWindow, 3) : null
    })
  };
}

function oneLine(run) {
  if (run.error) return `run ${run.index + 1} ${run.variant}: FAILED: ${run.error}`;
  const m = run.frames.measure;
  const extra = `${run.selection && run.selection.requested !== "none" ? `; selected ${run.selection.reported.count}` : ""}${run.hover ? `; ${run.hover.pointerEvents?.eventsInWindow ?? "n/a"} pointer events, ${run.mainThread.taskMsPerPointerEvent ?? "n/a"} ms each, ${run.hover.pointerEvents?.fractionReceivedByCanvas ?? "n/a"} to the canvas` : ""}`;
  return `run ${run.index + 1} ${run.variant}: intervals ${m.intervals}, mean ${m.meanMs} ms, p50 ${m.p50Ms}, p95 ${m.p95Ms}, max ${m.maxMs}; main thread ${run.mainThread.taskMsPerFrame ?? "n/a"} ms/frame${extra}; load ${run.loadAverageBefore[0]} -> ${run.loadAverageAfter[0]}`;
}

async function runSeries(options, log = console.log, kind = "orbit") {
  const hover = kind === "hover";
  const builds = resolveVariantBuilds(options);
  const scrub = buildScrubber(builds.map((build) => ["{DIST}", build.dist]));
  const fixture = loadFixture(options.pipes);
  const playwright = loadPlaywright();
  const order = interleave(options.variants, options.pairs);
  const shared = builds.every((build) => sameBuildIdentity(build.buildIdentity, builds[0].buildIdentity));
  const document = {
    tool: TOOL, formatVersion: FORMAT_VERSION, kind: hover ? "hover-series" : "run-series", note: GUIDANCE_NOTE, createdUtc: new Date().toISOString(),
    arguments: recordedArguments(hover ? "hover" : "run", options),
    // One identity when every variant was served the same build; otherwise each variant's own, below.
    buildIdentity: shared ? builds[0].buildIdentity : null,
    variantsShareOneBuildIdentity: shared,
    variants: builds.map((build) => ({ name: build.name, query: build.query, dist: reducePath(build.dist), namedItsOwnBuild: build.ownBuild, buildIdentity: build.buildIdentity })),
    fixture: fixture.identity,
    host: hostIdentity(playwright.version), [hover ? "hoverPath" : "orbitPath"]: { ...(hover ? HOVER_PATH : ORBIT_PATH), movePeriodMs: MOVE_PERIOD_MS, tailMs: TAIL_MS },
    order: order.map((item) => item.variant), runs: []
  };
  for (const [index, item] of order.entries()) {
    // A server per run, rooted at that run's build alone: no run can be served another variant's files.
    const build = builds.find((entry) => entry.name === item.variant);
    const server = await startStaticServer(build.dist, options.port);
    try {
      const run = await measureOneRun(playwright.chromium, server.origin, fixture, options, item, index, kind);
      run.served = { dist: reducePath(build.dist), buildIndexHtmlSha256: build.buildIdentity.indexHtmlSha256, ...server.served };
      document.runs.push(run);
      log(scrub(oneLine(run)));
    } finally {
      await server.close();
      ACTIVE.server = null;
    }
  }
  document.finishedUtc = new Date().toISOString();
  writeOutput(options.out, document, scrub);
  log(`wrote ${reducePath(options.out)}`);
  return document;
}

// ---------------------------------------------------------------------------------------------------
// resources
// ---------------------------------------------------------------------------------------------------

function ledgerTotals(owned) {
  const total = (counts) => Object.values(counts).reduce((sum, value) => sum + value, 0);
  const keys = Object.keys(owned.live);
  return {
    consistent: keys.every((key) => owned.created[key] - owned.disposed[key] === owned.live[key]),
    liveTotal: total(owned.live), createdTotal: total(owned.created), disposedTotal: total(owned.disposed)
  };
}

function differingKeys(a, b) {
  return [...new Set([...Object.keys(a), ...Object.keys(b)])].filter((key) => a[key] !== b[key]).map((key) => ({ key, before: a[key] ?? null, after: b[key] ?? null }));
}

async function observeActivity(page, observeMs = 1000, sampleCount = 5) {
  const samples = [];
  const started = performance.now();
  for (let i = 0; i < sampleCount; i++) {
    const wait = started + (observeMs * i) / (sampleCount - 1) - performance.now();
    if (wait > 0) await sleep(wait);
    samples.push(await page.evaluate(inPageWatchSample));
  }
  const first = samples[0];
  const last = samples[samples.length - 1];
  const observed = {
    observedMs: round(last.pageNowMs - first.pageNowMs, 0),
    maxPendingPageRaf: Math.max(...samples.map((entry) => entry.pendingPageRaf)),
    maxProductOwnedPendingRaf: Math.max(...samples.map((entry) => entry.productOwnedPendingRaf ?? 0)),
    rafRequested: last.requested - first.requested, rafFired: last.fired - first.fired,
    drawCalls: last.draws - first.draws, clears: last.clears - first.clears,
    productSubmissions: (last.productSubmissionSequence ?? 0) - (first.productSubmissionSequence ?? 0),
    cameraSequenceDelta: (last.cameraSequence ?? 0) - (first.cameraSequence ?? 0)
  };
  observed.animationFramePending = observed.maxPendingPageRaf > 0 || observed.maxProductOwnedPendingRaf > 0;
  observed.anythingDrawn = observed.drawCalls > 0 || observed.clears > 0 || observed.productSubmissions > 0;
  observed.settled = !observed.animationFramePending && !observed.anythingDrawn && observed.rafFired === 0 && observed.rafRequested === 0;
  return observed;
}

async function runResources(options, log = console.log) {
  const dist = resolveDist(options.dist);
  const scrub = buildScrubber([["{DIST}", dist]]);
  const fixture = loadFixture(options.pipes);
  const playwright = loadPlaywright();
  const document = {
    tool: TOOL, formatVersion: FORMAT_VERSION, kind: "resources", note: GUIDANCE_NOTE, createdUtc: new Date().toISOString(),
    arguments: recordedArguments("resources", options), buildIdentity: buildIdentity(dist), fixture: fixture.identity,
    host: hostIdentity(playwright.version), loadAverageBefore: os.loadavg().map((value) => round(value, 2)), error: null
  };
  const server = await startStaticServer(dist, options.port);
  let browser = null;
  try {
    const launched = await launchBrowser(playwright.chromium, "vsync");
    browser = launched.browser;
    document.browser = { source: launched.source, version: browser.version(), addedArguments: launched.addedArguments };
    const context = await browser.newContext({ viewport: { width: options.window.width, height: options.window.height }, deviceScaleFactor: options.dpr });
    document.browser.webgl = (await probeBrowser(context)).webgl;
    const { page, cdp, consoleLog, modelReadyMs } = await openProduct(context, server.origin, options.query, fixture, { initScript: WATCH_INIT_SOURCE });
    document.console = consoleLog;
    document.modelReadyMs = modelReadyMs;

    await setTheme(page, "light");
    await setGeometry(page, "schematic");
    await setToggle(page, "toggle-viewport-labels", true);
    await isometricThenFit(page);

    const steps = [];
    const snapshots = [];
    const recordStep = async (name) => {
      const settle = await waitSettled(page, cdp);
      const diagnostics = await page.evaluate(inPageReadDiagnostics);
      const viewport = diagnostics.viewport;
      snapshots.push(viewport);
      steps.push({ step: name, settled: settle.settled, ...ledgerTotals(viewport.owned), ledgerGeneration: viewport.owned.generation,
        rendererGeometries: viewport.rendererInfo.geometries, rendererTextures: viewport.rendererInfo.textures,
        drawCalls: viewport.rendererInfo.calls, triangles: viewport.rendererInfo.triangles, geometry: viewport.geometry.mode, labels: viewport.labels.enabled });
      log(`step ${steps.length}: ${name}: live ${steps[steps.length - 1].liveTotal}, created - disposed = live: ${steps[steps.length - 1].consistent}`);
      return steps.length - 1;
    };
    const rounds = [];
    const initial = await recordStep("initial: light, schematic, labels on, Isometric, Fit");
    const group = async (name, there, back) => {
      const ends = [];
      for (const round of [1, 2]) {
        await there();
        await recordStep(`${name}: away (${round})`);
        await back();
        ends.push(await recordStep(`${name}: back (${round})`));
      }
      rounds.push({ group: name, afterFirstRoundStep: ends[0] + 1, afterSecondRoundStep: ends[1] + 1,
        liveIdentical: differingKeys(snapshots[ends[0]].owned.live, snapshots[ends[1]].owned.live).length === 0,
        rendererGeometriesIdentical: snapshots[ends[0]].rendererInfo.geometries === snapshots[ends[1]].rendererInfo.geometries,
        rendererTexturesIdentical: snapshots[ends[0]].rendererInfo.textures === snapshots[ends[1]].rendererInfo.textures });
    };
    await group("theme light -> dark -> light", () => setTheme(page, "dark"), () => setTheme(page, "light"));
    await group("geometry schematic -> actual-od -> schematic", () => setGeometry(page, "actual-od"), () => setGeometry(page, "schematic"));
    const labelsOn = await readToggle(page, "toggle-viewport-labels");
    await group("labels toggled and restored", () => setToggle(page, "toggle-viewport-labels", !labelsOn), () => setToggle(page, "toggle-viewport-labels", labelsOn));
    const gridOn = await readToggle(page, "toggle-viewport-grid");
    await group("grid toggled and restored", () => setToggle(page, "toggle-viewport-grid", !gridOn), () => setToggle(page, "toggle-viewport-grid", gridOn));

    const first = snapshots[initial];
    const last = snapshots[snapshots.length - 1];
    document.steps = steps;
    document.ledger = {
      before: first.owned, after: last.owned,
      liveIdenticalBeforeAndAfter: differingKeys(first.owned.live, last.owned.live).length === 0,
      liveDifferences: differingKeys(first.owned.live, last.owned.live),
      createdMinusDisposedEqualsLiveThroughout: steps.every((step) => step.consistent),
      roundTrips: rounds
    };
    document.rendererInfo = { before: first.rendererInfo, after: last.rendererInfo };
    document.canvas = last.canvas;

    // Settle check. The last input was the grid toggle above; recordStep then waited and read, and sent no input.
    await sleep(1500);
    document.settleCheck = {
      method: "page-side wrappers installed before any product script count pending requestAnimationFrame callbacks and WebGL draw and clear calls; the product's own pending-frame count and main-render submission count are read beside them",
      waitedAfterLastInputMs: 1500,
      ...(await observeActivity(page))
    };
    log(`settle check: pending frame ${document.settleCheck.animationFramePending}, anything drawn ${document.settleCheck.anythingDrawn}, settled ${document.settleCheck.settled}`);

    // Control: the same observation while an orbit is under way must report "not settled".
    const press = await findPressPoint(page, cdp);
    await page.mouse.down({ button: "left" });
    const orbiting = driveOrbit(page, press, { width: press.canvasRect.width, height: press.canvasRect.height }, 2600);
    await sleep(800);
    const during = await observeActivity(page);
    const orbit = await orbiting;
    await orbit.drain();
    await page.mouse.up({ button: "left" });
    document.orbitControl = { purpose: "the same observation made while a real-pointer orbit is under way; it should report not settled", ...during, pointer: orbit.stats };
    log(`orbit control: pending frame ${during.animationFramePending}, anything drawn ${during.anythingDrawn}, settled ${during.settled}`);
    const tail = await waitSettled(page, cdp);
    document.settledAgainAfterOrbit = tail.settled;
  } catch (error) {
    document.error = String(error?.message ?? error).split("\n").slice(0, 3).join(" | ");
  } finally {
    await closeBrowser(browser);
    await server.close();
    ACTIVE.server = null;
    document.loadAverageAfter = os.loadavg().map((value) => round(value, 2));
    document.finishedUtc = new Date().toISOString();
  }
  writeOutput(options.out, document, scrub);
  log(`wrote ${reducePath(options.out)}`);
  return document;
}

// ---------------------------------------------------------------------------------------------------
// Selection before a run (--select), through the product's own model tree with real input.
// ---------------------------------------------------------------------------------------------------

const TREE = '[data-testid="model-tree-virtual"]';
const SELECT_METHOD = "the model tree, with real pointer input: every expanded group is collapsed by its own header button, the Pipes group is opened, the first pipe row is clicked, and for more than one pipe the list is scrolled with the mouse wheel and the last row of the range is Shift-clicked";

function inPageTreeState() {
  const tree = document.querySelector('[data-testid="model-tree-virtual"]');
  if (!tree) return null;
  const rect = tree.getBoundingClientRect();
  const groups = Array.from(tree.querySelectorAll('[data-testid^="tree-group-"]')).map((element) => ({ testId: element.getAttribute("data-testid"), expanded: element.getAttribute("aria-expanded") === "true" }));
  const first = tree.querySelector('[data-testid^="tree-row-pipe-"][aria-posinset="1"]');
  const firstRect = first ? first.getBoundingClientRect() : null;
  return {
    groups, scrollTop: tree.scrollTop, rect: { left: rect.left, top: rect.top, width: rect.width, height: rect.height },
    firstPipe: first ? { setSize: Number(first.getAttribute("aria-setsize")), rowHeight: firstRect.height, contentTop: firstRect.top - rect.top + tree.scrollTop } : null
  };
}

function inPageSelectionSummary() {
  const snapshot = globalThis.__openPipeStressUiDiagnosticsV1?.readCurrent?.();
  const selection = snapshot && snapshot.viewport && snapshot.viewport.status !== "unavailable" ? snapshot.viewport.selection : null;
  if (!selection) return null;
  const types = {};
  for (const ref of selection.orderedRefs) types[ref.type] = (types[ref.type] ?? 0) + 1;
  return { count: selection.orderedRefs.length, types, primaryRef: selection.primaryRef ? { type: selection.primaryRef.type, id: selection.primaryRef.id } : null, inputKind: selection.inputKind, actionSequence: selection.actionSequence };
}

async function selectPipesThroughTree(page, cdp, select, fixture) {
  if (select === "none") return { requested: "none", method: "nothing is selected by the tool; the product's start-up selection stands", wantedPipes: 0, reported: await page.evaluate(inPageSelectionSummary) };
  const wanted = select === "one" ? 1 : select === "hundred" ? 100 : fixture.identity.pipeSegments;
  const started = performance.now();
  const collapsed = [];
  for (let guard = 0; guard < 40; guard++) {
    const state = await page.evaluate(inPageTreeState);
    if (!state) throw new Error("--select: the model tree is absent");
    const open = state.groups.find((group) => group.expanded);
    if (!open) break;
    await page.click(`${TREE} [data-testid="${open.testId}"]`, { timeout: TIMEOUTS.controlMs });
    collapsed.push(open.testId);
  }
  await page.click(`${TREE} [data-testid="tree-group-Pipes"]`, { timeout: TIMEOUTS.controlMs });
  const rowOf = (position) => `${TREE} [data-testid^="tree-row-pipe-"][aria-posinset="${position}"]`;
  await page.waitForSelector(rowOf(1), { timeout: TIMEOUTS.controlMs });
  const state = await page.evaluate(inPageTreeState);
  if (!state.firstPipe || state.firstPipe.setSize !== fixture.identity.pipeSegments) throw new Error(`--select: the Pipes group lists ${state.firstPipe?.setSize ?? "no"} rows; the fixture has ${fixture.identity.pipeSegments} pipes`);
  await page.click(rowOf(1), { timeout: TIMEOUTS.controlMs });
  let wheelEvents = 0;
  if (wanted > 1) {
    const target = state.firstPipe.contentTop + (wanted - 1) * state.firstPipe.rowHeight - state.rect.height / 2;
    await page.mouse.move(state.rect.left + state.rect.width / 2, state.rect.top + state.rect.height / 2);
    for (let attempt = 0; attempt < 16; attempt++) {
      const now = await page.evaluate(inPageTreeState);
      const delta = target - now.scrollTop;
      if (await page.$(rowOf(wanted)) && Math.abs(delta) < state.rect.height / 2) break;
      await page.mouse.wheel(0, delta);
      wheelEvents += 1;
      await sleep(250);
    }
    await page.click(rowOf(wanted), { modifiers: ["Shift"], timeout: TIMEOUTS.controlMs });
  }
  let reported = null;
  const deadline = performance.now() + 120000;
  for (;;) {
    reported = await page.evaluate(inPageSelectionSummary);
    if (reported && reported.count === wanted && reported.types.pipe === wanted) break;
    if (performance.now() > deadline) throw new Error(`--select ${select}: asked for ${wanted} pipes; the diagnostics surface reports ${reported ? `${reported.count} selected (${JSON.stringify(reported.types)})` : "no selection"}`);
    await sleep(200);
  }
  const settle = await waitSettled(page, cdp);
  return { requested: select, method: SELECT_METHOD, which: wanted === 1 ? "the first pipe in the tree's order" : `the first ${wanted} pipes in the tree's order`, wantedPipes: wanted, reported, collapsedGroups: collapsed, wheelEvents, tookMs: round(performance.now() - started, 0), settledAfter: settle.settled };
}

// ---------------------------------------------------------------------------------------------------
// hover: the pointer recorder. Counts the pointermove events the page dispatches and which of them the canvas received.
// ---------------------------------------------------------------------------------------------------

const POINTER_RECORDER_SOURCE = `(() => {
  const capacity = ${POINTER_EVENT_CAPACITY};
  const recorder = { t: new Float64Array(capacity), onCanvas: new Uint8Array(capacity), n: 0, overflow: false };
  const canvas = document.querySelector('[data-testid="viewport-canvas"] canvas');
  recorder.listener = (event) => {
    if (recorder.n < capacity) { recorder.t[recorder.n] = performance.now(); recorder.onCanvas[recorder.n] = event.target === canvas ? 1 : 0; recorder.n += 1; }
    else recorder.overflow = true;
  };
  document.addEventListener("pointermove", recorder.listener, { capture: true, passive: true });
  Object.defineProperty(globalThis, "__bCanvasOrbitProbePointer", { value: recorder, configurable: true });
  return true;
})()`;

function inPageCollectPointer(request) {
  const recorder = globalThis.__bCanvasOrbitProbePointer;
  if (!recorder) return null;
  document.removeEventListener("pointermove", recorder.listener, { capture: true });
  let inWindow = 0;
  let onCanvas = 0;
  for (let i = 0; i < recorder.n; i++) {
    if (recorder.t[i] < request.fromMs || recorder.t[i] > request.toMs) continue;
    inWindow += 1;
    onCanvas += recorder.onCanvas[i];
  }
  return { recorded: recorder.n, overflow: recorder.overflow, eventsInWindow: inWindow, receivedByCanvasInWindow: onCanvas, fractionReceivedByCanvas: inWindow ? Math.round((onCanvas / inWindow) * 1000) / 1000 : null };
}

// ---------------------------------------------------------------------------------------------------
// pairs: pure image functions, then the subcommand.
// A SIMPLIFIED form of the rule in {LANE}/proposals/P1_SECOND_PROFILE.md section 4.4: no band is predicted from
// display primitives and nothing is exclusive to the winner, so every newly Selection-coloured pixel in the region counts.
// ---------------------------------------------------------------------------------------------------

function parseHexColour(value) {
  const match = /^#([0-9a-f]{6})$/i.exec(String(value).trim());
  if (!match) throw new Error(`token colour "${value}" is not a six-digit hex value`);
  return [0, 2, 4].map((at) => parseInt(match[1].slice(at, at + 2), 16));
}

function relativeLuminance(r, g, b) {
  const linear = (channel) => { const c = channel / 255; return c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4; };
  return 0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b);
}

function contrastRatio(a, b) {
  const la = relativeLuminance(a[0], a[1], a[2]);
  const lb = relativeLuminance(b[0], b[1], b[2]);
  return (Math.max(la, lb) + 0.05) / (Math.min(la, lb) + 0.05);
}

const CRC_TABLE = (() => {
  const table = new Uint32Array(256);
  for (let n = 0; n < 256; n++) { let c = n; for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; table[n] = c >>> 0; }
  return table;
})();
function crc32(buffer) {
  let c = 0xffffffff;
  for (let i = 0; i < buffer.length; i++) c = CRC_TABLE[(c ^ buffer[i]) & 0xff] ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
}
const PNG_SIGNATURE = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);

/** Decodes an 8-bit, non-interlaced RGB or RGBA PNG (what a browser screenshot is) to RGBA bytes. */
function decodePng(buffer) {
  if (buffer.length < 8 || !buffer.subarray(0, 8).equals(PNG_SIGNATURE)) throw new Error("not a PNG");
  let at = 8;
  let header = null;
  const data = [];
  while (at + 8 <= buffer.length) {
    const length = buffer.readUInt32BE(at);
    const type = buffer.toString("latin1", at + 4, at + 8);
    const body = buffer.subarray(at + 8, at + 8 + length);
    if (type === "IHDR") header = { width: body.readUInt32BE(0), height: body.readUInt32BE(4), depth: body[8], colourType: body[9], interlace: body[12] };
    else if (type === "IDAT") data.push(body);
    else if (type === "IEND") break;
    at += 12 + length;
  }
  if (!header || header.depth !== 8 || header.interlace !== 0 || (header.colourType !== 2 && header.colourType !== 6)) throw new Error("unsupported PNG: only 8-bit non-interlaced RGB or RGBA is read");
  const channels = header.colourType === 6 ? 4 : 3;
  const stride = header.width * channels;
  const raw = zlib.inflateSync(Buffer.concat(data));
  if (raw.length !== (stride + 1) * header.height) throw new Error("PNG data has an unexpected length");
  const rows = Buffer.alloc(stride * header.height);
  for (let y = 0; y < header.height; y++) {
    const filter = raw[y * (stride + 1)];
    const line = y * (stride + 1) + 1;
    for (let x = 0; x < stride; x++) {
      const left = x >= channels ? rows[y * stride + x - channels] : 0;
      const up = y > 0 ? rows[(y - 1) * stride + x] : 0;
      const upLeft = y > 0 && x >= channels ? rows[(y - 1) * stride + x - channels] : 0;
      let predicted = 0;
      if (filter === 1) predicted = left;
      else if (filter === 2) predicted = up;
      else if (filter === 3) predicted = (left + up) >> 1;
      else if (filter === 4) { const p = left + up - upLeft; const pa = Math.abs(p - left); const pb = Math.abs(p - up); const pc = Math.abs(p - upLeft); predicted = pa <= pb && pa <= pc ? left : pb <= pc ? up : upLeft; }
      else if (filter !== 0) throw new Error(`PNG filter ${filter} is unknown`);
      rows[y * stride + x] = (raw[line + x] + predicted) & 0xff;
    }
  }
  const rgba = new Uint8Array(header.width * header.height * 4);
  let opaque = true;
  for (let i = 0, o = 0; i < rows.length; i += channels, o += 4) {
    rgba[o] = rows[i]; rgba[o + 1] = rows[i + 1]; rgba[o + 2] = rows[i + 2];
    rgba[o + 3] = channels === 4 ? rows[i + 3] : 255;
    if (rgba[o + 3] !== 255) opaque = false;
  }
  return { width: header.width, height: header.height, rgba, opaque };
}

/** Encodes RGBA bytes as a PNG (filter 0 on every row). Used by the self-test only, to check the decoder. */
function encodePng(width, height, rgba) {
  const chunk = (type, body) => {
    const head = Buffer.alloc(8);
    head.writeUInt32BE(body.length, 0);
    head.write(type, 4, "latin1");
    const tail = Buffer.alloc(4);
    tail.writeUInt32BE(crc32(Buffer.concat([head.subarray(4), body])), 0);
    return Buffer.concat([head, body, tail]);
  };
  const header = Buffer.alloc(13);
  header.writeUInt32BE(width, 0); header.writeUInt32BE(height, 4); header[8] = 8; header[9] = 6;
  const raw = Buffer.alloc((width * 4 + 1) * height);
  for (let y = 0; y < height; y++) Buffer.from(rgba.buffer, rgba.byteOffset + y * width * 4, width * 4).copy(raw, y * (width * 4 + 1) + 1);
  return Buffer.concat([PNG_SIGNATURE, chunk("IHDR", header), chunk("IDAT", zlib.deflateSync(raw)), chunk("IEND", Buffer.alloc(0))]);
}

/**
 * before and after are RGBA byte arrays of one size; pixelRatio is device pixels per CSS px.
 * Newly qualifying: within the tolerance of Selection after the pick and not before. Ground: within the tolerance of
 * the canvas ground after the pick. A pair is a newly qualifying pixel and a ground pixel whose centres are at most
 * pairDistanceCssPx apart, the two at the minimum contrast or more (their own colours, not the tokens'), each ground
 * pixel used once. Fixed order: newly qualifying pixels row by row from the top left; for each, the nearest free
 * ground pixel, ties broken by row and then by column. The matching is greedy, so it can undercount, never overcount.
 */
function haloPairAnalysis(before, after, width, height, pixelRatio, selectionRgb, groundRgb, rule = PAIRS_RULE) {
  const total = width * height;
  if (before.length !== total * 4 || after.length !== total * 4) throw new Error("the two images must have the stated size");
  const near = (image, i, rgb) => Math.abs(image[i * 4] - rgb[0]) <= rule.channelTolerance && Math.abs(image[i * 4 + 1] - rgb[1]) <= rule.channelTolerance && Math.abs(image[i * 4 + 2] - rgb[2]) <= rule.channelTolerance;
  let changed = false;
  for (let i = 0; i < total * 4 && !changed; i++) if (before[i] !== after[i]) changed = true;
  const selAfter = new Uint8Array(total);
  const groundAfter = new Uint8Array(total);
  const newly = new Uint8Array(total);
  let selBeforeCount = 0; let selAfterCount = 0; let groundAfterCount = 0; let newlyCount = 0; let overGround = 0; let lost = 0;
  for (let i = 0; i < total; i++) {
    const wasSelection = near(before, i, selectionRgb);
    if (wasSelection) selBeforeCount += 1;
    if (near(after, i, selectionRgb)) { selAfter[i] = 1; selAfterCount += 1; } else if (wasSelection) lost += 1;
    if (near(after, i, groundRgb)) { groundAfter[i] = 1; groundAfterCount += 1; }
    if (selAfter[i] && !wasSelection) { newly[i] = 1; newlyCount += 1; if (near(before, i, groundRgb)) overGround += 1; }
  }
  const reach = rule.pairDistanceCssPx * pixelRatio;
  const offsets = [];
  for (let dy = -Math.ceil(reach); dy <= Math.ceil(reach); dy++) for (let dx = -Math.ceil(reach); dx <= Math.ceil(reach); dx++) {
    if ((dx || dy) && dx * dx + dy * dy <= reach * reach + 1e-9) offsets.push([dx * dx + dy * dy, dy, dx]);
  }
  offsets.sort((a, b) => a[0] - b[0] || a[1] - b[1] || a[2] - b[2]);
  const innerReach = Math.max(1, Math.round(pixelRatio));
  const used = new Uint8Array(total);
  let pairs = 0; let pairsIgnoringContrast = 0; let withGroundInReach = 0; let inner = 0;
  const usedLoose = new Uint8Array(total);
  for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
    const i = y * width + x;
    if (!newly[i]) continue;
    let isInner = true;
    for (let dy = -innerReach; dy <= innerReach && isInner; dy++) for (let dx = -innerReach; dx <= innerReach; dx++) {
      const nx = x + dx; const ny = y + dy;
      if (nx < 0 || ny < 0 || nx >= width || ny >= height || !selAfter[ny * width + nx]) { isInner = false; break; }
    }
    if (isInner) inner += 1;
    let sawGround = false; let paired = false; let pairedLoose = false;
    const own = [after[i * 4], after[i * 4 + 1], after[i * 4 + 2]];
    for (const [, dy, dx] of offsets) {
      const nx = x + dx; const ny = y + dy;
      if (nx < 0 || ny < 0 || nx >= width || ny >= height) continue;
      const j = ny * width + nx;
      if (!groundAfter[j]) continue;
      sawGround = true;
      if (!pairedLoose && !usedLoose[j]) { usedLoose[j] = 1; pairedLoose = true; pairsIgnoringContrast += 1; }
      if (!paired && !used[j] && contrastRatio(own, [after[j * 4], after[j * 4 + 1], after[j * 4 + 2]]) >= rule.minimumContrast) { used[j] = 1; paired = true; pairs += 1; }
      if (paired && pairedLoose) break;
    }
    if (sawGround) withGroundInReach += 1;
  }
  const pass = changed && pairs >= rule.minimumPairs;
  const failureClass = pass ? null : (!changed || newlyCount === 0) ? "no-cue-seen" : withGroundInReach === 0 ? "no-ground-beside" : "too-few-pairs";
  return {
    imageChanged: changed, newlyQualifying: newlyCount, newlyOverGround: overGround, newlyOverFigure: newlyCount - overGround, newlyInner: inner,
    newlyWithGroundInReach: withGroundInReach, groundPixelsAfter: groundAfterCount, pairs, pairsIgnoringContrast,
    selectionColouredBefore: selBeforeCount, selectionColouredAfter: selAfterCount, selectionColouredLost: lost, passesWithNoCasing: pass, failureClass
  };
}

function readTokenColours(theme) {
  const relative = "apps/desktop/src/design/tokens.json";
  const file = path.join(requireWorkingRoot(), ...relative.split("/"));
  const bytes = readFileSync(file);
  const tokens = JSON.parse(bytes.toString("utf8"));
  // The token file keys its colours by dotted name under "color" ("canvas.selection"), each with a value per theme.
  const canvas = { selection: tokens?.color?.["canvas.selection"], bg: tokens?.color?.["canvas.bg"] };
  if (typeof canvas.selection?.[theme] !== "string" || typeof canvas.bg?.[theme] !== "string") throw new Error('the token file has no "canvas.selection" or "canvas.bg" colour for the theme');
  return { file: `{WORKING_ROOT}/${relative}`, sha256: createHash("sha256").update(bytes).digest("hex"), theme,
    selection: { token: "color.canvas.selection", hex: canvas.selection[theme], rgb: parseHexColour(canvas.selection[theme]) },
    ground: { token: "color.canvas.bg", hex: canvas.bg[theme], rgb: parseHexColour(canvas.bg[theme]) } };
}

function loadPointSamples(pipes) {
  const read = (relative) => {
    const file = path.join(requireWorkingRoot(), ...relative.split("/"));
    if (!existsSync(file)) throw new Error(`sample file is absent: {WORKING_ROOT}/${relative}`);
    const bytes = readFileSync(file);
    return { identity: { file: `{WORKING_ROOT}/${relative}`, sha256: createHash("sha256").update(bytes).digest("hex") }, json: JSON.parse(bytes.toString("utf8")) };
  };
  const interactions = read(`apps/desktop/e2e/ui-foundation/samples/ui-foundation-${pipes}.interactions.json`);
  const oracle = read(`apps/desktop/e2e/ui-foundation/samples/ui-foundation-${pipes}.point-oracle-v3.json`);
  const probes = new Map(oracle.json.probes.map((probe) => [probe.sample, probe]));
  const samples = interactions.json.point_selection.map((entry) => {
    const probe = probes.get(entry.sample);
    const sameAnchor = probe && probe.probe_anchor_ref.type === entry.probe_anchor_ref.type && probe.probe_anchor_ref.id === entry.probe_anchor_ref.id;
    if (!sameAnchor) throw new Error(`sample ${entry.sample}: the two sample files name different anchors`);
    return { sample: entry.sample, anchorRef: entry.probe_anchor_ref, authoredAnchor: probe.authored_anchor, nominalExpectedRef: probe.candidate_nominal_preflight_only?.oracle?.expectedHitRef ?? null };
  });
  return {
    samples, files: [interactions.identity, oracle.identity], nominalCamera: oracle.json.candidate_nominal_camera ?? null,
    fieldsUsed: ["interactions: point_selection[].sample, point_selection[].probe_anchor_ref", "point oracle: probes[].sample, probes[].probe_anchor_ref (cross-checked), probes[].authored_anchor (the point, projected by the product's read-only projectAuthoredPoint), probes[].candidate_nominal_preflight_only.oracle.expectedHitRef (the expected winner, nominal), candidate_nominal_camera (compared with the camera read back)"]
  };
}

function parseSampleRange(value, name) {
  const match = /^(\d{1,3})-(\d{1,3})$/.exec(value);
  if (!match || Number(match[1]) < 1 || Number(match[2]) < Number(match[1])) throw new UsageError(`--${name} must look like 1-200`);
  return { from: Number(match[1]), to: Number(match[2]) };
}

function inPageProject(point) {
  const surface = globalThis.__openPipeStressUiDiagnosticsV1;
  const snapshot = surface.readCurrent();
  const viewport = snapshot.viewport;
  const result = surface.projectAuthoredPoint({ modelGeneration: snapshot.model.generation, cameraSequence: viewport.camera.sequence, authoredPoint: { x: point[0], y: point[1], z: point[2] } });
  return { status: result.status, inside: result.status === "available" ? result.insideCanvasCss && result.insideClosedNdc : false, css: result.status === "available" ? { x: result.canvasCssPoint.x, y: result.canvasCssPoint.y } : null, cameraSequence: viewport.camera.sequence };
}

function inPageCamera() {
  const camera = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent().viewport.camera;
  return { sequence: camera.sequence, position: Array.from(camera.position), target: Array.from(camera.target), fovDegrees: camera.fovDegrees, aspect: camera.aspect };
}

const sameRef = (a, b) => Boolean(a && b && a.type === b.type && a.id === b.id);
const PROJECT_ROW = `${TREE} [data-testid^="tree-row-project-"]`;

async function resetToProjectSelection(page) {
  await page.click(PROJECT_ROW, { timeout: TIMEOUTS.controlMs });
  await page.waitForFunction(() => {
    const selection = globalThis.__openPipeStressUiDiagnosticsV1?.readCurrent?.()?.viewport?.selection;
    return Boolean(selection && selection.orderedRefs.length === 1 && selection.orderedRefs[0].type === "project");
  }, null, { timeout: TIMEOUTS.controlMs, polling: 50 });
}

async function runPairs(options, log = console.log) {
  const dist = resolveDist(options.dist);
  const scrub = buildScrubber([["{DIST}", dist]]);
  const fixture = loadFixture(options.pipes);
  const points = loadPointSamples(options.pipes);
  const colours = readTokenColours(options.theme);
  const playwright = loadPlaywright();
  const document = {
    tool: TOOL, formatVersion: FORMAT_VERSION, kind: "pairs", note: GUIDANCE_NOTE, createdUtc: new Date().toISOString(),
    ruleNote: "A SIMPLIFIED form of the rule in {LANE}/proposals/P1_SECOND_PROFILE.md section 4.4: no band is predicted from display primitives and nothing is exclusive to the winner, so every newly Selection-coloured pixel in the 48 x 48 CSS px region counts, whatever drew it (a halo, a recoloured body, the first profile's diamond cue, a neighbour). It answers how often ground lies beside the cue with no casing; it is not the second profile's witness.",
    describes: options.note ?? null,
    rule: { ...PAIRS_RULE, order: "newly qualifying pixels row by row from the top left; for each the nearest free ground pixel, ties by row then column; greedy, each ground pixel used once" },
    arguments: recordedArguments("pairs", options), buildIdentity: buildIdentity(dist), fixture: fixture.identity, sampleFiles: points.files, sampleFieldsUsed: points.fieldsUsed,
    tokens: colours, host: hostIdentity(playwright.version), loadAverageBefore: os.loadavg().map((value) => round(value, 2)), error: null, samples: []
  };
  const server = await startStaticServer(dist, options.port);
  let browser = null;
  try {
    const launched = await launchBrowser(playwright.chromium, "vsync");
    browser = launched.browser;
    document.browser = { source: launched.source, version: browser.version(), addedArguments: launched.addedArguments };
    const context = await browser.newContext({ viewport: { width: options.window.width, height: options.window.height }, deviceScaleFactor: options.dpr });
    document.browser.webgl = (await probeBrowser(context)).webgl;
    const { page, cdp, consoleLog, modelReadyMs } = await openProduct(context, server.origin, options.query, fixture);
    document.console = consoleLog;
    document.modelReadyMs = modelReadyMs;
    await setTheme(page, options.theme);
    await setGeometry(page, options.mode);
    await setToggle(page, "toggle-viewport-labels", false);
    await isometricThenFit(page);
    await waitSettled(page, cdp);
    const geometry = await page.evaluate(inPageCanvasGeometry);
    const rect = geometry.canvasRect;
    const camera = await page.evaluate(inPageCamera);
    const nominal = points.nominalCamera;
    const distance = nominal ? Math.hypot(...camera.position.map((value, i) => value - nominal.position[i])) : null;
    document.canvas = { cssWidth: round(rect.width, 2), cssHeight: round(rect.height, 2), pixelRatio: geometry.page.devicePixelRatio };
    document.camera = {
      readBack: { position: camera.position.map((value) => round(value, 4)), target: camera.target.map((value) => round(value, 4)), fovDegrees: camera.fovDegrees, aspect: round(camera.aspect, 4) },
      nominalOfTheSampleFile: nominal ? { position: nominal.position, target: nominal.target } : null,
      positionDistanceFromNominal: round(distance, 3),
      expectedWinnersApply: distance !== null && distance < 1e-3,
      note: "The sample file's expected winners are nominal, for its own camera. A hit depends on the camera position. Where the camera read back differs, a selected entity other than the expected one is not by itself a picking fault; the anchor entity is reported beside it."
    };

    // The resting point: a canvas point where a click selects nothing. It gives the canvas keyboard focus before the
    // first capture, and the pointer rests there for both captures, so neither holds a hover.
    let rest = null;
    for (const candidate of pressCandidates(geometry)) {
      if (!(await page.evaluate(inPageCanvasReceives, candidate))) continue;
      await resetToProjectSelection(page);
      await page.mouse.click(candidate.x, candidate.y);
      await waitSettled(page, cdp);
      const selection = await page.evaluate(inPageSelectionSummary);
      if (selection.count === 1 && selection.primaryRef?.type === "project") { rest = { x: candidate.x, y: candidate.y }; break; }
    }
    if (!rest) throw new Error("no canvas point was found where a click selects nothing");
    document.restingPoint = { x: round(rest.x - rect.left, 1), y: round(rest.y - rect.top, 1), basis: "canvas CSS px" };

    const half = PAIRS_RULE.regionCssPx / 2;
    for (const sample of points.samples.filter((entry) => entry.sample >= options.samples.from && entry.sample <= options.samples.to)) {
      const entry = { sample: sample.sample, anchorRef: sample.anchorRef, nominalExpectedRef: sample.nominalExpectedRef, status: "measured", reason: null };
      document.samples.push(entry);
      try {
        await resetToProjectSelection(page);
        await page.mouse.click(rest.x, rest.y);
        await waitSettled(page, cdp);
        const projected = await page.evaluate(inPageProject, sample.authoredAnchor);
        if (!projected.inside) { entry.status = "not-attempted"; entry.reason = `the anchor projects outside the canvas (${projected.status})`; continue; }
        const point = { x: rect.left + projected.css.x, y: rect.top + projected.css.y };
        entry.point = { x: round(projected.css.x, 2), y: round(projected.css.y, 2), basis: "canvas CSS px" };
        const clip = { x: Math.floor(point.x) - half, y: Math.floor(point.y) - half, width: PAIRS_RULE.regionCssPx, height: PAIRS_RULE.regionCssPx };
        if (clip.x < rect.left || clip.y < rect.top || clip.x + clip.width > rect.left + rect.width || clip.y + clip.height > rect.top + rect.height) { entry.status = "not-attempted"; entry.reason = "the region leaves the canvas"; continue; }
        if (!(await page.evaluate(inPageCanvasReceives, point))) { entry.status = "not-attempted"; entry.reason = "something other than the canvas receives the pointer at the point"; continue; }
        const beforeSelection = await page.evaluate(inPageSelectionSummary);
        if (!(beforeSelection.count === 1 && beforeSelection.primaryRef?.type === "project")) { entry.status = "not-attempted"; entry.reason = "the selection was not the project alone before the pick"; continue; }
        const before = decodePng(await page.screenshot({ clip, type: "png" }));
        await page.mouse.click(point.x, point.y);
        let selected = null;
        const deadline = performance.now() + 2000;
        for (;;) {
          selected = await page.evaluate(inPageSelectionSummary);
          if (selected.primaryRef?.type !== "project" || performance.now() > deadline) break;
          await sleep(50);
        }
        await page.mouse.move(rest.x, rest.y);
        await waitSettled(page, cdp);
        const after = decodePng(await page.screenshot({ clip, type: "png" }));
        const cameraAfter = await page.evaluate(inPageCamera);
        const picked = selected.primaryRef?.type === "project" ? null : selected.primaryRef;
        entry.selected = { count: picked ? selected.count : 0, ref: picked };
        entry.selectedIsNominalExpected = sameRef(picked, sample.nominalExpectedRef);
        entry.selectedIsAnchor = sameRef(picked, sample.anchorRef);
        entry.cameraUnchanged = cameraAfter.sequence === projected.cameraSequence;
        entry.imagesOpaque = before.opaque && after.opaque;
        if (before.width !== after.width || before.height !== after.height || before.width !== PAIRS_RULE.regionCssPx * geometry.page.devicePixelRatio) throw new Error(`the captures are ${before.width} x ${before.height} and ${after.width} x ${after.height}`);
        Object.assign(entry, haloPairAnalysis(before.rgba, after.rgba, before.width, before.height, geometry.page.devicePixelRatio, colours.selection.rgb, colours.ground.rgb));
      } catch (error) {
        entry.status = "failed";
        entry.reason = String(error?.message ?? error).split("\n")[0];
      }
      if (document.samples.length % 20 === 0) log(`  ${document.samples.length} samples done`);
    }
  } catch (error) {
    document.error = String(error?.message ?? error).split("\n").slice(0, 3).join(" | ");
  } finally {
    await closeBrowser(browser);
    await server.close();
    ACTIVE.server = null;
    document.loadAverageAfter = os.loadavg().map((value) => round(value, 2));
    document.finishedUtc = new Date().toISOString();
  }
  document.totals = pairsTotals(document.samples);
  writeOutput(options.out, document, scrub);
  const t = document.totals;
  log(document.error ? `pairs: FAILED: ${scrub(document.error)}` : `pairs: ${t.passesWithNoCasing} of ${t.samples} samples pass with no casing (measured ${t.measured}; no cue seen ${t.failures["no-cue-seen"]}, no ground beside ${t.failures["no-ground-beside"]}, too few pairs ${t.failures["too-few-pairs"]}; not attempted ${t.notAttempted}, failed ${t.failed}); selected = nominal expected ${t.selectedIsNominalExpected}, = anchor ${t.selectedIsAnchor}, nothing selected ${t.nothingSelected}`);
  log(`wrote ${reducePath(options.out)}`);
  return document;
}

function pairsTotals(samples) {
  const measured = samples.filter((entry) => entry.status === "measured");
  const failures = { "no-cue-seen": 0, "no-ground-beside": 0, "too-few-pairs": 0 };
  for (const entry of measured) if (entry.failureClass) failures[entry.failureClass] += 1;
  const passing = measured.filter((entry) => entry.passesWithNoCasing);
  const share = (entry) => (entry.newlyQualifying ? entry.newlyOverFigure / entry.newlyQualifying : null);
  return {
    samples: samples.length, measured: measured.length, notAttempted: samples.filter((entry) => entry.status === "not-attempted").length, failed: samples.filter((entry) => entry.status === "failed").length,
    passesWithNoCasing: passing.length, passesWhereSelectedIsNominalExpected: passing.filter((entry) => entry.selectedIsNominalExpected).length, failures,
    selectedIsNominalExpected: measured.filter((entry) => entry.selectedIsNominalExpected).length, selectedIsAnchor: measured.filter((entry) => entry.selectedIsAnchor).length,
    selectedIsNeither: measured.filter((entry) => entry.selected?.ref && !entry.selectedIsNominalExpected && !entry.selectedIsAnchor).length,
    nothingSelected: measured.filter((entry) => !entry.selected?.ref).length,
    medianNewlyQualifying: median(measured.map((entry) => entry.newlyQualifying)), medianPairs: median(measured.map((entry) => entry.pairs)),
    medianNewlyInner: median(measured.map((entry) => entry.newlyInner)), medianShareOverFigure: round(median(measured.map(share).filter((value) => value !== null)), 3)
  };
}

// ---------------------------------------------------------------------------------------------------
// summarize: medians and ranges, never a verdict.
// ---------------------------------------------------------------------------------------------------

const fmt = (value, digits = 2) => (value === null || value === undefined || !Number.isFinite(value) ? "n/a" : value.toFixed(digits));
const fmt3 = (value) => fmt(value, 3);
const pct = (value) => (value === null || value === undefined || !Number.isFinite(value) ? "n/a" : `${(value * 100).toFixed(1)}%`);
const medianRange = (values, format = fmt) => {
  const usable = values.filter((value) => value !== null && value !== undefined && Number.isFinite(value));
  if (!usable.length) return "n/a";
  const [low, high] = range(usable);
  return low === high ? format(low) : `${format(median(usable))} [${format(low)} to ${format(high)}]`;
};

function variantRows(document) {
  const names = [...new Set(document.runs.map((run) => run.variant))];
  return names.map((name) => {
    const all = document.runs.filter((run) => run.variant === name);
    const good = all.filter((run) => !run.error);
    const pick = (read) => good.map(read);
    const p95s = pick((run) => run.frames.measure.p95Ms);
    const consoleErrors = all.reduce((sum, run) => sum + (run.console?.errorCount ?? 0) + (run.console?.pageErrorCount ?? 0), 0);
    const firstError = all.flatMap((run) => [...(run.console?.errors ?? []), ...(run.console?.pageErrors ?? [])])[0];
    return {
      name, runs: all.length, failed: all.length - good.length, good,
      p95OfRunP95: percentileNearestRank(p95s, 95), p50OfRunP95: percentileNearestRank(p95s, 50), p95s,
      p50s: pick((run) => run.frames.measure.p50Ms), means: pick((run) => run.frames.measure.meanMs),
      overA: pick((run) => run.frames.measure.fractionOver16_7Ms), overB: pick((run) => run.frames.measure.fractionOver33_3Ms),
      taskPerFrame: pick((run) => run.mainThread?.taskMsPerFrame ?? null),
      calls: pick((run) => run.diagnostics?.beforeOrbit?.viewport?.rendererInfo?.calls ?? null),
      triangles: pick((run) => run.diagnostics?.beforeOrbit?.viewport?.rendererInfo?.triangles ?? null),
      load: mean(pick((run) => (run.loadAverageBefore[0] + run.loadAverageAfter[0]) / 2)),
      consoleErrors, firstError
    };
  });
}

function pairedDifferences(document) {
  const names = [...new Set(document.runs.map((run) => run.variant))];
  const reference = names[0];
  const byPair = (name) => new Map(document.runs.filter((run) => run.variant === name && !run.error).map((run) => [run.pair, run]));
  const base = byPair(reference);
  return names.slice(1).map((name) => {
    const other = byPair(name);
    const pairs = [...base.keys()].filter((pair) => other.has(pair));
    const difference = (read) => pairs.map((pair) => read(other.get(pair)) - read(base.get(pair)));
    return {
      name, reference, pairs: pairs.length,
      p95: difference((run) => run.frames.measure.p95Ms), p50: difference((run) => run.frames.measure.p50Ms), mean: difference((run) => run.frames.measure.meanMs),
      task: difference((run) => run.mainThread?.taskMsPerFrame ?? NaN),
      referenceMedianP95: median([...base.values()].map((run) => run.frames.measure.p95Ms)),
      referenceMedianMean: median([...base.values()].map((run) => run.frames.measure.meanMs))
    };
  });
}

function summarize(documents) {
  const lines = [];
  const series = documents.filter((entry) => entry.document.kind === "run-series" || entry.document.kind === "hover-series");
  const pairsDocuments = documents.filter((entry) => entry.document.kind === "pairs");
  const resources = documents.filter((entry) => entry.document.kind === "resources");
  lines.push("# Orbit probe summary", "", GUIDANCE_NOTE, "",
    "Medians and ranges only. Percentiles are nearest-rank; with three runs the p95 of the per-run p95 values is the largest of them. Intervals are between requestAnimationFrame timestamps in the measurement window.", "");
  if (series.length) {
    lines.push("## Series", "", "| File | Pipes | Mode | Labels | Theme | DPR | Window | Canvas (CSS px) | Pacing | Pacing calibration p50 (ms) | Uncapped took effect | Warm-up / measure (ms) | Build index.html SHA-256 | Browser | WebGL renderer |", "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|");
    for (const { name, document } of series) {
      const a = document.arguments;
      const good = document.runs.filter((run) => !run.error);
      const took = a.pacing === "uncapped" ? (good.length ? (good.every((run) => run.pacing.uncappedTookEffect) ? "yes" : good.some((run) => run.pacing.uncappedTookEffect) ? "in some runs" : "no") : "n/a") : "not requested";
      const canvases = [...new Set(good.map((run) => `${fmt(run.page.canvasCssWidth, 0)} x ${fmt(run.page.canvasCssHeight, 0)}`))].join(", ");
      lines.push(`| ${name} | ${a.pipes} | ${a.mode} | ${a.labels} | ${a.theme} | ${a.dpr} | ${a.window} | ${canvases || "n/a"} | ${a.pacing} | ${medianRange(good.map((run) => run.pacing.calibrationIntervalP50Ms))} | ${took} | ${a["warmup-ms"]} / ${a["measure-ms"]} | ${document.buildIdentity ? document.buildIdentity.indexHtmlSha256 : "differs by variant; see Builds"} | ${[...new Set(good.map((run) => run.browser.version))].join(", ") || "n/a"} | ${[...new Set(good.map((run) => run.browser.webgl?.renderer ?? "n/a"))].join(", ") || "n/a"} |`);
    }
    lines.push("", "## Builds", "", "A build identity is the SHA-256 of index.html with the names and sizes of the files under assets.", "",
      "| File | Variant | Query | Build directory | index.html SHA-256 | Asset files | Asset bytes | Against the first variant |", "|---|---|---|---|---|---|---|---|");
    const statements = [];
    for (const { name, document } of series) {
      const variants = document.variants ?? [];
      for (const [index, variant] of variants.entries()) {
        const against = index === 0 ? "the reference" : sameBuildIdentity(variant.buildIdentity, variants[0].buildIdentity) ? "same build identity" : "different build identity";
        lines.push(`| ${name} | ${variant.name} | ${variant.query === "" ? "(empty)" : `\`${variant.query}\``} | ${variant.dist} | ${variant.buildIdentity.indexHtmlSha256} | ${variant.buildIdentity.assetCount} | ${variant.buildIdentity.assetBytes} | ${against} |`);
      }
      if (variants.length > 1) {
        const sameBuild = variants.every((variant) => sameBuildIdentity(variant.buildIdentity, variants[0].buildIdentity));
        const sameQuery = variants.every((variant) => variant.query === variants[0].query);
        statements.push(`- ${name}: ${sameBuild && sameQuery ? "every variant has the same build identity and the same query, so this is an A/A series and its differences are noise." : sameBuild ? "every variant has the same build identity; the variants differ by query only." : "the variants do not share one build identity; they compare different builds."}`);
      }
    }
    if (statements.length) lines.push("", ...statements);
    lines.push("", "## Variants", "", "| File | Variant | Runs | Per-run p95 (ms): p50 | Per-run p95 (ms): p95 | Per-run p95 (ms): range | Per-run p50 (ms) | Mean interval (ms) | Over 16.7 ms | Over 33.3 ms | Main thread (ms/frame) | Draw calls | Triangles | Mean load (1 min) | Console errors |", "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|");
    for (const { name, document } of series) {
      for (const row of variantRows(document)) {
        const spread = range(row.p95s);
        lines.push(`| ${name} | ${row.name} | ${row.runs}${row.failed ? ` (${row.failed} failed)` : ""} | ${fmt(row.p50OfRunP95)} | ${fmt(row.p95OfRunP95)} | ${spread ? `${fmt(spread[0])} to ${fmt(spread[1])}` : "n/a"} | ${medianRange(row.p50s)} | ${medianRange(row.means, fmt3)} | ${medianRange(row.overA, pct)} | ${medianRange(row.overB, pct)} | ${medianRange(row.taskPerFrame, fmt3)} | ${medianRange(row.calls, (value) => fmt(value, 0))} | ${medianRange(row.triangles, (value) => fmt(value, 0))} | ${fmt(row.load)} | ${row.consoleErrors}${row.firstError ? `: ${String(row.firstError).replace(/\|/g, "/").slice(0, 80)}` : ""} |`);
      }
    }
    lines.push("", "## Paired differences", "", "Each later variant minus the first variant, pair by pair in run order: median [smallest to largest]. With one build under two names this is the noise floor.", "",
      "| File | Variant minus reference | Pairs | p95 difference (ms) | As a share of the reference p95 | p50 difference (ms) | Mean-interval difference (ms) | As a share of the reference mean | Main-thread difference (ms/frame) |", "|---|---|---|---|---|---|---|---|---|");
    for (const { name, document } of series) {
      for (const row of pairedDifferences(document)) {
        const share = (values, base) => (base ? medianRange(values.map((value) => value / base), pct) : "n/a");
        lines.push(`| ${name} | ${row.name} - ${row.reference} | ${row.pairs} | ${medianRange(row.p95)} | ${share(row.p95, row.referenceMedianP95)} | ${medianRange(row.p50)} | ${medianRange(row.mean, fmt3)} | ${share(row.mean, row.referenceMedianMean)} | ${medianRange(row.task, fmt3)} |`);
      }
    }
    lines.push("", "## Selection and hover", "", "Kind run: an orbit with the left button down. Kind hover: the pointer sweeps the canvas with no button pressed, and the frame and main-thread columns above describe that sweep. Selected is the count the product's diagnostics surface reported before the measurement. A pointer event is a pointermove the page dispatched in the window; the browser may merge moves that arrive within one frame. With select none the count reported is the product's start-up selection (the project).", "",
      "| File | Kind | Variant | Select | Selected (reported) | Selection took (ms) | Pointer events in window | Share received by the canvas | Main thread (ms per pointer event) | Script (ms per pointer event) | Product submissions in span | Camera sequence change |", "|---|---|---|---|---|---|---|---|---|---|---|---|");
    for (const { name, document } of series) {
      const isHover = document.kind === "hover-series";
      for (const variant of [...new Set(document.runs.map((run) => run.variant))]) {
        const good = document.runs.filter((run) => run.variant === variant && !run.error);
        const whole = (value) => fmt(value, 0);
        const motion = (run) => (isHover ? run.hover : run.orbit);
        lines.push(`| ${name} | ${isHover ? "hover" : "run"} | ${variant} | ${document.arguments.select ?? "none"} | ${medianRange(good.map((run) => run.selection?.reported?.count ?? null), whole)} | ${medianRange(good.map((run) => run.selection?.tookMs ?? null), whole)} | ${isHover ? medianRange(good.map((run) => run.hover?.pointerEvents?.eventsInWindow ?? null), whole) : "n/a"} | ${isHover ? medianRange(good.map((run) => run.hover?.pointerEvents?.fractionReceivedByCanvas ?? null), pct) : "n/a"} | ${isHover ? medianRange(good.map((run) => run.mainThread?.taskMsPerPointerEvent ?? null), fmt3) : "n/a"} | ${isHover ? medianRange(good.map((run) => run.mainThread?.scriptMsPerPointerEvent ?? null), fmt3) : "n/a"} | ${medianRange(good.map((run) => motion(run)?.productSubmissionsInSpan ?? null), whole)} | ${medianRange(good.map((run) => motion(run)?.cameraSequenceDelta ?? null), whole)} |`);
      }
    }
    lines.push("");
  }
  if (pairsDocuments.length) {
    lines.push("## Halo pairs (simplified rule, no casing)", "", "A simplified form of the rule in the lane's proposal P1, section 4.4: no predicted band and nothing exclusive to the winner, so every newly Selection-coloured pixel in the region counts, whatever drew it. A sample passes when the image changed and at least 4 pairs exist. No cue seen: the image did not change or no pixel newly qualified. No ground beside: pixels newly qualified and none has ground within 2 CSS px. Expected winners are the sample file's nominal ones; where the camera differs they need not apply.", "",
      "| File | Describes | Pipes | Mode | Theme | DPR | Canvas (CSS px) | Build index.html SHA-256 | Samples | Measured | Passes | Fail: no cue seen | Fail: no ground beside | Fail: too few pairs | Not attempted / failed | Selected = nominal expected | Selected = anchor | Selected = neither | Nothing selected | Expected winners apply (camera) | Median new px | Median pairs | Median inner px | Median share over figure |", "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|");
    for (const { name, document } of pairsDocuments) {
      const a = document.arguments;
      const t = document.totals;
      if (document.error) { lines.push(`| ${name} | failed: ${String(document.error).replace(/\|/g, "/").slice(0, 120)} | ${a.pipes} | ${a.mode} | ${a.theme} | ${a.dpr} | | | | | | | | | | | | | | | | | | |`); continue; }
      lines.push(`| ${name} | ${String(document.describes ?? "not stated").replace(/\|/g, "/")} | ${a.pipes} | ${a.mode} | ${a.theme} | ${a.dpr} | ${fmt(document.canvas.cssWidth, 0)} x ${fmt(document.canvas.cssHeight, 0)} | ${document.buildIdentity.indexHtmlSha256} | ${t.samples} | ${t.measured} | ${t.passesWithNoCasing} | ${t.failures["no-cue-seen"]} | ${t.failures["no-ground-beside"]} | ${t.failures["too-few-pairs"]} | ${t.notAttempted} / ${t.failed} | ${t.selectedIsNominalExpected} | ${t.selectedIsAnchor} | ${t.selectedIsNeither} | ${t.nothingSelected} | ${document.camera.expectedWinnersApply ? "yes" : `no (camera ${fmt(document.camera.positionDistanceFromNominal)} from nominal)`} | ${fmt(t.medianNewlyQualifying, 0)} | ${fmt(t.medianPairs, 0)} | ${fmt(t.medianNewlyInner, 0)} | ${pct(t.medianShareOverFigure)} |`);
    }
    lines.push("");
  }
  if (resources.length) {
    lines.push("## Resources", "", "| File | Pipes | Steps | live identical before and after | Round trips with identical live | created - disposed = live throughout | Settle check: frame pending / anything drawn / reported settled | Orbit control: frame pending / anything drawn / reported settled | Console errors |", "|---|---|---|---|---|---|---|---|---|");
    for (const { name, document } of resources) {
      if (document.error) { lines.push(`| ${name} | ${document.arguments.pipes} | failed: ${String(document.error).replace(/\|/g, "/").slice(0, 120)} | | | | | | |`); continue; }
      const word = (value) => (value ? "yes" : "no");
      const s = document.settleCheck;
      const o = document.orbitControl;
      const trips = document.ledger.roundTrips;
      lines.push(`| ${name} | ${document.arguments.pipes} | ${document.steps.length} | ${word(document.ledger.liveIdenticalBeforeAndAfter)} | ${trips.filter((trip) => trip.liveIdentical).length} of ${trips.length} | ${word(document.ledger.createdMinusDisposedEqualsLiveThroughout)} | ${word(s.animationFramePending)} / ${word(s.anythingDrawn)} (${s.drawCalls} draws) / ${word(s.settled)} | ${word(o.animationFramePending)} / ${word(o.anythingDrawn)} (${o.drawCalls} draws) / ${word(o.settled)} | ${(document.console?.errorCount ?? 0) + (document.console?.pageErrorCount ?? 0)} |`);
    }
    lines.push("");
  }
  return lines.join("\n");
}

function readDocuments(files) {
  if (!files.length) throw new UsageError("summarize needs at least one file");
  return files.map((file) => {
    const document = JSON.parse(readFileSync(file, "utf8"));
    if (document.tool !== TOOL) throw new Error(`${path.basename(file)} is not an output of this tool`);
    if (document.formatVersion !== FORMAT_VERSION) throw new Error(`${path.basename(file)} has format version ${document.formatVersion}; this tool reads ${FORMAT_VERSION}`);
    return { name: path.basename(file), document };
  });
}

// ---------------------------------------------------------------------------------------------------
// self-test
// ---------------------------------------------------------------------------------------------------

function shapeProblems(document, text, expectedRuns, kind = "orbit") {
  const hover = kind === "hover";
  const problems = [];
  const need = (condition, message) => { if (!condition) problems.push(message); };
  const number = (value) => typeof value === "number" && Number.isFinite(value);
  need(document.tool === TOOL, "tool name");
  need(document.formatVersion === FORMAT_VERSION, "format version");
  need(document.kind === (hover ? "hover-series" : "run-series"), "kind");
  need(Array.isArray(document.arguments?.argv) && document.arguments.argv[0] === (hover ? "hover" : "run"), "argument list");
  const identityOk = (identity) => /^[0-9a-f]{64}$/.test(identity?.indexHtmlSha256 ?? "") && Array.isArray(identity?.assets) && identity.assets.length > 0 &&
    identity.assets.every((asset) => typeof asset.name === "string" && Number.isInteger(asset.bytes));
  need(Array.isArray(document.variants) && document.variants.length > 0 && document.variants.every((variant) => typeof variant.name === "string" && typeof variant.query === "string" && identityOk(variant.buildIdentity)), "build identity of every variant: index.html hash, asset names and sizes");
  need(document.variantsShareOneBuildIdentity ? identityOk(document.buildIdentity) : document.buildIdentity === null, "series build identity: present when shared, null when the variants differ");
  need(/^[0-9a-f]{64}$/.test(document.fixture?.sha256 ?? ""), "fixture hash");
  need(Array.isArray(document.runs) && document.runs.length === expectedRuns, "run count");
  for (const run of document.runs ?? []) {
    const label = `run ${run.index + 1}`;
    need(run.error === null, `${label}: ${run.error}`);
    if (run.error) continue;
    need([run.loadAverageBefore, run.loadAverageAfter].every((load) => Array.isArray(load) && load.length === 3 && load.every(number)), `${label}: load averages`);
    need(typeof run.browser?.version === "string" && run.browser.version.length > 0, `${label}: browser version`);
    need(typeof run.browser?.webgl?.renderer === "string" && run.browser.webgl.renderer.length > 0, `${label}: WebGL renderer string`);
    const m = run.frames?.measure;
    need(m && Number.isInteger(m.intervals) && m.intervals > 10, `${label}: interval count`);
    need(m && [m.meanMs, m.p50Ms, m.p95Ms, m.p99Ms, m.maxMs, m.fractionOver16_7Ms, m.fractionOver33_3Ms].every(number), `${label}: interval statistics`);
    need(m && m.p50Ms <= m.p95Ms && m.p95Ms <= m.p99Ms && m.p99Ms <= m.maxMs, `${label}: percentile order`);
    need(m && m.fractionOver33_3Ms <= m.fractionOver16_7Ms && m.fractionOver16_7Ms <= 1, `${label}: fractions`);
    need(m && Array.isArray(m.longIntervals) && m.longIntervals.length === Math.round(m.fractionOver33_3Ms * m.intervals), `${label}: long-interval list matches the fraction over ${LIMIT_B_MS} ms`);
    need(run.frames.overflow === false, `${label}: recorder overflow`);
    need(run.mainThread?.status === "measured" && [run.mainThread.taskMsPerFrame, run.mainThread.scriptMsPerFrame, run.mainThread.layoutMsPerFrame, run.mainThread.styleMsPerFrame].every(number), `${label}: main-thread cost`);
    const viewport = run.diagnostics?.beforeOrbit?.viewport;
    need(viewport && ["calls", "triangles", "lines", "points", "geometries", "textures"].every((key) => Number.isInteger(viewport.rendererInfo?.[key])), `${label}: renderer information`);
    need(viewport && ["live", "created", "disposed"].every((key) => viewport.owned?.[key] && typeof viewport.owned[key] === "object"), `${label}: ownership ledger`);
    need(viewport && [viewport.canvas?.cssWidth, viewport.canvas?.cssHeight, viewport.canvas?.bufferWidth, viewport.canvas?.bufferHeight, viewport.canvas?.pixelRatio].every(number), `${label}: canvas sizes`);
    need(viewport && typeof viewport.geometry?.mode === "string" && typeof viewport.geometry?.odStatus === "string", `${label}: geometry mode and OD status`);
    need(viewport && typeof viewport.labels?.enabled === "boolean" && Number.isInteger(viewport.labels?.renderedCount), `${label}: label state`);
    need(typeof run.diagnostics?.beforeOrbit?.model?.identityHash === "string" && run.diagnostics.beforeOrbit.model.projectId === document.fixture.projectId, `${label}: model identity`);
    if (hover) {
      need(run.hover?.cameraSequenceDelta === 0, `${label}: the hover sweep left the camera alone`);
      need(run.hover?.modelGenerationUnchanged === true && run.hover?.geometryUnchanged === true && run.hover?.selectionUnchanged === true, `${label}: model, geometry and selection unchanged by the hover sweep`);
      need(Number.isInteger(run.hover?.pointerEvents?.eventsInWindow) && run.hover.pointerEvents.eventsInWindow > 10 && run.hover.pointerEvents.overflow === false, `${label}: pointer events recorded`);
      need(number(run.hover?.pointerEvents?.fractionReceivedByCanvas) && run.hover.pointerEvents.fractionReceivedByCanvas >= 0.9, `${label}: the canvas received the sweep`);
      need(number(run.mainThread?.taskMsPerPointerEvent) && number(run.mainThread?.scriptMsPerPointerEvent), `${label}: main-thread cost per pointer event`);
      need(number(run.hover?.productSubmissionsInSpan) && run.hover.productSubmissionsInSpan > 0, `${label}: the hover sweep made the product draw`);
    } else {
      need(Number.isInteger(run.orbit?.cameraSequenceDelta) && run.orbit.cameraSequenceDelta > 0, `${label}: the orbit moved the camera`);
      need(run.orbit?.modelGenerationUnchanged === true && run.orbit?.geometryUnchanged === true, `${label}: model and geometry unchanged by the orbit`);
      need(run.orbit?.selectionUnchanged === true, `${label}: selection unchanged by the orbit`);
    }
    const wantedSelection = { none: null, one: 1, hundred: 100, all: document.fixture.pipeSegments }[document.arguments.select ?? "none"];
    need(run.selection && (wantedSelection === null ? run.selection.requested === "none" : run.selection.reported?.count === wantedSelection && run.selection.reported.types?.pipe === wantedSelection), `${label}: the selection asked for is the selection reported`);
    need(wantedSelection === null || viewportSelectionCount(run) === wantedSelection, `${label}: the selection is still reported at the start of the measurement`);
    need(run.pointer?.movesSent > 0 && run.pointer.movesFailed === 0, `${label}: pointer moves`);
    need(run.console && Number.isInteger(run.console.errorCount) && Array.isArray(run.console.errors) && Array.isArray(run.console.pageErrors), `${label}: console record`);
    const variant = (document.variants ?? []).find((entry) => entry.name === run.variant);
    need(variant && run.served && run.served.dist === variant.dist && run.served.buildIndexHtmlSha256 === variant.buildIdentity.indexHtmlSha256 && run.served.requests > 0, `${label}: served from its own variant's build`);
  }
  need(!/(\/Users\/|\/home\/|\/private\/|\/var\/folders\/|[A-Za-z]:\\\\)/.test(text), "no machine path in the output text");
  return problems;
}

function viewportSelectionCount(run) {
  return run.diagnostics?.beforeOrbit?.viewport?.selection?.count ?? null;
}

function arithmeticChecks() {
  const failures = [];
  const check = (name, actual, expected) => {
    const same = typeof expected === "number" && typeof actual === "number" ? Math.abs(actual - expected) < 1e-9 : JSON.stringify(actual) === JSON.stringify(expected);
    if (!same) failures.push(`${name}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`);
  };
  const oneToTwenty = Array.from({ length: 20 }, (_, i) => i + 1);
  const oneToHundred = Array.from({ length: 100 }, (_, i) => i + 1);
  check("p50 of 1..20", percentileNearestRank(oneToTwenty, 50), 10);
  check("p95 of 1..20", percentileNearestRank(oneToTwenty, 95), 19);
  check("p99 of 1..100", percentileNearestRank(oneToHundred, 99), 99);
  check("p100 of 1..100", percentileNearestRank(oneToHundred, 100), 100);
  check("p95 of one value", percentileNearestRank([5], 95), 5);
  check("p50 of unsorted three", percentileNearestRank([3, 1, 2], 50), 2);
  check("p95 of three", percentileNearestRank([12.5, 11, 14], 95), 14);
  check("percentile of nothing", percentileNearestRank([], 95), null);
  check("median of four", median([4, 1, 3, 2]), 2.5);
  check("median of three", median([9, 1, 5]), 5);
  check("mean", mean([1, 2, 3, 6]), 3);
  check("range", range([3, -1, 7]), [-1, 7]);

  const gaps = [10, 10, 20, 40, 10, 10, 50, 10, 10, 10];
  const ts = new Float64Array(gaps.length + 1);
  ts[0] = 1000;
  gaps.forEach((gap, i) => { ts[i + 1] = ts[i] + gap; });
  const all = frameStats(ts, ts.length, 1000, 1180, LIMIT_A_MS, LIMIT_B_MS, 100, 10);
  check("frames in window", all.frames, 11);
  check("interval count", all.intervals, 10);
  check("mean interval", all.meanMs, 18);
  check("p50 interval", all.p50Ms, 10);
  check("p95 interval", all.p95Ms, 50);
  check("p99 interval", all.p99Ms, 50);
  check("max interval", all.maxMs, 50);
  check("fraction over 16.7 ms", all.fractionOver16_7Ms, 0.3);
  check("fraction over 33.3 ms", all.fractionOver33_3Ms, 0.2);
  check("fraction of missed refreshes at a 10 ms period", all.fractionMissedRefresh, 0.3);
  check("long-interval offsets", all.longIntervals, [[80, 40], [150, 50]]);
  check("span", all.spanMs, 180);
  const part = frameStats(ts, ts.length, 1020, 1100, LIMIT_A_MS, LIMIT_B_MS, 100, null);
  check("windowed frames", part.frames, 5);
  check("windowed mean", part.meanMs, 20);
  check("windowed long-interval offsets", part.longIntervals, [[60, 40]]);
  check("windowed missed refreshes without a period", part.fractionMissedRefresh, null);
  const capped = frameStats(ts, ts.length, 1000, 1180, LIMIT_A_MS, LIMIT_B_MS, 1, null);
  check("long-interval list cap", [capped.longIntervals.length, capped.longIntervalsTruncated], [1, true]);
  const empty = frameStats(ts, ts.length, 5000, 6000, LIMIT_A_MS, LIMIT_B_MS, 100, null);
  check("empty window", [empty.frames, empty.intervals, empty.p95Ms], [0, 0, null]);

  check("variants", parseVariants("off=;band=canvasProbe=edge-band"), [{ name: "off", dist: null, query: "" }, { name: "band", dist: null, query: "canvasProbe=edge-band" }]);
  check("variants with builds", parseVariants("base@builds/base=;line@builds/line=canvasProbe=1"), [{ name: "base", dist: "builds/base", query: "" }, { name: "line", dist: "builds/line", query: "canvasProbe=1" }]);
  check("variant with a build and no query", parseVariants("base@builds/base"), [{ name: "base", dist: "builds/base", query: "" }]);
  check("an @ inside a query names no build", parseVariants("a=q@r"), [{ name: "a", dist: null, query: "q@r" }]);
  let emptyBuildRefused = false;
  try { parseVariants("a@=x"); } catch (error) { emptyBuildRefused = error instanceof UsageError; }
  check("an empty build directory is refused", emptyBuildRefused, true);
  check("interleaved order", interleave(parseVariants("off=;band=x=1"), 3).map((item) => item.variant), ["off", "band", "off", "band", "off", "band"]);
  const canvas = { width: 800, height: 600 };
  check("path starts at the press point", pathOffset(0, canvas), { x: 0, y: 0 });
  check("path x extreme", round(pathOffset(ORBIT_PATH.xPeriodMs / 4, canvas).x, 6), round(ORBIT_PATH.xAmplitudeOfCanvasWidth * 800, 6));
  check("path closes", [round(pathOffset(ORBIT_PATH.xPeriodMs, canvas).x, 6), round(pathOffset(ORBIT_PATH.xPeriodMs, canvas).y, 6)].map((value) => Math.abs(value)), [0, 0]);

  const scrub = buildScrubber([["{DIST}", path.join(os.tmpdir(), "some-dist")]]);
  check("scrub: dist", scrub(`open ${path.join(os.tmpdir(), "some-dist", "index.html")}`), `open {DIST}${path.sep}index.html`);
  check("scrub: home", scrub(path.join(os.homedir(), "x", "y.json")).startsWith("{"), true);
  check("reduce: outside", reducePath(path.join(os.tmpdir(), "probe-dist")), "{OUTSIDE_REPOSITORY}/probe-dist");
  check("reduce: inside", reducePath(path.join(requireWorkingRoot(), "apps", "desktop")), "{WORKING_ROOT}/apps/desktop");

  // Additions of slice C2: --select, the hover path, and the pure image functions of "pairs" on synthetic images.
  const runFlags = (extra) => parseFlags(["--pipes", "1000", "--mode", "schematic", "--labels", "off", "--theme", "light", "--dpr", "1", "--window", "1440x900", "--pacing", "vsync", "--variants", "a=", "--pairs", "1", "--out", "x.json", ...extra], SPECS.run).options;
  check("--select defaults to none", runFlags([]).select, "none");
  check("--select all", runFlags(["--select", "all"]).select, "all");
  let badSelectRefused = false;
  try { runFlags(["--select", "some"]); } catch (error) { badSelectRefused = error instanceof UsageError; }
  check("an unknown --select is refused", badSelectRefused, true);
  check("hover takes the options of run", parseFlags(["--pipes", "10000", "--mode", "actual-od", "--labels", "off", "--theme", "dark", "--dpr", "2", "--window", "1440x900", "--pacing", "uncapped", "--variants", "a=", "--pairs", "1", "--select", "one", "--out", "x.json"], SPECS.hover).options.select, "one");
  check("hover path starts at the centre", hoverOffset(0, canvas), { x: 0, y: 0 });
  check("hover path closes", [round(hoverOffset(HOVER_PATH.xPeriodMs, canvas).x, 6), round(hoverOffset(HOVER_PATH.xPeriodMs, canvas).y, 6)].map((value) => Math.abs(value)), [0, 0]);
  check("hover path x extreme", round(hoverOffset(HOVER_PATH.xPeriodMs / 4, canvas).x, 6), round(HOVER_PATH.xAmplitudeOfCanvasWidth * 800, 6));
  check("hover sends a move at least every 8 ms", MOVE_PERIOD_MS <= 8, true);
  check("sample range", parseSampleRange("3-17", "samples"), { from: 3, to: 17 });
  check("contrast of black on white", round(contrastRatio([0, 0, 0], [255, 255, 255]), 2), 21);
  check("contrast of a colour with itself", contrastRatio([16, 109, 206], [16, 109, 206]), 1);
  check("hex colour", parseHexColour("#106dce"), [16, 109, 206]);

  const SEL = [16, 109, 206]; const GROUND = [235, 237, 239]; const TUBE = [166, 171, 177]; const HOVER = [93, 148, 218];
  const paint = (size, colourAt) => {
    const image = new Uint8Array(size * size * 4);
    for (let y = 0; y < size; y++) for (let x = 0; x < size; x++) { const c = colourAt(x, y); image.set([c[0], c[1], c[2], 255], (y * size + x) * 4); }
    return image;
  };
  const radius = (x, y, size) => Math.hypot(x + 0.5 - size / 2, y + 0.5 - size / 2);
  const images = (size, scale) => ({
    discOnGround: paint(size, (x, y) => (radius(x, y, size) <= 6 * scale ? TUBE : GROUND)),
    ringOnGround: paint(size, (x, y) => { const r = radius(x, y, size); return r <= 6 * scale ? TUBE : r <= 8 * scale ? SEL : GROUND; }),
    hoverRingOnGround: paint(size, (x, y) => { const r = radius(x, y, size); return r <= 6 * scale ? TUBE : r <= 8 * scale ? HOVER : GROUND; }),
    tubeField: paint(size, () => TUBE),
    ringOverTubeField: paint(size, (x, y) => { const r = radius(x, y, size); return r > 6 * scale && r <= 8 * scale ? SEL : TUBE; }),
    bandOnGround: paint(size, (x) => (Math.abs(x + 0.5 - size / 2) <= 5 * scale ? TUBE : GROUND)),
    bandRecoloured: paint(size, (x) => (Math.abs(x + 0.5 - size / 2) <= 5 * scale ? SEL : GROUND))
  });
  const one = images(48, 1);
  const analyse = (before, after, size = 48, ratio = 1) => haloPairAnalysis(before, after, size, size, ratio, SEL, GROUND);
  const ring = analyse(one.discOnGround, one.ringOnGround);
  check("ring on ground: passes", [ring.passesWithNoCasing, ring.failureClass, ring.imageChanged], [true, null, true]);
  check("ring on ground: every new pixel lay over ground and none is inner", [ring.newlyQualifying > 50, ring.newlyOverGround === ring.newlyQualifying, ring.newlyOverFigure, ring.newlyInner], [true, true, 0, 0]);
  check("ring on ground: pairs never exceed new pixels or ground pixels", ring.pairs >= 4 && ring.pairs <= ring.newlyQualifying && ring.pairs <= ring.groundPixelsAfter, true);
  const covered = analyse(one.tubeField, one.ringOverTubeField);
  check("ring wholly over a tube-coloured field: fails for want of ground", [covered.passesWithNoCasing, covered.failureClass, covered.newlyQualifying > 50, covered.newlyWithGroundInReach, covered.pairs, covered.groundPixelsAfter], [false, "no-ground-beside", true, 0, 0, 0]);
  const body = analyse(one.bandOnGround, one.bandRecoloured);
  check("recoloured body with ground beside: the simplified rule passes it", [body.passesWithNoCasing, body.failureClass], [true, null]);
  check("recoloured body: told from a halo by its inner pixels and by lying over the figure", [body.newlyInner > 0, body.newlyOverGround, body.newlyOverFigure === body.newlyQualifying], [true, 0, true]);
  const same = analyse(one.ringOnGround, one.ringOnGround);
  check("unchanged image: fails, no cue seen", [same.passesWithNoCasing, same.failureClass, same.imageChanged, same.newlyQualifying], [false, "no-cue-seen", false, 0]);
  const hoverRing = analyse(one.discOnGround, one.hoverRingOnGround);
  check("ring in the hover colour: fails, no cue seen", [hoverRing.passesWithNoCasing, hoverRing.failureClass, hoverRing.imageChanged, hoverRing.newlyQualifying], [false, "no-cue-seen", true, 0]);
  const two = images(96, 2);
  const ringTwo = analyse(two.discOnGround, two.ringOnGround, 96, 2);
  check("ring on ground at pixel ratio 2: passes, none inner", [ringTwo.passesWithNoCasing, ringTwo.newlyInner], [true, 0]);
  const lonelyGround = paint(48, (x, y) => (x === 24 && y === 24 ? GROUND : SEL));
  const lonely = analyse(one.tubeField, lonelyGround);
  check("each ground pixel is used once", [lonely.pairs, lonely.passesWithNoCasing, lonely.failureClass, lonely.newlyWithGroundInReach], [1, false, "too-few-pairs", 12]);
  const lowContrast = paint(48, (x) => (x < 24 ? [60, 150, 250] : [200, 200, 200]));
  const dim = haloPairAnalysis(one.tubeField, lowContrast, 48, 48, 1, SEL, [235, 237, 239]);
  check("a pair under 3:1 is not counted", [round(contrastRatio([60, 150, 250], [200, 200, 200]), 1) < 3, dim.pairs, dim.pairsIgnoringContrast > 0, dim.failureClass], [true, 0, true, "too-few-pairs"]);
  const decoded = decodePng(encodePng(48, 48, one.ringOnGround));
  check("PNG round trip", [decoded.width, decoded.height, decoded.opaque, Buffer.from(decoded.rgba).equals(Buffer.from(one.ringOnGround))], [48, 48, true, true]);
  const totals = pairsTotals([{ status: "measured", ...ring, selected: { ref: { type: "pipe", id: "p" } }, selectedIsNominalExpected: true, selectedIsAnchor: true }, { status: "measured", ...covered, selected: { ref: null }, selectedIsNominalExpected: false, selectedIsAnchor: false }, { status: "not-attempted" }]);
  check("pairs totals", [totals.samples, totals.measured, totals.notAttempted, totals.passesWithNoCasing, totals.failures["no-ground-beside"], totals.nothingSelected, totals.selectedIsNominalExpected], [3, 2, 1, 1, 1, 1, 1]);

  const synthetic = (variant, pair, p95, meanMs, task) => ({ variant, pair, error: null, frames: { measure: { p95Ms: p95, p50Ms: meanMs, meanMs, fractionOver16_7Ms: 0, fractionOver33_3Ms: 0 } }, mainThread: { taskMsPerFrame: task }, loadAverageBefore: [1, 1, 1], loadAverageAfter: [3, 1, 1], console: { errorCount: 0, pageErrorCount: 0, errors: [], pageErrors: [] } });
  const document = { runs: [synthetic("a", 1, 10, 8, 2), synthetic("b", 1, 11, 8.5, 2.5), synthetic("a", 2, 12, 9, 2), synthetic("b", 2, 11, 8, 1.5), synthetic("a", 3, 14, 10, 2), synthetic("b", 3, 17, 12, 3)] };
  const rows = variantRows(document);
  check("summary: p50 of per-run p95", rows[0].p50OfRunP95, 12);
  check("summary: p95 of per-run p95", rows[0].p95OfRunP95, 14);
  check("summary: mean load", rows[0].load, 2);
  const paired = pairedDifferences(document)[0];
  check("summary: paired p95 differences", paired.p95, [1, -1, 3]);
  check("summary: paired mean differences", paired.mean, [0.5, -1, 2]);
  check("summary: reference median p95", paired.referenceMedianP95, 12);
  return failures;
}

async function selfTest(options, log = console.log) {
  const failures = arithmeticChecks();
  log(failures.length ? `arithmetic: ${failures.length} FAILED` : "arithmetic: all checks passed");
  for (const failure of failures) log(`  ${failure}`);
  if (failures.length) return 1;

  const dist = resolveDist(options.dist);
  const scratch = mkdtempSync(path.join(os.tmpdir(), "orbit-probe-self-test-"));
  try {
    // Build cases that need no browser: a directory without index.html, and a variant with no build at all.
    const flags = (variants, extra = []) => parseFlags(["--pipes", "1000", "--mode", "schematic", "--labels", "on", "--theme", "light", "--dpr", "1",
      "--window", "1440x900", "--pacing", "vsync", "--variants", variants, "--pairs", "1", "--warmup-ms", "500", "--measure-ms", "2000",
      "--port", String(options.port), "--out", path.join(scratch, "self-test.json"), ...extra], SPECS.run).options;
    const empty = path.join(scratch, "no-build");
    mkdirSync(empty);
    const refusal = async (runOptions) => { try { await runSeries(runOptions, () => {}); return null; } catch (error) { return error instanceof UsageError ? error.message : `unexpected: ${error?.message ?? error}`; } };
    const noIndex = await refusal(flags(`good@${dist}=;bad@${empty}=`));
    const noBuild = await refusal(flags("orphan="));
    const buildCasesOk = ACTIVE.launches === 0 && typeof noIndex === "string" && noIndex.includes('variant "bad"') && noIndex.includes("index.html") &&
      typeof noBuild === "string" && noBuild.includes('variant "orphan" has no build');
    log(buildCasesOk ? "build cases: a directory without index.html and a variant with no build are refused before any browser starts" : `build cases: FAILED (launches ${ACTIVE.launches}; ${noIndex}; ${noBuild})`);

    // Two separate copies of one build, one two-second run each, each served from its own copy.
    const copy = path.join(scratch, "second-copy");
    cpSync(dist, copy, { recursive: true });
    const out = path.join(scratch, "self-test.json");
    const document = await runSeries(flags(`first@${dist}=;second@${copy}=`), (line) => log(`  ${line}`));
    const text = readFileSync(out, "utf8");
    const written = JSON.parse(text);
    const problems = shapeProblems(written, text, 2);
    if (!(written.variantsShareOneBuildIdentity === true && written.variants.length === 2 && sameBuildIdentity(written.variants[0].buildIdentity, written.variants[1].buildIdentity))) problems.push("two copies of one build must report equal build identities");
    if (written.variants.length === 2 && written.variants[0].dist === written.variants[1].dist) problems.push("the two copies must be recorded as two directories");
    if (written.runs.some((run) => run.served?.notFound)) problems.push("a request for a file with an extension was not found in the build");
    log(problems.length ? `output shape: ${problems.length} problem(s)` : "output shape: valid; two copies of one build report equal build identities, each run served from its own copy");
    for (const problem of problems) log(`  ${problem}`);
    const table = summarize([{ name: "self-test.json", document }]);
    const tableOk = table.includes("| self-test.json | first | 1 |") && table.includes("| self-test.json | second | 1 |") && table.includes("same build identity") && table.includes("A/A series");
    log(tableOk ? "summarize: produced a row for each variant and named the shared build identity" : "summarize: FAILED to produce the expected rows");

    // Additions of slice C2, one short run each on the 1,000-pipe fixture, labels off.
    const flagsOf = (spec, extra) => parseFlags(["--dist", dist, "--pipes", "1000", "--mode", "schematic", "--labels", "off", "--theme", "light", "--dpr", "1",
      "--window", "1440x900", "--pacing", "vsync", "--variants", "only=", "--pairs", "1", "--warmup-ms", "300", "--measure-ms", "1500", "--port", String(options.port), ...extra], spec).options;
    const c2Problems = [];
    for (const [kind, select] of [["orbit", "hundred"], ["orbit", "all"], ["hover", "one"]]) {
      const file = path.join(scratch, `c2-${kind}-${select}.json`);
      const series = await runSeries(flagsOf(kind === "hover" ? SPECS.hover : SPECS.run, ["--select", select, "--out", file]), (line) => log(`  ${line}`), kind);
      const seriesText = readFileSync(file, "utf8");
      for (const problem of shapeProblems(JSON.parse(seriesText), seriesText, 1, kind)) c2Problems.push(`${kind} --select ${select}: ${problem}`);
      const seriesTable = summarize([{ name: "c2.json", document: series }]);
      if (!seriesTable.includes(`| c2.json | ${kind === "hover" ? "hover" : "run"} | only | ${select} | ${{ one: 1, hundred: 100, all: 1000 }[select]} |`)) c2Problems.push(`${kind} --select ${select}: the summary lacks the selection row`);
    }
    const pairsFile = path.join(scratch, "c2-pairs.json");
    const pairsDocument = await runPairs(parseFlags(["--dist", dist, "--pipes", "1000", "--mode", "schematic", "--theme", "light", "--samples", "1-4", "--note", "self-test", "--port", String(options.port), "--out", pairsFile], SPECS.pairs).options, (line) => log(`  ${line}`));
    const pairsText = readFileSync(pairsFile, "utf8");
    const pairsWritten = JSON.parse(pairsText);
    if (pairsWritten.error) c2Problems.push(`pairs: ${pairsWritten.error}`);
    if (pairsWritten.kind !== "pairs" || pairsWritten.samples?.length !== 4 || pairsWritten.totals?.samples !== 4) c2Problems.push("pairs: four samples expected");
    for (const entry of pairsWritten.samples ?? []) {
      if (entry.status === "measured" && !(Number.isInteger(entry.pairs) && Number.isInteger(entry.newlyQualifying) && typeof entry.passesWithNoCasing === "boolean" && entry.cameraUnchanged === true && entry.imagesOpaque === true)) c2Problems.push(`pairs: sample ${entry.sample} lacks its counts`);
      if (entry.status === "failed") c2Problems.push(`pairs: sample ${entry.sample} failed: ${entry.reason}`);
    }
    if (!(pairsWritten.totals?.measured > 0)) c2Problems.push("pairs: no sample was measured");
    if (!/^[0-9a-f]{64}$/.test(pairsWritten.tokens?.sha256 ?? "") || pairsWritten.sampleFiles?.length !== 2) c2Problems.push("pairs: token file and sample file identities");
    if (/(\/Users\/|\/home\/|\/private\/|\/var\/folders\/|[A-Za-z]:\\\\)/.test(pairsText)) c2Problems.push("pairs: a machine path is in the output text");
    if (!summarize([{ name: "c2-pairs.json", document: pairsDocument }]).includes("| c2-pairs.json | self-test | 1000 |")) c2Problems.push("pairs: the summary lacks its row");
    log(c2Problems.length ? `select, hover and pairs: ${c2Problems.length} problem(s)` : "select, hover and pairs: run --select hundred and all, hover --select one and pairs on four samples gave valid output; the counts reported are the counts asked for");
    for (const problem of c2Problems) log(`  ${problem}`);
    const passed = buildCasesOk && problems.length === 0 && tableOk && c2Problems.length === 0;
    log(passed ? "self-test: PASS" : "self-test: FAIL");
    return passed ? 0 : 1;
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}

// ---------------------------------------------------------------------------------------------------
// Entry.
// ---------------------------------------------------------------------------------------------------

async function shutdown(signal) {
  try { await closeBrowser(ACTIVE.browser); } catch { /* nothing to close */ }
  try { await ACTIVE.server?.close(); } catch { /* nothing to close */ }
  process.exit(signal === "SIGINT" ? 130 : 143);
}

async function main() {
  const [subcommand, ...rest] = process.argv.slice(2);
  if (!subcommand || subcommand === "--help" || subcommand === "-h" || subcommand === "help") { process.stdout.write(USAGE); return 0; }
  if (!(subcommand in SPECS)) throw new UsageError(`unknown subcommand "${subcommand}"`);
  const { options, positionals } = parseFlags(rest, SPECS[subcommand]);
  if (subcommand !== "summarize" && positionals.length) throw new UsageError(`unexpected argument "${positionals[0]}"`);
  process.on("SIGINT", () => { shutdown("SIGINT"); });
  process.on("SIGTERM", () => { shutdown("SIGTERM"); });
  if (subcommand === "run" || subcommand === "hover") {
    const document = await runSeries(options, console.log, subcommand === "hover" ? "hover" : "orbit");
    return document.runs.some((run) => run.error) ? 1 : 0;
  }
  if (subcommand === "pairs") return (await runPairs(options)).error ? 1 : 0;
  if (subcommand === "resources") return (await runResources(options)).error ? 1 : 0;
  if (subcommand === "self-test") return selfTest(options);
  process.stdout.write(`${summarize(readDocuments(positionals))}\n`);
  return 0;
}

// The pieces are exported so that a reviewer can exercise them one by one; the command line is the interface.
export { frameStats, percentileNearestRank, median, pathOffset, parseVariants, interleave, summarize, startStaticServer, launchBrowser, closeBrowser, loadFixture, loadPlaywright, openProduct, setTheme, setGeometry, setToggle, isometricThenFit, waitSettled, findPressPoint, driveOrbit, inPageReadDiagnostics, hoverOffset, selectPipesThroughTree, haloPairAnalysis, contrastRatio, decodePng, encodePng, runPairs };

const invokedDirectly = Boolean(process.argv[1]) && import.meta.url === pathToFileURL(realpathSync(process.argv[1])).href;
if (invokedDirectly) {
  main().then((code) => { process.exitCode = code; }, (error) => {
    const scrub = buildScrubber();
    if (error instanceof UsageError) { process.stderr.write(`${scrub(error.message)}\n\n${USAGE}`); process.exitCode = 2; return; }
    process.stderr.write(`orbit_probe: ${scrub(String(error?.message ?? error).split("\n")[0])}\n`);
    process.exitCode = 1;
  });
}
