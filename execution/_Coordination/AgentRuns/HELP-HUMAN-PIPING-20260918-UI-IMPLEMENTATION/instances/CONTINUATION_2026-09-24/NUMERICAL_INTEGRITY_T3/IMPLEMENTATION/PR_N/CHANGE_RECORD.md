# PR-N: a correctly rounded Euclidean norm replaces libm `hypot` on published paths: change record

- **Code commit:** `8dd64c1835` on `codex/piping-t3-correct-norm-20261008`, one commit on main `7eae707bb7`. Its 21 maintained files equal NUM `ef8ab78473` (the norm branch `7bd84e0526` merged into NUM) outside the execution records.
- **Repair commit:** `8ca80508b6` (repair round 01, for RV126's S-1 and notes). Its production edits are comment-only, and each hunk is line-neutral, so the release builds are unchanged; it also changes two tests. It changes 4 of the 21 maintained files and adds a fifth, the test file `product_final_case_tests.rs`, so the PR now carries 22. Its message states S-1's correction; the code commit's message is not recut (RR "PR-N (#1163), RV126's B-1: the platform-independent rank screen is accepted, option (i)").
- **Implemented by** I109 (T3): `R/I109/platform_norm_01/` (round 1), `R/I109/pr_n_01/` (round 2), `R/I109/pr_n_package_01/` (round 3, this package) and `R/I109/pr_n_repair_01/` (repair round 01).
- **Ruled in** RR "I109: a correctly rounded norm replaces libm `hypot` on published paths; glibc was the correctly rounded side; it goes as its own PR after B1".
- **Path convention:** `P` is `projects/chirality-piping`; `PP` is `P/core/product_physics`.

## 1. What changes and why

**Why.** `f64::hypot` calls the platform's libm, which is not correctly rounded.
- macOS arm64's `hypot` misrounds 16.7% of random comparable-magnitude pairs (`R/I109/platform_norm_01/_run_records/oracle/platform_hypot_rate_mac.json`).
- glibc and macOS differ in the last bit, so the published support magnitudes depended on the machine. B1 worked around this with exact per-platform pins, and three tests (`t13` and the runner's two `load_reference` both-mode tests) failed on the Mac against Linux-taken pins.

**The norm.** `P/core/solver/frame_kernel/src/correct_norm.rs` defines `norm2(a, b)` = RN(√(a² + b²)) and `norm3(a, b, c)` = RN(√(a² + b² + c²)), rounding to nearest with ties to even.
- **Operations:** IEEE `+ − × ÷`, `sqrt` and `mul_add` only. `mul_add` is a correctly rounded fma: the hardware's, or the platform's `fma` where there is none (libm's on baseline x86_64, compiler-builtins' on wasm32). Given a correctly rounded `fma`, as Rust's `mul_add` and IEEE 754 require, every platform gives the same bits. RV126 measured a wasm32 build (software `fma`) bit-identical to aarch64 (hardware FMA) on 2,721,296 results (`R/REVIEW_RV126/pr_n_01/REVIEW.md` §1, N-4). x86_64 is evidenced by the glibc dispatches' pins (§3).
- **Method:**
  1. Sort the absolute values and scale exactly so the largest lies in [1, 2).
  2. Split the squares exactly with `mul_add`.
  3. Take a double-double square root as the candidate.
  4. Correct the candidate one ulp at a time on the destination's grid (subnormal results and the overflow threshold included), using the exact sign of S − m² at the neighbouring midpoints, computed by an error-free expansion sum.
- **Special values:** C Annex F's `hypot` semantics.
- **Every magnitude formerly formed with libm `hypot` is now a correctly rounded norm.** A 3-component one is the correctly rounded 3-norm, not a hypot chain. It has one rounding and does not depend on the order of the components. On random triples, the correctly rounded chain differs from it 16.6% of the time and changes with the component order 26.2% of the time.
- **Not every published magnitude is the norm** (RV126 S-1). Two published magnitudes never used `hypot` and are unchanged: the selected-source support magnitude `source_receipt::scaled_norm` (m·√((a² + b²) + c²), with m the largest absolute component and a, b, c the components divided by m) and the nodal `displacement_magnitude` (√ of a sum of `powi(2)` squares). Both are deterministic IEEE, so they do not depend on the platform, but they are not correctly rounded. The code commit's message ("A published 3-component magnitude is the correctly rounded 3-norm") and its comment at the top of `PP/src/lib.rs` ("published norms are correctly rounded") said more than this. The repair commit rewords the comment, and its message and this record carry the correction.

**The call sites.** 30 of the 32 product `hypot` calls now call the norm: every one that reaches published bytes, a receipt or diagnostic text, plus the rank screen (an admission decision) and `elastic_section` (no caller). The other 2 are in `performance_harness`, which is not a product dependency.

| Where | Symbol | What |
|---|---|---|
| `PP/src/lib.rs` | `solve_load_case_observed` | the ordinary route's support force magnitude; the formation guard's end q = \|(My, Mz)\| |
| `PP/src/lib.rs` | `append_signed_support_results` | support-action force and moment magnitude rows |
| `PP/src/lib.rs` | `append_combined_vector_magnitude` | combined displacement and support magnitudes |
| `PP/src/preview_physics.rs` | `render`, `append_combination_results`, `tangent_diagnostics` | i·\|(My, Mz)\|/Z; combination magnitudes; the tangent-discontinuity \|cross\| (its `atan2` remains libm) |
| `PP/src/pressure_runtime.rs` | `traverse_region` | the pressure region's chord length and collinearity residual (a guard and its diagnostic text) |
| `PP/src/case_state/resolve.rs` | `resolve_case` | the published `reference_length_m` |
| `PP/src/retained_product.rs` | `observables_view` | the retained support guard |
| `frame_kernel/.../product_certificate/final_case.rs` | `support_hypot` | the retained certificate's support projection, still charged as two scalar operations |
| `frame_kernel/src/rigid_body.rs` | `assess_rigid_body` | **the rank screen** (characteristic length L; Jacobi rotation): one admission decision on every platform, which closes its T3-close item. Near its threshold the Restrained decision changes in both directions, and at extreme scale one witness class changes; L also scales a witnessed mechanism's published node motions, which move by an ulp wherever L did (below). K5's B10 source scan now covers all of `rigid_body.rs`. |
| `P/core/loads/stress_recovery/src/elastic_section.rs` | `evaluate_elastic_section` | the bending amplitude. The function has no caller outside its module (I109 round 2's correction). It is included by `#[path]`, so there is no new dependency. |

**The rank screen's changed decisions.** RV126 compared the norm's screen with the libm screen on this Mac, on synthetic bodies (`R/REVIEW_RV126/pr_n_01/REVIEW.md` §2a, B-1 and N-5). ROOT accepted the changes as the cost of one decision on every platform (RR "PR-N (#1163), RV126's B-1: the platform-independent rank screen is accepted, option (i)").
- **The Restrained decision changes in both directions near the threshold.** Of 1,045,305 near-threshold probes, 59,600 are Restrained only with the norm and 66,967 only with libm. Every change lies within 1.5·10⁻³ relative of the threshold, where the libm decision was already non-monotone and platform-dependent (macOS and glibc `hypot` differ there). No true mechanism is admitted in either version.
- **One witness class changes at 10²⁰⁰ scale.** In 1 of 300,000 random bodies, MechanismWitnessed becomes NumericallyUnresolved. Both refuse, but the refusal's integrity code (`integrity_failure_code`) and its published content change.
- **A witnessed mechanism's published node motions move by an ulp wherever L did** (N-5). L scales the mechanism's translations, which are published in the blocking diagnostic text and bit-encoded in the retained wire. 22,966 of 246,587 synthetic witnessed mechanisms change node-motion bits.
- No committed input lies in the band, and no committed pin moves.

There are no manifest, `Cargo.lock` or reviewed-input changes.

**The moved values.** Every one is one ulp, and each new value is the exact correctly rounded norm of its own components. Before is macOS libm; after is the norm, which equals glibc.

| Value | Components (exact binary64) | Before (macOS) | After | Committed bytes |
|---|---|---|---|---|
| case C `rigid:N0` support force magnitude (W-C2 dense; SF-2 (C, B, A) dense; the rf_skew milestone's dense ordinary run) | Fx `-0x1.d5ap-46`, Fy `-0x1.41b68p-40`, Fz `0x1.4561a2ed5be5bp-40` | `1.6258317075882521e-12` | `1.6258317075882523e-12` | **re-pinned** (below) |
| `load_reference/connected` root force magnitude: sparse `cold` | `-0x1.2374d355759ep+18`, `0x1.0cd79f70bfe37p+13`, 0 | `298575.2677594065` | `298575.26775940653` | unchanged: the committed raw (a Linux pin) is now reproduced on the Mac |
| the same root force magnitude: sparse and dense `hot` | `0x1.1e2f73dbd5f8dp+20`, `0x1.aab36f291fd53p+12`, 0 | `1172235.122530021` | `1172235.1225300212` | unchanged; as above |
| `load_reference_fallback_uz` sparse anchor moment magnitude (`t13`) | 0, `0x1.0c6f7a0b5ed8bp-19`, `0x1p-43` | `2.000000000000002e-06` | `2.0000000000000025e-06` | unchanged; as above |
| m08 intensified row, case L2, pipe b–c end i | the test's own model | `18516192.493245974` | `18516192.493245978` | not committed: the test's expectation now uses `norm2` |

The components and checks are in `R/I109/pr_n_package_01/_run_records/moved_values.json`, from I109's base-against-candidate regeneration. Of 66 envelopes regenerated from every committed request and model fixture, 62 are byte-identical; the other 4 hold the moves above.

**The re-pins.** Each is the `rigid:N0` value above plus the hashes over it, with one pin for every platform:
- the W-C2 dense fixture: document `f2800bd4…` → `c11f7566…`, receipt `612e23ca…` → `ca6a62a6…`, published bytes `a77c010b…` → `604e4a33…` (`W_C2_PINNED`);
- `CBA_PINNED` dense: `7aeecbac…`/`c719bd8d…` → `255785d2…`/`a320a5d3…`;
- u1's `ORDINARY_SHA256` dense: (69366, `aa8b93ba…`), now asserted on every target;
- the reader corpus (`retained_precision_cases.json`): cases `w_c2_dense_scrutiny` and `sf2_c_b_a_dense_scrutiny`, and the value inside mutation `f1_p4_parity_on_w2_published_case_a_dense`;
- the PY reader's 07n snapshot pins (`N07_W_C2`, `N07_PP_PINNED`);
- B1's glibc-only variants (`W_C2_DENSE_GLIBC`, `CBA_DENSE_GLIBC` and their `cfg`) are retired;
- the bit-spelled `CAP_MAXIMAL_RING` stays. Its check now applies the absolute escape only to the three nonzero near-zero residues (RV125 A1-N1).

## 2. Scope: what is and is not platform-independent after this PR

**Platform-independent:**
- every magnitude formed by the norm (§1's call sites), given a correctly rounded `fma` (§1);
- the rank screen's admission decision and a witnessed mechanism's published node motions. Compared with the libm screen, decisions within the screen's rounding-noise band changed, one witness class changed at 10²⁰⁰ scale, and node motions moved by an ulp wherever L did (§1, "The rank screen's changed decisions");
- `source_receipt::scaled_norm` and `displacement_magnitude`, which are deterministic IEEE but not correctly rounded (§1, S-1);
- the readers' agreement, because they recompute norms only inside a 64ε guard, far above one ulp;
- `powi` on variables: one fixed multiplication sequence;
- `to_degrees`: one multiplication.

**Still platform-dependent:** the remaining libm calls on product paths (I109 round 1, `R/I109/platform_norm_01/RETURN.md` §7). This PR does not change them:

| Function | Calls | Where | What depends on them |
|---|---:|---|---|
| `sin`, `cos` | 48 | `curved_bend` (46) and `nonlinear_integration`'s curved symmetry chord (2) | every model with a curved bend, through the solve |
| `atan2` | 2 | the bend's included angle; the tangent-discontinuity warning angle | as above, plus the warning text |
| `asin` | 1 | the implied bend angle check | an admission decision and its text |
| `exp`, `exp_m1` | 4 | the logarithmic thermal stretches and strain | logarithmic-law thermal cases |

No committed pin exposes them today: the 40 manifests pass on both platforms. Correctly rounded implementations are a later round.

## 3. The evidence

- **The oracle** (`R/I109/platform_norm_01/_run_records/oracle/`): 0 misrounded in 22,000,000 results.
  - That is norm3 and norm2 on 11,000,000 triples, checked against exact integer square roots.
  - The triples cover the whole range including subnormal and overflow, plus 1,000,000 adversarial ones (exact and near midpoints, sticky third components, binade and range boundaries).
  - Debug and release builds give identical bits.
  - 1,200 of the cases are committed as `correct_norm_vectors`.
- **The 40 manifests on the Mac** (CI's numerical profile, fresh targets, `R/I109/pr_n_01/_run_records/checks/`): **2,786 ok, 81 ignored, 0 FAILED**.
  - Against I109's round-1 base (NUM `af53e1447c`), 3 outcomes change, each from FAILED to ok: `t13` and the two runner `load_reference` tests.
  - The added tests are the norm's tests, plus B1's ring check, which merged after that base and is already on main.
- **glibc:**
  - Diagnostic dispatch **37808190331** on `b9dea77a85` (the norm plus B1's platform fix, before the re-pins) succeeded in every job. On glibc the norm reproduces B1's glibc variant documents, which are byte-identical to the Mac's correctly rounded bytes. `t13`, the runner tests, m08 and the ring pins pass.
  - Dispatch **37820998162** on `7bd84e0526` (the re-pinned head, whose 21 files equal `8dd64c1835`'s; RV126 diffed each) **succeeded**: every job except the accessibility barrier, which was skipped (as in 37824479785), numerical cargo suite included. It is the first glibc run of the single pins: u1's unconditional ordinary pin, the single W-C2 fixture, the single SF-2 pin, m08 with `norm2` and the tightened ring check.
  - The full-SHA dispatch **37824479785** on `dab19291a8` (the code commit plus this package) **succeeded**: the numerical cargo suite, the source coverage and remainder jobs and the source-mode desktop E2E.
- **The readers:**
  - The PY files that read the corpus (`test_retained_precision_contract.py`, `test_retained_precision_schema.py`): 1,079 passed.
  - The TS files that read the corpus are four: `retainedPrecision.test.ts`, `retainedPrecisionIntegration.test.tsx`, `retainedPrecisionResultExport.test.tsx` and `retainedPrecisionStressNeutral.test.tsx`. In round 2, I109 ran `retainedPrecision.test.ts` alone: 1,102 passed. RV126 ran all four at `dab19291a8`: 1,314 passed, 0 failed (`R/REVIEW_RV126/pr_n_01/REVIEW.md` §5).
  - The RS `result_export` suite: 199 passed. It is also part of the 40 manifests.
- **Repair round 01 at `8ca80508b6`** (the Mac, fresh targets, `R/I109/pr_n_repair_01/`): PP 743 passed, FK 550 passed, `result_export` 199 passed; the two PY files 1,079 passed; the four TS files 1,314 passed. Nothing failed. The production hunks are comment-only and line-neutral (`R/I109/pr_n_repair_01/_run_records/line_neutrality.txt`).
- **What is not shown:** bitwise equality of every published byte across platforms. Equality is shown wherever a committed pin compares bytes, and §2's libm paths are not covered.

## 4. The reviews and gates

WORKING_ITEMS gives each verdict.

| Gate | Revision | Verdict |
|---|---|---|
| RV126 (RV-N): fresh review of the norm module and every call site | `dab19291a8` | **FAIL on B-1 only** (1 BLOCKING, 1 SHOULD-FIX, 8 NOTE; `R/REVIEW_RV126/pr_n_01/REVIEW.md`). B-1 ruled accept, option (i) (§1). S-1 and the notes are answered by repair round 01 (`8ca80508b6`) and this package |
| RV126's confirmation of repair round 01 | `0c7490e1be` | **PASS**, 0 BLOCKING, 0 SHOULD-FIX, 8 NOTE, all answered (`R/REVIEW_RV126/pr_n_01/ADDENDUM_01.md`) |
| Pass B (I107), confirmed by RV124 (D1 call sites, §5) | `8dd64c1835`, carried over to `0c7490e1be` (token-identical code) | **DELTAS TO READ, no stop**; TEXT, forms and M unchanged; outcomes change only by `t13` and the two `load_reference` tests (FAILED → ok). **RV124 CONFIRMED**, with the carry-over (`R/I107/pr_n_passb_01/`, `R/REVIEW_RV124/pr_n_passb_01/CONFIRM.md`) |
| I112: T9 and the both-entry gate (published bytes on D1) | B `7eae707bb7` against C `8dd64c1835`, carried over to `0c7490e1be` | **T9 PASS; part 1 PASS** (884 of 884 under ROOT's rule: a difference passes only where the candidate is the exact correctly rounded norm of its own components and the base is the libm `hypot` (chain) value; 5,254 moved magnitudes, 10 of them by two ulps, and norm values in two diagnostic texts); gate_check 0 trusted breach triples on both sides; **part 2 PASS** (`R/I112/pr_n_gates_01/`) |
| Linux diagnostic dispatch 37820998162 | `7bd84e0526` | **success** (§3) |
| Full-SHA dispatch 37824479785 | `dab19291a8` | **success** (§3) |
| Hosted CI on the PR | `0c7490e1be` | **success** |
| Full-SHA dispatch 37833181297 (`target_base` `7eae707bb7`) | `0c7490e1be` | **success** |
| GEN-8 | `0c7490e1be` | **1 passed** |
| Exact-head DEC-025 with src-tauri | `0c7490e1be` against a fresh main baseline `7eae707bb7` | **ALL-DONE**. The sweep passes every surface on the Mac for the first time (no platform failure remains). 40 manifests: 0 FAILED; the only changed outcomes are `t13` and the two `load_reference` tests (FAILED → ok); +11 added and 3 removed (the norm's tests; three frame_kernel doc tests renamed by a line shift). pytest 4,426 passed, 32 skipped; vitest 141 files, 4,245 tests; both builds exit 0. src-tauri 116 = 116, identical. The record is the merge record |
| `source_equality.py` (checks 1–5) | the code commit plus this package | PASS, recorded in `R/I109/pr_n_package_01/` |
| `check_citations.py` | the code commit plus this package | PASS, recorded in `R/I109/pr_n_package_01/` |
| `source_equality.py` and `check_citations.py` | `0c7490e1be` | **PASS** against NUM `09a8af1d19` (22 maintained files), run by WORKING_ITEMS; and in `R/I109/pr_n_repair_01/` |

## 5. The D1 call sites

These come from SQ's call graph (`R/I104/b1_sq_01/_run_records/chain/callgraph_g5.py`), run with SQ's `run_point.sh` settings on `8dd64c1835`'s `P/core` from the root `run_linear_static_preview_value_with_retained_direct`. 2,813 of 3,497 nodes are reachable, among them `norm2`, `norm3` and all their helpers. **16 of the 17 production call sites are in D1:**
- every PP site in §1;
- `support_hypot`;
- `assess_rigid_body` (2 sites).

The ordinary-route sites count because the Direct entry runs the ordinary route as its plain run. **The one site outside D1** is `evaluate_elastic_section`, which has no caller.

For Pass B:
- `correct_norm` allocates nothing (fixed `[f64; 10]` stack arrays) and does not recurse.
- Its correction `loop` has no static bound in its form. By construction it runs at most two iterations, because the candidate is RN or a neighbour of RN.

The site list is in `R/I109/pr_n_package_01/_run_records/d1_norm_sites.json`.
