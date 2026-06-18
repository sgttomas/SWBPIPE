import { useState } from "react";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import {
  COMPLETENESS_STATUSES,
  DeclarationsEditor,
  LIBRARY_KINDS,
  REQUIRED_FOR_TARGETS,
  SLOT_KINDS,
  SOURCE_KINDS,
  VALUE_STATUSES,
  defaultLibraryValueRef,
  defaultRequiredInput,
  defaultSolverResultRef,
  defaultValueSlot,
  readRequiredInputIds,
  readValueSlotIds,
  setRequiredInputs,
  setValueSlots,
  uniqueDeclarationId,
  type DeclarationDocument
} from "./DeclarationsEditor";
import { buildDraftRulePackDocument } from "../../services/rulePackService";
import type { UnitCatalog } from "../../services/unitCatalogService";

// TP-C2-DECLEDITOR-001 — required-input / value-slot declaration form builders.
// Pure-helper tests pin the schema-valid default shapes, the unique-id rule,
// and the lossless array rewrite; component tests drive the editor through
// visible controls and assert the rewritten document (including that nested
// quantity_intent edits preserve the const-true relaxation flags).

const catalogFixture: UnitCatalog = {
  schema_version: "0.1.0",
  catalog_id: "unit-system:dec-018-si-dual-display",
  decision_basis: "DEC-018",
  calculation_basis: "si_canonical",
  storage_convention: "entered_units_preserved",
  entry_count: 4,
  entries: [
    {
      unit_id: "unit:meter",
      symbol: "m",
      dimension_id: "length",
      canonical: true,
      transform_kind: "identity",
      factor_representation: "1 m/m, SI canonical identity",
      offset_representation: null,
      provenance: "si_canonical",
      review_status: "accepted"
    },
    {
      unit_id: "unit:pascal",
      symbol: "Pa",
      dimension_id: "pressure",
      canonical: true,
      transform_kind: "identity",
      factor_representation: "1 Pa/Pa, SI canonical identity",
      offset_representation: null,
      provenance: "si_canonical",
      review_status: "accepted"
    },
    {
      unit_id: "unit:megapascal",
      symbol: "MPa",
      dimension_id: "pressure",
      canonical: false,
      transform_kind: "multiplicative",
      factor_representation: "1000000 Pa/MPa, exact public definition",
      offset_representation: null,
      provenance: "exact_public_definition",
      review_status: "accepted"
    },
    {
      unit_id: "unit:pound_force",
      symbol: "lbf",
      dimension_id: "force",
      canonical: false,
      transform_kind: "multiplicative",
      factor_representation: "0.45359237 kg/lb * 9.80665 m/s^2 per lbf",
      offset_representation: null,
      provenance: "conventional_public_constant",
      review_status: "accepted"
    }
  ],
  boundary: {
    source: "core/units open_pipe_stress_units catalog",
    protected_content_included: false,
    private_project_data_included: false,
    professional_approval_claimed: false,
    code_compliance_claimed: false
  }
};

afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function inputsOf(document: DeclarationDocument): Array<Record<string, unknown>> {
  return document.required_inputs as Array<Record<string, unknown>>;
}
function slotsOf(document: DeclarationDocument): Array<Record<string, unknown>> {
  return document.value_slots as Array<Record<string, unknown>>;
}

describe("DeclarationsEditor pure helpers", () => {
  it("reads declared required-input and value-slot ids in order", () => {
    const draft = buildDraftRulePackDocument();
    expect(readRequiredInputIds(draft)).toEqual(["user_required_input_1"]);
    expect(readValueSlotIds(draft)).toEqual(["user_limit_slot_1"]);
    // Malformed / absent arrays read as empty, never throw.
    expect(readRequiredInputIds({})).toEqual([]);
    expect(readValueSlotIds({ value_slots: "nope" })).toEqual([]);
  });

  it("generates a unique declaration id, skipping ids already present", () => {
    expect(uniqueDeclarationId("user_required_input", [])).toBe("user_required_input_1");
    expect(uniqueDeclarationId("user_required_input", ["user_required_input_1"])).toBe(
      "user_required_input_2"
    );
    // Collision on the length-derived candidate advances past it.
    expect(uniqueDeclarationId("user_limit_slot", ["user_limit_slot_2"])).toBe("user_limit_slot_3");
  });

  it("builds a schema-shaped default required input with a fresh id and TBD placeholders", () => {
    // toEqual pins the EXACT shape: every RequiredInput required member present,
    // the const-true relaxation flags true, the uppercase "TBD" dimension/unit
    // placeholder the codec and schema accept (never lowercase), and no
    // invented value.
    expect(defaultRequiredInput(["user_required_input_1"])).toEqual({
      input_id: "user_required_input_2",
      name: "User-defined required input (placeholder)",
      source_kind: "user_supplied_rule_value",
      quantity_intent: {
        dimension: "TBD",
        unit_ref: "TBD",
        unit_required: true,
        dimension_check_required: true
      },
      required_for: "rule_check",
      completeness_status: "missing_required_value",
      missing_value_diagnostic: "RULE_INPUT_MISSING",
      provenance: {
        source_name: "User private basis (draft placeholder)",
        source_type: "user_private_basis",
        source_location: "TBD",
        source_license: "TBD",
        contributor: "TBD",
        contributor_certification: "TBD",
        redistribution_status: "private_only",
        protected_content_review: "required_before_public_use",
        review_status: "pending"
      },
      redistribution_status: "private_only",
      provenance_required: true,
      redistribution_status_required: true
    });
  });

  it("builds a schema-shaped default value slot with a fresh id and not_provided status", () => {
    expect(defaultValueSlot(["user_limit_slot_1"])).toMatchObject({
      slot_id: "user_limit_slot_2",
      slot_kind: "TBD",
      quantity_intent: {
        dimension: "TBD",
        unit_ref: "TBD",
        unit_required: true,
        dimension_check_required: true
      },
      value_status: "not_provided",
      required_for: "rule_check",
      review_status: "pending",
      completeness_status: "missing_required_value"
    });
    // No invented numeric value is authored (the schema carries none).
    expect(defaultValueSlot([])).not.toHaveProperty("value");
  });

  it("replaces the declaration arrays losslessly, preserving the rest of the document", () => {
    const draft = buildDraftRulePackDocument();
    const next = setRequiredInputs(draft, [{ input_id: "x" }, { input_id: "y" }]);
    expect(readRequiredInputIds(next)).toEqual(["x", "y"]);
    // Other members (and the value_slots array) are untouched by reference-equal copy.
    expect(next.formula_declarations).toBe(draft.formula_declarations);
    expect(next.value_slots).toBe(draft.value_slots);
    const next2 = setValueSlots(draft, []);
    expect(next2.value_slots).toEqual([]);
    expect(next2.required_inputs).toBe(draft.required_inputs);
  });

  it("exposes only the schema enum vocabularies", () => {
    expect(SOURCE_KINDS).toEqual([
      "solver_result",
      "model_input",
      "user_supplied_rule_value",
      "private_library_value",
      "owner_design_basis",
      "TBD"
    ]);
    expect(REQUIRED_FOR_TARGETS).toEqual(["rule_check", "reporting", "diagnostic", "TBD"]);
    expect(COMPLETENESS_STATUSES).toEqual([
      "complete",
      "missing_required_value",
      "missing_unit",
      "missing_provenance",
      "missing_redistribution_status",
      "review_pending",
      "TBD"
    ]);
    expect(SLOT_KINDS).toEqual([
      "allowable",
      "stress_limit",
      "ratio_limit",
      "coefficient",
      "category_threshold",
      "source_reference",
      "TBD"
    ]);
    expect(VALUE_STATUSES).toEqual([
      "not_provided",
      "private_user_supplied",
      "public_permissive_reviewed",
      "invented_non_engineering_example",
      "protected_suspected",
      "TBD"
    ]);
    // LibraryValueRef.library_kind has NO "TBD" member in the schema (the three
    // concrete kinds only), so the vocab is exactly those three.
    expect(LIBRARY_KINDS).toEqual(["material", "section", "component"]);
  });

  it("builds a schema-shaped default library_value_ref: a concrete kind + TBD ids", () => {
    // toEqual pins the EXACT shape: all four required members present, the kind
    // defaulted to the first (only-resolved) kind, and the ids the visible
    // uppercase "TBD" placeholder (an unfilled reference resolves to nothing and
    // the input blocks — never a silent pass; never an invented private value).
    expect(defaultLibraryValueRef()).toEqual({
      library_kind: "material",
      library_id: "TBD",
      record_id: "TBD",
      slot_id: "TBD"
    });
  });

  it("builds a schema-shaped default solver_result_ref: only result_id as a TBD placeholder", () => {
    // toEqual pins the EXACT shape: the single required member present, the id
    // the visible uppercase "TBD" placeholder (an unfilled reference matches no
    // result row and the input blocks — never a silent pass; never a stored
    // result value).
    expect(defaultSolverResultRef()).toEqual({ result_id: "TBD" });
  });
});

function Harness({ initial }: { initial: DeclarationDocument }) {
  const [document, setDocument] = useState<DeclarationDocument>(initial);
  return (
    <>
      <DeclarationsEditor document={document} onChange={setDocument} disabled={false} />
      <pre data-testid="decl-doc">{JSON.stringify(document)}</pre>
    </>
  );
}

function harnessDoc(): DeclarationDocument {
  return JSON.parse(screen.getByTestId("decl-doc").textContent ?? "{}") as DeclarationDocument;
}

describe("DeclarationsEditor component", () => {
  it("lists the declared required inputs and value slots", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    expect(screen.getByTestId("rule-pack-required-inputs").textContent).toContain(
      "Required inputs (1)"
    );
    expect(screen.getByTestId("rule-pack-value-slots").textContent).toContain("Value slots (1)");
    expect((screen.getByTestId("rule-pack-input-id") as HTMLInputElement).value).toBe(
      "user_required_input_1"
    );
    expect((screen.getByTestId("rule-pack-slot-id") as HTMLInputElement).value).toBe(
      "user_limit_slot_1"
    );
    expect(screen.getByTestId("rule-pack-declarations-unit-policy").textContent).toContain(
      "unit_policy=stored_unit_refs_preserved"
    );
  });

  it("adds a required input with a fresh unique id and a schema-valid default", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.click(screen.getByTestId("rule-pack-input-add"));
    const inputs = inputsOf(harnessDoc());
    expect(inputs).toHaveLength(2);
    expect(inputs[1].input_id).toBe("user_required_input_2");
    expect(inputs[1].source_kind).toBe("user_supplied_rule_value");
    expect((inputs[1].quantity_intent as Record<string, unknown>).dimension).toBe("TBD");
  });

  it("edits required-input identity, typing, unit, and status fields", async () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.change(screen.getByTestId("rule-pack-input-id"), { target: { value: "hoop_stress" } });
    fireEvent.change(screen.getByTestId("rule-pack-input-name"), {
      target: { value: "Hoop stress at node" }
    });
    fireEvent.change(screen.getByTestId("rule-pack-input-source-kind"), {
      target: { value: "solver_result" }
    });
    fireEvent.change(screen.getByTestId("rule-pack-input-dimension"), {
      target: { value: "stress" }
    });
    fireEvent.change(screen.getByTestId("rule-pack-input-unit"), {
      target: { value: "invented_stress_unit" }
    });
    await waitFor(() =>
      expect(screen.getByTestId("rule-pack-declarations-unit-policy").textContent).toContain(
        "required_input:hoop_stress=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview(unit=invented_stress_unit;dimension=stress)"
      )
    );
    fireEvent.change(screen.getByTestId("rule-pack-input-required-for"), {
      target: { value: "reporting" }
    });
    fireEvent.change(screen.getByTestId("rule-pack-input-completeness"), {
      target: { value: "complete" }
    });
    const input = inputsOf(harnessDoc())[0];
    expect(input).toMatchObject({
      input_id: "hoop_stress",
      name: "Hoop stress at node",
      source_kind: "solver_result",
      required_for: "reporting",
      completeness_status: "complete"
    });
    // The nested quantity_intent edit preserves the const-true relaxation
    // flags (lossless: only dimension/unit_ref were touched).
    expect(input.quantity_intent).toEqual({
      dimension: "stress",
      unit_ref: "invented_stress_unit",
      unit_required: true,
      dimension_check_required: true
    });
  });

  it("uses the DEC-018 catalog for required-input unit refs without offering incompatible dimensions", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(catalogFixture);
    render(<Harness initial={buildDraftRulePackDocument()} />);

    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("get_unit_catalog"));
    fireEvent.change(screen.getByTestId("rule-pack-input-dimension"), {
      target: { value: "stress" }
    });

    const unitSelect = screen.getByTestId("rule-pack-input-unit") as HTMLSelectElement;
    await waitFor(() => expect(Array.from(unitSelect.options).map((option) => option.value)).toContain("Pa"));
    const optionValues = Array.from(unitSelect.options).map((option) => option.value);
    expect(optionValues).toContain("MPa");
    expect(optionValues).not.toContain("m");
    expect(optionValues).not.toContain("lbf");

    fireEvent.change(unitSelect, { target: { value: "MPa" } });
    expect(screen.getByTestId("rule-pack-declarations-unit-policy").textContent).toContain(
      "required_input:user_required_input_1=dec018_catalog_dimension_match(unit=MPa;dimension=stress)"
    );
    expect(inputsOf(harnessDoc())[0].quantity_intent).toEqual({
      dimension: "stress",
      unit_ref: "MPa",
      unit_required: true,
      dimension_check_required: true
    });
  });

  it("keeps an out-of-catalog stored unit visible as current instead of snapping it", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(catalogFixture);
    const document = setRequiredInputs(buildDraftRulePackDocument(), [
      {
        input_id: "legacy",
        quantity_intent: {
          dimension: "stress",
          unit_ref: "legacy_private_unit",
          unit_required: true,
          dimension_check_required: true
        }
      }
    ]);
    render(<Harness initial={document} />);

    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("get_unit_catalog"));
    const unitSelect = screen.getByTestId("rule-pack-input-unit") as HTMLSelectElement;
    await waitFor(() => expect(unitSelect.value).toBe("legacy_private_unit"));
    expect(screen.getByText("legacy_private_unit, catalog mismatch")).toBeTruthy();
    expect(screen.getByTestId("rule-pack-declarations-unit-policy").textContent).toContain(
      "required_input:legacy=dec018_catalog_dimension_mismatch(unit=legacy_private_unit;dimension=stress)"
    );

    fireEvent.change(unitSelect, { target: { value: "Pa" } });
    expect(inputsOf(harnessDoc())[0].quantity_intent).toEqual({
      dimension: "stress",
      unit_ref: "Pa",
      unit_required: true,
      dimension_check_required: true
    });
  });

  it("blocks removing the last required input and supports add-then-remove losslessly", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    // Schema floor: required_inputs minItems 1, so the only row's remove is blocked.
    expect(screen.getByTestId("rule-pack-input-remove")).toHaveProperty("disabled", true);

    fireEvent.click(screen.getByTestId("rule-pack-input-add"));
    // Edit the original (first) row, then remove the second; the first row must
    // survive verbatim with its edit, proving the rewrite is lossless.
    fireEvent.change(screen.getAllByTestId("rule-pack-input-id")[0], {
      target: { value: "kept_input" }
    });
    const firstBefore = inputsOf(harnessDoc())[0];
    fireEvent.click(screen.getAllByTestId("rule-pack-input-remove")[1]);
    const inputs = inputsOf(harnessDoc());
    expect(inputs).toHaveLength(1);
    expect(inputs[0]).toEqual(firstBefore);
    expect(inputs[0].input_id).toBe("kept_input");
  });

  it("adds and edits a value slot's kind, value status, and dimension", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.click(screen.getByTestId("rule-pack-slot-add"));
    expect(slotsOf(harnessDoc())).toHaveLength(2);
    expect(slotsOf(harnessDoc())[1].slot_id).toBe("user_limit_slot_2");

    fireEvent.change(screen.getAllByTestId("rule-pack-slot-kind")[0], {
      target: { value: "allowable" }
    });
    fireEvent.change(screen.getAllByTestId("rule-pack-slot-value-status")[0], {
      target: { value: "private_user_supplied" }
    });
    fireEvent.change(screen.getAllByTestId("rule-pack-slot-dimension")[0], {
      target: { value: "stress" }
    });
    const slot = slotsOf(harnessDoc())[0];
    expect(slot).toMatchObject({ slot_kind: "allowable", value_status: "private_user_supplied" });
    expect(slot.quantity_intent).toEqual({
      dimension: "stress",
      unit_ref: "TBD",
      unit_required: true,
      dimension_check_required: true
    });
  });

  it("uses the DEC-018 catalog for value-slot unit refs", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(catalogFixture);
    render(<Harness initial={buildDraftRulePackDocument()} />);

    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("get_unit_catalog"));
    fireEvent.change(screen.getByTestId("rule-pack-slot-dimension"), {
      target: { value: "stress" }
    });
    const slotUnitSelect = screen.getByTestId("rule-pack-slot-unit") as HTMLSelectElement;
    await waitFor(() =>
      expect(Array.from(slotUnitSelect.options).map((option) => option.value)).toContain("MPa")
    );
    fireEvent.change(slotUnitSelect, { target: { value: "Pa" } });
    expect(screen.getByTestId("rule-pack-declarations-unit-policy").textContent).toContain(
      "value_slot:user_limit_slot_1=dec018_catalog_dimension_match(unit=Pa;dimension=stress)"
    );

    expect(slotsOf(harnessDoc())[0].quantity_intent).toEqual({
      dimension: "stress",
      unit_ref: "Pa",
      unit_required: true,
      dimension_check_required: true
    });
  });

  it("blocks removing the last value slot", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    expect(screen.getByTestId("rule-pack-slot-remove")).toHaveProperty("disabled", true);
  });

  it("editing one declaration preserves sibling declarations verbatim", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    const slotBefore = slotsOf(harnessDoc())[0];
    // Edit a required input; the untouched value slot (including its full
    // provenance block) round-trips byte-for-byte.
    fireEvent.change(screen.getByTestId("rule-pack-input-name"), { target: { value: "changed" } });
    expect(slotsOf(harnessDoc())[0]).toEqual(slotBefore);
  });

  it("surfaces an out-of-vocabulary stored token as a (current) option, not a silent snap", () => {
    const document = setRequiredInputs(buildDraftRulePackDocument(), [
      { input_id: "legacy", source_kind: "future_source_kind_from_a_newer_schema" }
    ]);
    render(<Harness initial={document} />);
    const select = screen.getByTestId("rule-pack-input-source-kind") as HTMLSelectElement;
    expect(select.value).toBe("future_source_kind_from_a_newer_schema");
    expect(screen.getByText("(current) future_source_kind_from_a_newer_schema")).toBeTruthy();
  });

  it("surfaces an out-of-vocabulary stored dimension without snapping it to TBD", () => {
    // The shared DimensionSelect must not display-snap a future-schema
    // dimension token to "TBD" while the document still holds the real token.
    const document = setRequiredInputs(buildDraftRulePackDocument(), [
      {
        input_id: "legacy",
        quantity_intent: { dimension: "future_dimension_from_a_newer_schema", unit_ref: "TBD" }
      }
    ]);
    render(<Harness initial={document} />);
    const select = screen.getByTestId("rule-pack-input-dimension") as HTMLSelectElement;
    expect(select.value).toBe("future_dimension_from_a_newer_schema");
    expect(screen.getByText("(current) future_dimension_from_a_newer_schema")).toBeTruthy();
  });

  it("disables every control while a backend request is in flight", () => {
    render(
      <DeclarationsEditor
        document={buildDraftRulePackDocument()}
        onChange={() => {}}
        disabled
        disabledReason="busy"
      />
    );
    expect(screen.getByTestId("rule-pack-input-id")).toHaveProperty("disabled", true);
    expect(screen.getByTestId("rule-pack-input-add")).toHaveProperty("disabled", true);
    expect(screen.getByTestId("rule-pack-slot-add")).toHaveProperty("disabled", true);
  });

  // --- library_value_ref authoring (TP-C3-LIBREFAUTHOR-001) ---

  it("shows the library reference sub-form and seeds a complete reference when an input draws from a private library", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    // The default input draws from user_supplied_rule_value: no library sub-form.
    expect(screen.queryByTestId("rule-pack-input-library-ref")).toBeNull();

    fireEvent.change(screen.getByTestId("rule-pack-input-source-kind"), {
      target: { value: "private_library_value" }
    });
    // The sub-form appears and a COMPLETE four-member reference is seeded (never
    // a schema-invalid partial), with the IP-boundary note shown.
    expect(screen.getByTestId("rule-pack-input-library-ref")).toBeTruthy();
    expect(inputsOf(harnessDoc())[0].library_value_ref).toEqual(defaultLibraryValueRef());
    expect(screen.getByTestId("rule-pack-input-library-ref-note").textContent).toContain(
      "never embedded in the rule pack"
    );
  });

  it("edits the library reference fields and keeps the rest of the input verbatim", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.change(screen.getByTestId("rule-pack-input-source-kind"), {
      target: { value: "private_library_value" }
    });
    const inputBefore = inputsOf(harnessDoc())[0];

    fireEvent.change(screen.getByTestId("rule-pack-input-library-kind"), {
      target: { value: "section" }
    });
    fireEvent.change(screen.getByTestId("rule-pack-input-library-id"), {
      target: { value: "my_private_lib" }
    });
    fireEvent.change(screen.getByTestId("rule-pack-input-library-record"), {
      target: { value: "rec_A106B" }
    });
    fireEvent.change(screen.getByTestId("rule-pack-input-library-slot"), {
      target: { value: "allowable_stress" }
    });
    const input = inputsOf(harnessDoc())[0];
    expect(input.library_value_ref).toEqual({
      library_kind: "section",
      library_id: "my_private_lib",
      record_id: "rec_A106B",
      slot_id: "allowable_stress"
    });
    // Lossless: every non-reference member of the input is unchanged (toEqual
    // ignores the undefined-keyed reference on both sides).
    expect({ ...input, library_value_ref: undefined }).toEqual({
      ...inputBefore,
      library_value_ref: undefined
    });
  });

  it("completes a partial reference on first edit when an opened input lacks one", () => {
    // Edge case: a pack opened with source_kind private_library_value but NO
    // library_value_ref (e.g. authored before this slice). Editing one field
    // must still write a COMPLETE four-member reference, never a partial.
    const document = setRequiredInputs(buildDraftRulePackDocument(), [
      { input_id: "lib_input", source_kind: "private_library_value" }
    ]);
    render(<Harness initial={document} />);
    // The sub-form shows by source_kind even with no stored reference yet.
    expect(screen.getByTestId("rule-pack-input-library-ref")).toBeTruthy();
    // Remove is disabled until a reference actually exists.
    expect(screen.getByTestId("rule-pack-input-library-remove")).toHaveProperty("disabled", true);

    fireEvent.change(screen.getByTestId("rule-pack-input-library-record"), {
      target: { value: "rec_9" }
    });
    expect(inputsOf(harnessDoc())[0].library_value_ref).toEqual({
      library_kind: "material",
      library_id: "TBD",
      record_id: "rec_9",
      slot_id: "TBD"
    });
  });

  it("keeps a reference left on a non-library input visible and removable", () => {
    // A reference left behind after the source_kind was changed away from
    // private_library_value stays visible (never silently hidden) and removable.
    const document = setRequiredInputs(buildDraftRulePackDocument(), [
      {
        input_id: "legacy",
        source_kind: "solver_result",
        name: "kept",
        library_value_ref: {
          library_kind: "material",
          library_id: "lib_1",
          record_id: "rec_1",
          slot_id: "slot_1"
        }
      }
    ]);
    render(<Harness initial={document} />);
    expect(screen.getByTestId("rule-pack-input-library-ref")).toBeTruthy();
    expect(screen.getByTestId("rule-pack-input-library-remove")).toHaveProperty("disabled", false);

    fireEvent.click(screen.getByTestId("rule-pack-input-library-remove"));
    const input = inputsOf(harnessDoc())[0];
    // The reference member is dropped; every other member round-trips verbatim.
    expect("library_value_ref" in input).toBe(false);
    expect(input).toEqual({ input_id: "legacy", source_kind: "solver_result", name: "kept" });
    // With the reference gone and a non-library source_kind, the sub-form hides.
    expect(screen.queryByTestId("rule-pack-input-library-ref")).toBeNull();
  });

  // --- solver_result_ref authoring (TP-C4-SOLVERREFAUTHOR-001) ---

  it("shows the solver-result reference sub-form and seeds a complete reference when an input draws from a solved result", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    // The default input draws from user_supplied_rule_value: no solver sub-form.
    expect(screen.queryByTestId("rule-pack-input-solver-ref")).toBeNull();

    fireEvent.change(screen.getByTestId("rule-pack-input-source-kind"), {
      target: { value: "solver_result" }
    });
    // The sub-form appears and a complete (single-member) reference is seeded
    // (never a schema-invalid empty object), with the supersede note shown.
    expect(screen.getByTestId("rule-pack-input-solver-ref")).toBeTruthy();
    expect(inputsOf(harnessDoc())[0].solver_result_ref).toEqual(defaultSolverResultRef());
    expect(screen.getByTestId("rule-pack-input-solver-ref-note").textContent).toContain(
      "supersedes the run panel"
    );
    // Seeding the solver ref must NOT also seed a library ref (independent kinds).
    expect(inputsOf(harnessDoc())[0]).not.toHaveProperty("library_value_ref");
  });

  it("edits the solver-result reference id and keeps the rest of the input verbatim", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.change(screen.getByTestId("rule-pack-input-source-kind"), {
      target: { value: "solver_result" }
    });
    const inputBefore = inputsOf(harnessDoc())[0];

    fireEvent.change(screen.getByTestId("rule-pack-input-solver-result-id"), {
      target: { value: "result:stress:demo" }
    });
    const input = inputsOf(harnessDoc())[0];
    expect(input.solver_result_ref).toEqual({ result_id: "result:stress:demo" });
    // Lossless: every non-reference member of the input is unchanged (toEqual
    // ignores the undefined-keyed reference on both sides).
    expect({ ...input, solver_result_ref: undefined }).toEqual({
      ...inputBefore,
      solver_result_ref: undefined
    });
  });

  it("completes a partial solver reference on first edit when an opened input lacks one", () => {
    // Edge case: a pack opened with source_kind solver_result but NO
    // solver_result_ref (e.g. authored before this slice, or relying on the
    // caller-supplied run-panel binding). Editing the id must still write a
    // complete reference, never an empty object.
    const document = setRequiredInputs(buildDraftRulePackDocument(), [
      { input_id: "solver_input", source_kind: "solver_result" }
    ]);
    render(<Harness initial={document} />);
    // The sub-form shows by source_kind even with no stored reference yet.
    expect(screen.getByTestId("rule-pack-input-solver-ref")).toBeTruthy();
    // Remove is disabled until a reference actually exists.
    expect(screen.getByTestId("rule-pack-input-solver-remove")).toHaveProperty("disabled", true);

    fireEvent.change(screen.getByTestId("rule-pack-input-solver-result-id"), {
      target: { value: "result:disp:node-3:ux" }
    });
    expect(inputsOf(harnessDoc())[0].solver_result_ref).toEqual({
      result_id: "result:disp:node-3:ux"
    });
  });

  it("keeps a solver reference left on a non-solver input visible and removable", () => {
    // A reference left behind after the source_kind was changed away from
    // solver_result stays visible (never silently hidden) and removable.
    const document = setRequiredInputs(buildDraftRulePackDocument(), [
      {
        input_id: "legacy",
        source_kind: "user_supplied_rule_value",
        name: "kept",
        solver_result_ref: { result_id: "result:stress:legacy" }
      }
    ]);
    render(<Harness initial={document} />);
    expect(screen.getByTestId("rule-pack-input-solver-ref")).toBeTruthy();
    expect(screen.getByTestId("rule-pack-input-solver-remove")).toHaveProperty("disabled", false);

    fireEvent.click(screen.getByTestId("rule-pack-input-solver-remove"));
    const input = inputsOf(harnessDoc())[0];
    // The reference member is dropped; every other member round-trips verbatim.
    expect("solver_result_ref" in input).toBe(false);
    expect(input).toEqual({
      input_id: "legacy",
      source_kind: "user_supplied_rule_value",
      name: "kept"
    });
    // With the reference gone and a non-solver source_kind, the sub-form hides.
    expect(screen.queryByTestId("rule-pack-input-solver-ref")).toBeNull();
  });
});
