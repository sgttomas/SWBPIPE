// DEL-07-05 numerical health: the desktop wrapper over the actual wasm export
// (`numerical_health_json`) on committed producer outputs, with the reader's findings
// from the model-bound standing call, as the session passes them.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { numericalHealthView, readNumericalHealth } from "./numericalHealth";
import { numericalResultStanding } from "./resultStanding";
import type { MechanicsResult, PreviewModel } from "../../types";

const product = resolve(__dirname, "../../../../../fixtures/product_preview");
const read = <T,>(path: string) => JSON.parse(readFileSync(resolve(product, path), "utf8")) as T;

describe("numerical health over the wasm export", () => {
  it("reads a sensitive generic result held back by its verdict alone as verdict-only", () => {
    const model = read<PreviewModel>("rf_skew_t_cant_off_122_r1e-04.desktop_model.json");
    const result = read<MechanicsResult>("rf_skew_t_cant_off_122_r1e-04.desktop_result_dense_scrutiny.json");
    const standing = numericalResultStanding(result, model);
    expect(standing.eligible).toBe(false);
    expect([...standing.findings].sort()).toEqual(["NUMERICAL_CASE_EVIDENCE_INCOMPLETE", "NUMERICAL_INTEGRITY_NOT_QUALIFIED"]);
    const health = readNumericalHealth(result, standing.findings);
    expect(health).not.toBeNull();
    expect(health!.route).toBe("generic");
    expect(health!.verdict_only).toBe(true);
    expect(health!.cases.map((c) => c.state)).toEqual(["sensitive"]);
    const view = numericalHealthView(health, { eligible: false, verdictHold: true })!;
    expect(view.headline).toBe("Numerical health: 1 sensitive");
    expect(view.outputSentence).toBe("No results of this run can be exported or used by rule checks while 1 load case(s) are not checks passed.");
    // Without the model the reader cannot cover the requested cases: no verdict-only flag.
    expect(readNumericalHealth(result, numericalResultStanding(result).findings)!.verdict_only).toBe(false);
    expect(readNumericalHealth(result, null)!.verdict_only).toBe(false);
  });

  it("reads a pressure-source result's ordinary solve, never verdict-only", () => {
    const result = read<MechanicsResult>("pressure_source/physics_source/n05-dense_scrutiny.raw.json");
    const health = readNumericalHealth(result, numericalResultStanding(result).findings);
    expect(health).not.toBeNull();
    expect(health!.route).toBe("pressure_source");
    expect(health!.verdict_only).toBe(false);
    expect(health!.cases[0]).toMatchObject({ case_id: "case", state: "sensitive", integrity_diagnostic_id: "diagnostic:numerical-integrity:case" });
    const view = numericalHealthView(health, { eligible: true, verdictHold: false })!;
    expect(view.headline).toBe("Ordinary solve — Numerical health: 1 sensitive");
    expect(view.rowBadges).toBe(false);
  });

  it("reads a passing generic result green", () => {
    const result = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/results/preview_physics_unicode_ids_sparse.json"), "utf8")) as MechanicsResult;
    const view = numericalHealthView(readNumericalHealth(result, numericalResultStanding(result).findings), { eligible: false, verdictHold: false })!;
    expect([view.badge, view.headline]).toEqual(["green", "Numerical health: checks passed"]);
    for (const c of view.cases) if (c.conditioning) expect(c.conditioning).toMatch(/roughly (below 0\.1|\d+\.\d{1,2}) of the about 16/);
  });

  it("reads a withheld combination with a null state and gives it no badge or detail", () => {
    const result = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/results/preview_physics_invented_sparse.json"), "utf8")) as MechanicsResult;
    const health = readNumericalHealth(result, numericalResultStanding(result).findings);
    expect(health).not.toBeNull();
    const withheld = health!.combinations.filter((m) => m.withheld);
    expect(withheld.length).toBeGreaterThan(0);
    for (const m of withheld) expect(m.state).toBeNull();
    const view = numericalHealthView(health, { eligible: false, verdictHold: false })!;
    for (const m of withheld) expect(view.combinationState(m.combination_id)).toBeNull();
  });
});
