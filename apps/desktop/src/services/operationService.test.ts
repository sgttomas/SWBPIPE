import { describe, expect, it } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../types";
import { applyModelOperation, validateModelOperation } from "./operationService";

function sampleModel(): PreviewModel {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.product_preview.model",
    data_boundary: {},
    project: { id: "project:test", name: "Test", description: "test", units: { length: "m", force: "N", angle: "rad", pressure: "Pa", temperature: "degC" } },
    analysis_status: { mechanics: "ready", rule_check: "not_performed", professional_acceptance: "not_provided" },
    materials: [
      {
        id: "material:steel",
        label: "Invented steel",
        elastic_modulus: { value: 200000000000, unit: "Pa" },
        shear_modulus: { value: 77000000000, unit: "Pa" },
        thermal_expansion_coefficient: { value: 0.000012, unit: "1/degC" },
        provenance: "invented_example"
      }
    ],
    nodes: [
      { id: "node:N-1", label: "Anchor", position: { x: 0, y: 0, z: 0 }, provenance: "invented_example" },
      { id: "node:N-2", label: "Elbow", position: { x: 3.2, y: 0, z: 0 }, provenance: "invented_example" }
    ],
    pipe_segments: [
      {
        id: "pipe:P-1",
        label: "Run",
        from: "node:N-1",
        to: "node:N-2",
        section: {
          outside_diameter: { value: 0.168, unit: "m" },
          wall_thickness: { value: 0.007, unit: "m" }
        },
        material: "material:steel",
        provenance: "invented_example"
      }
    ],
    supports: [
      { id: "support:S-1", label: "Anchor support", node: "node:N-1", restraints: ["UX", "UY", "UZ"], provenance: "invented_example" }
    ],
    components: [],
    load_cases: [],
    diagnostics: []
  };
}

function intentFor(
  objectType: EditorOperationIntent["target"]["object_type"],
  targetRef: string,
  changeKind: EditorOperationIntent["change"]["change_kind"],
  fieldPath: string,
  before: string,
  after: string,
  unit: string,
  dimension: string
): EditorOperationIntent {
  return {
    operation_id: `op:test-${fieldPath.replace(/\./g, "-")}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    target: { object_type: objectType, ref: targetRef },
    change: {
      change_id: `change:test-${fieldPath.replace(/\./g, "-")}`,
      change_kind: changeKind,
      field_label: fieldPath,
      field_path: fieldPath,
      before,
      after,
      unit,
      dimension,
      source_note: "test"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: "test intent"
  };
}

describe("operationService browser-mode engine", () => {
  it("applies a current quantity edit to a new model document without mutating the input", async () => {
    const model = sampleModel();
    const snapshot = JSON.parse(JSON.stringify(model));
    const intent = intentFor("Material", "material:steel", "set_field", "elastic_modulus.value", "200000000000", "195000000000", "Pa", "stress");

    const outcome = await applyModelOperation(model, intent, null);

    expect(model).toEqual(snapshot);
    expect(outcome.application_route).toBe("browser_fixture_local_apply");
    expect(outcome.validation.application_status).toBe("applied_to_session_model");
    expect(outcome.acceptance.acceptance_basis).toBe("user_initiated_apply_in_local_session");
    expect(outcome.acceptance.acceptance_is_professional_approval).toBe(false);
    expect(outcome.applied_model?.materials?.[0].elastic_modulus.value).toBe(195000000000);
    expect(outcome.applied_model_backend_hash).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(outcome.diff_preview).toHaveLength(1);
    expect(outcome.diff_preview[0].before).toBe("200000000000");
    expect(outcome.professional_boundary.software_makes_compliance_claim).toBe(false);
  });

  it("validates without applying and reports generated diff rows", async () => {
    const model = sampleModel();
    const intent = intentFor("Node", "node:N-2", "set_field", "position.x", "3.2", "3.5", "m", "length");

    const outcome = await validateModelOperation(model, intent, null);

    expect(outcome.mode).toBe("validate_only");
    expect(outcome.validation.application_status).toBe("not_applied");
    expect(outcome.validation.diff_preview_status).toBe("generated");
    expect(outcome.applied_model).toBeNull();
  });

  it("applies an explicit create-node payload without mutating the input", async () => {
    const model = sampleModel();
    const snapshot = JSON.parse(JSON.stringify(model));
    const payload = {
      id: "node:N-3",
      label: "New node",
      position: { x: 4.5, y: 1.25, z: 0.75 },
      provenance: "user_entered_local_preview"
    };
    const intent = intentFor("Node", "node:N-3", "create_node", "nodes", "not_present", JSON.stringify(payload), "m", "length");
    intent.operation_kind = "create";

    const outcome = await applyModelOperation(model, intent, null);

    expect(model).toEqual(snapshot);
    expect(outcome.validation.application_status).toBe("applied_to_session_model");
    expect(outcome.validation.reference_validation).toBe("passed");
    expect(outcome.validation.unit_validation).toBe("passed");
    expect(outcome.diff_preview).toHaveLength(1);
    expect(outcome.diff_preview[0].field_path).toBe("nodes");
    expect(outcome.applied_model?.nodes).toHaveLength(3);
    expect(outcome.applied_model?.nodes[2]).toEqual(payload);
    expect(outcome.professional_boundary.software_makes_approval_claim).toBe(false);
  });

  it("applies an explicit empty load-case payload without mutating the input", async () => {
    const model = sampleModel();
    const snapshot = JSON.parse(JSON.stringify(model));
    const payload = {
      id: "load:L-1",
      label: "User load case",
      kind: "primitive_user_load",
      status: "draft",
      provenance: "user_entered_local_preview",
      primitive_loads: []
    };
    const intent = intentFor(
      "Load",
      "load:L-1",
      "create_load_case",
      "load_cases",
      "not_present",
      JSON.stringify(payload),
      "none",
      "dimensionless"
    );
    intent.operation_kind = "create";

    const outcome = await applyModelOperation(model, intent, null);

    expect(model).toEqual(snapshot);
    expect(outcome.validation.application_status).toBe("applied_to_session_model");
    expect(outcome.validation.reference_validation).toBe("passed");
    expect(outcome.validation.unit_validation).toBe("passed");
    expect(outcome.diff_preview).toHaveLength(1);
    expect(outcome.diff_preview[0].field_path).toBe("load_cases");
    expect(outcome.applied_model?.load_cases).toHaveLength(1);
    expect(outcome.applied_model?.load_cases[0]).toEqual(payload);
    expect(outcome.professional_boundary.software_makes_compliance_claim).toBe(false);

    const nonEmptyPayload = {
      ...payload,
      id: "load:L-2",
      primitive_loads: [{ id: "load:L-2-F" }]
    };
    const nonEmpty = intentFor(
      "Load",
      "load:L-2",
      "create_load_case",
      "load_cases",
      "not_present",
      JSON.stringify(nonEmptyPayload),
      "none",
      "dimensionless"
    );
    nonEmpty.operation_kind = "create";
    const blocked = await applyModelOperation(model, nonEmpty, null);
    expect(blocked.validation.application_status).toBe("blocked");
    expect(blocked.diagnostics.map((item) => item.code)).toContain("OP-CREATE-LOAD-CASE-PAYLOAD-INVALID");
    expect(blocked.applied_model).toBeNull();
  });

  it("applies an explicit concentrated-force primitive load payload without mutating the input", async () => {
    const model = sampleModel();
    model.load_cases.push({
      id: "load:L-1",
      label: "User load case",
      kind: "primitive_user_load",
      status: "draft",
      provenance: "user_entered_local_preview",
      primitive_loads: []
    });
    const snapshot = JSON.parse(JSON.stringify(model));
    const payload = {
      id: "load:L-1-F1",
      category: "concentrated_force",
      target: { type: "node", node: "node:N-2" },
      direction: "global_y",
      magnitude: { value: 250, unit: "N" },
      dimension: "force",
      provenance: "user_entered_local_preview"
    };
    const intent = intentFor(
      "Load",
      "load:L-1",
      "create_primitive_load",
      "primitive_loads",
      "not_present",
      JSON.stringify(payload),
      "N",
      "force"
    );
    intent.operation_kind = "create";

    const outcome = await applyModelOperation(model, intent, null);

    expect(model).toEqual(snapshot);
    expect(outcome.validation.application_status).toBe("applied_to_session_model");
    expect(outcome.validation.reference_validation).toBe("passed");
    expect(outcome.validation.unit_validation).toBe("passed");
    expect(outcome.diff_preview).toHaveLength(1);
    expect(outcome.diff_preview[0].field_path).toBe("primitive_loads");
    expect(outcome.applied_model?.load_cases[0].primitive_loads).toEqual([payload]);

    const duplicate = await applyModelOperation(outcome.applied_model!, intent, null);
    expect(duplicate.validation.application_status).toBe("blocked");
    expect(duplicate.diagnostics.map((item) => item.code)).toContain("OP-TARGET-ALREADY-EXISTS");

    const missingNodePayload = { ...payload, id: "load:L-1-F2", target: { type: "node", node: "node:missing" } };
    const missingNode = intentFor(
      "Load",
      "load:L-1",
      "create_primitive_load",
      "primitive_loads",
      "not_present",
      JSON.stringify(missingNodePayload),
      "N",
      "force"
    );
    missingNode.operation_kind = "create";
    const blocked = await applyModelOperation(model, missingNode, null);
    expect(blocked.validation.application_status).toBe("blocked");
    expect(blocked.diagnostics.map((item) => item.code)).toContain("OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND");
  });

  it("applies an explicit distributed-force primitive load payload without mutating the input", async () => {
    const model = sampleModel();
    model.load_cases.push({
      id: "load:L-1",
      label: "User load case",
      kind: "primitive_user_load",
      status: "draft",
      provenance: "user_entered_local_preview",
      primitive_loads: []
    });
    const snapshot = JSON.parse(JSON.stringify(model));
    const payload = {
      id: "load:L-1-D1",
      category: "distributed_force",
      target: { type: "element", pipe: "pipe:P-1" },
      direction: "global_z",
      magnitude: { value: 125, unit: "N/m" },
      dimension: "force_per_length",
      provenance: "user_entered_local_preview"
    };
    const intent = intentFor(
      "Load",
      "load:L-1",
      "create_primitive_load",
      "primitive_loads",
      "not_present",
      JSON.stringify(payload),
      "N/m",
      "force_per_length"
    );
    intent.operation_kind = "create";

    const outcome = await applyModelOperation(model, intent, null);

    expect(model).toEqual(snapshot);
    expect(outcome.validation.application_status).toBe("applied_to_session_model");
    expect(outcome.validation.reference_validation).toBe("passed");
    expect(outcome.validation.unit_validation).toBe("passed");
    expect(outcome.diff_preview).toHaveLength(1);
    expect(outcome.diff_preview[0].unit).toBe("N/m");
    expect(outcome.applied_model?.load_cases[0].primitive_loads).toEqual([payload]);

    const duplicate = await applyModelOperation(outcome.applied_model!, intent, null);
    expect(duplicate.validation.application_status).toBe("blocked");
    expect(duplicate.diagnostics.map((item) => item.code)).toContain("OP-TARGET-ALREADY-EXISTS");

    const missingPipePayload = { ...payload, id: "load:L-1-D2", target: { type: "element", pipe: "pipe:missing" } };
    const missingPipe = intentFor(
      "Load",
      "load:L-1",
      "create_primitive_load",
      "primitive_loads",
      "not_present",
      JSON.stringify(missingPipePayload),
      "N/m",
      "force_per_length"
    );
    missingPipe.operation_kind = "create";
    const blocked = await applyModelOperation(model, missingPipe, null);
    expect(blocked.validation.application_status).toBe("blocked");
    expect(blocked.diagnostics.map((item) => item.code)).toContain("OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND");
  });

  it("applies an explicit concentrated-moment primitive load payload without mutating the input", async () => {
    const model = sampleModel();
    model.load_cases.push({
      id: "load:L-1",
      label: "User load case",
      kind: "primitive_user_load",
      status: "draft",
      provenance: "user_entered_local_preview",
      primitive_loads: []
    });
    const snapshot = JSON.parse(JSON.stringify(model));
    const payload = {
      id: "load:L-1-M1",
      category: "concentrated_moment",
      target: { type: "node", node: "node:N-2" },
      direction: "rotation_z",
      magnitude: { value: 80, unit: "N*m" },
      dimension: "moment",
      provenance: "user_entered_local_preview"
    };
    const intent = intentFor(
      "Load",
      "load:L-1",
      "create_primitive_load",
      "primitive_loads",
      "not_present",
      JSON.stringify(payload),
      "N*m",
      "moment"
    );
    intent.operation_kind = "create";

    const outcome = await applyModelOperation(model, intent, null);

    expect(model).toEqual(snapshot);
    expect(outcome.validation.application_status).toBe("applied_to_session_model");
    expect(outcome.validation.reference_validation).toBe("passed");
    expect(outcome.validation.unit_validation).toBe("passed");
    expect(outcome.diff_preview).toHaveLength(1);
    expect(outcome.diff_preview[0].dimension).toBe("moment");
    expect(outcome.applied_model?.load_cases[0].primitive_loads).toEqual([payload]);

    const duplicate = await applyModelOperation(outcome.applied_model!, intent, null);
    expect(duplicate.validation.application_status).toBe("blocked");
    expect(duplicate.diagnostics.map((item) => item.code)).toContain("OP-TARGET-ALREADY-EXISTS");

    const missingNodePayload = { ...payload, id: "load:L-1-M2", target: { type: "node", node: "node:missing" } };
    const missingNode = intentFor(
      "Load",
      "load:L-1",
      "create_primitive_load",
      "primitive_loads",
      "not_present",
      JSON.stringify(missingNodePayload),
      "N*m",
      "moment"
    );
    missingNode.operation_kind = "create";
    const blocked = await applyModelOperation(model, missingNode, null);
    expect(blocked.validation.application_status).toBe("blocked");
    expect(blocked.diagnostics.map((item) => item.code)).toContain("OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND");
  });

  it("applies explicit pressure and thermal primitive load payloads without mutating the input", async () => {
    const model = sampleModel();
    model.load_cases.push({
      id: "load:L-1",
      label: "User load case",
      kind: "primitive_user_load",
      status: "draft",
      provenance: "user_entered_local_preview",
      primitive_loads: []
    });
    const snapshot = JSON.parse(JSON.stringify(model));
    const pressurePayload = {
      id: "load:L-1-P1",
      category: "pressure",
      target: { type: "element", pipe: "pipe:P-1" },
      direction: "global_x",
      magnitude: { value: 1200000, unit: "Pa" },
      dimension: "pressure",
      provenance: "user_entered_local_preview"
    };
    const pressureIntent = intentFor(
      "Load",
      "load:L-1",
      "create_primitive_load",
      "primitive_loads",
      "not_present",
      JSON.stringify(pressurePayload),
      "Pa",
      "pressure"
    );
    pressureIntent.operation_kind = "create";

    const pressureOutcome = await applyModelOperation(model, pressureIntent, null);

    expect(model).toEqual(snapshot);
    expect(pressureOutcome.validation.application_status).toBe("applied_to_session_model");
    expect(pressureOutcome.validation.reference_validation).toBe("passed");
    expect(pressureOutcome.validation.unit_validation).toBe("passed");
    expect(pressureOutcome.applied_model?.load_cases[0].primitive_loads).toEqual([pressurePayload]);

    const thermalPayload = {
      id: "load:L-1-T1",
      category: "thermal",
      target: { type: "element", pipe: "pipe:P-1" },
      direction: "global_z",
      magnitude: { value: 12.5, unit: "degC" },
      dimension: "temperature_interval",
      provenance: "user_entered_local_preview"
    };
    const thermalIntent = intentFor(
      "Load",
      "load:L-1",
      "create_primitive_load",
      "primitive_loads",
      "not_present",
      JSON.stringify(thermalPayload),
      "degC",
      "temperature_interval"
    );
    thermalIntent.operation_kind = "create";

    const thermalOutcome = await applyModelOperation(pressureOutcome.applied_model!, thermalIntent, null);

    expect(thermalOutcome.validation.application_status).toBe("applied_to_session_model");
    expect(thermalOutcome.validation.reference_validation).toBe("passed");
    expect(thermalOutcome.validation.unit_validation).toBe("passed");
    expect(thermalOutcome.applied_model?.load_cases[0].primitive_loads).toEqual([pressurePayload, thermalPayload]);

    const missingPipePayload = { ...pressurePayload, id: "load:L-1-P2", target: { type: "element", pipe: "pipe:missing" } };
    const missingPipe = intentFor(
      "Load",
      "load:L-1",
      "create_primitive_load",
      "primitive_loads",
      "not_present",
      JSON.stringify(missingPipePayload),
      "Pa",
      "pressure"
    );
    missingPipe.operation_kind = "create";
    const blocked = await applyModelOperation(model, missingPipe, null);
    expect(blocked.validation.application_status).toBe("blocked");
    expect(blocked.diagnostics.map((item) => item.code)).toContain("OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND");
  });

  it("applies explicit imposed-displacement primitive load payloads without mutating the input", async () => {
    const model = sampleModel();
    model.load_cases.push({
      id: "load:L-1",
      label: "User load case",
      kind: "primitive_user_load",
      status: "draft",
      provenance: "user_entered_local_preview",
      primitive_loads: []
    });
    const snapshot = JSON.parse(JSON.stringify(model));
    const displacementPayload = {
      id: "load:L-1-I1",
      category: "imposed_displacement",
      target: { type: "support", support: "support:S-1", dof: "UZ" },
      direction: "UZ",
      magnitude: { value: -0.006, unit: "m" },
      dimension: "displacement",
      provenance: "user_entered_local_preview"
    };
    const displacementIntent = intentFor(
      "Load",
      "load:L-1",
      "create_primitive_load",
      "primitive_loads",
      "not_present",
      JSON.stringify(displacementPayload),
      "m",
      "displacement"
    );
    displacementIntent.operation_kind = "create";

    const displacementOutcome = await applyModelOperation(model, displacementIntent, null);

    expect(model).toEqual(snapshot);
    expect(displacementOutcome.validation.application_status).toBe("applied_to_session_model");
    expect(displacementOutcome.validation.reference_validation).toBe("passed");
    expect(displacementOutcome.validation.unit_validation).toBe("passed");
    expect(displacementOutcome.applied_model?.load_cases[0].primitive_loads).toEqual([displacementPayload]);

    const rotationPayload = {
      id: "load:L-1-I2",
      category: "imposed_displacement",
      target: { type: "support", support: "support:S-1", dof: "RX" },
      direction: "RX",
      magnitude: { value: 0.01, unit: "rad" },
      dimension: "rotation",
      provenance: "user_entered_local_preview"
    };
    const rotationIntent = intentFor(
      "Load",
      "load:L-1",
      "create_primitive_load",
      "primitive_loads",
      "not_present",
      JSON.stringify(rotationPayload),
      "rad",
      "rotation"
    );
    rotationIntent.operation_kind = "create";

    const rotationOutcome = await applyModelOperation(displacementOutcome.applied_model!, rotationIntent, null);

    expect(rotationOutcome.validation.application_status).toBe("applied_to_session_model");
    expect(rotationOutcome.validation.reference_validation).toBe("passed");
    expect(rotationOutcome.validation.unit_validation).toBe("passed");
    expect(rotationOutcome.applied_model?.load_cases[0].primitive_loads).toEqual([displacementPayload, rotationPayload]);

    const missingSupportPayload = {
      ...displacementPayload,
      id: "load:L-1-I3",
      target: { type: "support", support: "support:missing", dof: "UZ" }
    };
    const missingSupport = intentFor(
      "Load",
      "load:L-1",
      "create_primitive_load",
      "primitive_loads",
      "not_present",
      JSON.stringify(missingSupportPayload),
      "m",
      "displacement"
    );
    missingSupport.operation_kind = "create";
    const blocked = await applyModelOperation(model, missingSupport, null);
    expect(blocked.validation.application_status).toBe("blocked");
    expect(blocked.diagnostics.map((item) => item.code)).toContain("OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND");
  });

  it("applies combination basis edits while whole terms remain deferred", async () => {
    const model = sampleModel();
    model.combinations = [
      {
        id: "combination:C-OP",
        label: "Operating",
        basis: "mechanics",
        terms: [{ load_case: "load:L-1", factor: 1 }],
        provenance: "invented_example"
      }
    ];
    const snapshot = JSON.parse(JSON.stringify(model));
    const intent = intentFor(
      "Combination",
      "combination:C-OP",
      "update_load",
      "basis",
      "mechanics",
      "mechanics_user_review",
      "none",
      "dimensionless"
    );

    const outcome = await applyModelOperation(model, intent, null);

    expect(model).toEqual(snapshot);
    expect(outcome.diagnostics).toEqual([]);
    expect(outcome.validation.application_status).toBe("applied_to_session_model");
    expect(outcome.applied_model?.combinations?.[0].basis).toBe("mechanics_user_review");
    expect(outcome.applied_model?.combinations?.[0].terms).toEqual([{ load_case: "load:L-1", factor: 1 }]);

    const empty = await applyModelOperation(
      model,
      intentFor("Combination", "combination:C-OP", "update_load", "basis", "mechanics", " ", "none", "dimensionless"),
      null
    );
    expect(empty.validation.application_status).toBe("blocked");
    expect(empty.diagnostics.map((item) => item.code)).toContain("OP-VALUE-EMPTY");

    const wholeTerms = await applyModelOperation(
      model,
      intentFor(
        "Combination",
        "combination:C-OP",
        "update_load",
        "terms",
        "load:L-1 x 1",
        "load:L-1 x 0.75",
        "none",
        "dimensionless"
      ),
      null
    );
    expect(wholeTerms.validation.application_status).toBe("blocked");
    expect(wholeTerms.diagnostics.map((item) => item.code)).toContain("OP-FIELD-EDIT-DEFERRED");
  });

  it("blocks stale before-values, unit mismatches, and unknown dimensions", async () => {
    const model = sampleModel();

    const stale = await applyModelOperation(
      model,
      intentFor("Material", "material:steel", "set_field", "elastic_modulus.value", "1", "2", "Pa", "stress"),
      null
    );
    expect(stale.validation.application_status).toBe("blocked");
    expect(stale.validation.before_state_validation).toBe("blocked_stale");
    expect(stale.diagnostics.map((item) => item.code)).toContain("OP-STALE-BEFORE-VALUE");

    const unitMismatch = await applyModelOperation(
      model,
      intentFor("Material", "material:steel", "set_field", "elastic_modulus.value", "200000000000", "29000", "ksi", "stress"),
      null
    );
    expect(unitMismatch.diagnostics.map((item) => item.code)).toContain("OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE");

    const unknownDimension = await applyModelOperation(
      model,
      intentFor("Material", "material:steel", "set_field", "elastic_modulus.value", "200000000000", "1", "Pa", "made_up"),
      null
    );
    expect(unknownDimension.diagnostics.map((item) => item.code)).toContain("OP-UNIT-DIMENSION-UNKNOWN");
  });

  it("applies restraint sets with vocabulary enforcement and reference checks", async () => {
    const model = sampleModel();

    const applied = await applyModelOperation(
      model,
      intentFor("Support", "support:S-1", "update_support", "restraints", "UX, UY, UZ", "UX, UZ", "none", "dimensionless"),
      null
    );
    expect(applied.applied_model?.supports[0].restraints).toEqual(["UX", "UZ"]);

    const invalidToken = await applyModelOperation(
      model,
      intentFor("Support", "support:S-1", "update_support", "restraints", "UX, UY, UZ", "FLY", "none", "dimensionless"),
      null
    );
    expect(invalidToken.diagnostics.map((item) => item.code)).toContain("OP-RESTRAINT-TOKEN-INVALID");

    const danglingRef = await applyModelOperation(
      model,
      intentFor("Element", "pipe:P-1", "set_field", "material", "material:steel", "material:missing", "none", "dimensionless"),
      null
    );
    expect(danglingRef.diagnostics.map((item) => item.code)).toContain("OP-REFERENCE-NOT-FOUND");
  });

  it("blocks underspecified viewport node gestures without inventing geometry", async () => {
    const model = sampleModel();
    const intent = intentFor("Node", "node:viewport-preview-created", "create_node", "viewport.create_node", "not_present", "node:x", "none", "dimensionless");
    intent.operation_kind = "create";

    const outcome = await applyModelOperation(model, intent, null);

    expect(outcome.validation.application_status).toBe("blocked");
    expect(outcome.diagnostics.map((item) => item.code)).toContain("OP-CREATE-NODE-SHAPE-INVALID");
  });

  it("echoes claimed model hashes without asserting cross-canonicalization equality", async () => {
    const model = sampleModel();
    const intent = intentFor("Material", "material:steel", "set_field", "label", "Invented steel", "Renamed", "none", "dimensionless");
    const outcome = await validateModelOperation(model, intent, {
      algorithm: "sha256",
      canonicalization: "jcs_like_sorted_object_keys",
      payload_scope: "model_payload",
      payload_ref: "project:test",
      value: "sha256:abc",
      hash_status: "computed_local_preview"
    });

    expect(outcome.model_basis.claimed_model_hash).toBe("sha256:abc");
    expect(outcome.model_basis.binding_status).toBe("claimed_hash_echoed_cross_canonicalization_equality_not_evaluated");
    expect(outcome.model_basis.backend_model_hash).toMatch(/^sha256:[0-9a-f]{64}$/);
  });
});
