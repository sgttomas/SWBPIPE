// RT-R6 slice 3 (DEL-08-04 AC-002, VER-002): every results table's CSV adapter. Per table:
// - each numeric field parses back exactly to the panel's conversion of the stored value, the
//   display rule's exact step (`exactDisplayQuantity`), or the stored value for the comparisons,
//   and its rounding is the screen's text;
// - AC-001 and R9/R12: units, statuses and stable identifiers are present, no numeric field is
//   empty, the preamble's provenance is real or an explicit token, and no private or protected
//   payload appears.
// All data are invented (`resultOverlayFixtures`) or the bundled invented reference.
import { describe, expect, it } from "vitest";
import { healthStanding, overlayModel, overlayResult, reaction, row } from "../../test-support/resultOverlayFixtures";
import { loadBundledMechanicsReference } from "../../services/previewService";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import { displayQuantityParts } from "../report/reportQuantityDisplay";
import { buildDisplacementExtremes, DISPLACEMENT_COMPONENTS, displacementExtremesView, FILTER_ALL } from "../results/displacementExtremes";
import { displacementExtremesCsv } from "../results/displacementExtremesCsv";
import { buildStressRatioTable, stressRatioView, type StressRatioGroup } from "../results/stressRatioTable";
import { csvRulePackRef, stressRatioCsv, STRESS_RATIO_STATUS_TEXT } from "../results/stressRatioTableCsv";
import { buildSupportLoadSummaries, SUPPORT_LOAD_COLUMNS } from "../results/supportLoadSummary";
import { supportLoadCsv, supportLoadsAllCsv } from "../results/supportLoadSummaryCsv";
import type { WeightSummary } from "../results/weightSummary";
import { weightCogCsv, weightSummaryCsv } from "../results/weightSummaryCsv";
import { compareRunInputs, flattenRunResult } from "../comparison/runComparison";
import { matchesRunComparisonFilters, runComparisonCsv, runComparisonEntries } from "../comparison/runComparisonCsv";
import { compareModelDocuments } from "../comparison/stateComparison";
import { stateComparisonCsv } from "../comparison/stateComparisonCsv";
import { csvAnalysisRoute, csvModelOnlyProvenance, csvResultHash, CSV_SOFTWARE_VERSION, csvUnitBasis, CSV_UNIT_BASIS_DISPLAY_RULE } from "./csvProvenance";
import { CSV_NOT_APPLICABLE, CSV_NOT_RECORDED, isCsvHashValue, serializeCsvTable, type CsvProvenance } from "./csvTable";
import { exactValueCell } from "./csvValues";
import { expectExactDisplayField, expectNoEmptyNumericField, expectStoredField, numericField, preambleValue, readCsv, textField } from "../../test/csvRead";

const HASH = `sha256:${"0123456789abcdef".repeat(4)}`;
const PROVENANCE: CsvProvenance = { modelHash: HASH, resultHash: CSV_NOT_RECORDED, analysisRoutes: ["openpipestress.result_semantics/0.3.0/invented"] };
const CONTEXT = { provenance: PROVENANCE, softwareVersion: CSV_SOFTWARE_VERSION, unitBasis: CSV_UNIT_BASIS_DISPLAY_RULE };
const PASSING = healthStanding({ eligible: true });
const L2 = { ref_type: "load_case", ref_id: "load:L-2" } as const;
type Row = MechanicsResult["results"][number];

/** Values chosen so the exact step and binary64 both show: a 17-digit Pa stress shifts to a
 * decimal no binary64 reads back, and a shortest decimal that does. */
const AWKWARD_PA = 123456789.12345679;

function overlayRows(): Row[] {
  const node = (id: string, x: number, y: number, z: number, extra: Partial<Row> = {}) => [
    row(`ux:${id}`, "global_nodal_displacement_x", id, x, "mm", extra),
    row(`uy:${id}`, "global_nodal_displacement_y", id, y, "mm", extra),
    row(`uz:${id}`, "global_nodal_displacement_z", id, z, "m", extra),
    row(`rx:${id}`, "global_nodal_rotation_x", id, -0, "rad", extra)
  ];
  return [
    ...node("node:A", 0.1 + 0.2, -1e-7, 1 / 3),
    ...node("node:C", -12.5, 2.5e21, 0.000001),
    ...node("node:A", 4, 5, 6, { id: "x", basis_ref: L2 }).map((item, index) => ({ ...item, id: `L2:${index}` })),
    reaction("support:S-A", "Fx", -1234.5678, "N"),
    reaction("support:S-A", "Fy", 0.30000000000000004, "N"),
    reaction("support:S-A", "Mz", 98765.4321, "N*m"),
    reaction("support:S-C", "Fx", 7, "kN"),
    row("s:P-1:i", "element_local_normal_stress_extreme", "pipe:P-1", AWKWARD_PA, "Pa", { metadata: { component: "sigma_plus", coordinate_system: "pipe_section", location: "end_i", basis: "invented", sign_convention: "invented" } }),
    row("s:P-1:j", "element_local_normal_stress_extreme", "pipe:P-1", -4.5e7, "Pa", { metadata: { component: "sigma_plus", coordinate_system: "pipe_section", location: "end_j", basis: "invented", sign_convention: "invented" } }),
    row("s:P-2:i", "element_local_normal_stress_extreme", "pipe:P-2", 80, "MPa", { metadata: { component: "sigma_plus", coordinate_system: "pipe_section", location: "end_i", basis: "invented", sign_convention: "invented" } })
  ];
}

const PRIVATE_MARKERS = /private_project_data|INVENTED_PRIVATE|password|secret|file_path|\/Users\/|C:\\\\|rule_pack_payload|formula_text/i;

function expectBoundary(text: string, kind: string): ReturnType<typeof readCsv> {
  const csv = readCsv(text);
  expect(preambleValue(csv, "table")).toBe(kind);
  expect(preambleValue(csv, "export_context")).toBe("local_private");
  expect(preambleValue(csv, "released_report")).toBe("false");
  expect(text).not.toMatch(PRIVATE_MARKERS);
  expect(text).not.toMatch(/certif|approv|complian|seal/i);
  expectNoEmptyNumericField(csv);
  return csv;
}

describe("displacement extremes CSV (displacement-extremes)", () => {
  it("writes every node of the filter, exact in the panel's display unit, with ids, cases and statuses", () => {
    const model = overlayModel();
    const data = buildDisplacementExtremes({ model, result: overlayResult(overlayRows()), standing: PASSING });
    const view = displacementExtremesView(data, { caseKeys: null, filter: FILTER_ALL, sort: "model" });
    const built = displacementExtremesCsv({ view, ...CONTEXT });
    expect(built.fileName).toBe("swbpipe-displacement-extremes-all.csv");
    const csv = expectBoundary(serializeCsvTable(built.table), "displacement-extremes");
    expect(csv.rows.map((line) => textField(csv, line, "Node id"))).toEqual(view.rows.map((item) => item.nodeId));
    let checked = 0;
    view.rows.forEach((item, index) => {
      for (const component of DISPLACEMENT_COMPONENTS) {
        for (const which of ["max", "min"] as const) {
          const extreme = item.extremes[component.id];
          const value = extreme[which];
          const got = numericField(csv, csv.rows[index], `${component.label} ${which}`);
          if (value) {
            expectExactDisplayField(got, value.value, value.unit);
            expect(textField(csv, csv.rows[index], `${component.label} ${which} result id`)).toBe(value.resultId);
            expect(textField(csv, csv.rows[index], `${component.label} ${which} case key`)).toBe(value.caseKey);
            checked += 1;
          } else {
            expect(got.field).toBe(extreme.note ?? "none");
          }
        }
      }
    });
    expect(checked).toBeGreaterThanOrEqual(12);
    // Mixed units (mm and m in uz? no: uz is all m) keep a header unit; negative zero stays -0.
    expect(csv.header).toContain("ux max [mm]");
    expect(csv.rows[0][csv.header.indexOf("rx max [rad]")]).toBe("-0");
  });
});

describe("support load summary CSV (support-loads, support-loads-all)", () => {
  const summaries = () => buildSupportLoadSummaries({ model: overlayModel(), result: overlayResult(overlayRows()), standing: PASSING });

  it("writes the shown support's case rows and its maximum and minimum, exact, with per-row units where kinds mix", () => {
    const all = summaries();
    const summary = all.supports.find((item) => item.supportId === "support:S-A")!;
    const built = supportLoadCsv({ summary, ...CONTEXT });
    expect(built.fileName).toBe("swbpipe-support-loads-support-s-a.csv");
    const csv = expectBoundary(serializeCsvTable(built.table), "support-loads");
    expect(csv.rows.map((line) => textField(csv, line, "Row"))).toEqual([...summary.rows.map(() => "case"), "maximum", "minimum"]);
    let checked = 0;
    summary.rows.forEach((caseRow, index) => {
      expect(textField(csv, csv.rows[index], "Case key")).toBe(caseRow.case.key);
      expect(textField(csv, csv.rows[index], "Support id")).toBe("support:S-A");
      for (const column of SUPPORT_LOAD_COLUMNS) {
        const value = caseRow.values[column.id];
        const got = numericField(csv, csv.rows[index], column.label);
        if (value && caseRow.standing === "admitted" && !(column.of === "support" && caseRow.reactionAbsence)) {
          expectExactDisplayField(got, value.value, value.unit);
          expect(textField(csv, csv.rows[index], `${column.label} result id`)).toBe(value.resultId);
          checked += 1;
        } else {
          expect(got.field).not.toBe("");
          expect(Number.isNaN(Number(got.field)) || got.field === "").toBe(true);
        }
      }
    });
    expect(checked).toBeGreaterThanOrEqual(3);
  });

  it("writes every support in one file, one row per support and case", () => {
    const all = summaries();
    const built = supportLoadsAllCsv({ summaries: all, ...CONTEXT });
    expect(built.fileName).toBe("swbpipe-support-loads-all.csv");
    const csv = expectBoundary(serializeCsvTable(built.table), "support-loads-all");
    const expected = all.supports.flatMap((summary) => summary.rows.map((item) => [summary.supportId, item.case.key]));
    expect(csv.rows.map((line) => [textField(csv, line, "Support id"), textField(csv, line, "Case key")])).toEqual(expected);
    // Forces in N and kN mix: the column keeps a per-row unit.
    const fx = csv.rows.map((line) => numericField(csv, line, "Fx"));
    expect(csv.header).toContain("Fx unit");
    expect(new Set(fx.filter((item) => !Number.isNaN(Number(item.field))).map((item) => item.unit))).toEqual(new Set(["N", "kN"]));
  });
});

// --- An invented rule pack and run (as `stressRatioTable.test.ts`) ---------------------------------
const L1 = { ref_type: "load_case", ref_id: "load:L-1" } as const;
function ruleRun(): SessionRuleCheckRun {
  const solver = (id: string, entity: string, location: string) => ({
    input_id: id, name: id, source_kind: "solver_result", quantity_intent: { dimension: "stress", unit_ref: "MPa" },
    solver_result_ref: { result_kind: "element_local_normal_stress_extreme", entity_ref: entity, location, component: "sigma_plus" }
  });
  const pack = {
    metadata: { rule_pack_id: "invented_pack", rule_pack_version: "1.2.3", name: "Invented pack" },
    checksums: { rule_pack_checksum: { algorithm: "sha256", canonicalization: "rfc8785_jcs", value: "ab".repeat(32) } },
    required_inputs: [solver("in_p1", "pipe:P-1", "end_i"),
      { input_id: "limit_value", name: "limit", source_kind: "user_supplied_rule_value", quantity_intent: { dimension: "stress", unit_ref: "MPa" } }],
    stress_categories: [{ category_id: "cat_a", name: "Invented A" }],
    required_load_cases: [{ required_load_case_id: "rl1", name: "one", basis_ref: L1 }],
    load_case_mappings: [{ category_ref: { ref_type: "stress_category", ref_id: "cat_a" }, required_load_case_refs: [{ ref_type: "required_load_case", ref_id: "rl1" }] }],
    check_definitions: [{ check_id: "chk_p1", category_ref: { ref_type: "stress_category", ref_id: "cat_a" }, required_input_refs: [{ ref_type: "required_input", ref_id: "in_p1" }, { ref_type: "required_input", ref_id: "limit_value" }] }]
  };
  return {
    rulePackDocument: pack,
    result: {
      document_kind: "rule_check_run_result", rule_pack_id: "invented_pack", grammar_version: "1.0.0", aggregate_status: "USER_RULE_FAILED",
      checks: [{
        check_id: "chk_p1", category_ref: "cat_a", basis_ref: L1, status: "USER_RULE_FAILED", acceptability_relation: "less_than_or_equal",
        bound_inputs: [{ input_id: "in_p1", source_kind: "solver_result", supplied: true, value: 123.45678912345679, unit: "MPa", result_id: "s:P-1:i" }],
        completeness_findings: [], evaluator_findings: [], diagnostic_codes: [],
        computed_value: { value: 123.45678912345679, unit_ref: "MPa", dimension: "stress" },
        limit_value: { value: 100, unit_ref: "MPa", dimension: "stress" }
      }],
      professional_boundary_notice: "invented"
    }
  } as unknown as SessionRuleCheckRun;
}

describe("stress and ratio table CSV (stress-ratio)", () => {
  it("writes an emitted stress group, every page, exact (Pa shown as MPa) with row ids", () => {
    const built = buildStressRatioTable({ model: overlayModel(), result: overlayResult(overlayRows()), standing: PASSING, ruleCheckRun: null });
    const group = built.groups.find((item) => item.id === "stress:element_local_normal_stress_extreme")!;
    const view = stressRatioView(group, { mode: "stress", threshold: "" });
    const out = stressRatioCsv({ group, view, ...CONTEXT, rulePacks: [], threshold: "" });
    expect(out.fileName).toBe("swbpipe-stress-ratio-stress-element-local-normal-stress-extreme.csv");
    const text = serializeCsvTable(out.table);
    const csv = expectBoundary(text, "stress-ratio");
    expect(text).not.toContain("rule_pack");
    const byKey = new Map(group.rows.map((item) => [item.key, item]));
    let exactDecimal = 0;
    for (const line of csv.rows) {
      const source = byKey.get(textField(csv, line, "Row key"))!;
      const got = numericField(csv, line, "Stress");
      expectExactDisplayField(got, source.stress!.value, source.stress!.unit);
      expect(textField(csv, line, "Stress result id")).toBe(source.stress!.resultId);
      if (source.stress!.value === AWKWARD_PA) {
        // 123.45678912345679 Pa shifted is exactly 123.45678912345679e-6: written as that decimal.
        expect(got.field).toBe("123.45678912345679");
        exactDecimal += 1;
      }
    }
    expect(exactDecimal).toBe(1);
    expect(csv.rows.length).toBe(group.rows.length);
  });

  it("writes a rule category with the table's own outcome wording and the rule pack by id, version and checksum", async () => {
    const run = ruleRun();
    const built = buildStressRatioTable({ model: overlayModel(), result: overlayResult(overlayRows()), standing: PASSING, ruleCheckRun: run });
    const group = built.groups.find((item: StressRatioGroup) => item.source === "rule_category");
    expect(group, JSON.stringify(built.run)).toBeDefined();
    const view = stressRatioView(group!, { mode: "ratio", threshold: "" });
    const { rulePackReference } = await import("../report/renderableReportInput");
    const pack = csvRulePackRef(await rulePackReference(run));
    expect(pack).toEqual({ id: "invented_pack", version: "1.2.3", checksum: `sha256:${"ab".repeat(32)}` });
    const text = serializeCsvTable(stressRatioCsv({ group: group!, view, ...CONTEXT, rulePacks: [pack], threshold: "" }).table);
    const csv = expectBoundary(text, "stress-ratio");
    expect(csv.preamble.filter(([key]) => key === "rule_pack")).toEqual([["rule_pack", `invented_pack 1.2.3 sha256:${"ab".repeat(32)}`]]);
    expect(text).not.toMatch(/required_inputs|check_definitions|limit_value|quantity_intent/);
    for (const line of csv.rows) {
      const outcome = group!.rows.find((item) => item.key === textField(csv, line, "Row key"))!.outcome!;
      expect(Object.values(STRESS_RATIO_STATUS_TEXT)).toContain(textField(csv, line, "Status"));
      expect(textField(csv, line, "Check")).toBe(outcome.checkId);
      if (outcome.computed) expectExactDisplayField(numericField(csv, line, "Computed"), outcome.computed.value, outcome.computed.unit);
      if (outcome.limit) expectExactDisplayField(numericField(csv, line, "Limit"), outcome.limit.value, outcome.limit.unit);
      if (outcome.ratio !== null) expectExactDisplayField(numericField(csv, line, "Ratio"), outcome.ratio, "");
    }
  });
});

describe("weight summary and centre of gravity CSV (weight, weight-cog)", () => {
  const summary = {
    state: "available", scope: { id: "model", kind: "model", label: "Whole model", pipeRefs: ["pipe:P-1"], unresolved: [] },
    gravity: { value: 9.80665, unit: "m/s^2", axis: "global_z" }, lengthUnit: "mm", entities: [],
    pipes: { count: 2, totals: [{ unit: "N", weight: 1234.5678901234567 }, { unit: "kN", weight: 0.5 }] },
    components: { count: 0, totals: [] }, total: null, centreOfGravity: { x: 0.1 + 0.2, y: -0, z: 1e-7 }, note: "Units differ; no total is formed.",
    notCounted: [], componentsNotIncluded: [], countedPipeCount: 1, partial: false, contentsCount: 0, insulationCount: 0, engineCalls: 1
  } as unknown as Extract<WeightSummary, { state: "available" }>;
  const model = csvModelOnlyProvenance(HASH);

  it("is model data: result_hash and analysis_route are not_applicable", () => {
    const text = serializeCsvTable(weightSummaryCsv({ summary, totalLabel: "Total", provenance: model, softwareVersion: CSV_SOFTWARE_VERSION, unitBasis: "u" }).table);
    const csv = expectBoundary(text, "weight");
    expect(preambleValue(csv, "result_hash")).toBe(CSV_NOT_APPLICABLE);
    expect(preambleValue(csv, "analysis_route")).toBe(CSV_NOT_APPLICABLE);
    expect(preambleValue(csv, "model_hash")).toBe(HASH);
    expect(csv.rows.map((line) => textField(csv, line, "Category id"))).toEqual(["pipes", "pipes", "components", "total"]);
    expectExactDisplayField(numericField(csv, csv.rows[0], "Weight"), 1234.5678901234567, "N");
    expectExactDisplayField(numericField(csv, csv.rows[1], "Weight"), 0.5, "kN");
    expect(numericField(csv, csv.rows[2], "Weight").field).toBe("none");
    expect(numericField(csv, csv.rows[3], "Weight").field).toBe(summary.note);
    expectExactDisplayField(numericField(csv, csv.rows[0], "Counted"), 2, "");
  });

  it("writes the centre of gravity exact in the model's length unit", () => {
    const built = weightCogCsv({ summary, cogLabel: "Centre of gravity", provenance: model, softwareVersion: CSV_SOFTWARE_VERSION, unitBasis: "u" });
    expect(built.fileName).toBe("swbpipe-weight-cog-model.csv");
    const csv = expectBoundary(serializeCsvTable(built.table), "weight-cog");
    for (const axis of ["x", "y", "z"] as const) {
      expectExactDisplayField(numericField(csv, csv.rows[0], axis.toUpperCase()), summary.centreOfGravity![axis], "mm");
    }
  });
});

describe("comparison CSV (run-comparison, state-comparison)", () => {
  it("writes the run comparison's matching rows as stored, with a and b following Left and Right", async () => {
    const { source } = await loadBundledMechanicsReference();
    const rows = source.results;
    // Compared through the view model with the comparison's own inputs.
    const left = { rows: flattenRunResult(source), standing: { eligible: true, status: "integrity_checked", contract: "generic", findings: [] }, semantic_identity: { route: "generic", contract_id: null, contract_sha256: null }, context: {}, diagnostics: [] };
    const right = { ...left, rows: flattenRunResult({ ...source, results: rows.map((item, index) => (index % 3 === 0 ? { ...item, value: item.value * 1.1 + 0.1 } : item)).slice(1) }) };
    const comparison = compareRunInputs(left as never, right as never);
    const filters = { status: "all" as const, family: "all", idFilter: "" };
    const entries = runComparisonEntries(comparison).filter((entry) => matchesRunComparisonFilters(entry, filters));
    const built = runComparisonCsv({
      leftId: "runrec:0001", rightId: "current-run", leftDescription: "left run", rightDescription: "the current run",
      sides: { a: { modelHash: HASH, resultHash: HASH, analysisRoutes: ["contract-a"] }, b: { modelHash: HASH, resultHash: CSV_NOT_RECORDED, analysisRoutes: [CSV_NOT_RECORDED] } },
      entries, filters, softwareVersion: CSV_SOFTWARE_VERSION, unitBasis: "stored"
    });
    expect(built.fileName).toBe("swbpipe-run-comparison-runrec-0001-current-run.csv");
    const csv = expectBoundary(serializeCsvTable(built.table), "run-comparison");
    expect(csv.preamble.slice(5, 11)).toEqual([
      ["model_hash_a", HASH], ["model_hash_b", HASH], ["result_hash_a", HASH], ["result_hash_b", "not_recorded"],
      ["analysis_route_a", "contract-a"], ["analysis_route_b", "not_recorded"]
    ]);
    expect(csv.rows.length).toBe(entries.length);
    let compared = 0;
    entries.forEach((entry, index) => {
      expect(textField(csv, csv.rows[index], "Result id")).toBe(entry.identity.result_id);
      if (entry.source.kind === "pair" && entry.source.pair.status === "compared") {
        const pair = entry.source.pair;
        expectStoredField(numericField(csv, csv.rows[index], "Left"), pair.left, pair.unit);
        expectStoredField(numericField(csv, csv.rows[index], "Right"), pair.right, pair.unit);
        expectStoredField(numericField(csv, csv.rows[index], "Delta (right - left)"), pair.delta, pair.unit);
        compared += 1;
      }
    });
    expect(compared).toBeGreaterThan(3);
  });

  it("writes the state comparison's field changes as stored, with both states' model hashes and no result", () => {
    const left = { ...overlayModel(), nodes: overlayModel().nodes.map((node) => ({ ...node, position: { ...node.position, x: node.position.x * 1000 } })) } as PreviewModel;
    const right = { ...overlayModel(), nodes: [...overlayModel().nodes.slice(1), { id: "node:E", position: { x: -0.5, y: 0, z: 0 } }] } as unknown as PreviewModel;
    const comparison = compareModelDocuments(left, right);
    const state = (id: string, hash: string) => ({ state_id: id, state_name: id, model_hash: hash } as never);
    const built = stateComparisonCsv({
      left: state("state:1", HASH), right: state("state:2", `sha256:${"f".repeat(64)}`), rows: comparison.rows,
      filters: { classification: "all", family: "all", idFilter: "" }, softwareVersion: CSV_SOFTWARE_VERSION, unitBasis: "stored"
    });
    const csv = expectBoundary(serializeCsvTable(built.table), "state-comparison");
    expect(csv.preamble.slice(5, 11)).toEqual([
      ["model_hash_a", HASH], ["model_hash_b", `sha256:${"f".repeat(64)}`], ["result_hash_a", "not_applicable"], ["result_hash_b", "not_applicable"],
      ["analysis_route_a", "not_applicable"], ["analysis_route_b", "not_applicable"]
    ]);
    type Line = { item: (typeof comparison.rows)[number]; change: (typeof comparison.rows)[number]["changes"][number] | null };
    const changes = comparison.rows.flatMap((item): Line[] => (item.changes.length ? item.changes.map((change) => ({ item, change })) : [{ item, change: null }]));
    expect(csv.rows.length).toBe(changes.length);
    let numbers = 0;
    changes.forEach(({ item, change }, index) => {
      expect(textField(csv, csv.rows[index], "Id")).toBe(item.id);
      for (const name of ["left", "right"] as const) {
        const value = change?.[name];
        if (typeof value === "number") { expectStoredField(numericField(csv, csv.rows[index], name === "left" ? "Left value" : "Right value"), value, ""); numbers += 1; }
      }
    });
    expect(numbers).toBeGreaterThan(0);
  });
});

describe("preamble provenance (AC-001, R11)", () => {
  it("states the result hash and route only from the result's own record, else a token that is never a hash", () => {
    const result = overlayResult([]);
    expect(csvResultHash(null, null)).toBe(CSV_NOT_APPLICABLE);
    expect(csvResultHash(result, null)).toBe(CSV_NOT_RECORDED);
    const run = (runId: string, version = "0.3.0") => ({
      schema_version: version,
      analysis_run: { run_id: runId, hashes: [{ algorithm: "sha256", canonicalization: "c", payload_ref: { object_type: "ResultEnvelope", ref: "r" }, payload_scope: "received_result", value: "cd".repeat(32) }] }
    }) as unknown as AnalysisRunEnvelope;
    expect(csvResultHash(result, run(result.run_id))).toBe(`sha256:${"cd".repeat(32)}`);
    expect(csvResultHash(result, run("run:other"))).toBe(CSV_NOT_RECORDED);
    expect(csvResultHash(result, run(result.run_id, "0.2.0"))).toBe(CSV_NOT_RECORDED);
    expect(csvAnalysisRoute(null)).toBe(CSV_NOT_APPLICABLE);
    expect(csvAnalysisRoute(result)).toBe(CSV_NOT_RECORDED);
    expect(csvAnalysisRoute({ ...result, producer: { component_name: "p", component_version: "1", semantic_contract_id: "openpipestress.result_semantics/0.3.0/pressure-2" } }))
      .toBe("openpipestress.result_semantics/0.3.0/pressure-2");
    for (const token of [CSV_NOT_APPLICABLE, CSV_NOT_RECORDED]) expect(isCsvHashValue(token)).toBe(false);
    expect(csvUnitBasis({ project: { units: { length: "mm", force: "N" } } } as unknown as PreviewModel, "basis")).toBe("basis; project units force=N, length=mm");
  });

  it("states the bundled reference's tables completely, with a real model hash shape", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    const cases = [...new Set(source.results.map((item) => item.basis_ref?.ref_id).filter((id): id is string => typeof id === "string" && id.startsWith("load:")))];
    const standing = healthStanding({ eligible: true, states: Object.fromEntries(cases.map((id) => [id, "checks_passed"])) });
    const data = buildDisplacementExtremes({ model, result: source, standing });
    const view = displacementExtremesView(data, { caseKeys: null, filter: FILTER_ALL, sort: "model" });
    const text = serializeCsvTable(displacementExtremesCsv({ view, provenance: { modelHash: HASH, resultHash: csvResultHash(source, null), analysisRoutes: [csvAnalysisRoute(source)] }, softwareVersion: CSV_SOFTWARE_VERSION, unitBasis: csvUnitBasis(model, CSV_UNIT_BASIS_DISPLAY_RULE) }).table);
    const csv = expectBoundary(text, "displacement-extremes");
    expect(csv.rows.length).toBe(view.rows.length);
    expect(preambleValue(csv, "software_version")).toBe(CSV_SOFTWARE_VERSION);
    view.rows.forEach((item, index) => {
      const ux = item.extremes.ux.max;
      if (ux) expectExactDisplayField(numericField(csv, csv.rows[index], `${DISPLACEMENT_COMPONENTS[0].label} max`), ux.value, ux.unit);
    });
    const summaries = buildSupportLoadSummaries({ model, result: source, standing });
    const all = readCsv(serializeCsvTable(supportLoadsAllCsv({ summaries, ...CONTEXT }).table));
    expect(all.rows.length).toBe(summaries.supports.reduce((count, item) => count + item.rows.length, 0));
  });

  it("agrees with the screen: rounding the exact cell gives the panel's text", () => {
    for (const [value, unit] of [[AWKWARD_PA, "Pa"], [-0, "Pa"], [1e-7, "mm"], [0.1 + 0.2, "N"]] as const) {
      const cell = exactValueCell(value, unit);
      const field = cell.kind === "number" ? (Object.is(cell.value, -0) ? "-0" : String(cell.value)) : cell.kind === "decimal" ? cell.text : "";
      expectExactDisplayField({ field, unit: displayQuantityParts(0, unit).unit }, value, unit);
    }
  });
});

