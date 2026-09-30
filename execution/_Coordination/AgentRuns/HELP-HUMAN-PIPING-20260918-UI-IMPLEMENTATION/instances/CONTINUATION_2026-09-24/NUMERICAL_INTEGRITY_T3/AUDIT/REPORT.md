# Independent T3 audit, 2026-09-30

## Result and basis

**One new SHOULD-FIX finding and three NOTEs.** No new false solver publication
has been demonstrated. The principal finding is a counterexample to the
published-scale closeness premise used in the A1 honesty argument. Its effect
on a realized solver case remains open. This report does not accept an amended
design, set W1 limits, or authorize F2a reliance.

The audit basis is main `74b3c7313491f27f71c4361d5e1657ee4a39e2f1` (PR #1063),
confirmed against GitHub by `git ls-remote` during discovery. The branch is
`codex/piping-t3-audit-20260930`. T3 means this report's parent directory; FK,
H and VR mean `projects/chirality-piping/core/solver/frame_kernel`,
`projects/chirality-piping/core/solver/performance_harness` and
`projects/chirality-piping/validation/benchmarks/numerical_robustness`.
All source line numbers below are at that basis.

The owner directly commissioned an independent audit and authorized broader
discovery, then continuation. Execution used Codex's native host tools, with
no delegated agents. Root AGENTS, TASK, Piping AGENTS and the chirality-change
skill were read; their origins and SHA256 values are in
`_run_records/basis.json`. TASK's bounded-executor guidance was used; this
owner-directed audit has its own expressly authorized branch and records.
No other role body or reusable workflow was activated.

Scope expanded beyond the handoff because prior publication and evidence
failures occurred at interfaces between individually checked stages. The added
checks cover publication-scale coupling, stop/cache evidence, feature and
product reach, and whether the preserved records suffice for independent replay.
This is a bounded audit of T3, not certification of the entire repository.

## Findings

| ID | Severity | Location | Evidence and consequence | Suggested remedy |
|---|---|---|---|---|
| AUD-T3-01 | SHOULD-FIX | `T3/DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md:342`; `T3/REVIEW/K4_REVIEW.md:456`; `FK/src/structural/retained/adaptive.rs:296`, `:380`, `:1904`, `:2598` | The proof transfers a bound using the verification scale to one using a scale reconstructed from rounded binary64 rows. Subnormal row rounding can be amplified by multiplication by L or division by L. The asserted relative-plus-half-subnormal closeness does not hold generally. An independent exact-rational counterexample gives a published scale 20% below the verification scale; an abstract stop-rule example then misses its published interval by 12.5%. **This is a proof counterexample, not a realized source-model or Rust-solver counterexample.** A second boundary makes the published coupled scale zero although the verification scale is positive. | Independently re-derive the complete coupling error, including raw subnormal operands, L and 1/L amplification, threshold crossings and zero published scales. Either prove reachability restrictions sufficient to exclude the cases, or amend the bound/encoding/guard through ROOT and D2. Add a realized-source probe and independent exact oracle before closing the finding. See `A1_A2_REVIEW.md`. |
| AUD-T3-02 | NOTE | `T3/HANDOFF_2026-09-30_AUDIT_PAUSE.md:156` | The "one exception" instruction is incomplete. Besides REVIEW, the manifests in `DESIGN_NUMERICS/_run_records`, `DESIGN_STANDING/_run_records`, `REFERENCES/_run_records` and `REFERENCES_ELOAD/_run_records` use their parent directories. Running the stated convention produces missing-file errors; running from the actual bases verifies all 90 manifests and 9,088 entries. There is no observed hash mismatch. | Add an append-only replay erratum listing all five bases, or provide a small explicit manifest-to-base map. Do not rewrite the hash-bound manifests merely to normalize their paths. |
| AUD-T3-03 | NOTE | `T3/HANDOFF_2026-09-30_AUDIT_PAUSE.md:215`; `T3/IMPLEMENTATION/S11K/RETURN.md:186`, `:194`; `T3/IMPLEMENTATION/K2B/RETURN.md:1121`; `T3/IMPLEMENTATION/K6/RETURN.md:37`; `T3/IMPLEMENTATION/K6B/RETURN.md:14`, `:513` | A blanket "no TASK Git writes" conclusion would be false. Three implementer returns disclose intent-to-add/reset pairs, K6b discloses a fetch, and S11-K records a fast-forward expressly allowed by its fix brief. These are historical disclosures, not newly alleged hidden operations. An empty final index cannot prove that no transient index operation occurred. | Close the process item with an exception ledger separating prohibited index operations, the fetch, and the authorized fast-forward. Preserve the reported nil net effects as reported facts; do not infer complete command-history compliance from Git commit authorship. |
| AUD-T3-04 | NOTE | `T3/IMPLEMENTATION/M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt:10`; `T3/IMPLEMENTATION/KF3_MERGE/dec025/suites.log:1`; `T3/IMPLEMENTATION/KF2_MERGE/RECORD.md:78`; `FK/tests/retained_k4/gen_k4_vectors.py:119`; `.github/workflows/piping-desktop-e2e.yml:183` | Reproducibility has external dependencies. The six merge folders retain per-manifest count summaries and derived comparisons, but no `dec025/suites/*.log` files. KF2's large raw gate JSONL files are retained only in scratch with recorded hashes. K4's maintained generator imports pinned inputs from the dated execution tree, which numerical CI's sparse checkout omits. Thus hash verification and green CI do not mean a fresh checkout alone can independently regenerate every claimed check. | Locate and preserve the original raw logs/gate files before pruning, or explicitly inventory what is retained only by hash. Preserve their original hashes and add sanitized copies where proportionate. Consider stable maintained copies of generator inputs, with original provenance, in a separate tooling change. This note does not claim the historical gates failed. |

Severity for AUD-T3-01 is SHOULD-FIX because the universal proof premise is
refuted but a reachable false publication is not established. ROOT should
obtain design closure before carrying the published-bound assurance into F2a.
If an unmutated source model produces a false claim, escalate to BLOCKING.

## A1 and A2

`A1_A2_REVIEW.md` contains the independent derivation, counterexample, code
trace and limits. A1's local three-term rounding construction is consistent
with its intended formula, but that does not close the upstream scale transfer.
A2's minimum-over-formed-certified-bounds argument is sound under the standing
R7 premises. The examined consumers preserve missing-bound rejection and use B
through the certified upper-bound property. No additional A2 defect was found
by source/proof review. No fresh Rust oracle, mutation suite or scale run was
executed, so this is not a blanket runtime verification of A2 or all R7 lemmas.

## Merge and gate cross-checks

The following six PRs were checked against live GitHub metadata and local Git:

| Slice | PR | Candidate head | Merge |
|---|---|---|---|
| K4 | #1054 | `5a46a6278af3c857a52ecb9be6880990493190f8` | `ab02ee3a68a7af8eb120fa7caa4a2d3daba47191` |
| KF1 | #1056 | `66adfede42de817efb5e0090342c02e3e4382f21` | `0f5d8c7b46570dbb8e7efabe542f99d04d63aad9` |
| V-K | #1057 | `5f0d394262516e4053322730506cb88dfb36a2f0` | `f8400d29059bfa742cffe14028fcd733e89f7a96` |
| K6b | #1058 | `597c81ba4b3dcf9a1154ef438a3c827054a57ad9` | `78f55f927db47c7f44299fd32793d4b64e8b3572` |
| KF3 | #1059 | `aa83f67969c2f618034b856ba6e1fc13ae10762e` | `dd61120ff271b02bf5fcb032f264564af5e8bb09` |
| KF2 | #1060 | `522167ac62f27ad999a4416d10b95f922ff8c665` | `7ad3a9adf479a500ae0d133fa08ace84e4ec4bf2` |

- All six heads, merge identities and merge times match GitHub. Each candidate
  is the merge commit's second parent. Every recorded dispatch base is an
  ancestor of its candidate. Relevant merged trees (`projects/chirality-piping`,
  `tools`, `.github`) match the gated candidates.
- All 30 named final-head CI runs (four pull_request runs and one dispatch per
  slice) succeeded on the recorded candidate heads. Archived selection plans
  confirm each full-SHA `target_base`, candidate head, `mode: full`, full
  coverage selection and `numerical_required: true`.
- KF3's first parent differs from the dispatch base, as disclosed. Relevant
  tree equality holds. This validates the stated consequence of that specific
  slip, not the skipped pre-merge procedure.
- Each preserved DEC-025 SWEEP JSON names the final candidate and a clean tree.
  Suite summaries reproduce the recorded deltas: K4 FK +127 (plus K6's already
  disclosed operation_applier recovery), KF1 FK +8, V-K VR +47, K6b H +20,
  KF3 FK +15 and H +1, KF2 FK +15 passed and +1 ignored.
- The per-manifest summaries retain failures in product_physics and headless;
  stored comparisons name the three known Mac tests. Their identities could
  not be freshly derived from omitted per-manifest logs (AUD-T3-04).
- Named final-head review/confirmation passages were inspected in the committed
  reviews and rulings. Their existence is not a replacement for this audit's
  own design check. Historical GEN-8 passes are recorded in merge records;
  their actual host execution was not independently re-witnessed.

Evidence: `_run_records/merges.json`, `ci.json`, `dispatches.json` and
`dec025.json`. This audit triggered no workflow dispatch and merged nothing.

## Figures and memory reliance

The sample concentrates on corrected or decision-bearing figures; it is not
an exhaustive recalculation of every number in the 3,080-line rulings file.
`_run_records/figures.json` recomputes the two TREE memory comparisons from
raw run summaries and H/VR counts, the B1 coefficients from the preserved probe,
10,000-member outcomes/work/heap from both record sets, and all six suite
summary deltas. `_run_records/ci.json` supplies the twelve numerical-job
durations. The cited rounded CI minutes agree using decimal half-up rounding
(22.25 minutes -> 22.3, 21.45 -> 21.5); Python's default ties-to-even display
would misleadingly suggest two discrepancies.

The corrected KF3-B2 figures hold: the known shift omission is **10,799,688 B**
for each TREE frame. Final H kernel terms plus VR's fixed term exceed the
measured peaks by **6,970,722 B** and **7,511,329 B**. These margins do not prove
that H's phase model bounds the shifted path. The phase omission is derived;
the like-for-like measured excess is absent. No new material figure error was
found in this sample.

E_max has actual consumers beyond presentation: H's counts/admission machinery,
H's binary backstop, VR's port, VR's runner/backstop, and the observation analyses.
Maintained-source searches found no ordinary product caller consuming the W1
estimate. The work graph still leaves final W1 limits unset and F2a unmerged;
this is source/record evidence, not a universal claim about informal reliance.

The audit replayed **all 36 recorded admission decisions** (V-K B and KF3 B),
using the audit-basis runner with recorded baselines and prior-run history. Every original decision field
matches. A sensitivity replay substitutes final H kernel terms, VR's fixed term
and the known shift delta, then recalibrates rho against that same estimate.
**Zero admission decisions change.** This is deliberately not called the
corrected E_max: it has not re-derived all transient overlaps and new refusal
slots. K6c must still do that work and repeat the admission check on its final
formula. Evidence: `_run_records/admissions.json`.

## Open design questions and additional observations

- **KF3-B1 remains an open design question.** The recorded coefficients reproduce
  the routed values. At 4,000 members the three recorded ratios are close but
  not literally identical across precision. The probe uses rescaled loads and
  the K4 adapter; its 10,000-member interpretation remains extrapolation.
  A revised lambda split must retain the complete R7 inequality, including
  the `(1 + 2^-P)` multiplier, rather than treating a rounded residual budget
  of 127 units as a new exact acceptance threshold. No split is selected here.
- **KF2's dense-screen proposal stays separate.** Its zero-prefix induction
  supports omitting provably exact-zero operations from a count. It does not
  establish the proposed product class changes without runs, and published
  operation counts remain a contract consequence. No screen change is approved.
- **Stop/cache evidence:** the reviewed implementation carries refusals out
  before propagating stopped verification builds; budget errors are not A2
  bound refusals. Existing RV23C-N1 remains an uncommitted-regression-test gap,
  not a newly discovered cache bug. No new error-path runtime claim is made.
- **Production isolation:** current retained-api references were examined in
  the kernel, H and VR, together with feature guards. The factor loop covered
  by R7's lemmas has no arithmetic-loop change in the diff from `cef218a10`;
  changes there are observers/accessors, visibility, documentation and a gated
  geometry fault. This is source inspection, not a fresh compiled feature audit.

## Execution limits, repairs and return

The supplied original checkout, T3 scratch root and venv are absent on this
host, and an escalated process check found no memory guard. Accordingly no
cargo/rustc job, timed measurement, dense matrix experiment, DEC-025 rerun or
original-scratch cleanup was performed. The owner was asked where the runtime
resources moved; the source/proof/record audit continued independently.

GEN-8 was run on the clean base with installed Python 3.13, bytecode and pytest
cache disabled, and live roots required: **1 passed, 10 deselected** in 33.72 s.
The prescribed venv was unavailable; this substitution is disclosed and is
not represented as the original Mac environment. The audit did not alter any
runtime resource or weaken a test.

**Repairs merged: none. ROOT-owned branches changed: none.** Only this audit
branch and additive `T3/AUDIT/` files were created. No source repair, instruction
change, hash-bound evidence edit or ruling was made. K6c remains the already
briefed repair for KF3-B2 and VR's stale estimate. It touches H's
`src/k6/w1/counts.rs` and `numerical_robustness`. Any repair for AUD-T3-01 would
likely touch `frame_kernel/src/structural/retained/**` and D2's contract, and
must be explicitly flagged and separately reviewed. Prefer returning both to
the implementation/design agents. Audit records have not received independent
review and are not an accepted design amendment.

The short return is `HANDOFF.md`; reproduction instructions and command limits
are in `_run_records/README.md`.
