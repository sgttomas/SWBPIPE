// W-15 slice c (DEL-03-05 Design `eccentric-reducer-offset.md` §4, P11): the report's
// component provenance carries a reducer's entered eccentric offset, its convention
// and the reference-section meaning of the body's stress rows, scaled or not; other
// components keep their record unchanged. Every value is invented.
import { describe, expect, it } from "vitest";
import type { MechanicsResult, PreviewModel } from "../../types";
import { ECCENTRIC_OFFSET_CONVENTION, ECCENTRIC_REDUCER_STRESS_BASIS } from "../component-creation/eccentricOffsetConvention";
import { reportComponentProvenance } from "./ReportPanel";

const result = { results: [] } as unknown as MechanicsResult;

function model(components: PreviewModel["components"]): PreviewModel {
  return { components } as unknown as PreviewModel;
}

describe("report component provenance: eccentric reducer", () => {
  it("states the offset, its convention and the reference-section stress basis for a reducer with an offset, scaled or not", () => {
    for (const modifiers of [undefined, { stiffness_scaling_user_value: { value: 1000, unit: "1" }, source_reference: "invented" }]) {
      const [record] = reportComponentProvenance(model([{ id: "component:R", label: "R", kind: "reducer", node: "node:A", provenance: "invented",
        geometry: { rigid_pipe_ref: "pipe:AB", eccentric_offset: { x: 0, y: -50, z: 0, unit: "mm" } }, ...(modifiers ? { modifiers } : {}) }]), result);
      expect(record).toMatchObject({ eccentric_offset: { x: 0, y: -50, z: 0, unit: "mm" }, eccentric_offset_convention: ECCENTRIC_OFFSET_CONVENTION,
        eccentric_reducer_stress_basis: ECCENTRIC_REDUCER_STRESS_BASIS });
      expect(ECCENTRIC_REDUCER_STRESS_BASIS).toContain("whether or not a stiffness scale is applied");
    }
  });

  it("adds nothing to a reducer without an offset or to another kind", () => {
    const records = reportComponentProvenance(model([
      { id: "component:R", label: "R", kind: "reducer", node: "node:A", provenance: "invented", geometry: { rigid_pipe_ref: "pipe:AB" } },
      { id: "component:V", label: "V", kind: "valve", node: "node:A", provenance: "invented",
        geometry: { rigid_pipe_ref: "pipe:AB", eccentric_offset: { x: 0, y: -50, z: 0, unit: "mm" } } }
    ]), result);
    for (const record of records) expect(record).not.toHaveProperty("eccentric_offset");
  });

  it("states option A of R-c1 (convention test)", () => {
    expect(ECCENTRIC_OFFSET_CONVENTION).toContain("from the centerline at this component's node to the centerline at the other end of its body");
  });
});
