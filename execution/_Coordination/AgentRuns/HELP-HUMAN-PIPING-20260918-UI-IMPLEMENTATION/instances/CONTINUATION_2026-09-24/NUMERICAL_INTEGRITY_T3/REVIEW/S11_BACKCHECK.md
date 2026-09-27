# T3 V1 — backcheck of S11 containment revision 2

Type 2 TASK V1, the reviewer of `REVIEW/S11_CHECK.md`, 2026-09-26. The T3 manager requested this backcheck; ROOT made it top priority.

- **Scope.** Read-only. Standard-library Python only. No cargo, no Git write. T1 was read only through `git show` and `git grep f3270ea79`.
- **Inputs at the T3 branch head `4862a72a9`:**
  - `T3/DESIGN_NUMERICS/S11_CONTAINMENT.md` revision 2 (sha256 `4bec712cdd69a048…`, verified), with revision 1 at `_run_records/S11_CONTAINMENT_revision1.md`;
  - its probes `probe_s11_rev2.*` and `scan_element_loads.*`. SHA256SUMS verify. I reran `probe_s11_rev2.py` and its output is byte-identical;
  - the last three sections of `T3/ROOT_RULINGS_V1.md`: ROOT's rulings on the S11 check, the verbatim no-interim text, and decisions D-S11-1 to D-S11-4.
- **Also used.** As the manager asked, I used DESIGN revision 2 §4.1.2 (at `63c3d503c`) only for the zero-witness question.
- **Line numbers** refer to `c61a540ea` for main and `f3270ea79` for T1.

## 1. Verdict: **BLOCKING**

Revision 2 resolves most of S11-V1 to S11-V7, and its core machinery is sound: the ledger, the exact recovery sums, one rounding function, T1's three sites, and the invariant.

Two BLOCKING gaps remain against ROOT's ruling 1 ("every post-solve recovery sum") and ruling 2 ("bypass impossible by construction"):

- **S11B-1.** The prescribed-motion right-hand side `f − Σ K_fc·g` is still folded in binary64 inside the kernel, after the force leaves the ledger. On T1's 0.4.0 route this publishes a silent error above 1e-9 on the body scale. The M03 guard sits far below its floor there.
- **S11B-2.** Two published cross-source sums of element pressure loads are missing from E1–E14.

Both fixes are small. Everything else is SHOULD-FIX or NOTE.

## 2. Answers

### 1. S11-V1 to S11-V7

| Finding | Status in revision 2 |
|---|---|
| S11-V1 (recovery-side folds) | **Resolved for the sites enumerated** (E1–E14, §2.2 and §4.4). The enumeration is still incomplete: see S11B-1 and S11B-2 |
| S11-V2 (floor; bypass) | **Floor stated** (§5.1). **Construction overstated:** see S11B-3 |
| S11-V3 (granularity) | **Resolved** (§4.2). Curved thermal uses `push_product`, and exact-pressure pushes group operands. One more pre-summing producer input is `pressure_for_pipe` (S11B-2) |
| S11-V4 (T1 sites; one rounding function) | **Resolved**: all three T1 sites (§4.5), and `Expansion::rounded()` removed (§4.1.2). The accumulator's specification needs completing for products (S11B-4). Test F5 needs a precondition (S11B-6) |
| S11-V5 (nonlinear loop) | **Correctly recorded as open with T5** (§10.1, with the call sites) |
| S11-V6 (rationale) | **Resolved.** ROOT's verbatim text is quoted (§2.3), and the count is no longer presented as a filter |
| S11-V7 (disclosure) | **Resolved** (§8.3): disclosure in the change records, the fixture diff with a stop rule, and no in-band marker (D-S11-4) |

### 2. Recovery-sum completeness: three sites D1 missed

**S11B-1 (BLOCKING): the prescribed-motion reduced right-hand side, on the force side, after the ledger.**

- **Where.** `FK/structural.rs:603-606` (`prepare_structural`: `b = force[i]; for (j, u) in prescribed { b -= K[i][j]·u }`), which is the M03 solve's own right-hand side. And `FK/lib.rs:870-877` (`reduce_system_for_boundary`).
- **Behaviour.** Each is a binary64 fold of the ledger's net and every `K_ic·g_c` product. With zero-valued restraints (every pre-0.4 route) it is a no-op. On T1's 0.4.0 route, nonzero support motions are live: `reduce_system_with_prescribed_displacements` in `PP`, and `StructuralSystem.prescribed`.
- **Probe P** (`_run_records/s11_backcheck/probe_s11_backcheck.*`, D1's binary64 element unchanged):
  - **Model.** Two 3 m members. Both end nodes are fully restrained with the same UY settlement g, which is a rigid translation and so no member force. A small nodal moment m sits on the free middle node. The rotation couplings ±6EI/L²·g cancel, and m, which comes first in the fold, is absorbed.
  - **Results.**

| g, m | Folded RHS, exact recovery: member-moment error on the member (body) scale | Exact RHS, binary64 `K_e·u` (the W1 class, out of S11) | Guard ratio / target | Row amplification (guard complete up to 1.2e4) |
|---|---|---|---|---|
| 0.05 m, 0.0137 N·m | **1.21e-9** | 7.6e-10 | 3.4e-4 | 2.6e7 |
| 0.20 m, 0.0137 N·m | **2.57e-9** | 7.6e-10 | 8.5e-5 | 1.1e8 |
| 0.05 m, 1.3 N·m | 1.4e-11 | 1.8e-11 | 3.8e-4 | 2.8e5 |

- **Consequence.** A Passed, Current, silent error above 1e-9 on the body scale. The ratio of 1.3e7 falls inside ROOT's own exposure condition. Neither the ledger nor the site test (which reads `PP` files only) sees it, and the guard is three to four orders below its target.
- **Note.** The W1-class `K_e·u` error in column (b) is of similar size. It is out of S11's scope, but this model is a good W1 and RF-RANGE control too.
- **Required change.**
  - Form the reduced right-hand side exactly, with the same accumulator: the ledger net as terms, plus `add_product(−K_ic, g_c)`, rounded once. Do this in `FK` at both sites (T1-disjoint, so it can join S11-K dormant), and wire it in S11-F.
  - Add this model to S11-F's tests on the 0.4.0 route, with a mutation that restores the fold.
  - T1's legacy observation lane (`observation_force`, the same fold) feeds the protected DEC-050/053 observations only. Leave it as it is, and say so.

**S11B-2 (BLOCKING): two published cross-source sums of element pressure loads.** These fall under ROOT's ruling 1: "every post-solve recovery sum that folds element loads".

- `PP:10780-10802` `pressure_for_pipe`: `pressure += load.magnitude.value` over every pressure load on the pipe. It feeds the published hoop and longitudinal pressure stresses at ends and stations (`PP:2558-2569`, then `recover_section_stress`). T1 has it at `lib.rs:11400-11418`.
- `PP:8368-8382` `append_expansion_joint_pressure_thrust_results`: `entry.axial_load += load.axial_load` over the pressure loads on a joint. It is published as the result value `expansion_joint_pressure_thrust_load_review` (`PP:8393-8396`). T1 has it at `lib.rs:8974-8992`.
- **Required change.** Add both as E15 and E16 (exact sum of the per-load values, rounded once, in S11-F), with tests.
- **Checked and not a miss:**
  - `open_formula_summary_mpa` (`PP:8971-8995`): a two-term signed sum, then the non-cancelling branch of the max;
  - the per-load local transform (`PP:7477-7479`): formation;
  - the membrane `fused_mul_add` (`pressure_exact/source_geometry.rs:135-165`): one rounding of two operands;
  - `stress_recovery::summarize_components`: not published by `PP`;
  - the lumped lever-rule conversion (`primitive_loads/src/lib.rs:2188-2219`): one contribution per load per node;
  - T1's product-physics additions: its only new load sums are the eigen fold (named) and the observation lane above.

### 3. ExactAccumulator: is it correctly rounded?

**The design is sound, but its specification is incomplete for products (S11B-4, SHOULD-FIX).** I checked it with a reference implementation (probe X): an integer fixed point at quantum 2^-2148, 68 × 64-bit limbs, nearest-even projection including the subnormal range. It agrees with `Fraction` bit for bit on every case:

- V1's probe-D terms {2^-110, 2^-53, 1} give 1.0000000000000002. The naive sum gives 1.0.
- `[1e8, −1e8]`, `[−0.0]` and `[]` give +0.0.
- `1.3 + 1.35·4.1e7 − 1.35·4.1e7` as exact products gives 1.3.
- The largest products (1.7e308)² cancel exactly.
- Width: 2148 + 2048 + 64 carry bits = 4260, which is at most 4352. So 68 limbs are enough for 2^64 maximal products, and a 106-bit significand product fits `u128`.

What the specification must add:

1. **The quantum is 2^-2148 once `add_product` exists.** `pressure_sum`'s quantum is 2^-1074, and so is its `project`.
2. **Projection must round at the binary64 subnormal quantum.** `pressure_sum::project`'s branch `highest < 52 → f64::from_bits(sign | a[0])` (`pressure_sum.rs:85-87`) is exact only when the quantum is 2^-1074. At 2^-2148 it must round (3·2^-1076 → 2^-1074; a tie at 2^-1075 → 0). D1's "`project` does the same" holds only for sums.
3. **A nonzero net that rounds to zero is not an exact zero.** Correct rounding gives −0.0 for a negative underflow (−2^-1080 → −0.0). The specification says only "exact zero → +0.0". It must choose one of: IEEE signed zero; +0.0 as a stated deviation; or `NonRepresentable`. It must use that choice at every site, because the bit-equality checks see it.

### 4. Bypass by construction

**Partly (S11B-3, SHOULD-FIX).** The private `AssembledForce` and `LoadLedger::finish` prevent a `PP` producer from writing into the typed force. They do not prevent a fold after `values()`, because every kernel, sparse and nonlinear entry still takes `&[f64]`. Three ways through:

- **Kernel-side folds of the force with other load terms.** S11B-1 is a live example. The site test reads only `PP`, `source_recovery.rs`, `source_receipt.rs`, `pressure_runtime.rs` and `self_weight.rs`.
- **A copy.** `let mut f = force.values().to_vec(); f[i] += x; solve(&f)` compiles. The regex `force\w*\s*\[[^\]]*\]\s*[-+]?=` matches only names that begin with `force`. It also misses the `iter_mut` pattern that `PP:1839-1840` uses today, and T1's `observation_force` (`*value -= …`).
- **Post-solve recovery sums outside the E-list** (S11B-2). No type touches them.

**Required change.**
- Make the solve entry points (`FK` `prepare_structural`, `reduce_system*`, the sparse and nonlinear seams) take an `FK`-owned force type built only by the ledger, and move the ledger into `FK`, since `primitive_loads` already depends on `FK`. Or keep `&[f64]`, but move every force-with-load fold into `FK` through the accumulator (S11B-1).
- Strengthen the site test: extend it to `FK/lib.rs`, `FK/structural.rs` and `SA`; forbid `values().to_vec()` and `to_owned()` of an `AssembledForce`; forbid `iter_mut`, `+=` and `-=` on any binding derived from it. Otherwise state plainly that the construction is limited to the `PP` seams.

### 5. The invariant and the mutation kills

- **Invariant: confirmed** on D1's rerun, for all nine loadings. C3-detect flags every unfavourable order. C3-full stays within 4.6e-16 of each quantity's own magnitude. In (G, −G, 0.3), neither flags and both are exact.
- **Mutation kills: confirmed as claimed.** They are killed at G ≥ 1e7 in (G, n, −G) and (n, G, −G), and not killed at 1e5 (worst 3.9e-11) or 1e6 (6.2e-10).
- **Two margins to fix** (S11B-5, NOTE):
  - At G = 1e7 the root-shear margin is only 1.24 (1.24e-9). The 6.8e-9 kill comes from the midspan moment.
  - M1 ("E1 or E4") needs one mutant per site. With E1 alone restored, G = 1e7 is killed only by root shear, at 1.24×.
  - So make G = 1e8 and G = 1e80 the required kill set, and give each E-site its own mutant.

### 6. The guard floor statement

- **Correct as algebra, and correctly scoped** to "a published quantity governed by that DOF's net". The guard is silent iff `e_i ≤ 64·γ(m_i)·d_i`, which bounds that quantity's relative error by `64·γ(m_i)·d_i/|f_i|`.
- **Relation to R1's net scale.** It is stated properly (§5.2): under C3-full, acceptance does not depend on d_i. That holds only where every load term is ledger- or recovery-routed, and S11B-1 is a case where it is not (amplification 2.6e7 to 1.1e8).
- **NOTE.** The design quotes two completeness limits: m = 92 → 6.54e-13 → about 1.5e3 in the text, and m = 2k + 2 → 4.26e-14 → about 2.3e4 in the probe. State the product's m.

### 7. T1 sites, F5, and the composite `Err`

- **Sites.** All three are named with the right change (§4.5). Replay is in-run only (`source_receipt.rs:472`, `composite.rs:692`), as D1 says. T1's eigen fold uses `load.axial_load * local_x[axis]` (`source_recovery.rs:1272`), which matches the ledger granularity of §4.2.
- **F5 (S11B-6, SHOULD-FIX).** "Eigen terms … and a 1.3 N nodal load, in that order" cannot be authored. The order is fixed by the producer: the nodal fold first, then the eigen pairs (`source_receipt.rs:218-219`; `source_recovery.rs:1270` "after the nodal fold"). The unfavourable (1.3, +N, −N) occurs at the interior node of two colinear members. F5, and likewise F4, F6 and K4, must **assert as a precondition** that the binary64 fold differs from the correctly rounded net. Otherwise the test can pass vacuously and fail to kill M8 or M9.
- **The composite `Err`.** No path found, provided all four sites use the one function. S11B-1 is an ordinary-route value error, not an `Err` path: the exact context handles `K_fc·g` exactly and checks only `system.force` coverage. Receipt hashing of values ≥ 2^53 remains V1-S5's separate item (D2 and T6).

### 8. ROOT's D-S11-1 boundary, and the narrower rule in DESIGN §4.1.2

**The boundary holds only under DESIGN's narrower rule. S11 §4.1.3 and test K5 need an erratum (S11B-7, SHOULD-FIX). Accept D1's offer.**

- **FK:554, the intended-action `ResidualRow`.** This is inside ROOT's boundary.
  - The row is consumed only by its `Debug` rendering in diagnostic messages (`PP:883`, `:2046`, `:2057`) and by the gate through `r.abs()`.
  - The other residual consumers (`structural.rs:970`, `product_equilibrium.rs:66`) read the original-residual rows, not these.
  - `OrdinaryAttempt` carries only the diagnostic reference (`source_receipt.rs:276-299`).
  - The rendered text does enter the envelope that `publication_sha256` hashes. That digest is taken over the published bytes as produced and re-hashed from the same bytes, so a deterministic witness (the constant −0.0 for an empty expansion) cannot cause a mismatch.
  - **Boundary wording to adopt:** "never in a value that is independently recomputed and compared (fold and coverage checks, replay, `rows.rs` `compare`), and never in a receipt field; digests over rendered published text are consistent by construction".
- **E3, E4 and E5 (S11 §4.1.3, K5).** These are **outside** the boundary.
  - They are published result values.
  - For selected cases, published rows and derived rows are compared bit for bit with rows rebuilt from retained recovery (`source_receipt/rows.rs:306-307`, used at `:604` and `:625`).
  - Downstream sums consume them (E6 takes E5's output).
- **D1's evidence for them is misattributed.**
  - The committed −0.0 member stresses in `source_blocks/multicase-dense_scrutiny.raw.json` are **5** rows, not 7.
  - All five are `end_i` stress components of the two **selected** cases (`retained_source_blocks_exact_v1`). Their values come from the retained path, not from E3–E5, and are verified by the receipt. They are not j-side negations either.
- **Erratum content.**
  - Drop the witness at E3, E4 and E5: they use +0.0.
  - Rewrite K5 accordingly.
  - Let the committed-fixture diff and its stop rule report any sign change there to ROOT.
  - Keep the witness only at FK:554, as DESIGN §4.1.2 does.

### 9. D1's new findings

- **N-S11-R: confirmed, with a count correction.**
  - `FK/structural.rs:554` is `let r = residual.rounded();`.
  - An exact-zero residual is the empty expansion. The committed −0.0 strings show that `iter().sum()` gives −0.0 on the pinned toolchain, so the sign depends on the standard library's float-sum starting value.
  - `grep` finds **245** `normalized_residual: -0.0` values under `P/fixtures`, in **31 files**. **37** fixtures carry intended-action rows at all. "37 fixtures, 245 strings" is right if read that way.
  - Low severity, as ROOT ruled.
- **Formation silent zero (`FK/lib.rs:717-726`): confirmed, with one addition.**
  - `validate_positive_finite` checks the inputs only (`:699-705`). The products and quotients (`e·a/L`, `12·e·iy/L³`, `L³`, …) can underflow to 0 or to a subnormal with no error.
  - A subnormal result is also a silent loss of precision, not only a zero.
  - The coefficients underflow at different thresholds (`12EI/L³` against `4EI/L`), so a partial underflow changes the element's stiffness relations, not only its scale.
  - W2's refusal of subnormal stiffness entries (`DESIGN` §4.7) does not catch an entry that underflowed to exactly 0.
  - Realistic SI inputs stay far from underflow. RF-RANGE and PHYS-R4-type inputs reach it. K2's checked formation must test every intermediate product and quotient for a zero or subnormal result from nonzero operands.

## 3. Findings

| ID | Severity | Where | Evidence | Consequence | Required change |
|---|---|---|---|---|---|
| **S11B-1** | **BLOCKING** | §2.2 E-list, §4.3; `FK/structural.rs:603-606`, `FK/lib.rs:870-877` | Probe P: 1.21e-9 and 2.57e-9 on the member (body) scale; guard 3.4e-4 and 8.5e-5 of target; amplification 2.6e7 to 1.1e8 | A silent error above 1e-9, published as Current, on T1's 0.4.0 prescribed-motion route after S11-F | Exact reduced right-hand side (ledger terms plus `add_product(−K_ic, g_c)`, rounded once) at both `FK` sites. Test and mutation on the 0.4.0 route |
| **S11B-2** | **BLOCKING** | §2.2 E-list (ROOT ruling 1) | `PP:10780-10802` → published pressure stresses; `PP:8368-8382` → published `expansion_joint_pressure_thrust_load_review` (T1 `:11400`, `:8974`) | Two published cross-source element-load folds are unrepaired | Add E15 and E16, each an exact sum rounded once, in S11-F, with tests |
| S11B-3 | SHOULD-FIX | §4.3 | Solve seams take `&[f64]`; the regex is name-based; `iter_mut` and copies are not caught; the test does not read `FK` | "Bypass does not compile" is overstated | An `FK`-owned ledger-built force type at the solve seams, or every force fold done in `FK` through the accumulator; a strengthened site test; or state the limit plainly |
| S11B-4 | SHOULD-FIX | §4.1.1 | Probe X; `pressure_sum.rs:85-87` | With products, the projection must round at the subnormal quantum; the underflow-to-zero result is unspecified | Specify quantum 2^-2148, subnormal rounding, and one rule for underflow to zero at every site. Add probe X's cases to K1 |
| S11B-5 | NOTE | §5.3, §9 K4, M1 | Rerun: the G = 1e7 root-shear margin is 1.24 | Kill margins are thin at 1e7 | Required kill set G = 1e8 and 1e80; one mutant per E-site |
| S11B-6 | SHOULD-FIX | §9 F5 (and F4, F6, K4) | The order is fixed by the producer (nodal first) | A test can pass vacuously | Assert as a precondition that the fold differs from the correctly rounded net |
| S11B-7 | SHOULD-FIX | §4.1.3, K5 against DESIGN §4.1.2 and D-S11-1 | `rows.rs:306-307`, `:604`, `:625`; the multicase −0.0 rows are 5 `end_i` stresses of selected cases | E3–E5 witnesses would sit on bit-compared values; D1's evidence is misattributed | An erratum restricting the witness to FK:554, with K5 rewritten and the stop rule reporting any sign change |
| S11B-8 | NOTE | §5.1 | Two m values in text and probe | The completeness limit is ambiguous | State the product's m |
| S11B-9 | NOTE | §7, the formation silent zero | Counts: 245 values in 31 of 37 files. Formation: subnormal as well as zero, and partial underflow | Map precision | Record in the map; K2 checks every intermediate |

## 4. Confirmed

- D1's probe reruns byte-identically.
- The invariant and the mutation table.
- All §2.1 force-side sites.
- The fourteen E-sites exist as described.
- The guard-floor algebra.
- The single-function rule and `Expansion::rounded()`'s three `FK` uses.
- T1's three sites and the in-run-only replay.
- No new composite-`Err` path.
- D-S11-2 to D-S11-4 are reflected in the slices.
- R10's element-load scan (at most three element loads on one element, and nine pairs with two or more).

## 5. Not checked

- No product build or run, and no fixture diff.
- The `CB` terms API and the curved E8–E11 algebra were checked for structure, not by probe.
- I did not trace the retained path's production of the −0.0 selected-case stresses beyond identifying them.
- The `DESIGN.md` revision 2 backcheck is separate (`BACKCHECK_R2.md`).

## 6. Run records (`T3/REVIEW/_run_records/s11_backcheck/`)

- `probe_s11_backcheck.py.txt` → `probe_s11_backcheck.stdout.json` (probes P and X). It imports D1's `probe_skew_precision.py` unchanged. Run it from that folder.
- Standard-library Python 3.11.15, `nice 19`, under 1 s. Hashes are in `T3/REVIEW/_run_records/SHA256SUMS`.
