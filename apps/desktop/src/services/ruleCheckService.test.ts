import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import {
  deriveRuleCheckBindingPlan,
  loadDemoRuleCheckPack,
  runRuleChecks,
  RULE_CHECK_BACKEND_DIAGNOSTIC
} from "./ruleCheckService";
import type { RulePackDocument } from "./rulePackService";

// Phase C4 GUI slice (TP-C4-CHECKGUI-001). jsdom has no Tauri runtime, so the
// run route pins the honest browser-preview seam (explicit desktop-only
// diagnostic, no synthesized fallback) by default; tests that exercise the
// backend path opt in by setting __TAURI_INTERNALS__ and mocking invoke. The
// binding-plan deriver is pure and tested directly.

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
const invokeMock = vi.mocked(invoke);

beforeEach(() => {
  invokeMock.mockReset();
});

afterEach(() => {
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("deriveRuleCheckBindingPlan", () => {
  it("partitions required inputs by source kind and collects value slots", () => {
    const pack: RulePackDocument = {
      required_inputs: [
        {
          input_id: "actual",
          name: "Actual",
          source_kind: "solver_result",
          quantity_intent: { dimension: "stress", unit_ref: "demo_unit" }
        },
        {
          input_id: "limit",
          name: "Limit",
          source_kind: "user_supplied_rule_value",
          quantity_intent: { dimension: "stress", unit_ref: "demo_unit" }
        },
        {
          input_id: "lib_allowable",
          name: "Library allowable",
          source_kind: "private_library_value",
          quantity_intent: { dimension: "stress", unit_ref: "demo_unit" }
        }
      ],
      value_slots: [
        {
          slot_id: "ratio_limit",
          slot_kind: "ratio_limit",
          quantity_intent: { dimension: "dimensionless", unit_ref: "ratio" }
        }
      ]
    };

    const plan = deriveRuleCheckBindingPlan(pack);

    expect(plan.solverInputs).toEqual([
      { input_id: "actual", name: "Actual", dimension: "stress", unit_ref: "demo_unit" }
    ]);
    expect(plan.valueInputs).toEqual([
      {
        ref_id: "limit",
        name: "Limit",
        source_kind: "user_supplied_rule_value",
        dimension: "stress",
        unit_ref: "demo_unit"
      }
    ]);
    expect(plan.libraryInputs).toEqual([{ input_id: "lib_allowable", name: "Library allowable" }]);
    expect(plan.valueSlots).toEqual([
      { slot_id: "ratio_limit", slot_kind: "ratio_limit", dimension: "dimensionless", unit_ref: "ratio" }
    ]);
  });

  it("returns an empty plan for a document with no inputs or slots", () => {
    const plan = deriveRuleCheckBindingPlan({});
    expect(plan.solverInputs).toHaveLength(0);
    expect(plan.valueInputs).toHaveLength(0);
    expect(plan.valueSlots).toHaveLength(0);
    expect(plan.libraryInputs).toHaveLength(0);
  });

  it("falls back to TBD dimension/unit and the id as name when metadata is absent", () => {
    const plan = deriveRuleCheckBindingPlan({
      required_inputs: [{ input_id: "x", source_kind: "user_supplied_rule_value" }]
    });
    expect(plan.valueInputs[0]).toEqual({
      ref_id: "x",
      name: "x",
      source_kind: "user_supplied_rule_value",
      dimension: "TBD",
      unit_ref: "TBD"
    });
  });
});

describe("runRuleChecks", () => {
  it("reports the desktop-only unavailable route in browser preview (no Tauri)", async () => {
    const route = await runRuleChecks({ rulePackDocument: { metadata: { rule_pack_id: "x" } } });
    expect(route.route).toBe("unavailable_browser_preview");
    if (route.route === "unavailable_browser_preview") {
      expect(route.diagnostic).toBe(RULE_CHECK_BACKEND_DIAGNOSTIC);
    }
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("invokes run_rule_checks with omitted empty bindings when the backend is present", async () => {
    invokeMock.mockResolvedValue({
      document_kind: "openpipestress.rule_check.run",
      rule_pack_id: "x",
      grammar_version: "1.0.0",
      aggregate_status: "RULE_INPUTS_INCOMPLETE",
      checks: [],
      professional_boundary_notice: "notice"
    });
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};

    const route = await runRuleChecks({
      rulePackDocument: { metadata: { rule_pack_id: "x" } },
      solverResultBindings: [],
      suppliedValueBindings: []
    });

    expect(route.route).toBe("tauri_backend");
    expect(invokeMock).toHaveBeenCalledTimes(1);
    const [command, args] = invokeMock.mock.calls[0];
    expect(command).toBe("run_rule_checks");
    expect(args).toHaveProperty("rulePackDocument");
    // Empty binding arrays are omitted (backend treats absent as unsupplied).
    expect(args).not.toHaveProperty("solverResultBindings");
    expect(args).not.toHaveProperty("suppliedValueBindings");
  });

  it("forwards non-empty bindings and prefers a solved envelope over the model", async () => {
    invokeMock.mockResolvedValue({
      document_kind: "openpipestress.rule_check.run",
      rule_pack_id: "x",
      grammar_version: "1.0.0",
      aggregate_status: "USER_RULE_CHECKED",
      checks: [],
      professional_boundary_notice: "notice"
    });
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};

    await runRuleChecks({
      rulePackDocument: { metadata: { rule_pack_id: "x" } },
      model: { project: { id: "p" } } as never,
      solvedEnvelope: { run_id: "run:1", results: [] } as never,
      solverResultBindings: [{ input_id: "actual", result_id: "result:stress:demo" }],
      suppliedValueBindings: [{ ref_id: "limit", value: 100, unit: "demo_unit", dimension: "stress" }]
    });

    const [, args] = invokeMock.mock.calls[0];
    const typed = args as Record<string, unknown>;
    expect(typed).toHaveProperty("solvedEnvelope");
    expect(typed).not.toHaveProperty("model");
    expect(typed.solverResultBindings).toEqual([{ input_id: "actual", result_id: "result:stress:demo" }]);
    expect(typed.suppliedValueBindings).toEqual([
      { ref_id: "limit", value: 100, unit: "demo_unit", dimension: "stress" }
    ]);
  });
});

describe("loadDemoRuleCheckPack", () => {
  it("loads the bundled invented demonstration pack with a runnable shape", async () => {
    const demo = (await loadDemoRuleCheckPack()) as Record<string, unknown>;
    const metadata = demo.metadata as Record<string, unknown>;
    expect(metadata.rule_pack_id).toBe("invented_demo_rule_pack");
    expect(demo.grammar_version).toBe("1.0.0");
    const plan = deriveRuleCheckBindingPlan(demo);
    expect(plan.solverInputs.map((input) => input.input_id)).toContain("demo_actual_quantity");
    expect(plan.valueInputs.map((input) => input.ref_id)).toContain("demo_limit_quantity");
    expect(plan.valueSlots.map((slot) => slot.slot_id)).toContain("demo_limit_slot");
  });
});
