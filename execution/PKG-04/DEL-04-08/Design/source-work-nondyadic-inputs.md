# Source route: the exact-arithmetic cost of non-dyadic inputs

This design examines O4's coverage finding (b) (PR #1468, "Coverage findings"; [o4-source-extension.md](o4-source-extension.md) §5, "Census"). On 3D skew single members ((1,2,2), the Z-hinge layout) with ordinary decimal inputs, retained-source recovery (`pressure-source-1`) falls back with `Exact(Budget)` at the unchanged 8,000,000 per-case limit (`PHYSICS_SOURCE_WORK_LIMIT`, PP `lib.rs:939`). This happens in 10 of 60 Euler–Bernoulli runs (17 %) and 22 of 60 Timoshenko runs (37 %). Planar members and dyadic twins never fall back. The owner decided on 2026-10-20 ("All as recommended") that a remedy be designed now. Its goal is to reduce the exact-arithmetic cost so that these cases fit the unchanged limit.

**Status: design only.** No product file, fixture, reviewed input or ScopeOfWork changes here. The basis is main `fa019d9cef` (O4 merged as #1468). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Summary

- **Where the charge goes** (section 2; derived from the code, with an uncommitted cost model).
  - A selected case's ledger is about 2 × E_live + 0.93M. E_live is one pass of exact work: Preparation, Solve, the evaluation of all 73 functional rows and all 97 projections. The captured replay repeats that pass in full, by design. The fixed charges (closure, preflights, descriptors, finalization, maxima) make up the rest.
  - For decimal Euler–Bernoulli inputs, E_live averages 3.32M (range 2.87–3.84M). For the dyadic twins it is 2.60M.
- **Why decimal inputs cost more** (section 2.3). The charge per term pair grows with the output expansion's length, so a product costs roughly |a|·|b|·(98 + 6ℓ).
  - The dyadic twins differ only in the spring stiffness and the moment, which are powers of two there. So the gap arises downstream of the displacement numerators: they average 14.0 terms (decimal) against 9.6 (dyadic). Every row evaluated from them inherits the difference.
  - Preparation costs the same in both styles.
  - Timoshenko inputs cost about 2 % more on average, with statistically equal lengths. Its higher fallback rate looks like a threshold effect near the limit, not a separate cause.
- **The redundancy** (section 2.4). Of the 30 station section rows, 22 are, by construction, the end row scaled by −1, with extra moment products whose fraction factor is 0 at the i end. That is components 0–3 at all five stations, and components 4–5 at the i-end station (`source_recovery.rs:1775–1786`). Today each is evaluated again in full: about 17 % of E_live. The evaluation also forms coefficient recipes for DOFs prescribed at exactly ±0, and then multiplies them by zero: about 3.7 %.
- **Recommended remedy: R-A** (section 3). It removes that duplicated work and changes no value, check, limit or charging rule:
  - *B1.* A row whose descriptor is bitwise the negation of an earlier row's takes that row's exact Ratio, negated termwise. Its projection still runs.
  - *B2.* A coefficient recipe on a DOF prescribed at ±0.0 is skipped. An exponent pre-check refuses exactly where the skipped recipe would have refused.
  
  The same evaluator runs in the live attempt and in the replay. So the replay still recomputes everything from the current invocation, by the same rule.
- **Expected gain.**
  - In the cost model: about −20 % of E_live and about −18 % of the case charge. No Z-hinge decimal run stays above 8M; the model's maximum is 7.06M (Euler–Bernoulli) and 7.03M (Timoshenko).
  - The model under-predicts the product's fallback rate (13 % and 23 % against the census's 17 % and 37 %), so the product's own gain is established by the census of section 5 before implementation is accepted.
  - Planar members gain proportionally. Small frames refused at block order above 4 are not affected.
- **Cost.**
  - One evaluator change in `exact_boundary/functionals.rs` with two killing tests.
  - The independent numerical review.
  - An actual-source D3 registration.
  - Re-pinned work counters wherever they are committed (section 4).
- **Reserved for the owner** (section 6).
  - **AC-002's "charged work … unchanged".** The J7 RF census pins charged work. Any reduction moves those pins while leaving classification, counts, first causes, selections and frozen-row verdicts unchanged. So R-A needs the owner to amend AC-002's wording or rule that the re-pin falls within it. This is the only reserved choice R-A needs.
  - **Further reductions** (section 3.3). Expansion compression could save about 30 %, but adds a new metered primitive to RF §5's "one rule". Other candidates change published terms or descriptors. Each is a second stage with its own ruling, and none is recommended now.
  - Raising the 8,000,000 limit is not designed here (REQ-003 (e), AX-005).

## 0. Basis and dependencies

**Basis.** Main `fa019d9cef`. Paths are relative to `projects/chirality-piping/`; PP is `core/product_physics/src/`, FK `core/solver/frame_kernel/src/structural/`. The census figures (17 %, 37 %, 21/50 and 20/38 above 7.5M) are O4's, from `o4_m3_judging_tests.rs` `o4_census_source_work_single_members` (lines 962–1003) as reported in PR #1468. Their raw output (`O4_CENSUS_OUT`) is not committed. This design's own figures come from a standard-library Python re-implementation of `Expansion`, `Work` and the metered rule. It was written for this design in `/tmp` scratch and is not committed; tags mark each figure:
- **[D]** derived from the code;
- **[E]** estimated with that model;
- **[M]** measured in a committed document.

The model reproduces RF's measured (1,2,2) congruence charge: 83,882 [E] against 84,268 [M] (rf-source-method-extension.md §3.5). It reproduces the census's pattern too: decimal runs above 8M and dyadic runs never. Its section properties and κ only approximate the product's, so its per-case figures are estimates.

**Assumptions from concurrent work, and sequencing.**
- **Stack-headroom repair (b7-plan item 5a).** It shrinks debug frames in `freeze_case`, `source_residual::recover` and `member` on the retained W1 path, then re-registers D3 and puts the six stack witnesses into a maintained check.
  - R-A touches none of those functions. Its code is the source route's exact evaluator (FK `exact_boundary/functionals.rs`), which is not on that 35-frame chain.
  - Assumption: 5a's frame repair does not change `exact_boundary`, `functionals` or `source_recovery`. If it does, R-A rebases on it.
  - Sequencing: R-A's implementation follows 5a, because both register under D3 in the same generated profile block. R-A re-runs the six witnesses at R/64 in the registered profile, as AC-004 requires after 5a.
- **W1 coverage repair** (w1-coverage-repair.md, epoch 6). It changes the W1 kernel's loaded-maximum search and the receipt's scales; R-A changes the source route's evaluator. They share no file and no law.
  - Assumption: the W1 repair moves no source-route work counter. Its §1.4 lists none.
  - Sequencing: the two also share only the D3 block. Whichever lands second re-registers on the other's profile. Neither waits for the other's design.
- **O4 as built.** R-A keeps O4's work law (o4-source-extension.md §3.5) and the D5/MJ9 accounting. Only the counted work of the evaluation stage moves.

## 1. The ledger and the limit

The per-case ledger is the metered `Work` of RF §5 (C₂ = 96 per term pair, plus 6·len + 1 before each grow-expansion add). It also carries the finalization and publication reservations of RF §2.9 and O4 §3.5. `Work::charge` refuses before any work when the next total would pass the limit (RF §5.2 (1)). The case then falls back with `Exact(Budget)` and publishes its ordinary result (REQ-003 (c)). RF §5.2 (3) states the rule's soundness: executed `Work` arithmetic is bounded by the charged total, which never exceeds the limit. R-A keeps that rule unchanged. It removes executed arithmetic, so it removes the charges for that arithmetic. It adds no uncharged work.

## 2. Cost anatomy of one Z-hinge attempt

### 2.1 The problem [D]

- **The layout** (`census_document` "zhinge122"): one (1,2,2) member; N0 rigid in UX, UY, UZ, RX and RY, with a soft RZ spring; N1 rigid in UZ, RX and RY; one moment about z at N1.
- **DOFs.** n = 12. Four are free: N0.RZ, N1.UX, N1.UY and N1.RZ. They form one block of order 4. The other 8 DOFs are prescribed at exactly 0.0.
- **Rows.** 73 functional rows: 12 end, 30 section, 12 nodal, 1 spring and 18 support. There are 24 DOF projections.
- **The congruence.** T has 8 nonzeros per 3×3 block, and K_local has 40. So 120 of the 144 exact congruence entries are nonzero.
- **The response** is a near-rigid rotation about global z, so 49 of the 73 rows are exact zeros. Each is still evaluated at full cost before it cancels.

### 2.2 Where one live pass spends its work [E]

Shares of E_live, decimal Euler–Bernoulli:

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
| Preparation: congruence Tᵀ·(K·T) (163–209) | 2.5 % |
| Solve: block `ratio_add` | 2.7 % |
| Other (symmetry, scatter, solve precharges, numerators, DOF projections) | about 3.5 % |

Outside E_live:
- `prepare_sources` runs three times (about 52k each);
- `source_preflight` four times (about 8k each);
- `descriptors_charge` five times (about 32k each);
- retention (about 45k), output (2,720 [D]), the maxima and norms (about 56k);
- the finalization reservation, about 0.52M [E]. The (3,4,0) member's was measured at 379,940 [M] (RF §2.9).

Together these make about 0.93M [E].

**Repetition [D].**
- The full exact pass runs twice, live and in the captured replay (`functionals.rs` `replay_against`, about 977–1057). The binding check (`check_binding_with_budget`) repeats no arithmetic.
- Within one pass, minors are cached, and the adjugate is formed once in Preparation and reused in Solve.
- Section rows are not reused, and neither are the prescribed-zero coefficient recipes (2.4).

### 2.3 Why decimal inputs are dearer [D, E]

- **The charge.** A metered pair costs 96 + (6ℓ + 1) + (6ℓ + 1), where ℓ is the output's current length (`metered_product`, `add_product_charged`). So `mul(a, b)` grows roughly with |a|·|b|·ℓ.
- **53-bit atoms.** A two-product of two 53-bit significands leaves a nonzero low part. Grow-expansion with zero elimination keeps it, so each pair adds about two terms. Short (dyadic) atoms whose significand widths sum to 53 bits or less give lo = 0, and the sweep merges neighbours.
- **What the dyadic twins change.** In the Z-hinge twins, K_local and T are 53-bit in both styles: π enters A, I and J, entries are divided by L, and T comes from l/3 and a square root. Only k and m₁ become powers of two. Mean expansion lengths [E], with canonical lengths in brackets:

| Quantity | Decimal | Dyadic |
|---|---|---|
| block entry | 3.5 (3.0) | 3.5 (3.0) |
| Δ4 | 26.7 (11.3) | 26.2 (10.8) |
| adjugate entry | 19.5 (9) | 19.1 (9) |
| displacement numerator (adj·m₁) | **14.0 (10)** | **9.6 (9)** |
| reaction numerator | 14.0 (12) | 12.6 (10.8) |
| nonzero row numerator | 23.4 (11.1) | 19.8 (9.8) |

- **Where the gap lands.** The whole decimal-to-dyadic gap, 3.32M against 2.60M per pass, comes downstream of the displacement numerators. Section rows take −267k of it, reactions −200k, support rows −123k and end rows −93k.
- **Representation alone cannot help.** Exact power-of-two scaling, or factoring out a common exponent, saves nothing [D]: C₂ is a flat 96 per pair, and significand widths do not change. A big-integer engine would need a new cost model, which RF §5's ruling ("one rule") does not provide. It is not designed here.

### 2.4 The duplicated work [D]

- **Section rows.** `member_descriptors` (`source_recovery.rs:1741–1797`) builds each station row as `scaled_row(&rows[component], -1.0)` from the member's i-end row.
  - For components 4 and 5 it appends the shear row's products times [sign, fraction, length].
  - At the i-end station the fraction is 0.0, so every appended product carries an exact zero factor.
  - So components 0–3 at all five stations, and components 4–5 at the i end, are the i-end row times −1 exactly: 22 of the 30 section rows per member.
  - `evaluate` (`functionals.rs` about 585–630) recomputes each from its products. With the −1.0 factor it reaches `mul`'s ±1 shortcut (about 514–522), but only after every coefficient recipe and block numerator of the row has been formed again.
- **Prescribed zeros.** `evaluate` forms every term's coefficient `recipe` before it reads `c.source.prescribed`. For a DOF prescribed at ±0.0, `mul(coefficient, scalar(0.0))` returns the empty expansion, so the coefficient's recipe was computed for nothing. Eight of the twelve DOFs here are prescribed at zero.

## 3. The remedy

### 3.1 R-A: reuse the negated row, and skip zero-prescribed recipes

**B1: negated rows.**
- **Recognition (structural, bitwise).** In `evaluate`, a descriptor d is a negated copy of an earlier descriptor s of the same set when all of the following hold:
  - d's terms list s's DOFs in the same order;
  - each of d's products is the corresponding product of s with one appended −1.0 atom, after which d may carry further products that each contain an exact ±0.0 atom;
  - the same holds for the offset.
  
  The check is charged at its actual cost: the atoms it compares, per the cost model of RF §5.3.
- **Value.** d's Ratio is s's Ratio with its numerator negated termwise; the denominator is shared. That is d's exact value: the appended zero products contribute exactly zero, and a −1 factor negates exactly.
- **Byte preservation, and the gating test.** The long path's own arithmetic is sign-symmetric under round-to-nearest-even: two-sum, the fused two-product, power-of-two scaling, zero elimination and the ±1 shortcut. So the long path would produce the same terms negated. A gating test asserts this bit for bit (3.2). Where it would fail, the implementation stops for diagnosis rather than accept moved bytes.
- **Projection still runs.** B1 does not derive an interval by negation. Every row keeps its own `project` call, so no property of `project` is assumed.
- **Fallback.** A descriptor that does not match the pattern takes the long path. So does any descriptor in a set where s is not earlier in order.

**B2: zero-prescribed recipes.**
- **The skip.** In `evaluate`, a term whose DOF is prescribed at exactly +0.0 or −0.0 skips its coefficient `recipe`.
- **The pre-check.** A skipped recipe could have refused only by `Range` or `Budget`. `Budget` refusals can only disappear under less work, as RF §5.2 (5) already states for the metered rule. A `Range` refusal is preserved by an exponent pre-check: for each skipped product, the sum of its atoms' binary exponents is checked against the bounds `add_product` enforces, and it refuses with the same `Range` error. So no refusal other than `Budget` can change.
- **Prescribed values that are not zero** keep today's path.

**The replay.** `replay_against` calls the same `evaluate`, so the replay recomputes every row from the current invocation by the same rule. Nothing retained from the live attempt is reused. This keeps the replay's independence: RF's "(d)" option, reusing cofactors, would not, and is not proposed.

**What does not change:**
- every value, interval, bound, basis and published byte of a selected case;
- the descriptors, `normalized_source_sha256`, `functional_plan_sha256`, `finalization_size` and the unit counts;
- the block search, Sylvester's warrant, the adjugate solve, the shared-denominator check, `BlockWitness` and replay;
- C₂ = 96 and the metered rule;
- the 256-DOF, 16,384-term and unit, 256-expansion-term, 8,000,000 and 64,000,000 limits;
- every reader check. The readers check the ledger's structure and limits and do not recompute charges (RF §5.4).

**What moves:** the counted work of the Evaluation stage, in the live attempt and the replay. Every committed counter that records it moves with it (section 4).

### 3.2 Soundness, mutants and tests

- **Soundness.**
  - B1's value is exact by the identity −1·x = −x and 0·x = 0.
  - B2's skipped contribution is exactly zero.
  - Both charge before they execute, and they execute less, so RF §5.2's argument stands unchanged: executed `Work` arithmetic never exceeds the charged total.
  - The maintained control ("charged ≥ counted", RF §5.4) still runs over the census and its replays.
  - The independent metering oracle (`metered_mul_charges_match_the_independent_oracle`) gains B1's comparison charge and B2's pre-check charge, each derived independently.
- **Gating test (`source_negated_rows_equal_the_long_evaluation`).** Over the J7 RF census, the 24 raw source controls, MJ6/MJ9 and the census sweeps of section 5, every B1-derived Ratio equals the long evaluation's bit for bit, and every B2 case gives the same values and errors.
- **Mutants**, each killed by the named test:

| Mutant | Killed by |
|---|---|
| B1 negates a row whose products differ in one atom | gating test (value mismatch); and `b1_recognition_refuses_near_matches` (a descriptor with one changed atom takes the long path) |
| B1 skips the −1 (reuses the row unnegated) | gating test; J7 frozen-row verdicts |
| B1 accepts an appended product with a nonzero atom | `b1_recognition_refuses_near_matches` |
| B2 skips a recipe on a DOF prescribed at a nonzero value | gating test; `pressure_source_join` load/reference-state controls (prescribed motion) |
| B2 omits the exponent pre-check | `b2_skipped_recipe_keeps_range_refusal` (an in-family descriptor whose skipped product would underflow) |
| an uncharged shortcut | the charged ≥ counted control and the oracle |

### 3.3 Further reductions (second stage; not recommended now)

Measured in the same model [E], against E_live, after R-A:

| Candidate | Saving | What it changes, and who rules |
|---|---|---|
| R1: station moment rows formed from end-row values, −num_c + s·f·L·num_shear | about −10 % | Ratio terms differ. Published values agreed in the model, but bit-equality is not guaranteed. Rows are recognized in `evaluate` or by a changed plan, and `functional_plan_sha256` would move. Needs its own gating census. |
| R4: support rows taken from the Solve reactions | about −14 % | Terms differ, and recognition is intricate or the plan changes. As R1. |
| F1: one extra `Work::add(e, 0.0)` sweep at k, the minors, the determinants and the numerators | about −10 % | An existing charged primitive. Expansion terms and witness bits change, which can move projections, `response_digest` and J7's frozen bytes. Needs the AC-002 ruling below for more than work. |
| F2: Shewchuk compression of expansions at the same points | about −30 % | A new metered primitive, charged before it runs (12·len + 2), with the counter and oracle extended. It amends the scope of RF §5's "one rule" (HELP_HUMAN's ruling, 2026-10-16), so it needs that ruling again. Published values agreed in the model in all but 2 of 11,640 projections, so its byte effect needs its own gating. |
| R3: skip the unit-denominator multiplications in reaction `ratio_add` | about −1.8 % | Reaction ratio terms change. |
| Upper-triangle symmetry check | about −0.2 % | No byte effect. Could join R-A. |

Together these measured −66 % in the model (case maximum about 3.3M). They are listed so that a later coverage need can take them in order, with their rulings. They are not needed for the Z-hinge gap if section 5 confirms R-A's gain.

**Not available:** raising `PHYSICS_SOURCE_WORK_LIMIT`. That is a priced-source change and a limit relaxed to obtain a bound, on which REQ-003 (e) and AX-005 bear. Reserved for the owner and not designed here.

## 4. Committed bytes that move, and the checks that bind them

R-A changes counted work and nothing else. Every committed record of a source attempt's work moves, by its own writer:
- each `pressure-source-1` receipt's `work` and `invocation_work`, and the hashes that bind them (`receipt_sha256` and the envelope digests);
- the SELECTED diagnostic text, which prints the work summary;
- the 24 raw source controls' work leaves (RF §5.5 counted 6 leaves each: 3 counters, 2 hashes and 1 decline text);
- their derived documents and J2's bases;
- MJ6/MJ9's counters;
- J7's `first_cause.work.charged` and its envelope/diagnostic digests, `fixtures/results/j7_rf_census_expected.json`, regenerated through `SWBPIPE_J7_RF_CENSUS_WRITE=1`;
- RF §5.5's five budget-cliff fallback tests (`source_receipt/load_state_fallback_tests.rs`). They are re-derived by RF §5.5's own rule (recompute the cliff, scale it within D1's caps, or restate it with a private limit and a positive control).

A check modelled on `o4_text_only_envelope_diff`, named `work_only_envelope_diff`, requires every regenerated source document to differ from its pre-R-A bytes only in work counters, the decline or diagnostic text that embeds them, and the hashes binding those. No value token, interval, row, class, first cause, selection or response digest may move. The J7 census additionally asserts unchanged classification, counts, first causes, selections and frozen-row verdicts.

**D3.** The changed evaluator is priced source (RF registered the source method under D3; `retained-pricing-rules.md` §7.4). So R-A needs an actual-source D3 registration at the unchanged policy, and the pricing tool's `generate --check`. R-A allocates no new owner and removes expansion clones, so no priced phase should grow. The registration measures this rather than assuming it.

## 5. The census (specified; executed later)

**Purpose:**
- establish R-A's gain on the product itself;
- confirm byte preservation;
- attribute the residual cost;
- do all of this before R-A is adopted on production paths.

**Probe.**
- Under the existing `exact-operation-count` dev feature, record per labelled site, for every exact attempt:
  - charge and pair counts, for each table site of section 2.2;
  - each row by index and kind, and each projection;
  - the attempt stage and phase: live, captured replay, binding, finalization or maxima.
- Record, test-side in exact rationals, the actual and canonical lengths of k, the block entries, Δ2–Δ4, the adjugate entries, the displacement and reaction numerators and each row numerator.
- R-A runs behind a test switch, which is a dev-only cargo feature (section 0, 5a). Production paths stay unchanged until the census is reported.

**Inputs:**
- O4's sweeps `o4_census_source_work_single_members` and `o4_census_source_work_small_frames`, in both styles, shear models and allowances;
- the J7 RF census;
- the 24 raw source controls, the PIN-OFF-345 controls and MJ6/MJ9;
- the O4 source twins and declared-problem selections (`validation/references/o4_retained_source/declared_problem/required_selection.json`).

**Records**, per case, with the switch off and on:
- selection or first cause, and the case and invocation charges;
- every published byte, compared entry by entry: `value_bits`, interval, both bounds, basis, `response_digest`, both source hashes and `receipt_sha256`;
- the envelope, results and summary digests;
- the charged ≥ counted control and the oracle's verdict.

**Pre-registered decision rules.**
- *Byte rule.* Any difference outside the work counters and the text and hashes that bind them stops R-A for diagnosis. The gating test's premise would then be false.
- *Gain rule.* R-A is adopted if:
  - at least 90 % of the Z-hinge decimal runs that fall back today select with it on, in both shear models;
  - no case that selects today stops selecting.
  
  If fewer qualify, the report states the residual distribution against 8M. The design then returns to HELP_HUMAN with section 3.3's next candidate and its ruling. No limit is touched.
- *Attribution.* The per-site table replaces section 2.2's model estimates. A Timoshenko-specific site cost above 5 % of E_live would revise section 2.3's threshold reading.

**What it needs:**
- A debug build of the `frame_kernel` and `product_physics` tests with the feature, under the host's heavy-run slot and memory rules.
- Private drivers only.
- A pilot of ten documents per sweep to measure time and memory first.
- It writes nothing under `validation/references`. Committed fixtures are regenerated only in R-A's implementation PR, after the census.

## 6. Reserved for the owner

- **AC-002.** Its owner text of 2026-10-19 reads: "the RF census classification, counts, first causes, selections, charged work and frozen-row verdicts are unchanged; its envelope, results and summary digests change only as a stated product-text, identity or published-row change requires".
  - R-A changes the census's charged work and the digests that bind the work text. Classification, counts, first causes, selections and frozen-row verdicts stay unchanged.
  - The owner can rule either that this re-pin falls within AC-002, or that AC-002 reads, for example: "… first causes, selections and frozen-row verdicts are unchanged; its charged work changes only as a reviewed work-reduction change requires, with every charge still at least the executed work; its envelope, results and summary digests change only as a stated product-text, identity, published-row or work change requires".
  - Without that ruling R-A cannot be integrated. The Z-hinge cases keep falling back fail-closed, as today.
- **Second-stage candidates** (3.3): each with its own ruling. F2 also needs HELP_HUMAN's ruling on RF §5's charging scope. No recommendation is made now.
- **The work limit.** Unchanged and not designed (REQ-003 (e), AX-005). Raising it remains the owner's choice, with D3 and independent review.
- **Placement.** R-A is not in b7-plan's epoch order. Agent 0 and the owner place it; section 0 states its sequencing constraints.

## 7. Verification and review

- **This design:** the dual review, an independent Claude reviewer and an independent Codex reviewer on the same revision.
- **The implementation (R-A):**
  - the gating test, the mutants of 3.2 and the census of section 5;
  - VER-002's checks and the J7 census;
  - `work_only_envelope_diff`;
  - the actual-source D3 run;
  - the independent numerical review that Root `AGENTS.md` requires for numerical meaning.
