// DEL-04-11 REQ-008 / AC-008: applying every selection of one sizing run in one batch. The model and library are the
// invented four-span sizing input (read, not changed). Two runs are applied:
// - the product's own four-span result (`fixtures/results/hanger_sizing_result_four_span.json`): two constant
//   selections (support:B by fallback, support:C) and support:D with no fitting record;
// - a run formed here, bound by the desktop's checked hash to that model and library as a sizing run binds them, with
//   one variable selection (support:B), one constant selection (support:C) and support:D with no fitting record. Its
//   sized loads are invented; only the members the act reads are formed.
import { describe, expect, it } from "vitest";
import fourSpanInput from "../../../../../fixtures/results/hanger_sizing_input_four_span.json";
import fourSpanResult from "../../../../../fixtures/results/hanger_sizing_result_four_span.json";
import { canonicalJsonString, canonicalSha256HexCheckedV1 } from "../../services/hashService";
import type { PreviewModel } from "../../types";
import type { HangerSizingCandidate, HangerSizingSizedDocument } from "../results/hangerSizingResult";
import { describeRunApplication, HANGER_SIZING_RUN_NOTHING_TEXT, prepareRunApplication } from "./runApplication";
import { HANGER_SIZING_MODEL_CHANGED_TEXT } from "./sizedSelection";

const model = () => structuredClone(fourSpanInput.request.model) as unknown as PreviewModel;
const library = () => structuredClone(fourSpanInput.library) as Record<string, unknown>;
const productResult = () => structuredClone(fourSpanResult) as unknown as HangerSizingSizedDocument;
const LIBRARY = "invented.hangers.sizing";
const SIZING_SHA = "5".repeat(64);
/** support:B and support:C as drawn in the four-span model. */
const BEFORE = { family: "vertical_support", restraints: ["UZ"], provenance: "invented_test_value" };

async function recordSha(lib: Record<string, unknown>, hangerId: string) {
  return canonicalSha256HexCheckedV1((lib.hanger_records as { hanger_id: string }[]).find((entry) => entry.hanger_id === hangerId));
}

const common = (support: string, node: string) => ({
  support, node, dof: "UZ" as const, family_policy: "variable_else_constant" as const, w_n: 5000, u_cold_m: 0, operating: [], span_m: [0, 0.001] as const,
  required_travel_m: 0.005, failure_counts: { variable: null, constant: null }
});
const variable = (support: string, node: string, recordSha256: string, hangerId = "VS-200-L"): HangerSizingCandidate => ({
  ...common(support, node), outcome: "selected",
  selected: { family: "variable_spring", hanger_id: hangerId, record_sha256: recordSha256, k_n_per_m: 200000, f_cold_n: 5197.5, f_cold_condition_n: 5197.5,
    envelope_n: [4600, 5716], operating: [{ case: "load:O1", f_n: 5000, variation: 0.04 }] },
  governing: { travel_case: "load:O1", variation_case: "load:O1" },
  verification_assumption: { role: "spring", k_n_per_m: 200000, f_cold_n: 5197.5 },
  verification: null as never
});
const constant = (support: string, node: string, recordSha256: string): HangerSizingCandidate => ({
  ...common(support, node), outcome: "selected",
  selected: { family: "constant_effort", hanger_id: "CE-1", record_sha256: recordSha256, load_n: 3714.285714285714 },
  governing: { travel_case: "load:O1", variation_case: null },
  verification_assumption: { role: "force", force_n: 3714.285714285714 },
  verification: null as never
});
const noFit = (support: string, node: string): HangerSizingCandidate => ({
  ...common(support, node), outcome: "no_record_fits", selected: null, governing: null,
  verification_assumption: { role: "force", force_n: 4571.428571428572 }, verification: null
});

async function formed(source: PreviewModel, lib: Record<string, unknown>, candidates: HangerSizingCandidate[]): Promise<HangerSizingSizedDocument> {
  return {
    status: "sized",
    inputs: {
      canonical_profile: "openpipestress_jcs_ijson_v1",
      sizing_request: { design_operating_case: "load:O1" },
      sizing_request_sha256: SIZING_SHA,
      solve_request: { sha256: await canonicalSha256HexCheckedV1({ model: source, materials: [] }) },
      library: { library_id: LIBRARY, sha256: await canonicalSha256HexCheckedV1(lib) }
    },
    result: { candidates }
  } as unknown as HangerSizingSizedDocument;
}

async function mixedRun(source: PreviewModel, lib: Record<string, unknown>) {
  return formed(source, lib, [
    variable("support:B", "node:B", await recordSha(lib, "VS-200-L")),
    constant("support:C", "node:C", await recordSha(lib, "CE-1")),
    noFit("support:D", "node:D")
  ]);
}

const runNote = (document: HangerSizingSizedDocument, hangerId: string, sha: string) =>
  `Hanger sizing run with sizing request sha256 ${document.inputs.sizing_request_sha256} and solve request sha256 ${document.inputs.solve_request.sha256}; ` +
  `library ${LIBRARY} sha256 ${document.inputs.library.sha256}; record ${hangerId} sha256 ${sha}.`;
const FROM_RECORD = (hangerId: string) => `Family, stiffness, travel range and references from the library record library:hanger:${LIBRARY}#${hangerId}.`;

describe("applying every selection of a sizing run in one batch", () => {
  it("joins the builder's operations for a variable and a constant selection, leaves the no-fit support out and names it, and writes nothing", async () => {
    const source = model();
    const original = await canonicalJsonString(source);
    const lib = library();
    const document = await mixedRun(source, lib);
    const prepared = await prepareRunApplication(document, source, lib);
    if (prepared.kind !== "prepared") throw new Error(prepared.reason);
    const { batch } = prepared;

    // Each selected candidate contributes its configuration change and its provenance record, in the run's order.
    expect(batch.operations.map((operation) => [operation.target.ref, operation.change.field_path])).toEqual([
      ["support:B", "configuration"], ["support:B", "provenance_record"], ["support:C", "configuration"], ["support:C", "provenance_record"]
    ]);
    expect(batch.operations.every((operation) => operation.audit_boundary.requires_user_acceptance === true && operation.operation_status === "proposed")).toBe(true);
    expect(new Set(batch.operations.map((operation) => operation.change.change_id)).size).toBe(4);
    expect(batch.operations.some((operation) => operation.target.ref === "support:D")).toBe(false);

    const [variableChange, variableRecord, constantChange, constantRecord] = batch.operations;
    expect(JSON.parse(variableChange.change.before)).toEqual(BEFORE);
    expect(JSON.parse(variableChange.change.after)).toEqual({
      family: "variable_spring_hanger",
      restraints: ["UZ"],
      provenance: `library:hanger:${LIBRARY}#VS-200-L`,
      hanger: {
        hanger_type: "variable_spring_hanger",
        source_reference: "invented-local:VS-200-L",
        stiffness: { dof: "UZ", value: { value: 200000, unit: "N/m" } },
        travel_range: { value: 50, unit: "mm" },
        installed_load: { value: 5197.5, unit: "N" },
        cold_load: { value: 5197.5, unit: "N" },
        hot_load: { value: 5000, unit: "N" }
      }
    });
    expect(variableChange.change.source_note).toBe(
      `${runNote(document, "VS-200-L", await recordSha(lib, "VS-200-L"))} ` +
      "Sized values: hanger.cold_load 5197.5 N and hanger.installed_load 5197.5 N, both the sized cold load (the setting at the drawn position); " +
      `hanger.hot_load 5000 N, the sized operating load in the design operating case load:O1. ${FROM_RECORD("VS-200-L")}`
    );
    expect(variableRecord.change.before).toBe("not_present");
    expect(variableRecord.change.source_note).toBe(`library:hanger:${LIBRARY}#VS-200-L`);

    expect(JSON.parse(constantChange.change.before)).toEqual(BEFORE);
    expect(JSON.parse(constantChange.change.after)).toEqual({
      family: "constant_effort_support",
      restraints: ["UZ"],
      provenance: `library:hanger:${LIBRARY}#CE-1`,
      hanger: {
        hanger_type: "constant_effort_support",
        source_reference: "invented-local:CE-1",
        travel_range: { value: 100, unit: "mm" },
        constant_load: { value: 3714.285714285714, unit: "N" }
      }
    });
    expect(constantChange.change.source_note).toBe(
      `${runNote(document, "CE-1", await recordSha(lib, "CE-1"))} Sized value: hanger.constant_load 3714.285714285714 N, the hot-load basis. ${FROM_RECORD("CE-1")}`
    );
    expect(constantRecord.change.source_note).toBe(`library:hanger:${LIBRARY}#CE-1`);

    expect(prepared.changed).toEqual([
      { support: "support:B", hangerId: "VS-200-L", family: "variable_spring" },
      { support: "support:C", hangerId: "CE-1", family: "constant_effort" }
    ]);
    expect(prepared.unchanged).toEqual(["support:D"]);
    expect(prepared.description).toBe(
      "Applies 2 sized selections of this run in one batch: support:B, record VS-200-L (variable spring); support:C, record CE-1 (constant effort). " +
      "Left unchanged: no record fits: support:D."
    );
    // Preparing writes nothing.
    expect(await canonicalJsonString(source)).toBe(original);
  });

  it("prepares the product's own four-span result: both constant selections with their hot-load bases, and support:D left unchanged", async () => {
    const source = model();
    const lib = library();
    const document = productResult();
    const prepared = await prepareRunApplication(document, source, lib);
    if (prepared.kind !== "prepared") throw new Error(prepared.reason);
    const configurations = prepared.batch.operations.filter((operation) => operation.change.field_path === "configuration");
    expect(configurations.map((operation) => [operation.target.ref, JSON.parse(operation.change.after).hanger.constant_load])).toEqual([
      ["support:B", { value: 4571.428571428572, unit: "N" }],
      ["support:C", { value: 3714.285714285714, unit: "N" }]
    ]);
    expect(configurations[0].change.source_note).toContain(`record CE-2 sha256 ${await recordSha(lib, "CE-2")}`);
    expect(configurations[0].change.source_note).toContain(`sizing request sha256 ${document.inputs.sizing_request_sha256}`);
    expect(prepared.description).toBe(
      "Applies 2 sized selections of this run in one batch: support:B, record CE-2 (constant effort); support:C, record CE-1 (constant effort). " +
      "Left unchanged: no record fits: support:D."
    );
  });

  it("offers none of the run when one selection cannot be applied, naming that support", async () => {
    const source = model();
    const lib = library();
    const document = await formed(source, lib, [
      constant("support:C", "node:C", await recordSha(lib, "CE-1")),
      variable("support:B", "node:B", await recordSha(lib, "VS-200-UY"), "VS-200-UY")
    ]);
    const prepared = await prepareRunApplication(document, source, lib);
    expect(prepared.kind).toBe("not_offered");
    expect(prepared.kind === "not_offered" && prepared.reason).toBe(
      "support:B: Record VS-200-UY declares its stiffness on UY, but support:B was sized on UZ, so it is not offered. " +
      "The run is applied whole or not at all, so none of its selections is offered."
    );
  });

  it("offers none of the run when the builder refuses one selection, naming that support", async () => {
    const source = model();
    const lib = library();
    const refused = constant("support:C", "node:C", await recordSha(lib, "CE-1"));
    (refused.selected as { load_n: number }).load_n = -1;
    const document = await formed(source, lib, [variable("support:B", "node:B", await recordSha(lib, "VS-200-L")), refused]);
    const prepared = await prepareRunApplication(document, source, lib);
    expect(prepared.kind === "not_offered" && prepared.reason).toBe(
      "support:C: A sized load is not a positive finite force; nothing is applied. The run is applied whole or not at all, so none of its selections is offered."
    );
  });

  it("offers nothing when the model has changed since the run, or when no support has a fitting record", async () => {
    const source = model();
    const lib = library();
    const document = await mixedRun(source, lib);
    const changed = model();
    changed.supports.find((support) => support.id === "support:D")!.provenance = "edited after sizing";
    expect(await prepareRunApplication(document, changed, lib)).toEqual({ kind: "not_offered", reason: HANGER_SIZING_MODEL_CHANGED_TEXT });
    const none = await formed(source, lib, [noFit("support:D", "node:D")]);
    expect(await prepareRunApplication(none, source, lib)).toEqual({ kind: "not_offered", reason: HANGER_SIZING_RUN_NOTHING_TEXT });
  });

  it("describes a run with nothing left unchanged without the unchanged clause", () => {
    expect(describeRunApplication([{ support: "support:B", hangerId: "CE-1", family: "constant_effort" }], [])).toBe(
      "Applies 1 sized selection of this run in one batch: support:B, record CE-1 (constant effort)."
    );
  });
});
