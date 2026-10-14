import { afterEach, describe, expect, it, vi } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { MechanicsResult, PreviewModel } from "../types";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { pollPreviewMechanicsJob, receivedResultBytes, runPreviewMechanics, startPreviewMechanicsJob, takePreviewMechanicsJobResult } from "./previewService";
import { createSolveJobTransport, resultArrayBuffer } from "../test/nativeMechanicsReplay";

afterEach(() => {
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});
const product = resolve(__dirname, "../../../../fixtures/product_preview");
const MODEL = JSON.parse(readFileSync(resolve(product, "rf_skew_t_cant_off_122_r1e-04.desktop_model.json"), "utf8")) as PreviewModel;
/** PP's pins of the G10 milestone's protected ordinary bytes (`g10_desktop_milestone_model.rs`); the copies are
 * the native desktop crate's (`solves_hand_the_desktop_the_producers_exact_bytes`). */
const PINS = [["sparse_interactive", 68215, "262350431963ec51fe025007f4381fdfc84326f68b80a7c13f73bed768c1a580"],
  ["dense_scrutiny", 68236, "9fc0332eaf7798ed96f2013beb3cca2f7f429830b1e5719bdf535da8c02aad3d"]] as const;
const producerBytes = (mode: string) => readFileSync(resolve(product, `rf_skew_t_cant_off_122_r1e-04.desktop_result_${mode}.json`), "utf8");
const sha256 = (text: string) => createHash("sha256").update(text, "utf8").digest("hex");

/** b7-plan R1, "Exact bytes to the desktop": the desktop receives the producer's exact bytes for the G10 milestone
 * model, through the direct solve and the job, and keeps them beside the result it reads from them. */
describe("the desktop receives the producer's exact bytes (G10 milestone)", () => {
  for (const [mode, length, digest] of PINS) {
    it(`${mode}: direct solve and job`, async () => {
      const bytes = producerBytes(mode);
      expect([Buffer.byteLength(bytes, "utf8"), sha256(bytes)], "the producer's bytes").toEqual([length, digest]);
      (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
      const sent: Array<{ model: unknown; mode: unknown }> = [];
      const transport = createSolveJobTransport((request) => { sent.push({ model: request.model, mode: request.mode }); return bytes; }, { jobIdPrefix: "g10" });
      invokeMock.mockImplementation(transport.invoke);
      const direct = await runPreviewMechanics(MODEL, mode);
      // The start carries the model document's UTF-8 JSON text and the mode header; no JSON-args model.
      expect(invokeMock.mock.calls[0][0]).toBe("start_preview_mechanics_job");
      expect(ArrayBuffer.isView(invokeMock.mock.calls[0][1])).toBe(true);
      expect(invokeMock.mock.calls[0][2]).toEqual({ headers: { "x-swbpipe-solver-mode": mode } });
      expect(sent).toEqual([{ model: MODEL, mode }]);
      const received = receivedResultBytes(direct)!;
      expect([Buffer.byteLength(received, "utf8"), sha256(received)], "the bytes the desktop received").toEqual([length, digest]);
      expect(direct).toEqual(JSON.parse(bytes));
      expect(Object.keys(direct)).toEqual(Object.keys(JSON.parse(bytes)));
      const start = await startPreviewMechanicsJob(MODEL, mode);
      if (start.mode !== "backend_job") throw new Error("backend job expected");
      const status = await pollPreviewMechanicsJob(start.job_id);
      expect(status).not.toHaveProperty("result");
      expect(status.result_bytes).toBe(length);
      const taken = await takePreviewMechanicsJobResult(start.job_id);
      expect(sha256(receivedResultBytes(taken)!), "the job's bytes").toBe(digest);
      // The bytes describe the result only while it is unchanged: a copy or an edit has none.
      expect(receivedResultBytes(structuredClone(direct))).toBeNull();
      (direct as MechanicsResult & { model_ref: string }).model_ref = "project:edited";
      expect(receivedResultBytes(direct)).toBeNull();
    });
  }
  it("refuses a solve answer that is not the producer's bytes", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const transport = createSolveJobTransport(() => producerBytes("sparse_interactive"));
    // The take answers a parsed value (or a string) instead of the producer's bytes.
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) =>
      command === "take_preview_mechanics_job_result" ? JSON.parse(producerBytes("sparse_interactive")) : transport.invoke(command, args, options));
    await expect(runPreviewMechanics(MODEL, "sparse_interactive")).rejects.toThrow("PREVIEW-SOLVE-RESULT-BYTES-REQUIRED");
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) =>
      command === "take_preview_mechanics_job_result" ? producerBytes("sparse_interactive") : transport.invoke(command, args, options));
    await expect(runPreviewMechanics(MODEL, "sparse_interactive")).rejects.toThrow("PREVIEW-SOLVE-RESULT-BYTES-REQUIRED");
  });
  it("refuses result bytes that are not UTF-8", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const transport = createSolveJobTransport(() => producerBytes("sparse_interactive"));
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) =>
      command === "take_preview_mechanics_job_result" ? new Uint8Array([0x7b, 0xff, 0x7d]).buffer : transport.invoke(command, args, options));
    await expect(runPreviewMechanics(MODEL, "sparse_interactive")).rejects.toThrow("PREVIEW-SOLVE-RESULT-UTF8-INVALID");
    // The unreadable delivery is released from native custody at once.
    expect(invokeMock.mock.calls.filter(([command]) => command === "dispose_preview_mechanics_result")).toHaveLength(1);
    expect(resultArrayBuffer("{}").byteLength).toBe(2);
  });
});
