import { buildAnalysisRunV03, analysisRecordProjection, modelLoadBasisRefs, ANALYSIS_RUN_SCHEMA_VERSION_RETIRED } from "../../services/analysisRunCompatibility";
import { nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { render, screen, cleanup } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { buildHistoricalRunContext, buildBundledReferenceContext, HistoricalRunPanel } from "./HistoricalRunContext";
import { numericalResultStanding } from "./resultStanding";
import { loadBundledMechanicsReference, hasNativeMechanicsInvocation } from "../../services/previewService";
import savedModel from "../../../../../fixtures/model_operations/precision_connected_ui_model.json";
import savedResult from "../../../../../fixtures/results/preview_physics_connected_sparse.json";
import { canonicalSha256HexCheckedV1, computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import type { AnalysisRunEnvelope, LocalProjectEnvelope, PreviewModel, MechanicsResult } from "../../types";
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

/** A saved run with a current (0.3.0) analysis record over the connected
 * preview-physics-2 producer output and its model (B1 retired the synthetic
 * precision-1 header these tests used), so they exercise the saved-run integrity
 * checks only. */
async function savedEnvelope() {
  const model = structuredClone(savedModel) as unknown as PreviewModel;
  const mechanics_result = structuredClone(savedResult) as unknown as MechanicsResult;
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: mechanics_result.producer!.component_name, solver_version: mechanics_result.producer!.component_version, solver_build_ref: "synthetic@1", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
  const analysis_run = await buildAnalysisRunV03(mechanics_result, inputManifest);
  const model_hash = await computeModelHash(model);
  const payload = { model, mechanics_result, analysis_run, model_hash, editor_intents: [], proposal: null, selected_review_target: null };
  const project_envelope_hash = await computeProjectEnvelopeHash(payload);
  return { ...payload, project_envelope_hash } as unknown as LocalProjectEnvelope;
}

async function rehash(saved: LocalProjectEnvelope) {
  saved.project_envelope_hash = await computeProjectEnvelopeHash({ model: saved.model, editor_intents: saved.editor_intents ?? [], proposal: saved.proposal ?? null, selected_review_target: saved.selected_review_target ?? null, mechanics_result: saved.mechanics_result, analysis_run: saved.analysis_run, model_hash: saved.model_hash });
  return saved;
}

describe("transient HistoricalRunContext integrity", () => {
  it.each(["0.1.0", "0.2.0"])("refuses a saved %s analysis record by name and verifies none of its claims", async version => {
    const saved = await savedEnvelope();
    saved.analysis_run!.schema_version = version;
    await rehash(saved);
    const before = JSON.stringify(saved);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).toContain(ANALYSIS_RUN_SCHEMA_VERSION_RETIRED);
    expect(context!.findings).not.toEqual(expect.arrayContaining(["HISTORICAL_RESULT_HASH_MISMATCH"]));
    expect(context!.findings).not.toContain("HISTORICAL_ANALYSIS_HASH_MISMATCH");
    expect(context!.findings).not.toContain("HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE");
    // The saved evidence stays readable and unchanged; only a re-run records a current run.
    expect(context!.analysisRun).toBe(saved.analysis_run);
    expect(context!.mechanicsResult).toBe(saved.mechanics_result);
    expect(JSON.stringify(saved)).toBe(before);
  });

  it("names an unknown saved record version as unsupported", async () => {
    const saved = await savedEnvelope();
    saved.analysis_run!.schema_version = "0.9.0";
    const findings = (await buildHistoricalRunContext(await rehash(saved)))!.findings;
    expect(findings).toContain("HISTORICAL_ANALYSIS_SCHEMA_VERSION_UNSUPPORTED");
    expect(findings).not.toContain(ANALYSIS_RUN_SCHEMA_VERSION_RETIRED);
  });

  it("verifies a current record and reports a changed record claim", async () => {
    const saved = await savedEnvelope();
    expect(saved.analysis_run!.schema_version).toBe("0.3.0");
    const findings = (await buildHistoricalRunContext(saved))!.findings;
    expect(findings).not.toEqual(expect.arrayContaining(["HISTORICAL_RESULT_HASH_MISMATCH", "HISTORICAL_ANALYSIS_HASH_MISMATCH", "HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE", ANALYSIS_RUN_SCHEMA_VERSION_RETIRED]));
    const tampered = await savedEnvelope();
    tampered.analysis_run!.analysis_run.hashes.find((hash) => hash.payload_scope === "analysis_run_record")!.value = "0".repeat(64);
    const tamperedFindings = (await buildHistoricalRunContext(await rehash(tampered)))!.findings;
    expect(tamperedFindings).toContain("HISTORICAL_ANALYSIS_HASH_MISMATCH");
    expect(tamperedFindings).not.toContain("HISTORICAL_RESULT_HASH_MISMATCH");
  });

  it("matches the bound received-result hash without changing saved evidence", async () => {
    const saved = await savedEnvelope();
    const storedResultHash = saved.analysis_run!.analysis_run.hashes.find((hash) => hash.payload_scope === "received_result")!.value;
    expect(await canonicalSha256HexCheckedV1(saved.mechanics_result)).toBe(storedResultHash);
    const before = JSON.stringify(saved);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).toContain("HISTORICAL_INPUT_MANIFEST_MISSING");
    expect(context!.findings).not.toContain("HISTORICAL_RESULT_HASH_MISMATCH");
    expect(context!.findings).not.toContain("HISTORICAL_ENVELOPE_HASH_MISMATCH");
    expect(context!.findings).not.toContain("HISTORICAL_MODEL_HASH_MISMATCH");
    expect(context!.mechanicsResult).toBe(saved.mechanics_result);
    expect(context!.analysisRun).toBe(saved.analysis_run);
    expect(context!.modelHash).toBe(saved.model_hash);
    expect(context!.envelopeHash).toBe(saved.project_envelope_hash);
    expect(context).not.toHaveProperty("inputManifest");
    expect(JSON.stringify(saved)).toBe(before);
  });

  it("still reports a real changed result value against the bound received-result hash", async () => {
    const saved = await savedEnvelope();
    saved.mechanics_result!.results.find((row) => row.kind === "element_local_axial_force")!.value += 1;
    const before = JSON.stringify(saved);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).toContain("HISTORICAL_RESULT_HASH_MISMATCH");
    expect(context!.findings).not.toContain("HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE");
    expect(JSON.stringify(saved)).toBe(before);
  });

  it("preserves saved bytes and reports missing historical manifest without rebinding inputs", async () => {
    const saved = await savedEnvelope();
    const before = JSON.stringify(saved);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.designation).toBe("historical_saved_run");
    expect(context!.findings).toContain("HISTORICAL_INPUT_MANIFEST_MISSING");
    expect(context!.findings).not.toContain("HISTORICAL_RESULT_HASH_MISMATCH");
    expect(context!.mechanicsResult).toEqual(saved.mechanics_result);
    expect(context!.analysisRun).toEqual(saved.analysis_run);
    expect(context).not.toHaveProperty("inputManifest");
    expect(JSON.stringify(saved)).toBe(before);
  });

  it("surfaces inconsistent identities and all saved hash findings while retaining history", async () => {
    const saved = await savedEnvelope();
    saved.mechanics_result!.model_ref = "project:inconsistent";
    saved.mechanics_result!.run_id = "run:inconsistent";
    saved.model_hash!.value = "sha256:inconsistent";
    saved.project_envelope_hash!.value = "sha256:inconsistent";
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).toEqual(expect.arrayContaining(["HISTORICAL_MODEL_REF_MISMATCH", "HISTORICAL_RUN_REF_MISMATCH", "HISTORICAL_MODEL_HASH_MISMATCH", "HISTORICAL_ENVELOPE_HASH_MISMATCH", "HISTORICAL_RESULT_HASH_MISMATCH"]));
    expect(context!.mechanicsResult).toEqual(saved.mechanics_result);
  });

  it("retains missing original hashes without inventing a manifest", async () => {
    const saved = await savedEnvelope();
    saved.model_hash = null;
    saved.project_envelope_hash = null;
    saved.analysis_run!.analysis_run.hashes = [];
    const context = await buildHistoricalRunContext(saved);
    expect(context!.modelHash).toBeNull();
    expect(context!.findings).toEqual(expect.arrayContaining(["HISTORICAL_MODEL_HASH_MISSING", "HISTORICAL_ENVELOPE_HASH_MISSING", "HISTORICAL_RESULT_HASH_MISSING"]));
  });
  it("keeps valid saved mechanics readable when analysis evidence is malformed", async () => {
    const saved = await savedEnvelope();
    saved.analysis_run = { analysis_run: { run_id: "saved-malformed", hashes: "corrupt" } } as unknown as LocalProjectEnvelope["analysis_run"];
    const before = JSON.stringify(saved.analysis_run);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).toContain("HISTORICAL_ANALYSIS_EVIDENCE_INVALID");
    expect(context!.mechanicsResult).toEqual(saved.mechanics_result);
    expect(JSON.stringify(context!.analysisRun)).toBe(before);
  });

  it.each([
    ["false", false],
    ["zero", 0],
    ["empty string", ""],
    ["array", []],
    ["incomplete object", { run_id: "received-incomplete" }],
    ["invalid row", {
      schema_version: "0.2.0", document_kind: "MechanicsResult", run_id: "received-invalid-row",
      model_ref: "model:invalid", status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" },
      summary: {}, results: [{ id: "unsafe", kind: "displacement_magnitude", value: "not-a-number", unit: "mm", entity_ref: "node:unsafe" }], diagnostics: []
    }]
  ])("preserves a malformed %s carrier while withholding it from rendering", async (_label, carrier) => {
    const saved = await savedEnvelope();
    (saved as unknown as { mechanics_result: unknown }).mechanics_result = carrier;
    const before = structuredClone(carrier);
    const context = await buildHistoricalRunContext(saved);
    expect(context).not.toBeNull();
    expect(context!.designation).toBe("historical_saved_run");
    expect(context!.findings).toContain("HISTORICAL_MECHANICS_EVIDENCE_MALFORMED");
    expect(context!.mechanicsResult).toBeNull();
    expect(context!.rawMechanicsResult).toEqual(before);
    const expectedEnvelope = await computeProjectEnvelopeHash({
      model: saved.model, editor_intents: saved.editor_intents ?? [], proposal: saved.proposal ?? null,
      selected_review_target: saved.selected_review_target ?? null,
      mechanics_result: carrier, analysis_run: saved.analysis_run, model_hash: saved.model_hash
    } as never);
    expect(context!.envelopePayloadHash).toBe(expectedEnvelope!.value);
    expect((saved as unknown as { mechanics_result: unknown }).mechanics_result).toEqual(before);
  });

});

it("keeps unsupported raw0.2 readable history and unqualified without rewriting evidence", async () => {
 const saved = await savedEnvelope();
 for (const header of ["producer", "numerical_quality", "formulation_basis"]) delete (saved.mechanics_result as unknown as Record<string, unknown>)[header];
 const before = JSON.stringify(saved.mechanics_result);
 const context = await buildHistoricalRunContext(saved);
 expect(context?.mechanicsResult).not.toBeNull();
 expect(context?.findings).toContain("SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED");
 expect(JSON.stringify(saved.mechanics_result)).toBe(before);
});

it("retains contradictory null-header historical bytes and hash claims while withholding interpretation", async () => {
  const saved = await savedEnvelope();
  Object.assign(saved.mechanics_result!, { producer: null });
  const before = JSON.stringify(saved);
  const context = await buildHistoricalRunContext(saved);
  expect(context?.mechanicsResult).not.toBeNull();
  expect(context?.rawMechanicsResult).toEqual(saved.mechanics_result);
  expect(context?.findings).toContain("SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED");
  expect(JSON.stringify(saved)).toBe(before);
});

/** B7b: a reopened pressure-2 record is read by the record validator; a forged record's
 * refusal is a displayed finding only. The record is rehashed after each forgery, so its
 * own checksum and the project envelope hash still match and only the validator refuses. */
describe("a reopened exact (pressure-2) record", () => {
  async function savedExact() {
    const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
    const model = pair.model, mechanics_result = pair.source;
    const inputManifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: mechanics_result.producer!.component_name, solver_version: mechanics_result.producer!.component_version, solver_build_ref: "synthetic@1", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
    const analysis_run = await buildAnalysisRunV03(mechanics_result, inputManifest, undefined, modelLoadBasisRefs(model));
    const model_hash = await computeModelHash(model);
    return rehash({ model, mechanics_result, analysis_run, model_hash, editor_intents: [], proposal: null, selected_review_target: null } as unknown as LocalProjectEnvelope);
  }
  async function forge(edit: (run: AnalysisRunEnvelope["analysis_run"], source: MechanicsResult, envelope: AnalysisRunEnvelope) => void) {
    const saved = await savedExact();
    edit(saved.analysis_run!.analysis_run, saved.mechanics_result!, saved.analysis_run!);
    const own = saved.analysis_run!.analysis_run.hashes.find(h => h.payload_scope === "analysis_run_record")!;
    own.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(saved.analysis_run!));
    return rehash(saved);
  }
  it("reads a genuine record without a record finding", async () => {
    const findings = (await buildHistoricalRunContext(await savedExact()))!.findings;
    expect(findings).toStrictEqual(["HISTORICAL_INPUT_MANIFEST_MISSING"]);
  });
  // A malformed record the validator reads unguarded is one named finding, never the
  // runtime's text (e.g. "Cannot read properties of null").
  it.each([
    ["a null run_contract_status", (_run: Json, _source: MechanicsResult, envelope: Json) => { envelope.run_contract_status = null; }],
    ["a null reproducibility", (run: Json) => { run.reproducibility = null; }],
    ["a null result ref", (run: Json) => { run.result_refs[0] = null; }],
  ] as const)("names a malformed record with %s HISTORICAL_ANALYSIS_EVIDENCE_INVALID and changes nothing", async (_name, edit) => {
    const saved = await forge(edit as never), before = JSON.stringify(saved);
    const standing = numericalResultStanding(saved.mechanics_result!, saved.model);
    const context = (await buildHistoricalRunContext(saved))!;
    expect(context.findings).toStrictEqual(["HISTORICAL_INPUT_MANIFEST_MISSING", "HISTORICAL_ANALYSIS_EVIDENCE_INVALID"]);
    expect(context.mechanicsResult).toBe(saved.mechanics_result);
    expect(context.analysisRun).toBe(saved.analysis_run);
    expect(numericalResultStanding(context.mechanicsResult!, saved.model)).toStrictEqual(standing);
    render(<HistoricalRunPanel context={context} />);
    expect(screen.getByText("HISTORICAL_ANALYSIS_EVIDENCE_INVALID")).toBeInTheDocument();
    expect(JSON.stringify(saved)).toBe(before);
  });
  // The saved load basis is read as saved; the current model is not consulted for it, so a
  // model without load_cases still reopens. An absent list is named by the validator.
  it.each([
    ["absent", (run: Json) => { delete run.load_basis_refs; }, "ANALYSIS_SOURCE_LOAD_BASIS_MISMATCH"],
    ["null", (run: Json) => { run.load_basis_refs = null; }, "ANALYSIS_LOAD_BASIS_INVALID"],
  ] as const)("reopens a record whose load basis is %s on a model without load_cases and names it", async (_name, edit, code) => {
    const saved = await forge(edit as never);
    delete (saved.model as Json).load_cases;
    saved.model_hash = await computeModelHash(saved.model);
    await rehash(saved);
    const before = JSON.stringify(saved);
    // The model's missing cases show in the source's standing; the record adds only its code.
    const standing = numericalResultStanding(saved.mechanics_result!, saved.model).findings;
    expect(standing).toContain("REQUESTED_NUMERICAL_BASIS_UNAVAILABLE");
    const context = (await buildHistoricalRunContext(saved))!;
    expect(context.findings).toStrictEqual(["HISTORICAL_INPUT_MANIFEST_MISSING", ...standing, code]);
    expect(context.mechanicsResult).toBe(saved.mechanics_result);
    expect(context.analysisRun).toBe(saved.analysis_run);
    expect(JSON.stringify(saved)).toBe(before);
  });
  it.each([
    ["its table switched to pressure-source-1's", (run: Json) => { run.reproducibility.semantic_contract = { id: "openpipestress.result_semantics/0.3.0/pressure-source-1", sha256: "84f37c63c6cc5ec7539d6ee2b39c338bd2c6ef16b25772005fe0c8f9300fddb9" }; }, "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"],
    ["the source's evidence embedded", (run: Json, source: MechanicsResult) => { run.contract_evidence = structuredClone(source.contract_evidence); }, "ANALYSIS_PHYSICS_SOURCE_DOWNGRADE_FORBIDDEN"],
    ["a retained-precision receipt embedded", (run: Json) => { run.retained_precision = { receipt_sha256: "0".repeat(64) }; }, "ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"],
  ] as const)("names a forged record with %s, displays it, and changes nothing", async (_name, edit, code) => {
    const saved = await forge(edit as never), before = JSON.stringify(saved);
    const context = (await buildHistoricalRunContext(saved))!;
    expect(context.findings).toContain(code);
    expect(context.findings).not.toContain("HISTORICAL_ANALYSIS_HASH_MISMATCH");
    expect(context.findings).not.toContain("HISTORICAL_ENVELOPE_HASH_MISMATCH");
    expect(context.mechanicsResult).toBe(saved.mechanics_result);
    expect(context.analysisRun).toBe(saved.analysis_run);
    render(<HistoricalRunPanel context={context} />);
    expect(screen.getByText(code)).toBeInTheDocument();
    expect(JSON.stringify(saved)).toBe(before);
  });
});

afterEach(cleanup);
describe("bundled reference inspection", () => {
  it("retains source and original model without Current evidence or saved-history designation", async () => {
    const reference = await loadBundledMechanicsReference();
    const before = JSON.stringify(reference);
    const context = buildBundledReferenceContext(reference);
    expect(context.designation).toBe("bundled_reference");
    expect(context.rawMechanicsResult).toBe(reference.source);
    expect(context.referenceModel).toBe(reference.model);
    expect(context.rawAnalysisRun).toBeNull();
    expect(context.modelHash).toBeNull();
    expect(context.envelopeHash).toBeNull();
    expect(hasNativeMechanicsInvocation(reference.source, reference.model)).toBe(false);
    render(<HistoricalRunPanel context={context} />);
    expect(screen.getByRole("region", { name: "Bundled reference — not a solve for the current model" })).toBeInTheDocument();
    expect(screen.queryByText("Historical saved run")).not.toBeInTheDocument();
    expect(JSON.stringify(reference)).toBe(before);
  });
});

// RT-A4 §10.4 display rule: a project saved before RT-A4 reopens with its standing as recorded under
// its table, shown apart from the reopen's own findings; a failed reopen check stays an ordinary
// finding beside it.
describe("a project saved before RT-A4 (historical read)", () => {
  const dir = "../../../../../fixtures/persistence/recorded_pre_rt_a4_pf/preview_physics_2";
  async function opened(edit: (run: Json) => void = () => undefined): Promise<LocalProjectEnvelope> {
    const { readFileSync } = await import("node:fs");
    const { resolve } = await import("node:path");
    const text = (name: string) => readFileSync(resolve(__dirname, dir, name), "utf8");
    const transient = JSON.parse(text("transient_run_input.json"));
    const model = JSON.parse(transient.input_manifest_canonical).model_basis.model_payload as PreviewModel;
    const analysis_run = structuredClone(transient.run_record) as Json;
    edit(analysis_run);
    const claim = analysis_run.analysis_run.hashes.find((hash: Json) => hash.payload_scope === "analysis_run_record");
    claim.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(analysis_run));
    const payload = { model, mechanics_result: JSON.parse(text("preview_physics_connected_sparse.json")), analysis_run, model_hash: await computeModelHash(model), editor_intents: [], proposal: null, selected_review_target: null };
    return { ...payload, project_envelope_hash: await computeProjectEnvelopeHash(payload as never) } as unknown as LocalProjectEnvelope;
  }
  const TABLE = { id: "openpipestress.result_semantics/0.3.0/preview-physics-2", sha256: "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2" };

  it("shows the historical reader's findings as recorded under the table, apart from the reopen's", async () => {
    const context = (await buildHistoricalRunContext(await opened()))!;
    expect(context.recordedTableRead).toEqual(TABLE);
    expect(context.findings).toStrictEqual(["HISTORICAL_INPUT_MANIFEST_MISSING"]);
    render(<HistoricalRunPanel context={context} />);
    const recorded = screen.getByTestId("historical-run-recorded-reader");
    expect(recorded).toHaveTextContent(`As recorded under ${TABLE.id} (sha256 ${TABLE.sha256}); historical evidence only, not a current standing`);
    for (const finding of context.historicalReaderFindings) expect(recorded).toHaveTextContent(finding);
  });

  it("a failed reopen validation is an ordinary finding, never folded into the as-recorded read", async () => {
    const context = (await buildHistoricalRunContext(await opened((run) => { run.analysis_run.source_block_recovery = { forged: true }; })))!;
    expect(context.findings).toContain("SOURCE_RECOVERY_METADATA_CONTRADICTION");
    expect(context.checkFindings).toContain("SOURCE_RECOVERY_METADATA_CONTRADICTION");
    expect(context.historicalReaderFindings).not.toContain("SOURCE_RECOVERY_METADATA_CONTRADICTION");
    render(<HistoricalRunPanel context={context} />);
    expect(screen.getByTestId("historical-run-recorded-reader")).not.toHaveTextContent("SOURCE_RECOVERY_METADATA_CONTRADICTION");
    expect(screen.getByText("SOURCE_RECOVERY_METADATA_CONTRADICTION")).toBeInTheDocument();
  });

  // Focused confirmation r6 (#98): the panel's own standing line classified the result with the
  // current reader only, so a run read correctly under its recorded table still said
  // "Unsupported numerical contract … Needs recompute". It is now classified under the table.
  it("the results panel states the run as read under its recorded table, as recorded", async () => {
    for (const [variant, name, table] of [
      ["preview_physics_2", null, TABLE],
      ["preview_physics_retained_1", "retained_precision_milestone_successor_sparse_interactive", {
        id: "openpipestress.result_semantics/0.3.0/preview-physics-retained-1",
        sha256: "b001070e5fedeeb101522b22ce0d61bb7cf85f82e86b8d851713f8d9fc8c4c46" }],
    ] as const) {
      let envelope: LocalProjectEnvelope;
      if (name === null) envelope = await opened();
      else {
        const { readFileSync } = await import("node:fs");
        const { resolve } = await import("node:path");
        const text = (file: string) => readFileSync(resolve(__dirname, "../../../../../fixtures/persistence/recorded_pre_rt_a4_pf", variant, file), "utf8");
        const document = JSON.parse(text(`${name}.json`));
        const model = document.invocation.request.model as PreviewModel;
        const payload = { model, mechanics_result: document.source, analysis_run: JSON.parse(text(`${name}.analysis_run.json`)), model_hash: await computeModelHash(model), editor_intents: [], proposal: null, selected_review_target: null };
        envelope = { ...payload, project_envelope_hash: await computeProjectEnvelopeHash(payload as never) } as unknown as LocalProjectEnvelope;
      }
      const context = (await buildHistoricalRunContext(envelope))!;
      expect(context.recordedTableRead).toEqual(table);
      const { unmount } = render(<HistoricalRunPanel context={context} />);
      const standing = screen.getByTestId("numerical-result-standing");
      expect(standing).toHaveTextContent(`As recorded under ${context.recordedTableRead!.id} (sha256 ${context.recordedTableRead!.sha256}); historical evidence only, not a current standing`);
      expect(standing).not.toHaveTextContent("Unsupported numerical contract");
      unmount();
    }
  });

  it("a run read under the current tables has no historical reader scope", async () => {
    const context = (await buildHistoricalRunContext(await savedEnvelope()))!;
    expect(context.recordedTableRead).toBeNull();
    expect(context.historicalReaderFindings).toStrictEqual([]);
    render(<HistoricalRunPanel context={context} />);
    expect(screen.queryByTestId("historical-run-recorded-reader")).toBeNull();
  });
});
