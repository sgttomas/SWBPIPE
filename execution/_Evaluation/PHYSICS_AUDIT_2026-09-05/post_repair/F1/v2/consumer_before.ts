import { describe, expect, it } from "vitest";
import type { MechanicsResult, PreviewModel } from "../types";
import {
  appliedRuleCheckStatus,
  buildAnalysisRunPreview,
  loadPreviewModel,
  runPreviewMechanics,
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
  schema_version: "0.1.0",
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
): Promise<CurrentSessionInputManifestEvidence> {
  const model = cloneModel(await loadPreviewModel());
  model.project.id = result.model_ref;
  return buildCurrentSessionInputManifest({
    model,
    solver: {
      solver_name: "open_pipe_stress_product_physics",
      solver_version: "0.1.0",
      solver_build_ref: "open_pipe_stress_product_physics@0.1.0",
      solver_mode: "sparse_interactive",
      settings: {
        nonlinear_iteration_policy:
          "DEC-046-CV-B-product-preview-active-set-count-v1",
        sparse_evidence_lane: true,
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

describe("previewService mechanics browser fallback", () => {
  it("returns the bundled solved fixture when no model payload is supplied", async () => {
    const result = await runPreviewMechanics();

    expect(result.status.mechanics).toBe("MECHANICS_SOLVED");
    expect(result.model_ref).toBe("project:invented-loop-01");
    expect(result.results.length).toBeGreaterThan(0);
  });

  it("keeps the bundled solved fixture for the unchanged bundled model", async () => {
    const model = await loadPreviewModel();
    const result = await runPreviewMechanics(model);

    expect(result.status.mechanics).toBe("MECHANICS_SOLVED");
    expect(result.model_ref).toBe("project:invented-loop-01");
    expect(result.results.length).toBeGreaterThan(0);
  });

  it("surfaces nonlinear support loop evidence rows and diagnostics", async () => {
    const result = await runPreviewMechanics();
    const run = await buildRun(result);

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
    expect(
      result.results.find(
        (item) =>
          item.id ===
          "result:nonlinear-support:support-NL-130-FRIC:uz-reaction",
      ),
    ).toMatchObject({
      kind: "nonlinear_support_final_reaction",
      entity_ref: "support:NL-130-FRIC",
      unit: "N",
      // DEC-067 bounded sliding: the sliding support carries the +mu*N
      // tangential reaction opposing its negative-Z motion.
      value: 0.490101,
    });
    expect(
      result.results.find(
        (item) =>
          item.id ===
          "result:nonlinear-support:support-NL-130-FRIC:friction-normal-reaction",
      ),
    ).toMatchObject({
      kind: "nonlinear_support_friction_normal_reaction_derived",
      entity_ref: "support:NL-130-FRIC",
      value: 48.952652,
      unit: "N",
    });
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
    ).toBe("ratio");
  });

  it("blocks edited-model fixture reuse instead of publishing stale result rows", async () => {
    const original = await loadPreviewModel();
    const edited = cloneModel(original);
    edited.project.id = "project:edited-browser-solve";
    edited.materials![0].elastic_modulus.value = 195_000_000_000;

    const result = await runPreviewMechanics(edited);

    expect(result.model_ref).toBe("project:edited-browser-solve");
    expect(result.status.mechanics).toBe("MODEL_INCOMPLETE");
    expect(result.status.rule_check).toBe("RULE_INPUTS_INCOMPLETE");
    expect(result.results).toEqual([]);
    expect(result.summary.node_count).toBe(edited.nodes.length);
    expect(result.summary.segment_count).toBe(edited.pipe_segments.length);
    expect(result.summary.support_count).toBe(edited.supports.length);
    expect(result.summary.load_case_count).toBe(edited.load_cases.length);
    expect(result.diagnostics).toHaveLength(1);
    expect(result.diagnostics[0]).toMatchObject({
      code: "BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL",
      severity: "blocking",
      affected_refs: ["project:edited-browser-solve"],
    });
    expect(original.project.id).toBe("project:invented-loop-01");
    expect(original.materials![0].elastic_modulus.value).toBe(200_000_000_000);
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
    expect(hashByScope(withFail, "result_envelope")).toBe(
      hashByScope(base, "result_envelope"),
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
    const result = await runPreviewMechanics();
    const manifest = await manifestFor(result);
    const env = await buildAnalysisRunPreview(result, {
      inputManifest: manifest,
    });
    const resultEnvelopeHash = hashByScope(env, "result_envelope");
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
    const result = await runPreviewMechanics();
    const manifest = await manifestFor(result);
    const mismatched = structuredClone(result);
    mismatched.model_ref = "project:different-model";

    await expect(
      buildAnalysisRunPreview(mismatched, { inputManifest: manifest }),
    ).rejects.toThrow("ANALYSIS-RUN-INPUT-MANIFEST-MODEL-MISMATCH");
  });

  it("blocks transformed manifest evidence until its ref and hash are recomputed", async () => {
    const result = await runPreviewMechanics();
    const manifest = await manifestFor(result);
    manifest.manifest.solver_basis.settings.sparse_evidence_lane = false;

    await expect(
      buildAnalysisRunPreview(result, { inputManifest: manifest }),
    ).rejects.toThrow("INPUT-MANIFEST-HASH-MISMATCH");
  });

  it("blocks wrong-prefix and wrong-model manifest refs with a valid digest", async () => {
    const result = await runPreviewMechanics();
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

  it("keeps family and dimension bound to exact kind semantics when unit text is deceptive", async () => {
    const result = structuredClone(await runPreviewMechanics());
    const target = result.results.find(
      (item) => item.kind === "element_local_axial_force",
    );
    expect(target).toBeDefined();
    target!.unit = "MPa";
    const env = await buildAnalysisRunPreview(result, {
      inputManifest: await manifestFor(result),
    });
    const bound = env.analysis_run.result_refs.find(
      (item) => item.result_ref.ref === target!.id,
    );
    expect(bound).toMatchObject({
      result_family: "force",
      source_dimension: "force",
    });
  });

  it("blocks an explicit dimension that contradicts exact result kind semantics", async () => {
    const result = structuredClone(await runPreviewMechanics());
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
