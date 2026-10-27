// The report's spring hanger rule, shared with A8's "Hangers" filter (DEL-07-05 Design
// `displacement-extremes.md` §2.1). Every `SupportFamily` value, an absent, null, empty and
// unknown family, crossed with every `SpringHanger.hanger_type` value, no hanger record, a record
// without a type, and an empty and unknown type: the shared function equals the report's rule as
// it stood before extraction, and A8 classifies and filters each support the same way.
import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { healthStanding, overlayModel, overlayResult, row } from "../../test-support/resultOverlayFixtures";
import type { PreviewModel } from "../../types";
import { FILTER_HANGERS, buildDisplacementExtremes, displacementExtremesView } from "../results/displacementExtremes";
import { isSpringHangerSupport } from "./springHangerSupport";

type Support = PreviewModel["supports"][number];

const schema = JSON.parse(readFileSync(path.join(process.cwd(), "..", "..", "schemas", "model.schema.yaml"), "utf8")) as {
  $defs: { SupportFamily: { enum: string[] }; SpringHanger: { properties: { hanger_type: { enum: string[] } } } };
};
const FAMILIES = schema.$defs.SupportFamily.enum;
const HANGER_TYPES = schema.$defs.SpringHanger.properties.hanger_type.enum;
const ABSENT = Symbol("absent");

/** The report's predicate verbatim as it stood inline in `ReportPanel.tsx` and
 * `renderableReportInput.ts` before it was extracted: the reference the shared rule must keep. */
function reportRuleBeforeExtraction(support: Support): boolean {
  return (
    support.family === "variable_spring_hanger" ||
    support.family === "constant_effort_support" ||
    support.hanger?.hanger_type === "variable_spring_hanger" ||
    support.hanger?.hanger_type === "constant_effort_support"
  );
}

const familyCases: Array<string | null | typeof ABSENT> = [...FAMILIES, ABSENT, null, "", "strut"];
const hangerCases: Array<Record<string, unknown> | null | typeof ABSENT> = [
  ABSENT, null, {}, ...HANGER_TYPES.map((hanger_type) => ({ hanger_type })), { hanger_type: "" }, { hanger_type: "strut" }
];

function grid(): Support[] {
  const supports: Support[] = [];
  familyCases.forEach((family, f) => hangerCases.forEach((hanger, h) => {
    const support: Record<string, unknown> = { id: `support:${f}-${h}`, node: `node:${f}-${h}`, restraints: [] };
    if (family !== ABSENT) support.family = family;
    if (hanger !== ABSENT) support.hanger = hanger;
    supports.push(support as unknown as Support);
  }));
  return supports;
}

describe("spring hanger rule shared by the report and A8", () => {
  it("covers the schema's vocabularies, spring_hanger included", () => {
    expect(FAMILIES).toContain("spring_hanger");
    expect([...HANGER_TYPES].sort()).toEqual(["constant_effort_support", "spring_hanger", "variable_spring_hanger"]);
    expect(grid()).toHaveLength(familyCases.length * hangerCases.length);
  });

  it("equals the report's rule before extraction for every family and hanger type", () => {
    for (const support of grid()) expect([support.id, isSpringHangerSupport(support)]).toEqual([support.id, reportRuleBeforeExtraction(support)]);
  });

  it("does not count spring_hanger, by family or by hanger type", () => {
    expect(isSpringHangerSupport({ id: "s", node: "n", family: "spring_hanger", restraints: [] } as unknown as Support)).toBe(false);
    expect(isSpringHangerSupport({ id: "s", node: "n", family: "spring", hanger: { hanger_type: "spring_hanger" }, restraints: [] } as unknown as Support)).toBe(false);
    expect(isSpringHangerSupport({ id: "s", node: "n", family: "spring_hanger", hanger: { hanger_type: "constant_effort_support" }, restraints: [] } as unknown as Support)).toBe(true);
  });

  it("gives A8's support classification and Hangers filter the same answer for every support", () => {
    const supports = grid();
    const nodes = supports.map((support, index) => ({ id: support.node, label: "", position: { x: index, y: 0, z: 0 } }));
    const model = { ...overlayModel(), nodes, supports } as unknown as PreviewModel;
    const data = buildDisplacementExtremes({ model, result: overlayResult([row("d", "global_nodal_displacement_x", nodes[0].id, 1, "mm")]), standing: healthStanding({ eligible: true }) });
    expect(data.state).toBe("available");
    const flags = new Map(data.nodes.flatMap((node) => node.supports.map((item) => [item.supportId, item.hanger] as const)));
    for (const support of supports) expect([support.id, flags.get(support.id)]).toEqual([support.id, reportRuleBeforeExtraction(support)]);
    const listed = displacementExtremesView(data, { caseKeys: null, filter: FILTER_HANGERS, sort: "model" }).rows.map((item) => item.nodeId);
    expect(listed).toEqual(supports.filter(reportRuleBeforeExtraction).map((support) => support.node));
  });

  it("is the one definition: the report's files import it and keep no inline copy", () => {
    for (const file of ["ReportPanel.tsx", "renderableReportInput.ts"]) {
      const source = readFileSync(path.join(process.cwd(), "src", "features", "report", file), "utf8");
      expect(source).toContain('import { isSpringHangerSupport } from "./springHangerSupport";');
      expect(source).not.toMatch(/hanger\?\.hanger_type === "(variable_spring_hanger|constant_effort_support)"/);
      expect(source).not.toMatch(/support\.family === "(variable_spring_hanger|constant_effort_support)"/);
    }
  });
});
