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
| CS-01 | `_CONTEXT.md` Decomposition Reference pins a revision other than 0.12 (for example "Accepted Revision 0.7, current_basis") | DECLARED_STATE · STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO | The frozen decomposition is revision 0.12. A catch-up repairs the pin with no decision (C3). C6(c) names "revision pins" under `STALE_REVIEW_OR_EVIDENCE`; its "setup-era origin text" class also fits, and all calibration workers used `STALE_SETUP_SPECIFICATION`. The more specific named class is chosen so that the whole corpus is consistent; the cause tag carries the clustering in either case |
| CS-02 | `_CONTEXT.md` PREPARATION Notes ("Structural scaffold only …") | HISTORY · ALIGNED | The history rule (C1). The note accurately records what the PREPARATION pass did |
| CS-03 | `_CONTEXT.md` SCA-00x control-surface notes | HISTORY · ALIGNED | The history rule (C1). The notes record how the control surface was created |
| CS-04 | `_CONTEXT.md` Architecture Basis Injection pins a revision other than 0.12 | DECLARED_STATE · STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO | As CS-01, for the pin. The row covers the pin only. Where any other part of the block diverges for this deliverable, add a `.sNN` row for each such part and judge it on its own: "Resolved Baseline", "Applicable Basis IDs", the PKG-00 `SEMANTIC_READY` statement, or a "Still TBD" item since ruled |
| CS-05 | `_CONTEXT.md` Architecture Gate Rule (PKG-01 to PKG-12 wait for PKG-00 readiness) | DECLARED_STATE · STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · NONE · RECORD · NO | Superseded by D-43 and HUMAN-STEER-PKG00-EXCLUSION-001. The catch-up needs no decision |
| CS-06 | `_CONTEXT.md` Package Reference, Objective Support, Scope Coverage or Scope Detail | Variant **OK**: DECLARED_STATE · ALIGNED. Variant **DRIFT**: DECLARED_STATE · STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO | Mechanical comparison at the freeze, recorded in the `MechanicalCheck` column. Package Reference is compared on all four fields of the SOFTWARE_DECOMP rev 0.12 package table row: name, scope, assigned scope items and exclusions. Objective Support and Scope Coverage are compared with `docs/_Registers/Deliverables.csv`. Scope Detail is compared statement by statement with `docs/_Registers/ScopeLedger.csv`. A Scope Detail block whose items run together on one line has no row here and is judged normally; there are four at the freeze (DEL-13-02, 14-03, 14-04 and 16-03), all with matching statements. At the freeze, 306 are OK and 16 DRIFT, all in the assigned-scope-items lists of PKG-01, PKG-02 and PKG-07 |
| CS-07 | `_CONTEXT.md` Context Budget QA | NON_NORMATIVE · NOT_ASSESSED | Setup context-budget metadata; makes no claim about the product. Context Envelope blocks are **not** covered, because their notes sometimes state normative constraints. They are judged normally |

Blocks in `_CONTEXT.md` that no CS row covers (Context Envelope, deliverable
identification, local notes) are judged normally.

## Shared text without a keyed row

`CLAIM_KEYS_V2.csv` gives every unit a `BodySHA256` and a `SharedTextCount`.
The body hash is taken over the text without its heading line. Some shared
bodies have no CS row, because they cannot be settled without reading the
evidence for the deliverables concerned. Examples:
- Remaining items repeated across the PKG-07 deliverables;
- Architecture Basis Injection variants;
- `STATUS#history` blocks.

For these, consistency is enforced rather than pre-decided:

1. The wave plan puts deliverables that share such a body in the same package
   under the same manager. Where practical, the same worker judges them.
2. The pattern situations below cover most of these cases (CP-06 to CP-08 for
   Remaining items, CP-04 for rename residue).
3. The validator's batch mode flags any two rows with the same `BodySHA256`
   whose disposition, cause, tier or layers differ. It also flags two rows
   with the same `CanonicalSituation` that differ in the same way. A flagged
   row needs `CANONICAL_DEPARTURE` with a reason; otherwise the verifier
   resolves the pair.
4. Verifiers review 100% of shared-body rows.

This records why a keyed row for each such hash is not needed (R0 PR review
finding 6).

## Pattern situations

| ID | Pattern | Required treatment |
|---|---|---|
| CP-01 | **Four-document residue.** Text naming `Datasheet.md`, `Specification.md`, `Guidance.md` or `Procedure.md` as current surfaces, after the SOW or architecture-basis conversion | STALE_SETUP_SPECIFICATION (origin text) or STALE_REVIEW_OR_EVIDENCE (a later declaration) · `REPRESENTATION_MIGRATED` · LOCAL_DESIGN · NONE · RECORD · NO |
| CP-02 | **Stale basis pointer.** A decomposition, DAG or revision pin, a section reference, or a code anchor moved without a behaviour change (for example `App.tsx::handle*` after PR #803) | STALE_REVIEW_OR_EVIDENCE · `BASIS_POINTER_STALE` · LOCAL_DESIGN · NONE · RECORD · NO |
| CP-03 | **D-41-era "current declaration" blocks** (for example "D-41 R5 T7 PDU-055 current declaration") | Their own DECLARED_STATE row (C1). They never change sibling dispositions. When they pin superseded revisions: CP-02. When they delegate residuals to `## Remaining`: A4 (the clause is not relied on) |
| CP-04 | **Rename residue.** The deliverable, or an active code identifier it names, still carries the former product name OpenPipeStress | A finding (ruling item 3): STALE_REVIEW_OR_EVIDENCE · `RENAME_OR_IDENTITY` · LOCAL_DESIGN · NONE · RECORD · **OWNER**. `AuthorityNeeded` is OWNER because R3 clusters these rows into the single class the owner rules at R4. Record it **once, on the SURFACE row** of each surface that carries it (C1 common-defect rule); items are judged on their own substance. **Active code identifiers are residue** (owner, R0 ruling addendum item 2: "If they are meant to identify active parts of the code then we need to revise them to be compatible with the current name"). (AGENT reading of that conditional answer, open to the owner's correction:) this includes the four identifiers the owner kept on 2026-09-18, because they identify active code: `.opsproj`, `openpipestress_jcs_ijson_v1`, `openpipestress_result_semantics_v0_2` and `openpipestress-projects.sqlite3` (`ACTIVATION_2026-09-18.md` §2, now context). These four are **code-change candidates for R4**, not deliverable edits:
  - The rows for the hash and result-semantics identifiers take `BaselineClass=FROZEN_CONTRACT`, tier `PROJECT_BASELINE` and layers `RECORD;BASELINE`, because renaming them changes a frozen contract.
  - The rows for `.opsproj` and the store filename take `BaselineClass=NONE`, tier `PROJECT_BASELINE` and layer `RECORD`, and note the persistence-compatibility obligation that DEC-101 attaches to identity renames: existing saved files must still open.
  - Other rename residue keeps the default fields above. Mentions of the former name in run records, history, or references to past states are not residue |
| CP-05 | **`_STATUS.md` "Last Updated" older than its own history entries** | STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT · LOCAL_DESIGN · NONE · RECORD · NO, on the `STATUS` SURFACE row |
| CP-06 | **Remaining item describing landed work** (text accurate, no open action) | ALIGNED with `NO_OPEN_ACTION` in Notes (A4) |
| CP-07 | **Remaining item whose condition has passed or been ruled.** For example "check before merge" after the merge, "prospective criteria" after D-72, or D-70's revision not applied | REMAINING_STATE_MISMATCH · RECORD_DRIFT · LOCAL_DESIGN · RULED_CRITERION when a ruling overtook it (else NONE) · RECORD · NO |
| CP-08 | **Remaining item or declaration citing an evidence file that does not exist in the frozen tree** (for example `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json`) | REMAINING_STATE_MISMATCH (Remaining) or STALE_REVIEW_OR_EVIDENCE (elsewhere) · RECORD_DRIFT · LOCAL_DESIGN · NONE · RECORD · OWNER |
| CP-09 | **Output-matrix `OUT-001` and SOW parity rows** | Consult `EVIDENCE_MAP.csv`, which lists every parity record found by content signature. If any PASS record's production hash matches the frozen SOW (`AnyPassMatchesFrozen=YES`), the row is ALIGNED and cites that record. If PASS records exist but none matches: STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN · LOCAL_DESIGN · NONE · RECORD · NO, citing the latest record. If there is no record (`NONE_FOUND`; five deliverables at the freeze): run the A3 discovery step first; if nothing is found, UNKNOWN · EVIDENCE_NOT_LOCATED |
| CP-10 | **A declared-open hold that the code has settled without a ruling** (for example state library, undo storage) | IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · PROJECT_BASELINE · NONE · RECORD · OWNER (C6(d)) |
| CP-11 | **"Met by construction."** A claim that holds only because the governed behaviour does not exist | Never ALIGNED (C6(a)): PARTIALLY_IMPLEMENTED or DOCUMENTED_UNIMPLEMENTED, with the gap's cause (`DEFERRED_BY_RULING`, `NOT_STARTED` or `PARTIAL_SLICE`) |
| CP-12 | **Verbatim validator or tool result asserted without a record** | Cite the frozen record or GATE record. If there is none, write "not rerun" in Notes and dispose UNKNOWN · EVIDENCE_NOT_LOCATED (A6) |

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
