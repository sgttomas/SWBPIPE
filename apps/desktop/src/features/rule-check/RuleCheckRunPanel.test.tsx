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

  it("surfaces a private-library reference for a private_library_value input", () => {
    render(<RuleCheckRunPanel model={modelStub} result={resultStub} />);
    const pack = {
      metadata: { rule_pack_id: "p" },
      required_inputs: [
        {
          input_id: "lib_allow",
          name: "Library allowable",
          source_kind: "private_library_value",
          quantity_intent: { dimension: "stress", unit_ref: "demo_unit" },
          library_value_ref: {
            library_kind: "material",
            library_id: "lib:steel",
            record_id: "mat:a",
            slot_id: "allow:Sh"
          }
        }
      ]
    };
    fireEvent.change(screen.getByTestId("rule-check-pack-json"), {
      target: { value: JSON.stringify(pack) }
    });
    const row = screen.getByTestId("rule-check-library-input-lib_allow");
    expect(row.textContent).toContain("material:lib:steel");
    expect(row.textContent).toContain("allow:Sh");
    expect(row.textContent).toContain("never embedded in the rule pack");
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

// Phase C4 app-held envelope wiring (TP-C4-APPAGG-001). The panel lifts the
// worst-of rule-check aggregate to the app via onAggregateChange so it can be
// recorded in the app-held analysis-run envelope. It reports the aggregate on a
// successful desktop run and null whenever there is no current run outcome.
describe("RuleCheckRunPanel aggregate lift (TP-C4-APPAGG-001)", () => {
  const passingRun = {
    document_kind: "openpipestress.rule_check.run",
    rule_pack_id: "invented_demo_rule_pack",
    grammar_version: "1.0.0",
    aggregate_status: "USER_RULE_FAILED",
    checks: [
      {
        check_id: "demo_training_check",
        status: "USER_RULE_FAILED",
        acceptability_relation: "less_than_or_equal",
        bound_inputs: [],
        completeness_findings: [],
        evaluator_findings: [],
        diagnostic_codes: []
      }
    ],
    professional_boundary_notice: "Software rule-check evidence only; not a professional claim."
  };

  it("lifts the worst-of aggregate to onAggregateChange on a desktop run", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(passingRun);
    const onAggregateChange = vi.fn();

    render(<RuleCheckRunPanel model={modelStub} result={resultStub} onAggregateChange={onAggregateChange} />);
    fireEvent.click(screen.getByTestId("rule-check-load-demo"));
    await screen.findByTestId("rule-check-binding-plan");
    fireEvent.click(screen.getByTestId("rule-check-run"));

    await screen.findByTestId("rule-check-run-result");
    expect(onAggregateChange).toHaveBeenCalledWith("USER_RULE_FAILED");
  });

  it("clears the lifted aggregate on a new pack and on the desktop-only browser seam", async () => {
    const onAggregateChange = vi.fn();
    render(<RuleCheckRunPanel model={modelStub} result={resultStub} onAggregateChange={onAggregateChange} />);

    // Loading a pack resets any prior run aggregate held by the app.
    fireEvent.click(screen.getByTestId("rule-check-load-demo"));
    await screen.findByTestId("rule-check-binding-plan");
    expect(onAggregateChange).toHaveBeenCalledWith(null);

    // A run in browser preview has no backend evaluator: report the seam and clear.
    onAggregateChange.mockClear();
    fireEvent.click(screen.getByTestId("rule-check-run"));
    await waitFor(() =>
      expect(screen.getByTestId("rule-check-run-status").textContent).toContain("RULE-CHECK-BACKEND-DESKTOP-ONLY")
    );
    expect(onAggregateChange).toHaveBeenCalledWith(null);
  });
});

// Phase C3 resolution-preview/browse picker (TP-C3-LIBREFPICKER-001). The panel
// previously surfaced the authored library_value_ref read-only; these tests pin
// the richer, read-only-to-the-pack picker: a "Preview resolution" action that
// queries the local store (desktop-only) and surfaces whether the reference
// resolves, plus a browse of the available records/slots. It never mutates the
// pack and never overrides the authored reference at run time.
describe("RuleCheckRunPanel library reference preview", () => {
  const libraryPack = {
    metadata: { rule_pack_id: "p" },
    required_inputs: [
      {
        input_id: "lib_allow",
        name: "Library allowable",
        source_kind: "private_library_value",
        quantity_intent: { dimension: "stress", unit_ref: "demo_unit" },
        library_value_ref: {
          library_kind: "material",
          library_id: "lib:steel",
          record_id: "mat:a",
          slot_id: "allow:Sh"
        }
      }
    ]
  };

  const steelEntry = {
    project_id: "project:c4-run-test",
    library_kind: "material",
    library_id: "lib:steel",
    library_name: "Invented steel allowables",
    privacy_class: "private_user_data",
    storage_mode: "local_sqlite",
    created_at_unix: 1,
    updated_at_unix: 1
  };

  function renderWithLibraryPack() {
    render(<RuleCheckRunPanel model={modelStub} result={resultStub} />);
    fireEvent.change(screen.getByTestId("rule-check-pack-json"), {
      target: { value: JSON.stringify(libraryPack) }
    });
  }

  it("offers a Preview resolution button for a library input that carries a reference", () => {
    renderWithLibraryPack();
    expect(screen.getByTestId("rule-check-library-preview-lib_allow")).toBeInTheDocument();
  });

  it("previews a resolving reference and browses the local records/slots", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValueOnce([steelEntry]); // list_local_libraries
    invokeMock.mockResolvedValueOnce({
      document: {
        material_records: [
          { material_id: "mat:a", allowables: [{ allowable_id: "allow:Sh" }, { allowable_id: "allow:Sc" }] }
        ]
      }
    }); // open_local_library

    renderWithLibraryPack();
    fireEvent.click(screen.getByTestId("rule-check-library-preview-lib_allow"));

    const resolution = await screen.findByTestId("rule-check-library-resolution-lib_allow");
    expect(resolution).toHaveAttribute("data-status", "resolves");
    expect(resolution.textContent).toContain("never embedded");
    const browse = screen.getByTestId("rule-check-library-browse-lib_allow");
    expect(browse.textContent).toContain("lib:steel");
    expect(browse.textContent).toContain("mat:a (referenced)");
    expect(browse.textContent).toContain("allow:Sh (referenced)");
    expect(browse.textContent).toContain("allow:Sc");
    expect(invokeMock).toHaveBeenNthCalledWith(1, "list_local_libraries", { projectId: "project:c4-run-test" });
    expect(invokeMock).toHaveBeenNthCalledWith(2, "open_local_library", {
      projectId: "project:c4-run-test",
      libraryKind: "material",
      libraryId: "lib:steel"
    });
  });

  it("reports library_missing and lists what is available when the reference points nowhere", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValueOnce([{ ...steelEntry, library_id: "lib:other", library_name: "Other alloys" }]);

    renderWithLibraryPack();
    fireEvent.click(screen.getByTestId("rule-check-library-preview-lib_allow"));

    const resolution = await screen.findByTestId("rule-check-library-resolution-lib_allow");
    expect(resolution).toHaveAttribute("data-status", "library_missing");
    expect(screen.getByTestId("rule-check-library-browse-lib_allow").textContent).toContain("lib:other");
    // library_missing short-circuits before open_local_library.
    expect(invokeMock).toHaveBeenCalledTimes(1);
  });

  it("distinguishes record_missing from slot_missing against the stored library", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    // record_missing: the stored library has no mat:a record.
    invokeMock.mockResolvedValueOnce([steelEntry]);
    invokeMock.mockResolvedValueOnce({
      document: { material_records: [{ material_id: "mat:z", allowables: [{ allowable_id: "allow:Sh" }] }] }
    });
    renderWithLibraryPack();
    fireEvent.click(screen.getByTestId("rule-check-library-preview-lib_allow"));
    let resolution = await screen.findByTestId("rule-check-library-resolution-lib_allow");
    expect(resolution).toHaveAttribute("data-status", "record_missing");

    // slot_missing: mat:a exists but lacks the allow:Sh slot.
    invokeMock.mockResolvedValueOnce([steelEntry]);
    invokeMock.mockResolvedValueOnce({
      document: { material_records: [{ material_id: "mat:a", allowables: [{ allowable_id: "allow:Sc" }] }] }
    });
    fireEvent.click(screen.getByTestId("rule-check-library-preview-lib_allow"));
    await waitFor(() =>
      expect(screen.getByTestId("rule-check-library-resolution-lib_allow")).toHaveAttribute(
        "data-status",
        "slot_missing"
      )
    );
    resolution = screen.getByTestId("rule-check-library-resolution-lib_allow");
    expect(resolution.textContent).toContain("blocks at RULE_INPUTS_INCOMPLETE");
  });

  it("reports the desktop-only backend seam for library preview in browser preview", async () => {
    renderWithLibraryPack();
    fireEvent.click(screen.getByTestId("rule-check-library-preview-lib_allow"));
    const resolution = await screen.findByTestId("rule-check-library-resolution-lib_allow");
    expect(resolution).toHaveAttribute("data-status", "unavailable");
    expect(resolution.textContent).toContain("LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY");
  });
});
