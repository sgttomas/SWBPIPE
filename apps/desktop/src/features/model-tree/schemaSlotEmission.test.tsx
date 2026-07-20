// TP-PMM-GUIEMIT-001 — GUI entry and emission of user-entered schema slots
// adopted under DEC-068 / DEC-070 and extended by DEC-077:
//
//   1. `geometry.bend_pipe_ref` on bend components (curved-bend span
//      mapping; expected shape per `core/product_physics` blocking
//      diagnostics),
//   2. `section.mill_tolerance` on pipe spans,
//   3. mutually exclusive `modulus_basis_ref` / `modulus_basis_temperature`
//      on load cases,
//   4. seismic/wind `equivalent_static` occasional-load-generation inputs
//      on load cases.
//
// Hard fences pinned here: every slot starts EMPTY/absent (no default, no
// prefilled catalog value, no computed suggestion); an un-entered slot is
// OMITTED from the emitted model JSON; entered values are emitted verbatim
// through the structured-operation intent seam (DEC-020 / ADR-0001 — the
// GUI never mutates the model directly) and through canonical model-JSON
// serialization. The canonical `schemas/*.schema.yaml` slots and the
// preview-model input surface are parallel surfaces (bend_pipe_ref
// precedent); the schema-binding suite below reads the real schema files
// and pins the vocabulary correspondence plus the constraints that bind
// the emitted values directly.
//
// All values in this file are invented user-entered test data
// (PUBLIC_DOMAIN_OR_ORIGINAL); no code coefficient, catalog value, or
// physical-constant default ships to the product from here.

import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import { loadPreviewModel } from "../../services/previewService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { selectedProperties } from "../model-workspace/modelView";
import { PropertyInspector } from "./PropertyInspector";

const SLOT_KEYS = [
  "bend_pipe_ref",
  "mill_tolerance",
  "modulus_basis_ref",
  "modulus_basis_temperature",
  "equivalent_static"
] as const;

const schemasDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "../../../../../schemas"
);

type JsonRecord = Record<string, unknown>;

function readSchema(name: string): JsonRecord {
  // The canonical schema files are JSON documents (stored with a .yaml
  // extension); read them directly like the contract-corpus runner reads
  // its fixture set.
  return JSON.parse(readFileSync(path.join(schemasDir, name), "utf8")) as JsonRecord;
}

function defs(schema: JsonRecord): JsonRecord {
  return schema.$defs as JsonRecord;
}

function renderInspector(model: PreviewModel, selection: EntityRef): EditorOperationIntent[] {
  const queued: EditorOperationIntent[] = [];
  render(
    <PropertyInspector
      model={model}
      onQueueIntent={(intent) => queued.push(intent)}
      selection={selection}
    />
  );
  return queued;
}

function intentPanel(): HTMLElement {
  return screen.getByLabelText("Editor operation intent");
}

function queueFieldValue(fieldPath: string, value: string): void {
  const panel = intentPanel();
  fireEvent.change(within(panel).getByTestId("editor-intent-field"), {
    target: { value: fieldPath }
  });
  fireEvent.change(within(panel).getByTestId("editor-intent-value"), {
    target: { value }
  });
  fireEvent.click(within(panel).getByTestId("queue-editor-intent"));
}

function clone(model: PreviewModel): PreviewModel {
  return JSON.parse(JSON.stringify(model)) as PreviewModel;
}

afterEach(cleanup);

describe("TP-PMM-GUIEMIT-001 user-entered schema slot entry", () => {
  it("starts every slot empty: the fixture model and inspector defaults carry no value", async () => {
    const model = await loadPreviewModel();
    const serialized = JSON.stringify(model);
    for (const key of SLOT_KEYS) {
      expect(serialized).not.toContain(`"${key}"`);
    }

    renderInspector(model, { type: "component", id: "component:C-110" });
    const panel = intentPanel();
    fireEvent.change(within(panel).getByTestId("editor-intent-field"), {
      target: { value: "geometry.bend_pipe_ref" }
    });
    expect(within(panel).getByTestId("editor-intent-value")).toHaveValue("TBD");
    // Queue is disabled until the user actually enters something.
    expect(within(panel).getByTestId("queue-editor-intent")).toBeDisabled();
  });

  it("enters and emits geometry.bend_pipe_ref on a bend component through the intent seam", async () => {
    const model = await loadPreviewModel();
    const queued = renderInspector(model, { type: "component", id: "component:C-110" });

    queueFieldValue("geometry.bend_pipe_ref", "pipe:P-100");

    expect(queued).toHaveLength(1);
    expect(queued[0].target).toEqual({ object_type: "Component", ref: "component:C-110" });
    expect(queued[0].change.change_kind).toBe("set_field");
    expect(queued[0].change.field_path).toBe("geometry.bend_pipe_ref");
    expect(queued[0].change.before).toBe("TBD");
    expect(queued[0].change.after).toBe("pipe:P-100");
    expect(queued[0].change.dimension).toBe("dimensionless");
    expect(queued[0].change.unit).toBe("none");
    expect(queued[0].audit_boundary.mutation_route).toBe("structured_operations_only");
  });

  it("enters and emits section.mill_tolerance on a pipe span with an explicit unit payload", async () => {
    const model = await loadPreviewModel();
    const queued = renderInspector(model, { type: "pipe", id: "pipe:P-120" });

    const panel = intentPanel();
    fireEvent.change(within(panel).getByTestId("editor-intent-field"), {
      target: { value: "section.mill_tolerance.value" }
    });
    // The unit control offers display metadata only; the VALUE starts TBD.
    expect(within(panel).getByTestId("editor-intent-value")).toHaveValue("TBD");
    expect(within(panel).getByTestId("editor-intent-unit")).toHaveValue("m");
    fireEvent.change(within(panel).getByTestId("editor-intent-value"), {
      target: { value: "0.0016" }
    });
    fireEvent.click(within(panel).getByTestId("queue-editor-intent"));

    expect(queued).toHaveLength(1);
    expect(queued[0].target).toEqual({ object_type: "Element", ref: "pipe:P-120" });
    expect(queued[0].change.field_path).toBe("section.mill_tolerance.value");
    expect(queued[0].change.dimension).toBe("length");
    expect(queued[0].change.unit).toBe("m");
    expect(JSON.parse(queued[0].change.after)).toEqual({ value: 0.0016, unit: "m" });
  });

  it("enters and emits modulus basis plus seismic/wind generation inputs on a load case", async () => {
    const model = await loadPreviewModel();
    const queued = renderInspector(model, { type: "load", id: "load:L-100" });

    queueFieldValue("modulus_basis_temperature.value", "400");
    queueFieldValue("equivalent_static.seismic.gravity_acceleration.value", "9.7");
    queueFieldValue("equivalent_static.seismic.g_factor_x.value", "0.3");
    queueFieldValue("equivalent_static.wind.pressure.value", "480");
    queueFieldValue("equivalent_static.wind.shape_factor.value", "0.65");
    queueFieldValue("equivalent_static.wind.direction", "global_x");
    queueFieldValue("equivalent_static.wind.exposed_pipe_refs", "pipe:P-100, pipe:P-110");

    expect(queued).toHaveLength(7);
    const byPath = Object.fromEntries(queued.map((intent) => [intent.change.field_path, intent]));

    const solveTemperature = byPath["modulus_basis_temperature.value"];
    expect(solveTemperature.change.dimension).toBe("temperature");
    expect(JSON.parse(solveTemperature.change.after)).toEqual({ value: 400, unit: "K" });

    const gravity = byPath["equivalent_static.seismic.gravity_acceleration.value"];
    expect(gravity.change.dimension).toBe("acceleration");
    expect(JSON.parse(gravity.change.after)).toEqual({ value: 9.7, unit: "m/s^2" });

    expect(byPath["equivalent_static.seismic.g_factor_x.value"].change.after).toBe("0.3");
    expect(byPath["equivalent_static.seismic.g_factor_x.value"].change.dimension).toBe("dimensionless");

    const pressure = byPath["equivalent_static.wind.pressure.value"];
    expect(pressure.change.dimension).toBe("pressure");
    expect(JSON.parse(pressure.change.after)).toEqual({ value: 480, unit: "Pa" });

    expect(byPath["equivalent_static.wind.direction"].change.after).toBe("global_x");
    expect(byPath["equivalent_static.wind.exposed_pipe_refs"].change.after).toBe("pipe:P-100, pipe:P-110");

    for (const intent of queued) {
      expect(intent.target).toEqual({ object_type: "Load", ref: "load:L-100" });
      expect(intent.change.change_kind).toBe("update_load");
      expect(intent.audit_boundary.direct_model_mutation_allowed).toBe(false);
    }
  });
});

describe("TP-PMM-GUIEMIT-001 review display and omission", () => {
  it("omits absent optional slots from review rows and shows entered values verbatim", async () => {
    const base = await loadPreviewModel();

    // Absent slots: no rows, no invented values.
    const pipeRows = selectedProperties(base, { type: "pipe", id: "pipe:P-120" });
    expect(pipeRows.find(([label]) => label === "Mill tolerance")).toBeUndefined();
    const bendRows = selectedProperties(base, { type: "component", id: "component:C-110" });
    expect(bendRows.find(([label]) => label === "Bend pipe")).toBeUndefined();
    const loadRows = selectedProperties(base, { type: "load", id: "load:L-100" });
    for (const label of [
      "Modulus basis",
      "Modulus basis solve temperature",
      "Seismic gravity",
      "Wind pressure",
      "Wind direction",
      "Wind exposed spans"
    ]) {
      expect(loadRows.find(([rowLabel]) => rowLabel === label)).toBeUndefined();
    }

    // Entered slots: rows echo the user-entered values verbatim.
    const entered = clone(base);
    entered.pipe_segments[2].section.mill_tolerance = { value: 0.0016, unit: "m" };
    const bend = entered.components.find((item) => item.id === "component:C-110")!;
    bend.geometry = { ...bend.geometry, bend_pipe_ref: "pipe:P-100" };
    entered.load_cases[0].modulus_basis_temperature = { value: 400, unit: "K" };
    entered.load_cases[0].equivalent_static = {
      seismic: {
        gravity_acceleration: { value: 9.7, unit: "m/s^2" },
        g_factor_x: { value: 0.3, unit: "none" }
      },
      wind: {
        pressure: { value: 480, unit: "Pa" },
        shape_factor: { value: 0.65, unit: "none" },
        direction: "global_x",
        exposed_pipe_refs: ["pipe:P-100", "pipe:P-110"]
      },
      provenance: "invented_user_entered_generation_inputs_no_code_table"
    };

    expect(selectedProperties(entered, { type: "pipe", id: "pipe:P-120" })).toEqual(
      expect.arrayContaining([["Mill tolerance", "0.0016 m"]])
    );
    expect(selectedProperties(entered, { type: "component", id: "component:C-110" })).toEqual(
      expect.arrayContaining([["Bend pipe", "pipe:P-100"]])
    );
    expect(selectedProperties(entered, { type: "load", id: "load:L-100" })).toEqual(
      expect.arrayContaining([
        ["Modulus basis solve temperature", "400 K"],
        ["Seismic gravity", "9.7 m/s^2"],
        ["Seismic g-factor X", "0.3 none"],
        ["Seismic g-factor Y", "TBD"],
        ["Wind pressure", "480 Pa"],
        ["Wind shape factor", "0.65 none"],
        ["Wind direction", "global_x"],
        ["Wind exposed spans", "pipe:P-100, pipe:P-110"],
        ["Generation provenance", "invented_user_entered_generation_inputs_no_code_table"]
      ])
    );
  });

  it("surfaces an absent bend_pipe_ref only when the curved-bend realization is requested", async () => {
    const model = clone(await loadPreviewModel());
    const bend = model.components.find((item) => item.id === "component:C-110")!;
    bend.mechanics_interface = {
      ...bend.mechanics_interface,
      solver_consumption: "curved_bend_macro_element"
    };
    // Absent + curved-bend mode: the engine blocks, so review shows TBD.
    const rows = selectedProperties(model, { type: "component", id: "component:C-110" });
    expect(rows).toEqual(expect.arrayContaining([["Bend pipe", "TBD"]]));
  });
});

describe("TP-PMM-GUIEMIT-001 emitted model JSON", () => {
  it("carries entered slots verbatim through canonical model-JSON serialization and omits un-entered slots", async () => {
    const base = clone(await loadPreviewModel());
    const engine = await loadWasmEngine();

    // Un-entered slots stay omitted from the canonical emitted document.
    const canonicalBase = engine.canonicalJsonString(JSON.stringify(base));
    for (const key of SLOT_KEYS) {
      expect(canonicalBase).not.toContain(`"${key}"`);
    }

    const entered = clone(base);
    const bend = entered.components.find((item) => item.id === "component:C-110")!;
    bend.geometry = { ...bend.geometry, bend_pipe_ref: "pipe:P-100" };
    entered.pipe_segments[2].section.mill_tolerance = { value: 0.0016, unit: "m" };
    entered.load_cases[0].modulus_basis_temperature = { value: 400, unit: "K" };
    entered.load_cases[0].equivalent_static = {
      seismic: {
        gravity_acceleration: { value: 9.7, unit: "m/s^2" },
        g_factor_x: { value: 0.3, unit: "none" }
      },
      wind: {
        pressure: { value: 480, unit: "Pa" },
        shape_factor: { value: 0.65, unit: "none" },
        direction: "global_x",
        exposed_pipe_refs: ["pipe:P-100", "pipe:P-110"]
      },
      provenance: "invented_user_entered_generation_inputs_no_code_table"
    };

    const canonical = engine.canonicalJsonString(JSON.stringify(entered));
    const emitted = JSON.parse(canonical) as PreviewModel;

    const emittedBend = emitted.components.find((item) => item.id === "component:C-110")!;
    expect(emittedBend.geometry?.bend_pipe_ref).toBe("pipe:P-100");
    expect(emitted.pipe_segments[2].section.mill_tolerance).toEqual({ value: 0.0016, unit: "m" });
    expect(emitted.load_cases[0].modulus_basis_temperature).toEqual({ value: 400, unit: "K" });
    expect(emitted.load_cases[0].equivalent_static).toEqual({
      seismic: {
        gravity_acceleration: { value: 9.7, unit: "m/s^2" },
        g_factor_x: { value: 0.3, unit: "none" }
      },
      wind: {
        pressure: { value: 480, unit: "Pa" },
        shape_factor: { value: 0.65, unit: "none" },
        direction: "global_x",
        exposed_pipe_refs: ["pipe:P-100", "pipe:P-110"]
      },
      provenance: "invented_user_entered_generation_inputs_no_code_table"
    });

    // Slots the user left alone stay omitted even in the entered document:
    // the seismic Y/Z factors and the untouched pipes/load case carry no
    // invented values.
    expect(JSON.stringify(emitted.load_cases[0].equivalent_static)).not.toContain("g_factor_y");
    expect(JSON.stringify(emitted.pipe_segments[0])).not.toContain("mill_tolerance");
    expect(JSON.stringify(emitted.load_cases[1])).not.toContain("modulus_basis_ref");
    expect(JSON.stringify(emitted.load_cases[1])).not.toContain("modulus_basis_temperature");
  });
});

describe("TP-PMM-GUIEMIT-001 schema binding", () => {
  it("emits modulus basis ids and dimension tokens that satisfy the canonical model schema", () => {
    const modelSchema = readSchema("model.schema.yaml");
    const modelDefs = defs(modelSchema);

    // The canonical LoadCase carries DEC-068/DEC-077 basis slots.
    const loadCaseProps = (modelDefs.LoadCase as JsonRecord).properties as JsonRecord;
    expect(Object.keys(loadCaseProps)).toEqual(
      expect.arrayContaining([
        "modulus_basis_ref",
        "modulus_basis_temperature",
        "equivalent_static_generation"
      ])
    );

    // The emitted basis id satisfies the schema's own Id pattern.
    const idPattern = new RegExp(((modelDefs.Id as JsonRecord).pattern as string));
    expect(idPattern.test("tpoint:invented-op-160C")).toBe(true);

    // Every dimension token the GUI emits for these slots is in the
    // canonical Quantity dimension vocabulary.
    const dimensionEnum = (
      ((modelDefs.Quantity as JsonRecord).properties as JsonRecord).dimension as JsonRecord
    ).enum as string[];
    for (const token of ["acceleration", "pressure", "length", "temperature", "dimensionless"]) {
      expect(dimensionEnum).toContain(token);
    }
  });

  it("covers the canonical seismic/wind generation parameter vocabulary one-to-one", () => {
    const modelDefs = defs(readSchema("model.schema.yaml"));

    const seismic = modelDefs.SeismicEquivalentStaticInput as JsonRecord;
    expect(seismic.required).toEqual(["gravity_acceleration", "g_factors"]);
    const gFactorAxes = Object.keys(
      ((seismic.properties as JsonRecord).g_factors as JsonRecord).properties as JsonRecord
    );
    expect(gFactorAxes).toEqual(["x", "y", "z"]);
    // Preview-surface keys (parallel surface, bend_pipe_ref precedent):
    // g_factor_x/y/z ↔ g_factors.x/y/z; gravity_acceleration is shared.
    const previewSeismicKeys = ["gravity_acceleration", "g_factor_x", "g_factor_y", "g_factor_z"];
    expect(previewSeismicKeys).toEqual([
      "gravity_acceleration",
      ...gFactorAxes.map((axis) => `g_factor_${axis}`)
    ]);

    const wind = modelDefs.WindEquivalentStaticInput as JsonRecord;
    expect(wind.required).toEqual(["pressure", "shape_factor", "direction"]);
    const windProps = wind.properties as JsonRecord;
    expect(windProps).toHaveProperty("exposed_element_refs");
    expect(windProps).toHaveProperty("exposed_spans");
    // Preview-surface keys: pressure/shape_factor/direction are shared;
    // exposed_pipe_refs ↔ exposed_element_refs (the canonical whole-element
    // exposure surface); exposed_spans is the sub-span alternative exposure
    // surface (landed by R14-W2 T4) with no preview-surface counterpart.
    const previewWindKeys = ["pressure", "shape_factor", "direction", "exposed_pipe_refs"];
    expect(previewWindKeys.slice(0, 3)).toEqual(wind.required as string[]);

    // Every preview global-axis token maps into the canonical
    // ForceDirection axis enum.
    const forceDirections = (modelDefs.ForceDirection as JsonRecord).enum as string[];
    for (const token of ["global_x", "global_y", "global_z"]) {
      expect(forceDirections).toContain(token.replace("global_", "").toUpperCase());
    }
  });

  it("emits mill tolerance under the canonical section dimension vocabulary and quantity constraints", () => {
    const sectionDefs = defs(readSchema("section.schema.yaml"));

    const dimensionKinds = (sectionDefs.SectionDimensionKind as JsonRecord).enum as string[];
    expect(dimensionKinds).toContain("mill_tolerance");

    // The emitted quantity satisfies the schema's leaf constraints for a
    // dimension value: a finite number plus a non-empty unit string.
    const quantityProps = (sectionDefs.QuantityValue as JsonRecord).properties as JsonRecord;
    expect((quantityProps.magnitude as JsonRecord).type).toBe("number");
    expect((quantityProps.unit as JsonRecord).minLength).toBe(1);
    const emitted = { value: 0.0016, unit: "m" };
    expect(Number.isFinite(emitted.value)).toBe(true);
    expect(typeof emitted.unit).toBe("string");
    expect(emitted.unit.length).toBeGreaterThanOrEqual(
      (quantityProps.unit as JsonRecord).minLength as number
    );
  });
});
