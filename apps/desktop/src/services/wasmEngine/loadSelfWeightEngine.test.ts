import { expect, it, vi } from "vitest";
import { loadSelfWeightEngine } from "./loadSelfWeightEngine";
it("missing browser artifact gives actionable error and retries",async()=>{
 const runtime=globalThis.process;
 let errors:unknown[]=[];
 try {
  vi.stubGlobal("process",{...runtime,versions:{}});
  for(let attempt=0;attempt<2;attempt++) {
   try {await loadSelfWeightEngine();} catch(error){errors.push(error);}
  }
 } finally {vi.stubGlobal("process",runtime);}
 expect(errors).toHaveLength(2);
 for(const error of errors) expect(String(error)).toMatch(/SELF-WEIGHT-ENGINE-ASSET-ABSENT.*npm run build:wasm/);
});
