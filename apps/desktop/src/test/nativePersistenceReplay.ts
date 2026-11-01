/** C1 slice 5: the native persistence commands' actual replies for the session tests
 * (`test-support/native-replay/native_persistence_replies.json`, written by
 * `src-tauri/src/native_persistence_tests.rs` from the commands on a real store, in order per store,
 * for the solve of `nativeMechanicsReplayPair("sparse_interactive", {profile: "pressure"})`). A
 * test takes one store's scenarios in order: each request the session sends must equal the recorded
 * request exactly (its job id written `$JOB`), and is answered with the recorded reply as Tauri
 * delivers it (a parsed JSON body, or the rejection). Any other request fails the test. */
import { isDeepStrictEqual } from "node:util";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const REPLIES = resolve(__dirname, "../test-support/native-replay/native_persistence_replies.json");

function withJob(value: unknown, jobId: string): unknown {
  if (value === jobId) return "$JOB";
  if (Array.isArray(value)) return value.map((item) => withJob(item, jobId));
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, withJob(item, jobId)]));
  return value;
}

export function nativePersistenceReplay(store: string) {
  const document = JSON.parse(readFileSync(REPLIES, "utf8")) as { scenarios: Json[] };
  const scenarios = document.scenarios.filter((scenario) => scenario.store === store);
  let next = 0;
  return {
    /** Answers the next recorded request of this store. */
    invoke(command: string, args: Json): unknown {
      const scenario = scenarios[next];
      if (!scenario) throw new Error(`NATIVE_PERSISTENCE_REPLAY: ${command} after the store's ${scenarios.length} recorded requests`);
      const recorded = withJob(args, String(args?.jobId));
      if (scenario.command !== command || !isDeepStrictEqual(recorded, scenario.args)) {
        const differing = Object.keys({ ...scenario.args, ...(recorded as object) }).filter((key) => !isDeepStrictEqual((recorded as Json)?.[key], scenario.args[key]));
        throw new Error(`NATIVE_PERSISTENCE_REPLAY: request ${next} is ${command}, recorded ${scenario.command}; members differing: ${differing.join(", ")}`);
      }
      next += 1;
      if (!scenario.reply.ok) throw scenario.reply.error;
      return structuredClone(scenario.reply.value);
    },
    remaining: () => scenarios.length - next,
    scenarios
  };
}
