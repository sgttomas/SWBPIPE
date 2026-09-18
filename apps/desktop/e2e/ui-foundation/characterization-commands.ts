import { execFile } from "node:child_process";
import { promisify } from "node:util";
import type { Page } from "@playwright/test";
import { createHash } from "node:crypto";
import { writeFile, mkdir } from "node:fs/promises";
import path from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import { readCandidateDiagnostics, ensureViewportToggle, treeRowTestId, requireUniqueConnectedMainCanvas, projectCandidateAuthoredPoint,
  validateCandidateMeasuredCameraBinding, routeModelFixture, waitForFirstUsable, validateMainCanvasHitTarget, type LoadedFixture } from "./benchmark-harness";
import { normalizedCanvasPoint, canvasLocalToClient, type HitTargetEvidence, type Rect } from "./causal-method-contract";
import { assertBoundCandidateDocumentResponse } from "./candidate-server-response";

export const FILTER_STIMULUS = Object.freeze({ name: "browser-keyboard-insert-text/v1", command: "focus then page.keyboard.insertText(query)",
  timingStart: "existing first trusted input listener", timingEnd: "unchanged final stopped-content causal witness",
  semantics: "one whole-query Chromium Input.insertText; no keydown/keyup events or hardware keyboard claim",
  comparability: "explicit keyboard API instead of historical fill; locked Playwright fill itself uses insertText for text input", expectedInputEvents: 1 });
export async function prepareFilterInput(page: Page, timeout: number) {
  const input=page.getByTestId("model-tree-filter-input");
  if(await input.inputValue()!=="")throw new Error("filter requires empty input before arming");
  await input.focus({timeout});
}
export async function insertFilterQuery(page: Page, query: string) {
  if (!query) throw new Error("frozen filter query is empty");
  await page.keyboard.insertText(query);
}
export type TypedIdentity = { type: string; id: string };
export function benchmarkCommands(page: Page, timeout: number) {
  return {
    async select(ref: TypedIdentity) {
      if (!ref.type || !ref.id) throw new Error("typed identity required");
      await page.getByTestId(treeRowTestId(ref, "candidate")).click({ timeout });
    },
    async home() { const tree = page.getByTestId("model-tree-virtual"); await tree.focus({ timeout }); await tree.press("Home", { timeout }); },
    async camera() { await ensureViewportToggle(page, "viewport-box-select", false, timeout); await page.getByTestId("viewport-view-isometric").click({ timeout }); await page.getByTestId("viewport-fit-model").click({ timeout }); },
    async labels(enabled: boolean) { await ensureViewportToggle(page, "toggle-viewport-labels", enabled, timeout); },
    async geometry(mode: "centerline" | "actual-od") { await page.getByTestId(mode === "centerline" ? "viewport-geometry-schematic" : "viewport-geometry-actual-od").click({ timeout }); },
    query() { return readCandidateDiagnostics(page, true); }
  };
}
const positive = (n: unknown) => typeof n === "number" && Number.isFinite(n) && n > 0;
const natural = (n: unknown) => Number.isSafeInteger(n) && (n as number) >= 0;
export function validateBoundaryMetadata(value: any, expected: any, previous?: any) {
  const v = value?.snapshot?.viewport, c = v?.canvas, camera = v?.camera, model = value?.snapshot?.model, p = value?.presentation;
  if (!value?.id || value.runId !== expected.runId || JSON.stringify(value.bindings) !== JSON.stringify(expected.bindings) ||
      ![c?.cssWidth,c?.cssHeight,c?.bufferWidth,c?.bufferHeight,c?.dpr,p?.browserDpr].every(positive) ||
      ![c?.cssLeft,c?.cssTop].every(Number.isFinite) || c.dpr > 2 || p.browserDpr !== 2 ||
      Math.abs(c.bufferWidth - Math.floor(c.cssWidth*c.dpr)) > 1 || Math.abs(c.bufferHeight - Math.floor(c.cssHeight*c.dpr)) > 1 ||
      p.windowWidth !== 1440 || p.windowHeight !== 920 || typeof v?.labels?.enabled !== "boolean" || !natural(v.labels.renderedCount) || !natural(v.labels.budget) ||
      v.labels.renderedCount > v.labels.budget || (!v.labels.enabled && v.labels.renderedCount !== 0) ||
      !["schematic","actual-od"].includes(v?.geometry?.mode) || !natural(v.geometry.odGeneration) || typeof v.geometry.odStatus !== "string" || !v.geometry.odStatus ||
      (v.geometry.mode === "actual-od" && (v.geometry.odStatus !== "available" || v.geometry.odGeneration <= 0)) ||
      camera?.kind !== "perspective" || !natural(camera?.sequence) || !["position","target","up","localRenderOrigin"].every(k => Array.isArray(camera[k]) && camera[k].length === 3 && camera[k].every(Number.isFinite)) ||
      ![camera.fovDegrees,camera.near,camera.far,camera.aspect].every(positive) || camera.far <= camera.near ||
      !Number.isSafeInteger(model?.generation) || model.generation <= 0 || typeof model.identityHash !== "string" || !model.identityHash ||
      !["light","dark"].includes(p?.theme) || !["comfortable","compact"].includes(p?.density) ||
      !Array.isArray(p?.panels) || p.panels.length !== 2 || p.panels.some((panel: any) => typeof panel.visible !== "boolean" || !panel.visible || !positive(panel.width) || !positive(panel.height)))
    throw new Error("required boundary metadata invalid or unavailable");
  if (previous) {
    const stable = (m: any) => ({ bindings:m.bindings, model:m.snapshot.model.identityHash, generation:m.snapshot.model.generation,
      canvas:m.snapshot.viewport.canvas, theme:m.presentation.theme, density:m.presentation.density, panels:m.presentation.panels,
      browserDpr:m.presentation.browserDpr, windowWidth:m.presentation.windowWidth, windowHeight:m.presentation.windowHeight });
    if (JSON.stringify(stable(previous)) !== JSON.stringify(stable(value))) throw new Error("boundary profile/model/binding drift");
  }
  return value;
}
export async function captureBoundary(page: Page, expected: any, id: string, previous?: any) {
  const { snapshot } = await readCandidateDiagnostics(page, true);
  const presentation = await page.evaluate(() => {
    const shell = document.querySelector(".app-shell");
    return { browserDpr: devicePixelRatio, windowWidth: innerWidth, windowHeight: innerHeight,
      theme: shell?.getAttribute("data-theme"), density: shell?.getAttribute("data-density"),
      panels: [".panel.model-tree", ".panel.inspector"].map(selector => {
        const node = document.querySelector(selector), r = node?.getBoundingClientRect();
        return { selector, visible: Boolean(node && r && r.width > 0 && r.height > 0 && getComputedStyle(node).visibility !== "hidden" && getComputedStyle(node).display !== "none"), width:r?.width, height:r?.height };
      }) };
  });
  return validateBoundaryMetadata({ id, runId:expected.runId, bindings:expected.bindings, snapshot, presentation,
    limitations: "boundary snapshot only; not continuous foreground, display or occlusion monitoring" }, expected, previous);
}
export async function persistBoundary(directory: string, metadata: any) {
  const bytes = `${JSON.stringify(metadata,null,2)}\n`, file = path.join(directory, `${metadata.id}.json`);
  await writeFile(file,bytes,{flag:"wx"});
  return { id:metadata.id, file, sha256:createHash("sha256").update(bytes).digest("hex"), runId:metadata.runId };
}

// Smoke deliberately has no causal instrumentation or epoch. Call only on the
// fresh validateMainCanvasHitTarget read, which first requires one connected canvas.
export function assertSmokeMainCanvasHitTarget(evidence: HitTargetEvidence & { canvasRect: Rect }): void {
  const rect=evidence?.canvasRect, point=evidence?.clientPoint;
  if(evidence?.status!=="PASS_ACTUAL_CONNECTED_MAIN_CANVAS_TARGET" || evidence.canvasConnected!==true ||
      evidence.exactCanvasTarget!==true || evidence.targetTag!=="CANVAS" || evidence.targetTestId!=="viewport-canvas" ||
      evidence.canvasEpoch!==null || evidence.armedCanvasEpoch!==null || evidence.exactArmedCanvas!==null ||
      ![point?.x,point?.y,rect?.x,rect?.y,rect?.width,rect?.height].every(Number.isFinite) || rect.width<=0 || rect.height<=0 ||
      point.x<rect.x || point.x>rect.x+rect.width || point.y<rect.y || point.y>rect.y+rect.height)
    throw new Error(`smoke pointer target is not the uninstrumented current main canvas: ${JSON.stringify(evidence)}`);
}

// This separate route never starts tracing, arms the latency observer or calls a scorer.
export async function runCharacterizationSmoke(page: Page, fixture: LoadedFixture, expected: any, directory: string, timeout: number, treeExpectation: (model: any, query: string) => any) {
  await mkdir(directory,{recursive:true});
  const commands=benchmarkCommands(page,timeout), steps:any[]=[];
  const errors:string[]=[];
  try {
    await routeModelFixture(page,fixture);
    assertBoundCandidateDocumentResponse(await page.goto("/",{waitUntil:"domcontentloaded",timeout}));
    await waitForFirstUsable(page,fixture,timeout,"candidate"); steps.push({action:"assignment",status:"VISIBLE_FIRST_USABLE"});
    const first=await captureBoundary(page,expected,"smoke-initial"); await persistBoundary(directory,first);
    await commands.home(); await commands.select({type:"project",id:fixture.model.project.id});
    await commands.labels(false); await commands.camera(); await delay(500); await validateCandidateMeasuredCameraBinding(page,fixture.pointOracle);
    const before=(await commands.query()).snapshot, sample=fixture.samples.point_selection[0], probe=fixture.pointOracle.probes.find((p:any)=>p.sample===sample.sample);
    const projection=await projectCandidateAuthoredPoint(page,{modelGeneration:before.model.generation,cameraSequence:before.viewport.camera.sequence,
      authoredPoint:{x:probe.authored_anchor[0],y:probe.authored_anchor[1],z:probe.authored_anchor[2]}});
    if (projection.status!=="available" || !projection.insideCanvasCss) throw new Error("smoke point projection unavailable");
    let canvas=await requireUniqueConnectedMainCanvas(page); const point=canvasLocalToClient(projection.canvasCssPoint,canvas.box);
    assertSmokeMainCanvasHitTarget(await validateMainCanvasHitTarget(page,point)); await page.mouse.click(point.x,point.y); await delay(500);
    let after=(await commands.query()).snapshot;
    if (JSON.stringify(after.viewport.selection.orderedRefs)!==JSON.stringify([probe.candidate_runtime.oracle.expectedHitRef])) throw new Error("smoke typed point selection mismatch");
    steps.push({action:"point",sample:sample.sample,expectedRef:probe.candidate_runtime.oracle.expectedHitRef});
    await commands.home(); await commands.select({type:"project",id:fixture.model.project.id}); await commands.camera(); await delay(500);
    const boxSample=fixture.samples.box_selection[0], box=fixture.pointOracle.candidate_box_selection.samples.find((s:any)=>s.sample===boxSample.sample);
    await page.getByTestId("viewport-selection-filter").selectOption({label:({all:"All",pipes:"Pipes",nodes:"Nodes",supports:"Supports",components:"Components"} as any)[boxSample.filter]},{timeout});
    await ensureViewportToggle(page,"viewport-box-select",true,timeout); canvas=await requireUniqueConnectedMainCanvas(page);
    const start=normalizedCanvasPoint(canvas.box,{x:boxSample.start_normalized[0],y:boxSample.start_normalized[1]}), end=normalizedCanvasPoint(canvas.box,{x:boxSample.end_normalized[0],y:boxSample.end_normalized[1]});
    assertSmokeMainCanvasHitTarget(await validateMainCanvasHitTarget(page,start)); assertSmokeMainCanvasHitTarget(await validateMainCanvasHitTarget(page,end));
    await page.mouse.move(start.x,start.y); await page.mouse.down(); await page.mouse.move(end.x,end.y,{steps:8}); await page.mouse.up(); await delay(500);
    after=(await commands.query()).snapshot;
    if (JSON.stringify(after.viewport.box.orderedRefs)!==JSON.stringify(box.orderedRefs) || JSON.stringify(after.viewport.box.primaryRef)!==JSON.stringify(box.primaryRef)) throw new Error("smoke box oracle mismatch");
    steps.push({action:"box",sample:boxSample.sample,expected:box});
    const query=fixture.samples.tree_filters[0].query; await page.getByTestId("model-tree-filter-input").fill("",{timeout}); await commands.home(); await prepareFilterInput(page,timeout); await insertFilterQuery(page,query); await delay(500);
    after=(await commands.query()).snapshot; if(after.viewport.filter.query!==query)throw new Error("smoke keyboard insertion query mismatch");
    const expectedTree=treeExpectation(fixture.model,query);
    const visibleRows=await page.locator('.panel.model-tree [role="treeitem"]').evaluateAll(rows=>rows.map(row=>({testId:row.getAttribute("data-testid"),label:row.querySelector(".tree-item-label")?.textContent?.trim() ?? row.textContent?.trim(),
      level:Number(row.getAttribute("aria-level")),position:Number(row.getAttribute("aria-posinset")),setSize:Number(row.getAttribute("aria-setsize"))})));
    if(after.viewport.filter.visibleCount!==expectedTree.visibleCount || visibleRows.length!==expectedTree.mountedCount || visibleRows.some((row:any,i:number)=>{
      const expected=expectedTree.rows[i];return !expected || ["testId","level","position","setSize"].some(k=>row[k]!==expected[k]) || !row.label?.includes(expected.label);
    }))throw new Error("smoke filter full count/typed mounted content mismatch");
    steps.push({action:"filter",query,stimulus:FILTER_STIMULUS,expectedTree,visibleRows}); await page.getByTestId("model-tree-filter-input").fill("",{timeout});
    for(const mode of ["centerline","actual-od"] as const) {
      const cold=(await commands.query()).snapshot; if(mode==="actual-od" && cold.viewport.geometry.odGeneration!==0)throw new Error("smoke OD cold precondition lost");
      await commands.labels(true); await commands.geometry(mode);
      await page.waitForFunction(({mode})=>{const s=(globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();return s?.viewport?.geometry?.mode===(mode==="centerline"?"schematic":"actual-od") && (mode==="centerline" || (s.viewport.geometry.odStatus==="available"&&s.viewport.geometry.odGeneration>0));},{mode},{timeout,polling:50});
      await commands.camera(); await delay(500);
      const b=await captureBoundary(page,expected,`smoke-${mode}`,first); await persistBoundary(directory,b);
      canvas=await requireUniqueConnectedMainCanvas(page); const s=normalizedCanvasPoint(canvas.box,{x:.5,y:.5});
      assertSmokeMainCanvasHitTarget(await validateMainCanvasHitTarget(page,s)); await page.mouse.move(s.x,s.y); await page.mouse.down(); await page.mouse.move(s.x+30,s.y+20,{steps:8}); await page.mouse.up(); await delay(500);
      after=(await commands.query()).snapshot; if(after.viewport.camera.sequence<=b.snapshot.viewport.camera.sequence)throw new Error("smoke orbit camera did not change");
      steps.push({action:"orbit",mode,labels:after.viewport.labels,conversion:after.viewport.geometry});
    }
  } catch(error) { errors.push(String(error)); }
  const result={status:errors.length?"FAIL_UNTIMED_SMOKE":"PASS_UNTIMED_SMOKE",qualification:false,cohortContribution:0,fixtureSize:expected.fixtureSize,steps,errors,claim:"control-path smoke only; no timing, trace or performance score"};
  await writeFile(path.join(directory,"smoke-result.json"),`${JSON.stringify(result,null,2)}\n`,{flag:"wx"});
  if(errors.length)throw new Error(`smoke failed: ${errors.join("; ")}`);
  return result;
}

export function validateDisplayProfile(raw: any, expected: any, previous?: any) {
  const devices=raw?.SPDisplaysDataType;
  const displays=Array.isArray(devices)?devices.flatMap((gpu:any)=>(gpu.spdisplays_ndrvs??[]).map((display:any)=>({gpu:gpu.sppci_model,display}))):[];
  const online=displays.filter((v:any)=>v.display.spdisplays_online==="spdisplays_yes");
  const matches=online.filter((v:any)=>v.display.spdisplays_main==="spdisplays_yes");
  if(matches.length!==1 || !expected || matches[0].gpu!=="Apple M5 Max")throw new Error("missing or ambiguous main online display");
  const d=matches[0].display, identity={name:d._name,vendor:d["_spdisplays_display-vendor-id"],product:d["_spdisplays_display-product-id"],
    serial:d["_spdisplays_display-serial-number"],pixels:d._spdisplays_pixels,resolution:d._spdisplays_resolution,mirror:d.spdisplays_mirror};
  if(!Object.values(identity).every(v=>typeof v==="string"&&v.length>0) || !/@ 60\.00Hz$/.test(identity.resolution) || identity.mirror!=="spdisplays_off" ||
    Object.keys(identity).some(k => (identity as any)[k] !== expected[k]) || (previous && Object.keys(identity).some(k => previous[k] !== (identity as any)[k])))throw new Error("external display profile drift or unavailable 60 Hz binding");
  return identity;
}
export async function captureDisplayProfile(directory: string, label: string, expected: any, previous?: any) {
  if(process.platform!=="darwin")throw new Error("required macOS display profile unavailable");
  const {stdout}=await promisify(execFile)("/usr/sbin/system_profiler",["SPDisplaysDataType","-json"],{timeout:30000,maxBuffer:4*1024*1024});
  const file=path.join(directory,`display-${label}.json`);await writeFile(file,stdout,{flag:"wx"});
  const identity=validateDisplayProfile(JSON.parse(stdout),expected,previous);
  return {identity,file,sha256:createHash("sha256").update(stdout).digest("hex"),limitation:"pre/post host boundary only; no window-to-display or continuous-monitoring proof"};
}
