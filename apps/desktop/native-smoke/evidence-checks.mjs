// The pure parts of the evidence journeys' runner (`evidence-journeys.mjs`): each
// expectation the runner holds the launched app to is computed here, from the
// repository's governing text, the maintained fixtures and the bytes the app wrote,
// never from the app's own account of them. No function here launches, builds or
// writes anything; `evidence-checks.test.mjs` tests them with `node --test`.
import { createHash } from "node:crypto";
import os from "node:os";
import path from "node:path";

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

/** The journeys --all runs, in order: autosave-recovery first, whose first launch is the run's shortest. */
export const ALL_JOURNEYS = ["autosave-recovery", "report-exact", "report-pressure-free"];
/** Named alone, never part of --all. */
export const SEPARATE_JOURNEYS = ["reference-system"];

/** The runner's command line: `--all` or `--journey a,b`, or `--build-only` (check, build and bind the app,
 * then exit before any copy or launch; it takes no journey). A launch invocation builds nothing: anything that
 * would need building refuses it by name, as `report-standing-journey.mjs`. Returns { journeys, buildOnly } or
 * { error }. */
export function parseRunnerArgs(argv) {
  const args = new Set(argv);
  const known = new Set(["--all", "--journey", "--build-only"]);
  const valueOf = (flag) => { const at = argv.indexOf(flag); return at >= 0 ? argv[at + 1] : undefined; };
  const valued = new Set(["--journey"].map((flag) => argv.indexOf(flag)).filter((at) => at >= 0).map((at) => at + 1));
  const unknown = argv.filter((item, index) => !known.has(item) && !valued.has(index));
  if (unknown.length > 0) return { error: `unknown arguments: ${unknown.join(" ")}` };
  const buildOnly = args.has("--build-only");
  if (buildOnly) {
    if (args.has("--all") || args.has("--journey")) return { error: "--build-only builds and binds the app and takes no journey" };
    return { journeys: [], buildOnly: true };
  }
  const value = valueOf("--journey");
  if (args.has("--journey") && (!value || value.startsWith("--"))) return { error: "--journey needs one or more journey names" };
  if (args.has("--all") && args.has("--journey")) return { error: "choose --all or --journey, not both" };
  const journeys = args.has("--all") ? [...ALL_JOURNEYS] : (value ?? "").split(",").filter(Boolean);
  if (journeys.length === 0 || journeys.some((name) => !ALL_JOURNEYS.includes(name) && !SEPARATE_JOURNEYS.includes(name))) {
    return { error: `choose --all, --build-only or --journey with one or more of ${ALL_JOURNEYS.join(", ")} (or ${SEPARATE_JOURNEYS.join(", ")} alone)` };
  }
  if (journeys.some((name) => SEPARATE_JOURNEYS.includes(name)) && journeys.length > 1) return { error: `${SEPARATE_JOURNEYS.join(", ")} runs alone: name it as the only journey` };
  return { journeys, buildOnly: false };
}

// ---------------------------------------------------------------- what a run's record states

/** The owner's proxy's term, word for word, which every run's record carries, and the disclosure that ends it. How
 * each part is made true is the runner's header; the real Library is observed by the tripwire of `launch-env.mjs`
 * (`watchedLibraryPaths`: the product-named paths only), which is narrower than a listing of the whole Library. */
export const CONFINEMENT_TERM = "the product is confined by construction (both home variables, an environment allowlist, TMPDIR inside the throwaway home, no chooser that can fall through to a system dialog), and what is observed is the real Library compared before and after and the throwaway home itself; a run's record states that nothing else on the disk was watched.";
export const NOTHING_ELSE_WATCHED = "Nothing else on the disk was watched.";

/** What the tripwire can and cannot show, for the run's record. */
export const TRIPWIRE_LIMITS = "The tripwire records, before the first launch and after the last, the presence and modification time of each product-named path under the real user's Library (launch-env.mjs watchedLibraryPaths), by lstat of that exact path; it lists no folder and reads no contents. It cannot show what was read, a change restored in between, or anything at another path, and it does not establish that the app wrote nothing else outside its throwaway home.";

/** The strict chooser's exit code (`native_smoke.rs` STRICT_CHOOSER_EXIT); the switch itself is
 * `launch-env.mjs` STRICT_CHOOSER_ENV, set on every launch by `buildLaunchEnv`. A node test checks it agrees. */
export const STRICT_CHOOSER_EXIT = 86;

/** The strict chooser's limit, stated exactly in the run's record. */
export const STRICT_CHOOSER_LIMIT = "a wrong chooser can still consume a queued answer inside the throwaway home, which writes there, and the strict stop then fires at the next chooser.";

/** Which processes the runner signals and waits for. */
export const PROCESS_SCOPE = "Every launch runs under the worktree build lock's runUnderLock as /usr/bin/caffeinate -di <the run's private app copy>, in a process group of its own; the runner signals only that group (at its time limit, at a stop signal, or, for the autosave journey, by its abort), never a process found by a scan. After each launch the census (launch-env.mjs awaitNoProcesses) waits for no process running the private copy, and no throwaway home is removed until it finds none. Helpers the system starts for the web view (started by launchd through XPC) are outside that group and that census; the runner neither signals nor waits for them.";

/** The rule for a throwaway home a run could not remove. */
export const LEFT_HOME_RULE = "A throwaway home left by a run counts as open until it is removed. Only the lane that made it removes it, after confirming that no process runs the run's private app copy; nobody removes another lane's home.";

// ---------------------------------------------------------------- the throwaway homes and their removal

/** The file a journey writes into each throwaway home it creates, holding the run's token. */
export const HOME_MARKER = ".swbpipe-evidence-home-token";

/** Why `target` must not be removed, or null: only a throwaway home this run created (`created`), named as
 * `launch-env.mjs` createThrowawayHome names it, directly inside a `swbpipe-evidence-<journey>-` folder directly
 * inside the run's folder `runRoot`. */
export function homeRemovalRefusal(target, created, runRoot) {
  const base = String(runRoot).replace(/\/+$/, "");
  if (typeof target !== "string" || !target.startsWith("/") || target.split("/").includes("..")) return `not an absolute path without ..: ${target}`;
  if (!created.has(target)) return `not a home this run created: ${target}`;
  if (!target.startsWith(`${base}/`)) return `not under the run's folder (${base}): ${target}`;
  const parts = target.slice(base.length + 1).split("/");
  if (parts.length !== 2 || !/^swbpipe-native-smoke-home-[A-Za-z0-9]+$/.test(parts[1]) || !/^swbpipe-evidence-[a-z-]+-[A-Za-z0-9]+$/.test(parts[0])) return `not a throwaway home folder: ${target}`;
  return null;
}

/** The token in `home`'s marker: { token } when HOME_MARKER is a regular file (not a link, opened without following
 * one), or { failure } naming why it could not be read. `fs` is node:fs (a seam for the tests). */
export function readHomeMarker(home, fs) {
  const file = path.join(home, HOME_MARKER);
  let fd;
  try {
    fd = fs.openSync(file, fs.constants.O_RDONLY | fs.constants.O_NOFOLLOW);
  } catch (error) {
    return { failure: `its marker ${file} cannot be opened as a file (${error.code ?? error.message})` };
  }
  try {
    if (!fs.fstatSync(fd).isFile()) return { failure: `its marker ${file} is not a regular file` };
    return { token: fs.readFileSync(fd, "utf8") };
  } catch (error) {
    return { failure: `its marker ${file} cannot be read (${error.code ?? error.message})` };
  } finally {
    fs.closeSync(fd);
  }
}

/** Why a recursive removal of `home` must not happen now, or null. `home` and every folder above it up to and
 * including `root` (the run's own folder) must still be real directories (not links) with the device and inode the
 * runner recorded when it created them (`recorded`: path → { dev, ino }); and `home`'s marker must hold `token`, the
 * random token made for this run (`readMarker(home)` is readHomeMarker's answer). A marker that cannot be read is
 * reported as that, never as a mismatch or a match. Neither check alone is taken as ownership. `lstat` is
 * fs.lstatSync. Limits: the folders above `root` (the system's temporary folder) are not checked; a replacement that
 * copied the token, or a swap after this check and before the removal, is not excluded. */
export function removalIdentityRefusal(home, root, recorded, lstat, { token, readMarker }) {
  const base = String(root).replace(/\/+$/, "");
  if (!home.startsWith(`${base}/`)) return `${home} is not under the run's folder ${base}`;
  const chain = [];
  for (let at = home; at.length >= base.length; at = path.dirname(at)) {
    chain.push(at);
    if (at === base) break;
  }
  if (chain.at(-1) !== base) return `${home} does not lead to the run's folder ${base}`;
  for (const dir of chain) {
    const want = recorded.get(dir);
    if (!want) return `no identity was recorded for ${dir}`;
    let seen;
    try { seen = lstat(dir); } catch (error) { return `${dir} cannot be examined (${error.code ?? error.message})`; }
    if (seen.isSymbolicLink()) return `${dir} is now a link`;
    if (!seen.isDirectory()) return `${dir} is no longer a directory`;
    if (seen.dev !== want.dev || seen.ino !== want.ino) return `${dir} is not the directory the kit recorded (device and inode differ)`;
  }
  if (typeof token !== "string" || token.length < 32) return "no run token was given to compare the marker with";
  const marker = readMarker(home);
  if (marker?.failure || typeof marker?.token !== "string") return `${home}: ${marker?.failure ?? "its marker gave no answer"}`;
  if (marker.token !== token) return `${home}: its marker does not hold this run's token`;
  return null;
}

/** The act no machine finishes in these journeys; a person observes it. Save canonical HTML and Print are native
 * routes in this build, driven and judged by `report-standing-journey.mjs` (and Print by `run.mjs` too); these
 * journeys record whether their controls are shown and do not press them. */
export const UNFINISHED_BY_MACHINE = [
  { act: "Copy", why: "Copy would write the owner's system pasteboard; the selection it would take is observed, its enabled state and the key press are not" }
];

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
