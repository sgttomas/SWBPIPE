# R3 T4B — representation, rename and record-drift classes

T4B classifies the 625 divergent rows of `R3/CORPUS_CLAIMS.csv` whose
effective CauseTag is `REPRESENTATION_MIGRATED` (339), `RENAME_OR_IDENTITY`
(86) or `RECORD_DRIFT` (200). It assigns them to eight classes. All 86
rename rows stay in one class, T4B-C01, for a single R4 ruling (CONVENTIONS
C6(e); R0 ruling item 3). Four classes need an owner decision:
- the rename ruling (C01, 86 rows);
- five ISSUED or baseline-bearing record rows (C02);
- the PKG-00 SEMANTIC_READY injection sub-claims, cluster SR-1 (C03, 42 rows);
- rows that cite evidence missing from the frozen tree, mainly the deleted
  export plan (C04, 17 rows).

The other four classes are text catch-ups with no decision needed. They
cover 475 rows and route to `R5_RECORD_REPAIR`:
- four-document residue (C05, 336 rows);
- stale `_STATUS` and memory surfaces (C06, 59);
- overtaken Remaining items (C07, 21);
- other declarations overtaken by later records or rulings (C08, 59).

Every row is `RECORD`-layer drift except the three frozen-contract rename
rows, which are `RECORD;BASELINE`. T4B describes records, not engineering
adequacy. It proposes routing only; it re-disposes no row.

Evidence conventions: `FREEZE:` paths are at
`00115c71931bcae79909602d653740d3bb72dfa1`, relative to
`projects/chirality-piping/`. Ledger keys cite the sealed forward ledgers,
read with their effective values.

## Class table

| Class | Name | Rows | Owning authority | Route |
|---|---|---|---|---|
| T4B-C01 | Rename and identity residue (single R4 ruling) | 86 | OWNER | OWNER_DECISION |
| T4B-C02 | ISSUED and baseline-bearing record drift | 5 | OWNER | OWNER_DECISION |
| T4B-C03 | PKG-00 SEMANTIC_READY injection sub-claims (SR-1) | 42 | OWNER | OWNER_DECISION |
| T4B-C04 | Declarations citing evidence absent from the freeze (CP-08) | 17 | OWNER | OWNER_DECISION |
| T4B-C05 | Four-document residue (CP-01) | 336 | NONE | R5_RECORD_REPAIR |
| T4B-C06 | Stale status and memory surfaces (CP-05 and analogues) | 59 | NONE | R5_RECORD_REPAIR |
| T4B-C07 | Remaining items overtaken by rulings or events | 21 | NONE | R5_RECORD_REPAIR |
| T4B-C08 | Other declarations overtaken by later records or rulings | 59 | NONE | R5_RECORD_REPAIR |

**How rows are assigned.** Each row takes the first rule that matches, in
this order:
1. The effective cause is `RENAME_OR_IDENTITY`: C01.
2. BaselineClass is `ISSUED`, or the tier is `PROJECT_BASELINE`: C02.
3. The key is a `CONTEXT#architecture-basis-injection.s01` sub-claim: C03.
4. CanonicalSituation is `CP-08`: C04.
5. The cause is `REPRESENTATION_MIGRATED`: C05.
6. UnitKind is `SURFACE`: C06.
7. The disposition is `REMAINING_STATE_MISMATCH`: C07.
8. Anything else: C08.

## T4B-C01 — Rename and identity residue (single R4 ruling)

**Description.** These are the deliverable surfaces, and the active code
identifiers they name, that still carry the former product name
OpenPipeStress after DEC-101 and SCA-010 renamed the product SWBPIPE. R0
ruling item 3 holds that DEC-101 does not reach deliverable Scope of Work
files, so this residue is a finding and never an accepted divergence. CP-04
records it once per surface, on the SURFACE row (C1 common-defect rule), with
`AuthorityNeeded = OWNER` so that R3 can present one class for one ruling.

The class has four variants, all kept here as one ruling unit:
- **Default text and identifier residue** (76 rows, LOCAL_DESIGN). Examples
  are the product name in SOW text, crate and binary names such as
  `open_pipe_stress_*` and `openpipestress-runner`, and schema `$id` URLs on
  `openpipestress.org`.
- **Persistence identifiers** (6 rows, PROJECT_BASELINE, BaselineClass NONE):
  the `.opsproj` container extension. The owner kept this identifier, and the
  store filename, on 2026-09-18. DEC-101 attaches a persistence-compatibility
  obligation to renaming them: existing saved files must still open.
- **Frozen-contract identifiers** (3 rows, PROJECT_BASELINE,
  FROZEN_CONTRACT, `RECORD;BASELINE`): the hash-profile identifier
  `openpipestress_jcs_ijson_v1`.
- **ISSUED text** (1 row, `DEL-01-01:SOW`, `LIFECYCLE_REASSESSMENT_REQUIRED`
  with BaselineClass ISSUED).

**Signature.** Cause `RENAME_OR_IDENTITY`, CanonicalSituation CP-04 on all
86 rows.

- **Population:** 86 rows, 81 deliverables, 18 packages.
- **Packages (rows):** PKG-00 (5), PKG-01 (4), PKG-02 (6), PKG-03 (5), PKG-04 (1), PKG-05 (5), PKG-06 (2), PKG-07 (5), PKG-08 (7), PKG-09 (4), PKG-10 (5), PKG-11 (5), PKG-12 (5), PKG-13 (4), PKG-14 (4), PKG-15 (4), PKG-16 (4), PKG-17 (11).
- **Deliverables** (prefix `DEL-` omitted): 00-01, 00-02, 00-03, 00-07, 00-08, 01-01, 01-02, 01-03, 01-04, 02-01, 02-02, 02-04, 02-05, 03-01, 03-02, 03-03, 03-06, 03-08, 04-03, 05-01, 05-02, 05-03, 05-04, 05-05, 06-01, 06-05, 07-01, 07-03, 07-05, 07-06, 07-07, 08-01, 08-02, 08-03, 08-04, 08-05, 08-06, 09-02, 09-03, 09-04, 09-05, 10-01, 10-02, 10-03, 10-04, 10-05, 11-01, 11-02, 11-03, 11-04, 11-05, 12-01, 12-02, 12-03, 12-04, 12-05, 13-01, 13-02, 13-03, 13-04, 14-01, 14-02, 14-03, 14-05, 15-01, 15-02, 15-03, 15-04, 16-01, 16-02, 16-03, 16-04, 17-01, 17-02, 17-03, 17-04, 17-05, 17-06, 17-07, 17-08, 17-09.
- **Signature** (Disposition · Tier · Baseline · Layers): STALE_REVIEW_OR_EVIDENCE · LOCAL_DESIGN · NONE · RECORD (76); STALE_REVIEW_OR_EVIDENCE · PROJECT_BASELINE · NONE · RECORD (6); STALE_REVIEW_OR_EVIDENCE · PROJECT_BASELINE · FROZEN_CONTRACT · RECORD;BASELINE (3); LIFECYCLE_REASSESSMENT_REQUIRED · LOCAL_DESIGN · ISSUED · RECORD (1).
- **Rows with adopted resolutions:** `DEL-00-01:AB` (RESOLVED_PAIR); `DEL-00-02:AB` (RESOLVED_PAIR); `DEL-00-03:AB` (RESOLVED_PAIR); `DEL-00-07:AB` (RESOLVED_PAIR); `DEL-00-08:AB` (RESOLVED_PAIR); `DEL-01-01:SOW` (CONTESTED); `DEL-02-05:STATUS.s01` (RESOLVED_PAIR); `DEL-05-01:SOW` (OBSERVED); `DEL-05-02:SOW` (OBSERVED); `DEL-05-03:SOW` (OBSERVED); `DEL-05-04:SOW.s01` (FIELD); `DEL-05-05:SOW.s01` (OBSERVED); `DEL-08-06:SOW` (OBSERVED); `DEL-09-04:SOW` (OBSERVED); `DEL-09-05:SOW` (OBSERVED); `DEL-10-04:SOW` (FIELD); `DEL-10-05:SOW` (FIELD); `DEL-11-02:SOW` (OBSERVED); `DEL-14-05:SOW` (FIELD).

**Owning authority.** OWNER.

**Route.** `OWNER_DECISION`.

**Decision needed.** One ruling on how rename residue is repaired, per
variant. The options, as they stand in the evidence:
1. **Deliverable text residue.** Authorise one R5 record-repair sweep. It
   would replace the former name in the SOW, AB, STATUS and CONTEXT surfaces
   listed here, and leave history, run records and references to past states
   untouched (CP-04). This is the only option the rulings leave open for
   deliverable text: R0 item 3 rules out accepted divergence.
2. **Active code identifiers, default variant** (crate, binary and schema
   `$id` names). Either:
   - (a) rename them through code-fix briefs, following DEC-101 (iv), which
     names binary, crate and schema `$id` namespace for the identity tranche
     (`DEL-10-05:SOW` Notes); or
   - (b) the owner records them as retained identifiers.

   The owner's R0 addendum words point to (a) for identifiers of active code.
3. **The four identifiers kept on 2026-09-18.** Agent 0 read these as
   residue (R0 addendum item 2, an AGENT reading that is open to the owner's
   correction). For each identifier, the options are:
   - (a) rename it: with read-compatibility for existing `.opsproj` files
     and stores, and through a new contract version for the two
     frozen-contract identifiers;
   - (b) keep it, by the owner correcting Agent 0's reading, so that the
     2026-09-18 selection stands.

   Only `openpipestress_jcs_ijson_v1` has `FROZEN_CONTRACT` rows in this
   partition. The store filename `openpipestress-projects.sqlite3` is
   mentioned only in the Notes of `DEL-02-05:SOW`, a default-variant row,
   and recorded in that deliverable's notes for R3. It has no
   persistence-variant row. `openpipestress_result_semantics_v0_2` is named
   in CP-04, but none of the 86 rows mentions it. The ruling should cover
   both.
4. **ISSUED text** (`DEL-01-01:SOW`, CONTESTED). This row is routed to R4
   twice: through finding group FG-DEL-01-01-02 and through this class (W2
   gate assessment, "Owner rulings needed"). The options are:
   - (a) CP-04 as written, with BaselineClass ISSUED, repaired inside the
     rename ruling through the ISSUED change path;
   - (b) fold it into the C6(d) ISSUED finding group (R0 ruling item 4).

   The owner picks one route, so that the row is ruled once.

**On-ruling mechanism.**
- Text residue: an R5 record-repair tranche per package, under the owner's
  separate R5 authorisation (D-73), through the ordinary change path
  (`.agents/skills/chirality-change/SKILL.md`).
- Identifier renames: `CODE_FIX_CANDIDATE` briefs, drafted by Agent 0 under
  `R3/CODE_FIX_BRIEF_CANDIDATES/` and not executed by this run.
  - Frozen-contract identifiers also need a contract-version change under
    the baseline change path.
  - Persistence identifiers carry a compatibility test obligation.
- `DEL-01-01`: the ISSUED change path.

**Risk if unrepaired.**
- Deliverables and code keep naming a product that no longer exists, and
  the identity stays split between SWBPIPE and OpenPipeStress.
- Every later rename raises the cost of the frozen-contract and persistence
  migration.
- The double routing of `DEL-01-01` could produce two inconsistent rulings.

**Visible exceptions.**
- **CONTESTED:** `DEL-01-01:SOW` (see item 4).
- **RESOLVED_PAIR:**
  - five `.opsproj` and runner rows: `DEL-00-01:AB`, `DEL-00-02:AB`,
    `DEL-00-07:AB`, `DEL-00-08:AB` and `DEL-02-05:STATUS.s01`;
  - `DEL-00-03:AB`, where the runner binary is the default variant.
- **OBSERVED:**
  - `DEL-11-02:SOW`. The DEC-022 grammar identifier
    `open_pipe_stress_declared_expression`, which is active code, appears
    not recorded in DEL-11-02's ledger. The ruling's scope should name it.
  - `DEL-08-06:SOW`: code-only residue recorded on a SOW surface that does
    not itself name the identifier.
  - `DEL-05-01/02/03:SOW` and `DEL-09-04/05:SOW`: frontmatter
    decomposition-basis pins without their own row. The pins belong to
    T4A's cause, `BASIS_POINTER_STALE`, but are keyed here because no pin
    key exists.
  - `DEL-05-05:SOW.s01`: placement.
- **FIELD:**
  - `DEL-05-04:SOW.s01`: placement.
  - `DEL-14-05:SOW`. RemainingWork should also name the schema `$id` and
    titles.
  - `DEL-10-04:SOW` and `DEL-10-05:SOW`: VerificationClass.

**Representative keys.**
- `DEL-00-01:AB`: persistence variant. `.opsproj` is active code, at
  `FREEZE:core/reporting/report_package/src/lib.rs:56`
  (`CONTAINER_EXTENSION`).
- `DEL-14-02:STATUS`: frozen-contract variant. `openpipestress_jcs_ijson_v1`
  is a schema const, at
  `FREEZE:schemas/stress_neutral_export.v0.2.schema.json:388`.
- `DEL-12-01:SOW`: default variant. The SOW names the former product at
  CLM-011 (L171) and CLM-023 (L341), per the ledger Notes.
- For the ruling's scope, the two identifiers without a row in this
  partition:
  - the store filename, at `FREEZE:apps/desktop/src-tauri/src/lib.rs:32`;
  - the result-semantics identifier, at
    `FREEZE:schemas/analysis_run.v0.2.schema.json:194`.

## T4B-C02 — ISSUED and baseline-bearing record drift

**Description.** Five record-drift rows cannot be repaired as ordinary
catch-ups, because the text sits in an ISSUED deliverable or restates a
project baseline.
- **Four rows in the ISSUED `DEL-01-01`:**
  - three four-document-residue blocks (`CLM-018.s01`, `CLM-019.r06` and
    `CLM-020`), with a declared `CANONICAL_DEPARTURE` from CP-01's
    BaselineClass NONE to ISSUED;
  - the SOW status field `CLM-002.r11`, which still reads IN_PROGRESS,
    although `_STATUS.md` records ISSUED since 2026-06-03.
- **One row in `DEL-17-03`:** AC-001 still states canonical-JSON and
  JCS-compatible hashing. REQ-006 was narrowed under DEC-074 E1 on
  2026-07-12, and the implementation now hashes sorted compact ASCII JSON
  and asserts no JCS conformance.

- **Population:** 5 rows, 2 deliverables, 2 packages.
- **Packages (rows):** PKG-01 (4), PKG-17 (1).
- **Deliverables** (prefix `DEL-` omitted): 01-01, 17-03.
- **Signature** (Disposition · Tier · Baseline · Layers): STALE_SETUP_SPECIFICATION · LOCAL_DESIGN · ISSUED · RECORD (3); STALE_REVIEW_OR_EVIDENCE · LOCAL_DESIGN · ISSUED · RECORD (1); IMPLEMENTED_DIFFERENTLY · PROJECT_BASELINE · NONE · RECORD;BASELINE (1).
- **Rows with adopted resolutions:** none.

**Owning authority.** OWNER.

**Route.** `OWNER_DECISION`.

**Decision needed.**
1. **`DEL-01-01`.** Whether to authorise the ISSUED change path to restate
   the four rows. The options in the evidence are:
   - (a) amend the ISSUED SOW now, bundled with the R0 item 4 finding group
     for `DEL-01-01`, which already goes to R4 as one ISSUED change-path
     item;
   - (b) leave the ISSUED text unchanged until the next ISSUED revision,
     with the drift recorded. No ruling currently supports accepted
     divergence for this drift (C6(g)), so option (b) needs an owner record
     (A3a).
2. **`DEL-17-03` AC-001.** Two options:
   - (a) reword AC-001 to the implemented project-local hash basis, per
     DEC-074 E1 (the ledger RemainingWork);
   - (b) hold the JCS-compatible wording, and treat the implementation as
     the gap.

   This decision is coupled to the W3 owner item that the "JCS-compatible"
   hash labels are not RFC 8785 (PKG-14, PKG-17), and to `DEL-17-02`
   REQ-007. It should be ruled together with them.

**On-ruling mechanism.**
- `DEL-01-01`: the ISSUED change path, a revision of the ISSUED SOW with
  lifecycle handling, not an ordinary R5 edit.
- `DEL-17-03`: an R5 record repair of AC-001 if the owner chooses (a); a
  `CODE_FIX_CANDIDATE` or engineering item if the owner chooses (b).

**Risk if unrepaired.**
- The ISSUED governance baseline keeps directing readers to retired
  documents and a wrong status.
- AC-001 keeps asserting a hash-conformance property the code disclaims,
  which is a claims-boundary exposure on an interop contract.

**Visible exceptions.** No resolution rows. Two notes:
- The three `DEL-01-01` CP-01 rows carry a declared `CANONICAL_DEPARTURE`.
- `DEL-17-03` AC-001 is the only `IMPLEMENTED_DIFFERENTLY` row in the
  partition.

**Representative keys.**
- `DEL-01-01:SOW#CLM-018.s01`: four-document residue in ISSUED text. The
  ledger cites the H1 acceptance record
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/snapshots/I0/H1_EVIDENCE/HELP_HUMAN_ACCEPTANCE.md`
  and conversion commit 3c7f6abfb.
- `DEL-01-01:SOW#CLM-002.r11`: the status text predates the same-day
  ISSUED transition (first present f604012b2).
- `DEL-17-03:SOW#completion-and-reliance-basis-epistemology/AC-001`: the
  evidence is `core/handoff/native_json/package.py` and
  `schemas/native_json_export.schema.json` (ledger ImplementationEvidence).

## T4B-C03 — PKG-00 SEMANTIC_READY injection sub-claims (SR-1)

**Description.** Each of these deliverables' `_CONTEXT.md` carries an
architecture-basis injection statement: PKG-00 (Software Architecture
Runway) is at SEMANTIC_READY and supplies dispatchable architecture-basis
constraints. CS-04 mints a `.s01` sub-claim for that statement. At the
freeze:
- all eight PKG-00 deliverables are IN_PROGRESS (D-40 / DEC-072);
- the architecture basis lives in `ArchitectureBasis.md` and SOFTWARE_DECOMP
  rev 0.12;
- the PKG-00 readiness gate was superseded (D-43,
  HUMAN-STEER-PKG00-EXCLUSION-001).

The disposition `STALE_REVIEW_OR_EVIDENCE` is owner-confirmed (Direction 8;
CONVENTIONS F3). The owner added that the status is outdated and should be
advanced "upon my subsequent approval in the proper workflow".

The cause is the contested corpus cluster SR-1, RECORD_DRIFT or
SCOPE_REDIRECTED_BY_RULING (W3 assessment). T8 owns that reading. The
routing below holds under either cause.

- **Population:** 42 rows, 42 deliverables, 12 packages.
- **Packages (rows):** PKG-02 (5), PKG-03 (1), PKG-04 (3), PKG-07 (5), PKG-08 (3), PKG-09 (2), PKG-10 (5), PKG-11 (5), PKG-12 (2), PKG-13 (4), PKG-14 (3), PKG-15 (4).
- **Deliverables** (prefix `DEL-` omitted): 02-01, 02-02, 02-03, 02-04, 02-05, 03-07, 04-04, 04-05, 04-06, 07-01, 07-02, 07-05, 07-07, 07-08, 08-04, 08-05, 08-06, 09-04, 09-05, 10-01, 10-02, 10-03, 10-04, 10-05, 11-01, 11-02, 11-03, 11-04, 11-05, 12-04, 12-05, 13-01, 13-02, 13-03, 13-04, 14-01, 14-02, 14-03, 15-01, 15-02, 15-03, 15-04.
- **Signature** (Disposition · Tier · Baseline · Layers): STALE_REVIEW_OR_EVIDENCE · LOCAL_DESIGN · NONE · RECORD (42).
- **Rows with adopted resolutions:** all 42 rows carry OWNER_CONFIRMED.

**Owning authority.** OWNER.

**Route.** `OWNER_DECISION`.

**Decision needed.** Whether, and to what state, PKG-00's lifecycle is
advanced, and how the 42 injection statements are then restated. The
options, as they stand in the evidence:
- (a) Approve the PKG-00 lifecycle advancement in the lifecycle workflow,
  then restate each statement against the new state.
- (b) Restate the statements without a readiness state: the basis is
  `ArchitectureBasis.md` and SOFTWARE_DECOMP rev 0.12. This reflects that
  the readiness gate was superseded (D-43), and it makes the text
  independent of the PKG-00 lifecycle.
- (c) Both.

Direction 8 already says the status should be advanced after the owner's
approval. The open choice is the target state and the order.

**On-ruling mechanism.**
- The PKG-00 status advance goes through the lifecycle workflow, as an
  owner approval. This run never makes that change (F3).
- The 42 `_CONTEXT.md` statements are then repaired in one R5
  record-repair tranche.

**Risk if unrepaired.**
- 42 deliverable contexts tell agents a readiness gate exists that was
  superseded.
- Dispatch or setup logic that reads the injection statement may wait on,
  or assert, a state that does not exist.

**Visible exceptions.**
- Every row is OWNER_CONFIRMED on the disposition. The cause stays
  contested (SR-1).
- R3 observation 1 covers `AuthorityNeeded`: OWNER on `DEL-13-01`,
  `DEL-13-03` and `DEL-13-04`, NO on the other 39.

**Representative keys.**
- `DEL-07-01:CONTEXT#architecture-basis-injection.s01`: the gate was
  superseded (D-43, HUMAN-STEER-PKG00-EXCLUSION-001).
- `DEL-09-04:CONTEXT#architecture-basis-injection.s01`: D-40 / DEC-072
  moved PKG-00 from CHECKING to IN_PROGRESS.
- `DEL-13-01:CONTEXT#architecture-basis-injection.s01`: RemainingWork
  routes the readiness-state advance to R4, "then refresh the statement".

## T4B-C04 — Declarations citing evidence absent from the freeze (CP-08)

**Description.** These rows cite a governing or evidence file that does not
exist in the frozen tree. Their substance usually holds in code, but the
cited basis is gone. There are two groups.
- **Export plan (15 rows, PKG-17).** The rows cite PLAN-EXPORT-INTEROP
  (`plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`). The ledgers record that
  commit 349a2ab33 (2026-06-03) deleted it with no archived copy. The file
  is absent at `FREEZE:plans/`.
- **N7 intake (2 rows, PKG-07).** Two Remaining items cite
  `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json`, which exists nowhere in the
  frozen tree. The companion `REVIEW_RETURN_V2.md` and
  `PRECOMMIT_PARENT_FAN_IN_V1.md` do exist.

The export-plan routing is the contested W3 cluster "Export plan" (OWNER or
stale pointer). T8 owns that reading.

- **Population:** 17 rows, 6 deliverables, 2 packages.
- **Packages (rows):** PKG-07 (2), PKG-17 (15).
- **Deliverables** (prefix `DEL-` omitted): 07-02, 07-09, 17-01, 17-02, 17-05, 17-06.
- **Signature** (Disposition · Tier · Baseline · Layers): STALE_REVIEW_OR_EVIDENCE · LOCAL_DESIGN · NONE · RECORD (14); REMAINING_STATE_MISMATCH · LOCAL_DESIGN · NONE · RECORD (2); STALE_SETUP_SPECIFICATION · LOCAL_DESIGN · NONE · RECORD (1).
- **Rows with adopted resolutions:** none.

**Owning authority.** OWNER.

**Route.** `OWNER_DECISION`.

**Decision needed.**
1. **Export plan.** Three options:
   - (a) Treat PLAN-EXPORT-INTEROP as a stale pointer. Re-point each row to
     a surviving authority, then repair it by R5. The W2 gate assessment
     notes that the accepted SCA-004 names the export formats, and
     `DEL-17-04` implements the first solver-specific target (ledger
     `DEL-17-01:SOW#CLM-005/F-17-01-001`).
   - (b) Restore the plan from history and re-adopt it as an accepted
     source.
   - (c) Withdraw it as a source, and have the rows that relied on it for
     strategy or ordering (for example `DEL-17-02` REQ-026) re-derive their
     basis.
2. **N7 intake.** Two options:
   - (a) Record where the intake evidence actually lives, if it exists
     outside the tree.
   - (b) Restate the two Remaining items to cite the existing review return
     and fan-in records only.

**On-ruling mechanism.**
- Option (a) or (c): an R5 record-repair tranche over the 17 rows. The
  Source Basis Register and `_REFERENCES.md` entries are included.
- Option (b): an owner adoption record for the restored plan, then R5
  repair of the citations.

**Risk if unrepaired.**
- PKG-17 requirements and source registers rest on an unlocatable source.
  `DEL-17-02` REQ-026 even grants a citation permission against it.
- Reviewers cannot check the stated basis for export strategy and target
  ordering.

**Visible exceptions.** No resolution rows. The export-plan rows belong to
the contested W3 cluster (T8).

**Representative keys.**
- `DEL-17-02:SOW#CLM-020/DEL-17-02-REQ-026`: the REQ text, at
  `FREEZE:execution/PKG-17_*/1_Working/DEL-17-02_*/ScopeOfWork.md:266`.
- `DEL-17-01:SOW#CLM-004.r01`: the ledger records the deletion commit
  349a2ab33.
- `DEL-07-02:STATUS#remaining/R11`: the missing N7 intake JSON.

## T4B-C05 — Four-document residue (CP-01)

**Description.** This is text that still names `Datasheet.md`,
`Specification.md`, `Guidance.md` or `Procedure.md` as current surfaces. The
four documents were replaced by `ScopeOfWork.md` in the representation
conversion. The residue takes three forms:
- verbatim front matter of the retired kit, preserved by the SOW migration;
- maintenance and verification steps that review or require the four
  documents;
- records lists that name them as active evidence.

The disposition follows the F3 origin test:
- `STALE_SETUP_SPECIFICATION` for text present at the initial migration;
- `STALE_REVIEW_OR_EVIDENCE` for later declarations.

This is the largest class in the partition. It reaches every package that
has a Scope of Work, except PKG-00, whose deliverables use
`ArchitectureBasis.md`.

- **Population:** 336 rows, 82 deliverables, 17 packages.
- **Packages (rows):** PKG-01 (7), PKG-02 (19), PKG-03 (22), PKG-04 (12), PKG-05 (14), PKG-06 (18), PKG-07 (36), PKG-08 (21), PKG-09 (24), PKG-10 (31), PKG-11 (28), PKG-12 (29), PKG-13 (11), PKG-14 (1), PKG-15 (7), PKG-16 (2), PKG-17 (54).
- **Deliverables** (prefix `DEL-` omitted): 01-02, 01-03, 01-04, 02-01, 02-02, 02-03, 02-04, 03-01, 03-02, 03-03, 03-04, 03-05, 03-06, 03-07, 03-08, 04-01, 04-02, 04-03, 04-04, 04-06, 05-01, 05-02, 05-03, 05-04, 05-05, 06-01, 06-02, 06-03, 06-04, 06-05, 07-01, 07-02, 07-03, 07-04, 07-05, 07-06, 07-07, 07-08, 08-01, 08-02, 08-03, 08-04, 08-05, 08-06, 09-02, 09-03, 09-04, 09-05, 10-01, 10-02, 10-03, 10-04, 10-05, 11-01, 11-02, 11-03, 11-04, 11-05, 12-01, 12-02, 12-03, 12-04, 12-05, 13-01, 13-02, 13-03, 13-04, 14-05, 15-01, 15-02, 15-04, 16-01, 16-04, 17-01, 17-02, 17-03, 17-04, 17-05, 17-06, 17-07, 17-08, 17-09.
- **Signature** (Disposition · Tier · Baseline · Layers): STALE_SETUP_SPECIFICATION · LOCAL_DESIGN · NONE · RECORD (284); STALE_REVIEW_OR_EVIDENCE · LOCAL_DESIGN · NONE · RECORD (52).
- **Rows with adopted resolutions:** `DEL-10-04:SOW#CLM-015.r01` (FIELD); `DEL-10-04:SOW#CLM-016.s01` (FIELD); `DEL-10-04:SOW#CLM-020.s01` (FIELD); `DEL-10-04:SOW#CLM-021.s01` (FIELD); `DEL-10-04:SOW#CLM-022` (FIELD); `DEL-10-04:SOW#CLM-023` (FIELD); `DEL-10-05:SOW#CLM-013` (FIELD); `DEL-10-05:SOW#CLM-019` (FIELD); `DEL-10-05:SOW#CLM-020` (FIELD); `DEL-10-05:SOW#CLM-021` (FIELD).

**Owning authority.** NONE. CP-01 sets `AuthorityNeeded = NO`, a
deliverable catch-up that needs no decision.

**Route.** `R5_RECORD_REPAIR`.

**On-ruling mechanism.** Under the owner's separate R5 authorisation, run a
record-repair tranche per package. It would:
- replace references to the four documents with `ScopeOfWork.md`;
- drop or mark as historical the verbatim retired front matter;
- reword the verification and records checks to the SOW representation.

The repair would go through the ordinary change path, with no lifecycle
change. Where a block also carries other stale content (for example
`DEL-10-04:SOW#CLM-021.s01` also expects SEMANTIC_READY), the repair should
take the row's Notes into account.

**Risk if unrepaired.**
- Agents and reviewers are directed to documents that do not exist.
- Verification steps can never pass as written.
- The volume of the residue (336 rows) hides substantive findings in the
  same SOWs.

There is no product or baseline risk.

**Visible exceptions.** Ten FIELD rows in `DEL-10-04` and `DEL-10-05`. Their
OtherCorrections ask for VerificationClass NONE, or cited evidence, and
record an empty DecisionBasis. No value changes.

**Representative keys.**
- `DEL-03-02:SOW#CLM-021`: records name the four documents as active
  evidence, at
  `FREEZE:execution/PKG-03_*/1_Working/DEL-03-02_*/ScopeOfWork.md:303`.
- `DEL-03-02:SOW#CLM-001`: verbatim retired datasheet front matter
  (`doc_kind deliverable.datasheet`, created 2026-04-30).
- `DEL-03-07`: the four-document list, at
  `FREEZE:execution/PKG-03_*/1_Working/DEL-03-07_*/ScopeOfWork.md:239`.

## T4B-C06 — Stale status and memory surfaces (CP-05 and analogues)

**Description.** These are deliverable `_STATUS.md` and `MEMORY` surfaces
whose own metadata or history disagrees with other records.
- **56 rows are CP-05:** "Last Updated" is older than the file's own
  history entries. Typically "Last Updated" reads 2026-07-12 while a
  2026-07-16 DEC-081 claims-language entry follows it.
- **Three rows are analogues, not keyed to CP-05:**
  - `DEL-16-04:STATUS`: the file was edited on 2026-07-28 without a new
    "Last Updated".
  - `DEL-07-09:STATUS`: three Remaining items disagree with later records.
  - `DEL-01-01:MEMORY`: one refresh entry names DAG-006 where its run
    record names DAG-005.

- **Population:** 59 rows, 58 deliverables, 15 packages.
- **Packages (rows):** PKG-01 (5), PKG-02 (3), PKG-03 (4), PKG-06 (5), PKG-07 (7), PKG-08 (5), PKG-09 (2), PKG-10 (3), PKG-11 (5), PKG-12 (3), PKG-13 (4), PKG-14 (3), PKG-15 (4), PKG-16 (2), PKG-17 (4).
- **Deliverables** (prefix `DEL-` omitted): 01-01, 01-02, 01-03, 01-04, 02-01, 02-03, 02-05, 03-03, 03-04, 03-06, 03-08, 06-01, 06-02, 06-03, 06-04, 06-05, 07-01, 07-02, 07-04, 07-05, 07-06, 07-08, 07-09, 08-02, 08-03, 08-04, 08-05, 08-06, 09-02, 09-05, 10-01, 10-02, 10-03, 11-01, 11-02, 11-03, 11-04, 11-05, 12-03, 12-04, 12-05, 13-01, 13-02, 13-03, 13-04, 14-01, 14-03, 14-05, 15-01, 15-02, 15-03, 15-04, 16-02, 16-04, 17-02, 17-07, 17-08, 17-09.
- **Signature** (Disposition · Tier · Baseline · Layers): STALE_REVIEW_OR_EVIDENCE · LOCAL_DESIGN · NONE · RECORD (59).
- **Rows with adopted resolutions:** none.

**Owning authority.** NONE.

**Route.** `R5_RECORD_REPAIR`.

**On-ruling mechanism.** Under R5, one mechanical record-repair tranche:
- set "Last Updated" to the latest history entry;
- reconcile the three analogue surfaces as their RemainingWork states.

No lifecycle state changes. The Current State values are accurate on the
CP-05 rows. `DEL-01-01` is ISSUED, but its MEMORY row carries BaselineClass
NONE, and a history-log correction does not revise the ISSUED SOW.

**Risk if unrepaired.** Low. Readers misjudge how current a status file is.
Since 2026-09-19, work selection no longer runs through `_STATUS.md` (C9),
which limits the operational effect.

**Visible exceptions.** No resolution rows. Three rows have no
CanonicalSituation (the analogues above).

**Representative keys.**
- `DEL-03-03:STATUS`: "Last Updated" 2026-07-12, at
  `FREEZE:execution/PKG-03_*/1_Working/DEL-03-03_*/_STATUS.md:4`, against
  the 2026-07-16 entry at line 20.
- `DEL-16-04:STATUS`: the CP-05 analogue (commit 9b5207670, per the ledger
  Notes).
- `DEL-01-01:MEMORY`: the DAG token was rewritten by commit 11c5497f6.

## T4B-C07 — Remaining items overtaken by rulings or events

**Description.** These are `STATUS#remaining/*` items whose condition has
passed or has been ruled (CP-07), plus two related mismatches.
- **12 rows are overtaken by a ruling** (BaselineClass RULED_CRITERION).
  The rulings include:
  - D-72, the redesigned-product criteria ruled in part;
  - DEC-079 (D-07b, contributor intake);
  - DEC-089 (D-06b, signing posture);
  - DEC-078 (D-12);
  - DEC-018 (unit catalog and conversion constants);
  - D-40 / DEC-072 (no deliverable is CHECKING).
- **7 CP-07 rows are overtaken by events** (BaselineClass NONE): the
  PR #789 merge, the N7 V2 rereview PASS, and a closeout receipt.

The two non-CP-07 rows also carry BaselineClass NONE, which makes 9 NONE
rows in total:
- `DEL-05-04:STATUS#remaining/R01` declares "None", while the deliverable's
  own declaration holds an open stale-hash negative (F2).
- `DEL-13-02:STATUS#remaining/R01` is accurate and open, but its framing
  asserts unit-vocabulary evidence that no longer matches the frozen code.

- **Population:** 21 rows, 16 deliverables, 7 packages.
- **Packages (rows):** PKG-01 (1), PKG-05 (2), PKG-07 (10), PKG-09 (3), PKG-10 (3), PKG-11 (1), PKG-13 (1).
- **Deliverables** (prefix `DEL-` omitted): 01-03, 05-02, 05-04, 07-01, 07-02, 07-03, 07-06, 07-09, 09-01, 09-02, 09-05, 10-02, 10-03, 10-04, 11-02, 13-02.
- **Signature** (Disposition · Tier · Baseline · Layers): REMAINING_STATE_MISMATCH · LOCAL_DESIGN · RULED_CRITERION · RECORD (12); REMAINING_STATE_MISMATCH · LOCAL_DESIGN · NONE · RECORD (9).
- **Rows with adopted resolutions:** `DEL-10-04:STATUS#remaining/R02` (FIELD).

**Owning authority.** NONE, except that `DEL-13-02:STATUS#remaining/R01`
carries `AuthorityNeeded = REVIEW`: the human disposition of finding
PKG13-DEL-13-02-PKG02-001 is still pending.

**Route.** `R5_RECORD_REPAIR`, restating each item as its RemainingWork
says. The `DEL-13-02` item also needs the pending review disposition, under
the review workflow. That item is linked to the contested unit-vocabulary
cluster (T8).

**On-ruling mechanism.**
- An R5 record-repair tranche that restates or removes each item and cites
  the overtaking ruling or event.
- `DEL-05-04` R01: either record the held negative (FG-DEL-05-04-01) or
  rule it out of scope. The second option is an owner or scope choice if it
  is taken.

T9 also reviews Remaining state as a cross-cutting view. T4B's class
assignment stands alone.

**Risk if unrepaired.**
- Remaining sections misreport open work. Items that read as gated on
  already-ruled decisions invite duplicate asks to the owner.
- `DEL-05-04` hides an open negative.

**Visible exceptions.**
- FIELD: `DEL-10-04:STATUS#remaining/R02` (VerificationClass).
- AuthorityNeeded REVIEW: `DEL-13-02:STATUS#remaining/R01`.
- Hidden open negative: `DEL-05-04:STATUS#remaining/R01` (F2).

**Representative keys.**
- `DEL-07-02:STATUS#remaining/R03`: the pre-merge condition passed; PR #789
  merged at 8468a33c.
- `DEL-10-04:STATUS#remaining/R02`: awaiting D-06b, which was ruled by
  DEC-089. Cited implementation: `tools/release/package_release_artifact.py`.
- `DEL-01-03:STATUS#remaining/R01`: the D-07b gate was ruled by DEC-079 on
  2026-07-15.

## T4B-C08 — Other declarations overtaken by later records or rulings

**Description.** These are non-surface, non-Remaining declarations that
disagree with later records. Recurring themes:
- **The deliverable's own lifecycle or readiness text:** "SEMANTIC_READY",
  "setup status", or "Draft setup artifact", where `_STATUS.md` records
  IN_PROGRESS.
- **Review-finding states overtaken:** PKG-03 text says findings remain
  pending, although human Gate A dispositioned them ACCEPT_AS_IS / RESOLVED
  on 2026-06-05.
- **TBDs that a ruling has since closed:**
  - DEC-027 (maintainer roster, release authority and closed intake) is not
    reflected in the PKG-01 policy artifacts and `CONTRIBUTING.md`.
  - DEC-018 (unit catalog) is not reflected in DEL-09-01.
- **Seven `PARTIALLY_IMPLEMENTED` rows**, where the governed artifact itself
  needs updating: five in PKG-01 (DEC-027 not reflected) and two in
  DEL-17-01 (DAG-005 admitted without intake step 1).

`DEL-01-01:SOW#CLM-011/AC-01-01-04` sits in an ISSUED deliverable but
carries BaselineClass NONE. Its RemainingWork points at the maintainer
policy artifact, not the ISSUED SOW text. If a repair touches the SOW
criterion itself, it needs the ISSUED change path (see C02).
- **Counts and graph pointers overtaken:** dependency-row counts that name
  DAG-006 against DAG-010, a resolved conflict-table entry, and AB
  realized-artifact descriptions that no longer match the tree.

- **Population:** 59 rows, 32 deliverables, 13 packages.
- **Packages (rows):** PKG-00 (3), PKG-01 (5), PKG-02 (3), PKG-03 (16), PKG-06 (5), PKG-07 (3), PKG-08 (4), PKG-09 (4), PKG-10 (1), PKG-11 (4), PKG-12 (6), PKG-13 (2), PKG-17 (3).
- **Deliverables** (prefix `DEL-` omitted): 00-01, 00-03, 01-01, 01-02, 01-03, 01-04, 02-04, 03-01, 03-02, 03-03, 03-07, 03-08, 06-02, 06-04, 06-05, 07-05, 07-07, 07-08, 08-01, 08-02, 08-04, 08-06, 09-01, 10-04, 11-01, 11-02, 11-03, 12-03, 12-05, 13-04, 17-01, 17-02.
- **Signature** (Disposition · Tier · Baseline · Layers): STALE_REVIEW_OR_EVIDENCE · LOCAL_DESIGN · NONE · RECORD (43); STALE_SETUP_SPECIFICATION · LOCAL_DESIGN · NONE · RECORD (7); PARTIALLY_IMPLEMENTED · LOCAL_DESIGN · NONE · RECORD (7); STALE_REVIEW_OR_EVIDENCE · LOCAL_DESIGN · NONE · RECORD;LIFECYCLE (2).
- **Rows with adopted resolutions:** `DEL-00-01:AB#open-holds-and-routed-questions.s01` (CONTESTED); `DEL-06-02:SOW#CLM-024.r05` (WEAK); `DEL-06-02:SOW#CLM-024.r06` (WEAK); `DEL-08-02:SOW#CLM-003` (OBSERVED); `DEL-10-04:SOW#CLM-003.r08` (OBSERVED).

**Owning authority.** NONE (`AuthorityNeeded = NO` on all 59).

**Route.** `R5_RECORD_REPAIR`.

**On-ruling mechanism.** An R5 record-repair tranche. It would:
- restate each declaration against the record that overtook it;
- for the seven `PARTIALLY_IMPLEMENTED` rows, update the governed artifacts
  themselves (the PKG-01 policy TBD blocks, `CONTRIBUTING.md`, and the
  DEL-17-01 source register or `_REFERENCES.md` for DAG-005) to reflect the
  recorded rulings.

No lifecycle state is changed by the repair. Where the text states the
deliverable's own readiness, the repair records the current `_STATUS.md`
state and does not advance it.

**Risk if unrepaired.**
- Deliverable text contradicts recorded human rulings, for example on
  closed contributor intake or the unit catalog.
- Governance artifacts under PKG-01 keep listing, as TBD, authority that
  has already been recorded. For contributor-facing text, this misstates
  the project's posture.

**Visible exceptions.**
- **CONTESTED:** `DEL-00-01:AB#open-holds-and-routed-questions.s01`. The
  verifier would split the row: the overtaken items keep these fields, and
  the state-library clause would take CP-10's fields (IMPLEMENTED_DIFFERENTLY
  · AUTHORITY_UNCLEAR · PROJECT_BASELINE · OWNER). That split part is an
  owner item already on the W2 list ("holds that code settled with no
  ruling") and should reach R4 from there. It is not a T4B record repair.
- **WEAK:** `DEL-06-02:SOW#CLM-024.r05` and `CLM-024.r06`. Their effective
  disposition is STALE_REVIEW_OR_EVIDENCE, where the sealed value was
  STALE_SETUP_SPECIFICATION. The carried correction notes that
  `tools/validation/validate_enum.py` does exist, at the repository root
  (confirmed at the freeze).
- **OBSERVED:** `DEL-08-02:SOW#CLM-003` (RECORD_DRIFT against
  DOC_BEHIND_CODE across worker groups) and `DEL-10-04:SOW#CLM-003.r08`
  (the cause of the SEMANTIC_READY lifecycle-target question, for R3 to
  harmonise). The routing is the same under either cause.

**Representative keys.**
- `DEL-03-08:SOW#CLM-015`: the conflict table says the findings await human
  disposition. The text is at
  `FREEZE:execution/PKG-03_*/1_Working/DEL-03-08_*/ScopeOfWork.md:204`; the
  Gate A disposition came on 2026-06-05.
- `DEL-01-03:SOW#CLM-012/DEL-01-03-REQ-08`: `CONTRIBUTING.md` does not
  reflect DEC-027's closed intake (ledger Notes, CONTRIBUTING.md L18).
- `DEL-12-05:SOW#CLM-020.r09`: the text says `_STATUS.md` remains
  SEMANTIC_READY; it is IN_PROGRESS.

## Coverage

- **Population.** `R3/CORPUS_CLAIMS.csv`, filtered to rows with
  `Divergent = YES` and an effective CauseTag in {REPRESENTATION_MIGRATED,
  RENAME_OR_IDENTITY, RECORD_DRIFT}: 625 rows (339 + 86 + 200). This matches
  the R3 plan's 625.
- **Output.** `T4B_CLASSES.csv` has 625 body rows and the sentinel
  `#END,,625`.
- **Check.** A read-only Python script (`csv` module) confirmed:
  - every population key appears exactly once in the output, and no output
    key falls outside the population;
  - the (ClaimKey, DeliverableID) pairs match;
  - every row has one of the eight class IDs.
- **Class counts.** C01 86, C02 5, C03 42, C04 17, C05 336, C06 59, C07 21,
  C08 59. Total 625.
- **The rename set.** All 86 RENAME_OR_IDENTITY rows are in C01, and C01
  holds no other cause.
- **Ledger join.** All 625 rows were found in the sealed forward ledgers,
  excluding `superseded_*/` folders. Two rows carry resolved values
  (`ValuesResolved = YES`, the `DEL-06-02` WEAK pair). Effective values were
  used throughout.
- **Draft files.** No `RESOLUTIONS_DRAFT*.csv` or merged-draft file was
  read.
- **Other tasks.** No other task's files under `R3/TASKS/` were read.

## R3 observations

These are observations, not corrections. The effective values stand.

1. **Inconsistent `AuthorityNeeded` on SR-1 sub-claims.** The same
   `CONTEXT#architecture-basis-injection.s01` statement carries
   `AuthorityNeeded = OWNER` on `DEL-13-01`, `DEL-13-03` and `DEL-13-04`,
   and `NO` on the other 39 rows. Direction 8 makes the PKG-00 status
   advance an owner approval, and the statements' repair depends on it. So
   OWNER is the better-supported value corpus-wide. C03 is routed to the
   owner either way.
2. **Kept identifiers without their CP-04 variant rows.** No row in this
   partition carries the persistence variant for
   `openpipestress-projects.sqlite3`. It is mentioned only in the Notes of
   the default-variant row `DEL-02-05:SOW`, and it is active code at
   `FREEZE:apps/desktop/src-tauri/src/lib.rs:32`. No row in this partition
   mentions `openpipestress_result_semantics_v0_2`, which is active at
   `FREEZE:schemas/analysis_run.v0.2.schema.json:194`. Both are named by
   CP-04. The R4 rename ruling should cover them explicitly, whether or not
   another partition keys them.
3. **Unrecorded grammar identifier.** The DEC-022 grammar identifier
   `open_pipe_stress_declared_expression` is not recorded in DEL-11-02's
   ledger (OBSERVED on `DEL-11-02:SOW`). It should be named in the rename
   ruling's scope.
4. **The `DEL-01-01` rename row is routed twice.** Through FG-DEL-01-01-02
   and through C01. C01 carries it, and the owner's choice (C01, item 4)
   should make it one route.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
