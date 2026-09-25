import historicalMechanicsFixture from "../../../../fixtures/product_preview/invented_mechanics_result.json";
import sparseMechanicsFixture from "../../../../fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json";
import denseMechanicsFixture from "../../../../fixtures/product_preview/invented_mechanics_result_precision_1_dense.json";
import { buildAnalysisRunV02 } from "./analysisRunCompatibility";
import { numericalResultStanding, sourceContract, PRECISION_CONTRACT_ID } from "../features/results/numericalResultQuality";
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
  runPreviewMechanics,
  startPreviewMechanicsJob,
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

describe("previewService browser fixture execution boundary", () => {
  it("returns the actual bundled producer result when no model payload is supplied", async () => {
    const result = await runPreviewMechanics();

    expect(result.status.mechanics).toBe(sparseMechanicsFixture.status.mechanics);
    expect(result.model_ref).toBe("project:invented-loop-01");
    expect(result.results.length).toBe(sparseMechanicsFixture.results.length);
  });

  it("keeps the genuine fixture result for the unchanged bundled model", async () => {
    const model = await loadPreviewModel();
    const result = await runPreviewMechanics(model);

    expect(result.status.mechanics).toBe(sparseMechanicsFixture.status.mechanics);
    expect(result.model_ref).toBe("project:invented-loop-01");
    expect(result.results.length).toBe(sparseMechanicsFixture.results.length);
  });

  it("preserves historical rounded nonlinear fixture evidence without calling it a fresh solve", async () => {
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
      value: 0.411203,
    });
    const frictionNormal = result.results.find(
      (item) =>
        item.id ===
        "result:nonlinear-support:support-NL-130-FRIC:friction-normal-reaction",
    );
    expect(frictionNormal).toMatchObject({
      kind: "nonlinear_support_friction_normal_reaction_derived",
      entity_ref: "support:NL-130-FRIC",
      value: 41.120279,
      unit: "N",
    });
    expect(frictionReaction?.value).toBe(
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
    expect(original.project.id).toBe("project:invented-loop-01");
    expect(original.materials![0].elastic_modulus.value).toBe(200_000_000_000);
  });

  it.each([
    ["sparse_interactive", sparseMechanicsFixture],
    ["dense_scrutiny", denseMechanicsFixture],
  ] as const)("returns genuine %s fixture bytes and preserves producer quality", async (mode, fixture) => {
    const model = await loadPreviewModel();
    const result = await runPreviewMechanics(model, mode);
    expect(result).toEqual(fixture);
    expect(sourceContract(result)).toBe("precision");
    expect(result.numerical_quality).toEqual(fixture.numerical_quality);
    const current = numericalResultStanding(result, model);
    if (!["checks_passed", "sensitive"].includes(fixture.numerical_quality.status)) {
      expect(current.eligible).toBe(false);
      expect(current.findings).toContain("NUMERICAL_INTEGRITY_NOT_QUALIFIED");
    } else {
      expect(current.eligible).toBe(true);
    }
    const record = await buildAnalysisRunPreview(result, { inputManifest: await manifestFor(result, mode) });
    expect(record.schema_version).toBe("0.3.0");
    expect(result).toEqual(fixture);
  });

  it("defaults to exact sparse fixture and returns independent copies", async () => {
    const first = await runPreviewMechanics();
    expect(first).toEqual(sparseMechanicsFixture);
    first.diagnostics.push({ id: "caller-mutation", code: "CALLER_MUTATION", severity: "info", message: "test mutation" });
    expect(await runPreviewMechanics()).toEqual(sparseMechanicsFixture);
    expect(await runPreviewMechanics(null, "dense_scrutiny")).toEqual(denseMechanicsFixture);
  });

  it("compares model content canonically rather than object key insertion order", async () => {
    const model = await loadPreviewModel();
    const reordered = Object.fromEntries(Object.entries(model).reverse()) as PreviewModel;
    expect(await runPreviewMechanics(reordered)).toEqual(sparseMechanicsFixture);
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
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("rejects crossed, missing and ambiguous actual mode evidence", async () => {
    const model = await loadPreviewModel();
    const sparse = structuredClone(sparseMechanicsFixture) as MechanicsResult;
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

describe("buildAnalysisRunPreview rule-check aggregate (TP-C4-APPAGG-001)", () => {
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

describe("analysis-run input-manifest and source-dimension binding", () => {
  it("binds the exact manifest independently from the result envelope and declares stiffness dimensions", async () => {
    const result = await precisionFixtureSource();
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
    expect(
      byId.get(
        "result:component-stiffness:component-C-150:axial",
      ),
    ).toBe("linear_stiffness");
    expect(
      byId.get(
        "result:component-stiffness:component-C-150:torsional",
      ),
    ).toBe("rotational_stiffness");
  });

  it("blocks manifest evidence for a different model", async () => {
    const result = await precisionFixtureSource();
    const manifest = await manifestFor(result);
    const mismatched = structuredClone(result);
    mismatched.model_ref = "project:different-model";

    await expect(
      buildAnalysisRunPreview(mismatched, { inputManifest: manifest }),
    ).rejects.toThrow("ANALYSIS-RUN-INPUT-MANIFEST-MODEL-MISMATCH");
  });

  it("blocks transformed manifest evidence until its ref and hash are recomputed", async () => {
    const result = await precisionFixtureSource();
    const manifest = await manifestFor(result);
    manifest.manifest.solver_basis.settings.sparse_evidence_lane = false;

    await expect(
      buildAnalysisRunPreview(result, { inputManifest: manifest }),
    ).rejects.toThrow("INPUT-MANIFEST-HASH-MISMATCH");
  });

  it("blocks wrong-prefix and wrong-model manifest refs with a valid digest", async () => {
    const result = await precisionFixtureSource();
    const manifest = await manifestFor(result);
    for (const refValue of [
      `result-envelope:project-invented-loop-01:${manifest.manifest_sha256}`,
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
    const result = structuredClone(await precisionFixtureSource());
    const target = result.results.find(
      (item) => item.kind === "element_local_axial_force",
    );
    expect(target).toBeDefined();
    target!.unit = "MPa";
    await expect(buildAnalysisRunPreview(result, {
      inputManifest: await manifestFor(result),
    })).rejects.toThrow("SOURCE_UNIT_CONTRADICTION");
  });

  it("blocks an explicit dimension that contradicts exact result kind semantics", async () => {
    const result = structuredClone(await precisionFixtureSource());
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

describe("native mechanics failure boundary", () => {
  it("forwards the exact requested native mode without a fixture fallback", async () => {
    const model = await loadPreviewModel();
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(structuredClone(denseMechanicsFixture));
    expect(await runPreviewMechanics(model, "dense_scrutiny")).toEqual(denseMechanicsFixture);
    expect(invokeMock).toHaveBeenCalledExactlyOnceWith("run_preview_mechanics_with_solver_mode", { model, solverMode: "dense_scrutiny" });
  });
  it("propagates direct native solve rejection without a bundled result", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const failure = new Error("invented native IPC solve failure");
    invokeMock.mockRejectedValue(failure);
    await expect(runPreviewMechanics()).rejects.toBe(failure);
    expect(invokeMock).toHaveBeenCalledTimes(1);
  });

  it("propagates native job-start rejection without browser designation", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const failure = new Error("invented native IPC start failure");
    invokeMock.mockRejectedValue(failure);
    await expect(startPreviewMechanicsJob()).rejects.toBe(failure);
    expect(invokeMock).toHaveBeenCalledTimes(1);
  });

  it("uses browser job designation only when the host is absent", async () => {
    await expect(startPreviewMechanicsJob()).resolves.toEqual({ mode: "browser_fixture_no_backend_job" });
    expect(invokeMock).not.toHaveBeenCalled();
  });
});

it("does not enrich precision or unsupported raw0.2 carriers while reading history", () => {
 const raw = {schema_version:"0.2.0",results:[{id:"tiny",kind:"global_nodal_rotation_x",value:-1e-12,unit:"rad",entity_ref:"n"}]} as MechanicsResult;
 expect(bindSourceResultDimensions(raw)).toBe(raw);
 expect(Object.hasOwn(raw.results[0], "dimension")).toBe(false);
 expect(raw.results[0].value).toBe(-1e-12);
});

async function precisionFixtureSource(): Promise<MechanicsResult> {
 return runPreviewMechanics();
}
it("Current analysis composition refuses legacy and unknown raw without a historical fallback",async()=>{
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
