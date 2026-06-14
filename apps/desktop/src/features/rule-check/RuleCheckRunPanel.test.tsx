import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { RuleCheckRunPanel } from "./RuleCheckRunPanel";
import type { MechanicsResult, PreviewModel } from "../../types";

// Phase C4 GUI slice (TP-C4-CHECKGUI-001). jsdom has no Tauri runtime, so the
// browser-preview tests pin the honest desktop-only seam; the desktop-render
// test opts into the backend path by setting __TAURI_INTERNALS__ and mocking
// invoke so the per-check outcome rendering can be asserted.

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
const invokeMock = vi.mocked(invoke);

const modelStub = {
  project: { id: "project:c4-run-test", name: "Invented C4 Run Test Project" }
} as unknown as PreviewModel;

const resultStub = {
  run_id: "run:c4-test",
  status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" },
  results: [
    { id: "result:stress:demo", kind: "stress", value: 50, unit: "demo_unit", entity_ref: "pipe:demo" }
  ]
} as unknown as MechanicsResult;

beforeEach(() => {
  invokeMock.mockReset();
});

afterEach(() => {
  cleanup();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("RuleCheckRunPanel", () => {
  it("surfaces project scope and the no-solve reason honestly", () => {
    render(<RuleCheckRunPanel model={null} result={null} />);
    expect(screen.getByTestId("rule-check-run-scope").textContent).toContain(
      "create or open a local project first"
    );
    expect(screen.getByTestId("rule-check-run-solve-status").textContent).toContain(
      "No solved mechanics result"
    );
    cleanup();

    render(<RuleCheckRunPanel model={modelStub} result={resultStub} />);
    expect(screen.getByTestId("rule-check-run-scope").textContent).toContain("project:c4-run-test");
    expect(screen.getByTestId("rule-check-run-solve-status").textContent).toContain("1 result row");
  });

  it("disables Run until a pack is loaded", () => {
    render(<RuleCheckRunPanel model={modelStub} result={resultStub} />);
    expect(screen.getByTestId("rule-check-run")).toHaveProperty("disabled", true);
  });

  it("loads the bundled demo pack and derives binding controls", async () => {
    render(<RuleCheckRunPanel model={modelStub} result={resultStub} />);
    fireEvent.click(screen.getByTestId("rule-check-load-demo"));

    await screen.findByTestId("rule-check-binding-plan");
    // solver_result input -> result-row select; user value input + value slot -> entry fields.
    expect(screen.getByTestId("rule-check-solver-select-demo_actual_quantity")).toBeInTheDocument();
    expect(screen.getByTestId("rule-check-value-input-demo_limit_quantity")).toBeInTheDocument();
    expect(screen.getByTestId("rule-check-slot-input-demo_limit_slot")).toBeInTheDocument();
    // The solved result row is offered as a binding option.
    expect(
      screen.getByTestId("rule-check-solver-select-demo_actual_quantity").textContent
    ).toContain("result:stress:demo");
    expect(screen.getByTestId("rule-check-run")).toHaveProperty("disabled", false);
  });

  it("reports a JSON parse error for an invalid pasted pack", () => {
    render(<RuleCheckRunPanel model={modelStub} result={resultStub} />);
    fireEvent.change(screen.getByTestId("rule-check-pack-json"), { target: { value: "{not valid json" } });
    expect(screen.getByTestId("rule-check-pack-parse-error").textContent).toContain(
      "RULE-CHECK-PACK-JSON-INVALID"
    );
    expect(screen.getByTestId("rule-check-run")).toHaveProperty("disabled", true);
  });

  it("reports the desktop-only backend seam when run in browser preview", async () => {
    render(<RuleCheckRunPanel model={modelStub} result={resultStub} />);
    fireEvent.click(screen.getByTestId("rule-check-load-demo"));
    await screen.findByTestId("rule-check-binding-plan");

    fireEvent.click(screen.getByTestId("rule-check-run"));
    await waitFor(() =>
      expect(screen.getByTestId("rule-check-run-status").textContent).toContain(
        "RULE-CHECK-BACKEND-DESKTOP-ONLY"
      )
    );
    expect(screen.queryByTestId("rule-check-run-result")).not.toBeInTheDocument();
  });

  it("renders per-check outcomes and the aggregate from a desktop run", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue({
      document_kind: "openpipestress.rule_check.run",
      rule_pack_id: "invented_demo_rule_pack",
      grammar_version: "1.0.0",
      aggregate_status: "USER_RULE_FAILED",
      checks: [
        {
          check_id: "demo_training_check",
          status: "USER_RULE_FAILED",
          computed_value: { value: 1.5, dimension: "dimensionless", unit_ref: "ratio" },
          limit_value: { value: 1, dimension: "dimensionless", unit_ref: "ratio" },
          acceptability_relation: "less_than_or_equal",
          bound_inputs: [
            { input_id: "demo_actual_quantity", source_kind: "solver_result", supplied: true },
            { input_id: "demo_limit_quantity", source_kind: "user_supplied_rule_value", supplied: true }
          ],
          completeness_findings: [],
          evaluator_findings: [],
          diagnostic_codes: []
        }
      ],
      professional_boundary_notice: "Software rule-check evidence only; not a professional claim."
    });

    render(<RuleCheckRunPanel model={modelStub} result={resultStub} />);
    fireEvent.click(screen.getByTestId("rule-check-load-demo"));
    await screen.findByTestId("rule-check-binding-plan");

    fireEvent.click(screen.getByTestId("rule-check-run"));

    await screen.findByTestId("rule-check-run-result");
    expect(screen.getByTestId("rule-check-aggregate-status").textContent).toContain("USER_RULE_FAILED");
    expect(screen.getByTestId("rule-check-aggregate-status")).toHaveAttribute(
      "data-status",
      "USER_RULE_FAILED"
    );
    const outcome = screen.getByTestId("rule-check-outcome-demo_training_check");
    expect(outcome.textContent).toContain("USER_RULE_FAILED");
    expect(outcome.textContent).toContain("computed=1.5 ratio");
    expect(screen.getByTestId("rule-check-professional-boundary").textContent).toContain(
      "not a professional claim"
    );
    expect(invokeMock).toHaveBeenCalledWith("run_rule_checks", expect.objectContaining({ rulePackDocument: expect.anything() }));
  });
});
