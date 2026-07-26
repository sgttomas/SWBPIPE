---
doc_id: CB-2026-07-25-DEL-08-01-COMPPROV-CROSS-LAYER-001
doc_kind: coordination.candidate_brief
status: candidate_not_adopted
prepared: 2026-07-25
package_id: PKG-08
deliverable_id: DEL-08-01
frozen_sha: 2f8d35ceb30da734ca6dff24dcab36dded8c9b35
decision_basis: DEC-021, DEC-025, DEC-081, DEC-085, DEC-087, DAG-008
---

# CANDIDATE Brief — DEL-08-01 TypeScript-to-Rust Component-Provenance Test

**Status:** `CANDIDATE / NOT ADOPTED / PROPOSAL ONLY / STOPPED`
**Prepared by:** WORKING_ITEMS for HELP_HUMAN
**Package:** `PKG-08`
**Deliverable:** `DEL-08-01`
**Representation:** `SOW_V1`
**Implementation release:** `NONE`

This document analyzes one `DEL-08-01/_STATUS.md ## Remaining` bullet:

> Add the cross-layer TypeScript-to-Rust component-provenance test
> (hardening).

The source-backed design cannot currently be released as a test-only
implementation tranche. The TypeScript builder produces a private,
pending-review payload; the accepted UI boundary withholds that payload from
native IPC; and no accepted transformation produces a lossless,
export-eligible `RenderableReportInput` for the Rust renderer. Inventing such
a payload or passing the raw private payload directly to Rust would evade the
actual boundary this residual is meant to test.

This candidate therefore records a deterministic `STOP / HOLD` and the
minimum evidence required for a future rerun. It does not adopt itself,
authorize implementation, create a ruling, change deliverable state, or
activate owner standing approval. The `.opsproj`
compatibility-window/versioning bullet remains outside this candidate.

## 1. Accepted basis and representation

The future rerun resolves `REPO_ROOT` with
`git rev-parse --show-toplevel` and sets
`WORKING_ROOT={REPO_ROOT}/projects/chirality-piping`.

This candidate is frozen to:

- project basis
  `2f8d35ceb30da734ca6dff24dcab36dded8c9b35`;
- root and project `AGENTS.md`, `agents/AGENT_WORKING_ITEMS.md`, the selected
  committed `loop/WORKPLAN_2026-07-18b_piping_loop.md`, and valid
  `Receipt-70`;
- active, owner-approved `execution/_DAG/DAG-008/`;
- all 14 active `EXECUTION / UPSTREAM` rows for `DEL-08-01` recomputed
  `SATISFIED`;
- the live `DEL-08-01` `ScopeOfWork.md`, `_STATUS.md`, `MEMORY.md`,
  `_CONTEXT.md`, `_REFERENCES.md`, dependency records, and accepted
  `TP-R4-D8-COMPPROVREPORT-001` run record;
- `software-workflow.json` under the ratified
  `docs/SOFTWARE_WORKFLOW_PROFILE.md`.

`DEL-08-01` is `SOW_V1`: `ScopeOfWork.md` declares
`schema: chirality-deliverable-sow/v1`, and no legacy `Datasheet.md`,
`Specification.md`, `Guidance.md`, or `Procedure.md` file remains in the
deliverable directory. This candidate does not migrate or amend the
representation.

## 2. Source-backed current seam and exact gap

The live path has three materially different stages.

### 2.1 Raw TypeScript builder output

`buildRenderableReportInput` in
`apps/desktop/src/features/report/renderableReportInput.ts` owns the
frontend-to-renderer input mapping.

- Lines 73-83 classify session provenance as
  `user_supplied_or_private / private_only / pending /
  private_project_data`.
- Lines 114-187 collect `component.provenance`,
  `geometry.*_reference`, and `modifiers.source_reference`; emit component
  rows into `report_sections.user_supplied_values`; and add component
  provenance records to `provenance_notes`.
- Lines 301-338 preserve the positive and missing-provenance paths.
- Lines 374-380 and 449-450 retain private classification in referenced
  envelopes and the calculation-report provenance.

`apps/desktop/src/features/report/renderedReport.test.tsx:239-392` proves
those structures for invented `component:C-110` and `component:C-999` and
expressly proves that invented user text does not change the private,
private-only, pending classification.

The fixture values may be invented, but the payload's declared boundary is
still private. “Invented” describes test value origin; it does not convert
`private_project_data / private_only / pending` into accepted public-export
metadata.

### 2.2 Accepted pre-IPC control

`controlReportRendererInput` in
`apps/desktop/src/features/report/reportRedactionProjector.ts:15-21` calls
the public-report redaction control with
`requireLosslessMaterialization: true`.

The accepted control in
`apps/desktop/src/features/redaction-controls/redactionExportControls.ts`
classifies private or private-only values for a public report as
`PRIVATE_DATA_REDACTED`. Because any redaction is destructive and the route
requires lossless materialization, the controlled result is `blocked=true`,
`payload=null`, and `materialization_withheld=true`.

`RenderedReportPanel.tsx:67-88` returns before
`renderCalculationReport(...)` when that control blocks.
`renderedReport.test.tsx:424-449` proves the user-local report is blocked
before native IPC and that no hash, save, print, or preview surface appears.

This is the live accepted security boundary. It is not a sanitizing adapter
that returns an export-eligible full `RenderableReportInput`.

### 2.3 Rust renderer boundary

The Tauri command at
`apps/desktop/src-tauri/src/lib.rs:1291-1303` is a thin
`serde_json::from_value::<RenderableReportInput>` plus renderer call. It does
not sanitize or reclassify the payload.

Rust
`core/reporting/report_renderer/tests/render.rs:26-83,103-158` constructs a
separate Rust-native fixture with
`InventedPublicExample / InventedNonEngineeringExample / Accepted`
provenance and proves component provenance appears in hash-bound HTML.
Lines 259-268 prove only that Rust-native input's JSON round trip.

The renderer maps calculation-report provenance into lint provenance and
uses `PublicReportExample` targets in
`core/reporting/report_renderer/src/lib.rs:239-305,844-940`. However, current
private or pending metadata is not itself a demonstrated renderer privacy
block:

- report-generator boundary-gap diagnostics are warning-level;
- protected-content linter pending-review findings are warning-level; and
- the renderer adds only blocking validation diagnostics or blocking lint
  findings to `blocking_reasons`.

Therefore a direct Rust test of the raw private fixture could bypass the
accepted pre-IPC control and may render bytes without proving export
eligibility. Such a test would not close the residual faithfully.

### 2.4 Exact residual

The missing evidence is not merely common JSON syntax. It is a lawful,
source-backed chain:

```text
raw private builder output
  -> accepted pre-IPC decision
  -> accepted lossless export-eligible RenderableReportInput
  -> Rust deserialize/render
```

The first two stages exist and intentionally end in `blocked/null`. The third
stage does not exist. Consequently the fourth stage cannot be bound to an
export-eligible TypeScript-produced payload without inventing a transformation
or bypassing the accepted control.

## 3. Smallest source-faithful future design

A future amended candidate must keep two lanes distinct.

### Lane A — raw private boundary

1. Commit one invented raw fixture equal to the full deterministic output of
   the existing TypeScript builder case.
2. Prove deep equality between builder output and that fixture.
3. Prove `controlReportRendererInput(raw)` returns `blocked=true`,
   `payload=null`, `materialization_withheld=true`, and at least one
   `PRIVATE_DATA_REDACTED` decision.
4. Prove the native renderer is not invoked.

### Lane B — export-eligible Rust boundary

1. Invoke a live, accepted TypeScript transformation whose contract explicitly
   produces a lossless, export-eligible `RenderableReportInput`.
2. Prove that output is distinct from the raw private payload and retains the
   required positive and missing component-provenance evidence.
3. Commit or otherwise bind the exact transformed bytes without a second
   hand-built fixture.
4. Deserialize those exact bytes as Rust `RenderableReportInput`, render
   twice, and assert deterministic hash-bound HTML and the same component
   evidence.

Lane B is mandatory. Lane A alone only strengthens existing same-layer
redaction coverage and does not close the named TypeScript-to-Rust residual.

No live accepted Lane B transformation was found at the frozen basis.
Therefore this design stops before implementation.

## 4. Write fence — withheld

There is no adoption-ready implementation write fence.

The former three-path proposal is withdrawn because it coupled one raw private
fixture to an unblocked/public Rust assertion:

- `fixtures/reports/invented/component_provenance_renderable_input.json`;
- `apps/desktop/src/features/report/renderedReport.test.tsx`;
- `core/reporting/report_renderer/tests/render.rs`.

A future rerun may use those paths, but only after it names the accepted Lane
B producer and computes the complete exact fence, including any distinct
eligible fixture or transformation test surface. This candidate does not
pre-authorize those paths, an evidence sweep artifact, production changes, or
any other implementation write.

Concurrent implementation ownership, generated evidence, run records,
deliverable-state updates, receipts, commits, pushes, or PRs are likewise not
released.

## 5. Exclusions and boundaries

This candidate must not be used to:

- hand-edit private metadata into
  `InventedPublicExample / InventedNonEngineeringExample / Accepted`;
- call a manually relabeled payload “sanitized” or “export eligible”;
- send the raw private builder payload directly to Rust and treat successful
  deserialization or rendering as proof that the public boundary passed;
- weaken or bypass `controlReportRendererInput`,
  `requireLosslessMaterialization`, or the pre-IPC stop;
- change production TypeScript, Rust, Tauri, renderer, report-section,
  redaction, persistence, report-package, PDF, schema, or API behavior;
- change a manifest, lockfile, dependency, toolchain profile, report member
  vocabulary, JSON/serde spelling, hash, gate, canonicalization,
  professional-boundary behavior, or component semantics;
- create a generator script, run Node from Cargo, run Cargo from Vitest,
  generate a tracked fixture during a test, or depend on test order;
- use real project data, private/protected standards content, proprietary
  catalog data, code-derived factors, or network access;
- edit another deliverable, package, DAG, decomposition, ruling, register,
  receipt, project instruction, lifecycle, release, or publication surface;
- address the `.opsproj` compatibility-window/versioning residual.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 6. Acceptance criteria for a future amended candidate

These criteria define the minimum future design. They are not implementation
authority.

### AC-01 — Exact raw TypeScript binding

The invented TypeScript case builds the full raw payload and compares it to a
single committed raw fixture with deep equality. Equality includes hashes,
report sections, enums, references, result rows, professional-boundary fields,
and every `private_project_data / private_only / pending` classification.
Selective snapshots are not a substitute.

### AC-02 — Raw boundary remains fail-closed

The exact raw payload is passed to `controlReportRendererInput`. The test
proves `blocked=true`, `payload=null`,
`summary.materialization_withheld=true`, a
`PRIVATE_DATA_REDACTED` decision/finding, no native invocation, and no
renderer output surface. The raw fixture is never called public or
export-eligible.

### AC-03 — Accepted transformation prerequisite

The amended candidate cites a live accepted transformation and its contract,
owner, input/output classification semantics, and exact source location. The
transformation must produce a lossless export-eligible
`RenderableReportInput`; a test helper, hand-edited second fixture, metadata
replacement, or renderer bypass does not satisfy this criterion.

If AC-03 is absent, the future candidate stops and no implementation is
released.

### AC-04 — Exact eligible-byte Rust binding

Only the output of the AC-03 transformation may cross the test boundary.
TypeScript binds its exact bytes; Rust deserializes those same bytes directly
as `RenderableReportInput` without repair or a parallel hand-built fixture.
Two Rust renders have identical HTML and SHA-256, the hash is 64 lowercase
hexadecimal characters, the document is scriptless with no external
references, and the renderer outcome matches the accepted transformation
contract.

### AC-05 — Positive component provenance survives both lanes

Both the raw and accepted eligible representations retain at least:

- `component-provenance:component:C-110`;
- category `component_provenance:bend`;
- source `component:component:C-110`;
- source name `Invented elbow marker component provenance`;
- `component.provenance=invented_example_user_entered_bend_values_no_code_table`;
- `geometry.bend_geometry_source_reference=invented_user_entered_preview_geometry`;
- `modifiers.source_reference=invented_user_entered_preview_no_code_table`;
- `missing_data_finding=false`.

Rust HTML proves the eligible representation retains the same evidence.

### AC-06 — Missing-provenance path survives both lanes

Both representations, and the Rust HTML for the eligible representation,
retain:

- `component-provenance:component:C-999`;
- category `component_provenance:rigid`;
- `missing_data_finding=true`;
- diagnostic code `COMPONENT_PROVENANCE_MISSING`;
- diagnostic class `PROVENANCE_WARNING`;
- affected component `component:C-999`;
- visible fallback `component provenance missing`.

The transformation must not relabel missing provenance as accepted input or
silently omit it.

### AC-07 — Exact containment

The amended candidate computes an exact, complete, disjoint write fence from
the accepted transformation seam. It separately names any authorized evidence
artifact. No production, schema, lifecycle, release, professional-claim, or
unrelated project state change may be hidden inside a test tranche.

## 7. Conditional checks and execution prerequisites

No checks are released now because no implementation is released.

If a future amended candidate retains the former desktop and renderer test
surfaces, deterministic profile selection must include at least:

| Check ID | Applicable reason |
|---|---|
| `desktop-test` | TypeScript boundary and fixture binding |
| `desktop-build` | TypeScript/build integrity |
| `piping-pytest` | `core/**` renderer-test change under the project profile |
| `evidence-sweep` | DEC-025 Cargo and desktop surfaces |
| `harness-self-check` | Always-check in the project profile |

The future executor must use
`tools/software_workflow/run_registered_checks.py`, isolate normalized runner
output unless a tracked evidence target is separately authorized, and run:

```text
cargo fmt --manifest-path core/reporting/report_renderer/Cargo.toml -- --check
git diff --check
```

The final code-touching branch also requires the plan's full DEC-025
five-surface sweep before push eligibility. A green sweep is development
evidence only.

At candidate preparation time the checkout has Cargo/Rust and the
`wasm32-unknown-unknown` target, but lacks the project-local Vitest,
Playwright, TypeScript, and Vite binaries and lacks `wasm-bindgen`. No install
or network access is authorized. This toolchain hold is secondary to the
missing accepted transformation: provisioning tools alone does not release
implementation.

## 8. DEC-085 / DEC-087 analysis — STOP, no activation

- **Ontology:** raw private builder output and export-eligible renderer input
  are distinct boundary objects unless an accepted transformation proves
  their relationship.
- **Epistemology:** identical raw bytes across TypeScript and Rust would prove
  serde compatibility but would not prove that the public-report control
  authorized those bytes.
- **Praxeology:** the existing source can prove Lane A; it cannot execute Lane
  B without a new accepted semantic act.
- **Axiology:** stopping is preferable to weakening the pre-IPC boundary,
  inventing clearance metadata, or turning renderer acceptance into a false
  export-eligibility claim.

Material alternatives rejected:

1. **Raw fixture rendered directly by Rust:** bypasses the accepted pre-IPC
   stop and conflates renderer capability with export authorization.
2. **Hand-built sanitized fixture:** has no accepted producer and can drift
   independently, preserving the original evidence gap.
3. **Changing builder classifications:** is a production/security semantic
   change and contradicts the accepted user-local posture.
4. **Changing the renderer or linter to force a block:** is production scope,
   not the named test-only hardening residual.
5. **Editing Tauri:** the command is a thin serde/renderer shell and contains
   no accepted transformation.
6. **Cross-running Node and Cargo or runtime fixture generation:** adds
   orchestration/dependency pressure and does not create an accepted
   classification transform.
7. **Lane A only:** useful same-layer coverage, but does not close the
   TypeScript-to-Rust residual.

No `CLASSIFY_ELIGIBLE`, `ACTIVATE_OWNER_STANDING_APPROVAL`, adoption, or
`OwnerCaseSelection` effect is recorded. The correct present disposition is
`STOP / HOLD_FOR_ACCEPTED_TRANSFORMATION`.

## 9. Adoption and rerun gate

This exact candidate is not adoption-ready for implementation. Only the human
project authority may decide whether to:

1. authorize a separately scoped design/change tranche for an explicit
   export-eligible transformation;
2. redefine the residual as raw serde-compatibility evidence, with the
   security and claim implications stated expressly; or
3. leave the residual held.

After an accepted transformation exists, WORKING_ITEMS must rerun source
discovery and issue a new or amended candidate that names:

- the new frozen execution SHA and accepted transformation basis;
- separate raw and eligible objects;
- the complete exact write fence;
- one serialized implementation owner;
- the full acceptance criteria and registered checks;
- the toolchain-preflight hold;
- fresh independent final-byte refutation;
- separate authority for state, receipt, Git, push, PR, or merge action.

Until that human act and rerun occur, no implementation node is released.

## 10. Stop and invalidation triggers

Stop before writing or continuing if:

- no accepted AC-03 transformation exists;
- a proposed eligible payload is hand-edited, test-only, manually relabeled,
  or otherwise lacks a live accepted producer;
- raw private bytes would reach Rust by bypassing the accepted pre-IPC
  control;
- private/pending data would be called public, sanitized, cleared, or
  export-eligible without accepted evidence;
- a production, schema, Tauri, renderer, linter, redaction, manifest,
  lockfile, public-export inventory, deliverable-state, or other path appears
  necessary;
- real private/protected data or an external/network source appears necessary;
- the execution SHA, exact fence, assertions, checks, acceptance criteria,
  scope, risk, lifecycle, release, publication, or
  professional-accountability meaning changes;
- the offline prerequisite preflight fails or tries to install/download.

Any change to these candidate bytes requires a new hash and fresh independent
final-byte verification. Any future implementation change after a check
requires rerunning every affected focused check and all registered terminal
checks.

## 11. Effects and notices

This candidate has no implementation, dependency, lifecycle, stage, issuance,
release, publication, professional-acceptance, evidence-sweep, Git, push, PR,
or merge effect.

No cross-package notice is required for this proposal-only stop.
`DEL-08-03` report-section semantics are consumed read-only. If a future
design needs another package's surface, the manager reports that dependency
upward and waits for an amended, human-authorized scope; it does not widen
this candidate.
