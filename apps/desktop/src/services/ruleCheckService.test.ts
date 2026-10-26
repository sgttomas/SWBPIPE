import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../test/nativeMechanicsReplay";
import { loadBundledMechanicsReference, loadPreviewModel, runPreviewMechanics } from "./previewService";
import {
  deriveRuleCheckBindingPlan,
  loadDemoRuleCheckPack,
  parseSolverResultRef,
  resolveSolverSelector,
  rowTieRod,
  runRuleChecks,
  ruleBindingPrecheck,
  ruleCheckOutcomeIdentity,
  ruleCheckOutcomeTestId,
  selectSolverRows,
  solverInputBases,
  solverSelectorForRow,
  RULE_CHECK_BACKEND_DIAGNOSTIC,
  N_RULE_CHECK_NOT_NUMERICALLY_ELIGIBLE,
  type RuleCheckBasisRef,
  type SolverResultRef
} from "./ruleCheckService";
import connectorCorpus from "../../../../fixtures/results/pressure_v3_connector_reader_corpus.json";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { createSolveJobTransport } from "../test/nativeMechanicsReplay";
import { hasNativeMechanicsInvocation } from "./previewService";
import {
  LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE,
  SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED,
  numericalResultStanding
} from "../features/results/resultStanding";
import { N_LOAD_REFERENCE_RULE_CHECK } from "../features/results/loadReferenceDisclosure";
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
// Epoch 8 seam, unit simulation only: a source placed in `grantedCustody` passes the
// service's native-invocation gate without any source registration, so the rule service's
// own standing step is reached for an unregistered pressure-source-1 source. Every other
// source keeps the production check unchanged.
const custodySeam = vi.hoisted(() => ({ grantedCustody: new WeakSet<object>() }));
vi.mock("./previewService", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./previewService")>();
  return {
    ...actual,
    hasNativeMechanicsInvocation: (...args: Parameters<typeof actual.hasNativeMechanicsInvocation>) =>
      (args[0] && custodySeam.grantedCustody.has(args[0])) || actual.hasNativeMechanicsInvocation(...args)
  };
});

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
          solver_result_ref: { result_kind: "stress", entity_ref: "pipe:demo", location: "end_i" }
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
      ],
      required_load_cases: [
        { required_load_case_id: "rlc_a", name: "Invented case A", basis_ref: { ref_type: "load_case", ref_id: "load:A" } },
        { required_load_case_id: "rlc_b", name: "Invented case B", basis_ref: { ref_type: "combination", ref_id: "combo:B" } }
      ],
      stress_categories: [{ category_id: "cat_x", name: "Invented category X" }],
      load_case_mappings: [
        {
          mapping_id: "map_x",
          category_ref: { ref_id: "cat_x", ref_type: "stress_category" },
          required_load_case_refs: [
            { ref_id: "rlc_b", ref_type: "required_load_case" },
            { ref_id: "rlc_a", ref_type: "required_load_case" }
          ]
        }
      ],
      check_definitions: [
        { check_id: "check_x", category_ref: { ref_id: "cat_x", ref_type: "stress_category" } },
        { check_id: "check_orphan", category_ref: { ref_id: "cat_none", ref_type: "stress_category" } }
      ]
    };

    const plan = deriveRuleCheckBindingPlan(pack);

    expect(plan.solverInputs).toEqual([
      {
        input_id: "actual",
        name: "Actual",
        dimension: "stress",
        unit_ref: "demo_unit",
        solver_result_ref: { result_kind: "stress", entity_ref: "pipe:demo", location: "end_i" }
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
    const caseA = { required_load_case_id: "rlc_a", name: "Invented case A", basis_ref: { ref_type: "load_case", ref_id: "load:A" } };
    const caseB = { required_load_case_id: "rlc_b", name: "Invented case B", basis_ref: { ref_type: "combination", ref_id: "combo:B" } };
    expect(plan.requiredLoadCases).toEqual([caseA, caseB]);
    expect(plan.stressCategories).toEqual([{ category_id: "cat_x", name: "Invented category X" }]);
    // Bases in the mapping's order; a check whose category has no mapping has none.
    expect(plan.checks).toEqual([
      { check_id: "check_x", category_ref: "cat_x", bases: [caseB, caseA], input_refs: [] },
      { check_id: "check_orphan", category_ref: "cat_none", bases: [], input_refs: [] }
    ]);
  });

  it("reads a 0.4.0 result_id reference as no selector (validation refuses it by name)", () => {
    const plan = deriveRuleCheckBindingPlan({
      required_inputs: [
        { input_id: "legacy", source_kind: "solver_result", solver_result_ref: { result_id: "result:stress:demo" } }
      ]
    });
    expect(plan.solverInputs[0].solver_result_ref).toBeUndefined();
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

describe("solver-row selectors (display mirror of rule_solver_selection.rs)", () => {
  const caseA: RuleCheckBasisRef = { ref_type: "load_case", ref_id: "load:A" };
  const caseB: RuleCheckBasisRef = { ref_type: "load_case", ref_id: "load:B" };
  const row = (id: string, basis: RuleCheckBasisRef | null, metadata: Record<string, string>, kind = "stress", entity = "pipe:demo") =>
    ({ id, kind, value: 1, unit: "Pa", entity_ref: entity, ...(basis ? { basis_ref: basis } : {}), metadata }) as unknown as MechanicsResult["results"][number];
  const rows = [
    row("r:a:i:nom", caseA, { location: "end_i", component: "axial", basis: "recovered; section_basis=nominal_section" }),
    row("r:a:i:red", caseA, { location: "end_i", component: "axial", basis: "recovered; section_basis=reduced_section" }),
    row("r:b:i:nom", caseB, { location: "end_i", component: "axial", basis: "recovered; section_basis=nominal_section" }),
    row("r:a:j", caseA, { location: "end_j", component: "axial", basis: "recovered" }),
    row("r:a:j:twin", caseA, { location: "end_j", component: "axial", basis: "recovered" }),
    row("r:echo", null, { location: "end_i", component: "axial", basis: "recovered; section_basis=nominal_section" }),
    row("r:a:bad", caseA, { location: "end_k", component: "axial", basis: "recovered; section_basis=gross_section" })
  ];
  const nominal: SolverResultRef = { result_kind: "stress", entity_ref: "pipe:demo", location: "end_i", component: "axial", section_basis: "nominal_section" };

  it("binds exactly the row of the basis, and an absent member matches only a row without it", () => {
    expect(selectSolverRows({ results: rows }, nominal, caseA).map((r) => r.id)).toEqual(["r:a:i:nom"]);
    expect(selectSolverRows({ results: rows }, nominal, caseB).map((r) => r.id)).toEqual(["r:b:i:nom"]);
    // Without section_basis the selector matches neither section twin.
    expect(selectSolverRows({ results: rows }, { ...nominal, section_basis: undefined }, caseA)).toEqual([]);
    // A malformed section token matches nothing; a row without basis_ref is never selected.
    expect(selectSolverRows({ results: rows }, { ...nominal, location: "end_k", section_basis: undefined }, caseA)).toEqual([]);
    expect(selectSolverRows({ results: rows }, nominal, { ref_type: "load_case", ref_id: "load:none" })).toEqual([]);
  });

  it("resolves per required load case: one row, none, several, or an unsolved basis", () => {
    const loadCases = [
      { required_load_case_id: "rlc_a", name: "A", basis_ref: caseA },
      { required_load_case_id: "rlc_b", name: "B", basis_ref: caseB },
      { required_load_case_id: "rlc_c", name: "C", basis_ref: { ref_type: "combination", ref_id: "combo:C" } as RuleCheckBasisRef }
    ];
    const resolved = resolveSolverSelector({ results: rows }, nominal, loadCases);
    expect(resolved.status).toBe("incomplete");
    if (resolved.status === "no_result_rows") throw new Error("unreachable");
    expect(resolved.bases.map((entry) => [entry.required_load_case_id, entry.status])).toEqual([
      ["rlc_a", "resolves"], ["rlc_b", "resolves"], ["rlc_c", "basis_unsolved"]
    ]);
    const twins = resolveSolverSelector({ results: rows }, { result_kind: "stress", entity_ref: "pipe:demo", location: "end_j", component: "axial" }, loadCases.slice(0, 2));
    if (twins.status === "no_result_rows") throw new Error("unreachable");
    expect(twins.bases.map((entry) => entry.status)).toEqual(["row_ambiguous", "row_missing"]);
    expect(resolveSolverSelector({ results: rows }, nominal, loadCases.slice(0, 2)).status).toBe("resolves");
    expect(resolveSolverSelector({ results: [] }, nominal, loadCases).status).toBe("no_result_rows");
    expect(resolveSolverSelector(null, nominal, loadCases).status).toBe("no_result_rows");
  });

  it("offers the selector of a row, which selects that row alone within its basis", () => {
    const offered = solverSelectorForRow({}, rows[1])!;
    expect(offered).toEqual({ result_kind: "stress", entity_ref: "pipe:demo", location: "end_i", component: "axial", section_basis: "reduced_section" });
    expect(selectSolverRows({ results: rows }, offered, caseA).map((r) => r.id)).toEqual(["r:a:i:red"]);
    expect(solverSelectorForRow({}, rows[5])).toBeUndefined();
    expect(solverSelectorForRow({}, rows[6])).toBeUndefined();
  });

  it("separates the two tie rods of one connector on the reader corpus", () => {
    const corpus = connectorCorpus as unknown as Record<string, unknown>;
    const envelopes = Object.values(corpus).flatMap(function collect(value: unknown): MechanicsResult[] {
      if (Array.isArray(value)) return value.flatMap(collect);
      if (typeof value !== "object" || value === null) return [];
      const record = value as Record<string, unknown>;
      return Array.isArray(record.results) && record.contract_evidence ? [record as unknown as MechanicsResult] : Object.values(record).flatMap(collect);
    });
    const rodRows = envelopes.flatMap((envelope) => envelope.results.filter((r) => r.kind === "connector_tie_rod_force_v1").map((r) => ({ envelope, r })));
    expect(rodRows.length).toBeGreaterThan(1);
    for (const { envelope, r } of rodRows) {
      const selector = solverSelectorForRow(envelope, r)!;
      expect(selector.tie_rod).toBeTruthy();
      expect(selectSolverRows(envelope, selector, r.basis_ref as RuleCheckBasisRef).map((match) => match.id)).toEqual([r.id]);
    }
  });

  // PR #1629 review r1 finding 1 (mirror of rule_solver_selection.rs
  // a_tie_rod_selector_binds_only_the_exact_rod_id_of_the_record): a row id
  // encodes its rod lossily (`:` to `-`), so a selector binds a rod row only
  // when its tie_rod equals, exactly, the rod id the connector record holds.
  it("binds a tie-rod selector only to the exact rod id of the connector record", () => {
    const corpus = connectorCorpus as unknown as { cases: Array<{ envelope: MechanicsResult }> };
    const envelope = corpus.cases.map((entry) => entry.envelope)
      .find((candidate) => candidate.results.filter((r) => r.kind === "connector_tie_rod_force_v1").length === 2)!;
    const caseU5A: RuleCheckBasisRef = { ref_type: "load_case", ref_id: "load:U5A" };
    const rodRows = (source: MechanicsResult, tie_rod: string) =>
      selectSolverRows(source, { result_kind: "connector_tie_rod_force_v1", entity_ref: "component:C-J", location: "tie_rod", component: "t_r", tie_rod }, caseU5A).map((r) => r.id);
    const rods = (source: MechanicsResult) => (source.contract_evidence as unknown as { connector: Array<{ tie_rods: Array<{ id: string }> }> }).connector[0].tie_rods;
    expect(rods(envelope).map((rod) => rod.id)).toEqual(["rod:yp", "rod:ym"]);
    const yp = rodRows(envelope, "rod:yp"), ym = rodRows(envelope, "rod:ym");
    expect(yp).toHaveLength(1);
    expect(ym).toHaveLength(1);
    expect(yp).not.toEqual(ym);
    // Alias: rod-yp encodes to the same row id but is not a rod of the record.
    expect(rodRows(envelope, "rod-yp")).toEqual([]);
    // Renamed: the record's rod:yp is now rod-yp; the old name binds nothing.
    const renamed = structuredClone(envelope);
    rods(renamed)[0].id = "rod-yp";
    expect(rodRows(renamed, "rod:yp")).toEqual([]);
    expect(rodRows(renamed, "rod-yp")).toEqual(yp);
    // Removed: the record holds no rod for that row; nothing binds it.
    const removed = structuredClone(envelope);
    rods(removed)[0].id = "rod:zz";
    expect(rodRows(removed, "rod:yp")).toEqual([]);
    expect(rodRows(removed, "rod:zz")).toEqual([]);
    expect(rodRows(removed, "rod:ym")).toEqual(ym);
    // Ambiguous record (two rods encode to the row id): neither binds it.
    const doubled = structuredClone(envelope);
    rods(doubled).push({ ...rods(doubled)[0], id: "rod-yp" });
    expect(rodRows(doubled, "rod:yp")).toEqual([]);
    expect(rodRows(doubled, "rod-yp")).toEqual([]);
    expect(rowTieRod(doubled, doubled.results.find((r) => r.id === yp[0])!, caseU5A)).toBeUndefined();
    // The authoring offer never names an alias.
    expect(solverSelectorForRow(envelope, envelope.results.find((r) => r.id === yp[0])!)?.tie_rod).toBe("rod:yp");
    expect(solverSelectorForRow(doubled, doubled.results.find((r) => r.id === yp[0])!)).toBeUndefined();
  });

  // PR #1629 review r1 finding 3: an input's bases are those of the checks
  // that use it (required_input_refs), as the runner binds it.
  it("plans each input's bases from the checks that use it", () => {
    const ref = (ref_type: string, ref_id: string) => ({ ref_type, ref_id });
    const plan = deriveRuleCheckBindingPlan({
      required_inputs: ["x", "y", "z"].map((input_id) => ({ input_id, source_kind: "solver_result", solver_result_ref: nominal })),
      required_load_cases: [
        { required_load_case_id: "rlc_a", name: "A", basis_ref: caseA },
        { required_load_case_id: "rlc_b", name: "B", basis_ref: caseB }
      ],
      stress_categories: [{ category_id: "cat_a" }, { category_id: "cat_b" }],
      load_case_mappings: [
        { mapping_id: "m_a", category_ref: ref("stress_category", "cat_a"), required_load_case_refs: [ref("required_load_case", "rlc_a")] },
        { mapping_id: "m_b", category_ref: ref("stress_category", "cat_b"), required_load_case_refs: [ref("required_load_case", "rlc_b"), ref("required_load_case", "rlc_a")] }
      ],
      check_definitions: [
        { check_id: "c_a", category_ref: ref("stress_category", "cat_a"), required_input_refs: [ref("required_input", "x"), ref("required_input", "y")] },
        { check_id: "c_b", category_ref: ref("stress_category", "cat_b"), required_input_refs: [ref("required_input", "y")] }
      ]
    });
    const ids = (inputId: string) => solverInputBases(plan, inputId).map((basis) => basis.required_load_case_id);
    expect(ids("x")).toEqual(["rlc_a"]);
    expect(ids("y")).toEqual(["rlc_a", "rlc_b"]);
    expect(ids("z")).toEqual([]);
    // Case B publishes no nominal row here: x (used only on case A) resolves; y is incomplete on B; z, used by no check, never resolves.
    const onlyA = { results: rows.filter((r) => r.id !== "r:b:i:nom") };
    expect(resolveSolverSelector(onlyA, nominal, solverInputBases(plan, "x")).status).toBe("resolves");
    expect(resolveSolverSelector(onlyA, nominal, solverInputBases(plan, "y")).status).toBe("incomplete");
    expect(resolveSolverSelector(onlyA, nominal, solverInputBases(plan, "z")).status).toBe("incomplete");
  });

  it("parses a selector as the validator does", () => {
    expect(parseSolverResultRef({ result_kind: "k", entity_ref: "e", tie_rod: "rod:1" })).toEqual({ result_kind: "k", entity_ref: "e", tie_rod: "rod:1" });
    expect(parseSolverResultRef({ result_id: "r" })).toBeUndefined();
    expect(parseSolverResultRef({ result_kind: "k", entity_ref: "" })).toBeUndefined();
    expect(parseSolverResultRef({ result_kind: "k", entity_ref: "e", location: "" })).toBeUndefined();
    expect(parseSolverResultRef({ result_kind: "k", entity_ref: "e", section_basis: "gross_section" })).toBeUndefined();
    expect(parseSolverResultRef({ result_kind: "k", entity_ref: "e", result_id: "r" })).toBeUndefined();
  });
});

describe("rule-check outcome identity (design §2.7)", () => {
  it("tells repeated check ids apart by basis", () => {
    const one = { check_id: "c", basis_ref: { ref_type: "load_case" as const, ref_id: "load:A" } };
    const two = { check_id: "c", basis_ref: { ref_type: "combination" as const, ref_id: "load:A" } };
    expect(ruleCheckOutcomeIdentity(one)).toBe(JSON.stringify(["c", "load_case", "load:A"]));
    expect(ruleCheckOutcomeIdentity(one)).not.toBe(ruleCheckOutcomeIdentity(two));
    expect(ruleCheckOutcomeTestId(one)).toBe("rule-check-outcome-c--load_case--load:A");
    expect(ruleCheckOutcomeTestId({ check_id: "c", basis_ref: null })).toBe("rule-check-outcome-c--no-basis");
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
      suppliedValueBindings: []
    });

    expect(route.route).toBe("tauri_backend");
    // An ungated (preview-physics-2) run carries no load/reference-state line (epoch 8, §4.4).
    expect(route).toMatchObject({ loadReferenceDisclosure: null });
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
    expect(args).not.toHaveProperty("sourceBlockInvocation");
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
      suppliedValueBindings: [{ ref_id: "limit", value: 100, unit: "demo_unit", dimension: "stress" }],
      projectId: "project:lib"
    });

    const [, args] = invokeMock.mock.calls[0];
    const typed = args as Record<string, unknown>;
    expect(typed).toHaveProperty("solvedEnvelope");
    expect(typed).toHaveProperty("model", pair.model);
    expect(typed.solvedEnvelope).toStrictEqual(JSON.parse(JSON.stringify(solvedEnvelope)));
    expect(invokeMock).toHaveBeenCalledTimes(1);
    // Solver inputs bind only through the pack's own selector (format 0.5.0):
    // no caller-supplied solver binding is ever sent.
    expect(typed).not.toHaveProperty("solverResultBindings");
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
    expect(demo.schema_version).toBe("0.5.0");
    expect(plan.solverInputs.map((input) => input.input_id)).toContain("demo_actual_quantity");
    // The demo's selector names an invented kind no producer publishes (design §4).
    expect(plan.solverInputs[0].solver_result_ref).toEqual({ result_kind: "invented_demo_quantity", entity_ref: "pipe:P-100" });
    expect(plan.requiredLoadCases.map((entry) => entry.basis_ref)).toEqual([
      { ref_type: "load_case", ref_id: "load:L-100" },
      { ref_type: "load_case", ref_id: "load:L-200" }
    ]);
    expect(plan.checks).toEqual([
      { check_id: "demo_training_check", category_ref: "demo_category_a", bases: plan.requiredLoadCases, input_refs: ["demo_actual_quantity", "demo_limit_quantity"] }
    ]);
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
  // A pack whose solver inputs select the given rows, on the rows' own bases.
  const solverPack = (source: MechanicsResult, selected: Array<MechanicsResult["results"][number] | SolverResultRef>): RulePackDocument => {
    const bases = [...new Set(source.results.flatMap((r) => (r.basis_ref ? [JSON.stringify(r.basis_ref)] : [])))].map((text) => JSON.parse(text));
    return {
      required_inputs: selected.map((entry, index) => ({
        input_id: `input_${index}`,
        name: `Input ${index}`,
        source_kind: "solver_result",
        solver_result_ref: "result_kind" in entry ? entry : solverSelectorForRow(source, entry)
      })),
      required_load_cases: bases.map((basis, index) => ({ required_load_case_id: `rlc_${index}`, name: `Case ${index}`, basis_ref: basis })),
      // One check uses every input on every basis, so the precheck reads each.
      stress_categories: [{ category_id: "cat" }],
      load_case_mappings: [{ mapping_id: "map", category_ref: { ref_type: "stress_category", ref_id: "cat" }, required_load_case_refs: bases.map((_, index) => ({ ref_type: "required_load_case", ref_id: `rlc_${index}` })) }],
      check_definitions: [{ check_id: "check", category_ref: { ref_type: "stress_category", ref_id: "cat" }, required_input_refs: selected.map((_, index) => ({ ref_type: "required_input", ref_id: `input_${index}` })) }]
    };
  };
  it("refuses no binding to a composite's rows (B4/D1 (b): the source-blocks-1 summary refusal is gone)", () => {
    const source = structuredClone(pressureSourceMixed) as unknown as MechanicsResult;
    const maximum = source.results.find((r) => r.kind === "pipe_elastic_normal_stress_maximum_v2")!;
    const force = source.results.find((r) => r.kind === "element_local_axial_force")!;
    expect(ruleBindingPrecheck(source, deriveRuleCheckBindingPlan(solverPack(source, [maximum, force])))).toEqual([]);
  });
  it("gives a preview-physics-2 binding to a retired id no special finding (B3: a missing result); the new rows bind", () => {
    const source = structuredClone(previewInvented) as unknown as MechanicsResult;
    // A selector naming a kind the source does not publish selects no row, so no finding.
    const findings = ruleBindingPrecheck(source, deriveRuleCheckBindingPlan(solverPack(source, [{ result_kind: "retired_reaction_kind", entity_ref: "support:S-100" }])));
    expect(findings).toEqual([]);
    const maximum = source.results.find((r) => r.kind === "pipe_elastic_normal_stress_maximum_v2")!;
    const intensified = source.results.find((r) => r.kind === "component_equal_factor_intensified_bending_stress_v1")!;
    expect(ruleBindingPrecheck(source, deriveRuleCheckBindingPlan(solverPack(source, [maximum, intensified])))).toEqual([]);
  });
  it("refuses a stored precision-1 statement before invoking the rule backend (B1)", async () => {
    const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    const source = retiredPrecisionStatement();
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    await expect(runRuleChecks({ rulePackDocument: { metadata: { rule_pack_id: "x" } }, model: pair.model, solvedEnvelope: source })).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    expect(invokeMock).not.toHaveBeenCalled();
  });
  it("refuses a received pressure-2 copy before invoking the rule backend: it carries no native invocation (b7-plan R2)", async () => {
    // T4-U2: the L line's pressure-2 result (invented data), as received bytes.
    const source = structuredClone(lLine.sparse_result) as unknown as MechanicsResult;
    const model = structuredClone(lLine.authored_model) as unknown as PreviewModel;
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    await expect(runRuleChecks({ rulePackDocument: { metadata: { rule_pack_id: "x" } }, model, solvedEnvelope: source })).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    expect(invokeMock).not.toHaveBeenCalled();
  });
  it("admits a transient pressure-2 native solve at eligible standing and releases it after the rule backend used it (epoch 8)", async () => {
    const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" }), replay = createNativeMechanicsReplay({ profile: "pressure" });
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(async (command, args, options) => command === "run_rule_checks" ? RULE_RESPONSE : replay.invoke(command, args, options));
    const route = await runRuleChecks({ rulePackDocument: { metadata: { rule_pack_id: "x" } }, model: pair.model });
    expect(route).toMatchObject({ route: "tauri_backend", result: RULE_RESPONSE, loadReferenceDisclosure: null });
    expect(invokeMock.mock.calls.map(([command]) => command)).toEqual([
      "start_preview_mechanics_job", "poll_preview_mechanics_job", "take_preview_mechanics_job_result",
      "run_rule_checks", "dispose_preview_mechanics_result"
    ]);
    expect(replay.custody()).toEqual([]);
  });
});

/* -------------------------------------------------------------------------- *
 * Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §4.4, §5.4, §6.1 "Rule check"):
 * the exact routes reach `run_rule_checks` only at the Rust reader's eligible standing,
 * on the Current path (a supplied envelope) and the transient path (the service solves,
 * then disposes). Unit transport replay of committed producer bytes through mocked IPC;
 * the rule backend's answer is a unit simulation, NOT rule-evaluation evidence.
 * -------------------------------------------------------------------------- */
const RULE_RESPONSE = {
  document_kind: "openpipestress.rule_check.run",
  rule_pack_id: "invented-test-only",
  grammar_version: "1.0.0",
  aggregate_status: "RULE_INPUTS_INCOMPLETE",
  checks: [],
  professional_boundary_notice: "notice"
};
const fixtureRoot = resolve(__dirname, "../../../../");
const fixtureText = (path: string) => readFileSync(resolve(fixtureRoot, path), "utf8");
type Mode = "sparse_interactive" | "dense_scrutiny";
type ExactInput = { model: PreviewModel; bytes: string };
const ORDINARY = "fixtures/product_preview/pressure/load_reference";
const JOINED = "fixtures/product_preview/pressure_source/load_reference_source";
const FALLBACK = "core/reporting/result_export/tests/fixtures/load_reference_fallback_uz";
/** A committed request model and its producer bytes. For pressure-2 the session's case
 * metadata is added where the request lacks it (invented labels; the product reads no
 * physics from them). A pressure-source-1 request is sent unchanged: its receipt binds
 * the model it was solved for. */
function committed(requestPath: string, rawPath: string, options: { unchanged?: boolean } = {}): ExactInput {
  const model = structuredClone(JSON.parse(fixtureText(requestPath)).model) as PreviewModel;
  if (!options.unchanged) for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
  return { model, bytes: fixtureText(rawPath) };
}
const ADMITTED: Record<string, { route: "pressure" | "pressure_source"; loadReference: boolean; input: (mode: Mode) => ExactInput }> = {
  "pressure-2": { route: "pressure", loadReference: false, input: mode => {
    const pair = nativeMechanicsReplayPair(mode, { profile: "pressure" });
    return { model: pair.model, bytes: pair.bytes };
  } },
  "pressure-2 load/reference connected": { route: "pressure", loadReference: true, input: mode => committed(`${ORDINARY}/connected.request.json`, `${ORDINARY}/connected-${mode}.raw.json`) },
  "pressure-2 load/reference pressure": { route: "pressure", loadReference: true, input: mode => committed(`${ORDINARY}/pressure.request.json`, `${ORDINARY}/pressure-${mode}.raw.json`) },
  "pressure-source-1 n05": { route: "pressure_source", loadReference: false, input: mode => committed("fixtures/product_preview/pressure_source/physics_source/n05.request.json", `fixtures/product_preview/pressure_source/physics_source/n05-${mode}.raw.json`, { unchanged: true }) },
  "pressure-source-1 mixed": { route: "pressure_source", loadReference: false, input: mode => committed("fixtures/product_preview/pressure_source/physics_source/mixed.request.json", `fixtures/product_preview/pressure_source/physics_source/mixed-${mode}.raw.json`, { unchanged: true }) },
};
/** Registered natively, and not numerically eligible on Rust's standing, with the first
 * finding the service reports. `eigen_motion` is not among them: its committed receipt
 * binds an invocation other than the desktop's `{model, materials: []}` request, so the
 * desktop never registers it (`SOURCE_BLOCKS_INVOCATION_HASH`); it is covered below. */
const REFUSED: Record<string, { finding: string; input: (mode: Mode) => ExactInput }> = Object.fromEntries([
  ...["n05", "n06", "fields", "mixed"].map(name => [`pressure-source-1 load/reference ${name}`, {
    finding: LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE,
    input: (mode: Mode) => committed(`${JOINED}/${name}.request.json`, `${JOINED}/${name}-${mode}.raw.json`, { unchanged: true })
  }]),
  ["pressure-2 load/reference fallback", { finding: "NUMERICAL_INTEGRITY_NOT_QUALIFIED", input: (mode: Mode) => committed(`${FALLBACK}.request.json`, `${FALLBACK}-${mode}.raw.json`) }],
]);
/** The disclosure line from the published records, in published order, without duplicates. */
function expectedDisclosure(bytes: string): string {
  const records = JSON.parse(bytes).contract_evidence.load_reference_states as { reference_configuration_id: string; load_case_id: string; history: { kind: string } }[];
  const distinct = (values: string[]) => [...new Set(values)].join(", ");
  return N_LOAD_REFERENCE_RULE_CHECK
    .replace("{configurations}", distinct(records.map(r => r.reference_configuration_id)))
    .replace("{cases}", distinct(records.map(r => r.load_case_id)))
    .replace("{histories}", distinct(records.map(r => r.history.kind)));
}
/** The native job transport answering exactly this model and mode, and the mocked rule backend. */
function installNative({ model, bytes }: ExactInput, mode: Mode) {
  const transport = createSolveJobTransport((sent) => {
    expect(sent.mode).toBe(mode);
    expect(sent.model).toEqual(model);
    return bytes;
  });
  invokeMock.mockImplementation(async (command, args, options) => command === "run_rule_checks" ? RULE_RESPONSE : transport.invoke(command, args, options));
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  return transport;
}
const commands = () => invokeMock.mock.calls.map(([command]) => command as string);
const TRANSIENT_COMMANDS = ["start_preview_mechanics_job", "poll_preview_mechanics_job", "take_preview_mechanics_job_result"];
const exactRulePack = { metadata: { rule_pack_id: "invented-test-only" } };

describe("epoch 8: the exact routes reach the rule backend only at Rust's eligible standing", () => {
  const admittedCurrent = Object.keys(ADMITTED).flatMap(name => (["sparse_interactive", "dense_scrutiny"] as Mode[]).map(mode => [name, mode] as const));
  it.each(admittedCurrent)("%s (%s), Current path: the supplied envelope reaches run_rule_checks with its invocation arguments", async (name, mode) => {
    const { route, loadReference, input } = ADMITTED[name];
    const exact = input(mode);
    installNative(exact, mode);
    const source = await runPreviewMechanics(exact.model, mode);
    expect(hasNativeMechanicsInvocation(source, exact.model, mode)).toBe(true);
    expect(numericalResultStanding(source, exact.model)).toMatchObject({ contract: route, eligible: true, findings: [] });
    invokeMock.mockClear();
    const answer = await runRuleChecks({ rulePackDocument: exactRulePack, model: exact.model, solvedEnvelope: source });
    expect(answer).toStrictEqual({ route: "tauri_backend", result: RULE_RESPONSE, loadReferenceDisclosure: loadReference ? expectedDisclosure(exact.bytes) : null });
    expect(commands()).toEqual(["run_rule_checks"]);
    const args = invokeMock.mock.calls[0][1] as Record<string, unknown>;
    expect((args.solvedEnvelope as MechanicsResult).run_id).toBe(source.run_id);
    expect((args.solvedEnvelope as MechanicsResult).producer).toEqual(source.producer);
    expect(args.model).toEqual(exact.model);
    // pressure-source-1 carries its registered invocation; pressure-2 needs none.
    if (route === "pressure_source") expect(args.sourceBlockInvocation).toEqual({ request: { model: exact.model, materials: [] }, solver_mode: mode });
    else expect(args).not.toHaveProperty("sourceBlockInvocation");
  });

  it.each(Object.keys(ADMITTED))("%s, transient path: the service solves, reaches run_rule_checks, then disposes the result", async name => {
    const { route, loadReference, input } = ADMITTED[name];
    const mode: Mode = "sparse_interactive";
    const exact = input(mode);
    const transport = installNative(exact, mode);
    const answer = await runRuleChecks({ rulePackDocument: exactRulePack, model: exact.model });
    expect(answer).toStrictEqual({ route: "tauri_backend", result: RULE_RESPONSE, loadReferenceDisclosure: loadReference ? expectedDisclosure(exact.bytes) : null });
    expect(commands()).toEqual([...TRANSIENT_COMMANDS, "run_rule_checks", "dispose_preview_mechanics_result"]);
    expect(transport.custody()).toEqual([]);
    expect(transport.disposed()).toHaveLength(1);
    const args = invokeMock.mock.calls[3][1] as Record<string, unknown>;
    expect((args.solvedEnvelope as MechanicsResult).run_id).toBe(JSON.parse(exact.bytes).run_id);
    expect(args.model).toEqual(exact.model);
    if (route === "pressure_source") expect(args.sourceBlockInvocation).toEqual({ request: { model: exact.model, materials: [] }, solver_mode: mode });
    else expect(args).not.toHaveProperty("sourceBlockInvocation");
  });

  const refusedCurrent = Object.keys(REFUSED).flatMap(name => (["sparse_interactive", "dense_scrutiny"] as Mode[]).map(mode => [name, mode] as const));
  it.each(refusedCurrent)("%s (%s), Current path: a refused standing stops the run before invoke, with Rust's first finding", async (name, mode) => {
    const { finding, input } = REFUSED[name];
    const exact = input(mode);
    installNative(exact, mode);
    const source = await runPreviewMechanics(exact.model, mode);
    expect(hasNativeMechanicsInvocation(source, exact.model, mode)).toBe(true);
    const standing = numericalResultStanding(source, exact.model);
    expect(standing.eligible).toBe(false);
    expect(standing.findings[0]).toBe(finding);
    invokeMock.mockClear();
    await expect(runRuleChecks({ rulePackDocument: exactRulePack, model: exact.model, solvedEnvelope: source }))
      .rejects.toThrow(new Error(`${finding}: ${N_RULE_CHECK_NOT_NUMERICALLY_ELIGIBLE}`));
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it.each(Object.keys(REFUSED))("%s, transient path: a refused standing stops the run before invoke and the solve is disposed", async name => {
    const { finding, input } = REFUSED[name];
    const mode: Mode = "sparse_interactive";
    const exact = input(mode);
    const transport = installNative(exact, mode);
    await expect(runRuleChecks({ rulePackDocument: exactRulePack, model: exact.model }))
      .rejects.toThrow(new Error(`${finding}: ${N_RULE_CHECK_NOT_NUMERICALLY_ELIGIBLE}`));
    expect(commands()).toEqual([...TRANSIENT_COMMANDS, "dispose_preview_mechanics_result"]);
    expect(transport.custody()).toEqual([]);
  });

  it("pressure-source-1 load/reference eigen_motion is never registered by the desktop solve, so neither path reaches the rule backend", async () => {
    const mode: Mode = "sparse_interactive";
    const exact = committed(`${JOINED}/eigen_motion.request.json`, `${JOINED}/eigen_motion-${mode}.raw.json`, { unchanged: true });
    const transport = installNative(exact, mode);
    const source = await runPreviewMechanics(exact.model, mode);
    expect(hasNativeMechanicsInvocation(source, exact.model, mode)).toBe(false);
    expect(numericalResultStanding(source, exact.model).eligible).toBe(false);
    invokeMock.mockClear();
    await expect(runRuleChecks({ rulePackDocument: exactRulePack, model: exact.model, solvedEnvelope: source })).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    expect(invokeMock).not.toHaveBeenCalled();
    await expect(runRuleChecks({ rulePackDocument: exactRulePack, model: exact.model })).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    expect(commands()).toEqual([...TRANSIENT_COMMANDS, "dispose_preview_mechanics_result"]);
    // The transient result is released; only the test's own held source remains in custody.
    expect(transport.disposed()).toHaveLength(1);
    expect(transport.custody()).toHaveLength(1);
  });

  it.each(["n05", "n06", "fields", "mixed", "eigen_motion"])("an unregistered pressure-source-1 load/reference %s source stops before invoke with SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED", async name => {
    // Received bytes without a source registration. The seam grants them native custody
    // (unit simulation), so the service's own standing step decides, without an invocation.
    const exact = committed(`${JOINED}/${name}.request.json`, `${JOINED}/${name}-sparse_interactive.raw.json`, { unchanged: true });
    const source = JSON.parse(exact.bytes) as MechanicsResult;
    custodySeam.grantedCustody.add(source);
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    expect(numericalResultStanding(source, exact.model).findings[0]).toBe(SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED);
    await expect(runRuleChecks({ rulePackDocument: exactRulePack, model: exact.model, solvedEnvelope: source }))
      .rejects.toThrow(new Error(`${SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED}: ${N_RULE_CHECK_NOT_NUMERICALLY_ELIGIBLE}`));
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("an unregistered plain pressure-source-1 source stops before invoke with SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED", async () => {
    const exact = committed("fixtures/product_preview/pressure_source/physics_source/n05.request.json", "fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json", { unchanged: true });
    const source = JSON.parse(exact.bytes) as MechanicsResult;
    custodySeam.grantedCustody.add(source);
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    await expect(runRuleChecks({ rulePackDocument: exactRulePack, model: exact.model, solvedEnvelope: source }))
      .rejects.toThrow(new Error(`${SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED}: ${N_RULE_CHECK_NOT_NUMERICALLY_ELIGIBLE}`));
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it.each(Object.keys(ADMITTED))("%s: a copy of an eligible Current source, or the source under an edited model, never reaches the rule backend", async name => {
    const mode: Mode = "sparse_interactive";
    const exact = ADMITTED[name].input(mode);
    installNative(exact, mode);
    const source = await runPreviewMechanics(exact.model, mode);
    invokeMock.mockClear();
    await expect(runRuleChecks({ rulePackDocument: exactRulePack, model: exact.model, solvedEnvelope: structuredClone(source) })).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    const edited = structuredClone(exact.model); edited.nodes[0].position.x += 1;
    await expect(runRuleChecks({ rulePackDocument: exactRulePack, model: edited, solvedEnvelope: source })).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    expect(invokeMock).not.toHaveBeenCalled();
  });
});
