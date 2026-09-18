// Explicit owner-frozen authorization for one new full-workload demonstration.
import { readFileSync, mkdirSync, readdirSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import path from 'node:path';
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const sha = value => /^[a-f0-9]{64}$/.test(value ?? '');
export const FRESH_DEMO_BINDINGS = Object.freeze([
  'UI_FOUNDATION_COHORT_ID', 'UI_FOUNDATION_EVIDENCE_DIR', 'UI_FOUNDATION_MANIFEST_SHA256',
  'UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST', 'UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST_SHA256',
  'UI_FOUNDATION_CANDIDATE_SOURCE_ROOT', 'UI_FOUNDATION_CANDIDATE_OUTPUT_ROOT', 'UI_FOUNDATION_CANDIDATE_SOURCE_STAGE',
  'UI_FOUNDATION_CANDIDATE_ORACLE_DIR', 'UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256',
  'UI_FOUNDATION_METHOD_MANIFEST_PATH', 'UI_FOUNDATION_METHOD_MANIFEST_SHA256',
  'PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH', 'PLAYWRIGHT_CHROMIUM_EXECUTABLE_SHA256',
  'UI_FOUNDATION_REFERENCE_PROFILE', 'UI_FOUNDATION_REFERENCE_PROFILE_SHA256'
]);
export function validateFreshDemoPolicy(policy, env) {
  if(policy?.schema !== 'ui-foundation.fresh-demo-policy/v1' || policy.fixtureSize !== 10000 || policy.runNumber !== 1 ||
    !/^[a-f0-9]{40}$/.test(policy.productRevision ?? '') || policy.refreshHz !== 120 ||
    !['timed','smoke'].includes(policy.purpose) || policy.qualificationCohort !== false || typeof policy.authority !== 'string' || !policy.authority.trim() ||
    !policy.bindings || Object.keys(policy.bindings).length !== FRESH_DEMO_BINDINGS.length ||
    FRESH_DEMO_BINDINGS.some(k => typeof policy.bindings[k] !== 'string' || !policy.bindings[k] || env[k] !== policy.bindings[k]))
    throw new Error('fresh demonstration policy/binding mismatch');
  if(env.UI_FOUNDATION_PHASE !== 'candidate' || env.UI_FOUNDATION_COLLECTION_MODE !== 'characterization' ||
    env.UI_FOUNDATION_PIPE_COUNTS !== (policy.purpose === 'smoke' ? '1000,10000' : '10000') || env.UI_FOUNDATION_RUNS !== (policy.purpose === 'smoke' ? '1,2,3,4,5' : '1') ||
    env.UI_FOUNDATION_SMOKE !== (policy.purpose === 'smoke' ? 'controls' : undefined) || env.UI_FOUNDATION_DIAGNOSTIC_MODE !== undefined ||
    env.UI_FOUNDATION_ACTION_TIMEOUT_MS !== undefined || Object.keys(env).some(k => k.startsWith('UI_FOUNDATION_CONTINUATION_') && env[k] !== undefined))
    throw new Error('fresh demonstration selection or mode conflict');
  if(!/^[A-Za-z0-9._:-]{1,48}$/.test(env.UI_FOUNDATION_COHORT_ID) || env.UI_FOUNDATION_CANDIDATE_SOURCE_STAGE !== 'final' ||
    FRESH_DEMO_BINDINGS.filter(k => k.endsWith('_SHA256')).some(k => !sha(env[k])) ||
    FRESH_DEMO_BINDINGS.filter(k => /(?:_ROOT|_DIR|_PATH|_PROFILE|_MANIFEST)$/.test(k)).some(k => !path.isAbsolute(env[k])) ||
    path.resolve(env.UI_FOUNDATION_CANDIDATE_OUTPUT_ROOT) !== path.join(path.resolve(env.UI_FOUNDATION_CANDIDATE_SOURCE_ROOT), 'apps/desktop/dist'))
    throw new Error('fresh demonstration final provenance inputs invalid');
  return policy;
}
export function freshDemoPolicy(env = process.env) {
  const file = env.UI_FOUNDATION_FRESH_DEMO_POLICY, expected = env.UI_FOUNDATION_FRESH_DEMO_POLICY_SHA256;
  if(file === undefined && expected === undefined) return null;
  if(!file || !path.isAbsolute(file) || !sha(expected)) throw new Error('fresh demonstration hash-bound policy required');
  const bytes = readFileSync(file);
  if(digest(bytes) !== expected) throw new Error('fresh demonstration policy hash drift');
  return validateFreshDemoPolicy(JSON.parse(bytes.toString()), env);
}
export function enterFreshDemo(env = process.env) {
  const policy = freshDemoPolicy(env);
  if(!policy || policy.purpose !== 'timed') throw new Error('fresh timed demonstration policy required');
  const root = env.UI_FOUNDATION_EVIDENCE_DIR;
  mkdirSync(root, { recursive: true });
  // Playwright may create its own output folder before beforeAll. No prior run evidence is allowed.
  if(readdirSync(root).some(name => name !== 'playwright-artifacts')) throw new Error('fresh demonstration evidence root already used');
  writeFileSync(path.join(root, 'fresh-demo-entered.json'), JSON.stringify({schema:'ui-foundation.fresh-demo-entry/v1',
    policySha256:env.UI_FOUNDATION_FRESH_DEMO_POLICY_SHA256, productRevision:policy.productRevision,
    cohortId:env.UI_FOUNDATION_COHORT_ID, startedAt:new Date().toISOString()}, null, 2) + '\n', {flag:'wx'});
  return policy;
}
export function freshDemoDisposition(record) {
  const valid = record?.collection?.evidenceValidity === 'VALID' && record?.collection?.collectionCompleteness === 'COMPLETE';
  return {mode:'characterization', attemptDisposition:valid?'COMPLETED':'ABORTED',
    collectionCompleteness:valid?'COMPLETE_SINGLE_DEMONSTRATION':'INCOMPLETE_SINGLE_DEMONSTRATION',
    evidenceValidity:valid?'VALID':'INVALID_OR_UNAVAILABLE', targetOutcome:record?.status ?? 'UNAVAILABLE'};
}
