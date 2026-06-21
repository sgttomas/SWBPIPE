import { describe, expect, it } from "vitest";
import type { MechanicsResult, PreviewModel } from "../types";
import {
  appliedRuleCheckStatus,
  buildAnalysisRunPreview,
  loadPreviewModel,
  runPreviewMechanics,
} from "./previewService";

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
    const run = await buildAnalysisRunPreview(result);

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
      value: 0,
    });
    expect(
      result.results.find(
        (item) =>
          item.id ===
          "result:nonlinear-support:support-NL-130-FRIC:friction-normal-reaction",
      ),
    ).toMatchObject({
      kind: "nonlinear_support_friction_normal_reaction_input",
      entity_ref: "support:NL-130-FRIC",
      value: 10,
      unit: "N",
    });
    expect(
      result.results
        .find(
          (item) =>
            item.id ===
            "result:nonlinear-support:support-NL-130-FRIC:friction-normal-reaction",
        )
        ?.metadata?.basis,
    ).toContain("derived_normal_force_model=TBD");
    expect(
      result.diagnostics.some((item) => item.code === "TOLERANCE_POLICY_TBD"),
    ).toBe(true);
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
    ).toBe("nonlinear_support");
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
    const env = await buildAnalysisRunPreview(solvedResultStub);
    expect(env.analysis_run.analysis_status).toContain(
      "RULE_INPUTS_INCOMPLETE",
    );
    expect(env.analysis_run.analysis_status).not.toContain("USER_RULE_FAILED");
  });

  it("drives a recognized aggregate into analysis_status and the run-record hash, leaving the solve-envelope hash byte-stable", async () => {
    const base = await buildAnalysisRunPreview(solvedResultStub);
    const withFail = await buildAnalysisRunPreview(
      solvedResultStub,
      "USER_RULE_FAILED",
    );

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
    const env = await buildAnalysisRunPreview(
      solvedResultStub,
      "USER_RULE_CHECKED",
    );
    expect(env.analysis_run.analysis_status).toContain("USER_RULE_CHECKED");
    expect(env.analysis_run.analysis_status).not.toContain(
      "RULE_INPUTS_INCOMPLETE",
    );
  });

  it("ignores an unrecognized aggregate, reproducing the no-aggregate envelope exactly (no false pass)", async () => {
    const base = await buildAnalysisRunPreview(solvedResultStub);
    const bogus = await buildAnalysisRunPreview(
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
