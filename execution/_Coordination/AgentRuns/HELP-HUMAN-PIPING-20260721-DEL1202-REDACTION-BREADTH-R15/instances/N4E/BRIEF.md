# N4E Sealed Brief — Exact N5D remediation attempt 5

AgentRole: TASK  
TaskSkill: `software-bounded-implementation`  
ApplyEdits: true  
RequestedBy: HELP_HUMAN through WORKING_ITEMS  
RunID: HELP-HUMAN-PIPING-20260721-DEL1202-REDACTION-BREADTH-R15  
ChildInstanceID: N4E  
PackageID: PKG-12  
DeliverableID: DEL-12-02  
FrozenBase: `0c066652cd527eb1559f715e914262d2bda42602`

## Objective

Act as the sole active serialized implementation owner. Remediate exactly the
three N5D findings without widening candidate v6, changing governed state, or
disturbing attempts 1–4.

## Authorized product/test write paths

- `projects/chirality-piping/core/security/redaction/route_control.py`
- `projects/chirality-piping/tests/security/test_redaction_export_controls.py`
- `projects/chirality-piping/tests/test_handoff_export_workflow.py` (A1:
  expectation/fixture-only alignment for seven direct REXC-CORE-001 failures)
- `projects/chirality-piping/apps/desktop/src/features/redaction-controls/redactionExportControls.ts`
- `projects/chirality-piping/apps/desktop/src/features/redaction-controls/redactionExportControls.test.ts`
- `projects/chirality-piping/apps/desktop/src/features/redaction-controls/ControlledExportLink.tsx`
- `projects/chirality-piping/apps/desktop/src/features/redaction-controls/ControlledExportLink.test.tsx`
- `projects/chirality-piping/apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx`
- `projects/chirality-piping/apps/desktop/src/features/local-fea-handoff/LocalFeaHandoffPanel.test.tsx` (A2: expectation-only alignment for two
  direct DOTH-HANDOFF-002 controlled-output failures)
- `projects/chirality-piping/apps/desktop/src/App.test.tsx`
- `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts` (A3:
  expectation-only alignment for the controlled-export H4 payload)

Evidence writes are limited to `instances/N4E/**` plus the one new attempt-5
sweep artifact after all gates. Do not edit prior attempts, prior sweeps, the
candidate, route/owner maps, status, memory, deliverable run records, receipt,
protected/release tools, policy, parity corpus, schemas, lifecycle, or Git.

Versioned amendment A1 governs the additional test path above and prohibits
all product compatibility weakening or unrelated test changes.

Versioned amendment A2 governs the local-FEA test path and prohibits product,
handoff-semantic, public-authority, or unrelated assertion changes.

Versioned amendment A3 governs the E2E path and prohibits product, flow,
public-authority, unrelated-coverage, or external-artifact-policy changes.

## Exact remediation requirements

### A. Remove safe-key public inference

- Delete/disable the Python and TypeScript broad token, suffix, or key-name
  heuristics as a public authority source.
- An unmetadataed scalar under any opaque mapping/object/list/array remains
  unknown in public, shared, and downstream contexts and is redacted/blocked
  according to the existing contract; local-private unknown behavior remains
  `warning_only` unless a separate known-private rule applies.
- Retain only explicit leaf metadata, complete exact-record-local privacy plus
  redistribution metadata, or the existing exact PCF/MBF structural allowlist
  when its route, document, deliverable, root container, and normalized path
  all match.
- Add Python and TypeScript adversarial tests for `schema`,
  `nested_deliverable_id`, `target_family`, matching suffix/token lookalikes,
  wrong routes, wrong deliverables, and opaque nested structures.

### B. CAEPIPE Parser CSV explicit intent

- The Parser CSV member owns its own adjacent explicit local-private intent.
  Harness JSON intent must not authorize it.
- Before intent, no href, handoff, write, or raw CSV exposure is permitted when
  the CSV contains IDs, load-case IDs, result magnitudes, or other known
  private values. After intent, preserve the existing local-private
  warning/decision evidence and exact CSV payload without source mutation.
- Add focused component/App tests proving independent intent, fail-closed
  behavior, no download/write side effect before intent, and exposure only
  after the Parser CSV checkbox itself is selected.

### C. Observable sanitized evidence

- Before an enabled download/handoff link, render sanitized per-decision and
  per-finding evidence sufficient to inspect path, classification/action, and
  reason/severity. Counts alone are insufficient.
- Never render or duplicate raw private values in the evidence UI.
- Preserve blocked-link behavior and accessible names/semantics. Add focused
  accessibility-oriented tests for allowed and blocked outcomes.

## Verification and sweep discipline

Run focused tests first, then the union required by
`projects/chirality-piping/software-workflow.json`: piping pytest, desktop
Vitest, production build, H4 source and production-dist evidence with output
outside the repository, relevant Rust checks, full practitioner harness,
self-check, claims/path/receipt/JSON/containment/protected/state/diff
validators. Record failed intermediate evidence honestly.

Before any sweep, produce a complete tracked and untracked dirty-path
inventory, including ignored generated test-output checks, with zero fence
violations and zero dirty `test-results/` paths. Report the terminal pre-sweep
decision and exact intended command to the parent. Only then invoke exactly
one attempt-5 registered evidence sweep. Never rerun it. Identify and hash its
single new artifact, recheck containment read-only, and mark attempts 1–4
superseded/non-acceptance without changing their bytes.

## Return

Return `SUCCESS` or `BLOCK` with exact changed paths, test counts, validator
results, sweep count/path/hash, containment, residual limitations, and explicit
confirmation of no state/receipt/Git effect. A fresh N5E remains mandatory.
