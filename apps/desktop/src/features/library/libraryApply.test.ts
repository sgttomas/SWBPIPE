// W-17 library apply (DEL-03-01 Design apply-library-records.md §4): mapping
// tables, the E/ν rule, dispositions, components, the batch through the real
// wasm applier, undo/redo through the session, and the private-data boundary.
// Every library value here is invented.
import { createHash } from "node:crypto";
import { act, renderHook, waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { applyOperationBatch } from "../../services/operationBatchService";
import { computeModelHash } from "../../services/hashService";
import { loadPreviewModel } from "../../services/previewService";
import type { UnitCatalog, UnitCatalogEntry } from "../../services/unitCatalogService";
import type { PreviewModel } from "../../types";
import { modelDocumentDiagnostics } from "../model-data/documentDiagnostics";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import {
  buildComponentApply, buildMaterialApply, buildMaterialInsert, buildSectionApply, buildSectionInsert,
  combineDispositions, defaultMaterialMode, hangerLibraryProvenance, libraryRecordVersion, type LibraryApplyPlan
} from "./libraryApply";

type Json = Record<string, any>;

const unit = (unit_id: string, symbol: string, dimension_id: string): UnitCatalogEntry => ({
  unit_id, symbol, dimension_id, canonical: false, transform_kind: "multiplicative", factor_representation: "invented test entry",
  offset_representation: null, provenance: "exact_public_definition", review_status: "accepted"
});
const catalog: UnitCatalog = {
  schema_version: "0.1.0", catalog_id: "unit-system:test", decision_basis: "DEC-018", calculation_basis: "si_canonical",
  storage_convention: "entered_units_preserved", entry_count: 5,
  entries: [unit("unit:pascal", "Pa", "pressure"), unit("unit:gigapascal", "GPa", "pressure"), unit("unit:one", "1", "dimensionless"),
    unit("unit:per_degree_celsius", "1/degC", "thermal_expansion_coefficient"), unit("unit:kilogram_per_cubic_meter", "kg/m^3", "density")],
  boundary: { source: "test", protected_content_included: false, private_project_data_included: false, professional_approval_claimed: false, code_compliance_claimed: false }
};

const PRIVATE = { source_name: "Invented private test source", source_location: "invented-local:test", source_license: "private",
  contributor: "Test author", contributor_certification: "Invented values, no engineering authority", redistribution_status: "private_only", review_status: "pending" };
const PUBLIC = { ...PRIVATE, source_name: "Invented public test source", redistribution_status: "public_permissive", review_status: "accepted" };

function property(kind: string, magnitude: number | string, unitId: string, extra: Json = {}): Json {
  return { property_id: `p.${kind}${extra.suffix ?? ""}`, property_kind: kind, value_status: "private_user_supplied", required_for: "mechanics_solve",
    provenance: extra.provenance ?? PRIVATE, review_status: extra.review ?? "pending",
    ...(extra.temperature ? { temperature_ref: { ref_type: "material_temperature_point", ref_id: "t.hot" } } : {}),
    value: { magnitude, unit_ref: { ref_type: "UnitDefinition", ref_id: unitId }, dimension_id: "stress", quantity_kind: "unit_bearing",
      unit_required: true, missing_unit_behavior: "diagnostic_blocking", provenance: extra.valueProvenance ?? extra.provenance ?? PRIVATE } };
}

function materialLibrary(properties: Json[], record: Json = {}, library: Json = {}): Json {
  return {
    schema_version: "0.1.0",
    material_library: { library_id: "matlib.invented.private", name: "Invented private material library name", library_scope: "private_user_library",
      privacy_class: "private_user_data", provenance: PRIVATE, review_status: "pending", ...library },
    material_records: [{ material_id: "mat.invented.steel", name: "Invented steel", material_family: "invented_example",
      privacy_class: "private_user_data", redistribution_status: "private_only", properties,
      allowables: [{ allowable_id: "mat.invented.allowable.secret", allowable_kind: "user_rule_allowable", value_status: "not_provided",
        public_repository_value_policy: "no_public_code_specific_values", required_for: "rule_check", provenance: PRIVATE, review_status: "pending" }],
      completeness: [], provenance: PRIVATE, review_status: "pending", ...record }]
  };
}

const E = () => property("elastic_modulus", 2.0e11, "unit:pascal");
const NU = () => property("poisson_ratio", 0.3, "unit:one");
const STORED_G = 70123456789.12345;
const G = () => property("shear_modulus", STORED_G, "unit:pascal");

/** Independent RFC 8785 for the plain ASCII records used here: sorted keys,
 * ECMAScript number rendering (JSON.stringify), no whitespace. */
function jcs(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(jcs).join(",")}]`;
  if (value && typeof value === "object") {
    return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${jcs((value as Json)[key])}`).join(",")}}`;
  }
  return JSON.stringify(value);
}
const independentVersion = (record: unknown) => `sha256:${createHash("sha256").update(jcs(record), "utf8").digest("hex")}`;

async function demo(): Promise<PreviewModel> {
  return structuredClone(await loadPreviewModel());
}

async function apply(model: PreviewModel, plan: LibraryApplyPlan): Promise<PreviewModel> {
  const outcome = await applyOperationBatch(model, plan.batch, await computeModelHash(model));
  expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
  const next = outcome.applied_model!;
  expect(modelDocumentDiagnostics(next).filter((finding) => finding.severity === "blocking")).toEqual([]);
  return next;
}

const bits = (value: number) => new BigUint64Array(new Float64Array([value]).buffer)[0];

describe("material insert: the E/ν rule", () => {
  it("derives G = E/(2(1+ν)) when the record stores none, keeps ν, and names the basis", async () => {
    const model = await demo();
    const document = materialLibrary([E(), NU(), property("density", 7800, "unit:kilogram_per_cubic_meter")]);
    const plan = await buildMaterialInsert({ model, document, recordId: "mat.invented.steel", newId: "material:library-steel", mode: "e_nu", catalog });
    const next = await apply(model, plan);
    const material = next.materials!.find((item) => item.id === "material:library-steel")!;
    // Independent arithmetic: 2e11 / 2.6.
    expect(Math.abs(material.shear_modulus!.value - 76923076923.07692) / 76923076923.07692).toBeLessThan(1e-15);
    expect(material.shear_modulus!.unit).toBe("Pa");
    expect(material.poisson_ratio).toEqual({ value: 0.3, unit: "1" });
    expect(material.constitutive_basis).toBe("homogeneous_isotropic_E_nu_v1");
    expect(material.provenance).toBe("library:material:matlib.invented.private#mat.invented.steel");
    expect(material.provenance_record!.verification_note).toContain("derived from E and ν (isotropic)");
    expect(material.provenance_record!.verification_note).toContain("p.elastic_modulus (200000000000 Pa)");
    expect(material.provenance_record!.verification_note).toContain("p.poisson_ratio (0.3 1)");
    expect(plan.copied.find((row) => row.member === "shear_modulus")!.source).toBe("derived from E and ν (isotropic)");
    expect(plan.notCopied.map((row) => row.source)).toEqual(expect.arrayContaining(["p.density", "mat.invented.allowable.secret"]));
    // library_version is the record's RFC 8785 SHA-256, computed independently.
    const record = document.material_records[0];
    expect(material.provenance_record!.library_ref).toEqual({ library_kind: "material", library_id: "matlib.invented.private",
      library_version: independentVersion(record), record_id: "mat.invented.steel" });
    expect(await libraryRecordVersion(record)).toBe(independentVersion(record));
    // The private-data boundary: no allowable id, non-copied slot or library name.
    const saved = JSON.stringify(next);
    for (const absent of ["mat.invented.allowable.secret", "p.density", "Invented private material library name"]) {
      expect(saved.includes(absent), absent).toBe(false);
    }
  });

  it("stores a derived G in Pa, bit for bit the applier's exact-route derivation, whatever E's unit", async () => {
    const model = await demo();
    const engine = await loadWasmEngine();
    for (const [magnitude, nu] of [[210, 0.3], [193.0000000001, 0.29], [0.1 + 0.2, 1 / 3], [207, 0.49999999999999994], [69, -0.9999999999999999]] as const) {
      const document = materialLibrary([property("elastic_modulus", magnitude, "unit:gigapascal"), property("poisson_ratio", nu, "unit:one")]);
      const plan = await buildMaterialInsert({ model, document, recordId: "mat.invented.steel", newId: "material:gpa", mode: "e_nu", catalog });
      const material = JSON.parse(plan.batch.operations[0].change.after);
      expect(material.elastic_modulus).toEqual({ value: magnitude, unit: "GPa" });
      expect(material.shear_modulus.unit).toBe("Pa");
      const expected = JSON.parse(engine.derivedShearModulusJson(magnitude, "GPa", nu)).value as number;
      expect(bits(material.shear_modulus.value)).toBe(bits(expected));
      // The same order in plain binary64: E·1e9 (the catalog's GPa factor), then E / (2(1+ν)).
      expect(bits(material.shear_modulus.value)).toBe(bits((magnitude * 1e9) / (2 * (1 + nu))));
    }
  });

  it("refuses rather than derives when the record stores G or ν it cannot copy, naming the slot (HELP_HUMAN rule, literally)", async () => {
    const model = await demo();
    const insert = (properties: Json[], mode: "e_nu" | "e_g") =>
      buildMaterialInsert({ model, document: materialLibrary(properties), recordId: "mat.invented.steel", newId: "material:x", mode, catalog });
    const temperatureG = property("shear_modulus", 1.0e10, "unit:pascal", { temperature: true, suffix: ".hot" });
    const rejectedG = property("shear_modulus", 8e10, "unit:pascal", { valueProvenance: { ...PRIVATE, redistribution_status: "rejected" }, suffix: ".r" });
    const ineligibleG = { ...property("shear_modulus", 8e10, "unit:pascal", { suffix: ".tbd" }), value_status: "TBD" };
    for (const [slot, id] of [[temperatureG, "p.shear_modulus.hot"], [rejectedG, "p.shear_modulus.r"], [ineligibleG, "p.shear_modulus.tbd"]] as const) {
      await expect(insert([E(), NU(), slot], "e_nu")).rejects.toThrow(new RegExp(`LIBRARY-APPLY-MEMBER-BLOCKED.*${id.replaceAll(".", "\\.")}.*never derived`));
      await expect(insert([E(), slot], "e_g")).rejects.toThrow(new RegExp(`LIBRARY-APPLY-MEMBER-BLOCKED.*${id.replaceAll(".", "\\.")}`));
    }
    // A copyable G alongside a temperature-referenced one: the stored G is copied, not derived.
    const plan = await insert([E(), NU(), G(), temperatureG], "e_nu");
    expect(JSON.parse(plan.batch.operations[0].change.after).shear_modulus.value).toBe(STORED_G);
    // A stored ν that cannot be copied refuses both modes; it is never discarded.
    const quarantinedNu = property("poisson_ratio", 0.3, "unit:one", { review: "quarantined", suffix: ".q" });
    await expect(insert([E(), quarantinedNu], "e_nu")).rejects.toThrow(/LIBRARY-APPLY-MEMBER-BLOCKED.*p\.poisson_ratio\.q/);
    await expect(insert([E(), G(), quarantinedNu], "e_g")).rejects.toThrow(/LIBRARY-APPLY-NU-NOT-DISCARDED.*p\.poisson_ratio\.q/);
  });

  it("never overwrites a stored G (bitwise) and never discards ν", async () => {
    const model = await demo();
    const document = materialLibrary([E(), NU(), G()]);
    const plan = await buildMaterialInsert({ model, document, recordId: "mat.invented.steel", newId: "material:library-steel", mode: "e_nu", catalog });
    const material = (await apply(model, plan)).materials!.find((item) => item.id === "material:library-steel")!;
    expect(bits(material.shear_modulus!.value)).toBe(bits(STORED_G));
    expect(material.poisson_ratio!.value).toBe(0.3);
    expect(material.provenance_record!.verification_note).toContain("stored in the record (p.shear_modulus), copied as stored; not derived");
    await expect(buildMaterialInsert({ model, document, recordId: "mat.invented.steel", newId: "material:x", mode: "e_g", catalog }))
      .rejects.toThrow(/LIBRARY-APPLY-NU-NOT-DISCARDED.*poisson_ratio/);
  });

  it("refuses a mode whose member the record lacks, naming it, and unresolved units or magnitudes", async () => {
    const model = await demo();
    const insert = (properties: Json[], mode: "e_nu" | "e_g") =>
      buildMaterialInsert({ model, document: materialLibrary(properties), recordId: "mat.invented.steel", newId: "material:x", mode, catalog });
    await expect(insert([E(), G()], "e_nu")).rejects.toThrow(/LIBRARY-APPLY-MEMBER-MISSING.*poisson_ratio/);
    await expect(insert([E()], "e_g")).rejects.toThrow(/LIBRARY-APPLY-MEMBER-MISSING.*shear_modulus/);
    await expect(insert([NU(), G()], "e_nu")).rejects.toThrow(/LIBRARY-APPLY-MEMBER-MISSING.*elastic_modulus/);
    await expect(insert([property("elastic_modulus", 123, "unit.MPa"), NU()], "e_nu")).rejects.toThrow(/LIBRARY-APPLY-UNIT-UNRESOLVED.*unit\.MPa/);
    await expect(insert([property("elastic_modulus", "2e11x", "unit:pascal"), NU()], "e_nu")).rejects.toThrow(/LIBRARY-APPLY-MAGNITUDE-INVALID/);
    await expect(insert([E(), { ...E(), property_id: "p.e2" }, NU()], "e_nu")).rejects.toThrow(/LIBRARY-APPLY-AMBIGUOUS-SOURCE.*elastic_modulus/);
    // A string magnitude parses; E/G with a stored G and no ν writes no basis.
    const plan = await insert([property("elastic_modulus", "2.0e11", "unit:pascal"), G()], "e_g");
    const material = JSON.parse(plan.batch.operations[0].change.after);
    expect(material.elastic_modulus).toEqual({ value: 2e11, unit: "Pa" });
    expect(material.constitutive_basis).toBeUndefined(); expect(material.poisson_ratio).toBeUndefined();
    await apply(model, plan);
  });
});

describe("material apply: basis, retained values and dispositions", () => {
  it("keeps the basis by default, keeps a private retained alpha private, and drops it only with replace all", async () => {
    const model = await demo();
    const target = model.materials![0];
    expect(defaultMaterialMode(target)).toBe("e_g");
    target.provenance_record = { source_name: "Invented private alpha source", redistribution_status: "private_only", review_status: "pending" };
    const publicDoc = materialLibrary([property("elastic_modulus", 2.1e11, "unit:pascal", { provenance: PUBLIC, review: "accepted" }),
      property("shear_modulus", 8.0e10, "unit:pascal", { provenance: PUBLIC, review: "accepted" })],
    { privacy_class: "public_permissive", redistribution_status: "public_permissive", provenance: PUBLIC, review_status: "accepted" },
    { privacy_class: "public_permissive", provenance: PUBLIC, review_status: "accepted" });
    const kept = await buildMaterialApply({ model, document: publicDoc, recordId: "mat.invented.steel", targetId: target.id, mode: "e_g", replace: "none", catalog });
    expect(kept.provenanceRecord.redistribution_status).toBe("private_only");
    expect(kept.provenanceRecord.verification_note).toContain("thermal_expansion_coefficient");
    const next = await apply(model, kept);
    expect(next.materials![0].thermal_expansion_coefficient).toEqual(target.thermal_expansion_coefficient);
    expect(next.materials![0].constitutive_basis).toBeUndefined();
    const all = await buildMaterialApply({ model, document: publicDoc, recordId: "mat.invented.steel", targetId: target.id, mode: "e_g", replace: "all", catalog });
    expect(all.removed).toEqual(["thermal_expansion_coefficient"]);
    expect(all.provenanceRecord).toMatchObject({ redistribution_status: "public_permissive", review_status: "accepted" });
    expect((await apply(model, all)).materials![0].thermal_expansion_coefficient).toBeUndefined();
  });

  it("refuses retained values under a quarantined previous record unless replace all, and keeps an exact basis", async () => {
    const model = await demo();
    const target = model.materials![0];
    target.provenance_record = { source_name: "Invented quarantined source", review_status: "quarantined" };
    const document = materialLibrary([E(), NU()]);
    await expect(buildMaterialApply({ model, document, recordId: "mat.invented.steel", targetId: target.id, mode: "e_nu", replace: "none", catalog }))
      .rejects.toThrow(/LIBRARY-APPLY-RETAINED-QUARANTINED.*thermal_expansion_coefficient/);
    const plan = await buildMaterialApply({ model, document, recordId: "mat.invented.steel", targetId: target.id, mode: "e_nu", replace: "all", catalog });
    expect(plan.removed).toEqual(["thermal_expansion_coefficient"]);
    const next = await apply(model, plan);
    expect(next.materials![0].constitutive_basis).toBe("homogeneous_isotropic_E_nu_v1");
    expect(defaultMaterialMode(next.materials![0])).toBe("e_nu");
    // E/G onto a material that holds ν would discard it: refused.
    next.materials![0].provenance_record = { source_name: "Invented", review_status: "pending" };
    await expect(buildMaterialApply({ model: next, document: materialLibrary([E(), G()]), recordId: "mat.invented.steel", targetId: target.id, mode: "e_g", replace: "all", catalog }))
      .rejects.toThrow(/LIBRARY-APPLY-NU-NOT-DISCARDED.*already has poisson_ratio/);
    // The diff names each removal and the derived G replacing the stored one.
    const rows = JSON.parse(plan.batch.operations[0].change.after);
    expect(rows.thermal_expansion_coefficient).toBeNull();
  });

  it("combines dispositions conservatively in both directions and normalizes TBD", () => {
    const pub = { provenance: PUBLIC, review_status: "accepted" };
    const priv = { provenance: PRIVATE, review_status: "pending" };
    expect(combineDispositions([pub, priv])).toEqual({ redistribution_status: "private_only", review_status: "pending" });
    expect(combineDispositions([priv, pub])).toEqual({ redistribution_status: "private_only", review_status: "pending" });
    expect(combineDispositions([{ privacy_class: "private_user_supplied", ...pub }])).toEqual({ redistribution_status: "private_only", review_status: "accepted" });
    expect(combineDispositions([{ redistribution_status: "TBD", review_status: "TBD" }])).toEqual({ redistribution_status: "unknown", review_status: "pending" });
    for (const privacy_class of ["project_private", "private_user_data", "private_user_supplied", "quarantine_metadata_only"]) {
      expect(combineDispositions([{ privacy_class, ...pub }]).redistribution_status, privacy_class).toBe("private_only");
    }
    expect(combineDispositions([pub, {}])).toEqual({ redistribution_status: "unknown", review_status: "pending" });
  });

  it("excludes a quarantined or value-level rejected slot (partial apply) and refuses a rejected record", async () => {
    const model = await demo();
    const quarantined = property("thermal_expansion_coefficient", 1.3e-5, "unit:per_degree_celsius", { review: "quarantined" });
    const rejectedAlpha = property("thermal_expansion_coefficient", 1.2e-5, "unit:per_degree_celsius", { valueProvenance: { ...PRIVATE, redistribution_status: "rejected" }, suffix: ".r" });
    const plan = await buildMaterialInsert({ model, document: materialLibrary([E(), NU(), quarantined]), recordId: "mat.invented.steel", newId: "material:partial", mode: "e_nu", catalog });
    expect(plan.notCopied).toEqual(expect.arrayContaining([{ source: "p.thermal_expansion_coefficient", reason: "excluded: quarantined" }]));
    const rejectedPlan = await buildMaterialInsert({ model, document: materialLibrary([E(), NU(), rejectedAlpha]), recordId: "mat.invented.steel", newId: "material:partial", mode: "e_nu", catalog });
    expect(rejectedPlan.notCopied).toEqual(expect.arrayContaining([{ source: "p.thermal_expansion_coefficient.r", reason: "excluded: rejected" }]));
    const material = JSON.parse(plan.batch.operations[0].change.after);
    expect(material.thermal_expansion_coefficient).toBeUndefined();
    expect(material.provenance_record.verification_note).toContain("derived from E and ν");
    await expect(buildMaterialInsert({ model, document: materialLibrary([E(), NU()], { review_status: "rejected" }), recordId: "mat.invented.steel", newId: "material:x", mode: "e_nu", catalog }))
      .rejects.toThrow(/LIBRARY-APPLY-SOURCE-REJECTED/);
  });
});

describe("hanger provenance", () => {
  it("keeps a project_private hanger source private and refuses a blocked copied quantity", async () => {
    const library = { library_id: "invented.hangers.project", name: "Invented project hangers", privacy_class: "project_private", provenance: PUBLIC };
    const record = { hanger_id: "invented.hanger", name: "Invented", privacy_class: "project_private", provenance: PUBLIC };
    const quantity = { magnitude: 17, unit: "N", dimension: "force", provenance: PUBLIC };
    const { provenanceRecord } = await hangerLibraryProvenance(library, record, [quantity]);
    expect(provenanceRecord.redistribution_status).toBe("private_only");
    await expect(hangerLibraryProvenance(library, { ...record, privacy_class: "public_permissive_reviewed" }, [{ ...quantity, provenance: { ...PUBLIC, review_status: "quarantined" } }]))
      .rejects.toThrow(/LIBRARY-APPLY-SOURCE-REJECTED.*quarantined/);
  });
});

// ---- sections ---------------------------------------------------------------

function dimension(kind: string, magnitude: number, extra: Json = {}): Json {
  return { dimension_id: `d.${kind}`, dimension_kind: kind, value_status: extra.status ?? "private_user_supplied", required_for: "section_property_calculation",
    provenance: PRIVATE, review_status: "pending",
    value: { magnitude, unit: "m", dimension: "length", value_status: extra.status ?? "private_user_supplied", provenance: PRIVATE } };
}
function sectionLibrary(dimensions: Json[]): Json {
  return {
    schema_version: "0.1.0",
    section_library: { library_id: "seclib.invented.private", name: "Invented private section library name", library_scope: "private_user_library",
      privacy_class: "private_user_supplied", provenance: PRIVATE, review_status: "pending" },
    section_records: [{ section_id: "sec.invented.pipe", name: "Invented library pipe", section_type: "pipe", privacy_class: "private_user_supplied",
      redistribution_status: "private_only", dimensions,
      properties: [{ property_id: "sp.area", property_kind: "cross_section_area", value_status: "calculated", calculation_status: "calculated", required_for: "mechanics_solve", provenance: PRIVATE, review_status: "pending" }],
      completeness: [], provenance: PRIVATE, review_status: "pending" }]
  };
}

/** The demo with one shared section bound to two pipes. */
async function boundModel(): Promise<PreviewModel> {
  const model = await demo();
  const geometry = { outside_diameter: { value: 0.168, unit: "m" }, wall_thickness: { value: 0.007, unit: "m" } };
  model.sections = [{ id: "section:shared", name: "Invented shared", section_type: "pipe", properties: structuredClone(geometry), provenance: "invented old section source" }];
  for (const pipe of model.pipe_segments.slice(0, 2)) { pipe.section_ref = "section:shared"; Object.assign(pipe.section, structuredClone(geometry)); }
  return model;
}

describe("section insert and apply", () => {
  it("inserts a pipe section and applies to a shared section, listing and updating every bound pipe", async () => {
    const model = await boundModel();
    const document = sectionLibrary([dimension("outside_diameter", 0.2), dimension("wall_thickness", 0.009), dimension("corrosion_allowance", 0.001)]);
    const insert = await buildSectionInsert({ model, document, recordId: "sec.invented.pipe", newId: "section:library" });
    expect((await apply(model, insert)).sections!.find((section) => section.id === "section:library")!.properties).toEqual({
      outside_diameter: { value: 0.2, unit: "m" }, wall_thickness: { value: 0.009, unit: "m" } });
    expect(insert.notCopied.map((row) => row.source)).toEqual(["d.corrosion_allowance", "sp.area"]);
    const plan = await buildSectionApply({ model, document, recordId: "sec.invented.pipe", targetId: "section:shared", replace: "none" });
    expect(plan.dependentPipes).toEqual(model.pipe_segments.slice(0, 2).map((pipe) => pipe.id).sort());
    const outcome = await applyOperationBatch(model, plan.batch, await computeModelHash(model));
    const rows = outcome.operation_outcomes[0].diff_preview.filter((row) => row.object_type === "Element").map((row) => row.entity_ref);
    expect(rows.sort()).toEqual(plan.dependentPipes);
    for (const pipe of outcome.applied_model!.pipe_segments.slice(0, 2)) {
      expect(pipe.section.outside_diameter).toEqual({ value: 0.2, unit: "m" });
      expect(pipe.section.wall_thickness).toEqual({ value: 0.009, unit: "m" });
    }
  });

  it("refuses a section apply that copies nothing", async () => {
    const model = await boundModel();
    await expect(buildSectionApply({ model, document: sectionLibrary([dimension("outside_diameter", 0.2, { status: "missing" })]), recordId: "sec.invented.pipe", targetId: "section:shared", replace: "none" }))
      .rejects.toThrow(/LIBRARY-APPLY-NOTHING-COPIED/);
  });

  it("refuses the whole change when a bound pipe's supplement fails, leaving the model unchanged", async () => {
    const model = await boundModel();
    model.pipe_segments[1].section.mill_tolerance = { value: 0.006, unit: "m" };
    const plan = await buildSectionApply({ model, document: sectionLibrary([dimension("outside_diameter", 0.2), dimension("wall_thickness", 0.005)]),
      recordId: "sec.invented.pipe", targetId: "section:shared", replace: "none" });
    const outcome = await applyOperationBatch(model, plan.batch, await computeModelHash(model));
    expect(outcome.validation.application_status).toBe("blocked");
    expect(outcome.applied_model).toBeNull();
    expect(outcome.operation_outcomes[0].diagnostics.map((d) => d.code)).toContain("OP-SECTION-BINDING-INVALID");
  });
});

// ---- components --------------------------------------------------------------

function slot(kind: string, magnitude: number | null, unitSymbol: string, extra: Json = {}): Json {
  return { field_id: `f.${kind}${extra.suffix ?? ""}`, field_kind: kind, value_status: extra.status ?? "private_user_supplied",
    public_repository_value_policy: "private_user_supplied_only", required_for: "mechanics_solve", provenance: PRIVATE, review_status: "pending",
    ...(magnitude === null ? {} : { value: { magnitude, unit: unitSymbol, dimension: "length", value_status: extra.status ?? "private_user_supplied", provenance: PRIVATE } }) };
}
function componentLibrary(type: string, fields: Json[]): Json {
  return {
    schema_version: "0.1.0",
    component_library: { library_id: "complib.invented.private", name: "Invented private component library name", library_scope: "private_user_library",
      privacy_class: "private_user_supplied", provenance: PRIVATE, review_status: "pending" },
    component_records: [{ component_id: "comp.invented", name: "Invented component", component_type: type, privacy_class: "private_user_supplied",
      redistribution_status: "private_only", fields, completeness: [], provenance: PRIVATE, review_status: "pending" }]
  };
}
const LIB = "library:component:complib.invented.private#comp.invented";

describe("component apply", () => {
  it("maps a rigid record per table, keeps center_of_gravity under its previous attribution and excludes the scalar", async () => {
    const model = await demo();
    const document = componentLibrary("valve", [slot("weight", 500, "N"), slot("end_a_size", 0.17, "m"), slot("end_b_size", 0.17, "m"),
      slot("center_of_gravity", 0.1, "m"), slot("hardware_reference", null, ""), slot("outside_diameter", 0.17, "m")]);
    const plan = await buildComponentApply({ model, document, recordId: "comp.invented", targetId: "component:C-130", replace: "none" });
    const after = JSON.parse(plan.batch.operations[0].change.after);
    expect(after["geometry.rigid_component_source_reference"]).toBe(`invented_user_entered_rigid_component_preview_geometry; ${LIB}: end_a_size, end_b_size, weight`);
    expect(plan.notCopied.map((row) => row.source)).toEqual(["f.center_of_gravity", "f.hardware_reference", "f.outside_diameter"]);
    const valve = (await apply(model, plan)).components.find((component) => component.id === "component:C-130")!;
    expect(valve.geometry!.weight).toEqual({ value: 500, unit: "N" });
    expect((valve.geometry as Json).center_of_gravity).toEqual({ x: 0.12, y: 0.02, z: 0.0, unit: "m" });
    // A rigid element with no previous reference cannot credit retained values.
    const unsourced = await demo();
    delete (unsourced.components.find((component) => component.id === "component:C-130")!.geometry as Json).rigid_component_source_reference;
    await expect(buildComponentApply({ model: unsourced, document, recordId: "comp.invented", targetId: "component:C-130", replace: "none" }))
      .rejects.toThrow(/LIBRARY-APPLY-RETAINED-UNSOURCED.*center_of_gravity/);
  });

  it("protects required members under replace group and otherwise combines the modifiers attribution", async () => {
    const model = await demo();
    const document = componentLibrary("bend", [slot("sif_user_value", 1.3, "1")]);
    await expect(buildComponentApply({ model, document, recordId: "comp.invented", targetId: "component:C-110", replace: "group" }))
      .rejects.toThrow(/LIBRARY-APPLY-REQUIRED-MEMBER.*flexibility_factor_user_value/);
    const plan = await buildComponentApply({ model, document, recordId: "comp.invented", targetId: "component:C-110", replace: "none" });
    const after = JSON.parse(plan.batch.operations[0].change.after);
    expect(after["modifiers.source_reference"]).toBe(`invented_user_entered_preview_no_code_table; ${LIB}: sif_user_value`);
    const bend = (await apply(model, plan)).components.find((component) => component.id === "component:C-110")!;
    expect(bend.modifiers!.flexibility_factor_user_value).toEqual({ value: 1.08, unit: "none" });
    // A record that supplies the whole modifiers group replaces the reference outright.
    const whole = await buildComponentApply({ model, recordId: "comp.invented", targetId: "component:C-110", replace: "none",
      document: componentLibrary("bend", [slot("sif_user_value", 1.3, "1"), slot("flexibility_factor_user_value", 1.1, "1")]) });
    expect(JSON.parse(whole.batch.operations[0].change.after)["modifiers.source_reference"]).toBe(LIB);
  });

  it("copies the directional SIF pair only onto a realized curved bend, and refuses mismatch and ambiguity", async () => {
    const model = await demo();
    const pair = componentLibrary("bend", [slot("sif_in_plane_user_value", 1.2, "1"), slot("sif_out_of_plane_user_value", 1.1, "1"), slot("bend_radius", 0.5, "m")]);
    const geometryOnly = await buildComponentApply({ model, document: pair, recordId: "comp.invented", targetId: "component:C-110", replace: "none" });
    expect(geometryOnly.notCopied.map((row) => row.source)).toEqual(["f.sif_in_plane_user_value", "f.sif_out_of_plane_user_value"]);
    const realizedModel = await demo();
    realizedModel.components.find((component) => component.id === "component:C-110")!.mechanics_interface!.solver_consumption = "curved_bend_macro_element";
    const realized = await buildComponentApply({ model: realizedModel, document: pair, recordId: "comp.invented", targetId: "component:C-110", replace: "none" });
    const after = JSON.parse(realized.batch.operations[0].change.after);
    expect(after["modifiers.sif_in_plane_user_value"]).toEqual({ value: 1.2, unit: "1" });
    expect(after["modifiers.sif_out_of_plane_user_value"]).toEqual({ value: 1.1, unit: "1" });
    await expect(buildComponentApply({ model, document: componentLibrary("valve", [slot("weight", 1, "N")]), recordId: "comp.invented", targetId: "component:C-110", replace: "none" }))
      .rejects.toThrow(/LIBRARY-APPLY-KIND-MISMATCH/);
    await expect(buildComponentApply({ model, document: componentLibrary("bend", [slot("bend_radius", 0.5, "m"), slot("bend_radius", 0.6, "m", { suffix: ".b" })]), recordId: "comp.invented", targetId: "component:C-110", replace: "none" }))
      .rejects.toThrow(/LIBRARY-APPLY-AMBIGUOUS-SOURCE.*bend_radius/);
    // A branch record matches tee, branch and branch_connection kinds.
    const branch = await buildComponentApply({ model, document: componentLibrary("branch", [slot("flexibility_factor_user_value", 1.2, "1")]), recordId: "comp.invented", targetId: "component:C-120", replace: "none" });
    await apply(model, branch);
  });
});

describe("batch, undo/redo and re-apply", () => {
  it("applies once, undoes to the exact previous model, redoes, and re-apply records the new digest", async () => {
    const { result } = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(result.current.model.model).not.toBeNull());
    const basis = structuredClone(result.current.model.model!);
    const beforeHash = (await computeModelHash(basis))!.value;
    const document = materialLibrary([E(), NU()]);
    const plan = await buildMaterialApply({ model: basis, document, recordId: "mat.invented.steel", targetId: basis.materials![0].id, mode: "e_nu", replace: "none", catalog });
    await act(async () => { await result.current.operations.handleQueueOperationBatch(plan.batch); });
    const queued = result.current.operations.queuedBatches[0];
    await act(async () => { await result.current.operations.handleRunOperationBatch(queued, false); });
    expect(result.current.model.model).toEqual(basis);
    await act(async () => { await result.current.operations.handleRunOperationBatch(queued, true); });
    const applied = structuredClone(result.current.model.model!);
    expect(applied.materials![0].provenance_record!.library_ref!.library_version).toBe(independentVersion(document.material_records[0]));
    expect(result.current.operations.undoStack).toHaveLength(1);
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect((await computeModelHash(result.current.model.model!))!.value).toBe(beforeHash);
    act(() => result.current.operations.handleRedoSessionModelEdit());
    expect(result.current.model.model).toEqual(applied);
    // The stored record changes: re-apply records the new digest.
    const revised = materialLibrary([property("elastic_modulus", 2.05e11, "unit:pascal"), NU()]);
    const reapply = await buildMaterialApply({ model: applied, document: revised, recordId: "mat.invented.steel", targetId: applied.materials![0].id, mode: "e_nu", replace: "none", catalog });
    expect(reapply.provenanceRecord.library_ref!.library_version).toBe(independentVersion(revised.material_records[0]));
    expect(reapply.provenanceRecord.library_ref!.library_version).not.toBe(applied.materials![0].provenance_record!.library_ref!.library_version);
    await act(async () => { await result.current.operations.handleQueueOperationBatch(reapply.batch); });
    await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches.at(-1)!, true); });
    const reapplied = result.current.model.model!.materials![0].provenance_record!;
    expect(reapplied.library_ref!.library_version).toBe(independentVersion(revised.material_records[0]));
    expect(reapplied.verification_note).toContain(`previous library reference library:material:matlib.invented.private#mat.invented.steel (${independentVersion(document.material_records[0])})`);
  });
});

// HELP_HUMAN, 2026-10-23: "Each refusal must tell the user how to proceed".
// Every desktop-side library-apply refusal names its cause and the way
// forward; the texts are pinned in libraryApply.messages.json.
describe("refusal messages name the cause and how to proceed", () => {
  async function scenarios(): Promise<Array<[string, () => Promise<unknown>]>> {
    const model = await demo();
    const withNu = structuredClone(model);
    withNu.materials![0] = { ...withNu.materials![0], constitutive_basis: "homogeneous_isotropic_E_nu_v1", poisson_ratio: { value: 0.3, unit: "1" } };
    const bound = await boundModel();
    const insert = (properties: Json[], mode: "e_nu" | "e_g", record: Json = {}, library: Json = {}) =>
      () => buildMaterialInsert({ model, document: materialLibrary(properties, record, library), recordId: "mat.invented.steel", newId: "material:x", mode, catalog });
    const temperatureG = property("shear_modulus", 1.0e10, "unit:pascal", { temperature: true, suffix: ".hot" });
    const quarantinedNu = property("poisson_ratio", 0.3, "unit:one", { review: "quarantined", suffix: ".q" });
    const quarantinedSection = structuredClone(bound);
    quarantinedSection.sections![0].provenance_record = { source_name: "Invented", review_status: "quarantined" };
    const quarantinedTarget = structuredClone(model);
    quarantinedTarget.materials![0].provenance_record = { source_name: "Invented", review_status: "quarantined" };
    const unsourced = await demo();
    delete (unsourced.components.find((component) => component.id === "component:C-130")!.geometry as Json).rigid_component_source_reference;
    return [
      ["record missing", () => buildMaterialInsert({ model, document: materialLibrary([E()]), recordId: "mat.absent", newId: "material:x", mode: "e_g", catalog })],
      ["source rejected", insert([E(), NU()], "e_nu", { review_status: "rejected" })],
      ["magnitude invalid", insert([property("elastic_modulus", "2e11x", "unit:pascal"), NU()], "e_nu")],
      ["unit unresolved", insert([property("elastic_modulus", 123, "unit.MPa"), NU()], "e_nu")],
      ["ambiguous source", insert([E(), { ...E(), property_id: "p.e2" }, NU()], "e_nu")],
      ["E missing", insert([NU(), G()], "e_nu")],
      ["ν blocked (E/ν)", insert([E(), quarantinedNu], "e_nu")],
      ["ν missing (E/ν)", insert([E(), G()], "e_nu")],
      ["ν unit", insert([E(), property("poisson_ratio", 0.3, "unit:pascal")], "e_nu")],
      ["G blocked (E/ν)", insert([E(), NU(), temperatureG], "e_nu")],
      ["ν provided (E/G)", insert([E(), NU(), G()], "e_g")],
      ["ν blocked (E/G)", insert([E(), G(), quarantinedNu], "e_g")],
      ["G blocked (E/G)", insert([E(), temperatureG], "e_g")],
      ["G missing (E/G)", insert([E()], "e_g")],
      ["target ν (E/G apply)", () => buildMaterialApply({ model: withNu, document: materialLibrary([E(), G()]), recordId: "mat.invented.steel", targetId: withNu.materials![0].id, mode: "e_g", replace: "none", catalog })],
      ["retained quarantined", () => buildMaterialApply({ model: quarantinedTarget, document: materialLibrary([E(), NU()]), recordId: "mat.invented.steel", targetId: model.materials![0].id, mode: "e_nu", replace: "none", catalog })],
      ["material target missing", () => buildMaterialApply({ model, document: materialLibrary([E(), G()]), recordId: "mat.invented.steel", targetId: "material:absent", mode: "e_g", replace: "none", catalog })],
      ["id invalid", () => buildMaterialInsert({ model, document: materialLibrary([E(), G()]), recordId: "mat.invented.steel", newId: "9bad", mode: "e_g", catalog })],
      ["id taken", () => buildMaterialInsert({ model, document: materialLibrary([E(), G()]), recordId: "mat.invented.steel", newId: model.materials![0].id, mode: "e_g", catalog })],
      ["section type", () => buildSectionInsert({ model, document: { ...sectionLibrary([]), section_records: [{ ...sectionLibrary([]).section_records[0], section_type: "rigid" }] }, recordId: "sec.invented.pipe", newId: "section:x" })],
      ["section member missing", () => buildSectionInsert({ model, document: sectionLibrary([dimension("outside_diameter", 0.2)]), recordId: "sec.invented.pipe", newId: "section:x" })],
      ["section nothing copied", () => buildSectionApply({ model: bound, document: sectionLibrary([]), recordId: "sec.invented.pipe", targetId: "section:shared", replace: "none" })],
      ["section replace all", () => buildSectionApply({ model: bound, document: sectionLibrary([dimension("outside_diameter", 0.2)]), recordId: "sec.invented.pipe", targetId: "section:shared", replace: "all" })],
      ["section target missing", () => buildSectionApply({ model: bound, document: sectionLibrary([dimension("outside_diameter", 0.2)]), recordId: "sec.invented.pipe", targetId: "section:absent", replace: "none" })],
      ["component kind", () => buildComponentApply({ model, document: componentLibrary("valve", [slot("weight", 1, "N")]), recordId: "comp.invented", targetId: "component:C-110", replace: "none" })],
      ["component no values", () => buildComponentApply({ model, document: componentLibrary("terminal", [slot("weight", 1, "N")]), recordId: "comp.invented", targetId: "component:C-140", replace: "none" })],
      ["component nothing copied", () => buildComponentApply({ model, document: componentLibrary("bend", [slot("center_of_gravity", 0.1, "m")]), recordId: "comp.invented", targetId: "component:C-110", replace: "none" })],
      ["component required", () => buildComponentApply({ model, document: componentLibrary("bend", [slot("sif_user_value", 1.3, "1")]), recordId: "comp.invented", targetId: "component:C-110", replace: "group" })],
      ["component unsourced", () => buildComponentApply({ model: unsourced, document: componentLibrary("valve", [slot("weight", 500, "N")]), recordId: "comp.invented", targetId: "component:C-130", replace: "none" })],
      ["component target missing", () => buildComponentApply({ model, document: componentLibrary("bend", [slot("bend_radius", 0.5, "m")]), recordId: "comp.invented", targetId: "component:absent", replace: "none" })],
      ["hanger library rejected", () => hangerLibraryProvenance({ library_id: "h", name: "H", privacy_class: "private_user_supplied", provenance: { ...PRIVATE, review_status: "rejected" } },
        { hanger_id: "h1", name: "H1", provenance: PRIVATE }, [])],
      ["unit unresolved (ν)", insert([E(), property("poisson_ratio", 0.3, "unit.one")], "e_nu")],
      ["unit unresolved (α)", insert([E(), NU(), property("thermal_expansion_coefficient", 1.2e-5, "unit.per_C")], "e_nu")],
      ["retained quarantined (section)", () => buildSectionApply({ model: quarantinedSection, document: sectionLibrary([dimension("outside_diameter", 0.2)]), recordId: "sec.invented.pipe", targetId: "section:shared", replace: "none" })],
      ["hanger quantity", () => hangerLibraryProvenance({ library_id: "h", name: "H", privacy_class: "private_user_supplied", provenance: PRIVATE },
        { hanger_id: "h1", name: "H1", provenance: PRIVATE }, [{ magnitude: 1, unit: "N", dimension: "force", provenance: { ...PRIVATE, review_status: "quarantined" } }])]
    ];
  }

  it("pins every refusal text", async () => {
    const actual: Record<string, string> = {};
    for (const [label, run] of await scenarios()) {
      try { await run(); actual[label] = "NOT REFUSED"; } catch (error) { actual[label] = (error as Error).message; }
    }
    const pinned = (await import("./libraryApply.messages.json")).default as Record<string, string>;
    if (process.env.LIBRARY_APPLY_MESSAGES_UPDATE === "1") {
      const { writeFileSync } = await import("node:fs");
      writeFileSync("src/features/library/libraryApply.messages.json", `${JSON.stringify(actual, null, 2)}\n`);
      return;
    }
    expect(actual).toEqual(pinned);
    for (const [label, message] of Object.entries(actual)) {
      // A cause, then at least one sentence telling the user what to do.
      expect(message, label).toMatch(/^LIBRARY-APPLY-[A-Z-]+: .+\. (Choose|Fix|Correct|Use|Select|Enter|Refresh|Check|Apply|Keep|Have|Review)/);
    }
  });
});
