// Historical attachment-centre transition control for the earlier 97b18c96 generation.
// Requires absolute --old-dir and --new-dir inputs containing both 1000/10000 generated
// candidate-runtime-point-oracle-v3 plans: only component/support visual geometry changes,
// and new plans bind the exact geometryHash below. It does not validate current plans.
// Those old/new input directories are not supplied in this repository; without both,
// this historical control cannot be executed or claimed as passing.
// Offline controls over actual generator output; never a source of runtime expectations.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { primitiveCenterIndex } from "./point-hit-oracle.mjs";
const root = path.dirname(fileURLToPath(import.meta.url));
const args = new Map();
for (let i = 2; i < process.argv.length; i += 2) args.set(process.argv[i], process.argv[i + 1]);
const newDir = args.get("--new-dir"), oldDir = args.get("--old-dir");
assert(newDir && oldDir && path.isAbsolute(newDir) && path.isAbsolute(oldDir), "absolute new/old oracle directories required");
const geometryHash = "97b18c9671fc7f98e1cbb94bf6833f737c91e3ef4580d02c711d306c82486aee";
const close = (actual, expected, tolerance = 1e-10) => {
  assert(Number.isFinite(actual) && Number.isFinite(expected)); assert(Math.abs(actual - expected) <= tolerance, `${actual} != ${expected}`);
};
function independentCenter(model, ref) {
  const node = (id) => { const n = model.nodes.find((entry) => entry.id === id); assert(n, "attachment geometry absent"); return [n.position.x, n.position.y, n.position.z]; };
  if (ref.type === "node") return node(ref.id);
  if (ref.type === "pipe") {
    const pipe = model.pipe_segments.find((entry) => entry.id === ref.id); assert(pipe, "pipe absent");
    const a = node(pipe.from), b = node(pipe.to); return a.map((v, i) => (v + b[i]) / 2);
  }
  assert(["component", "support"].includes(ref.type));
  const attachment = model[ref.type === "component" ? "components" : "supports"].find((entry) => entry.id === ref.id);
  assert(attachment, "attachment absent"); const p = node(attachment.node);
  // Deliberately independent literal expectations: neither primitive index nor generator supplies offsets here.
  return [p[0], p[1] + (ref.type === "component" ? .2 : -.26), p[2]];
}
function checkGeometry(model, probe) {
  const p = probe.candidate_runtime.visual_plan; assert(p, "plan absent");
  assert.equal(p.source.geometrySourceSha256, geometryHash, "geometry source");
  const expected = independentCenter(model, probe.candidate_runtime.oracle.expectedHitRef);
  assert.equal(p.authoredCenter.length, 3); p.authoredCenter.forEach((v, i) => close(v, expected[i]));
  // Independent camera-space projection of the source-correct world center.
  const cross = (a,b) => [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]];
  const dot = (a,b) => a.reduce((sum,v,i) => sum+v*b[i],0), norm = (a) => a.map((v) => v/Math.hypot(...a));
  const c = p.camera, forward = norm(c.target.map((v,i) => v-c.position[i])), right = norm(cross(forward,c.up)), up = norm(cross(right,forward));
  const delta = expected.map((v,i) => v-c.position[i]), hh = dot(delta,forward)*Math.tan(c.fovDegrees*Math.PI/360);
  close(p.centerCss.x, p.canvas.cssLeft + (1+dot(delta,right)/(hh*c.aspect))*p.canvas.cssWidth/2, 1e-8);
  close(p.centerCss.y, p.canvas.cssTop + (1-dot(delta,up)/hh)*p.canvas.cssHeight/2, 1e-8);
  assert.deepEqual(p.clip, { x: Math.floor(p.centerCss.x)-24, y: Math.floor(p.centerCss.y)-24, width:48, height:48 });
}
let points = 0, boxes = 0, faults = 0; const inventory = {}, representatives = new Map();
for (const n of [1000,10000]) {
  const name = `ui-foundation-${n}.candidate-runtime-point-oracle-v3.json`;
  const [fresh, prior, model] = await Promise.all([readFile(path.join(newDir,name)), readFile(path.join(oldDir,name)), readFile(path.join(root,"fixtures",`ui-foundation-${n}.model.json`))].map(async (p) => JSON.parse(await p)));
  assert.equal(fresh.probes.length,200); assert.equal(prior.probes.length,200);
  assert.deepEqual(fresh.candidate_box_selection,prior.candidate_box_selection); boxes += fresh.candidate_box_selection.samples.length;
  assert.deepEqual(fresh.candidate_preflight,prior.candidate_preflight);
  for (let i=0;i<fresh.probes.length;i++) {
    const probe=fresh.probes[i], old=prior.probes[i], strip=(p) => { const result=structuredClone(p); delete result.candidate_runtime.visual_plan; return result; };
    assert.deepEqual(strip(probe),strip(old),"all click/sample/picking/projection invariants");
    checkGeometry(model,probe); points++;
    const type=probe.candidate_runtime.oracle.expectedHitRef.type, plan=probe.candidate_runtime.visual_plan, previous=old.candidate_runtime.visual_plan;
    const geometry=(p) => { const {sha256,source,...result}=p; return result; };
    const changed=JSON.stringify(geometry(plan))!==JSON.stringify(geometry(previous));
    assert.equal(changed,["component","support"].includes(type),"only attachment visual geometry may change");
    inventory[type]??={total:0,changed:0};inventory[type].total++;if(changed)inventory[type].changed++;
    if(!representatives.has(type))representatives.set(type,{model,probe});
    if(n===1000 && probe.sample===7) {
      plan.authoredCenter.forEach((v,i)=>close(v,[9.8,.479336,.000505][i],1e-12));
      close(plan.centerCss.x,816.6744617106,1e-8);close(plan.centerCss.y,678.0499515777,1e-8);
      assert.deepEqual(plan.clip,{x:792,y:654,width:48,height:48});
    }
  }
}
assert.deepEqual([...representatives.keys()].sort(),["component","node","pipe","support"]);
for(const [type,{model,probe}] of representatives) {
  const ref=probe.candidate_runtime.oracle.expectedHitRef, expected=independentCenter(model,ref), shift=[103,-27,58];
  const shifted=primitiveCenterIndex(model,shift).get(`${type}\0${ref.id}`); assert(shifted);
  shifted.forEach((v,i)=>close(v,expected[i]-shift[i]));
  const translated=structuredClone(model);translated.nodes.forEach((n)=>{n.position.x+=shift[0];n.position.y+=shift[1];n.position.z+=shift[2];});
  const moved=structuredClone(probe), plan=moved.candidate_runtime.visual_plan;
  plan.authoredCenter=plan.authoredCenter.map((v,i)=>v+shift[i]);for(const key of ["position","target"])plan.camera[key]=plan.camera[key].map((v,i)=>v+shift[i]);
  checkGeometry(translated,moved);
  for(const mutate of [(p)=>{delete p.candidate_runtime.visual_plan.source.geometrySourceSha256;},(p)=>{p.candidate_runtime.visual_plan.source.geometrySourceSha256="stale";},(p)=>{p.candidate_runtime.visual_plan.authoredCenter[1]+=1;}]) {
    const bad=structuredClone(probe);mutate(bad);assert.throws(()=>checkGeometry(model,bad));faults++;
  }
  if(["component","support"].includes(type)) {
    const field=type==="component"?"components":"supports", item=model[field].find((v)=>v.id===ref.id), node=model.nodes.find((v)=>v.id===item.node);
    for(const offset of [0,type==="component"?-.2:.26]) {
      const bad=structuredClone(probe);bad.candidate_runtime.visual_plan.authoredCenter[1]=node.position.y+offset;
      assert.throws(()=>checkGeometry(model,bad));faults++;
    }
    const wrong=structuredClone(model);wrong[field].find((v)=>v.id===ref.id).node=model.nodes.find((v)=>v.id!==item.node && v.position.x!==node.position.x).id;
    assert.throws(()=>checkGeometry(wrong,probe));faults++;
    const missing=structuredClone(model);missing.nodes=missing.nodes.filter((v)=>v.id!==item.node);
    assert.throws(()=>checkGeometry(missing,probe));assert.equal(primitiveCenterIndex(missing).has(`${type}\0${ref.id}`),false);faults++;
  }
}
process.stdout.write(JSON.stringify({status:"PASS_ACTUAL_GENERATED_GEOMETRY_AND_INVARIANTS",points,boxes,inventory,types:[...representatives.keys()],faults,
  allowances:{world:1e-10,projectedCss:1e-8,sample7World:1e-12},claim:"offline oracle controls; no live visual qualification"},null,2)+"\n");
