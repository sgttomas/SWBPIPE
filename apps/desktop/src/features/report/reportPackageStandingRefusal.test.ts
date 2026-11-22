/** DEL-08-01 REQ-024 and LIM-016: the package refusal of a pressure-free result whose numerical
 * standing is not eligible, its ground, and the words each surface composes from it.
 *
 * - The ground (`packageStandingGround`) is "health" where some load case's solve quality is not
 *   checks passed and "record" where every load case reads checks passed yet the result is not
 *   eligible. Both sides are typed results here; the "record" side is constructed, because no
 *   product output has it (the producer cites each case's integrity diagnostic), and it is handled
 *   because the reader admits such a record as readable (`reader_readability.rs`
 *   `case_record_consistent`), so the rendered report takes the result.
 * - `buildReportPackageRequest` refuses the product's solved, readable Timoshenko witness that is
 *   not eligible (`preview_physics_timoshenko_sensitive_torsion_sparse.json`, which
 *   `core/product_physics/tests/pf_timoshenko_envelopes.rs` keeps current) with a
 *   `ReportPackageStandingRefusal`: its code as data, its reading and the rendered report's
 *   advice. The product's blocked Timoshenko envelope, which the rendered report does not take,
 *   and a pressure-free Euler–Bernoulli result that is not eligible keep the builder's bare code.
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
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { numericalResultStanding } from "../results/resultStanding";
import { PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "../results/numericalResultQuality";
import {
  buildReportPackageRequest,
  CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE,
  packageStandingGround,
  REPORT_PACKAGE_INSTEAD_RENDERED,
  REPORT_PACKAGE_ROUTE_NOT_ADMITTED,
  reportPackageNotSavedText,
  reportPackageSourceUnavailableReason,
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
  const timoshenko = (qualities: string[]) => ({ ...typed(qualities), producer: { semantic_contract_id: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } }) as unknown as MechanicsResult;

  it("the report panel says, in place of the two package buttons", () => {
    expect(reportPackageSourceUnavailableReason(timoshenko(["sensitive"]), null)).toBe(
      "No report package for this result: its numerical health does not allow one. The report states it and can be printed or saved as HTML."
    );
    expect(reportPackageSourceUnavailableReason(timoshenko(["checks_passed"]), null)).toBe(
      "No report package for this result: a load case's numerical record is incomplete. The report states it and can be printed or saved as HTML."
    );
  });

  it("the package save's status line says, with the rendered report's advice only where it takes the result", () => {
    expect(reportPackageNotSavedText(new ReportPackageStandingRefusal("health", REPORT_PACKAGE_INSTEAD_RENDERED))).toBe(
      "Report package not saved: this result's numerical health does not allow one. Nothing was written. The report can be printed or saved as HTML."
    );
    expect(reportPackageNotSavedText(new ReportPackageStandingRefusal("record", REPORT_PACKAGE_INSTEAD_RENDERED))).toBe(
      "Report package not saved: a load case's numerical record is incomplete. Nothing was written. The report can be printed or saved as HTML."
    );
    expect(reportPackageNotSavedText(new ReportPackageStandingRefusal("health", null))).toBe(
      "Report package not saved: this result's numerical health does not allow one. Nothing was written."
    );
    expect(reportPackageNotSavedText(new ReportPackageStandingRefusal("record", null))).toBe(
      "Report package not saved: a load case's numerical record is incomplete. Nothing was written."
    );
  });

  it("the shareable refusal's last sentence follows the session's ground, the health form where there is none", () => {
    const lead = `${REPORT_PACKAGE_ROUTE_NOT_ADMITTED}: a shareable report package carries pressure-free (preview-physics-2) results and exact pressure route (pressure-3, pressure-source-1) results; this result is ${PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID}.`;
    expect(userReleasedReportPackageUnavailableReason(timoshenko(["checks_passed"]))).toBe(`${lead} A local private package can carry it when its numerical health allows.`);
    expect(userReleasedReportPackageUnavailableReason(timoshenko(["sensitive"]), "health")).toBe(`${lead} A local private package can carry it when its numerical health allows.`);
    expect(userReleasedReportPackageUnavailableReason(timoshenko(["checks_passed"]), "record")).toBe(`${lead} A local private package can carry it when every load case's numerical record is complete.`);
  });
});

describe("the package request's refusal of a pressure-free result that is not eligible", () => {
  it("refuses the product's solved Timoshenko witness that is not eligible with its ground, its code as data and the rendered report's advice", async () => {
    const input = await session(witnessModel(), text(WITNESS), "invented-pf-timoshenko-witness-package-job");
    expect(input.result.producer?.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    expect(input.result.status.mechanics).toBe("MECHANICS_SOLVED");
    expect(numericalResultStanding(input.result, input.model).eligible).toBe(false);
    const refusal = await buildReportPackageRequest(input).then(() => null, (error: unknown) => error);
    expect(refusal).toBeInstanceOf(ReportPackageStandingRefusal);
    const standing = refusal as ReportPackageStandingRefusal;
    expect([standing.code, standing.ground, standing.reading, standing.instead]).toEqual([
      CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE, "health", "this result's numerical health does not allow one", REPORT_PACKAGE_INSTEAD_RENDERED
    ]);
    expect(standing.message).toBe(`${CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE}: this result's numerical health does not allow one`);
  });

  it("keeps the bare code for the product's blocked Timoshenko envelope, which the rendered report does not take", async () => {
    const input = await session(straightModel(), text(BLOCKED), "invented-pf-timoshenko-blocked-package-job");
    expect(input.result.status.mechanics).toBe("MODEL_INCOMPLETE");
    const refusal = await buildReportPackageRequest(input).then(() => null, (error: unknown) => error);
    expect(refusal).not.toBeInstanceOf(ReportPackageStandingRefusal);
    expect((refusal as Error).message).toBe(CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE);
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
