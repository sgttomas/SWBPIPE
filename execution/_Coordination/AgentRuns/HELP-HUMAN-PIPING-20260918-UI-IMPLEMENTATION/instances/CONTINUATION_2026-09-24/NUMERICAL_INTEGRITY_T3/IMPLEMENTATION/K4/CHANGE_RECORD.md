# K4 change record: the W1a kernel method (retained precision)

This is the PR record for slice K4 of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. I12 (TASK, on the owner's Mac) implemented it; the detail is in `RETURN.md`, and the run evidence is in `_run_records/`.

- **Branch:** `codex/piping-k4-20260928`, from main `e7d930d49` (K2b merged, PR #1040); main `7ac7b1c37` (K6, PR #1053) merged in at `8f8023a20`, with no FK overlap.
- **Checked revisions:** A1 `cef218a10`; A2 `3ed6c0e26`; the Q10 pins `3668ee8a4`; the prescribed-row fix and the V4-S3 probe `5ad1b6174`; the 5a.3 plan (A3-0) `03e7250b2`; A3a `8dfade92e`; A3b `bb7757ac5`; B `8a59458a1`; the main merge `8f8023a20`; C (mutations; no code change); D `7d8fa9c0e` (this record, the run records and the tightened honesty predicate). Addendum 1 (RV19's fixes) is on D's tree; ROOT records its commit, the PR and the merge revisions.
- **Basis:** D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`) as amended by revision 5a.3, whose governing text is R7 §5 (`REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md`, `5502aef9…`; "D1 revision 5a.3 SELECTED"); `TASK_BRIEFS/I12_K4_IMPLEMENTATION.md` with its rulings Q1–Q12; ROOT's K4 rulings in `ROOT_RULINGS_V1.md`, through "K4: rulings on RV19's review" (RETURN §1 and addendum 1), which add D1 revision 5a.3's amendment A1.

## What changes

K4 adds W1a's kernel method under `FK/src/structural/retained/`: a per-case primitive source; the exact load ledger at p; formation and exact assembly at p; the p-factor with its screens; solve and refinement with the hybrid residual gate; recovery before rounding; combinations as their own solve; and the adaptive schedule with revision 5a.3's acceptance rule (V, the verification estimate, the certified per-block bound B = min(Uc, S), θ, the g check, the charge and the ceiling floor Φ), the classification and the evidence. **K4 has no product caller (Q1); F2a wires W1.** Every item is `pub(crate)` inside the private `mod retained`, so nothing K4 adds is reachable from outside FK.

| File | Change | Lines |
|---|---|---|
| `FK/src/structural/retained/{wide_sum,source,ledger,assemble,factor,recover,combine,adaptive}.rs` | new: the method (A1, A2; 5a.3 at A3a and A3b) | 529, 743, 224, 795, 813, 504, 133, 3,118 |
| `FK/src/structural/retained/{directed,bound,verify}.rs` | new (5a.3): directed rounding; the certified bounds and the shift; E, ê, Φ and the verification pass | 201, 851, 1,194 |
| `FK/src/structural/retained/mod.rs` | +20: the module lines and documentation | 44 |
| `FK/src/exact_sum.rs` | +19: the read accessor `net_parts` (Q3); no behaviour change | — |
| `FK/tests/s11_site_table.rs` | +87: K4's eleven files and 36 rows (Q8) | — |
| `FK/tests/retained_k4/` | new: 15 test modules (122 tests), `support.rs`, `models.rs`, the generator `gen_k4_vectors.py` (4,427 lines, `--check`), 22 vector files (about 9.4 MB) and `SHA256SUMS` | — |
| `T3/IMPLEMENTATION/K4/` | `PLAN_A3_5A3.md`, `RETURN.md`, this record, `_run_records/` | — |

- **`factor.rs`'s loop** (`factor()`, `pivot_passes`, `negative_pair`, `solve_scaled`, `solve`) is byte-identical since A3a; Lemmas D and E are read against it.
- **Unchanged:** every manifest and lockfile; K3a's and K3's code, tests and vectors; K-D5's tests; `wide.rs`, `multi.rs`, `structural.rs`, `lib.rs`, `sparse.rs`, `load_ledger.rs`; SA, PP, NI, `sparse_direct`; the fixtures and R1's references.

## Checks

On `aarch64-apple-darwin` with rustc 1.97.1, `CARGO_INCREMENTAL=0`, `--offline --locked`; from A3 on, one cargo job, `-j 4`, `RUST_TEST_THREADS=2`, under the Mac host rules and the memory guard (no kill).

- **K4's tests:** 122 of 122 at D (382.6 s at 2 threads), with the S11 site table's 3. They cover the multi-term sum, the ledger, the source, formation, the factor, the schedule and stop rule, recovery, combinations, the classification with item 6a, the references and RF-LARGE, and 5a.3's E-UNIT, E-UC, E-ESTIMATE, E-CHARGE, E-HEADROOM and SD-G5 (RETURN §11).
- **Controls:** all 131 cases and 4 combinations equal GEN's schedule (the bit oracle) and R7's expectations. **The honesty predicate was tightened at D** to the claim each row publishes, with the binary64 publication rounding stated (RETURN §22.9). At D, every selected control with an expectation satisfied it: 99 controls and 5,490 rows, the worst at 0.28 of its allowance. Thirteen selected controls had no expectation and were skipped, as were unpublishable rows (RV19-2, fixed in addendum 1). Every selected control of the controls test passes a test-only G5a checker. No stop.
- **Moved outcomes against 5a.2,** accepted by ROOT: DIRECTIONAL-SPAN, selected at 256 under 5a.2, is now Unresolved(Ceiling); 5a.2's publication was within its claim. CEIL-A and CEIL-B are now Unresolved(ResolutionScaleUnencodable). RIGID-UNLOADED moves from Unresolved to 512. The new control EHAT-OVERFLOW is Unresolved(ResolutionScaleUnencodable) (RETURN §22.5).
- **References:** R1's 128 K4 cases give 6,475 passes, 0 failures, and 51 not covered (exactly §4.10's list), with 8 refusals. RF-LARGE is selected at 128 at 10 and 100 members. At D its honesty was checked only against R1's 1e-9 predicate; it is checked row by row, with G5a, in addendum 1.
- **Generator:** `gen_k4_vectors.py --check` matches 23 of 23 at B; K3a's and K3's `--check` pass.
- **Suites (B):** CI's 39-manifest numerical profile ran on main `7ac7b1c37`'s piping source with K4's FK, against the Mac baseline of the same tree. Only `frame_kernel` changes, 267 → 389. The three known Mac failures are identical on both sides. FK 389, SD 30 and NI 134 pass. FK's full suite passes again at D (389).
- **Kernel only:** the scan shows no product path to K4's code (RETURN §17). By the brief, T9 and the both-entry gate are not run.
- **Mutations (C):** each mutant ran from a clean `git archive 8f8023a20` with a fresh target; the NONE controls pass. Every killable mutant is killed: R7 §7's list, K4-M34 to M40, K4-M24 (by CEIL5A3), K4's earlier mutants and the D-series. The four derivation guards (R7-M17, M21, M24, and M27 at design precision) move no control, and each is caught at evidence or unit level. At D, under the tightened predicate, the false claims of R7-M1, K4-M24 and K4-M37 are caught as dishonest, with two exceptions: PRESCRIBED-TAIL and PRESCRIBED-TAIL-FREE. Their true rows lie below binary64's range (RETURN §22.9).
- **Hygiene:** K4's files are rustfmt-clean, and the non-test build of FK has no warnings. `cargo fmt --check` reports three files that K4 does not touch, the same as on main.

## Remaining

- **For ROOT:** the independent review, including RETURN §22.1 (Q12's support-group E, which is honesty-relevant), §22.2, §9 steps 10 to 12, and §6 item 4. Then hosted CI, recording the numerical job's time (Q12), and the merge record.
- **Open or argued, from the selection:** listed in RETURN §19. None of them is a step of the honesty guarantee.
- **Routed:** Q9 (a body with no data block has no B_b entry, and θ = 0) goes to F2a and D2. The shift's cost on large models goes to K6b and V-K.
- **Deferred by ruling:** F2a's limits (Q5, as amended); the public export (Q9 of the brief; RETURN §16 lists it); unifying `Binary64Outcome` with K2b's `Representability` (Q11); partial directional grounds (O1, W4/K5); W1b, until F3 meets R7 §6.5.

## Addendum 1: RV19's review

RV19 reviewed head `7d8fa9c0e` (`REVIEW/K4_REVIEW.md`) and returned FAIL, with 1 BLOCKING and 5 SHOULD-FIX. ROOT's binding rulings, "K4: rulings on RV19's review" (numerics `e9f8ebe1d`), include D1 revision 5a.3's amendment A1. The fixes are on `7d8fa9c0e`'s tree; ROOT records the commit. The detail is in RETURN's addendum 1, and the run evidence in `_run_records/rv19/`.

| Finding | Fix |
|---|---|
| RV19-1 (BLOCKING) | `adaptive.rs`: the stop rule's S\* leaves out every row whose candidate value has no binary64 value, as the classification does (O9). This can only lower S\*, so it only withholds. New control OVF-ROT-928 (withheld at every p), with OVF-ROT-900 beside it (128). The reverted fix is killed. |
| RV19-6 (amendment A1) | `adaptive.rs` `row_bound`: where 0 < S\* < 2^-988, b_row = fl↑(fl↑(2^-64·S\*) + fl↑(2^-53·\|q\|) + 2^-1074), decided exactly; GEN likewise. New control TINY-S-995, with TINY-S-900 beside it. Bound bits change only on PT-B (26 rows), PTF-B (45) and TINY-S-995 (24). The reverted amendment is killed by TINY-S-995 (819 times the allowance). |
| RV19-2 | GEN writes each expectation's 128-bit exact value, a range marker for values outside binary64, and, for models with irrational geometry, a decimal solve's error bound. `compare_honest` checks every published row and every unpublishable row, and fails on any missing expectation. The 13 selected controls without expectations now have them. RF-LARGE's 100-member frames run G5a and `compare_honest`. |
| RV19-3 | `combine.rs`: operands whose stations or support groups differ are `OperandsDiffer`, with RV19's t = 0.25 and 0.75 test. |
| RV19-4 | GROUP-DIR and GROUP-DIR-X: support groups holding directional springs, in GEN, E-UNIT, E-UC, E-CHARGE and E-ESTIMATE. RV19-M6 is killed. |
| RV19-5 | A unit test pins ‖SĀS‖ as the larger of the 1-norm and the ∞-norm. RV19-M2 is killed. |
| NOTEs | N1 and N2 are corrected in RETURN. N4: `wide_sum` returns `Span` instead of the release-mode `break`. N6: no `powi` in the tests. N3, N5 and N7 are recorded. |

**Files** (against `7d8fa9c0e`):
- `FK/src/structural/retained/{adaptive,combine,wide_sum}.rs`;
- `K4T/`: the tests, `models.rs`, `support.rs`, GEN, and the regenerated `models.txt`, `models5a3.txt`, `outcomes.txt`, `classification.txt`, `scale.txt`, `bounds.txt`, `charge.txt`, `estimate.txt` and `SHA256SUMS`;
- `T3/IMPLEMENTATION/K4/{RETURN.md,CHANGE_RECORD.md}` and `_run_records/rv19/`.

No other file changes. The vector data is now about 11.2 MB; `models5a3.txt` is 2.0 MB.

**Checks:**
- **K4's suite:** 125 of 125, the N5 streams included. The controls are token-equal to GEN.
- **The honesty check:** every selected control and combination of the controls test (117) has its 9,412 values and unpublishable rows checked against GEN's 128-bit expectations. The worst is 0.949 at B1-C-A u.0.3, RV19's oracle's figure. RF-LARGE's six 100-member frames are honest (worst 0.97) and pass G5a.
- **Evidence:**
  - R7-M1 now also catches PRESCRIBED-TAIL and PRESCRIBED-TAIL-FREE (∞);
  - K4-M24 catches CEIL5A3 (670);
  - K4-M37 catches D's eight, plus both tails and R115-SEED3.
- **FK's full suite:** 392 passed (D's 389 plus the three new K4 tests).
- **The S11 site table:** 3 of 3.
- **rustfmt:** clean on K4's files.
- **`gen_k4_vectors.py --check`:** 23 of 23 OK (16 min 56 s).
- **Mutants** (each from a clean copy of the candidate): the reverted RV19-1 fix, the reverted A1, RV19-M2 and RV19-M6 are all killed; NONE passes.

**Remaining:**
- RV19's delta check, including amendment A1's derivation and code (RETURN A1.2) and the high-precision expectations (A1.3);
- D2's G5 takes b_row;
- ROOT adopts A1 into the design text;
- then hosted CI and the merge record.

## Addendum 2: RV19's delta check at `a5fa0eaf7`

RV19's delta check PASSES with 0 BLOCKING and 1 SHOULD-FIX. ROOT's rulings (numerics `83909efd4`) ask for RV19-D4 and DN2 before merge. Both are done on `a5fa0eaf7`'s tree; ROOT records the commit. The detail is in RETURN's addendum 2, and the evidence in `_run_records/rv19d/`.

| Item | Change |
|---|---|
| RV19-D4 (SHOULD-FIX) | `method_tests.rs`: `sd_g5_a_row_the_candidate_cannot_publish_sets_no_s_star` gives `decide` four vectors, the overflow and underflow pairs each way round. A row whose candidate cannot be published sets no S\*; one whose verification only cannot be published does. RV19's mutant RV19-D4 (the skip keyed on the verification value) is killed by that vector alone, from a clean copy of `a5fa0eaf7` with this change. NONE passes 116 of 116. |
| DN2 | GEN: `models.txt`'s PRECISION-RULE gains expectations (the combination as its own case). `combine_tests.rs`: a new PRECISION-RULE test (selected at 256; G5a; `compare_honest`), and `check_combo` (B1-C, B1-E) runs `compare_honest` and G5a too. RETURN's unscoped sentence is scoped. |

**Files** (against `a5fa0eaf7`):
- `K4T/{method_tests,combine_tests}.rs`;
- `K4T/gen_k4_vectors.py`;
- `K4T/models.txt` and `K4T/SHA256SUMS`;
- `T3/IMPLEMENTATION/K4/{RETURN.md,CHANGE_RECORD.md}`, `_run_records/SHA256SUMS` and `_run_records/rv19d/`.

No source file changes.

**Checks:**
- K4's suite: 127 tests;
- FK's full suite: 394 passed (addendum 1's 392 plus the two new tests);
- `gen_k4_vectors.py --check`: 23 of 23 OK (14 min 43 s);
- rustfmt clean;
- the memory guard logged no kill.
