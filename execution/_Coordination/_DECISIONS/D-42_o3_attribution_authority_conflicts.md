# D-42 — O3 Attribution Authority Conflicts

**Status:** RULED — Option O-A (2026-07-15)

**Date prepared:** 2026-07-12

**Prepared by:** agent under the owner's D-41/DEC-074 execution-time
`AUTHORITY_CONFLICT` return condition

## Decision statement

Resolve two O3 attributions that contradict the receiving deliverables'
accepted scope. No implementation/documentation attribution may proceed for
these surfaces until the owner rules.

## Conflict 1 — SURF-011 build-readiness panel

- Proposed O3 target: DEL-09-05 Release quality gate checklist.
- Conflict: DEL-09-05 is checklist/process/CI-criteria scope and explicitly
  excludes implementing GUI workflows. The concrete panel implements a
  Build/Package Readiness GUI and its embedded packet identifies
  `deliverable_id: "DEL-10-04"`, `package_id: "PKG-10"`, `scope_item:
  "SOW-032"`.
- Exact evidence: DEL-09-05 `_CONTEXT.md` and `Specification.md` scope;
  `apps/desktop/src/features/build-readiness/BuildReadinessPanel.tsx` panel
  and packet metadata.

## Conflict 2 — SURF-021 export-unit disclosure helper

- Proposed O3 target: DEL-17-02 Export package, profile, and stable-ID map
  contracts.
- Conflict: DEL-17-02 is explicitly contract-only and excludes executable
  exporters, tests, parsers, harnesses, and GUI features. SURF-021 is an
  executable TypeScript helper consumed by eleven desktop panels. Its unit
  policy may conform to and consume DEL-17-02 without implementation ownership
  residing there.
- Exact evidence: DEL-17-02 `Specification.md` and `Datasheet.md` scope;
  `apps/desktop/src/features/exportUnitDisclosure.ts` and its consumers.

## Options

### O-A — Follow accepted scope and existing implementation identity
(recommended)

1. Attribute SURF-011 to DEL-10-04 as the existing build/package/CI owner.
2. Classify SURF-021 as shared desktop export infrastructure that consumes
   DEL-17-02's common export-unit contract; DEL-17-02 remains the contract
   owner, not the GUI/helper implementation owner.

This resolves both conflicts without expanding a deliverable's accepted scope.

### O-B — Keep both surfaces shared

Classify SURF-011 and SURF-021 as shared product infrastructure, retaining
DEL-10-04 and DEL-17-02 only as relevant contract/evidence homes. This avoids
scope expansion but discards SURF-011's existing explicit DEL-10-04 identity.

### O-C — Expand the originally named receiving scopes

Route DEL-09-05 and DEL-17-02 through governed SCOPE_CHANGE to add these GUI
implementation responsibilities. This is contrary to DEC-074's no-scope-
expansion clarification and therefore requires a separate scope-change act.

### O-D — Leave unmapped

Retain both run-level attribution findings unresolved. This preserves the
conflict but does not close PDU-077.

## Recommendation

Adopt O-A. It follows the live accepted scopes, preserves DEL-17-02's
contract ownership, and respects SURF-011's embedded DEL-10-04 identity.

## On-ruling mechanism

On ruling, codify the selected disposition in the next free `DEC-XXX`, update
the D-42 register row, record the result in the D-41 run basis, seed any
needed receiving-deliverable Remaining item, and execute only the selected
bounded attribution/documentation change. No lifecycle transition or product
behavior change occurs under the attribution ruling itself.

## Human ruling and disposition

**Ruling (2026-07-15, Ryan Tufts, human project authority, in-session
decision slate):** Option **O-A**, approved as recommended. Verbatim slate
ruling over the rows presented in the order D-42, D-38, D-12, D-07b (with
D-06b noted as NOT_PREPARED):

> My rulings:
>
> O-A, O-B, O-B, O-B, we don't need to act on D-06b right now.

Mapping for this row: D-42 → O-A. Recorded by agent at owner direction
(K-AUTH-1; D-GOV-04). Codified as `DEC-076` in
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12; register row D-42 →
RULED. Per the on-ruling mechanism: the disposition is recorded in the D-41
run basis
(`execution/_Reconciliation/DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305/RUN_BASIS.md`),
a Remaining item is seeded on DEL-10-04, and the bounded
attribution/documentation update plus the PDU-077 implementation-surface and
claim-concordance re-extraction execute as ordinary governed loop work citing
`DEC-076`. No lifecycle transition or product behavior change occurs under
this ruling.
