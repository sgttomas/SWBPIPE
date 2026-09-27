# T3 V1 — independent review of D1 (numerics) and D2 (standing, envelopes, transport)

Type 2 TASK V1 for the T3 WORKING_ITEMS manager, 2026-09-26. Fresh-context reviewer: I wrote neither design and advised neither designer.

- **Inputs reviewed.** `T3/DESIGN_NUMERICS/DESIGN.md` revision 1 (sha256 `7199390f…ebf726b`, verified) with `_run_records/` (SHA256SUMS verified). `T3/DESIGN_STANDING/DESIGN.md` (sha256 `185e178e…ec2bca2`, verified) with `_run_records/` (SHA256SUMS verified). D2's advance note `T3/MANAGER_NOTES/D2_ON_D1_RETIREMENT.md` read at `a68218c81` (sha256 prefix `100b9571c316c98e`), as ROOT asked.
- **Bases.** T3 branch at `e8ab6c87d`. Product source at `c61a540ea`. T1 at `f3270ea79`, read only with `git show` and `git grep`. Line numbers below are at those commits. `P/`, `T3/`, `PP` (`P/core/product_physics/src/lib.rs`), `FK` and `SA` are as in the designs.
- **Paths in this record** use `T3/` and `<worktree>` placeholders.

## 1. Verdict: **BLOCKING**

There is one BLOCKING finding, V1-B1, in D1. D1's stop rule, together with its residual check at p + 64, accepts a wrong answer whenever a source contribution is lost in the same way at p and at 2p. I built a concrete example with admitted inputs, using D1's own emulation. Under D1's trigger and standing, that example is a path to publishing a wrong value as a selected, Current result. The required change is small and local.

D2 has no BLOCKING finding. Every D2 change either fails closed or removes standing. D2 does have SHOULD-FIX findings: its transport finding reaches further than it states (V1-S5), and its host-`exp` acceptance rule breaks parity between languages (V1-S6). The interface between the two designs has three unresolved seams (V1-S1 to V1-S3).

## 2. ROOT's four questions

### Q1. The interface, if D1's D-4 option A is selected

- **physics-source-1.** Yes: under D-4 A, fresh solves stop emitting it at D1's F2 (D1 §4.4, l.312).
- **load-reference-source-1.** No, not as D1 is written. D1 says the joined identity is "Unchanged by D1. Switching it waits for T1's merge, W1b, and D2's joined-eligibility design" (D1 l.313). So under D1 it stays fresh at least until F3, the W1b facade slice. D2 makes S-E conditional on D1's answer (D2 I-4, l.407-409; §4.2.3, l.263), and D1 makes the switch conditional on D2's design. Each waits for the other.
  - D2's advance note (`a68218c81`) says D-4 A "answers I-4 no" and so makes S-E optional. That reads more into D1 than D1 says. See V1-S1.
- **Is S-E still worth building?** In part.
  - Its joined-standing wiring (S-1), route test and route carriers are worth building only if the joined identity stays fresh past F3. Otherwise they serve two things: the window from T1's merge to F3, and historical joined results that carry an invocation.
  - Its re-derivation core, `RESOLVED-CASE-REDERIVATION-v1`, re-derives the 0.4.0 resolved operands (E, ν and α per selected member, eigenstrain, support motions). Those are exactly the inputs to D1's W1b `PrimitiveSource`. It is also what D1's 0.4.0 successor would need for invocation-bound standing, if I-2 takes the receipt route.
  - **Recommendation.** ROOT first rules the joined identity's fate. Then split S-E into S-E1, the re-derivation core (kept, and scheduled with F3), and S-E2, the joined standing wiring and carriers (built only if the joined identity stays fresh past F3). Until then the joined identity stays `needs_recompute`. That is safe.
- **S-D.** Its reach shrinks to the window before F2. After F2 no pre-0.4 fresh solve selects exact-block, so the `Err` guard becomes unreachable, as D2's note says. S-D is small and closes a live finding in that window, so keep it unless F2 lands first.
- **S-F.** Its producer gate (`is_exact` at `PP:1885`) is superseded by F2. Its standing, fresh-set and text parts still apply.
- **I-3 against the R-B retirement condition.** They do not line up yet.
  - D2's R-B gate (I-3, D2 l.406) covers the preview route only, with N05 and N06 as positive controls. It names no budget or standing condition.
  - D1 states no retirement gate. F2 retires exact-block selection on both routes, and its only in-scope check is value agreement with the exact-block oracle (D1 l.312, l.524).
  - D2's note already widens the gate to "every route". The gate also needs D1's budgets (D-8, still undecided) to admit every case that is selected today. It needs the successor identities to earn standing in all three languages, and their readers have no owner yet (V1-S3). See V1-S2.

### Q2. D1's central claim, the order-≤2 argument and the stop rule

- **The claim holds.** Solving the rounded binary64 element matrices exactly misses 1e-9. Rebuilding each element from its binary64 primitives at precision p meets 1e-9. The evidence:
  - I reran `probe_skew_precision.py` and got byte-identical output.
  - The D1 reference (basic-deformation formation run exactly) is not circular. It equals, entry for entry, an exact textbook 12×12 Euler–Bernoulli element under the exact transform, for the axial, skew and oblique geometries (my probe, check T: 0 differing entries).
  - One caveat. D1's column (C) actually solves the *assembled* binary64 matrix, where a + k has already been rounded, not the exactly summed contributions that the exact-block method uses. I added the missing contribution-exact method. D1's conclusions are unchanged: 1.05e-5 (axis bending), 4.7e-6 (skew N05), 1.07e-3 (oblique), and nonpositive pivots for N06-class skew, k = 1e-28 and the six-member run (V1-N1).
- **The order-≤2 argument (D1 §2.3) is correct.** Solved exactly from the represented contributions, the 2-DOF axial, transverse-translation and torsion soft modes have zero error. The single rounded coefficient ±a keeps the null vector exact. A bending soft mode needs a root rotation, tip translation and tip rotation (three DOFs), and it misses by 1.05e-5 (check O). Note that the assembled-matrix form fails even the 2-DOF blocks, at 1.3e-4, 4.7e-8 and 5.3e-7. The distinction in V1-N1 therefore matters.
- **The stop rule behaves as D1 claims on D1's own counterexamples.**
  - k = 1e-28: 128 bits rejected at 1.8e-4; 256 bits accepted against 512 at 5.3e-43, with an error of 5.0e-43.
  - Six members: 128 bits rejected at 2.05e-19.
  - k = 0 mechanism: nonpositive pivot at 128, 256 and 512 bits.
  - Mechanism handling is doubly guarded at every precision: a noise pivot at p differs from the noise pivot at 2p, so agreement fails even if the screen passes.
- **The stop rule is not sound as an acceptance rule, as written.**
  - Two tip loads of 1e80 and −1e80 on one DOF absorb a third load of 1e-8. They absorb it identically at 128 and 256 bits, and in the residual system re-formed at p + 64 = 192.
  - The candidates agree to 1.8e-28, or exactly, so 128 bits is accepted with a 50 % to 100 % error.
  - Combinations formed at p have the same weakness and are not covered by the stop rule at all. See V1-B1.
  - Separately, the body-coupled scale S\* guarantees 1e-9 only for quantities at or above about 5.4e-11·S\*. That is a gap in what the rule guarantees (V1-S8). My attempt to realize a miss from it did not produce one.

### Q3. D2's transport finding

- **Verified at `c61a540ea`.** Every checked canonical carrier refuses every finite |x| > 2^53 − 1, and every such binary64 value is integral:
  - Rust `derivative::guard_json` (`P/core/reporting/result_export/src/derivative.rs:13-19`);
  - the checked profile (`P/core/serialization/canonical_json/src/lib.rs:94-108`);
  - Python (`canonical_json/adapter.py:40-46`);
  - TS (`apps/desktop/src/features/result-export/resultExportAdapter.ts:25`);
  - the source receipt's `hash`/`checked` (`source_receipt.rs:28-35`).
- **D2 understates the reach.**
  - The same checked hash runs over the **raw request** at invocation capture: `run_linear_static_preview_value_with_mode` → `CapturedInvocation::parse` → `checked(&raw)` (`PP:1239-1240`; `source_receipt.rs:64-73`).
  - Both production callers propagate that `Err`: desktop `solve_preview_mechanics_with_mode` (`apps/desktop/src-tauri/src/lib.rs:1562`) and headless `run_preview_model_value_mode` (`P/core/runner/headless/src/lib.rs:737` via `produce()?` at `:865`).
  - T1 is the same (`lib.rs:1414-1415` at `f3270ea79`).
- **Consequences for today's published results:**
  1. A production solve whose request contains any finite number with |x| ≥ 2^53, such as a 1e16 N/m "rigid" spring, returns `Err("CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT: …")` before it solves anything. This holds on every route, ordinary included, on main and on T1.
  2. A source-identity invocation whose own envelope contains such a published value fails receipt finalization (`source_receipt.rs:751`, `:904-905`). The whole invocation is lost to the `PP:1249-1253` `Err`, including any correctly recovered case.
  3. An ordinary result carrying such a value still publishes its raw envelope. The headless digest then fails (`headless/src/lib.rs:741-745`), so there is no canonical document, `QualifiedPreviewEvidence` or AnalysisRun (`canonical_export_unavailability`).
  4. No value is silently altered. Every path is a refusal.
- **D2's PR-5a and PR-5b predictions are wrong.** They put 1e16 in the request (`d2_probe/src/main.rs.txt:104-106, 194-195`), so they fail at capture with a different code, and S-D can never reach them. D2 §6.1's post-implementation expectation, "PR-5 … never `Err`" (l.431), would fail.
- **DD-8's split to T6 is sound for export carriers and persistence.** The capture boundary is not a carrier, though. It sits in `product_physics` (T3/T1 files) and needs a named owner. See V1-S5.

### Q4. Silent-wrong exposure

- **D1: yes, V1-B1 (BLOCKING).**
  - A Sensitive N05-class case triggers W1.
  - A load contribution absorbed identically at p, 2p and p + 64 leaves every screen passing.
  - The case is published as selected with "checks passed at precision p; 2p agreement verified", which is D1's route to Current (D1 l.325).
  - Main withholds the same Sensitive case from Current today. D1 would therefore create a new route to Current for a wrong value.
- **D2: none found.**
  - S-A tightens a reader.
  - S-B refuses display.
  - S-D publishes ordinarily with ordinary standing. No failed or Sensitive case gains standing; I checked this against T1's SF-1 code.
  - S-E grants eligibility only after J4 re-derivation. The DD-4 bound admits at most a one-ulp difference in a host-`exp` intermediate, and everything downstream is re-derived from the recorded value. That changes values by about 1e-16 relative, so it is not a wrong value, but it is a parity defect (V1-S6).
  - S-F only removes standing.
- **Main, independent of both designs.**
  - Main already has the binary64 form of V1-B1 for **Passed** cases. Nodal loads are summed in binary64 (`P/core/loads/primitive_loads/src/lib.rs:1405`, called at `PP:1810`). M03's intended-action audit starts from the already summed `system.force` (`FK/structural.rs:513-532`), so load contributions are never audited.
  - Loads {+F, +f, −F} on one DOF with F/f > 2^53 therefore publish the response to f as zero. When the stiffness is ordinary, the case is Passed and Current.
  - This is a live silent-wrong path on main today. Neither design introduces it or closes it, and it is not in `STAGE0_MAP.md` (V1-S11).

## 3. Findings

| ID | Severity | Design, section | Evidence | Consequence | Required change |
|---|---|---|---|---|---|
| **V1-B1** | **BLOCKING** | D1 §4.1.2 (l.199), §4.1.4 (l.214), §4.1.6 (l.236-246), §4.1.1 `combine` (l.187), §3.1 claim "never accepted a failing candidate" (l.152) | `REVIEW/_run_records/v1_stop_rule_probe.*`, check L, using D1's emulation unchanged. N05-class skew (k = 1e-4) with tip loads {1e80, 1e-8, −1e80} on one DOF (exact sum 1e-8), plus a 2e-8 load on another DOF. 128 vs 256 bits disagree by 1.8e-28, so 128 is **accepted**. Its error is 0.5 of the body scale, and strict relative error on nonzero quantities is 2.0 (7 quantities fail). Loads only: agreement is exactly 0, accepted, error 1.0. The load formed at p + 64 = 192 bits is 0.0, so the residual gate cannot see it. At 512 bits the answer is exact, but it is never reached. Separately, `combine` forms Σ cᵢ·uᵢ at p on retained states only, outside the stop rule, so a term smaller than 2^-p of the others is lost unverified. | A wrong value is published as a selected, "checks passed" case, which is D1's route to Current. Main withholds it (Sensitive). The stop rule's soundness claim is false for common-mode loss: a contribution lost identically at p and 2p. | (1) Form every source-level linear sum from binary64 inputs **exactly**, with the existing `Expansion` machinery, and round once to p: nodal and constant-effort loads per DOF, and later W1b load contributions. Or detect any absorbed contribution and escalate, as `contribution_sums` does for stiffness (`FK/structural.rs:395-414`). (2) Compute combinations exactly as expansions over the p-bit retained states, rounded once, and put combination outputs under the stop rule. (3) Add this case, and an absorbed-term combination, to §7.3's mutation and negative controls. (4) Restate §3.1's claim as limited to precision-dependent error. |
| V1-S1 | SHOULD-FIX | Interface: D1 §4.4 (l.313) and §5 item 4 (l.502) against D2 §5 I-4 (l.407-409) and §4.2.3 (l.263); D2 note `a68218c81` | D1 defers the joined identity's switch to "T1's merge, W1b, and D2's joined-eligibility design". D2 defers S-E to D1's answer. D2's note assumes D-4 A ends joined freshness, which D1 does not say. Neither design states whether physics-source-1 stays in the fresh sets once it is no longer emitted fresh (that affects historical physics-source-1 standing). | S-E could be built for an identity that stops being fresh at F3, or skipped while the identity is still fresh. | ROOT rules explicitly (a) when `load-reference-source-1` stops being fresh (at F3 or never within T3), and (b) whether historical physics-source-1 keeps eligibility. Then split S-E into S-E1 (re-derivation core, reusable for D1's 0.4.0 successor) and S-E2 (joined standing, route test and carriers, only if the identity stays fresh past F3). Both designs cite the ruling. |
| V1-S2 | SHOULD-FIX | Interface: D1 §4.4 (l.312), §6 F2 (l.524), §7.1 (l.538-549); D2 §5 I-3 (l.406), §4.5.2-4.5.3 | D2's R-B gate covers the preview route only and has no budget or standing condition. D1 retires exact-block on both routes, checked only by value agreement with the oracle. W1 budgets (D-8) are undecided. The successor identities' standing has no owner (V1-S3). | Retirement could demote results that are Current today (all-selected source-blocks-1, eligible physics-source-1) to not-Current for fresh solves. | One retirement gate for F2 and S-F, with four conditions. Coverage: every committed `fixtures/product_preview/{source_blocks,physics_source}` request, solved fresh in both modes, is W1-selected. Budgets: D-8 limits admit them. Standing: they are Current or eligible in Rust, Python and TS through the successor readers. Values: the projections match exact-block within the unchanged 1e-9. |
| V1-S3 | SHOULD-FIX | D1 §4.4 (l.310-311), §6 row "R: readers \| per D2" (l.526), atomic F2 PR (l.532); D2 §5 I-2 (l.405), §4.7 (no D1 row), §7 (no slice) | D2 designs no reader, standing, fresh-set or schema slice for `<preview-retained>` or `<physics-retained>`. D2's I-2 leaves open whether standing comes from `checks_passed` or from a verified receipt. D1 §5.5 says Python and TS cannot replay the kernel. | F2's atomic PR has no designed reader half. It fails closed (an unknown identity is not Current), but coverage is incomplete. | Before F2, name one design owner to specify the successor-identity reader contract. It covers receipt validation, the standing basis (`checks_passed` against invocation-bound replay), cross-language parity without kernel replay, the fresh sets and the schema branches, with shared case files as in D2 §4.7. |
| V1-S4 | SHOULD-FIX | D1 §5 item 1 (l.495); D2 §5 I-6 (l.411) | D1's receipt carries "pivot margin minimum". In D1's own probe that is 3.4e16 to 1.3e140 (`probe_skew_precision.stdout.json`), and at p = 128 it is typically about 1/(64·m·2^-128), far above 2^53. Every binary64 value that large is integral, so the checked profile refuses it (see Q3). | If the W1 receipt is hashed like today's receipts (`openpipestress_jcs_ijson_v1`), almost every W1 receipt fails. D1 says there is no invocation-level failure path, but does not say what a per-case hashing failure does. | D1 names the receipt's canonical profile. It encodes such fields safely: as bit strings, like `source_receipt.rs:36-38` `bits()`, as a log2 margin, or under the binary64 profile. It states the per-case outcome when hashing fails. |
| V1-S5 | SHOULD-FIX | D2 §3.1 (l.41), §3.6 (l.115-119), §4.6.2 item 3 (l.365), §6.1 PR-5 (l.427, l.431); `PROBE_PLAN.txt` l.24-26 | Capture hashes the raw request with the checked profile (`PP:1239-1240`, `source_receipt.rs:64-73`). The desktop (`src-tauri/src/lib.rs:1562`) and headless (`headless/src/lib.rs:737`, `:865`) callers propagate the `Err`. T1 is the same (`lib.rs:1414-1415`). PR-5a and PR-5b put 1e16 in the request (`d2_probe/src/main.rs.txt:104-106`). | Any request with a finite \|x\| ≥ 2^53 is refused before solving, on every route. PR-5a and PR-5b will fail with `CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT`, not the predicted code. S-D cannot reach them, and §6.1's expectation would fail. | Correct §3.1, §3.6, §4.6.2 and §6.1. Add the capture boundary to T3's range requirement and RF-RANGE cases, with a named owner (it is a solve entry in `product_physics`, not a T6 carrier). Rebuild the receipt-hash fallback probe with request values ≤ 2^53 − 1 and a published value ≥ 2^53, for example a large stress in Pa. |
| V1-S6 | SHOULD-FIX | D2 §4.2.1 host-rounded fields (l.213), DD-4 (l.511), §6.2 S-E control (l.441) | Acceptance is "within one ulp of the reader's own `exp`". Two faithful `exp` results, from the producer host and a reader host (glibc, macOS libm, V8), can each be one of the two floats bracketing the true value. So a recorded value one ulp from the producer's can be two ulps from a reader whose libm rounds the other way. | Outcomes depend on host and language for adversarial or tampered inputs: the native Rust gate can accept while TS refuses, which is the parity break D2 §4.2.2 rules out. The "1 ulp accepted, 2 ulp refused, the same in all three languages" control is itself host-dependent. | Make acceptance independent of libm. For example, accept iff the recorded value is a faithful rounding of `exp(arg)`, decided against an exact enclosure computed with integer or rational arithmetic in each language. Or take DD-4's third option (logarithmic-law cases stay `needs_recompute`). |
| V1-S7 | SHOULD-FIX | D1 §1 (l.34), §4.7 (l.337, l.339, l.343), §5 item 7 (l.505) | `StructuralReport` is published through `{:?}` in every `NUMERICAL_INTEGRITY_CHECKS_PASSED` or `NUMERICAL_INTEGRITY_SENSITIVE` message (`PP:877-887`, also `:2041-2058`). §5.7 puts `force_scale_exponent` in "the integrity report". §4.7 says b is "chosen at construction, before any allowance is evaluated" but also "first try b = 0". "Unscaled exactly" does not hold when the unscaled value is subnormal. | Every envelope's bytes would change even at b = 0, contradicting "every current result stays bit-identical" and affecting byte-compared committed raws. The b-selection rule is ambiguous. | Keep b out of the `Debug`-rendered report when b = 0, or declare the byte change and its fixture impact. State the try-b = 0-first rule as normative. Apply D1 §5.6's representability outcome to W2 publication. |
| V1-S8 | SHOULD-FIX | D1 §4.1.6 (l.237-246), §1 (l.17) | Acceptance requires \|q_p − q_2p\| ≤ 2^-64·S\*, with S\* body-coupled. This implies 1e-9 relative only for \|q\| ≥ about 5.4e-11·S\*. Check W (N06-class skew, tip moment (m, 2m, 1e-15·m)) did **not** realize a miss: 128 bits was accepted and every nonzero quantity published exactly. | For user models, legitimately small nonzero quantities (RF-WEAK) are not guaranteed to 1e-9 by the rule. The constant is the acceptance threshold for Current, not merely "not a comparison tolerance". | State the guarantee the rule implies. Require VP-ROBUST (with R1 and V2) to check that R1's zero scales are at least about 5.4e-11·S\*, or adopt the per-member option for weakly coupled bodies. Add a weak-coupling control. |
| V1-S9 | SHOULD-FIX | D1 §4.11 (l.478-481), D-2 | The plan tests p = 53 against hardware and p = 64L against Python rational vectors. It lists no targeted hard cases. | Correct rounding and range handling underpin every W1 result. Random vectors rarely hit ties, carry-outs or sticky-bit paths. | Add targeted classes: ties to even at each limb boundary, carry-out renormalization, massive cancellation, exact and near-exact ÷ and √, conversion to binary64 across the subnormal boundary without double rounding, overflow and underflow, and i64 exponent extremes. Add a large seeded differential against the Python `Fraction` oracle, and seeded rounding mutants. With that plan, in-repo is preferable to `dashu-float` (whose correct rounding is equally unverified, and which changes 23 lockfiles). |
| V1-S10 | SHOULD-FIX | D2 §7 (l.470, l.472, l.478) | S-A and S-C both write `result_export/src/physics_source.rs` and `analysis_runs/physics_source.py` (and possibly `physicsSourceRecovery.ts`), yet are scheduled "in parallel and independently". | The pre-merge slices are not disjoint. | Serialize S-C after S-A, or merge them. |
| V1-S11 | SHOULD-FIX | Not in either design; `STAGE0_MAP.md` §2.1 | Nodal loads are summed in binary64 (`primitive_loads/src/lib.rs:1405`, via `PP:1810`). The intended-action audit uses the summed `system.force` (`FK/structural.rs:513-532`). Check L's `ordinary_binary64` rows reproduce the loss (errors 0.5 and 1.0). | A live silent-wrong path on main for Passed cases: an absorbed, cancelled load is published as a zero response, Current. | Add it to T3 scope as an M03 intended-action item: audit load contributions exactly and make a failed audit a W1 trigger (this is the same machinery as V1-B1 (1)). Or record it as a named open item with an owner. |
| V1-N1 | NOTE | D1 §3.1 table column (C) (l.136) | The probe's `method_represented_exact` solves the assembled `represented_f64` matrix (`+=`, including `K[dof][dof] += k`). It is the same computation as the "promoted" column. Rerun with contributions summed exactly: 1.0477e-5, 4.66e-6, 1.075e-3, and nonpositive pivots where D1 reports them (check C). | The conclusion stands. The column label and its evidence misstate what was solved. | Relabel the column, or replace it with the contribution-exact results. |
| V1-N2 | NOTE | D1 §4.3 trigger (l.299) | Negative energy is verified against the stored binary64 matrix (`FK/structural.rs:1287-1327`, `:580-587`). For skewed or bending-soft models the represented matrix can be indefinite while the primitive model is positive definite (D1's own nonpositive-pivot rows). | A stable model could be refused as `NUMERICAL_INTEGRITY_NEGATIVE_ENERGY` with W1 never tried. That is a coverage and label gap, not a wrong value. | Consider triggering W1 when the witness exists only against represented entries, and let W1's check at p against the primitive model decide. |
| V1-N3 | NOTE | D1 D-5 (l.300) | `v1_passed_boundary_probe.*` uses an equilibrated-rcond proxy. In D1's pin family no Passed case misses 1e-9: axis k = 10 is Passed at 1.4e-10, and axis k = 1 is Sensitive at 7.8e-10. | Consistent with D1's deferral to P1. | None beyond P1. |
| V1-N4 | NOTE | D1 §6 (l.517, l.520, l.530) | K2 and K5 both write `SA` and are ordered together. | A write conflict. | Serialize K5 against K2. |
| V1-N5 | NOTE | D1 §4.2 (l.281-282), §8 (l.607) | "Components, releases" and "equivalent static" remain "later" without a named owner. | The closure rule needs a named owner for each remainder. | Name the owning tranche or record them as open. |
| V1-N6 | NOTE | D1 §4.8 (l.396) | The address-space cap relies on `RLIMIT_AS`, which macOS does not enforce. | The host-protection claim holds on Linux only. | State it, or use a macOS-effective guard. |
| V1-N7 | NOTE | D2 §3.4 (l.86) | The scan counts only receipt-bearing envelopes. At T1, the SF-1 fallback raws `result_export/tests/fixtures/load_reference_fallback_uz-*.raw.json` do carry the diagnostic. | "At all" is overstated. The S-A conclusion, which concerns receipt-bearing readers, is unaffected. | Reword. |
| V1-N8 | NOTE | D2 DD-7 (l.514) | Option (ii) removes Current from stored all-selected source-blocks-1 results whose only defect is the conservative summary, which rule binding already refuses. D2 discloses this. | A product choice. | ROOT weighs (i) against (ii). |

## 4. Confirmed independently

- **D1 probe.** It reran byte-identically (Python 3.11.15), and its SHA256SUMS verify.
- **D1's reference.** It equals the exact textbook element for three geometries (check T).
- **The order-≤2 reasoning** (check O).
- **The central claim and the rejection of (C),** with contributions summed exactly (check C).
- **The stop rule on D1's counterexamples.**
- **No automatic dense fallback on main.** `dense_fallback_message` is only `None` (`PP:3418`).
- **SUP-17.** The text at `PP:1362` is as D1 cites.
- **The local stiffness formation order** (`FK/lib.rs:719-726`) matches D1's probe.
- **D2's single-emitter claim.** The one emitter is at `PP:1888-1897` with `affected_refs = [case id]` (T1 `lib.rs:2425-2438`). It fires only in the `Err` arm, so the case is never selected.
- **The D2 scan counts** as scoped: no committed JSON on main contains the code.
- **F1's fidelity to T1's SF-1.** I compared the wrapper (T1 `lib.rs:1445-1467`) and the decline (`:2402-2437`). The republication selects nothing, and each case keeps its ordinary quality under whole-envelope standing. No failed or Sensitive case gains standing, and no case that is Current today loses it. R-1a and R-1b are described correctly.
- **The binding gate.** `result_envelope_binding.rs:258-260` gates on standing only, as D2 says.
- **The checked guards** in all four languages and in receipt hashing (Q3).
- **No protected predicate changes** in either design. D1's 2^-64 constant is a new method-policy acceptance threshold, within ROOT's authority. D2's DD-4 is a verification bound on a producer intermediate, not a comparison criterion.
- **D1's W1a coverage phasing is disclosed honestly** (D1 §4.2, §8).

## 5. Not checked

- **No cargo build, Rust probe or test suite** (host hold). D2's `d2_probe` was not run. The capture refusal (Q3, V1-S5) is established by reading the source, not by execution.
- **Not read:** R1 references (uncommitted work in progress in the worktree; not read, and not yet refuted by V2), `NUMERICAL_REFERENCE.md`, T1's CP2 and CP3 wire records in depth (so not J4's field list against them), `source_receipt/composite.rs::validate_publication`, and the TS reader code for parity.
- **Not verified:** D2's "at most √2" `N_SB` text, D1's crates.io facts, and the DEC-053 figures.
- **The rcond in `v1_passed_boundary_probe`** is a proxy, not the product estimator.

## 6. Run records (`T3/REVIEW/_run_records/`)

- `v1_stop_rule_probe.py.txt` → `v1_stop_rule_probe.stdout.json`: checks T, C, O, L and W. It imports D1's probe unchanged. Run it from that folder with `python3 v1_stop_rule_probe.py.txt`.
- `v1_passed_boundary_probe.py.txt` → `v1_passed_boundary_probe.stdout.txt`.
- `python_version.txt` and `SHA256SUMS`.
- Everything is standard-library Python, single-threaded, run at `nice 19`, under 1 s each.
- Records read: Root `AGENTS.md` (`c8ce87ef342902cb`), `agents/AGENT_TASK.md` (`1a13a5b00b3ce01f`), `_COMMON.md` (`892a2e4e6f8b2e68`), `V1_DESIGN_REVIEW.md` (`87bd120513c58834`), `STAGE0_MAP.md` (`d5af1a0be4fa422d`), `STAGE1_PLAN.md` (`f5c75dcf5bb1bb4a`), the D1 and D2 briefs, `OWNER_DIRECTION.md`, both designs and their run records, and the D2 note at `a68218c81`.
- I made no Git write, changed no product code and did not touch T1's worktree.
