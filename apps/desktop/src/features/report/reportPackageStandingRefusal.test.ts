/** DEL-08-01 REQ-024 and LIM-016: the package refusal of a pressure-free result whose numerical
 * standing is not eligible, its ground, and the words each surface composes from it. A
 * pressure-free Timoshenko result is not refused for its standing: its packages carry it.
 *
 * - The ground (`packageStandingGround`) is "health" where some load case's solve quality is not
 *   checks passed and "record" where every load case reads checks passed yet the result is not
 *   eligible. Both sides are typed results here; the "record" side is constructed, because no
 *   product output has it (the producer cites each case's integrity diagnostic), and it is handled
 *   because the reader admits such a record as readable (`reader_readability.rs`
 *   `case_record_consistent`), so the rendered report takes the result.
 * - `buildReportPackageRequest` builds the request of the product's solved, readable Timoshenko
 *   witness that is not eligible (`preview_physics_timoshenko_sensitive_torsion_sparse.json`, which
 *   `core/product_physics/tests/pf_timoshenko_envelopes.rs` keeps current), carrying its standing
 *   record, and the shareable cell admits it. The product's blocked Timoshenko envelope, which the
 *   reader does not read as solved, is still refused, and not for its standing; a pressure-free
 *   Euler–Bernoulli result that is not eligible keeps the builder's bare code, and the File menu
 *   states its ground (`workspaceSession.ts`, `reportPackageRequestRefusal`).
 *
 * Unit transport replay of committed producer bytes through mocked IPC; NOT a native UI
 * qualification witness. Every id is invented. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { createSolveJobTransport } from "../../test/nativeMechanicsReplay";
import { REPORT_PACKAGE_DENIED_STATES } from "../../test/reportPackageDeniedStates";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { admitExactReportSource } from "./exactReportAdmission";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { numericalResultStanding } from "../results/resultStanding";
import { PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "../results/numericalResultQuality";
import {
  buildReportPackageRequest,
  CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE,
  packageStandingGround,
  reportPackageNotSavedText,
  ReportPackageStandingRefusal,
  userReleasedReportPackageUnavailableReason
} from "./reportPackageRequest";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const text = (path: string) => readFileSync(resolve(root, path), "utf8");
const SOLVER = { solver_name: "open_pipe_stress_product_physics", solver_version: "0.2.0", solver_build_ref: "open_pipe_stress_product_physics@0.2.0", solver_mode: "sparse_interactive", settings: {} };
const WITNESS = "fixtures/results/preview_physics_timoshenko_sensitive_torsion_sparse.json";
const BLOCKED = "fixtures/results/preview_physics_timoshenko_partial_span_blocked_sparse.json";

afterEach(() => {
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  delete (window as Json).__TAURI_INTERNALS__;
});

/** A typed result whose load cases have these solve qualities (the aggregate is their worst). */
function typed(qualities: string[], status = qualities.find((q) => q !== "checks_passed") ?? "checks_passed") {
  return { numerical_quality: { status, cases: qualities.map((solve_quality) => ({ solve_quality })) } } as unknown as MechanicsResult;
}

/** The witness's model: the committed sensitive torsion model with Timoshenko straight members
 * selected, as the envelope test solves it, plus the display members the desktop shell needs. */
function witnessModel(): PreviewModel {
  const model = JSON.parse(text("fixtures/product_preview/numerical_sensitive_torsion_model.json")) as Json;
  model.model_settings = { shear_model: "timoshenko_annulus_energy_v1" };
  if (typeof model.project.name !== "string") model.project.name = model.project.id;
  for (const loadCase of model.load_cases) if (loadCase.status === undefined) loadCase.status = "preview_only";
  if (model.components === undefined) model.components = [];
  return model as PreviewModel;
}

function straightModel(): PreviewModel {
  const model = structuredClone(JSON.parse(text("validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-TIMOSHENKO.request.json")).model) as Json;
  model.project.name = model.project.id;
  for (const loadCase of model.load_cases) loadCase.status = "preview_only";
  return model as PreviewModel;
}

/** A session over `bytes` for `model`, and the package request's input. */
async function session(model: PreviewModel, bytes: string, jobIdPrefix: string) {
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(createSolveJobTransport(() => bytes, { jobIdPrefix }).invoke);
  const result = await runPreviewMechanics(model);
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: SOLVER, active_rule_packs: [], external_assets: [] });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  return { model, result, inputManifest, analysisRun, projectSummary: null, comparison: null, ruleCheckAggregate: null, ruleCheckRun: null };
}

describe("the ground of a result that is not eligible", () => {
  it("is health where some load case's solve quality is not checks passed", () => {
    for (const qualities of [["sensitive"], ["checks_passed", "unresolved"], ["not_assessed"], ["failed", "checks_passed"]]) {
      expect(packageStandingGround(typed(qualities)), qualities.join(",")).toBe("health");
    }
    // An aggregate that is not checks passed is the health ground too.
    expect(packageStandingGround(typed(["checks_passed"], "sensitive"))).toBe("health");
  });

  it("is record where every load case reads checks passed (constructed: no product output has it)", () => {
    expect(packageStandingGround(typed(["checks_passed"]))).toBe("record");
    expect(packageStandingGround(typed(["checks_passed", "checks_passed"]))).toBe("record");
  });
});

describe("the words of each ground", () => {
  it("the package save's status line says", () => {
    expect(reportPackageNotSavedText(new ReportPackageStandingRefusal("health", null))).toBe(
      "Report package not saved: this result's numerical health does not allow one. Nothing was written."
    );
    expect(reportPackageNotSavedText(new ReportPackageStandingRefusal("record", null))).toBe(
      "Report package not saved: a load case's numerical record is incomplete. Nothing was written."
    );
    const refusal = new ReportPackageStandingRefusal("health", null);
    expect([refusal.code, refusal.ground, refusal.reading, refusal.message]).toEqual([
      CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE, "health", "this result's numerical health does not allow one",
      `${CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE}: this result's numerical health does not allow one`
    ]);
  });

  it("the shareable cell admits a pressure-free Timoshenko result whatever its standing", () => {
    const timoshenko = (qualities: string[]) => ({ ...typed(qualities), producer: { semantic_contract_id: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } }) as unknown as MechanicsResult;
    for (const qualities of [["sensitive"], ["checks_passed"]]) {
      expect(userReleasedReportPackageUnavailableReason(timoshenko(qualities)), qualities.join(",")).toBeNull();
    }
  });
});

describe("the package request's refusal of a pressure-free result that is not eligible", () => {
  it("builds the request of the product's solved Timoshenko witness that is not eligible, carrying its standing record", async () => {
    const input = await session(witnessModel(), text(WITNESS), "invented-pf-timoshenko-witness-package-job");
    expect(input.result.producer?.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    expect(input.result.status.mechanics).toBe("MECHANICS_SOLVED");
    expect(numericalResultStanding(input.result, input.model).eligible).toBe(false);
    const request = (await buildReportPackageRequest(input)) as Json;
    const admission = await admitExactReportSource({ model: input.model, result: input.result, analysisRun: input.analysisRun, inputManifest: input.inputManifest, purpose: "report" });
    expect(request.report.numerical_standing).toEqual(admission.numericalStanding);
    expect(request.report.numerical_standing.cases.map((c: Json) => c.health)).toEqual(["sensitive"]);
    expect("pf_timoshenko" in request.report).toBe(false);
    expect(request.source_results).toEqual([input.result]);
  });

  it("still refuses the product's blocked Timoshenko envelope, which the reader does not read as solved, and not for its standing", async () => {
    const input = await session(straightModel(), text(BLOCKED), "invented-pf-timoshenko-blocked-package-job");
    expect(input.result.status.mechanics).toBe("MODEL_INCOMPLETE");
    const refusal = await buildReportPackageRequest(input).then(() => null, (error: unknown) => error);
    expect(refusal).toBeInstanceOf(Error);
    expect(refusal).not.toBeInstanceOf(ReportPackageStandingRefusal);
    // A refusal led by its code, not the standing words a person reads for a solved result.
    expect((refusal as Error).message).toMatch(/^[A-Z][A-Z0-9_-]*(:|$)/);
  });

  it("keeps the bare code for a pressure-free Euler–Bernoulli result that is not eligible", async () => {
    const [, make] = REPORT_PACKAGE_DENIED_STATES[0];
    const { model, bytes } = make();
    const input = await session(structuredClone(model), bytes, "invented-pf-euler-bernoulli-not-eligible-package-job");
    expect(numericalResultStanding(input.result, input.model).eligible).toBe(false);
    const refusal = await buildReportPackageRequest(input).then(() => null, (error: unknown) => error);
    expect(refusal).not.toBeInstanceOf(ReportPackageStandingRefusal);
    expect((refusal as Error).message).toBe(CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE);
  });
});
