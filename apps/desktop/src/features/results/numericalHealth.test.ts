// DEL-07-05 numerical health §2.6, §3 and §7: the presentation over hand-written
// fixtures of `numerical_health_json`'s output shape (the Rust export is exercised by
// its own tests). States, header counts, combination inheritance, the pressure-source
// wording, the output sentence, the chip/rail hold predicate, the figure's format and
// the fail-closed reading.
import { afterEach, describe, expect, it, vi } from "vitest";
import { loadedWasmEngine, type WasmOperationEngine } from "../../services/wasmEngine/loadWasmEngine";
import { readerSourceText } from "./resultReader";
import {
  formatDigits,
  headerBadge,
  numericalHealthHold,
  numericalHealthView,
  parseNumericalHealth,
  readNumericalHealth,
  type NumericalHealth
} from "./numericalHealth";
import type { MechanicsResult } from "../../types";

type Raw = Record<string, unknown>;
function healthCase(caseId: string, state: string, extra: Raw = {}): Raw {
  return { case_id: caseId, state, code: null, integrity_diagnostic_id: `diagnostic:numerical-integrity:${caseId}`, rcond: null, digits: null, conditioning_sensitive: false, causes: [], ...extra };
}
function raw(route: string, verdictOnly: boolean, cases: Raw[], combinations: Raw[] = []): Raw {
  return { route, verdict_only: verdictOnly, cases, combinations };
}
function health(route: string, verdictOnly: boolean, cases: Raw[], combinations: Raw[] = []): NumericalHealth {
  const parsed = parseNumericalHealth(raw(route, verdictOnly, cases, combinations));
  expect(parsed).not.toBeNull();
  return parsed!;
}

describe("parseNumericalHealth (fail closed)", () => {
  it("reads the published shape", () => {
    const parsed = health("generic", true, [healthCase("LC1", "sensitive", {
      code: "NUMERICAL_INTEGRITY_SENSITIVE", rcond: 1e-11, digits: 11.0, conditioning_sensitive: true,
      causes: [{ kind: "formation", diagnostic_id: "diagnostic:numerical-integrity:LC1", locations: [{ kind: "node_dof", node: "N1", dof: "UX" }] }]
    })], [{ combination_id: "C1", state: "sensitive", cases: ["LC1"], withheld: false }]);
    expect(parsed.cases[0]).toMatchObject({ case_id: "LC1", state: "sensitive", digits: 11, conditioning_sensitive: true });
    expect(parsed.combinations[0].cases).toEqual([{ case_id: "LC1", state: null }]);
  });

  it("refuses any other shape", () => {
    const good = raw("generic", false, [healthCase("LC1", "checks_passed")]);
    expect(parseNumericalHealth(good)).not.toBeNull();
    for (const bad of [
      null, [], "x",
      { ...good, route: "exact" },
      { ...good, verdict_only: "true" },
      { ...good, cases: {} },
      raw("generic", false, [healthCase("LC1", "unresolved")]),
      raw("generic", false, [healthCase("LC1", "checks_passed"), healthCase("LC1", "sensitive")]),
      raw("generic", false, [healthCase("LC1", "checks_passed", { digits: -1 })]),
      raw("generic", false, [healthCase("LC1", "checks_passed", { digits: "7.8" })]),
      raw("generic", false, [healthCase("LC1", "checks_passed", { causes: [{ kind: "stiffness_contrast", locations: [] }] })]),
      raw("generic", false, [healthCase("LC1", "checks_passed", { causes: [{ kind: "formation", locations: [{ kind: "dof_index", index: 3 }] }] })]),
      raw("generic", false, [healthCase("", "checks_passed")]),
      // verdict_only is the generic route's flag only.
      raw("pressure_source", true, [healthCase("LC1", "sensitive")]),
      raw("retained", true, [healthCase("LC1", "sensitive")])
    ]) {
      expect(parseNumericalHealth(bad)).toBeNull();
    }
  });
});

describe("numericalHealthView", () => {
  it("is absent for the retained and unsupported routes, whose texts are unchanged", () => {
    expect(numericalHealthView(health("retained", false, [healthCase("LC1", "checks_passed")]), { eligible: true, verdictHold: false })).toBeNull();
    expect(numericalHealthView(health("unsupported", false, []), { eligible: false, verdictHold: false })).toBeNull();
    expect(numericalHealthView(null, { eligible: false, verdictHold: false })).toBeNull();
  });

  it("maps each state to its badge and headline", () => {
    const view = numericalHealthView(health("generic", false, [
      healthCase("A", "checks_passed"), healthCase("B", "sensitive"), healthCase("C", "quality_unresolved"),
      healthCase("D", "solve_failed"), healthCase("E", "not_assessed")
    ]), { eligible: false, verdictHold: false })!;
    expect(view.cases.map((c) => [c.caseId, c.badge, c.headline])).toEqual([
      ["A", "green", "Numerical health: checks passed"],
      ["B", "red", "Numerical health: sensitive"],
      ["C", "red", "Numerical health: quality unresolved"],
      ["D", "red", "Numerical health: solve failed"],
      ["E", "grey", "Numerical health: not assessed"]
    ]);
    expect(view.headline).toBe("Numerical health: 1 solve failed, 1 quality unresolved, 1 sensitive, 1 not assessed, 1 checks passed");
    expect(view.shortHeadline).toBe("Numerical health: solve failed");
    expect(view.badge).toBe("red");
  });

  it("counts load cases once and colours the header red, then grey, then green", () => {
    const passing = numericalHealthView(health("generic", false, [healthCase("A", "checks_passed"), healthCase("B", "checks_passed")]), { eligible: true, verdictHold: false })!;
    expect([passing.badge, passing.headline, passing.shortHeadline]).toEqual(["green", "Numerical health: checks passed", "Numerical health: checks passed"]);
    const grey = numericalHealthView(health("generic", false, [healthCase("A", "checks_passed"), healthCase("B", "not_assessed")]), { eligible: false, verdictHold: false })!;
    expect([grey.badge, grey.headline, grey.shortHeadline]).toEqual(["grey", "Numerical health: 1 not assessed, 1 checks passed", "Numerical health: not assessed"]);
    const red = numericalHealthView(health("generic", false, [healthCase("A", "sensitive"), healthCase("B", "sensitive"), healthCase("C", "not_assessed")],
      [{ combination_id: "C1", state: "sensitive", cases: ["A", "B"], withheld: false }]), { eligible: false, verdictHold: false })!;
    expect([red.badge, red.headline]).toEqual(["red", "Numerical health: 2 sensitive, 1 not assessed"]);
    expect(headerBadge([])).toBe("grey");
    const none = numericalHealthView(health("generic", false, []), { eligible: false, verdictHold: false })!;
    expect([none.badge, none.headline]).toEqual(["grey", "Numerical health: not assessed"]);
  });

  it("gives combinations their inherited state and text, with no badge when withheld", () => {
    const view = numericalHealthView(health("generic", false, [healthCase("LC1", "sensitive"), healthCase("LC2", "checks_passed")], [
      { combination_id: "C1", state: "sensitive", cases: ["LC1", "LC2"], withheld: false },
      { combination_id: "C2", state: "not_assessed", cases: [{ case_id: "LC2" }, { case_id: "LC9" }], withheld: false },
      { combination_id: "C3", state: "sensitive", cases: [{ case_id: "LC1", state: "sensitive" }], withheld: true }
    ]), { eligible: false, verdictHold: false })!;
    expect(view.combinations).toEqual([
      { combinationId: "C1", state: "sensitive", badge: "red", text: "Inherited from its load cases: LC1: sensitive; LC2: checks passed." },
      // A case missing from numerical_quality reads "not assessed".
      { combinationId: "C2", state: "not_assessed", badge: "grey", text: "Inherited from its load cases: LC2: checks passed; LC9: not assessed." },
      { combinationId: "C3", state: "sensitive", badge: null, text: "Inherited from its load cases: LC1: sensitive." }
    ]);
    expect(view.combinationState("C3")?.badge).toBeNull();
    expect(view.caseState("LC1")).toBe("sensitive");
    expect(view.caseState("LC9")).toBeNull();
  });

  it("writes the sensitive detail with its causes in the design's order", () => {
    const view = numericalHealthView(health("generic", true, [healthCase("LC1", "sensitive", {
      digits: 10.97, conditioning_sensitive: true,
      causes: [
        { kind: "formation_recovery", diagnostic_id: null, locations: [{ kind: "text", text: "member end forces" }] },
        { kind: "formation", diagnostic_id: null, locations: [{ kind: "node_dof", node: "N1", dof: "UX" }, { kind: "node_dof", node: "N2", dof: "RZ" }] },
        { kind: "load_fidelity", diagnostic_id: "diagnostic:load", locations: [{ kind: "ref", ref: "load:weight" }] },
        { kind: "curved_bend_short", diagnostic_id: null, locations: [{ kind: "ref", ref: "pipe:bend-1" }] }
      ]
    })]), { eligible: false, verdictHold: true })!;
    expect(view.cases[0].sentences).toEqual([
      "The solver's checks flagged LC1 as numerically sensitive.",
      "Its values are kept for inspection.",
      "The scaled stiffness is poorly conditioned.",
      "The solver could not confirm that every applied load was carried at full precision (load:weight).",
      "The solver's formation check could not confirm some stiffness or load terms at full precision (node N1, UX; node N2, RZ).",
      "The solver could not confirm some recovered member or support quantities at full precision (member end forces).",
      "Solving the unchanged model again in the same solver mode gives the same result. A very short or very stiff element next to flexible ones, stiffnesses that differ by many orders of magnitude, or a weakly restrained direction are common causes."
    ]);
    expect(view.cases[0].conditioning).toBe(
      "Estimated sensitivity of the scaled linear solve: rounding can be magnified by roughly 11.0 of the about 16 decimal digits the computer carries (the solver marks a case sensitive at about 7.8). This is an estimate; it does not estimate the error in individual results."
    );
    expect(view.cases[0].locations).toHaveLength(5);
    const bare = numericalHealthView(health("generic", true, [healthCase("LC1", "sensitive")]), { eligible: false, verdictHold: true })!;
    expect(bare.cases[0].sentences.slice(2)).toEqual([
      "See the solver diagnostic for the reason.",
      "Solving the unchanged model again in the same solver mode gives the same result. A very short or very stiff element next to flexible ones, stiffnesses that differ by many orders of magnitude, or a weakly restrained direction are common causes."
    ]);
    expect(bare.cases[0].conditioning).toBeNull();
  });

  it("writes the passed, unresolved, failed and not-assessed details", () => {
    const view = numericalHealthView(health("generic", false, [
      healthCase("P", "checks_passed", { digits: 0 }),
      healthCase("U", "quality_unresolved", { digits: 9 }),
      healthCase("R", "quality_unresolved", { code: "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", causes: [{ kind: "recovery_basis_unqualified", diagnostic_id: null, locations: [] }] }),
      healthCase("M", "solve_failed", { integrity_diagnostic_id: null, causes: [{ kind: "mechanism", diagnostic_id: null, locations: [{ kind: "text", text: "UX" }] }] }),
      healthCase("N", "solve_failed", { causes: [{ kind: "negative_energy", diagnostic_id: null, locations: [] }] }),
      healthCase("F", "solve_failed"),
      healthCase("X", "not_assessed", { integrity_diagnostic_id: null })
    ]), { eligible: false, verdictHold: false })!;
    const byId = Object.fromEntries(view.cases.map((c) => [c.caseId, c]));
    expect(byId.P.sentences).toEqual(["The solver's residual and conditioning checks passed for P. These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right."]);
    expect(byId.P.conditioning).toContain("roughly below 0.1 of the about 16");
    // The figure shows for passed and sensitive cases only.
    expect(byId.U.conditioning).toBeNull();
    expect(byId.U.sentences).toEqual([
      "The solver could not confirm the numerical quality of U at working precision.",
      "See the solver diagnostic for the reason.",
      "A near-mechanism or extreme stiffness contrast are common causes."
    ]);
    expect(byId.R.sentences[1]).toBe("Its support-state solution is kept for inspection, but its numerical quality could not be confirmed.");
    expect(byId.M.sentences).toEqual(["The solver could not complete a solution for M.", "The model can move without resistance (a mechanism). Review connectivity, member releases and supports."]);
    expect(byId.M.integrityDiagnosticId).toBeNull();
    expect(byId.N.sentences[1]).toBe("The assembled stiffness is not positive in some direction. Review element and support properties.");
    expect(byId.F.sentences[1]).toBe("See the solver diagnostic for the reason.");
    expect(byId.X.sentences).toEqual(["The solver did not assess the numerical quality of X."]);
  });

  it("prefixes the pressure-source route, adds the eligible wording and places no row badges", () => {
    const eligible = numericalHealthView(health("pressure_source", false, [healthCase("case", "sensitive", { digits: 10.2, conditioning_sensitive: true })]), { eligible: true, verdictHold: false })!;
    expect(eligible.headline).toBe("Ordinary solve — Numerical health: 1 sensitive");
    expect(eligible.cases[0].headline).toBe("Ordinary solve — Numerical health: sensitive");
    expect(eligible.rowBadges).toBe(false);
    expect(eligible.outputSentence).toBeNull();
    expect(eligible.cases[0].sentences).toEqual([
      "The solver's checks flagged case as numerically sensitive.",
      "The scaled stiffness is poorly conditioned.",
      "Solving the unchanged model again in the same solver mode gives the same result. A very short or very stiff element next to flexible ones, stiffnesses that differ by many orders of magnitude, or a weakly restrained direction are common causes.",
      "The values shown come from the producer's recovery of this case, which passed its own checks. This badge describes the ordinary solve only."
    ]);
    const notEligible = numericalHealthView(health("pressure_source", false, [healthCase("case", "sensitive")]), { eligible: false, verdictHold: false })!;
    expect(notEligible.cases[0].sentences).toContain("Its values are kept for inspection.");
    expect(notEligible.cases[0].sentences).not.toContain("The values shown come from the producer's recovery of this case, which passed its own checks. This badge describes the ordinary solve only.");
    // The generic route's rows carry badges.
    expect(numericalHealthView(health("generic", false, [healthCase("LC1", "checks_passed")]), { eligible: true, verdictHold: false })!.rowBadges).toBe(true);
  });

  it("adds the output sentence only under the verdict-only hold on the generic route", () => {
    const cases = [healthCase("LC1", "sensitive"), healthCase("LC2", "not_assessed"), healthCase("LC3", "checks_passed")];
    expect(numericalHealthView(health("generic", true, cases), { eligible: false, verdictHold: true })!.outputSentence)
      .toBe("No results of this run can be exported or used by rule checks while 2 load case(s) are not checks passed.");
    expect(numericalHealthView(health("generic", true, cases), { eligible: false, verdictHold: false })!.outputSentence).toBeNull();
    expect(numericalHealthView(health("generic", false, cases), { eligible: false, verdictHold: false })!.outputSentence).toBeNull();
    expect(numericalHealthView(health("pressure_source", false, cases), { eligible: false, verdictHold: true })!.outputSentence).toBeNull();
  });
});

describe("numericalHealthHold (chip and rail predicate)", () => {
  const sensitive = [healthCase("LC1", "sensitive")];
  it("holds only for a non-Current result whose other requirements hold, with verdict_only and a red or grey case", () => {
    expect(numericalHealthHold({ isCurrent: false, otherCurrentRequirementsHold: true, health: health("generic", true, sensitive) })).toBe(true);
    expect(numericalHealthHold({ isCurrent: false, otherCurrentRequirementsHold: true, health: health("generic", true, [healthCase("LC1", "not_assessed")]) })).toBe(true);
    expect(numericalHealthHold({ isCurrent: true, otherCurrentRequirementsHold: true, health: health("generic", true, sensitive) })).toBe(false);
    expect(numericalHealthHold({ isCurrent: false, otherCurrentRequirementsHold: false, health: health("generic", true, sensitive) })).toBe(false);
    expect(numericalHealthHold({ isCurrent: false, otherCurrentRequirementsHold: true, health: health("generic", true, [healthCase("LC1", "checks_passed")]) })).toBe(false);
  });
  it("keeps Needs recompute for missing evidence, pressure-source and retained results, and no indicator", () => {
    // Missing or malformed evidence findings: Rust gives verdict_only false.
    expect(numericalHealthHold({ isCurrent: false, otherCurrentRequirementsHold: true, health: health("generic", false, sensitive) })).toBe(false);
    expect(numericalHealthHold({ isCurrent: false, otherCurrentRequirementsHold: true, health: health("pressure_source", false, sensitive) })).toBe(false);
    expect(numericalHealthHold({ isCurrent: false, otherCurrentRequirementsHold: true, health: health("retained", false, sensitive) })).toBe(false);
    expect(numericalHealthHold({ isCurrent: false, otherCurrentRequirementsHold: true, health: null })).toBe(false);
  });
});

describe("formatDigits", () => {
  it("prints one decimal, two near the sensitivity line, and below 0.1 at zero", () => {
    expect(formatDigits(0)).toBe("below 0.1");
    expect(formatDigits(3.04)).toBe("3.0");
    expect(formatDigits(7.69)).toBe("7.7");
    expect(formatDigits(7.7)).toBe("7.70");
    expect(formatDigits(7.8239)).toBe("7.82");
    expect(formatDigits(7.9)).toBe("7.90");
    expect(formatDigits(7.91)).toBe("7.9");
    expect(formatDigits(10.9756)).toBe("11.0");
  });
});

describe("readNumericalHealth", () => {
  const engine = loadedWasmEngine() as WasmOperationEngine;
  const original = Object.getOwnPropertyDescriptor(engine, "numericalHealthJson");
  afterEach(() => {
    if (original) Object.defineProperty(engine, "numericalHealthJson", original);
    else delete (engine as { numericalHealthJson?: unknown }).numericalHealthJson;
  });
  const result = { schema_version: "0.3.0", run_id: "run:invented-health", results: [], diagnostics: [] } as unknown as MechanicsResult;

  it("carries the result text and the reader's findings to the Rust export and reads its answer", () => {
    const binding = vi.fn(() => JSON.stringify(raw("generic", true, [healthCase("LC1", "sensitive")])));
    engine.numericalHealthJson = binding;
    const read = readNumericalHealth(result, ["NUMERICAL_INTEGRITY_NOT_QUALIFIED", "NUMERICAL_CASE_EVIDENCE_INCOMPLETE"]);
    expect(read?.verdict_only).toBe(true);
    expect(binding).toHaveBeenCalledWith(readerSourceText(result), JSON.stringify(["NUMERICAL_INTEGRITY_NOT_QUALIFIED", "NUMERICAL_CASE_EVIDENCE_INCOMPLETE"]));
  });

  it("fails closed without the export, on a throw and on an unexpected shape", () => {
    delete (engine as { numericalHealthJson?: unknown }).numericalHealthJson;
    expect(readNumericalHealth(result, [])).toBeNull();
    engine.numericalHealthJson = () => { throw new Error("WASM-ENGINE-INPUT-JSON-INVALID"); };
    expect(readNumericalHealth(result, [])).toBeNull();
    engine.numericalHealthJson = () => JSON.stringify({ route: "generic", verdict_only: true });
    expect(readNumericalHealth(result, [])).toBeNull();
    engine.numericalHealthJson = () => "not json";
    expect(readNumericalHealth(result, [])).toBeNull();
  });
});
