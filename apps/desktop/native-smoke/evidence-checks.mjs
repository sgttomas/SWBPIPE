// The pure parts of the evidence journeys' runner (`evidence-journeys.mjs`): each
// expectation the runner holds the launched app to is computed here, from the
// repository's governing text, the maintained fixtures and the bytes the app wrote,
// never from the app's own account of them. No function here launches, builds or
// writes anything; `evidence-checks.test.mjs` tests them with `node --test`.
import { createHash } from "node:crypto";
import os from "node:os";

export const sha256Hex = (bytes) => createHash("sha256").update(bytes).digest("hex");

// RFC 8785 JCS for JSON documents (finite numbers; ECMAScript number text), as run.mjs.
export function canonical(value) {
  if (Array.isArray(value)) return `[${value.map(canonical).join(",")}]`;
  if (value && typeof value === "object") {
    return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(",")}}`;
  }
  return JSON.stringify(value);
}
/** SHA-256 (bare hex) of the RFC 8785 text of `document`. */
export const documentSha256 = (document) => sha256Hex(Buffer.from(canonical(document), "utf8"));

/** Whether `bytes` are exactly the RFC 8785 text of the JSON they parse to. */
export function isCanonicalText(bytes) {
  try {
    return Buffer.compare(Buffer.from(canonical(JSON.parse(bytes.toString("utf8"))), "utf8"), bytes) === 0;
  } catch {
    return false;
  }
}

// ---------------------------------------------------------------- the session-lock guard

/** Whether `ioreg -n Root -d1 -a` (an XML property list) reports the console session's
 * screen locked. A window does not paint on a locked session, so no launch is made then.
 * The key is present only while a session is locked; anything but `<true/>` after it is
 * read as unlocked. */
export function screenLockedFromIoreg(plist) {
  return /<key>CGSSessionScreenIsLocked<\/key>\s*<true\s*\/>/.test(String(plist));
}

/** Whether process `pid` runs under the host's slot wrapper: an ancestor's command runs
 * `t3_slot_run.sh`, the job `t3_slot.sh` starts while it holds a slot's lock. `ps` is the
 * output of `ps -axo pid=,ppid=,command=`. The slot is then taken before the runner builds or
 * asks for the foreground, and held until the runner ends. */
export function underHostSlot(ps, pid) {
  const rows = new Map(String(ps).split("\n").map((line) => line.match(/^\s*(\d+)\s+(\d+)\s+(.*)$/)).filter(Boolean)
    .map(([, child, parent, command]) => [Number(child), { parent: Number(parent), command }]));
  const seen = new Set();
  for (let at = rows.get(pid)?.parent; at && at > 1 && !seen.has(at); at = rows.get(at)?.parent) {
    seen.add(at);
    if (/(^|\/|\s)t3_slot_run\.sh(\s|$)/.test(rows.get(at)?.command ?? "")) return true;
  }
  return false;
}

// ---------------------------------------------------------------- timeouts on a loaded host

/** A driver timeout (`TIMEOUT waiting for <label>: <n> ms elapsed, limit <n> ms, at <unix ms>`) with
 * the load average sampled nearest its moment, or null when `error` is not a timeout. `samples` are
 * `{ at, load: [1m, 5m, 15m] }`, taken while the app ran. */
export function driverTimeout(error, samples) {
  const match = /TIMEOUT waiting for (.+?): (\d+) ms elapsed, limit (\d+) ms, at (\d{13})/.exec(String(error ?? ""));
  if (!match) return null;
  const at = Number(match[4]);
  const nearest = samples.reduce((best, sample) => (best === null || Math.abs(sample.at - at) < Math.abs(best.at - at) ? sample : best), null);
  return { label: match[1], elapsed_ms: Number(match[2]), limit_ms: Number(match[3]), at, load: nearest?.load ?? null, load_sampled_at: nearest?.at ?? null };
}
/** One line for a timeout: what was waited for, how long, and the load then. */
export function timeoutText({ label, elapsed_ms, limit_ms, load, load_sampled_at, at }) {
  const shown = load ? `${load.map((value) => value.toFixed(2)).join(" / ")} (1/5/15 min${load_sampled_at !== undefined && at !== undefined ? `, sampled ${Math.round((load_sampled_at - at) / 1000)} s from the timeout` : ""})` : "not sampled";
  return `TIMEOUT waiting for ${label}: ${Math.round(elapsed_ms / 1000)} s elapsed of a ${Math.round(limit_ms / 1000)} s limit; load average ${shown}, on ${os.cpus().length} processors`;
}

// ---------------------------------------------------------------- the command line

/** The journeys --all runs, in order: the first launch of the run is the shortest one (autosave-recovery's
 * first launch: settings, one save, two edits and at most one autosave interval before it is ended), so
 * the web view's data location is decided as early in a run as it can be. */
export const ALL_JOURNEYS = ["autosave-recovery", "report-exact", "report-pressure-free"];
/** Named alone, never part of --all. */
export const SEPARATE_JOURNEYS = ["reference-system"];

/** The runner's command line: `--all` or `--journey a,b`, `--skip-build`, `--library-listing <path>`
 * (the real-Library lister; `env.SWBPIPE_LIBRARY_LISTING` stands in for it), and `--build-only` (check,
 * build and bind the app and its identity, then exit before any launch; it takes no journey and needs no
 * lister). A journey run without a lister is refused. Returns { journeys, buildOnly, skipBuild,
 * libraryListing } or { error }. */
export function parseRunnerArgs(argv, env = {}) {
  const args = new Set(argv);
  const known = new Set(["--all", "--journey", "--skip-build", "--build-only", "--library-listing"]);
  const valueOf = (flag) => { const at = argv.indexOf(flag); return at >= 0 ? argv[at + 1] : undefined; };
  const valued = new Set(["--journey", "--library-listing"].map((flag) => argv.indexOf(flag)).filter((at) => at >= 0).map((at) => at + 1));
  const unknown = argv.filter((item, index) => !known.has(item) && !valued.has(index));
  if (unknown.length > 0) return { error: `unknown arguments: ${unknown.join(" ")}` };
  const buildOnly = args.has("--build-only");
  if (buildOnly) {
    if (args.has("--all") || args.has("--journey") || args.has("--skip-build")) return { error: "--build-only builds and binds the app and takes no journey and no --skip-build" };
    return { journeys: [], buildOnly: true, skipBuild: false, libraryListing: null };
  }
  const value = valueOf("--journey");
  if (args.has("--journey") && (!value || value.startsWith("--"))) return { error: "--journey needs one or more journey names" };
  if (args.has("--all") && args.has("--journey")) return { error: "choose --all or --journey, not both" };
  const journeys = args.has("--all") ? [...ALL_JOURNEYS] : (value ?? "").split(",").filter(Boolean);
  if (journeys.length === 0 || journeys.some((name) => !ALL_JOURNEYS.includes(name) && !SEPARATE_JOURNEYS.includes(name))) {
    return { error: `choose --all, --build-only or --journey with one or more of ${ALL_JOURNEYS.join(", ")} (or ${SEPARATE_JOURNEYS.join(", ")} alone)` };
  }
  if (journeys.some((name) => SEPARATE_JOURNEYS.includes(name)) && journeys.length > 1) return { error: `${SEPARATE_JOURNEYS.join(", ")} runs alone: name it as the only journey` };
  const listing = valueOf("--library-listing");
  if (args.has("--library-listing") && (!listing || listing.startsWith("--"))) return { error: "--library-listing needs the lister's path" };
  const libraryListing = listing ?? env.SWBPIPE_LIBRARY_LISTING ?? null;
  if (!libraryListing) return { error: "not launched: no real-Library lister given (--library-listing <path> or SWBPIPE_LIBRARY_LISTING)" };
  if (!libraryListing.startsWith("/")) return { error: `the real-Library lister's path must be absolute: ${libraryListing}` };
  return { journeys, buildOnly: false, skipBuild: args.has("--skip-build"), libraryListing };
}

/** Why `target` must not be removed, or null: only a throwaway home this run created, named `home`,
 * directly inside a `swbpipe-evidence-<journey>-` folder strictly under the system temporary folder. */
export function homeRemovalRefusal(target, created, tmp) {
  const base = String(tmp).replace(/\/+$/, "");
  if (typeof target !== "string" || !target.startsWith("/") || target.split("/").includes("..")) return `not an absolute path without ..: ${target}`;
  if (!created.has(target)) return `not a home this run created: ${target}`;
  if (!target.startsWith(`${base}/`)) return `not under the system temporary folder (${base}): ${target}`;
  const parts = target.slice(base.length + 1).split("/");
  if (parts.at(-1) !== "home" || !/^swbpipe-evidence-[a-z-]+-[A-Za-z0-9]+$/.test(parts.at(-2) ?? "")) return `not a throwaway home folder: ${target}`;
  return null;
}

// ---------------------------------------------------------------- the home redirection

/** The launch environment's home: HOME for the app's own paths, CFFIXED_USER_HOME for Foundation's home
 * directory, through which the system web view places its website data and caches. */
export const homeRedirection = (home) => ({ HOME: home, CFFIXED_USER_HOME: home });

/** Why a launch environment must not be used, or null: HOME and CFFIXED_USER_HOME must both be set,
 * equal, absolute, and strictly under the system temporary folder `tmp`. */
export function homeRedirectionRefusal(env, tmp) {
  const home = env.HOME;
  const fixed = env.CFFIXED_USER_HOME;
  if (!home) return "HOME is not set";
  if (!fixed) return "CFFIXED_USER_HOME is not set";
  if (home !== fixed) return `HOME (${home}) and CFFIXED_USER_HOME (${fixed}) differ`;
  if (!home.startsWith("/") || home.split("/").includes("..")) return `HOME (${home}) is not an absolute path without ..`;
  const base = String(tmp).replace(/\/+$/, "");
  if (!base.startsWith("/") || !home.startsWith(`${base}/`)) return `HOME (${home}) is not under the system temporary folder (${base})`;
  return null;
}

/** The web view's data under a home, from a listing of paths relative to it: anything the system web view
 * keeps for an app named `name` (Library/WebKit/<name>, Library/Caches/<name>, Library/HTTPStorages/<name>,
 * Library/Cookies), and any other Library entry naming it. The app's own Application Support folder is
 * not web view data and is left out. */
export function webviewDataEntries(paths, name) {
  const own = /^Library\/Application Support\//;
  return paths.filter((entry) => !own.test(entry) && (
    new RegExp(`^Library/(WebKit|Caches|HTTPStorages)/${name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}(/|$|\\.)`).test(entry)
    || /^Library\/Cookies(\/|$)/.test(entry)
    || (entry.startsWith("Library/") && entry.split("/").includes(name))));
}

// ---------------------------------------------------------------- governing text

/** The text of PRD §19.3's required notice: the first `text` fence after the heading. */
export function prdRequiredNotice(prd) {
  const at = prd.search(/^###\s+19\.3\s+Required Notice\s*$/m);
  if (at < 0) throw new Error("docs/PRD.md has no §19.3 Required Notice heading");
  const rest = prd.slice(at);
  const body = rest.indexOf("\n") + 1;
  const next = rest.slice(body).search(/^#{1,3}\s/m);
  const section = next < 0 ? rest : rest.slice(0, body + next);
  const fence = section.match(/```text\n([\s\S]*?)\n```/);
  if (!fence) throw new Error("docs/PRD.md §19.3 has no text fence");
  return normalizeSpace(fence[1]);
}
export const normalizeSpace = (text) => String(text).replace(/\s+/g, " ").trim();


// ---------------------------------------------------------------- the package container

const CRC_TABLE = (() => {
  const table = new Uint32Array(256);
  for (let n = 0; n < 256; n += 1) {
    let c = n;
    for (let k = 0; k < 8; k += 1) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    table[n] = c >>> 0;
  }
  return table;
})();
export function crc32(bytes) {
  let c = 0xffffffff;
  for (const byte of bytes) c = CRC_TABLE[(c ^ byte) & 0xff] ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
}

/** The members of a stored (uncompressed) ZIP archive, as the report package writes it,
 * read by this runner and not by the product: the end record, the central directory and
 * each local header, checked against each other, with each member's CRC-32 recomputed.
 * Throws on anything else: a compressed member, a data descriptor, an extra field, a
 * comment, a mismatch or a duplicate name. */
export function readStoredZip(buffer) {
  const bytes = Buffer.from(buffer);
  const eocd = bytes.length - 22;
  if (eocd < 0 || bytes.readUInt32LE(eocd) !== 0x06054b50) throw new Error("no end-of-central-directory record at the end (a comment or another archive form)");
  if (bytes.readUInt16LE(eocd + 20) !== 0) throw new Error("the archive has a comment");
  if (bytes.readUInt16LE(eocd + 4) !== 0 || bytes.readUInt16LE(eocd + 6) !== 0) throw new Error("a multi-disk archive");
  const count = bytes.readUInt16LE(eocd + 10);
  if (bytes.readUInt16LE(eocd + 8) !== count) throw new Error("the entry counts differ");
  const centralSize = bytes.readUInt32LE(eocd + 12);
  const centralOffset = bytes.readUInt32LE(eocd + 16);
  if (centralOffset + centralSize !== eocd) throw new Error("the central directory does not end at the end record");
  const members = [];
  const names = new Set();
  let at = centralOffset;
  let expectedOffset = 0;
  for (let index = 0; index < count; index += 1) {
    if (bytes.readUInt32LE(at) !== 0x02014b50) throw new Error(`central entry ${index}: bad signature`);
    const method = bytes.readUInt16LE(at + 10);
    const flags = bytes.readUInt16LE(at + 8);
    const crc = bytes.readUInt32LE(at + 16);
    const compressed = bytes.readUInt32LE(at + 20);
    const size = bytes.readUInt32LE(at + 24);
    const nameLength = bytes.readUInt16LE(at + 28);
    const extraLength = bytes.readUInt16LE(at + 30);
    const commentLength = bytes.readUInt16LE(at + 32);
    const offset = bytes.readUInt32LE(at + 42);
    const name = bytes.subarray(at + 46, at + 46 + nameLength).toString("latin1");
    if (method !== 0 || flags !== 0 || compressed !== size) throw new Error(`${name}: not a stored member without flags`);
    if (extraLength !== 0 || commentLength !== 0) throw new Error(`${name}: an extra field or a comment`);
    if (!/^[\x20-\x7e]+$/.test(name)) throw new Error(`member ${index}: a name that is not printable ASCII`);
    if (names.has(name)) throw new Error(`${name}: a duplicate name`);
    names.add(name);
    if (offset !== expectedOffset) throw new Error(`${name}: not where the previous member ends`);
    if (bytes.readUInt32LE(offset) !== 0x04034b50) throw new Error(`${name}: bad local header signature`);
    const local = {
      method: bytes.readUInt16LE(offset + 8), flags: bytes.readUInt16LE(offset + 6), crc: bytes.readUInt32LE(offset + 14),
      compressed: bytes.readUInt32LE(offset + 18), size: bytes.readUInt32LE(offset + 22),
      nameLength: bytes.readUInt16LE(offset + 26), extraLength: bytes.readUInt16LE(offset + 28)
    };
    const localName = bytes.subarray(offset + 30, offset + 30 + local.nameLength).toString("latin1");
    if (local.method !== 0 || local.flags !== 0 || local.crc !== crc || local.compressed !== size || local.size !== size || local.extraLength !== 0 || localName !== name) {
      throw new Error(`${name}: the local header differs from the central directory`);
    }
    const start = offset + 30 + local.nameLength;
    const data = bytes.subarray(start, start + size);
    if (data.length !== size) throw new Error(`${name}: truncated`);
    if (crc32(data) !== crc) throw new Error(`${name}: CRC-32 differs from its recomputation`);
    members.push({ name, bytes: Buffer.from(data) });
    expectedOffset = start + size;
    at += 46 + nameLength;
  }
  if (expectedOffset !== centralOffset) throw new Error("bytes between the last member and the central directory");
  return members;
}

/** What the runner itself reads in a saved report package: the manifest (the first
 * member), each member's SHA-256 and length beside the manifest's row for it, the HTML
 * member's text and the notices in it. `notice` is PRD §19.3's notice text. */
export function inspectReportPackage(buffer, notice) {
  const members = readStoredZip(buffer);
  if (members[0]?.name !== "package_manifest.json") throw new Error("the first member is not package_manifest.json");
  const manifestBytes = members[0].bytes;
  const manifest = JSON.parse(manifestBytes.toString("utf8"));
  const rows = Array.isArray(manifest.members) ? manifest.members : [];
  const content = members.slice(1).map((member) => ({ name: member.name, sha256: sha256Hex(member.bytes), byte_length: member.bytes.length }));
  const inventoryMatches = rows.length === content.length && rows.every((row, index) =>
    row.file_name === content[index].name && row.sha256_hex === content[index].sha256 && row.byte_length === content[index].byte_length);
  const roleOf = (role) => rows.findIndex((row) => row.role === role);
  const htmlIndex = roleOf("canonical_rendered_report_html");
  const resultIndex = roleOf("result_export");
  const html = htmlIndex >= 0 ? members[htmlIndex + 1].bytes.toString("utf8") : null;
  let result = null;
  try { result = resultIndex >= 0 ? JSON.parse(members[resultIndex + 1].bytes.toString("utf8")) : null; } catch { result = null; }
  const flat = html === null ? "" : normalizeSpace(html);
  const occurrences = (needle) => flat.split(needle).length - 1;
  const userRelease = manifest.user_release ?? null;
  const releaseNotice = flat.match(/USER-RELEASED REPORT — [\s\S]*?model SHA-256 ([0-9a-f]{64})/);
  const binding = userRelease?.content_binding?.members;
  return {
    file_sha256: sha256Hex(Buffer.from(buffer)),
    file_byte_length: Buffer.from(buffer).length,
    member_names: members.map((member) => member.name),
    package_identity_sha256: sha256Hex(manifestBytes),
    manifest_canonical: isCanonicalText(manifestBytes),
    export_profile_id: manifest.export_profile_id ?? null,
    export_blocked: manifest.diagnostics?.export_blocked ?? null,
    inventory_matches: inventoryMatches,
    html_sha256: htmlIndex >= 0 ? content[htmlIndex].sha256 : null,
    html_manifest_sha256: htmlIndex >= 0 ? rows[htmlIndex].sha256_hex : null,
    html_binding_sha256: Array.isArray(binding) ? binding.find((entry) => entry.role === "canonical_rendered_report_html")?.sha256 ?? null : null,
    content_binding_matches: Array.isArray(binding) ? binding.length === content.length && binding.every((entry, index) =>
      entry.file_name === content[index].name && entry.role === rows[index]?.role && entry.sha256 === content[index].sha256) : null,
    required_notice_count: html === null ? 0 : occurrences(normalizeSpace(notice)),
    user_release_notice_count: occurrences("USER-RELEASED REPORT —"),
    user_release_notice_model_sha256: releaseNotice?.[1] ?? null,
    local_private_notice_count: occurrences("LOCAL PRIVATE REPORT —"),
    user_release: userRelease,
    result_contract_id: result?.result_envelope?.producer?.semantic_contract_id ?? null
  };
}

/** A CSV table file as the product writes it (csvTable.ts): a UTF-8 BOM, `key,value` preamble rows,
 * one empty row, the header row and the data rows, each ended by CRLF; fields quoted per RFC 4180
 * (a quoted field may hold commas, quotes and line ends). Read by this runner, not by the product. */
export function parseCsvFile(text) {
  const source = String(text);
  const bom = source.startsWith("﻿");
  const body = bom ? source.slice(1) : source;
  const records = [];
  let field = "";
  let record = [];
  let quoted = false;
  let crlfOnly = true;
  for (let index = 0; index < body.length; index += 1) {
    const char = body[index];
    if (quoted) {
      if (char === '"' && body[index + 1] === '"') { field += '"'; index += 1; }
      else if (char === '"') quoted = false;
      else field += char;
      continue;
    }
    if (char === '"' && field === "") { quoted = true; continue; }
    if (char === ",") { record.push(field); field = ""; continue; }
    if (char === "\r" && body[index + 1] === "\n") { record.push(field); records.push(record); field = ""; record = []; index += 1; continue; }
    if (char === "\n") { crlfOnly = false; record.push(field); records.push(record); field = ""; record = []; continue; }
    field += char;
  }
  if (field !== "" || record.length > 0) records.push([...record, field]);
  const blank = records.findIndex((row) => row.length === 1 && row[0] === "");
  if (blank < 0) return { bom, crlf_only: crlfOnly, preamble: [], header: records[0] ?? [], rows: records.slice(1) };
  return { bom, crlf_only: crlfOnly, preamble: records.slice(0, blank), header: records[blank + 1] ?? [], rows: records.slice(blank + 2) };
}

/** The fields of a report-package status line (`outcome=…; code=…; …`). */
export function packageStatusFields(status) {
  const fields = {};
  for (const part of String(status ?? "").split(/;\s*/)) {
    const match = part.match(/^([a-z_0-9]+)=(.*)$/);
    if (match) fields[match[1]] = match[2].trim();
  }
  return fields;
}

// ---------------------------------------------------------------- autosave records

/** A record's identity from its name (`<slot>-<13-digit unix millis>-<model hash>.json`,
 * DEL-02-05 Design `autosave-and-backup.md` §1), or null for any other name. */
export function parseAutosaveName(name) {
  const match = /^([0-9a-f]{32})-(\d{13})-([0-9a-f]{64})\.json$/.exec(name);
  return match ? { slot: match[1], written_unix_millis: Number(match[2]), model_hash: match[3] } : null;
}
/** The slot of a project id: the first 32 hex digits of the SHA-256 of its UTF-8 bytes. */
export const autosaveSlot = (projectId) => sha256Hex(Buffer.from(projectId, "utf8")).slice(0, 32);

/** What the runner holds a record on disk to: its name parses, its slot is the project's,
 * its bytes are their own RFC 8785 text and hash to the name's model hash, and it was
 * written no earlier than `notBefore` and no more than `intervalMs` before `endedAt`. */
export function judgeAutosaveRecord({ name, bytes, projectId, endedAt, intervalMs, notBefore }) {
  const parsed = parseAutosaveName(name);
  const hash = sha256Hex(bytes);
  let documentHash = null;
  try { documentHash = documentSha256(JSON.parse(bytes.toString("utf8"))); } catch { documentHash = null; }
  return {
    name_parses: parsed !== null,
    slot_is_project: parsed?.slot === autosaveSlot(projectId),
    canonical_text: isCanonicalText(bytes),
    bytes_hash_is_name_hash: parsed?.model_hash === hash,
    document_hash_is_name_hash: parsed?.model_hash === documentHash,
    model_hash: parsed?.model_hash ?? null,
    written_unix_millis: parsed?.written_unix_millis ?? null,
    age_at_end_ms: parsed ? endedAt - parsed.written_unix_millis : null,
    within_one_interval: parsed !== null && endedAt - parsed.written_unix_millis <= intervalMs && parsed.written_unix_millis <= endedAt,
    after_the_edit: parsed !== null && parsed.written_unix_millis >= notBefore
  };
}

/** The process to end: the one whose command is exactly the private launch copy (with or
 * without arguments), from `ps -axo pid=,command=` output. Null when there is not exactly one. */
export function launchedPid(psOutput, executable) {
  const found = String(psOutput).split("\n").map((line) => line.match(/^\s*(\d+)\s+(.*)$/)).filter(Boolean)
    .filter(([, , command]) => command === executable || command.startsWith(`${executable} `)).map(([, pid]) => Number(pid));
  return found.length === 1 ? found[0] : null;
}

// ---------------------------------------------------------------- the project store

/** Every table of an SQLite store (opened with `node:sqlite`'s DatabaseSync), in name order,
 * each with its rows in a total order, blobs as hex: a logical dump two copies can be
 * compared by, whatever their page layout or WAL state. */
export function dumpStore(db) {
  const tables = db.prepare("SELECT name, sql FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name").all();
  const value = (cell) => (cell instanceof Uint8Array ? { blob: Buffer.from(cell).toString("hex") } : typeof cell === "bigint" ? { int: cell.toString() } : cell);
  return tables.map(({ name, sql }) => {
    const columns = db.prepare(`SELECT name FROM pragma_table_info(?)`).all(name).map((column) => column.name);
    const order = columns.map((_, index) => index + 1).join(", ");
    const rows = order ? db.prepare(`SELECT * FROM "${name.replace(/"/g, '""')}" ORDER BY ${order}`).all() : [];
    return { name, sql, rows: rows.map((row) => columns.map((column) => value(row[column]))) };
  });
}
