/** C1 slice 4: the native result commands' replies for the webview's tests come from the actual Rust
 * commands. Each request the webview sends (its command, its raw-ingress argument object with the job
 * id written `$JOB`, and the bytes the job holds) is keyed; `native_replay_outcomes.json`, written by
 * `src-tauri/src/native_replay_tests.rs` from the commands themselves, holds the reply for each
 * recorded key: the verdict's JSON value as Tauri delivers it parsed, byte bodies as an ArrayBuffer,
 * refusals as their serialized rejection. An unrecorded request fails the test (never a permissive
 * reply). With `NATIVE_REPLAY_RECORD=<dir>` the request is written there instead, for
 * `scripts/native-replay-merge.mjs` to fold into `native_replay_requests.json`; the Rust test then
 * writes its replies (`NATIVE_REPLAY_OUTCOMES_WRITE=1`). */
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const OUTCOMES = resolve(__dirname, "../test-support/native-replay/native_replay_outcomes.json");
const FIXTURE_ROOT = resolve(__dirname, "../../../..");
export const NATIVE_REPLAY_JOB = "$JOB";

type Reply = { ok: true; body: "json"; value: unknown } | { ok: true; body: "raw"; base64: string } | { ok: false; error: unknown };
let outcomes: Map<string, Reply> | null = null;
function loaded(): Map<string, Reply> {
  if (!outcomes) {
    outcomes = new Map();
    try {
      const document = JSON.parse(readFileSync(OUTCOMES, "utf8")) as { outcomes: { key: string; reply: Reply }[] };
      for (const item of document.outcomes) outcomes.set(item.key, item.reply);
    } catch { /* no replies recorded yet */ }
  }
  return outcomes;
}

function withJob(value: unknown, jobId: string): unknown {
  if (value === jobId) return NATIVE_REPLAY_JOB;
  if (Array.isArray(value)) return value.map((item) => withJob(item, jobId));
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, withJob(item, jobId)]));
  return value;
}
const sha256 = (text: string) => createHash("sha256").update(text, "utf8").digest("hex");

/** The fixture files a held source may be, by digest (so a request names a path, not a copy). */
const KNOWN_SOURCES = [
  "fixtures/results/pressure_connected_ui_mechanics_sparse.json",
  "fixtures/results/pressure_connected_ui_mechanics_dense.json",
  "fixtures/results/preview_physics_connected_sparse.json",
  "fixtures/results/preview_physics_connected_dense.json"
];
let known: Map<string, string> | null = null;
function knownPath(sha: string): string | null {
  if (!known) {
    known = new Map();
    for (const path of KNOWN_SOURCES) {
      try { known.set(sha256(readFileSync(resolve(FIXTURE_ROOT, path), "utf8")), path); } catch { /* absent */ }
    }
  }
  return known.get(sha) ?? null;
}

export class NativeReplayUnrecorded extends Error {
  constructor(command: string, key: string) {
    super(`NATIVE_REPLAY_REQUEST_UNRECORDED: ${command} ${key.slice(0, 16)}… has no reply from the actual command; record it (NATIVE_REPLAY_RECORD=<dir>), merge it and run native_replay_tests with NATIVE_REPLAY_OUTCOMES_WRITE=1.`);
    this.name = "NativeReplayUnrecorded";
  }
}

/** The reply the actual command gave for this request, delivered as Tauri delivers it. */
export function nativeReplayReply(command: string, args: Record<string, unknown>, jobId: string, sourceText: string): unknown {
  const sourceSha = sha256(sourceText);
  const recordedArgs = withJob(args, jobId);
  const key = sha256(`${sourceSha}\u0000${command}\u0000${JSON.stringify(recordedArgs)}`);
  const reply = loaded().get(key);
  if (!reply) {
    const dir = process.env.NATIVE_REPLAY_RECORD;
    if (dir) {
      mkdirSync(dir, { recursive: true });
      const path = knownPath(sourceSha);
      writeFileSync(resolve(dir, `${key}.json`), JSON.stringify({ key, command, args: recordedArgs, source_sha256: sourceSha, ...(path ? { source_path: path } : { source_text: sourceText }) }));
    }
    throw new NativeReplayUnrecorded(command, key);
  }
  if (!reply.ok) throw reply.error;
  if (reply.body === "json") return structuredClone(reply.value);
  const bytes = Buffer.from(reply.base64, "base64");
  return bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
}
