// The report's spring hanger rule, shared with A8's "Hangers" filter (DEL-07-05 Design
// `displacement-extremes.md` §2.1). Every `SupportFamily` value, an absent, null, empty, unknown and
// padded family, crossed with every `SpringHanger.hanger_type` value, no hanger record, a record
// without a type, and an empty, unknown, padded and padding-only type: the shared function equals
// the solver's classification, taken as an explicit table from the Rust, and A8 classifies and
// filters each support the same way. Padding uses ASCII spaces and the characters on which Rust's
// `str::trim` and JavaScript's `trim` disagree (U+0085, U+FEFF) or which neither removes (U+001C);
// the trim itself is checked against Rust's whitespace set over every BMP code point.
import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { healthStanding, overlayModel, overlayResult, row } from "../../test-support/resultOverlayFixtures";
import type { PreviewModel } from "../../types";
import { FILTER_HANGERS, buildDisplacementExtremes, displacementExtremesView } from "../results/displacementExtremes";
import { formatSpringHangerSummary } from "./ReportPanel";
import { RUST_WHITESPACE, isSpringHangerSupport, springHangerKind, trimLikeRust } from "./springHangerSupport";

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
// Rust's `str::trim` removes U+0085 (NEL), which JavaScript's `trim` keeps, and keeps U+FEFF (BOM),
// which JavaScript's `trim` removes; neither removes U+001C (FILE SEPARATOR).
const NEL = "\u0085", BOM = "﻿", FS = "\u001c";
const NEL_PADDED = `${NEL}spring_hanger${NEL}`, BOM_PADDED = `${BOM}spring_hanger${BOM}`, FS_PADDED = `${FS}spring_hanger${FS}`;
/** A case label: the value quoted, with every character outside printable ASCII shown as `\uXXXX`. */
const show = (value: string) => `"${value.replace(/[^\x20-\x7e]/g, (c) => `\\u${c.charCodeAt(0).toString(16).padStart(4, "0")}`)}"`;

/** The code points for which Rust's `char::is_whitespace` holds: the Unicode `White_Space` property,
 * which `str::trim` removes from both ends (Rust std `char::is_whitespace`: "Returns true if this
 * char has the White_Space property"; Unicode `PropList.txt`). Printed by a probe over every `char`
 * U+0000-U+10FFFF under rustc 1.97.1 (Unicode 17.0.0), which also asserted that `str::trim` removes
 * exactly these. None lies outside the BMP. */
const RUST_WHITE_SPACE_RANGES: Array<[number, number]> = [
  [0x0009, 0x000d], [0x0020, 0x0020], [0x0085, 0x0085], [0x00a0, 0x00a0], [0x1680, 0x1680],
  [0x2000, 0x200a], [0x2028, 0x2029], [0x202f, 0x202f], [0x205f, 0x205f], [0x3000, 0x3000]
];
const RUST_WHITE_SPACE: ReadonlySet<number> = new Set(
  RUST_WHITE_SPACE_RANGES.flatMap(([from, to]) => Array.from({ length: to - from + 1 }, (_, offset) => from + offset))
);
const ASTRAL_SAMPLES = [0x10000, 0x1680 + 0x10000, 0x1f600, 0xe0020, 0x10ffff];

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
  'hanger_type " spring_hanger "': true,
  // Rust trims U+0085 and keeps U+FEFF and U+001C: a padding-only type is blank only for U+0085.
  'hanger_type "\\u0085spring_hanger\\u0085"': true,
  'hanger_type "\\ufeffspring_hanger\\ufeff"': false,
  'hanger_type "\\u001cspring_hanger\\u001c"': false,
  'hanger_type "\\u0085"': "family",
  'hanger_type "\\ufeff"': false,
  'hanger_type "\\u001c"': false
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
  "family snubber": false,
  "no family": false,
  "null family": false,
  'family ""': false,
  'family "strut"': false,
  'family " spring_hanger "': true,
  'family "\\u0085spring_hanger\\u0085"': true,
  'family "\\ufeffspring_hanger\\ufeff"': false,
  'family "\\u001cspring_hanger\\u001c"': false
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
  ["no family", ABSENT], ["null family", null],
  ...["", "strut", PADDED, NEL_PADDED, BOM_PADDED, FS_PADDED].map((family): [string, string] => [`family ${show(family)}`, family])
];
const hangerCases: Array<[string, Record<string, unknown> | null | typeof ABSENT]> = [
  ["no hanger record", ABSENT], ["null hanger record", null], ["no hanger_type", {}],
  ...HANGER_TYPES.map((hanger_type): [string, Record<string, unknown>] => [`hanger_type ${hanger_type}`, { hanger_type }]),
  ...["", "strut", PADDED, NEL_PADDED, BOM_PADDED, FS_PADDED, NEL, BOM, FS].map((hanger_type): [string, Record<string, unknown>] => [`hanger_type ${show(hanger_type)}`, { hanger_type }])
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
    // something else (unknown, or padded with characters Rust does not trim), which the old rule
    // counted.
    const changed = grid().filter((support) => isSpringHangerSupport(support) !== reportRuleBeforeCorrection(support));
    const added = changed.filter((support) => isSpringHangerSupport(support)).map((support) => support.id);
    const removed = changed.filter((support) => !isSpringHangerSupport(support)).map((support) => support.id);
    const familyDecides = ["no hanger record", "null hanger record", "no hanger_type", 'hanger_type ""', `hanger_type ${show(NEL)}`];
    const spellings = ["hanger_type spring_hanger", `hanger_type ${show(PADDED)}`, `hanger_type ${show(NEL_PADDED)}`];
    const expectedAdded = familyCases.flatMap(([familyName, family]) => hangerCases.map(([hangerName]) => `${familyName} | ${hangerName}`).filter((id) => {
      const hangerName = id.split(" | ")[1];
      const oldFamily = family === "variable_spring_hanger" || family === "constant_effort_support";
      if (spellings.includes(hangerName)) return !oldFamily;
      return familyDecides.includes(hangerName) && (family === "spring_hanger" || family === PADDED || family === NEL_PADDED);
    }));
    expect(added).toEqual(expectedAdded);
    const notHangers = ['hanger_type "strut"', ...[BOM_PADDED, FS_PADDED, BOM, FS].map((type) => `hanger_type ${show(type)}`)];
    expect(removed).toEqual(["variable_spring_hanger", "constant_effort_support"].flatMap((family) => notHangers.map((hangerName) => `family ${family} | ${hangerName}`)));
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
    const imports: Record<string, string> = {
      "ReportPanel.tsx": 'import { isSpringHangerSupport, springHangerKind } from "./springHangerSupport";',
      "renderableReportInput.ts": 'import { isSpringHangerSupport } from "./springHangerSupport";'
    };
    for (const [file, importLine] of Object.entries(imports)) {
      const source = readFileSync(path.join(process.cwd(), "src", "features", "report", file), "utf8");
      expect(source).toContain(importLine);
      expect(source).not.toMatch(/hanger\?\.hanger_type === "(variable_spring_hanger|spring_hanger|constant_effort_support)"/);
      expect(source).not.toMatch(/support\.family === "(variable_spring_hanger|spring_hanger|constant_effort_support)"/);
      // The summary's split reads the solver's subtype, not the raw spelling.
      expect(source).not.toMatch(/item\.hanger_type === "(variable_spring_hanger|spring_hanger|constant_effort_support)"/);
    }
  });

  it("trims exactly as Rust's str::trim: the White_Space set over every BMP code point and astral samples", () => {
    expect([...RUST_WHITESPACE].sort((a, b) => a - b)).toEqual([...RUST_WHITE_SPACE].sort((a, b) => a - b));
    const codePoints = [...Array.from({ length: 0x10000 }, (_, cp) => cp), ...ASTRAL_SAMPLES];
    const mismatches: string[] = [];
    for (const cp of codePoints) {
      const c = String.fromCodePoint(cp);
      const trims = RUST_WHITE_SPACE.has(cp);
      if ((trimLikeRust(`${c}${c}x${c}`) === "x") !== trims) mismatches.push(`trim U+${cp.toString(16)}`);
      // Through the predicate: a family padded with the character counts exactly when Rust trims it.
      if (isSpringHangerSupport({ family: `${c}spring_hanger${c}` }) !== trims) mismatches.push(`family U+${cp.toString(16)}`);
    }
    expect(mismatches).toEqual([]);
    // Interior whitespace is kept, as in Rust.
    expect(trimLikeRust(`${NEL} a${NEL}b ${NEL}`)).toBe(`a${NEL}b`);
  });

  // Codex r1 (P2): JavaScript's `trim` removes U+FEFF and keeps U+0085, the reverse of the solver.
  const paddings: Array<[string, string, boolean]> = [
    ["U+0085", NEL, true],
    ["U+FEFF", BOM, false],
    ["U+001C", FS, false]
  ];
  it("reads an explicit hanger type padded as the solver does (U+0085 counts; U+FEFF and U+001C do not)", () => {
    for (const [name, pad, counts] of paddings) {
      const support = { family: "spring", hanger: { hanger_type: `${pad}spring_hanger${pad}` } };
      expect([name, isSpringHangerSupport(support), springHangerKind(support)]).toEqual([name, counts, counts ? "variable" : null]);
      // A hanger family does not rescue an explicit type the solver reads as unknown.
      const overFamily = { family: "constant_effort_support", hanger: { hanger_type: `${pad}spring_hanger${pad}` } };
      expect([name, springHangerKind(overFamily)]).toEqual([name, counts ? "variable" : null]);
    }
  });

  it("reads a padded family as the solver does when there is no explicit hanger type", () => {
    for (const [name, pad, counts] of paddings) {
      for (const hanger of [undefined, {}]) {
        const support = { family: `${pad}spring_hanger${pad}`, hanger };
        expect([name, hanger, isSpringHangerSupport(support), springHangerKind(support)]).toEqual([name, hanger, counts, counts ? "variable" : null]);
      }
    }
  });

  it("falls back to the family only when the explicit type is blank to Rust", () => {
    for (const [name, pad, counts] of paddings) {
      // A padded family behind an empty explicit type.
      const padded = { family: `${pad}constant_effort_support${pad}`, hanger: { hanger_type: "" } };
      expect([name, springHangerKind(padded)]).toEqual([name, counts ? "constant_effort" : null]);
      // An explicit type of padding only: blank to Rust (so the family decides) only for U+0085.
      const paddingOnly = { family: "constant_effort_support", hanger: { hanger_type: `${pad}${pad}` } };
      expect([name, springHangerKind(paddingOnly)]).toEqual([name, counts ? "constant_effort" : null]);
    }
  });

  // Codex r1 (P3): the summary's variable/constant-effort split follows the same solver reading, so
  // a padded spelling and a family-only hanger fall in their group.
  it("splits the summary by the solver's subtype, padded and family-only hangers included", () => {
    const record = (hanger_type: string, support_family: string, rows: number) =>
      ({ hanger_type, support_family, result_refs: Array.from({ length: rows }, (_, i) => `r${i}`) }) as unknown as Parameters<typeof formatSpringHangerSummary>[0][number];
    const cases: Array<[Parameters<typeof formatSpringHangerSummary>[0], string]> = [
      [[record(" spring_hanger ", "spring", 5), record("constant_effort_support", "constant_effort_support", 2)], "2 hanger records; variable=1; constant_effort=1; rows=7"],
      [[record(NEL_PADDED, "spring", 5)], "1 hanger record; variable=1; constant_effort=0; rows=5"],
      [[record("not provided", "spring_hanger", 5), record("not provided", "variable_spring_hanger", 1)], "2 hanger records; variable=2; constant_effort=0; rows=6"],
      [[record("not provided", " constant_effort_support ", 2), record("", `${NEL}constant_effort_support`, 1)], "2 hanger records; variable=0; constant_effort=2; rows=3"],
      [[record(`${NEL}`, "spring_hanger", 4)], "1 hanger record; variable=1; constant_effort=0; rows=4"]
    ];
    for (const [records, expected] of cases) expect(formatSpringHangerSummary(records)).toBe(expected);
  });
});
