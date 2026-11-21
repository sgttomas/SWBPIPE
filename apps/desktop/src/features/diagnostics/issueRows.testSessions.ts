// Two sessions for the Issues list's tests, from the maintained fixtures and builders:
// - the blank model as File > New builds it (`buildBlankLocalModelDocument`), with the design
//   knowledge fixture the browser preview loads and the solve checks' three findings for it, as
//   core/product_physics/src/validation.rs `validate_required_collections` states them;
// - the demonstration model and its preview-physics result
//   (fixtures/product_preview/invented_demo_*), with the knowledge fixture and a model-check run
//   whose findings are the result's leading solve diagnostics (the checks' findings equal them,
//   DEL-04-06 AC-008), and the result's diagnostics presented as the Diagnostics list presents
//   them (`suppliedDiagnosticMessage` on its combination).
import demoModelFixture from "../../../../../fixtures/product_preview/invented_demo_model.json";
import demoResultFixture from "../../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";
import knowledgeFixture from "../../../../../fixtures/product_preview/invented_design_knowledge.json";
import { buildBlankLocalModelDocument } from "../../services/projectService";
import type { Diagnostic, PreviewModel } from "../../types";
import { documentCombinationIdentities, suppliedDiagnosticMessage } from "../results/suppliedDiagnostics";
import { resolveModelCheckEntity, type ModelCheckFinding, type ModelCheckRun } from "./modelChecks";
import type { IssueInputs } from "./issueRows";

export const knowledgeDiagnostics = (knowledgeFixture as unknown as { diagnostics: Diagnostic[] }).diagnostics;

export function checkRun(model: PreviewModel, solve: readonly Diagnostic[], others: readonly Pick<ModelCheckFinding, "group" | "diagnostic">[] = []): ModelCheckRun {
  const findings = [...solve.map((diagnostic) => ({ group: "solve" as const, diagnostic })), ...others].map((item, index): ModelCheckFinding => ({
    key: `${item.group}:${index}:${item.diagnostic.id ?? item.diagnostic.code}`,
    group: item.group,
    diagnostic: item.diagnostic,
    entity: (item.diagnostic.affected_refs ?? []).map((ref) => resolveModelCheckEntity(model, ref)).find(Boolean) ?? null
  }));
  return {
    model, solve: { kind: "checked" }, findings,
    notShown: { solve: 0, slots: 0, library: 0 }, notShownBlocking: 0, notShownBlockingByGroup: { solve: 0, slots: 0, library: 0 }
  };
}

/** The solve's three findings on a model with no pipe and no load (validation.rs). */
export const BLANK_SOLVE_FINDINGS: readonly Diagnostic[] = [
  { id: "diagnostic:physics:nodes-missing", code: "NODE_INPUT_MISSING", severity: "blocking", message: "linear-static preview requires at least two explicit nodes", affected_refs: ["nodes"], source: "core/product_physics" },
  { id: "diagnostic:physics:pipes-missing", code: "PIPE_INPUT_MISSING", severity: "blocking", message: "linear-static preview requires at least one explicit pipe segment", affected_refs: ["pipe_segments"], source: "core/product_physics" },
  { id: "diagnostic:physics:loads-missing", code: "LOAD_INPUT_MISSING", severity: "blocking", message: "linear-static preview requires at least one explicit primitive load; no hidden loads are applied", affected_refs: ["load_cases"], source: "core/product_physics" }
];

export function blankSession(): IssueInputs {
  const model = buildBlankLocalModelDocument(new Date("2026-11-21T00:00:00Z"));
  return { model, modelCheckRun: checkRun(model, BLANK_SOLVE_FINDINGS), knowledgeDiagnostics };
}

/** How many of the demonstration result's diagnostics the solve's checks state before a solve. */
export const DEMO_LEADING_CHECK_FINDINGS = 7;

export function demoSession(): IssueInputs & { resultDiagnostics: readonly Diagnostic[] } {
  const model = structuredClone(demoModelFixture) as unknown as PreviewModel;
  const result = structuredClone(demoResultFixture) as unknown as { diagnostics: Diagnostic[] };
  const combinations = documentCombinationIdentities(result as never);
  const presented = result.diagnostics.map((diagnostic) => ({ ...diagnostic, message: suppliedDiagnosticMessage(diagnostic, combinations, false) }));
  return {
    model,
    modelCheckRun: checkRun(model, result.diagnostics.slice(0, DEMO_LEADING_CHECK_FINDINGS)),
    knowledgeDiagnostics,
    resultDiagnostics: result.diagnostics,
    presentedResultDiagnostics: presented
  };
}
