import type {UnitCatalog,UnitCatalogEntry} from "../../services/unitCatalogService";
import {expect,it} from "vitest";
import demo from "../../../../../fixtures/product_preview/invented_demo_model.json";
import {buildMaterialInsert} from "../library/libraryApply";
import {resolveLibraryDispositions,screenModelDocumentDispositions} from "./modelDocumentSave";
import {applyOperationBatch} from "../../services/operationBatchService";
import {computeModelHash} from "../../services/hashService";
import {modelDocumentShapeDiagnostics,modelDocumentSlotValidation} from "./modelDocumentShape";
type Json=Record<string,any>;
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

it("save screens the same library identity that a real library insert recorded",async()=>{
 const id="\ufeffinvented.library";
 const stored=materialLibrary([E(),NU()],{}, {library_id:id});
 const model=structuredClone(demo) as any;
 const plan=await buildMaterialInsert({model,document:stored,recordId:"mat.invented.steel",newId:"material:feff",mode:"e_nu",catalog});
 const outcome=await applyOperationBatch(model,plan.batch,await computeModelHash(model));
 expect(outcome.validation.application_status).toBe("applied_to_session_model");
 const next=outcome.applied_model!;
 const ref=next.materials!.find(m=>m.id==="material:feff")!.provenance_record!.library_ref!;
 expect(ref.library_id).toBe(id);
 expect(modelDocumentShapeDiagnostics(next)).toEqual([]);expect(modelDocumentSlotValidation(next).blocking).toEqual([]);
 const other=structuredClone(stored);other.material_library.library_id="invented.library";other.material_records[0].review_status="quarantined";
 const calls:any[]=[];
 const resolution=await resolveLibraryDispositions(next,async(project,kind,key)=>{
  calls.push([project,kind,key]);const document=key===id?stored:key==="invented.library"?other:null;
  return {route:"tauri_backend",envelope:document?{document}:null} as any;
 });
 const screened=screenModelDocumentDispositions(next,{explicitLocalPrivateIntent:true,resolvedLibraries:resolution.dispositions});
 expect(calls[0][2]).toBe(id);
 expect(screened.blocking).toEqual([]);
});
