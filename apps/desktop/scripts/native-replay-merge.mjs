// C1 slice 4: folds the requests recorded by `src/test/nativeReplayStore.ts`
// (NATIVE_REPLAY_RECORD=<dir>) into `src/test-support/native-replay/native_replay_requests.json`,
// one entry per key, sorted. `src-tauri/src/native_replay_tests.rs` then answers each with its
// actual command (NATIVE_REPLAY_OUTCOMES_WRITE=1 writes the replies).
// Usage: node scripts/native-replay-merge.mjs <recorded-dir> [--replace]
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const [dir, mode] = process.argv.slice(2);
if (!dir) throw new Error("usage: node scripts/native-replay-merge.mjs <recorded-dir> [--replace]");
const target = resolve(import.meta.dirname, "../src/test-support/native-replay/native_replay_requests.json");
const entries = new Map();
if (mode !== "--replace") {
  try { for (const item of JSON.parse(readFileSync(target, "utf8")).requests) entries.set(item.key, item); } catch { /* new file */ }
}
for (const name of readdirSync(dir).filter((file) => file.endsWith(".json"))) {
  const item = JSON.parse(readFileSync(resolve(dir, name), "utf8"));
  entries.set(item.key, item);
}
const requests = [...entries.values()].sort((a, b) => (a.key < b.key ? -1 : a.key > b.key ? 1 : 0));
writeFileSync(target, `${JSON.stringify({ document_kind: "swbpipe.desktop.native_replay_requests", note: "Recorded from the webview's tests (src/test/nativeReplayStore.ts); answered by src-tauri/src/native_replay_tests.rs.", requests }, null, 1)}\n`);
console.log(`${requests.length} requests`);
