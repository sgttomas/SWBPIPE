# RV19: independent review of slice K4 (the W1a kernel method, retained precision)

- **Reviewer:** RV19 (Type 2 TASK, independent reviewer), dispatched by ROOT.
- **PR:** #1054, `codex/piping-k4-20260928`, head `7d8fa9c0e397569e048e14af6036106d53f46ae6`; base main `7ac7b1c37`, merged in at `8f8023a20`.
- **Date:** 2026-09-29.
- **Verdict: FAIL.** 1 BLOCKING, 5 SHOULD-FIX, 7 NOTEs.

**Summary.**
- **One false publication (RV19-1, BLOCKING).** The stop rule's S\* counts rows that the candidate cannot publish (binary64 overflow or underflow), but the classification's S\* excludes them (ROOT's O9). With a displacement of about 2^1029 m in the body, K4 accepts at 128 a node rotation of −2^901 that is pure rounding noise (the exact value is 0), and classes it `relative_verified`. The fix is small: I probed it, and it moves no control.
- **A design-level gap in the published bound (RV19-6, SHOULD-FIX, for ROOT).** Where S\* < 2^-988, D1 classes every row `absolute_verified` with b = fl↑(2^-64·S\*). That b lies below the binary64 resolution of the rows themselves, so R7 §5.2's "\|q_pub − q\*\| ≤ b·(1 + 2^-22)" fails by up to about 2^11, even though K4's retained values are within b. A tip load of 2^-995 N shows it on 7 rows (ratios 222 to 819). K4 implements the pinned formulas; the remedy is a design ruling.
- **Everything else I checked on the honesty path holds.** That covers R7 §5's acceptance rule (a) to (d), directed rounding, the certified bounds, W⁺, E, ê, Φ, the hybrid gate and the classification.
  - The Hager–Higham estimate never decides honesty.
  - My own exact oracle (1,600-digit solves, written from D1 and not from GEN) finds every selected control and combination within its published claim: 115 publications and 8,086 rows. Thirteen of these selected controls (six of them RF-LARGE at 10 members), and the six RF-LARGE frames at 100 members, are not checked against the tightened predicate in K4 (RV19-2).
- **The items ROOT directed to me:**
  - §6 item 4: **confirmed.** It follows from Lemma C, and R7's statement of Lemma C already presupposes it.
  - §22.1: confirmed, with a NOTE on its count.
  - §22.2: confirmed.
  - §9 steps 10 to 12: confirmed, with a NOTE on wording.
  - §22.9: correct in direction, but it skips rows and controls silently (RV19-2).
- **Test gaps.** Two of my eight mutants survive, and each is a SHOULD-FIX test gap:
  - RV19-4: a support group's E without its directional contributors (Q12). No control has such a group.
  - RV19-5: θ without Lemma C's 1-norm.
- **Unchanged or verified:**
  - kernel only;
  - the merge;
  - `gen_k4_vectors.py --check` (23 of 23);
  - FK's full suite (389);
  - the records' checksums;
  - GEN's schedule against R7's emulation (59 of 59).

## Reviewer, brief and delegation

- **Reviewer.** RV19 ran as a background subagent of ROOT's HELP_HUMAN session, started through the Agent tool. ROOT is its only return path.
  - I did not design, implement or test K4.
  - I delegated nothing.
  - I made no Git write, no index operation and no GitHub write.
  - I read the PR's state with `gh pr view` and the remote heads with `git ls-remote`.
  - My writes are this file and `REVIEW/_run_records/k4_review/**`, left uncommitted.
- **Read:**
  - Root `AGENTS.md`, `agents/AGENT_TASK.md`, `TASK_BRIEFS/_COMMON.md`, and `I8R_K1_RESUME.md:24-50`;
  - `TASK_BRIEFS/I12_K4_IMPLEMENTATION.md`, with its rulings Q1–Q12;
  - `DESIGN_NUMERICS/DESIGN.md` §4.1 (revision 5a.2), as cited by R7 and the brief;
  - `REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md` (sha256 `5502aef9…`, checked): §2–§5 in full, §6.6 and §7;
  - `V4_VERIFICATION.md`, for the verdict chain;
  - `_run_records_r7/run_controls7.stdout.json`;
  - every K4 section of `ROOT_RULINGS_V1.md` at the numerics head (`730063388` at dispatch; `306e225fe` at the end, which adds only the I16 and I17 briefs);
  - on the K4 branch: `IMPLEMENTATION/K4/{PLAN_A3_5A3.md (outline), RETURN.md, CHANGE_RECORD.md}` and the `_run_records/` I cite;
  - `REVIEW/K3_REVIEW.md` and `K5_REVIEW.md`, for method;
  - the code: all of `directed.rs`, `bound.rs`, `verify.rs` and `wide_sum.rs`; `adaptive.rs` (the stop rule, the scales, the classification, the schedule, the verification wiring and `CasePrep`); `factor.rs` (`factor`, the solves, `pivot_passes`); `recover.rs`; `combine.rs`; the member formation and `assemble_bounded` in `assemble.rs`; `ledger.rs`'s projection; and the `exact_sum.rs` diff;
  - the tests `models.rs`, `method_tests.rs` (the controls test, G5a and E-CHARGE), `scale_tests.rs` (E-UC), `directed_tests.rs`, `recover_tests.rs`, `references_tests.rs` (the RF-LARGE lane) and `golden_work_counts`;
  - GEN's `inv_norm1_upper` and its caller.
- **Placeholders.** `<wt>` is the T3 worktrees root; `FK/` is `P/core/solver/frame_kernel/`; `K4T/` is `FK/tests/retained_k4/`; `GEN` is `K4T/gen_k4_vectors.py`; R7 is as above. Line numbers are at `7d8fa9c0e`.
- **Host** (`checks/toolchain.txt`):
  - rustc 1.97.1 with `--offline --locked`, `-j 4` and `RUST_TEST_THREADS=2`, with at most two cargo jobs of mine at once;
  - every build from `git archive 7d8fa9c0e` copies under `<wt>/rv19/`, with my own targets, all deleted at the end;
  - the memory guard ran throughout and its log records no kill (`checks/memguard.txt`).

## Revisions (`checks/revisions.txt`, `checks/merge_check.txt`)

- **The PR.** PR #1054 is OPEN and MERGEABLE at head `7d8fa9c0e`. The remote branch agrees, and main is still `7ac7b1c37`. Hosted checks: 12 SUCCESS, 4 SKIPPED.
- **The merge `8f8023a20` adds exactly main's delta.**
  - Its parents are `8a59458a1` (B) and `7ac7b1c37`.
  - `git diff 8a59458a1 8f8023a20` and `git diff 59cb20073 7ac7b1c37` (the merge base to main) have the same sha256 (`259349db…`): 1,145 files, +443,449 in both.
  - `git show --remerge-diff` is empty, and the delta touches no FK file.
  - After the merge, D changes only K4's records and three test files (`method_tests.rs`, `models.rs`, `recover_tests.rs`).

## Findings

| ID | Class | Site | Evidence | Fix |
|---|---|---|---|---|
| RV19-1 | **BLOCKING** | `FK/src/structural/retained/adaptive.rs:1496-1544` (`scales_at`: the only filter is `meta.input_derived`, `:1508`), called by the stop rule at `:1631`, against `classify_rows_floored` `:2159-2176` (`if let Some(x) = v.value()`, `:2170`) | **A selected row outside its claim** (§1.4). **OVF-ROT-928:** one member (0,0,0)→(1,2,0), E = G = 1, A = Iy = Iz = J = 2^-100, y_ref (0,0,1); node 0 fixed; load 2^928·(1,2,0) along the member.<br>– Selected at 128. u_x, u_y and \|u\| at node 1 publish `Overflow` (about 2^1029).<br>– **u(1,Rz) publishes −2^901 (−1.69e271) as `relative_verified`.** Its exact value is 0: an axial load gives pure extension, and the 256 state's Rz is exactly 0.0.<br>– Claim ratio: 9.0e15 against the tightened allowance, and 2^64 against 2^-64·S\*; RV19's oracle agrees.<br>– The stop rule accepted Rz against S\*_2p(rot) = S_tr/L_b ≈ 2^1029, formed from the overflowing rows. The classification then dropped those rows (O9), so S\*_pub(rot) = \|Rz\| itself.<br>**The same model at 2^900** (nothing overflows) publishes the same spurious Rz as `absolute_verified` with b ≈ 1.3e282, which is honest. `probes/probe_ovf.log`, `oracle/oracle_ovf.log`. | Apply O9 to the stop rule too: `scales_at` skips a row whose **candidate** value is nonzero and has no binary64 value. I probed exactly this (`fix/fix_probe_adaptive.diff`):<br>– OVF-ROT-928 becomes `Unresolved(Ceiling)`, rejected at 128, 256 and 512;<br>– OVF-ROT-900 is unchanged;<br>– **every K4 test passes** (118: all but the four N5 arithmetic streams, alongside K3's 45 and my 3 probes, 166 in all). The controls test still equals GEN token for token, so no control moves, SPRING-CARRIED, GS-ROT-y345 and M10-ANISO (which have `Underflow` rows) included.<br>Add OVF-ROT-928 as a control (GEN and `outcomes.txt`), with the reverted fix as a mutant killed by it. |
| RV19-2 | SHOULD-FIX | `K4T/models.rs:346-415` (`compare_honest`: `:367-369` and `:405-407` skip silently), `K4T/method_tests.rs:697-704` (`if expect.is_empty() { continue }`; `compared > 0`), `K4T/references_tests.rs:406-428` (RF-LARGE on R1's 1e-9 predicate) | **Coverage** (`probes/probe_coverage.log`; §2.5).<br>– **13 selected controls have no exact expectation** and are skipped: N03-RX, the four HH-FOOL forms, HH-SLENDER-m40, R115-SEED3 (selected at 512) and the six RF-LARGE 10-member frames. The six 100-member frames are checked only against R1's 1e-9 predicate, and G5a never runs on them.<br>– 33 expected rows on 3 controls (SPRING-CARRIED, GS-ROT-y345, M10-ANISO) publish `Underflow` and are skipped with no range check.<br>– A control passes with `compared > 0`.<br>So RETURN's "every selected control satisfies the claim it publishes" (status line, §22.9), §12.6's and §22.8's "RF-LARGE … honest under the tightened predicate and pass G5a", and CHANGE_RECORD's "every selected control" say more than K4's tests check.<br>RV19's oracle checked all 115 selected publications, the 13 included, and found no false claim. Every published row with a value has an expectation. | Give GEN's exact expectations to the 13 (HH-FOOL has 12 free DOFs; RF-LARGE-10 has 60), or state their exclusion.<br>Assert that every published row is compared, and that an `Underflow` or `Overflow` row matches its exact value's range. That needs the underflow marker ROOT routed at D.<br>Run `compare_honest` and G5a on RF-LARGE, or correct §12.6, §22.8 and CHANGE_RECORD. |
| RV19-3 | SHOULD-FIX | `FK/src/structural/retained/combine.rs:95-99` (operand identity: `stiffness_encoding` plus layout equality) | **The operands' station and support-group definitions are not compared.** `K4STF` omits stations and groups, and the layout holds only their ids.<br>**Probe** (`probes/probe_station.log`): a unit cantilever with a tip load of 1 N, station 1 at t = 0.25 (operand A, st.1.4 = 0.75) and at t = 0.75 (operand B, st.1.4 = 0.25).<br>– A + B is selected at 128 and publishes **st.1.4 = 1.5** `relative_verified`: A's station, doubled.<br>– Σcᵢ·(case i) is 1.0, and `RetainedCombination` documents "Combinations Σ cᵢ·(case i)".<br>A support group reused under the same id with different members is alike.<br>Kernel API only: F2a combines cases of one model. | Require equal operand sources apart from loads and constraint values (for example `K4SRC` without them). Otherwise return `OperandsDiffer`, with a unit test on this pair. |
| RV19-4 | SHOULD-FIX | `FK/src/structural/retained/verify.rs:262-271` (a support group's E from its directional contributors; Q12, RETURN §22.1) | **Mutant RV19-M6** drops the directional contributors from E_group. **It survives all 111 of K4's tests that it runs** (only the N5 streams and sum differentials are skipped).<br>No control has a support group with a directional spring: all five `support` lines in GEN's models list none.<br>ROOT named this path honesty-relevant: E_group feeds ê, V, (b) and (d). My probe of two such models (GROUP-DIR, GROUP-DIR-X) is honest under my oracle (`oracle/oracle_groups.log`), but nothing in K4 pins it. | Add a control with groups that hold directional springs, springs and restraints (for example `probes/rv19_group_models.txt`) to GEN, E-UNIT and E-CHARGE. Show RV19-M6 killed. |
| RV19-5 | SHOULD-FIX | `FK/src/structural/retained/verify.rs:1010` (`n.sas = wmax(&n.sas_one, &n.sas_inf)`; R7 item 6: "the larger of", because Lemma C uses the 1-norm) | **Mutant RV19-M2** (`n.sas = n.sas_inf`) **survives all 111 tests.**<br>– On every verification state of every control, ‖SĀS‖₁ ≤ ‖SĀS‖_∞ after rounding. Ā's triangles differ only through the rounding of \|D\|B̄ (`assemble_bounded`'s mirrored `(b, a)` read, `assemble.rs:729-739`).<br>– It is not equivalent in general. For honesty it is equivalent within Lemma C's slack: the triangles' difference is a relative O(2^-P), and 2^(7−P) exceeds (67 + 63.5·2^(P−q_W))·2^-P by a factor of about 1.9 (§5). | Add an E-CHARGE vector, or an SD-G5-style searched block, on which the 1-norm exceeds the ∞-norm. Show RV19-M2 killed. |
| RV19-6 | SHOULD-FIX (design; for ROOT) | `DESIGN.md` §4.1.6, "The threshold's own rounding (revision 4, N-2)": "when S\* < 2^-988, every row of that body and kind is classified `absolute_verified`", with b = fl↑(2^-64·S\*), as K4 implements them (`adaptive.rs:338-383`, `threshold`, `absolute_bound`, `classify`); R7 §5.2, "Scope of b"; `K4T/models.rs:351-352` (`tiny`) | **The published-value bound fails wherever S\* < 2^-988.**<br>– R7 §5.2 derives \|q_pub − q\*\| ≤ b·(1 + 2^-22) from \|q_p\| < 2^-34·S\* for an absolute row. Under the 2^-988 rule a row can reach \|q\| ≈ S\*, where half an ulp (2^-53·\|q\|) is up to 2^11·b. A subnormal row adds up to 2^-1075 absolute, which exceeds 2^-23·b whenever b < 2^-1052.<br>– **Probe TINY-S-995** (`probes/probe_tiny.log`, `oracle/oracle_tiny.log`): one unit member (0,0,0)→(3,4,0), node 0 fixed, Fx = 2^-995 at node 1. It is selected at 128, and every force, moment and translation row is `absolute_verified` with b between 2^-1059 and 2^-1054.<br>– Seven rows (u.1.0, u.1.1, mag.1, end.1.i/j.0 and end.1.i/j.2) have \|q_pub − q\*\| between 222 and 819 times b·(1 + 2^-22). K4's values are the correctly rounded truths, so the retained claim on q_p holds and only the published interval [q_pub ± b] misses.<br>– The same model at 2^-900 is honest.<br>– `compare_honest` passes these rows because every comparison allows half an ulp of e. | ROOT to rule, since the formulas are pinned (§4.1.6.1, D2's G5). Options:<br>– at S\* < 2^-988, b := max(fl↑(2^-64·S\*), fl↑(2^-53·\|q_pub\|) + 2^-1074);<br>– state that b bounds q_p only, and have D2's interval add the publication rounding;<br>– keep the relative class where it is decidable.<br>Then add TINY-S-995 as a control, and drop `compare_honest`'s ½ ulp(e) for absolute rows. |
| RV19-N1 | NOTE | RETURN §22.1 item 1 | "(20g + 49)·2^-P·E_group,exact … fits λ = 2^8 (≤ 68g)" is false at g = 1 (69 > 68). Counted against Ā-based E, which carries g, the terms are:<br>– reactions ≤ 65.5 (Lemma B(i)'s 20g + 45.5 over g);<br>– the component sum, 1;<br>– the magnitude, 1.5 (not 2.5: the rounded sum of squares, halved by √, plus √'s own rounding).<br>That totals ≤ 68, and ≤ 69 with 2.5, so it fits the Corollary's 62·2^-2p·ê slack either way. Item 5's "to first order" is in fact exact (\|‖a‖ − ‖b‖\| ≤ ‖a − b‖). The derivation stands. | Correct the arithmetic in §22.1. |
| RV19-N2 | NOTE | RETURN §9 steps 11 and 12 | – Step 11 says "Step 6's uncertified step is gone from route 2". Step 6 is route 1's; route 2's uncertified step is in step 9.<br>– Step 12's ρ = ‖Ā‖/‖K‖ should read ‖Ā‖/‖\|K\|_contrib‖ if step 5's κ is already the componentwise condition over \|K\|_contrib (steps 2 and 4). The 9·g_max bound I re-derived (§2.3) bounds that ratio; ‖\|K\|_contrib‖/‖K‖ is not bounded by it.<br>Both are availability only. | Reword. |
| RV19-N3 | NOTE | `K4T/models.rs:352-375` | `compare_honest`'s relative allowance is dominated by \|q\|·2^-53 + \|e\|·2^-53: one to two binary64 ulps (\|x\|·2^-53 is an upper bound on half an ulp). So it cannot see a false claim smaller than about an ulp, or a publication misrounding (K4-M33 is caught by a bit-exact test instead). That is inherent in binary64 expectations. | Optionally, check the retained P-bit values (`state(p)`) against the exact truth, \|q_p − q\*\| ≤ 2^-64·M, at full resolution. |
| RV19-N4 | NOTE | `FK/src/structural/retained/wide_sum.rs:255-258` | If the span invariant were ever violated, the `index >= SUM_LIMBS` branch would `break` in a release build (the `debug_assert` is off) and drop bits silently. The span argument (RETURN §5, item 3) shows this cannot happen, and I agree. | Defence in depth: return `SumRefusal::Span` there instead. |
| RV19-N5 | NOTE | `FK/src/structural/retained/adaptive.rs:1693-1695` (a displacement magnitude's own 2^(1−2p)·\|q_2p\|) | **Mutant RV19-M3** removes the term. Only `golden_work_counts` kills it, at the work-count level, which any refactor would also trip. Its effect is at most 2^(1−2p)·\|q\| (2^-255 relative at 128), far below binary64 resolution. | Add a magnitude-row boundary vector to SD-G5, beside the translation row's. |
| RV19-N6 | NOTE | `K4T/models.rs:352, 361, 363, 373, 386`; `K4T/method_tests.rs:308-312, 870` | The tests use `2f64.powi(n)`. The results are exact for these powers of two, but that is the kind of dependence ROOT's Q7-reversed lesson excludes. | Use `f64::from_bits` constants. |
| RV19-N7 | NOTE | `K4T/directed_tests.rs:78-109`; `_run_records/c/results.jsonl` | – The test "a directed result is never on the wrong side" exercises only `div_toward` (and exact `mul`/`add`). My mutant RV19-M1 (`round_toward` keeps a low nearest value) passes it, and is killed by the Fraction-oracle test and eight others.<br>– K4's `results.jsonl` records its NONE-S11 control as `SURVIVED`: a label, not a finding. | None required. |

## 1. Honesty: the acceptance rule against R7 §5

### 1.1 The stop rule (a) to (d), as implemented (`adaptive.rs:1600-1832`, `rule`)

- **(a)** For every row, \|q_p − q_2p\| + V_q ≤ 2^-64·max(\|q_2p\|, S\*), decided exactly (`:1672-1726`).
  - V_q = ê·2^(8−P) for force and moment rows (`add_wide_scaled(e, 1, 8 − big_p)`, with big_p = P = 2p).
  - V_q = W⁺_q for the other rows that are not input-derived, plus 2^(1−P)·\|q_2p\| for a displacement magnitude. Input-derived rows carry no V; they are published from their exact terms (`publish_prescribed`).
- **(b)** For every force and moment row, W_q ≤ 2^(6−P)·ê (`:1728-1750`).
- **(c)** In R7's order, with ROOT's A3-0 Q7:
  - `uc`: a block with data and no B_c (`:1752-1756`);
  - θ_c > 1/2 on any block (`:1757-1763`);
  - the g check (`:1764-1767`).
- **(d)** C_q ≤ 60·2^-P·ê at P = 256 and 512 (that is, p = 128 and 256), and C_q ≤ 2^-86·M_q at P = 1024 (`:1768-1799`).
- **The floor.** At P = 1024, S\*(force) and S\*(moment) are floored by Φ = `phi_512(ê)` (`:1642-1656`). The same bits go to the classification through `decision.floor` (`finish_selected`, `:2811-2830`).
- **ê.** ê comes from E per body, rounded upward to binary64 (`resolution_scale`, `verify.rs:289-319`), and is coupled in binary64 (`e_hat`, `:321-327`). The single-source rule (S4) holds: the stop rule, V, (b) and (d) all use `hat()`, lifted exactly, and Φ uses the same ê.
- **It matches R7 §5.1 exactly, with one exception: which rows set S\*.** That is RV19-1.

### 1.2 The verification pass (`verify.rs:693-1190`) against R7 §4.1.6.3 items 1 to 12

- **Item 1.** r is one exact expansion per free row:
  - the exact ledger (`add_to`);
  - contributions formed at q_W (`build_verify_shared`: `form_members` at q_W, or the state's own entries at 1024);
  - the springs' binary64 stiffnesses;
  - the prescribed columns as their exact terms c·v (at q_W ≥ 448 bits, a product of two binary64 values is exact).

  The lower triangle is read through the upper entry's (a, b). That is exact, because K_e (`upper_index`) and the directional blocks (`block[b][a] = v`, `assemble.rs:501-502`) are stored symmetric. Nothing is reused from the gate.
- **Items 2 and 3.** δ̂ uses the state's factor. Ŵ is the recovery of δ̂ with an empty ledger.
- **Item 5.** r₂ is the same expansion minus K^c·δ̂.
- **Item 6.** Every norm is one exact expansion, rounded upward, and scaled by s_i = 2^scale. ‖SĀ\|u⁰\|‖ uses the exact prescribed value (the sign-adjusted \|t\| sum at `:900-906`). ‖ā_q S‖₁ is E's expansion with s at free DOFs, 0 at constrained ones, and upward stages (`:915-933`).
- **Items 7a to 7d, 9 and 10:**
  - the data flags follow 7a (`bound.rs:116-146`);
  - B_c = min(Uc_c, S_c) over the blocks with data;
  - a block with data and no bound is `uc_missing`;
  - θ_c = B_c·‖SĀS‖·2^(7−P), upward;
  - the g check's scope is item 10's (`:1021-1043`).
- **Items 11 and 12.**
  - t₁ = 2^(7−q_W)·B_b·N_u, with N_u = ‖SĀ\|u⁰\|‖ + 2·B_b·‖SĀS‖·‖S·r‖;
  - t₂ = 70·2^-P·‖S⁻¹δ̂‖;
  - t₃ = 3·B_b·‖S·r₂‖.

  Each term is formed by directed products, and the body's norms are the maxima over its blocks with data. C_q is one expansion, rounded upward. W⁺ = \|δ̂_i\| + s_i(t₁ + t₃), and a magnitude adds its constrained components' exact \|u_P,c − Σcv\| (`:1100-1148`).

  All of this matches R7. RV19-M7 (W⁺ without s_i) and RV19-M8 (θ at 2^(6−P)) are killed by E-CHARGE.

### 1.3 Directed rounding and the certified bounds

- **`directed.rs`.**
  - `round_toward` rounds to nearest, then steps once when the exact sign of (exact − nearest) shows the wrong side. `div_toward` decides with the exact sign of q·b − a.
  - The step is one P-bit unit, or half a unit below a power of two toward zero. It adds one exact term and rounds exactly.
  - `binary64_up` handles `Underflow` (the next value above 0) and `Overflow` (+∞, which then stops as `ResolutionScale`).
  - A result outside `Wide`'s range is `AttemptStop::Exponent`: terminal, never a wrong bound.
- **`bound.rs`.**
  - **The Uc passes.** `u_pass` and `nl_pass` implement 7b, every operation upward and monotone on nonnegative data. With `div_toward(…, Up)` by a positive pivot, c ≥ M(L)⁻ᵀD⁻¹M(L)⁻¹e.
  - **Uc_c.** It uses γ_m = m/(2^P − m) upward, with m = 2·(all free DOFs) + 2; t_c upward; 1 − t_c downward; and U_c/(1 − t_c) upward.
  - **The shifted loop** is `factor()`'s loop operation for operation (I compared the two). It uses d̃ = fl(K̃_ii − σ), the test d′ > 0, and a failed pivot replaced by 1. Block independence holds: cross-block products are exact zeros.
  - **The shift's terms.** δ_c = 2^(1−P)·max\|d̃\| is exact, and σ′ = σ − (γN′ + δ), with the bracket upward and the difference downward. S = ⌈√n_c⌉/σ′ upward, with `ceil_sqrt` exact.
  - **The profile.** The shifted profile is built from the verification's K, ordering and scales, exactly as `factor()` builds its own.
  - **The kills.** RV19-M1 (a wrong-side upward rounding), M4 (1 − t_c rounded upward) and M5 (δ_c halved) are each killed by E-UC, E-CHARGE, the F2 family and the low-precision stress.
- **The estimate serves availability only.**
  - est_c reaches the method only in the condition screen, in `shift_needed` and in σ_c = 1/(2·est_c) (`verify.rs:935-950`).
  - Lemma E holds for any σ > 0, and S_c exists only if every shifted pivot is positive. est never enters V, (b), θ, C or W⁺.
  - rcond is published as model information only. **Honesty never rests on the estimate.**

### 1.4 RV19-1, the false publication

- **Where the two scales part.**
  - O9 (ROOT's checkpoint-0 ruling) reads: unpublishable rows "are listed, excluded from S\* and the classification".
  - K4 applies it in `classify_rows_floored` (`:2167-2173`: rows with no binary64 value do not enter S\*_pub).
  - The stop rule's `scales_at` (`:1496-1544`) takes every row that is not input-derived, including a row whose candidate value overflows binary64. So S\*_2p can exceed S\*_pub by any factor.
  - R7 §5.2 states that S\*_pub lies below S\*_2p by at most a relative (2^-64 + 2^-52). That is false for such a case, and the relative and absolute claims both rest on it.
- **Why only translation and rotation.** An overflowing force or moment row makes E overflow, which stops the case with `ResolutionScaleUnencodable`. A displacement is not covered by E. With soft members (EA/L = 2^-100·√5) and a binary64 load, u can exceed 2^1024 while E stays near 2^931.
- **The construction** (`probes/rv19_probe.rs.txt`, `ovf_rot`). The load lies along the member, so the exact solution is pure extension and θ = 0.
  - At 128, the rounded e_x (1/√5, 2/√5) leaves a transverse residue of 2^-128·\|u\|. That gives Rz = −2^901 at 128 and 0 at 256.
  - The stop rule compares \|Δ\| = 2^901 with 2^-64·S\*_2p(rot) = 2^-64·(S_tr/L_b) ≈ 2^965, and accepts. V, (b), (c) and (d) pass: E ≈ 2^931 and θ ≈ 5e-73.
  - The publication drops u_x, u_y and \|u\| (`Overflow`), so S\*_pub(tr) = L_b·S\*_pub(rot) and S\*_pub(rot) = 2^901. Rz is classed `relative_verified` with claim 2^-64·2^901, against a truth of 0.
- **Reach.** The kernel API only: no product caller exists, and it needs displacements beyond 1.8e308 m. It is still a selected row outside its claim, which the brief classes BLOCKING.
- **The underflow side of the same gap.** An `Underflow` row inside S\*_2p can let other rows publish b = 0 over a nonzero truth below binary64's range (the PRESCRIBED-TAIL pattern). The probed fix removes this side too.

### 1.5 The independent oracle (`oracle/rv19_oracle.py.txt`)

- **What it is.** It is written from D1 §4.1's element (B, D, T, the Gram–Schmidt frame), the springs, the directional blocks and K4's documented published set (recovery, stations, reactions, groups and magnitudes), not from GEN.
  - It parses GEN's model files independently. It solves the intended system by Gaussian elimination in 1,600-digit decimal, lifting every binary64 input exactly.
  - It checks each published row against the claim that row carries:
    - absolute: b·(1 + 2^-22), or 2^-21 at 512;
    - relative: 2^-64·max(\|q\|, S\*_pub)·(1 + 2^-21) + 2^-53·\|q\|;
    - input-derived: q_pub is exactly the correctly rounded q\*;
    - `Underflow` and `Overflow`: the truth lies in their range.
- **Result:** 115 selected publications (109 cases and 6 combinations, RF-LARGE-10 included) and 8,086 rows are **all within their claims**. The worst is 0.949 of the allowance, at B1-C-A u.0.3. No unpublishable row is inconsistent.
- **Sensitivity:** it flags RV19-1 (FALSE, ratio 9.0e15) and passes OVF-ROT-900.
- **Independence.** It is independent of GEN's solver and of K4's schedule. It shares the model files with GEN, and D1's element definition with everyone.

### 1.6 RV19-6, the published bound below S\* = 2^-988

- **Two statements that do not meet.**
  - D1's revision-4 rule classes every row `absolute_verified` once S\* < 2^-988, because fl(2^-34·S\*) is no longer exact there.
  - D1 says acceptance bounds the candidate's error by 2^-64·S\*. R7 §5.2 extends that to the published value, \|q_pub − q\*\| ≤ b·(1 + 2^-22), by bounding the publication rounding with \|q_p\| < 2^-34·S\*.
- **Why they fail together.** Under the 2^-988 rule, that premise fails for every row with \|q\| ≥ 2^-34·S\*. Their binary64 rounding (up to 2^-53·\|q\|, or 2^-1075 in the subnormal range) can exceed b by up to 2^11.
- **The probe.** TINY-S-995 shows it on seven rows. K4's retained values there are exact to the last P bit (the published values are the correctly rounded truths), so this is not a K4 arithmetic error. It is a published interval that does not contain the truth.
- **The ruling it needs.** K4 follows the pinned formulas, so the ruling belongs to ROOT (D1 and D2).

## 2. The items ROOT directed to me

### 2.1 RETURN §6 item 4 (I12's reading): **confirmed**

- **Claim:** at a selected p, θ_c ≤ 1/2 with a certified B_c forces every data-carrying block's K\*_c to be nonsingular.
- **Derivation:**
  - Lemmas D and E give ‖K̃_c⁻¹‖₁ ≤ B_c; K̃_c is itself invertible (Lemma D's Neumann step, or Lemma E's λ_min > σ′ > 0).
  - Write S·K\*_c·S = K̃_c + E. Lemma B(i), under the g check (which covers every member with a free DOF in a data block), gives \|E\| ≤ 67·2^-P·SĀS ≤ 2^(7−P)·SĀS entrywise.
  - Then ‖K̃_c⁻¹E‖₁ ≤ θ_c ≤ 1/2, so I + K̃_c⁻¹E is invertible, and so is K̃_c + E = K̃_c(I + K̃_c⁻¹E). Since S is invertible, K\*_c is nonsingular, and positive definite because frames and positive springs give K\* ⪰ 0.
- **Contrapositive:** a singular K\*_c makes I + K̃_c⁻¹E singular, so ‖K̃_c⁻¹E‖ ≥ 1 and θ_c ≥ 1. A data-carrying mechanism is therefore rejected at every p, with `theta` or `uc`, and ends `Unresolved(Ceiling)` unless another stop comes first.
- **R7 already implies it.** Lemma C's statement, ‖(S·K\*_c·S)⁻¹‖₁ ≤ 2·B_c, presupposes the inverse, and its Neumann step proves that it exists.
- **Conditions:**
  - It inherits Lemma B's standing premise (V4's count against the Rust) and Lemmas D and E.
  - It applies only to blocks with data. The code rejects every such block without B (`uc_missing`) and tests θ on every block with B.
  - The no-data residue stays under R7's premise, as I12 says. A mechanism in a block with no data publishes exact zeros, a valid but non-unique solution. It is not detected there, and the pivot screen, which is global, may or may not stop it.

### 2.2 RETURN §22.1 (Q12, the support-group E): confirmed, with RV19-N1 and RV19-4

- **K4's form** (`verify.rs:245-274`) is one exact sum over every contributor of the three components, rounded once. It mirrors `recover.rs:420-451`'s component sums: the same `restrained[c]` gate and the same component match.
- **The requirements:**
  - **Resolution:** the count is ≤ 68 to 69 relative to E_group (RV19-N1), inside λ = 2^8 and the Corollary's slack.
  - **Direction:** nearest per stage, then upward to binary64 for E(body, kind), as R7 §3.1 prescribes.
  - **Zero rule:** E_group = 0 forces every contributor's E = 0. A structurally zero Ā_cj has K_cj = 0, and k > 0, so v = 0 and the published row is +0.
  - **Estimate and charge:** \|‖v(u\*)‖ − ‖v(u_P)‖\| ≤ ‖v_lin(δ̂)‖ + ‖v_lin(x̃ − δ̂)‖ + ‖v_lin(y)‖ by the triangle inequality. The first term is Ŵ_group. The others are ≤ Σ_c‖a\*_c S‖₁·(t₁ + t₃) ≤ ‖ā_group S‖₁·(t₁ + t₃), and the magnitude's own roundings sit inside t₂'s 70.
- **The derivation holds.** The directional-contributor path is untested (RV19-4). My two probes of it are honest.

### 2.3 RETURN §22.2 (the certified bound above 40 DOFs): confirmed

- **`inv_norm1_upper`** (GEN `:2877-2937`):
  - X ≈ K̃⁻¹ in fixed point; its accuracy is irrelevant to the bound;
  - ‖X‖₁ is an exact rational;
  - R = I − K̃X is formed exactly from K̃'s dyadic entries, scaled to integers;
  - ‖R‖₁ < 1 and ‖R‖₁ < 2^-100 are asserted;
  - the bound emitted is ‖X‖₁/(1 − ‖R‖₁). K̃X = I − R gives K̃⁻¹ = X(I − R)⁻¹, so that is an upper bound on ‖K̃⁻¹‖₁.
- **The matrix is the right one.** It is GEN's emulated K̃_P, restricted to the block's elimination rows (`:2986-2994`). The `bnd` line's factor digest pins it to the Rust.
- **The assertion.** E-UC asserts Uc_c, S_c and B_c ≥ that bound exactly (`support::wide_at_least`; `scale_tests.rs:343-347`). The 40-DOF threshold only chooses the exact inverse or this bound.

### 2.4 RETURN §9 steps 10 to 12: confirmed, with RV19-N2

- **Step 10** restates the ceiling as R7 §6.6 and the Corollary do:
  - V charges 1024's resolution;
  - W's residual is over the 1024-formed contributions;
  - C ≤ 2^-86·M holds under θ ≤ 1/2, since Lemma C's P = 1024 case uses the single assembly rounding;
  - b = fl↑(2^-64·Φ) with 1 + 2^-22.

  "The factor 4" is right: 2^8·2^-512·ê against 2^-64·2^-438·ê.
- **Step 11** is right that neither route is now an honesty step. It mislabels which route step 6 belongs to (RV19-N2).
- **Step 12's constant, re-derived.** In the bounded row, B̄_ra = 1 (or 1/L) ≤ Σ over the 3-group of \|B_ra′\| (a unit vector's 1-norm is ≥ 1, the 1/L rows alike). So each column of g·B̄ᵀ\|D\|B̄ is ≤ 3g·Σ over the 3-group of \|K_e\|_contrib's columns, and ‖Ā‖₁ ≤ 9·g_max·‖\|K\|_contrib‖₁. The springs and directional blocks are equal in both. This is availability only, as ROOT recorded.

### 2.5 RETURN §22.9 (the tightened predicate)

- **What is right:**
  - **Direction:** each row is checked against the claim it publishes.
  - **Absolute rows:** b·(1 + 2^-22 or 2^-21) plus half an ulp of e.
  - **Relative rows:** the stop rule's bound plus the publication rounding.
  - **Input-derived rows:** half an ulp.
  - **The derived keys:** N and T duplicate `end.m.j.0` and `.3`, and take their claims. Mb and Mbs take the sum of the two claims plus 2^-50·\|v\|, which is valid (\|‖a‖ − ‖b‖\| ≤ Σ\|a_k − b_k\|, and the binary64 √(y² + z²) needs a few ulps).
- **How loose it is.** For absolute rows it is tight wherever \|q\| < 2^-34·S\*, since half an ulp of e is then below 2^-23·b. It is not tight under the S\* < 2^-988 rule, where that half ulp hides RV19-6. Relative rows are limited to binary64 resolution (RV19-N3).
- **The gaps (RV19-2).**
  - Unpublishable rows are skipped (`:367-369`), with no range check.
  - Expected keys that are not published are skipped (`:405-407`), and so is a derived key whose operand is unpublishable.
  - `compared > 0` suffices.
  - Selected controls with empty expectations are skipped (`method_tests.rs:697`).

  My coverage probe (`probes/probe_coverage.log`) measured each of these (RV19-2's evidence).
- **Can `compared` pass with rows missing?** Yes. Today the only missing rows are the 33 `Underflow` rows, whose truths my oracle confirms lie below 2^-1075.
- **The evidence pass.** `_run_records/c/evidence_tight.jsonl` shows what RETURN says:
  - R7-M1 catches ASSEMBLY-SAT at R.1.0 (9.0e15), F-2 and F-2-CEIL at N.1, and F-2-SPOS at N.1 (16.0);
  - K4-M24 catches CEIL5A3 at N.3 (670);
  - K4-M37 catches all eight;
  - NONE's only entries are N02, N03-RZ and N04, the refusals.

## 3. Kernel only (`checks/kernel_only_scan.txt`)

- **Visibility.**
  - `mod retained;` is private (`FK/src/structural.rs:5`).
  - No item in K4's twelve files is `pub` without `(crate)`.
  - `FK/src/lib.rs` has `pub mod structural;` and re-exports nothing from `retained`. `structural.rs`'s `pub use` lines are `formation_check` and `sparse`.
- **Callers.**
  - Inside FK, only `formation_check.rs:32` names `retained`, and it uses K3a's `wide`, which predates K4.
  - No code outside FK names a K4 module or `net_parts`.
  - `net_parts` is called only by `ledger.rs:33` and its tests.
- **`exact_sum.rs`.** The diff is 19 added lines: `net_parts(&self)`, built from the file's own `compare` and `subtract`, with no other line changed. So it changes no behaviour, and `exact_sum`'s tests pass in the FK suite.
- **The write set.**
  - No manifest or lockfile changes; the only `Cargo.toml` names in the diff are log files under `_run_records/b/suites/`.
  - Outside `retained/`, `retained_k4/` and the records, the only changes are `exact_sum.rs` and `s11_site_table.rs` (both declared).
  - K3a's and K3's files are untouched.
- **The FK build.** FK's non-test library builds with `RUSTFLAGS=-D warnings` (`checks/build_warnings.log`). rustfmt `--check` is clean on K4's files, `exact_sum.rs` and the site table.

## 4. Arithmetic correctness

- **`wide_sum.rs`.**
  - Terms are placed exactly at an integer offset above the anchor, and the span check precedes any mutation.
  - `shift_up` preserves the value within the checked span. Carry headroom is 64 bits, enough for fewer than 2^64 terms.
  - Netting is one compare and one subtract; the single rounding is K3's `from_integer`; an exact zero and an empty sum give +0.
  - `add_scaled` and `add_product_of` are exact or refuse.
  - I agree with RETURN §5's argument. RV19-N4 is defence in depth only.
- **The ledger and assembly.**
  - The ledger is exact per DOF and enters every sum through `add_to`. The data flag uses "any nonzero term", which is conservative.
  - Assembly and recovery are one exact expansion per entry or component.
  - The combination's prescribed rows are published from their exact terms (`exact_publication`).
- **Recovery before rounding.** Each published row is rounded once from its P-bit value, and an exact zero publishes as +0.0 (`publish_value`).
- **The oracle.** It agrees with every selected row (§1.5), including stations, spring and directional spring actions, reactions, and group and node magnitudes.

## 5. The generator and the vectors

- **The check.** `gen_k4_vectors.py --check` from the clean archive gives 23 of 23 OK (the 22 vector files and SHA256SUMS), in 13 min 36 s with two cargo jobs running beside it (`checks/gen_check.log`). The pinned inputs' sha256 assertions pass. No `__pycache__` was written.
- **GEN against R7's emulation** (`checks/emu7_vs_gen.txt`). On all 59 of R7's controls, GEN's selected precision and its sequence of attempt outcomes and rejection reasons equal emu7's (run_controls7's R6 column, which R7 records as identical to R7's). Spot checks:
  - SKEW6-K1E-12 rejects at u.0.3 (layout index 3);
  - LEVER2 rejects by `verification_estimate` and then `stop_rule` at end.1.i.2 (index 37);
  - THETA-STUB-COUPLED rejects by `theta`, and G-PRESC-MEMBER by `g_validity`.
- **Where they are independent:** the authors (GEN is I12's, K4's author; emu7 is DS1's), the implementations (GEN rounds with Fractions at every P rounding; emu7 keeps the norms, θ and C as exact Fractions), and GEN's exact reference solves.
- **Where they are not:** both emulate K4's Rust order and loops (RCM, the factor loop, the estimate's indexing), both implement R7's text, and they share the control models (DS1's, ported). They are two readings of one specification, not an independent physical check. §1.5's oracle supplies that.

## 6. Mutants (`mutations/`)

**The harness.** Each run uses a clean `git archive 7d8fa9c0e` of FK and a fresh target, and applies exact string edits with their counts asserted. It runs every `structural::retained::` test except the N5 streams, the sum differentials and K3's own (111 tests). The NONE control passes 111 of 111.

| Mutant | Edit | Result |
|---|---|---|
| RV19-M1 | `directed.rs`: `round_toward(Up)` keeps a nearest value below the exact one | killed (9 tests: the directed Fraction oracle, E-UC, E-CHARGE, the F2 family, the stress, SD-G5's profiles, golden work) |
| RV19-M2 | `verify.rs:1010`: ‖SĀS‖ = ∞-norm only | **survives** (RV19-5; honesty-equivalent within Lemma C's slack, not in general) |
| RV19-M3 | `adaptive.rs:1693-1695`: no 2^(1−2p)·\|q_2p\| for a magnitude | killed only by `golden_work_counts` (RV19-N5) |
| RV19-M4 | `bound.rs:391`: 1 − t_c rounded upward | killed (7 tests: E-UC, E-CHARGE, the F2 family, the stress, golden work) |
| RV19-M5 | `bound.rs:744`: δ_c = 2^-P·max\|d̃\| | killed (8 tests) |
| RV19-M6 | `verify.rs:268-270`: a support group's E without directional contributors | **survives** (RV19-4; not equivalent) |
| RV19-M7 | `verify.rs`: W⁺ adds t₁ and t₃ without s_i | killed (E-CHARGE, RF-LARGE E-CHARGE, golden work) |
| RV19-M8 | `verify.rs:1018`: θ with 2^(6−P) | killed (E-CHARGE, RF-LARGE E-CHARGE) |

**RV19-M2's equivalence, derived.**
- Lemma C needs ‖K̃_c⁻¹E‖₁ ≤ 1/2, with \|E\| ≤ (67·2^-P + 63.5·2^-q_W)·SĀS (≤ 68·2^-P·SĀS at p = 512, where q_W = P).
- With the ∞-norm, θ_∞ = B·2^(7−P)·‖SĀS‖_∞. The 1-norm and the ∞-norm of SĀS differ only by the asymmetry of Ā's two triangles, a relative O(2^-P) per entry.
- So ‖K̃_c⁻¹E‖₁ ≤ B·68·2^-P·‖SĀS‖₁ ≤ (68/128)·θ_∞·(1 + O(2^-P)) < 1/2 whenever θ_∞ ≤ 1/2.
- The guarantee survives. The text's "the larger of" is still what R7 specifies, and nothing tests it.

**K4's own campaign** (`_run_records/c/results.jsonl`, 88 records):
- NONE passes; NONE-S11 is recorded as "SURVIVED", which is its control label (RV19-N7);
- the other 86 are killed, the four derivation guards among them (each at evidence or unit level, as RETURN §17 says);
- this matches RETURN §17.

## 7. Records

- **Checksums** (`checks/records_check.txt`).
  - `IMPLEMENTATION/K4/_run_records/SHA256SUMS` verifies: 214 of 214, covering every other file in the folder. Its own sha256 is `f79cfe57…`, as ROOT recorded.
  - `K4T/SHA256SUMS` verifies: 22 of 22.
  - Scans of the records, K4's sources and its tests find no home, temp or tool paths, no user or host names and no model identifiers.
- **RETURN's claims against the records:**
  - **The work tables (§14)** equal the pinned `golden_work_counts` and `A3B_WORK`. Checked on N05, N06, TWO-SPAN and SKEW6-K1E-12: own, K4-sum, stop-rule, verification-pass, verification-shared, scale, estimate, charge, bound, shift, refinement, residual formation, formation and condition.
  - **The mutation table (§17):** as §6 above.
  - **The controls:** 131 cases and 4 combinations in `outcomes.txt`; "99 controls, 5,490 rows, worst 0.28" is `d/honesty.log`'s line.
  - **The references (§12):** the tallies are asserted by `references_tests.rs`, which pass in my FK run.
  - **The evidence pass (§22.9):** as §2.5 above.
  - **The one overstatement** is RV19-2's "every selected control" and "RF-LARGE honest under the tightened predicate and G5a".
- **FK's full suite** from the clean archive (`checks/fk_full.log`): 389 passed, 0 failed. That is the lib's 323 in 710.2 s at 2 threads, with K4's 122 among them, plus 66 integration tests, the S11 site table's 3 included.

## 8. What I ran, and what I did not

**Ran:**
- FK's full suite once (389);
- the FK library build with `-D warnings`;
- rustfmt `--check`;
- `gen_k4_vectors.py --check`;
- RV19's probe module in a copy of the archive:
  - OVF-ROT;
  - the dump of every selected publication;
  - compare_honest's coverage;
  - the station combination;
  - the support groups;
  - the tiny-scale body (RV19-6);
- the independent oracle (Python standard library, 1,600-digit decimal);
- the RV19-1 fix probe (118 K4 tests, 45 of K3's and 3 probes pass; OVF-ROT withheld);
- NONE and eight mutants;
- the GEN-against-emu7 comparison;
- the merge and kernel-only checks.

**Git and GitHub:** `git archive`, `show`, `diff`, `log`, `grep`, `merge-base`, `rev-parse` and `ls-remote`, and `gh pr view`. All are read-only; there were no writes and no index operations.

**Not done:**
- the 39-manifest profile, T9, DEC-025 and the hosted CI (ROOT's);
- RF-LARGE at 100 members in my oracle (dense 600-DOF decimal solves), which is the reason it is in RV19-2;
- RF-LARGE-1000;
- a timing claim (none is made).

**Deleted at the end:** my archive copies (`<wt>/rv19/`) and every RV19 target.

## Delta check at a5fa0eaf7

- **Reviewer:** RV19, on ROOT's request.
- **Head:** `a5fa0eaf7c26837ec18e2e9b16f08498f0cc4606` on `codex/piping-k4-20260928`. Its parent is `7d8fa9c0e`, the head reviewed above.
- **Basis:** "K4: rulings on RV19's review" in `ROOT_RULINGS_V1.md`, numerics head `e9f8ebe1d`, including D1 revision 5a.3 amendment A1. I12's account is RETURN addendum 1, CHANGE_RECORD's addendum and `_run_records/rv19/`.
- **Date:** 2026-09-29.
- **Verdict: PASS.** 0 BLOCKING, 1 SHOULD-FIX, 4 NOTEs.

**Summary.**
- **RV19-1 is closed.** It is closed on both the overflow and the underflow side, and the "stricter only" argument holds.
  - `scales_at` is the stop rule's only S\* source. OVF-ROT-928 is now withheld, and OVF-ROT-900 is unchanged.
  - The reverted fix is killed by the controls test at OVF-ROT-928.
- **Amendment A1 holds.** I re-derived it independently, including a subnormal S\*. `row_bound` implements it:
  - the exact sum is rounded upward once;
  - it applies only where 0 < S\* < 2^-988;
  - b = 0 stays at S\* = 0.

  The reverted amendment is killed at TINY-S-995 (819.2 at end.1.i.2). On every publication selected at both heads, the only bounds that move are PT-B's (26 rows) and PTF-B's (45). No value, class or scale changes, and TINY-S-995 is new.
- **RV19-2's expectations are sound and tight.**
  - GEN's 128-bit tokens agree with my 1,600-digit oracle on all 28,735 keys of 138 models and combinations, the RF-LARGE 100-member frames included, within the slack `compare_honest` allows.
  - That slack is at most 2^-74 of any row's allowance.
  - `compare_honest` now fails on a missing expectation and range-checks unpublishable rows.
  - The 13 controls and RF-LARGE-100 are covered.
  - My oracle finds all 120 selected publications (8,272 rows) and the six 100-member frames (15,378 rows) within their claims. Its worst ratios equal K4's.
- **RV19-3, 4 and 5 are fixed.** RV19-M6, RV19-M2 and my RV19-D3 are killed.
- **One test gap remains (RV19-D4).** Keying the skip on the verification value instead of the candidate's survives, although K4's code is right.

### D.1 Read, ran, Git

- **Read:**
  - the ruling section;
  - RETURN addendum 1 (A1.1–A1.8) and the corrected §9, §12.6, §22.1, §22.8 and §22.9;
  - CHANGE_RECORD;
  - the full source diff: `adaptive.rs`, `combine.rs` and `wide_sum.rs`;
  - the test diffs: `models.rs`, `method_tests.rs`, `recover_tests.rs`, `references_tests.rs`, `combine_tests.rs`, `classification_tests.rs`, `verify_tests.rs` and `support.rs`;
  - GEN's diff: `solve_hp`, `x_token`, `range_marker`, `expectation_lines`, `row_bound`, `scales_at_em` and `models_rv19`;
  - I12's `_run_records/rv19/` results and evidence.
- **Ran**, from a clean `git archive a5fa0eaf7` under `<wt>/rv19/`, with one cargo job at `-j 4` and `RUST_TEST_THREADS=2` while ROOT's sweep ran:
  - FK's full suite, 392 of 392 (the lib's 326 in 684 s), with K4's 125 among them;
  - the FK library build with `-D warnings`, and rustfmt `--check` on K4's files;
  - RV19's probe module on a copy: OVF-ROT, the station combination, the coverage probe, the dump of every selected publication, the groups, TINY-S and the RF-LARGE 100-member frames;
  - my oracle, plus two new scripts: `rv19_xcheck.py`, GEN's tokens against my solve, and `rv19_slack.py`, the slack δ against each allowance;
  - NONE and eight mutants, each from a clean archive with a fresh target.
- **Git and GitHub:** `git archive`, `diff`, `show`, `grep`, `log`, `rev-parse` and `ls-remote`, and `gh pr view`. All are read-only.
- **The PR:** OPEN and MERGEABLE at `a5fa0eaf7`; hosted checks are 12 SUCCESS and 4 SKIPPED.
- **Records:** `REVIEW/_run_records/k4_review/delta/`. `k4_review/SHA256SUMS` is regenerated and now covers 80 files.

### D.2 Findings at the delta

| ID | Class | Site | Evidence | Fix |
|---|---|---|---|---|
| RV19-D4 | SHOULD-FIX | `FK/src/structural/retained/adaptive.rs:1678-1681` (the skip keyed on `candidate`) | **Mutant RV19-D4** keys the skip on `verification` instead, and **survives all 114 tests it runs.**<br>– The two keys differ only when a row's p and 2p values straddle binary64's overflow or underflow threshold. An accepted candidate agrees with its verification within 2^-64, so that needs a value within 2^-64 of the threshold.<br>– In that case the mutant reopens RV19-1's gap for that row: S\*_2p keeps a row that S\*_pub drops. So it is not equivalent.<br>– K4's code is right, and it matches the classification, which drops rows by their published, that is candidate, value. | Add an SD-G5-style vector on `decide`: a row whose candidate overflows while its verification does not (and the underflow pair). Assert that the row sets no S\*. Show RV19-D4 killed. |
| RV19-DN1 | NOTE | GEN `range_marker` (`gen_k4_vectors.py:1148`); `K4T/models.rs:39, 97-99` | – GEN marks three keys of RF-LARGE-TREE-n00010-ROT (T.6, end.6.j.3, end.6.j.4) `underflow`. Their tokens (about 2^-1983) lie far below their own `err` (2^-783), and my solve gives 0 to 1,600 digits. The marker's "nonzero exact value" is noise there.<br>– `compare_honest` never reads `Exact::range`: `range_consistent` decides by \|x\| and δ, which admit an exact 0. So no check is affected. | Emit the marker only when \|x\| > 2^err, or drop the unused field. |
| RV19-DN2 | NOTE | RETURN.md:761 ("Every selected control and combination satisfies the new checks") | `models.txt`'s combination PRECISION-RULE (SKEW-K1E-28-AXIAL + SKEW-K1E-28) is selected at 256. No K4 test names it, and it has no expectations. My oracle finds it honest (worst 0.32 of its allowance). The scoped statements (RETURN:7, CHANGE_RECORD:70, "of the controls test (117)") are accurate. | Scope RETURN:761, or check PRECISION-RULE. |
| RV19-DN3 | NOTE | `K4T/models.rs:423-443` (`claim_ratio`) | The check is refutation-style: max(0, \|q − x\| − δ) ≤ a. A pass therefore bounds \|q − q\*\| by a + 2δ. I measured δ/a ≤ 2^-74 on all 23,589 published rows with values (`oracle/slack*.log`), and no row has a = 0 with δ > 0, so the difference is immaterial. | None. |
| RV19-DN4 | NOTE | GEN `expectation_lines` (`err = 4·\|x₃₀₀ − x₂₄₀\|`) | `err` is an estimate, not a certified bound. My 1,600-digit solve confirms \|x − q\*\| ≤ 2^-127·\|x\| + 2^err on every key, the irrational models included (err ≤ 2^-397 of the model's largest value on RF-LARGE-TREE-n00010-ROT, and ≤ 2^-761 on the 100-member ROT frames). | None: it is stated. |

### D.3 RV19-1 (check 1)

- **The fix.**
  - `rule` builds `skip` from the candidate: nonzero, and `to_binary64().value()` is none (`adaptive.rs:1678-1681`).
  - `scales_at` (`:1537`) leaves those rows out, as `classify_rows_floored` leaves out rows with no binary64 value.
  - For every row that is not input-derived, "candidate unpublishable" is the same set as "published value none", because `publish_value` gives +0.0 for an exact zero and otherwise `to_binary64`. So both S\* are formed from the same rows, on the overflow and the underflow side alike. R7 §5.2's premise, S\*_pub within a relative 2^-64 + 2^-52 of S\*_2p, holds again.
  - In the subnormal range the gap is absolute (≤ 2^-1075, which A1's 2^-1074 term covers; D.4).
- **Only source.** `scales_at` has one production caller (`:1682`); its two other callers are tests (`checks/revisions.txt`). In `rule` the scales feed:
  - M_q of (a);
  - the 1024 allowance 2^-86·M_q of (d);
  - the Φ floor;
  - the evidence ratios.

  V, (b), `uc`, θ and g do not read S\*, and the classification forms its own S\*_pub.
- **Stricter only: confirmed.**
  - Each S\* is a maximum over a subset of the previous rows, and Φ is unchanged. So every allowance of (a) and (d) can only shrink, and the fixed rule accepts only what the old rule accepted.
  - A skipped row still meets (a) against its own \|q_2p\|.
  - My earlier fix probe and I12's full run agree: no control moves.
  - My publication diff over all 115 publications selected at both heads (`checks/publication_diff.txt`) shows no value, class or scale change.
- **The controls.**
  - **OVF-ROT-928:** 128, 256 and 512 are rejected by the stop rule (at u.1.2, layout index 8), and it ends `Unresolved(Ceiling)`, as GEN has it.
  - **OVF-ROT-900:** selected at 128. It publishes the spurious Rz as `absolute_verified` with b ≈ 1.3e282, and it is honest under my oracle.
- **The reverted fix, RV19-1R,** is killed by `every_control_follows_gens_schedule…`: OVF-ROT-928 is selected at 128 against GEN's withholding.

### D.4 Amendment A1 (check 2)

- **Derivation, checked independently.** |q_pub − q\*| ≤ |q_pub − q_p| + |q_p − q\*|.
  - **Rounding to nearest.** For a normal result, the half-spacing of the binade that holds fl(x) is at most 2^-53·|fl(x)|. This holds at a binade boundary too: rounding up to 2^e from below costs at most 2^(e−54). So |q_pub − q_p| ≤ 2^-53·|q_pub|. For a subnormal result the bound is 2^-1075. A nonzero q_p that rounds to zero is `Unpublishable` and carries no bound.
  - **The candidate.** In a body and kind with S\* < 2^-988, M_q = S\*_2p. The accepted candidate gives |q_p − q\*| ≤ 2^-64·S\*_2p, within R7 §5.2's factor.
  - **S\*_2p against S\*_pub.** With RV19-1's fix, S\*_2p exceeds S\*_pub by at most a relative 2^-64 + 2^-52 plus 2^-1075 absolute (subnormal publication and the binary64 coupling). 2^-64 of that absolute part is below the 2^-1074 − 2^-1075 that b_row's last term leaves spare.
  - **So** |q_pub − q\*| ≤ b_row·(1 + 2^-22), or (1 + 2^-21) at 512. Where S\* ≥ 2^-988, an absolute row has |q| < 2^-34·S\*, and a subnormal row's 2^-1075 is ≤ 2^-23·b, since b ≥ 2^-1052. So the plain b holds there, and b = 0 holds at S\* = 0. Relative rows are normal numbers.
  - The derivation holds.
- **The code** (`row_bound`, `adaptive.rs:376`):
  - it returns `absolute_bound` when S\* = 0 or S\* ≥ 2^-988;
  - otherwise fl↑(2^-53·|q|) is `q / 2^53`, stepped up when the exact scaling back falls short. That is exact, including into the subnormal range, where it yields 2^-1074 for any nonzero q below 2^-1022;
  - the three terms go into one `ExactWideSum`, rounded upward once by `directed_ratio(…, Up)` over 1. That is the least binary64 ≥ the exact sum;
  - the unreachable fallback is +∞, never a low bound;
  - `classify` passes it the published binary64 q and the classification's S\* (coupled, floored at 512).
  - The unit test's five pinned values match my hand computation (`0x4008001`, `0x8001`, `0x200400001`, and the two plain cases).
- **The mutants:**
  - A1R (reverted) is killed by the controls test at TINY-S-995 (819.2 at end.1.i.2), the A1 unit test, classification set 19 and item 6a's test. **So TINY-S-995's 128-bit expectation is precise enough.**
  - RV19-D1 (A1 without its 2^-1074 term) is killed by the A1 unit test, item 6a and the classification vectors.
  - RV19-D2 (fl(2^-53·|q|) to nearest) is killed by the classification vectors (set 19).
- **The bounds that move.** My publication diff shows 26 PT-B rows and 45 PTF-B rows, and nothing else among the 115 publications selected at both heads. TINY-S-995 is new; K4's controls test prints its 24 rows.
- **My oracle's view.** It checks absolute rows against b·(1 + 2^-22) with no publication allowance. TINY-S-995 is now within its claims (worst 0.726). It was 819× at `7d8fa9c0e`.

### D.5 RV19-2 (check 3)

- **The expectations.**
  - `x:` is the exact value rounded once to 128 bits: an exact rational solve where the geometry allows it, otherwise `solve_hp` in 300-digit decimal, with `err` from a 240-digit re-solve.
  - The markers flag values outside binary64's range.
  - **Soundness, checked independently** (`oracle/xcheck*.log`). For all 28,735 keys of 138 models and combinations, my 1,600-digit solve satisfies |x − q\*| ≤ 2^-127·|x| + 2^err.
    - That covers every exact and every `solve_hp` model: HH-FOOL, HH-SLENDER-m40, N03-RX, R115-SEED3, OVF-ROT, and RF-LARGE's 10- and 100-member AX and ROT frames.
    - Every marker agrees with the truth's binary64 range, except the three noise markers of RV19-DN1.
  - **Independence.** GEN is K4's author's tool, but its solves are independent of K4's Rust arithmetic, and my oracle is independent of both. They share only the model files and D1's element definition.
- **`compare_honest`** (`models.rs:489`):
  - it walks every published value, the derived keys included: no claim or no expectation is ∞;
  - it range-checks every unpublishable row (`range_consistent`: `Underflow` needs |x| − δ ≤ 2^-1075, and `Overflow` needs the same sign with |x| + δ reaching 2^1024 − 2^970);
  - it fails any expectation that is neither a row nor a derived key;
  - the absolute allowance no longer adds half an ulp of e;
  - the controls test asserts that every selected run has expectations, that `compared` equals rows plus derived keys, and that `honest` equals the number selected (117). `honest_large` does the same with G5a for the six 100-member frames.
- **The evidence pass** (I12's `evidence.jsonl`) catches R7-M1's PRESCRIBED-TAIL and PRESCRIBED-TAIL-FREE (∞, N.1), as ROOT asked. The earlier catches stand.
- **"Every selected control" statements.** Scoped, they match the tests. The one unscoped sentence is RV19-DN2.

### D.6 RV19-3, 4 and 5 (check 4)

- **RV19-3.** `combine.rs:99-106` also compares `stations()` and `supports()`: canonical, sorted by id, and whole structures, so fractions, members and group contents all count.
  - My station probe is now withheld (`OperandsDiffer`), and the new test covers the station and group cases.
  - RV19-D3 (the groups dropped from the check) is killed by that test.
- **RV19-4.** GROUP-DIR and GROUP-DIR-X join GEN, E-UNIT, E-UC, E-CHARGE and E-ESTIMATE. RV19-M6 is killed by E-UNIT and E-CHARGE. Both are honest under my oracle.
- **RV19-5.** `the_sas_norm_is_the_larger_of_the_one_and_infinity_norms` raises each transposed entry of N05's Ā at 256 by 2^20, requires at least one block where the 1-norm is the larger, and asserts `sas` = the larger. RV19-M2 is killed by it.

### D.7 The NOTEs and new issues (check 5)

- **N1 and N2** are corrected in RETURN §22.1 and §9, correctly.
- **N4.** `wide_sum.rs:255-263` now returns `SumRefusal::Span` when a bit or a carry would be lost. The sum is left partly updated, but every caller propagates the refusal and ends its attempt. It is unreachable under the span argument.
- **N6.** No `powi` remains in the tests except the source scan's own word list and its lexer control. rustfmt is clean, and the non-test build has no warnings.
- **N3, N5 and N7** are recorded.
- **New issues from the diff:** none BLOCKING.
  - The skip, `row_bound` and the combination check are correct.
  - `row_bound`'s fallback is +∞, a safe bound.
  - The write set holds: no file outside `retained/`, `retained_k4/` and the K4 records, no manifest or lockfile, no new `pub` item, and `structural.rs`, `lib.rs` and `exact_sum.rs` are untouched.

### D.8 Records (check 6)

- **Checksums.** `IMPLEMENTATION/K4/_run_records/SHA256SUMS` verifies: 237 of 237, covering every other file, and its sha256 is `8c8423b9…`. `K4T/SHA256SUMS` verifies: 22 of 22.
- **The changed-files record.** `rv19/changed_files.txt`'s 27 sha256 match the files at `a5fa0eaf7`.
- **Scans.** There are no home, temp or tool paths, no user or host names and no model identifiers.
- **The figures agree with I12's records:**
  - K4's claims line, "117 controls, 9,412 rows, worst 0.949 (B1-C-A u.0.3)";
  - A1's list, "PT-B 26, PTF-B 45, TINY-S-995 24";
  - RF-LARGE-100's worst ratios, which equal my oracle's to four figures (0.9034, 0.9171, 0.8850, 0.9596, 0.9608 and 0.9717).

### D.9 Status of the findings at 7d8fa9c0e

| ID | Status at `a5fa0eaf7` |
|---|---|
| RV19-1 (BLOCKING) | **Resolved** (D.3). |
| RV19-2 | **Resolved** (D.5); RV19-DN2 scopes one sentence. |
| RV19-3 | **Resolved** (D.6). |
| RV19-4 | **Resolved**: RV19-M6 is killed. |
| RV19-5 | **Resolved**: RV19-M2 is killed. |
| RV19-6 | **Resolved** by amendment A1 (D.4). D2's G5 of b_row is routed. |
| RV19-N1, N2, N4, N6 | Corrected or fixed. |
| RV19-N3, N5, N7 | Recorded. N3 is partly met: the claims are now checked at 128-bit resolution. |

**Deleted at the end:** my delta copies (`<wt>/rv19/`) and every RV19 target.

## Confirmation at 5a46a6278

- **Reviewer:** RV19, on ROOT's request.
- **Head:** `5a46a6278af3c857a52ecb9be6880990493190f8`. Its parent is `a5fa0eaf7`. It closes RV19-D4 and RV19-DN2, recorded as K4's RETURN and CHANGE_RECORD addendum 2.
- **Date:** 2026-09-29.
- **Verdict: PASS.** There are no findings.

**The checks** (records: `REVIEW/_run_records/k4_review/final/`; `k4_review/SHA256SUMS` regenerated, now 93 files):

1. **No source change** (`revisions.txt`).
   - `git diff a5fa0eaf7 5a46a6278` changes 15 files, none under a `src/` directory: `K4T/{method_tests.rs, combine_tests.rs, gen_k4_vectors.py, models.txt, SHA256SUMS}` and the K4 records.
   - `models.txt` changes by one hunk after `combo PRECISION-RULE`: 36 added `expect` lines, and nothing removed.
2. **The new vector covers both straddles, and RV19-D4 is killed.**
   - `sd_g5_a_row_the_candidate_cannot_publish_sets_no_s_star` runs `decide` on four pairs:
     - candidate at the overflow threshold, verification just below;
     - the swap;
     - candidate at 2^-1075, which underflows, with verification at 2^-1075 + 2^-1200 (through the rotation coupling at L_b = 2^-60);
     - the swap.
   - For each pair I derived the verdict under the candidate key and under the verification key, and each turns over.
   - The runs used clean `git archive 5a46a6278` copies with fresh targets and one cargo job:
     - **NONE** passes 116 of 116;
     - **RV19-D4** is killed by the new vector alone, at its first, overflow, verdict;
     - **RV19-D4u** is RV19-D4 with the vector's two overflow verdicts removed from the test. It is killed by the underflow verdicts alone, so both halves are load-bearing.
3. **PRECISION-RULE.**
   - My cross-check finds its 36 tokens (32 rows plus N, T and the two Mb keys) equal to my 1,600-digit solve of the combination within the tokens' slack (`xcheck_precision_rule.log`).
   - K4 selects it at 256. My oracle finds it within its claims (worst 0.32), and all 120 publications at this head as at `a5fa0eaf7` (`oracle_all.log`).
   - `precision_rule_is_selected_at_256_and_honest` and `check_combo` (B1-C, B1-E) now run `compare_honest`, which fails on any unexpected or missing row and asserts every row checked, and G5a.
   - RETURN's addendum-1 status line is scoped.
4. **The records** (`records_check.txt`).
   - `IMPLEMENTATION/K4/_run_records/SHA256SUMS` verifies: 244 of 244, covering every other file; its sha256 is `8a324713…`.
   - `K4T/SHA256SUMS` verifies: 22 of 22.
   - `rv19d/changed_files.txt`'s 7 sha256 match.
   - There are no home, temp or tool paths, no user or host names and no model identifiers.

**Git and GitHub:** read-only (`archive`, `diff`, `log`, `ls-remote`, and `gh pr view`: OPEN, MERGEABLE, head `5a46a6278`). The memory guard recorded no kill.

**Deleted at the end:** my copies (`<wt>/rv19/`) and targets.
