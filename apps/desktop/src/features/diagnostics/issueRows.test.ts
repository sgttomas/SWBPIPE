import { describe, expect, it } from "vitest";
import type { Diagnostic } from "../../types";
import { buildIssueList, countIssues, issueCountLine, sameFact, type IssueList } from "./issueRows";
import { BLANK_SOLVE_FINDINGS, blankSession, checkRun, DEMO_LEADING_CHECK_FINDINGS, demoSession, knowledgeDiagnostics } from "./issueRows.testSessions";

/** The count as it reads from the rows themselves. */
function recount(list: IssueList) {
  const tier = (value: string) => list.rows.filter((row) => row.tier === value).length;
  return { toFix: tier("solve"), modelValidity: tier("modelValidity"), ruleCheck: tier("ruleCheck"), warnings: tier("warning"), notes: tier("note"), total: list.rows.length };
}

const KINDS = ["solve", "modelValidity", "ruleCheck", "warning", "note"];

describe("buildIssueList: the blank model", () => {
  it("lists the solve's three findings first, each with the act that repairs it, and counts 3 to fix", () => {
    const list = buildIssueList(blankSession());
    const fix = list.rows.filter((row) => row.tier === "solve");
    expect(fix.map((row) => row.code)).toEqual(["NODE_INPUT_MISSING", "PIPE_INPUT_MISSING", "LOAD_INPUT_MISSING"]);
    expect(list.rows.slice(0, 3)).toEqual(fix);
    expect(fix.map((row) => row.act)).toEqual([
      { command: "insert.node", label: "Add a node" },
      { command: "insert.pipe", label: "Add a pipe" },
      { command: "insert.load", label: "Add a load" }
    ]);
    expect(fix.map((row) => row.message)).toEqual([
      "The model needs at least two nodes before it can be solved.",
      "The model needs at least one pipe segment before it can be solved.",
      "The model needs at least one load before it can be solved; no hidden loads are applied."
    ]);
    // The engine's own words stay on each row's entry.
    expect(fix.map((row) => row.sources[0]!.diagnostic.message)).toEqual(BLANK_SOLVE_FINDINGS.map((item) => item.message));
    // They name a collection, not an entity a person could select.
    expect(fix.every((row) => row.entity === null)).toBe(true);
    const count = countIssues(list);
    expect(count.toFix).toBe(3);
    expect(issueCountLine(count)).toBe("3 to fix before Run. 2 warnings, 1 note.");
  });

  it("does not count as to fix a blocking finding that does not withhold Run", () => {
    const list = buildIssueList(blankSession());
    const target = list.rows.find((row) => row.code === "BLANK_PROJECT_AUTHORING_TARGET")!;
    expect(target.severity).toBe("blocking");
    expect(target.tier).toBe("warning");
    // The blank model's own statement, then the knowledge's warning and note.
    expect(list.rows.map((row) => row.code)).toEqual([
      "NODE_INPUT_MISSING", "PIPE_INPUT_MISSING", "LOAD_INPUT_MISSING",
      "BLANK_PROJECT_AUTHORING_TARGET", "SUPPORT_STIFFNESS_UNRESOLVED", "RULE_CHECK_NOT_PERFORMED"
    ]);
    expect(countIssues(list)).toEqual({ ...recount(list), checked: true, notListedToFix: 0 });
    expect(recount(list)).toEqual({ toFix: 3, modelValidity: 0, ruleCheck: 0, warnings: 2, notes: 1, total: 6 });
  });

  it("lists a row with no entity and no act as it stands", () => {
    const list = buildIssueList(blankSession());
    const note = list.rows.find((row) => row.code === "RULE_CHECK_NOT_PERFORMED")!;
    expect(note).toMatchObject({ tier: "note", entity: null, act: null, members: [], solverMessage: false });
    expect(note.message).toBe(knowledgeDiagnostics.find((item) => item.code === "RULE_CHECK_NOT_PERFORMED")!.message);
  });

  it("knows nothing to fix only from checks of this model value", () => {
    const session = blankSession();
    // Before the checks run, and after the model changes, "to fix before Run" is not known.
    for (const run of [null, checkRun({ ...session.model }, BLANK_SOLVE_FINDINGS)]) {
      const list = buildIssueList({ ...session, modelCheckRun: run });
      expect(list.rows.some((row) => row.tier === "solve")).toBe(false);
      expect(list.checked).toBe(false);
      expect(issueCountLine(countIssues(list))).toBe("Model not checked. 2 warnings, 1 note.");
    }
  });

  it("lists the solve's refusal to read the model as one to fix, and the counted findings not listed", () => {
    const session = blankSession();
    const refused = { ...checkRun(session.model, []), solve: { kind: "refused" as const, refusal: "invented refusal text" } };
    const list = buildIssueList({ ...session, modelCheckRun: refused });
    expect(list.rows[0]).toMatchObject({ tier: "solve", code: "", message: "invented refusal text", act: null });
    const bounded = { ...checkRun(session.model, BLANK_SOLVE_FINDINGS), notShownBlockingByGroup: { solve: 2, slots: 0, library: 0 } };
    expect(issueCountLine(countIssues(buildIssueList({ ...session, modelCheckRun: bounded })))).toBe("3 to fix before Run, and 2 more not listed. 2 warnings, 1 note.");
  });
});

describe("buildIssueList: the demonstration solve", () => {
  it("lists SH-140's three hanger-load warnings as one row naming the three loads", () => {
    const session = demoSession();
    const list = buildIssueList(session);
    const rows = list.rows.filter((row) => row.code === "SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD");
    expect(rows).toHaveLength(1);
    const row = rows[0]!;
    expect(row.entity).toEqual({ type: "support", id: "support:SH-140" });
    expect(row.members).toEqual(["hanger.installed_load", "hanger.cold_load", "hanger.hot_load"]);
    expect(row.message).toBe(
      "Variable spring hanger support:SH-140: entered installed load (hanger.installed_load), cold load (hanger.cold_load) and hot load (hanger.hot_load) — not applied as preload; treated as a linear spring from zero load (review evidence only)."
    );
    // Each load arrived twice: from the model checks and in the result.
    expect(row.sources.map((source) => source.route)).toEqual(["model-check", "result", "model-check", "result", "model-check", "result"]);
    expect(row.solverMessage).toBe(true);
    expect(row.tier).toBe("warning");
  });

  it("lists each fact the checks and the result both state once", () => {
    const session = demoSession();
    const list = buildIssueList(session);
    const bend = list.rows.filter((row) => row.code === "BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD");
    expect(bend).toHaveLength(1);
    expect(bend[0]!.sources.map((source) => source.route)).toEqual(["model-check", "result"]);
    // The result's presented copy quotes the text as supplied; the row does not read from it.
    expect(bend[0]!.sources[1]!.diagnostic.message.startsWith("Supplied diagnostic text, unverified")).toBe(true);
    // The row reads in a plain sentence formed from the model's bend, not from the quoted copy.
    expect(bend[0]!.message).toBe("component:C-110 is a geometry-only bend solved as a straight chord without flexibility factor k (scalar SIF is applied as equal-factor intensification). Realize the bend to apply k.");
    expect(bend[0]!.sources[0]!.diagnostic.message).toBe(session.resultDiagnostics.find((item) => item.code === "BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD")!.message);
    // One row per fact: the result's, the model's and the knowledge's diagnostics, less the
    // leading findings the checks state again, less the two hanger loads joined to the third.
    const expected = session.resultDiagnostics.length + session.model.diagnostics.length + knowledgeDiagnostics.length - 2;
    expect(list.rows).toHaveLength(expected);
    expect(DEMO_LEADING_CHECK_FINDINGS).toBe(7);
  });

  it("orders warnings by the entity in the model's order, rows that name none first, and counts that same list", () => {
    const list = buildIssueList(demoSession());
    const warnings = list.rows.filter((row) => row.tier === "warning");
    expect(warnings.map((row) => [row.code, row.entity?.id ?? null])).toEqual([
      ["RULE_INPUTS_MISSING", null],
      ["NONLINEAR_COMBINATION_REQUIRES_SOLVE", null],
      ["RULE_CHECK_INPUTS_MISSING", null],
      ["HIGH_DISPLACEMENT_REVIEW", "node:N-140"],
      ["SUPPORT_STIFFNESS_UNRESOLVED", "support:S-120"],
      ["SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD", "support:SH-140"],
      ["SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED", "support:CE-120"],
      ["BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD", "component:C-110"]
    ]);
    expect(list.rows.map((row) => row.tier)).toEqual([...list.rows.map((row) => row.tier)].sort((a, b) => KINDS.indexOf(a) - KINDS.indexOf(b)));
    const count = countIssues(list);
    expect(count).toEqual({ ...recount(list), checked: true, notListedToFix: 0 });
    expect(issueCountLine(count)).toBe(`Nothing to fix before Run. ${count.warnings} warnings, ${count.notes} notes.`);
  });
});

describe("the same fact by two routes", () => {
  const library: Diagnostic = { id: "library-ref:LIBRARY_REF_UNRESOLVED:support:S-120", code: "LIBRARY_REF_UNRESOLVED", severity: "warning", message: "invented library text", affected_refs: ["support:S-120"], source: "invented" };

  it("is one row: a library reference the checks and the store's findings both list", () => {
    const session = demoSession();
    const run = checkRun(session.model, [], [{ group: "library", diagnostic: library }]);
    const list = buildIssueList({ model: session.model, modelCheckRun: run, libraryRefDiagnostics: [{ ...library }] });
    const rows = list.rows.filter((row) => row.code === "LIBRARY_REF_UNRESOLVED");
    expect(rows).toHaveLength(1);
    expect(rows[0]!.sources.map((source) => [source.route, source.group ?? null])).toEqual([["model-check", "library"], ["library", null]]);
    expect(rows[0]!.entity).toEqual({ type: "support", id: "support:S-120" });
  });

  it("is the same code and references with the same id or the same message, and nothing less", () => {
    expect(sameFact(library, { ...library, id: undefined })).toBe(true);
    expect(sameFact(library, { ...library, message: "another text" })).toBe(true);
    expect(sameFact(library, { ...library, id: "another", message: "another text" })).toBe(false);
    expect(sameFact(library, { ...library, affected_refs: ["support:S-130"] })).toBe(false);
    expect(sameFact(library, { ...library, code: "ANOTHER_CODE" })).toBe(false);
  });
});

describe("the five kinds of PRD §14.4", () => {
  // One finding of each kind on the blank model, by the route that carries it.
  function session() {
    const base = blankSession();
    const slot: Diagnostic = { id: "model-document:INVENTED_SLOT:pipes:0", code: "INVENTED_SLOT", severity: "blocking", message: "invented slot text (at pipes)", affected_refs: [], source: "invented" };
    const ruleClass: Diagnostic = { id: "invented:rule-class", code: "INVENTED_RULE_INPUT", diagnostic_class: "RULE_CHECK_BLOCKING", severity: "blocking", message: "invented rule input text" };
    const ruleFlag = { id: "invented:rule-flag", code: "INVENTED_RULE_FLAG", severity: "warning", message: "invented flagged text", blocks_rule_check: true } as Diagnostic;
    const model = { ...base.model, diagnostics: [...base.model.diagnostics, ruleClass] };
    return {
      ...base,
      model,
      modelCheckRun: checkRun(model, BLANK_SOLVE_FINDINGS.slice(0, 1), [{ group: "slots", diagnostic: slot }]),
      documentDiagnostics: [{ ...slot }],
      knowledgeDiagnostics: [...knowledgeDiagnostics, ruleFlag]
    };
  }

  it("puts each row in one kind, in display order, and counts each kind on its own", () => {
    const list = buildIssueList(session());
    expect(list.rows.map((row) => [row.tier, row.code])).toEqual([
      ["solve", "NODE_INPUT_MISSING"],
      // The checks' slot finding and the validator's are one fact, which blocks no solve.
      ["modelValidity", "INVENTED_SLOT"],
      ["ruleCheck", "INVENTED_RULE_INPUT"],
      ["ruleCheck", "INVENTED_RULE_FLAG"],
      ["warning", "BLANK_PROJECT_AUTHORING_TARGET"],
      ["warning", "SUPPORT_STIFFNESS_UNRESOLVED"],
      ["note", "RULE_CHECK_NOT_PERFORMED"]
    ]);
    expect(list.rows[1]!.sources.map((source) => source.route)).toEqual(["model-check", "document"]);
    const count = countIssues(list);
    expect(count).toEqual({ ...recount(list), checked: true, notListedToFix: 0 });
    expect(count.toFix + count.modelValidity + count.ruleCheck + count.warnings + count.notes).toBe(count.total);
    expect(issueCountLine(count)).toBe("1 to fix before Run. 1 model validity finding, 2 blocking the rule check, 2 warnings, 1 note.");
  });

  it("names a kind that blocks something even when nothing withholds Run", () => {
    // The checks list nothing; the validator's blocking slot finding still stands on its own route.
    const base = session();
    const list = buildIssueList({ ...base, modelCheckRun: checkRun(base.model, []) });
    expect(issueCountLine(countIssues(list))).toBe("Nothing to fix before Run. 1 model validity finding, 2 blocking the rule check, 2 warnings, 1 note.");
    // With nothing listed at all, there is nothing to fix.
    const empty = { ...base.model, diagnostics: [] };
    expect(issueCountLine(countIssues(buildIssueList({ model: empty, modelCheckRun: checkRun(empty, []) })))).toBe("Nothing to fix.");
  });

  it("carries the class a producer supplied, and invents none", () => {
    const list = buildIssueList(session());
    expect(list.rows.find((row) => row.code === "INVENTED_RULE_INPUT")!.classes).toEqual(["RULE_CHECK_BLOCKING"]);
    expect(list.rows.find((row) => row.code === "INVENTED_RULE_FLAG")!.classes).toEqual([]);
    expect(list.rows.find((row) => row.code === "NODE_INPUT_MISSING")!.classes).toEqual([]);
  });
});

describe("plain sentences on the list", () => {
  const engine = "region requires equal normalized bore; radius difference exceeds 64*epsilon*radius_scale (scale=5e-2 m); this is a representation guard, not a physical reducer allowance";
  const bore: Diagnostic = { id: "diagnostic:pressure-runtime:invented:PRESSURE_REGION_BORE_MISMATCH", code: "PRESSURE_REGION_BORE_MISMATCH", severity: "blocking", message: engine, affected_refs: ["load:L-100", "region:R-1", "pipe:P-110"], source: "core/product_physics" };

  it("shows the plain sentence and keeps the engine's message among the row's entries", () => {
    const session = demoSession();
    const list = buildIssueList({ model: session.model, modelCheckRun: checkRun(session.model, [bore]) });
    const row = list.rows.find((item) => item.code === "PRESSURE_REGION_BORE_MISMATCH")!;
    expect(row.message).toBe("pipe:P-110 has a different pressure bore from the rest of region region:R-1 of load:L-100. End the region where the bore changes.");
    expect(row.sources.map((source) => source.diagnostic.message)).toEqual([engine]);
    expect(row.tier).toBe("solve");
  });

  it("leaves text the result quotes as supplied in its quotation", () => {
    const session = demoSession();
    const quoted = { ...bore, message: `Supplied diagnostic text, unverified (not a combination certificate statement): "${engine}"` };
    const list = buildIssueList({ model: session.model, resultDiagnostics: [bore], presentedResultDiagnostics: [quoted] });
    expect(list.rows.find((row) => row.code === "PRESSURE_REGION_BORE_MISMATCH")!.message).toBe(quoted.message);
  });
});
