// A check whose library-reference collection fails keeps the engine's answer
// (DEL-07-07 REQ-008: the engine's answer is kept when a later group fails): the
// solve group's blocking finding stands, and the library group reads as not
// completed. The invented model is the bundled one with pipe P-100's material emptied.
import { render, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import * as wasmEngine from "../../services/wasmEngine/loadWasmEngine";
import { loadPreviewModel } from "../../services/previewService";
import type { PreviewModel } from "../../types";
import { readSolveGate } from "../workspace/solveGates";
import { ModelChecksPanel } from "./ModelChecksPanel";
import { modelChecksNotCompleted, runModelChecks, type ModelCheckRun } from "./modelChecks";

it("keeps the engine's findings when the library-reference collection throws, and marks only the library group not completed", async () => {
  const model = JSON.parse(JSON.stringify(await loadPreviewModel())) as PreviewModel;
  model.pipe_segments.find((pipe) => pipe.id === "pipe:P-100")!.material = "";
  const run = await runModelChecks(model, () => { throw new Error("invented library store failure"); });
  expect(run.solve).toEqual({ kind: "checked" });
  expect(run.findings.filter((item) => item.group === "solve").map((item) => [item.diagnostic.code, item.diagnostic.severity])).toContainEqual(["MATERIAL_INPUT_MISSING", "blocking"]);
  expect(run.findings.some((item) => item.group === "library")).toBe(false);
  expect(run.libraryNotCompleted).toBe("invented library store failure");
  expect(readSolveGate(model, run).kind).toBe("withheld");

  const view = render(<ModelChecksPanel model={model} run={run} onRun={() => {}} onSelectEntity={() => {}} />);
  expect(view.getByTestId("model-checks-library-not-completed")).toHaveTextContent("The library-reference findings could not be collected (invented library store failure); that group is not complete.");
  const library = view.container.querySelector('[data-group="library"]') as HTMLElement;
  expect(within(library).getByText("not completed")).toBeInTheDocument();
  expect(view.queryByTestId("model-checks-solve-unchecked")).toBeNull();
});

it("says a check that started and failed did not complete, not that it did not run", async () => {
  const model = await loadPreviewModel();
  const view = render(<ModelChecksPanel model={model} run={modelChecksNotCompleted(model, new Error("invented failure"))} onRun={() => {}} onSelectEntity={() => {}} />);
  expect(view.getByTestId("model-checks-solve-unchecked")).toHaveTextContent("The model checks did not complete: invented failure");
  expect(view.getByTestId("model-checks-solve-unchecked")).not.toHaveTextContent("did not run");
});

describe("the engine's checks say whether they did not run or did not complete, through the real catch paths", () => {
  afterEach(() => {
    vi.restoreAllMocks();
    invokeMock.mockReset();
    delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  });
  const native = () => { (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {}; };
  async function drawer(run: ModelCheckRun) {
    const view = render(<ModelChecksPanel model={run.model} run={run} onRun={() => {}} onSelectEntity={() => {}} />);
    const message = view.getByTestId("model-checks-solve-unchecked").textContent ?? "";
    const labels = Array.from(view.container.querySelectorAll("[data-group] > .muted")).map((item) => item.textContent);
    view.unmount();
    return { message, labels };
  }

  it("a native check that fails after admission did not complete", async () => {
    const model = await loadPreviewModel();
    native();
    invokeMock.mockRejectedValue(new Error("invented worker termination"));
    const run = await runModelChecks(model);
    expect(run.solve).toMatchObject({ kind: "unavailable", started: true });
    const { message, labels } = await drawer(run);
    expect(message).toBe("The model checks did not complete: Error: invented worker termination");
    expect(labels).toEqual(["not completed", "not completed", "not completed"]);
  });

  it("a native refusal to a running worker did not complete", async () => {
    const model = await loadPreviewModel();
    native();
    invokeMock.mockRejectedValue({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "worker_footprint_exceeded", grant_bytes: 2, footprint_bytes: 3 }, pool: null });
    const run = await runModelChecks(model);
    expect(run.solve).toMatchObject({ kind: "unavailable", started: true });
    expect((await drawer(run)).message).toMatch(/^The model checks did not complete: Refused by the native memory pool/);
  });

  it("the pool's refusal to admit the check, before its start, did not run", async () => {
    const model = await loadPreviewModel();
    native();
    invokeMock.mockRejectedValue({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "worker_limit_reached", running: 4, limit: 4 }, pool: null });
    const run = await runModelChecks(model);
    expect(run.solve).toEqual({ kind: "unavailable", reason: expect.stringContaining("Model checks could not start because all worker slots are occupied") });
    const { message, labels } = await drawer(run);
    expect(message).toMatch(/^The model checks did not run: Refused by the native memory pool: Model checks could not start/);
    expect(labels).toEqual(["not run", "not run", "not run"]);
  });

  it("a retired request is never sent, and did not run", async () => {
    const model = await loadPreviewModel();
    native();
    const run = await runModelChecks(model, () => [], { stillCurrent: () => false });
    expect(invokeMock).not.toHaveBeenCalled();
    expect(run.solve).toMatchObject({ kind: "unavailable" });
    expect(run.solve).not.toHaveProperty("started");
    expect((await drawer(run)).message).toMatch(/^The model checks did not run: /);
  });

  it("a wasm call that throws did not complete", async () => {
    const model = await loadPreviewModel();
    vi.spyOn(wasmEngine, "loadedWasmEngine").mockReturnValue({ modelChecksJson: () => { throw new Error("invented wasm trap"); } } as never);
    const run = await runModelChecks(model);
    expect(run.solve).toMatchObject({ kind: "unavailable", started: true });
    expect((await drawer(run)).message).toBe("The model checks did not complete: The model checks did not return a report: Error: invented wasm trap");
  });

  it("the engine not loaded did not run", async () => {
    const model = await loadPreviewModel();
    vi.spyOn(wasmEngine, "loadedWasmEngine").mockReturnValue(null);
    const run = await runModelChecks(model);
    expect(run.solve).not.toHaveProperty("started");
    const { message, labels } = await drawer(run);
    expect(message).toMatch(/^The model checks did not run: The model checks are in the wasm engine, which is not loaded/);
    expect(labels).toEqual(["not run", "not run", "not run"]);
  });
});

describe("the sentence beside evidence kept over a later check that could not finish", () => {
  async function blockingRun(model: PreviewModel): Promise<ModelCheckRun> {
    const run = await runModelChecks(model);
    return { ...run, laterCheckNotCompleted: "invented admission refusal" };
  }
  async function emptiedModel() {
    const model = JSON.parse(JSON.stringify(await loadPreviewModel())) as PreviewModel;
    model.pipe_segments.find((pipe) => pipe.id === "pipe:P-100")!.material = "";
    return model;
  }

  it("says the findings still apply only while the drawer shows a run of the same model", async () => {
    const model = await emptiedModel();
    const run = await blockingRun(model);
    const view = render(<ModelChecksPanel model={model} run={run} onRun={() => {}} onSelectEntity={() => {}} />);
    expect(view.getByTestId("model-checks-later-not-completed")).toHaveTextContent("A later run of the checks on this model could not finish (invented admission refusal). The findings below are from the last run that finished, and they still apply.");
    expect(view.queryByTestId("model-checks-stale")).toBeNull();
  });

  it("names the evidence as about the earlier model once the model has changed, and never says it still applies", async () => {
    const model = await emptiedModel();
    const run = await blockingRun(model);
    const edited = structuredClone(model);
    const view = render(<ModelChecksPanel model={edited} run={run} onRun={() => {}} onSelectEntity={() => {}} />);
    expect(view.getByTestId("model-checks-stale")).toHaveTextContent("The model has changed since these checks ran");
    const sentence = view.getByTestId("model-checks-later-not-completed");
    expect(sentence).toHaveTextContent("A later run of the checks on the earlier model could not finish (invented admission refusal). The findings below are from the last run that finished on that earlier model.");
    expect(view.container).not.toHaveTextContent("still apply");
  });

  it("names a kept refusal with no findings as the refusal, above it, and never as findings", async () => {
    const model = await loadPreviewModel();
    const run: ModelCheckRun = {
      model, solve: { kind: "refused", refusal: "request: missing field `outside_diameter`" }, findings: [],
      notShown: { solve: 0, slots: 0, library: 0 }, notShownBlocking: 0, notShownBlockingByGroup: { solve: 0, slots: 0, library: 0 },
      laterCheckNotCompleted: "invented admission refusal"
    };
    const view = render(<ModelChecksPanel model={model} run={run} onRun={() => {}} onSelectEntity={() => {}} />);
    const sentence = view.getByTestId("model-checks-later-not-completed");
    expect(sentence).toHaveTextContent("A later run of the checks on this model could not finish (invented admission refusal). The refusal below is from the last run that finished, and it still applies.");
    expect(sentence).not.toHaveTextContent("findings");
    const refusal = view.getByTestId("model-checks-solve-unchecked");
    expect(sentence.compareDocumentPosition(refusal) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  });
});
