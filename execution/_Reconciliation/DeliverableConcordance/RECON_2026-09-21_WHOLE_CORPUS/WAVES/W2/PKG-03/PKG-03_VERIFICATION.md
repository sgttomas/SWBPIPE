VERDICT: RERUN DEL-03-07

# PKG-03 verification — wave W2 (DOUBLE sampling)

This is a fresh verification that used only the evidence. The verifier is a TASK (Type 2) whose parent is HELP_HUMAN Agent 0 of run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION`. It worked under
`briefs/R2-VERIFIER_brief.md` (sha256 `47fb3c52…5b00`, verified). It is
independent of the PKG-03 manager and workers. The evidence checkout is `00115c719`
(`{FREEZE}`), read-only. No builds, test runs or git writes took place. The verifier did
not read the July ledgers. The manager's transcripts and the workers'
returns were read only for their flags. They are not evidence about the claims.

The verdict applies the brief's rule as written. DEL-03-07 has one firm error in
a 100%-sampled class (F2/F4 `GAP_WORDING_CHECKED` rows), and correcting it
changes the tier from a quiet row to `INVARIANT` (§4, D1). Every
other deliverable is `ACCEPT WITH CONTESTED ROWS`. §7 says why Agent 0 may
prefer to settle D1 through `RESOLUTIONS.csv` (F6) instead of a full rerun.

## 1. Mechanical checks

- **Seals.** For all eight forward ledgers, the recomputed SHA-256 equals
  `<DEL>_SEAL.txt` and the manager return.
- **Single mode.** `validate_ledger_v2.py --reverse --inventory
  ROUTING/PKG-03_capabilities.csv --notes-gap` passes with 0 findings for all
  eight ledgers. This holds with `--repo-root` set to the repository checkout
  and with it set to `{FREEZE}`.
- **Batch mode.** Over the eight PKG-03 ledgers: `PASS batch of 8 ledgers: 0
  consistency findings`. No `WAVES/W2/RESOLUTIONS.csv` exists.
- **Cross-package batch.** The run covered all 31 W2 ledgers present plus W1, with
  `--resolutions WAVES/W1/RESOLUTIONS.csv`. It raised 8 findings, none of them on
  a PKG-03 row. All of them are in PKG-04.
- **Canonical conformance.** Checked on 100% of rows:
  - CS-01, CS-02, CS-04, CS-06 (OK) and CS-07 are uniform across all eight
    deliverables.
  - CP-09 matches `EVIDENCE_MAP.csv`. The ALIGNED rows are in DEL-03-01, 02, 05
    and 07, where `AnyPassMatchesFrozen=YES`. The `EVIDENCE_OVERTAKEN` rows are in
    DEL-03-03, 04, 06 and 08, where it is `NO`.
- **F3 origin spot checks.** Five strings were located in the frozen history
  with `git grep` at `7bee9ae41` and `1b62eb5b8`, and with `git log -S`. All five
  confirm the worker's stated origin:
  - DEL-03-04 CLM-020 and CLM-021;
  - DEL-03-05 CLM-004;
  - DEL-03-06 CLM-005 and CLM-020;
  - DEL-03-02 CLM-020.

## 2. Sample selection (deterministic)

- **Method.** Each row goes to the first class that fits, in the order A to H.
  Within each class, per deliverable, rows are sorted by SHA-256 of the
  `ClaimKey`. The sample takes the first ⌈rate×n⌉ rows. Class F instead sorts
  priority rows first: `LOW`/`MEDIUM` confidence, `NONE_FOUND` verification or
  `NONE` verification class. The rows are then sorted by hash.
- **Rates (DOUBLE).** Classes A, B and C are 100%, D is 50%, E is 50%, F is 40%
  and G is 20%. Class H, the ALIGNED non-normative rows that fall in no brief
  class, is sampled at 10% and disclosed as an addition.

| Class | Meaning | Rate |
|---|---|---|
| A | INVARIANT / ACCEPTED_DIVERGENCE / AUTHORITY_CONFLICT / UNKNOWN / LRR / ISSUED / PROTECTED_CHECK / FROZEN_CONTRACT | 100% |
| B | `SharedTextCount>1`; `.sNN` rows inherit the parent's count | 100% |
| C | ALIGNED with `GAP_WORDING_CHECKED:` or `OPEN_ACTION:` | 100% |
| D | ALIGNED with `PRODUCT_CALLER: NONE` | 50% |
| E | other non-aligned | 50% |
| F | ALIGNED normative | 40% |
| G | structural or canonical | 20% |
| H | other ALIGNED (disclosed addition) | 10% |

The sample is 284 forward rows. Of these, 68 are ALIGNED normative rows.

## 3. Per-deliverable results

`AN` is the number of ALIGNED normative rows sampled. `FFA` is the firm
false-alignment rate among them.

| DEL | Rows | Sampled (A/B/C/D/E/F/G/H) | Firm | Weak | Field | AN | FFA |
|---|---|---|---|---|---|---|---|
| DEL-03-01 | 95 | 36 (0/7/1/0/12/9/4/3) | 0 | 3 (W1, W2, W4) | 0 | 9 | 0.0% |
| DEL-03-02 | 85 | 33 (0/8/3/0/7/9/3/3) | 0 | 2 (W1, W2) | 0 | 12 | 0.0% |
| DEL-03-03 | 64 | 27 (0/11/0/0/6/5/3/2) | 0 | 2 (W1, W2) | 0 | 5 | 0.0% |
| DEL-03-04 | 69 | 28 (3/7/3/0/6/3/4/2) | 0 | 2 (W3, W5) | 0 | 4 | 0.0% |
| DEL-03-05 | 72 | 30 (2/7/2/0/7/6/4/2) | 0 | 4 (W3, W4, W6, W7) | 0 | 7 | 0.0% |
| DEL-03-06 | 73 | 29 (0/8/0/0/10/5/3/3) | 1 (D2) | 1 (W3) | 0 | 5 | 20.0% |
| DEL-03-07 | 103 | 42 (2/10/6/0/7/10/4/3) | 1 (D1) | 1 (W8) | 7 (X1) | 14 | 7.1% |
| DEL-03-08 | 124 | 59 (3/12/0/26/11/1/4/2) | 0 | 0 | 0 | 12 | 0.0% |

- **Firm error rates on sampled rows.** DEL-03-06 is 1/29 (3.4%) and DEL-03-07
  is 1/42 (2.4%). Both are under the 10% trigger.
- **DEL-03-07 triggers a rerun** under the second clause: a firm error in a
  100%-sampled class that changes the tier (D1).
- **DEL-03-06's D2** is in the 40% class and changes no owner routing, so it
  triggers nothing.

## 4. Package-level firm false-alignment rate

The rate is **2 / 68 = 2.9%**, within the 5% scale-out gate.

## 5. Disagreements

### Firm

**D1 — `DEL-03-07:SOW#CLM-021`** (class C; REQUIREMENT, Principles)

- **The row says.** ALIGNED. Its Notes defer the unit-coverage question to
  CLM-003.r06 and CLM-009.r05, and end with `GAP_WORDING_CHECKED` ("the checker
  introduces nothing").
- **What I found.** The sixth principle reads "Maintain unit metadata for
  numeric engineering values; do not introduce unitless accepted records."
  - In `core/library_import/provenance_checker.py`, `_validate_nested_values`
    (L296–331) skips every object without a `magnitude` key. `_walk_value_objects`
    yields only dicts, so bare numbers are never inspected.
  - The Rust mirror does the same: `library_import_document/src/lib.rs::validate_nested_values`,
    L425–435.
  - Schema validation runs only for the hanger kind (L742–800).
  - So a material, section or component record carrying a bare numeric value is
    accepted without a unit finding. The row's own Notes record this gap, and F1
    bars ALIGNED "even when the same gap is also recorded on another row". The
    wording clause does not cure an unmet element.
- **Right values.** PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · NONE ·
  BASELINE, with `FindingGroup` FG-DEL-03-07-02. These match the worker's own
  CLM-003.r06 and CLM-009.r05.

**D2 — `DEL-03-06:SOW#completion-and-reliance-basis-epistemology/AC-001`**
(class F; ACCEPTANCE)

- **The row says.** ALIGNED. Its Notes argue that "unresolved solver mappings"
  stays true of the contract because the contract defines no per-axis mapping.
- **What I found.** AC-001 requires the contract to preserve "unresolved
  taxonomy and solver mappings".
  - The frozen component contract already carries the ruled mapping for the
    expansion-joint family. The `solver_consumption` enum is in
    `schemas/component.schema.yaml` L498–505. The fixture
    `fixtures/component/invented_component_library_valid.json` gives the family
    contract `mechanics_geometry_and_user_flexibility`, per DEC-045.
  - The same ledger disposes the same substance as overtaken on CLM-005 (the
    "per-axis stiffness shape and solver DOF mapping" TBD), CLM-027 and CLM-028,
    all STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE under FG-DEL-03-06-01.
  - The acceptance row therefore records as current an element that its own
    ledger records as overtaken.
- **Right values.** STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE · LOCAL_DESIGN ·
  NONE · RECORD · NO, with FG-DEL-03-06-01. SCOPE_REDIRECTED_BY_RULING is also
  defensible.
- **Confidence.** Medium.

### Weak (readings the conventions should settle)

**W1 — Shared `.s01` split three ways.**
`DEL-03-01/02/03:CONTEXT#architecture-basis-injection.s01` take
STALE_REVIEW_OR_EVIDENCE. `DEL-03-04/05/06/07/08` take STALE_SETUP_SPECIFICATION
for the identical text: the parent body is `696d7086…`, `SharedTextCount` 8, all
in PKG-03.

- G1 reads the PKG-00 `SEMANTIC_READY` readiness statement as a "review
  state/metadata" F3 exception. The text is first present at `7bee9ae41`, and a
  lifecycle-readiness statement is not a named exception. I would take
  STALE_SETUP_SPECIFICATION.
- The pair is a shared-situation conflict (§6).

**W2 — Missing `.s02` in DEL-03-01/02/03.** On
`DEL-03-01/02/03:CONTEXT#architecture-basis-injection`, no `.s02` exists for
the "Still TBD" list. CS-04 directs a `.sNN` row for "a 'Still TBD' item since
ruled". DEC-017 settled the physical project package/container item, and
DEL-03-04 to 08 each split `.s02`. G1's note ("make no claim this deliverable
contradicts") is a narrower reading of "diverges for this deliverable".

**W3 — Schema-only SOWs after product work landed.** G2 records DOC_BEHIND_CODE
on the DEL-03-04, 05 and 06 SOWs. G1 does not for DEL-03-03, although the
DEC-070 curved-bend macro-element consumes its bend fields
(`core/product_physics/src/lib.rs` L89–100 and L3860–3900). Within G2:

- DEL-03-06 treats the solver mapping as overtaken by DEC-045.
- DEL-03-05 AC-001 and its SOW row keep "exact solver treatment TBD" as
  accurate, although DEC-045 ruled `mechanics_geometry_only` for the
  rigid/semi-rigid family.

Rows affected in the sample are DEL-03-05 AC-001 (ALIGNED; I lean
STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING) and the DEL-03-04 and
DEL-03-06 `.s02` cause (see W7).

**W4 — AB-00-04 round-trip treated differently.**

- DEL-03-01 `CLM-011/REQ-03-01-007` and VER-001 are PARTIALLY_IMPLEMENTED, on
  the ground that "no material round-trip harness located".
  - The frozen tree round-trips materials inside model snapshots. See
    `apps/desktop/src-tauri/src/lib.rs::local_project_store_uses_sqlite_fts5_and_round_trips_model_snapshot`,
    L5672ff, whose `materials[]` carry E and G. The envelope round-trip tests
    are in `tests/test_project_persistence_service.py`.
  - The requirement's own text still records a library-level harness as TBD,
    so the disposition is defensible. The note overstates the absence.
- DEL-03-05 `CLM-010.r08` is ALIGNED on envelope-level round-trip. This is the
  same evidence class judged the other way.
- R3 should settle the reading once.

**W5 — Layer for unit-check invariant rows.** DEL-03-04 `RQ-005` uses layer
VALIDATION, while G3 uses BASELINE for the same subject (DEL-03-07 CLM-003.r06
and CLM-009.r05, DEL-03-08 RQ-003). C5 has no units layer, and verification is
not validation (A5). The conventions should name one.

**W6 — DEL-03-05 `SOW#CLM-014`.** Disposed UNKNOWN · EVIDENCE_NOT_LOCATED with a
smallest check that the worker had already run: the folder and fixture sidecars
were searched. The declaration cites documentation that does not exist, so by
analogy with CP-08 it could take STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT. The
tier is unchanged.

**W7 — `.s02` cause split.** On `CONTEXT#architecture-basis-injection.s02`, G2
(DEL-03-04/05/06) records DOC_BEHIND_CODE on the strength of lockfiles, CI and
export schemas. G3 (DEL-03-07/08) records SCOPE_REDIRECTED_BY_RULING on the
strength of DEC-017 and PKG-17. The body is the same. See §6.

**W8 — DEL-03-07 `SOW#CLM-016`, step 6** ("confirm unit metadata where values
are present"). The row is ALIGNED as performable. The same bare-number gap as D1
limits what the step can confirm. This is borderline, and I do not re-dispose it.

### Field

**X1 — DEL-03-07: F7 marker missing (G3 self-disclosed).** Sampled ALIGNED rows
whose implementation evidence cites only the Python checker, which has no
product caller, lack `PRODUCT_CALLER: NONE`. The rows are:

- `CLM-017.r01`, `CLM-017.r04`, `CLM-017.r06` and `CLM-024.r01`;
- `completion-and-reliance-basis-epistemology/AC-001`;
- `output-and-evaluation-matrix/OUT-001`;
- `CONTEXT#anticipated-artifacts`.

The dispositions stand: the claims are about the checker, and the Rust port does
have product callers.

### Rows confirmed

I checked the rest of the sample against `{FREEZE}` and found no disagreement.
That includes every INVARIANT and UNKNOWN row:

- DEL-03-04 RQ-002, RQ-005 and RQ-006;
- DEL-03-05 CLM-013;
- DEL-03-07 CLM-003.r06 and CLM-009.r05;
- DEL-03-08 RQ-003, RQ-004 and CLM-026.

Confirmed behaviour includes:

- the material schema's DEC-077/092 interpolation text and the retired
  OpenDecision topic;
- the `UNIT_INPUT_INVALID` product checks;
- `Quantity(10,"kg","length")` being accepted by
  `core/section_properties/calculator.py` (`_validate_quantity` and
  `_single_unit` check the dimension label and string equality only);
- redistribution status never being inspected;
- optional corrosion allowance and mill tolerance treated as "no reduction", as
  the SOW declares;
- the PDU-019 and PDU-013 holds (the `OPEN_ACTION` targets are correct).

## 6. Batch consistency and shared-situation conflicts

- **Validator batch.** Passes, as §1 records.
- **Validator gap.** Batch mode compares only keyed rows by `BodySHA256` or
  `CanonicalSituation`. It does not compare `.sNN` sub-claims of a shared body,
  so the following conflict passed unflagged.
- **Conflict: the shared `CONTEXT#architecture-basis-injection` body
  (`696d7086…`, all 8 PKG-03 deliverables).**
  - `.s01` (PKG-00 readiness): 3 rows STALE_REVIEW_OR_EVIDENCE (G1) against 5
    rows STALE_SETUP_SPECIFICATION (G2, G3). The cause, SCOPE_REDIRECTED_BY_RULING,
    agrees.
  - Across packages, the same `.s01` elsewhere takes RECORD_DRIFT (DEL-02, 04,
    07) or SCOPE_REDIRECTED_BY_RULING (DEL-01, 03, 16), under either stale class.
  - `.s02` (Still TBD): absent in DEL-03-01/02/03. DOC_BEHIND_CODE in 04/05/06.
    SCOPE_REDIRECTED_BY_RULING in 07/08.
  - Agent 0 should resolve both sub-claims once, corpus-wide. My reading is
    STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING for both, with `.s02`
    required wherever DEC-017 applies.
- **No other shared-body conflicts** appear in PKG-03.

## 7. Reverse pass

- **Checks made.**
  - 100% of the 51 non-NOT_MINE answers: CLAIMED_BY 11, PARTIAL 14, COVERS 21,
    CONSTRAINS 4, UNKEYED 1.
  - A 10% hash-selected NOT_MINE sample: 296 of 2,957.
  - F5: the NOT_MINE answers whose EntryPoints hit a path the deliverable's
    forward ledger cites total 102. Of these, 24 (≥20% per deliverable) were
    sampled.
- **Findings.**
  - All sampled F5 reasons address the capability. G2's reasons are partly
    templated but name the capability first.
  - No NOT_MINE in the sample looks like a missed claim.
- **Distribution.** SAMPLE-routed rows: 206 NOT_MINE, 1 COVERS, 1 UNKEYED (1.0%
  non-NOT_MINE, 208 answers). AREA rows: 49 of 2,800 non-NOT_MINE (1.75%). The
  rates are comparable. Most SAMPLE rows are VIEW, SHELL or WSUI surfaces that
  are obviously foreign, so the comparison is weak evidence of independence.
- **Capabilities claimed by more than one PKG-03 deliverable.** These are
  consistent, apart from the answer-type splits noted:

  | Capability | Answers |
  |---|---|
  | RC-03-0073 component schema | 02 CLAIMED_BY; 03/04/05/06 PARTIAL (family slots) |
  | RC-03-0109 component fixtures | 02 CLAIMED_BY; 03–06 PARTIAL; 07 COVERS |
  | RC-03-0326 section schema | 02 CLAIMED_BY; 08 PARTIAL (calculator hooks) |
  | RC-03-0162 material fixture | 01 CLAIMED_BY; 07 COVERS |
  | RC-03-0072 and RC-03-0107 import checker | 07 CLAIMED_BY; 02/03 CONSTRAINS; 04/05/06 COVERS |
  | RC-03-0294 component insertion | 04/05/06 PARTIAL; 03 NOT_MINE |

  - The import-checker split is weak. CONSTRAINS (the AB-00-07 basis) and
    COVERS are both defensible, but the same relation should take one answer.
  - For component insertion, DEL-03-03 answered NOT_MINE although the capability
    inserts bends too. Its reason (the SOW excludes downstream operation
    behaviour) is specific. This is the same scope split as W3.
- **Suspected missed claims.** None firm. The DEC-070 curved-bend solver
  capabilities (RC-03-0319, 0032, 0173, 0220) are NOT_MINE for DEL-03-03,
  because its SOW excludes solver implementation. That is consistent, but R3
  should confirm where bend-field consumption is owned.
- **Anchored answers.** Two of the non-NOT_MINE SAMPLE answers carry EntryPoints
  that the answering deliverable's own forward ledger cites:
  - RC-03-0091 (`core/product_physics::derive_pipe_section`), answered UNKEYED by
    DEL-03-08;
  - RC-03-0373 (`physical_to_analytical/contract.py`), answered COVERS by
    DEL-03-05.

  Both answers are reasonable. RC-03-0091 conflicts with DEL-03-08's own forward
  rows CLM-003.r06 and RQ-007, which assign the product routine to "other
  deliverables". R3 should settle ownership of SOW-051's product-runtime
  realization.
- **G3 routing note.** G3 says the Rust port has no hanger path. That is wrong:
  `validate_hanger_import` exists (`library_import_document/src/lib.rs` L797).
  G3's PARTIAL answer on RC-03-0198 is correct.

## 8. What the owner must see

1. **DEL-03-07 rerun trigger.** D1 is one row. The same gap is already carried
   as INVARIANT under FG-DEL-03-07-02 on two sibling rows, so no finding is lost
   if Agent 0 records D1 and X1 in `WAVES/W2/RESOLUTIONS.csv` (F6) instead of
   rerunning. The brief's rule nonetheless says rerun, and that choice belongs
   to Agent 0 or the owner.
2. **Confirmed possible defects in product code.** These are recorded by the
   workers and confirmed in the frozen code:
   - The library-import checker, in both Python and Rust, accepts bare numeric
     values in material, section and component records without a unit finding.
   - The section calculator never checks a unit string against its dimension.
   - The section calculator never checks redistribution status.
   - The section calculator drops input provenance on its derived outputs.
3. **The product runtime's section and mass routine is separate** (Rust,
   `core/product_physics`). It leaves out the corrosion allowance that the
   Python calculator applies. Its ownership is unsettled (RC-03-0091).
4. **A corpus-wide shared-situation conflict** on the PKG-00 readiness and
   Still-TBD sub-claims (§6). The batch validator does not check `.sNN`
   sub-claims of shared bodies. This is a tooling gap worth closing before the
   next wave.
5. **Two cross-worker readings need one answer each:**
   - SOWs that describe only the schema while DEC-045 and DEC-070 product work
     has landed (W3);
   - envelope-level against library-level AB-00-04 round-trip (W4).
6. **Worker custody notes.** G2's scratch files vanished after sealing, and G3
   listed another worker's folder. The seals still match, and no `_scratch_*`
   files remain in `WAVES/W2/PKG-03/`.
7. **Gate evidence.** The `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55` evidence was
   recorded on candidate `9d55bce`, not the freeze `00115c7`. A6 sanctions it,
   and every row says "not rerun". This is recorded for the owner's awareness,
   not as a disagreement.

Agent dispositions are not owner rulings. Standard claim fence applies (F-PIP-2;
claims taxonomy per DEC-081).
