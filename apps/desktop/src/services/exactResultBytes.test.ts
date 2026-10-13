import { afterEach, describe, expect, it, vi } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { MechanicsResult, PreviewModel } from "../types";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import { pollPreviewMechanicsJob, receivedResultBytes, runPreviewMechanics, startPreviewMechanicsJob } from "./previewService";

afterEach(() => {
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});
const product = resolve(__dirname, "../../../../fixtures/product_preview");
const MODEL = JSON.parse(readFileSync(resolve(product, "rf_skew_t_cant_off_122_r1e-04.desktop_model.json"), "utf8")) as PreviewModel;
/** PP's pins of the G10 milestone's protected ordinary bytes (`g10_desktop_milestone_model.rs`); the copies are
 * the native desktop crate's (`solves_hand_the_desktop_the_producers_exact_bytes`). */
const PINS = [["sparse_interactive", 68531, "d623c9e7783903fe5783d24527f1b1662b289b75c03d67635418425ff83e31eb"],
  ["dense_scrutiny", 69649, "767c50a8d903b10b8d0ae416c60526dcb5310546e8f89c21a340a3f19f7f7d99"]] as const;
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
      invokeMock.mockImplementation(async (command: string, args?: Record<string, unknown>) => {
        if (command === "run_preview_mechanics_with_solver_mode") return bytes;
        if (command === "start_preview_mechanics_job_with_solver_mode") return { job_id: "g10", backend_cancellation_token: "g10:token", state: "queued", cancellation_scope: "cooperative_checkpoints_not_preemptive" };
        if (command === "poll_preview_mechanics_job" && args?.jobId === "g10") return { job_id: "g10", state: "completed", cancellation_requested: false, cancellation_status: "not_requested", cancellation_scope: "cooperative_checkpoints_not_preemptive", result: bytes, error_message: null };
        throw new Error(`unexpected ${command}`);
      });
      const direct = await runPreviewMechanics(MODEL, mode);
      expect(invokeMock).toHaveBeenCalledWith("run_preview_mechanics_with_solver_mode", { model: MODEL, solverMode: mode });
      const received = receivedResultBytes(direct)!;
      expect([Buffer.byteLength(received, "utf8"), sha256(received)], "the bytes the desktop received").toEqual([length, digest]);
      expect(direct).toEqual(JSON.parse(bytes));
      expect(Object.keys(direct)).toEqual(Object.keys(JSON.parse(bytes)));
      await startPreviewMechanicsJob(MODEL, mode);
      const status = await pollPreviewMechanicsJob("g10");
      expect(sha256(receivedResultBytes(status.result)!), "the job's bytes").toBe(digest);
      // The bytes describe the result only while it is unchanged: a copy or an edit has none.
      expect(receivedResultBytes(structuredClone(direct))).toBeNull();
      (direct as MechanicsResult & { model_ref: string }).model_ref = "project:edited";
      expect(receivedResultBytes(direct)).toBeNull();
    });
  }
  it("refuses a solve answer that is not the producer's bytes", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(JSON.parse(producerBytes("sparse_interactive")));
    await expect(runPreviewMechanics(MODEL, "sparse_interactive")).rejects.toThrow("PREVIEW-SOLVE-RESULT-BYTES-REQUIRED");
  });
});
