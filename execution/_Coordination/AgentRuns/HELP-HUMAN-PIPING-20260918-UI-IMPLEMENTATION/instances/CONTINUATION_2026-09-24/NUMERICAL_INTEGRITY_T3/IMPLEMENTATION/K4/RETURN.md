# I12 return: slice K4 (the W1a kernel method)

> **DRAFT (checkpoint D not reached).** Drafted at `3ed6c0e26` while D1 revision 5a.3 ("the stop rule's resolution floor") is drafted and verified. Every place that waits on it is marked **[pending D1 5a.3]**; checkpoint B and C results are marked **[checkpoint B]** and **[checkpoint C]**. Sections that do not depend on S\* are written in final form.

**Status (at A2): no stop.**
- W1a's kernel method is implemented under `FK/src/structural/retained/`, with no product caller (Q1).
- A combination is its own solve (ROOT's F-1 ruling).
- 89 K4 tests pass (one of them a probe with no assertion yet, §21); K3a's, K3's, K-D5's and `exact_sum`'s tests are unchanged and pass.
- R1's 128 K4 cases through the adapter: 0 reference failures; the not-covered set equals §4.10's list.
- The generator's bit-for-bit emulation of the method (O8) reproduces five retained-state digests.
- **Open by ruling:** the S\* resolution floor (F-2, F-3) **[pending D1 5a.3]**; checkpoints B, C and D.

**Where:**
- Branch `codex/piping-k4-20260928` in `<wt>/k4`, from main `e7d930d49` (K2b merged).
- Checkpoints committed by ROOT: A1 `cef218a10`; A2 `3ed6c0e26`. Later revisions are recorded by ROOT.

**Abbreviations:** `P/` = `projects/chirality-piping/`; `FK/` = `P/core/solver/frame_kernel/`; `T3/` = the NUMERICAL_INTEGRITY_T3 folder; `K4T/` = `FK/tests/retained_k4/`; `GEN` = `K4T/gen_k4_vectors.py`.

## 1. Brief, basis, delegation and rulings

- **Brief:** `T3/TASK_BRIEFS/I12_K4_IMPLEMENTATION.md` (`55e5c5a2…` at dispatch; `740c380a…` with ROOT's later amendments, on the numerics branch at `2f48511b7`), with its "ROOT rulings for this slice" Q1–Q12.
- **Basis** (read in the brief's order; hashes checked where pinned): Root `AGENTS.md`; `agents/AGENT_TASK.md`; D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`), §4.1, §4.3, §4.10, §5 items 1 and 7, §6's K4 row, §7.3; K3's, K1's, K2b's and K-D5's records as the brief lists; R1's `references.py` (`80d473a7…`) and `references.json` (`7b176dbb…`); `floor_kinds.json` (`83265615…`); the code on the base. `GEN` pins and checks each input's sha256 before running.
- **Delegation mechanism:** a delegated-harness-native background subagent (D-GOV-35) of ROOT's HELP_HUMAN session, started with the brief and resumed by ROOT's messages at each checkpoint; each return went back through the subagent hand-back to ROOT. I did not delegate. I made no Git writes and no index operations; ROOT committed and pushed.
- **ROOT's rulings applied:**
  - the brief's Q1–Q12;
  - "K4: Q5 amended" (F2a's limits come from K6 and V-K; K4 ships the mechanism only);
  - "K4: rulings on I12's checkpoint-0 plan" (`bacf939a8`): the plan (`e1253faa…`, 591 lines) approved as written; O1 (Q6 amended); O2; O5; O8; O11;
  - "K4: A1 findings F-1 to F-3, the stop rule's blind spot" (`2f48511b7`): F-1 option (a); F-2 and F-3 routed to D1 revision 5a.3; no test pins their present behaviour; K4 does not reach B until the addendum is implemented;
  - ROOT's relayed A2 rulings: the combination's own schedule from 128 is confirmed and **O7's start rule is superseded**; the constructed-ceiling combination and K4-M24's kill are deferred to the addendum (§8.4); the prescribed-tail instance is an F-2-class case for the addendum (§8.5); the NP-A basis is stated (§12.5).
  - D1 revision 5a.3 **[pending D1 5a.3]**.

## 2. Files and line counts (against `e7d930d49`, at `3668ee8a4` plus the prescribed-row fix and the V4-S3 probe)

| File | Change | Lines |
|---|---|---|
| `FK/src/structural/retained/wide_sum.rs` | new: the correctly rounded exact multi-term sum (Q2(a)) | 503 |
| `FK/src/structural/retained/source.rs` | new: `PrimitiveSource`, validation, bodies, encodings | 743 |
| `FK/src/structural/retained/ledger.rs` | new: the exact ledger and its projection | 202 |
| `FK/src/structural/retained/assemble.rs` | new: formation, assembly, reduction at p | 559 |
| `FK/src/structural/retained/factor.rs` | new: geometry first, RCM, the p-factor and its screens | 763 |
| `FK/src/structural/retained/recover.rs` | new: layout, recovery, publication, the state encoding | 504 |
| `FK/src/structural/retained/adaptive.rs` | new: schedule, stop rule, S\*, classification, budgets, reuse, evidence, the prescribed rows' publication **[pending D1 5a.3]** | 2,149 |
| `FK/src/structural/retained/combine.rs` | new: combinations as their own solve | 133 |
| `FK/src/structural/retained/mod.rs` | +15: the `pub(crate) mod` lines and documentation | 39 |
| `FK/src/exact_sum.rs` | +19: the `net_parts` accessor (Q3) | — |
| `FK/tests/s11_site_table.rs` | +61: K4's eight files in SOURCES and 25 rows (Q8) | — |
| `K4T/{wide_sum,ledger,source,assemble,factor,adaptive,recover,combine,references,classification}_tests.rs` | new: 16, 7, 4, 9, 9, 19, 5, 10, 6, 4 tests | 538, 372, 243, 724, 436, 942, 253, 392, 589, 219 |
| `K4T/support.rs`, `K4T/models.rs` | new: test-only SHA-256, SplitMix64, token helpers; model parsing and the comparison | 518, 362 |
| `K4T/gen_k4_vectors.py` | new: the standard-library generator, with `--check` | 1,923 |
| `K4T/*.txt`, `K4T/SHA256SUMS` | new: generated vectors (§11.1) | 44,185 |

- **Size:** the vector data is about 4.8 MB (R1's cases 1.0 MB, the samples 1.3 MB each).
- **Unchanged:** every manifest and lockfile; K3a's and K3's code, tests, generators and vectors; `wide.rs`, `multi.rs`, `structural.rs`, `lib.rs`, `sparse.rs`, `load_ledger.rs`, `rigid_body.rs`, `exact_boundary.rs`, `formation_check.rs`; SA, PP, NI, `sparse_direct`; the fixtures and R1's references.
- Recounted at the candidate **[checkpoint D]**.

## 3. How each K4 item was met (D1 §4.1; the brief's "What K4 adds")

1. **`source.rs`: `PrimitiveSource`,** "an immutable per-case declaration with an identity digest" (§4.1.1).
   - Nodes (binary64), `StraightMember {id, node_i, node_j, E, G, A, Iy, Iz, J, y_reference}`, springs `{id, dof, k > 0}`, the kernel-only `DirectionalSpring {id, node, kind, direction, k}` (Q6), constraints `{dof, value}`, nodal loads `{dof, value, source_id}`, stations `{id, member, fraction}` and support groups (O2, O3).
   - "Only supported families can be constructed": the type admits nothing else. "Validation rejects nonfinite or nonpositive properties and incomplete partitions. It also rejects any derived primitive that is subnormal": `SourceError` names each refusal (22 variants); a zero-length or degenerate axis is decided exactly (no tolerance).
   - Canonical order (members, springs, stations and groups by id; constraints by DOF; loads by DOF, source id and value); bodies are the member graph's components numbered by lowest node.
   - The identity is the canonical encoding `K4SRC` (Q10, §13); the factor-reuse identity is `K4STF` (nodes, members, springs, directional springs and the constrained DOF set).
2. **`ledger.rs`: the load ledger.** "Each DOF's load is the exact sum of its identified contributions, rounded once to p", with "the same contribution granularity" as S11-F's ledger: one `ExactAccumulator` per DOF, one term per contribution. "`ExactAccumulator` … gains one further projection, to `Wide<L>` at p": `net_parts` (Q3) netted once with the file's own compare and subtract, then K3's `from_integer`. **Wherever the ledger joins another sum** (the reduced rhs, a residual, a reaction, a combination) it enters exactly through `add_to`, never as a value already rounded to p. The ledger's canonical encoding is `K4LED`.
3. **The correctly rounded exact multi-term sum** (`wide_sum.rs`): the contract of the brief's item 3, argued in §5.
4. **`assemble.rs`.** The frame as the product's algorithm, "No axis tolerance is used at p"; frame dot products formed exactly (ROOT's approved refinement: strictly more accurate, one rounding per step). B = B_local·T and D; DB one exact expansion per entry and K_e = Bᵀ(DB) one exact expansion per upper entry. "Each pattern entry of K is formed as one exact expansion of its p-bit element contributions and binary64 spring stiffnesses … rounded once to p", on K1's `SparsePattern` (`from_positions`). Directional springs contribute fl(fl(k·n_a n_b)/fl(nᵀn)) (Q6). "Each rhs_i is one exact expansion: the ledger terms plus the exact products −K_ic·u_c … rounded once to p. Neither K nor rhs is ever rounded back to binary64." The prescribed u_c at p is the case's binary64 value, or a combination's exact sum rounded once (F-1).
5. **`factor.rs`.** "Geometry first": FK's `assess_rigid_body` per body before any factor; "A witnessed mechanism is refused and never escalated". Directional grounds as amended by O1 (§6). The ordering is a port of `sparse_direct`'s RCM (Q7) on the structural free–free pattern. Radix equilibration as M03 (s_i = −⌊e(K_ii)/2⌋, exact). The profile LDLᵀ at p, every operation rounded to p. The pivot screen "d_i > 64·γ_p(m_i)·c_i", decided exactly as d_i(2^p − m_i) − 64·m_i·c_i > 0: "A failed pivot at p escalates to the next p … It is never read as a mechanism." Negative energy "for pattern pairs only, at p, against the intended K", decided exactly with the approved allowance. Hager–Higham on the equilibrated K with the p-factor: "rcond ≤ 2^-(p-1) counts as unresolved at p and escalates", published with the label "sensitivity to matrix-entry perturbation, not to authored parameters".
6. **Solve and refinement.** "Evaluate r = f − K u against the intended system re-formed at p + 64"; "Each r_i is one exact expansion of the p-bit products and the ledger terms", rounded once to p for the correction. The gate |r_i|(2^p − m_i) ≤ 64·m_i·d_i is decided exactly (d_i the coalesced |f_i| + Σ|K_ij u_j|, O12). "At most three corrections; then escalate" (a non-decreasing worst row also stops). The ceiling's solve as ruled under Q4 (§9).
7. **`recover.rs`: recovery before rounding.** d_local = T u, e = B_local d_local, Q = D e, end actions B_localᵀ Q; stations (m(t) = t·(M_j + M_i) − M_i, exact); spring actions −k u; directional spring actions −K_s u; reactions K_c u − f_c; node displacement magnitudes and support-group magnitudes. "Each component … is one exact expansion of its (at most five) product terms, rounded once"; each reaction "one exact expansion of K_cj·u_j products and the ledger terms". Each published quantity is rounded to binary64 once with K3's `Binary64Outcome`; an exact zero is +0.0 (Q11). A prescribed row (an input-derived displacement) is published from its exact sum of terms c·v, rounded once to binary64 by the exact accumulator (§8.7), never from its p-rounded value. There is never a silent value: underflow and overflow rows are `Unpublishable` (O9).
8. **`combine.rs`: combinations** (§4.1.1, refined by ROOT's F-1 ruling; §8). A combination is its own solve: the combined exact ledger Σcᵢfᵢ and the combined prescribed values, on its operands' shared stiffness source, reusing their cached factor, with its own schedule and stop rule. "Its published outputs go through the stop rule … with S\* taken from the combination's own body scale"; it "escalates independently of its operand cases"; at the ceiling it is withheld (`CombinationUnresolved`) and "its operand cases keep their standing". The Σcᵢuᵢ formation is withdrawn.
9. **`adaptive.rs`.**
   - **The schedule.** "Candidates at p = 128, 256 and 512, each verified at 2p. The ceiling is 1024 bits." "The verification solve at 2p repeats formation from the binary64 operands." "A rejected 128 candidate's 256 verification becomes the next candidate … at most four solves."
   - **The stop rule.** "|q_p − q_2p| ≤ 2^-64 · max(|q_2p|, S\*)" on every published quantity, decided exactly; S\* per body and kind at 2p with §4.1.6.1's coupling rounded once at 2p **[pending D1 5a.3]**. "A body with all scales zero … must agree exactly."
   - **The classification** on the published value: `absolute_verified` iff |q| < fl(R·S\*), R = 2^-34 (`0x3DD0000000000000`); S\* < 2^-988 makes every row of the body and kind `absolute_verified`; b = fl↑(2^-64·S\*); restrained and prescribed DOFs are `input_derived`; the per-member stress scales with the pinned k constants and k_i = fl↑(k√2·i) (`stress_scale`, `intensified_k`) **[pending D1 5a.3]**.
   - **Failure.** "At the ceiling, or when the budget runs out, the case is unresolved … with the attempted precisions and the reason … Nothing is relabelled as solved."
   - **The evidence** (§5 item 1) as kernel types: the attempts (p, role, outcome, reason, residual basis, corrections, pivot margin, rcond, residual summary, work by stage, shared work, storage counts); the selected and verification p; the stop-rule summary rounded upward; R's bits and S\* per body and kind as bits; `input_derived_dofs`; the `absolute_verified` ids with b; `not_covered` (empty in the kernel) **[pending D1 5a.3]**; `unpublishable`; the pivot margin minimum (rounded downward), rcond, the residual summary (rounded upward); the geometry; the source, ledger and retained-state encodings.
10. **Budgets and work** (§4.1.7; Q5 as amended). "Counted in limb-multiply equivalents per attempt. Successful, failed and verification work is all charged." `CaseLimit` and `InvocationMeter` are required parameters with no `Default` and no numbers. Factor reuse: "Linear cases that share a modulus basis and state share one p-factor per precision": each case's limit counts the full shared work; the invocation meter counts it once (§14).
11. **Determinism** (§4.1.8). "The same `PrimitiveSource` therefore gives a bit-identical retained state on every platform": integer-only arithmetic, canonical order, no allocation-dependent order; the retained-state encoding `K4RST` is "over the canonical limbs of u_p and every member's Q". Tested run to run, under list permutations and under factor reuse; five digests are pinned by an independent emulation (O8).
12. **The opaque result.** `RetainedSolve` is bound to its source and precision; `publish()` returns the once-rounded rows. "No caller can supply a matrix, factor, closure or label": the entry points take only `PrimitiveSource`s, factors of combinations, limits and a meter.

## 4. Positions and refinements as ruled

ROOT approved the checkpoint-0 plan as written (`bacf939a8`). Each position, with the design's words:
- **The multi-term sum** (`wide_sum.rs`): stack magnitudes, a span limit of 8,128 bits that refuses, one rounding through `from_integer` (§5). A span refusal is terminal and the maximum span is recorded (O4).
- **Values only ever widen,** and each lower-precision quantity is rounded once from its exact sum.
- **The pivot screen, the residual gate and the stop rule are decided exactly,** with no binary64 γ.
- **Directed roundings of the evidence ratios:** the stop-rule and residual summaries upward (D2's G5a "≤ 2^-64" stays sound), the pivot margin downward.
- **Radix equilibration as M03,** rcond on the equilibrated K.
- **RCM on the structural free–free pattern** (Q7). V-K's equality test feeds both paths the same adjacency.
- **The negative-energy allowance:** a witness when E < −64·γ_p(4)·(K_ii + K_jj + 2|K_ij|).
- **Frame dot products formed exactly:** a refinement of "every operation rounded to p", strictly more accurate, one rounding per step.
- **Item C at p = 53 on normal results only,** with the exact-projection form added (a p = 53 rounding then a binary64 conversion is a double rounding in the subnormal range).
- **Item E's K-D5 cross-check is a test-only port** (K-D5's functions are private).
- **Factor reuse:** each case's limit counts the full shared work; the invocation meter counts it once.
- **The budget API:** required parameters, no `Default`, no numbers.
- **The canonical encodings** (§13).
- **O2: `SupportGroup`,** so the per-support magnitudes are formed at p and checked by the stop rule; F2a maps the product's supports onto groups.
- **O3: ids** on members, springs, stations and groups.
- **O5: an evidence-level kill of K4-M11** (§7).
- **O6: the K4-M16 control** (REACTIONS-ONLY; the every-quantity control covers spring actions, §11).
- **O7: superseded** by ROOT's F-1 ruling and A2 confirmation: a combination is a case with the net load and prescribed terms and starts where a case starts (128); no operand state is used and no operand is solved on demand.
- **O8: the bit-for-bit emulation** (§11.1).
- **O9: unpublishable rows** are listed, excluded from S\* and the classification; F2a decides their standing.
- **O10** (the conventions of the plan's §8) and **O12** (M03's coalesced residual denominator).
- **O1: Q6 amended** (§6). **O11:** the debug suite grew by about 3.4 minutes (§15), under the 20-minute report threshold.
- **Q5 amended:** K4 is unaffected; it ships the mechanism.
- **RF-MECH-K0:** the adapter passes k = 0 and source validation refuses it (`NonPositiveSpring`); with the zero spring omitted, geometry refuses the mechanism. Both are tested.

## 5. The correctly rounded exact multi-term sum (`wide_sum.rs`): the correctness argument

**Structure.** `ExactWideSum` holds two unsigned magnitudes (positive and negative terms), each `[u64; 128]` on the stack (8,192 bits), an anchor (the exponent of bit 0, `i128`), the used-limb count and the highest set bit. There is no heap allocation, per term or per sum; `clear()` zeroes only the used prefix.

**Terms.** `add_wide` (a `Wide<M>` value through `parts()`), `add_binary64` (an exact lift; NaN and ∞ refused), `add_product` (the exact product of two values of at most the context's precision, through K3's `two_product`: s + e = a·b exactly, both added), `add_integer` (±magnitude·2^e: the ledger's net), `add_wide_scaled` and `add_scaled` (a value or another sum's exact net times a small integer and a power of two, for the exact screens and gates).

**The argument.**
1. **Each term is an exact dyadic integer placed exactly.** A term is trimmed to its lowest and highest set bits; its lowest bit sits at an integer offset ≥ 0 above the anchor after any re-anchoring; the limb-wise add with carry is exact integer addition.
2. **The span check precedes any mutation.** The new span (highest bit − lowest set bit + 1, over the sum and the term) must be ≤ 8,128 bits, else `SumRefusal::Span` and the sum is unchanged. A lowest bit below the anchor shifts both magnitudes up exactly (the span check guarantees the room).
3. **No carry is lost.** Each magnitude holds a sum of terms each below 2^(span); with 64 bits of headroom (8,192 − 8,128) the magnitude cannot exceed its buffer for fewer than 2^64 terms. No K4 sum comes near that count.
4. **The netting is exact:** one compare and one subtraction of the smaller magnitude from the larger give the exact signed value.
5. **The rounding happens once:** `from_integer(negative, &magnitude[..used], anchor)` rounds that exact integer to the context's precision, to nearest with ties to even, decided by every bit however far down (K3, independently reviewed and tested on long magnitudes with far sticky bits and ties). An exact zero gives +0; an empty sum gives +0.
6. **The only inexact exits are refusals:** `Span`, `Exponent` (an anchor outside `i64`, or a result outside `Wide`'s range) and `NonFinite`; none drops a bit.
7. **The span is enough for binary64 inputs:** every nonzero binary64 value and every exact product of two has its leading bit in [2^-2148, 2^2048); a K4 term has at most 2,048 significant bits; so a sum of such terms spans at most 4,196 + 2,048 = 6,244 < 8,128 bits. Larger spans come only from extreme primitives and are refused.
8. **Work** is charged per limb touched, shifted, netted and rounded (`SumWork`), and the context charges `two_product` and `from_integer`.

**Evidence (tests B):** the Fraction oracle on 397 targeted sums at seven precisions (ties decided by far tails, cancellation to +0, gaps beyond the width, every term type, the span boundary 8,128 accepted and 8,129 refused in both orders, the exponent refusal); RV12's counterexample (1 + 2^-127, 2^-128, −2^-400 gives 1 + 2^-127 at 128; a fold misrounds it); 7 × 10^5 seeded differential sums (§15).

## 6. Geometry first with directional grounds (O1): the amended rule and its safety

**The rule (ROOT's O1).** Per node and kind, the directional springs **together with that node's global-axis springs and rigid DOFs of the kind** ground the kind fully when their directions span R³, decided exactly (triple products through exact expansions; `spans_space`). The kind is then passed to `assess_rigid_body` as grounded at the node. A body with any remaining directional ground that does not span is **not assessed geometrically**: no witness is sought and no outcome of `assess_rigid_body` is used. It proceeds as a `NumericallyUnresolved` body does, and its evidence records `BodyGeometry::NotAssessed([(node, kind), …])`.

**Why this is safe (the derivation).**
1. **What does not change.** "A witnessed mechanism is refused and never escalated" (§4.1.3) still holds: such a body is simply not witnessed. Bodies without directional springs, and bodies whose directional grounds span, take the unchanged path. Directional springs are kernel-only; F2a never builds one, so the product's geometry-first path is unchanged.
2. **A stable body that is not assessed** is solved as any other: nothing about its published values depends on the geometric screen. The cost is work only.
3. **A true mechanism that is not assessed.** Let the intended K\* have a null vector z. At p, K_p = K\* + ΔK with |ΔK| ≤ c·2^-p·|K| componentwise (c the formation's rounding count), so σ_min(K_p) ≲ c·2^-p·‖K‖ and κ(K_p) ≳ 2^p/c. Then at every p one of these happens:
   - **the pivot screen fails** (a pivot of order σ_min is below 64·γ_p(m)·c_i), and the attempt escalates;
   - **the condition screen fails** when the estimate sees κ: rcond_est ≤ 2^-(p−1). Hager–Higham estimates ‖K^-1‖₁ from below, so rcond_est ≥ the true rcond; the screen is sure to fire only if the estimate is within a factor of about c/2 of the truth (**the uncertified step**, as everywhere in M03);
   - **otherwise the stop rule rejects.** If the load excites z (zᵀf ≠ 0), the solution's component along z is about zᵀf/σ_min(K_p), which grows like 2^p: q_p and q_2p differ by a factor near 2^p. If it does not, the component along z is rounding noise amplified by 1/σ_min: O(|u|) at p and a different O(|u|) at 2p, since the two formations and solves round independently. Either way |q_p − q_2p| is of the order of the values, far above 2^-64·S\*.
   - At the ceiling there is no candidate left: the case is `Unresolved(Ceiling)` and nothing is published.
4. **The residual risk** is that the p and 2p solutions agree along z to within 2^-64·S\* by coincidence, for every candidate. The two solves have independent roundings of order 2^-p and 2^-2p relative to K, so such agreement needs the noise components to match to 64 bits; no mechanism makes that systematic. This is an argument, not a proof, and it relies on the same stop-rule premise as `NumericallyUnresolved` bodies **[pending D1 5a.3: restate if the addendum changes S\* for the kinds involved]**.
5. **Evidence:** a cantilever whose root is grounded in-plane by one directional spring (with the rigid UZ) is `NotAssessed` and ends `Unresolved(Ceiling)` after escalating stops at 128, 256 and 512; a second, spanning direction makes it `Restrained` and selected at 128. RF-SKEW-T-PIN-AX's six cases solve, so RF-SKEW is 36 compared.
6. **Routed:** a full geometric treatment of partial directional grounds belongs to W4/K5 (§4.9).

## 7. The p + 64 residual (O5): the published-value equivalence of K4-M11, and its kill

**K4-M11** forms the residual against K_p (the factor's own K) instead of K_q re-formed at q = p + 64.

1. **What refinement converges to.** With the residual against K_r, iterative refinement with the p-factor converges (when κ·2^-p ≪ 1) to u_r\* = K_r^-1 f, the solution of the system the residual uses. The correct variant converges to u_q\* with |u_q\* − u\*| ≈ |K^-1 ΔK_q u| where ΔK_q = K_q − K\* is formation error at 2^-q; the mutant converges to u_p\* with the formation error at 2^-p.
2. **When they differ in the attempt.** The gate at p compares |r|(2^p − m) with 64·m·d. The mutant's residual never sees K_p's formation error (u already solves K_p to the gate's accuracy), so it makes no correction. The correct residual sees (K_q − K_p)u; that exceeds the gate only on rows whose assembled entry cancels its contributions while the row's other terms are small. TWO-SPAN (two collinear spans of lengths 1 and 1 + 2^-40, a moment at the shared node) is such a row: the correct variant corrects once at 128 and once at 256; the mutant corrects zero times.
3. **When they differ in the published values.** Both variants publish only an accepted candidate: |q_p − q_2p| ≤ 2^-64·max(|q_2p|, S\*). The verification value q_2p is within about κ·2^-2p of the truth in both variants (its own formation error; both variants treat 2p alike). So each accepted q_p lies within 2^-64·max(|q_2p|, S\*) + κ·2^-2p of the truth. Rounding to binary64 (a relative 2^-53 step) maps two such values to the same binary64 number except when a rounding boundary lies between them: they then differ by one ulp. **So the variants' published values agree to within one binary64 ulp and within 2^-64·S\* + 2^-53|q| of each other, far below the 1e-9 criterion; no published-value test at the criterion distinguishes them.**
4. **Availability.** The mutant's q_p carries the p-formation error, so its acceptance needs κ·2^-p ≲ 2^-64: it may escalate where the correct variant accepts. It never publishes a value outside the acceptance bound. So the mutant is equivalent at the criterion and differs in evidence and availability only.
5. **The kill (ROOT's O5).** On returned attempt evidence, which F2a publishes: TWO-SPAN's attempts are (128, residual basis 192, 1 correction) and (256, 320, 1) (`the_p_plus_64_residual_takes_one_correction_on_two_span_and_records_its_basis`); the golden work pins its refinement and residual-formation work (128: 37,577 and 57,274; 256: 38,808 and 187,120 limb-multiply equivalents; `golden_work_counts`). The mutant forms no K_q (residual formation drops) and corrects zero times.

## 8. Combinations (ROOT's F-1 ruling)

1. **The finding.** Σcᵢuᵢ formed from the operands' retained states loses operand loads that cancel below both p's and 2p's resolution, and the stop rule cannot see a loss common to both precisions. CEIL-A − CEIL-B (loads differing by a relative 2^-1060) was selected at 128 with every row an exact 0, classed `AbsoluteVerified` with bound 0; the truth is nonzero.
2. **Why (a) is the same combination.** With one linear K and linear recovery, the exact state is linear in the loads and prescribed values: u(Σcᵢfᵢ, Σcᵢvᵢ) = Σcᵢ·u(fᵢ, vᵢ), and every recovered quantity (reactions include −f) is linear in (u, f). So solving the net case equals combining the operands' exact results: the two formulations agree in exact arithmetic (§4.1.1, §4.1.2 "formed exactly and rounded once", within their intent).
3. **Why (a) avoids the loss.** The combined ledger holds the exact products cᵢ·v of every operand's load terms (`RetainedLedger::combined`); each prescribed value is the exact sum of cᵢ·vᵢ rounded once at p (`CasePrep::combination`, `prescribed_at`). So the combination's right-hand side is "one exact expansion of the combined ledger Σcᵢfᵢ and the combined prescribed coupling, rounded once to p", and its errors are those of its own solve, relative to its own values. It reuses the operands' cached factor (merged caches, including cached factor stops) and runs the case schedule from 128 with its own stop rule. The operands are borrowed and never change.
4. **The constructed ceiling (deferred to the addendum; K4-M24).** Under (a), a combination's outcome is the outcome of its net case. A conditioning-limited body gives a relative error of about κ·2^-p in every kind that carries values, for any nonzero load on it (δu ≈ K_p^-1 ΔK u: its component along the softest direction is about 2^-p·‖K‖·|u|/σ_min). So operands accepted at p ≤ 512 imply κ·2^-p ≲ 2^-64 for their K, and the net case, on the same K, is accepted by 512 too. A combination at the ceiling with every operand selected therefore needs a kind whose S\* is set by values far below its elastic-action scale: **the F-2/F-3 class**. The `Ceiling → CombinationUnresolved` mapping is one match arm; withholding with attempts and unchanged operands is tested through a budget. The constructed ceiling and K4-M24's kill wait on the addendum **[pending D1 5a.3]**.
5. **The prescribed-tail instance (an F-2-class case, routed by ROOT to the design task).** A combined prescribed value whose exact sum needs more than 2p bits (for example 1 + 2^-1100 from operands prescribing 1 and 2^-1100) rounds identically at p and 2p, so a loss below both resolutions is again invisible to the stop rule **[pending D1 5a.3]**.
6. **Evidence (tests I, continued in item 7):** CEILING equals CEIL-NET bit for bit (spr.3.3 = −2^-60; the withdrawn Σcᵢuᵢ gives zeros); B1-C and B1-E equal their net cases bit for bit and B1-E's truth is reproduced where Σcᵢuᵢ at 128 is off by more than 1e-9; 2·SKEW6 − SKEW6 is rejected at 128 by its own stop rule; SKEW6 − SKEW6 is selected at 128 below its operands' 256; prescribed values combine exactly; cached factor stops and builds are reused and the invocation is charged only the combination's own work; invalid combinations return their reasons.

7. **A combination's prescribed rows, rounded once (V4's NOTE on 5a.3; ROOT's rulings on V4's verification, `085638e58`).**
   - **The defect (at `3ed6c0e26` and `3668ee8a4`):** a combination's prescribed value was rounded once at p (`prescribed_at`), and its published row was then that p-bit value rounded to binary64: a double rounding. Operands prescribing 1, 2^-53 and 2^-150 at one DOF combine to exactly 1 + 2^-53 + 2^-150; at p = 128 that is 1 + 2^-53, a tie that rounds to even, 1.0. The correct value is 1 + 2^-52.
   - **The fix:** `CasePrep::publish_prescribed` replaces each published input-derived displacement row by `exact_publication` of its exact terms: an `ExactAccumulator` of the products c·v, rounded once to binary64 (`round`), an exact zero +0.0, a nonzero sum rounding to zero `Underflow`, one beyond the range `Overflow`; a subnormal result is labelled through K3's conversion of the (exact) rounded value. The retained state keeps u at p, and the stop rule still compares the p and 2p rows; only the publication changes. A case's single binary64 value publishes unchanged.
   - **The control:** `a_combined_prescribed_value_is_published_from_its_exact_sum_rounded_once` (`combine_tests.rs`) combines three PRESCRIBED variants (1, 2^-53 and 2^-150 at node 2's UY). On `3668ee8a4`'s code it fails (published `Normal(1.0)`, expected `Normal(1.0000000000000002)`); with the fix it passes, selected at 128, and the single case's row stays 1.0.
   - **The mutant:** K4-M33, "a prescribed row published from its p-rounded state", killed by that control (§17).

## 9. The ceiling (Q4(a)): the forward-error argument

At 1024 (verification-only) the residual is formed against K_1024 itself (no p + 64), as one exact expansion rounded once, gated at 64·γ_1024, with at most three corrections; the attempt records its residual basis as 1024.

**Route 1 (perturbation bound).**
1. **What the verification must deliver.** q_1024 is only a comparator for the 512 candidate; acceptance needs |q_1024 − q\*| ≪ 2^-64·max(|q\*|, S\*).
2. **Formation.** K_1024 = K\* + ΔK_f with |ΔK_f| ≤ (c_f + 1)·2^-1024·|K|_contrib componentwise, where |K|_contrib is the magnitude of the entry's contributions (for an element entry Σ_r Σ_s |B_ra||D_rs||B_sb|), c_f is the element formation's first-order error coefficient counted in §9.1, and the assembled entry adds one rounding. The ledger is exact, so f has no formation error.
3. **Backward error.** The exact gate bounds |f − K_1024 u|_i ≤ 64·γ_1024(m_i)·(|f_i| + Σ_j |K_ij||u_j|).
4. **Combined:** u_1024 solves (K\* + ΔK)u = f + Δf with |ΔK|, |Δf| ≤ ε·(|K|, |f|), ε ≈ (64m + c_f + 1)·2^-1024.
5. **Perturbation bound** (Oettli–Prager, Skeel): ‖u_1024 − u\*‖/‖u\*‖ ≲ κ(K\*)·ε/(1 − κ·ε) in the equilibrated norm.
6. **The uncertified step.** The 512 candidate passed rcond_512 > 2^-511, so est(κ(K̃)) < 2^511. Hager–Higham is a lower-bound estimator of ‖K^-1‖ and is not certified; the argument assumes the true κ exceeds the estimate by at most a factor F.
7. **Result:** forward error ≲ F·(64m + c_f + 1)·2^-513; for m ≤ 2^20 that is ≲ F·2^-486, a 2^50 margin under the stop rule's threshold for every F < 2^372.
8. **From u to the published quantities:** recovery at 1024 is exact expansions rounded once, so each quantity's error is its linear image of u's error plus 2^-1024 relative; the kind-coupled S\* bounds those images body-wise. The mixed-unit step (equilibrated norm to per-kind body scales) is where this is an argument, not a proof.

**Route 2 (first-order comparison).**
9. Both the 512 and 1024 solves are dominated, to first order, by perturbations proportional to their unit roundoffs (formation and backward error), so err_1024 ≈ 2^-512·err_512 in the first-order regime, and acceptance bounds err_1024 ≲ 2^-512·2^-64·S\*/(1 − 2^-512). The first-order regime is κ·2^-512 ≪ 1, which rcond_512 > 2^-511 **suggests but does not certify** (the uncertified step of this route). Near that boundary the formation perturbation at 512 can hide a singular intended K; then the 512 solve is wrong at O(1) and in a different direction from the 1024 solve, and the rule rejects.
10. **[pending D1 5a.3] The step that relied on §4.1.9.** The plan's step 10 said the stop rule can accept a wrong 512 candidate only through a precision-independent (common-mode) error, which revision 2's exact sums remove (§4.1.9), and route 2 assumed "no common-mode term". F-2 and F-3 refute §4.1.9's claim as stated: a precision-dependent loss below both p's and 2p's resolution passes too. This step is restated once the addendum's resolution floor is selected.

### 9.1 c_f: the formation's roundings, counted from `assemble.rs` (at `3ed6c0e26`)

Notation: u = 2^-p; every operation named below rounds once to p, to nearest; `lift` of a binary64 input and the D doublings (`mul_pow2`) are exact; each "exact expansion" (`ExactWideSum`) rounds once. The first-order bounds ignore O(u²) terms and assume no underflow or overflow. θ is the angle between `y_reference` and the chord (the source refuses only θ = 0, decided exactly, so the frame's errors grow like 1/sin θ for a nearly parallel reference).

| Step | Code (`FK/src/structural/retained/assemble.rs`) | Roundings in the step | Longest rounding chain | First-order relative error |
|---|---|---|---|---|
| d_k = x_j,k − x_i,k | :166–171, one exact expansion of two binary64 values | 1 | 1 | ≤ 1u (of d_k) |
| s_d = d·d | :172 → `normalize` :138 → `dot` :113–127, one exact expansion | 1 | 2 | ≤ 3u (nonnegative terms: 2u from d, 1u) |
| L = √s_d | :139 `ctx.sqrt` | 1 | 3 | ≤ 2.5u |
| e_x,k = d_k/L | :142–144 `ctx.div` | 1 | 4 | ≤ 4.5u |
| proj = y_ref·e_x | :178 `dot` | 1 | 5 | ≤ 5.5u of Σ\|y_ref,k e_x,k\| ≤ \|y_ref\| |
| yc_k = y_ref,k − proj·e_x,k | :180–185, one exact expansion | 1 | 6 | ≤ (10/sin θ + 1)u of \|yc\| |
| e_y = yc/\|yc\| | :186 → :138, :139, :142–144 | 3 | 9 | ≤ (10/sin θ + 4.5)u (norm-wise) |
| zc = e_x × e_y | :187–193, one exact expansion per component | 1 | 10 | ≤ (10/sin θ + 10)u |
| e_z = zc/\|zc\| | :194 → :138, :139, :142–144 | 3 | 13 | ≤ (10/sin θ + 13.5)u |
| 1/L | :195 `ctx.div` | 1 | 4 | ≤ 3.5u |
| E·A, G·J, E·I_z, E·I_y | :205–206, one exact expansion (one product) | 1 | 1 | ≤ 1u |
| EA/L, GJ/L, EI_z/L, EI_y/L | :207 `ctx.div` (:209–213) | 1 | 5 | ≤ 4.5u |
| B: ±e_x,k, e_y,k, e_z,k | :219–231 (copies and negations) | 0 | 4, 9, 13 | 4.5u, (10/sin θ + 4.5)u, (10/sin θ + 13.5)u |
| B: ±(1/L)·e_y,k, ±(1/L)·e_z,k | :217–218 `ctx.mul` | 1 | 10, 14 | ≤ (10/sin θ + 9)u, ≤ (10/sin θ + 18)u |
| D: 4c, 2c | :235–238 (exact scalings) | 0 | 5 | ≤ 4.5u |
| DB_rc = Σ_s D_rs B_sc (≤ 2 products) | :247–256, one exact expansion | 1 | 15 | ≤ ε_D + ε_B + 1u of Σ_s \|D_rs B_sc\| |
| K_e[a][b] = Σ_r B_ra DB_rb (≤ 6 products) | :257–266, one exact expansion | 1 | 16 | see below |
| Assembled K_ij | :490–509, one exact expansion of the element entries and the binary64 spring stiffnesses | 1 | 17 | c_f + 1 of the contributions' magnitude |
| Directional block k·n_a n_b/(nᵀn) | :316 (`dot`), :323, :324, :325 | 4 | 3 | ≤ 4u |

**The element entry's coefficient.** The error of K_e[a][b] is at most Σ_r Σ_s \|B_ra\|\|D_rs\|\|B_sb\|·(ε_B,ra + ε_D + ε_B,sb + 1u) + 1u·\|K_e[a][b]\|. The largest pair is two transverse entries of the z-plane or y-plane rows, ±(1/L)·e_z,k with (10/sin θ + 18)u each, so

  c_f ≤ 2·(10/sin θ + 18) + 4.5 + 1 + 1 = 20/sin θ + 43.5 (in units of u, relative to Σ\|B\|\|D\|\|B\|).

For a reference perpendicular to the chord (sin θ = 1: the N series, and R1's members along (3,4,0) and the axes) c_f ≤ 63.5; for R1's (1,2,2) members with y_reference (0,0,1), sin θ = √5/3 and c_f ≤ 70.4. The assembled entry adds 1. The longest rounding chain is 16 roundings for an element entry and 17 for an assembled entry; a spring stiffness enters exactly, and a directional block carries at most 4u. The plan's estimate "c_f ≲ 20" counted chain length, not the error coefficient. The ceiling argument's margin is unaffected: with m ≤ 2^20 the term 64m dominates, and step 7's forward error stays ≲ F·2^-486 for any c_f ≤ 2^20 (sin θ ≥ 2^-15), a 2^50 margin for every F < 2^372. A reference within about 2^-15 rad of the chord would need its own count. (Evidence that the bound is loose: the rigid-mode tests find |K_e·r| within 1.38·u of the same magnitudes at every precision; §11 E.)

## 10. Findings F-2 and F-3 (routed to D1 revision 5a.3)

- **F-2:** one member, a prescribed ux = 1 (EA/L = 512) and a load of 2^-300: u cannot represent 1 + 2^-309 at 128 or 256, so the end actions and reactions are 0 at both and S\*(force) = 0; the case is selected at 128 as exact, the truth is N = 2^-300.
- **F-3:** an unloaded body moved rigidly by prescribed values: K_e's one-ulp rigid-mode leakage (the rounded DB) gives reactions of about 2^-(p−27) at every p; S\*(force) is the leakage; the case ends `Unresolved(Ceiling)`. Safe; nothing wrong is published.
- **Routed** as ROOT ruled: S\* has no resolution floor tied to the elastic-action scale; this refutes §4.1.9's claim. No K4 test pins either behaviour. **[pending D1 5a.3]**: the rule selected, its implementation, and the controls.

## 11. Tests (89 in `K4T/`; all pass; one is the V4-S3 probe, §21)

**B (multi-term sum; 16 in `wide_sum_tests.rs`):** the committed vectors' sha256; 397 targeted sums against the Fraction oracle; RV12's counterexample; zero, negation, reuse; the refused span adds nothing; 7 differential streams of 10^5 sums; the four N5 streams (D).
**C (ledger; 7):** netting with (+, 0) and the −2148 quantum; the projection against Fraction at every K4 precision; p = 53 on normal results and the exact form at 1024; RF-CANCEL and check L exact; the prescribed-coupled tie decided by the ledger's tail (K4-M8: loads 2^-128 and 2^-400); order-independent encoding; the Q10 pins (§13).
**Source (4):** canonical order and bodies; every validation refusal; the exact degenerate-axis decision; order-independent encodings.
**E (assembly; 9):** assembled K equals the Fraction emulation bit for bit (7 models × 8 precisions, 5,280 entries); rigid modes within 64 units of 2^-p against the expansion's own terms (worst 1.38); p = 53 against `local_stiffness` (same zero pattern, 8 ulps); K-D5's re-formation at 128 within 1 ulp of the largest entry (bound 64); list permutations bit-identical; the duplicate operand (a sequential fold loses the weak member at 128 and 256); the prescribed-motion control; `from_positions` equals `from_connectivity`; the rhs's prescribed coupling is exact.
**F (factor; 9 + 1):** RCM on `sparse_direct`'s small graphs; the rcond control (n = 70, every pivot exactly 1, κ ≈ 4^70: 128 escalates, 256 passes); the negative-energy pair and its definite and singular neighbours; PIVOT (fails at 128, accepted at 256 against 512); witnessed mechanisms refused with no attempt and the RX companion solves; the exact span decision; the non-spanning directional ground (O1); TWO-SPAN's p + 64 correction (O5); the ceiling's residual basis; the three-correction cap (N09-B with a perturbed residual basis: 0, 1, 2, 3 corrections, then an escalating residual-gate stop).
**G (schedule and stop rule; in `adaptive_tests.rs`):** N05 and N06 at 128; SKEW-K1E-28 (128 fails on the condition screen, 256 accepted against 512); SKEW6-K1E-12 rejected at 128 with its verification reused; four solves at most; ZERO-TORSION-345; ALL-ZERO-BODY; REACTIONS-ONLY; the ceiling case; the exact predicate at ±1 ulp and K4-M14's |q_p| variant; every published quantity takes part (N06: 8 quantity types, 4 kinds).
**H (recovery; 5):** 43 models against exact references, every published kind compared (u, mag, end, st including N01's t = 0.25, spr, dspr, R, sf, sm, N, T, Mb, Mbs); +0.0; subnormal, underflow and overflow outcomes; N06's torque and spring action; D16's recovery half (a unit control on `recover`: the code-order fold of e_x·u_i loses a term the exact expansion keeps).
**I (combinations; 10):** §8.6 and §8.7.
**J (classification; 4):** 405 row sets bit for bit against the generator's binary64 reimplementation; ±1 ulp around t; S\* < 2^-988; b at S\* < 2^-1011 and at 0; k_i and the stress scales (300 and 1,500 vectors); input-derived rows; S8-W's far-node rows (29 and 39 `absolute_verified`).
**K (references; 6):** §12.
**L (budgets, work, determinism):** a case limit equal to the need selects, one less gives `Budget(Case)`; the invocation limit; failed and verification work charged; `from_integer`'s length charged; golden work; runs and permutations deterministic; factor reuse bit-identical to separate solves; the source scan (26 functions, lexer self-control).
**O8:** five retained-state digests (§11.1).

### 11.1 Vectors and the O8 emulation

`GEN` (standard library only; imports K3's generator and R1's `references.py` read-only; writes no bytecode) produces `sum_targeted`, `sum_differential` (+ sample), `ledger`, `streams` (+ sample), `formation`, `models` (49 models and 4 combinations, incl. K-D5's D5C-1 controls and K2b's spring-carried case), `r1_cases` (R1's 128 K4 cases through the adapter), `classification`, `o8_states` and `encodings` (Q10, §13), and `SHA256SUMS`. `--check` regenerates in memory and compares byte for byte (13 of 13 OK).

**O8 (approved; not disproportionate):** `emulate_state` mirrors, with Fractions rounded to p at every step the Rust code rounds, the formation at p and at p + 64, the exact assembly, the RCM order (a port of `sparse_direct`'s rules), the radix equilibration, the profile LDLᵀ with its exact pivot screen, the reduced rhs, the solves, the p + 64 residual loop and the recovery of Q; `state_sha` encodes `K4RST`. It reproduces the Rust retained states bit for bit:

| Case | p | sha256 (first 12) | corrections |
|---|---|---|---|
| N05 | 128 | `a0351c9e0d4a` | 0 |
| N05 | 256 | `e94741023f0f` | 0 |
| N06 | 128 | `4a8145155a9a` | 0 |
| N06 | 256 | `e17a84a7042a` | 0 |
| SKEW-K1E-4 | 128 | `797c1a3978b9` | 0 |

### 11.2 S\*-dependent assertions **[pending D1 5a.3]**

To be updated mechanically if the addendum changes S\*: SD-G1 (SKEW6 rejected at 128; SKEW-K1E-60's four attempts), SD-G2 (ZERO-TORSION-345 at 128), SD-G3 (ALL-ZERO-BODY exact, bound 0), SD-G4 (REACTIONS-ONLY rejected by a reaction), SD-G5 (the predicate boundary tests), SD-G6 (the every-quantity control), SD-L1 (the golden work: the stop-rule column), SD-I1 (2·SKEW6 − SKEW6 rejected at 128), SD-I2 (SKEW6 − SKEW6 at 128, exact +0.0 with bound 0), SD-J1 (the classification vectors), SD-J2 (S8-W's far-node rows), SD-K1 (the floor check's not-covered set). More generally every asserted selected precision rests on the stop rule; these are the assertions where S\*'s form decides.

## 12. References and routed cases (K)

### 12.1 The adapter (plan §12.1)
Kernel inputs are fl() of the value `references.py` uses on the case's basis: coordinates, E, G, k and loads; A = fl(A_basis), Iy = Iz = fl(I_basis), J = 2·fl(I_basis) with π = PI_Q; directions are R1's integer vectors. At a node where any spring of a kind has a non-axis direction, every spring of that kind there becomes a `DirectionalSpring`. y_reference = (0,0,1) unless the member is parallel to Z, then (1,0,0). Rigid DOFs are constraints at 0; RF-CANCEL's authored contributions are one load each (sources c0, c1, …); every member has a station at t = 0.5. The chain of 1,005 members (RF-MECH-LINE-IN-CHAIN1000) is taken from `references.py`'s own full model. tw = fl(T/k_t), ext = fl(N/k_a) with k_t = fl(fl(G·J)/L), k_a = fl(fl(E·A)/L) and L FK's binary64 length. R1's RF-CANCEL UDL cases are W1b's and excluded.

### 12.2 Results (the unchanged predicate |obs − exp| ≤ 1e-9·max(|exp|, scale), R1's class scales; RF-CANCEL's binding net-governed column)

| Family | Cases | Selected at | Passes | Absolute-range passes | Not covered | Failures |
|---|---|---|---|---|---|---|
| RF-CHAIN | 30 | 128 (30) | 2,700 | 0 | 0 | 0 |
| RF-SKEW | 36 | 128 (36) | 994 | 0 | 2 | 0 |
| RF-WEAK | 9 | 128 (9) | 566 | 0 | 46 | 0 |
| RF-FINITE | 6 | 128 (6) | 748 | 0 | 0 | 0 |
| RF-MECH | 9 | 8 refused; companion 128 | 26 | 0 | 0 | 0 |
| RF-CANCEL | 38 | 128 (38) | 1,441 | 0 | 3 | 0 |

- 150 R1 rows are exact zeros the kernel does not publish (the off-axis components of axis springs); each is asserted zero by `GEN`.
- **The floor check** (S\* from the reference values with K4's §4.1.6.1 functions, one body per case, variant F for twist and extension): the not-covered set equals `floor_kinds.json`'s F lists (F_rec for RF-CANCEL) restricted to K4's cases, row for row: RF-WEAK 46, RF-CANCEL 3, RF-SKEW 2 **[pending D1 5a.3]**.
- **The discrimination check:** every discriminating value control fails the predicate; the outcome control NC-FALSE-MECHANISM is discriminated by the companion being selected; RF-MECH's controls are discriminated by the refusals; 152 controls are non-discriminating by R1's own analysis.
- **The represented basis:** RF-SKEW-A-CANT-AX-122-r1e-12 and RF-FINITE-THIRTIETHS-O1e6 are compared on `expected_represented`.

### 12.3 The routed cases
- **K-D5's D5C-1 controls** PROBE_D, PROBE_C and BENDING_SOFT: within 1e-9 of their exact references, selected at 128. **RF-SKEW-T-CANT-OFF-122-r1e-04** passes.
- **The spring-carried G = 1e-300 case** (K2b's ruling A): selected at 128; θ = 1.0 exactly; the spring action −1.0; the member torque and the root reaction (about 2^-1081) are `Underflow`, classed `Unpublishable` (O9).
- **B1-L:** exact with the ledger, at 128.
- **N05, N06 and N05 with a transverse tip force:** within 1e-9, with the internal torque and the spring action.

### 12.4 The exact-block oracle (plan §12.2)
The represented binary64 system is built from FK's element formation (`FrameElement::global_stiffness`), springs on the diagonal and the loads, with complete stiffness contributions; `exact_boundary::Context::new` accepts all nine in-scope cases (N01, N05, N06, N08 ×4, N09-B, N09-T), and W1's displacements and reactions agree with `project_displacement` and `project_reaction` under the criterion with the body-level coupled scale; the worst is 1.0e-6 of the criterion (N09-B, R.0.5).

### 12.5 NP-A: which basis the comparison uses, and why
- **The comparison is on the intended basis.** N05's reference is the exact rational solution of the intended model: the N-series inputs (E, G, the section values as fl() of R1's exact values, the spring, the load) taken as exact values and solved with exact formation and assembly (`GEN`'s `solve_exact`). That is the system W1a claims to solve: it re-forms K from the primitives at p and never assembles in binary64.
- **NP-A's represented answer** (`fixtures.json`, `NP.A`, `stored_exact_root` and `stored_exact_tip`) is the exact solution of the product's **stored** binary64 system, whose assembled entries were rounded to binary64 (the soft spring's contribution is partly lost in the root diagonal). A result equal to it would mean W1a had solved the binary64-assembled K (the mutant D1).
- **Result:** K4's N05 root rotation is 9.999999999999999e-5 and its tip rotation 1.0000000000462794e-4; NP-A's represented values are 1.0000016987352618e-4 and 1.0000016987815412e-4, a relative 1.7e-7 away, far outside 1e-9. N06's stored status is singular; K4 solves N06 on the intended basis at 128.

## 13. Canonical encodings (Q10)

All little-endian and versioned; lists in canonical order; no hash is computed in FK.
- **`K4SRC\x01`** (`PrimitiveSource::encoding`): node count and coordinates (f64 bits); members by id (id, i, j, the six property bits, y_reference bits); springs by id; directional springs by id; constraints by DOF (DOF, value bits); loads by (DOF, source id with a u32 length prefix, value bits); stations by id; support groups by id.
- **`K4STF\x01`** (`stiffness_encoding`, the factor-reuse identity): the same without loads, constraint values, stations and groups (constraint DOFs kept).
- **`K4LED\x01`** (`RetainedLedger::encoding`): loaded DOFs ascending; each exact net as (sign, exponent of the lowest set bit, odd magnitude as a u32 limb count and limbs); an exactly cancelled DOF is (0, 0, empty).
- **`K4RST\x01`** (`PrecisionState::encoding`): p, L; u for every DOF ascending, then each member's Q[0..6] by id; each value as K3's `parts()` (sign byte, exponent i64, L limbs) with a zero's sign normalized to +.
- **`K4CMB\x01`** (a combination's source identity): the operand count, then per operand the factor's bits and the operand's `K4SRC` with a length prefix.
- **Pins (Q10: "The tests pin the encodings' sha256 through the generator's `hashlib` output").**
  - `GEN`'s `source_bytes` and `ledger_bytes` build `K4SRC`, `K4STF` and `K4LED` from the models' own lists, in their canonical order, independently of `source.rs` and `ledger.rs`; `ledger_terms` forms each exact net with Fractions (a combination's as the exact Σ c·v). `encodings.txt` records each encoding's sha256 (`hashlib`) and its bytes.
  - The set: `K4SRC`, `K4STF` and `K4LED` of N01 (stations), N05 and N06 (springs, a support group), SKEW6-K1E-12 (six members), PRESCRIBED (nonzero constraint values, no loads), DIRECTIONAL-SPAN (directional springs), B1-C-A (a 1e80 load) and B1-L (several loads on one DOF); and the combination ledgers of B1-C, B1-E and CEILING: 27 encodings.
  - `canonical_encodings_equal_the_generators_independent_bytes` (`ledger_tests.rs`) asserts byte equality and the sha256 for all 27.
  - The retained-state encodings are pinned through O8's independent emulation (§11.1).

## 14. Work and storage (deterministic counts; limb-multiply equivalents)

**Units:** K3's `limb_multiply_cost` per `Wide` operation (counted by the contexts, each recorded once per attempt), plus `SumWork` (limbs touched, shifted, netted and rounded, and `from_integer`'s input length). Integer bookkeeping (pattern, RCM, geometry) is reported as storage, not charged.

**Per attempt, by stage** (own stages: the case's; shared: formation, assembly, residual formation, factor and condition, counted in full against the case, once against the invocation). The stop-rule column and SKEW6's attempt list are **[pending D1 5a.3]**; the other columns are final for the attempted precision.

| Case | p | rhs | solve | refinement | recovery | stop rule | formation | assembly | residual formation | factor | condition |
|---|---|---|---|---|---|---|---|---|---|---|---|
| N05 | 128 | 104 | 10,038 | 23,252 | 6,828 | 80,007 | 27,278 | 949 | 28,227 | 28,849 | 90,465 |
| N05 | 256 | 104 | 10,038 | 23,841 | 6,860 | — | 27,278 | 949 | 92,691 | 28,891 | 90,586 |
| N06 | 128 | 104 | 10,038 | 40,772 | 6,817 | 80,009 | 27,278 | 949 | 28,227 | 28,849 | 90,458 |
| N06 | 256 | 104 | 10,038 | 41,365 | 6,865 | — | 27,278 | 949 | 92,691 | 28,891 | 90,576 |
| TWO-SPAN | 128 | 152 | 8,460 | 37,577 | 6,706 | 128,670 | 55,026 | 1,720 | 57,274 | 20,528 | 79,947 |
| TWO-SPAN | 256 | 152 | 8,460 | 38,808 | 7,063 | — | 55,842 | 1,960 | 187,120 | 20,636 | 80,020 |
| SKEW6-K1E-12 | 128 | 352 | 64,278 | 81,078 | 19,584 | 2,712 | 191,594 | 5,427 | 200,498 | 412,168 | 449,248 |
| SKEW6-K1E-12 | 256 | 352 | 64,278 | 100,032 | 21,290 | 293,232 | 196,982 | 5,637 | 619,826 | 412,818 | 449,981 |
| SKEW6-K1E-12 | 512 | 688 | 226,974 | 164,905 | 60,330 | — | 620,816 | 9,933 | 2,164,328 | 1,464,694 | 1,409,710 |

**Storage** (pattern entries, both triangles; profile entries of the RCM-ordered free rows; limbs per entry 4 at p = 128 and 256, 8 at 512, 16 at 1024):

| Case | DOFs | Free | Pattern | Profile |
|---|---|---|---|---|
| N05, N06 | 12 | 7 | 144 | 28 |
| N09-B | 12 | 6 | 144 | 21 |
| SKEW-K1E-28 | 12 | 9 | 144 | 45 |
| DIRECTIONAL-SPAN | 12 | 9 | 144 | 45 |
| TWO-SPAN | 18 | 6 | 252 | 21 |
| PRESCRIBED | 18 | 9 | 252 | 45 |
| SKEW6-K1E-12 | 42 | 39 | 684 | 330 |

## 15. Streams: seeds, counts and digests

Seeds are the 8-byte ASCII tags read big-endian; each stream's digest is the sha256 of its records, with chunk digests in the manifests; the first 1,000 records of each are committed (the sums' term lists as their sha256).

| Stream | Width | p | Seed (tag) | Count | sha256 | Debug wall time |
|---|---|---|---|---|---|---|
| p128 (N5) | 4 | 128 | `4b345f5030313238` (K4_P0128) | 10^6 | `9924f48b…ca455` | 28.5 s |
| p192 (N5) | 4 | 192 | `4b345f5030313932` (K4_P0192) | 10^6 | `eae821b9…547fd` | 28.4 s |
| p320 (N5) | 8 | 320 | `4b345f5030333230` (K4_P0320) | 10^6 | `6175b245…76936` | 72.5 s |
| p576 (N5) | 16 | 576 | `4b345f5030353736` (K4_P0576) | 10^6 | `48ec75a7…1ac9` | 208.1 s |
| sum128 | 4 | 128 | `4b34533030313238` (K4S00128) | 10^5 | `e06891fb…389db` | 4.8 s |
| sum192 | 4 | 192 | `4b34533030313932` | 10^5 | `8c30f67b…c6dd5` | 5.5 s |
| sum256 | 4 | 256 | `4b34533030323536` | 10^5 | `d71c1012…90b30` | 6.2 s |
| sum320 | 8 | 320 | `4b34533030333230` | 10^5 | `777853d6…8e5ce` | 8.7 s |
| sum512 | 8 | 512 | `4b34533030353132` | 10^5 | `6c4acb2b…d4274` | 11.5 s |
| sum576 | 16 | 576 | `4b34533030353736` | 10^5 | `a27c7348…a51cd` | 16.0 s |
| sum1024 | 16 | 1024 | `4b34533031303234` | 10^5 | `b8296c33…0ef68` | 19.0 s |

The N5 streams use K3's operand rules (a test-only port of K3's SplitMix64 and generator functions) and K3's Fraction oracle. The debug wall times are observations (A1). K4's whole suite takes 205.6 s of debug wall time at 4 threads (A2): O11's growth is about 3.4 minutes.

## 16. The F2a and V-K interface (exact signatures at `3ed6c0e26`)

All items are `pub(crate)` inside FK; `FK/structural.rs` declares `mod retained;` privately. Entry points without a non-test caller carry `#[allow(dead_code)] // F2a API` (or the named consumer).

**Entry points** (`retained::adaptive`, `retained::combine`):
```rust
pub(crate) fn solve_cases(sources: &[PrimitiveSource], case_limit: CaseLimit, meter: &mut InvocationMeter) -> Vec<CaseOutcome>;
pub(crate) fn solve_case(source: PrimitiveSource, case_limit: CaseLimit, meter: &mut InvocationMeter) -> CaseOutcome;
impl RetainedCombination {
    pub(crate) fn solve(operands: &[(f64, &RetainedSolve)], case_limit: CaseLimit, meter: &mut InvocationMeter) -> CombinationOutcome;
}
impl CaseLimit { pub(crate) fn new(limb_multiply_equivalents: u64) -> Self; pub(crate) fn get(self) -> u64; }
impl InvocationMeter { pub(crate) fn new(limit: u64) -> Self; pub(crate) fn charged(&self) -> u64; pub(crate) fn limit(&self) -> u64; pub(crate) fn exhausted(&self) -> bool; }
```
**The source** (`retained::source`):
```rust
impl PrimitiveSource {
    pub(crate) fn new(parts: SourceParts) -> Result<Self, SourceError>;
    pub(crate) fn encoding(&self) -> Vec<u8>;            // K4SRC
    pub(crate) fn stiffness_encoding(&self) -> Vec<u8>;  // K4STF
    // read accessors: nodes, members, springs, directional_springs, constraints, loads, stations, supports,
    // node_count, dof_count, constraint(global) -> Option<f64>, free_dofs, body_of_node, body_count, body_nodes, member_index
}
pub(crate) struct SourceParts { nodes: Vec<[f64; 3]>, members: Vec<StraightMember>, springs: Vec<Spring>,
    directional_springs: Vec<DirectionalSpring>, constraints: Vec<Constraint>, loads: Vec<NodalLoad>,
    stations: Vec<Station>, supports: Vec<SupportGroup> }             // all fields pub(crate); Default
pub(crate) struct StraightMember { id: u32, node_i: u32, node_j: u32, elastic_modulus: f64, shear_modulus: f64, area: f64,
    second_moment_y: f64, second_moment_z: f64, torsion_constant: f64, y_reference: [f64; 3] }
pub(crate) struct Spring { id: u32, dof: Dof, stiffness: f64 }
pub(crate) struct DirectionalSpring { id: u32, node: u32, kind: SpringKind, direction: [f64; 3], stiffness: f64 } // kernel-only (Q6): F2a never builds one
pub(crate) enum SpringKind { Translation, Rotation }
pub(crate) struct Constraint { dof: Dof, value: f64 }
pub(crate) struct NodalLoad { dof: Dof, value: f64, source_id: String }
pub(crate) struct Station { id: u32, member: u32, fraction: f64 }
pub(crate) struct SupportGroup { id: u32, node: u32, restrained: [bool; 6], springs: Vec<u32>, directional_springs: Vec<u32> }
pub(crate) struct Dof { node: u32, component: Component }  // global(), from_global()
pub(crate) enum Component { Ux, Uy, Uz, Rx, Ry, Rz }
pub(crate) enum SourceError { /* 22 variants, each naming its item */ }
pub(crate) enum MemberProperty { ElasticModulus, ShearModulus, Area, SecondMomentY, SecondMomentZ, TorsionConstant, YReference }
```
**Outcomes, publication and evidence** (`retained::adaptive`, `retained::recover`, `retained::factor`, `retained::combine`, `retained::ledger`):
```rust
pub(crate) enum CaseOutcome { Selected(Box<RetainedSolve>),
    Refused { refusal: Refusal, geometry: Vec<BodyGeometry> },
    Unresolved { reason: UnresolvedReason, attempts: Vec<AttemptRecord>, geometry: Vec<BodyGeometry> } }
impl RetainedSolve {
    pub(crate) fn publish(&self) -> &Publication;
    pub(crate) fn evidence(&self) -> &RetainedEvidence;
    pub(crate) fn source(&self) -> &PrimitiveSource;   // for a combination: its first operand's stiffness source
    pub(crate) fn selected_precision(&self) -> u32;
    pub(crate) fn state(&self, precision: u32) -> Option<&PrecisionState>;
}
impl PrecisionState { pub(crate) fn precision(&self) -> u32; pub(crate) fn published(&self) -> Vec<Binary64Outcome>; pub(crate) fn encoding(&self) -> Vec<u8>; }
pub(crate) struct Publication { rows: Vec<PublishedRow>, body_scales: Vec<(u32, Kind, u64)> }
pub(crate) struct PublishedRow { id: QuantityId, kind: Kind, body: u32, value: Binary64Outcome, class: RowClass }
pub(crate) enum RowClass { RelativeVerified, AbsoluteVerified { bound_bits: u64 }, InputDerived, Unpublishable }
pub(crate) enum QuantityId { Displacement(Dof), DisplacementMagnitude(u32), EndAction { member: u32, end: End, component: Component },
    StationAction { station: u32, component: Component }, SpringAction { spring: u32, component: Component },
    DirectionalSpringAction { spring: u32, component: Component }, Reaction(Dof), SupportForceMagnitude(u32), SupportMomentMagnitude(u32) }
pub(crate) enum Kind { Translation, Rotation, Force, Moment }
pub(crate) enum End { I, J }
pub(crate) struct RetainedEvidence { method: &'static str, policy: &'static str, attempts: Vec<AttemptRecord>,
    selected_precision: u32, verification_precision: u32, stop_rule: Vec<(u32, Kind, f64)>, floor_ratio_bits: u64,
    body_scales: Vec<(u32, Kind, u64)>, input_derived_dofs: Vec<Dof>, absolute_verified: Vec<(QuantityId, u64)>,
    not_covered: Vec<QuantityId>, unpublishable: Vec<(QuantityId, Binary64Outcome)>, pivot_margin_min: f64, rcond: f64,
    rcond_label: &'static str, residual_worst: f64, corrections: u8, geometry: Vec<BodyGeometry>,
    source_encoding: Vec<u8>, ledger_encoding: Vec<u8>, retained_state_encoding: Vec<u8> }   // [pending D1 5a.3]
pub(crate) struct AttemptRecord { precision: u32, role: AttemptRole, outcome: AttemptOutcome, residual_basis: u32, corrections: u8,
    pivot_margin_min: Option<f64>, rcond: Option<f64>, residual_worst: Option<f64>, work: AttemptWork, k4_work: SumWork,
    stages: StageWork, shared_work: u64, shared_stages: StageWork, shared_built_here: bool, stop_rule_work: u64, storage: StorageCounts }
pub(crate) enum AttemptRole { Candidate, Verification, VerificationThenCandidate }
pub(crate) enum AttemptOutcome { Accepted, Verified, Rejected(AttemptReason), Failed(AttemptReason), Solved }
pub(crate) enum AttemptReason { Stop(AttemptStop), StopRule { quantity: QuantityId, body: u32, kind: Kind }, VerificationFailed }
pub(crate) enum AttemptStop { Budget(BudgetScope), Span, Exponent, Arithmetic(WideError), Structure, Pivot { global_dof: usize },
    ZeroDiagonal { global_dof: usize }, NegativeEnergy { i: usize, j: usize }, Condition, ResidualGate { global_dof: usize } }
pub(crate) enum BudgetScope { Case, Invocation }
pub(crate) enum Refusal { MechanismWitnessed { body: u32, rigid_parameters: [f64; 6] }, GeometryUnavailable { body: u32, error: StructuralError },
    NegativeEnergy { i: usize, j: usize }, LedgerUnavailable(LedgerRefusal), Structure }
pub(crate) enum UnresolvedReason { Ceiling, Budget(BudgetScope), ExactSumSpan, ExponentRange, ZeroDiagonal { global_dof: usize }, Arithmetic(WideError) }
pub(crate) enum BodyGeometry { Restrained, NumericallyUnresolved, NotAssessed(Vec<(u32, SpringKind)>) }
pub(crate) enum LedgerRefusal { Accumulator(SumError) }
pub(crate) enum CombinationOutcome { Selected(Box<RetainedSolve>), Unresolved { reason: CombinationReason, attempts: Vec<AttemptRecord> } }
pub(crate) enum CombinationReason { CombinationUnresolved, NoOperands, NestedCombination, OperandsDiffer,
    LedgerUnavailable(LedgerRefusal), Unresolved(UnresolvedReason), Refused(Refusal) }
pub(crate) struct StageWork { formation, assembly, residual_formation, factor, condition, rhs, solve, refinement, recovery, stop_rule: u64 }
pub(crate) struct StorageCounts { pattern_entries: usize, profile_entries: usize, limbs_per_entry: usize }
```
**Constants and classification helpers** (F2a's receipt and D2's G-checks):
```rust
pub(crate) const METHOD_TOKEN: &str = "contribution_preserving_multiprecision_v1";
pub(crate) const POLICY: &str = "M03-INTEGRITY-MP-v1";
pub(crate) const RCOND_LABEL: &str;   // "sensitivity to matrix-entry perturbation, not to authored parameters"
pub(crate) const PRECISIONS: [u32; 4] = [128, 256, 512, 1024];
pub(crate) const FLOOR_RATIO_BITS: u64 = 0x3DD0_0000_0000_0000;
pub(crate) const K_SQRT2_BITS: u64 = 0x3FF6_A09E_667F_3BCD;
pub(crate) const K_TWO_SQRT2_BITS: u64 = 0x4006_A09E_667F_3BCD;
pub(crate) fn threshold(s_star: f64) -> f64;                 // fl(R·S*)          [pending D1 5a.3]
pub(crate) fn absolute_bound(s_star: f64) -> f64;            // fl↑(2^-64·S*)     [pending D1 5a.3]
pub(crate) fn classify(value: f64, s_star: f64) -> RowClass; //                   [pending D1 5a.3]
pub(crate) fn coupled_scales(s: [f64; 4], extent: f64) -> [f64; 4];
pub(crate) fn body_extent(coordinates: &[[f64; 3]]) -> f64;
pub(crate) fn stress_scale(fo: f64, mo: f64, area: f64, modulus: f64, k: f64) -> f64;
pub(crate) fn intensified_k(i: f64) -> f64;
```
**For V-K** (Q7's cross-crate equality test): `retained::factor::reverse_cuthill_mckee(adjacency: &[Vec<usize>]) -> Vec<usize>`.
**K3's and K4's work types in the evidence:** `wide::multi::{Binary64Outcome, AttemptWork, WidthWork}`, `wide::WideError`, `wide_sum::SumWork`, and `crate::exact_sum::SumError`, `crate::structural::StructuralError` (already public).

**The export list (Q9: the first consumer outside FK adds it).** Raise to `pub` and re-export from `FK/structural.rs` (for example `pub use retained::{…}` beside the private `mod retained;`):
- from `adaptive`: `solve_cases`, `solve_case`, `CaseLimit`, `InvocationMeter`, `CaseOutcome`, `RetainedSolve` (its five accessors), `PrecisionState` (`precision`, `published`, `encoding`), `Publication`, `PublishedRow`, `RowClass`, `RetainedEvidence`, `AttemptRecord`, `AttemptRole`, `AttemptOutcome`, `AttemptReason`, `AttemptStop`, `StageWork`, `StorageCounts`, `BudgetScope`, `Refusal`, `UnresolvedReason`, the seven constants, `threshold`, `absolute_bound`, `classify`, `coupled_scales`, `body_extent`, `stress_scale`, `intensified_k`;
- from `source`: `PrimitiveSource` (with `new`, the encodings and the read accessors), `SourceParts`, `StraightMember`, `Spring`, `DirectionalSpring` (V-K only), `SpringKind`, `Constraint`, `NodalLoad`, `Station`, `SupportGroup`, `Dof`, `Component`, `SourceError`, `MemberProperty`;
- from `recover`: `QuantityId`, `Kind`, `End`; from `factor`: `BodyGeometry`, and `reverse_cuthill_mckee` for V-K; from `ledger`: `LedgerRefusal`; from `combine`: `RetainedCombination`, `CombinationOutcome`, `CombinationReason`;
- K3's `Binary64Outcome`, `AttemptWork`, `WidthWork`, `WideError`, and K4's `SumWork` (as field types of the evidence).
- **Not exported:** `CasePrep`, `GroupPrep`, `GroupCache`, `Shared`, `Solved`, `StageGuard`, `ExactWideSum`, `RetainedLedger`'s internals, the formation and factor functions: "No caller can supply a matrix, factor, closure or label."
- **[pending D1 5a.3]:** the addendum may add evidence fields (for example a published resolution scale for D2's G5a–G5c) and change the classification helpers' definitions.

## 17. Suites, T9 and mutations

- **Suites (39 manifests, `--no-fail-fast`) against the Mac baseline of main:** **[checkpoint B]**.
- **T9 (Mac-only):** **[checkpoint B]**.
- **Mutations** (clean copy and target per mutant, NONE first, at most three at once): **[checkpoint C]**. K4-M24's kill waits on the addendum (§8.4). Added at ROOT's request: **K4-M33**, a prescribed row published from its p-rounded state (double rounding), killed by `a_combined_prescribed_value_is_published_from_its_exact_sum_rounded_once` (§8.7).

## 18. Toolchain and host

- `aarch64-apple-darwin`, rustc 1.97.1 (`RUSTUP_TOOLCHAIN=1.97.1`), `CARGO_INCREMENTAL=0`, `--offline --locked`, `-j 8`, `RUST_TEST_THREADS=4`, at most two cargo jobs; target `<wt>/k4-target`; Python 3 standard library (`<VENV>` not needed by `GEN`). T9 is a Mac-only comparison.
- The Mac host rules applied throughout: no dense matrix above 1,000 members (the 1,005-member chain is refused by geometry before any factor; the exact-block oracle builds dense 12×12 systems only); the memory guard ran and logged no event.

## 19. What was not done, and open items

- **[pending D1 5a.3]:** the S\* resolution floor, its implementation and controls; the S\*-dependent assertions (§11.2); the ceiling argument's step 10 (§9); O1's step 4 if S\* changes for its kinds (§6); the constructed-ceiling combination and K4-M24 (§8.4); the prescribed-tail case (§8.5); the evidence's `not_covered` field.
- **Checkpoints B, C and D.**
- **Deferred by ruling:** F2a's limits (Q5 amended); the public export (Q9); unifying `Binary64Outcome` with K2b's `Representability` (Q11); a full geometric treatment of partial directional grounds (O1, W4/K5).
- **Incidents:** at A1 the generator's imports left git-ignored `__pycache__` directories in K3's test directory and R1's folder; they were removed before any commit and `GEN` now sets `sys.dont_write_bytecode`.

## 20. Records (placeholder)

`_run_records/` **[checkpoint D]**: the checkpoint-0 plan (`e1253faa…`) and its scratch probes; the A1 and A2 test logs; the generator `--check` logs; the storage-count script; B's suites and T9; C's mutation table; `source_sha256.txt`; `toolchain.txt`; `SHA256SUMS`.

## 21. V4-S3: the coalesced residual gate refuses a chord-component y_reference (a probe) **[pending D1 5a.3]**

ROOT reversed its O12 ruling (`085638e58`): the gate's denominator becomes the bounded operator at contribution level, defined by the revised addendum. **The gate is unchanged here.** `probe_v4_s3_y_reference_with_a_chord_component` (`adaptive_tests.rs`, no assertion yet) solves a cantilever run along (3,4,0) (1 or 3 members of the N-series section, the root fixed, a tip load, no prescribed motion) and reports, per precision, the worst gate ratio |r_i|(2^p − m_i)/(64·m_i·d_i) at each evaluation of the refinement loop (the first solve and up to three corrections), with M03's coalesced d_i.

- **V4's finding is confirmed in Rust.** With y_reference (3,4,5), a load exciting one mode only (an in-plane transverse force, an out-of-plane force, an axial force, or a torque) is refused by the gate at 128, 256 and 512, and the case ends `Unresolved(Ceiling)`:

| y_reference | Members | Tip load | Outcome | Gate ratios at 128, 256, 512 (first evaluation → after three corrections) | At 1024 |
|---|---|---|---|---|---|
| (3,4,5) | 1 | in-plane transverse F | Unresolved(Ceiling) | 7.7e15 → 5.1e15; 1.7e16 → 4.3e15; 2.4e16 → 4.3e15 | 5.2e-4 |
| (3,4,5) | 1 | out-of-plane F | Unresolved(Ceiling) | 4.4e35 → 7.2e15; 1.5e74 → 7.2e15; 1.7e151 → 7.2e15 | 7.3e-4 |
| (3,4,5) | 1 | axial F | Unresolved(Ceiling) | 4.4e35 → 6.2e15; 1.5e74 → 3.9e15; 1.7e151 → 6.4e15 | 2.6e-4 |
| (3,4,5) | 1 | torque | Unresolved(Ceiling) | 4.4e35 → 7.2e15; 1.5e74 → 5.1e15; 1.7e151 → 5.7e15 | 6.0e-4 |
| (3,4,5) | 3 | the same four loads | Unresolved(Ceiling) each | first 4.2e16–3.0e35, 1.7e16–1.0e74, 4.7e16–1.2e151 → 2.6e15–7.7e15 | 2.9e-4–5.3e-4 |
| (3,4,5) | 1 and 3 | general (all six components) | selected at 128 | 9.6e-4, 6.4e-4 (no correction) | — |
| (0,0,1) | 1 and 3 | each of the five loads | selected at 128 | 3.2e-4 to 3.0e-3 at every precision (no correction) | — |

- **Reading.** With a chord component in y_reference, e_y's in-chord components are exact zeros only in exact arithmetic; at p they are O(2^-p) residues, so K_p couples modes that K_q (at p + 64) couples 2^64 times less. For a load exciting one mode, the other modes' rows have a coalesced d_i of the order of K_q's coupling times the solution, while the residual carries K_p's coupling: the ratio starts near 2^p/(64m) and, after corrections, settles near 2^52–2^53 (about 6e15, V4's figure), independent of p. The ceiling passes because its residual basis is K itself (Q4). A general load gives every row a large d_i, and y_reference (0,0,1) makes the frame exact.
- **To do under the revised addendum:** change the denominator as specified, turn this probe into a control with assertions, and add its mutant.
