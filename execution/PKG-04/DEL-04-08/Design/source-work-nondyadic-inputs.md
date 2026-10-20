# Source route: the exact-arithmetic cost of non-dyadic inputs

This design examines O4's coverage finding (b) (PR #1468, "Coverage findings"; [o4-source-extension.md](o4-source-extension.md) §5, "Census"). On 3D skew single members ((1,2,2), the Z-hinge layout) with ordinary decimal inputs, retained-source recovery (`pressure-source-1`) does not complete within the unchanged 8,000,000 per-case limit (`PHYSICS_SOURCE_WORK_LIMIT`, PP `lib.rs:939`). This happens in 10 of 60 Euler–Bernoulli runs (17 %) and 22 of 60 Timoshenko runs (37 %). Planar members and dyadic twins always complete. The owner decided on 2026-10-20 ("All as recommended") that a remedy be designed now. Its goal is to reduce the exact-arithmetic cost so that these cases fit the unchanged limit.

**Status: design only.** No product file, fixture, reviewed input or ScopeOfWork changes here. The basis is main `fa019d9cef` (O4 merged as #1468). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

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

## Summary

- **Today's outcome** (section 1, derived from the code). The Z-hinge census documents carry no load/reference-state fields, so they take the pre-0.4 path, which has no replay screen (PP `lib.rs` about 4990–5011).
  - The live attempt costs about E_live + 0.14M, which selects well within 8M. The limit is crossed later, in the captured replay and receipt finalization.
  - On that path a finalization failure pushes the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` (`lib.rs` about 6614–6625). The caller then returns the blocked envelope, which carries no result rows (`lib.rs` 2923–2924, `blocked_envelope` 13963).
  - **What the user gets.** The blocked envelope carries the ordinary pressure identity, so the product entries' `source_finalization_failed` test does not fire (`lib.rs` 1082–1088, 13982, 2508–2511). The entries publish that blocked envelope: no result rows, with the blocking diagnostic. A run-level `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` (`lib.rs` 2501–2503, 3229–3231) arises only when the invocation-level receipt fails after a case selected (`finalize_composite`, about 3118–3135).
  - So today these cases publish **no result rows**, not ordinary results. That is fail-closed, but it is not REQ-003 (c)'s ordinary route with a receipt failure record. RF §2.9 and DEL-04-07 `pressure-contract.md` record it ("the pre-0.4 path retains its named refusal"). O4's census raw output, which is not committed, would confirm it; section 5 records both the internal and the public outcome.
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
- **Expected gain.** About −12 % to −20 % of E_live; the reviewers' independent models differ in B1's share. That is about −11 % to −18 % of the case charge.
  - On the models' estimates, every Z-hinge decimal run's case charge falls below 8M: the models' maxima after R-A are about 7.1M to 7.5M.
  - The models under-predict the product's failure rate, so the product's gain, and whether every case then fits, is established by the census of section 5 before adoption is decided.
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

**Basis.** Main `fa019d9cef`. Paths are relative to `projects/chirality-piping/`; PP is `core/product_physics/src/`, FK `core/solver/frame_kernel/src/structural/`.
- **The census figures** (17 %, 37 %, 21/50 and 20/38 above 7.5M) are O4's, from `o4_m3_judging_tests.rs` `o4_census_source_work_single_members` (lines 962–1003), as reported in PR #1468. Their raw output (`O4_CENSUS_OUT`) is not committed.
- **This design's own figures** come from two standard-library Python cost models in `/tmp` scratch, not committed. One was written for the design, the other independently by a reviewer. Both re-implement `Expansion`, `Work` and the metered rule. The design's model reproduces RF's measured (1,2,2) congruence charge: 83,882 [E] against 84,268 [M] (rf-source-method-extension.md §3.5).
- **Tags** mark each figure: **[D]** derived from the code; **[E]** estimated with a model; **[M]** measured in a committed document.

**Assumptions from concurrent work, and sequencing.**
- **Stack-headroom repair (b7-plan item 5a).** It shrinks debug frames in `freeze_case`, `source_residual::recover` and `member` on the retained W1 path, then re-registers D3 and puts the six stack witnesses into a maintained check.
  - R-A's code is the source route's exact evaluator (FK `exact_boundary/functionals.rs`), which is not on that chain.
  - Assumption: 5a's frame repair does not change `exact_boundary`, `functionals` or `source_recovery`. If it does, R-A rebases on it.
  - Sequencing: R-A's implementation follows 5a, because both register under D3 in the same generated profile block. R-A re-runs the six witnesses at R/64 in the registered profile, as AC-004 requires after 5a.
  - The census switch (5) is an opt-in feature that no `[dev-dependencies]` entry enables, so the witness build compiles none of it.
- **W1 coverage repair** (w1-coverage-repair.md, epoch 6). It changes the W1 kernel's loaded-maximum search and the receipt's scales; R-A changes the source route's evaluator. They share no numerical law or source file.
  - Assumption: the W1 repair moves no source-route work counter. Its §1.4 lists none.
  - Both censuses add opt-in test features to the kernel's and product_physics' `Cargo.toml`, and both register under D3. Whichever lands second rebases those files and re-registers on the other's profile. Neither waits for the other's design.
- **O4 as built.** R-A keeps O4's work law (o4-source-extension.md §3.5) and the D5/MJ9 accounting. The direct change is in Evaluation-stage work. Secondary changes follow from text lengths (section 4).

## 1. The ledger, the limit, and today's outcome

- **The rule.** The per-case ledger is the metered `Work` of RF §5 (C₂ = 96 per term pair, plus 6·len + 1 before each grow-expansion add), with the finalization and publication reservations of RF §2.9 and O4 §3.5. `Work::charge` refuses before any work when the next total would pass the limit (RF §5.2 (1)).
- **Where these cases cross it.** The live attempt fits; its estimate is about E_live + 0.14M [E]. So these cases cross the limit in the captured replay and receipt finalization, which share the case ledger (RF §2.9). They take the pre-0.4 path: `case_state::is_load_state` is false for documents without reference configurations, expansion laws or analysis states, which `census_document` authors none of.
- **The resulting outcome.**
  - *Inside the run* (`lib.rs` about 6614–6625): the selected source response is lost, and the case pushes the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`, whose text carries the `Budget` error and the finalization work. The caller then returns the blocked envelope: no result rows, empty physical evidence and incomplete status (`lib.rs` 2923–2924; `blocked_envelope`, 13963).
  - *At the product entries:* the blocked envelope carries the ordinary pressure identity (`mechanics_producer_for_model`, `lib.rs` 1082–1088, used at 13982), so `source_finalization_failed` (2508–2511) does not fire. The entries therefore publish the blocked envelope.
    - The run-level `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` (2501–2503, 3229–3231) arises only when the invocation-level `finalize_composite` fails after the producer has been set to pressure-source (about 3118–3135).
    - The pinned `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` asserts that `Err` on its own input, and calls the inherited outcome "Characterization only, not an endorsement … a separate open finding assigned to T3".
    - The Z-hinge overrun arises on the case's own ledger, in the per-case captured replay and finalization, so it is expected to take the blocked-envelope path.
  - This is the recorded pre-0.4 behaviour (RF §2.9; DEL-04-07 `pressure-contract.md`). It is fail-closed, but the user gets no result rows for the run, not the ordinary result of REQ-003 (c). That differs from a failure later, at the invocation's publication reservation, and from the 0.4 load/reference-state path. There, a selected join that cannot finalize is republished on the ordinary route (SF-1R). The 0.4 replay screen alone does not guarantee finalization (`lib.rs` 4996–4999).
- **Confirmation.** O4's census function calls `run_linear_static_preview_observed` directly and records `selected`, the case charge and the `SOURCE_BLOCK_RECOVERY` diagnostics (`o4_m3_judging_tests.rs` 945–959). So it sees the same blocked envelope the entries publish; it would not see an invocation-level `Err`. Its raw output would confirm the code; section 5 records both.

RF §5.2 (3) states the rule's soundness: executed `Work` arithmetic is bounded by the charged total, which never exceeds the limit. R-A keeps that rule unchanged. It removes executed arithmetic, so it removes the charges for that arithmetic. Every new step it adds is charged before it runs.

## 2. Cost anatomy of one Z-hinge attempt

### 2.1 The problem [D]

- **The layout** (`census_document` "zhinge122"): one (1,2,2) member; N0 rigid in UX, UY, UZ, RX and RY, with a soft RZ spring; N1 rigid in UZ, RX and RY; one moment about z at N1.
- **DOFs.** n = 12. Four are free: N0.RZ, N1.UX, N1.UY and N1.RZ. They form one block of order 4. The other 8 DOFs are prescribed at exactly 0.0.
- **Rows.** 73 functional rows: 12 end, 30 section, 12 nodal, 1 spring and 18 support (`source_recovery.rs` about 1897–1925). There are 24 DOF projections.
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
- **What the dyadic twins change [D].** `census_document` (`o4_m3_judging_tests.rs` 895–907) rounds D and t to 6 significant bits, L to 8, k and m₁ to powers of two, and sets E = 1.5·2^37 and ν = 0.3125. The mill tolerance is 0.125·t. The corrosion allowance is 0.25·t for t < 0.006, and otherwise the decimal 0.0015 in both styles (line 909).
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
  3. no coefficient of s is a single term ±1.0.
  
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
- **The window.** Atoms are already validated finite (`functionals.rs` `products_charge`, about 434). So the recipe's remaining refusals are `exact_radix` bit loss on a lo or hi part, and two-sum overflow in `Expansion::add` (FK `structural.rs` about 790–880).
  - **The predicate.** Over all terms of the row on that DOF, every atom is a normal binary64 with binary exponent e satisfying |e| ≤ 150. Every product has at most 5 atoms. The DOF has at most 64 products in the row.
  - **Why no refusal is possible inside it.**
    - Write each atom as an integer times 2^(e − 52). Every partial product of j ≤ 5 atoms, and every term of every expansion formed from it by two-product and two-sum, is then an integer multiple of 2^(Σe − 52j) ≥ 2^(−150·5 − 260) = 2^-1010. That is above the subnormal grid 2^-1074, so no `exact_radix` scaling loses a bit.
    - Every such magnitude is below 2^(Σ(e + 1)) ≤ 2^755, and a sum of at most 64 of them is below 2^761. So no two-sum overflows. The expansion-length cap can only raise `Budget`.
    - Today's Z-hinge atoms (stiffness entries, transform entries, signs, fractions and lengths) lie far inside the band.
  - **The charge.** One unit per atom inspected plus one per product, charged before the inspection.
  - **Outside the window** the recipe runs, as today.
  - **Tests.** `b2_window_keeps_every_refusal` places inputs immediately inside and outside each bound (|e| = 150 and 151, 5 and 6 atoms, 64 and 65 products). It also covers the reviewer's rescued-underflow product [k, sign, 0.25, 4] with k = 0x1.1abe4b73fefb7p-1021, which is outside the window and must refuse as today.
- **Consequence.** Inside the window the recipe could not have refused, so no `Range` outcome can change. A `Budget` refusal can only disappear under less work (RF §5.2 (5)). If a later stage then refuses, that cause is exposed instead.
- **Cross-attempt effect.** In a multi-case invocation, a case that newly selects charges the shared invocation ledger. A later case then has less room and could lose its selection, or reach the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` at its publication reservation. RF §5.2 (6) states this cross-attempt exception; it fails closed. The census includes multi-case invocations (5).
- **Prescribed values that are not zero** keep today's path.

**The replay.** `replay_against` calls the same `evaluate` on freshly prepared descriptors, so the replay recomputes every row from the current invocation by the same rule. Nothing retained from the live attempt is reused. (Reusing the live attempt's cofactors in the replay was considered and rejected: it would replace the replay's recomputation with retained state.)

**What does not change:**
- every value, interval, bound, basis and published byte of a case selected today (by the bit-identity argument above, which the gating test checks);
- the descriptors, `normalized_source_sha256`, `functional_plan_sha256`, `finalization_size` and the unit counts;
- the block search, Sylvester's warrant, the adjugate solve, the shared-denominator check, `BlockWitness` and replay;
- C₂ = 96 and the metered rule;
- the 256-DOF, 16,384-term and unit, 256-expansion-term, 8,000,000 and 64,000,000 limits;
- every reader check. The readers check the ledger's structure and limits and do not recompute charges (RF §5.4).

**What moves:**
- *directly,* the counted Evaluation-stage work, in the live attempt and the replay;
- *secondarily,* every charge that depends on the length of a text that prints work. The SELECTED diagnostic embeds the recovery summary (`lib.rs` about 5205–5211), and publication charges 12 × diagnostic text length (`source_receipt.rs` about 790–798; b7-plan "D5/MJ9's accounting clarification"). So a work figure that crosses a decimal digit boundary moves publication work too.

### 3.2 Soundness, mutants and tests

- **Soundness.**
  - B1's value is exact by −1·x = −x and 0·x = 0.
  - B2's skipped contribution is exactly zero, and the window excludes every refusal.
  - Both charge before they execute, and every step they add is charged, so RF §5.2's argument stands: executed `Work` arithmetic never exceeds the charged total.
  - The maintained control ("charged ≥ counted", RF §5.4) still runs over the census and its replays.
  - The independent metering oracle (`metered_mul_charges_match_the_independent_oracle`) gains B1's recognition, negation and copy charges and B2's window charge, each derived independently.
- **Gating test (`source_negated_rows_equal_the_long_evaluation`).**
  - Over the J7 RF census, the 24 raw source controls, `o4_source_controls`, MJ6/MJ9 and section 5's inputs, every B1-derived Ratio equals the long evaluation's bit for bit.
  - Every B2 case gives the same values, and the same errors apart from `Budget`.
  - Its constructed inputs include the reviewer's re-grow witness, a source row with three-atom products, and a Selector-basis member of unit stiffness. Each must take the long path.
  - **Coverage assertion.** The test also records which rows take B1 and which take B2's skip. On the Z-hinge census inputs it asserts that all 22 negated rows per member, exact-zero rows included, take B1. So a byte-equality pass cannot hide rows that silently fell back to the long path.
- **Mutants**, each killed by the named test:

| Mutant | Killed by |
|---|---|
| B1 negates a row whose products differ in one atom | gating test; `b1_recognition_refuses_near_matches` |
| B1 reuses the row unnegated | gating test; J7 frozen-row verdicts |
| B1 accepts an extra term whose product has no zero atom | `b1_recognition_refuses_near_matches` |
| B1 drops the static atom-count or unit condition, or the dynamic unit check | gating test on the constructed inputs (three-atom row, unit stiffness, the re-grow witness) |
| B2 skips a recipe on a DOF prescribed at a nonzero value | gating test; `pressure_source_join` load/reference-state controls (prescribed motion) |
| B2 skips outside the window | `b2_window_keeps_every_refusal`: rescued intermediate underflow (for example [k, sign, 0.25, 4] with k near 2^-1021), low-tail bit loss, addition overflow, zero atoms and unit atoms; each refusal must be unchanged |
| an uncharged shortcut or recognition | the charged ≥ counted control and the oracle |

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
- MJ6/MJ9's counters;
- the pressure-source reader corpus `fixtures/results/pressure_source_reader_corpus.json`, `fixtures/results/loadReferenceSourceLedger.cases.json` and `core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json`, wherever they carry work;
- J7 (`fixtures/results/j7_rf_census_expected.json`, through `SWBPIPE_J7_RF_CENSUS_WRITE=1`).
  - J7 runs the direct method independently of public selection (`j7_rf_census.rs` about 252–295), and records selected method work. So all 66 directly selected entries move in their method work: 62 `selected_method_only` and 4 `selected_published`.
  - Only the 4 published entries, the RF-SKEW PIN-OFF-345 r1e-08 and r1e-12 cases in both modes, also move in their envelope and diagnostic digests.
  - Of the other 256 entries, 186 have work fixed before Evaluation (168 block-order, 10 `Budget`, 8 range refusals), and 70 carry no source work (58 HASH-001, 12 axis-gate). J7's 10 `Budget` first causes are all at source closure (the upfront descriptor-unit gate; RF §3.6), before any evaluation, so its classification cannot change.

**Tests that need more than regeneration:**
- **`cached_source_context_preserves_current_public_serializations`** pins work digits (RF §5.5). Its pins are re-pinned.
- **RF §5.5's five budget-cliff fallback tests** (`source_receipt/load_state_fallback_tests.rs`) are re-derived by RF §5.5's own rule: recompute the cliff, scale it within D1's caps, or restate it with a private limit and a positive control.
- **`u4b_source_work_law_reconciles_every_control_with_its_pre_u4b_counters`** (`source_receipt/pressure_source_context_tests.rs` about 635–735). It asserts `charged == old_charged + ΔF + ΔQ` against a hard-coded table of historical pre-U4b counters (`PRE_U4B_WORK`). R-A changes `charged` without a U4b delta. Its `publication == old_publication + ΔP` assertion (about 727–729) also moves wherever a work figure changes its digit count, through the 12 × diagnostic-length publication charge. So both are restated: either the law gains an independently derived R-A delta per control, or the table is rebased to the pre-R-A counters with the rebase stated.
- **`o4_source_work_deltas_follow_the_d5_law`** is expected to hold unchanged. It compares Euler–Bernoulli and Timoshenko twins, whose R-A savings are derived by the same law; it runs in the PR's checks.

**The work-only diff.** A check modelled on `o4_text_only_envelope_diff`, `work_only_envelope_diff`, requires every regenerated document of a case selected or declined today to differ from its pre-R-A bytes only in work counters, the text that embeds them, and the hashes binding those. No value token, interval, row, class, first cause, selection or `response_digest` may move. The J7 census also asserts unchanged classification, counts, first causes, selections and frozen-row verdicts.

**D3.** The changed evaluator is priced source (RF registered the source method under D3; `retained-pricing-rules.md` §7.4). R-A therefore needs an actual-source D3 registration at the unchanged policy and the pricing tool's `generate --check`.
- B1's per-row hint vector, at most one index per descriptor, is a new priced owner. It changes the signatures that carry it: the functional evaluation entry (`evaluate_functionals`) and `replay_against`.
- B2 reads the existing `c.source.prescribed` list. If the implementation adds a per-DOF zero-prescription vector, that is a new priced owner, sized n bits or bytes at most 256.
- The registration measures every phase rather than assuming no growth.

## 5. The census (specified; executed later)

**Purpose:**
- establish R-A's gain on the product;
- confirm byte preservation on the cases selected today;
- judge every newly selected case;
- confirm today's outcome code;
- attribute the residual cost.

All of this comes before adoption is decided.

**The switch and the probe.**
- R-A runs behind a default-off runtime switch in an opt-in cargo feature, forwarded from `product_physics` to the kernel.
  - No `[dev-dependencies]` entry enables it. RF's `exact-operation-count` is reached through `core/product_physics/Cargo.toml` `[dev-dependencies]` and so compiles into every test build; the new feature must not.
  - The census commands pass `--features`, and a guard test asserts that the feature is off in the ordinary test build.
  - So production paths, the committed pins and the stack witnesses run the long path until adoption.
  - The switch and probe live on the census branch, and the switch is removed at adoption: a merged on/off switch would be a second charging regime keyed on a flag, which RF §5's ruling excludes. Anything merged is priced, and D3 applies.
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
  - against a switch-off run under a test-only raised private limit (REQ-003 (e) permits test-only overrides). The comparison applies the preserved cases' allowance exactly: only the work counters, the text that embeds them, and the hashes binding those may differ. Each differing hash (`publication_sha256`, `receipt_sha256`, the envelope digests) is recomputed independently from the compared bytes and must match. That shows B1 and B2 reproduce the long path's bytes.
  
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

## 6. Reserved for the owner

- **AC-002.** Its owner text of 2026-10-19 reads: "the RF census classification, counts, first causes, selections, charged work and frozen-row verdicts are unchanged; its envelope, results and summary digests change only as a stated product-text, identity or published-row change requires".
  - R-A changes the census's charged work on its 66 directly selected entries (62 method-only), and, on the 4 publicly selected entries, the envelope and diagnostic digests that bind that work. Its results and summary digests carry no work (`j7_rf_census.rs` about 333–340), so they do not move. Classification, counts, first causes, selections and frozen-row verdicts stay unchanged.
  - The owner can rule either that this re-pin falls within AC-002, or that AC-002 reads, for example: "… first causes, selections and frozen-row verdicts are unchanged; its charged work changes only as a reviewed work-reduction change requires, with every charge still at least the executed work; its envelope and diagnostics digests change only as a stated product-text, identity, published-row or work change requires, and its results and summary digests only as a stated product-text, identity or published-row change requires".
  - Without that ruling R-A cannot be integrated, and these cases keep today's outcome, a blocked envelope with no result rows (section 1).
- **The pre-0.4 blocked envelope** (option, no recommendation).
  - Today a pre-0.4 case that selects but cannot finalize publishes a blocked envelope with no result rows and the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`. An invocation-level receipt failure refuses the run with `SOURCE_BLOCKS_FINALIZATION_FAILED` (section 1).
  - Extending the replay screen (`reserve_captured_replay`) to the pre-0.4 path would not catch these cases. The screen passes when the live charge is at most the limit less itself, about 4M, and these live attempts cost about 3.7–4.0M [E]; they fail later.
  - The option is to extend the 0.4 path's ordinary republication on finalization failure (SF-1R) to the pre-0.4 path, with or without the screen. A blocked envelope would become an ordinary publication with its failure recorded.
  - It gains no verified coverage. It changes recorded behaviour: RF §2.9 and DEL-04-07 `pressure-contract.md` ("the pre-0.4 path retains its named refusal"), with the pinned test `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback`, whose comment assigns the outcome to T3 as "a separate open finding".
  - Whether R-A alone, R-A plus this, or neither is wanted is the owner's choice. R-A reduces how often the blocked outcome occurs; this changes what it is.
- **Second-stage candidates** (3.3), each with its own ruling. F2 also needs HELP_HUMAN's ruling on RF §5's charging scope. No recommendation is made now.
- **The work limit.** Unchanged and not designed (REQ-003 (e), AX-005). Raising it remains the owner's choice, with D3 and independent review.
- **Placement.** R-A is not in b7-plan's epoch order. Agent 0 and the owner place it; section 0 states its sequencing constraints.

## 7. Verification and review

- **This design:** the dual review, an independent Claude reviewer and an independent Codex reviewer on the same revision. Round 1 reviewed revision 1; revision 2 answers it.
- **The implementation (R-A):**
  - the gating test, the mutants of 3.2 and the census of section 5;
  - VER-002's checks and the J7 census;
  - `work_only_envelope_diff`;
  - the actual-source D3 run;
  - the independent numerical review that Root `AGENTS.md` requires for numerical meaning.
