// The report's spring hanger rule, shared with A8's "Hangers" filter (DEL-07-05 Design
// `displacement-extremes.md` §2.1). Every `SupportFamily` value, an absent, null, empty, unknown and
// whitespace-padded family, crossed with every `SpringHanger.hanger_type` value, no hanger record, a
// record without a type, and an empty, unknown and whitespace-padded type: the shared function
// equals the solver's classification, taken as an explicit table from the Rust, and A8 classifies
// and filters each support the same way.
import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { healthStanding, overlayModel, overlayResult, row } from "../../test-support/resultOverlayFixtures";
import type { PreviewModel } from "../../types";
import { FILTER_HANGERS, buildDisplacementExtremes, displacementExtremesView } from "../results/displacementExtremes";
import { isSpringHangerSupport } from "./springHangerSupport";

type Support = PreviewModel["supports"][number];

const projectRoot = path.join(process.cwd(), "..", "..");
const schema = JSON.parse(readFileSync(path.join(projectRoot, "schemas", "model.schema.yaml"), "utf8")) as {
  $defs: { SupportFamily: { enum: string[] }; SpringHanger: { properties: { hanger_type: { enum: string[] } } } };
};
const FAMILIES = schema.$defs.SupportFamily.enum;
const HANGER_TYPES = schema.$defs.SpringHanger.properties.hanger_type.enum;
const ABSENT = Symbol("absent");
// The padded spelling the solver's own test solves as a hanger
// (`f3_invalid_family_blocks_every_payload_before_mechanics`, core/product_physics/src/lib.rs).
const PADDED = " spring_hanger ";

/** The solver's classification verbatim (`core/product_physics/src/lib.rs`). The table below is
 * read from it; the test fails if the Rust changes, so the table cannot silently go stale. */
const SOLVER_SOURCE = [
  `pub(crate) fn support_hanger_type(support: &PreviewSupport) -> Option<&str> {
    support
        .hanger
        .as_ref()
        .and_then(|hanger| hanger.hanger_type.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| support.family.as_deref().map(str::trim))
}`,
  `pub(crate) fn is_variable_spring_hanger(support: &PreviewSupport) -> bool {
    matches!(
        support_hanger_type(support),
        Some("variable_spring_hanger" | "spring_hanger")
    )
}`,
  `pub(crate) fn is_constant_effort_support(support: &PreviewSupport) -> bool {
    matches!(
        support_hanger_type(support),
        Some("constant_effort_support")
    )
}`,
  // The review rows' filter: the solver's hanger set is the union of the two.
  `.filter(|support| is_variable_spring_hanger(support) || is_constant_effort_support(support))`
];

/** The solver's answer for each hanger case, read from `SOLVER_SOURCE`: a non-empty trimmed explicit
 * type decides by itself; an absent, null or blank type leaves the decision to the family. */
const SOLVER_BY_HANGER: Record<string, boolean | "family"> = {
  "no hanger record": "family",
  "null hanger record": "family",
  "no hanger_type": "family",
  "hanger_type variable_spring_hanger": true,
  "hanger_type spring_hanger": true,
  "hanger_type constant_effort_support": true,
  'hanger_type ""': "family",
  'hanger_type "strut"': false,
  'hanger_type " spring_hanger "': true
};
/** ...and for each family, when the family decides (the trimmed family is one of the three). */
const SOLVER_BY_FAMILY: Record<string, boolean> = {
  "family anchor": false,
  "family guide": false,
  "family line_stop": false,
  "family vertical_support": false,
  "family spring": false,
  "family variable_spring_hanger": true,
  "family spring_hanger": true,
  "family constant_effort_support": true,
  "family nonlinear": false,
  "no family": false,
  "null family": false,
  'family ""': false,
  'family "strut"': false,
  'family " spring_hanger "': true
};

/** The report's predicate verbatim as it stood before this correction (at `c935a469c2`): it
 * counted neither `spring_hanger` spelling, did not trim, and let a hanger family count whatever the
 * explicit type said. Kept to document the corrected behaviour. */
function reportRuleBeforeCorrection(support: Support): boolean {
  return (
    support.family === "variable_spring_hanger" ||
    support.family === "constant_effort_support" ||
    support.hanger?.hanger_type === "variable_spring_hanger" ||
    support.hanger?.hanger_type === "constant_effort_support"
  );
}

const familyCases: Array<[string, string | null | typeof ABSENT]> = [
  ...FAMILIES.map((family): [string, string] => [`family ${family}`, family]),
  ["no family", ABSENT], ["null family", null], ['family ""', ""], ['family "strut"', "strut"], [`family ${JSON.stringify(PADDED)}`, PADDED]
];
const hangerCases: Array<[string, Record<string, unknown> | null | typeof ABSENT]> = [
  ["no hanger record", ABSENT], ["null hanger record", null], ["no hanger_type", {}],
  ...HANGER_TYPES.map((hanger_type): [string, Record<string, unknown>] => [`hanger_type ${hanger_type}`, { hanger_type }]),
  ['hanger_type ""', { hanger_type: "" }], ['hanger_type "strut"', { hanger_type: "strut" }], [`hanger_type ${JSON.stringify(PADDED)}`, { hanger_type: PADDED }]
];

function grid(): Support[] {
  const supports: Support[] = [];
  familyCases.forEach(([familyName, family], f) => hangerCases.forEach(([hangerName, hanger], h) => {
    const support: Record<string, unknown> = { id: `${familyName} | ${hangerName}`, node: `node:${f}-${h}`, restraints: [] };
    if (family !== ABSENT) support.family = family;
    if (hanger !== ABSENT) support.hanger = hanger;
    supports.push(support as unknown as Support);
  }));
  return supports;
}

function solverCounts(support: Support): boolean {
  const [familyName, hangerName] = support.id.split(" | ");
  const byHanger = SOLVER_BY_HANGER[hangerName];
  expect(byHanger, hangerName).toBeDefined();
  expect(SOLVER_BY_FAMILY[familyName], familyName).toBeDefined();
  return byHanger === "family" ? SOLVER_BY_FAMILY[familyName] : byHanger;
}

describe("spring hanger rule shared by the report and A8", () => {
  it("covers the schema's vocabularies, spring_hanger included", () => {
    expect(FAMILIES).toContain("spring_hanger");
    expect([...HANGER_TYPES].sort()).toEqual(["constant_effort_support", "spring_hanger", "variable_spring_hanger"]);
    expect(grid()).toHaveLength(familyCases.length * hangerCases.length);
    expect(Object.keys(SOLVER_BY_HANGER)).toEqual(hangerCases.map(([name]) => name));
    expect(Object.keys(SOLVER_BY_FAMILY)).toEqual(familyCases.map(([name]) => name));
  });

  it("takes its table from the solver's classification as it stands", () => {
    const solver = readFileSync(path.join(projectRoot, "core", "product_physics", "src", "lib.rs"), "utf8");
    for (const text of SOLVER_SOURCE) expect(solver).toContain(text);
  });

  it("equals the solver's classification for every family and hanger type", () => {
    for (const support of grid()) expect([support.id, isSpringHangerSupport(support)]).toEqual([support.id, solverCounts(support)]);
  });

  it("counts spring_hanger by family or by hanger type, which the report's rule before the correction left out", () => {
    const cases: Array<[Record<string, unknown>, boolean]> = [
      [{ family: "spring_hanger" }, true],
      [{ family: "spring", hanger: { hanger_type: "spring_hanger" } }, true],
      [{ family: "spring_hanger", hanger: { hanger_type: "spring_hanger" } }, true],
      [{ family: "spring_hanger", hanger: { hanger_type: PADDED } }, true],
      [{ family: "spring_hanger", hanger: { hanger_type: "constant_effort_support" } }, true]
    ];
    for (const [fields, counts] of cases) {
      const support = { id: "s", node: "n", restraints: [], ...fields } as unknown as Support;
      expect([fields, isSpringHangerSupport(support)]).toEqual([fields, counts]);
    }
    // The corrected cases, exactly: every support the solver reads as `spring_hanger` (padded or
    // not) that the old rule left out, and a hanger family whose explicit type the solver reads as
    // something else, which the old rule counted.
    const changed = grid().filter((support) => isSpringHangerSupport(support) !== reportRuleBeforeCorrection(support));
    const added = changed.filter((support) => isSpringHangerSupport(support)).map((support) => support.id);
    const removed = changed.filter((support) => !isSpringHangerSupport(support)).map((support) => support.id);
    const familyDecides = ["no hanger record", "null hanger record", "no hanger_type", 'hanger_type ""'];
    const spellings = ["hanger_type spring_hanger", `hanger_type ${JSON.stringify(PADDED)}`];
    const expectedAdded = familyCases.flatMap(([familyName, family]) => hangerCases.map(([hangerName]) => `${familyName} | ${hangerName}`).filter((id) => {
      const hangerName = id.split(" | ")[1];
      const oldFamily = family === "variable_spring_hanger" || family === "constant_effort_support";
      if (spellings.includes(hangerName)) return !oldFamily;
      return familyDecides.includes(hangerName) && (family === "spring_hanger" || family === PADDED);
    }));
    expect(added).toEqual(expectedAdded);
    expect(removed).toEqual(['family variable_spring_hanger | hanger_type "strut"', 'family constant_effort_support | hanger_type "strut"']);
  });

  it("gives A8's support classification and Hangers filter the same answer for every support", () => {
    const supports = grid();
    const nodes = supports.map((support, index) => ({ id: support.node, label: "", position: { x: index, y: 0, z: 0 } }));
    const model = { ...overlayModel(), nodes, supports } as unknown as PreviewModel;
    const data = buildDisplacementExtremes({ model, result: overlayResult([row("d", "global_nodal_displacement_x", nodes[0].id, 1, "mm")]), standing: healthStanding({ eligible: true }) });
    expect(data.state).toBe("available");
    const flags = new Map(data.nodes.flatMap((node) => node.supports.map((item) => [item.supportId, item.hanger] as const)));
    for (const support of supports) expect([support.id, flags.get(support.id)]).toEqual([support.id, solverCounts(support)]);
    const listed = displacementExtremesView(data, { caseKeys: null, filter: FILTER_HANGERS, sort: "model" }).rows.map((item) => item.nodeId);
    expect(listed).toEqual(supports.filter(solverCounts).map((support) => support.node));
  });

  it("is the one definition: the report's files import it and keep no inline copy", () => {
    for (const file of ["ReportPanel.tsx", "renderableReportInput.ts"]) {
      const source = readFileSync(path.join(process.cwd(), "src", "features", "report", file), "utf8");
      expect(source).toContain('import { isSpringHangerSupport } from "./springHangerSupport";');
      expect(source).not.toMatch(/hanger\?\.hanger_type === "(variable_spring_hanger|spring_hanger|constant_effort_support)"/);
      expect(source).not.toMatch(/support\.family === "(variable_spring_hanger|spring_hanger|constant_effort_support)"/);
    }
  });
});
