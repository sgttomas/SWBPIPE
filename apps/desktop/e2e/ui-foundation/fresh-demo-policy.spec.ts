import { test, expect } from '@playwright/test';
import { mkdtempSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { FRESH_DEMO_BINDINGS, validateFreshDemoPolicy, freshDemoPolicy, enterFreshDemo, freshDemoDisposition } from './fresh-demo-policy.mjs';
const hash = (s: string) => createHash('sha256').update(s).digest('hex');
function setup(root: string) {
  const env: NodeJS.ProcessEnv = {UI_FOUNDATION_PHASE:'candidate', UI_FOUNDATION_COLLECTION_MODE:'characterization', UI_FOUNDATION_PIPE_COUNTS:'10000', UI_FOUNDATION_RUNS:'1'};
  for(const key of FRESH_DEMO_BINDINGS) env[key]=key.endsWith('_SHA256')?'a'.repeat(64):path.join(root,key);
  Object.assign(env, {UI_FOUNDATION_COHORT_ID:'fresh-demo-01',UI_FOUNDATION_EVIDENCE_DIR:path.join(root,'raw'),UI_FOUNDATION_CANDIDATE_SOURCE_STAGE:'final'});
  env.UI_FOUNDATION_CANDIDATE_OUTPUT_ROOT=path.join(env.UI_FOUNDATION_CANDIDATE_SOURCE_ROOT!,'apps/desktop/dist');
  const policy={schema:'ui-foundation.fresh-demo-policy/v1',purpose:'timed',fixtureSize:10000,runNumber:1,productRevision:'b'.repeat(40),refreshHz:120,qualificationCohort:false,authority:'owner direction; sealed root release',bindings:Object.fromEntries(FRESH_DEMO_BINDINGS.map(k=>[k,env[k]]))};
  return {env,policy};
}
test('fresh policy requires exact one-run authorization and all prospective bindings',()=>{
  const {env,policy}=setup('/tmp/demo');
  expect(validateFreshDemoPolicy(policy,env)).toBe(policy);
  for(const key of FRESH_DEMO_BINDINGS) expect(()=>validateFreshDemoPolicy(policy,{...env,[key]:'changed'})).toThrow();
  for(const patch of [{fixtureSize:1000},{runNumber:2},{productRevision:''},{refreshHz:60},{qualificationCohort:true},{authority:''}]) expect(()=>validateFreshDemoPolicy({...policy,...patch},env)).toThrow();
  for(const patch of [{UI_FOUNDATION_RUNS:'1,2'},{UI_FOUNDATION_PIPE_COUNTS:'1000'},{UI_FOUNDATION_SMOKE:'controls'},{UI_FOUNDATION_DIAGNOSTIC_MODE:'assignment-collection'},{UI_FOUNDATION_CONTINUATION_TOKEN:'old'},{UI_FOUNDATION_ACTION_TIMEOUT_MS:'1'}]) expect(()=>validateFreshDemoPolicy(policy,{...env,...patch})).toThrow();
});
test('fresh hash and exclusive entry reject policy drift and output replay',()=>{
  const root=mkdtempSync(path.join(tmpdir(),'fresh-demo-'));
  try {
    const {env,policy}=setup(root),file=path.join(root,'policy.json'),bytes=JSON.stringify(policy);
    writeFileSync(file,bytes); env.UI_FOUNDATION_FRESH_DEMO_POLICY=file;env.UI_FOUNDATION_FRESH_DEMO_POLICY_SHA256=hash(bytes);
    expect(freshDemoPolicy(env)).toEqual(policy);expect(enterFreshDemo(env)).toEqual(policy);
    expect(()=>enterFreshDemo(env)).toThrow(/already used/);
    expect(()=>freshDemoPolicy({...env,UI_FOUNDATION_FRESH_DEMO_POLICY_SHA256:'0'.repeat(64)})).toThrow(/hash drift/);
    expect(()=>freshDemoPolicy({...env,UI_FOUNDATION_FRESH_DEMO_POLICY_SHA256:undefined})).toThrow();
    expect(freshDemoPolicy({})).toBeNull();
  } finally {rmSync(root,{recursive:true,force:true});}
});
test('fresh smoke is explicit, keeps both-size controls and never consumes timed entry',()=>{
  const {env,policy}=setup('/tmp/demo');policy.purpose='smoke';
  Object.assign(env,{UI_FOUNDATION_SMOKE:'controls',UI_FOUNDATION_PIPE_COUNTS:'1000,10000',UI_FOUNDATION_RUNS:'1,2,3,4,5'});
  expect(validateFreshDemoPolicy(policy,env)).toBe(policy);
  expect(()=>validateFreshDemoPolicy(policy,{...env,UI_FOUNDATION_SMOKE:undefined})).toThrow();
});
test('single demonstration retains target failure and invalid/incomplete disposition',()=>{
  const record={status:'FAIL_TARGETS',collection:{evidenceValidity:'VALID',collectionCompleteness:'COMPLETE'}};
  expect(freshDemoDisposition(record)).toMatchObject({attemptDisposition:'COMPLETED',targetOutcome:'FAIL_TARGETS'});
  expect(freshDemoDisposition({...record,collection:{...record.collection,evidenceValidity:'INVALID'}}).attemptDisposition).toBe('ABORTED');
  expect(freshDemoDisposition({...record,collection:{...record.collection,collectionCompleteness:'INCOMPLETE'}}).attemptDisposition).toBe('ABORTED');
  expect(freshDemoDisposition(null).attemptDisposition).toBe('ABORTED');
});
