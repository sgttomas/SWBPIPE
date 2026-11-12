// The Solve command's gate (DEL-07-07 REQ-008, AC-005; DEL-07-04 AC-001), read
// from the model checks' last run. Invented models and findings.
import { describe, expect, it } from "vitest";
import type { Diagnostic, PreviewModel } from "../../types";
import type { ModelCheckFinding, ModelCheckGroupId, ModelCheckRun } from "../diagnostics/modelChecks";
import { readSolveGate, solveWithheldReason } from "./solveGates";

const model = { nodes: [], pipe_segments: [] } as unknown as PreviewModel;

function finding(group: ModelCheckGroupId, severity: Diagnostic["severity"], code: string): ModelCheckFinding {
  return { key: `${group}:${code}`, group, entity: null, diagnostic: { id: `diagnostic:${code}`, code, severity, message: `invented ${code}`, affected_refs: ["node:invented"] } };
}

function run(overrides: Partial<ModelCheckRun> = {}): ModelCheckRun {
  return {
    model,
    solve: { kind: "checked" },
    findings: [],
    notShown: { solve: 0, slots: 0, library: 0 },
    notShownBlocking: 0,
    notShownBlockingByGroup: { solve: 0, slots: 0, library: 0 },
    ...overrides
  };
}

describe("readSolveGate", () => {
  it("is open when the current run's solve group holds only warnings and info", () => {
    expect(readSolveGate(model, run({ findings: [finding("solve", "warning", "W"), finding("solve", "info", "I")] }))).toEqual({ kind: "open" });
  });

  it.each(["blocking", "error"] as const)("withholds on a solve finding of severity %s and carries the record unaltered", (severity) => {
    const standing = finding("solve", severity, "INVENTED_INPUT_MISSING");
    const reading = readSolveGate(model, run({ findings: [finding("solve", "warning", "W"), standing] }));
    expect(reading.kind).toBe("withheld");
    if (reading.kind !== "withheld") return;
    expect(reading.findings).toEqual([standing]);
    expect(reading.findings[0].diagnostic).toBe(standing.diagnostic);
    expect(reading.diagnostics).toEqual([standing.diagnostic]);
    expect(reading.diagnostics[0]).toBe(standing.diagnostic);
    expect(reading.model).toBe(model);
    expect(reading.notListedBlocking).toBe(0);
    expect(reading.refusal).toBeNull();
  });

  it("withholds when the solve cannot read the model, with the solve's refusal text", () => {
    const reading = readSolveGate(model, run({ solve: { kind: "refused", refusal: "invented refusal" } }));
    expect(reading).toEqual({ kind: "withheld", model, findings: [], diagnostics: [], notListedBlocking: 0, refusal: "invented refusal" });
  });

  it("withholds on unlisted blocking findings the report counted in the solve group", () => {
    const reading = readSolveGate(model, run({ notShown: { solve: 3, slots: 0, library: 0 }, notShownBlocking: 2, notShownBlockingByGroup: { solve: 2, slots: 0, library: 0 } }));
    expect(reading).toEqual({ kind: "withheld", model, findings: [], diagnostics: [], notListedBlocking: 2, refusal: null });
  });

  it("stays open on blocking slot, library and unlisted non-solve findings: only the solve group withholds", () => {
    const reading = readSolveGate(model, run({
      findings: [finding("slots", "blocking", "SLOT"), finding("library", "error", "LIB")],
      notShown: { solve: 0, slots: 4, library: 1 },
      notShownBlocking: 5,
      notShownBlockingByGroup: { solve: 0, slots: 4, library: 1 }
    }));
    expect(reading).toEqual({ kind: "open" });
  });

  it("stays open with only a rule-check-blocking finding standing (it is not a model-check finding)", () => {
    const ruleCheckBlocking = { ...model, diagnostics: [{ id: "d", code: "RULE_INPUT_MISSING", severity: "blocking", message: "invented rule-check input", diagnostic_class: "rule_check_blocking" }] } as unknown as PreviewModel;
    expect(readSolveGate(ruleCheckBlocking, run({ model: ruleCheckBlocking }))).toEqual({ kind: "open" });
  });

  it("is unknown with no run, with a run of another model value, and with the checks unavailable", () => {
    expect(readSolveGate(model, null)).toEqual({ kind: "unknown", reason: "not_checked" });
    expect(readSolveGate(null, run())).toEqual({ kind: "unknown", reason: "not_checked" });
    const stale = run({ findings: [finding("solve", "blocking", "B")] });
    expect(readSolveGate({ ...model } as PreviewModel, stale)).toEqual({ kind: "unknown", reason: "model_changed" });
    expect(readSolveGate(model, run({ solve: { kind: "unavailable", reason: "invented" } }))).toEqual({ kind: "unknown", reason: "checks_unavailable" });
  });

  it("states the reason with the count of standing findings and where they are listed", () => {
    const reading = readSolveGate(model, run({ findings: [finding("solve", "blocking", "B")], notShownBlockingByGroup: { solve: 1, slots: 0, library: 0 } }));
    if (reading.kind !== "withheld") throw new Error("expected withheld");
    expect(solveWithheldReason(reading)).toBe("Solve is withheld: 2 solve-blocking findings stand for this model. The findings are listed in Model checks, in the Issues drawer.");
  });
});
