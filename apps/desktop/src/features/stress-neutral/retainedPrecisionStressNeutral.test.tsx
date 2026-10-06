/** T6S-3 and T6S-5 (I74 PLAN 1.1 and 1.3; RR decisions 2, 3, 5 and 12): the
 * per-route desktop output policy, and stress-neutral export of the F2a preview
 * successor `openpipestress.result_semantics/0.3.0/preview-physics-retained-1`.
 *
 * Inputs: PP's pinned milestone successors (D-U6-5, by the carrier case file's
 * sha256), delivered through mocked IPC with their pinned request models, so that
 * the product's own capture registers them; and the shared corpus's synthetic
 * two-case bases, read by id. The policy's own tests are in
 * `../results/outputPolicy.test.ts`. Unit tests over mocked IPC only: NOT a native
 * witness (CQ-10 and CQ-11 keep that for B8). */
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
vi.mock("../result-export/nativeResultSave", () => ({ isNativeResultSaveRuntime: vi.fn(() => false), saveNativeResultJson: vi.fn() }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { canonicalSha256HexCheckedV1, checkedJsonText } from "../../services/hashService";
import { buildAnalysisRunPreview, runPreviewMechanics, type PreviewSolverMode } from "../../services/previewService";
import { buildCurrentSessionInputManifest, type CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { numericalResultStanding, RETAINED_PRECISION_DOWNGRADE_FORBIDDEN } from "../results/numericalResultQuality";
import {
  RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED, RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE, RETAINED_PRECISION_VALIDATION_REQUIRED, registerRetainedPrecision,
} from "../results/retainedPrecisionStanding";
import { validateRetainedPrecision, type RowClassification } from "../results/retainedPrecision";
import { LOAD_REFERENCE_OUTPUT_REFUSAL, N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE, surfaceOutputRefusal } from "../results/outputPolicy";
import {
  RETAINED_ABSOLUTE_VERIFIED, RETAINED_NOT_COVERED, retainedClassDisclosure, retainedNotCoveredMessage, retainedPrecisionSummaryLine, retainedRowClassesFromReader, rustLowerExp,
} from "../results/retainedPrecisionDisclosure";
import { buildStressNeutralExportPacket, strictWitnessDisposition, validateStressNeutralExportPacket, StressNeutralExportPanel } from "./StressNeutralExportPanel";
import { ResultExportPanel } from "../result-export/ResultExportPanel";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
afterEach(() => { cleanup(); invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; });
const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const caseFile = json("fixtures/results/retained_precision_carrier_cases.json");
const corpus = json("fixtures/results/retained_precision_cases.json");
const MODES = ["sparse_interactive", "dense_scrutiny"] as const;
const ABSOLUTE_CODE = "SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-ABSOLUTE-VERIFIED";
const NOT_COVERED_CODE = "SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-NOT-COVERED";
const SUCCESSOR_TABLE = "fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json";

/** PP's pinned milestone successor bytes and their request model, checked by sha256. */
function milestone(mode: PreviewSolverMode) {
  const entry = caseFile.fixtures[`milestone_${mode}`];
  const bytes = readFileSync(resolve(root, entry.path));
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(entry.sha256);
  const doc = JSON.parse(bytes.toString("utf8"));
  return { source: doc.source as MechanicsResult, model: doc.invocation.request.model as PreviewModel };
}
/** Mocked direct IPC: the native command returns these bytes for this mode. */
async function deliver(source: MechanicsResult, model: PreviewModel, mode: PreviewSolverMode): Promise<MechanicsResult> {
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async () => structuredClone(source));
  return runPreviewMechanics(model, mode);
}
/** Invented manifest evidence for the AnalysisRun record (not the product's builder). */
function manifestFor(source: MechanicsResult, model: PreviewModel, mode: string) {
  return { manifest_ref: { object_type: "InputManifest", ref: "manifest:invented-t6s-stress-neutral" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: source.model_ref, model_payload: model }, solver_basis: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "unit-transport-replay-not-native-witness", solver_mode: mode } } } as unknown as CurrentSessionInputManifestEvidence;
}
async function eligible(mode: PreviewSolverMode) {
  const { source, model } = milestone(mode);
  const received = await deliver(source, model, mode);
  expect(numericalResultStanding(received, model)).toMatchObject({ eligible: true, findings: [] });
  const manifest = manifestFor(received, model, mode);
  const analysisRun = await buildAnalysisRunV03(received, manifest as Json, undefined, modelLoadBasisRefs(model));
  return { source, received, model, analysisRun, manifest };
}
/** A corpus base, read by id and registered with its own invocation. The corpus
 * invocation has no `materials` member, so it cannot pass through the product's
 * IPC capture (`{model, materials: []}`). TEST STAND-IN for the live native capture
 * (D-U7-4): it holds only for this exact model, so a moved model is not live. */
async function corpusBase(id: string) {
  const entry = corpus.cases.find((c: Json) => c.id === id);
  expect(entry?.id).toBe(id);
  const source = structuredClone(entry.source) as MechanicsResult, invocation = structuredClone(entry.invocation), model = structuredClone(invocation.request.model) as PreviewModel;
  const captured = checkedJsonText(model);
  await registerRetainedPrecision(source, invocation, (m) => { try { return checkedJsonText(m) === captured; } catch { return false; } });
  const analysisRun = await buildAnalysisRunV03(source, manifestFor(source, model, invocation.solver_mode) as Json, undefined, modelLoadBasisRefs(model));
  return { source, model, analysisRun, entry };
}
/** The preview-physics-1 control, through the native transport replay (not a native witness). */
async function previewControl(mode: PreviewSolverMode = "sparse_interactive") {
  const { model } = nativeMechanicsReplayPair(mode, { profile: "preview" });
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(createNativeMechanicsReplay({ profile: "preview" }).invoke);
  const result = await runPreviewMechanics(model, mode);
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "unit-transport-replay", solver_mode: mode, settings: {} }, active_rule_packs: [], external_assets: [] });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  return { model, result, analysisRun };
}
async function loadReferenceScenario() {
  const result = json("fixtures/product_preview/load_reference/pressure-sparse_interactive.raw.json") as MechanicsResult;
  const model = structuredClone(json("fixtures/product_preview/load_reference/pressure.request.json").model) as PreviewModel;
  const analysisRun = await buildAnalysisRunV03(result, manifestFor(result, model, "sparse_interactive") as Json, undefined, modelLoadBasisRefs(model));
  return { result, model, analysisRun };
}
const moved = (model: PreviewModel) => { const copy = structuredClone(model) as Json; copy.nodes[0].position.x += 1; return copy as PreviewModel; };
async function rawSha(text: string) { const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(text)); return [...new Uint8Array(digest)].map(x => x.toString(16).padStart(2, "0")).join(""); }
/** Recomputes every member, manifest and package checksum after a deliberate tamper,
 * so that only the relational check under test can refuse. */
async function rehash(packet: Json) {
  const payloads: Json = { "stress_neutral_results.csv": packet.csv_text, "result_rows.json": packet.result_rows, "unit_system_disclosure.json": packet.unit_system_disclosure, "unit_preservation_witnesses.json": packet.unit_preservation_witnesses, "stable_id_map.json": packet.stable_id_map, "loss_report.json": packet.loss_report, "validation_report.json": packet.validation_report, "diagnostics.json": packet.diagnostics };
  const non = packet.manifest.checksums.filter((x: Json) => x.payload_ref.ref !== "manifest.json");
  for (const c of non) c.value = c.payload_ref.ref.endsWith(".csv") ? await rawSha(payloads[c.payload_ref.ref]) : await canonicalSha256HexCheckedV1(payloads[c.payload_ref.ref]);
  const seed = { manifest_id: packet.manifest.manifest_id, source_result_ref: packet.source_result_ref, source_run_ref: packet.source_run_ref, source_model_ref: packet.source_model_ref, received_source_checksums: packet.received_source_checksums, unresolved_assumption_refs: packet.unresolved_assumption_refs, reproducibility_refs: packet.reproducibility_refs, export_profile_ref: packet.manifest.export_profile_ref, boundary_notes: packet.manifest.boundary_notes, member_checksums: non, diagnostics: packet.diagnostics };
  const m = packet.manifest.checksums.find((x: Json) => x.payload_ref.ref === "manifest.json"); m.value = await canonicalSha256HexCheckedV1(seed);
  const by = new Map(packet.manifest.checksums.map((x: Json) => [x.payload_ref.ref, x]));
  for (const member of packet.manifest.package_members) member.checksum = structuredClone(by.get(member.filename));
  const projection = structuredClone(packet); delete projection.package_checksum;
  packet.package_checksum.value = await canonicalSha256HexCheckedV1(projection);
  return packet;
}

describe("the panels (T6S-3): load/reference-state refusal and the summary line", () => {
  it("the stress-neutral builder and validator keep T1's load/reference-state refusal", async () => {
    const { result, model, analysisRun } = await loadReferenceScenario();
    await expect(buildStressNeutralExportPacket({ model, result, analysisRun })).rejects.toThrow(LOAD_REFERENCE_OUTPUT_REFUSAL);
    await expect(validateStressNeutralExportPacket({ schema_version: "0.3.0" }, result)).rejects.toThrow(LOAD_REFERENCE_OUTPUT_REFUSAL);
  });
  it.each(MODES)("%s: each panel shows one text-only per-case summary line from classificationSummary", async (mode) => {
    const { received, model, analysisRun, manifest } = await eligible(mode);
    const line = "Retained precision, per case: case: 69 verified only to an absolute bound; 0 uncovered.";
    expect(retainedPrecisionSummaryLine(received, model)).toBe(line);
    const before = checkedJsonText(received);
    render(<StressNeutralExportPanel model={model} result={received} analysisRun={analysisRun} />);
    expect(screen.getByTestId("stress-neutral-retained-precision-summary").textContent).toBe(line);
    cleanup();
    render(<ResultExportPanel model={model} result={received} analysisRun={analysisRun} inputManifest={manifest} />);
    expect(screen.getByTestId("result-export-retained-precision-summary").textContent).toBe(line);
    expect(checkedJsonText(received)).toBe(before);
    // Unregistered bytes have no validated classes: no line.
    expect(retainedPrecisionSummaryLine(structuredClone(received), model)).toBeNull();
    expect(retainedPrecisionSummaryLine((await previewControl()).result, model)).toBeNull();
  });
});

describe("the disclosure texts are Rust derivative's (D-U6-2; CQ-1)", () => {
  /** Rust's `{:e}` of each binary64 word, computed by Rust (`format!("{:e}", f64::from_bits(w))`), never by a
   * JavaScript or Python printer, which round an exact decimal tie to even (RV101 SF-1). Sources: RV101's
   * oracle (`REVIEW_RV101/t6s_01/evidence/oracle/exp_differences.txt`, the nine ties its 60,000 random words
   * found) and I75's REPAIR_01 oracle (17- and 16-digit ties in both rounding directions, powers of two, edges). */
  const RUST_LOWER_EXP: [string, string][] = [
    // RV101's nine: exact ties whose lower 17-digit candidate is even, where V8 printed the lower one.
    ["43180467b3a7ed6d", "1.6900607208313233e15"], ["c30c9bbde3da0f6a", "-1.0065674027176773e15"], ["42ea8090bb0f6d84", "2.3311589051479613e14"],
    ["c31a75bb9d549349", "-1.8619495133442103e15"], ["c318d4bc7a74fae9", "-1.7473263536206663e15"], ["c31cb54f6b434481", "-2.0201630136281923e15"],
    ["c2e29836995a3554", "-1.6355968278980263e14"], ["4318648617b9f199", "1.7164816318782463e15"], ["c30d5a455d56eb5a", "-1.0327535362287153e15"],
    // REPAIR_01, further 17-digit ties whose lower candidate is even: Rust rounds up, where V8 printed the lower one.
    ["41e828b048392000", "3.2425580177851563e9"], ["42e86ef03293cd54", "2.1491804252529063e14"], ["43125257db1bdf89", "1.2892717181152983e15"],
    ["43192e4c68f9b819", "1.7719450328386623e15"], ["c27860cd4b34c280", "-1.6752525115641563e12"], ["c2e8a8db907a3504", "-2.1690813800900013e14"],
    ["c312425386148ee5", "-1.2848690200012093e15"], ["c318e6d73ae9a7dd", "-1.7523028804510633e15"],
    // REPAIR_01, 17-digit ties whose lower candidate is odd: Rust and V8 both print the upper, even one.
    ["4214380e5eecf800", "2.1709952955242188e10"], ["42ee8b1a4d04b53c", "2.6866232434013788e14"], ["43175a4e0a0b4a4b", "1.6433039217015228e15"],
    ["c27fc97cc9b3b580", "-2.1843901550673438e12"], ["c2ef2de9841ccc6c", "-2.7425641391062738e14"], ["c316f95fb90dd717", "-1.6166597523839418e15"],
    // REPAIR_01, exact 18-digit decimals whose shortest round-trip form is shorter than 17 digits, with no tie at that length.
    ["41fb8c9dcbe67000", "7.395204286402344e9"], ["42fd60557ba2c72a", "5.167934117100666e14"], ["430c2cfcbe3bbac9", "9.913454228580731e14"], ["c2fdab6df7dfbc2e", "-5.219539451893149e14"],
    // REPAIR_01, 16-digit ties whose lower candidate is even: Rust rounds up, where V8 printed the lower one.
    ["4308628432e3716a", "8.579649212534213e14"], ["c30731c42934190a", "-8.160803798720333e14"], ["42a00d5eddf248a0", "8.824806111524313e12"],
    ["42d53a7c5f8c5008", "9.336375569235213e13"], ["4305eb94c439580a", "7.712498363379213e14"], ["c2a0c37b6b45b520", "-9.215887647450563e12"],
    ["c2d58f3c07cc4028", "-9.481972660249663e13"], ["c305d338a1728242", "-7.679018363986643e14"],
    // REPAIR_01, 16-digit ties whose lower candidate is odd: Rust and V8 both print the upper, even one.
    ["42a01f6d5421fbe0", "8.863582130429938e12"], ["42d5535b114624b8", "9.379101731035488e13"], ["43061913d79e8386", "7.775028125246568e14"],
    ["c2a054a3462dc360", "-8.977851291361688e12"], ["c2d48c0cbcee3f18", "-9.036696674124438e13"], ["c305214e406a1be6", "-7.434493103481568e14"],
    // REPAIR_01, powers of two whose nearest decimal of the shortest length falls outside the narrower lower interval: the shortest form stands.
    ["0060000000000000", "7.120236347223045e-307"], ["2160000000000000", "6.256509672447191e-148"], ["5580000000000000", "7.167183174968974e103"],
    ["8060000000000000", "-7.120236347223045e-307"], ["a160000000000000", "-6.256509672447191e-148"], ["d580000000000000", "-7.167183174968974e103"],
    // REPAIR_01, 17-digit ties at a power of two and just above one.
    ["3e60000000000000", "2.9802322387695313e-8"], ["be60000000000000", "-2.9802322387695313e-8"], ["4310000000000001", "1.1258999068426243e15"], ["c310000000000001", "-1.1258999068426243e15"],
    // REPAIR_01, edges: integers, halves, 1e21, 2^53, the least subnormal, the subnormal/normal boundary, the greatest finite, both zeros, five corpus bounds.
    ["3ff0000000000000", "1e0"], ["3ff8000000000000", "1.5e0"], ["40934a0000000000", "1.2345e3"], ["444b1ae4d6e2ef50", "1e21"], ["4340000000000000", "9.007199254740992e15"],
    ["3fe0000000000000", "5e-1"], ["0000000000000001", "5e-324"], ["000fffffffffffff", "2.225073858507201e-308"], ["0010000000000000", "2.2250738585072014e-308"],
    ["7fefffffffffffff", "1.7976931348623157e308"], ["0000000000000000", "0e0"], ["8000000000000000", "-0e0"], ["bff8000000000000", "-1.5e0"],
    ["3b1a378ea78c5ce9", "5.4215527659630466e-24"], ["3b73a92a30553261", "2.6020852139652104e-22"], ["3b8d7dbf487fcb91", "7.806255641895631e-22"],
    ["3c4b0f1c3b53cee9", "2.9337453183237533e-18"], ["3c62f327f1759218", "8.218177976187931e-18"],
  ];
  const fromWord = (word: string) => { const view = new DataView(new ArrayBuffer(8)); view.setBigUint64(0, BigInt(`0x${word}`)); return view.getFloat64(0); };
  it("prints b as Rust's {:e} does: shortest round-trip digits, exact ties rounded up, and no '+' in the exponent", () => {
    for (const [word, text] of RUST_LOWER_EXP) expect([word, rustLowerExp(fromWord(word))]).toStrictEqual([word, text]);
    // A tie bound prints in a disclosure exactly as Rust's class_disclosure prints it.
    expect(retainedClassDisclosure("support_reaction_component_v2", "N", { result_id: "r", basis_ref: { ref_type: "load_case", ref_id: "case" }, normalized_bits: "0000000000000000", scale_bits: null, bound_bits: "43180467b3a7ed6d", class: "absolute_verified" })!.message)
      .toContain("b = 1.6900607208313233e15 N (binary64 43180467b3a7ed6d)");
    expect(retainedClassDisclosure("displacement_magnitude", "mm", { result_id: "r", basis_ref: { ref_type: "load_case", ref_id: "case" }, normalized_bits: "0000000000000000", scale_bits: null, bound_bits: "4308628432e3716a", class: "absolute_verified" })!.message)
      .toContain("b = 8.579649212534213e14 m (binary64 4308628432e3716a)");
    for (const value of [Number.NaN, Number.POSITIVE_INFINITY]) expect(() => rustLowerExp(value)).toThrow("RETAINED_PRECISION_BOUND_NOT_FINITE");
  });
  it("states Rust class_disclosure's code and message for each class", () => {
    const row = (cls: RowClassification["class"], bound: string | null): RowClassification => ({ result_id: "r", basis_ref: { ref_type: "load_case", ref_id: "case" }, normalized_bits: "0000000000000000", scale_bits: null, bound_bits: bound, class: cls });
    expect(retainedClassDisclosure("displacement_magnitude", "mm", row("absolute_verified", "3b1a378ea78c5ce9"))).toStrictEqual({ code: RETAINED_ABSOLUTE_VERIFIED, message: "displacement_magnitude: retained_precision_absolute_verified; verified only to the receipt's absolute bound b = 5.4215527659630466e-24 m (binary64 3b1a378ea78c5ce9), below the relative accuracy floor; source value/unit and annotation retained; withheld from rule binding and reliance" });
    expect(retainedClassDisclosure("support_reaction_component_v2", "kN*m", row("absolute_verified", "3ff0000000000000"))!.message).toContain("b = 1e0 N*m (binary64 3ff0000000000000)");
    expect(retainedClassDisclosure("open_formula_stress_summary", "MPa", row("absolute_verified", "0000000000000001"))!.message).toContain("b = 5e-324 Pa (binary64 0000000000000001)");
    const uncovered = { code: RETAINED_NOT_COVERED, message: "k: retained_precision_not_covered; no verified accuracy for this quantity kind; source value/unit and annotation retained; withheld from rule binding and reliance" };
    expect(retainedClassDisclosure("k", "N", row("not_covered", null))).toStrictEqual(uncovered);
    // A unit the reader does not normalize names no bound: the row is disclosed as uncovered.
    expect(retainedClassDisclosure("k", "kPa", row("absolute_verified", "3b1a378ea78c5ce9"))).toStrictEqual(uncovered);
    for (const cls of ["relative_verified", "input_derived", "non_quantity"] as const) expect(retainedClassDisclosure("k", "N", row(cls, null))).toBeNull();
    expect(retainedClassDisclosure("k", "N", undefined)).toBeNull();
    expect(() => retainedClassDisclosure("k", "N", row("absolute_verified", null))).toThrow("RETAINED_PRECISION_BOUND_MISSING");
  });
});

describe("the package's withholding precedence (checkpoint reading R-1)", () => {
  it("a blocking disposition keeps precedence over a class; a class takes precedence over diagnostic work", () => {
    const carrier = milestone("sparse_interactive").source;
    const base = carrier.results.find(r => r.id === "result:disp:N0")!;
    const classed = (id: string): ReadonlyMap<string, RowClassification> => new Map([[id, { result_id: id, basis_ref: { ref_type: "load_case", ref_id: "case" }, normalized_bits: "0000000000000000", scale_bits: null, bound_bits: null, class: "not_covered" }]]);
    // An unknown kind is a blocking unknown-semantic withholding, class or not.
    const unknown = { ...structuredClone(base), id: "r:unknown", kind: "unknown_native_quantity" };
    expect(strictWitnessDisposition(unknown, carrier, null).disposition).toBe("unknown_semantic");
    expect(strictWitnessDisposition(unknown, carrier, classed("r:unknown")).disposition).toBe("unknown_semantic");
    // A diagnostic-work row (info) takes its class instead, with Rust's message.
    const work = { ...structuredClone(base), id: "r:work", kind: "nonlinear_support_free_dof_work_residual", unit: "N*m", metadata: { ...structuredClone(base.metadata!), component: "free_dof_work_residual" } } as MechanicsResult["results"][number];
    expect(strictWitnessDisposition(work, carrier, null).disposition).toBe("diagnostic_work");
    expect(strictWitnessDisposition(work, carrier, classed("r:work"))).toMatchObject({ disposition: "retained_not_covered", message: retainedNotCoveredMessage("nonlinear_support_free_dof_work_residual") });
    // Without a class, an ordinary row keeps its witness.
    expect(strictWitnessDisposition(base, carrier, classed("r:other")).disposition).toBe("eligible");
  });
});

describe.each(MODES)("%s: stress-neutral export of an eligible successor (T6S-5)", (mode) => {
  it("builds and validates a package that carries the receipt, the evidence and the class findings", async () => {
    const { received, model, analysisRun } = await eligible(mode);
    const before = checkedJsonText({ received, model, analysisRun });
    const packet = await buildStressNeutralExportPacket({ model, result: received, analysisRun });
    expect(checkedJsonText({ received, model, analysisRun })).toBe(before);
    expect(packet.schema_version).toBe("0.3.0");
    expect(checkedJsonText(packet.retained_precision)).toBe(checkedJsonText(received.retained_precision));
    expect(checkedJsonText(packet.contract_evidence)).toBe(checkedJsonText(received.contract_evidence));
    expect(packet.semantic_contract.id).toBe("openpipestress.result_semantics/0.3.0/preview-physics-retained-1");
    expect(packet.export_profile).toMatchObject({ profile_id: "ops.stress_neutral.v3", csv_encoding: "utf-8", csv_row_order: "unicode_scalar_value_result_id" });
    expect(packet.export_profile.source_basis_refs).toContainEqual({ object_type: "ExternalReference", ref: SUCCESSOR_TABLE });
    expect(packet.manifest.checksums.find((c: Json) => c.payload_ref.ref === "stress_neutral_results.csv").canonicalization).toBe("utf8_csv_record_lf_v1");
    expect(packet.source_annotations).toHaveLength(received.results.length);
    // Every CSV row and value is unchanged.
    expect(packet.result_rows).toHaveLength(received.results.length);
    for (const row of packet.result_rows) expect(row.value).toBe(received.results.find(r => r.id === row.result_id)!.value);
    expect(packet.csv_text.trimEnd().split("\n")).toHaveLength(received.results.length + 1);
    // S-d: each absolute_verified row's witness is withheld with an info finding carrying Rust's message.
    const classes = (await validateRetainedPrecision(received)).classifications;
    const absolute = classes.filter(c => c.class === "absolute_verified");
    expect(absolute).toHaveLength(69);
    expect(classes.filter(c => c.class === "not_covered")).toHaveLength(0);
    const findings = packet.diagnostics.filter((d: Json) => d.code === ABSOLUTE_CODE);
    expect(findings.map((d: Json) => d.source.ref).sort()).toStrictEqual(absolute.map(c => c.result_id).sort());
    expect(packet.diagnostics.filter((d: Json) => d.code === NOT_COVERED_CODE)).toHaveLength(0);
    for (const finding of findings) {
      const raw = received.results.find(r => r.id === finding.source.ref)!;
      expect(finding).toMatchObject({ class: "unit_preservation_witness", severity: "info", message: retainedClassDisclosure(raw.kind, raw.unit, absolute.find(c => c.result_id === raw.id))!.message });
      expect(packet.unit_preservation_witnesses.some((w: Json) => w.result_id === raw.id)).toBe(false);
    }
    expect(findings.find((d: Json) => d.source.ref === "result:disp:N0").message).toBe("displacement_magnitude: retained_precision_absolute_verified; verified only to the receipt's absolute bound b = 5.4215527659630466e-24 m (binary64 3b1a378ea78c5ce9), below the relative accuracy floor; source value/unit and annotation retained; withheld from rule binding and reliance");
    const withheld = packet.diagnostics.filter((d: Json) => d.class === "unit_preservation_witness").length;
    expect(packet.unit_preservation_witnesses.length + withheld).toBe(received.results.length);
    expect(packet.loss_report.find((entry: Json) => entry.category === "exported").reason).toContain(`and ${withheld} rows have explicit witness-withholding findings. Of these, 69 rows are verified only to the retained-precision receipt's absolute bound and 0 rows are uncovered; both are withheld from rule binding and reliance.`);
    expect(packet.export_profile.boundary_notes.some((note: string) => note.startsWith("The retained-precision receipt travels whole with this package."))).toBe(true);
    await expect(validateStressNeutralExportPacket(packet, received, analysisRun, modelLoadBasisRefs(model))).resolves.toBeUndefined();
    // I67's F4 closed: the transport header carries the receipt, and a header-only check passes.
    await expect(validateStressNeutralExportPacket(structuredClone(packet))).resolves.toBeUndefined();
  });
  it("refuses each negative control with its expected code", async () => {
    const { source, received, model, analysisRun } = await eligible(mode);
    const build = (result: MechanicsResult, m: PreviewModel = model) => buildStressNeutralExportPacket({ model: m, result, analysisRun });
    await expect(build(structuredClone(received))).rejects.toThrow(RETAINED_PRECISION_VALIDATION_REQUIRED); // a copied successor
    await expect(build(received, moved(model))).rejects.toThrow(RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED); // a moved model
    await expect(build(structuredClone(source))).rejects.toThrow(RETAINED_PRECISION_VALIDATION_REQUIRED); // an unregistered build's bytes
    const relabelled = structuredClone(received) as Json;
    relabelled.producer.semantic_contract_id = "openpipestress.result_semantics/0.3.0/preview-physics-1";
    relabelled.formulation_basis.profile_id = "product_preview_mechanics_v1";
    await expect(build(relabelled)).rejects.toThrow("SN-SOURCE-CONTRACT-UNSUPPORTED"); // a relabelled statement
    const packet = await build(received);
    const tamper = async (edit: (p: Json) => void) => { const p = structuredClone(packet); edit(p); return rehash(p); };
    const withSource = (p: Json) => validateStressNeutralExportPacket(p, received, analysisRun, modelLoadBasisRefs(model));
    // A relabelled package header reads unsupported (the downgrade guard).
    const relabelledHeader = await tamper(p => { p.producer.semantic_contract_id = "openpipestress.result_semantics/0.3.0/preview-physics-1"; p.formulation_basis.profile_id = "product_preview_mechanics_v1"; });
    await expect(validateStressNeutralExportPacket(relabelledHeader)).rejects.toThrow("SN-PRECISION-CONTRACT-MISMATCH");
    // A forged class code, an edited disclosure message and a wrong severity.
    const firstAbsolute = (p: Json) => p.diagnostics.find((d: Json) => d.code === ABSOLUTE_CODE);
    const forged = await tamper(p => { firstAbsolute(p).code = NOT_COVERED_CODE; });
    await expect(withSource(forged)).rejects.toThrow("SN-PRECISION-WITNESS-BINDING-MISMATCH");
    const edited = await tamper(p => { firstAbsolute(p).message = firstAbsolute(p).message.replace("withheld from rule binding and reliance", "bindable"); });
    await expect(withSource(edited)).rejects.toThrow("SN-PRECISION-WITNESS-BINDING-MISMATCH");
    const severity = await tamper(p => { firstAbsolute(p).severity = "blocking"; });
    await expect(validateStressNeutralExportPacket(severity)).rejects.toThrow("SN-WITNESS-CATEGORY-ACCOUNTING-MISMATCH");
    await expect(withSource(severity)).rejects.toThrow("SN-PRECISION-WITNESS-BINDING-MISMATCH");
    // A receipt that does not bind the source, or none at all.
    const unbound = await tamper(p => { p.retained_precision.receipt_sha256 = "0".repeat(64); });
    await expect(withSource(unbound)).rejects.toThrow("SN-RETAINED-PRECISION-RECEIPT-MISMATCH");
    await expect(validateStressNeutralExportPacket(unbound)).rejects.toThrow("RETAINED_PRECISION_RECEIPT_MISMATCH");
    const missing = await tamper(p => { delete p.retained_precision; });
    await expect(withSource(missing)).rejects.toThrow("SN-RETAINED-PRECISION-RECEIPT-MISMATCH");
    await expect(validateStressNeutralExportPacket(missing)).rejects.toThrow("SN-PRECISION-CONTRACT-MISMATCH");
    // The loss report's counted reason binds the class findings.
    const recount = await tamper(p => { const entry = p.loss_report.find((e: Json) => e.category === "exported"); entry.reason = entry.reason.replace("Of these, 69 rows", "Of these, 68 rows"); });
    await expect(withSource(recount)).rejects.toThrow("SN-RETAINED-PRECISION-LOSS-COUNT-MISMATCH");
    // Annotations bind the supplied source.
    const annotations = await tamper(p => { p.source_annotations[0].source_row.value += 1; p.source_annotations[0].source_row_sha256 = "0".repeat(64); });
    await expect(withSource(annotations)).rejects.toThrow("SN-SOURCE-ANNOTATION-BINDING-MISMATCH");
  });
  it("shows the package in the panel, and the specific standing reason otherwise", async () => {
    const { received, model, analysisRun } = await eligible(mode);
    render(<StressNeutralExportPanel model={model} result={received} analysisRun={analysisRun} />);
    expect((await screen.findByTestId("stress-neutral-summary", undefined, { timeout: 10_000 })).textContent).toContain(`available; rows=${received.results.length}`);
    expect(screen.queryByTestId("stress-neutral-load-reference-output-unavailable")).toBeNull();
    cleanup();
    render(<StressNeutralExportPanel model={moved(model)} result={received} analysisRun={analysisRun} />);
    expect(screen.getByTestId("stress-neutral-load-reference-output-unavailable").textContent).toBe(`${RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}`);
    expect(screen.queryByTestId("stress-neutral-summary")).toBeNull();
    expect(document.querySelectorAll("a[download]")).toHaveLength(0);
  });
});

describe("a receipt on another identity is refused", () => {
  it("a preview-physics-1 package carrying a receipt", async () => {
    const { model, result, analysisRun } = await previewControl();
    const packet = await buildStressNeutralExportPacket({ model, result, analysisRun });
    await expect(validateStressNeutralExportPacket(packet, result)).resolves.toBeUndefined();
    const carried = structuredClone(packet);
    carried.retained_precision = structuredClone(milestone("sparse_interactive").source.retained_precision);
    await rehash(carried);
    await expect(validateStressNeutralExportPacket(carried, result)).rejects.toThrow(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);
    await expect(validateStressNeutralExportPacket(carried)).rejects.toThrow("SN-PRECISION-CONTRACT-MISMATCH");
    const legacy = { schema_version: "0.2.0", retained_precision: carried.retained_precision };
    await expect(validateStressNeutralExportPacket(legacy)).rejects.toThrow("SN-LEGACY-PRECISION-METADATA-FORBIDDEN");
  });
});

describe("the corpus's synthetic two-case bases, read by id", () => {
  it("admits two_case_synthetic when eligible", async () => {
    const { source, model, analysisRun } = await corpusBase("two_case_synthetic");
    expect(numericalResultStanding(source, model)).toMatchObject({ eligible: true, findings: [] });
    expect(numericalResultStanding(source, moved(model))).toMatchObject({ eligible: false, findings: [RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED] });
    const classes = (await retainedRowClassesFromReader(source))!;
    const absolute = [...classes.values()].filter(c => c.class === "absolute_verified");
    expect(absolute).toHaveLength(68);
    const perCase = (id: string) => absolute.filter(c => c.basis_ref.ref_id === id).length;
    expect(retainedPrecisionSummaryLine(source, model)).toBe(`Retained precision, per case: case:six-component-load: ${perCase("case:six-component-load")} verified only to an absolute bound; 0 uncovered. case:zero-load: ${perCase("case:zero-load")} verified only to an absolute bound; 0 uncovered.`);
    const packet = await buildStressNeutralExportPacket({ model, result: source, analysisRun });
    expect(packet.diagnostics.filter((d: Json) => d.code === ABSOLUTE_CODE).map((d: Json) => d.source.ref).sort()).toStrictEqual(absolute.map(c => c.result_id).sort());
    await expect(validateStressNeutralExportPacket(packet, source, analysisRun, modelLoadBasisRefs(model))).resolves.toBeUndefined();
    await expect(validateStressNeutralExportPacket(structuredClone(packet))).resolves.toBeUndefined();
  });
  it("refuses two_case_facade_after_certificate_synthetic, which is not numerically eligible", async () => {
    const { source, model, analysisRun } = await corpusBase("two_case_facade_after_certificate_synthetic");
    expect(numericalResultStanding(source, model)).toMatchObject({ eligible: false, findings: [RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE] });
    expect(surfaceOutputRefusal(source, model, "stress-neutral")).toBe(`${RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}`);
    await expect(buildStressNeutralExportPacket({ model, result: source, analysisRun })).rejects.toThrow(RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE);
  });
});
