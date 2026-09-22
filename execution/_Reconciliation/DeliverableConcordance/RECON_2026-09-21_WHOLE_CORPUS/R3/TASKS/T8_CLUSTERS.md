# T8 — Contested corpus clusters (R3)

TASK T8 in R3 of run HELP-HUMAN-PIPING-20260921-RECONCILIATION reads the seven
corpus-wide clusters that `WAVES/W3/W3_ASSESSMENT.md` ("Corpus-wide clusters left
contested") left for one R3 resolution. It covers 176 claim rows in 90
deliverables and 17 packages (172 divergent, 4 ALIGNED), each in exactly one cluster. The members come from
the adopted resolutions (CONTESTED, OWNER_CONFIRMED, WEAK, FIELD, FIRM, OBSERVED
and RESOLVED_PAIR rows that carry the cluster's text), the verification reports
that name the cluster, and `R3/CORPUS_CLAIMS.csv` (FindingGroup members and
ledger rows that cite the cluster's subject). Five clusters get a proposed
reading that Agent 0 can record without an owner ruling, because they turn on
convention text and frozen evidence: SR-1, unit vocabulary, F1 on CONTEXT, tier
of in-scope REQs, and acceptance-workflow cause. Two clusters need an owner
decision on substance, and the proposed cause or field reading only shapes the
packet: DEC-009 (whether Python engines under `core/` may stand against the
ruled Rust core) and the export plan (whether to restore, re-point or retire the
deleted PLAN-EXPORT-INTEROP). No row value changes. Wherever a proposed reading
differs from an effective value, it is recorded as an `R3_OBSERVATION` in
`T8_ROWS.csv`.

## Overview

| ID | Cluster | Rows | Packages | Proposed reading | Owner decision? | Main route |
|---|---|---|---|---|---|---|
| T8-K1 | SR-1 (PKG-00 at SEMANTIC_READY: cause) | 87 (81 core + 6 adjacent) | PKG-01 to PKG-16 | Cause `RECORD_DRIFT` | No new decision. The status advance is already an R4 item under Direction 8 (T9) | R5_RECORD_REPAIR |
| T8-K2 | DEC-009 (Python engines against the Rust core) | 20 | PKG-13, 14, 15, 17 | `IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · PROJECT_BASELINE · BASELINE · OWNER` | **Yes** | OWNER_DECISION |
| T8-K3 | Unit vocabulary | 23 | PKG-03, 13, 15 | Tier follows the gap: `INVARIANT` for unit-safety gaps, `PROJECT_BASELINE` for the enum catch-up. AuthorityNeeded `REVIEW`. DEL-15-03 split out | Only the DEL-15-03 redaction part (already an owner item) | CODE_FIX_CANDIDATE |
| T8-K4 | F1 on CONTEXT | 3 | PKG-05, 14 | F1 reaches CONTEXT-typed rows | No | CODE_FIX_CANDIDATE (via the FindingGroup) |
| T8-K5 | Export plan | 24 | PKG-17 | CP-08 fields with `OWNER`; disposition class set by F3 origin | **Yes** | OWNER_DECISION |
| T8-K6 | Tier of in-scope REQs | 7 | PKG-07, 14, 17 | Scope-item test: `PROJECT_BASELINE` only when the scope item names the unmet element | No (DEL-07-05 keeps its existing OWNER routing) | mixed |
| T8-K7 | Acceptance-workflow cause | 12 | PKG-05, 15 | `DEFERRED_BY_RULING` (PRD §21.3 plus DEC-081) | No | NO_ACTION |

Owning authority for the convention readings (K1, K3, K4, K6, K7): Agent 0
records them as R3 readings. Because C3, C7 and F1 are RULED conventions, the
R4 packet should show them to the owner for confirmation, as the SEMANTIC_READY
reading was shown under Direction 8. That confirmation is not a new decision on
substance.

---

## T8-K1 — SR-1: cause of the "PKG-00 at SEMANTIC_READY" sub-claim

**What is split.** Every `CONTEXT#architecture-basis-injection.s01` row (`.s02`
for DEL-16-0x) is `STALE_REVIEW_OR_EVIDENCE · LOCAL_DESIGN · NONE · RECORD ·
NO`. The owner confirmed that disposition (Direction 8,
`OWNER_DIRECTIONS.md:139-160`). The cause splits along worker lines, 42
`RECORD_DRIFT` against 39 `SCOPE_REDIRECTED_BY_RULING` across the 81
OWNER_CONFIRMED rows (for example PKG-08 G1/G2, PKG-09 G1/G2, PKG-12 G1/G2 and
PKG-14 G1/G2, per their verification reports §5).

**Evidence.**
- The statement restates SCA-001, which is still in the frozen decomposition.
  `SOFTWARE_DECOMP.md:432` authorizes PKG-00 at `SEMANTIC_READY` as an
  architecture-basis candidate for sealed brief injection, and the glossary row
  at `:86` does the same.
- D-43/SCA-006 (`SOFTWARE_DECOMP.md:35`) is recorded as a "scope-neutral
  currency amendment": the kits were consolidated into `ArchitectureBasis.md`,
  and "no scope item, dependency, requirement, or package membership changes".
  No located ruling changed what the sentence asks for. PKG-00 still supplies
  architecture-basis constraints.
- All eight PKG-00 `_STATUS.md` files read `**Current State:** IN_PROGRESS`
  (for example `execution/PKG-00_…/DEL-00-01_…/_STATUS.md:3`). The readiness
  label disagrees with the status records.
- Direction 8 says the status "is outdated as a status and should be advanced"
  on later owner approval. This is a record-state lag, not a redirected scope.
- The superseded gate rule is a different block (CS-05, which already carries
  `SCOPE_REDIRECTED_BY_RULING`). The G1 notes (for example
  DEL-09-01 `.s01`, "the PKG-00 gating it describes was superseded (D-43; CS-05
  basis)") carry the CS-05 reasoning over to a sentence that states a
  readiness level, not a gate.

**Proposed reading.** `RECORD_DRIFT` ("status … declarations disagree with other
records", C7). The remaining gap is the stale readiness label, and C7 says the
gap's cause wins. The 39 rows that carry `SCOPE_REDIRECTED_BY_RULING` get an
R3_OBSERVATION; no row changes.

**Adjacent rows (6).** These are a deliverable's own readiness target or
state: DEL-06-02 CLM-024.r06 (WEAK), DEL-09-05 CLM-003 (CONTESTED; the
verifier's `.sNN` split was not minted), DEL-10-05 CLM-002.r05 (CONTESTED),
DEL-10-04 CLM-003.r08, DEL-10-01 CLM-004 and DEL-10-03 CLM-004 (all OBSERVED).
The same reading, `RECORD_DRIFT`, fits their readiness element. For the two
DEL-10 identification tables, the worker read the remaining gap as artifact
form (`DOC_BEHIND_CODE`). Both causes are defensible under the one-cause rule,
and no change is proposed there.

- **Owning authority:** NONE for the cause (convention reading). OWNER for the
  separate status advance (Direction 8 item 2, a T9 lifecycle item).
- **Route:** R5_RECORD_REPAIR, after the owner acts on the status advance.
- **On-ruling mechanism:** Agent 0 records one cause for SR-1 in
  `R3_SYNTHESIS.md`. An R5 record-repair tranche, authorized separately (D-73),
  restates the readiness sentence in each `_CONTEXT.md` to the state the owner
  approves through the lifecycle workflow.
- **Risk if unrepaired:** 87 context surfaces keep feeding sealed briefs an
  outdated readiness label. The cause split would also send one situation to
  two R4 classes (record drift and ruling redirect).
- **Representative keys:** DEL-09-04:CONTEXT#architecture-basis-injection.s01
  (RECORD_DRIFT; Notes: all eight PKG-00 deliverables IN_PROGRESS);
  DEL-09-01:CONTEXT#architecture-basis-injection.s01 (SCOPE_REDIRECTED_BY_RULING,
  citing the CS-05 gate); DEL-16-01:CONTEXT#architecture-basis-injection.s02.
- **Visible exceptions:** 81 OWNER_CONFIRMED, 2 CONTESTED (DEL-09-05 CLM-003,
  DEL-10-05 CLM-002.r05), 3 OBSERVED (DEL-10-04 CLM-003.r08, DEL-10-01 CLM-004,
  DEL-10-03 CLM-004) and 1 WEAK (DEL-06-02 CLM-024.r06).

## T8-K2 — DEC-009: Python engines where DEC-009 adopts a Rust core

**What is split.** For the same fact (a domain engine or contract builder under
`core/` is Python, with no Rust counterpart), the corpus has three treatments:
- PKG-13: `IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · PROJECT_BASELINE · BASELINE · OWNER`.
- DEL-14-04 (and DEL-17-07/08/09 as sealed): `AUTHORITY_UNCLEAR · RECORD;BASELINE · OWNER`.
- DEL-14-03: `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · AuthorityNeeded NO`.

DEL-15-02 `.s03` is `UNKNOWN · AUTHORITY_UNCLEAR`. DEL-17-03 to 17-06 record no
DEC-009 finding at all, although they ship Python `core/handoff` code
(PKG-17_VERIFICATION.md W-1b).

**Evidence.**
- DEC-009 (`SOFTWARE_DECOMP.md:600`): "Adopt Rust core/application services …";
  status "Accepted; exact dependency versions and component/state libraries
  remain TBD". The implementation language of the core is not held open.
- The Python engines include `core/constraints/validation/engine.py`
  (DEL-13-03), `core/model_transform/physical_to_analytical/contract.py`
  (DEL-13-04), `core/comparison/model_state/engine.py` (DEL-14-03),
  `core/comparison/analysis_run/engine.py` (DEL-14-04) and
  `core/handoff/target_mapping/contract.py` (DEL-15-02).
- No located ruling permits Python core slices. The Python mentions in the
  decomposition are dev tooling only (DEC-060 at `SOFTWARE_DECOMP.md:651`,
  `coverage.py` "dev-tooling only").
- The PKG-13 verifier (§5 C1) and the PKG-14 verifier (§4.2) both read
  `AUTHORITY_UNCLEAR` as doubtful, because DEC-009 is not silent (C6(d), C6(f)).

**Proposed reading (cause and fields).** The reading is
`IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · PROJECT_BASELINE · BASELINE ·
AuthorityNeeded OWNER`. Implementation that departs from a ruled claim is
`IMPLEMENTED_DIFFERENTLY` (C6(d)), and "code appears to contradict a governing
claim" is `POSSIBLE_DEFECT` (C7). DEL-14-03's `PARTIAL_SLICE / NO` understates
the owner question. Its F7 fact (no product caller) is also true and goes to
T12. DEL-15-02 `.s03` is the least clear case. Whether a contract or reference
layer counts as "core/application services" is exactly the question the owner
decides, so its `UNKNOWN · AUTHORITY_UNCLEAR` may stand until then.

**OWNER DECISION.** Do Python domain engines and contract builders under
`core/` satisfy the DEC-009 baseline? The options, as they stand in the
evidence:
1. **Port.** Move the engines to the Rust core per DEC-009. This is what the
   rows' RemainingWork asks for (DEL-13-03, DEL-13-04). The rows then stay
   `POSSIBLE_DEFECT` until ported, and each port is a CODE_FIX_CANDIDATE brief.
2. **Permit.** Rule that Python core slices are permitted, through a decision
   record that amends DEC-009 on the scope-change or decision path. The rows
   then become stale records (`SCOPE_REDIRECTED_BY_RULING`) repaired at R5.
3. **Split.** Permit Python as a contract, reference or test layer (DEL-15-02's
   framing), but require product runtime services to be Rust. Each engine is
   then classified by whether it has a product caller (see T12).

- **Owning authority:** OWNER.
- **Route:** OWNER_DECISION. DEL-17-01 and 17-02 are NO_ACTION: they are
  documentation-only and contract-only, so the Rust clause does not diverge.
- **On-ruling mechanism:** a decision record amending or confirming DEC-009
  through the governing decision path. After that, either CODE_FIX_CANDIDATE
  briefs (option 1) or an R5 record-repair tranche (option 2), with the scope
  split per engine for option 3.
- **Risk if unrepaired:** a ruled baseline is contradicted in at least 15
  deliverables with no ruling. In PKG-17 the finding is visible only in
  resolution OtherCorrections (see R3 observations), so it can drop out of R4.
- **Representative keys:** DEL-13-03:SOW#CLM-004.r05 (POSSIBLE_DEFECT; Notes
  cite DEC-009 and A2); DEL-14-04:CONTEXT#architecture-basis-injection.s02
  (AUTHORITY_UNCLEAR); DEL-14-03:CONTEXT#architecture-basis-injection.s03
  (PARTIAL_SLICE, NO).
- **Visible exceptions:** 10 CONTESTED (DEL-13-03 ×2, DEL-13-04 ×2, DEL-14-01
  CLM-005, DEL-14-03 ×2, DEL-14-04 ×2, DEL-15-02 `.s03`), 7 WEAK (DEL-17-03 to
  17-09 ABI rows) and 2 RESOLVED_PAIR (DEL-17-01, DEL-17-02).

## T8-K3 — Unit vocabulary (tier, disposition and AuthorityNeeded)

**What is split.** The PKG-13 rerun verifier's §6 and §8 item 2 describe one
situation. It carries `INVARIANT` or `PROJECT_BASELINE`, `PARTIALLY_IMPLEMENTED`
or `IMPLEMENTED_DIFFERENTLY`, and `REVIEW` or `OWNER` across DEL-13-01,
DEL-13-02, DEL-03-07 and DEL-15-03. The verifier noted that C3 does not say
whether unit safety is among INVARIANT's covered subjects.

**Evidence.**
- OPS-K-UNIT-1 (`docs/CONTRACT.md:33`): "All calculations, formulas, imported
  values, and exports must be unit-aware and dimensionally checked". It is
  enforced by schema validation and CI (`:57`, `:58`). The C3 `INVARIANT`
  definition begins "restates a contract, specification or boundary
  invariant". The covered-subject list then maps layers. It does not narrow
  that first clause.
- There are **two distinct defects** in DEL-13-01/02:
  1. **Bare values.** A parameter declared `value_kind` quantity validates with
     a bare string and no unit (DEL-13-02 CLM-009 Notes; DEL-13-01
     REQ-13-01-006). DEL-03-07 is the import-gate analogue: bare JSON numbers
     are never inspected (CLM-003.r06).
  2. **Enum catch-up.** `Quantity.dimension` omits `force_per_length`. The
     accepted vocabulary has it (`schemas/units.schema.yaml:259`,
     `docs/SPEC.md:174`). The test compares the schema against its own stale
     copy.
- DEL-15-03 is **a different defect**: PR #307 routes the exporter output
  through redaction, so unit-manifest values become redacted (DEL-15-03
  CLM-005 Notes). It is a W3 owner item. The PKG-15 verifier's adopted FIELD
  corrections raise four of its rows to `INVARIANT` for the unit element.
  Those corrections sit in OtherCorrections and are not applied.

**Proposed reading.** The tier follows the gap (F8):
- **Unit-safety gaps are `INVARIANT`.** This covers bare-value acceptance and
  export unit loss. The gap touches OPS-K-UNIT-1's subject.
- **The enum omission is `PROJECT_BASELINE`.** It is a catch-up against the
  accepted PKG-02 vocabulary and makes nothing unit-unsafe, which matches the
  FIRM values on DEL-13-01 CLM-005.r05.

The disposition follows each row's subject (C6(b)), so no uniform value is
needed. Enforcement claims that are partly met are `PARTIALLY_IMPLEMENTED`, and
false declared alignment is `IMPLEMENTED_DIFFERENTLY`. `AuthorityNeeded` is
`REVIEW` for DEL-13-01/02 and DEL-03-07. The fix brings code into line with an
existing invariant and vocabulary, so no product choice is open. The human
disposition of `PKG13-DEL-13-02-PKG02-001` stays pending, and DEL-03-07 needs
engineering input on which fields are legitimately dimensionless. DEL-03-07's
effective `OWNER` and DEL-13-01 REQ-13-01-006's `PROJECT_BASELINE` get an
R3_OBSERVATION. **DEL-15-03 comes out of the vocabulary cluster.** Its question
is whether downstream_tool redaction of invented public content is intended.
That is an owner decision already on the W3 owner list, and its tier reads
`INVARIANT` by the same rule. DEL-15-02 CLM-005.r01 (Units) is adjacent. Its
`INVARIANT` tier agrees with this reading, which is derived from
OPS-K-UNIT-1 and not from the rerun launch hint (W3 departure 5).

- **Owning authority:** REVIEW (DEL-13-01/02, DEL-03-07) and ENGINEERING for
  the dimensionless-field list. OWNER only for the DEL-15-03 redaction item.
- **Route:** CODE_FIX_CANDIDATE: bind `Parameter.value` to `Quantity` when
  `value_kind` is quantity, add `force_per_length`, derive the test set from
  `units.schema.yaml`, and reject or schema-check bare numerics at the import
  gate. R5_RECORD_REPAIR applies to DEL-13-02 R01, and OWNER_DECISION to
  DEL-15-03.
- **On-ruling mechanism:** a candidate code-and-test brief under
  `R3/CODE_FIX_BRIEF_CANDIDATES/`, executed only by an authorized loop, then
  the pending human disposition. DEL-15-03 goes in the R4 packet with the
  PR #307 item.
- **Risk if unrepaired:** constraint parameters and imported records can carry
  unitless magnitudes past schema checks. A valid `force_per_length` quantity is
  rejected. Exported packages lose unit metadata.
- **Representative keys:** DEL-13-02:SOW#CLM-009 (both defects);
  DEL-13-01:SOW#CLM-005.r05 (enum only, FIRM); DEL-03-07:SOW#CLM-003.r06
  (import gate).
- **Visible exceptions:** 8 CONTESTED (the DEL-13-02 FG-01 rows), 1 FIRM
  (DEL-13-01 CLM-005.r05), 1 OBSERVED (DEL-03-07 CLM-021.s02, whose cause is
  POSSIBLE_DEFECT against PARTIAL_SLICE; this reading takes POSSIBLE_DEFECT)
  and 3 FIELD (DEL-15-03 REQ-002, AC-001 and CLM-019, all unapplied tier
  corrections).

## T8-K4 — F1 on CONTEXT-typed purpose rows

**What is split.** DEL-14-02 and DEL-14-01 `SOW#CLM-024` are `CONTEXT ·
ALIGNED` with a `GAP_WORDING_CHECKED` clause, although their Notes point to an
unmet element carried on another row (PKG-14_VERIFICATION.md §4.1). The
verifier asked for one corpus-wide answer.

**Evidence.**
- F1 (`CONVENTIONS.md` Part F) applies to "a row whose own evidence,
  RemainingWork or Notes record an unmet element of its claim", "even when the
  same gap is also recorded on another row". It has no claim-type exception.
- C2 lists `CONTEXT` as a claim type distinct from `NON_NORMATIVE`. Only
  `NON_NORMATIVE` pre-typed units are exempt (C1).
- CS-07 (`CANONICAL_SITUATIONS.md:36`) says Context Envelope blocks "are judged
  normally" because they "sometimes state normative constraints".
- Corpus practice already applies F1 to CONTEXT rows. There are 8 effective
  `PARTIALLY_IMPLEMENTED` CONTEXT rows, including DEL-13-02:SOW#CLM-024 (the
  same purpose slot, FG-01) and DEL-12-04:SOW#CLM-027.
- F4's `GAP_WORDING_CHECKED` must say why the wording is not an unmet element
  "of this claim". The DEL-14-02 clause instead says the gap is carried on
  CLM-011.r03, which is the case F1 excludes.

**Proposed reading.** F1 reaches CONTEXT-typed rows. Both rows read
`PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE · NONE · BASELINE`,
joining FG-DEL-14-02-02 (empty `rule_pack_refs`/`library_refs`,
`core/analysis_runs/records.py` L132-133) and FG-DEL-14-01-02 (no product save
path). DEL-05-02:SOW#CLM-021 (a CONTEXT guidance row, CONTESTED in PKG-05 W-3)
falls under the same rule. Whether its deferred rule-pack supply gap is part of
its own subject is a row-level judgment the verifier placed at the edge, so it
goes to REVIEW.

- **Owning authority:** NONE (convention reading, shown to the owner for
  confirmation).
- **Route:** CODE_FIX_CANDIDATE via each FindingGroup; REVIEW for DEL-05-02
  CLM-021.
- **On-ruling mechanism:** Agent 0 records the F1 reading in `R3_SYNTHESIS.md`.
  The rows join their FindingGroups' R4/R5 routing. No ledger is patched (F6).
- **Risk if unrepaired:** purpose rows claim coverage the product lacks. The
  PKG-14 sampled rate understates error in a 100%-sampled class, and the
  pattern matches PKG-12's outlier errors.
- **Representative keys:** DEL-14-02:SOW#CLM-024; DEL-14-01:SOW#CLM-024;
  DEL-05-02:SOW#CLM-021.
- **Visible exceptions:** all 3 rows are CONTESTED.

## T8-K5 — Export plan (the deleted `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`)

**What is split.** Rows citing PLAN-EXPORT-INTEROP are treated two ways in
PKG-17 (PKG-17_VERIFICATION.md W-2):
- G1/G2 apply CP-08: `RECORD_DRIFT · LOCAL_DESIGN · RECORD · OWNER` (DEL-17-01
  ×8, DEL-17-02 ×3, DEL-17-05 ×1, DEL-17-06 ×3).
- G3 uses `STALE_SETUP_SPECIFICATION · BASIS_POINTER_STALE · NO` (DEL-17-07,
  17-08 and 17-09).

**Evidence.**
- The file is absent from the freeze. No match for
  `EXPORT_FORMAT_INTEROPERABILITY_PLAN*` anywhere in the tree; `plans/` holds
  no export plan. The ledgers record its deletion by 349a2ab33 (2026-06-03,
  "Retire DEV-001 and archive legacy coordination files").
- It is a governing source, not an evidence file. SCA-004's brief lists it as
  source basis (`execution/_ScopeChange/SCA-004_2026-05-18_0000/Brief.md:12`),
  and SCA-004 created PKG-17 (`SOFTWARE_DECOMP.md:33`).
- **Code still requires it.** `core/handoff/pcf_export/package.py:29`, `:422`
  and `:533`, and `apps/desktop/src/features/pcf-export/PcfExportPanel.tsx:236`,
  carry `PLAN-EXPORT-INTEROP` as a required source-basis reference. Every PCF
  export package names a document that no longer exists.
- CP-02 covers pointers that moved "without a behaviour change". A deleted
  source whose repair is a choice is not such a pointer. CP-08's pattern
  (`RECORD_DRIFT · LOCAL_DESIGN · RECORD · OWNER`) fits the substance, although
  it is written for evidence files (as the verifier notes).

**Proposed reading.** Use CP-08's cause, tier, layer and `OWNER` for the
plan element. The disposition class follows F3 origin, so G3's
`STALE_SETUP_SPECIFICATION` stays for text present at 7bee9ae41. The five G3
rows whose cause is `BASIS_POINTER_STALE` and AuthorityNeeded `NO` get an
R3_OBSERVATION. Four rows whose own cause is another one
(`REPRESENTATION_MIGRATED` on DEL-17-07 CLM-004 and CLM-027 and DEL-17-09
CLM-007; `DOC_BEHIND_CODE` on DEL-17-08 X-002) keep that cause under the
one-cause rule. They still join the decision population.

**OWNER DECISION.** What becomes of PLAN-EXPORT-INTEROP? The options, as in
the rows' RemainingWork (for example DEL-17-01 CLM-004.r01):
1. **Restore** the plan to the tree, from history, as a governed source.
2. **Re-point** the source ID and its citations to a successor document, such
   as the DEL-17-01 registers and SCA-004 itself.
3. **Retire** the source ID. Remove it from the SOWs, `Source_Basis_Register.md`
   and the code's required source-basis refs. The deleting commit's title
   ("Retire … archive legacy coordination files") may indicate that retirement
   was intended, but a commit message is not a ruling (A2 applies to merged
   PRs).

- **Owning authority:** OWNER.
- **Route:** OWNER_DECISION, then R5_RECORD_REPAIR across the 24 rows and a
  CODE_FIX_CANDIDATE for the source-basis refs in `pcf_export`.
- **On-ruling mechanism:** an owner ruling in the R4 packet. R5 record repair of
  the SOW citations and registers. A code brief to change the required
  reference if option 2 or 3 is chosen.
- **Risk if unrepaired:** PKG-17 SOWs and product export packages keep citing a
  basis nobody can read. The CF-001 conflict (DEL-17-07) rests on a section of
  the deleted plan and stays open.
- **Representative keys:** DEL-17-01:SOW#CLM-004.r01 (CP-08, OWNER);
  DEL-17-07:SOW#CLM-008 (G3, BASIS_POINTER_STALE, NO);
  DEL-17-07:SOW#CLM-046/DEL-17-07-CF-001.
- **Visible exceptions:** 2 CONTESTED (DEL-17-07 CLM-008 and CF-001). The
  other 22 rows carry no resolution.

## T8-K6 — Tier of REQs that trace to an in-scope item

**What is split.** For CP-11 feature gaps that no ruling defers, DEL-07-05
(×2) and DEL-17-02/07/09 (×4) read `LOCAL_DESIGN`, while DEL-14-02 CLM-011.r09
reads `PROJECT_BASELINE`. All seven are RESOLVED_PAIR. The cross-wave verifier
left the tier point open (`CORPUS_CROSS_WAVE_VERIFICATION.md:87-93`, `:311-313`).

**Evidence.**
- C3: `PROJECT_BASELINE` is for a claim that "restates a ruled decision,
  accepted scope or baseline, or its substance conflicts with one", and
  `LOCAL_DESIGN` is for "a deliverable-local design choice".
- F8: the tier follows the remaining gap.
- Nearly every deliverable REQ traces to some IN scope item. Tracing alone
  would therefore empty `LOCAL_DESIGN` for REQs.
- The scope items at the freeze:
  - SOW-072 (`SOFTWARE_DECOMP.md:180`) names "rule-pack references, library
    references".
  - SOW-023 (`:131`) names "ratios" in GUI results review.
  - SOW-074 (`:182`) names handoff contents and "unsupported/approximated/delegated
    behavior reporting". It says nothing about timestamps or solver-ready
    blocking.
  - SOW-075 (`:183`) names "optional user-owned external harness metadata".

**Proposed reading (scope-item test).** A REQ reads `PROJECT_BASELINE` when its
unmet element is itself named in the accepted scope item. It reads
`LOCAL_DESIGN` when the unmet element is a deliverable-local elaboration of how
that item is met.
- **DEL-14-02 r09:** PROJECT_BASELINE, as effective.
- **DEL-07-05 CLM-004.r07 and REQ-07-05-005:** PROJECT_BASELINE. The effective
  `LOCAL_DESIGN` gets an R3_OBSERVATION. `AuthorityNeeded OWNER` stands: either
  build the supported ratio path, or rule that ratios stay unavailable. PR #787
  is a design change, not a ruling.
- **DEL-17-02 REQ-014 and REQ-053, DEL-17-09 REQ-010:** LOCAL_DESIGN, as
  effective.
- **DEL-17-07 REQ-034:** LOCAL_DESIGN. This one is borderline, because SOW-074
  names approximated/delegated reporting, but the unmet element is the
  component-to-PCF mapping inside the conservative-PCF scope.

- **Owning authority:** NONE for the convention (shown to the owner for
  confirmation). OWNER for the existing DEL-07-05 choice.
- **Route:** OWNER_DECISION (DEL-07-05 ×2), CODE_FIX_CANDIDATE (DEL-14-02 r09,
  which is also on the W3 owner list as "analysis-run records always write empty
  rule-pack and library references"), and NO_ACTION for the tier of the four
  PKG-17 rows. Their implementation gaps stay with T6.
- **On-ruling mechanism:** Agent 0 records the test in `R3_SYNTHESIS.md`. The
  DEL-07-05 pair goes to the R4 packet at PROJECT_BASELINE.
- **Risk if unrepaired:** a missing in-scope capability (ratio display) routes
  as a local design gap and falls below the owner's baseline view.
- **Representative keys:** DEL-14-02:SOW#CLM-011.r09; DEL-07-05:SOW#CLM-004.r07;
  DEL-17-02:SOW#CLM-019/DEL-17-02-REQ-014.
- **Visible exceptions:** all 7 rows are RESOLVED_PAIR, and no row value is
  contested beyond the tier point.

## T8-K7 — Acceptance-workflow cause

**What is split.** The absent human-acceptance workflow is
`DEFERRED_BY_RULING` in DEL-15-04 (FG-DEL-15-04-02) and `PARTIAL_SLICE` in
DEL-05-04 (FG-DEL-05-04-01, 9 rows). The cross-wave verifier found both
defensible (`CORPUS_CROSS_WAVE_VERIFICATION.md:257-263`).

**Evidence.**
- PRD §21.3 (`docs/PRD.md:1340-1344`): an acceptance record exists only "if
  stored in future releases", and "MVP shall not require a formal acceptance
  workflow".
- DEC-081 (`SOFTWARE_DECOMP.md:672`) reserves `ENGINEER_ACCEPTED` with "no
  acceptance workflow created, §21.3 MVP posture unchanged".
- OI-007 (`:575`) keeps the approval workflow TBD.
- DEL-05-04's landed slice is schema-level: `bound_hashes` and
  `invalidates_on_hash_change`. Its unmet element is enforcement and a negative
  test for records that nothing can create
  (`core/project_persistence/service.py:127` seeds `human_acceptance_refs: []`).

**Proposed reading.** `DEFERRED_BY_RULING` for both deliverables. The remaining
gap exists only because the MVP carries no acceptance workflow, and C7 lets the
gap's cause win. DEL-05-04's schema slice is reflected in its disposition,
`PARTIALLY_IMPLEMENTED`, which stays. `INVARIANT · CLAIMS` stand. The 9
DEL-05-04 rows get an R3_OBSERVATION.

DEL-15-04:SOW#…/AC-001 (CONTESTED, ALIGNED) names "external human-owned
hash-bound acceptance only" (`DEL-15-04/ScopeOfWork.md:183`). The hash-bound
element holds only because no acceptance can be represented (CP-11), so it reads
`PARTIALLY_IMPLEMENTED · DEFERRED_BY_RULING · INVARIANT · CLAIMS` in
FG-DEL-15-04-02. That is the verifier's second reading and the row's own R7
treatment.

- **Confidence:** MEDIUM. DEC-081's clause states what the decision does not
  create rather than using the word "defer". The PRD's MVP sentence carries the
  deferral.
- **Owning authority:** NONE (convention reading). The owner may confirm it.
- **Route:** NO_ACTION. The rows are correctly non-aligned, and the work is
  deferred until an acceptance workflow is authorized.
- **On-ruling mechanism:** none now. When the owner authorizes an acceptance
  workflow, the FG rows become implementation work through the scope-change or
  lifecycle path.
- **Risk if unrepaired:** low. The OPS-K-AUTH-2 enforcement gap stays
  documented. If the cause split persists, one deferral shows as two R4
  classes.
- **Representative keys:** DEL-05-04:SOW#CLM-011/REQ-05-04-008;
  DEL-15-04:SOW#CLM-010.r07; DEL-15-04:SOW#completion-and-reliance-basis-epistemology/AC-001.
- **Visible exceptions:** 4 RESOLVED_PAIR (DEL-05-04 CLM-008 and CLM-023;
  DEL-15-04 CLM-010.r07 and CLM-012.r03) and 1 CONTESTED (DEL-15-04 AC-001).

---

## Coverage

- `T8_ROWS.csv` has 176 body rows (plus `#END`) covering 90 deliverables and 17
  packages. Each ClaimKey appears exactly once: a script asserted the keys are
  unique and that every one exists in `R3/CORPUS_CLAIMS.csv`.
  - 172 rows are `Divergent = YES`. The 4 ALIGNED rows are the three F1-on-CONTEXT
    rows and DEL-15-04 AC-001.
  - Per cluster: SR-1 87, EXPORT_PLAN 24, UNIT_VOCABULARY 23, DEC-009 20,
    ACCEPTANCE_WORKFLOW 12, TIER_IN_SCOPE_REQ 7, F1_ON_CONTEXT 3.
  - Per route: R5_RECORD_REPAIR 88, OWNER_DECISION 52, NO_ACTION 18,
    CODE_FIX_CANDIDATE 17, REVIEW 1.
- **Population checks.**
  - SR-1 includes all 81 OWNER_CONFIRMED rows in the W1 (9), W2 (23) and W3 (49)
    adopted resolutions, plus 6 adjacent rows found by searching the
    resolutions for `SEMANTIC_READY`.
  - DEC-009 includes every CONTESTED or WEAK resolution naming the cluster,
    plus the FindingGroup member DEL-14-03 CLM-004.
  - Unit vocabulary includes the FindingGroups named in PKG-13_RERUN1 §6.
    DEL-15-03 is limited to the 8 FG-02 rows whose own subject names units.
  - The export plan includes every non-aligned PKG-17 ledger row whose Notes or
    sources cite the plan. DEL-17-01 REQ-001 is excluded (its gap is DAG-005).
  - Tier and acceptance use the rows the cross-wave report names, plus their
    FindingGroups.
- **Inputs read.**
  - The adopted `RESOLUTIONS.csv` files for W1, W2, W3 and CROSS_WAVE. Every
    `RESOLUTIONS_DRAFT*.csv` and the merged draft were ignored.
  - The W2 and W3 assessments.
  - The PKG-08, 09, 12, 13 (both), 14, 15 (rerun) and 17 verification reports,
    and the cross-wave report.
  - `CONVENTIONS.md`, `CANONICAL_SITUATIONS.md` and OWNER_DIRECTIONS
    Direction 8.
  - The sealed forward ledgers, and the freeze.
- No other task's files were read.

## R3 observations

1. **Effective values differing from the proposed readings (no correction).**
   - SR-1 cause on 39 rows (plus DEL-10-05 CLM-002.r05).
   - DEC-009 cause and layers on DEL-14-04 ×2, and PARTIAL_SLICE/NO on DEL-14-03
     ×3.
   - The unit tier on DEL-13-01 REQ-13-01-006, and AuthorityNeeded OWNER on
     DEL-03-07 ×3.
   - ALIGNED on DEL-14-01/02 CLM-024 and DEL-15-04 AC-001.
   - The G3 export-plan cause and AuthorityNeeded on 5 rows.
   - LOCAL_DESIGN on DEL-07-05 ×2.
   - PARTIAL_SLICE on DEL-05-04 ×9.
2. **Findings carried only in OtherCorrections.**
   - The PKG-17 DEC-009 element for DEL-17-03 to 17-09 was never minted as a
     `.sNN`. For DEL-17-07/08/09, the WEAK resolution moved the sealed
     IMPLEMENTED_DIFFERENTLY finding off the effective row. R4 must pick it up
     from OtherCorrections, or it is lost.
   - The same applies to the unapplied DEL-15-03 FIELD tier corrections
     (4 rows to INVARIANT).
3. **Code references a deleted governing document.** DEL-17-07:SOW#CLM-014/
   DEL-17-07-REQ-015 is ALIGNED on `REQUIRED_SOURCE_BASIS_REFS`, but one of the
   refs it requires is PLAN-EXPORT-INTEROP (`core/handoff/pcf_export/package.py:29`),
   whose target is absent. That row is outside this cluster's divergent set and
   is reported here only.
4. **Python mentions outside the named DEC-009 packages.** DEL-16-02
   REQ-16-02-002 (a Python validation-preview engine) and DEL-07-08 REQ-002 (a
   Python contract fixture copied into the desktop) name Python artifacts, but
   no verifier put them in the DEC-009 cluster, and they are not included here.
   Agent 0 may want to check PKG-16's Python engines against the DEC-009
   decision once it is made.
5. **Process disclosure.** While locating evidence, I ran read-only `git grep`
   and `git log` inside the freeze checkout. Rule 9 forbids git use. Nothing was
   written. Every fact cited above was re-derived with plain `grep`/`find`, or
   taken from the ledgers (the deletion commit ID and its title come from the
   ledgers and PKG-17 W-2).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
