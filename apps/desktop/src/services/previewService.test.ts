import historicalMechanicsFixture from "../../../../fixtures/product_preview/invented_demo_result_legacy_0_1.json";
// The bundled producer results as their bytes read (signed zeros kept), as previewService reads them (b7-plan R2).
import sparseMechanicsFixtureText from "../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json?raw";
import denseMechanicsFixtureText from "../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_dense.json?raw";
const sparseMechanicsFixture = JSON.parse(sparseMechanicsFixtureText), denseMechanicsFixture = JSON.parse(denseMechanicsFixtureText);
import inventedPreviewModel from "../../../../fixtures/product_preview/invented_preview_model.json";
import { createNativeMechanicsReplay, createSolveJobTransport, nativeMechanicsReplayPair, resultArrayBuffer, solveBytes } from "../test/nativeMechanicsReplay";
import { checkedJsonText } from "./hashService";
import { ResourceUnavailableError } from "./memoryPool";
import { buildAnalysisRunV02 } from "./analysisRunCompatibility";
import { PRECISION_CONTRACT_ID } from "../features/results/numericalResultQuality";
import { sourceContract } from "../features/results/resultReader";
import { numericalResultStanding } from "../features/results/resultStanding";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
afterEach(() => {
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});
import type { MechanicsResult, PreviewModel } from "../types";
import {
  appliedRuleCheckStatus,
  bindSourceResultDimensions,
  buildAnalysisRunPreview,
  loadDesignKnowledge,
  loadPreviewModel,
  loadBundledMechanicsReference,
  hasNativeMechanicsInvocation,
  retainedNativeMechanicsInvocation,
  pollPreviewMechanicsJob,
  cancelPreviewMechanicsJob,
  deliveredMechanicsResultJob,
  discardPreviewMechanicsJobResult,
  disposeDeliveredMechanicsResult,
  runPreviewMechanics,
  startPreviewMechanicsJob,
  takePreviewMechanicsJobResult,
  validateBrowserMechanicsFixture,
  type PreviewSolverMode,
} from "./previewService";
import {
  buildCurrentSessionInputManifest,
  type CurrentSessionInputManifestEvidence,
} from "./inputManifestService";

function cloneModel(model: PreviewModel): PreviewModel {
  return JSON.parse(JSON.stringify(model)) as PreviewModel;
}

// Minimal solved envelope whose plain solve leaves rule_check incomplete (the
// solve runs no user rule checks), so the aggregate-threading behavior is
// isolated from any fixture content.
const solvedResultStub = {
  schema_version: "0.2.0",
  producer: {component_name:"open_pipe_stress_product_physics", component_version:"0.2.0", semantic_contract_id:PRECISION_CONTRACT_ID},
  numerical_quality: {value_representation:"finite_binary64",publication_quantization:"none",integrity_policy:"M03-INTEGRITY-v1",status:"not_assessed",cases:[]},
  formulation_basis: {profile_id:"product_preview_mechanics_v1", limitations:["Synthetic consumer test; no physics proof"]},
  document_kind: "openpipestress.product_preview.mechanics_result",
  run_id: "run:appagg-test",
  model_ref: "project:appagg-test",
  status: {
    mechanics: "MECHANICS_SOLVED",
    rule_check: "RULE_INPUTS_INCOMPLETE",
    professional_acceptance: "NOT_PROVIDED",
  },
  summary: {},
  results: [],
  diagnostics: [],
} as unknown as MechanicsResult;

async function manifestFor(
  result: MechanicsResult,
  solverMode: PreviewSolverMode = "sparse_interactive",
): Promise<CurrentSessionInputManifestEvidence> {
  const model = cloneModel(await loadPreviewModel());
  model.project.id = result.model_ref;
  return buildCurrentSessionInputManifest({
    model,
    solver: {
      solver_name: "open_pipe_stress_product_physics",
      solver_version: "0.2.0",
      solver_build_ref: "open_pipe_stress_product_physics@0.2.0",
      solver_mode: solverMode,
      settings: {
        nonlinear_iteration_policy:
          "DEC-046-CV-B-product-preview-active-set-count-v1",
        sparse_evidence_lane: solverMode === "sparse_interactive",
      },
    },
    active_rule_packs: [],
    external_assets: [],
  });
}

async function buildRun(
  result: MechanicsResult,
  ruleCheckAggregate?: string | null,
) {
  return buildAnalysisRunPreview(result, {
    inputManifest: await manifestFor(result),
    ruleCheckAggregate,
  });
}

function hashByScope(
  env: Awaited<ReturnType<typeof buildAnalysisRunPreview>>,
  scope: string,
): string | undefined {
  return env.analysis_run.hashes.find((entry) => entry.payload_scope === scope)
    ?.value;
}

describe("previewService explicit reference inspection and browser refusal", () => {
  it("loads the actual bundled producer result only as explicit reference data", async () => {
    const reference = await loadBundledMechanicsReference();
    const result = reference.source;
    expect(reference.standing).toBe("reference_only");
    expect(reference.provenance).toMatchObject({ fresh_invocation_performed: false, current_use_eligible: false });
    expect(hasNativeMechanicsInvocation(result, await loadPreviewModel())).toBe(false);

    expect(result.status.mechanics).toBe(sparseMechanicsFixture.status.mechanics);
    expect(result.model_ref).toBe("project:invented-demo-loop-01");
    expect(reference.model.project.id).toBe(result.model_ref);
    expect(result.results.length).toBe(sparseMechanicsFixture.results.length);
  });

  it("refuses fresh browser solves for omitted, null and unchanged bundled models", async () => {
    const model = await loadPreviewModel();
    for (const request of [undefined, null, model]) {
      await expect(runPreviewMechanics(request)).rejects.toThrow("BROWSER_SOLVE_BACKEND_REQUIRED_REFERENCE_ONLY");
      await expect(startPreviewMechanicsJob(request)).rejects.toThrow("BROWSER_SOLVE_BACKEND_REQUIRED_REFERENCE_ONLY");
    }
    expect(invokeMock).not.toHaveBeenCalled();
    expect((await loadBundledMechanicsReference()).source).toEqual(sparseMechanicsFixture);
  });

  it("preserves the historical-format carrier's nonlinear evidence without calling it a fresh solve", async () => {
    const result = structuredClone(historicalMechanicsFixture) as MechanicsResult;
    const run = await buildAnalysisRunV02(result, await manifestFor(result));

    expect(
      result.results.find(
        (item) => item.id === "result:nonlinear-support:iteration-count",
      ),
    ).toMatchObject({
      kind: "nonlinear_support_active_set_iteration_count",
      value: 2,
      unit: "count",
    });
    expect(
      result.results.find(
        (item) =>
          item.id === "result:nonlinear-support:support-NL-140:uy-reaction",
      ),
    ).toMatchObject({
      kind: "nonlinear_support_final_reaction",
      entity_ref: "support:NL-140",
      unit: "N",
    });
    expect(
      result.results.find(
        (item) =>
          item.id ===
          "result:nonlinear-support:support-NL-130-FRIC:state-code",
      ),
    ).toMatchObject({
      kind: "nonlinear_support_active_set_state_code",
      entity_ref: "support:NL-130-FRIC",
      value: 3,
    });
    const frictionReaction = result.results.find(
      (item) =>
        item.id ===
        "result:nonlinear-support:support-NL-130-FRIC:uz-reaction",
    );
    expect(frictionReaction).toMatchObject({
      kind: "nonlinear_support_final_reaction",
      entity_ref: "support:NL-130-FRIC",
      unit: "N",
      value: 0.5237328200600512,
    });
    const frictionNormal = result.results.find(
      (item) =>
        item.id ===
        "result:nonlinear-support:support-NL-130-FRIC:friction-normal-reaction",
    );
    expect(frictionNormal).toMatchObject({
      kind: "nonlinear_support_friction_normal_reaction_derived",
      entity_ref: "support:NL-130-FRIC",
      value: 52.373281987314456,
      unit: "N",
    });
    // The accepted same-iterate current-normal law, at the old fixture's 6-digit rounding.
    expect(Math.round((frictionReaction?.value ?? Number.NaN) * 1_000_000) / 1_000_000).toBe(
      Math.round(
        0.01 * (frictionNormal?.value ?? Number.NaN) * 1_000_000,
      ) / 1_000_000,
    );
    const normalBasis = result.results.find(
      (item) =>
        item.id ===
        "result:nonlinear-support:support-NL-130-FRIC:friction-normal-reaction",
    )?.metadata?.basis;
    expect(normalBasis).toContain("derived_support_reaction");
    expect(normalBasis).toContain("source_ref=support:S-130");
    expect(normalBasis).toContain("source_dof=uy");
    expect(normalBasis).not.toContain("derived_normal_force_model=TBD");
    expect(
      result.diagnostics.some((item) => item.code === "TOLERANCE_POLICY_TBD"),
    ).toBe(false);
    expect(
      result.results
        .find(
          (item) =>
            item.id === "result:nonlinear-support:iteration-count",
        )
        ?.metadata?.basis,
    ).toContain("DEC-046-CV-B-product-preview-active-set-count-v1");
    expect(
      result.diagnostics.some(
        (item) => item.code === "NONLINEAR_SUPPORT_LOOP_CONVERGED",
      ),
    ).toBe(true);
    expect(
      run.analysis_run.result_refs.find(
        (item) =>
          item.result_ref.ref === "result:nonlinear-support:iteration-count",
      )?.result_family,
    ).toBeNull();
  });

  it("throws the specific edited-model backend error before creating a raw result", async () => {
    const original = await loadPreviewModel();
    const edited = cloneModel(original);
    edited.project.id = "project:edited-browser-solve";
    edited.materials![0].elastic_modulus.value = 195_000_000_000;
    await expect(runPreviewMechanics(edited)).rejects.toThrow("BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL");
    expect(invokeMock).not.toHaveBeenCalled();
    expect(original.project.id).toBe("project:invented-demo-loop-01");
    expect(original.materials![0].elastic_modulus.value).toBe(200_000_000_000);
  });

  it.each([
    ["sparse_interactive", sparseMechanicsFixture],
    ["dense_scrutiny", denseMechanicsFixture],
  ] as const)("returns genuine %s fixture bytes and preserves producer quality", async (mode, fixture) => {
    const model = await loadPreviewModel();
    const reference = await loadBundledMechanicsReference(mode);
    const result = reference.source;
    expect(result).toEqual(fixture);
    expect(sourceContract(result)).toBe("preview_physics");
    expect(result.numerical_quality).toEqual(fixture.numerical_quality);
    // Numerical evidence remains unchanged; its standing cannot prove a new
    // invocation. This analysis-record exercise is reference interpretation.
    const numerical = numericalResultStanding(result, model);
    if (fixture.numerical_quality.status !== "checks_passed") {
      expect(numerical.eligible).toBe(false);
      expect(numerical.findings).toContain("NUMERICAL_INTEGRITY_NOT_QUALIFIED");
    }
    expect(reference.provenance).toMatchObject({ recorded_solver_mode: mode, fresh_invocation_performed: false, current_use_eligible: false });
    expect(hasNativeMechanicsInvocation(result, model, mode)).toBe(false);
    const record = await buildAnalysisRunPreview(result, { inputManifest: await manifestFor(result, mode) });
    expect(record.schema_version).toBe("0.3.0");
    expect(result).toEqual(fixture);
  });

  it("defaults to exact sparse fixture and returns independent copies", async () => {
    const first = (await loadBundledMechanicsReference()).source;
    expect(first).toEqual(sparseMechanicsFixture);
    first.diagnostics.push({ id: "caller-mutation", code: "CALLER_MUTATION", severity: "info", message: "test mutation" });
    expect((await loadBundledMechanicsReference()).source).toEqual(sparseMechanicsFixture);
    expect((await loadBundledMechanicsReference("dense_scrutiny")).source).toEqual(denseMechanicsFixture);
  });

  it("compares model content canonically rather than object key insertion order", async () => {
    const model = await loadPreviewModel();
    const reordered = Object.fromEntries(Object.entries(model).reverse()) as PreviewModel;
    await expect(runPreviewMechanics(reordered)).rejects.toThrow("BROWSER_SOLVE_BACKEND_REQUIRED_REFERENCE_ONLY");
    expect((await loadBundledMechanicsReference()).source).toEqual(sparseMechanicsFixture);
    const edited = cloneModel(model);
    edited.nodes[0].position.y += 0.5;
    await expect(runPreviewMechanics(edited, "dense_scrutiny")).rejects.toThrow("BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL");
  });

  it("does not let a caller mutate the known fixture model comparison basis", async () => {
    const model = await loadPreviewModel();
    const originalY = model.nodes[0].position.y;
    model.nodes[0].position.y += 0.5;
    await expect(runPreviewMechanics(model)).rejects.toThrow("BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL");
    expect((await loadPreviewModel()).nodes[0].position.y).toBe(originalY);
  });

  it("rejects unknown runtime modes without invoking or choosing another fixture", async () => {
    const invalid = "unregistered_solver" as PreviewSolverMode;
    await expect(runPreviewMechanics(null, invalid)).rejects.toThrow("PREVIEW_SOLVER_MODE_UNSUPPORTED");
    await expect(startPreviewMechanicsJob(null, invalid)).rejects.toThrow("PREVIEW_SOLVER_MODE_UNSUPPORTED");
    await expect(loadBundledMechanicsReference(invalid)).rejects.toThrow("PREVIEW_SOLVER_MODE_UNSUPPORTED");
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("rejects crossed, missing and ambiguous actual mode evidence", async () => {
    // The bundled results bind the demo model (also the default session model),
    // never another model such as the invented preview model.
    const model = (await loadBundledMechanicsReference()).model, other = structuredClone(inventedPreviewModel) as unknown as PreviewModel;
    expect(model).toEqual(await loadPreviewModel());
    const sparse = structuredClone(sparseMechanicsFixture) as MechanicsResult;
    expect(() => validateBrowserMechanicsFixture(sparse, "sparse_interactive", other)).toThrow("BROWSER_FIXTURE_MODEL_BINDING_MISMATCH");
    expect(() => validateBrowserMechanicsFixture(sparse, "dense_scrutiny", model)).toThrow("BROWSER_FIXTURE_SOLVER_MODE_BINDING_MISMATCH");
    const modeRow = sparse.results.find(row => row.kind === "linear_solver_mode_basis");
    expect(modeRow).toBeDefined();
    for (const mutate of [
      (result: MechanicsResult) => { result.results = result.results.filter(row => row.kind !== "linear_solver_mode_basis"); },
      (result: MechanicsResult) => { result.results.push(structuredClone(modeRow!)); },
      (result: MechanicsResult) => { result.results.find(row => row.kind === "linear_solver_mode_basis")!.value = 2; },
      (result: MechanicsResult) => { result.results.find(row => row.kind === "linear_solver_mode_basis")!.metadata!.basis += "; solver_mode=dense_scrutiny"; },
    ]) {
      const changed = structuredClone(sparse);
      mutate(changed);
      expect(() => validateBrowserMechanicsFixture(changed, "sparse_interactive", model)).toThrow("BROWSER_FIXTURE_SOLVER_MODE_BINDING_MISMATCH");
    }
    expect(() => validateBrowserMechanicsFixture(structuredClone(historicalMechanicsFixture) as MechanicsResult, "sparse_interactive", model))
      .toThrow("BROWSER_FIXTURE_PRODUCER_CONTRACT_UNSUPPORTED");
  });

});

describe("appliedRuleCheckStatus (TP-C4-APPAGG-001)", () => {
  it("uses a recognized rule-check aggregate over the solve rule_check", () => {
    expect(
      appliedRuleCheckStatus("RULE_INPUTS_INCOMPLETE", "USER_RULE_FAILED"),
    ).toBe("USER_RULE_FAILED");
    expect(
      appliedRuleCheckStatus("RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED"),
    ).toBe("USER_RULE_CHECKED");
    expect(
      appliedRuleCheckStatus("USER_RULE_CHECKED", "RULE_INPUTS_INCOMPLETE"),
    ).toBe("RULE_INPUTS_INCOMPLETE");
  });

  it("falls back to the solve rule_check for an absent or unrecognized aggregate (no silent coercion)", () => {
    expect(appliedRuleCheckStatus("RULE_INPUTS_INCOMPLETE")).toBe(
      "RULE_INPUTS_INCOMPLETE",
    );
    expect(appliedRuleCheckStatus("RULE_INPUTS_INCOMPLETE", null)).toBe(
      "RULE_INPUTS_INCOMPLETE",
    );
    expect(appliedRuleCheckStatus("RULE_INPUTS_INCOMPLETE", "")).toBe(
      "RULE_INPUTS_INCOMPLETE",
    );
    // A non-rule-check status string is never trusted (no false pass).
    expect(
      appliedRuleCheckStatus(
        "RULE_INPUTS_INCOMPLETE",
        "HUMAN_APPROVED_FOR_PROJECT",
      ),
    ).toBe("RULE_INPUTS_INCOMPLETE");
  });
});

// These synthetic/reference analysis contracts do not establish native Current.
describe("buildAnalysisRunPreview reference rule-check aggregate (TP-C4-APPAGG-001)", () => {
  it("defaults to the solve envelope rule_check when no aggregate is supplied", async () => {
    const env = await buildRun(solvedResultStub);
    expect(env.analysis_run.analysis_status).toContain(
      "RULE_INPUTS_INCOMPLETE",
    );
    expect(env.analysis_run.analysis_status).not.toContain("USER_RULE_FAILED");
  });

  it("drives a recognized aggregate into analysis_status and the run-record hash, leaving the solve-envelope hash byte-stable", async () => {
    const base = await buildRun(solvedResultStub);
    const withFail = await buildRun(solvedResultStub, "USER_RULE_FAILED");

    expect(withFail.analysis_run.analysis_status).toContain("USER_RULE_FAILED");
    expect(withFail.analysis_run.analysis_status).not.toContain(
      "RULE_INPUTS_INCOMPLETE",
    );
    // The raw solve envelope is never mutated: its hash is byte-identical.
    expect(hashByScope(withFail, "received_result")).toBe(
      hashByScope(base, "received_result"),
    );
    // The analysis-run record honestly binds the rule-check outcome: hash differs.
    expect(hashByScope(withFail, "analysis_run_record")).not.toBe(
      hashByScope(base, "analysis_run_record"),
    );
  });

  it("records a USER_RULE_CHECKED aggregate likewise", async () => {
    const env = await buildRun(solvedResultStub, "USER_RULE_CHECKED");
    expect(env.analysis_run.analysis_status).toContain("USER_RULE_CHECKED");
    expect(env.analysis_run.analysis_status).not.toContain(
      "RULE_INPUTS_INCOMPLETE",
    );
  });

  it("ignores an unrecognized aggregate, reproducing the no-aggregate envelope exactly (no false pass)", async () => {
    const base = await buildRun(solvedResultStub);
    const bogus = await buildRun(
      solvedResultStub,
      "HUMAN_APPROVED_FOR_PROJECT",
    );
    expect(bogus.analysis_run.analysis_status).toContain(
      "RULE_INPUTS_INCOMPLETE",
    );
    expect(bogus.analysis_run.analysis_status).not.toContain(
      "HUMAN_APPROVED_FOR_PROJECT",
    );
    expect(hashByScope(bogus, "analysis_run_record")).toBe(
      hashByScope(base, "analysis_run_record"),
    );
  });
});

describe("reference analysis-run input-manifest and source-dimension binding", () => {
  it("binds the exact manifest independently from the result envelope and declares stiffness dimensions", async () => {
    const result = await referenceFixtureSource();
    const manifest = await manifestFor(result);
    const env = await buildAnalysisRunPreview(result, {
      inputManifest: manifest,
    });
    const resultEnvelopeHash = hashByScope(env, "received_result");
    const byId = new Map(
      env.analysis_run.result_refs.map((item) => [
        item.result_ref.ref,
        item.source_dimension,
      ]),
    );

    expect(env.analysis_run.reproducibility.input_manifest_refs).toEqual([
      manifest.manifest_ref,
    ]);
    expect(
      env.analysis_run.reproducibility.input_manifest_hashes,
    ).toEqual([
      expect.objectContaining({
        payload_ref: manifest.manifest_ref,
        payload_scope: "input_manifest",
        value: manifest.manifest_sha256,
      }),
    ]);
    expect(manifest.manifest_sha256).toMatch(/^[0-9a-f]{64}$/);
    expect(manifest.manifest_sha256).not.toBe(resultEnvelopeHash);
    // The demo's stiffness rows are its spring hanger's (it has no joint).
    expect(
      byId.get(
        "result:spring-hanger:support-SH-140:stiffness",
      ),
    ).toBe("linear_stiffness");
    expect([...byId.keys()].some((id) => id.includes("component-stiffness"))).toBe(false);
  });

  it("blocks manifest evidence for a different model", async () => {
    const result = await referenceFixtureSource();
    const manifest = await manifestFor(result);
    const mismatched = structuredClone(result);
    mismatched.model_ref = "project:different-model";

    await expect(
      buildAnalysisRunPreview(mismatched, { inputManifest: manifest }),
    ).rejects.toThrow("ANALYSIS-RUN-INPUT-MANIFEST-MODEL-MISMATCH");
  });

  it("blocks transformed manifest evidence until its ref and hash are recomputed", async () => {
    const result = await referenceFixtureSource();
    const manifest = await manifestFor(result);
    manifest.manifest.solver_basis.settings.sparse_evidence_lane = false;

    await expect(
      buildAnalysisRunPreview(result, { inputManifest: manifest }),
    ).rejects.toThrow("INPUT-MANIFEST-HASH-MISMATCH");
  });

  it("blocks wrong-prefix and wrong-model manifest refs with a valid digest", async () => {
    const result = await referenceFixtureSource();
    const manifest = await manifestFor(result);
    for (const refValue of [
      `result-envelope:project-invented-demo-loop-01:${manifest.manifest_sha256}`,
      `input-manifest:project-different:${manifest.manifest_sha256}`,
    ]) {
      const invalid = structuredClone(manifest);
      invalid.manifest_ref.ref = refValue;
      await expect(
        buildAnalysisRunPreview(result, { inputManifest: invalid }),
      ).rejects.toThrow("INPUT-MANIFEST-HASH-MISMATCH");
    }
  });

  it("blocks a unit that contradicts the exact kind semantics", async () => {
    const result = structuredClone(await referenceFixtureSource());
    const target = result.results.find(
      (item) => item.kind === "element_local_axial_force",
    );
    expect(target).toBeDefined();
    target!.unit = "MPa";
    // The bundled reference is preview-physics-1, so its reader refuses the
    // changed unit before the kind-semantics check (SOURCE_UNIT_CONTRADICTION,
    // still pinned on a precision-1 source in reportPackageRequest.test.ts).
    // R2: the Rust reader's code (TS: SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID).
    await expect(buildAnalysisRunPreview(result, {
      inputManifest: await manifestFor(result),
    })).rejects.toThrow("SOURCE_PREVIEW_PHYSICS_ROW_SIGNATURE");
  });

  it("blocks an explicit dimension that contradicts exact result kind semantics", async () => {
    const result = structuredClone(await referenceFixtureSource());
    const target = result.results.find(
      (item) => item.kind === "element_local_axial_force",
    );
    expect(target).toBeDefined();
    target!.dimension = "stress";

    await expect(
      buildAnalysisRunPreview(result, {
        inputManifest: await manifestFor(result),
      }),
    ).rejects.toThrow("ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH");
  });
});


describe("native loader fallback compatibility", () => {
  it("retains the model fixture after native model loader rejection", async () => {
    const fixture = await loadPreviewModel();
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockRejectedValue(new Error("invented native model loader failure"));
    await expect(loadPreviewModel()).resolves.toEqual(fixture);
    expect(invokeMock).toHaveBeenCalledExactlyOnceWith("load_preview_model", undefined);
  });

  it("retains the knowledge fixture after native knowledge loader rejection", async () => {
    const fixture = await loadDesignKnowledge();
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockRejectedValue(new Error("invented native knowledge loader failure"));
    await expect(loadDesignKnowledge()).resolves.toEqual(fixture);
    expect(invokeMock).toHaveBeenCalledExactlyOnceWith("load_design_knowledge", undefined);
  });
});

/** The model and solver mode a start command carried: the UTF-8 JSON body and the mode header. */
function startRequest(call: unknown[]): { model: unknown; text: string; mode: unknown } {
  const [command, body, options] = call as [string, Uint8Array, { headers: Record<string, string> }];
  expect(command).toBe("start_preview_mechanics_job");
  expect(ArrayBuffer.isView(body)).toBe(true);
  const text = new TextDecoder("utf-8", { fatal: true }).decode(body);
  return { model: JSON.parse(text), text, mode: options.headers["x-swbpipe-solver-mode"] };
}
const startCalls = () => invokeMock.mock.calls.filter(([command]) => command === "start_preview_mechanics_job");

describe("native mechanics failure boundary", () => {
  it("sends the captured model as raw UTF-8 bytes with the exact requested mode header", async () => {
    const { model, source } = nativeMechanicsReplayPair("dense_scrutiny");
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(createNativeMechanicsReplay().invoke);
    expect(await runPreviewMechanics(model, "dense_scrutiny")).toEqual(source);
    expect(startCalls()).toHaveLength(1);
    const sent = startRequest(startCalls()[0]);
    expect(sent.mode).toBe("dense_scrutiny");
    expect(sent.model).toEqual(model);
    expect(sent.text).toBe(checkedJsonText(model));
    // No JSON-args model and no budget/pool policy travel with the model document.
    expect(sent.text).not.toMatch(/budget|pool_bytes/);
    expect(invokeMock.mock.calls.map(([command]) => command)).toEqual(["start_preview_mechanics_job", "poll_preview_mechanics_job", "take_preview_mechanics_job_result"]);
  });
  it("propagates native job-start rejection without a bundled result", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const failure = new Error("invented native IPC solve failure");
    invokeMock.mockRejectedValue(failure);
    await expect(runPreviewMechanics(nativeMechanicsReplayPair().model)).rejects.toBe(failure);
    await expect(startPreviewMechanicsJob(nativeMechanicsReplayPair().model)).rejects.toBe(failure);
    expect(invokeMock).toHaveBeenCalledTimes(2);
  });

  it("refuses a native solve without a model document before any IPC", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    await expect(runPreviewMechanics()).rejects.toThrow("PREVIEW-SOLVE-MODEL-REQUIRED");
    await expect(startPreviewMechanicsJob(null)).rejects.toThrow("PREVIEW-SOLVE-MODEL-REQUIRED");
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("refuses browser job start rather than inventing a backend job", async () => {
    await expect(startPreviewMechanicsJob()).rejects.toThrow("BROWSER_SOLVE_BACKEND_REQUIRED_REFERENCE_ONLY");
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("reads a typed start refusal as ResourceUnavailableError carrying the refusal and pool", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const pool = { capacity_bytes: 16 * 2 ** 30, recommended_pool_bytes: 12 * 2 ** 30, pool_bytes: 12 * 2 ** 30, pool_above_recommended: false,
      invocation_budget_bytes: 12 * 2 ** 30, qualified_invocation_budget_bytes: 12 * 2 ** 30, reserved_bytes: 0,
      reserved_by_purpose: { ingress: 0, result: 0, custody: 0 }, unpriced_solve_running: false, queued_jobs: 0, running_jobs: 0,
      retained_results: 0, webview_custody_results: 0, headroom: { available_bytes: 2 * 2 ** 30, pressure: "normal", sample_age_ms: 20 },
      ordinary_route_pricing: "unpriced", last_refusal: { kind: "headroom_insufficient", required_bytes: 3 * 2 ** 30, available_bytes: 2 * 2 ** 30 } };
    invokeMock.mockRejectedValue({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_insufficient", required_bytes: 3 * 2 ** 30, available_bytes: 2 * 2 ** 30 }, pool });
    const failure = await runPreviewMechanics(nativeMechanicsReplayPair().model).catch((error: unknown) => error);
    expect(failure).toBeInstanceOf(ResourceUnavailableError);
    expect((failure as ResourceUnavailableError).refusal).toEqual({ kind: "headroom_insufficient", required_bytes: 3 * 2 ** 30, available_bytes: 2 * 2 ** 30 });
    expect((failure as ResourceUnavailableError).pool?.pool_bytes).toBe(12 * 2 ** 30);
    expect((failure as Error).message).toBe("RESOURCE_UNAVAILABLE: Not enough free memory on this Mac right now: needs 3 GiB, 2 GiB available.");
    invokeMock.mockRejectedValue({ code: "INVALID_REQUEST", message: "solver mode header missing" });
    await expect(runPreviewMechanics(nativeMechanicsReplayPair().model)).rejects.toThrow("INVALID_REQUEST: solver mode header missing");
  });

  it("rejects a refused terminal job with its typed refusal and takes nothing", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const pair = nativeMechanicsReplayPair(), replay = createNativeMechanicsReplay();
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
      const answer = await replay.invoke(command, args, options);
      return command === "poll_preview_mechanics_job"
        ? { ...answer, state: "refused", result_available: false, result_bytes: null, resource_refusal: { kind: "pressure_elevated", level: "critical" } }
        : answer;
    });
    const failure = await runPreviewMechanics(pair.model).catch((error: unknown) => error);
    expect(failure).toBeInstanceOf(ResourceUnavailableError);
    expect((failure as ResourceUnavailableError).refusal).toEqual({ kind: "pressure_elevated", level: "critical" });
    expect(invokeMock.mock.calls.some(([command]) => command === "take_preview_mechanics_job_result")).toBe(false);
  });

  it("reports the queue reason of a waiting job before it completes", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const pair = nativeMechanicsReplayPair(), replay = createNativeMechanicsReplay();
    let polls = 0;
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
      const answer = await replay.invoke(command, args, options);
      if (command !== "poll_preview_mechanics_job" || ++polls > 1) return answer;
      return { ...answer, state: "queued", result_available: false, result_bytes: null, queue_reason: { kind: "solve_slot_busy" } };
    });
    const seen: Array<[string, unknown]> = [];
    const result = await runPreviewMechanics(pair.model, "sparse_interactive", { onStatus: (status) => seen.push([status.state, status.queue_reason]) });
    expect(seen).toEqual([["queued", { kind: "solve_slot_busy" }], ["completed", null]]);
    expect(hasNativeMechanicsInvocation(result, pair.model)).toBe(true);
  });
});

describe("delivered result custody", () => {
  it("disposes a delivered result once, by its job, and ignores results it did not deliver", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const pair = nativeMechanicsReplayPair(), replay = createNativeMechanicsReplay();
    invokeMock.mockImplementation(replay.invoke);
    const result = await runPreviewMechanics(pair.model);
    const jobId = deliveredMechanicsResultJob(result);
    expect(jobId).toEqual(expect.any(String));
    expect(replay.custody()).toEqual([jobId]);
    expect(await disposeDeliveredMechanicsResult(structuredClone(result))).toBe(false);
    expect(await disposeDeliveredMechanicsResult(result)).toBe(true);
    expect(await disposeDeliveredMechanicsResult(result)).toBe(false);
    expect(invokeMock.mock.calls.filter(([command]) => command === "dispose_preview_mechanics_result")).toEqual([["dispose_preview_mechanics_result", { jobId }]]);
    expect(replay.custody()).toEqual([]);
    expect(deliveredMechanicsResultJob(result)).toBeNull();
    // Disposal releases native custody only; the held value and its registration are unchanged.
    expect(hasNativeMechanicsInvocation(result, pair.model)).toBe(true);
  });

  it("keeps the capture when the take is refused (custody_full) so a later take registers", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const pair = nativeMechanicsReplayPair(), replay = createNativeMechanicsReplay();
    let refuseTake = true;
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
      if (command === "take_preview_mechanics_job_result" && refuseTake) {
        refuseTake = false;
        throw { code: "RESOURCE_UNAVAILABLE", refusal: { kind: "custody_full", delivered: 4, limit: 4 }, pool: null };
      }
      return replay.invoke(command, args, options);
    });
    const failure = await runPreviewMechanics(pair.model).catch((error: unknown) => error);
    expect(failure).toBeInstanceOf(ResourceUnavailableError);
    expect((failure as ResourceUnavailableError).refusal).toEqual({ kind: "custody_full", delivered: 4, limit: 4 });
    expect(startCalls()).toHaveLength(1);
    const { job_id } = await (invokeMock.mock.results[0].value as Promise<{ job_id: string }>);
    // The result stayed with the native host; the app takes it once custody allows.
    const taken = await takePreviewMechanicsJobResult(job_id);
    expect(taken).toEqual(pair.source);
    expect(hasNativeMechanicsInvocation(taken, pair.model)).toBe(true);
  });

  it("discards a superseded completed job's result without registering it", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const pair = nativeMechanicsReplayPair(), replay = createNativeMechanicsReplay();
    invokeMock.mockImplementation(replay.invoke);
    const start = await startPreviewMechanicsJob(pair.model);
    if (start.mode !== "backend_job") throw new Error("expected simulated backend job");
    expect(start.resource).toEqual({ invocation_budget_bytes: 12 * 2 ** 30, pool_bytes: 12 * 2 ** 30, route_pricing: "ordinary_unpriced", ingress_reserved_bytes: expect.any(Number) });
    expect(await discardPreviewMechanicsJobResult(start.job_id)).toBe(true);
    // Released natively without a copy to the app: no take.
    expect(invokeMock.mock.calls.map(([command]) => command)).toEqual(["start_preview_mechanics_job", "dispose_preview_mechanics_result"]);
    expect(invokeMock).toHaveBeenLastCalledWith("dispose_preview_mechanics_result", { jobId: start.job_id });
    // The retired capture can never register a later delivery.
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) =>
      command === "take_preview_mechanics_job_result" ? resultArrayBuffer(pair.bytes) : replay.invoke(command, args, options));
    expect(hasNativeMechanicsInvocation(await takePreviewMechanicsJobResult(start.job_id), pair.model)).toBe(false);
  });
});

it("does not enrich precision or unsupported raw0.2 carriers while reading history", () => {
 const raw = {schema_version:"0.2.0",results:[{id:"tiny",kind:"global_nodal_rotation_x",value:-1e-12,unit:"rad",entity_ref:"n"}]} as MechanicsResult;
 expect(bindSourceResultDimensions(raw)).toBe(raw);
 expect(Object.hasOwn(raw.results[0], "dimension")).toBe(false);
 expect(raw.results[0].value).toBe(-1e-12);
});

async function referenceFixtureSource(): Promise<MechanicsResult> {
 return (await loadBundledMechanicsReference()).source;
}
it("analysis composition refuses legacy and unknown raw without a historical fallback",async()=>{
 const legacy=structuredClone(historicalMechanicsFixture) as MechanicsResult, before=JSON.stringify(legacy);
 await expect(buildAnalysisRunPreview(legacy,{inputManifest:await manifestFor(legacy)})).rejects.toThrow("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
 const unknown={...legacy,schema_version:"0.2.0"};
 await expect(buildAnalysisRunPreview(unknown,{inputManifest:await manifestFor(unknown)})).rejects.toThrow("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
 expect(JSON.stringify(legacy)).toBe(before);
});

it("uses verified model requested bases even when result rows have no basis", async () => {
  const raw = structuredClone(solvedResultStub);
  const inputManifest = await manifestFor(raw);
  const model = inputManifest.manifest.model_basis.model_payload;
  const record = await buildAnalysisRunPreview(raw, { inputManifest });
  expect(record.analysis_run.load_basis_refs).toEqual([
    ...model.load_cases.map(c => ({ object_type: "LoadCase", ref: c.id })),
    ...(model.combinations ?? []).map(c => ({ object_type: "Combination", ref: c.id })),
  ]);
  expect(record.analysis_run.load_basis_refs.length).toBeGreaterThan(0);
});

/** These exercise the application's private provenance boundary through mocked
 * IPC. The underlying model/result pairs are genuinely captured producer data;
 * these tests are NOT an actual native UI solve/inspection qualification. */
describe("native invocation provenance under bounded unit transport replay", () => {
  function installReplay() {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const replay = createNativeMechanicsReplay();
    invokeMock.mockImplementation(replay.invoke);
    return replay;
  }
  it.each(["sparse_interactive", "dense_scrutiny"] as const)("registers only the exact returned %s object and retains a defensive request copy", async mode => {
    const pair = nativeMechanicsReplayPair(mode), before = JSON.stringify(pair);
    expect(pair.provenance.actual_native_ui_qualification).toBe(false);
    expect(pair.provenance.notice).toContain("NOT actual native UI qualification");
    expect(hasNativeMechanicsInvocation(pair.source, pair.model, mode)).toBe(false);
    installReplay();
    const result = await runPreviewMechanics(pair.model, mode);
    expect(result).toEqual(pair.source);
    expect(hasNativeMechanicsInvocation(result, pair.model, mode)).toBe(true);
    const invocation = retainedNativeMechanicsInvocation(result, pair.model);
    expect(invocation).toEqual({ request: { model: pair.model, materials: [] }, solver_mode: mode });
    invocation!.request.model.nodes[0].position.x += 1;
    expect(retainedNativeMechanicsInvocation(result, pair.model)).toEqual({ request: { model: pair.model, materials: [] }, solver_mode: mode });
    for (const received of [pair.source, structuredClone(result), JSON.parse(JSON.stringify(result)) as MechanicsResult]) {
      expect(hasNativeMechanicsInvocation(received, pair.model, mode)).toBe(false);
      expect(retainedNativeMechanicsInvocation(received, pair.model)).toBeNull();
    }
    expect(JSON.stringify(pair)).toBe(before);
  });

  it("keeps explicitly imported reference data unregistered even after analysis/hash construction", async () => {
    const model = await loadPreviewModel(), reference = await loadBundledMechanicsReference();
    await buildRun(reference.source);
    expect(hasNativeMechanicsInvocation(reference.source, model)).toBe(false);
    expect(retainedNativeMechanicsInvocation(reference.source, model)).toBeNull();
    expect(reference.source).toEqual(sparseMechanicsFixture);
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("does not register a delivered result that answers an uncaptured (unchecked) model", async () => {
    const pair = nativeMechanicsReplayPair();
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const transport = createSolveJobTransport(() => pair.bytes);
    invokeMock.mockImplementation(transport.invoke);
    // A non-finite number cannot be captured as checked JSON: the model still travels, nothing registers.
    const unchecked = structuredClone(pair.model); unchecked.nodes[0].position.x = Number.NaN;
    const received = await runPreviewMechanics(unchecked);
    expect(received).toEqual(pair.source);
    expect(hasNativeMechanicsInvocation(received, pair.model)).toBe(false);
    expect(retainedNativeMechanicsInvocation(received, pair.model)).toBeNull();
  });

  it("rejects source, same-ID model and solver-mode changes, including a signed-zero-only source change", async () => {
    const pair = nativeMechanicsReplayPair(); installReplay();
    const result = await runPreviewMechanics(pair.model);
    expect(hasNativeMechanicsInvocation(result, pair.model, "dense_scrutiny")).toBe(false);
    const changed = structuredClone(pair.model); changed.nodes[0].position.x += 0.25;
    expect(hasNativeMechanicsInvocation(result, changed)).toBe(false);
    const zero = result.results.find(row => Object.is(row.value, 0));
    expect(zero).toBeDefined();
    const jsonBefore = JSON.stringify(result); zero!.value = -0;
    expect(JSON.stringify(result)).toBe(jsonBefore); // JCS/text alone cannot witness this mutation.
    expect(hasNativeMechanicsInvocation(result, pair.model)).toBe(false);
    expect(retainedNativeMechanicsInvocation(result, pair.model)).toBeNull();
    const second = await runPreviewMechanics(pair.model);
    second.results[0].unit = "tampered-unit";
    expect(hasNativeMechanicsInvocation(second, pair.model)).toBe(false);
  });

  it("preserves the submitted model and refuses fresh standing after the caller changes while transport is pending", async () => {
    const pair = nativeMechanicsReplayPair(), expectedModel = structuredClone(pair.model);
    const replay = installReplay();
    let release!: () => void;
    const gate = new Promise<void>(resolve => { release = resolve; });
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
      const response = await replay.invoke(command, args, options);
      await gate;
      return response;
    });
    const pending = runPreviewMechanics(pair.model);
    pair.model.nodes[0].position.x += 0.5;
    release(); const result = await pending;
    // FE02 binds the live caller separately from the immutable wire capture.
    // The received response remains inspectable, but a stale completion cannot
    // acquire fresh standing even when queried using a copy of the old model.
    expect(hasNativeMechanicsInvocation(result, expectedModel)).toBe(false);
    expect(hasNativeMechanicsInvocation(result, pair.model)).toBe(false);
    expect(startRequest(invokeMock.mock.calls[0]).model).toEqual(expectedModel);
  });

  it("sends a byte copy of the capture: a transport that rewrites the sent bytes cannot reach the captured invocation", async () => {
    const pair = nativeMechanicsReplayPair(), replay = installReplay();
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
      const response = await replay.invoke(command, args, options);
      if (command === "start_preview_mechanics_job") (args as Uint8Array).fill(0x20);
      return response;
    });
    const result = await runPreviewMechanics(pair.model);
    expect(result).toEqual(pair.source);
    expect(retainedNativeMechanicsInvocation(result, pair.model)).toEqual({ request: { model: pair.model, materials: [] }, solver_mode: "sparse_interactive" });
  });

  it("the replay refuses a different model, omitted request, wrong mode and unknown commands/jobs", async () => {
    const pair = nativeMechanicsReplayPair(), replay = installReplay();
    const changed = structuredClone(pair.model); changed.materials![0].elastic_modulus.value += 1;
    await expect(runPreviewMechanics(changed)).rejects.toThrow("NATIVE_REPLAY_MODEL_MISMATCH");
    await expect(replay.invoke("start_preview_mechanics_job", { model: pair.model }, { headers: { "x-swbpipe-solver-mode": "sparse_interactive" } })).rejects.toThrow("NATIVE_REPLAY_REQUEST_INVALID");
    await expect(replay.invoke("start_preview_mechanics_job", new TextEncoder().encode(checkedJsonText(pair.model)))).rejects.toThrow("NATIVE_REPLAY_REQUEST_INVALID");
    await expect(replay.invoke("start_preview_mechanics_job", new TextEncoder().encode(checkedJsonText(pair.model)), { headers: { "x-swbpipe-solver-mode": "unregistered" } })).rejects.toThrow("NATIVE_REPLAY_MODE_UNSUPPORTED");
    await expect(replay.invoke("run_preview_mechanics_with_solver_mode", { model: pair.model, solverMode: "sparse_interactive" })).rejects.toThrow("NATIVE_REPLAY_COMMAND_UNSUPPORTED");
    await expect(replay.invoke("not-an-engine-command")).rejects.toThrow("NATIVE_REPLAY_COMMAND_UNSUPPORTED");
    await expect(pollPreviewMechanicsJob("unknown-replay-job")).rejects.toThrow("NATIVE_REPLAY_JOB_UNKNOWN");
  });

  it("registers the first taken result of a completed known job, never a poll or an unknown job's bytes", async () => {
    const pair = nativeMechanicsReplayPair("dense_scrutiny"), replay = installReplay();
    const start = await startPreviewMechanicsJob(pair.model, pair.mode);
    expect(start.mode).toBe("backend_job");
    if (start.mode !== "backend_job") throw new Error("expected simulated backend job");
    const first = await pollPreviewMechanicsJob(start.job_id);
    expect(first.state).toBe("completed");
    expect(first).not.toHaveProperty("result");
    expect(first.result_available).toBe(true);
    const taken = await takePreviewMechanicsJobResult(start.job_id);
    expect(taken).toEqual(pair.source);
    expect(hasNativeMechanicsInvocation(taken, pair.model, pair.mode)).toBe(true);
    // A second take is refused by the native host; no duplicate object is minted.
    await expect(takePreviewMechanicsJobResult(start.job_id)).rejects.toThrow("NATIVE_REPLAY_RESULT_UNAVAILABLE");
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) =>
      command === "take_preview_mechanics_job_result" ? resultArrayBuffer(pair.bytes) : replay.invoke(command, args, options));
    const unknown = await takePreviewMechanicsJobResult("unknown-terminal");
    expect(unknown).toEqual(pair.source);
    expect(hasNativeMechanicsInvocation(unknown, pair.model, pair.mode)).toBe(false);
    expect(hasNativeMechanicsInvocation(taken, pair.model, pair.mode)).toBe(true);
  });

  it("does not register anything from a running poll before the completed take", async () => {
    const pair = nativeMechanicsReplayPair(), replay = installReplay();
    const start = await startPreviewMechanicsJob(pair.model);
    if (start.mode !== "backend_job") throw new Error("expected simulated backend job");
    const terminal = await replay.invoke("poll_preview_mechanics_job", { jobId: start.job_id });
    invokeMock.mockResolvedValueOnce({ ...terminal, state: "running", result_available: false, result_bytes: null });
    const running = await pollPreviewMechanicsJob(start.job_id);
    expect(running.state).toBe("running");
    expect(running).not.toHaveProperty("result");
    invokeMock.mockImplementation(replay.invoke);
    expect((await pollPreviewMechanicsJob(start.job_id)).state).toBe("completed");
    expect(hasNativeMechanicsInvocation(await takePreviewMechanicsJobResult(start.job_id), pair.model)).toBe(true);
  });

  it.each(["cancelled", "failed", "refused"] as const)("retires the capture of a %s job so later delivered bytes never register", async state => {
    const pair = nativeMechanicsReplayPair(), replay = installReplay();
    const start = await startPreviewMechanicsJob(pair.model);
    if (start.mode !== "backend_job") throw new Error("expected simulated backend job");
    const terminal = await replay.invoke("poll_preview_mechanics_job", { jobId: start.job_id });
    invokeMock.mockResolvedValueOnce({ ...terminal, state, result_available: false, result_bytes: null, resource_refusal: state === "refused" ? { kind: "queue_full", queued: 8, limit: 8 } : null });
    const stopped = await pollPreviewMechanicsJob(start.job_id);
    expect(stopped.state).toBe(state);
    if (state === "refused") expect(stopped.resource_refusal).toEqual({ kind: "queue_full", queued: 8, limit: 8 });
    invokeMock.mockImplementation(replay.invoke);
    const late = await takePreviewMechanicsJobResult(start.job_id);
    expect(late).toEqual(pair.source);
    expect(hasNativeMechanicsInvocation(late, pair.model)).toBe(false);
  });

  it("invalidates an accepted cancellation before a late completed take", async () => {
    const pair = nativeMechanicsReplayPair(), replay = installReplay();
    const start = await startPreviewMechanicsJob(pair.model);
    if (start.mode !== "backend_job") throw new Error("expected simulated backend job");
    const cancellation = await cancelPreviewMechanicsJob(start.job_id, start.backend_cancellation_token);
    expect(cancellation.accepted).toBe(true);
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) =>
      command === "take_preview_mechanics_job_result" ? resultArrayBuffer(pair.bytes) : replay.invoke(command, args, options));
    const returned = await takePreviewMechanicsJobResult(start.job_id);
    expect(returned).toEqual(pair.source);
    expect(hasNativeMechanicsInvocation(returned, pair.model)).toBe(false);
  });

  it("invalidates a mismatched terminal identity and cannot revive the original job capture", async () => {
    const pair = nativeMechanicsReplayPair(), replay = installReplay();
    const start = await startPreviewMechanicsJob(pair.model);
    if (start.mode !== "backend_job") throw new Error("expected simulated backend job");
    const terminal = await replay.invoke("poll_preview_mechanics_job", { jobId: start.job_id });
    invokeMock.mockResolvedValueOnce({ ...terminal, job_id: `${start.job_id}:wrong` });
    await expect(pollPreviewMechanicsJob(start.job_id)).rejects.toThrow("SOLVE-JOB-IDENTITY-MISMATCH");
    invokeMock.mockImplementation(replay.invoke);
    const later = await takePreviewMechanicsJobResult(start.job_id);
    expect(hasNativeMechanicsInvocation(later, pair.model)).toBe(false);
  });
});


it.each(["sparse_interactive", "dense_scrutiny"] as const)("explicit ordinary precision replay keeps its captured %s source and rejects the physics model", async mode => {
  const pair = nativeMechanicsReplayPair(mode, { profile: "precision" });
  expect(pair.provenance.capture_ref).toContain("PRECISION_UI_COMPANION.json");
  expect(pair.source.producer!.semantic_contract_id).toBe(PRECISION_CONTRACT_ID);
  expect(pair.source.results).toHaveLength(mode === "sparse_interactive" ? 67 : 68);
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const replay = createNativeMechanicsReplay({ profile: "precision" });
  invokeMock.mockImplementation(replay.invoke);
  const received = await runPreviewMechanics(pair.model, mode);
  expect(received).toEqual(pair.source);
  expect(hasNativeMechanicsInvocation(received, pair.model, mode)).toBe(true);
  await expect(runPreviewMechanics(nativeMechanicsReplayPair(mode).model, mode)).rejects.toThrow("NATIVE_REPLAY_MODEL_MISMATCH");
});

it.each(["sparse_interactive", "dense_scrutiny"] as const)("preview-physics-1 replay registers its %s source only when the closed reader checks pass", async mode => {
  const pair = nativeMechanicsReplayPair(mode, { profile: "preview" });
  expect(pair.provenance.source_fixture_ref).toBe(`fixtures/results/preview_physics_connected_${mode === "sparse_interactive" ? "sparse" : "dense"}.json`);
  expect(sourceContract(pair.source)).toBe("preview_physics");
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const replay = createNativeMechanicsReplay({ profile: "preview" });
  invokeMock.mockImplementation(replay.invoke);
  const received = await runPreviewMechanics(pair.model, mode);
  expect(received).toEqual(pair.source);
  expect(hasNativeMechanicsInvocation(received, pair.model, mode)).toBe(true);
  expect(numericalResultStanding(received, pair.model).eligible).toBe(true);
  // A dangling result: reference fails the preview-evidence check, so no registration is minted.
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    const answer = await replay.invoke(command, args, options);
    if (command !== "take_preview_mechanics_job_result") return answer;
    const edited = JSON.parse(new TextDecoder().decode(answer));
    edited.diagnostics[0].affected_refs = [...(edited.diagnostics[0].affected_refs ?? []), "result:reaction:support-fixture-root"];
    return resultArrayBuffer(solveBytes(edited));
  });
  const tampered = await runPreviewMechanics(pair.model, mode);
  expect(hasNativeMechanicsInvocation(tampered, pair.model, mode)).toBe(false);
  // R2: the Rust reader's code (TS: SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: dangling result reference).
  expect(numericalResultStanding(tampered, pair.model).findings.join(" ")).toContain("SOURCE_PREVIEW_PHYSICS_DANGLING_RESULT_REF");
});
