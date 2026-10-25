// The demo's report-package save arguments (`demoReportPackageRequest.ts`): bound,
// canonical and reproducible. With `OPENPIPESTRESS_DEMO_REPORT_PACKAGE_REQUEST` set
// to an absolute path, writes their raw args body there for the native whole-command
// memory check (`src-tauri/src/lib.rs`
// `report_package_save_demo_request_peak_is_within_its_lease`). Unit transport
// replay of committed producer bytes; NOT a native UI qualification witness.
import { createHash } from "node:crypto";
import { writeFileSync } from "node:fs";
import { isAbsolute } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { canonicalJsonString } from "../services/hashService";
import {
  buildDemoReportPackageSaveArgs,
  DEMO_REPORT_PACKAGE_REQUEST_ENV,
  demoDenseSolveTransport,
  demoModel,
  demoReportPackageSaveBody
} from "./demoReportPackageRequest";

afterEach(() => { invokeMock.mockReset(); delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__; });

const sha256 = (text: string) => createHash("sha256").update(text, "utf8").digest("hex");

async function build() {
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(demoDenseSolveTransport().invoke);
  return buildDemoReportPackageSaveArgs();
}

describe("the demo's report-package save arguments", () => {
  it("are the session's bound canonical texts, reproducible byte for byte", async () => {
    const args = await build();
    const request = JSON.parse(args.requestCanonicalText);
    const manifest = JSON.parse(args.inputManifestCanonicalText);
    expect(args.requestCanonicalText).toBe(await canonicalJsonString(request));
    expect(args.modelCanonicalText).toBe(await canonicalJsonString(demoModel()));
    expect(args.inputManifestCanonicalText).toBe(await canonicalJsonString(manifest));
    expect(args.screening.payload_sha256).toBe(`sha256:${sha256(args.requestCanonicalText)}`);
    expect(request.audit_manifest.model_hash.value).toBe(sha256(args.modelCanonicalText));
    expect(request.audit_manifest.input_manifest_hash.value).toBe(sha256(args.inputManifestCanonicalText));
    expect(request.export_profile_id).toBe("desktop_local_private_report_package_1");
    expect(request.source_results).toHaveLength(1);
    expect(request.source_results[0].model_ref).toBe(demoModel().project.id);
    expect(manifest.solver_basis.solver_mode).toBe("dense_scrutiny");
    // The dense whole-run documents make it a multi-megabyte request.
    expect(args.requestCanonicalText.length).toBeGreaterThan(5_000_000);
    const body = demoReportPackageSaveBody(args);
    const again = demoReportPackageSaveBody(await build());
    expect(Buffer.from(again).equals(Buffer.from(body))).toBe(true);
    const target = process.env[DEMO_REPORT_PACKAGE_REQUEST_ENV];
    if (target) {
      if (!isAbsolute(target)) throw new Error(`${DEMO_REPORT_PACKAGE_REQUEST_ENV} must be an absolute path`);
      writeFileSync(target, body);
    }
  });

  it("replays only a dense solve of the demo model", async () => {
    const { invoke } = demoDenseSolveTransport();
    const start = (model: unknown, mode: string) =>
      invoke("start_preview_mechanics_job", new TextEncoder().encode(JSON.stringify(model)), { headers: { "x-swbpipe-solver-mode": mode } });
    await expect(start(demoModel(), "sparse_interactive")).rejects.toThrow("DEMO_REPLAY_REQUEST_MISMATCH");
    const edited = demoModel();
    edited.project.name = "Edited";
    await expect(start(edited, "dense_scrutiny")).rejects.toThrow("DEMO_REPLAY_REQUEST_MISMATCH");
    await expect(start(demoModel(), "dense_scrutiny")).resolves.toMatchObject({ state: "queued" });
  });
});
