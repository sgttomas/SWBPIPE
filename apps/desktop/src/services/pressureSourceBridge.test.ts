/** J3/J5 thin bridge: controlled Rust verdicts over invented native IPC bytes.
 * These tests exercise vocabulary, custody and presentation only; they do not
 * validate mechanics or provide a native witness. The shared J2 corpus separately
 * checks real Rust/native/wasm outcomes. No TypeScript reader is reproduced here. */
import { afterEach, describe, expect, it, vi } from "vitest";
import modelFixture from "../../../../fixtures/product_preview/invented_preview_model.json";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../types";

const seam = vi.hoisted(() => ({
  nextId: 0,
  invoke: vi.fn(),
  hashChecked: vi.fn(),
  plans: new Map<string, { unbound: unknown; bound: unknown; validation: unknown; transport: unknown }>(),
  reads: [] as { kind: string; reader?: string; sourceText: string; modelText?: string; invocationText?: string }[],
}));
vi.mock("@tauri-apps/api/core", () => ({ invoke: seam.invoke }));
vi.mock("./hashService", async importOriginal => {
  const actual = await importOriginal<typeof import("./hashService")>();
  return { ...actual, canonicalSha256HexCheckedV1: (value: unknown) =>
    seam.hashChecked.getMockImplementation() ? seam.hashChecked(value) : actual.canonicalSha256HexCheckedV1(value) };
});
vi.mock("./wasmEngine/loadWasmEngine", async importOriginal => {
  const actual = await importOriginal<typeof import("./wasmEngine/loadWasmEngine")>();
  return {
    ...actual,
    loadedWasmEngine: () => {
      const base = actual.loadedWasmEngine();
      if (!base) return null;
      const plan = (sourceText: string) => seam.plans.get(JSON.parse(sourceText).run_id);
      return {
        ...base,
        readerDispatchJson: (sourceText: string, modelText?: string, invocationText?: string) => {
          const picked = plan(sourceText);
          if (!picked) return base.readerDispatchJson(sourceText, modelText, invocationText);
          seam.reads.push({ kind: "dispatch", sourceText, modelText, invocationText });
          return JSON.stringify(invocationText === undefined ? picked.unbound : picked.bound);
        },
        readerValidateJson: (reader: string, sourceText: string, modelText?: string, invocationText?: string) => {
          const picked = plan(sourceText);
          if (!picked) return base.readerValidateJson(reader, sourceText, modelText, invocationText);
          seam.reads.push({ kind: "validate", reader, sourceText, modelText, invocationText });
          return JSON.stringify(picked.validation);
        },
        readerTransportJson: (reader: string, sourceText: string) => {
          const picked = plan(sourceText);
          if (!picked) return base.readerTransportJson(reader, sourceText);
          seam.reads.push({ kind: "transport", reader, sourceText });
          return JSON.stringify(picked.transport);
        },
      };
    },
  };
});

import { readerDispatch, readerSourceText, readerTransport, readerValidate, rememberReceivedText } from "../features/results/resultReader";
import { PHYSICS_SOURCE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_ID } from "../features/results/numericalResultQuality";
import {
  LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE, SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED,
  numericalResultStanding, registerSourceRecovery, retainedSourceRecoveryInvocation, sourceRecoveryModeMatches,
} from "../features/results/resultStanding";
import { OUTPUT_SURFACES, PRESSURE_SOURCE_OUTPUT_REFUSAL, surfaceOutputRefusal } from "../features/results/outputPolicy";
import {
  hasNativeMechanicsInvocation, receivedResultBytes, runPreviewMechanics,
  startPreviewMechanicsJob, pollPreviewMechanicsJob,
} from "./previewService";
import { analysisRecordProjection, buildAnalysisRunV03, validateAnalysisRunV03, verifyAnalysisRunRecord } from "./analysisRunCompatibility";
import { canonicalSha256HexCheckedV1 } from "./hashService";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const MODE = "sparse_interactive" as const;
const PRESSURE_SOURCE_CONTRACT_SHA256 = "8a3e3ebb7df1d14bcc6ac4e31f23e865b1d0ded011832d29d97fa13a0f6f8681";
afterEach(() => {
  seam.invoke.mockReset(); seam.hashChecked.mockReset(); seam.plans.clear(); seam.reads.length = 0;
  delete (window as Json).__TAURI_INTERNALS__;
});

const SOURCE_ROUTES = ["source_blocks", "physics_source", "pressure_source"] as const;
const SOURCE_BINDINGS = {
  source_blocks: { id: SOURCE_BLOCKS_CONTRACT_ID, sha256: "5f299065f15a157bbedf9467a598994ae684c4ecb3f851bbcb291981ec550a9f", policy: "SOURCE-BLOCKS-1", profile: "product_preview_mechanics_v1" },
  physics_source: { id: PHYSICS_SOURCE_CONTRACT_ID, sha256: "5dd0614fe485a89f6c184be152d2d580a4deb9c85e88090a4f21a26ccc686a94", policy: "PHYSICS-SOURCE-1", profile: "exact_straight_pressure_v2" },
  pressure_source: { id: PRESSURE_SOURCE_CONTRACT_ID, sha256: PRESSURE_SOURCE_CONTRACT_SHA256, policy: "PRESSURE-SOURCE-1", profile: "exact_pressure_v3" },
};
function scenario(version: "0.3.0" | "0.4.0", route: typeof SOURCE_ROUTES[number] = "pressure_source") {
  const binding = SOURCE_BINDINGS[route];
  const model = structuredClone(modelFixture) as unknown as PreviewModel;
  (model as Json).schema_version = version;
  (model as Json).pressure_contract = route === "pressure_source" ? { version: "3.0.0", mode: "exact_pressure_v3" } : { version: "2.0.0", mode: "exact_straight_v2" };
  const source = {
    schema_version: "0.2.0", document_kind: "openpipestress.product_preview.mechanics_result",
    run_id: `invented:pressure-bridge:${++seam.nextId}`, model_ref: model.project.id,
    producer: { component_name: "open_pipe_stress_product_physics", component_version: "0.2.0", semantic_contract_id: binding.id },
    numerical_quality: { status: "sensitive", cases: [] },
    formulation_basis: { profile_id: binding.profile, limitations: ["Invented bridge input; Rust verdict is controlled by this test"] },
    status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" },
    results: [{ id: "invented:pressure-bridge:row", kind: "global_nodal_rotation_x", value: -0, unit: "rad", entity_ref: "invented:node", metadata: { component: "nodal_rotation_x" } }],
    diagnostics: [], summary: {}, contract_evidence: { invented_bridge_evidence: "copied unchanged" }, source_block_recovery: { body: { policy: binding.policy } },
  } as unknown as MechanicsResult;
  const bytes = `\n${JSON.stringify(source, null, 2).replace('"value": 0', '"value": -0.0')}\n`;
  const eligible = version === "0.3.0";
  const verdict = (findings: string[]) => ({
    ok: true, route, binding: { id: binding.id, sha256: binding.sha256 }, error: null,
    standing: { route, token: eligible && findings.length === 0 ? "numerically_eligible" : "needs_recompute",
      status: eligible && findings.length === 0 ? "integrity_checked" : "needs_recompute", eligible: eligible && findings.length === 0, findings },
  });
  const plan = {
    unbound: verdict([SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED]),
    bound: verdict(eligible ? [] : [LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE]),
    validation: { ok: true, eligible, findings: [] }, transport: { ok: true },
  };
  seam.plans.set(source.run_id, plan);
  (window as Json).__TAURI_INTERNALS__ = {};
  seam.invoke.mockImplementation(async (command: string, args: Json) => {
    expect(command).toBe("run_preview_mechanics_with_solver_mode");
    expect(args).toEqual({ model, solverMode: MODE });
    return bytes;
  });
  return { model, source, bytes, plan };
}

describe("pressure-source thin desktop bridge", () => {
  it("sends the explicit raw and transport reader names with the exact received text", () => {
    const { bytes, model } = scenario("0.3.0");
    const source = JSON.parse(bytes) as MechanicsResult;
    rememberReceivedText(source, bytes);
    expect(readerDispatch(source).route).toBe("pressure_source");
    expect(readerValidate("pressure-source", source, undefined, { request: { model, materials: [] }, solver_mode: MODE })).toMatchObject({ eligible: true });
    expect(() => readerTransport("pressure-source", source)).not.toThrow();
    expect(seam.reads.filter(read => read.kind !== "dispatch")).toMatchObject([
      { kind: "validate", reader: "pressure-source", sourceText: bytes },
      { kind: "transport", reader: "pressure-source", sourceText: bytes },
    ]);
    expect(readerSourceText(source)).toBe(bytes);
  });

  it("registers a passing native reading with its captured invocation, while copies have no custody", async () => {
    const { model, source, bytes } = scenario("0.3.0");
    expect(numericalResultStanding(source, model).eligible).toBe(false);
    const received = await runPreviewMechanics(model, MODE);
    expect(receivedResultBytes(received)).toBe(bytes);
    expect(hasNativeMechanicsInvocation(received, model, MODE)).toBe(true);
    expect(numericalResultStanding(received, model)).toMatchObject({ contract: "pressure_source", eligible: true, findings: [] });
    expect(retainedSourceRecoveryInvocation(received, model)).toEqual({ request: { model, materials: [] }, solver_mode: MODE });
    expect(sourceRecoveryModeMatches(received, model, "dense_scrutiny")).toBe(false);
    const reading = seam.reads.find(read => read.kind === "validate")!;
    expect(reading.reader).toBe("pressure-source"); expect(reading.sourceText).toBe(bytes);
    expect(JSON.parse(reading.invocationText!)).toEqual({ request: { model, materials: [] }, solver_mode: MODE });
    const copy = structuredClone(received);
    expect(hasNativeMechanicsInvocation(copy, model)).toBe(false);
    expect(retainedSourceRecoveryInvocation(copy, model)).toBeNull();
    expect(numericalResultStanding(copy, model).findings).toEqual([SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED]);
    for (const surface of OUTPUT_SURFACES) expect(surfaceOutputRefusal(received, model, surface)).toBe(PRESSURE_SOURCE_OUTPUT_REFUSAL);
  });

  it("presents the Rust 0.4 false-eligibility verdict despite valid native custody", async () => {
    const { model } = scenario("0.4.0");
    const received = await runPreviewMechanics(model, MODE);
    expect(hasNativeMechanicsInvocation(received, model, MODE)).toBe(true);
    expect(numericalResultStanding(received, model)).toMatchObject({ contract: "pressure_source", status: "needs_recompute", eligible: false,
      findings: [LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE] });
    expect(retainedSourceRecoveryInvocation(received, model)).toBeNull();
    expect(sourceRecoveryModeMatches(received, model, MODE)).toBe(false);
    expect(seam.reads.some(read => read.kind === "dispatch" && read.invocationText !== undefined)).toBe(true);
  });

  it("invalidates native and source custody after a signed-zero-only source edit or a caller-model edit", async () => {
    const first = scenario("0.3.0");
    const received = await runPreviewMechanics(first.model, MODE);
    expect(Object.is(received.results[0].value, -0)).toBe(true);
    const before = JSON.stringify(received); received.results[0].value = 0;
    expect(JSON.stringify(received)).toBe(before);
    expect(receivedResultBytes(received)).toBeNull();
    expect(hasNativeMechanicsInvocation(received, first.model)).toBe(false);
    expect(retainedSourceRecoveryInvocation(received, first.model)).toBeNull();
    const second = scenario("0.3.0"), unchanged = structuredClone(second.model);
    const returned = await runPreviewMechanics(second.model, MODE);
    second.model.nodes[0].position.x += 1;
    expect(hasNativeMechanicsInvocation(returned, unchanged)).toBe(false);
    expect(retainedSourceRecoveryInvocation(returned, unchanged)).toBeNull();
  });

  it("does not register a Rust-refused reading and keeps received bytes inspectable", async () => {
    const { model, bytes, plan } = scenario("0.3.0");
    plan.validation = { ok: false, error: { code: "PHYSICS_SOURCE_RECEIPT_HASH", detail: null } } as Json;
    const returned = await runPreviewMechanics(model, MODE);
    expect(receivedResultBytes(returned)).toBe(bytes);
    expect(hasNativeMechanicsInvocation(returned, model)).toBe(false);
    expect(retainedSourceRecoveryInvocation(returned, model)).toBeNull();
  });

  it("does not give an unsupported claimed pressure-source header native registration", async () => {
    const { model, plan } = scenario("0.3.0");
    plan.unbound = { ok: false, route: "unsupported", binding: null, error: { code: "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED", detail: null },
      standing: { route: "unsupported", token: "unsupported", status: "needs_recompute", eligible: false, findings: ["SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED"] } } as Json;
    const returned = await runPreviewMechanics(model, MODE);
    expect(hasNativeMechanicsInvocation(returned, model)).toBe(false);
    expect(seam.reads.some(read => read.kind === "validate")).toBe(false);
  });

  it("keeps a completion inspectable without registration when the live caller changes in transit", async () => {
    const { model, bytes } = scenario("0.3.0"), original = structuredClone(model);
    let release!: () => void;
    const gate = new Promise<void>(resolve => { release = resolve; });
    seam.invoke.mockImplementation(async () => { await gate; return bytes; });
    const pending = runPreviewMechanics(model, MODE);
    model.nodes[0].position.x += 0.25; release();
    const returned = await pending;
    expect(receivedResultBytes(returned)).toBe(bytes);
    expect(hasNativeMechanicsInvocation(returned, original)).toBe(false);
    expect(retainedSourceRecoveryInvocation(returned, original)).toBeNull();
    expect(seam.reads.some(read => read.kind === "validate")).toBe(false);
  });

  it("registers a first known job completion and refuses custody for its terminal replay", async () => {
    const { model, bytes } = scenario("0.3.0");
    const jobId = "invented:pressure-bridge:job";
    seam.invoke.mockImplementation(async (command: string) => {
      if (command === "start_preview_mechanics_job_with_solver_mode") return { job_id: jobId, backend_cancellation_token: "token", state: "queued", cancellation_scope: "test_only" };
      if (command === "poll_preview_mechanics_job") return { job_id: jobId, state: "completed", cancellation_requested: false,
        cancellation_status: "not_requested", cancellation_scope: "test_only", result: bytes, error_message: null };
      throw new Error(`unexpected bridge command ${command}`);
    });
    const start = await startPreviewMechanicsJob(model, MODE);
    if (start.mode !== "backend_job") throw new Error("expected the test native job");
    const first = await pollPreviewMechanicsJob(start.job_id);
    expect(hasNativeMechanicsInvocation(first.result, model, MODE)).toBe(true);
    expect(numericalResultStanding(first.result!, model).eligible).toBe(true);
    const duplicate = await pollPreviewMechanicsJob(start.job_id);
    expect(hasNativeMechanicsInvocation(duplicate.result, model, MODE)).toBe(false);
    expect(retainedSourceRecoveryInvocation(duplicate.result!, model)).toBeNull();
  });
});


function recordManifest(source: MechanicsResult) {
  return { manifest_ref: { object_type: "InputManifest" as const, ref: "invented:pressure-bridge:manifest" }, manifest_sha256: "2".repeat(64),
    manifest: { model_basis: { model_ref: source.model_ref }, solver_basis: {
      solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "invented:pressure-bridge:build",
    } } };
}
async function resealRecord(record: AnalysisRunEnvelope) {
  record.analysis_run.hashes.find(hash => hash.payload_scope === "analysis_run_record")!.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(record));
  expect(await verifyAnalysisRunRecord(record)).toBe("match");
}

describe("current pressure-source record wrapper", () => {
  it.each(["0.3.0", "0.4.0"] as const)("records %s against the reviewed pressure binding and existing source rows, without granting custody", async version => {
    const { source, model } = scenario(version), before = structuredClone(source);
    const record = await buildAnalysisRunV03(source, recordManifest(source));
    expect(source).toEqual(before);
    expect(record.schema_version).toBe("0.3.0");
    expect(record.analysis_run.reproducibility.semantic_contract).toEqual({ id: PRESSURE_SOURCE_CONTRACT_ID, sha256: PRESSURE_SOURCE_CONTRACT_SHA256 });
    expect(record.analysis_run.result_refs[0].semantic_contract).toEqual({ id: PRESSURE_SOURCE_CONTRACT_ID, sha256: PRESSURE_SOURCE_CONTRACT_SHA256, signature_id: "supported-source-005" });
    expect(record.analysis_run.source_block_recovery).toEqual(source.source_block_recovery);
    expect(record.analysis_run.source_block_recovery).not.toBe(source.source_block_recovery);
    expect(record.analysis_run.contract_evidence).toEqual(source.contract_evidence);
    expect(record.analysis_run.contract_evidence).not.toBe(source.contract_evidence);
    expect(seam.reads.some(read => read.kind === "validate" && read.reader === "source-recovery")).toBe(true);
    expect(seam.reads.filter(read => read.kind === "validate").every(read => read.reader === "source-recovery" && read.invocationText === undefined)).toBe(true);
    expect(retainedSourceRecoveryInvocation(source, model)).toBeNull();
    expect(numericalResultStanding(source, model).eligible).toBe(false);
    expect(await verifyAnalysisRunRecord(record)).toBe("match");
  });

  it("keeps the Rust portable-consistency refusal before building a current record", async () => {
    const { source, plan } = scenario("0.3.0");
    plan.validation = { ok: false, error: { code: "SOURCE_BLOCKS_PUBLICATION_HASH", detail: null } } as Json;
    await expect(buildAnalysisRunV03(source, recordManifest(source))).rejects.toThrow("SOURCE_RECOVERY_PUBLICATION_HASH");
  });

  it.each([
    ["source_block_recovery", "ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH"],
    ["contract_evidence", "ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH"],
  ] as const)("refuses a coherently resealed changed %s copy", async (field, code) => {
    const { source } = scenario("0.3.0");
    const record = await buildAnalysisRunV03(source, recordManifest(source));
    (record.analysis_run as Json)[field] = { invented_substitution: true };
    await resealRecord(record);
    await expect(validateAnalysisRunV03(record, source)).rejects.toThrow(code);
  });

  it("refuses coherently resealed binding and row-source substitutions", async () => {
    const { source } = scenario("0.3.0");
    const record = await buildAnalysisRunV03(source, recordManifest(source));
    const wrongBinding = structuredClone(record);
    wrongBinding.analysis_run.reproducibility.semantic_contract!.sha256 = "0".repeat(64);
    await resealRecord(wrongBinding);
    await expect(validateAnalysisRunV03(wrongBinding, source)).rejects.toThrow("ANALYSIS_SEMANTIC_CONTRACT_MISMATCH");
    const wrongRow = structuredClone(record);
    wrongRow.analysis_run.result_refs[0].hash_refs[0].value = "0".repeat(64);
    await resealRecord(wrongRow);
    await expect(validateAnalysisRunV03(wrongRow, source)).rejects.toThrow("ANALYSIS_ROW_SOURCE_BINDING_MISMATCH");
  });

  it("recomputes the received-source digest even when the saved record checksum still matches", async () => {
    const { source } = scenario("0.3.0");
    const record = await buildAnalysisRunV03(source, recordManifest(source));
    const changed = structuredClone(source); changed.results[0].value = 1;
    expect(await verifyAnalysisRunRecord(record)).toBe("match");
    await expect(validateAnalysisRunV03(record, changed)).rejects.toThrow("ANALYSIS_RECEIVED_SOURCE_MISMATCH");
  });
});


function expectNoSourceCustody(source: MechanicsResult, model: PreviewModel) {
  expect(hasNativeMechanicsInvocation(source, model, MODE)).toBe(false);
  expect(retainedSourceRecoveryInvocation(source, model)).toBeNull();
  expect(sourceRecoveryModeMatches(source, model, MODE)).toBe(false);
  expect(numericalResultStanding(source, model)).toMatchObject({ eligible: false, findings: [SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED] });
}

describe("source registration follows final native capture", () => {
  it.each(SOURCE_ROUTES)("%s does not qualify a passed reader when the final canonical hash rejects", async route => {
    const { model, bytes } = scenario("0.3.0", route);
    seam.hashChecked.mockRejectedValue(new Error("invented final hash failure"));
    const returned = await runPreviewMechanics(model, MODE);
    expect(receivedResultBytes(returned)).toBe(bytes);
    expect(seam.reads.some(read => read.kind === "validate" && read.reader === route.replaceAll("_", "-"))).toBe(true);
    expect(seam.hashChecked).toHaveBeenCalledWith(returned);
    expectNoSourceCustody(returned, model);
  });

  it.each(SOURCE_ROUTES)("%s remains unqualified during the final hash and after a late job identity refusal", async route => {
    const { model, bytes } = scenario("0.3.0", route);
    const jobId = `invented:source-capture:${route}:late-identity`;
    let polls = 0;
    seam.invoke.mockImplementation(async (command: string) => {
      if (command === "start_preview_mechanics_job_with_solver_mode") return { job_id: jobId, backend_cancellation_token: "token", state: "queued", cancellation_scope: "test_only" };
      if (command === "poll_preview_mechanics_job") return { job_id: ++polls === 1 ? jobId : "invented:wrong-job", state: "completed", cancellation_requested: false,
        cancellation_status: "not_requested", cancellation_scope: "test_only", result: polls === 1 ? bytes : null, error_message: null };
      throw new Error(`unexpected capture command ${command}`);
    });
    let release!: () => void, entered!: () => void, received!: MechanicsResult;
    const hashing = new Promise<void>(resolve => { entered = resolve; });
    const gate = new Promise<void>(resolve => { release = resolve; });
    seam.hashChecked.mockImplementation(async (source: MechanicsResult) => {
      received = source; entered(); await gate; return "3".repeat(64);
    });
    const start = await startPreviewMechanicsJob(model, MODE);
    if (start.mode !== "backend_job") throw new Error("expected the test native job");
    const pending = pollPreviewMechanicsJob(start.job_id);
    await hashing;
    expectNoSourceCustody(received, model);
    // A second, refused response invalidates the same capture while the first
    // completion is still awaiting its final hash.
    await expect(pollPreviewMechanicsJob(start.job_id)).rejects.toThrow("SOLVE-JOB-IDENTITY-MISMATCH");
    release();
    const completed = await pending;
    expect(completed.result).toBe(received);
    expect(receivedResultBytes(received)).toBe(bytes);
    expectNoSourceCustody(received, model);
  });

  it.each(SOURCE_ROUTES)("%s activates after successful final registration and preserves direct-registration behavior", async route => {
    const { model, source } = scenario("0.3.0", route);
    const invocation = { request: { model, materials: [] }, solver_mode: MODE };
    await registerSourceRecovery(source, invocation, model);
    expect(hasNativeMechanicsInvocation(source, model)).toBe(false);
    expect(retainedSourceRecoveryInvocation(source, model)).toEqual(invocation);
    const returned = await runPreviewMechanics(model, MODE);
    expect(hasNativeMechanicsInvocation(returned, model, MODE)).toBe(true);
    expect(retainedSourceRecoveryInvocation(returned, model)).toEqual(invocation);
    expect(numericalResultStanding(returned, model).eligible).toBe(true);
  });
});
