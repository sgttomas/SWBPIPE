// TP-PMM-GUIEMIT-001 (W-13 design model-document-0.7.0.md §5.2): the desktop
// emission golden. Under a fixed clock this drives each authoring path through
// the desktop's own intent builders and the wasm applier — the blank project
// and the blank load/reference project; node, material, section, pipe,
// support, load and combination creation; and every model-data slot (tags,
// notes, provenance records, lines, assumptions and open issues, constraints,
// equipment interfaces and a rule-pack reference) — and records each emitted
// document in fixtures/model_document/desktop_emitted.json. The pytest
// (tests/test_model_document_schema.py) validates every entry against
// `$defs/DesktopDocument`.
//
// The test fails when a fresh emission differs from the committed golden.
// Regenerate it explicitly with DESKTOP_EMISSION_UPDATE=1 and review the diff.
// All values are invented test input.
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { afterAll, beforeAll, describe, expect, it, vi } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { buildBlankLoadStateModelDocument, buildBlankLocalModelDocument } from "../../services/projectService";
import { applyModelOperation } from "../../services/operationService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { buildNodeCreationSubmission, buildRouteSubmission } from "../viewport/routeDraft";
import { inspectorAuthoringPaths } from "../model-tree/PropertyInspector";
import { loadCaseAuthoringPaths } from "../load-cases/LoadCaseManagerPanel";
import { RECORD_FAMILIES, annotationIntent, recordIntent, rulePackReferenceFromDocument } from "./modelDataIntents";
import { buildMaterialInsert } from "../library/libraryApply";
import type { UnitCatalog } from "../../services/unitCatalogService";

const here = path.dirname(fileURLToPath(import.meta.url));
const projectRoot = path.resolve(here, "../../../../..");
const GOLDEN = path.join(projectRoot, "fixtures/model_document/desktop_emitted.json");
const DEMO_RULE_PACK = path.join(projectRoot, "fixtures/product_preview/invented_demo_rule_pack.json");
const FIXED_CLOCK = new Date("2026-01-01T00:00:00.000Z");
const PUBLIC_PROVENANCE = {
  source_name: "Invented emission handbook",
  source_location: "invented://emission/handbook",
  source_license: "invented-permissive",
  contributor: "Invented contributor",
  contributor_certification: "invented-attestation",
  redistribution_status: "public_permissive",
  review_status: "pending"
};

const EMISSION_CATALOG = {
  entries: [
    { unit_id: "unit:pascal", symbol: "Pa", dimension_id: "pressure" },
    { unit_id: "unit:one", symbol: "1", dimension_id: "dimensionless" }
  ]
} as unknown as UnitCatalog;
const libraryValue = (magnitude: number, unitId: string) => ({
  magnitude, unit_ref: { ref_type: "UnitDefinition", ref_id: unitId }, dimension_id: "stress", quantity_kind: "unit_bearing",
  unit_required: true, missing_unit_behavior: "diagnostic_blocking", provenance: PUBLIC_PROVENANCE
});
const libraryProperty = (kind: string, magnitude: number, unitId: string) => ({
  property_id: `mat.invented.emission.${kind}`, property_kind: kind, value_status: "provided", required_for: "mechanics_solve",
  provenance: PUBLIC_PROVENANCE, review_status: "pending", value: libraryValue(magnitude, unitId)
});
const INVENTED_MATERIAL_LIBRARY = {
  schema_version: "0.1.0",
  material_library: { library_id: "matlib.invented.emission", name: "Invented emission library", library_scope: "public_schema_fixture",
    privacy_class: "public_permissive", provenance: PUBLIC_PROVENANCE, review_status: "pending" },
  material_records: [{
    material_id: "mat.invented.emission", name: "Invented emission steel", material_family: "invented_example", privacy_class: "public_permissive",
    redistribution_status: "public_permissive", properties: [libraryProperty("elastic_modulus", 200000000000, "unit:pascal"), libraryProperty("poisson_ratio", 0.3, "unit:one")],
    allowables: [], completeness: [], provenance: PUBLIC_PROVENANCE, review_status: "pending"
  }]
};

async function apply(model: PreviewModel, intent: EditorOperationIntent): Promise<PreviewModel> {
  const outcome = await applyModelOperation(model, intent, await computeModelHash(model));
  const blocking = outcome.diagnostics.filter((item) => item.severity === "blocking");
  expect(blocking, `${intent.change.change_kind} ${intent.target.ref}`).toEqual([]);
  expect(outcome.validation.application_status).toBe("applied_to_session_model");
  return outcome.applied_model as PreviewModel;
}

function single(build: ReturnType<typeof buildNodeCreationSubmission>): EditorOperationIntent {
  if (!build.ok || build.submission.kind !== "single") throw new Error(`draft refused: ${JSON.stringify(build)}`);
  return build.submission.intent;
}

async function emit(): Promise<Array<{ path: string; document: PreviewModel }>> {
  const entries: Array<{ path: string; document: PreviewModel }> = [];
  const record = (name: string, document: PreviewModel) => entries.push({ path: name, document: structuredClone(document) });

  let model = buildBlankLocalModelDocument(FIXED_CLOCK);
  record("blank_project", model);
  record("blank_load_reference_project", buildBlankLoadStateModelDocument(FIXED_CLOCK));

  // Nodes (viewport node creation).
  for (const [index, x] of [[1, "0"], [2, "3.2"], [3, "3.2"]] as const) {
    const draft = { id: `node:N-${index}`, label: `Node ${index}`, x, y: index === 3 ? "2" : "0", z: "0", coordinateUnit: "m", provenance: "user_entered_local_preview" };
    model = await apply(model, single(buildNodeCreationSubmission(model, [], draft, index, "passed")));
  }
  record("node_creation", model);

  // Material and section (inspector creation).
  const { defaultMaterialDraft, buildCreateMaterialIntent, defaultSectionDraft, buildCreateSectionIntent, defaultSupportDraft, buildCreateSupportIntent } = inspectorAuthoringPaths;
  const material = { ...defaultMaterialDraft(model, []), elasticModulus: "200000000000", shearModulus: "77000000000", thermalExpansion: "0.000012" };
  model = await apply(model, buildCreateMaterialIntent(material, model, null));
  record("material_creation", model);
  const section = { ...defaultSectionDraft(model, []), outsideDiameter: "0.168", wallThickness: "0.007" };
  model = await apply(model, buildCreateSectionIntent(section, model, null));
  record("section_creation", model);

  // Pipes (viewport route between existing nodes).
  for (const [id, from, to] of [["pipe:P-1", "node:N-1", "node:N-2"], ["pipe:P-2", "node:N-2", "node:N-3"]] as const) {
    const draft = {
      startNodeId: from, endMode: "existing" as const, existingEndNodeId: to,
      newEnd: { id: "", label: "", x: "", y: "", z: "", coordinateUnit: "m", provenance: "" },
      pipe: { id, label: `Pipe ${id}`, materialId: material.id, outsideDiameter: "0.168", wallThickness: "0.007", lengthUnit: "m",
        yReferenceX: "0", yReferenceY: "0", yReferenceZ: "1", provenance: "user_entered_local_preview" }
    };
    model = await apply(model, single(buildRouteSubmission(model, [], draft, 10, "passed")));
  }
  record("pipe_creation", model);

  // Support (inspector creation).
  const support = defaultSupportDraft(model, { type: "node", id: "node:N-1" }, []);
  model = await apply(model, buildCreateSupportIntent(support, model, null));
  record("support_creation", model);

  // Load case and a primitive load, then a combination (Load Case Manager).
  const { defaultLoadCaseDraft, buildCreateLoadCaseIntent, defaultPrimitiveLoadDraft, buildCreatePrimitiveLoadIntent, defaultCombinationDraft, buildCreateCombinationIntent } = loadCaseAuthoringPaths;
  model = await apply(model, buildCreateLoadCaseIntent({ model, draft: defaultLoadCaseDraft("load:L-1") }));
  model = await apply(model, buildCreatePrimitiveLoadIntent({ model, draft: { ...defaultPrimitiveLoadDraft(model, "load:L-1"), targetNode: "node:N-2" }, unitCatalogRoute: null }));
  record("load_creation", model);
  model = await apply(model, buildCreateCombinationIntent({ model, draft: defaultCombinationDraft(model, "combination:C-1") }));
  record("combination_creation", model);

  // Every model-data slot, through the intents the model-data UI queues.
  const canonicalize = canonicalJsonString;
  const projectId = model.project.id;
  model = await apply(model, await annotationIntent(canonicalize, "Project", projectId, model.project, "tags", ["invented", "rev-A"], "Tags"));
  model = await apply(model, await annotationIntent(canonicalize, "Project", projectId, model.project, "notes", "Invented project note.", "Notes"));
  record("project_tags_and_notes", model);
  const node = model.nodes[0] as Record<string, unknown>;
  model = await apply(model, await annotationIntent(canonicalize, "Node", "node:N-1", node, "tags", ["anchor"], "Tags"));
  const pipe = model.pipe_segments[0] as Record<string, unknown>;
  model = await apply(model, await annotationIntent(canonicalize, "Element", "pipe:P-1", pipe, "notes", "Invented pipe note.", "Notes"));
  const loadCase = model.load_cases[0] as Record<string, unknown>;
  model = await apply(model, await annotationIntent(canonicalize, "Load", "load:L-1", loadCase, "tags", ["sustained"], "Tags"));
  record("entity_tags_and_notes", model);
  const storedMaterial = model.materials![0] as Record<string, unknown>;
  model = await apply(model, await annotationIntent(canonicalize, "Material", material.id, storedMaterial, "provenance_record", PUBLIC_PROVENANCE, "Provenance record"));
  const storedSupport = model.supports[0] as Record<string, unknown>;
  model = await apply(model, await annotationIntent(canonicalize, "Support", support.id, storedSupport, "provenance_record", PUBLIC_PROVENANCE, "Provenance record"));
  record("provenance_records", model);
  model = await apply(model, await recordIntent(canonicalize, RECORD_FAMILIES.line, "create", { id: "line:L-001", label: "Invented line", pipe_refs: ["pipe:P-1", "pipe:P-2"], provenance: "user_entered" }));
  record("line", model);
  const assumption = { assumption_id: "assumption:A-001", kind: "assumption" as const, statement: "Invented assumption.", status: "unresolved" as const, affected_refs: [support.id] };
  model = await apply(model, await recordIntent(canonicalize, RECORD_FAMILIES.assumption, "create", assumption));
  const issue = { assumption_id: "assumption:A-002", kind: "open_issue" as const, statement: "Invented open issue.", status: "unresolved" as const };
  model = await apply(model, await recordIntent(canonicalize, RECORD_FAMILIES.assumption, "create", issue));
  model = await apply(model, await recordIntent(canonicalize, RECORD_FAMILIES.assumption, "update", { ...issue, status: "resolved", resolution_note: "Invented resolution." }, issue));
  record("assumptions_and_open_issues", model);
  model = await apply(model, await recordIntent(canonicalize, RECORD_FAMILIES.constraint, "create",
    { constraint_id: "constraint:K-001", name: "Invented clearance", constraint_kind: "clearance", state: "proposed", target_refs: ["pipe:P-1"], provenance: "user_entered" }));
  model = await apply(model, await recordIntent(canonicalize, RECORD_FAMILIES.equipmentInterface, "create",
    { id: "equipment-interface:E-001", label: "Invented nozzle", node_ref: "node:N-3", equipment_label: "Invented vessel", provenance: "user_entered" }));
  record("constraints_and_equipment_interfaces", model);
  const pack = JSON.parse(readFileSync(DEMO_RULE_PACK, "utf8")) as Record<string, unknown>;
  model = await apply(model, await recordIntent(canonicalize, RECORD_FAMILIES.rulePackRef, "create", rulePackReferenceFromDocument(pack)));
  record("rule_pack_ref", model);
  // W-17: one library-applied entity (an E/ν record with no stored G inserted
  // through the Library Manager's builder; public invented provenance).
  model = await apply(model, (await buildMaterialInsert({
    model, document: INVENTED_MATERIAL_LIBRARY, recordId: "mat.invented.emission", newId: "material:library-emission", mode: "e_nu", catalog: EMISSION_CATALOG
  })).batch.operations[0]);
  record("library_material_insert", model);
  return entries;
}

describe("TP-PMM-GUIEMIT-001 desktop emission golden", () => {
  beforeAll(async () => {
    vi.useFakeTimers({ toFake: ["Date"] });
    vi.setSystemTime(FIXED_CLOCK);
    await loadWasmEngine();
  });
  afterAll(() => {
    vi.useRealTimers();
  });

  it("emits each authoring path's document exactly as the committed golden records it", async () => {
    const entries = await emit();
    const golden = {
      document_kind: "openpipestress.test.desktop_emission_golden",
      description:
        "Desktop-emitted model documents, one per authoring path, under a fixed clock (TP-PMM-GUIEMIT-001; W-13 design model-document-0.7.0.md §5.2). Invented data. Regenerate with DESKTOP_EMISSION_UPDATE=1.",
      clock: FIXED_CLOCK.toISOString(),
      entries
    };
    const text = `${JSON.stringify(golden, null, 2)}\n`;
    if (process.env.DESKTOP_EMISSION_UPDATE === "1") {
      writeFileSync(GOLDEN, text);
      return;
    }
    const committed = readFileSync(GOLDEN, "utf8");
    expect(text === committed, "fresh emission differs from fixtures/model_document/desktop_emitted.json; regenerate with DESKTOP_EMISSION_UPDATE=1 and review the diff").toBe(true);
    expect(entries.map((entry) => entry.path)).toEqual([
      "blank_project", "blank_load_reference_project", "node_creation", "material_creation", "section_creation",
      "pipe_creation", "support_creation", "load_creation", "combination_creation", "project_tags_and_notes",
      "entity_tags_and_notes", "provenance_records", "line", "assumptions_and_open_issues",
      "constraints_and_equipment_interfaces", "rule_pack_ref", "library_material_insert"
    ]);
  });
});
