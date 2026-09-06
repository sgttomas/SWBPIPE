import {it,expect,vi} from "vitest";
import {writeFileSync} from "node:fs";
import modelFixture from "/Users/ryan/.codex/worktrees/8728/chirality/projects/chirality-piping/fixtures/product_preview/invented_preview_model.json";
const {invokeMock}=vi.hoisted(()=>({invokeMock:vi.fn()}));
vi.mock("@tauri-apps/api/core",()=>({invoke:invokeMock}));
import {runPreviewMechanics,startPreviewMechanicsJob} from "/Users/ryan/.codex/worktrees/8728/chirality/projects/chirality-piping/apps/desktop/src/services/previewService.ts";
it("captures actual service behavior under injected native invocation failure",async()=>{
 const observations:any[]=[];
 for(const edited of [false,true]){
  const model=structuredClone(modelFixture);
  if(edited)model.nodes[1].position.x+=0.125;
  vi.unstubAllGlobals();
  const baseline=await runPreviewMechanics(model as any,"dense_scrutiny");
  vi.stubGlobal("window",{__TAURI_INTERNALS__:{}});
  invokeMock.mockReset();invokeMock.mockRejectedValue(new Error("I1_R_INJECTED_NATIVE_FAILURE"));
  const receipt=await startPreviewMechanicsJob(model as any,"dense_scrutiny");
  const result=await runPreviewMechanics(model as any,"dense_scrutiny");
  expect(invokeMock.mock.calls.map(c=>c[0])).toEqual(["start_preview_mechanics_job_with_solver_mode","run_preview_mechanics_with_solver_mode"]);
  observations.push({edited,receipt,baseline,result,equalsBrowserBaseline:JSON.stringify(result)===JSON.stringify(baseline),calls:invokeMock.mock.calls});
 }
 vi.unstubAllGlobals();
 writeFileSync("/Users/ryan/.codex/worktrees/8728/chirality/projects/chirality-piping/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/I1/supplement_v1/R/OBSERVATIONS.json",JSON.stringify({runtime:"node service test with Tauri detection shim; actual source imported; native invoke only mocked",observations},null,2)+"\n");
});
