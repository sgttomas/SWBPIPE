// DEL-04-11 REQ-008 / AC-008: every selection of one sizing run, applied as one batch on one acceptance and restored by
// one undo. The session is the desktop's own (`useWorkspaceSession`) running the actual wasm operation engine; the run
// is a variable and a constant selection with one no-fit support on the invented four-span sizing input, formed and
// bound as in `../hanger-sizing/runApplication.test.ts`. The session's model is that input's model with the session's
// `diagnostics: []`, which the headless input does not carry, so the run is bound to the session's model. The product's
// own four-span result (`fixtures/results/hanger_sizing_result_four_span.json`) is bound to the headless input's model
// itself, and its batch is applied by the same wasm engine call the session's acceptance makes.
import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import fourSpanInput from "../../../../../fixtures/results/hanger_sizing_input_four_span.json";
import fourSpanResult from "../../../../../fixtures/results/hanger_sizing_result_four_span.json";
import * as previewService from "../../services/previewService";
import { canonicalJsonString, canonicalSha256HexCheckedV1, computeModelHash } from "../../services/hashService";
import { applyOperationBatch } from "../../services/operationBatchService";
import type { PreviewModel } from "../../types";
import type { HangerSizingCandidate, HangerSizingSizedDocument } from "../results/hangerSizingResult";
import { prepareRunApplication } from "../hanger-sizing/runApplication";
import { useWorkspaceSession } from "../workspace/workspaceSession";

const model = () => structuredClone(fourSpanInput.request.model) as unknown as PreviewModel;
const library = () => structuredClone(fourSpanInput.library) as Record<string, unknown>;
const LIBRARY = "invented.hangers.sizing";

async function recordSha(lib: Record<string, unknown>, hangerId: string) {
  return canonicalSha256HexCheckedV1((lib.hanger_records as { hanger_id: string }[]).find((entry) => entry.hanger_id === hangerId));
}

const common = (support: string, node: string) => ({
  support, node, dof: "UZ" as const, family_policy: "variable_else_constant" as const, w_n: 5000, u_cold_m: 0, operating: [], span_m: [0, 0.001] as const,
  required_travel_m: 0.005, failure_counts: { variable: null, constant: null }
});

async function mixedRun(source: PreviewModel, lib: Record<string, unknown>): Promise<HangerSizingSizedDocument> {
  const candidates: HangerSizingCandidate[] = [
    { ...common("support:B", "node:B"), outcome: "selected",
      selected: { family: "variable_spring", hanger_id: "VS-200-L", record_sha256: await recordSha(lib, "VS-200-L"), k_n_per_m: 200000, f_cold_n: 5197.5,
        f_cold_condition_n: 5197.5, envelope_n: [4600, 5716], operating: [{ case: "load:O1", f_n: 5000, variation: 0.04 }] },
      governing: { travel_case: "load:O1", variation_case: "load:O1" },
      verification_assumption: { role: "spring", k_n_per_m: 200000, f_cold_n: 5197.5 }, verification: null as never },
    { ...common("support:C", "node:C"), outcome: "selected",
      selected: { family: "constant_effort", hanger_id: "CE-1", record_sha256: await recordSha(lib, "CE-1"), load_n: 3714.285714285714 },
      governing: { travel_case: "load:O1", variation_case: null },
      verification_assumption: { role: "force", force_n: 3714.285714285714 }, verification: null as never },
    { ...common("support:D", "node:D"), outcome: "no_record_fits", selected: null, governing: null,
      verification_assumption: { role: "force", force_n: 4571.428571428572 }, verification: null }
  ];
  return {
    status: "sized",
    inputs: {
      canonical_profile: "openpipestress_jcs_ijson_v1",
      sizing_request: { design_operating_case: "load:O1" },
      sizing_request_sha256: "5".repeat(64),
      solve_request: { sha256: await canonicalSha256HexCheckedV1({ model: source, materials: [] }) },
      library: { library_id: LIBRARY, sha256: await canonicalSha256HexCheckedV1(lib) }
    },
    result: { candidates }
  } as unknown as HangerSizingSizedDocument;
}

async function session() {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue({ ...model(), diagnostics: [] } as PreviewModel);
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}

type Session = Awaited<ReturnType<typeof session>>;
const support = (source: PreviewModel, id: string) => source.supports.find((entry) => entry.id === id)!;

afterEach(() => { vi.restoreAllMocks(); });

describe("a sizing run's selections through the operation route", () => {
  it("are applied as one batch on one acceptance, leave the no-fit support byte-identical, and one undo restores the model exactly", async () => {
    const result = await session();
    const before = structuredClone(result.current.model.model!);
    const original = await canonicalJsonString(before);
    const document = await mixedRun(before, library());

    const prepared = await prepareRunApplication(document, result.current.model.model!, library());
    if (prepared.kind !== "prepared") throw new Error(prepared.reason);
    await act(async () => { await result.current.operations.handleQueueOperationBatch(prepared.batch); });
    expect(await canonicalJsonString(result.current.model.model!)).toBe(original);
    expect(result.current.operations.undoStack).toHaveLength(0);

    const queued = result.current.operations.queuedBatches;
    if (queued[0].requiresPreview) await act(async () => { await result.current.operations.handleRunOperationBatch(queued[0], false); });
    const entry = result.current.operations.queuedBatches[0];
    await act(async () => { await result.current.operations.handleRunOperationBatch(entry, true); });
    const outcome = result.current.operations.batchOutcomes[entry.key];

    // One batch, committed as one, with one receipt and one undo checkpoint.
    expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
    expect(outcome.simulation_disposition).toBe("committed_as_one_batch");
    expect(outcome.operation_outcomes).toHaveLength(prepared.batch.operations.length);
    expect(result.current.operations.batchReceipts).toHaveLength(1);
    expect(result.current.operations.undoStack).toHaveLength(1);
    expect(result.current.operations.queuedBatches).toHaveLength(0);

    const applied = result.current.model.model!;
    for (const operation of prepared.batch.operations.filter((candidate) => candidate.change.field_path === "configuration")) {
      const written = support(applied, operation.target.ref);
      const after = JSON.parse(operation.change.after);
      expect({ family: written.family, restraints: written.restraints, provenance: written.provenance, hanger: written.hanger }).toEqual(after);
      expect(written.provenance_record).toBeDefined();
    }
    expect(support(applied, "support:B").family).toBe("variable_spring_hanger");
    expect(support(applied, "support:C").family).toBe("constant_effort_support");
    expect(await canonicalJsonString(support(applied, "support:D"))).toBe(await canonicalJsonString(support(before, "support:D")));
    // Every other part of the model is as it was.
    const untouched = (source: PreviewModel) => ({ ...source, supports: source.supports.filter((entry) => !["support:B", "support:C"].includes(entry.id)) });
    expect(await canonicalJsonString(untouched(applied))).toBe(await canonicalJsonString(untouched(before)));

    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(result.current.operations.undoStack).toHaveLength(0);
    expect(result.current.operations.redoStack).toHaveLength(1);
    expect(await canonicalJsonString(result.current.model.model!)).toBe(original);
  });

  it("of the product's own four-span result are committed by the operation engine as one batch, leaving support:D byte-identical", async () => {
    const source = model();
    const prepared = await prepareRunApplication(structuredClone(fourSpanResult) as unknown as HangerSizingSizedDocument, source, library());
    if (prepared.kind !== "prepared") throw new Error(prepared.reason);
    const outcome = await applyOperationBatch(source, prepared.batch, await computeModelHash(source));
    expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
    expect(outcome.simulation_disposition).toBe("committed_as_one_batch");
    const applied = outcome.applied_model!;
    expect(support(applied, "support:B").hanger?.constant_load).toEqual({ value: 4571.428571428572, unit: "N" });
    expect(support(applied, "support:C").hanger?.constant_load).toEqual({ value: 3714.285714285714, unit: "N" });
    expect(await canonicalJsonString(support(applied, "support:D"))).toBe(await canonicalJsonString(support(source, "support:D")));
  });
});
