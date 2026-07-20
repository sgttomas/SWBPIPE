# T7 D-54 Reasoned-Selection Rationale — DEL-09-04 Reproduction-Manual Stale-Text Refresh

**Run:** HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W4 / T7
**Author:** WORKING_ITEMS (Agent 1, PKG-09 package manager)
**Brief:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T7_DEL-09-04_VALMANUAL_STALE_TEXT.md` (`CB-2026-07-20-T7-DEL-09-04-VALMANUAL-STALE-001`)
**Date:** 2026-07-20

## 1. Sources Opened (live tree at post-T6 head `db9197a5d`)

- Routing source: `instances/W1/RETURN.md` §5 item 2; chain evidence
  `instances/W1/T2/EXECUTE_RETURN.md` (§3.8 pinned-case delta, before/after
  stdout SHA-256, "Recorded Follow-On") and `instances/W1/T2/VERIFY_IMPL.md`
  (defect D1, fallback fixture).
- The page `docs/validation_manual/headless_runner_reproduction.md` read in
  full at head (case-1 row; existing case-3 dated note; Part 2 pinned
  figures; Rerun Consequence; Remaining E2 Work; Review Checks).
- Frozen input `validation/witness/inputs/tp_runner_015_final_cli_solve_input.json`
  (`supports[6]` = `support:CE-120`, family `constant_effort_support`,
  `restraints: []`, `hanger.constant_load` 375 N) — parsed directly.
- `core/product_physics/src/lib.rs` (`SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED`
  envelope-level warning emission) and
  `core/product_physics/src/validation.rs` (info-text clause).
- W2 chain (`instances/W2/RETURN.md`, W2-C7: del1005 byte-identity at the
  W2 head; wind schema change confined to surfaces absent from this page).
- The committed T6 bundle
  `validation/evidence/benchmarks/BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/`
  (head whole-suite mechanics: 24 cases, 11 matched + 13 blocked, exit 1)
  and the suite readiness assertion (`fixtures.len() == 24`).
- DEL-09-04 `_STATUS.md` (two owner-gated Remaining bullets, W3 rows 7–8),
  `MEMORY.md`, the R13 brief
  (`CANDIDATE_BRIEF_2026-07-19_DEL-09-04_VALMANUAL_REFRESH.md`, structural
  model and rerun-trigger source), and the R13 run record.
- W3 return (rows 7–8 owner-gating), W4 dispatch transcript, D-52/D-54
  packets, DEC-065/DEC-080/DEC-081 rows.

## 2. Limits Screen (D-54 §3.1, run first)

No fast-reject hit. Docs + bounded state only; no acceptance, promotion,
lifecycle, release, publication, spend, merge, or external act; no
protected data; both owner-gated Remaining bullets byte-preserved; every
outcome-determining premise is live-tree-grounded (frozen input bytes,
live emission source, committed T6 bundle, preserved W1/W2 chain
evidence) with the one head-behavior claim gated on a mandatory live
offline run before any page write. Ambiguity checked: the W4 dispatch
names exactly this follow-on and the NO-OP alternative is addressed with
evidence (§4 item 5).

## 3. Four-Lens Analysis

- **Ontology.** The object is documentation currency on an existing
  draft-evidence manual page — not evidence creation, not witness
  change, not acceptance. The dated-note device keeps three distinct
  things distinct: the frozen committed witness (truthful for pinned
  commits), the live head behavior (witnessed by a fresh run), and the
  page's expectation text (corrected in place with dated framing) —
  exactly the ontology the existing case-3 note established.
- **Epistemology.** Every asserted value has a named warrant: the
  warning code, subject support id, and empty-restraints cause from the
  frozen input bytes + live source + mandatory live run; the wave
  provenance from the merged commits and preserved W1 chain (with its
  recorded SHAs); the currency figures from the committed T6 bundle and
  the crate assertion. The tranche invents no value; a live-run
  discrepancy halts the write.
- **Praxeology.** The R13 precedent form gives operators one stable
  pattern (dated notes over pinned witnesses); the corrected cell
  prevents the next reproduction run from inheriting a stale
  expectation; fail-closed ordering (verify head behavior BEFORE
  editing) makes the page unable to outrun its evidence.
- **Axiology.** Truthfulness of committed history preserved (witnesses
  and bundles never edited); evidence over plausibility (mandatory live
  anchor); human responsibility conserved (owner gates named and
  untouched; the warning described as review evidence, not defect);
  claim posture within DEC-081.

## 4. Materially Important Alternatives (rejected, with reasons)

1. **Full-page restructure** (R13 did one): nothing structural went
   stale this time; two notes and one cell suffice; smaller diff, lower
   drift risk.
2. **Silent in-place cell edit without a dated note**: erases the
   historical relationship between the committed witness bytes and the
   page, breaking the R12→R13 precedent that pinned artifacts stay
   truthful under dated framing.
3. **Regenerate the solve witness to the post-T2 bytes**: forbidden —
   frozen surface, DEL-10-05 lane, and it would convert a docs tranche
   into a witness change.
4. **Fix the fallback fixture text and/or the `validation.rs` info-text
   here**: outside the docs lane (fixture/code surfaces); the fixture is
   a pinned earlier-generation output analogous to committed witnesses;
   the info-text is literally true. Recorded as dispositions instead.
5. **Truthful NO-OP**: refuted by the live tree — the case-1 "Expected
   evidence" cell asserts empty diagnostics while post-T2 sources emit
   the non-consumption warning into the result envelope and regenerated
   output no longer byte-matches the committed witness (W1 T2 recorded
   before/after SHAs; live emission source present at head). The Part 2
   figures alone might arguably stand as pinned history, but the case-1
   cell alone defeats NO-OP.
6. **Currency note without the T6-bundle anchor** (assert only "24
   fixtures"): weaker; the committed bundle supplies head whole-suite
   tallies as citable derivative evidence, so the note can be fully
   witnessed.

## 5. Selected Outcome

Two dated 2026-07-20 notes (case-1 historical note in the case-3
precedent form; Part 2 currency note anchored to the committed T6
bundle) plus the corrected case-1 expected-evidence cell; three recorded
no-edit dispositions; History/MEMORY/run-record-only state updates; both
Remaining bullets byte-preserved; mandatory live offline head
verification before any page write.

## 6. Attempted Failure Mode (adversarial self-check)

Attempted: classify the case-1 correction as weakening a frozen
expectation (an acceptance-criteria change). Fails: the page's
expectation text is documentation, not an acceptance criterion; the
committed witness stays byte-frozen and pinned-truthful; DEC-046 and all
threshold gates untouched. Attempted: classify the Part 2 currency note
as promoting the T6 bundle into release/acceptance evidence. Fails: the
note quotes the bundle's own derivative/non-authoritative label and
restates the TBD owner gates. Attempted: treat the live-run requirement
as a disguised reproduction run needing DEC-080 bundle machinery. Fails:
DEC-080 governs clean-checkout reproduction bundles; a spot verification
with ephemeral outputs creates no evidence artifact and the R13
precedent expressly permitted exactly this class of offline spot-run.

## 7. Classification

`CLASSIFY_ELIGIBLE`; `ACTIVATE_OWNER_STANDING_APPROVAL`;
`AgentJudgment: SELECT_AND_ADVANCE`; `OwnerCaseSelection: NONE`;
`EffectStatus: HELD` pending fresh-context independent refutation.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
