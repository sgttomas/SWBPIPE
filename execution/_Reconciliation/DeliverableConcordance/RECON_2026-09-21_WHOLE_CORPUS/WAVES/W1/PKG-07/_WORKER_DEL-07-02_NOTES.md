# Worker G1 carry-forward notebook — W1 PKG-07 (DEL-07-02, DEL-07-09, DEL-07-01)

TASK (Type 2) worker, run HELP-HUMAN-PIPING-20260921-RECONCILIATION. Frozen
state `00115c719`. Agent judgments, not owner rulings. Standard claim fence
applies (F-PIP-2; claims taxonomy per DEC-081).

How recurring situations are judged, so the three ledgers treat them alike:

1. **Splitting `.rNN` blocks.** Split a table block when its rows take
   different dispositions; the parent then becomes `CONTAINER`, unless it has
   prose of its own, in which case it is assessed directly. Tables whose rows
   all take one disposition are assessed as one block. For bullet blocks, use a
   `.sNN` only for a part that takes a different disposition (for example the
   revision-0.7 pin in DEL-07-02 CLM-009).
2. **GUI component/state-library hold.** Any row that declares the library
   choice open or TBD is CP-10 (IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR ·
   PROJECT_BASELINE · NONE · RECORD · OWNER). The code holds transient state in
   bespoke React context and session modules, `package.json` adds no library,
   and DEL-00-05 still records the choice as OPEN, routed to a human ruling.
   Rows that only oblige the documents to keep the TBD (DEL-07-01 REQ-03,
   VER-07) are judged on the documents: they comply, so ALIGNED, with a
   pointer to the CP-10 rows. Dependency versions are left to implementation
   by DEC-012, so a "versions TBD" row is setup residue, not CP-10.
3. **D-41 PDU-055 declarations** (rev 0.8, DAG-007): CP-03 in
   `CanonicalSituation`, CP-02 fields. The delegation to Remaining is not
   relied on (A4). Blanket supersession does not change sibling rows.
4. **Shared Remaining bodies across PKG-07.**
   - "prospective owner-ruled criteria" (07-02 R02 = 07-01 R03) → CP-07 with
     RULED_CRITERION (D-72 ruled items 1-4 and 6).
   - "Performance qualification remains OPEN ... owner reserves sequencing"
     (07-09 R02) → CP-07 with RULED_CRITERION (D-70 decided the sequencing;
     D-72 ruled the criteria).
   - "D-69 metadata increment ... before merge; PR #789" (07-02 R03 = 07-01
     R04 = 07-09 R03) → CP-07 with NONE (the merge, not a ruling, overtook it).
   - AX omission (07-02 R04 = 07-01 R05 = 07-09 R04) → UNKNOWN ·
     EVIDENCE_NOT_LOCATED · LOCAL_DESIGN · NONE · RECORD · NO. The text matches
     FINAL_ACCEPTANCE and the V103 archive, but the observation predates the
     SWBPIPE shell work.
   - Missing `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json` (07-02 R11 = 07-09 R09)
     → CP-08 (REMAINING_STATE_MISMATCH · RECORD_DRIFT · ... · OWNER).
   - The ROOT FINAL_ACCEPTANCE "bounded UI foundation" items, with their open
     residuals, → ALIGNED.
   - Landed-state narrative with no open action → CP-06 ALIGNED, with
     NO_OPEN_ACTION.
5. **Parity (CP-09).** Matrix OUT-001 and VER-001 both →
   STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN, citing the CHANGE-P2 parity
   and manager validation. Purpose-section OUT-001 ("a contract ... is
   produced") is judged on the contract's existence and facet coverage; the
   staleness of its text sits on the block rows.
6. **AC-001 "contract preserves the current implemented slice".** Where the
   contract text no longer describes the implemented slice →
   PARTIALLY_IMPLEMENTED · DOC_BEHIND_CODE · LOCAL_DESIGN.
7. **Protected-content review of UI fixtures.** A3 discovery found no record →
   UNKNOWN · EVIDENCE_NOT_LOCATED · INVARIANT · IP_DATA.
8. **Checksum and rule-pack or private-library reference fields.** They live in
   features/rule-packs and features/secret-private-library →
   PARTIALLY_IMPLEMENTED · OWNERSHIP_ELSEWHERE · LOCAL_DESIGN.
9. **`_CONTEXT.md` Architecture Basis Injection** beyond CS-04: `.s01` for the
   PKG-00 SEMANTIC_READY statement (all eight PKG-00 deliverables are
   IN_PROGRESS) → STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT. `.s02` for the
   Still-TBD container item, which DEC-017 has since ruled →
   STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING. Both LOCAL_DESIGN ·
   NONE · RECORD · NO.
10. **Relocated handler anchors** (`App.tsx::handle*` → `workspaceSession.ts`,
    PR #803, stated no behaviour change) → CP-02.
11. **Rename residue.** It is a finding only where the deliverable's own
    surfaces carry the former name; there it goes on the SURFACE row (CP-04).
    Active code that carries the name but is not named by the deliverable is
    noted as context: `core/gui/model_tree/engine.py`, and for DEL-07-01
    `schemas/viewport_editor.schema.yaml` and the viewport_editor crate.
12. **`_STATUS.md` Last Updated older than its own history** → CP-05 on the
    STATUS SURFACE row.
13. **Evidence tokens.** Deliverable-folder paths contain spaces, so they are
    cited in ContextRefs only. Test cases are named in Notes, and only the test
    files go in evidence columns. Suite status comes from
    `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` and was not rerun.
