import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { loadPreviewModel } from "../../services/previewService";
import type { PreviewModel } from "../../types";
import { loadedWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES, runModelChecks, utf8ByteLength } from "./modelChecks";

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

// DEL-04-06 Design `on-demand-model-checks.md`: the on-demand checks run the
// solve's own pre-formation checks through the wasm engine, beside the
// document validator's slot checks; invented changes to the bundled demo.
function clone(model: PreviewModel): PreviewModel {
  return JSON.parse(JSON.stringify(model)) as PreviewModel;
}

/** The demo with a second line no element joins to it, anchored at its first node. */
function withSecondLine(model: PreviewModel): PreviewModel {
  const next = clone(model);
  next.nodes.push(
    { ...next.nodes[0], id: "node:checks-c", position: { x: 40, y: 40, z: 0 } },
    { ...next.nodes[0], id: "node:checks-d", position: { x: 41, y: 40, z: 0 } }
  );
  next.pipe_segments.push({ ...next.pipe_segments[0], id: "pipe:checks-c-d", from: "node:checks-c", to: "node:checks-d" });
  next.supports.push({ ...next.supports[0], id: "support:checks-c", node: "node:checks-c" });
  return next;
}

describe("on-demand model checks", () => {
  it("lists the solve's topology warning with its code and text, linked to the entity, and leaves the model unchanged", async () => {
    const model = withSecondLine(await loadPreviewModel());
    const before = JSON.stringify(model);
    const run = await runModelChecks(model);
    expect(JSON.stringify(model)).toBe(before);
    expect(run.model).toBe(model);
    expect(run.solve).toEqual({ kind: "checked" });
    const disconnected = run.findings.filter((item) => item.diagnostic.code === "MODEL_TOPOLOGY_DISCONNECTED");
    expect(disconnected).toHaveLength(1);
    expect(disconnected[0].group).toBe("solve");
    expect(disconnected[0].diagnostic).toMatchObject({ id: "diagnostic:topology:disconnected", severity: "warning", source: "core/product_physics" });
    expect(disconnected[0].diagnostic.message).toContain("form 2 disconnected parts");
    expect(disconnected[0].diagnostic.message).toContain("(from node:checks-c; 2 nodes; supported)");
    expect(disconnected[0].entity).toEqual({ type: "node", id: model.nodes[0].id });
  });

  it("reports the solve's restraint pre-check as blocking, with no entity when it names none", async () => {
    const model = clone(await loadPreviewModel());
    model.supports = [];
    const run = await runModelChecks(model);
    const missing = run.findings.find((item) => item.diagnostic.code === "SUPPORT_INPUT_MISSING");
    expect(missing?.diagnostic).toMatchObject({ severity: "blocking", affected_refs: ["supports"] });
    expect(missing?.entity).toBeNull();
  });

  it("names a pipe end that points nowhere, linked to the pipe", async () => {
    const model = clone(await loadPreviewModel());
    model.pipe_segments[0] = { ...model.pipe_segments[0], to: "node:checks-missing" };
    const run = await runModelChecks(model);
    const unknown = run.findings.find((item) => item.group === "solve" && item.diagnostic.code === "PIPE_ENDPOINT_UNKNOWN");
    expect(unknown?.diagnostic.severity).toBe("blocking");
    expect(unknown?.entity).toEqual({ type: "pipe", id: model.pipe_segments[0].id });
  });

  it("states that the solve's checks did not run when the solve cannot read the model, with the solve's refusal", async () => {
    const model = clone(await loadPreviewModel());
    (model.nodes[0] as unknown as Record<string, unknown>).undeclared_member = true;
    const run = await runModelChecks(model);
    expect(run.solve.kind).toBe("refused");
    expect(run.solve.kind === "refused" && run.solve.refusal).toContain("undeclared_member");
    expect(run.findings.some((item) => item.group === "solve")).toBe(false);
  });

  // Codex review r1 of #1665: model-sized work runs under the native memory
  // pool in the app, and under an explicit bound in the browser preview.
  it("in the app, runs the solve's checks through the native command under ingress admission, never the wasm engine", async () => {
    const model = withSecondLine(await loadPreviewModel());
    const engine = loadedWasmEngine()!;
    const wasm = vi.spyOn(engine, "modelChecksJson");
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue({ document_kind: "openpipestress.desktop.model_checks", checked: true, diagnostics_not_shown: 3, slot_findings: [], slot_findings_not_shown: 2,
      diagnostics: [{ id: "diagnostic:topology:disconnected", code: "MODEL_TOPOLOGY_DISCONNECTED", severity: "warning", message: "invented native text", source: "core/product_physics", affected_refs: [model.nodes[0].id] }] });
    const run = await runModelChecks(model);
    // The test invoke decodes the raw args-json-v1 body: the command received exactly `{model}`.
    expect(invokeMock).toHaveBeenCalledWith("check_preview_model", { model: JSON.parse(JSON.stringify(model)) });
    expect(wasm).not.toHaveBeenCalled();
    expect(run.solve).toEqual({ kind: "checked" });
    expect(run.notShown).toEqual({ solve: 3, slots: 2, library: 0 });
    expect(run.findings.find((item) => item.group === "solve")?.diagnostic.message).toBe("invented native text");
  });

  it("in the app, a native memory-pool refusal leaves the solve's checks not run, with the refusal named", async () => {
    const model = clone(await loadPreviewModel());
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockRejectedValue({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "pool_exhausted", requested_bytes: 1024 ** 3, reserved_bytes: 12 * 1024 ** 3, pool_bytes: 12 * 1024 ** 3 } });
    const run = await runModelChecks(model);
    expect(run.solve).toEqual({ kind: "unavailable", reason: "Refused by the native memory pool: The memory pool is fully reserved: this request needs 1 GiB, and 12 GiB of the 12 GiB pool is already reserved." });
    // Nothing ran: no solve, slot or library finding is listed.
    expect(run.findings).toEqual([]);
  });

  it("in the browser preview, refuses a model text above the bound by name without passing it to the engine", async () => {
    const model = clone(await loadPreviewModel());
    model.nodes[0] = { ...model.nodes[0], provenance: "x".repeat(MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES) };
    const engine = loadedWasmEngine()!;
    const wasm = vi.spyOn(engine, "modelChecksJson");
    const run = await runModelChecks(model);
    expect(wasm).not.toHaveBeenCalled();
    expect(run.solve.kind).toBe("unavailable");
    expect(run.solve.kind === "unavailable" && run.solve.reason).toMatch(/^MODEL-CHECKS-INPUT-TOO-LARGE: the browser preview checks a model text of at most 4194304 bytes/);
  });

  // Codex review r2 of #1665 (its probes, kept as regressions).
  it("counts the browser bound in UTF-8 bytes before any transfer into wasm", async () => {
    const model = await loadPreviewModel();
    const big = { ...model, project: { ...model.project, name: "漢".repeat(1_500_000) } };
    const text = JSON.stringify(big);
    expect(text.length).toBeLessThan(MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES);
    expect(new TextEncoder().encode(text).length).toBeGreaterThan(MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES);
    const engine = loadedWasmEngine()!;
    const checks = vi.spyOn(engine, "modelChecksJson");
    const slots = vi.spyOn(engine, "validateModelDocumentJson");
    const run = await runModelChecks(big);
    expect(checks).not.toHaveBeenCalled();
    expect(slots).not.toHaveBeenCalled();
    expect(run.solve.kind === "unavailable" && run.solve.reason).toMatch(/^MODEL-CHECKS-INPUT-TOO-LARGE/);
    expect(utf8ByteLength("a\u00e9\u6f22\u{1F600}")).toBe(1 + 2 + 3 + 4);
  });

  it("keeps a long id whole, so its finding still selects the support", async () => {
    const model = clone(await loadPreviewModel());
    const id = "support:" + "é".repeat(1500);
    model.supports = [{ ...model.supports[0], id, family: "review_unknown" }];
    const run = await runModelChecks(model);
    const finding = run.findings.find((item) => item.group === "solve" && item.diagnostic.code === "SUPPORT_FAMILY_INVALID")!;
    expect(finding.diagnostic.affected_refs?.[0]).toBe(id);
    expect(finding.entity).toEqual({ type: "support", id });
  });

  it("runs no slot check on its own: a refused or oversized model reaches no validator, and the slot findings share the report bound", async () => {
    const model = await loadPreviewModel();
    const slots = vi.spyOn(loadedWasmEngine()!, "validateModelDocumentJson");
    const oversized = await runModelChecks({ ...model, project: { ...model.project, name: "a".repeat(4_300_000) } });
    expect(oversized.solve.kind).toBe("unavailable");
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockRejectedValue({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "pool_exhausted", requested_bytes: 1024 ** 3, reserved_bytes: 12 * 1024 ** 3, pool_bytes: 12 * 1024 ** 3 } });
    const refused = await runModelChecks(model);
    expect(refused.solve.kind).toBe("unavailable");
    delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
    expect(slots).not.toHaveBeenCalled();
    // 4,000 assumptions naming missing entities: the slot findings are listed within the bound and the rest counted.
    const flooded = clone(model) as PreviewModel & { unresolved_assumptions?: unknown[] };
    flooded.unresolved_assumptions = Array.from({ length: 4000 }, (_, i) => ({ assumption_id: `assumption:review-${i}`, kind: "assumption" as const, status: "unresolved" as const, statement: "Invented probe", affected_refs: ["missing:" + "x".repeat(600)] }));
    const run = await runModelChecks(flooded);
    expect(utf8ByteLength(JSON.stringify(run.findings.map((item) => item.diagnostic)))).toBeLessThanOrEqual(1024 * 1024 + 512);
    expect(run.notShown.slots).toBeGreaterThan(0);
  });
});
