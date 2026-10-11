import { readFileSync } from "node:fs";
import { resolve } from "node:path";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

/**
 * The shared retained-precision reader corpus: one logical document stored as ordered snapshot files, each kept
 * under 50 MB (PY's `tests/test_fixture_file_size.py`). A new snapshot is a new file, appended here, in PY's
 * `tests/retained_precision_corpus.py` and in RS's `tests/retained_precision_contract.rs`. `cases`, `mutations`
 * and `must_pass` concatenate in file order; any other member appears once or identically in each file. Read from
 * disk, not as `?raw` module imports: the module transform of tens of MB of text exhausts the default heap.
 */
export const RETAINED_PRECISION_CORPUS_FILES = [
  "fixtures/results/retained_precision_cases.json", // 07m + 07n
  "fixtures/results/retained_precision_cases_07o.json", // 07o (B2), compact JSON
] as const;
const LIST_MEMBERS = new Set(["cases", "mutations", "must_pass"]);
const PROJECT_ROOT = resolve(__dirname, "../../../../");

export function readRetainedPrecisionCorpus(): Json {
  const merged: Record<string, Json> = {};
  for (const name of RETAINED_PRECISION_CORPUS_FILES) {
    const part: Record<string, Json> = JSON.parse(readFileSync(resolve(PROJECT_ROOT, name), "utf8"));
    for (const [key, value] of Object.entries(part)) {
      if (LIST_MEMBERS.has(key)) {
        if (!Array.isArray(value)) throw new Error(`${name}: ${key} is not an array`);
        merged[key] = [...(merged[key] ?? []), ...value];
      } else if (key in merged) {
        if (JSON.stringify(merged[key]) !== JSON.stringify(value)) throw new Error(`${name}: corpus member ${key} differs from an earlier file`);
      } else {
        merged[key] = value;
      }
    }
  }
  return merged;
}
