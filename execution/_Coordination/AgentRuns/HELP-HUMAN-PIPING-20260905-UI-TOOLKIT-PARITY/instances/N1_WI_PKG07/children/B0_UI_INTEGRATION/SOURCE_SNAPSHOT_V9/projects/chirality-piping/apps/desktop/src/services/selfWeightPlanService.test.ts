import { expect, it } from "vitest";
import { loadWasmEngine } from "./wasmEngine/loadWasmEngine";
import { loadSelfWeightEngine } from "./wasmEngine/loadSelfWeightEngine";
import { generateSelfWeightPlan } from "./selfWeightPlanService";
import { convertDisplayQuantities } from "./displayQuantityService";
import { validateOperationBatch } from "./operationBatchService";
import type { PreviewModel } from "../types";
it("generated shared Wasm artifacts answer batch, conversion and authenticated selfweight planning",async()=>{
 const engine=await loadWasmEngine();
 const model={schema_version:"invented",document_kind:"preview",project:{id:"invented"},analysis_status:{mechanics:"draft",rule_check:"not_run",professional_acceptance:"not_assessed"},extension:{retained:"full original"},nodes:[{id:"a",position:{x:0,y:0,z:0}},{id:"b",position:{x:1,y:0,z:0}}],pipe_segments:[{id:"pipe",from:"a",to:"b",material:"invented",provenance:"invented pipe",section:{outside_diameter:{value:0.1,unit:"m"},wall_thickness:{value:0.01,unit:"m"},material_density:{value:1000,unit:"kg/m^3"}}}],supports:[]} as unknown as PreviewModel;
 const original=JSON.stringify(model);
 const request={case_id:"self",label:"Invented",pipe_refs:["pipe"],gravity:{value:-7,unit:"m/s^2",axis:"global_z"},provenance:"explicit invented input",source_model_hash:`sha256:${engine.canonicalSha256Hex(original)}`};
 const outcome=await generateSelfWeightPlan(model,request);expect(outcome.status).toBe("ready");expect(outcome.plan?.changes.length).toBeGreaterThan(0);expect(outcome).not.toHaveProperty("applied_model");
 const stale=await generateSelfWeightPlan({...model,extension:{retained:"changed"}} as PreviewModel,request);expect(stale.status).toBe("blocked");expect(stale.plan).toBeNull();
 const malformed=JSON.parse((await loadSelfWeightEngine()).generateSelfWeightPlanJson("{","{}"));expect(malformed.status).toBe("blocked");
 const batch=await validateOperationBatch(model,{batch_id:"empty",operations:[]},null);expect(batch.application_route).toBe("local_wasm_engine");expect(batch.applied_model).toBeNull();expect(batch.acceptance).toBeNull();
 expect(await convertDisplayQuantities([{id:"length",value:1,from_unit:"m",to_unit:"mm",dimension_id:"length"}])).toEqual([{id:"length",status:"converted",value:1000,unit:"mm"}]);
 expect(JSON.stringify(model)).toBe(original);
});
