/** The received result's guards still refuse, with digests in place of kept texts: a tree
 * changed between the receipt and the end of validation is not registered; a tree changed
 * after registration loses its registration, its received bytes and its intake text until it
 * is the received tree again; and a reader reads the rewritten text of a changed tree, not the
 * received one. Each decision is also checked against main's fingerprint and written text
 * (`test-support/receiptBaseline.ts`). */
import { afterEach, describe, expect, it, vi } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { MechanicsResult, PreviewModel } from "../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const seam = vi.hoisted(() => ({ invoke: vi.fn(), onHash: null as null | ((value: unknown) => void | Promise<void>), hashed: [] as unknown[] }));
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../test/nativeIngress")).ingressDecodingInvoke(seam.invoke) }));
vi.mock("./hashService", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./hashService")>();
  return {
    ...actual,
    canonicalSha256HexCheckedV1: async (value: unknown, text?: string) => {
      seam.hashed.push(value);
      if (seam.onHash) await seam.onHash(value);
      return actual.canonicalSha256HexCheckedV1(value, text);
    },
  };
});
import { deliveredIntakeText, hasNativeMechanicsInvocation, receivedResultBytes, runPreviewMechanics } from "./previewService";
import { createSolveJobTransport } from "../test/nativeMechanicsReplay";
import { readerSourceText } from "../features/results/resultReader";
import { jsonTextKeepingNegativeZero } from "./jsonText";
import { baselineNativeContentFingerprintOf } from "../test-support/receiptBaseline";

afterEach(() => {
  seam.invoke.mockReset(); seam.onHash = null; seam.hashed.length = 0;
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});
const product = resolve(__dirname, "../../../../fixtures/product_preview");
const MODEL = JSON.parse(readFileSync(resolve(product, "rf_skew_t_cant_off_122_r1e-04.desktop_model.json"), "utf8")) as PreviewModel;
const MODE = "sparse_interactive";
const BYTES = readFileSync(resolve(product, `rf_skew_t_cant_off_122_r1e-04.desktop_result_${MODE}.json`), "utf8");

async function solve(): Promise<MechanicsResult> {
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const transport = createSolveJobTransport(() => BYTES, { jobIdPrefix: "receipt-guards" });
  seam.invoke.mockImplementation(transport.invoke);
  return runPreviewMechanics(MODEL, MODE);
}
/** The first finite non-zero number in the result's rows, with a way to set it. */
function firstValue(result: Json): { get: () => number; set: (value: number) => void } {
  const row = (result.results as Json[]).find((r) => typeof r.value === "number" && r.value !== 0);
  return { get: () => row.value, set: (value) => { row.value = value; } };
}
const held = (result: MechanicsResult) => ({
  registered: hasNativeMechanicsInvocation(result, MODEL, MODE),
  bytes: receivedResultBytes(result) === BYTES,
  intake: deliveredIntakeText(result) !== null,
});

describe("the received result's guards, with digests in place of kept texts", () => {
  it("registers an unchanged result: its registration, received bytes and intake text hold", async () => {
    const result = await solve();
    expect(seam.hashed).toEqual([result]);
    expect(held(result)).toEqual({ registered: true, bytes: true, intake: true });
  });

  it("does not register a tree changed between the receipt and the end of validation", async () => {
    let changed: Json = null;
    seam.onHash = (value) => { changed = value; (value as Json).run_id = `${(value as Json).run_id}:changed during validation`; };
    const result = await solve();
    expect(changed).toBe(result);
    expect(held(result)).toEqual({ registered: false, bytes: false, intake: false });
    // Restored, it is the received tree again (its bytes), but validation did not register it.
    (result as Json).run_id = (result as Json).run_id.replace(":changed during validation", "");
    expect(held(result)).toEqual({ registered: false, bytes: true, intake: false });
  });

  it("does not register a tree whose number became -0 during validation, though its JSON text is unchanged", async () => {
    seam.onHash = (value) => { firstValue(value).set(-0); };
    const before = JSON.parse(BYTES) as Json;
    firstValue(before).set(0);
    const result = await solve();
    expect(JSON.stringify(result)).toBe(JSON.stringify(before));
    expect(held(result).registered).toBe(false);
  });

  it("refuses a registered tree changed before a read, and holds again when it is the received tree again", async () => {
    const result = await solve();
    const value = firstValue(result), original = value.get(), fingerprint = baselineNativeContentFingerprintOf(result);
    for (const changed of [original * (1 + Number.EPSILON), -original, -0]) {
      value.set(changed);
      expect(baselineNativeContentFingerprintOf(result) === fingerprint, String(changed)).toBe(false);
      expect(held(result), String(changed)).toEqual({ registered: false, bytes: false, intake: false });
      value.set(original);
      expect(held(result), String(changed)).toEqual({ registered: true, bytes: true, intake: true });
    }
    // A member added after registration.
    (result as Json).added_after_registration = null;
    expect(held(result)).toEqual({ registered: false, bytes: false, intake: false });
    delete (result as Json).added_after_registration;
    expect(held(result)).toEqual({ registered: true, bytes: true, intake: true });
    // A member moved: the fingerprint, as main's, does not depend on member order, so the
    // registration holds exactly when main's fingerprint text is unchanged.
    const first = Object.keys(result)[0], item = (result as Json)[first];
    delete (result as Json)[first]; (result as Json)[first] = item;
    expect(held(result).registered).toBe(baselineNativeContentFingerprintOf(result) === fingerprint);
  });

  it("reads the rewritten text of a changed tree, never the received one, and the received text again once restored", async () => {
    const result = await solve();
    expect(readerSourceText(result)).toBe(BYTES);
    const value = firstValue(result), original = value.get();
    for (const changed of [-original, -0]) {
      value.set(changed);
      const read = readerSourceText(result);
      expect(read).not.toBe(BYTES);
      expect(read).toBe(jsonTextKeepingNegativeZero(result));
      value.set(original);
      expect(readerSourceText(result)).toBe(BYTES);
    }
    // A member moved changes the written text: the reader reads the restatement.
    const first = Object.keys(result)[0], item = (result as Json)[first];
    delete (result as Json)[first]; (result as Json)[first] = item;
    expect(readerSourceText(result)).toBe(jsonTextKeepingNegativeZero(result));
    expect(readerSourceText(result)).not.toBe(BYTES);
  });
});
