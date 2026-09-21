# DEL-12-03 — R0 calibration notes

Worker: TASK (Type 2), run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`.
Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`. Forward ledger sealed
at SHA-256 `d60de270c56108e88ab7cb22496ddf612067274fd9d54b1de688224389f1d7ed`
(92 rows: 76 issued keys plus 16 `.sNN` sub-claims). Reverse answers cover all
96 pilot capabilities. These are agent dispositions and not owner rulings.
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/`.
- The SOW's short forms such as `core/security/telemetry_policy/` and
  `apps/desktop/...` resolve under `projects/chirality-piping/`. Evidence
  columns use the full repository-relative form.
- The "four-document kit" (`Datasheet.md`, `Specification.md`, `Guidance.md`,
  `Procedure.md`), named in CLM-015/021/022 and in MEMORY, maps to
  `D/ScopeOfWork.md`. Commit `bd97c4a41` (PR #233, migration authority
  D-GOV-16) replaced the four documents with it. The provenance record is the root
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/candidates/W_P3/PIP-PKG12/DEL-12-03/finalization.json`
  (every block `PRESERVED`).
- "Desktop seam" = `apps/desktop/src/services/telemetryPolicyService.ts`.
  "Panel" = `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx`.
- The App interaction evidence is the single case `App.test.tsx::renders the engineering workspace from invented local fixtures`.
  It holds the telemetry assertions at about lines 3990–4075.

## Judgment calls

- **Rename retained (CLM-010, CLM-018, CLM-024).** I marked these
  `ACCEPTED_DIVERGENCE` (`RENAME_OR_IDENTITY`). DEC-101's obligations say that
  "other governance documents take the name at their next amendment". Whether
  a deliverable SOW is such a document is an owner reading. If it is not, these
  rows become `IMPLEMENTED_DIFFERENTLY`/`DOC_BEHIND_CODE`. Their behavioural
  content aligns in full.
- **Stricter code (CLM-005.s05).** The text says a forbidden field is "excluded".
  The code rejects the whole attempt instead. I marked this
  `IMPLEMENTED_DIFFERENTLY` and flagged that it is not a privacy weakening.
- **Blocks with items but distinct prose (CLM-016, CLM-022).** Each has one
  child item (AC-001 / VER-001), but the block prose carries claims the item
  does not. I assessed these as blocks rather than `CONTAINER`.
- **Sub-claims.** I split CLM-005 (5 rows) and CLM-021 (11 rows) because they
  contain aligned and divergent rows side by side. I left CLM-004, CLM-020 and
  CLM-026 whole, since they have a single dominant disposition.
- **Security-invariant rows (TEL-REQ-009, TEL-REQ-010).** I used
  `PARTIALLY_IMPLEMENTED` with `INVARIANT` tier rather than
  `VERIFIED_NOT_VALIDATED`. The gap is missing verification scope (no
  product-level test, no consumer routing), not a missing validation basis.
  OPS-K-PRIV-2 also names "security review", and I found no security review
  record for it. Human dispositions RF-001 and RF-002 are still `TBD`.
- **TEL-REQ-008.** I read "project diagnostic envelope when available" as the
  `Diagnostic` definition in `schemas/model.schema.yaml`. That definition
  exists and the helper does not conform to it, so the row is
  `PARTIALLY_IMPLEMENTED`. Under a narrower reading ("available" = wired into
  project diagnostics) the row would be `ALIGNED`.
- **Excluded source.** DEC-074 O7 is adopted by reference and its text lives in
  a `PROPOSED_*` file this brief excludes. I did not read it. The affected rows
  are flagged `ADOPTED_BY_REFERENCE` in Notes.
- **SourceReliability.** Requirement and acceptance rows that cite agent-run
  test evidence are `UNVERIFIED`. Declared-state, context, history, container
  and non-normative rows are `NOT_APPLICABLE`. No row is `REVIEWED`, because no
  human disposition covers the cited records.
- **Selectability.** `NOT_APPLICABLE` on every row. Since 2026-09-19 Piping
  selects work through owner-steered work graphs, not `## Remaining`.

## Convention friction

1. **Gate evidence cannot go in evidence columns.** The validator
   path-checks every evidence token against the frozen tree.
   `GATE_EVIDENCE/**` and the root W-P3 records either post-date the freeze or
   sit outside the Piping tree, so A6's suite-level pass status ended up in
   `ContextRefs`. *Fix:* allow a `GATE:` token prefix, or a separate
   `GateEvidence` column that is exempt from the frozen-tree check.
2. **No divergence layer for record drift.** Ten rows are pure
   deliverable-text staleness, and none of the six layers fits, so I recorded
   `NONE` beside a non-aligned disposition. That reads oddly. *Fix:* add a
   `RECORD` (documentation or declared-state) layer, or state that `NONE` is
   correct for record drift.
3. **Minor metadata drift has no disposition.** `_STATUS.md` "Last Updated"
   predates its own latest history entry. `ALIGNED` understates it, and every
   stale disposition overstates it. *Fix:* allow `ALIGNED` with a mandatory
   `Notes` observation, or add a `COSMETIC_DRIFT` cause usable with `ALIGNED`.
4. **Migration-preserved text.** Under the SOW standard, preserved legacy
   blockquotes are "source context", and ID-shaped text inside them is not a
   local definition. The extractor still issued `ITEM` keys for the quoted
   TEL-REQ and TEL-TEST rows and treated the blocks as live claims. The
   conventions do not say whether a `PRESERVED` block is a current claim or
   provenance. *Fix:* add a C1 rule, for example "a `PRESERVED` block is
   assessed as a current claim; the migration record goes in `ContextRefs`",
   or have the extractor mark quoted items.
5. **ACCEPTED_DIVERGENCE for staged rulings.** DEC-101 permits a lag ("at next
   amendment") rather than a divergent end state. C6 does not say whether a
   permitted lag counts. *Fix:* state that a ruling which schedules a catch-up
   supports `ACCEPTED_DIVERGENCE` until the named trigger occurs.
6. **BaselineClass on non-aligned rows.** Part D says it is empty on quiet rows
   but does not say what to write otherwise. I used `NONE`. *Fix:* make
   `NONE` explicit as the default for non-aligned rows.
7. **Container pointer.** The output matrix ties OUT-001 to CLM-009, which is a
   bare heading (`NON_NORMATIVE`). The conventions do not say how to treat a
   pointer to a non-normative unit. It is minor, but it recurs across
   migrated SOWs.

## Unit-grain observations

- The grain is about right for requirements: TEL-REQ and TEL-TEST items map
  one-to-one onto testable behaviour.
- It is too coarse for the method tables. CLM-021 (11 checks) and CLM-005
  (5 conditions) needed sub-claims. Table rows in migrated SOWs are natural
  units, and the extractor could issue them directly.
- It is too fine for headings. Four CLM heading blocks (001/009/017/023) and
  five section containers produce nine structural rows with no signal.
- The CONTEXT surface yields 12 units, 9 of them trivially aligned register
  echoes. A single SURFACE row plus the revision-pin rows would lose nothing.

## UNKNOWN rows

None. The closest to UNKNOWN are the following; each check is the smallest that
would settle it:

- **TEL-REQ-009 / R01 (DEL-10-02 clause).** Read DEC-074 O7 option A text, and
  the DEL-10-02 declaration-admission gate, to confirm whether consumer
  interception is deferred by ruling. If it is, TEL-REQ-009 moves to
  `DEFERRED_BY_RULING`.
- **TEL-TEST-007.** Confirm whether `tools/validation/validate_claims_language.py`
  scopes `docs/security/*.md`.
- **CONTEXT architecture basis.** Confirm the frozen PKG-00 lifecycle state.

## Did the reverse pass change my view?

Yes, on one point. CAP-ADAPT-001, CAP-ADAPT-006 and CAP-ADAPT-008 show that
adapter declarations and plugin manifests are blocked unless
`telemetry_allowed` is false (`ADAPTER_TELEMETRY_MUST_BE_DISABLED` in
`core/adapters/framework/adapter_framework.py`; `_verify_privacy` in
`plugin_verification.py`). That is a declaration-level no-bypass control for
TEL-REQ-009, and the sealed row does not cite it.

- **What still holds:** the disposition `PARTIALLY_IMPLEMENTED` stands. Runtime
  attempts are still not routed through the seam, as R01 says.
- **What is incomplete:** the row's Notes and ImplementationEvidence should also
  cite the adapter framework.
- **Related records:** TEL-TEST-006 (`DOCUMENTED_UNIMPLEMENTED`) is unchanged,
  since no route is exercised. R01 already names a "declaration-admission gate"
  under DEL-10-02, which is consistent.

Inventory coverage gap: the pilot inventory has no capability for the desktop
seam (`telemetryPolicyService.ts`) or the telemetry panel. These are DEL-12-03's
product-facing implementation (DEC-074 O3). R3's unmapped set should check
whether the full inventory includes them.

I left the sealed forward file unedited.
