// C1 slice 4: folds the requests recorded by `src/test/nativeReplayStore.ts`
// (NATIVE_REPLAY_RECORD=<dir>) into `src/test-support/native-replay/native_replay_requests.json`,
// one entry per key, sorted; a held source that is no committed fixture file is kept once, in
// `sources` by its digest. `src-tauri/src/native_replay_tests.rs` then answers each with its
// actual command (NATIVE_REPLAY_OUTCOMES_WRITE=1 writes the replies).
// Usage: node scripts/native-replay-merge.mjs <recorded-dir> [--replace]
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const [dir, mode] = process.argv.slice(2);
if (!dir) throw new Error("usage: node scripts/native-replay-merge.mjs <recorded-dir> [--replace]");
const target = resolve(import.meta.dirname, "../src/test-support/native-replay/native_replay_requests.json");
const entries = new Map();
const sources = new Map();
if (mode !== "--replace") {
  try {
    const document = JSON.parse(readFileSync(target, "utf8"));
    for (const [sha, text] of Object.entries(document.sources ?? {})) sources.set(sha, text);
    for (const item of document.requests) entries.set(item.key, item);
  } catch { /* new file */ }
}
for (const name of readdirSync(dir).filter((file) => file.endsWith(".json"))) {
  const { source_text: text, ...item } = JSON.parse(readFileSync(resolve(dir, name), "utf8"));
  if (text !== undefined) sources.set(item.source_sha256, text);
  entries.set(item.key, item);
}
const requests = [...entries.values()].sort((a, b) => (a.key < b.key ? -1 : a.key > b.key ? 1 : 0));
const named = new Set(requests.filter((item) => !item.source_path).map((item) => item.source_sha256));
const kept = Object.fromEntries([...sources].filter(([sha]) => named.has(sha)).sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0)));
for (const sha of named) if (!(sha in kept)) throw new Error(`a request names source ${sha} without a path or a kept text`);
writeFileSync(target, `${JSON.stringify({ document_kind: "swbpipe.desktop.native_replay_requests", note: "Recorded from the webview's tests (src/test/nativeReplayStore.ts); answered by src-tauri/src/native_replay_tests.rs.", sources: kept, requests }, null, 1)}\n`);
console.log(`${requests.length} requests, ${Object.keys(kept).length} kept sources`);
