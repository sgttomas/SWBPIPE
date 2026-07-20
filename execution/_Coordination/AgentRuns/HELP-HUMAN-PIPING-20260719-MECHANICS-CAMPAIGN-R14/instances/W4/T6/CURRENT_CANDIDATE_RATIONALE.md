# T6 D-54 Reasoned-Selection Rationale — DEL-09-01 Benchmark-Evidence-System Bounded Construction

**Run:** HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W4 / T6
**Author:** WORKING_ITEMS (Agent 1, PKG-09 package manager)
**Brief:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T6_DEL-09-01_BENCH_EVIDENCE_SYSTEM.md` (`CB-2026-07-20-T6-DEL-09-01-BENCH-EVIDENCE-001`)
**Date:** 2026-07-20

## 1. Sources Opened (live tree at `e315fb840`, branch `claude/piping-r14-pkg09-evidence`)

- W3 assessment `instances/W3/RETURN.md` (row-3 classification: the only
  partially-agent-lawful PKG-09 row; lawful slice = evidence-system
  construction; row closure owner-gated).
- W4 dispatch (T6 scope text and hard exclusions, quoted in the brief).
- DEL-09-01 `_STATUS.md` (IN_PROGRESS; four Remaining rows), `MEMORY.md`
  (R5-era 21-family/33-test evidence point, line 748), `Dependencies.csv`.
- PDU-037 and PDU-013 rows in
  `execution/_Reconciliation/DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305/PROPOSED_DELIVERABLE_UPDATES.csv`.
- `validation/benchmarks/mechanics/src/lib.rs` (24-fixture
  `fixture_inventory()`, `assert_eq!(fixtures.len(), 24)`,
  `MechanicsBenchmark`/`BenchmarkProvenance` structs,
  `RedistributionStatus::PublicOriginal`, `tolerance_policy_is_unresolved`),
  `validation/benchmarks/mechanics/README.md`,
  `validation/hand_calcs/mechanics/` inventory.
- `core/runner/headless/src/benchmark_binding.rs` + bin (whole-suite
  default, per-case reporting, fail-closed blocking codes,
  `REGRESSION_EVIDENCE_CLAIM_POSTURE`), DEC-065 exit policy.
- del1005 committed witness family (benchmark input shape),
  `validation/evidence/` home layout (DEC-080 bundle convention),
  `tests/test_export_public_openpipestress.py` (evidence subtree excluded
  from public export).
- DAG-007 DEL-09-01 rows (3 ANCHOR, 4 SATISFIED, 7 TBD-with-refresh-note),
  `execution/_DAG/_LATEST.md`.
- DEC-054 register row + `plans/PLAN_2026-06-17_prd_completion.md` §3 D9
  exit-refresh row; D-52/DEC-085; D-54/DEC-087; DEC-081; DEC-046; DEC-026.
- `software-workflow.json`; R14 W1/W2 briefs and the R13 DEL-09-04 brief
  as structural models.

## 2. Limits Screen (D-54 §3.1, run first)

No fast-reject hit. The tranche: creates derivative evidence only;
promotes no tolerance/threshold/acceptance/release/label; performs no
verification→validation promotion; performs no lifecycle, stage,
issuance, publication, merge, spend, or external act; exposes no
protected data (suite content is project-original `PublicOriginal`;
the evidence subtree is excluded from public export anyway); touches no
owner-gated row (all four Remaining rows byte-preserved); requires no
unavailable evidence (every index field is extractable from committed
content or the captured head run). The seven TBD-maturity DAG rows were
examined: their recorded cause is refresh-scope ("target maturity not
verified from permitted read set"), the identical posture carried the
governed 2026-07-12 PDU-037 evidence refresh, and this tranche asserts
nothing about upstream maturity — it documents the committed suite as it
exists. Ambiguity was checked and not found: the W4 dispatch and W3
row-3 classification name exactly this slice as lawful.

## 3. Four-Lens Analysis

- **Ontology.** The proposed object is a derivative evidence bundle — a
  new instance of the existing evidence-package class
  (`validation/evidence/` homes; DEC-080 bundle convention), not new
  decomposition truth, not a lifecycle object, not normative content.
  Proposal (this brief), verification (brief verifier), execution
  (bundle assembly), and the withheld owner acts (row closure, threshold
  promotion, release integration) remain distinct classes.
- **Epistemology.** Every asserted index field has a committed warrant:
  provenance fields from the crate's `BenchmarkProvenance` records;
  family/inventory from `fixture_inventory()` and its enforced README
  mirrors; witness anchors existence-checked; per-case head status from
  the captured runner output; test counts from the executed suite run.
  Claims stay at regression/verification-evidence strength (DEC-081); no
  claim exceeds its warrant; blocked cases are recorded as emitted, not
  reinterpreted.
- **Praxeology.** The bundle gives future operators a bounded,
  rerunnable, fail-closed path: a deterministic bundle-local assembly
  script, hash manifest, frozen-surface guards, exact write fence, and
  rerun triggers producing a new bundle id rather than edits. It
  advances the one agent-lawful PKG-09 queue item without touching any
  gate the owner must later exercise.
- **Axiology.** Evidence over plausibility (the index extracts committed
  facts; the head capture records what IS); truthful attribution (owner
  standing approval vs agent selection recorded separately); human
  responsibility preserved (row closure, thresholds, release integration
  all named as withheld owner acts); IP/privacy posture conserved
  (PublicOriginal provenance recorded, export exclusion noted);
  reversibility (additive bundle, no product change).

## 4. Materially Important Alternatives (rejected, with reasons)

1. **Crate-level provenance exporter** (add a binary/test to the suite
   crate that dumps the inventory as JSON): strongest extraction
   fidelity, but it is a code change to the suite crate — excluded by
   the W4 dispatch (derivative/evidence-only; no solver/schema/runner or
   crate code changes) and unnecessary for the evidence purpose.
2. **README-only refresh** (update the mirrors and MEMORY): smallest
   diff, but it constructs neither the named provenance/redistribution
   index nor a labeled derivative artifact set, and the mirrors are
   already enforced by the readiness test; it would under-deliver the
   W3-named lawful slice.
3. **Cross-suite index (mechanics + stress + nonlinear)**: more complete
   §16.2 coverage, but the stress/nonlinear suites belong to
   DEL-09-02/DEL-09-03 whose Remaining rows are held/owner-gated per W3;
   writing their evidence under a DEL-09-01 tranche would blur
   deliverable ownership and widen containment for no queue-mandated
   gain. Mechanics-only keeps the fence exact.
4. **`docs/` home for the index**: readable, but the docs lane is the
   validation-manual surface (DEL-09-04), the artifact is evidence (not
   manual prose), and `validation/evidence/` is the accepted home class
   with an existing immutable-bundle convention and public-export
   exclusion.
5. **`tools/` assembly script** (shared tool): reusable, but it changes
   the tool surface, invites future maintenance obligations outside this
   deliverable, and determinism is fully served by a bundle-local
   script.
6. **Striking or annotating the §16.2 Remaining row**: refuted directly
   by the W4 dispatch ("The row is NOT struck at closeout") and W3 (row
   closure is judged at the owner's R5-exit gate).
7. **Deferral**: no positive support — DEC-054 named the residual
   ordinary work, PDU-037 is `AuthorityNeeded=NO` for evidence refresh,
   the campaign queue directs exactly this slice, and no risk record
   counsels waiting.

## 5. Selected Outcome

One immutable, hash-manifested derivative evidence bundle under
`validation/evidence/benchmarks/BENCHEVID_DEL0901_<UTC>Z_<sha12>/`
containing the whole-suite head capture, the extracted 24-fixture
family/provenance/redistribution index, the verification-refresh record,
and the deterministic bundle-local assembly script — with
History/MEMORY/run-record-only deliverable state updates, all four
Remaining rows byte-preserved, and every owner gate named and withheld.

## 6. Attempted Failure Mode (adversarial self-check)

Attempted: classify the head whole-suite capture as an unlawful "new
witness" or as reproduction-acceptance work. The attempt fails: the
capture is a recorded observation inside a derivative evidence bundle
(the same epistemic class as sweep and coverage artifacts), touches no
`validation/witness/**` path, asserts no acceptance, and DEC-080's
actor-neutral precedent covers agent-executed evidence runs. Attempted:
classify the expected nonzero whole-suite exit (blocked cases) as a
check failure that should block closeout. The attempt fails: the brief
records the DEC-065 semantics up front — blocked cases are fail-closed
regression evidence, recorded exactly as emitted; only capture/assembly
integrity failures block. Attempted: find a second defensible outcome so
strong it demands referral. Under D-54 §3.2 another defensible option is
not itself a referral condition; the §4 alternatives were each rejected
for recorded project-grounded reasons.

## 7. Classification

`CLASSIFY_ELIGIBLE`; `ACTIVATE_OWNER_STANDING_APPROVAL`;
`AgentJudgment: SELECT_AND_ADVANCE`; `OwnerCaseSelection: NONE`;
`EffectStatus: HELD` pending fresh-context independent refutation.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
