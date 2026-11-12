// DEL-04-11 REQ-008: a run's application prepared together with its precondition. The model, library and result are
// the committed four-span input and the product's four-span result (two constant selections, support:D unfitted),
// whose binding holds as committed. The stored library is a test reader over a value the test replaces; a revised
// library is the same library with another name, under the same id.
import { describe, expect, it, vi } from "vitest";
import fourSpanInput from "../../../../../fixtures/results/hanger_sizing_input_four_span.json";
import fourSpanResult from "../../../../../fixtures/results/hanger_sizing_result_four_span.json";
import type { PreviewModel } from "../../types";
import type { HangerSizingSizedDocument } from "../results/hangerSizingResult";
import { HANGER_SIZING_RUN_NOTHING_TEXT, prepareRunApplication } from "./runApplication";
import { prepareSizedApplication, type StoredHangerLibraryReader } from "./sizedApplication";
import { HANGER_SIZING_LIBRARY_CHANGED_TEXT, HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT, HANGER_SIZING_MODEL_CHANGED_TEXT } from "./sizedSelection";

const model = () => structuredClone(fourSpanInput.request.model) as unknown as PreviewModel;
const library = () => structuredClone(fourSpanInput.library) as Record<string, unknown>;
const result = () => structuredClone(fourSpanResult) as unknown as HangerSizingSizedDocument;
const PROJECT = fourSpanInput.request.model.project.id;
const revised = () => {
  const changed = library();
  (changed.hanger_library as Record<string, unknown>).name = "Invented hanger sizing fixture, revised";
  return changed;
};

/** A stored library the test can replace or remove. */
function store() {
  const stored: { document: Record<string, unknown> | null } = { document: library() };
  const read = vi.fn<StoredHangerLibraryReader>(async () => (stored.document === null ? { kind: "unavailable" } : { kind: "read", document: structuredClone(stored.document) }));
  return { stored, read };
}

describe("a run's application with its precondition", () => {
  it("is the run's batch, read from the stored library under the result's id, with a precondition that holds while that library does", async () => {
    const { stored, read } = store();
    const prepared = await prepareSizedApplication(result(), model(), PROJECT, read);
    if (prepared.kind !== "prepared") throw new Error(prepared.reason);
    expect(read).toHaveBeenCalledWith(PROJECT, "invented.hangers.sizing");
    const expected = await prepareRunApplication(result(), model(), library());
    if (expected.kind !== "prepared") throw new Error(expected.reason);
    const strip = (batch: typeof prepared.batch) => batch.operations.map(({ change, target }) => ({ target, before: change.before, after: change.after, note: change.source_note }));
    expect(strip(prepared.batch)).toEqual(strip(expected.batch));
    expect(prepared.changed.map((change) => change.support)).toEqual(["support:B", "support:C"]);
    expect(prepared.unchanged).toEqual(["support:D"]);

    expect(await prepared.precondition()).toBeNull();
    stored.document = revised();
    expect(await prepared.precondition()).toBe(HANGER_SIZING_LIBRARY_CHANGED_TEXT);
    stored.document = null;
    expect(await prepared.precondition()).toBe(HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT);
    stored.document = library();
    expect(await prepared.precondition()).toBeNull();
  });

  it("refuses with a closed word and its reason", async () => {
    const changedModel = model();
    changedModel.supports.find((support) => support.id === "support:D")!.provenance = "edited after sizing";
    expect(await prepareSizedApplication(result(), changedModel, PROJECT, store().read))
      .toEqual({ kind: "refused", refusal: "model_changed", reason: HANGER_SIZING_MODEL_CHANGED_TEXT });

    const revisedStore = store();
    revisedStore.stored.document = revised();
    expect(await prepareSizedApplication(result(), model(), PROJECT, revisedStore.read))
      .toEqual({ kind: "refused", refusal: "library_changed", reason: HANGER_SIZING_LIBRARY_CHANGED_TEXT });

    const missing = store();
    missing.stored.document = null;
    expect(await prepareSizedApplication(result(), model(), PROJECT, missing.read))
      .toEqual({ kind: "refused", refusal: "library_unavailable", reason: HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT });
    expect(await prepareSizedApplication(result(), model(), PROJECT, async () => { throw new Error("unreadable"); }))
      .toEqual({ kind: "refused", refusal: "library_unavailable", reason: HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT });

    const none = result();
    (none.result as { candidates: unknown[] }).candidates = none.result.candidates.filter((candidate) => candidate.outcome === "no_record_fits");
    expect(await prepareSizedApplication(none, model(), PROJECT, store().read))
      .toEqual({ kind: "refused", refusal: "nothing_to_apply", reason: HANGER_SIZING_RUN_NOTHING_TEXT });

    // A published constant load that the builder refuses (not a positive force) stops the whole run.
    const refusedLoad = result();
    (refusedLoad.result.candidates[1].selected as { load_n: number }).load_n = -1;
    const stopped = await prepareSizedApplication(refusedLoad, model(), PROJECT, store().read);
    expect(stopped.kind === "refused" && stopped.refusal).toBe("candidate_not_applicable");
    expect(stopped.kind === "refused" && stopped.reason).toMatch(/^support:C: A sized load is not a positive finite force/);
  });
});
