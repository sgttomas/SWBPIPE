---
doc_id: TDP-2026-07-25-DEL-08-01-RENDERABLE-ELIGIBILITY-001
doc_kind: coordination.transformation_design_proposal
status: proposal_only_owner_selection_required
prepared: 2026-07-25
package_id: PKG-08
deliverable_id: DEL-08-01
accepted_candidate_sha256: 030c5f7821ac93ce71f64f8b48fbfcf454d858dbd27faf49c22dd1c641c12229
---

# Proposal — lawful Lane B `RenderableReportInput` eligibility

**Status:** `PROPOSAL ONLY / OWNER SELECTION REQUIRED / NO IMPLEMENTATION`
**Objective:** determine whether and how a lossless, export-eligible
`RenderableReportInput` could lawfully exist between the raw private
TypeScript payload and the Rust renderer.

## 1. Result

The answer depends on the meaning of “export-eligible”:

- **Strict stopped-candidate answer:** no live accepted transformation produces
  a distinct, standalone, lossless public/export-eligible
  `RenderableReportInput` for `DREP-IPC-003`.
- **Broader accepted-path answer:** R16 already carries the exact raw
  `buildRenderableReportInput` object as the nested `report` member of a
  losslessly controlled local-private package request, then Rust deserializes
  and renders that member. This is an accepted lossless TypeScript-to-Rust
  path, but the report remains private and the package route expressly creates
  no public/releasable claim.

The repository has:

1. a TypeScript builder that intentionally emits a private,
   private-only, pending-review report input;
2. a public-report control that redacts private values and withholds the
   payload whenever lossless materialization would be destroyed;
3. a local-private package route that may retain private values after explicit
   user intent, but does not make them public or export-eligible; and
4. a Rust renderer that accepts a deserialized `RenderableReportInput` after
   the TypeScript service boundary, but does not create eligibility.

It does not have an authority-bearing review decision, promotion contract, or
public-origin builder that can lawfully supply a **public** Lane B. The
remaining gap is normative, not a missing helper function: the owner must
decide whether local-private package eligibility can close the residual or
whether Lane B must remain standalone/public.

The strict raw-to-eligible chain is possible only if the owner first defines
who may make an eligibility decision, what evidence it binds, and what
“lossless” means. Until then, the stopped candidate remains stopped.

## 2. Accepted authority already present

### 2.1 Public/private requirements

`docs/IP_AND_DATA_BOUNDARY.md:31-35` leaves the final contributor mechanism,
maintainer roster/quorum, legal-review authority, and human-acceptance
workflow `TBD`. Lines 37-45 allow invented or permissively licensed public
data with provenance. Lines 58-72 require public-data provenance and a review
disposition. Lines 86-90 default reports and examples to excluding private
project/material/component/rule-pack data unless a user intentionally exports
or contributes it with documented redistribution rights.

The accepted DEL-12-02 contract adds:

- shared/public exports must not silently include private values and must
  redact or block them
  (`DEL-12-02/ScopeOfWork.md:48-74`);
- runtime integration, legal review, and approval choices remain `TBD`
  (`:57-59,81-95`);
- unknown or insufficient redistribution evidence must warn, redact, or block
  rather than silently include (`:184-198`, especially REXC-REQ-002,
  REXC-REQ-005, and REXC-REQ-012);
- redaction operates on an export representation and must not mutate the
  source (`REXC-REQ-007`);
- safe reproducibility metadata should survive where safe
  (`REXC-REQ-008`); and
- later runtime integration must preserve provenance/privacy status and
  exercise no-bypass tests (`:328-342`).

These are constraints on a future design. They do not supply the missing
eligibility authority.

### 2.2 Report and renderer requirements

DEL-08-01 requires auditable provenance, missing-data findings, deterministic
output, public protected-content controls, and application-service no-bypass
behavior. Its current residual remains the cross-layer component-provenance
hardening test; `_STATUS.md` remains `IN_PROGRESS`.

DEC-021 establishes deterministic, self-contained, scriptless, hash-bound
Rust-rendered HTML. It does not decide whether an input is public or private.

R18 accepts the stopped candidate only and authorizes this proposal. It
expressly prohibits implementation.

## 3. Observable behavior

### 3.1 Lane A — raw/private

`apps/desktop/src/features/report/renderableReportInput.ts:73-83` assigns
`user_supplied_or_private / private_only / pending /
private_project_data` to the session. Component records retain the same
classification at `:114-167`; missing component provenance becomes an
explicit warning at `:172-187`; and report/envelope metadata remains private
at `:374-380,449-450`.

`renderedReport.test.tsx:239-392` verifies the positive and missing component
paths and proves that user-controlled “invented” or “cleared” text cannot
promote classification.

### 3.2 Accepted pre-IPC control

`reportRedactionProjector.ts:15-21` routes renderer input through
`DREP-IPC-003 / public_report` with
`requireLosslessMaterialization=true`.

`redactionExportControls.ts:118-132,324-385` classifies private/private-only
records as `PRIVATE_DATA_REDACTED` on public contexts.
At `:851-885`, any redact/omit/block decision is destructive; a lossless route
therefore returns `blocked=true`, `payload=null`, and
`materialization_withheld=true`.

`RenderedReportPanel.tsx:67-88` stops before rendering when that result blocks.
`renderedReport.test.tsx:424-452` proves no IPC, hash, save, print, or preview
surface occurs.

`services/reportRenderService.ts:47-70` independently refuses anything that is
not a non-blocked `DREP-IPC-003` controlled result. This is an accepted
no-bypass boundary, not a transformer.

### 3.3 Accepted local-private report-package analogue

`reportRedactionProjector.ts:23-32` exposes
`DREP-PACKAGE-SAVE-009 / local_private`. With explicit UI-owned intent,
`reportRedactionProjector.test.ts:20-39` proves private bytes may be retained
with warning.

The path continues across the language boundary:

- `reportPackageRequest.ts:120-156,310-348` places the exact raw builder result
  in `request.report`;
- `App.tsx:963-986` sends only the non-blocked controlled package request to
  native save;
- `core/reporting/report_package/src/wire.rs:656-690` selects
  `request.report`, deserializes it as `RenderableReportInput`, and passes it
  to the package assembler; and
- `report_package/src/lib.rs:100-123,194-197,277-309` renders the HTML/PDF
  members from that input.

R16's adopted route matrix identifies the route as local-private,
UI-intent-controlled, source-nonmutating, and explicitly excludes any
public/releasable claim
(`R16/ROUTE_MATRIX.csv`, row `R16-R02`). Its accepted run record confirms
current-private copies and leaves this cross-layer hardening residual open
(`WORKING_ITEMS_RUN_2026-07-22_DEL0801_REPORT_PACKAGE_R16.md:20-37,61-70`).

This is accepted, lossless, and cross-layer. It is non-equivalent to the
stopped candidate's strict Lane B because the controlled TypeScript object is
the larger package request, the nested report remains raw/private, and there
is no distinct standalone eligible report payload.

### 3.4 Rust boundary

`apps/desktop/src-tauri/src/lib.rs:1291-1303` only deserializes
`RenderableReportInput`, renders it, and serializes the outcome.

`report_renderer/src/lib.rs:844-940` validates and lints public report
surfaces, but adds only blocking diagnostics/findings to
`blocking_reasons`. Report-generator private/pending boundary gaps are
warning-level (`report_generator/src/lib.rs:142-156,403-419,705-718`).

The existing unblocked Rust fixture uses
`InventedPublicExample / InventedNonEngineeringExample / Accepted`
provenance (`report_renderer/tests/render.rs:13-40`) and is independently
constructed (`:43-118`). Its JSON round trip (`:259-268`) proves Rust
serialization only, not TypeScript eligibility.

## 4. Missing contract

The missing contract must answer all of the following before a transformer can
exist:

- whether “export eligible” means public/shared only or may include explicit
  local-private package eligibility;
- whether a nested, controlled package `report` member qualifies as a
  TypeScript-produced Lane B object;
- whether Lane B must be distinct from raw bytes or eligibility may be a route
  decision over unchanged private bytes;
- the intended export context and eligibility vocabulary;
- the actor legally/governance-authorized to decide that exact data may be
  public;
- the evidence and immutable input hash that decision binds;
- whether value-preserving promotion from private to public is allowed at all;
- the required provenance, license, contributor certification, review status,
  and redistribution status for every promoted record;
- the meaning of “lossless” across changed metadata and unchanged semantic
  values;
- treatment of unlisted, unknown, private-only, pending,
  protected-suspected, and quarantined fields;
- audit/receipt schema, persistence, revocation, versioning, and review
  ownership;
- how a deliberately missing component-provenance negative case remains
  visible without pretending that missing domain provenance is accepted
  source provenance; and
- whether the Rust boundary must independently reject private-classified input
  as defense in depth.

These are new normative and acceptance decisions. Source behavior cannot
answer them.

## 5. Options

### O-0 — Redefine Lane B as the accepted R16 local-private package path

Bind the exact controlled package request's nested `report` member to Rust
wire deserialization and package rendering. Preserve
`private_project_data / private_only / pending`, component C-110, the C-999
missing-provenance warning, UI-owned explicit intent, source non-mutation, and
the no-public-claim posture.

**Benefit:** reuses accepted behavior; proves the actual lossless private
workflow; requires no public promotion, sanitizer, or automatic relabeling.

**Cost/risk:** changes the stopped candidate's ontology and closure criterion.
Lane B becomes local-private package eligibility, not a distinct standalone
public renderer input. The future fence must include package control and Rust
wire/package tests, not the former three paths.

### O-1 — Authority-bearing, fail-closed eligibility promotion

Create a governed eligibility decision outside the raw payload. The decision
would bind:

- the canonical raw-input hash;
- the exact authorized reviewer and authority basis;
- source/license/rights evidence;
- allowed output context and field/path scope;
- accepted public provenance for each promoted record;
- decision version, expiry/revocation posture, and audit reference.

A pure deterministic transformer would consume the immutable raw payload plus
that decision. It would never inspect words such as “invented” or “cleared”
to infer eligibility, never mutate the raw object, and never blanket-replace
metadata. It would either:

- emit a complete eligible `RenderableReportInput` plus a path-level
  derivation manifest; or
- fail closed without a payload.

The output must then pass `controlReportRendererInput` with zero destructive
decisions, `materialization_withheld=false`, and payload equality. Only that
controlled payload may cross to Rust.

**Benefit:** satisfies the strict raw → accepted decision → eligible bytes →
Rust chain while retaining auditability.

**Cost/risk:** creates the largest new normative surface: review authority,
decision schema, reclassification doctrine, persistence/revocation, runtime
integration, and cross-package ownership. A metadata-only “promotion” without
the external decision would violate the no-relabel rule.

### O-2 — Separately originated public-example Lane B

Author Lane B from an accepted invented/public source before the private
session builder. A public-example builder would require explicit accepted
public provenance as input and produce the exact bytes consumed by Rust. Lane
A would continue to prove the raw private builder and pre-IPC block.

**Benefit:** strongest separation of private runtime data from committed
public fixtures; smaller rights/revocation surface; no private-to-public
promotion.

**Cost/risk:** it is not a transformation of the raw Lane A payload. Selecting
it requires the owner to redefine the residual as “two-origin cross-layer
contract coverage” rather than the strict raw-to-eligible chain. A
separately hand-built Rust fixture remains prohibited.

### O-3 — Redacted public render representation

Use the existing redaction decision model to remove or replace private values
and render a new public representation.

**Benefit:** follows existing public-export redaction direction and preserves
safe metadata where possible.

**Cost/risk:** the current public route is intentionally destructive and
withholds lossless materialization. Redacted output is not the same full
`RenderableReportInput`, may not satisfy its semantic validators, and cannot
prove component-value preservation. This option requires a new render-view
contract and changes the residual.

### O-4 — Explicit local-private renderer route

Allow the exact private payload to render locally after explicit user intent,
analogous to the local-private package route.

**Benefit:** can be value-lossless and useful for local project review.

**Cost/risk:** it is private, not public/export-eligible. It requires a new
renderer route and privacy posture and does not satisfy Lane B.

### O-5 — Raw bypass or automatic relabel

Send raw bytes directly to Tauri/Rust, or replace
`private_project_data / private_only / pending` with public/accepted labels
inside a helper.

**Disposition:** reject. It bypasses accepted controls, lacks authority, and
would turn renderer acceptance into a false eligibility claim.

## 6. Exact owner decisions required

| ID | Decision required | Why existing authority is insufficient |
|---|---|---|
| D1 | Is Lane B `public_report`, `public_example`, or `local_private`? | Those contexts have different allowed actions and legal meanings. |
| D2 | Select O-0, O-1, O-2, O-3, O-4, or retain STOP. | The options change the residual's ontology and implementation class. |
| D3 | If O-1, may an authority-bearing decision promote unchanged values, or is any private-to-public promotion prohibited? | Current policy permits intentional export with documented rights but defines no promotion mechanism. |
| D4 | Who may issue and revoke eligibility: owner, designated maintainer, qualified legal reviewer, or a quorum? | Maintainer/legal/human-acceptance authority remains `TBD`. |
| D5 | Define “lossless”: exact semantic values, exact non-classification fields, exact serialized bytes, or another invariant. | A classification/provenance change necessarily changes bytes. |
| D6 | Define decision granularity and mandatory evidence for every field/record. | Blanket payload-level clearance conflicts with explicit-metadata-only and no-inference rules. |
| D7 | Decide whether the C-999 negative case may be public test data while retaining `missing_data_finding=true`, and define its fixture-source provenance separately from the deliberately missing component-domain provenance. | Current source conflates session provenance with component-domain absence. |
| D8 | Select the authority artifact's schema/home, persistence, versioning, expiry/revocation, and owning package. | No accepted eligibility-decision contract exists. |
| D9 | Decide whether Rust must independently block private-classified input even after TypeScript no-bypass controls. | Current renderer treats private/pending boundary gaps as warnings. |
| D10 | State the revised residual and closure criterion. | O-2/O-3/O-4 do not satisfy the strict stopped-candidate chain unchanged. |

No agent recommendation can enact these decisions.

## 7. Proposed acceptance contract if O-1 is selected

The following is a proposal for a later governed candidate, not adopted
criteria:

1. The transformer accepts only an immutable raw
   `RenderableReportInput` and a separately authenticated eligibility decision
   bound to its RFC 8785/JCS SHA-256.
2. It is pure, deterministic, non-mutating, offline, and performs no
   value-text inference.
3. Every output field is either byte/semantic-value identical to its named raw
   source or an explicit decision-supplied provenance/classification field;
   a derivation manifest records the mapping.
4. Unlisted, unknown, private-only, pending, rejected, quarantined,
   protected-suspected, expired, revoked, hash-mismatched, or incomplete
   evidence causes `STOP / NO PAYLOAD`.
5. No protected content, real secret, or third-party right is promotable by
   owner assertion alone.
6. The raw object and its hash remain unchanged and retained as Lane A
   evidence.
7. The output contains only the owner-approved eligibility vocabulary and
   remains subject to protected-content, professional-boundary, and
   report-validation gates.
8. C-110 evidence is preserved exactly. C-999 remains an explicit missing-data
   finding, with public fixture-origin provenance distinct from the missing
   component-domain provenance.
9. `controlReportRendererInput(output)` produces a non-blocked
   `DREP-IPC-003` result, zero destructive decisions,
   `materialization_withheld=false`, and an exactly equal payload.
10. TypeScript binds the exact eligible bytes; Rust deserializes those bytes
    without repair, renders twice deterministically, and proves the positive
    and missing component paths.
11. A clean linter/render is evidence only, never legal clearance,
    professional approval, or release readiness.

## 8. Future candidate fence implications

No exact implementation fence can be released before D1-D10 are answered.

If O-1 is selected, a future impact assessment will likely need to consider,
without pre-authorizing:

- an eligibility-decision contract/schema and its tests;
- a TypeScript transformation/service boundary and audit manifest;
- redaction-control integration proving zero destructive decisions;
- separate raw and eligible exact-byte fixtures;
- the existing TypeScript rendered-report test;
- the Rust renderer integration test; and
- cross-package DEL-12-02 ownership for privacy/approval semantics.

That is no longer the former three-path test-only tranche. Production and
schema surfaces may be necessary, so exact paths, one integration owner,
dependencies, checks, and lifecycle effects must be recomputed.

O-0 instead requires a newly computed fence covering the TypeScript
raw/request/control binding and Rust report-package wire/render verification.
It must retain private classifications and exclude all public/releasable
claims. The former fixture/UI/standalone-renderer fence is insufficient
because it omits the package-control and wire owners.

O-2 may avoid a promotion decision but still needs a public-origin contract
and exact TypeScript/Rust byte binding. O-3 needs a distinct redacted render
schema/validator. O-4 needs a local-private renderer route and cannot be called
Lane B public eligibility.

## 9. Non-binding recommendation

Recommend **O-0 if the residual's intended purpose is cross-layer preservation
and rendering hardening for the actual accepted private workflow**. It is the
smallest source-backed option and introduces no classification transition.
The owner must expressly redefine Lane B as
`local-private package eligibility`, preserve every private/pending label, and
prohibit any public/releasable claim.

If the intended purpose is specifically standalone **public** rendering, do
not use O-0 as semantic closure. Keep the residual held and select O-1 only as
a governed, authority-bearing promotion, never a code-local sanitizer. O-1 is
the only option that can satisfy the strict public chain, but requires all
D1-D10 decisions and a cross-package impact assessment. O-2 remains the safer
alternative for public invented repository fixtures if the owner also
redefines the chain as two-origin coverage.

Do not select O-3 or O-4 as unchanged closure of the current residual, and
never select O-5.

## 10. On-selection next mechanism

1. The human owner records D1-D10 and the selected option in a SHA-bound
   decision record.
2. HELP_HUMAN routes a cross-package impact assessment to SCOPE_CHANGE because
   the selection may add normative criteria, product behavior, and DEL-12-02
   ownership beyond PKG-08.
3. The owning manager recomputes accepted dependencies and issues a new
   proposal/candidate with an exact write fence, acceptance criteria, checks,
   stop gates, one integration owner, and fresh independent refutation.
4. The human separately adopts that exact future candidate.
5. Only a later implementation release may authorize code or test work.

Until all five steps occur, disposition remains
`HOLD_FOR_OWNER_SELECTION / NO_IMPLEMENTATION`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
