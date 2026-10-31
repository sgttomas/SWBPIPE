// G7/G8 draft helpers (DEL-07-03 `Work/authoring-speed.md` §3). Invented ids.
import { describe, expect, it } from "vitest";
import { addEverySource, copyAnalysisState, copyPressureRegions, ELEMENT_STATE_UNEDITED, fillFromTemplate, fillTargets, rederiveRegionId, SUPPORT_STATE_UNEDITED, supportStateCopyRefusal } from "./bulkFill";

const PIPES = ["pipe:A", "pipe:B", "pipe:C", "pipe:D"];

describe("fill targets", () => {
  it("selects every target, an inclusive range in model order, or the chosen ones in model order", () => {
    expect(fillTargets(PIPES, { kind: "missing" })).toEqual(PIPES);
    expect(fillTargets(PIPES, { kind: "range", from: "pipe:B", to: "pipe:C" })).toEqual(["pipe:B", "pipe:C"]);
    expect(fillTargets(PIPES, { kind: "chosen", ids: ["pipe:D", "pipe:A"] })).toEqual(["pipe:A", "pipe:D"]);
  });
  it("selects nothing for a reversed range or an unknown end", () => {
    expect(fillTargets(PIPES, { kind: "range", from: "pipe:C", to: "pipe:B" })).toEqual([]);
    expect(fillTargets(PIPES, { kind: "range", from: "pipe:X", to: "pipe:B" })).toEqual([]);
  });
});

describe("fill from a template row", () => {
  const template = { pipe_ref: "pipe:A", material_selection: { kind: "exact_point", material_ref: "material:M" }, operating_temperature: { value: "210", unit: "degC" } };
  it("appends an independent copy per target the list does not name, and keeps existing rows in place", () => {
    const existing = { pipe_ref: "pipe:C", thermal_state: { kind: "unchanged_reference", provenance: "own" } };
    const { rows, added, skipped } = fillFromTemplate([template, existing], template, "pipe_ref", PIPES);
    expect(added).toEqual(["pipe:B", "pipe:D"]);
    expect(skipped).toEqual(["pipe:A", "pipe:C"]);
    expect(rows.map(r => r.pipe_ref)).toEqual(["pipe:A", "pipe:C", "pipe:B", "pipe:D"]);
    expect(rows[1]).toBe(existing);
    expect(rows[2]).toEqual({ ...template, pipe_ref: "pipe:B" });
    (rows[2].material_selection as { kind: string }).kind = "changed";
    expect(template.material_selection.kind).toBe("exact_point");
    expect((rows[3].material_selection as { kind: string }).kind).toBe("exact_point");
  });
  it("copies only what the template holds: an absent member stays absent", () => {
    const { rows } = fillFromTemplate([{ pipe_ref: "pipe:A" }], { pipe_ref: "pipe:A" }, "pipe_ref", ["pipe:B"]);
    expect(rows[1]).toEqual({ pipe_ref: "pipe:B" });
  });
});

describe("every primitive of a case", () => {
  it("adds each unlisted primitive with the entered factor, and requires the factor", () => {
    const { rows, added } = addEverySource([{ source_ref: "p:2", factor: "0.5" }], ["p:1", "p:2", "p:3"], "1");
    expect(added).toEqual(["p:1", "p:3"]);
    expect(rows).toEqual([{ source_ref: "p:2", factor: "0.5" }, { source_ref: "p:1", factor: "1" }, { source_ref: "p:3", factor: "1" }]);
    expect(() => addEverySource([], ["p:1"], "")).toThrow(/factor/);
    expect(() => addEverySource([], ["p:1"], "one")).toThrow(/finite/);
  });
});

describe("copy from another case", () => {
  it("copies an analysis state and leaves out load sources naming primitives this case does not hold", () => {
    const source = { contract: "c", element_states: [{ pipe_ref: "pipe:A" }], load_sources: [{ source_ref: "p:sus:1", factor: 1 }, { source_ref: "p:shared", factor: 2 }] };
    const { draft, dropped } = copyAnalysisState(source, ["p:shared", "p:sam:1"]);
    expect(draft.load_sources).toEqual([{ source_ref: "p:shared", factor: 2 }]);
    expect(dropped).toEqual(["p:sus:1"]);
    expect(draft.element_states).toEqual(source.element_states);
    expect(draft.element_states).not.toBe(source.element_states);
  });
  it("re-derives region ids where the source case's token is a whole segment, and keeps other ids", () => {
    expect(rederiveRegionId("region:RS-SUS-RG-1", "load:RS-SUS", "load:RS-OPE")).toBe("region:RS-OPE-RG-1");
    expect(rederiveRegionId("region:RS-SUSX-RG-1", "load:RS-SUS", "load:RS-OPE")).toBe("region:RS-SUSX-RG-1");
    expect(rederiveRegionId("region:main", "load:RS-SUS", "load:RS-OPE")).toBe("region:main");
    const copied = copyPressureRegions([{ id: "region:RS-SUS-RG-1", member_pipe_ids: ["pipe:A"] }], "load:RS-SUS", "load:RS-SAM");
    expect(copied).toEqual([{ id: "region:RS-SAM-RG-1", member_pipe_ids: ["pipe:A"] }]);
  });
});

describe("review findings (PR #38)", () => {
  it("N1: never copies a row's own unshown override or reference, and reports it", () => {
    const template = { pipe_ref: "pipe:A", analysis_basis_override: { reason: "only pipe:A" }, mass_state_ref: "mass:A", material_selection: { kind: "exact_point" } };
    const { rows, omitted } = fillFromTemplate([template], template, "pipe_ref", ["pipe:B"], ELEMENT_STATE_UNEDITED);
    expect(rows[1]).toEqual({ pipe_ref: "pipe:B", material_selection: { kind: "exact_point" } });
    expect(omitted).toEqual(["analysis_basis_override", "mass_state_ref"]);
    const copied = copyAnalysisState({
      element_states: [template],
      support_states: [
        { support_ref: "support:S", participation: { kind: "active_model_device" }, base_motion: { x: 1 }, device_reference: "d" },
        { support_ref: "support:L", participation: { kind: "locked_equivalent_support", components: ["UX"] } }
      ]
    }, []);
    expect(copied.draft.element_states).toEqual([{ pipe_ref: "pipe:A", material_selection: { kind: "exact_point" } }]);
    expect(copied.draft.support_states).toEqual([{ support_ref: "support:S", participation: { kind: "active_model_device" } }]);
    expect(copied.omitted.sort()).toEqual([...ELEMENT_STATE_UNEDITED, ...SUPPORT_STATE_UNEDITED].sort());
    expect(copied.supportsLeftOut).toEqual(["support:L"]);
    expect(supportStateCopyRefusal({ participation: { kind: "locked_equivalent_support" } })).toMatch(/components/);
  });
  it("N2/N4: refuses a source without regions and ids that would collide, so the draft is never replaced", () => {
    expect(() => copyPressureRegions([], "load:A", "load:B")).toThrow(/no pressure regions/);
    expect(() => copyPressureRegions(undefined, "load:A", "load:B")).toThrow(/no pressure regions/);
    expect(() => copyPressureRegions([{ id: "region:A-RG-1" }, { id: "region:B-RG-1" }], "load:A", "load:B")).toThrow(/region:B-RG-1/);
  });
});
