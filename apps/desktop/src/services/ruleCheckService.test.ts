import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../test/nativeMechanicsReplay";
import { loadBundledMechanicsReference, loadPreviewModel, runPreviewMechanics } from "./previewService";
import {
  classifySolverResultReference,
  deriveRuleCheckBindingPlan,
  loadDemoRuleCheckPack,
  runRuleChecks,
  ruleBindingPrecheck,
  RULE_CHECK_BACKEND_DIAGNOSTIC
} from "./ruleCheckService";
import pressureSourceMixed from "../../../../fixtures/product_preview/pressure_source/physics_source/mixed-sparse_interactive.raw.json";
import previewInvented from "../../../../fixtures/results/preview_physics_invented_sparse.json";
import { retiredPrecisionStatement } from "../test-support/retiredPrecision";
import lLine from "../../../../fixtures/model_operations/pressure_v3_l_line_authoring.json";
import type { MechanicsResult, PreviewModel } from "../types";
import type { RulePackDocument } from "./rulePackService";

// Phase C4 GUI slice (TP-C4-CHECKGUI-001). jsdom has no Tauri runtime, so the
// run route pins the honest browser-preview seam (explicit desktop-only
// diagnostic, no synthesized fallback) by default; tests that exercise the
// backend path opt in by setting __TAURI_INTERNALS__ and mocking invoke. All
// native transport/rule responses below are unit simulations, NOT actual native
// UI qualification or rule-evaluation evidence. The
// binding-plan deriver is pure and tested directly.

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

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
          quantity_intent: { dimension: "stress", unit_ref: "demo_unit" },
          solver_result_ref: { result_id: "result:stress:demo" }
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
          quantity_intent: { dimension: "stress", unit_ref: "demo_unit" },
          library_value_ref: {
            library_kind: "material",
            library_id: "lib:x",
            record_id: "rec:y",
            slot_id: "slot:z"
          }
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
      {
        input_id: "actual",
        name: "Actual",
        dimension: "stress",
        unit_ref: "demo_unit",
        solver_result_ref: { result_id: "result:stress:demo" }
      }
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
    expect(plan.libraryInputs).toEqual([
      {
        input_id: "lib_allowable",
        name: "Library allowable",
        library_value_ref: {
          library_kind: "material",
          library_id: "lib:x",
          record_id: "rec:y",
          slot_id: "slot:z"
        }
      }
    ]);
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

describe("classifySolverResultReference", () => {
  const rows = [
    { id: "result:stress:demo", kind: "stress", value: 50, unit: "demo_unit", entity_ref: "pipe:demo" }
  ] as never;

  it("resolves when the authored result id is present", () => {
    expect(classifySolverResultReference(rows, "result:stress:demo")).toBe("resolves");
  });

  it("reports result_missing when result rows exist but the id is absent", () => {
    expect(classifySolverResultReference(rows, "result:absent")).toBe("result_missing");
  });

  it("reports no_result_rows when no solved result rows are available", () => {
    expect(classifySolverResultReference([], "result:stress:demo")).toBe("no_result_rows");
    expect(classifySolverResultReference(null, "result:stress:demo")).toBe("no_result_rows");
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

  it("performs a simulated native solve when needed and omits empty rule binding arrays", async () => {
    const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }), replay = createNativeMechanicsReplay({ profile: "preview" });
    const ruleResponse = {
      document_kind: "openpipestress.rule_check.run",
      rule_pack_id: "x",
      grammar_version: "1.0.0",
      aggregate_status: "RULE_INPUTS_INCOMPLETE",
      checks: [],
      professional_boundary_notice: "notice"
    };
    invokeMock.mockImplementation(async (command, args, options) => command === "run_rule_checks" ? ruleResponse : replay.invoke(command, args, options));
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};

    const route = await runRuleChecks({
      rulePackDocument: { metadata: { rule_pack_id: "x" } },
      model: pair.model,
      solverResultBindings: [],
      suppliedValueBindings: []
    });

    expect(route.route).toBe("tauri_backend");
    // The transient solve goes through the job path and is released after the rule run used it.
    expect(invokeMock.mock.calls.map(([command]) => command)).toEqual([
      "start_preview_mechanics_job", "poll_preview_mechanics_job", "take_preview_mechanics_job_result",
      "run_rule_checks", "dispose_preview_mechanics_result"
    ]);
    const [, body, options] = invokeMock.mock.calls[0];
    expect(JSON.parse(new TextDecoder().decode(body as Uint8Array))).toEqual(pair.model);
    expect(options).toEqual({ headers: { "x-swbpipe-solver-mode": "sparse_interactive" } });
    expect(replay.custody()).toEqual([]);
    const [command, args] = invokeMock.mock.calls[3];
    expect(command).toBe("run_rule_checks");
    expect(args).toHaveProperty("rulePackDocument");
    expect(args).toHaveProperty("model", pair.model);
    // The envelope travels as the JSON text Tauri itself sent for it.
    expect(args).toHaveProperty("solvedEnvelope", JSON.parse(JSON.stringify(pair.source)));
    // Empty binding arrays are omitted (backend treats absent as unsupplied).
    expect(args).not.toHaveProperty("solverResultBindings");
    expect(args).not.toHaveProperty("suppliedValueBindings");
  });

  it("forwards non-empty bindings and retains both solved envelope and model coverage basis", async () => {
    const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }), replay = createNativeMechanicsReplay({ profile: "preview" });
    const ruleResponse = {
      document_kind: "openpipestress.rule_check.run",
      rule_pack_id: "x",
      grammar_version: "1.0.0",
      aggregate_status: "USER_RULE_CHECKED",
      checks: [],
      professional_boundary_notice: "notice"
    };
    invokeMock.mockImplementation(async (command, args, options) => command === "run_rule_checks" ? ruleResponse : replay.invoke(command, args, options));
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};

    const solvedEnvelope = await runPreviewMechanics(pair.model);
    invokeMock.mockClear();
    await runRuleChecks({
      rulePackDocument: { metadata: { rule_pack_id: "x" } },
      model: pair.model,
      solvedEnvelope,
      solverResultBindings: [{ input_id: "actual", result_id: "result:stress:demo" }],
      suppliedValueBindings: [{ ref_id: "limit", value: 100, unit: "demo_unit", dimension: "stress" }],
      projectId: "project:lib"
    });

    const [, args] = invokeMock.mock.calls[0];
    const typed = args as Record<string, unknown>;
    expect(typed).toHaveProperty("solvedEnvelope");
    expect(typed).toHaveProperty("model", pair.model);
    expect(typed.solvedEnvelope).toStrictEqual(JSON.parse(JSON.stringify(solvedEnvelope)));
    expect(invokeMock).toHaveBeenCalledTimes(1);
    expect(typed.solverResultBindings).toEqual([{ input_id: "actual", result_id: "result:stress:demo" }]);
    expect(typed.suppliedValueBindings).toEqual([
      { ref_id: "limit", value: 100, unit: "demo_unit", dimension: "stress" }
    ]);
    expect(typed.projectId).toBe("project:lib");
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


describe("rule source provenance boundary in unit transport replay", () => {
  const rulePackDocument = { metadata: { rule_pack_id: "x" } };
  it("requires model coverage before invoking any native command", async () => {
    (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__ = {};
    await expect(runRuleChecks({rulePackDocument})).rejects.toThrow("RULE_NUMERICAL_CASE_COVERAGE_UNAVAILABLE");
    expect(invokeMock).not.toHaveBeenCalled();
  });
  it("rejects bundled references and unregistered imported source pairs", async () => {
    const reference = await loadBundledMechanicsReference(), model = await loadPreviewModel(), pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__ = {};
    await expect(runRuleChecks({rulePackDocument,model,solvedEnvelope:reference.source})).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    await expect(runRuleChecks({rulePackDocument,model:pair.model,solvedEnvelope:pair.source})).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    expect(invokeMock).not.toHaveBeenCalled();
  });
  it("rejects cloned source, source mutation and changed model without invoking rule evaluation", async () => {
    const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }), replay = createNativeMechanicsReplay({ profile: "preview" });
    (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(replay.invoke);
    const source = await runPreviewMechanics(pair.model);
    invokeMock.mockClear();
    await expect(runRuleChecks({rulePackDocument,model:pair.model,solvedEnvelope:structuredClone(source)})).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    const changed = structuredClone(pair.model); changed.nodes[0].position.x += 1;
    await expect(runRuleChecks({rulePackDocument,model:changed,solvedEnvelope:source})).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    source.results[0].unit = "altered";
    await expect(runRuleChecks({rulePackDocument,model:pair.model,solvedEnvelope:source})).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    expect(invokeMock).not.toHaveBeenCalled();
  });
  it("propagates a required native solve failure without invoking the rule backend", async () => {
    const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockRejectedValue(new Error("simulated native solve failure"));
    await expect(runRuleChecks({rulePackDocument,model:pair.model})).rejects.toThrow("simulated native solve failure");
    expect(invokeMock).toHaveBeenCalledTimes(1);
    expect(invokeMock.mock.calls[0][0]).toBe("start_preview_mechanics_job");
  });
  it("releases the transient solve result even when the rule backend fails", async () => {
    const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }), replay = createNativeMechanicsReplay({ profile: "preview" });
    (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(async (command, args, options) => {
      if (command === "run_rule_checks") throw new Error("simulated rule backend failure");
      return replay.invoke(command, args, options);
    });
    await expect(runRuleChecks({rulePackDocument,model:pair.model})).rejects.toThrow("simulated rule backend failure");
    expect(invokeMock.mock.calls.at(-1)?.[0]).toBe("dispose_preview_mechanics_result");
    expect(replay.custody()).toEqual([]);
    expect(replay.disposed()).toHaveLength(1);
  });
});


describe("T0R rule-binding pre-check (mirror of rule_binding_refusal; display only)", () => {
  const solverPack = (resultId?: string): RulePackDocument => ({
    required_inputs: [
      { input_id: "authored", name: "Authored", source_kind: "solver_result", ...(resultId ? { solver_result_ref: { result_id: resultId } } : {}) },
      { input_id: "manual", name: "Manual", source_kind: "solver_result" }
    ]
  });
  it("refuses no binding to a composite's rows (B4/D1 (b): the source-blocks-1 summary refusal is gone)", () => {
    const source = structuredClone(pressureSourceMixed) as unknown as MechanicsResult;
    const maximum = source.results.find((r) => r.kind === "pipe_elastic_normal_stress_maximum_v2")!;
    const force = source.results.find((r) => r.kind === "element_local_axial_force")!;
    expect(ruleBindingPrecheck(source, deriveRuleCheckBindingPlan(solverPack(maximum.id)), [{ input_id: "manual", result_id: force.id }])).toEqual([]);
  });
  it("gives a preview-physics-1 binding to a retired id no special finding (B3: a missing result); the new rows bind", () => {
    const source = structuredClone(previewInvented) as unknown as MechanicsResult;
    const findings = ruleBindingPrecheck(source, deriveRuleCheckBindingPlan(solverPack("result:reaction:support-S-100")), [{ input_id: "manual", result_id: "result:stress:pipe-P-120" }]);
    expect(findings).toEqual([]);
    const maximum = source.results.find((r) => r.kind === "pipe_elastic_normal_stress_maximum_v2")!;
    const intensified = source.results.find((r) => r.kind === "component_equal_factor_intensified_bending_stress_v1")!;
    expect(ruleBindingPrecheck(source, deriveRuleCheckBindingPlan(solverPack(maximum.id)), [{ input_id: "manual", result_id: intensified.id }])).toEqual([]);
  });
  it("refuses a stored precision-1 statement before invoking the rule backend (B1)", async () => {
    const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    const source = retiredPrecisionStatement();
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    await expect(runRuleChecks({ rulePackDocument: { metadata: { rule_pack_id: "x" } }, model: pair.model, solvedEnvelope: source })).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    expect(invokeMock).not.toHaveBeenCalled();
  });
  it("refuses a pressure-1 source before invoking the rule backend: the desktop's fresh set excludes it (b7-plan R2)", async () => {
    // T4-U2: the L line's pressure-1 result (invented data).
    const source = structuredClone(lLine.sparse_result) as unknown as MechanicsResult;
    const model = structuredClone(lLine.authored_model) as unknown as PreviewModel;
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    await expect(runRuleChecks({ rulePackDocument: { metadata: { rule_pack_id: "x" } }, model, solvedEnvelope: source })).rejects.toThrow(/^RULE_/);
    expect(invokeMock).not.toHaveBeenCalled();
  });
  it("refuses a transient pressure-1 native solve as not fresh and releases it without invoking the rule backend", async () => {
    const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" }), replay = createNativeMechanicsReplay({ profile: "pressure" });
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(replay.invoke);
    await expect(runRuleChecks({ rulePackDocument: { metadata: { rule_pack_id: "x" } }, model: pair.model })).rejects.toThrow("RULE_SOURCE_IDENTITY_NOT_FRESH");
    expect(invokeMock.mock.calls.map(([command]) => command)).not.toContain("run_rule_checks");
    expect(invokeMock.mock.calls.at(-1)?.[0]).toBe("dispose_preview_mechanics_result");
    expect(replay.custody()).toEqual([]);
  });
});
