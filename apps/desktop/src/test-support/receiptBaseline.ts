/** The values test's oracle (`receiptValues.test.ts`): main's receipt functions as they were
 * before the full-copy receipt took digests in place of whole texts, copied unchanged from
 * main 0e6436e3a. They are kept here only so that the new path's values are compared in the
 * same test with the old path's, computed by code that is not the new code:
 * - `nativeContentFingerprintOf`, `canonicalJson`, `sortJson`: `services/previewService.ts`;
 * - `assertFinite`, `writtenText` (untimed): `features/results/resultReader.ts`.
 * `checkedJsonText` (hashService.ts) and `jsonTextKeepingNegativeZero` (jsonText.ts) are
 * main's unchanged functions, pinned by `tools/after_worker_family/family_table.py`, and are
 * imported. Test input only; no product code imports this. */
import { checkedJsonText } from "../services/hashService";
import { jsonTextKeepingNegativeZero } from "../services/jsonText";

export function baselineNativeContentFingerprintOf(value: unknown): string {
  const checked = checkedJsonText(value);
  const negativeZeros: string[] = [];
  const visit = (item: unknown, path: string) => {
    if (typeof item === "number" && Object.is(item, -0)) negativeZeros.push(path);
    else if (item && typeof item === "object") {
      for (const [key, child] of Object.entries(item)) visit(child, `${path}/${key.replaceAll("~", "~0").replaceAll("/", "~1")}`);
    }
  };
  visit(value, "");
  return `${canonicalJson(JSON.parse(checked))}\nnegative_zero_paths=${JSON.stringify(negativeZeros.sort())}`;
}

function canonicalJson(value: unknown): string {
  return JSON.stringify(sortJson(value));
}

function sortJson(value: unknown): unknown {
  if (Array.isArray(value)) {
    return value.map(sortJson);
  }
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value)
        .sort(([left], [right]) => left.localeCompare(right))
        .map(([key, nested]) => [key, sortJson(nested)]),
    );
  }
  return value;
}

class NotJson extends Error {}
function assertFinite(value: unknown): void {
  if (typeof value === "number") { if (!Number.isFinite(value)) throw new NotJson(); return; }
  if (Array.isArray(value)) { for (const item of value) assertFinite(item); return; }
  if (value !== null && typeof value === "object") for (const item of Object.values(value)) assertFinite(item);
}
/** The reader's `writtenText`, untimed. */
export function baselineWrittenText(value: unknown): string {
  assertFinite(value);
  return jsonTextKeepingNegativeZero(value);
}
