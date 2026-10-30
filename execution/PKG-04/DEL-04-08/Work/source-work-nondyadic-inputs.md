# Source route: the exact-arithmetic cost of non-dyadic inputs

This design examines O4's coverage finding (b) (PR #1468, "Coverage findings"; [o4-source-extension.md](o4-source-extension.md) §5, "Census"). On 3D skew single members ((1,2,2), the Z-hinge layout) with ordinary decimal inputs, retained-source recovery (`pressure-source-1`) does not complete within the unchanged 8,000,000 per-case limit (`PHYSICS_SOURCE_WORK_LIMIT`, PP `lib.rs:939`). This happens in 10 of 60 Euler–Bernoulli runs (17 %) and 22 of 60 Timoshenko runs (37 %). Planar members and dyadic twins always complete. The owner decided on 2026-10-20 ("All as recommended") that a remedy be designed now. Its goal is to reduce the exact-arithmetic cost so that these cases fit the unchanged limit.

**Status: design and census.** No product file, fixture, reviewed input or ScopeOfWork changes here. The census of section 5 has run; its results are section 5.1. It ran on the census branch `claude/piping-ra-census-20261021` at `4d956a6b8a` (as repaired after its review), which is not for merging. Revisions 1–4 were written on main `fa019d9cef` (O4 merged as #1468). Revision 5 re-checks them on main `dd8fb0e124` (#1483 merged) and on #1484's head `56676a25a0`, which merged unchanged as main `9e03991d39` (section 0). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Revision 2** answers the first round of dual review (Claude Opus 5.5 and Codex). The changes:
- **B1.** Revision 1's argument for bit-identity was false: `mul`'s −1 shortcut re-grows an expansion where +1 copies it, and a reviewer built a witness. B1's recognition is narrowed to the shapes where the long path provably yields the termwise negation. Its candidate is fixed by index, and its comparison and copies are charged (3.1).
- **B2.** Revision 1's exponent-sum pre-check could not preserve `Range` refusals. B2 now skips a recipe only inside a conservative window in which no refusal is possible, and runs it otherwise (3.1).
- **Today's outcome.** The census documents take the pre-0.4 path, where the limit is crossed after selection. So the likely current outcome is a failed finalization, not an ordinary fallback (1, 6).
- **The census rules** separate preserved cases from newly selected ones. Newly selected cases are judged by the declared-problem oracle and by a test-only long-path comparison (5).
- **Adoption.** The 90 % rule is withdrawn. The target is every case, and adoption is not automatic (5, 6).
- **Input description.** The dyadic twins' inputs are described correctly, and causal claims that rested on the model are withdrawn (2.3).
- **The lists of moved bytes and pins** are completed. They include the U4b work-law test, which hard-codes historical counters (4).
- **Estimates.** The cost estimates now carry the second reviewer's lower B1 share (2.2, Summary). The census switch is opt-in and outside every test build (5). There are wording and formula repairs throughout.

**Revision 3** answers the second round. The changes:
- **Today's outcome** is a refused run, not a blocked envelope with ordinary rows. The product entries return `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`, and the internal envelope is the blocked envelope, with no result rows. The section 6 option is restated as ordinary republication on the pre-0.4 path, since the replay screen alone would not catch these cases (1, 6).
- **B1's dynamic condition** is narrowed to the proved subset: no `fixed` contribution and one contributing block, so `combine` performs no multiplication. Two reviewer witnesses join the gating inputs (3.1, 3.2).
- **B2's window** is specified concretely, with its derivation and charge (3.1).
- **The J7 inventory** counts all 66 directly selected entries, 62 of them method-only, whose work moves (4, 6).
- **The comparison for newly selected cases** applies the preserved cases' allowance exactly, with every changed binding recomputed (5).
- **Smaller repairs:** B1's candidate hint and the charge bounds; the U4b test's publication law; the corrosion allowance of the dyadic twins.

**Revision 4** answers the third round. The changes:
- **The public outcome** is the published blocked envelope, with no result rows and the blocking diagnostic. Revision 3's run-level `Err` arises only when the invocation-level receipt fails (1, 6).
- **B1's block condition** reads "at most one block", so exact-zero rows, 16 of the 22 negated rows here, stay covered (3.1).
- **The J7 counts** are corrected (Summary, 4), and the hint vector is recorded as a priced owner (4).
- **The gating test** asserts which rows take B1, so a byte-equality pass cannot hide a silent fallback (3.2).

**Revision 5** re-checks revision 4 against main as built after epoch 6 (`dd8fb0e124`, with #1484 at `56676a25a0`). No numerical argument, rule, estimate or owner option changes. The changes:
- **Basis.** None of the source-route files R-A edits or cites has changed since `fa019d9cef`, so their citations stand. The citations into `o4_m3_judging_tests.rs` move by +35, and FK `structural.rs`'s window is restated. The row-construction citation and the dyadic twins' allowance condition are made precise (0, 2.1, 2.3).
- **Section 0's assumptions** are restated as built: 5a's six witnesses, REQ-006 (e) (no bearing), and the W1 repair's features, which landed first (0).
- **Section 4** adds two records that R-A's source edits move: the two product-preview generation records and the hosted-routing digests. The J7 counts are recounted from the fixture and hold. The cached-context pin is regenerated rather than edited (4).
- **D3** starts from §8.12's corner-1 profile. The pricing pins that move are named (4).
- **The census switch** reuses the existing `census-probe` feature and its guards rather than adding a feature, under W1R-FG's constraints. The gating test's long evaluation stays reachable after adoption through a test-only entry, so no product switch remains (3.2, 5).
- **Census plan.** Nothing found invalidates it.
- **Its dual review** (Codex and Claude, on `9f76450127`) found nothing blocking. Its repairs are folded in:
  - section 4 now names every per-reader runtime-read hash, MJ6's `CURRENT_SOURCE_BASIS`, the native record and challenge literals, and a fuller, explicitly non-exhaustive list of pricing pins;
  - section 5 sets W1's census switches to their production values in every census thread. Under `census-probe`, W1's corner-1 switch otherwise defaults off. It also adds a control on committed bytes and corrects which files W1R-FG stops on;
  - the test-only long entry's callers, reach, feature comments and gating level are stated.

## Summary

- **Today's outcome** (section 1, derived from the code). The Z-hinge census documents carry no load/reference-state fields, so they take the pre-0.4 path, which has no replay screen (PP `lib.rs` about 4990–5011).
  - The live attempt costs about E_live + 0.14M, which selects well within 8M. The limit is crossed later, in the captured replay and receipt finalization.
  - On that path a finalization failure pushes the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` (`lib.rs` about 6614–6625). The caller then returns the blocked envelope, which carries no result rows (`lib.rs` 2923–2924, `blocked_envelope` 13963).
  - **What the user gets.** The blocked envelope carries the ordinary pressure identity, so the product entries' `source_finalization_failed` test does not fire (`lib.rs` 1082–1088, 13982, 2508–2511). The entries publish that blocked envelope: no result rows, with the blocking diagnostic. A run-level `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` (`lib.rs` 2501–2503, 3229–3231) arises only when the invocation-level receipt fails after a case selected (`finalize_composite`, about 3118–3135).
  - So at the time of this design these cases published **no result rows**, not ordinary results (since [pre04-failure-unification.md](pre04-failure-unification.md), such an invocation republishes ordinarily). That is fail-closed, but it is not REQ-003 (c)'s ordinary route with a receipt failure record. RF §2.9 and DEL-04-07 `pressure-contract.md` record it ("the pre-0.4 path retains its named refusal"). O4's census raw output, which is not committed, would confirm it; section 5 records both the internal and the public outcome.
- **Where the charge goes** (section 2; derived from the code, with uncommitted cost models).
  - A selected case's ledger is about 2 × E_live + 0.93M. E_live is one pass of exact work: Preparation, Solve, and the evaluation of all 73 functional rows and all 97 projections. The captured replay repeats that pass in full, by design.
  - For decimal Euler–Bernoulli inputs, E_live averages about 3.3M; for the dyadic twins, about 2.6M.
- **Why decimal inputs cost more** (section 2.3). A metered pair costs 98 + 6ℓ₁ + 6ℓ₂, where ℓ₁ and ℓ₂ are the output's length before each of its two adds. Expansions of 53-bit atoms grow about twice as fast as those of short atoms.
  - The dyadic twins shorten D, t, L, E, ν, k and m₁. But K_local and T stay 53-bit through π, the division by L and the normalization, so only k and m₁ become short atoms in the exact problem.
  - The model attributes the gap to the expansions downstream of the displacement numerators. That attribution is an estimate, which section 5's matched-input ablations establish.
- **The redundancy** (section 2.4). Of the 30 station section rows per member, 22 are, by construction, the i-end row scaled by −1. Two of those carry extra terms whose products contain an exact zero (`source_recovery.rs:1775–1786`, 1811–1835). Each is still evaluated in full. Coefficient recipes are also formed for DOFs prescribed at exactly ±0 and then multiplied by zero.
- **Recommended remedy: R-A** (section 3).
  - *B1.* A recognized negated row takes its source row's exact Ratio, negated termwise. Recognition is restricted to shapes in which the long path yields exactly that, so published bytes do not change.
  - *B2.* A zero-prescribed recipe is skipped only inside a window in which it could not have refused.
  
  The same evaluator runs in the live attempt and the replay. No value, check, limit or charging rule changes.
- **Measured gain** (section 5.1, the census). R-A removes 18.4 % of the live pass, Preparation to Projection, on decimal Euler–Bernoulli Z-hinge inputs and 17.9 % on Timoshenko. Of the whole direct-method charge it removes 17.7 % (Euler–Bernoulli; B1 alone 14.8 %, B2 alone 3.4 %) and 17.2 % (Timoshenko; B1 alone 14.4 %, B2 alone 3.3 %).
  - Every Z-hinge decimal case that is blocked today, 10 of 60 Euler–Bernoulli and 22 of 60 Timoshenko, selects within the unchanged 8M limit. The largest case charge is 7.241M.
  - No case selected today is lost. On those inputs every pre-registered rule holds: method-level equality, the shadow comparison of every evaluation, both readers, and the declared-problem oracle and its 578 required selections.
  - R-A leaves some cases over the limit: three PEXACT Timoshenko families of the oracle's φ = 2 set, and many-case invocations that pass the 64M invocation ledger. In the latter, the fail-closed form at the product entry can change (section 5.1).
  - The models' estimate was −12 % to −20 %, with maxima of 7.1M to 7.5M.
- **Cost.**
  - One evaluator change in FK `exact_boundary/functionals.rs` with gating and killing tests.
  - The independent numerical review.
  - An actual-source D3 registration.
  - Re-pinned work counters, and the U4b work-law test restated (section 4).
- **Reserved for the owner** (section 6):
  - **AC-002's "charged work … unchanged".** R-A moves the census's charged work, on its 66 directly selected entries (62 method-only), and, on the 4 publicly selected RF PIN-OFF-345 entries, the envelope and diagnostic digests that bind it. Classification, counts, first causes, selections and frozen-row verdicts stay unchanged. The owner can rule that this falls within AC-002, or amend its wording. This is the only ruling R-A itself needs.
  - **The blocked envelope.** Extending the 0.4 path's ordinary republication on finalization failure (SF-1R) to the pre-0.4 path would turn a blocked envelope with no result rows into an ordinary publication. That would change recorded behaviour, with no coverage gain. It is presented as an option, without a recommendation.
  - **Further reductions** (3.3), each a second stage with its own ruling. None is recommended now.
  - Raising the 8,000,000 limit is not designed here (REQ-003 (e), AX-005).

## 0. Basis and dependencies

**Basis.** Main `dd8fb0e124`, checked also on #1484's head `56676a25a0` (revision 5). Revisions 1–4 were written on `fa019d9cef`. Paths are relative to `projects/chirality-piping/`; PP is `core/product_physics/src/`, FK `core/solver/frame_kernel/src/structural/`.
- **What moved under the citations.** Between `fa019d9cef` and `dd8fb0e124`, and on to `56676a25a0`, no byte changed in PP `lib.rs`, `source_recovery.rs`, `source_recovery/`, `source_receipt.rs`, `source_receipt/` or `case_state/`, or in FK `exact_boundary.rs` or `exact_boundary/`. No committed source-route fixture or reference changed either. So every citation into those files stands as revision 4 gave it; a TASK contributor re-read each one on both heads. Three cited files moved:
  - `o4_m3_judging_tests.rs`: W1-E's test `o4_w1e_sharper_allowance_refusal_keeps_the_ordinary_result` was inserted at line 748 (+35 lines), and the `census-probe` inclusion of `w1e_census_tests.rs` was appended. Its citations below carry the shift.
  - FK `structural.rs` gained the `CENSUS_PROBE` constant (+6 lines at 69), and #1484 adds `LOADED_MAXIMUM_HOOKS` (+6 more). On `dd8fb0e124` `Expansion::add` begins at 797 and `add_product_charged` ends at 883. On `9e03991d39` (#1484) `impl Expansion` is at 802, `add` at 803–829 and `add_product_charged` at 860–889, a few lines past revision 4's "about 790–880".
  - Both `Cargo.toml` files gained the off features (section 5).
- **The census figures** (17 %, 37 %, 21/50 and 20/38 above 7.5M) are O4's, from `o4_m3_judging_tests.rs` `o4_census_source_work_single_members` (lines 997–1038), as reported in PR #1468. Their raw output (`O4_CENSUS_OUT`) is not committed.
- **This design's own figures** come from two standard-library Python cost models in `/tmp` scratch, not committed. One was written for the design, the other independently by a reviewer. Both re-implement `Expansion`, `Work` and the metered rule. The design's model reproduces RF's measured (1,2,2) congruence charge: 83,882 [E] against 84,268 [M] (rf-source-method-extension.md §3.5).
- **Tags** mark each figure: **[D]** derived from the code; **[E]** estimated with a model; **[M]** measured in a committed document.

**Concurrent work as built, and sequencing** (revision 5 re-checked each assumption against the merged code).
- **Stack-headroom repair (b7-plan item 5a, #1476).** It split `freeze_case`'s closure into stack phases on the retained W1 path, re-registered D3 (`retained-pricing-rules.md` §8.10), and put the six stack witnesses into maintained checks. The six are three witnesses in two modes (`witness_headroom_w1_at_one_mebibyte`, `witness_w2_deep_milestone_publishes`, `witness_w_c2_publishes`, each `sparse` and `dense`; `tools/release/check_release_readiness.py` `STACK_WITNESSES`).
  - **Holds.** 5a changed none of `exact_boundary`, `functionals` or `source_recovery`, so R-A does not rebase on it.
  - **Sequencing.** R-A's implementation follows 5a and registers under D3 on the current profile (section 4). It re-runs the six witnesses at R/64 in the registered profile, as AC-004 requires after 5a. They run one process each through `check_release_readiness.py --profile stack-witnesses`, which is the release sweep's record on the qualification host. The hosted numerical job runs them as an early warning whenever product_physics is selected (`tools/ci/numerical_ci.py` `cargo_plan`). Neither command passes a cargo feature.
- **REQ-006 (e) and its amendment (#1481, #1482).** These state the sharper-allowance limit of retained-precision successors. They bear on W1-E publication, not on the source route's work or outcomes, so they do not affect R-A.
- **W1 coverage repair (epoch 6: corner 2 in #1483, corner 1 in #1484).** It changes the W1 kernel's loaded-maximum search, the receipt's scales and the W1R pricing rules. R-A changes the source route's evaluator. The two share no numerical law and no source file that R-A edits.
  - **Holds.** The W1 repair moves no source-route work counter, and no source-route fixture or source file changed (above).
  - **Landed first.** The W1 repair's census added the opt-in `census-probe` feature to both manifests, and corner 1 adds `loaded-maximum-hooks`. Both are guarded as off features by the pricing tool's W1R-FG rule (`retained-pricing-rules.md` §8.11 and §8.12). R-A, landing second, takes this feature discipline as given (section 5) and registers on corner 1's profile (section 4).
- **O4 as built.** R-A keeps O4's work law (o4-source-extension.md §3.5) and the D5/MJ9 accounting. The direct change is in Evaluation-stage work. Secondary changes follow from text lengths (section 4).
- **#1484 merged unchanged.** Its head `56676a25a0` is main `9e03991d39`'s second parent, so the facts read on it hold on main.

## 1. The ledger, the limit, and today's outcome

- **The rule.** The per-case ledger is the metered `Work` of RF §5 (C₂ = 96 per term pair, plus 6·len + 1 before each grow-expansion add), with the finalization and publication reservations of RF §2.9 and O4 §3.5. `Work::charge` refuses before any work when the next total would pass the limit (RF §5.2 (1)).
- **Where these cases cross it.** The live attempt fits; its estimate is about E_live + 0.14M [E]. So these cases cross the limit in the captured replay and receipt finalization, which share the case ledger (RF §2.9). They take the pre-0.4 path: `case_state::is_load_state` is false for documents without reference configurations, expansion laws or analysis states, which `census_document` authors none of.
- **The resulting outcome.**
  - *Inside the run* (`lib.rs` about 6614–6625): the selected source response is lost, and the case pushes the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`, whose text carries the `Budget` error and the finalization work. The caller then returns the blocked envelope: no result rows, empty physical evidence and incomplete status (`lib.rs` 2923–2924; `blocked_envelope`, 13963).
  - *At the product entries:* the blocked envelope carries the ordinary pressure identity (`mechanics_producer_for_model`, `lib.rs` 1082–1088, used at 13982), so `source_finalization_failed` (2508–2511) does not fire. The entries therefore publish the blocked envelope.
    - The run-level `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` (2501–2503, 3229–3231) arises only when the invocation-level `finalize_composite` fails after the producer has been set to pressure-source (about 3118–3135).
    - The pinned `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` asserts that `Err` on its own input, and calls the inherited outcome "Characterization only, not an endorsement … a separate open finding assigned to T3".
    - The Z-hinge overrun arises on the case's own ledger, in the per-case captured replay and finalization, so it is expected to take the blocked-envelope path.
  - This was the recorded pre-0.4 behaviour (RF §2.9; DEL-04-07 `pressure-contract.md`) until [pre04-failure-unification.md](pre04-failure-unification.md) unified the routes: such a case now republishes the invocation ordinarily. It is fail-closed, but the user gets no result rows for the run, not the ordinary result of REQ-003 (c). That differs from a failure later, at the invocation's publication reservation, and from the 0.4 load/reference-state path. There, a selected join that cannot finalize is republished on the ordinary route (SF-1R). The 0.4 replay screen alone does not guarantee finalization (`lib.rs` 4996–4999).
- **Confirmation.** O4's census function calls `run_linear_static_preview_observed` directly and records `selected`, the case charge and the `SOURCE_BLOCK_RECOVERY` diagnostics (`o4_m3_judging_tests.rs` 980–994). So it sees the same blocked envelope the entries publish; it would not see an invocation-level `Err`. Its raw output would confirm the code; section 5 records both.

RF §5.2 (3) states the rule's soundness: executed `Work` arithmetic is bounded by the charged total, which never exceeds the limit. R-A keeps that rule unchanged. It removes executed arithmetic, so it removes the charges for that arithmetic. Every new step it adds is charged before it runs.

## 2. Cost anatomy of one Z-hinge attempt

### 2.1 The problem [D]

- **The layout** (`census_document` "zhinge122"): one (1,2,2) member; N0 rigid in UX, UY, UZ, RX and RY, with a soft RZ spring; N1 rigid in UZ, RX and RY; one moment about z at N1.
- **DOFs.** n = 12. Four are free: N0.RZ, N1.UX, N1.UY and N1.RZ. They form one block of order 4. The other 8 DOFs are prescribed at exactly 0.0.
- **Rows.** 73 functional rows: 12 end, 30 section, 12 nodal, 1 spring and 18 support. The nodal and spring rows are built at `source_recovery.rs` about 1455–1503 and the support rows at about 1504–1540, the end and section rows in `member_descriptors` (1741–1800), and the member and support rows are read back at about 1897–1925, the nodal rows at about 1963–1964. There are 24 DOF projections.
- **The congruence.** T has 8 nonzeros per 3×3 block, and K_local has 40. So 120 of the 144 exact congruence entries are nonzero.
- **The response** is a near-rigid rotation about global z, so 49 of the 73 rows are exact zeros. Each is still evaluated at full cost before it cancels.

Both reviewers confirmed this structure, one with an exact-rational re-solve.

### 2.2 Where one live pass spends its work [E]

Shares of E_live, decimal Euler–Bernoulli, from the design's model:

| Stage | Share |
|---|---|
| Evaluation: section rows | 22.7 % |
| Solve: reactions Σ k_ij·num_j (`exact_boundary.rs` about 1097) | 19.2 % |
| Evaluation: support rows | 11.7 % |
| Preparation: leading minors Δ2–Δ4 (about 630) | 9.1 % |
| Evaluation: end rows | 8.5 % |
| Preparation: adjugate (about 652) | 7.9 % |
| Projection: functional rows | 6.9 % |
| Evaluation: coefficient recipes | 5.2 % |
| Solve: block `ratio_add` | 2.7 % |
| Preparation: congruence Tᵀ·(K·T) (163–209) | 2.5 % |
| Other (symmetry, scatter, solve precharges, numerators, DOF projections) | about 3.5 % |

**The 22 negated rows' share.** The design's model gives them 17 % of E_live. The reviewer's model gives them 52–56 % of the section-row work, about 12–14 % of E_live. The reviewer also notes that each of the eight unmatched component 4–5 rows carries 12 more terms with 5-atom products, so it costs more than a matched row. The census measures this share per row (5).

**Outside E_live:**
- `prepare_sources` runs three times (about 52k each [E]);
- `source_preflight` four times (about 8k each);
- `descriptors_charge` five times (about 32k each);
- retention (about 45k), output (2,720 [D]), the maxima and norms (about 56k);
- the finalization reservation, about 0.52M [E]. The (3,4,0) member's was measured at 379,940 [M] (RF §2.9).

Together these make about 0.93M [E].

**Repetition [D].**
- The full exact pass runs twice, live and in the captured replay (`functionals.rs` `replay_against`, about 977–1057). The binding check (`check_binding_with_budget`) repeats no arithmetic.
- Within one pass, minors are cached, and the adjugate is formed once and reused in Solve.
- Section rows are not reused, and neither are the prescribed-zero coefficient recipes (2.4).

### 2.3 Why decimal inputs are dearer [D, E]

- **The charge [D].** A metered pair costs 96 + (6ℓ₁ + 1) + (6ℓ₂ + 1). ℓ₁ and ℓ₂ are the output's length before the pair's lo and hi adds (`metered_product`, `exact_boundary.rs` about 363–384; `add_product_charged`). So `mul(a, b)` costs about |a|·|b|·(98 + 12ℓ̄), where ℓ̄ is the mean output length during the product.
- **53-bit atoms [D].** A two-product of two 53-bit significands leaves a nonzero low part. Grow-expansion with zero elimination keeps it, so each pair adds about two terms. Short atoms whose significand widths sum to 53 bits or less give lo = 0, and the sweep merges neighbours.
- **What the dyadic twins change [D].** `census_document` (`o4_m3_judging_tests.rs` 930–942) rounds D and t to 6 significant bits, L to 8, k and m₁ to powers of two, and sets E = 1.5·2^37 and ν = 0.3125. Under the allowance setting `both`, the mill tolerance is 0.125·t, and the corrosion allowance is 0.25·t for t < 0.006 and otherwise the decimal 0.0015, in both styles (line 942); under the other setting both are zero.
  - K_local and T still carry 53-bit atoms: π enters A, I and J, entries are divided by L, and T comes from l/3 and a square-root normalization.
  - So in the exact problem it is chiefly k and m₁ that become short atoms. Other atoms change value but not width.
- **Lengths [E], design model.** Mean expansion lengths, with canonical lengths in brackets:

  | Quantity | Decimal | Dyadic |
  |---|---|---|
  | block entry | 3.5 (3.0) | 3.5 (3.0) |
  | Δ4 | 26.7 (11.3) | 26.2 (10.8) |
  | adjugate entry | 19.5 (9) | 19.1 (9) |
  | displacement numerator (adj·m₁) | **14.0 (10)** | **9.6 (9)** |
  | reaction numerator | 14.0 (12) | 12.6 (10.8) |
  | nonzero row numerator | 23.4 (11.1) | 19.8 (9.8) |

  In that model, Preparation's lengths agree across the styles and the gap is downstream of the displacement numerators. These are model observations under changed values. The census's matched-input ablations (5) establish the attribution.
- **Timoshenko.** The design's model sees about +2 % on average. PR #1468 measured +2–5 %, and about +11 % near φ = 2. A third of the sweep is near φ = 2, which together with a threshold effect near the limit would account for the higher Timoshenko failure rate. The census attributes it (5).
- **Representation alone cannot help [D].** Exact power-of-two scaling, or factoring out a common exponent, saves nothing: C₂ is a flat 96 per pair, and significand widths do not change. A big-integer engine would need a new cost model, which RF §5's ruling ("one rule") does not provide. It is not designed here.

### 2.4 The duplicated work [D]

- **Section rows.** `member_descriptors` (`source_recovery.rs:1741–1797`) builds each station row as `scaled_row(&rows[component], -1.0)` from the member's i-end row. For components 4 and 5 it also calls `append_scaled_products`, which appends new `AffineTerm`s: the shear row's terms with [sign, fraction, length] appended to each product (1811–1835). At the i-end station the fraction is 0.0, so every appended product contains an exact zero.
  - So components 0–3 at all five stations are the i-end row with −1.0 appended to every product: 20 rows.
  - Components 4–5 at the i end are the same, followed by appended terms whose products all contain a zero: 2 rows.
  - That is 22 of the 30 section rows per member.
- **How `evaluate` treats them.** `evaluate` (`functionals.rs` about 563–630) forms each term's coefficient by `recipe`, which folds the product's atoms through `mul`. For the −1.0 atom it takes the ±1 shortcut inside the recipe, which re-grows the expansion rather than copying it (514–528). It then accumulates the coefficients, multiplies each by the displacement numerator, and combines per block.
- **Prescribed zeros.** `evaluate` forms every term's coefficient before it reads `c.source.prescribed`. For a DOF prescribed at ±0.0, `mul(coefficient, scalar(0.0))` returns the empty expansion, so the coefficient's recipe was computed for nothing. Eight of the twelve DOFs here are prescribed at zero.

## 3. The remedy

### 3.1 R-A: reuse the negated row, and skip zero-prescribed recipes

**B1: negated rows.**
- **Candidate by hint.** PP builds the descriptors (`member_descriptors`), so PP passes FK's `evaluate`, beside the flat descriptor list, one optional candidate index per row. A section row (member, station, component c) names its member's i-end row c. The hint is not hashed and changes no descriptor. FK verifies it by the recognition below, and a missing or mismatched hint takes the long path. There is no search. The replay rebuilds the hints with the descriptors through `prepare_sources`.
- **Recognition (static, bitwise, on the descriptors).** Every condition must hold:
  1. d's first terms list s's terms in order, with equal DOFs, and each of d's products equals the corresponding product of s with one appended atom −1.0. The offset likewise matches.
  2. Any further terms of d are such that every product contains an exact ±0.0 atom. Before its first zero atom, each such product is an already-evaluated product, here the shear row's [K, T], followed only by ±1 atoms. So the long path could not have refused before reaching the zero.
  3. Every product of s has at most two atoms, and no atom of s is ±1.0.
- **Recognition (dynamic, on s's evaluation).** Every condition must hold:
  1. s's `fixed` is empty: no offset and no contribution from a nonzero prescribed value;
  2. at most one block contributes a nonzero numerator to s. With none, as for an exact-zero row, s and d both end as `constant(empty)`, Ratio{[], [1.0]}, without a `combine` call, and that equals B1's negation of s;
  3. no coefficient of s is a single term ±1.0. As built, this reads the coefficients after B2's skip: a coefficient B2 skipped on a zero-prescribed DOF is not formed, and on the long path it meets only `mul` with the empty prescribed value, which returns before `mul`'s unit shortcut, so it cannot reach the asymmetry below (Opus review, round 1).
  
  With (1) and (2), `combine` either is not called or returns the block's Ratio unchanged (`functionals.rs` about 542–549: a zero numerator returns the other operand), so no `mul` in `combine` touches a negated operand. These conditions hold for the Z-hinge rows, which have no offset, zero prescribed values and one block. That includes the exact-zero rows, 16 of the 22 negated rows, which have no contributing block.
- **Value.** d's Ratio is s's Ratio with its numerator negated termwise; the denominator is shared. That is d's exact value: the −1 factor negates exactly, and the appended zero products contribute exactly zero.
- **Why the bytes equal the long path's.**
  - Under the static conditions, each of d's long-path coefficient recipes re-grows either a single atom or a two-product output [lo, hi]. Re-growing either from empty reproduces its terms negated: hi = RN(a·b), and a tie resolves to hi.
  - Every later step is sign-symmetric under round-to-nearest-even: the metered two-product of a non-unit coefficient by a numerator, coefficient accumulation and the block numerator's grow-expansion with zero elimination.
  - `mul`'s ±1 shortcut copies for +1 and re-grows for −1. That is asymmetric only when the unit is the negated operand and its partner has more than one term. Dynamic condition (3) excludes this for the coefficients. Where the numerator is a single ±1, the unit sits on the un-negated side, where copy and re-grow act symmetrically. Conditions (1) and (2) exclude `combine`'s multiplications altogether.
  - The appended zero-product terms add nothing: their coefficients are empty, so they reach neither the numerator nor `fixed`.
  - An exact-zero source row is a gating input. So are two reviewer witnesses (3.2), each of which fails a condition, so it takes the long path. One is a unit coefficient re-growing a multi-term numerator. The other is a row 2·u_prescribed − 2·u_free with prescribed value 0.5, whose `fixed` is a unit that `combine` multiplies.
- **Projection still runs** on every row by the existing `project`, so no property of `project` is assumed.
- **Charges.**
  - The static recognition is charged before it runs, in units like `descriptors_charge`'s: one unit per compared atom and per term. Its cost is bounded by the row's descriptor size.
  - The dynamic checks are charged one unit per inspected expansion. The termwise negation of the numerator is charged its actual length, as `copy_exp` charges, and so is the shared denominator's clone. These depend on the solve, not the descriptor; each is at most the 256-term expansion cap.
  - Every such charge enters the independent oracle (3.2).
- **Fallback.** A row that fails any condition takes the long path, as today.

**B2: zero-prescribed recipes.**
- **The skip.** In `evaluate`, a term whose DOF is prescribed at exactly +0.0 or −0.0 skips its coefficient `recipe` only when a conservative window proves that the recipe cannot raise `Range`.
- **The window.** Atoms are already validated finite (`functionals.rs` `products_charge`, about 434). So the recipe's remaining refusals are `exact_radix` bit loss on a lo or hi part, and two-sum overflow in `Expansion::add` (FK `structural.rs` about 802–889 on `9e03991d39`).
  - **The predicate.** Over all terms of the row on that DOF, every atom is a normal binary64 with binary exponent e satisfying |e| ≤ 150. Every product has at most 5 atoms. The DOF has at most 64 products in the row.
  - **Why no refusal is possible inside it.**
    - Write each atom as an integer times 2^(e − 52). Every partial product of j ≤ 5 atoms, and every term of every expansion formed from it by two-product and two-sum, is then an integer multiple of 2^(Σe − 52j) ≥ 2^(−150·5 − 260) = 2^-1010. That is above the subnormal grid 2^-1074, so no `exact_radix` scaling loses a bit.
    - Every such magnitude is below 2^(Σ(e + 1)) ≤ 2^755, and a sum of at most 64 of them is below 2^761. So no two-sum overflows. The expansion-length cap can only raise `Budget`.
    - Today's Z-hinge atoms (stiffness entries, transform entries, signs, fractions and lengths) lie far inside the band.
  - **The charge.** One unit per atom inspected plus one per product, charged before the inspection.
  - **Outside the window** the recipe runs, as today.
  - **Tests.** `b2_window_keeps_every_refusal` places inputs immediately inside and outside each bound (|e| = 150 and 151, 5 and 6 atoms, 64 and 65 products). It also covers the reviewer's rescued-underflow product [k, sign, 0.25, 4] with k = 0x1.1abe4b73fefb7p-1021, which is outside the window and must refuse as today.
- **Consequence.** Inside the window the recipe could not have refused, so no `Range` outcome can change. A skipped recipe's `Budget` refusal disappears with its work (RF §5.2 (5)). If a later stage then refuses, that cause is exposed instead.
- **The overhead.** B2's window charges, and B1's recognition charges, are paid whether or not a shortcut then applies (the schedule below). So an evaluation in which neither applies charges more than today's: per evaluation 3n + the prescribed count + the sum of the block orders, and per long row |d.terms|, atoms(P) of each zero-prescribed term and, with B1 active, n + 2 on an i-end row, plus whatever B1 recognition a hinted row spends before it declines; and per attempt the plan's and the replay's hint charges, 2 × the descriptor count each. An accepted B1 row can also cost more than the long row it replaces where the row is very short: a source row of one single-atom term costs the long path about 14 units and B1 about 25 (its key bytes, recognition, the dynamic read and the copy). The source route's section rows, a dozen two-atom terms at about 130 units each before R-A, are always cheaper by B1 while their identifiers stay within the 128-byte text cap (Opus review, round 2). A case within that margin of the 8,000,000 or 64,000,000 limit can therefore refuse with `Budget` where today it completes. It falls back as REQ-003 (c) states; no limit, check or charging rule moves, and the refusal is `Budget`, never `Range`. The implementation review found this (Codex, the implementation's test slice); it corrects this section's earlier statement that a `Budget` refusal could only disappear. `b2_declined_scans_charge_exactly_their_overhead` pins the overhead on a recipe immediately outside the window. On every committed record the savings exceed it: no counted-work leaf rose in the re-pins (§4, "As built").
- **Cross-attempt effect.** In a multi-case invocation, a case that newly selects charges the shared invocation ledger. A later case then has less room and could lose its selection, or reach the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` at its publication reservation. RF §5.2 (6) states this cross-attempt exception; it fails closed. The census includes multi-case invocations (5).
- **Prescribed values that are not zero** keep today's path.

**As built: where B1 and B2 sit, and their exact charge schedule.**
- **Where they sit.** FK `exact_boundary/functionals.rs`: `mod shortcuts`, called from `evaluate`. B1's hints come from a hint rule (`HintRule`, a map from a descriptor's index to its candidate source row) given to `FunctionalPlan::new_with_hints` in the live attempt and as the expected rule of `RetainedFunctionalSet::replay_against`. The kernel charges the rule, then builds the hints from it, in each. product_physics' rule is `Sources::hint`: the member rows come first, each member's 12 end rows then its 30 section rows, so a section row's candidate is its member's i-end row of the same component. The implementation review (Codex, round 1) found that the hints were first built in `prepare_sources`, before their charge; the rule moved their construction behind it.
- **When they are active.** B2 is active in every product evaluation. B1 is active when the hint vector has the descriptors' length. The test-only `Response::evaluate_functionals_long` turns both off.
- **The schedule.** Every explicit charge, in order (n is the DOF count; atoms(P) is the number of atoms of the products P plus the number of products):
  - *The plan,* with a hint rule: after `descriptors_charge`, 2 × the descriptor count, for the rule's evaluation and the hints' store, before they are built.
  - *The replay,* with a hint rule: the same 2 × the descriptor count, before its hints are built and before its evaluation.
  - *At the start of an evaluation,* B2: 3n + the prescribed count (the zero-prescription vector, the window's two per-DOF counters and the read of the prescribed list), then the sum of the block orders (a guard: a zero-prescribed DOF is never free). If a zero-prescribed DOF lies in a block, B2 is off for the evaluation; the charges stand.
  - *For each row, first B1,* when the row is a section row whose hint names an earlier row j:
    - the source key's bytes (`key_bytes`). If the source is not the same case's i-end row of the same member and component, the row takes the long path.
    - Then atoms(s.offset) + atoms(d.offset) + |s.terms| + Σ atoms(s's term products) + |d.terms| + Σ atoms(d's term products). If static condition (1) or (3) fails, the row takes the long path.
    - If d has further terms: for each row r of ds[j − component ..= j], in order, key_bytes(r) + |r.terms| + Σ atoms(r's term products), stopping at the first that matches condition (2). If none matches, the row takes the long path. Each candidate's charge covers that candidate's atoms. The comparison also reads d's extra-term atoms past the candidate's prefix (the ±1 atoms up to the zero), which the static charge counted once; on a second or later candidate those reads are not charged again. That is at most five re-reads of atoms already charged, comparisons only, with no `Work` arithmetic (Opus review, round 2).
    - Then 1, to read the source row's recorded dynamic conditions. If they fail, the row takes the long path.
    - Then |numerator| + |denominator| of the source row's Ratio, and the negated Ratio is the row's value.
  - *For each row that takes the long path:*
    - B2: |d.terms|, then for each term on a zero-prescribed DOF, in order, atoms(that term's products). A DOF whose terms all lie in the window (normal atoms, |binary exponent| ≤ 150, at most 5 atoms per product, at most 64 products) skips those terms' recipes and accumulation.
    - Then the long path's own charges, without the skipped recipes.
    - For an i-end row with B1 active: n + 2, after the prescribed loop and before the fixed part is read, for the dynamic checks.
- **What the census measured.** The census implementation differs in three ways: it derived B1's candidate by an uncharged key search instead of the hint, it charged B2's per-row window also on rows that then took B1, and it had no hint charges. So the implementation's charges differ slightly from §5.1's; the re-pinned work is the implementation's.

**The replay.** `replay_against` calls the same `evaluate` on freshly prepared descriptors, so the replay recomputes every row from the current invocation by the same rule. Nothing retained from the live attempt is reused. (Reusing the live attempt's cofactors in the replay was considered and rejected: it would replace the replay's recomputation with retained state.)

**What does not change:**
- every value, interval, bound, basis and published byte of a case selected today (by the bit-identity argument above, which the gating test checks);
- the descriptors, `normalized_source_sha256`, `functional_plan_sha256`, `finalization_size` and the unit counts;
- the block search, Sylvester's warrant, the adjugate solve, the shared-denominator check, `BlockWitness` and replay;
- C₂ = 96 and the metered rule;
- the 256-DOF, 16,384-term and unit, 256-expansion-term, 8,000,000 and 64,000,000 limits;
- every reader check. The readers check the ledger's structure and limits and do not recompute charges (RF §5.4).

**What moves:**
- *directly,* the counted Evaluation-stage work, in the live attempt and the replay, and the Plan and Replay stages' hint charges;
- *secondarily,* every charge that depends on the length of a text that prints work. The SELECTED diagnostic embeds the recovery summary (`lib.rs` about 5205–5211), and publication charges 12 × diagnostic text length (`source_receipt.rs` about 790–798; b7-plan "D5/MJ9's accounting clarification"). So a work figure that crosses a decimal digit boundary moves publication work too.

### 3.2 Soundness, mutants and tests

- **Soundness.**
  - B1's value is exact by −1·x = −x and 0·x = 0.
  - B2's skipped contribution is exactly zero, and the window excludes every refusal.
  - Both charge before they execute, and every step they add is charged (apart from the bounded comparison re-reads of B1's extra-term search, §3.1, which do no `Work` arithmetic), so RF §5.2's argument stands: executed `Work` arithmetic never exceeds the charged total.
  - The maintained control ("charged ≥ counted", RF §5.4) still runs over the census and its replays.
  - The independent metering oracle (`metered_mul_charges_match_the_independent_oracle`) gains B1's recognition, negation and copy charges and B2's window charge, each derived independently.
- **Gating tests, as built.** Each compares every row's Ratio from the product evaluation with the long evaluation's (`Response::evaluate_functionals_long`, B1 and B2 off), bit for bit, and records each row's path.
  - `source_negated_rows_equal_the_long_evaluation` (PP `source_recovery/ra_gating_tests.rs`): the J7 RF census inputs and the J2 corpus's genuine source invocations (the 24 raw source controls and the RF bases), in both modes. Exactly 86 of these evaluations reach a plan, and the test asserts that count, that some rows take B1 and that some terms take B2's skip (3,024 rows and 6,650 terms).
  - `o4_source_controls_negated_rows_equal_the_long_evaluation` (PP `source_receipt/o4_source_tests.rs`): `o4_source_controls`.
  - **Coverage assertion,** `ra_gating_zhinge_negated_rows_take_b1` (PP `o4_m3_judging_tests.rs`): on the 24 Z-hinge census inputs, all 22 negated rows per member, exact-zero rows included, take B1. So a byte-equality pass cannot hide rows that silently fell back to the long path.
  - The constructed inputs are FK unit tests (`functionals/shortcut_tests.rs`): the re-grow witness, a source row with three-atom products, unit stiffness, unit coefficients, near-match rows, nonzero and signed-zero prescriptions and each window bound. Each must take the long path where §3.1 says so, with the long path's bytes or refusal.
  - **Not row-gated.** The load/reference-state controls (MJ6/MJ9, the only product inputs with nonzero prescribed values) and section 5's multi-case invocations do not reach a plan through the gate's direct method, which builds no load state. They are covered end to end instead: their documents are regenerated by their own writers, and `work_only_envelope_diff` requires each to differ from its pre-R-A bytes only in work counters, the text that embeds them and the hashes binding those (§4). The FK nonzero-prescription test kills B2's nonzero mutant directly. (Opus review, round 1, found the earlier wording wider than the tests.)
- **Mutants,** as run (Codex's mutant run at `73829f3c24`, the reviews' own, and the repairs' re-runs). Each is killed by the named tests:

| Mutant | Killed by |
|---|---|
| B1 negates a row whose products differ in one atom: the first, or the last before the appended −1.0 | `b1_recognition_refuses_near_matches`; `b1_refuses_a_difference_in_the_last_atom` |
| B1 accepts an appended atom other than exactly −1.0 (+1.0, −2.0, ±0.0) | `b1_recognition_refuses_near_matches` |
| B1 reuses the row unnegated | `b1_negates_valid_rows_and_preserves_zero_sources`; the oracle and the charged ≥ counted control; all three PP gating tests |
| B1 accepts an extra term whose product has no zero atom, or a non-unit atom before its zero | `b1_recognition_refuses_near_matches` (cases matching every source term); the PP gating tests |
| B1 drops the static atom-count or unit condition, or a dynamic check (unit coefficient, empty fixed part, at most one contributing block) | `b1_recognition_refuses_near_matches`; the oracle and the control |
| B1 uses a hint at or after its own row | `b1_recognition_refuses_near_matches`; the oracle and the control |
| B2 skips a recipe on a DOF prescribed at a nonzero value | `b2_window_keeps_every_refusal`, `b1_recognition_refuses_near_matches`, `all_fixed_and_mixed_partition_include_affine_offsets`; the oracle and the control |
| B2 skips outside the window (exponent 151, 6 atoms, 65 products, subnormal atoms) | `b2_window_keeps_every_refusal`: rescued intermediate underflow ([k, sign, 0.25, 4] with k = 0x1.1abe4b73fefb7p-1021), low-tail bit loss, addition overflow, zero atoms and unit atoms, each refusal unchanged; the exponent mutant also `b2_declined_scans_charge_exactly_their_overhead` |
| an uncharged shortcut, recognition, window scan or hint construction | `metered_mul_charges_match_the_independent_oracle` and `metered_operation_count_control_has_no_shortfall`; the window scans also `b2_declined_scans_charge_exactly_their_overhead` |

### 3.3 Further reductions (second stage; not recommended now)

Estimates against E_live after R-A, from the design's model:

| Candidate | Saving | What it changes, and who rules |
|---|---|---|
| R1: station moment rows formed from end-row values, −num_c + s·f·L·num_shear | about −10 % | Ratio terms differ, so projections, values and `response_digest` can move. Done inside `evaluate`, it changes no descriptor. Done by rewriting descriptors, it moves `functional_plan_sha256` and `normalized_source_sha256`, since the normalized payload carries `lowered_recovery_operands` (`source_receipt/source.rs` about 446–485). Needs its own gating and ruling. |
| R4: support rows taken from the Solve reactions | about −14 % | As R1. |
| F1: one extra `Work::add(e, 0.0)` sweep at k, the minors, the determinants and the numerators | about −10 % | An existing charged primitive, but expansion terms change, which can move projections, `response_digest` and J7's frozen bytes. Needs an AC-002 ruling for more than work. |
| F2: Shewchuk compression of expansions at the same points | about −30 % | A new metered primitive, charged before it runs, with the counter and oracle extended. It amends the scope of RF §5's "one rule" (HELP_HUMAN's ruling, 2026-10-16), so it needs that ruling again. Its byte effect needs its own gating. |
| R3: skip the unit-denominator multiplications in reaction `ratio_add` | about −1.8 % | `ratio_add` multiplies by [1.0] through metered `w.mul` (`exact_boundary.rs` about 517–523), which re-sweeps. Skipping it changes reaction terms, so DOF projections and `response_digest` can move. As F1. |
| Upper-triangle symmetry check | about −0.2 % | No byte effect: today's check tests both orders (`exact_boundary.rs` about 817–829). Could join R-A. |

Together these measured about −66 % in the design's model. They are listed so that a later coverage need can take them in order, with their rulings.

**Not available:** raising `PHYSICS_SOURCE_WORK_LIMIT`. That is a priced-source change and a limit relaxed to obtain a bound, on which REQ-003 (e) and AX-005 bear. Reserved for the owner and not designed here.

## 4. Committed bytes that move, and the checks that bind them

R-A changes counted work and the text lengths that print it. For a case selected today, nothing else moves. Each record is regenerated by its own writer:
- each `pressure-source-1` receipt's `work` and `invocation_work`, and the hashes binding them (`receipt_sha256`, envelope digests);
- the SELECTED diagnostic text;
- the 24 raw source controls' work leaves (RF §5.5 counted 6 leaves each), their derived documents and J2's bases;
- `fixtures/product_preview/pressure_source/o4/o4_source_controls.json` (its five `invocation_work` blocks), read by `source_receipt/o4_source_tests.rs` and `tests/test_o4_source_verifier.py`;
- MJ6/MJ9's counters, including MJ6's `CURRENT_SOURCE_BASIS` in `tests/pressure_source_join.rs`, which pins each raw publication's length and sha256;
- the pressure-source reader corpus `fixtures/results/pressure_source_reader_corpus.json`, `fixtures/results/loadReferenceSourceLedger.cases.json` and `core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json`, wherever they carry work;
- the load/reference-state twins `fixtures/results/pressure_source_load_state_*_{dense,sparse}.{document,analysis_run,stress_neutral}.json`, which the desktop parity tests read (DEL-04-07 Design `desktop-exact-outputs.md` §9). Their writers are result_export `tests/load_reference_source_contract.rs`, `tests/test_load_reference_source_readers.py` and `tests/test_stress_neutral_physics_source.py`. `fixtures/**` selects no registered check, so the desktop tests are run with them;
- J7 (`fixtures/results/j7_rf_census_expected.json`, through `SWBPIPE_J7_RF_CENSUS_WRITE=1`).
  - J7 runs the direct method independently of public selection (`j7_rf_census.rs` about 252–295), and records selected method work. So all 66 directly selected entries move in their method work: 62 `selected_method_only` and 4 `selected_published`.
  - Only the 4 published entries, the RF-SKEW PIN-OFF-345 r1e-08 and r1e-12 cases in both modes, also move in their envelope and diagnostic digests.
  - Of the other 256 entries, 186 have work fixed before Evaluation (168 block-order, 10 `Budget`, 8 range refusals), and 70 carry no source work (58 HASH-001, 12 axis-gate). J7's 10 `Budget` first causes are all at source closure (the upfront descriptor-unit gate; RF §3.6), before any evaluation, so its classification cannot change.
  - Revision 5 recounted these from the committed `j7_rf_census_expected.json`, which is unchanged since `fa019d9cef`. It holds 322 entries. The 66 entries carrying `method_only_comparison` are 62 `selected_method_only` and 4 `selected_published`. The 168 block-order entries are 83 not attempted, 78 declined with ordinary publication and 7 declined with a blocking refusal; all stop at Preparation. The 10 `budget` and 8 `checked_value_range` entries stop at `SourceClosure`.
- the two product-preview generation records, `fixtures/product_preview/demo_fixture_generation.json` and `preview_physics_fixture_generation.json`. Their `source_input_files` and `source_input_files_after` maps bind the hash of every generator input, including FK `exact_boundary.rs` and `exact_boundary/functionals.rs` and PP `lib.rs`, `source_recovery.rs` and `source_receipt/pressure_source_context_tests.rs`. So any change to those files refreshes both records, through their writer, in the same PR, as #1483 did;
- `tools/hosted-ci-routing.json`'s `source_trees` digests for every crate R-A edits (FK and PP). A change to any `.rs` file in a crate with runtime-read declarations must refresh that crate's digest in the same change; otherwise `tests/test_ci_numerical.py` fails and hosted selection falls back to every crate (`docs/CI_STRATEGY.md`). Separately, each edited `.rs` file that itself carries a runtime-read declaration has its own `piping_runtime_reads.reads[<file>].sha256`. `tools/ci/numerical_ci.py` rejects a stale per-reader hash before it checks the caller trees (about 199–212). The files concerned include `source_receipt/pressure_source_context_tests.rs`, which the U4b-law restatement edits, `source_receipt/o4_source_tests.rs` and `tests/pressure_source_join.rs` (MJ6's basis). So their entries are refreshed in the same change as well (both reviews of revision 5);

**Tests that need more than regeneration:**
- **`cached_source_context_preserves_current_public_serializations`** (`pressure_source_context_tests.rs` about 606) pins work digits (RF §5.5) by byte equality with the 24 committed raw controls. So it is re-pinned by regenerating those controls, not by editing constants in the test.
- **RF §5.5's five budget-cliff fallback tests** (`source_receipt/load_state_fallback_tests.rs`) are re-derived by RF §5.5's own rule: recompute the cliff, scale it within D1's caps, or restate it with a private limit and a positive control.
- **`u4b_source_work_law_reconciles_every_control_with_its_pre_u4b_counters`** (`source_receipt/pressure_source_context_tests.rs` about 635–735). It asserts `charged == old_charged + ΔF + ΔQ` against a hard-coded table of historical pre-U4b counters (`PRE_U4B_WORK`). R-A changes `charged` without a U4b delta. Its `publication == old_publication + ΔP` assertion (about 727–729) also moves wherever a work figure changes its digit count, through the 12 × diagnostic-length publication charge. So both are restated: either the law gains an independently derived R-A delta per control, or the table is rebased to the pre-R-A counters with the rebase stated.
- **`o4_source_work_deltas_follow_the_d5_law`** is expected to hold unchanged. It compares Euler–Bernoulli and Timoshenko twins, whose R-A savings are derived by the same law; it runs in the PR's checks.

**The work-only diff.** A check modelled on `o4_text_only_envelope_diff`, `work_only_envelope_diff`, requires every regenerated document of a case selected or declined today to differ from its pre-R-A bytes only in work counters, the text that embeds them, and the hashes binding those. No value token, interval, row, class, first cause, selection or `response_digest` may move. The J7 census also asserts unchanged classification, counts, first causes, selections and frozen-row verdicts.

**As built (the re-pins).** Each record was regenerated by its own writer on the implementation head, and `tests/test_ra_work_only_envelope_diff.py` binds every one of them from `9e03991d39`:
- the 24 raw source controls and the J2 corpus (all 486 entries; no `expected` verdict moves), each moving its case and invocation work, the printed method work and `publication_sha256`/`receipt_sha256`. MJ6's `CURRENT_SOURCE_BASIS` follows: physics n06 is one byte shorter in both modes, as its invocation work loses a digit;
- `o4_source_controls.json`, all five entries;
- J7: exactly the 66 directly selected entries move in `first_cause.work.charged` (62 method-only, 4 published), and only the 4 published entries also move `envelope_sha256` and `diagnostics_sha256`. Classification, counts, first causes, selections, `results_sha256`, `summary_sha256` and `public.identity` are unchanged;
- all 30 load/reference-state twins;
- not moved: `loadReferenceSourceLedger.cases.json` and `load_reference_source_mutations.json` hold mutation inputs, not current counters.

Across the 57 regenerated files, 2,032 counted-work leaves fell, 3,004 were unchanged and **none rose**. The U4b work law keeps its historical `PRE_U4B_WORK` table and gains a measured R-A term per control (the hinted against the plain plan, and the product against the long evaluation, on the receipt replay's own input); both its charged and its publication assertions reconcile exactly on all 24 controls. The term is measured with R-A's own evaluator, so it cannot catch a change in R-A's charges; the kernel's independent oracle does. RF §5.5's five budget cliffs are re-derived on their edges, each failing one unit below. `o4_source_work_deltas_follow_the_d5_law` holds unchanged. `o4_text_only_envelope_diff` reads R-A's regenerated files at `9e03991d39`, the bytes O4's regeneration left, so it stays bound to O4's change and the R-A check binds R-A's.

**D3.** The changed evaluator is priced source (RF registered the source method under D3; `retained-pricing-rules.md` §7.4). R-A therefore needs an actual-source D3 registration at the unchanged policy and the pricing tool's `generate --check`.
- B1's per-row hint vector, at most one index per descriptor, is a new priced owner. It changes the signatures that carry it: the functional evaluation entry (`evaluate_functionals`) and `replay_against`.
- B2 reads the existing `c.source.prescribed` list. If the implementation adds a per-DOF zero-prescription vector, that is a new priced owner, sized n bits or bytes at most 256.
- The registration measures every phase rather than assuming no growth.
- **Starting point (revision 5).** The registration starts from the profile registered on main `9e03991d39`: the W1 coverage repair's corner-1 registration (`retained-pricing-rules.md` §8.12). Its tree is priced at `aefefa5043`, with 1,562 coefficients, 319 atoms, 100 forms, 26 SourceUpper and no Estimate, and all 70 evaluations fit at the unchanged policy. R-A's registration is the next section of that file. It `compare`s against §8.12's tree and reports the coefficients and evaluations that move. If another registration lands first, R-A starts from that one instead.
- **Pins that move.** The pricing tool's full-file source pins include, among others:
  - `source_recovery.rs`, `source_receipt/source.rs` and FK `structural.rs` (`base/b7a_formation_model.py`);
  - PP `lib.rs`, read or pinned by several models, among them `base/b7a_domain_model.py`, `base/kd5_text_model.py` and `base/w1_reductions_model.py` `REDUCED_PINS`;
  - `source_receipt.rs` (`base/b7a_domain_model.py`, `lib/b7a_retired_carry.py`), which calls `replay_against` (about 259 and 375) and so is edited if B1's hints change that signature.

  The list is not exhaustive: the tool stops (exit 4) at any stale pin, and each pin moves only after its changed spans are read. Line-keyed loop and TEXT rules in an edited file, such as `exact_boundary/functionals.rs`'s entries in `loop_bounds.g4.json`, are carried by the tool's line map. Every remap is reported, and an unmapped key stops the run. FK `exact_boundary.rs` is read structurally by `base/rf_source_model.py`; if R-A edits it, that model's checks are re-read.
- **The native record and challenge literals.** Wherever R-A's pricing changes a phase's value, these are regenerated together with the profile: the law tests' `PINNED_RECORD` (`retained_memory_law_tests.rs` about 1760–1768: "regenerate the record with the profile, never one without the other") and the challenge binary's `W1_PHASE_BYTES` and `MAX_PHASE_BYTES` (`tests/retained_memory_challenge.rs` about 41–46). `generate --check` does not cover them, and their checks skip outside the pinned build identity. So they are checked in the registered build (Codex review of revision 5).
- **Code compiled out.** Census code under `census-probe` is compiled out of the registered build and blanked by W1R-FG (section 5), so it is not priced. B1 and B2 themselves are priced once they are compiled without the feature, at adoption.

## 5. The census (specified; executed later)

**Purpose:**
- establish R-A's gain on the product;
- confirm byte preservation on the cases selected today;
- judge every newly selected case;
- confirm today's outcome code;
- attribute the residual cost.

All of this comes before adoption is decided.

**The switch and the probe.**
- R-A runs behind a default-off runtime switch inside the existing opt-in `census-probe` feature. The kernel declares that feature and `product_physics` forwards it (revision 5; the W1 coverage repair landed it first, and its kernel declaration describes it as the shared census probe, its records and its test switches).
  - Reusing it adds no manifest line. So the existing guards apply unchanged:
    - no `[dev-dependencies]` entry enables it, and `census_probe_is_named_only_by_its_definition_and_forward` (PP `w1_source_census_tests.rs`; generalized on #1484 to each off feature) asserts that the kernel's declaration and PP's forward are its only manifest lines;
    - `census_probe_is_off_in_the_ordinary_test_build` asserts that it is off in the ordinary test build;
    - the pricing tool's W1R-FG manifest audit (`off_feature_audit`) stops on any other enabling line.
  - A new feature was considered and not chosen. It would need two new manifest lines, new W1R-FG manifest pins and `OFF_FEATURES` entries (`tools/retained_pricing/base/loopscan.py`), a new off-build constant and new guard calls, and none of these would buy any separation that matters. The W1 censuses are complete, and the switch reads as off unless a census turns it on.
  - RF's `exact-operation-count`, which the census reads, is reached through `core/product_physics/Cargo.toml` `[dev-dependencies]` and so compiles into every PP test build, as it does today.
  - Only the census commands pass `--features census-probe`. CI's cargo commands pass no features: `tools/ci/numerical_ci.py` names none, and VR's `ci_passes_no_cargo_features` (`validation/benchmarks/numerical_robustness/tests/feature_guard.rs`) checks `tools/release/check_release_readiness.py`. The stack witnesses run through that script without features.
  - So production paths, the committed pins and the stack witnesses run the long path until adoption.
- **The shared feature's other switches.** Under `census-probe`, W1's switches read their census defaults, not production's. On #1484, `final_case.rs` `corner1()` is `true` without the feature but reads `census_probe::switches().corner1` under it, and that switch defaults to `false` (`census_probe.rs` about 74–76). So a census build runs W1's pre-corner-1 search unless the switch is set. The source-route census entries call the ordinary exact run, which the reviewers read as not reaching W1. Even so, every R-A census run sets every W1 switch to its production value (corner 1 on, the others off) and records them with R-A's switch. A switch-off control also requires each committed source control's bytes, regenerated in the census build, to equal its committed bytes. That shows the census build's baseline is the product's.
  - The switches are thread-local, and `census_probe::with` restores them on drop. So they are set inside every census worker thread, including the large-stack threads the drivers spawn, not once on the test thread.
  - The census targets are `#[ignore]` and run with `--ignored`, as W1's are. So `census_probe_is_off_in_the_ordinary_test_build`, which has no `not(feature)` gate, is not run by a census command.
- **W1R-FG's constraints on the census code** (`retained-pricing-rules.md` §8.11):
  - In priced source such as `exact_boundary/functionals.rs`, every census item or statement carries exactly `#[cfg(feature = "census-probe")]` or `#[cfg(all(test, feature = "census-probe"))]`. An off-feature attribute in an expression position (a match arm, a field or a list element) stops the pricing run, so the switch is read in a gated statement, not in an expression.
  - A `#[cfg(not(feature = "census-probe"))]` arm is compiled in and stays priced.
  - Kernel-side probe code goes in the existing off-build `census_probe.rs` (re-exported as `retained_api::census_probe`) or in gated statements in `functionals.rs`. PP census modules are test files (`*_tests.rs`) included through a `#[cfg(feature = "census-probe")]` `#[path]` declaration. A new non-test file would not stop the run: `off_build_files` tracks only the reviewed names (`loopscan.py` about 265–275), and `callgraph_g5.py` prices any file without "test" in its name as live code. So no such file is added.
  - If census code merges, each of its files that reads a file (`fs::read*`, `File::open`) carries its own runtime-read declaration in `tools/hosted-ci-routing.json`. The numerical CI scans every `.rs` file in a crate, whatever its cfg (`numerical_ci.py` about 147–200), as `w1_source_census_tests.rs`'s declaration shows.
  - The census writers open files with `File::create`, not `OpenOptions`, which the numerical CI's read scanner refuses (#1483's census writer).
- **Before and at adoption.** The switch and probe live on the census branch. Unlike W1's census, they need not merge. If they do merge before adoption, they are compiled out of the registered build under W1R-FG, as W1's census was. At adoption the switch is removed and B1 and B2 run unconditionally in the product. A merged product-path switch would be a second charging regime keyed on a flag, which RF §5's ruling excludes.
- **The long evaluation after adoption.** The maintained gating test (3.2) still needs the long evaluation to compare against, in the ordinary test build, where `census-probe` is off.
  - B1's long path needs nothing new: a missing hint takes it.
  - For B2, the private evaluator `evaluate` (`functionals.rs` about 563) takes an internal shortcut flag. Both of its product callers pass it on: `evaluate_functionals` (about 666) and `RetainedFunctionalSet::replay_against` (about 1047). A check or mutant shows that the live attempt and the captured replay always pass the same setting.
  - The only way to pass it off is a test-only entry compiled under `#[cfg(any(test, feature = "exact-operation-count"))]`, the existing pattern for test-only kernel instrumentation that PP's dev-dependency reaches. The pricing tool already blanks that pattern (`loopscan.py` `TEST_ATTR`), and no product binary compiles it. Every PP target built with dev-dependencies does, including the fixture generators under `core/product_physics/examples/`. Those never call it.
  - This widens what `exact-operation-count` means, so its comment in FK's `Cargo.toml` and the header of `exact_boundary/exact_operation_count.rs` are updated to name the reference entry. No manifest guard covers this feature today, as before R-A.
  - The long path is then reachable at the kernel's evaluation entry, not end to end. So the gating comparison (3.2) runs per input on its response and plan, as `j7_rf_census.rs` (about 684) already reaches `replay_against`.
  - One implementation option carries B1's hints and B2's flag on `FunctionalPlan`, with `FunctionalPlan::new` setting the flag on and a test-only constructor setting it off. That avoids widening two signatures. `FunctionalPlan` is not serialized, and neither hints nor flag may enter `functional_plan_sha256`, `normalized_source_sha256` or any receipt field.
  - So the product has one regime, and the maintained tests can still compare against the long path.
- Under the same opt-in feature, together with the existing operation counter, the probe records:
  - charge and pair counts per labelled site, for each table site of 2.2, each row by index and kind, and each projection;
  - the attempt stage and phase: live, captured replay, binding, finalization or maxima.
- Test-side, in exact rationals, the census records the actual and canonical lengths of k, the block entries, Δ2–Δ4, the adjugate entries, the displacement and reaction numerators, and each row numerator.
- Every census result records the effective switch state.

**Inputs:**
- O4's sweeps `o4_census_source_work_single_members` and `o4_census_source_work_small_frames`, in both styles, shear models and allowances;
- matched-input ablations of the Z-hinge sweep: each of D, t, L, E, ν, k and m₁ switched to its dyadic form one at a time, to attribute the decimal cost;
- the J7 RF census;
- the 24 raw source controls, `o4_source_controls`, the PIN-OFF-345 controls and MJ6/MJ9;
- the O4 source twins and declared-problem selections (`validation/references/o4_retained_source/declared_problem/required_selection.json`);
- at least one multi-case invocation that puts a Z-hinge case before a case selected today, for RF §5.2 (6).

**Records**, per case, with the switch off and on:
- the outcome code, internally (selection, decline with its first cause, or the blocking finalization refusal) and at the product entry (published envelope, or the `SOURCE_BLOCKS_FINALIZATION_FAILED` refusal), and the case and invocation charges;
- every published byte: `value_bits`, interval, both bounds, basis, `response_digest`, both source hashes and `receipt_sha256`;
- the envelope, results and summary digests;
- the charged ≥ counted control and the oracle's verdict.

**Pre-registered rules.**
- **Preserved cases.** For a case whose outcome is the same off and on, any difference outside the work counters and the text and hashes that bind them stops R-A for diagnosis.
- **Newly selected cases** (off: decline or blocking refusal; on: selection) are judged two ways:
  - by the declared-problem oracle (`validation/references/o4_retained_source/declared_problem/`): every projection interval must contain the declared problem's exact value, and every stress check must recompute;
  - against the long path, at the method level and in every evaluation (revision 5, as prepared for the census). Revision 4 planned a switch-off publication under a test-only raised per-case limit. That cannot be formed: the receipt refuses any case budget above `PHYSICS_SOURCE_WORK_LIMIT` ("case budget exceeds selected receipt policy", `source_receipt.rs` about 777–790), and that check stays unchanged. No raised limit is needed either, because these cases' live attempts fit under 8M today (section 1). The comparison therefore has three parts:
    - **Method.** The direct method entry that J7 uses (`solve_ordinary`, at the unchanged limit) runs with the switch off and on. It covers the live attempt, without the replay or finalization. Every functional and DOF projection (value bits, interval, both bounds, basis), the `response_digest` and both source hashes must be equal. Only work may differ.
    - **Every evaluation.** The census's shadow check re-evaluates each live and replay evaluation that took B1 or B2 on the long path, on a private, unlimited ledger that no case ledger reads, and compares every Ratio bit for bit.
    - **Publication.** Every published value token of the newly selected case equals the switch-on method's projection, which the method comparison ties to the long path. Its receipt and envelope hashes are recomputed from the published bytes by the Rust reader and the Python verifier, which must return the selected outcome (AC-002's readers).

    Together these show that B1 and B2 reproduce the long path's values and bytes. A case whose switch-off method run itself exceeds the limit is reported, and its comparison rests on the shadow check and the publication check.
  
  Any failure stops R-A for diagnosis.
- **Lost selections.** A case selected off and not on, including through RF §5.2 (6), stops R-A for diagnosis.
- **The gain.** The census reports, for both shear models, how many of today's Z-hinge failures select with R-A, and the residual charge distribution against 8M. The target is all of them.
  - If some remain, the report names section 3.3's next candidate and its ruling.
  - Adoption of R-A is decided by Agent 0 and the owner on that report, together with the AC-002 ruling. It is not automatic.
- **Attribution.** The per-site table and the ablations replace section 2's estimates. A Timoshenko-specific site cost above 5 % of E_live, or a near-φ = 2 effect, is reported.

**What it needs:**
- A debug build of the `frame_kernel` and `product_physics` tests with the opt-in feature, under the host's heavy-run slot and memory rules.
- Private drivers only.
- A pilot of ten documents per sweep to measure time and memory first.
- It writes nothing under `validation/references`. Committed fixtures are regenerated only in R-A's implementation PR, after the census.

### 5.1 Census results

This section reports the census as repaired after its dual review (section 7).

**Basis.**
- Census branch `claude/piping-ra-census-20261021` at `4d956a6b8a`, on this design and main `9e03991d39`. Not for merging.
- **Code compiled only under `census-probe`:**
  - FK: R-A's switches and records in `census_probe.rs` (`pub mod ra`), site labels in `exact_boundary.rs`, and B1 and B2 in `functionals.rs` (`mod ra_census` and gated statements in `evaluate`);
  - PP: `ra_census_tests.rs`, `source_recovery/ra_method_census_tests.rs` and `source_recovery/ra_j7_census_tests.rs`.
- **Environment defaults, census builds only.** `CENSUS_PROBE_W1_PRODUCTION=1` starts every thread with W1's production switches. `RA_CENSUS_DEFAULT=on` starts every thread with R-A on, and a shadow mismatch then panics. These let subprocess runs take part.
- No limit, gate, check, oracle, committed fixture or reviewed input changed.
- **Builds.**
  - One PP lib test binary, at opt-level 2 with debug assertions and overflow checks (hosted CI's product_physics profile), `--features census-probe`.
  - The headless runner, built with `--features open_pipe_stress_product_physics/census-probe`.
  - Work counts do not depend on the optimization level.
- **Runs.** All went through `t3_slot.sh`, from the build at START `2026-10-21T04:00:18Z` to the last END at `2026-10-21T04:24:02Z`, every `rc=0`.
- **Pilot.** Ten documents took 1.3 s at 31 MB maximum resident set (`t3_exclusive.sh`). Raw output is in scratch, not committed.
- **The allowance as implemented.** Only these may differ between off and on:
  - `charged` under a `work` or `invocation_work` object, and `invocation_work.publication_charged`;
  - `publication_sha256` and `receipt_sha256`;
  - the message of a source-block diagnostic (SELECTED, UNAVAILABLE, FINALIZATION_FAILED) whose text differs only in its printed `charged`, `rejected` or `publication_charged` figures.

  Every allowed path is recorded. Revision 4's allowance, which admitted every hash and every digit in those messages, was too broad (census review). The rerun's allowed paths are exactly those four kinds.

**Every Z-hinge decimal case fits the unchanged 8,000,000 limit.** On O4's single-member sweep (sparse mode):

| Layout, shear model, inputs | R-A off: not selected | R-A on: not selected | Case charge on: max (mean) | Live pass, Preparation to Projection, off → on |
|---|---|---|---|---|
| Z-hinge, Euler–Bernoulli, decimal | 10 / 60 | 0 / 60 | 6.920M (6.327M) | 3.303M → 2.696M (−18.4 %) |
| Z-hinge, Timoshenko, decimal | 22 / 60 | 0 / 60 | 7.241M (6.557M) | 3.423M → 2.811M (−17.9 %) |
| Z-hinge, Euler–Bernoulli, dyadic | 0 / 60 | 0 / 60 | 5.692M | 2.652M → 2.198M (−17.1 %) |
| Z-hinge, Timoshenko, dyadic | 0 / 60 | 0 / 60 | 6.101M | 2.719M → 2.255M (−17.1 %) |
| (3,4,0), both models and styles | 0 / 240 | 0 / 240 | 6.879M | whole method charge −8.2 % to −11.0 % |

- The whole direct-method charge, which also includes SourceClosure, Plan and Retention (about 0.125M), falls by 17.7 % and 17.2 % on the decimal Z-hinge rows.
- R-A off reproduces O4's figures exactly.
- Every Z-hinge decimal case that does not select today selects with R-A. The largest case charge is 7.241M, 0.759M (9.5 %) under the limit.
- The small-frame sweep (480 documents) does not reach Evaluation, so nothing moves there.

**Today's outcome for the 32 blocked cases (section 1).**
- They take the pre-0.4 path and push the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` with `Exact(Budget)`, at charges from 7,993,763 to 8,000,000.
  - 26 fail in "captured source replay".
  - 6 fail in "composite current source".
- At the compatibility product entry, `run_linear_static_preview_value_with_mode`, each publishes the blocked envelope (identity `pressure-2`) and no `Err`.
- That the blocked envelope has no result rows comes from the code (section 1). The census records the class and identity.

**The pre-registered rules on the single-member sweep, ablations and two-case invocations: all met.**
- **Preserved cases.** No difference outside the allowance across 448 sweep, 909 ablation and 84 two-case documents, internal and public.
- **Newly selected cases:** 32 sweep, 171 ablation and 36 two-case documents. With the 16 newly selected many-case invocations below, that is 243 distinct requests.
  - *Method.* The direct method's response digest, and a digest of every retained Ratio and projection, are equal off and on for every document.
  - *Every evaluation.* 8,880 shadow evaluations were compared bit for bit with the long path, across the live and replay runs, the public-entry run and the method: 0 mismatches.
  - *Publication.* 108,000 published projections were checked against the switch-on method, each load case against its own direct method, on value bits, interval and both error bounds. All are equal.
    - The Rust reader returns the selected outcome on every newly selected envelope.
    - The Python verifier passes all 243 distinct newly selected envelopes.
  - *Not compared directly.* The source hashes (`normalized_source_sha256`, `functional_plan_sha256`) are not compared with a long-path run, because no long-path receipt exists for these cases. B1 and B2 run after the descriptors are built, so they cannot affect them. The oracle's record-hash and plan-binding checks tie the published hashes to the dumped payload.
  - *The declared-problem oracle.* It judges 207 of these requests (the sweep, two-case and single-input ablation cases), with its self-test passing. All case verdicts are PASS.
- **Lost selections:** none.
- **The maintained control** (charged ≥ counted): no `Work` call was charged less than it executed, on either side of any run.
  - The control observes `Work::mul` and `Work::add` only.
  - B1's recognition, negation and copy, and B2's window, are charged by the census implementation before they inspect, but the control does not check those charges. The implementation's oracle extension does (section 3.2).

**The declared-problem oracle's required selections, run unchanged** (census review). The `j2`, `rf`, `judged` and `phi2` batches ran with R-A off and on. The twins, product entries, oracle and `required_selection.json` are those of `declared_problem/run.sh`; only the census build and the environment switches differ.
- All 578 required triples select and pass, off and on.
- Every preserved run differs only within the allowance.
- In the `phi2` set:
  - 24 runs are newly selected, and the oracle passes them (360 case verdicts on, against the 336 required);
  - **24 runs stay blocked with R-A on.** These are the Timoshenko variants of the `phi2_zhinge` documents PEXACT-A r1e-08, PEXACT-B r1e-08 and PEXACT-B r1e-12.

**The PEXACT documents.** They are the oracle's invented, exactly φ = 2 sensitive members:
- the Z-hinge supports and load (PEXACT-A and -B) or the RF (3,4,0) supports (-C and -D);
- dyadic inputs, for example D = 0.3125, ν = 0.125, k = 2, m₁ = 0.125;
- thick-walled sections;
- long skew directions, (39,9,6) and (353,23,8) for A and B.

Results with R-A on:
- **Euler–Bernoulli:** every variant selects, off and on.
- **Timoshenko, PEXACT-A r1e-12 and PEXACT-D (both ratios):** newly selected.
- **Timoshenko, PEXACT-A r1e-08 and both PEXACT-B:** still over the limit. Their live passes, Preparation to Projection, are 3.61M and 3.95–3.97M with R-A (whole method charges 3.74M and 4.08–4.10M), and the captured replay still runs out at 8M.

They are not Z-hinge decimal cases. They are cases R-A leaves over the limit (section 6).

**Invocations near the 64M invocation ledger** (RF §5.2 (6); census review). These are the census's 36 invocations: twelve each of 8, 9 and 10 load cases, each on one decimal Z-hinge structure. Each later case's moment is scaled by 1 + i/10, as an authored decimal.
- **No selection is lost.** The unchanged declared-problem oracle passes all 16 newly selected many-case requests, 136 case verdicts (run by the Codex census reviewer, round 2).
  - 8 cases: 4 invocations select off and on, and 8 are blocked today and newly select.
  - 9 cases: 8 newly select and 4 stay blocked.
  - 10 cases: all 12 stay blocked.
- **The 16 invocations that stay blocked off and on differ beyond the allowance.**
  - *With R-A off,* 9 block when a case's own captured replay exhausts its 8M: case 1 in 6 of them, case 2 in 2, case 4 in 1. One blocks at a case's composite recipe reservation. Six block once the invocation ledger is nearly spent, three at a replay limited to what remains of it and three at the invocation publication reservation.
  - *With R-A on,* every block is bound by the invocation ledger. Eight fail at the composite publication reservation, charged 60.57M–63.85M, and three at the invocation publication reservation. Four fail at a replay limited to what remains, and one as described below.
  - So the blocking cause changes: mostly to a later case or to the invocation. In two 10-case invocations it moves the other way, from the invocation publication reservation to case 10's ledger-limited replay. The blocked envelope's per-case numerical-quality entries change with it.
  - At the product entry, the outcome moves between section 1's two fail-closed forms. Ten move from the blocked envelope (`pressure-2`) to the run-level `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`, two move the reverse way, and four keep their form. So, with R-A on, 5 end as blocked envelopes and 11 as `Err`. Neither form publishes result rows.
  - When the entry returns `Err`, the census does not keep that run's public-entry shadow results; the internal runs' shadow checks cover those evaluations.
- **One of them carries a misleading diagnostic,** an existing path that R-A only makes the ledger reach. In the 10-case invocation `zhinge122 (0.168, 0.007) long r=1e-12 eb decimal zero`, with R-A on:
  - all ten cases select;
  - two `SOURCE_BLOCK_RECOVERY_DERIVED_UNAVAILABLE` entries then report the composite recipe reservation refused at `Exact(Budget)`;
  - the blocking `FINALIZATION_FAILED` reads "source functional row binding is not unique: spring:N0/support_reaction_component_v2/node".

  From the code: when `composite_support_norms` fails, the product pushes the DERIVED_UNAVAILABLE diagnostic without appending that support's signed result rows (`lib.rs` about 5543–5557). `source_row_bindings` then finds no matching row and reports it as "not unique", because its test is `matched.len() != 1` (about 6693). The outcome is fail-closed, but the stated cause hides the budget refusal. The path is in the case solver that the pre-0.4 and 0.4.0 contracts share (`solve_prepared_load_case`), and any selected case whose support-norm reservation runs out reaches it, with R-A off as well.
  - On pre-0.4 it blocks with the misleading cause, as observed here.
  - On 0.4.0, the same message would become the withheld join's stated cause when the invocation republishes ordinarily (`record_load_state_join_failure`). The census did not exercise that.
  - It is outside R-A's scope and left unchanged (section 6).
- This is the cross-attempt effect that section 3.1 and RF §5.2 (6) foresee: fail-closed, with less work per case. Under section 5's rule it is a difference outside the allowance, so it is reported here for the owner (section 6). It is not repaired.
- The two-case invocations (largest invocation charge 15.0M) do not approach the invocation limit.

**What moves (section 4), as predicted.**
- **J7.**
  - With R-A off, all 322 entries equal the committed `j7_rf_census_expected.json` byte for byte.
  - With R-A on, the 62 method-only entries move only `first_cause/work/charged`, and the 4 published entries also move their envelope and diagnostics digests.
  - Nothing else moves. Classification, counts, first causes, selections, frozen-row verdicts and the results and summary digests are unchanged.
- **The 35 committed source controls.**
  - With R-A off, each equals its committed bytes, MJ6's `CURRENT_SOURCE_BASIS` included.
  - With R-A on, each differs only within the allowance: the SELECTED diagnostic's work figure, the case `work.charged`, `invocation_work.charged`, `publication_sha256` and `receipt_sha256`.
  - `publication_charged` does not move, and MJ9's accounting holds on both sides.
  - The ten load/reference-state controls cannot run the method-level publication check, because J7's direct method builds no load state. They are covered by the byte comparison and the shadow check.

**Attribution** (replacing section 2's estimates).
- **Shares of the live pass**, Preparation through Projection, for Z-hinge decimal Euler–Bernoulli inputs with R-A off:

  | Site | Share |
  |---|---|
  | section rows | 22.9 % |
  | Solve reactions | 19.1 % |
  | support rows | 11.8 % |
  | leading minors | 9.0 % |
  | projections | 8.6 % |
  | end rows | 8.5 % |
  | adjugate | 7.8 % |
  | all coefficient recipes | 5.0 % |
  | `ratio_add` | 3.0 % |
  | congruence | 2.5 % |

  Section 2.2's model is within 1.7 points on every site. Timoshenko's shares match within 0.4 points.
- **B1 and B2 separately** (whole direct-method charge, decimal Z-hinge):
  - B1 alone saves 14.8 % (Euler–Bernoulli) and 14.4 % (Timoshenko), and by itself selects every case, at 7.126M and 7.450M.
  - B2 alone saves 3.4 % and 3.3 %, and leaves 2 and 12 cases blocked. In 4 of those 14 cases the blocking cause moves from "captured source replay" to "composite current source": a later stage refuses once less work is spent, as section 3.1 states.
  - On every Z-hinge run, all 22 negated rows per member take B1 in every evaluation, and the 8 rows with nonzero-fraction shear terms fall back.
- **The matched-input ablations**, isolated (census review): each input made dyadic alone, with every other authored input, k, m₁ and the allowances included, held at the decimal document's value. Over 120 decimal Z-hinge documents:
  - m₁ alone saves 18.3 % of the pooled whole-method charge, 94 % of the decimal–dyadic gap. That is 101 % of the gap for Euler–Bernoulli and 88 % for Timoshenko.
  - Each of D, t, L, E, ν and k alone moves the charge by under 2 %.
  - The gap is downstream of the displacement numerators (adj·m₁). Preparation is about equal across the two styles.
- **Timoshenko near φ = 2.**
  - No Timoshenko-specific site exceeds 5 % of the live pass.
  - At the near-φ = 2 length, Timoshenko's whole-method charge is 6.9 % above Euler–Bernoulli's, against under 2 % at the other lengths.
  - 14 of today's 22 Timoshenko blocks arise there.

**What the census does not establish.**
- It covers O4's sweeps, the isolated ablations, two-case and 8–10-case invocations, the required-selection batches (both modes), J7 and the committed controls. It is not a proof that every input fits.
- **Charges.** The census build derives B1's candidate from the row key, uncharged, instead of receiving product_physics' hint. So the adopted implementation's charges can differ slightly; its gating test, oracle extension and D3 re-establish them.
- **Entries.** The census exercised the compatibility entry and the headless runner. It did not exercise the registered build's permitted Direct route (`permitted_run` on its reserved-stack thread): a census build is not the registered identity, so that route takes the ordinary route there.
- **What remains.** Implementation, its gating and mutant tests, the actual-source D3 and the independent numerical review remain (section 7).
- **Section 3.3.** The second-stage reductions are not needed for the Z-hinge decimal cases. Whether the PEXACT Timoshenko cases and the over-limit many-case invocations call for them under b7-plan item 7 is reserved (section 6).

## 6. Reserved for the owner

**Owner decisions, 2026-10-20** ("For the five decisions you put to me I approve your recommended option in each case"):
- **AC-002.** The amended wording below is approved. It is applied to the ScopeOfWork in R-A's implementation PR, and only if section 5's census shows every Z-hinge decimal case fitting the unchanged limit and the independent numerical review passes.
- **The pre-0.4 blocked envelope.** Deferred until R-A's census. Today's outcome stands until then.
- **Second-stage candidates.** These are designed further only if R-A leaves cases over the limit.
- **Placement.** R-A is b7-plan's epoch 7, after the W1 coverage repair. Its census comes first.

**After the census (section 5.1).** Evidence for the two items still open:
- **AC-002.** The census condition of the approval above is met: every Z-hinge decimal case fits the unchanged limit. The census measured exactly the moves the approved wording covers, and nothing else:
  - J7's charged work on its 66 directly selected entries;
  - the envelope and diagnostics digests of the 4 published entries.

  The other condition, the independent numerical review of the implementation, remains.
- **The pre-0.4 blocked envelope.** As repaired (section 5.1), the census does **not** show R-A removing every observed occurrence.
  - R-A removes it on the single-member sweep, the ablations, the two-case invocations, twelve of the PEXACT φ = 2 Timoshenko documents (24 runs), 8 of the 8-case invocations and 8 of the 9-case ones.
  - It remains for the PEXACT-A r1e-08 and PEXACT-B Timoshenko documents (24 runs), and for 16 invocations: 4 of 9 cases and 12 of 10.
  - In those invocations the product entry's fail-closed form changes: ten move from the blocked envelope to the run-level `SOURCE_BLOCKS_FINALIZATION_FAILED`, two move the reverse way, and four keep their form. With R-A on, 5 end as blocked envelopes and 11 as `Err`.
  - Extending the replay screen (`reserve_captured_replay`, which passes only when the live charge is at most the limit less itself) to the pre-0.4 path would refuse the two remaining PEXACT-B Timoshenko families (about 4.08M and 4.10M, 16 runs). It would pass PEXACT-A r1e-08 (about 3.74M) and every Z-hinge decimal case. The census evidence therefore narrows, but does not reverse, the statement under "The options as presented" that the screen would not catch these cases.
  - So R-A reduces how often the blocked outcome occurs, but it also changes its diagnostics, and on invocations over the 64M ledger its form.
  - One such invocation reaches an existing misleading diagnostic. "Source functional row binding is not unique" is reported where a support-norm budget refusal left the support's rows unappended (section 5.1). That path is in the shared case solver and exists with R-A off too. On 0.4.0 it would become the ordinary republication's stated cause, which the census did not exercise. So it bears on today's paths, not on R-A.

**Owner decisions, 2026-10-21** ("A accept your recommendations for the entire set."), after section 5.1:
- **AC-002.** The census condition was met; the amended wording was applied in #1497 after the implementation's independent numerical review.
- **R-A's charged overhead** (section 3.1, "The overhead") is accepted as disclosed: a case within the margin of the 8,000,000 or 64,000,000 limit could refuse with `Budget` and keep its ordinary result. No committed record's counted work rose.
- **The pre-0.4 blocked envelope.** R-A leaves the path, its pinned test and RF §2.9 unchanged. One failure behaviour for both routes is designed as its own unit after the replay ruling (`b7-plan.md`, "After B7b").
- **Second-stage candidates** (3.3) are not designed further: b7-plan item 7's "cases" reads as the Z-hinge decimal finding. The PEXACT φ = 2 documents and the over-limit many-case invocations keep their fail-closed outcome. (Later: the pre-0.4 unification unit, [pre04-failure-unification.md](pre04-failure-unification.md), makes that outcome an ordinary republication; PEXACT-B is declined at the replay screen.)

The options as presented:

- **AC-002.** Its owner text of 2026-10-19 reads: "the RF census classification, counts, first causes, selections, charged work and frozen-row verdicts are unchanged; its envelope, results and summary digests change only as a stated product-text, identity or published-row change requires".
  - R-A changes the census's charged work on its 66 directly selected entries (62 method-only), and, on the 4 publicly selected entries, the envelope and diagnostic digests that bind that work. Its results and summary digests carry no work (`j7_rf_census.rs` about 333–340), so they do not move. Classification, counts, first causes, selections and frozen-row verdicts stay unchanged.
  - The owner can rule either that this re-pin falls within AC-002, or that AC-002 reads, for example: "… first causes, selections and frozen-row verdicts are unchanged; its charged work changes only as a reviewed work-reduction change requires, with every charge still at least the executed work; its envelope and diagnostics digests change only as a stated product-text, identity, published-row or work change requires, and its results and summary digests only as a stated product-text, identity or published-row change requires".
  - Without that ruling R-A cannot be integrated, and these cases keep today's outcome, a blocked envelope with no result rows (section 1).
- **The pre-0.4 blocked envelope** (option, no recommendation; since taken up by [pre04-failure-unification.md](pre04-failure-unification.md)).
  - Today a pre-0.4 case that selects but cannot finalize publishes a blocked envelope with no result rows and the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`. An invocation-level receipt failure refuses the run with `SOURCE_BLOCKS_FINALIZATION_FAILED` (section 1).
  - Extending the replay screen (`reserve_captured_replay`) to the pre-0.4 path would not catch these cases. The screen passes when the live charge is at most the limit less itself, about 4M, and these live attempts cost about 3.7–4.0M [E]; they fail later.
  - The option is to extend the 0.4 path's ordinary republication on finalization failure (SF-1R) to the pre-0.4 path, with or without the screen. A blocked envelope would become an ordinary publication with its failure recorded.
  - It gains no verified coverage. It changes recorded behaviour: RF §2.9 and DEL-04-07 `pressure-contract.md` ("the pre-0.4 path retains its named refusal"), with the pinned test `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback`, whose comment assigns the outcome to T3 as "a separate open finding".
  - Whether R-A alone, R-A plus this, or neither is wanted is the owner's choice. R-A reduces how often the blocked outcome occurs; this changes what it is.
- **Second-stage candidates** (3.3), each with its own ruling. F2 also needs HELP_HUMAN's ruling on RF §5's charging scope. No recommendation is made now.
- **The work limit.** Unchanged and not designed (REQ-003 (e), AX-005). Raising it remains the owner's choice, with D3 and independent review.
- **Placement.** R-A is not in b7-plan's epoch order. Agent 0 and the owner place it; section 0 states its sequencing constraints.

## 7. Verification and review

- **This design:** the dual review, an independent Claude reviewer and an independent Codex reviewer on the same revision. Rounds 1–3 reviewed revisions 1–3, and revisions 2–4 answer them. Revision 5's as-built re-check was reviewed on `9f76450127`, and its repairs are folded into revision 5.
- **The implementation (R-A):**
  - the gating test, the mutants of 3.2 and the census of section 5;
  - VER-002's checks and the J7 census;
  - `work_only_envelope_diff`;
  - the actual-source D3 run;
  - the independent numerical review that Root `AGENTS.md` requires for numerical meaning.
