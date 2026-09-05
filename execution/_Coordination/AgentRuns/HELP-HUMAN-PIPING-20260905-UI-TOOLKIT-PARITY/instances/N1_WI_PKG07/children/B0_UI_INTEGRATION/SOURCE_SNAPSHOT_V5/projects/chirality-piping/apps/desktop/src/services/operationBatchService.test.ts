import { afterEach, describe, expect, it, vi } from "vitest";
import type { PreviewModel, EditorOperationIntent, ModelHashEvidence } from "../types";
const mocks = vi.hoisted(() => ({invoke:vi.fn(),validate:vi.fn(),apply:vi.fn(),display:vi.fn(),plan:vi.fn()}));
vi.mock("@tauri-apps/api/core",()=>({invoke:mocks.invoke}));
vi.mock("./wasmEngine/loadWasmEngine",()=>({loadWasmEngine:async()=>({validateOperationBatchJson:mocks.validate,applyOperationBatchJson:mocks.apply,convertDisplayQuantitiesJson:mocks.display})}));
vi.mock("./wasmEngine/loadSelfWeightEngine",()=>({loadSelfWeightEngine:async()=>({generateSelfWeightPlanJson:mocks.plan})}));
import { applyOperationBatch, validateOperationBatch } from "./operationBatchService";
import { convertDisplayQuantities } from "./displayQuantityService";
import { generateSelfWeightPlan, type SelfWeightRequest } from "./selfWeightPlanService";
const model = {nodes:[]} as unknown as PreviewModel;
const batch = {batch_id:"offline",operations:[{author_type:"agent",rationale:"retained",source:{source_ref:"import:1",source_channel:"offline",source_role:"agent"}} as unknown as EditorOperationIntent]};
const claim = {value:"sha256:source"} as ModelHashEvidence;
const request:SelfWeightRequest = {case_id:"loadcase:mass",label:"Mass",pipe_refs:["pipe:1"],gravity:{value:9.81,unit:"m/s^2",axis:"-Y"},provenance:"asserted source",source_model_hash:"sha256:source"};
afterEach(()=>{vi.clearAllMocks();delete (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__;});
describe("shared capability transports",()=>{
 it("preserves metadata and exact native camelCase batch args",async()=>{
  (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__={}; mocks.invoke.mockResolvedValue({status:"sentinel"});
  await validateOperationBatch(model,batch,claim); await applyOperationBatch(model,batch,claim);
  expect(mocks.invoke.mock.calls).toEqual([["validate_model_operation_batch",{model,batch,claimedModelHash:claim}],["apply_model_operation_batch",{model,batch,claimedModelHash:claim}]]);
 });
 it("uses Wasm JSON exports and retains honest route and rollback envelope",async()=>{
  const blocked={application_route:"local_wasm_engine",simulation_disposition:"rolled_back_no_model_published",applied_model:null,acceptance:null};
  mocks.apply.mockReturnValue(JSON.stringify(blocked));mocks.validate.mockReturnValue(JSON.stringify(blocked));
  expect(await applyOperationBatch(model,batch,claim)).toEqual(blocked);await validateOperationBatch(model,batch,claim);
  expect(mocks.apply).toHaveBeenCalledWith(JSON.stringify(model),JSON.stringify(batch),JSON.stringify(claim));
  expect(mocks.validate).toHaveBeenCalledWith(JSON.stringify(model),JSON.stringify(batch),JSON.stringify(claim));
  expect(mocks.invoke).not.toHaveBeenCalled();
 });
 it("surfaces Wasm input errors without manufactured outcome",async()=>{
  mocks.validate.mockReturnValue(JSON.stringify({error:{code:"INVALID",message:"bad input"}}));
  await expect(validateOperationBatch(model,batch,null)).rejects.toThrow("INVALID: bad input");
 });
 it("passes numeric conversions through both engines without local conversion",async()=>{
  const items=[{id:"q",value:1,from_unit:"m",to_unit:"mm",dimension_id:"length"}];
  const result=[{id:"q",status:"unavailable",message:"sentinel"}];mocks.display.mockReturnValue(JSON.stringify({items:result}));
  expect(await convertDisplayQuantities(items)).toEqual(result);expect(mocks.display).toHaveBeenCalledWith(JSON.stringify({items}));
  (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__={};mocks.invoke.mockResolvedValue({items:result});
  expect(await convertDisplayQuantities(items)).toEqual(result);expect(mocks.invoke).toHaveBeenCalledWith("convert_display_quantities",{input:{items}});
 });
 it("preserves blocked selfweight plan and original hash through both routes",async()=>{
  const result={status:"blocked",plan:null,input_model_unchanged:true};mocks.plan.mockReturnValue(JSON.stringify(result));
  expect(await generateSelfWeightPlan(model,request)).toEqual(result);expect(mocks.plan).toHaveBeenCalledWith(JSON.stringify(model),JSON.stringify(request));
  (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__={};mocks.invoke.mockResolvedValue(result);
  expect(await generateSelfWeightPlan(model,request)).toEqual(result);expect(mocks.invoke).toHaveBeenCalledWith("generate_self_weight_operation_plan",{model,request});
 });
});
