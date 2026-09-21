# Canonical situation table

Authored by Agent 0 under R0 ruling item 1 and `CONVENTIONS.md` C1. It gives
one required disposition, cause, tier and layer for each recurring situation,
so the same situation is judged the same way across 102 deliverables.

- A worker inherits the row. It may depart only with a written justification
  in Notes, and verifiers check conformance mechanically.
- The table is independently reviewed with the R0 PR and shown to the owner
  at the first-wave checkpoint.
- Every value here is an agent judgment, not an owner ruling.

There are two kinds of entry:

- **Keyed situations (CS-01 to CS-07).** These are assigned mechanically to
  specific claim keys in `CANONICAL_ASSIGNMENTS.csv` (built by
  `tools/build_canonical.py`, deterministic against the frozen commit). They
  cover the setup-era `_CONTEXT.md` blocks, which make up most of the shared
  text.
- **Pattern situations (CP-01 to CP-12).** A worker applies these whenever it
  recognizes the pattern. The worker writes the CP ID in `CanonicalSituation`.

## Keyed situations

Each situation lists: claim type · disposition · cause · tier · baseline class
· layer · `AuthorityNeeded`.

| ID | Situation | Fields | Basis |
|---|---|---|---|
| CS-01 | `_CONTEXT.md` Decomposition Reference pins a revision other than 0.12 (for example "Accepted Revision 0.7, current_basis") | DECLARED_STATE · STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO | The frozen decomposition is revision 0.12. A catch-up repairs the pin with no decision (C3, C6(c)) |
| CS-02 | `_CONTEXT.md` PREPARATION Notes ("Structural scaffold only …") | HISTORY · ALIGNED | The history rule (C1). The note accurately records what the PREPARATION pass did |
| CS-03 | `_CONTEXT.md` SCA-00x control-surface notes | HISTORY · ALIGNED | The history rule (C1). The notes record how the control surface was created |
| CS-04 | `_CONTEXT.md` Architecture Basis Injection pins a revision other than 0.12 | DECLARED_STATE · STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO | As CS-01. If the block's "Still TBD" items have since been ruled for this deliverable, add a `.sNN` row for each such item and judge it on its own |
| CS-05 | `_CONTEXT.md` Architecture Gate Rule (PKG-01 to PKG-12 wait for PKG-00 readiness) | DECLARED_STATE · STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · NONE · RECORD · NO | Superseded by D-43 and HUMAN-STEER-PKG00-EXCLUSION-001. The catch-up needs no decision |
| CS-06 | `_CONTEXT.md` Package Reference, Objective Support or Scope Coverage | Variant **OK**: DECLARED_STATE · ALIGNED. Variant **DRIFT**: DECLARED_STATE · STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO | Mechanical comparison with the SOFTWARE_DECOMP rev 0.12 package table and `docs/_Registers/Deliverables.csv` at the freeze (`MechanicalCheck` column). All 306 are OK at the freeze |
| CS-07 | `_CONTEXT.md` Context Envelope and Context Budget QA | NON_NORMATIVE · NOT_ASSESSED | Setup process metadata; makes no claim about the product |

Blocks in `_CONTEXT.md` that no CS row covers (Scope Detail, deliverable
identification, local notes) are judged normally.

## Pattern situations

| ID | Pattern | Required treatment |
|---|---|---|
| CP-01 | **Four-document residue.** Text naming `Datasheet.md`, `Specification.md`, `Guidance.md` or `Procedure.md` as current surfaces, after the SOW or architecture-basis conversion | STALE_SETUP_SPECIFICATION (origin text) or STALE_REVIEW_OR_EVIDENCE (a later declaration) · `REPRESENTATION_MIGRATED` · LOCAL_DESIGN · NONE · RECORD · NO |
| CP-02 | **Stale basis pointer.** A decomposition, DAG or revision pin, a section reference, or a code anchor moved without a behaviour change (for example `App.tsx::handle*` after PR #803) | STALE_REVIEW_OR_EVIDENCE · `BASIS_POINTER_STALE` · LOCAL_DESIGN · NONE · RECORD · NO |
| CP-03 | **D-41-era "current declaration" blocks** (for example "D-41 R5 T7 PDU-055 current declaration") | Their own DECLARED_STATE row (C1). They never change sibling dispositions. When they pin superseded revisions: CP-02. When they delegate residuals to `## Remaining`: A4 (the clause is not relied on) |
| CP-04 | **Rename residue.** The deliverable still names the product OpenPipeStress | A finding (ruling item 3): STALE_REVIEW_OR_EVIDENCE · `RENAME_OR_IDENTITY` · LOCAL_DESIGN · NONE · RECORD · NO. Record it **once, on the SURFACE row** of each surface that carries it (C1 common-defect rule). Items are judged on their own substance. Hard identifiers kept by owner ruling (`.opsproj`, `openpipestress_jcs_ijson_v1`, `openpipestress_result_semantics_v0_2`, `openpipestress-projects.sqlite3`) are not residue |
| CP-05 | **`_STATUS.md` "Last Updated" older than its own history entries** | STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT · LOCAL_DESIGN · NONE · RECORD · NO, on the `STATUS` SURFACE row |
| CP-06 | **Remaining item describing landed work** (text accurate, no open action) | ALIGNED with `NO_OPEN_ACTION` in Notes (A4) |
| CP-07 | **Remaining item whose condition has passed or been ruled.** For example "check before merge" after the merge, "prospective criteria" after D-72, or D-70's revision not applied | REMAINING_STATE_MISMATCH · RECORD_DRIFT · LOCAL_DESIGN · RULED_CRITERION when a ruling overtook it (else NONE) · RECORD · NO |
| CP-08 | **Remaining item or declaration citing an evidence file that does not exist in the frozen tree** (for example `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json`) | REMAINING_STATE_MISMATCH (Remaining) or STALE_REVIEW_OR_EVIDENCE (elsewhere) · RECORD_DRIFT · LOCAL_DESIGN · NONE · RECORD · OWNER |
| CP-09 | **Output-matrix `OUT-001` and SOW parity rows** | Consult `EVIDENCE_MAP.csv`. With a PASS parity whose hash matches the frozen SOW: ALIGNED. With a PASS parity whose hash differs: STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN · LOCAL_DESIGN · NONE · RECORD · NO. With no parity record: run the A3 discovery step first; if nothing is found, UNKNOWN · EVIDENCE_NOT_LOCATED |
| CP-10 | **A declared-open hold that the code has settled without a ruling** (for example state library, undo storage) | IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · PROJECT_BASELINE · NONE · RECORD · OWNER (C6(d)) |
| CP-11 | **"Met by construction."** A claim that holds only because the governed behaviour does not exist | Never ALIGNED (C6(a)): PARTIALLY_IMPLEMENTED or DOCUMENTED_UNIMPLEMENTED, with the gap's cause (`DEFERRED_BY_RULING`, `NOT_STARTED` or `PARTIAL_SLICE`) |
| CP-12 | **Verbatim validator or tool result asserted without a record** | Cite the frozen record or GATE record. If there is none, write "not rerun" in Notes and dispose UNKNOWN · EVIDENCE_NOT_LOCATED (A6) |

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
