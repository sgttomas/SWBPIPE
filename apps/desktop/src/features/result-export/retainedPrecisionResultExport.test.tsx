/** T6S-4 (I74 PLAN 1.2; RR decisions 4, 6 and 11; D2 4.9.7, 4.9.9; D-U6-2): the local
 * result JSON of the F2a preview successor
 * `openpipestress.result_semantics/0.3.0/preview-physics-retained-1`.
 *
 * - Byte parity with I76's Rust goldens (`derivative::derive_document`) for both pinned
 *   successors, from the same fixed desktop-shaped base and origin: the builder's own base
 *   and origin functions, with I76's fixed `test:` stand-ins and test-labelled origin limit,
 *   whose canonical bytes are pinned to I76's recorded inputs.
 * - The Current builder and panel, through mocked IPC and TEST-BUILT, hash-consistent
 *   input-manifest evidence (CQ-11): the pinned request model is not desktop-complete, so
 *   the product's manifest builder refuses it; the product flow is B8's native witness.
 * - The negative and multi-case controls. Unit tests over mocked IPC only: NOT a native
 *   witness, and no producer-origin claim (D-U7-6). */
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
vi.mock("./nativeResultSave", () => ({ isNativeResultSaveRuntime: vi.fn(() => false), saveNativeResultJson: vi.fn() }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { canonicalJsonString, canonicalSha256Hex, checkedJsonText } from "../../services/hashService";
import { buildAnalysisRunPreview, runPreviewMechanics, type PreviewSolverMode } from "../../services/previewService";
import { buildCurrentSessionInputManifest, type CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { numericalResultStanding, RETAINED_PRECISION_DOWNGRADE_FORBIDDEN } from "../results/numericalResultQuality";
import { RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED, RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE, RETAINED_PRECISION_VALIDATION_REQUIRED, registerRetainedPrecision } from "../results/retainedPrecisionStanding";
import { validateRetainedPrecision } from "../results/retainedPrecision";
import { LOAD_REFERENCE_OUTPUT_REFUSAL, N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE, surfaceOutputRefusal } from "../results/outputPolicy";
import { RETAINED_ABSOLUTE_VERIFIED, RETAINED_NOT_COVERED, retainedClassDisclosure } from "../results/retainedPrecisionDisclosure";
import {
  CURRENT_ORIGIN_LIMIT, buildCurrentResultExport, currentReceivedOrigin, currentResultDocumentBase, deriveResultDocument, validateResultDocument, type JsonObject,
} from "./resultExportAdapter";
import { ResultExportPanel } from "./ResultExportPanel";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
afterEach(() => { cleanup(); invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; });
const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const sha256 = (bytes: string | Buffer) => createHash("sha256").update(bytes).digest("hex");
const caseFile = json("fixtures/results/retained_precision_carrier_cases.json");
const corpus = json("fixtures/results/retained_precision_cases.json");
const MODES = ["sparse_interactive", "dense_scrutiny"] as const;

/** I76's goldens (committed at 055ee0c0bc) and its recorded inputs (R/I76/t6s_01/inputs/). */
const GOLDEN: Record<PreviewSolverMode, { path: string; sha256: string; base: string; origin: string; rows: number; nonQuantity: number }> = {
  sparse_interactive: { path: "fixtures/results/retained_precision_successor_derivative_sparse_interactive.json", sha256: "958df02e276538a96c4732302a46cb36a53c2298ca7e1f4deb88a3748c67c661", base: "5a48e00d7976ed7e2e34967d023219a7e5e23144b35c680415d1b3786bfc44d8", origin: "1a0f642ce9c9476924e6f8841ccadd3109c9f1f761fe404d7adb5c84759894cc", rows: 98, nonQuantity: 1 },
  dense_scrutiny: { path: "fixtures/results/retained_precision_successor_derivative_dense_scrutiny.json", sha256: "3f9905ad4c4bba688687714751abe965d18cea834701c32c379d81f573dcf682", base: "3f054be38290bf9ae6e6a236c192d6eda8c5a0867367d8a0559509fe6f515e78", origin: "04ee7eb957b9bdc7b617e588393a01107c7a1aa0a064087e45a58c481a682231", rows: 99, nonQuantity: 2 },
};
/** I76's fixed stand-ins (its CHECKPOINT_1 section 2): test values, not a qualified manifest or build. */
const MANIFEST_REF = "test:t6s-golden-reference-only-manifest";
const SOLVER_BUILD_REF = "test:t6s-golden-pinned-producer-test-bytes-not-native-attestation";
const ORIGIN_LIMIT = "Test-built desktop-shaped origin (T6S golden): pinned producer test bytes, not a qualified Current received carrier; independent authentic original producer bytes unavailable; dimension absence is not producer attestation";
const RUN_STAND_IN = {
  analysis_status: ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE"],
  professional_boundary: { human_review_required: true, software_makes_compliance_claim: false, software_makes_certification_claim: false, software_makes_sealing_claim: false, software_makes_approval_claim: false, software_makes_authentication_claim: false },
  hashes: [],
};

function milestone(mode: PreviewSolverMode) {
  const entry = caseFile.fixtures[`milestone_${mode}`];
  const bytes = readFileSync(resolve(root, entry.path));
  expect(sha256(bytes)).toBe(entry.sha256);
  const doc = JSON.parse(bytes.toString("utf8"));
  return { source: doc.source as MechanicsResult, model: doc.invocation.request.model as PreviewModel };
}
/** The fixed desktop-shaped base and origin: the Current builder's own functions, fed
 * I76's stand-ins in place of the AnalysisRun and manifest values (no record is built). */
async function goldenInputs(source: MechanicsResult, model: PreviewModel) {
  const run = { ...structuredClone(RUN_STAND_IN), run_id: source.run_id, load_basis_refs: modelLoadBasisRefs(model) } as Json;
  const base = currentResultDocumentBase(model, source, run, MANIFEST_REF, { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: SOLVER_BUILD_REF });
  const origin = await currentReceivedOrigin(model, source, MANIFEST_REF, false, ORIGIN_LIMIT);
  return { base, origin };
}
/** Every object key in a JSON value. */
function keysOf(value: unknown, out = new Set<string>()): Set<string> {
  if (Array.isArray(value)) for (const item of value) keysOf(item, out);
  else if (value && typeof value === "object") for (const [key, item] of Object.entries(value)) { out.add(key); keysOf(item, out); }
  return out;
}
async function deliver(source: MechanicsResult, model: PreviewModel, mode: PreviewSolverMode): Promise<MechanicsResult> {
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async () => structuredClone(source));
  return runPreviewMechanics(model, mode);
}
/** TEST-BUILT input-manifest evidence (CQ-11): the manifest builder's shape and its exact
 * hash and ref rules, around the pinned request model, which the product's builder refuses
 * (no desktop load-case status). Hash-consistent only; it qualifies no desktop session. */
async function testBuiltManifest(model: PreviewModel, source: MechanicsResult, mode: PreviewSolverMode): Promise<CurrentSessionInputManifestEvidence> {
  const manifest = {
    schema_version: "1.0.0", document_kind: "openpipestress.current_session_input_manifest",
    model_basis: { model_ref: model.project.id, model_payload: structuredClone(model) },
    unit_basis: { project_units: Object.fromEntries(Object.entries(model.project.units ?? {}).sort(([a], [b]) => a.localeCompare(b))) },
    solver_basis: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "test:t6s-unit-transport-replay-not-native-witness", solver_mode: mode, settings: {} },
    load_basis: { load_cases: structuredClone(model.load_cases), combinations: structuredClone(model.combinations ?? []) },
    active_rule_packs: [], external_assets: [],
    identity_policy: { canonicalization: "rfc8785_jcs", hash_algorithm: "sha256", canonical_bytes_scope: "entire_input_manifest_object" },
    replay_boundary: { included_as_package_member: false, portable_replay_claimed: false, current_session_ref_hash_integrity_only: true },
  } as unknown as CurrentSessionInputManifestEvidence["manifest"];
  const digest = await canonicalSha256Hex(manifest);
  const token = model.project.id.replace(/[^A-Za-z0-9._-]+/g, "-").replace(/^-+|-+$/g, "") || "current-session";
  return { manifest, manifest_ref: { object_type: "InputManifest", ref: `input-manifest:${token}:${digest}` }, manifest_sha256: digest, canonical_bytes: await canonicalJsonString(manifest) };
}
async function eligible(mode: PreviewSolverMode) {
  const { source, model } = milestone(mode);
  const received = await deliver(source, model, mode);
  expect(numericalResultStanding(received, model)).toMatchObject({ eligible: true, findings: [] });
  const inputManifest = await testBuiltManifest(model, received, mode);
  const analysisRun = await buildAnalysisRunV03(received, inputManifest as Json, undefined, modelLoadBasisRefs(model));
  return { source, received, model, analysisRun, inputManifest };
}
const moved = (model: PreviewModel) => { const copy = structuredClone(model) as Json; copy.nodes[0].position.x += 1; return copy as PreviewModel; };
/** Recomputes the derivative hash after a deliberate edit, so that only the relational check under test can refuse. */
async function rehashed(doc: JsonObject): Promise<JsonObject> {
  const payload = structuredClone(doc); delete payload.result_envelope.reproducibility.derivative_hash;
  doc.result_envelope.reproducibility.derivative_hash.value = await canonicalSha256Hex(payload);
  return doc;
}

describe.each(MODES)("%s: byte parity with I76's Rust golden (T6S-2, T6S-4)", (mode) => {
  it("derives the golden's exact canonical bytes from the same fixed desktop-shaped base and origin", async () => {
    const { source, model } = milestone(mode);
    const golden = GOLDEN[mode];
    const goldenText = readFileSync(resolve(root, golden.path), "utf8");
    expect(sha256(goldenText)).toBe(golden.sha256);
    const { base, origin } = await goldenInputs(source, model);
    // The inputs are I76's recorded inputs, byte for byte (canonical text).
    expect(await canonicalSha256Hex(base)).toBe(golden.base);
    expect(await canonicalSha256Hex(origin)).toBe(golden.origin);
    const before = checkedJsonText({ source, model, base, origin });
    const doc = await deriveResultDocument(base, model, source, origin);
    expect(checkedJsonText({ source, model, base, origin })).toBe(before);
    expect(await canonicalJsonString(doc)).toBe(goldenText);
    expect(await canonicalSha256Hex(doc)).toBe(golden.sha256);
    // The Rust golden validates under the TypeScript validator.
    await expect(validateResultDocument(JSON.parse(goldenText), source)).resolves.toBeUndefined();
  });
  it("carries the receipt whole, the evidence, the class disclosures, and no standing or origin claim", async () => {
    const { source, model } = milestone(mode);
    const golden = GOLDEN[mode];
    const { base, origin } = await goldenInputs(source, model);
    const doc = await deriveResultDocument(base, model, source, origin);
    const e = doc.result_envelope;
    expect(doc.schema_version).toBe("0.3.0");
    expect(checkedJsonText(e.retained_precision)).toBe(checkedJsonText(source.retained_precision));
    expect(checkedJsonText(e.contract_evidence)).toBe(checkedJsonText(source.contract_evidence));
    for (const key of ["producer", "numerical_quality", "formulation_basis"] as const) expect(checkedJsonText(e[key])).toBe(checkedJsonText(source[key]));
    expect(e.semantic_contract_ref).toStrictEqual({ ref_type: "semantic_contract", ref_id: "openpipestress.result_semantics/0.3.0/preview-physics-retained-1" });
    // Classes from the reader without an invocation; each absolute_verified row disclosed, not valued.
    const classes = (await validateRetainedPrecision(source)).classifications;
    const absolute = classes.filter(c => c.class === "absolute_verified");
    expect(source.results).toHaveLength(golden.rows);
    expect([classes.filter(c => c.class === "relative_verified").length, absolute.length, classes.filter(c => c.class === "input_derived").length, classes.filter(c => c.class === "non_quantity").length, classes.filter(c => c.class === "not_covered").length]).toStrictEqual([25, 69, 3, golden.nonQuantity, 0]);
    const disclosed = e.row_disclosures.filter((d: Json) => d.reason_code === RETAINED_ABSOLUTE_VERIFIED);
    expect(disclosed.map((d: Json) => d.source_result_id).sort()).toStrictEqual(absolute.map(c => c.result_id).sort());
    for (const d of disclosed) {
      const row = source.results.find(r => r.id === d.source_result_id)!;
      expect(d.message).toBe(retainedClassDisclosure(row.kind, row.unit, absolute.find(c => c.result_id === row.id))!.message);
      expect(d.message).not.toContain("e+");
      expect([d.source_value, d.source_unit]).toStrictEqual([row.value, row.unit]);
      expect(e.result_sets[0].values.some((v: Json) => v.result_id === row.id)).toBe(false);
      expect(e.unit_preservation_witnesses.some((w: Json) => w.source_result_ref.ref_id === row.id)).toBe(false);
      expect(e.row_accounting.find((a: Json) => a.source_result_id === row.id).disposition).toBe("disclosed");
    }
    expect(e.row_disclosures.filter((d: Json) => d.reason_code === RETAINED_NOT_COVERED)).toHaveLength(0);
    expect(e.row_disclosures).toHaveLength(69 + golden.nonQuantity);
    expect(e.result_sets[0].values).toHaveLength(golden.rows - 69 - golden.nonQuantity);
    expect(disclosed.find((d: Json) => d.source_result_id === "result:disp:N0").message).toBe("displacement_magnitude: retained_precision_absolute_verified; verified only to the receipt's absolute bound b = 5.4215527659630466e-24 m (binary64 3b1a378ea78c5ce9), below the relative accuracy floor; source value/unit and annotation retained; withheld from rule binding and reliance");
    // No standing token, no invocation and no producer-origin claim (CQ-4, CQ-5; D-U7-6).
    const text = JSON.stringify(doc);
    for (const token of ["numerically_eligible", "integrity_checked", "needs_recompute"]) expect(text).not.toContain(token);
    const keys = keysOf(doc);
    for (const key of ["request", "solver_mode", "invocation_bound"]) expect(keys.has(key)).toBe(false);
    expect(e.reproducibility.request_hash).toBeNull();
    expect(e.reproducibility.raw_source_hashes).toStrictEqual([]);
    expect(e.reproducibility.source_origin_bindings).toHaveLength(1);
    expect(e.reproducibility.source_origin_bindings[0]).toMatchObject({ authentic_producer_available: false, original_producer_checksum: null, origin_limit: ORIGIN_LIMIT });
  });
});

describe.each(MODES)("%s: the Current builder and panel export an eligible successor (CQ-11: test-built manifest evidence)", (mode) => {
  it("builds and validates the document through buildCurrentResultExport", async () => {
    const { received, model, analysisRun, inputManifest } = await eligible(mode);
    const before = checkedJsonText({ received, model, analysisRun, inputManifest });
    const doc = await buildCurrentResultExport({ model, result: received, analysisRun, inputManifest });
    expect(checkedJsonText({ received, model, analysisRun, inputManifest })).toBe(before);
    await expect(validateResultDocument(doc, received)).resolves.toBeUndefined();
    const e = doc.result_envelope;
    expect(checkedJsonText(e.retained_precision)).toBe(checkedJsonText(received.retained_precision));
    expect(e.row_disclosures.filter((d: Json) => d.reason_code === RETAINED_ABSOLUTE_VERIFIED)).toHaveLength(69);
    // The desktop's own origin: never producer-attested.
    expect(e.reproducibility.source_origin_bindings[0]).toMatchObject({ origin_class: "received_current_dimension_absent", authentic_producer_available: false, original_producer_checksum: null, origin_limit: CURRENT_ORIGIN_LIMIT });
    // Only the stand-in values differ from the golden: its disclosures and values are the golden's.
    const golden = JSON.parse(readFileSync(resolve(root, GOLDEN[mode].path), "utf8"));
    expect(await canonicalJsonString(e.row_disclosures)).toBe(await canonicalJsonString(golden.result_envelope.row_disclosures));
    expect(await canonicalJsonString(e.result_sets[0].values)).toBe(await canonicalJsonString(golden.result_envelope.result_sets[0].values));
  });
  it("the panel offers the local result JSON, with its summary line", async () => {
    const { received, model, analysisRun, inputManifest } = await eligible(mode);
    render(<ResultExportPanel model={model} result={received} analysisRun={analysisRun} inputManifest={inputManifest} />);
    expect((await screen.findByTestId("result-export-summary", undefined, { timeout: 10_000 })).textContent).toContain(`available; rows=${GOLDEN[mode].rows - 69 - GOLDEN[mode].nonQuantity}`);
    expect(screen.getByTestId("result-export-retained-precision-summary").textContent).toBe("Retained precision, per case: case: 69 verified only to an absolute bound; 0 uncovered.");
    cleanup();
    // A moved model is not the live capture: the panel refuses with the standing's code.
    render(<ResultExportPanel model={moved(model)} result={received} analysisRun={analysisRun} inputManifest={inputManifest} />);
    expect(await screen.findByText(`${RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}`, { exact: false })).toBeTruthy();
    expect(screen.queryByTestId("result-export-summary")).toBeNull();
    expect(document.querySelectorAll("a[download]")).toHaveLength(0);
  });
});

describe.each(MODES)("%s: the negative controls refuse with their expected codes", (mode) => {
  it("copied, moved and unregistered successors are refused by the Current builder", async () => {
    const { source, received, model, analysisRun, inputManifest } = await eligible(mode);
    const build = (result: MechanicsResult, m: PreviewModel = model) => buildCurrentResultExport({ model: m, result, analysisRun, inputManifest });
    const refusal = (code: string) => `${code}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}`;
    await expect(build(structuredClone(received))).rejects.toThrow(refusal(RETAINED_PRECISION_VALIDATION_REQUIRED));
    await expect(build(received, moved(model))).rejects.toThrow(refusal(RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED));
    await expect(build(structuredClone(source))).rejects.toThrow(refusal(RETAINED_PRECISION_VALIDATION_REQUIRED));
  });
  it("a relabelled statement, a receipt on another identity, a forged class code and an edited message are refused", async () => {
    const { source, model } = milestone(mode);
    const { base, origin } = await goldenInputs(source, model);
    const doc = await deriveResultDocument(base, model, source, origin);
    // A relabelled statement reads unsupported.
    const relabelled = structuredClone(source) as Json;
    relabelled.producer.semantic_contract_id = "openpipestress.result_semantics/0.3.0/preview-physics-1";
    relabelled.formulation_basis.profile_id = "product_preview_mechanics_v1";
    await expect(deriveResultDocument(base, model, relabelled, await currentReceivedOrigin(model, relabelled, MANIFEST_REF, false, ORIGIN_LIMIT))).rejects.toThrow("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
    await expect(validateResultDocument(doc, relabelled)).rejects.toThrow("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
    // A statement the reader refuses has no validated classes, so it derives nothing.
    const unread = structuredClone(source) as Json; unread.retained_precision.receipt_sha256 = "0".repeat(64);
    await expect(deriveResultDocument(base, model, unread, await currentReceivedOrigin(model, unread, MANIFEST_REF, false, ORIGIN_LIMIT))).rejects.toThrow("RETAINED_PRECISION_RECEIPT_MISMATCH");
    await expect(validateResultDocument(doc, unread)).rejects.toThrow("RETAINED_PRECISION_RECEIPT_MISMATCH");
    // The receipt must bind the source, whole.
    const edit = async (change: (d: Json) => void) => { const d = structuredClone(doc); change(d); return rehashed(d); };
    await expect(validateResultDocument(await edit(d => { d.result_envelope.retained_precision.receipt_sha256 = "0".repeat(64); }), source)).rejects.toThrow("RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH");
    await expect(validateResultDocument(await edit(d => { delete d.result_envelope.retained_precision; }), source)).rejects.toThrow("RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH");
    await expect(validateResultDocument(await edit(d => { delete d.result_envelope.contract_evidence; }), source)).rejects.toThrow("SOURCE_PHYSICAL_METADATA_MISMATCH");
    // Exact class-code consistency.
    const firstAbsolute = (d: Json) => d.result_envelope.row_disclosures.find((x: Json) => x.reason_code === RETAINED_ABSOLUTE_VERIFIED);
    await expect(validateResultDocument(await edit(d => { firstAbsolute(d).reason_code = RETAINED_NOT_COVERED; }), source)).rejects.toThrow("DISCLOSURE_SEMANTICS");
    await expect(validateResultDocument(await edit(d => { firstAbsolute(d).message = firstAbsolute(d).message.replace("withheld from rule binding and reliance", "bindable"); }), source)).rejects.toThrow("DISCLOSURE_SEMANTICS");
    await expect(validateResultDocument(await edit(d => { firstAbsolute(d).message = firstAbsolute(d).message.replace("e-24", "e+24"); }), source)).rejects.toThrow("DISCLOSURE_SEMANTICS");
    await expect(validateResultDocument(await edit(d => { d.result_envelope.row_disclosures.find((x: Json) => x.reason_code !== RETAINED_ABSOLUTE_VERIFIED).reason_code = RETAINED_ABSOLUTE_VERIFIED; }), source)).rejects.toThrow("DISCLOSURE_SEMANTICS");
    // A classed row claimed as a value.
    await expect(validateResultDocument(await edit(d => { d.result_envelope.row_accounting.find((a: Json) => a.source_result_id === "result:disp:N0").disposition = "exported_quantity"; }), source)).rejects.toThrow("SOURCE_ACCOUNTING_IDENTITY");
    // The derivative hash still binds an unresealed edit.
    const unsealed = structuredClone(doc); firstAbsolute(unsealed).source_unit = "m";
    await expect(validateResultDocument(unsealed, source)).rejects.toThrow("SOURCE_TARGET_VALUE");
  });
  it("a receipt on another identity's document is refused (RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)", async () => {
    const { model } = nativeMechanicsReplayPair(mode, { profile: "preview" });
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(createNativeMechanicsReplay({ profile: "preview" }).invoke);
    const result = await runPreviewMechanics(model, mode);
    const inputManifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "test:t6s-replay", solver_mode: mode, settings: {} }, active_rule_packs: [], external_assets: [] });
    const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
    const doc = await buildCurrentResultExport({ model, result, analysisRun, inputManifest });
    await expect(validateResultDocument(doc, result)).resolves.toBeUndefined();
    const receipt = milestone(mode).source.retained_precision;
    const carried = structuredClone(doc); carried.result_envelope.retained_precision = structuredClone(receipt);
    await expect(validateResultDocument(await rehashed(carried), result)).rejects.toThrow(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);
    const base = currentResultDocumentBase(model, result, analysisRun.analysis_run, inputManifest.manifest_ref.ref, inputManifest.manifest.solver_basis);
    base.result_envelope.retained_precision = structuredClone(receipt);
    await expect(deriveResultDocument(base, model, result, await currentReceivedOrigin(model, result, inputManifest.manifest_ref.ref, false, CURRENT_ORIGIN_LIMIT))).rejects.toThrow(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);
  });
});

describe("a load/reference-state result keeps T1's refusal", () => {
  it("in the Current builder, the derivative and the validator", async () => {
    const result = json("fixtures/product_preview/load_reference/pressure-sparse_interactive.raw.json") as MechanicsResult;
    const model = structuredClone(json("fixtures/product_preview/load_reference/pressure.request.json").model) as PreviewModel;
    expect(surfaceOutputRefusal(result, model, "result-export")).toBe(LOAD_REFERENCE_OUTPUT_REFUSAL);
    await expect(buildCurrentResultExport({ model, result, analysisRun: {} as Json, inputManifest: null })).rejects.toThrow(LOAD_REFERENCE_OUTPUT_REFUSAL);
    const origin = await currentReceivedOrigin(model, result, MANIFEST_REF, false, ORIGIN_LIMIT);
    await expect(deriveResultDocument({ result_envelope: {} }, model, result, origin)).rejects.toThrow(LOAD_REFERENCE_OUTPUT_REFUSAL);
    await expect(validateResultDocument({ schema_version: "0.3.0", result_envelope: { schema_version: "0.3.0" } }, result)).rejects.toThrow(LOAD_REFERENCE_OUTPUT_REFUSAL);
  });
});

describe("the corpus's synthetic two-case bases, read by id", () => {
  /** Registered with its own invocation and a TEST STAND-IN for the live native capture
   * (D-U7-4; checkpoint reading R-5): the corpus invocation has no `materials` member, so it
   * cannot pass the product's IPC capture. The stand-in holds for this exact model only. */
  async function corpusBase(id: string) {
    const entry = corpus.cases.find((c: Json) => c.id === id);
    expect(entry?.id).toBe(id);
    const source = structuredClone(entry.source) as MechanicsResult, invocation = structuredClone(entry.invocation), model = structuredClone(invocation.request.model) as PreviewModel;
    const captured = checkedJsonText(model);
    await registerRetainedPrecision(source, invocation, (m) => { try { return checkedJsonText(m) === captured; } catch { return false; } });
    return { source, model, mode: invocation.solver_mode as PreviewSolverMode };
  }
  it("admits two_case_synthetic when eligible, and its derivative discloses every class row of both cases", async () => {
    const { source, model } = await corpusBase("two_case_synthetic");
    expect(numericalResultStanding(source, model)).toMatchObject({ eligible: true, findings: [] });
    expect(surfaceOutputRefusal(source, model, "result-export")).toBeNull();
    const { base, origin } = await goldenInputs(source, model);
    const doc = await deriveResultDocument(base, model, source, origin);
    await expect(validateResultDocument(doc, source)).resolves.toBeUndefined();
    const absolute = (await validateRetainedPrecision(source)).classifications.filter(c => c.class === "absolute_verified");
    expect(absolute).toHaveLength(68);
    expect(new Set(absolute.map(c => c.basis_ref.ref_id))).toStrictEqual(new Set(["case:six-component-load", "case:zero-load"]));
    expect(doc.result_envelope.row_disclosures.filter((d: Json) => d.reason_code === RETAINED_ABSOLUTE_VERIFIED).map((d: Json) => d.source_result_id).sort()).toStrictEqual(absolute.map(c => c.result_id).sort());
    expect(checkedJsonText(doc.result_envelope.retained_precision)).toBe(checkedJsonText(source.retained_precision));
  });
  it("refuses two_case_facade_after_certificate_synthetic, which is not numerically eligible", async () => {
    const { source, model } = await corpusBase("two_case_facade_after_certificate_synthetic");
    expect(numericalResultStanding(source, model)).toMatchObject({ eligible: false, findings: [RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE] });
    const refusal = `${RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}`;
    expect(surfaceOutputRefusal(source, model, "result-export")).toBe(refusal);
    await expect(buildCurrentResultExport({ model, result: source, analysisRun: {} as Json, inputManifest: null })).rejects.toThrow(refusal);
  });
});

/** B3b-T (I101; I93 PLAN decision 21): byte parity with the exact successor's Rust goldens
 * (`retained_precision_derivative_golden.rs`, `B3B_EXACT`), derived from lane P's pinned m3x
 * successors (PP `EXACT_PINNED`) with the same fixed desktop-shaped base and origin. The
 * Current builder is not exercised: physics-retained-1 is outside the static fresh set. */
const EXACT_GOLDEN: Record<PreviewSolverMode, { successor: string; successorSha256: string; path: string; sha256: string; rows: number }> = {
  sparse_interactive: { successor: "fixtures/results/retained_precision_exact_successor_sparse_interactive.json", successorSha256: "02465c6c92ac2e4360a77910cb54803590b5a11042dfddb223bf78f9e856e5d6", path: "fixtures/results/retained_precision_exact_successor_derivative_sparse_interactive.json", sha256: "79d9930541303b003f215aee6503aa1cec73b45eeb351998a77856257c171ff8", rows: 98 },
  dense_scrutiny: { successor: "fixtures/results/retained_precision_exact_successor_dense_scrutiny.json", successorSha256: "31f10f04f6f335dfb1a7e5f904198972903bfc9208660031bbfaa5c547d347cc", path: "fixtures/results/retained_precision_exact_successor_derivative_dense_scrutiny.json", sha256: "32b4182c8d524dc0f03c2237a80fc41ca2e72e1a895e46d6f38ddc66eba1a891", rows: 99 },
};
describe.each(MODES)("%s: byte parity with the exact successor's Rust golden (B3b-T)", (mode) => {
  it("derives the golden's exact canonical bytes; it validates and carries the receipt and physics-1's evidence", async () => {
    const golden = EXACT_GOLDEN[mode];
    const bytes = readFileSync(resolve(root, golden.successor));
    expect(sha256(bytes)).toBe(golden.successorSha256);
    const pinned = JSON.parse(bytes.toString("utf8"));
    const source = pinned.source as MechanicsResult, model = pinned.invocation.request.model as PreviewModel;
    expect(source.results).toHaveLength(golden.rows);
    const goldenText = readFileSync(resolve(root, golden.path), "utf8");
    expect(sha256(goldenText)).toBe(golden.sha256);
    const { base, origin } = await goldenInputs(source, model);
    const doc = await deriveResultDocument(base, model, source, origin);
    expect(await canonicalJsonString(doc)).toBe(goldenText);
    await expect(validateResultDocument(JSON.parse(goldenText), source)).resolves.toBeUndefined();
    const e = doc.result_envelope;
    expect(e.semantic_contract_ref).toStrictEqual({ ref_type: "semantic_contract", ref_id: "openpipestress.result_semantics/0.3.0/physics-retained-1" });
    expect(checkedJsonText(e.retained_precision)).toBe(checkedJsonText(source.retained_precision));
    expect(checkedJsonText(e.contract_evidence)).toBe(checkedJsonText(source.contract_evidence));
    // Without its receipt the golden is refused against the exact successor.
    const stripped = JSON.parse(goldenText); delete stripped.result_envelope.retained_precision;
    await expect(validateResultDocument(stripped, source)).rejects.toThrow("RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH");
  });
});
