/** Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §5.4, §6.1, §6.2; owner ruling
 * 11B, 2026-10-21): the committed load/reference-state twins on the desktop's result JSON
 * export and stress-neutral export.
 *
 * - Validator acceptance: the desktop validators accept Rust's committed
 *   `*.document.json` and Python's committed `*.stress_neutral.json` (with its
 *   `*.analysis_run.json` record), each with the source raw its writer read
 *   (`load_reference_contract.rs`, `load_reference_source_contract.rs`,
 *   `test_stress_neutral_physics_source.py`, `test_load_reference_readers.py`):
 *   4 pressure-2 and 10 pressure-source-1 inputs.
 * - The admitted-case oracles: the pressure-2 load/reference-state twins, and the plain
 *   twins of both identities (`pressure_plain_connected_*`, written by the headless
 *   runner's `pressure_export.rs` and Python's `test_stress_neutral_physics_source.py`;
 *   `pressure_source_plain_n05_*`, by `physics_source_connected.rs` and the same Python
 *   test). Each committed raw is solved through the native transport replay (as
 *   `loadReference.resultsSessionState.test.ts` solves it), recorded and manifested by
 *   the desktop, and exported by the Current builders; the source-derived members equal
 *   the committed twins'. Run-specific members are excluded by name below.
 *   `load_basis_refs` is checked against its own basis (§6.2).
 * - The panels show their link and their disclosure line for these results; a model edit
 *   voids both. The refusals the design keeps are asserted: the pressure-source-1 twins
 *   and the pressure-2 fallback by standing, a copy, and a missing or mismatched record.
 *
 * Unit transport replay of committed producer bytes through mocked IPC; NOT a native UI
 * qualification witness. No digest is pinned. */
import { cleanup, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
vi.mock("./nativeResultSave", () => ({ isNativeResultSaveRuntime: vi.fn(() => false), saveNativeResultJson: vi.fn() }));
import type { AnalysisRunEnvelope, MechanicsResult, ObjectRef, PreviewModel } from "../../types";
import { createSolveJobTransport } from "../../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, hasNativeMechanicsInvocation, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest, type CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE, SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED, numericalResultStanding, registerSourceRecovery } from "../results/resultStanding";
import { N_LOAD_REFERENCE_EXPORT, N_LOAD_REFERENCE_PACKAGE } from "../results/loadReferenceDisclosure";
import { buildStressNeutralExportPacket, StressNeutralExportPanel, validateStressNeutralExportPacket } from "../stress-neutral/StressNeutralExportPanel";
import { buildCurrentResultExport, currentResultDocumentBase, validateResultDocument, type JsonObject } from "./resultExportAdapter";
import { ResultExportPanel } from "./ResultExportPanel";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Mode = "sparse_interactive" | "dense_scrutiny";
afterEach(() => { cleanup(); invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; });
const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const MODES: readonly (readonly [Mode, "sparse" | "dense"])[] = [["sparse_interactive", "sparse"], ["dense_scrutiny", "dense"]];
type Route = "pressure" | "pressure_source";
/** One committed input: the source raw and request its writers read, and its twins' stem. */
type Input = { label: string; route: Route; mode: Mode; rawPath: string; requestPath: string; twin: string };
const LOAD_REFERENCE = (route: Route, folder: string, stem: string, names: readonly string[]): Input[] =>
  names.flatMap(name => MODES.map(([mode, short]) => ({ label: `${name} ${mode}`, route, mode, rawPath: `${folder}/${name}-${mode}.raw.json`, requestPath: `${folder}/${name}.request.json`, twin: `fixtures/results/${stem}_${name}_${short}` })));
const ORDINARY_FOLDER = "fixtures/product_preview/pressure/load_reference";
/** pressure-2 load/reference-state twins (`load_reference_contract.rs`, `test_load_reference_readers.py`, `test_stress_neutral_physics_source.py`). */
const ORDINARY = LOAD_REFERENCE("pressure", ORDINARY_FOLDER, "pressure_load_state", ["connected", "pressure"]);
/** pressure-source-1 load/reference-state twins (`load_reference_source_contract.rs`, `test_stress_neutral_physics_source.py`). */
const JOINED = LOAD_REFERENCE("pressure_source", "fixtures/product_preview/pressure_source/load_reference_source", "pressure_source_load_state", ["n05", "n06", "fields", "mixed", "eigen_motion"]);
/** Plain twins (headless `pressure_export.rs` and `physics_source_connected.rs`; `test_stress_neutral_physics_source.py`). */
const PLAIN: Input[] = [
  ...MODES.map(([mode, short]) => ({ label: `pressure-2 connected ${mode}`, route: "pressure" as const, mode, rawPath: `fixtures/results/pressure_connected_mechanics_${short}.json`, requestPath: "core/product_physics/tests/fixtures/exact_pressure_connected_request.json", twin: `fixtures/results/pressure_plain_connected_${short}` })),
  ...MODES.map(([mode, short]) => ({ label: `pressure-source-1 n05 ${mode}`, route: "pressure_source" as const, mode, rawPath: `fixtures/product_preview/pressure_source/physics_source/n05-${mode}.raw.json`, requestPath: "fixtures/product_preview/pressure_source/physics_source/n05.request.json", twin: `fixtures/results/pressure_source_plain_n05_${short}` })),
];
/** Twins whose CSV bytes equal the desktop's (no value whose JavaScript and Python texts differ). */
const CSV_BYTE_EQUAL = [...ORDINARY, ...PLAIN.filter(input => input.route === "pressure")].map(input => input.twin);
const FALLBACK = "core/reporting/result_export/tests/fixtures/load_reference_fallback_uz";

/** The committed twins of one input, with its source raw and request model. */
function twins(input: Input) {
  return {
    raw: json(input.rawPath) as MechanicsResult, model: json(input.requestPath).model as PreviewModel,
    document: json(`${input.twin}.document.json`) as JsonObject, packet: json(`${input.twin}.stress_neutral.json`), record: json(`${input.twin}.analysis_run.json`) as AnalysisRunEnvelope,
  };
}

/** The native transport replay of the producer's captured bytes, as
 * `loadReference.resultsSessionState.test.ts` solves them: the committed request model
 * plus the case labels the desktop session requires where the request lacks them
 * (invented labels only; nothing reads physics from them). It registers the result as
 * the product's capture does; the record and manifest are the desktop's own. */
async function solve(rawPath: string, requestPath: string, mode: Mode) {
  const originalText = readFileSync(resolve(root, rawPath), "utf8");
  const model = structuredClone(json(requestPath).model) as PreviewModel;
  for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(createSolveJobTransport((sent) => {
    expect(sent.mode).toBe(mode);
    expect(sent.model).toEqual(model);
    return originalText;
  }).invoke);
  const result = await runPreviewMechanics(model, mode);
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "test:epoch8-exact-output-unit-transport-replay", solver_mode: mode, settings: {} }, active_rule_packs: [], external_assets: [] });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  invokeMock.mockReset();
  return { model, result, inputManifest, analysisRun };
}
/** A pressure-source-1 input whose committed request has no `materials` member
 * (`eigen_motion`) cannot pass the product's IPC capture (`{model, materials: []}`), so
 * it is registered with its own committed invocation, as Rust's `reader_verdict.rs`
 * test reads it: a TEST STAND-IN for the live native capture, holding for this exact
 * model only. Its record is built over invented manifest evidence. */
async function registeredStandIn(input: Input) {
  const request = json(input.requestPath), result = json(input.rawPath) as MechanicsResult, model = structuredClone(request.model) as PreviewModel;
  await registerSourceRecovery(result, { request, solver_mode: input.mode }, model);
  const manifest = { manifest_ref: { object_type: "InputManifest", ref: "manifest:invented-epoch8-stand-in" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: result.model_ref, model_payload: model }, solver_basis: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "test:epoch8-registered-stand-in", solver_mode: input.mode } } };
  const analysisRun = await buildAnalysisRunV03(result, manifest as Json, undefined, modelLoadBasisRefs(model));
  return { model, result, inputManifest: manifest as unknown as CurrentSessionInputManifestEvidence, analysisRun };
}

/** The published reference configurations, load cases and history kinds, in order, without duplicates. */
function disclosed(template: string, source: MechanicsResult): string {
  const records = (source.contract_evidence as Json).load_reference_states as Json[];
  const distinct = (values: string[]) => values.filter((v, i) => values.indexOf(v) === i).join(", ");
  return template.replace("{configurations}", distinct(records.map(r => r.reference_configuration_id)))
    .replace("{cases}", distinct(records.map(r => r.load_case_id))).replace("{histories}", distinct(records.map(r => r.history.kind)));
}
/** Two model edits that are not the captured model: a reference configuration and an analysis state. */
const editedReference = (model: PreviewModel) => { const copy = structuredClone(model) as Json; copy.reference_configurations[0].member_references[0].basis.installation_temperature.value += 1; return copy as PreviewModel; };
const editedState = (model: PreviewModel) => { const copy = structuredClone(model) as Json; copy.load_cases[0].analysis_state.element_states[0].operating_temperature.value += 1; return copy as PreviewModel; };
/** A plain model edit that is not the captured model. */
const editedNode = (model: PreviewModel) => { const copy = structuredClone(model) as Json; copy.nodes[0].position.x += 1; return copy as PreviewModel; };

/* -------------------------------------------------------------------------- *
 * The members compared, and the run-specific members excluded by name
 * -------------------------------------------------------------------------- */
/** Document members copied from the source, compared whole (and `source_block_recovery`
 * for pressure-source-1). */
const DOCUMENT_SOURCE_MEMBERS = ["producer", "numerical_quality", "formulation_basis", "contract_evidence", "semantic_contract_ref"] as const;
/** Document row arrays (rows and accounting), compared after the exclusions below. */
const DOCUMENT_ROW_ARRAYS = ["row_accounting", "review_evidence", "row_disclosures", "source_annotations", "unit_preservation_witnesses"] as const;
/** Run- or origin-specific members inside the document's rows, excluded by name:
 * - `provenance`: the exporting document's provenance (the Rust writer's base; the
 *   desktop's `derivativeProvenance`);
 * - `original_producer_row_checksum`: the Rust writers' origin is producer-attested, the
 *   desktop's received Current carrier is not;
 * - `target_row_checksum`: a hash over the target row (which holds `provenance`) under the
 *   document's own envelope id;
 * - `source_origin_ref`: the origin binding's id;
 * - `unit_system_ref`: the document's unit system id.
 * `received_carrier_row_checksum` keeps its value (the source row's digest) and drops
 * only its origin-specific scope and payload reference. A row's `basis_ref` that falls
 * back to the document's own `run_ref` is compared as that fallback. */
const DOCUMENT_ROW_EXCLUDED = ["provenance", "original_producer_row_checksum", "target_row_checksum", "source_origin_ref", "unit_system_ref"] as const;
/** Document members not compared with the twin:
 * - run-specific: identifiers, the origin and hashes over the desktop's own record and
 *   manifest (`reproducibility`), the solver build reference (`solver_version`), and the
 *   base's own members (`provenance`, `analysis_status`, `professional_boundary`,
 *   `rule_pack_refs`, `downstream_use`);
 * - `diagnostics`, compared in `expectDiagnosticsFollowSource` instead: the desktop maps
 *   each source diagnostic as Rust's derivative does (`documentDiagnostic`), so every
 *   member but its `provenance` (each derivative's own) equals the Rust twin's where Rust
 *   maps the source (the plain twins; the load/reference writer's template carries its own);
 * - `load_basis_refs`, checked against its own basis instead (§6.2). */
const DOCUMENT_RUN_SPECIFIC = ["envelope_id", "run_ref", "model_ref", "unit_system_ref", "solver_version", "reproducibility", "provenance", "diagnostics", "analysis_status", "professional_boundary", "rule_pack_refs", "downstream_use", "load_basis_refs"] as const;
/** Package members copied or derived from the source, compared after the exclusion below
 * (and `source_block_recovery` for pressure-source-1). */
const PACKAGE_SOURCE_MEMBERS = ["result_rows", "csv_text", "unit_preservation_witnesses", "stable_id_map", "source_annotations", "contract_evidence", "formulation_basis", "producer", "numerical_quality", "semantic_contract", "semantic_contract_ref"] as const;
/** Inside the package's rows and stable-ID map, excluded by name:
 * - `provenance`: the exporting builder's (Python's `package.py` vs. the desktop's preview
 *   provenance);
 * - `export_ref`: the exporter's own row identifier. Python's writer supplies it as a test
 *   argument (`{StressNeutralRow, <result_id>}`, `arguments()` in
 *   `test_stress_neutral_physics_source.py`); the desktop builder names its rows
 *   `{StressNeutralResultRow, stress-neutral-row:<index>}`. Reported, not source-derived.
 * Every other package member (identifiers, checksums over the desktop's own record,
 * boundary notes, diagnostics, loss and validation reports, manifest) is run- or
 * builder-specific and is not compared. */
const PACKAGE_ROW_EXCLUDED = ["provenance", "export_ref"] as const;
/** The CSV's records, with the value column read as its binary64 number. The desktop
 * writes JavaScript's shortest round-trip text (`0.00009999999999999999`, `1e-8`) where
 * Python writes its `repr` (`9.999999999999999e-05`, `1e-08`): the same binary64 value,
 * different bytes. Byte equality is asserted where it holds, and reported below. */
const csvRecords = (text: string) => text.split("\n").map(line => line.split(",").map((field, i) => i === 7 && line !== text.split("\n")[0] && field !== "" ? Number(field) : field));

function documentRows(doc: JsonObject, member: string): unknown {
  const e = doc.result_envelope, runRef = JSON.stringify(e.run_ref);
  const strip = (value: Json): Json => {
    if (Array.isArray(value)) return value.map(strip);
    if (!value || typeof value !== "object") return value;
    if (JSON.stringify(value) === runRef) return "<the document's run_ref>";
    const out: Json = {};
    for (const [key, item] of Object.entries(value)) {
      if ((DOCUMENT_ROW_EXCLUDED as readonly string[]).includes(key)) continue;
      out[key] = key === "received_carrier_row_checksum" && item && typeof item === "object" ? (item as Json).value : strip(item);
    }
    return out;
  };
  return strip(member === "values" ? e.result_sets.map((set: Json) => set.values) : e[member]);
}
function packageMember(packet: Json, member: string): unknown {
  const strip = (value: Json): Json => Array.isArray(value) ? value.map(strip)
    : value && typeof value === "object" ? Object.fromEntries(Object.entries(value).filter(([key]) => !(PACKAGE_ROW_EXCLUDED as readonly string[]).includes(key)).map(([key, item]) => [key, strip(item)])) : value;
  return member === "result_rows" || member === "stable_id_map" ? strip(packet[member]) : packet[member];
}
/** The desktop document's diagnostics: one entry per source diagnostic, in order, with
 * `code` and `message` copied verbatim and `affected_object` naming the source diagnostic
 * by `id` (`documentDiagnostic`, Rust's mapping). Where the Rust twin maps the source
 * diagnostics (the plain writers), every member but `provenance` equals the twin's. */
function expectDiagnosticsFollowSource(doc: JsonObject, rust: JsonObject, source: MechanicsResult, rustMapsSource: boolean) {
  const ours = doc.result_envelope.diagnostics as Json[], theirs = rust.result_envelope.diagnostics as Json[];
  expect(ours).toHaveLength(source.diagnostics.length);
  source.diagnostics.forEach((d, i) => {
    expect([ours[i].code, ours[i].message], `diagnostic ${i}`).toStrictEqual([d.code, d.message]);
    expect(ours[i].affected_object).toStrictEqual({ ref_type: "preview_diagnostic", ref_id: d.id });
  });
  if (rustMapsSource) {
    const withoutProvenance = (all: Json[]) => all.map(({ provenance: _provenance, ...rest }) => rest);
    expect(withoutProvenance(ours)).toStrictEqual(withoutProvenance(theirs));
  }
}
/** `{object_type: "LoadCase", ref}` (the record) to `{ref_type: "load_case", ref_id}` (the document, §6.2). */
const recordToDocumentRef = (item: ObjectRef) => { expect(item.object_type).toBe("LoadCase"); return { ref_type: "load_case", ref_id: item.ref }; };

/** §6.2: solve, record, manifest and export; the source-derived members equal the twins'. */
async function oracle(input: Input) {
  const { document: rust, packet: python, record: pythonRecord, raw } = twins(input);
  const { model, result, inputManifest, analysisRun } = await solve(input.rawPath, input.requestPath, input.mode);
  expect(hasNativeMechanicsInvocation(result, model, input.mode)).toBe(true);
  expect(numericalResultStanding(result, model)).toMatchObject({ contract: input.route, eligible: true, findings: [] });
  expect(result).toStrictEqual(raw);
  const receipt = input.route === "pressure_source" ? ["source_block_recovery"] : [];

  const doc = await buildCurrentResultExport({ model, result, analysisRun, inputManifest });
  await expect(validateResultDocument(doc, result)).resolves.toBeUndefined();
  for (const member of [...DOCUMENT_SOURCE_MEMBERS, ...receipt]) expect(doc.result_envelope[member], member).toStrictEqual(rust.result_envelope[member]);
  expect(Object.hasOwn(doc.result_envelope, "source_block_recovery")).toBe(input.route === "pressure_source");
  expect(documentRows(doc, "values"), "result_sets[].values").toStrictEqual(documentRows(rust, "values"));
  for (const member of DOCUMENT_ROW_ARRAYS) expect(documentRows(doc, member), member).toStrictEqual(documentRows(rust, member));
  expect(doc.result_envelope.unit_witness_policy).toBe(rust.result_envelope.unit_witness_policy);
  expectDiagnosticsFollowSource(doc, rust, result, PLAIN.includes(input));
  // Every member of either envelope is named: compared above, or run-specific.
  const named = new Set<string>([...DOCUMENT_SOURCE_MEMBERS, ...receipt, ...DOCUMENT_ROW_ARRAYS, ...DOCUMENT_RUN_SPECIFIC, "result_sets", "schema_version", "unit_witness_policy"]);
  expect(Object.keys(doc.result_envelope).filter(key => !named.has(key))).toStrictEqual([]);
  expect(Object.keys(rust.result_envelope).filter(key => !named.has(key))).toStrictEqual([]);

  // load_basis_refs against its own basis (§6.2), not the Rust document's IDs: the desktop
  // record's equal Python's committed record's, and the desktop document's are the record's
  // mapped LoadCase/ref to load_case/ref_id (and Python's record's, mapped the same way).
  expect(analysisRun.analysis_run.load_basis_refs).toStrictEqual(pythonRecord.analysis_run.load_basis_refs);
  expect(analysisRun.analysis_run.load_basis_refs.map(r => r.ref)).toStrictEqual(model.load_cases.map(c => c.id));
  expect(doc.result_envelope.load_basis_refs).toStrictEqual(analysisRun.analysis_run.load_basis_refs.map(recordToDocumentRef));
  expect(doc.result_envelope.load_basis_refs).toStrictEqual(pythonRecord.analysis_run.load_basis_refs.map(recordToDocumentRef));
  // The Rust twins use the same ref_type (their IDs are not compared: the load/reference
  // writer's are its template's `LC-TP-PHYS-014`).
  expect((rust.result_envelope.load_basis_refs as { ref_type: string }[]).map(r => r.ref_type)).toStrictEqual((rust.result_envelope.load_basis_refs as unknown[]).map(() => "load_case"));

  const packet = await buildStressNeutralExportPacket({ model, result, analysisRun });
  await expect(validateStressNeutralExportPacket(packet, result, analysisRun, modelLoadBasisRefs(model))).resolves.toBeUndefined();
  for (const member of [...PACKAGE_SOURCE_MEMBERS, ...receipt].filter(m => m !== "csv_text")) expect(packageMember(packet, member), member).toStrictEqual(packageMember(python, member));
  expect(csvRecords(packet.csv_text), "csv_text (values as binary64)").toStrictEqual(csvRecords(python.csv_text));
  if (CSV_BYTE_EQUAL.includes(input.twin)) expect(packet.csv_text, "csv_text bytes").toBe(python.csv_text);
  expect(Object.hasOwn(packet, "source_block_recovery")).toBe(input.route === "pressure_source");
  // The desktop record is the Python record's equal on the members the package binds.
  expect(analysisRun.analysis_run.reproducibility.semantic_contract).toStrictEqual(pythonRecord.analysis_run.reproducibility.semantic_contract);
  expect(analysisRun.analysis_run.result_refs.map(r => r.semantic_contract)).toStrictEqual(pythonRecord.analysis_run.result_refs.map(r => r.semantic_contract));
  return { doc, packet, analysisRun, model, result };
}

/** §6.2 ("the Python validator test reads the desktop-shaped members through its own
 * checks"): the desktop-built package and the 0.3 record it binds, committed for the plain
 * inputs as `apps/desktop/src/test-support/desktop-shaped/desktop_shaped_<twin>.{stress_neutral,analysis_run}.json`
 * and read by `tests/test_stress_neutral_physics_source.py`. They live outside `fixtures/`
 * because JavaScript writes a float zero as `0`, so a transported receipt is value- and
 * hash-equal to its original but not equal as a serde_json value, which the publication
 * corpus (`core/reporting/result_export/tests/hash_corpus.rs`) requires of every file under
 * `fixtures/`. The bytes are the desktop's
 * JSON values in the repository's carrier encoding (compact `JSON.stringify` plus a
 * newline; the download link indents the same values, `jsonDataHref`). Written only with
 * DESKTOP_STRESS_NEUTRAL_WRITE_FIXTURES=1; otherwise byte-checked. */
const DESKTOP_SHAPED_WRITE = process.env.DESKTOP_STRESS_NEUTRAL_WRITE_FIXTURES === "1";
const desktopShaped = (input: Input, member: "stress_neutral" | "analysis_run") => resolve(root, input.twin.replace("fixtures/results/", "apps/desktop/src/test-support/desktop-shaped/desktop_shaped_") + `.${member}.json`);
const desktopText = (value: unknown) => `${JSON.stringify(value)}\n`;

/* -------------------------------------------------------------------------- *
 * §6.2 validator acceptance: every committed twin, both modes
 * -------------------------------------------------------------------------- */
describe.each([["pressure-2 load/reference-state", ORDINARY], ["pressure-source-1 load/reference-state", JOINED], ["plain", PLAIN]] as const)("%s: the desktop validators accept the committed twins with their source (§6.2)", (_, family) => {
  it.each(family)("$label: Rust's document, and Python's package with its record", async (input) => {
    const { raw, model, document, packet, record } = twins(input);
    expect(document.result_envelope.semantic_contract_ref.ref_id).toBe(raw.producer!.semantic_contract_id);
    await expect(validateResultDocument(document, raw)).resolves.toBeUndefined();
    await expect(validateStressNeutralExportPacket(packet, raw, record, modelLoadBasisRefs(model))).resolves.toBeUndefined();
    // Both twins carry the source's evidence whole, and pressure-source-1's receipt only.
    expect(document.result_envelope.contract_evidence).toStrictEqual(raw.contract_evidence);
    expect(packet.contract_evidence).toStrictEqual(raw.contract_evidence);
    if (input.route === "pressure_source") {
      expect(document.result_envelope.source_block_recovery).toStrictEqual(raw.source_block_recovery);
      expect(packet.source_block_recovery).toStrictEqual(raw.source_block_recovery);
    } else {
      expect(Object.hasOwn(document.result_envelope, "source_block_recovery")).toBe(false);
      expect(Object.hasOwn(packet, "source_block_recovery")).toBe(false);
    }
  }, 120_000);
});

/* -------------------------------------------------------------------------- *
 * §6.2 the admitted-case oracles, and §5.4 the panels
 * -------------------------------------------------------------------------- */
describe("pressure-2 load/reference-state results: the Current builders equal the committed twins (§6.2)", () => {
  it.each(ORDINARY)("$label: the document and the package equal the twins' source-derived members", async (input) => {
    const { doc } = await oracle(input);
    expect(doc.result_envelope.contract_evidence.load_reference_states.length).toBeGreaterThan(0);
    // The Rust twin's load_basis_refs is its template's, not source-derived.
    expect(twins(input).document.result_envelope.load_basis_refs).toStrictEqual([{ ref_type: "load_case", ref_id: "LC-TP-PHYS-014" }]);
  }, 120_000);

  it.each(ORDINARY)("$label: each panel shows its link and its disclosure line, and a model edit voids both (§5.4, §6.1)", async (input) => {
    const { record: pythonRecord } = twins(input);
    const props = await solve(input.rawPath, input.requestPath, input.mode);
    const exportLine = disclosed(N_LOAD_REFERENCE_EXPORT, props.result), packageLine = disclosed(N_LOAD_REFERENCE_PACKAGE, props.result);
    // The line names the published load cases: the Python record's, here.
    for (const ref of pythonRecord.analysis_run.load_basis_refs) expect(exportLine).toContain(ref.ref);
    const resultView = render(<ResultExportPanel model={props.model} result={props.result} analysisRun={props.analysisRun} inputManifest={props.inputManifest} />);
    await screen.findByTestId("result-export-link", undefined, { timeout: 60_000 });
    expect(screen.getByTestId("result-export-load-reference-disclosure").textContent).toBe(exportLine);
    const stressView = render(<StressNeutralExportPanel model={props.model} result={props.result} analysisRun={props.analysisRun} />);
    await screen.findByTestId("stress-neutral-export-link", undefined, { timeout: 60_000 });
    expect(screen.getByTestId("stress-neutral-load-reference-disclosure").textContent).toBe(packageLine);
    for (const edit of [editedReference, editedState]) {
      const changed = edit(props.model);
      expect(hasNativeMechanicsInvocation(props.result, changed, input.mode)).toBe(false);
      resultView.rerender(<ResultExportPanel model={changed} result={props.result} analysisRun={props.analysisRun} inputManifest={props.inputManifest} />);
      stressView.rerender(<StressNeutralExportPanel model={changed} result={props.result} analysisRun={props.analysisRun} />);
      expect(screen.queryByTestId("result-export-link")).toBeNull();
      expect(screen.queryByTestId("result-export-load-reference-disclosure")).toBeNull();
      expect(screen.queryByTestId("stress-neutral-export-link")).toBeNull();
      expect(screen.queryByTestId("stress-neutral-load-reference-disclosure")).toBeNull();
      await waitFor(() => expect(screen.getByTestId("result-export-empty").textContent).toContain("CURRENT_"));
      resultView.rerender(<ResultExportPanel model={props.model} result={props.result} analysisRun={props.analysisRun} inputManifest={props.inputManifest} />);
      stressView.rerender(<StressNeutralExportPanel model={props.model} result={props.result} analysisRun={props.analysisRun} />);
      await screen.findByTestId("result-export-link", undefined, { timeout: 60_000 });
      await screen.findByTestId("stress-neutral-export-link", undefined, { timeout: 60_000 });
    }
  }, 240_000);

  it.each(MODES)("%s: a copy, a missing manifest and a mismatched record keep their refusals", async (mode) => {
    const props = await solve(`${ORDINARY_FOLDER}/connected-${mode}.raw.json`, `${ORDINARY_FOLDER}/connected.request.json`, mode);
    // A copy of the same bytes has no live native capture: not Current.
    const copy = structuredClone(props.result);
    expect(numericalResultStanding(copy, props.model).eligible).toBe(true);
    await expect(buildCurrentResultExport({ ...props, result: copy })).rejects.toThrow("CURRENT_NATIVE_INVOCATION_REQUIRED");
    render(<StressNeutralExportPanel model={props.model} result={copy} analysisRun={props.analysisRun} />);
    expect(screen.getByTestId("stress-neutral-empty")).toBeTruthy();
    expect(screen.queryByTestId("stress-neutral-load-reference-disclosure")).toBeNull();
    cleanup();
    // A missing manifest.
    await expect(buildCurrentResultExport({ ...props, inputManifest: null })).rejects.toThrow("CURRENT_INPUT_MANIFEST_UNAVAILABLE");
    // A mismatched record and manifest: the other case's, for this result.
    const other = await solve(`${ORDINARY_FOLDER}/pressure-${mode}.raw.json`, `${ORDINARY_FOLDER}/pressure.request.json`, mode);
    await expect(buildCurrentResultExport({ ...props, analysisRun: other.analysisRun })).rejects.toThrow("CURRENT_MODEL_STATE_BINDING_MISMATCH");
    await expect(buildStressNeutralExportPacket({ model: props.model, result: props.result, analysisRun: other.analysisRun })).rejects.toThrow("SN-SOURCE-CARRIER-HASH-MISMATCH");
    await expect(buildCurrentResultExport({ ...props, inputManifest: other.inputManifest })).rejects.toThrow("CURRENT_MODEL_PAYLOAD_MISMATCH");
  }, 240_000);
});

describe("plain exact results: the Current builders equal the committed plain twins (§6.2)", () => {
  it.each(PLAIN)("$label: the document and the package equal the twins' source-derived members; the desktop-shaped package is deterministic and committed", async (input) => {
    const { doc, packet, analysisRun } = await oracle(input);
    expect(Object.hasOwn(doc.result_envelope.contract_evidence, "load_reference_states")).toBe(false);
    // Determinism for fixed inputs: a second, independent solve, record and build.
    const again = await solve(input.rawPath, input.requestPath, input.mode);
    const rebuilt = await buildStressNeutralExportPacket({ model: again.model, result: again.result, analysisRun: again.analysisRun });
    expect(desktopText(again.analysisRun)).toBe(desktopText(analysisRun));
    expect(desktopText(rebuilt)).toBe(desktopText(packet));
    for (const [member, value] of [["stress_neutral", packet], ["analysis_run", analysisRun]] as const) {
      const path = desktopShaped(input, member);
      if (DESKTOP_SHAPED_WRITE) writeFileSync(path, desktopText(value));
      expect(readFileSync(path, "utf8"), `${member} bytes`).toBe(desktopText(value));
    }
    // The committed bytes read back as a package the desktop validator accepts.
    await expect(validateStressNeutralExportPacket(JSON.parse(readFileSync(desktopShaped(input, "stress_neutral"), "utf8")), again.result, JSON.parse(readFileSync(desktopShaped(input, "analysis_run"), "utf8")), modelLoadBasisRefs(again.model))).resolves.toBeUndefined();
  }, 180_000);

  it.each(PLAIN)("$label: each panel shows its link and no disclosure line, and a model edit voids the link", async (input) => {
    const props = await solve(input.rawPath, input.requestPath, input.mode);
    const resultView = render(<ResultExportPanel model={props.model} result={props.result} analysisRun={props.analysisRun} inputManifest={props.inputManifest} />);
    const stressView = render(<StressNeutralExportPanel model={props.model} result={props.result} analysisRun={props.analysisRun} />);
    await screen.findByTestId("result-export-link", undefined, { timeout: 60_000 });
    await screen.findByTestId("stress-neutral-export-link", undefined, { timeout: 60_000 });
    expect(screen.queryByTestId("result-export-load-reference-disclosure")).toBeNull();
    expect(screen.queryByTestId("stress-neutral-load-reference-disclosure")).toBeNull();
    const changed = editedNode(props.model);
    resultView.rerender(<ResultExportPanel model={changed} result={props.result} analysisRun={props.analysisRun} inputManifest={props.inputManifest} />);
    stressView.rerender(<StressNeutralExportPanel model={changed} result={props.result} analysisRun={props.analysisRun} />);
    expect(screen.queryByTestId("result-export-link")).toBeNull();
    expect(screen.queryByTestId("stress-neutral-export-link")).toBeNull();
  }, 240_000);
});

/** Design §10: a preview-physics-2 document keeps its bytes, so its `load_basis_refs`
 * keep the record's `object_type` as `ref_type` (`currentResultDocumentBase`); only the
 * exact routes name them `load_case`, as Rust's derivative does (checked in the oracles). */
describe("a preview-physics-2 document's load_basis_refs keep the record's object_type", () => {
  it("preview_physics_connected_sparse: ref_type is the record's object_type, ref_id its ref", async () => {
    const raw = json("fixtures/results/preview_physics_connected_sparse.json") as MechanicsResult;
    const model = json("fixtures/model_operations/precision_connected_ui_model.json") as PreviewModel;
    const manifest = { manifest_ref: { object_type: "InputManifest", ref: "manifest:invented-preview-shape" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: raw.model_ref, model_payload: model }, solver_basis: { solver_name: raw.producer!.component_name, solver_version: raw.producer!.component_version, solver_build_ref: "test:shape", solver_mode: "sparse_interactive" } } };
    const record = await buildAnalysisRunV03(raw, manifest as Json, undefined, modelLoadBasisRefs(model));
    expect(record.analysis_run.load_basis_refs.length).toBeGreaterThan(0);
    const base = currentResultDocumentBase(model, raw, record.analysis_run, "test:shape", { solver_name: raw.producer!.component_name, solver_version: raw.producer!.component_version, solver_build_ref: "test:shape" });
    expect(base.result_envelope.load_basis_refs).toStrictEqual(record.analysis_run.load_basis_refs.map(r => ({ ref_type: r.object_type, ref_id: r.ref })));
  });
});

describe("a source diagnostic without an id is refused by name", () => {
  it("currentResultDocumentBase refuses it; the document never drops the diagnostic's identity", async () => {
    const raw = json("fixtures/results/preview_physics_connected_sparse.json") as MechanicsResult;
    const model = json("fixtures/model_operations/precision_connected_ui_model.json") as PreviewModel;
    expect(raw.diagnostics.length).toBeGreaterThan(0);
    const run = { run_id: raw.run_id, load_basis_refs: [], hashes: [], analysis_status: [], professional_boundary: {} } as unknown as Parameters<typeof currentResultDocumentBase>[2];
    const solver = { solver_name: raw.producer!.component_name, solver_version: raw.producer!.component_version, solver_build_ref: "test:id" };
    expect(currentResultDocumentBase(model, raw, run, "test:id", solver).result_envelope.diagnostics[0].affected_object).toStrictEqual({ ref_type: "preview_diagnostic", ref_id: raw.diagnostics[0].id });
    const missing = structuredClone(raw); delete missing.diagnostics[0].id;
    expect(() => currentResultDocumentBase(model, missing, run, "test:id", solver)).toThrow(`RESULT_EXPORT_DIAGNOSTIC_ID_MISSING: ${raw.diagnostics[0].code}`);
  });
});

/** The stress-neutral CSV value text (pre-existing desktop writer; an observation for
 * DEL-17-06). The desktop writes JavaScript's shortest round-trip number text where
 * Python writes its `repr`; for the plain pressure-source-1 package the parsed binary64
 * values are equal row by row, and the text differs only in the value column of the rows
 * named here. */
const CSV_NUMBER_TEXT_ROWS = [
  "result:disp:independent-root:rx",
  "result:moment:independent-member:midspan:torsion",
  "result:moment:independent-member:quarter-1:torsion",
  "result:moment:independent-member:quarter-3:torsion",
  "result:moment:independent-member:torsion",
  "result:moment:independent-member:torsion:end-j",
  "result:support-action:4:case:18:independent-spring:Mx",
  "result:support-action:4:case:18:independent-spring:moment_magnitude",
];
describe("the stress-neutral CSV value text", () => {
  it.each(PLAIN.filter(input => input.route === "pressure_source"))("$label: equal binary64 values row by row; the text differs only in number formatting", async (input) => {
    const { packet } = await oracle(input);
    const python = twins(input).packet.csv_text as string;
    const ours = (packet.csv_text as string).split("\n"), theirs = python.split("\n");
    expect(ours).toHaveLength(theirs.length);
    const differing: string[] = [];
    for (const [i, line] of ours.entries()) {
      const a = line.split(","), b = theirs[i].split(",");
      expect(a).toHaveLength(b.length);
      for (const [column, field] of a.entries()) {
        if (field === b[column]) continue;
        // Only the value column, and only its text: the same binary64 value.
        expect(i).toBeGreaterThan(0);
        expect(column).toBe(7);
        expect(Object.is(Number(field), Number(b[column]))).toBe(true);
        differing.push(a[0]);
      }
    }
    expect(differing).toStrictEqual(CSV_NUMBER_TEXT_ROWS);
  }, 120_000);
});

/* -------------------------------------------------------------------------- *
 * §6.2 the pressure-source-1 twins stay refusal fixtures at the builders; the fallback
 * -------------------------------------------------------------------------- */
describe("pressure-source-1 load/reference-state results are refused by each builder's standing (§5.4, §6.2)", () => {
  it.each(JOINED)("$label: registered with its invocation, Rust's T1 finding; each builder refuses; the panels offer nothing", async (input) => {
    const captured = Object.hasOwn(json(input.requestPath), "materials");
    const { model, result, inputManifest, analysisRun } = captured ? await solve(input.rawPath, input.requestPath, input.mode) : await registeredStandIn(input);
    expect(hasNativeMechanicsInvocation(result, model, input.mode)).toBe(captured);
    const standing = numericalResultStanding(result, model);
    expect(standing).toMatchObject({ contract: "pressure_source", status: "needs_recompute", eligible: false });
    expect(standing.findings).toContain(LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE);
    await expect(buildCurrentResultExport({ model, result, analysisRun, inputManifest })).rejects.toThrow("CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE");
    await expect(buildStressNeutralExportPacket({ model, result, analysisRun })).rejects.toThrow("SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE");
    // Without its registered invocation (a copy), the standing names the missing registration.
    const copy = structuredClone(result);
    expect(numericalResultStanding(copy, model)).toMatchObject({ eligible: false, findings: [SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED] });
    await expect(buildCurrentResultExport({ model, result: copy, analysisRun, inputManifest })).rejects.toThrow("CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE");
    await expect(buildStressNeutralExportPacket({ model, result: copy, analysisRun })).rejects.toThrow("SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE");
    render(<ResultExportPanel model={model} result={result} analysisRun={analysisRun} inputManifest={inputManifest} />);
    render(<StressNeutralExportPanel model={model} result={result} analysisRun={analysisRun} />);
    await waitFor(() => expect(screen.getByTestId("result-export-empty").textContent).toContain("CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE"));
    expect(screen.queryByTestId("result-export-link")).toBeNull();
    expect(screen.queryByTestId("stress-neutral-export-link")).toBeNull();
    expect(screen.queryByTestId("result-export-load-reference-disclosure")).toBeNull();
    expect(screen.queryByTestId("stress-neutral-load-reference-disclosure")).toBeNull();
  }, 120_000);

  it.each(MODES)("%s: the pressure-2 fallback (load_reference_fallback_uz) is refused by each builder's standing", async (mode) => {
    const { model, result, inputManifest, analysisRun } = await solve(`${FALLBACK}-${mode}.raw.json`, `${FALLBACK}.request.json`, mode);
    expect(hasNativeMechanicsInvocation(result, model, mode)).toBe(true);
    const standing = numericalResultStanding(result, model);
    expect(standing).toMatchObject({ contract: "pressure", eligible: false });
    expect(standing.findings).toContain("NUMERICAL_INTEGRITY_NOT_QUALIFIED");
    await expect(buildCurrentResultExport({ model, result, analysisRun, inputManifest })).rejects.toThrow("CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE");
    await expect(buildStressNeutralExportPacket({ model, result, analysisRun })).rejects.toThrow("SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE");
    render(<StressNeutralExportPanel model={model} result={result} analysisRun={analysisRun} />);
    expect(screen.queryByTestId("stress-neutral-export-link")).toBeNull();
    expect(screen.queryByTestId("stress-neutral-load-reference-disclosure")).toBeNull();
  }, 120_000);
});
