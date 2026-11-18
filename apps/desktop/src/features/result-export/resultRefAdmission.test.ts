/** The Current builder's row binding admits and refuses exactly as it did when it searched the
 * record's result references from the start for every row.
 *
 * Each case below is admitted twice from equal inputs: once with the maintained lookup and
 * once with that search put in its place. The two outcomes (the whole document, or the
 * refusal) must be equal, and the refusal each case is for must be the one named.
 *
 * Unit transport replay of committed producer bytes through mocked IPC; not a native UI
 * qualification witness. */
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
const lookup = vi.hoisted(() => ({ searchFromStart: false, calls: { maintained: 0, searchFromStart: 0 } }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
vi.mock("./resultRefLookup", async importOriginal => {
  const actual = await importOriginal<typeof import("./resultRefLookup")>();
  return {
    firstMatchResultRefLookup: <T extends { result_ref: { ref: unknown } }>(refs: readonly T[]) => {
      if (!lookup.searchFromStart) { lookup.calls.maintained += 1; return actual.firstMatchResultRefLookup(refs); }
      lookup.calls.searchFromStart += 1;
      return (id: unknown) => refs.find(x => x.result_ref.ref === id);
    },
  };
});
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { buildAnalysisRunPreview, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentResultExport } from "./resultExportAdapter";

afterEach(() => { invokeMock.mockReset(); delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__; lookup.searchFromStart = false; });

async function current() {
  const { model } = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(createNativeMechanicsReplay({ profile: "preview" }).invoke);
  const result = await runPreviewMechanics(model);
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "open_pipe_stress_product_physics@0.2.0", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  return { model, result, inputManifest, analysisRun };
}
type Session = Awaited<ReturnType<typeof current>>;
type Refs = Session["analysisRun"]["analysis_run"]["result_refs"];
/** A named refusal by its text; an unnamed error by its class, whose message is the engine's. */
type Outcome = { document: string } | { refused: string } | { thrown: string };

/** The record with its result references replaced; the registered result object is kept. */
const withRefs = (args: Session, edit: (refs: Refs) => Refs): Session => {
  const analysisRun = structuredClone(args.analysisRun);
  analysisRun.analysis_run.result_refs = edit(analysisRun.analysis_run.result_refs);
  return { ...args, analysisRun };
};
async function admit(args: Session, searchFromStart: boolean): Promise<Outcome> {
  lookup.searchFromStart = searchFromStart;
  const before = lookup.calls[searchFromStart ? "searchFromStart" : "maintained"];
  try { return { document: JSON.stringify(await buildCurrentResultExport(args)) }; }
  catch (error) { return error instanceof TypeError ? { thrown: "TypeError" } : { refused: (error as Error).message }; }
  finally { expect(lookup.calls[searchFromStart ? "searchFromStart" : "maintained"]).toBe(before + 1); }
}
async function bothWays(args: Session): Promise<Outcome> {
  const searched = await admit(args, true), maintained = await admit(args, false);
  expect(maintained).toStrictEqual(searched);
  return maintained;
}
const ROW_BINDING = { refused: "CURRENT_ROW_BINDING_MISMATCH" };

describe("Current row binding against the search it replaced", () => {
  it("admits the maintained Current pair to the same document", async () => {
    const args = await current();
    expect(args.result.results.length).toBeGreaterThan(2);
    const outcome = await bothWays(args);
    expect("document" in outcome).toBe(true);
    expect(JSON.parse((outcome as { document: string }).document).result_envelope.row_accounting).toHaveLength(args.result.results.length);
  });

  it("refuses a duplicate reference, wherever the pair sits", async () => {
    const args = await current(), last = args.result.results.length - 1;
    // The row whose reference was overwritten has none; the list is still the rows' length.
    for (const [from, to] of [[0, 1], [1, 0], [0, last], [last, 0], [last - 1, last]])
      expect(await bothWays(withRefs(args, refs => { refs[to] = structuredClone(refs[from]); return refs; })), `${from} over ${to}`).toStrictEqual(ROW_BINDING);
    // Every entry the same reference.
    expect(await bothWays(withRefs(args, refs => refs.map(() => structuredClone(refs[0]))))).toStrictEqual(ROW_BINDING);
  });

  it("refuses a duplicate whose first entry is the unbound one", async () => {
    const args = await current();
    // Both entries name row 0. The first carries row 1's hash, so row 0 is refused on its first
    // match although the later entry is its true one. Row 1 has no entry, so the case is
    // refused whichever entry row 0 takes: the outcome does not distinguish the two.
    const outcome = await bothWays(withRefs(args, refs => {
      const [first, second] = [structuredClone(refs[0]), structuredClone(refs[1])];
      refs[0] = { ...second, result_ref: first.result_ref }; refs[1] = first;
      return refs;
    }));
    expect(outcome).toStrictEqual(ROW_BINDING);
  });

  it("refuses a missing reference", async () => {
    const args = await current(), last = args.result.results.length - 1;
    for (const at of [0, 1, last])
      expect(await bothWays(withRefs(args, refs => { refs[at].result_ref.ref = "result:absent"; return refs; })), `entry ${at}`).toStrictEqual(ROW_BINDING);
    // Values the record's JSON guard admits; it refuses `undefined` before the row binding.
    for (const ref of ["", null, 0])
      expect(await bothWays(withRefs(args, refs => { (refs[1].result_ref as { ref: unknown }).ref = ref; return refs; })), `reference ${String(ref)}`).toStrictEqual(ROW_BINDING);
  });

  it("refuses a reference of the wrong type, a changed row hash and a second hash", async () => {
    const args = await current();
    const edits: Array<(refs: Refs) => void> = [
      refs => { refs[2].result_ref.object_type = "Wrong"; },
      refs => { refs[2].hash_refs[0].value = refs[1].hash_refs[0].value; },
      refs => { refs[2].hash_refs.push(structuredClone(refs[2].hash_refs[0])); },
      refs => { refs[2].hash_refs = []; },
    ];
    for (const [index, edit] of edits.entries())
      expect(await bothWays(withRefs(args, refs => { edit(refs); return refs; })), `edit ${index}`).toStrictEqual(ROW_BINDING);
  });

  it("refuses references that are not held in an array", async () => {
    const args = await current();
    // The record's JSON guard, its text and the length comparison all admit these two.
    const arrayLike = await bothWays(withRefs(args, refs => ({ ...refs, length: refs.length }) as unknown as Refs));
    const noPrototype = await bothWays(withRefs(args, refs => Object.setPrototypeOf(refs, null) as Refs));
    for (const outcome of [arrayLike, noPrototype]) expect(outcome).toStrictEqual({ thrown: "TypeError" });
  });

  it("gives the same answer for references in another order", async () => {
    const args = await current();
    // The row binding finds each row's reference wherever it sits; what follows it decides.
    const reversed = await bothWays(withRefs(args, refs => refs.reverse()));
    const rotated = await bothWays(withRefs(args, refs => [...refs.slice(1), refs[0]]));
    for (const outcome of [reversed, rotated]) {
      expect("refused" in outcome).toBe(true);
      expect(outcome).not.toStrictEqual(ROW_BINDING);
    }
  });
});
