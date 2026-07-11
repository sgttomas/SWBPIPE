# R1+ Binding Convention Set (Owner-Adopted)

**Status:** ADOPTED for this run's R1+ phases. Owner ruling of record
(2026-07-11, in-session scale-out slate, Ryan Tufts), verbatim: **"Adopt +
scale out (Recommended)"** — adopting the R0b convention set plus the R0b
reviewer's 13 addenda as this run's binding encoding rules, and authorizing
R1 whole-corpus inventory plus the R2 package waves over all 101 deliverables
with the six calibration deliverables re-encoded as ordinary wave members.
The pinned plan revision (`551f84ef6be656f1603ce0acfa5e3935aa9683c7`) remains
the method authority; these rules bind ledger encodings for this run. The
R0/R0b ledgers predate this set and are calibration evidence only (RUN_BASIS
run-level note).

## Part A — Conventions 1–8

The eight conventions in `R0B_CALIBRATION/R0B_CONVENTIONS.md` apply verbatim,
with convention 1's `STALE_SETUP_SPECIFICATION` definition widened per
addendum 4 below.

## Part B — Addenda 1–13 (from `R0B_CALIBRATION/R0B_REVIEW.md` §6, adopted)

1. **Declared-state census:** exactly one `DECLARED_STATE` row per
   four-document kit surface (Specification, Datasheet, Guidance, Procedure),
   plus one each for `_STATUS.md` and `MEMORY.md` when they carry
   current-state declarations, plus one per deliverable-owned in-tree README;
   dated MEMORY log entries are historical records — drift inside them is a
   note on the MEMORY surface row, never a staleness disposition.
2. **Bootstrap item:** the seeded `(gated: D-41)` item is recorded verbatim
   only in the `_STATUS.md` surface row's `RecordedRemaining`, never as its
   own claim row, and is excluded from all residual, gate, and selectability
   analysis.
3. **Multi-residual surface rows:** `GateOrStageConstraint` lists one value
   per residual, semicolon-separated in `## Remaining` order;
   `SelectableUnderCurrentLoop=YES` iff any listed residual is ungated.
4. **Widened `STALE_SETUP_SPECIFICATION`:** "the surface's declaration no
   longer describes the frozen implemented slice" (covers setup-era
   future-tense prose and post-alignment drift); one controlled value.
5. **Declared-TBD branch:** a declared bounded TBD accepted by a named human
   ruling takes the substance disposition (`ACCEPTED_DIVERGENCE` if the
   ruling permits the deferral) with the failed homing recorded in
   `RemainingWork`; `UNKNOWN` is reserved for evidence-backed residual
   candidates whose status or home cannot be resolved; the candidate-home
   search is bounded to homes named in the deliverable's own records plus any
   recorded recommended-owner list.
6. **SourceReliability ladder:** `DECLARED_STATE` prose rows always
   `NOT_APPLICABLE`; project-original agent-generated technical evidence with
   agent audit but pending human disposition is `UNVERIFIED`; `REVIEWED`
   requires a named human ruling or recorded human disposition covering the
   cited record.
7. **ClaimClass completions:** validation-suite coverage/harness claims =
   `VALIDATION`; numeric/unit correctness = `MECHANICS`;
   result-envelope/report-content fidelity = `REPORTING`; GUI visibility of
   diagnostics = `GUI`.
8. **Unmapped-row hygiene:** `LifecycleState=UNMAPPED` (fixed token);
   `PKG-XX/UNMAPPED` when the surface is package-attributable, bare
   `UNMAPPED` only for cross-package surfaces; shared crates are ledgered at
   named-slice grain with mapping routed `AuthorityNeeded=OWNER`.
9. **Side-effect-free definition:** re-execution in the frozen tree requires
   build/bytecode artifacts redirected outside the tree (external
   `CARGO_TARGET_DIR`; `PYTHONDONTWRITEBYTECODE=1`) with a porcelain check
   after; writes into the frozen tree are forbidden even on git-ignored
   paths.
10. **Content-identical qualifier:** standardized string `content-identical
    at frozen SHA <sha> (diff empty over <paths>)` beside the recorded-pass
    marker for ancestor-commit evidence; such rows satisfy the
    overtaken-evidence bar, which extends to class-required *verification*
    evidence as well as validation.
11. **ACCEPTED_DIVERGENCE threshold:** requires a named
    ruling/decision/human-disposition record that *permits* the deferred
    state; recorded residuals, adoptions, or rehoming receipts alone yield
    `ALIGNED`.
12. **Aggregation grain and defaults:** package summaries histogram by
    `ClaimType` (acceptance rows never merged with requirement rows);
    `SelectableUnderCurrentLoop=NO` on rows with no recorded item; `ClaimID`
    fixed to `DEL-XX-XX-<TYPE>-NNN`; a run-level `NormativeSource` path alias
    may be declared once per ledger in the notes.
13. **Pending-disposition evidence:** validation evidence in
    `TECHNICALLY_ADDRESSED_PENDING_HUMAN` state caps row `Confidence` at
    MEDIUM and routes `AuthorityNeeded=OWNER`; the disposition must rest on
    independent grounds.

## Part C — Named repairs carried into R1/R2

- **R1:** correct the DEL-09-01 fixture/witness count to 21 in that
  deliverable's re-encoded wave ledger (the R0b ledger stays unmodified as
  calibration evidence; the FAIL is recorded in `R0B_REVIEW.md` §3).
- **R2:** the first SECURITY-class deliverable ledgered in the waves gets a
  reviewer spot-check of the convention-6 SECURITY encoding (unexercised in
  calibration).

## Part D — Wave plan (plan §8 R2, tractability partitions only)

W1 `PKG-00`–`PKG-03` (25) · W2 `PKG-04`–`PKG-05` (11) · W3 `PKG-06`–`PKG-08`
(19) · W4 `PKG-09`–`PKG-12` (20) · W5 `PKG-13`–`PKG-17` (26). The six
calibration deliverables are re-encoded as ordinary members of their waves.
Every ledger is deliverable-grained; wave-boundary reviewer checkpoints
sample each wave before its artifacts are committed.
