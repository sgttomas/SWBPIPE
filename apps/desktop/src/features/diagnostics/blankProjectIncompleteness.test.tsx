// What a person sees of a new blank project's incompleteness, and of the same project once a
// model has been entered (DEL-02-05 ontology, *Blank project*; DEL-04-06 REQ-013). The blank
// document stores no diagnostic of its own: the model checks compute that it is incomplete, and
// they stop saying so once the model is entered. The entered model is the W9-A L frame applied to
// the desktop's blank by one batch (fixtures/model_operations/agent_w9a_journey.json). Invented data.
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it } from "vitest";
import fixture from "../../../../../fixtures/model_operations/agent_w9a_journey.json";
import type { PreviewModel } from "../../types";
import { buildBlankLocalModelDocument } from "../../services/projectService";
import { computeModelHash } from "../../services/hashService";
import { stampAgentOperations } from "../../services/embeddedAgentSession";
import { applyOperationBatch } from "../../services/operationBatchService";
import { warmupOperationEngine } from "../../services/operationService";
import { initialSolveJob } from "../workspace/solveJobAudit";
import { SolvePanel } from "../solve/SolvePanel";
import { DiagnosticsPanel } from "./DiagnosticsPanel";
import { runModelChecks } from "./modelChecks";

afterEach(cleanup);
beforeAll(async () => {
  expect((await warmupOperationEngine()).state).toBe("ready");
});

const isBlocking = (severity: string) => severity === "blocking" || severity === "error";

async function enteredModel(blank: PreviewModel): Promise<PreviewModel> {
  const { proposalHandle, requestRef } = fixture.host;
  const hash = await computeModelHash(blank);
  const batch = {
    batch_id: `agent-${proposalHandle}`,
    operations: stampAgentOperations(fixture.routes.PF.operations as Record<string, unknown>[], proposalHandle, requestRef),
    source_model_hash: hash!.value
  };
  const outcome = await applyOperationBatch(blank, batch, hash);
  expect(outcome.validation.batch_validation_status).toBe("passed");
  return outcome.applied_model!;
}

function renderSurfaces(model: PreviewModel) {
  render(
    <>
      <SolvePanel model={model} result={null} analysisRun={null} running={false} solveJob={initialSolveJob()}
        solverMode="sparse_interactive" onRun={() => {}} onCancel={() => {}} onSolverModeChange={() => {}} />
      <DiagnosticsPanel model={model} knowledge={null} result={null} selectedDiagnosticId={null} onSelectDiagnostic={() => {}} />
    </>
  );
}

describe("a blank project's incompleteness", () => {
  it("is stated by the model checks, not by a diagnostic stored in the blank document", async () => {
    const blank = buildBlankLocalModelDocument(new Date(fixture.blank.createdAt));
    expect(blank.diagnostics).toEqual([]);

    const checks = await runModelChecks(blank);
    expect(checks.solve).toEqual({ kind: "checked" });
    // The solve's own input checks: no nodes, no pipe and no load, each blocking.
    const blocking = checks.findings.filter((item) => isBlocking(item.diagnostic.severity));
    expect(blocking.map((item) => [item.group, item.diagnostic.code])).toEqual([
      ["solve", "NODE_INPUT_MISSING"], ["solve", "PIPE_INPUT_MISSING"], ["solve", "LOAD_INPUT_MISSING"]
    ]);

    renderSurfaces(blank);
    // The Solve panel: the mechanics row shows the blank's authored label; nothing stored to count.
    expect(screen.getByTestId("readiness-mechanics")).toHaveTextContent("preview run not started; ready for preview diagnostics");
    expect(screen.getByTestId("readiness-diagnostics")).toHaveTextContent("0 diagnostics; 0 warnings; 0 blocking/error");
    // The Diagnostics panel lists no stored record of the blank.
    expect(screen.queryByTestId("diagnostic-BLANK_PROJECT_AUTHORING_TARGET")).toBeNull();
  });

  it("is no longer stated once the model is entered", async () => {
    const blank = buildBlankLocalModelDocument(new Date(fixture.blank.createdAt));
    const model = await enteredModel(blank);
    expect(model.diagnostics).toEqual([]);

    const checks = await runModelChecks(model);
    expect(checks.solve).toEqual({ kind: "checked" });
    expect(checks.findings.filter((item) => isBlocking(item.diagnostic.severity))).toEqual([]);

    renderSurfaces(model);
    expect(screen.getByTestId("readiness-diagnostics")).toHaveTextContent("0 blocking/error");
    expect(screen.getByTestId("diagnostics-panel").querySelectorAll(".diagnostic.blocking, .diagnostic.error")).toHaveLength(0);
  });
});
