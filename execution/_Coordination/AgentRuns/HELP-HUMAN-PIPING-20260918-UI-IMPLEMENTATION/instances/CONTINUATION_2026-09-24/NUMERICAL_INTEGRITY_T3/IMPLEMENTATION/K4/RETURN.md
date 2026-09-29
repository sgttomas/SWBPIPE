# I12 return: slice K4 (the W1a kernel method)

**Status (checkpoint D, with addenda 1 and 2 after RV19's review and delta check): complete; no stop.** Addendum 1 (at the end) records the fixes of RV19's findings, ROOT's amendment A1 and what they change; addendum 2 closes the delta check's RV19-D4 and DN2. The statements below are corrected to match.
- W1a's kernel method is implemented under `FK/src/structural/retained/`, with no product caller (Q1). A combination is its own solve (ROOT's F-1 ruling).
- **D1 revision 5a.3 (R7) is implemented:** the stop rule's resolution term V, the verification estimate, the formation charge with its certified per-block bound B_c = min(Uc_c, S_c), θ and the g check, the ceiling floor Φ (item 6a), the hybrid residual gate and the evidence fields (§22).
- **Tests:** 127 K4 tests pass (`K4T/`; 122 at D, three added at addendum 1, two at addendum 2), with the S11 site table's 3; FK's full suite, the SD and NI suites and CI's 39-manifest profile match the Mac baseline of main (§17). K3a's, K3's, K-D5's and `exact_sum`'s tests are unchanged and pass.
- **Controls:** every control and combination equals the generator's schedule (GEN, the bit oracle) and R7's expectations. Every selected control and combination of the controls test (117; RF-LARGE at 100 members runs in the references lane) and RF-LARGE's six 100-member frames satisfy the claim each row publishes, checked against GEN's 128-bit exact expectations with no row skipped (addendum 1; at D, 13 selected controls without expectations and the unpublishable rows were skipped, RV19-2), and pass a test-only G5a checker.
- **References:** R1's 128 K4 cases pass R1's predicate (6,475 passes, 0 failures), and the not-covered set equals §4.10's list; the RF-LARGE frames at 10 and 100 members are selected at 128 and honest.
- **Mutations:** every killable mutant is killed; the four derivation guards move no control (§17). At addendum 1, the reverted RV19-1 fix, the reverted amendment A1, RV19-M2 and RV19-M6 are killed too.
- **Moved outcomes** against revision 5a.2, all accepted by ROOT (§22.5): DIRECTIONAL-SPAN 256 → Unresolved(Ceiling) (5a.2's publication was within its claim); CEIL-A and CEIL-B → Unresolved(ResolutionScaleUnencodable); RIGID-UNLOADED Unresolved → 512.
- **The tightened predicate (§22.9):** no selected unmutated control fails it. Under it, the false claims of R7-M1, K4-M24 and K4-M37 are caught as dishonest (F-2-SPOS and CEIL5A3 newly). The exceptions are PRESCRIBED-TAIL and PRESCRIBED-TAIL-FREE, whose true rows lie below binary64's range.

**Where:**
- Branch `codex/piping-k4-20260928` in `<wt>/k4`. Base: main `e7d930d49` (K2b merged); main `7ac7b1c37` (K6) merged in at `8f8023a20` with no overlap in FK.
- Checkpoints committed by ROOT: A1 `cef218a10`; A2 `3ed6c0e26`; the Q10 pins and record drafts `3668ee8a4`; the prescribed-row fix and the V4-S3 probe `5ad1b6174`; A3-0 (the 5a.3 plan) `03e7250b2`; A3a `8dfade92e`; A3b `bb7757ac5`; B `8a59458a1`; the main merge `8f8023a20`; C (no code change). This D revision is recorded by ROOT.

**Abbreviations:** `P/` = `projects/chirality-piping/`; `FK/` = `P/core/solver/frame_kernel/`; `T3/` = the NUMERICAL_INTEGRITY_T3 folder; `K4T/` = `FK/tests/retained_k4/`; `GEN` = `K4T/gen_k4_vectors.py`; R7 = `T3/DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md` (`5502aef9…`).

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
  - "D1 revision 5a.3 SELECTED": R7 §5 (§5.1–§5.9) is the governing text; `DESIGN.md` stays pinned at 5a.2; R7's derivation sections are its warrant;
  - "K4: rulings on I12's A3-0 plan for revision 5a.3": the plan (`IMPLEMENTATION/K4/PLAN_A3_5A3.md`, `07186550…`, 784 lines) approved, with its Q1–Q18; `factor.rs`'s loop stays byte-identical;
  - "K4: rulings at A3b": the moved outcomes accepted (§22.5); DIRECTIONAL-SPAN's 5a.2 publication to be checked against GEN's exact solution (§22.3); ê overflow mapped to `ResolutionScaleUnencodable` (§22.4);
  - "K4: B accepted", with ROOT's bracketed correction of the A3b ruling (DIRECTIONAL-SPAN was selected at 256 by 5a.2);
  - "K4: C accepted; the honesty predicate tightened at D" (§22.9).

## 2. Files and line counts (against main `7ac7b1c37`, at `8f8023a20` plus D's test edits)

| File | Change | Lines |
|---|---|---|
| `FK/src/structural/retained/wide_sum.rs` | new: the correctly rounded exact multi-term sum (Q2(a)), with the exact product comparison (A3-0 Q6) | 529 |
| `FK/src/structural/retained/source.rs` | new: `PrimitiveSource`, validation, bodies, encodings | 743 |
| `FK/src/structural/retained/ledger.rs` | new: the exact ledger and its projection | 224 |
| `FK/src/structural/retained/assemble.rs` | new: formation, assembly, reduction at p; the bounded and wide formations of the hybrid gate | 795 |
| `FK/src/structural/retained/factor.rs` | new: geometry first, RCM, the p-factor and its screens; the read-only est_c observer and the L and D accessors (A3-0 Q4) | 813 |
| `FK/src/structural/retained/recover.rs` | new: layout, recovery, publication, the state encoding | 504 |
| `FK/src/structural/retained/directed.rs` | new (5a.3): directed wide rounding, up and down (R7 items 6, 7; A3-0 Q2) | 201 |
| `FK/src/structural/retained/bound.rs` | new (5a.3): blocks, Uc_c (Lemma D), est_c, the shifted loop and S_c (Lemma E), B_c (R7 item 7) | 851 |
| `FK/src/structural/retained/verify.rs` | new (5a.3): E and ê, Φ, the verification pass (Ŵ, δ̂, the norms, θ, g, C_q, W⁺) | 1,194 |
| `FK/src/structural/retained/adaptive.rs` | new: schedule, stop rule with V, S\*, the hybrid gate, classification with the floor, budgets, reuse, evidence, the prescribed rows' publication | 3,118 |
| `FK/src/structural/retained/combine.rs` | new: combinations as their own solve | 133 |
| `FK/src/structural/retained/mod.rs` | +20: the `pub(crate) mod` lines and documentation | 44 |
| `FK/src/exact_sum.rs` | +19: the `net_parts` accessor (Q3) | — |
| `FK/tests/s11_site_table.rs` | +87: K4's eleven files in SOURCES and their rows (Q8) | — |
| `K4T/{wide_sum,ledger,source,assemble,factor,directed,bound,verify,adaptive,scale,method,recover,combine,references,classification}_tests.rs` | new: 16, 7, 4, 9, 9, 2, 4, 3, 19, 6, 10, 5, 10, 13, 5 tests (122) | 538, 375, 243, 724, 436, 109, 270, 73, 1,112, 586, 1,825, 262, 439, 710, 288 |
| `K4T/support.rs`, `K4T/models.rs` | new: test-only SHA-256, SplitMix64, token helpers; model parsing and the comparisons, the tightened honesty check (§22.9) | 578, 471 |
| `K4T/gen_k4_vectors.py` | new: the standard-library generator, with `--check` | 4,427 |
| `K4T/*.txt`, `K4T/SHA256SUMS` | new: generated vectors (§11.1) | 80,739 |
| `T3/IMPLEMENTATION/K4/` | `PLAN_A3_5A3.md` (784, A3-0), this RETURN, `CHANGE_RECORD.md`, `_run_records/` (§20) | — |

- **Size:** the vector data is about 9.4 MB (the two samples 1.3 MB each, `bounds.txt` 1.3 MB, R1's cases 1.0 MB, `charge.txt` 0.8 MB, `r1_large.txt` 0.8 MB).
- **Unchanged:** every manifest and lockfile; K3a's and K3's code, tests, generators and vectors; `wide.rs`, `multi.rs`, `structural.rs`, `lib.rs`, `sparse.rs`, `load_ledger.rs`, `rigid_body.rs`, `exact_boundary.rs`, `formation_check.rs`; SA, PP, NI, `sparse_direct`; the fixtures and R1's references. The main merge at `8f8023a20` touched no FK file.
- **`factor.rs`'s loop** (`factor()`, `pivot_passes`, `negative_pair`, `solve_scaled`, `solve`) is byte-identical since A3a (A3-0 ruling), so Lemmas D and E stay tied to the loop as read.

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
6. **Solve and refinement.** "Evaluate r = f − K u against the intended system re-formed at p + 64"; "Each r_i is one exact expansion of the p-bit products and the ledger terms", rounded once to p for the correction. **The hybrid gate** (R7 §5.6, revision 5a.3): refinement is driven by the coalesced denominator d_i^c = |f_i| + Σ_j |K^q_ij·u_j| (M03's), with |r_i|(2^p − m_i) ≤ 64·m_i·d_i^c decided exactly; "at most three corrections", stopping early when the worst coalesced ratio does not decrease. When refinement ends without the coalesced test passing, the gate takes the **best state evaluated**: the smallest worst bounded ratio with d_i^b = |f_i| + Σ_j Ā^q_ij·|u_j|, compared exactly with `ExactWideSum::add_product_of` (A3-0 Q6), the earliest on a tie; it passes if every row passes with d_i^b, or the attempt fails (`ResidualGate`). d^b and Ā^q are formed lazily, only when the gate falls back (A3-0 Q17). The ceiling's solve as ruled under Q4 (§9); the same gate applies at q = 1024.
7. **`recover.rs`: recovery before rounding.** d_local = T u, e = B_local d_local, Q = D e, end actions B_localᵀ Q; stations (m(t) = t·(M_j + M_i) − M_i, exact); spring actions −k u; directional spring actions −K_s u; reactions K_c u − f_c; node displacement magnitudes and support-group magnitudes. "Each component … is one exact expansion of its (at most five) product terms, rounded once"; each reaction "one exact expansion of K_cj·u_j products and the ledger terms". Each published quantity is rounded to binary64 once with K3's `Binary64Outcome`; an exact zero is +0.0 (Q11). A prescribed row (an input-derived displacement) is published from its exact sum of terms c·v, rounded once to binary64 by the exact accumulator (§8.7), never from its p-rounded value. There is never a silent value: underflow and overflow rows are `Unpublishable` (O9).
8. **`combine.rs`: combinations** (§4.1.1, refined by ROOT's F-1 ruling; §8). A combination is its own solve: the combined exact ledger Σcᵢfᵢ and the combined prescribed values, on its operands' shared stiffness source, reusing their cached factor, with its own schedule and stop rule. "Its published outputs go through the stop rule … with S\* taken from the combination's own body scale"; it "escalates independently of its operand cases"; at the ceiling it is withheld (`CombinationUnresolved`) and "its operand cases keep their standing". The Σcᵢuᵢ formation is withdrawn.
9. **`adaptive.rs`.**
   - **The schedule.** "Candidates at p = 128, 256 and 512, each verified at 2p. The ceiling is 1024 bits." "The verification solve at 2p repeats formation from the binary64 operands." "A rejected 128 candidate's 256 verification becomes the next candidate … at most four solves."
   - **The stop rule** (R7 §5.1). Accept p when, decided exactly: (a) "|q_p − q_2p| + V_q ≤ 2^-64 · max(|q_2p|, S\*)" on every published quantity, with V_q = 2^(8−2p)·ê(body, kind) for force and moment rows and V_q = W⁺_q for translation and rotation rows; (b) the verification estimate W_q ≤ 2^(6−2p)·ê for every force and moment row; (c) at the verification, for every block that carries data, a certified B_c exists, θ_c ≤ 1/2, and g ≤ 2^(P−16) for every member with a nonzero prescribed DOF or a free DOF in such a block; (d) the charge, C_q ≤ 60·2^-2p·ê at 128 and 256 and C_q ≤ 2^-22·2^-64·M_q at 512. The rejection order is emu7's, (a), (b), `uc`, θ, g, (d) (A3-0 Q7), each with its own reason. S\* per body and kind at 2p with §4.1.6.1's coupling rounded once at 2p. **The ceiling floor** (item 6a): at p = 512, S\*(kind) := max(S\*(kind), Φ), Φ = fl↑(2^-438·ê), in the stop rule and in the classification, the same bits. "A body with all scales zero … must agree exactly."
   - **The classification** on the published value: `absolute_verified` iff |q| < fl(R·S\*), R = 2^-34 (`0x3DD0000000000000`); S\* < 2^-988 makes every row of the body and kind `absolute_verified`; b = fl↑(2^-64·S\*); restrained and prescribed DOFs are `input_derived`; the per-member stress scales with the pinned k constants and k_i = fl↑(k√2·i) (`stress_scale`, `intensified_k`). At a selected 512 the classification uses the floored scales (`classify_rows_floored`), so b = fl↑(2^-64·Φ) for a floored kind.
   - **Failure.** "At the ceiling, or when the budget runs out, the case is unresolved … with the attempted precisions and the reason … Nothing is relabelled as solved."
   - **The evidence** (§5 item 1) as kernel types: the attempts (p, role, outcome, reason, residual basis, corrections, pivot margin, rcond, residual summary, work by stage, shared work, storage counts); the selected and verification p; the stop-rule summary rounded upward; R's bits and S\* per body and kind as bits; `input_derived_dofs`; the `absolute_verified` ids with b; `not_covered` (empty in the kernel, since F2a owns the not-covered classes; A3-0 Q16); `unpublishable`; the pivot margin minimum (rounded downward), rcond, the residual summary (rounded upward); the geometry; the source, ledger and retained-state encodings. **Revision 5a.3's fields** (R7 §5.8): per body, `resolution_scale` (E_fo and E_mo bits, uncoupled, rounded upward); `verification_estimate` (the worst W_q/V_q per body and kind, rounded upward); `verification_charge` (the worst C_q over its allowance per body and kind, the largest θ_c and B_b per body, rounded upward); the stop-rule summary becomes the worst (|q_p − q_2p| + V_q)/M_q. Each attempt records its gate (`GateTest`: coalesced or bounded, the state chosen) and its verification (`VerificationSummary`).
10. **Budgets and work** (§4.1.7; Q5 as amended). "Counted in limb-multiply equivalents per attempt. Successful, failed and verification work is all charged." `CaseLimit` and `InvocationMeter` are required parameters with no `Default` and no numbers. Factor reuse: "Linear cases that share a modulus basis and state share one p-factor per precision": each case's limit counts the full shared work; the invocation meter counts it once (§14).
11. **Determinism** (§4.1.8). "The same `PrimitiveSource` therefore gives a bit-identical retained state on every platform": integer-only arithmetic, canonical order, no allocation-dependent order; the retained-state encoding `K4RST` is "over the canonical limbs of u_p and every member's Q". Tested run to run, under list permutations and under factor reuse; five digests are pinned by an independent emulation (O8).
12. **The opaque result.** `RetainedSolve` is bound to its source and precision; `publish()` returns the once-rounded rows. "No caller can supply a matrix, factor, closure or label": the entry points take only `PrimitiveSource`s, factors of combinations, limits and a meter.
13. **Revision 5a.3's verification** (R7 §4.1.6.2 and §4.1.6.3; `directed.rs`, `bound.rs`, `verify.rs`).
   - **E and ê** (item 6a, §4.1.6.2): the bounded operator B̄, Ā, g_m; every force and moment row's recovery expansion with absolute operands, each stage one exact sum rounded once to nearest at P; a support group's E one exact sum over all contributors, rounded once (Q12, derived in §22.1); E(body, kind) rounded upward once to binary64; ê coupled in binary64. An E that overflows, or an ê that overflows while E is finite, ends `Unresolved(ResolutionScaleUnencodable)` (A3-0 Q8; §22.4). Φ = fl↑(2^-438·ê) (`phi_512`).
   - **The verification pass** (`verify_state`, run only on states used as verifications, A3-0 Q18): the estimate's residual over the contributions at q_W = min(3p + 64, 1024), δ̂ with the verification's factor, Ŵ; the correction's residual; the norms ‖ā_q S‖₁ with every stage rounded upward (A3-0 Q2); θ_c, the g check, the charge C_q and W⁺, each with the directions R7 prescribes.
   - **The certified bounds** (item 7): blocks of the free–free pattern with data (7a); Uc_c from the verification's own L and D, rounded upward, the denominator 1 − t_c downward (Lemma D, 7b); est_c from the condition screen's own solves (7c; K4's rounding, A3-0 Q3); the shifted loop at σ_c and S_c (Lemma E, 7c), an operation-for-operation copy of the factor loop (A3-0 Q5); B_c = min(Uc_c, S_c) (7d).

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
- **Revision 5a.3's positions** (the A3-0 plan, approved with ROOT's Q1–Q18; "where a choice bears on honesty, R7's text governs; where it affects availability only, K4's practical form is accepted"):
  - GEN is the bit oracle and emu7 the selection-level cross-check; every difference is explained (§22.7) (Q1).
  - ‖ā_q S‖₁'s stages are rounded upward (Q2); est_c is K4's block sums rounded once, then one division, availability only (Q3).
  - The est_c observer in `condition` and the L and D read accessors are outside the pivot and rounding logic; a test pins `factor()`'s L and D bits (Q4). The shifted loop is a separate operation-for-operation copy, bound by the loop-parity test and K4-M35 (Q5).
  - The best-state comparison is exact (`add_product_of`); a span overflow stops with `Span`, an availability outcome (Q6). The rejection order is emu7's (Q7).
  - E rounding to +∞ is a terminal `Unresolved(ResolutionScaleUnencodable)` (Q8), and so is an ê overflow with E finite (the A3b ruling, §22.4). A body with no data block has no B_b entry and θ = 0 (Q9, routed to F2a and D2).
  - E-ESTIMATE's reference is GEN's exact rational solution (Q10); E-HEADROOM uses the P state's own ê (Q11).
  - A support group's E is one exact sum over all contributors, rounded once (Q12; derivation in §22.1).
  - RF-LARGE-100's debug runtime was measured (Q13; §12); CEIL5A3 was pinned only after GEN confirmed it (Q14; §8.4); the low-precision stress kills M27 (Q15); `not_covered` stays empty (Q16); d^b is formed lazily and work is charged where incurred (Q17); the verification pass runs only on verification states (Q18).

## 5. The correctly rounded exact multi-term sum (`wide_sum.rs`): the correctness argument

**Structure.** `ExactWideSum` holds two unsigned magnitudes (positive and negative terms), each `[u64; 128]` on the stack (8,192 bits), an anchor (the exponent of bit 0, `i128`), the used-limb count and the highest set bit. There is no heap allocation, per term or per sum; `clear()` zeroes only the used prefix.

**Terms.** `add_wide` (a `Wide<M>` value through `parts()`), `add_binary64` (an exact lift; NaN and ∞ refused), `add_product` (the exact product of two values of at most the context's precision, through K3's `two_product`: s + e = a·b exactly, both added), `add_integer` (±magnitude·2^e: the ledger's net), `add_wide_scaled` and `add_scaled` (a value or another sum's exact net times a small integer and a power of two, for the exact screens and gates), and `add_product_of` (±a·b of two sums' exact values, for revision 5a.3's exact ratio comparisons, A3-0 Q6; formed exactly or refused).

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
4. **The residual risk** is that the p and 2p solutions agree along z to within 2^-64·S\* by coincidence, for every candidate. The two solves have independent roundings of order 2^-p and 2^-2p relative to K, so such agreement needs the noise components to match to 64 bits; no mechanism makes that systematic. This is an argument, not a proof, and it relies on the same stop-rule premise as `NumericallyUnresolved` bodies.
   - **Restated under revision 5a.3.** The addendum closes this risk for every block that carries data: at a selected p, θ_c ≤ 1/2 with the certified B_c, and Lemma C's proof then shows by a Neumann series that S·K\*_c·S is invertible. A singular K\*_c in a data-carrying block therefore fails (c), with `theta` or `uc`, at every p, and the case ends `Unresolved(Ceiling)` (my reading of Lemma C, derived from its proof; not a sentence R7 states). A block with no data has u = 0 exactly at every precision and publishes +0 rows; that it is a mechanism is not detected there. That residue stays under R7's standing premise, "K\* nonsingular per body", and is routed with item 6.
5. **Evidence:** a cantilever whose root is grounded in-plane by one directional spring (with the rigid UZ) is `NotAssessed` and ends `Unresolved(Ceiling)` after escalating stops at 128, 256 and 512; a second, spanning direction makes it `Restrained` and selected at 128. RF-SKEW-T-PIN-AX's six cases solve, so RF-SKEW is 36 compared.
6. **Routed:** a full geometric treatment of partial directional grounds belongs to W4/K5 (§4.9).

## 7. The p + 64 residual (O5): the published-value equivalence of K4-M11, and its kill

**K4-M11** forms the residual against K_p (the factor's own K) instead of K_q re-formed at q = p + 64.

1. **What refinement converges to.** With the residual against K_r, iterative refinement with the p-factor converges (when κ·2^-p ≪ 1) to u_r\* = K_r^-1 f, the solution of the system the residual uses. The correct variant converges to u_q\* with |u_q\* − u\*| ≈ |K^-1 ΔK_q u| where ΔK_q = K_q − K\* is formation error at 2^-q; the mutant converges to u_p\* with the formation error at 2^-p.
2. **When they differ in the attempt.** The gate at p compares |r|(2^p − m) with 64·m·d. The mutant's residual never sees K_p's formation error (u already solves K_p to the gate's accuracy), so it makes no correction. The correct residual sees (K_q − K_p)u; that exceeds the gate only on rows whose assembled entry cancels its contributions while the row's other terms are small. TWO-SPAN (two collinear spans of lengths 1 and 1 + 2^-40, a moment at the shared node) is such a row: the correct variant corrects once at 128 and once at 256; the mutant corrects zero times.
3. **When they differ in the published values.** Both variants publish only an accepted candidate: |q_p − q_2p| ≤ 2^-64·max(|q_2p|, S\*). The verification value q_2p is within about κ·2^-2p of the truth in both variants (its own formation error; both variants treat 2p alike). So each accepted q_p lies within 2^-64·max(|q_2p|, S\*) + κ·2^-2p of the truth. Rounding to binary64 (a relative 2^-53 step) maps two such values to the same binary64 number except when a rounding boundary lies between them: they then differ by one ulp. Under revision 5a.3 the verification's estimate and charge form their own residuals at q_W over the contributions (R7 item 1), so the mutant does not reach them. **So the variants' published values agree to within one binary64 ulp and within 2^-64·S\* + 2^-53|q| of each other, far below the 1e-9 criterion; no published-value test at the criterion distinguishes them.**
4. **Availability.** The mutant's q_p carries the p-formation error, so its acceptance needs κ·2^-p ≲ 2^-64: it may escalate where the correct variant accepts. It never publishes a value outside the acceptance bound. So the mutant is equivalent at the criterion and differs in evidence and availability only.
5. **The kill (ROOT's O5).** On returned attempt evidence, which F2a publishes: TWO-SPAN's attempts are (128, residual basis 192, 1 correction) and (256, 320, 1) (`the_p_plus_64_residual_takes_one_correction_on_two_span_and_records_its_basis`); the golden work pins its refinement and residual-formation work (128: 37,577 and 57,414; 256: 38,808 and 187,516 limb-multiply equivalents re-pinned at A3a, where every formation also forms g exactly (R7 §4.1.6.2 item 2); refinement work is unchanged; `golden_work_counts`). The mutant forms no K_q (residual formation drops) and corrects zero times.

## 8. Combinations (ROOT's F-1 ruling)

1. **The finding.** Σcᵢuᵢ formed from the operands' retained states loses operand loads that cancel below both p's and 2p's resolution, and the stop rule cannot see a loss common to both precisions. CEIL-A − CEIL-B (loads differing by a relative 2^-1060) was selected at 128 with every row an exact 0, classed `AbsoluteVerified` with bound 0; the truth is nonzero.
2. **Why (a) is the same combination.** With one linear K and linear recovery, the exact state is linear in the loads and prescribed values: u(Σcᵢfᵢ, Σcᵢvᵢ) = Σcᵢ·u(fᵢ, vᵢ), and every recovered quantity (reactions include −f) is linear in (u, f). So solving the net case equals combining the operands' exact results: the two formulations agree in exact arithmetic (§4.1.1, §4.1.2 "formed exactly and rounded once", within their intent).
3. **Why (a) avoids the loss.** The combined ledger holds the exact products cᵢ·v of every operand's load terms (`RetainedLedger::combined`); each prescribed value is the exact sum of cᵢ·vᵢ rounded once at p (`CasePrep::combination`, `prescribed_at`). So the combination's right-hand side is "one exact expansion of the combined ledger Σcᵢfᵢ and the combined prescribed coupling, rounded once to p", and its errors are those of its own solve, relative to its own values. It reuses the operands' cached factor (merged caches, including cached factor stops) and runs the case schedule from 128 with its own stop rule. The operands are borrowed and never change.
4. **The constructed ceiling (K4-M24; deferred at A2, built under 5a.3 as CEIL5A3).** Under (a), a combination's outcome is the outcome of its net case. A conditioning-limited body gives a relative error of about κ·2^-p in every kind that carries values, for any nonzero load on it (δu ≈ K_p^-1 ΔK u: its component along the softest direction is about 2^-p·‖K‖·|u|/σ_min). So operands accepted at p ≤ 512 imply κ·2^-p ≲ 2^-64 for their K, and the net case, on the same K, is accepted by 512 too. A combination at the ceiling with every operand selected therefore needs a kind whose S\* is set by values far below its elastic-action scale: **the F-2/F-3 class**. The `Ceiling → CombinationUnresolved` mapping is one match arm; withholding with attempts and unchanged operands is tested through a budget. **Under revision 5a.3 (A3b; ROOT's Q14):** CEIL5A3 is the constructed ceiling. GEN confirmed that both operands are selected and that the combination ends `CombinationUnresolved` at the ceiling, and it is pinned in `outcomes.txt`. K4-M24 (a combination's acceptance tied to its operands: accepted at their precision) is killed by it: CEIL5A3 is selected at 512 under the mutant, and its published N.3 is outside its claim by 670 times the allowance under the tightened predicate (§17, §22.9). CEIL-A − CEIL-B no longer serves: under 5a.3 both operands end `Unresolved(ResolutionScaleUnencodable)` (§22.5).
5. **The prescribed-tail instance (an F-2-class case, routed by ROOT to the design task).** A combined prescribed value whose exact sum needs more than 2p bits (for example 1 + 2^-1100 from operands prescribing 1 and 2^-1100) rounds identically at p and 2p, so a loss below both resolutions is again invisible to the stop rule. **Under revision 5a.3** V makes the stop rule see it: PRESCRIBED-TAIL and PRESCRIBED-TAIL-FREE (the combination form, R7 §7) are rejected at 128 and 256 by rule (a) and selected at 512, where the floor Φ covers what 1024 cannot resolve, and are honest (R7 §6.1; `outcomes.txt`). Without V (R7-M1) they are selected at 128 with a false claim that binary64 cannot show (§22.9).
6. **Evidence (tests I, continued in item 7):** CEILING-S (CEIL-S-A − CEIL-S-B, loads 2^900 + 2^-160 and 2^900, the same ratio 2^-1060; F-1's control since A3b) equals CEIL-S-NET bit for bit (spr.3.3 = −2^-160; the withdrawn Σcᵢuᵢ gives zeros), and CEIL-A and CEIL-B end `Unresolved(ResolutionScaleUnencodable)`; B1-C and B1-E equal their net cases bit for bit and B1-E's truth is reproduced where Σcᵢuᵢ at 128 is off by more than 1e-9; 2·SKEW6 − SKEW6 is rejected at 128 by its own stop rule; SKEW6 − SKEW6 is selected at 128 below its operands' 256; prescribed values combine exactly; cached factor stops and builds are reused and the invocation is charged only the combination's own work; invalid combinations return their reasons.

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
6. **The uncertified step.** The 512 candidate passed rcond_512 > 2^-511, so est(κ(K̃)) < 2^511. Hager–Higham is a lower-bound estimator of ‖K^-1‖ and is not certified; the argument assumes the true κ exceeds the estimate by at most a factor F. (Under 5a.3 this step carries no honesty; see step 11.)
7. **Result:** forward error ≲ F·(64m + c_f + 1)·2^-513; for m ≤ 2^20 that is ≲ F·2^-486, a 2^50 margin under the stop rule's threshold for every F < 2^372.
8. **From u to the published quantities:** recovery at 1024 is exact expansions rounded once, so each quantity's error is its linear image of u's error plus 2^-1024 relative; the kind-coupled S\* bounds those images body-wise. The mixed-unit step (equilibrated norm to per-kind body scales) is where this is an argument, not a proof.

**Route 2 (first-order comparison).**
9. Both the 512 and 1024 solves are dominated, to first order, by perturbations proportional to their unit roundoffs (formation and backward error), so err_1024 ≈ 2^-512·err_512 in the first-order regime, and acceptance bounds err_1024 ≲ 2^-512·2^-64·S\*/(1 − 2^-512). The first-order regime is κ·2^-512 ≪ 1, which rcond_512 > 2^-511 **suggests but does not certify** (the uncertified step of this route). Near that boundary the formation perturbation at 512 can hide a singular intended K; then the 512 solve is wrong at O(1) and in a different direction from the 1024 solve, and the rule rejects.
10. **The step that relied on §4.1.9, restated under revision 5a.3 (R7 §6.6).** The plan's step 10 said the stop rule can accept a wrong 512 candidate only through a precision-independent (common-mode) error, which revision 2's exact sums remove; F-2 and F-3 refuted that as stated. Restated: the stop rule can accept a wrong 512 candidate only if the 1024 state's error is common to 512 and 1024, which is either a common-mode loss of an exact input (removed by the exact sums) or saturated rounding below 1024's resolution (R7 Lemma 2). Under 5a.3:
    - the 1024 state's recovery resolution is charged by V = 2^(8−1024)·ê;
    - its solve error is measured by the verification estimate, whose residual is one exact sum over the 1024-formed contributions, so K_1024's assembly roundings are included;
    - the formation error of individual 1024-formed entries (Q4: no re-formation above 1024) and the second-order remainder are charged, C_q ≤ 2^-22·2^-64·M_q under θ ≤ 1/2;
    - **honesty:** b = fl↑(2^-64·Φ) bounds the accepted 512 value with the factor 1 + 2^-22, derived with every step certified (R7 §4.1.6.3 Corollary). K4 implements each of these checks (§3 items 9 and 13), and the controls that exercise them are selected at 512 and honest under the tightened predicate (§22.9);
    - **availability:** the factor 4 between λ·2^-512·ê and 2^-64·Φ is the room for the 512 candidate's own resolution noise.
11. **Routes 1 and 2 after 5a.3.** Neither route is now a step of the honesty guarantee at the ceiling; they explain the 1024 state's accuracy, which is availability. **The uncertified steps of both routes are no longer needed** (R7 §6.6): route 1's in step 6 and route 2's in step 9, each resting on the Hager–Higham estimate. The inverse norm is bounded per block by B_c = min(Uc_c, S_c) (Lemmas D and E), and the estimate remains only in the condition screen and the choice of σ_c. [Corrected at addendum 1 (RV19-N2): D said "step 6's uncertified step is gone from route 2"; step 6 is route 1's.]
12. **Route 1's constant under the bounded majorant (R7 §6.6 and §8 item 5; V4-R8).** When the hybrid gate decides by its bounded test, step 3's backward error is relative to |f_i| + Σ_j Ā_ij·|u_j| instead of |f_i| + Σ_j |K_ij||u_j|. Ā ≥ |K|_contrib ≥ |K| entrywise, up to a relative 2^-(q−8) (R7 §5.6 point 1), so steps 4 and 5 hold with ε's matrix majorant Ā, and κ is replaced by ρ·κ with ρ = ‖Ā‖/‖|K|_contrib‖ in step 5's norm, step 5's κ being the componentwise condition over |K|_contrib that steps 2 and 4 use. [Corrected at addendum 1 (RV19-N2): D wrote ‖Ā‖/‖K‖; the bound below bounds ‖Ā‖/‖|K|_contrib‖, and ‖|K|_contrib‖/‖K‖ is not bounded by it.] **The constant, in the unscaled 1-norm (derived per element; the equilibrated norm is argued):** B̄ puts 1 (or 1/L) at each of a 3-component group's entries, where |B| has the components of a unit vector, whose 1-norm is ≥ 1. So each column of Ā_e = g·B̄ᵀ|D|B̄ is at most 9·g times the mean, over the same 3-component group, of |K_e|_contrib's columns; summing over elements and adding the springs (equal in both), ‖Ā‖₁ ≤ 9·g_max·‖|K|_contrib‖₁. In the equilibrated norm the spread of s_i within a group enters as well, which is where step 8's mixed-unit argument already sits. Step 7 becomes ≲ ρ·F·(64m + c_f + 1)·2^-513, and the 2^50 margin holds for every ρ·F < 2^372. Where the coalesced test decides, ρ = 1 and step 7 stands as written. On the controls, no 1024 verification falls back to the bounded test: the attempts that record a bounded gate are at 128, 256 and 512 only, on 13 controls (the 8 single-mode probe cases, LOADONLY-y345, GS-TRANS-y345, GS-ROT-y345-LOADED, M10-ANISO and DIRECTIONAL-WELL; `_run_records/d/k4_retained.log`).

### 9.1 c_f: the formation's roundings, counted from `assemble.rs` (at `3ed6c0e26`)

The line numbers are at `3ed6c0e26`. K's formation is unchanged since: A3a added the exact g (`gram_exponent`, from the same Gram–Schmidt residual) and the bounded and wide formations of the gate and the verification beside it, and made `Contribution` `pub(crate)`; no rounding of K, B or D moved.

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

## 10. Findings F-2 and F-3 (routed to D1 revision 5a.3; resolved by it)

- **F-2:** one member, a prescribed ux = 1 (EA/L = 512) and a load of 2^-300: u cannot represent 1 + 2^-309 at 128 or 256, so the end actions and reactions are 0 at both and S\*(force) = 0; the case is selected at 128 as exact, the truth is N = 2^-300.
- **F-3:** an unloaded body moved rigidly by prescribed values: K_e's one-ulp rigid-mode leakage (the rounded DB) gives reactions of about 2^-(p−27) at every p; S\*(force) is the leakage; the case ends `Unresolved(Ceiling)`. Safe; nothing wrong is published.
- **Routed** as ROOT ruled: S\* had no resolution floor tied to the elastic-action scale, which refuted §4.1.9's claim.
- **Resolved by revision 5a.3 (R7; implemented at A3a and A3b).** V charges the verification's resolution, and the floor Φ at 512 covers what 1024 cannot resolve. In K4, as GEN and R7 expect: F-2, F-2-SPOS and F-2-CEIL are rejected at 128 and 256 by the stop rule and selected at 512, honest; F-3-FREE, F-3-ROT and RIGID-UNLOADED likewise at 512, honest (RIGID-UNLOADED was `Unresolved` under 5a.2). Under the tightened predicate their rows satisfy their own published claims (§22.9). R7-M1 (drop V) moves F-2, F-2-SPOS and F-2-CEIL to 128, where their claims are false (§22.9), and R7-M3 (Φ at every p) moves F-2-CEIL and RIGID-UNLOADED (§17).

## 11. Tests (122 in `K4T/`, with the S11 site table's 3; all pass; one is the V4-S3 probe, §21)

**B (multi-term sum; 16 in `wide_sum_tests.rs`):** the committed vectors' sha256; 397 targeted sums against the Fraction oracle; RV12's counterexample; zero, negation, reuse; the refused span adds nothing; 7 differential streams of 10^5 sums; the four N5 streams (D).
**C (ledger; 7):** netting with (+, 0) and the −2148 quantum; the projection against Fraction at every K4 precision; p = 53 on normal results and the exact form at 1024; RF-CANCEL and check L exact; the prescribed-coupled tie decided by the ledger's tail (K4-M8: loads 2^-128 and 2^-400); order-independent encoding; the Q10 pins (§13).
**Source (4):** canonical order and bodies; every validation refusal; the exact degenerate-axis decision; order-independent encodings.
**E (assembly; 9):** assembled K equals the Fraction emulation bit for bit (7 models × 8 precisions, 5,280 entries); rigid modes within 64 units of 2^-p against the expansion's own terms (worst 1.38); p = 53 against `local_stiffness` (same zero pattern, 8 ulps); K-D5's re-formation at 128 within 1 ulp of the largest entry (bound 64); list permutations bit-identical; the duplicate operand (a sequential fold loses the weak member at 128 and 256); the prescribed-motion control; `from_positions` equals `from_connectivity`; the rhs's prescribed coupling is exact.
**F (factor; 9):** RCM on `sparse_direct`'s small graphs; the rcond control (n = 70, every pivot exactly 1, κ ≈ 4^70: 128 escalates, 256 passes); the negative-energy pair and its definite and singular neighbours; PIVOT (fails at 128, accepted at 256 against 512); witnessed mechanisms refused with no attempt and the RX companion solves; the exact span decision; the non-spanning directional ground (O1); TWO-SPAN's p + 64 correction (O5); the ceiling's residual basis.
**G (schedule and stop rule; in `adaptive_tests.rs`, 19 with L, O8 and the probe):** N05 and N06 at 128; SKEW-K1E-28 (128 fails on the condition screen, 256 accepted against 512); SKEW6-K1E-12 rejected at 128 with its verification reused; four solves at most; ZERO-TORSION-345; ALL-ZERO-BODY; REACTIONS-ONLY; the ceiling case; the exact predicate at ±1 ulp and K4-M14's |q_p| variant; every published quantity takes part (N06: 8 quantity types, 4 kinds); the three-correction cap (N09-B with a perturbed residual basis: 0, 1, 2, 3 corrections, then an escalating residual-gate stop).
**H (recovery; 5):** 43 models against exact references, each row against the claim it publishes (the tightened predicate, §22.9), every published kind compared (u, mag, end, st including N01's t = 0.25, spr, dspr, R, sf, sm, N, T, Mb, Mbs); +0.0; subnormal, underflow and overflow outcomes; N06's torque and spring action; D16's recovery half (a unit control on `recover`: the code-order fold of e_x·u_i loses a term the exact expansion keeps).
**I (combinations; 10):** §8.6 and §8.7.
**J (classification; 5):** 405 row sets bit for bit against the generator's binary64 reimplementation; item 6a's floor where Φ binds and nowhere else (SD-J1's additions: Φ binding and not, the fl↑ boundary, ê < 2^-584); ±1 ulp around t; S\* < 2^-988; b at S\* < 2^-1011 and at 0; k_i and the stress scales (300 and 1,500 vectors); input-derived rows; S8-W's far-node rows (29 and 39 `absolute_verified`).
**K (references; 13):** §12; RF-LARGE at 10 members and six frames at 100 members, each selected at 128 and honest.
**L (budgets, work, determinism):** a case limit equal to the need selects, one less gives `Budget(Case)`; the invocation limit; failed and verification work charged; `from_integer`'s length charged; golden work; runs and permutations deterministic; factor reuse bit-identical to separate solves; the source scan (26 functions, lexer self-control; `directed.rs`, `bound.rs` and `verify.rs` included).
**O8:** five retained-state digests (§11.1).
**Revision 5a.3: directed rounding (2, `directed_tests.rs`):** the directed operations against the Fraction oracle in both directions; a directed result is never on the wrong side, and an exact value does not move (E-UC's directed roundings).
**Revision 5a.3: E and the bounds (6, `scale_tests.rs`):** E-UNIT (g, Ā and E bit-equal to GEN's emulation of §4.1.6.2 at every precision; 3,096 lines of `scale.txt`) and on RF-LARGE at 100 members at 128 and 256; E-UC (Uc_c, S_c and B_c bit-equal to the emulation and never below the exact ‖K̃_c⁻¹‖₁, per block; `bounds.txt`) and on RF-LARGE's six 100-member frames at 256, against GEN's certified upper bounds ‖X‖₁/(1 − ‖R‖₁), where the forced shift succeeds at its first σ on all six; `factor()`'s L and D bits pinned and the shifted loop without a shift reproducing them (A3-0 Q4, Q5; K4-M35); blocks as the free–free components, with data per 7a.
**Revision 5a.3: bound (4, `bound_tests.rs`):** V4's F2 family, bit-equal to the emulation and never below the exact norm; the low-precision stress (P = 10 to 32) keeps every bound above the exact norm and M27's form does not (A3-0 Q15); SD-G5's searched profiles reaching each boundary of 7b to 7d (t just below 1 and at 1, the shift not triggered, succeeding first, at σ/2, failing three times, and σ′ ≤ 0); the exact product of two sums.
**Revision 5a.3: verification (3, `verify_tests.rs`):** Φ exact where the product is normal and rounded up below 2^-584; ê's coupling through the body extent in binary64; an ê that overflows while E encodes stops as E does (§22.4).
**Revision 5a.3: the method (10, `method_tests.rs`):**
- **the controls:** every control's schedule equals GEN's (`outcomes.txt`: 131 cases and 4 combinations at D; 137 cases and 4 combinations at addendum 1) and R7's expectations; at D, 99 selected controls honest against their exact solutions under the tightened predicate (5,490 rows, worst 0.28 of the allowance, at SKEW-K1E-12 end.1.i.1; 13 selected controls had no expectation and were skipped, RV19-2); at addendum 1, every selected control and combination of the lane (117), every row, against GEN's 128-bit expectations (9,412 checks, worst 0.949 at B1-C-A u.0.3); each passing the test-only G5a checker; the named figures (DEMOTION2's relative rows 7 → 4, EXACT-RIGID's b > 0, the probe set's bounded gates);
- **DIRECTIONAL-SPAN under 5a.2** (§22.3);
- **E-CHARGE and E-ESTIMATE** at every verification: C_q, θ_c, W⁺, every factor and the estimate bit-equal to GEN (`charge.txt`: 360 verification states, and 21 that stop before the charge; `estimate.txt`: 1,828 rows against GEN's exact rational solution, A3-0 Q10), and on RF-LARGE at 100 members at 256;
- **E-HEADROOM:** 322 state pairs, |q_P − q_2P| ≤ 2^8·2^-P·ê on each, with the P state's own ê (A3-0 Q11); the worst is 1.55·2^-P·ê (F-3-FREE at 512);
- **SD-G5** (five tests): each test decided exactly at its boundary in R7's order ((a) at ε·M − V and one 2p-ulp above, (b) at W = V/4 and one ulp above, θ at 1/2); the charge's boundaries at 256 and 512 and the translation row at |Δ| + W⁺ = ε·M; the gate's bounded test at its boundary on a best state that is not the last; the g check at 2^(P−16) and 2^(P−15); LEVER2's estimate residual, nonzero where the assembled one vanishes.

### 11.1 Vectors and the O8 emulation

`GEN` (standard library only; imports K3's generator and R1's `references.py` read-only; writes no bytecode) produces `sum_targeted`, `sum_differential` (+ sample), `ledger`, `streams` (+ sample), `formation`, `models` (49 models and 4 combinations, incl. K-D5's D5C-1 controls and K2b's spring-carried case), `r1_cases` (R1's 128 K4 cases through the adapter), `classification` (with SD-J1's selected p, E_fo and E_mo), `o8_states` and `encodings` (Q10, §13), and, for revision 5a.3: `models5a3` (82 models and 4 combinations, with their exact solutions: R7 §7's controls, the V4-S3 probe set, RF-LARGE's twelve frames at 10 and 100 members, CEIL-S, CEIL5A3, EHAT-OVERFLOW and DIRECTIONAL-WELL), `outcomes` (GEN's schedule for all 135), `scale` (E-UNIT), `bounds` and `profiles` (E-UC, SD-G5's 7b–7d profiles), `directed` (the directed operations), `charge` and `estimate` (E-CHARGE, E-ESTIMATE), `r1_large` (RF-LARGE at 10 and 100 members), `directional_span_exact` (§22.3), and `SHA256SUMS` (22 files). `--check` regenerates in memory and compares byte for byte (23 of 23 OK, SHA256SUMS included; 10 min 51 s at B).

**O8 (approved; not disproportionate):** `emulate_state` mirrors, with Fractions rounded to p at every step the Rust code rounds, the formation at p and at p + 64, the exact assembly, the RCM order (a port of `sparse_direct`'s rules), the radix equilibration, the profile LDLᵀ with its exact pivot screen, the reduced rhs, the solves, the p + 64 residual loop and the recovery of Q; `state_sha` encodes `K4RST`. It reproduces the Rust retained states bit for bit:

| Case | p | sha256 (first 12) | corrections |
|---|---|---|---|
| N05 | 128 | `a0351c9e0d4a` | 0 |
| N05 | 256 | `e94741023f0f` | 0 |
| N06 | 128 | `4a8145155a9a` | 0 |
| N06 | 256 | `e17a84a7042a` | 0 |
| SKEW-K1E-4 | 128 | `797c1a3978b9` | 0 |

### 11.2 S\*-dependent assertions, under revision 5a.3 (R7 §6.2)

- **Unchanged,** as R7 §6.2 measured or derived: SD-G1, SD-G2, SD-G3, SD-G4, SD-G6, SD-I1, SD-I2, SD-J2 and SD-K1. SD-G3 and SD-I2 have E = 0, so V = 0, Φ = 0 and W = 0. SD-G4: REACTIONS-ONLY is still rejected at 128 by a reaction and selected at 256.
- **SD-G5:** the translation vectors change only by W⁺, and R7's additions are in `method_tests.rs` and `bound_tests.rs` (above). R7's two "not tested" items (θ and g on a body without data; a body with a zero-state block) are covered by THETA-ZERO-BODY and THETA-STUB as controls.
- **SD-L1:** re-pinned at A3a (§14): every formation forms g exactly, and at p ≥ 256 the condition screen's solves give est_c per block; the E pass, the estimate, the bounds, the shift and the charge are their own stages.
- **SD-J1:** values unchanged for p ≠ 512 or E = 0; the format gains (selected p, E_fo, E_mo); item 6a's vectors added.

The A2 list, for reference: SD-G1 (SKEW6 rejected at 128; SKEW-K1E-60's four attempts), SD-G2 (ZERO-TORSION-345 at 128), SD-G3 (ALL-ZERO-BODY exact, bound 0), SD-G4 (REACTIONS-ONLY rejected by a reaction), SD-G5 (the predicate boundary tests), SD-G6 (the every-quantity control), SD-L1 (the golden work: the stop-rule column), SD-I1 (2·SKEW6 − SKEW6 rejected at 128), SD-I2 (SKEW6 − SKEW6 at 128, exact +0.0 with bound 0), SD-J1 (the classification vectors), SD-J2 (S8-W's far-node rows), SD-K1 (the floor check's not-covered set). More generally every asserted selected precision rests on the stop rule; these are the assertions where S\*'s form decides.

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
- **The floor check** (S\* from the reference values with K4's §4.1.6.1 functions, one body per case, variant F for twist and extension): the not-covered set equals `floor_kinds.json`'s F lists (F_rec for RF-CANCEL) restricted to K4's cases, row for row: RF-WEAK 46, RF-CANCEL 3, RF-SKEW 2. Unchanged under revision 5a.3 (SD-K1, R7 §6.2): every R1 case is selected at 128, where Φ does not apply, and the counts above are D's.
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

### 12.6 RF-LARGE (revision 5a.3's lane; A3-0 Q13)
- R1's RF-LARGE frames through the adapter (`r1_large.txt`, `models5a3.txt`): at 10 members, the six cases (CHAIN, TREE and CONT, each AX and ROT) are selected at 128 with 882 passes; at 100 members each of the six frames (600 free DOFs) is selected at 128, with 1,312 or 1,462 passes. [Corrected at addendum 1 (RV19-2): at D only the 10-member frames were in the controls test, and without expectations, so neither size was checked against the tightened predicate, and G5a did not run on the 100-member frames.] At addendum 1 the 10-member frames are checked in the controls test and the 100-member frames in their references tests: every row against GEN's high-precision expectations (`solve_hp`), and G5a. All twelve are honest (the 100-member frames' worst is 0.97 of an allowance, CONT-ROT's R.1.1).
- The shift runs at every 10-member verification except CONT-AX's, where Uc is tight (E-CHARGE's figures); on the six 100-member frames the forced shift succeeds at its first σ against GEN's certified upper bounds (E-UC, §22.2).
- Each 100-member frame takes 1 to 4 s in the debug suite, so all stay in the default suite; no `--ignored` lane was needed.

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

**Per attempt, by stage** (own stages: the case's; shared: formation, assembly, residual formation, factor and condition, counted in full against the case, once against the invocation; the verification pass and its shared stages are charged to the verification attempt, R7 item 7's "Work"). The stop rule is charged to the candidate: it is `decide`, with V, the estimate's test, θ, g and the charge (R7 §5.1), so it is 0 at a verification. Figures at D (unchanged since A3b; `golden_work_counts` pins them, SD-L1):

| Case | p | rhs | solve | refinement | recovery | stop rule | verification pass | formation | assembly | residual formation | factor | condition |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| N05 | 128 | 104 | 10,038 | 23,252 | 6,828 | 492,883 | — | 27,348 | 949 | 28,297 | 28,849 | 90,465 |
| N05 | 256 | 104 | 10,038 | 23,841 | 6,860 | 0 | 31,870 | 27,348 | 949 | 92,889 | 28,891 | 97,593 |
| N06 | 128 | 104 | 10,038 | 40,772 | 6,817 | 492,784 | — | 27,348 | 949 | 28,297 | 28,849 | 90,458 |
| N06 | 256 | 104 | 10,038 | 41,365 | 6,865 | 0 | 32,101 | 27,348 | 949 | 92,889 | 28,891 | 97,573 |
| TWO-SPAN | 128 | 152 | 8,460 | 37,577 | 6,706 | 527,858 | — | 55,166 | 1,720 | 57,414 | 20,528 | 79,947 |
| TWO-SPAN | 256 | 152 | 8,460 | 38,808 | 7,063 | 0 | 39,853 | 55,982 | 1,960 | 187,516 | 20,636 | 86,930 |
| SKEW6-K1E-12 | 128 | 352 | 64,278 | 81,078 | 19,584 | 2,738 | — | 192,014 | 5,427 | 200,918 | 412,168 | 449,248 |
| SKEW6-K1E-12 | 256 | 352 | 64,278 | 100,032 | 21,290 | 654,194 | 660,264 | 197,402 | 5,637 | 621,014 | 412,818 | 458,157 |
| SKEW6-K1E-12 | 512 | 688 | 226,974 | 164,905 | 60,330 | 0 | 2,177,595 | 622,004 | 9,933 | 2,168,204 | 1,464,694 | 1,435,788 |

SKEW6-K1E-12's 128 candidate is rejected by the stop rule; its 256 verification becomes the candidate (its stop rule, 654,194) and is also the 128 attempt's verification (its pass, 660,264).

**The verification pass and its shared stages** (revision 5a.3; `bounded_gate` is 0 on all four, since the coalesced gate passes):

| Case | p | scale (E) | estimate | charge | bound | shift | bounded formation (Ā) | wide formation (q_W) | Uc | total, own + K4 sum | shared | verification shared |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| N05 | 256 | 2,016 | 17,519 | 11,413 | 922 | 0 | 27,961 | 91,316 | 14,419 | 65,838 + 6,875 | 247,670 | 133,696 |
| N06 | 256 | 2,021 | 17,519 | 11,619 | 942 | 0 | 27,961 | 91,316 | 14,325 | 83,416 + 7,057 | 247,650 | 133,602 |
| TWO-SPAN | 256 | 4,527 | 16,378 | 18,005 | 943 | 0 | 64,854 | 185,142 | 13,176 | 81,006 + 13,330 | 353,024 | 263,172 |
| SKEW6-K1E-12 | 256 | 11,649 | 106,500 | 86,890 | 2,380 | 452,845 | 190,803 | 617,234 | 152,204 | 1,412,296 + 88,114 | 1,695,028 | 960,241 |
| SKEW6-K1E-12 | 512 | 25,733 | 355,447 | 243,205 | 6,827 | 1,546,383 | 445,395 | 2,171,792 | 389,090 | 2,504,272 + 126,220 | 5,700,623 | 3,006,277 |

- The 128 candidates' totals (own + K4 sum; shared): N05 530,316 + 2,789, 175,908; N06 547,822 + 2,693, 175,901; TWO-SPAN 576,310 + 4,443, 214,775; SKEW6-K1E-12 163,112 + 4,918, 1,259,775.
- **The shift** runs only on SKEW6-K1E-12 of these four (R7 measured it on 3 of its 59 controls, SKEW6-K1E-12 among them); there it is 69 % of the 256 pass and 71 % of the 512 pass, and 1.10 and 1.06 times the verification's own factor, within R7 §6.7's "at most quadruples". On large models its cost is for K6b and V-K (R7 §6.7).
- **Against A2:** the stop-rule stage grows from 80,007 to 492,883 on N05, since it is now `decide` with tests (a) to (d) (measured, not analysed further); refinement is unchanged; formation and residual formation grow by g's exact formation (§7), and condition, at p ≥ 256, by est_c's per-block sums.

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

The N5 streams use K3's operand rules (a test-only port of K3's SplitMix64 and generator functions) and K3's Fraction oracle. The debug wall times are observations (A1). K4's whole suite takes 205.6 s of debug wall time at 4 threads (A2): O11's growth is about 3.4 minutes. At D, K4's 122 tests take 382.6 s at 2 threads (`_run_records/d/k4_retained.log`), the N5 streams included.

## 16. The F2a and V-K interface (exact signatures at `8f8023a20`)

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
    source_encoding: Vec<u8>, ledger_encoding: Vec<u8>, retained_state_encoding: Vec<u8>,
    // revision 5a.3 (R7 §5.8); stop_rule now holds the worst (|q_p − q_2p| + V_q)/M_q per body and kind, rounded upward:
    resolution_scale: Vec<(u32, u64, u64)>,          // per body: E_fo and E_mo bits, uncoupled, rounded upward, finite
    verification_estimate: Vec<(u32, Kind, f64)>,    // per body and kind (force, moment): worst Ŵ_q/V_q, rounded upward
    verification_charge: Vec<(u32, Kind, f64)>,      // per body and kind: worst C_q over its allowance, rounded upward
    theta: Vec<(u32, f64)>,                          // per body: the largest θ_c over its blocks with data (0 without)
    certified_bound: Vec<(u32, u64)>,                // per body with a data block: B_b's bits, rounded upward (Q9: no entry otherwise)
    floor: Option<Vec<(u32, u64, u64)>> }            // Φ_fo and Φ_mo bits per body when the selected precision is 512
pub(crate) struct AttemptRecord { precision: u32, role: AttemptRole, outcome: AttemptOutcome, residual_basis: u32, corrections: u8,
    pivot_margin_min: Option<f64>, rcond: Option<f64>, residual_worst: Option<f64>, work: AttemptWork, k4_work: SumWork,
    stages: StageWork, shared_work: u64, shared_stages: StageWork, shared_built_here: bool, stop_rule_work: u64, storage: StorageCounts,
    // revision 5a.3:
    gate: Option<GateTest>, verification_work: u64, verification: Option<VerificationSummary>,
    verification_shared_work: u64, verification_shared_built_here: bool }
pub(crate) enum GateTest { Coalesced, Bounded { state: u8, evaluated: u8 } }   // which test passed; the best state chosen of those evaluated
pub(crate) struct VerificationSummary { resolution: Vec<[f64; 2]>, theta: Vec<f64>, bound: Vec<Option<f64>>, data_blocks: usize,
    shift_factorizations: u8, uc_missing: Option<usize>, g_max: u32, g_violation: Option<u32> }
pub(crate) enum AttemptRole { Candidate, Verification, VerificationThenCandidate }
pub(crate) enum AttemptOutcome { Accepted, Verified, Rejected(AttemptReason), Failed(AttemptReason), Solved }
pub(crate) enum AttemptReason { Stop(AttemptStop), StopRule { quantity: QuantityId, body: u32, kind: Kind }, VerificationFailed,
    // revision 5a.3's reasons (R7 item 13), in the rejection order (a), (b), uc, θ, g, (d):
    VerificationEstimate { quantity: QuantityId, body: u32, kind: Kind }, Uc { body: u32 }, Theta { body: u32 },
    GValidity { member: u32 }, Charge { quantity: QuantityId, body: u32, kind: Kind } }
pub(crate) enum AttemptStop { Budget(BudgetScope), Span, Exponent, Arithmetic(WideError), Structure, Pivot { global_dof: usize },
    ZeroDiagonal { global_dof: usize }, NegativeEnergy { i: usize, j: usize }, Condition, ResidualGate { global_dof: usize },
    ResolutionScale { body: u32, kind: Kind } }   // revision 5a.3: E or ê does not encode (A3-0 Q8; the A3b ruling)
pub(crate) enum BudgetScope { Case, Invocation }
pub(crate) enum Refusal { MechanismWitnessed { body: u32, rigid_parameters: [f64; 6] }, GeometryUnavailable { body: u32, error: StructuralError },
    NegativeEnergy { i: usize, j: usize }, LedgerUnavailable(LedgerRefusal), Structure }
pub(crate) enum UnresolvedReason { Ceiling, Budget(BudgetScope), ExactSumSpan, ExponentRange, ZeroDiagonal { global_dof: usize }, Arithmetic(WideError),
    ResolutionScaleUnencodable { body: u32, kind: Kind },   // F2a: receipt_encoding (Q8)
    CertifiedBoundUnencodable { body: u32 } }                // R7 §5.8: a B_b not below 2^1024; unreachable at a selected p (θ ≤ 1/2)
pub(crate) enum BodyGeometry { Restrained, NumericallyUnresolved, NotAssessed(Vec<(u32, SpringKind)>) }
pub(crate) enum LedgerRefusal { Accumulator(SumError) }
pub(crate) enum CombinationOutcome { Selected(Box<RetainedSolve>), Unresolved { reason: CombinationReason, attempts: Vec<AttemptRecord> } }
pub(crate) enum CombinationReason { CombinationUnresolved, NoOperands, NestedCombination, OperandsDiffer,
    LedgerUnavailable(LedgerRefusal), Unresolved(UnresolvedReason), Refused(Refusal) }
pub(crate) struct StageWork { formation, assembly, residual_formation, factor, condition, rhs, solve, refinement, recovery, stop_rule,
    bounded_gate, scale, estimate, charge, bound, shift, bounded_formation, wide_formation, uc: u64 }   // the last nine: revision 5a.3
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
pub(crate) fn threshold(s_star: f64) -> f64;                 // fl(R·S*), unchanged by 5a.3
pub(crate) fn absolute_bound(s_star: f64) -> f64;            // fl↑(2^-64·S*), unchanged by 5a.3
pub(crate) fn classify(value: f64, s_star: f64) -> RowClass; // unchanged by 5a.3; S* is floored first at 512
pub(crate) fn classify_rows(layout: &[QuantityMeta], values: &[Binary64Outcome], extents: &[f64]) -> Publication;
pub(crate) fn classify_rows_floored(layout: &[QuantityMeta], values: &[Binary64Outcome], extents: &[f64],
    floor: Option<&[[f64; 2]]>) -> Publication;              // item 6a: Φ_fo, Φ_mo per body at a selected 512
pub(crate) fn coupled_scales(s: [f64; 4], extent: f64) -> [f64; 4];
pub(crate) fn body_extent(coordinates: &[[f64; 3]]) -> f64;
pub(crate) fn stress_scale(fo: f64, mo: f64, area: f64, modulus: f64, k: f64) -> f64;
pub(crate) fn intensified_k(i: f64) -> f64;
// revision 5a.3 (`retained::verify`), for F2a's receipt and D2's G5a:
pub(crate) const PHI_SCALE_BITS: u64 = 0x2490_0000_0000_0000;   // 2^-438
pub(crate) fn e_hat(e: [f64; 2], extent: f64) -> [f64; 2];      // item 6a's coupling in binary64
pub(crate) fn resolution_hats(resolution: &[[f64; 2]], extents: &[f64]) -> Result<Vec<[f64; 2]>, AttemptStop>;
pub(crate) fn phi_512(e_hat: f64) -> f64;                       // fl↑(2^-438·ê)
```
**For V-K** (Q7's cross-crate equality test): `retained::factor::reverse_cuthill_mckee(adjacency: &[Vec<usize>]) -> Vec<usize>`.
**K3's and K4's work types in the evidence:** `wide::multi::{Binary64Outcome, AttemptWork, WidthWork}`, `wide::WideError`, `wide_sum::SumWork`, and `crate::exact_sum::SumError`, `crate::structural::StructuralError` (already public).

**The export list (Q9: the first consumer outside FK adds it).** Raise to `pub` and re-export from `FK/structural.rs` (for example `pub use retained::{…}` beside the private `mod retained;`):
- from `adaptive`: `solve_cases`, `solve_case`, `CaseLimit`, `InvocationMeter`, `CaseOutcome`, `RetainedSolve` (its five accessors), `PrecisionState` (`precision`, `published`, `encoding`), `Publication`, `PublishedRow`, `RowClass`, `RetainedEvidence`, `AttemptRecord`, `AttemptRole`, `AttemptOutcome`, `AttemptReason`, `AttemptStop`, `StageWork`, `StorageCounts`, `BudgetScope`, `Refusal`, `UnresolvedReason`, the seven constants, `threshold`, `absolute_bound`, `classify`, `coupled_scales`, `body_extent`, `stress_scale`, `intensified_k`;
- from `source`: `PrimitiveSource` (with `new`, the encodings and the read accessors), `SourceParts`, `StraightMember`, `Spring`, `DirectionalSpring` (V-K only), `SpringKind`, `Constraint`, `NodalLoad`, `Station`, `SupportGroup`, `Dof`, `Component`, `SourceError`, `MemberProperty`;
- from `recover`: `QuantityId`, `Kind`, `End`; from `factor`: `BodyGeometry`, and `reverse_cuthill_mckee` for V-K; from `ledger`: `LedgerRefusal`; from `combine`: `RetainedCombination`, `CombinationOutcome`, `CombinationReason`;
- K3's `Binary64Outcome`, `AttemptWork`, `WidthWork`, `WideError`, and K4's `SumWork` (as field types of the evidence).
- **Not exported:** `CasePrep`, `GroupPrep`, `GroupCache`, `Shared`, `Solved`, `StageGuard`, `ExactWideSum`, `RetainedLedger`'s internals, the formation and factor functions: "No caller can supply a matrix, factor, closure or label."
- **Revision 5a.3 adds to the export list:** `GateTest`, `VerificationSummary`, `classify_rows`, `classify_rows_floored` (with `recover`'s `QuantityMeta` and `layout`, which they read), and from `verify` `PHI_SCALE_BITS`, `e_hat`, `resolution_hats` and `phi_512`. The classification helpers' definitions are unchanged; item 6a floors S\* before them. Not exported: `verify_state`, `VerificationReport`, the bounds (`bound.rs`) and the directed operations (`directed.rs`), which are the kernel's own.

## 17. Suites, T9 and mutations

- **Suites (checkpoint B; 39 manifests, `--no-fail-fast`, CI's numerical cargo profile).** Run on main `7ac7b1c37`'s piping source with K4's FK overlaid (a `git archive`), against the Mac baseline of the same piping tree (K6's final-head run; `operation_applier` from its fresh-target re-run), under the host rule (`-j 4`, `RUST_TEST_THREADS=2`, own target). 39 manifests on both sides; the only change is `core/solver/frame_kernel`: 267 → 389 passed, K4's 122 tests. The failing sets are identical: the three known Mac failures (`product_physics`'s `s11g_tests::t13_committed_fallback_uz_is_byte_identical`; two `runner/headless` load-reference tests). FK 389, SD 30 and NI 134 pass (`_run_records/b/`).
- **FK's full suite at D,** after the tightened predicate: 389 passed, 0 failed (the lib's 323 in 629.7 s of debug wall time at 2 threads; `_run_records/d/fk_full.log`). The controls test and the recover tests, re-run on their own with output: 6 of 6 pass (`d/honesty.log`).
- **T9 and the gate are not run** (K4's brief: kernel only). The scan (`_run_records/b/kernel_only_scan.txt`, re-run at D): `mod retained;` is private in `FK/src/structural.rs`; no item in K4's files is `pub` without `(crate)`; no file outside `FK/src/structural/retained/` and `K4T/` names a K4 module; FK re-exports nothing from `retained`; `net_parts` is called only by `retained/ledger.rs` and its tests. So no product path, in FK or in any crate depending on it, can reach K4's code.
- **Mutations (checkpoint C; ROOT accepted).** From clean `git archive 8f8023a20` copies, one per mutant, each with a fresh target, serialized (one cargo job, `-j 4`, `RUST_TEST_THREADS=2`), exact string edits with their match counts checked; NONE first. 86 mutants (D13b, a second form of D13, among them) and two controls (NONE; NONE-S11 on the whole `core` tree, since the S11 site table reads other crates):
  - **R7 §7's killable list:** M1–M13, M15, M16, M18, M20, M22, M23, M25, M26, M28 and M29: all killed. M20, M23, M25, M26, M28 and M29 are availability kills, as R7 states; M16 (the last eligible state instead of the best) is killed at evidence level by SD-G5's gate test.
  - **K4-M34 to K4-M40** (5a.3's implementation mutants): all killed.
  - **K4's earlier mutants and the D-series,** where they still apply: all killed. K4-M24 is killed by CEIL5A3 (§8.4). K4-M30 now describes the design (O7's start rule was superseded), so its inverse, K4-M30i, is run and killed. K4-M12 and D13 are killed at bit level only (E-CHARGE's r̂), since the corrections repair the fold; D13b is killed by B1-L (ratio 5e8). K4-M33 is killed by `a_combined_prescribed_value_is_published_from_its_exact_sum_rounded_once` (§8.7).
  - **The derivation guards** (R7-M17, M21, M24, and M27 at design precision) move no control, as R7 expects, and each is caught at evidence or unit level: M17 by SD-G5's decision and charge-boundary tests; M21 by the golden work; M24 by E-CHARGE; M27 by the low-precision stress and E-CHARGE (the mutation table's "Killed by" column). They are kept for the derivation (§22.6).
  - R7-M3 leaves REACTIONS-ONLY at 256 in K4 (R7 expected 128; ROOT's note 1 at C).
  - Builds took 6 to 8 s each; the recorded test time is 29 minutes over the 88 runs (NONE, which runs every K4 test, 10 min 12 s). The evidence pass (a print-only insertion reporting every control's schedule and honesty) covered 32 mutants and NONE in 9 min 21 s. The full table is `_run_records/c/mutation_table.md`; the raw results are `results.jsonl` and `evidence.jsonl`.
- **The tightened predicate at D** (§22.9): the evidence pass was re-run for R7-M1, K4-M24 and K4-M37 with the tightened `compare_honest` (`_run_records/c/evidence_tight.jsonl`).

## 18. Toolchain and host

- `aarch64-apple-darwin` (macOS 26.6.2), rustc and cargo 1.97.1 (`RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`), `CARGO_INCREMENTAL=0`, `--offline --locked`; target `<wt>/k4-target` (the mutants each a fresh target of their own); Python 3.13 standard library for `GEN` (`<VENV>` not needed). A1 and A2 ran with `-j 8`, `RUST_TEST_THREADS=4` and at most two cargo jobs; from A3 on, by ROOT's host rule, one cargo job, `-j 4`, `RUST_TEST_THREADS=2`.
- The Mac host rules applied throughout: no dense matrix above 1,000 members (the 1,005-member chain is refused by geometry before any factor; RF-LARGE's largest frame has 100 members and is factored on its profile; the exact-block oracle builds dense 12×12 systems only). The memory guard (`<wt>/guard/memguard.sh`, floor 35 %) ran throughout, from before A1, and logged no kill.

## 19. What was not done, and open items

- **Open or argued, from the selection ("D1 revision 5a.3 SELECTED") and R7 §9; none is a step of the honesty guarantee:**
  - **Kept for the derivation, with no kill:** R7-M17, M21, M24, and M27 at design precision (§17, §22.6).
  - **Availability-only kills:** R7-M20, M23, M25, M26, M28 and M29 (§17).
  - **Argued, not derived:** M27's no-kill claim at the design's precisions (it rests on the condition screen, which uses the estimate); the θ-binding reading of THETA-STUB-COUPLED (θ = 28 at the verification of its 128 candidate, in R7's emulation and in K4; K4 rejects 128 by `theta` and selects 256, as GEN and R7 expect); LEVER2's outcome under every elimination order (K4 has RCM's).
  - **Measured, not proved:** the availability figures, and that no block is both est-fooled and Uc-loose (R7's emulation). In K4, the HH-FOOL LOADED forms, where all three shifts fail, have B = Uc, as E-CHARGE asserts.
  - **Standing premises:** Lemma B's count, as V4 recounted it against the Rust; Lemmas D and E tied to K4's factor loop as read, which is byte-identical since A3a (§2); K\* nonsingular per body (§6 item 4 for what 5a.3 adds there).
  - **Out of scope for now:** W1b until F3 meets R7 §6.5's obligations; the shift's cost on large models, for K6b and V-K (§14 has the small models' figures).
- **K4's own limits:**
  - G5a item 4's unit conversion is not exercised: the controls are in SI units (as in R7's emulation).
  - Route 1's constant under the bounded majorant is derived in the unscaled 1-norm and argued in the equilibrated norm (§9 step 12).
  - A mechanism in a block that carries no data is not detected there (§6 item 4).
  - The R1 families R7 §6.1 lists as not measured, and T1's support-motion fixtures, were not surveyed.
- **Routed:** Q9 (a body with no data block has no B_b entry and θ = 0) to F2a and D2.
- **Deferred by ruling:** F2a's limits (Q5 amended); the public export (Q9 of the brief; §16's list); unifying `Binary64Outcome` with K2b's `Representability` (Q11); a full geometric treatment of partial directional grounds (O1, W4/K5).
- **For the reviewer:** §22.1 (Q12's support-group E, honesty-relevant), §22.2 (the certified-bound conditions), §9 steps 10 to 12, and §6 item 4.
- **Incidents:**
  - At A1 the generator's imports left git-ignored `__pycache__` directories in K3's test directory and R1's folder; they were removed before any commit, and `GEN` now sets `sys.dont_write_bytecode`.
  - My A3b report said DIRECTIONAL-SPAN moved "from 128"; 5a.2 selected it at 256. Corrected at B (§22.3) and bracketed in ROOT's A3b ruling.

## 20. Records (`IMPLEMENTATION/K4/_run_records/`)

Machine paths are replaced by placeholders (`<wt>` the T3 worktree root, `<scratch>` the session scratch folder, `<repo>` and `<home>`); nothing else is edited. `SHA256SUMS` covers every file in the folder (`shasum -a 256 -c SHA256SUMS` from inside it).

| Folder or file | Contents |
|---|---|
| `cp0/` | the checkpoint-0 plan (`CHECKPOINT0_PLAN.md`, `e1253faa…`) and its probes (the exact-formation probe, the by-kind count, the 1,005-member chain model) |
| `a1/` | A1's K4 test log; the K3a, K3 and K4 generators' `--check` logs; R1's first run through the adapter; the generator's first runs, and a staging copy of its section E (`gen_part2.py`); the A1 combination probes |
| `a2/` | A2's K4 test log and the generator's `--check` logs; the storage-count script (§14) |
| `a3a/`, `a3b/` | the K4 test logs and the generator's `--check` logs at A3a (18 of 18 OK) and A3b (22 of 22); FK's other lib tests at A3b |
| `b/` | the 39-manifest run (`suites.log`, `suites/` per manifest, `manifests.txt`), its runner and the comparison with the baseline (`suites_vs_baseline.txt`, `compare_b.py`); the generator's `--check` at B (23 of 23); the kernel-only scan, re-run at D |
| `c/` | the mutation harness (`mut.py`, `mutants.py`, `evidence.py` as run at D, `render2.py`), the raw results (`results.jsonl`, `evidence.jsonl`, `evidence_tight.jsonl`), the table (`mutation_table.md`), the campaign logs, each mutant's test logs concatenated (`logs/`), and each evidence run's controls log (`evidence_logs/`; NONE, R7-M1, K4-M24 and K4-M37 are D's tightened runs) |
| `d/` | FK's full suite at D (`fk_full.log`), the controls and recovery tests with output (`honesty.log`), and K4's 122 tests with output (`k4_retained.log`: 382.6 s at 2 threads) |
| `rv19/` | addendum 1 (RV19's review): GEN's regenerations and its `--check`, K4's suite, FK's full suite and the S11 site table on the fixed tree, the mutation harness, results and logs (`mutations/`), the evidence pass, and the changed files with their sha256 |
| `source_sha256.txt` | the sha256 of K4's source, test and vector files at D |
| `toolchain.txt` | the toolchain, host and host rules |
| `SHA256SUMS` | the folder's own checksums |

## 21. V4-S3: the coalesced residual gate refuses a chord-component y_reference (resolved by 5a.3's hybrid gate)

ROOT reversed its O12 ruling (`085638e58`), and revision 5a.3 made the gate hybrid (R7 §5.6; §3 item 6). **What follows is the A2 probe, kept as the record of the coalesced gate;** the probe test still reports the coalesced ratios. `probe_v4_s3_y_reference_with_a_chord_component` (`adaptive_tests.rs`, a report with no assertion; the controls assert) solves a cantilever run along (3,4,0) (1 or 3 members of the N-series section, the root fixed, a tip load, no prescribed motion) and reports, per precision, the worst gate ratio |r_i|(2^p − m_i)/(64·m_i·d_i) at each evaluation of the refinement loop (the first solve and up to three corrections), with M03's coalesced d_i.

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
- **Under revision 5a.3 (A3b):** the 20 probe cases are controls (`PROBE-y345-*`, `PROBE-y001-*` in `models5a3.txt`) and, with LOADONLY-y345, **all are selected at 128, honest**, as R7 §5.6 predicts: the 8 single-mode y_ref (3,4,5) cases pass the bounded test at 128 on their best evaluated state (the second to the fourth of the two or four states evaluated), and the other 12 pass the coalesced test. GS-TRANS-y345 and GS-ROT-y345 are selected at 512 and GS-ROT-y345-LOADED at 128, as R7 states. The probe set's worst charge at 256 is 7.4e-49 of its allowance.
- **The mutants:** R7-M12 (the coalesced gate only, K4 at A2) and R7-M7 (the entrywise operator in Ā) return the 8 single-mode cases, LOADONLY-y345, GS-TRANS-y345 and GS-ROT-y345-LOADED to `Unresolved`; both are killed (§17).

## 22. D1 revision 5a.3: derivations, moved outcomes, guards, emu7 and the tightened predicate

### 22.1 Q12: the support-group E meets R7's requirements for the group's published magnitude

**K4's form** (`verify.rs` `formation_scale`, last block). For a support group at node n and each of its two magnitudes (force: components c = 0..2; moment: c = 3..5), E_group is one exact sum, rounded once to nearest at P (a stage, R7 §3.1 "Stage rounding"), of every contributor's own E_q over the range's components: the reaction's E_c = |f_c| + Σ_j Ā_cj·|u_j| when c is restrained, each grouped spring's E = |k|·|u_g| on its component, and each grouped directional spring's E for component c, Σ_b |k_cb|·|u_b|. Each contributor's E is the value of its own row, formed as §3.1 forms it. This is R7 §3.1's "Support-group components and magnitudes: the sum of their components' E_q", with each component's E_q the sum of its contributors'.

**What R7's lemmas require of E_q for a published row q** (R7 §3.1, §3.3, Lemma B, §6.3), and why E_group meets each for q = ‖v‖₂, v = (v_c) with v_c = Σ (contributors of component c):

1. **The resolution bound** (§3.3: |q_P − R\*(u_P)| ≤ λ_q·2^-P·E_q with λ_q ≤ 68g, the count's row "Support-group components and magnitudes ≤ 20g + 48"). Each contributor a of v_c is formed with |a_P − a\*| ≤ λ_a·2^-P·E_a (its own row's count: reactions ≤ 20g + 45.5, springs 2, directional ≤ 6), so |v_c,P − v_c\*| ≤ Σ_a λ_a·2^-P·E_a + 2^-P·|v_c,P| (the component's own sum, rounded once), with |v_c| ≤ Σ_a E_a·(1 + O(2^-P)) because each |a| ≤ E_a to first order (E_a is the same chain with every operand in absolute value). The magnitude's own roundings (squares, their sum, the square root) add ≤ 2.5·2^-P·‖v‖₂. By the reverse triangle inequality, |‖v_P‖₂ − ‖v\*‖₂| ≤ ‖v_P − v\*‖₂ ≤ Σ_c |v_c,P − v_c\*|. With ‖v‖₂ ≤ Σ_c |v_c| ≤ Σ_c Σ_a E_a = E_group,exact, the total is ≤ (max_a λ_a/g + 1 + 1.5)·2^-P·E_group,exact. Counted against E, which carries the member factor g (Ā = g·B̄ᵀ|D|B̄), a reaction's 20g + 45.5 is at most 65.5 relative to its E; the component's own sum adds 1; the magnitude adds 1.5 (the rounded sum of squares, halved by the square root, and the square root's own rounding). So the count is ≤ 68·2^-P·E_group,exact (69 with the coarser 2.5), within λ = 2^8 and the Corollary's slack of 62·2^-2p·ê either way, with Lemma B's factor 1 + 2^-8.8 for the higher-order terms, as for every other row. [Corrected at addendum 1 (RV19-N1): D's "(20g + 49) … fits λ = 2^8 (≤ 68g)" is false at g = 1.]
2. **The rounding direction.** E_group,P is E_group,exact rounded once to nearest at P: |E_group,P − E_group,exact| ≤ 2^-P·E_group,exact, the same relative stage error every other E_q carries and Lemma B absorbs. E(body, kind) is then the maximum over the body's force (moment) rows including this one, rounded **upward** once to binary64 (`resolution_scale`), R7 §3.1's direction; so V_q = 2^(8−P)·ê, the (b) threshold and the (d) allowance are formed from a value not below the group's own E_q.
3. **The zero rule** (G5a item 2). E_group = 0 forces every contributor's E_a = 0, hence each contributor's value 0 (a reaction's E_c = 0 forces f_c = 0 and every Ā_cj·|u_j| = 0, so every product K_cj·u_j = 0; a spring's or directional spring's E = 0 forces its action to 0), hence v = 0 and the published magnitude is +0.0.
4. **The sanity and lower bounds** (G5a items 3 and 4). |q| ≤ E_group·(1 + O(2^-P)) by the bound in 1, so the item-6 coupled S\* of the group's kind stays ≤ ê·(1 + 2^-40) wherever the other rows' do; item 4 bounds ê below by the members' rows, which the group's inclusion can only raise.
5. **The estimate and the charge** (§4.1.6.3). Ŵ for the magnitude is ‖v(δ̂)‖₂ recovered at P, which bounds |‖v(u\*)‖₂ − ‖v(u_P)‖₂| exactly by the reverse triangle inequality (|‖a‖ − ‖b‖| ≤ ‖a − b‖; "to first order" at D, corrected at addendum 1, RV19-N1), so the (b) test on it is the test R7 states; ‖ā_q S‖₁ for the magnitude is formed from the same contributor sum (the `formation_scale` call with w = s and upward stages), so C_q bounds the magnitude's formation charge as it does each component's.

So E_group meets, for the group's published magnitude, every requirement R7's lemmas place on E, and it is rounded in R7 §3.1's direction (nearest per stage, upward to binary64 for E(body, kind)). emu7 has no support-group rows; GEN emulates K4's form, and E-UNIT, E-CHARGE and the controls pin it.

### 22.2 The certified upper bound on ‖K̃_c⁻¹‖₁ for blocks above 40 DOFs (ROOT's ruling on A3a)

`gen_k4_vectors.py` `inv_norm1_upper` (RF-LARGE at 10 and 100 members; blocks of 40 DOFs or fewer keep the exact rational inverse):
- X ≈ K̃⁻¹ is formed in fixed point (integers scaled by 2^1024, truncating divisions), K̃'s dyadic entries scaled exactly to integers by 2^s.
- **‖X‖₁ is formed exactly as a rational** (the largest column sum of |X|, an integer over 2^1024), and **‖R‖₁ = ‖I − K̃X‖₁ exactly as a rational** (each column of 2^(s+1024)·I − (2^s·K̃)·(2^1024·X) in exact integers, over 2^(s+1024)). No rounding enters either.
- **‖R‖₁ < 1 is asserted explicitly** (and ‖R‖₁ < 2^-100, so that X is accurate), and the bound emitted is the exact rational ‖X‖₁/(1 − ‖R‖₁) ≥ ‖K̃⁻¹‖₁ (K̃⁻¹ = X(I − R)⁻¹, ‖(I − R)⁻¹‖₁ ≤ 1/(1 − ‖R‖₁)). E-UC asserts Uc_c, S_c and B_c ≥ that bound exactly, which is at least as strong as against the exact norm.

### 22.3 DIRECTIONAL-SPAN: 5a.2's publication was within its claim (ROOT's A3b ruling)

- **A correction to I12's A3b status.** 5a.2 selected DIRECTIONAL-SPAN at **256**, not 128. Its 128 candidate was rejected by 5a.2's own stop rule at u(0, Rx) (layout index 3; |q_128 − q_256| ≈ 5.6e14 × 2^-64·M: at κ ≈ 2^104 the 128 state's soft rotation is off by a relative 3.4e-5, and the 256 state's by far less than 2^-64). It was accepted at 256 against 512. Under 5a.3 the 256 and 512 candidates are rejected by the verification estimate, and the case is Unresolved(Ceiling).
- **The check** (`method_tests.rs`, `directional_span_under_5a2_was_selected_at_256_and_published_within_its_claim`). It rebuilds 5a.2's decision with `stop_rule` (5a.2's rule, with no V and no (b)–(d)) on K4's states, whose gates are all coalesced, so they are 5a.2's states. q\* is GEN's exact published quantity for every layout row (`directional_span_exact.txt`): u\* from the exact rational solve of the intended model, recovered at 4,096 bits with the exact ledger, and rounded to 512 bits.
- **Result.**
  - Every row satisfies |q_256 − q\*| ≤ 2^-64·M_q, with M_q = max(|q_512|, S\*) as 5a.2's stop rule formed it. The worst ratio is 1.5e-24, at node 1's displacement magnitude.
  - The 17 rows published as `absolute_verified` satisfy |q_pub − q\*| ≤ b·(1 + 2^-22).
  - **5a.2's publication was within b. It was not a false claim.** 5a.3 withholds a correct publication, which is the availability loss ROOT accepted: the 512 verification state's solve error (κ ≈ 2^104, so about 2^-408 relative) exceeds V/4 = 2^(6−512)·ê.

### 22.4 E finite while ê overflows (ROOT's A3b ruling)

- **The case.** E encodes, but ê_mo = fl(L_b·E_fo) or ê_fo = fl(E_mo/L_b) overflows binary64. V, the estimate's threshold and the charge's allowance would then be infinite, and no precision could pass.
- **The mapping.** `verify.rs` `resolution_hats` checks ê for every body after E has encoded for every body, force before moment. An overflow is `AttemptStop::ResolutionScale { body, kind }`, so the case ends `Unresolved(ResolutionScaleUnencodable)` (F2a: `receipt_encoding`), as for E's own overflow, and is not escalated. Before the ruling it ended `Arithmetic(NonFinite)` in `decide`.
- **The tests.**
  - `an_e_hat_that_overflows_while_e_encodes_stops_as_e_does` (`verify_tests.rs`): the moment case at L_b = 4, the force case at L_b = 1/4, and the finite edge (fl(2·f64::MAX/2) = f64::MAX, and L_b = 0).
  - The control **EHAT-OVERFLOW**: the skew pin with a 2^980 moment. At the 256 verification E_fo ≈ 9.9e307 and E_mo ≈ 1.2e306 encode, and L_b·E_fo overflows. The case ends `128:rejected:verification_failed`, `256:failed:ResolutionScale`, Unresolved(ResolutionScaleUnencodable { body: 0, kind: moment }). GEN emulates the same check (`verify_em`); every other record is unchanged, and GEN's diffs are purely additive.

### 22.5 The outcomes that moved from revision 5a.2 (accepted by ROOT at A3b)

Every other control of K4's A2 set keeps its A2 selected precision under 5a.3 (`outcomes.txt` against A2's assertions; R7 §6.2's "unchanged" tags).

| Control | 5a.2 (K4 at A2) | 5a.3 (K4, GEN) | Why |
|---|---|---|---|
| DIRECTIONAL-SPAN | selected at **256** (128 rejected by the stop rule at u(0, Rx)) | Unresolved(Ceiling): 128 rejected by the stop rule; 256 and 512 by the verification estimate | Its springs span R³ only through 2^-52 (κ ≈ 2^104); the verification's solve error stays above V/4 at every p. 5a.2's publication was within b (§22.3), so this is an availability loss, and honest. DIRECTIONAL-WELL keeps the directional-spring recovery compared, at 128. (Corrected at B: I12's A3b report said "from 128".) |
| CEIL-A, CEIL-B | selected | Unresolved(ResolutionScaleUnencodable): 128 rejected (its verification failed), 256 stops with `ResolutionScale` | E overflows under their 2^1013-rad rigid rotation (A3-0 Q8). F-1's control moves to CEIL-S (P = 2^900, ε = 2^-160, §8.6). |
| RIGID-UNLOADED | Unresolved(Ceiling) (F-3) | selected at 512, honest | The floor Φ, as R7 predicts (§10). |
| EHAT-OVERFLOW (new at B) | — | Unresolved(ResolutionScaleUnencodable) | §22.4. |

The 5a.3 controls (R7 §7, `models5a3.txt`) are new at A3b; each equals R7's expectation (§11).

### 22.6 The kept-for-derivation guards

R7 §9 and the selection keep four checks with no kill at the design's precisions. In K4 each moves no control (C; ROOT: "as R7 expects"), and each is caught at evidence or unit level:

| Guard | What the mutant does | Why it is kept (R7) | Caught in K4 by |
|---|---|---|---|
| R7-M17 | drops the charge (d) | the charge is t₁ to t₃ of the Theorem, and B its norm; with per-block bounds no single-block control is refused by the charge alone | SD-G5's decision and charge-boundary tests |
| R7-M21 | rounds the prescribed values (their sum at P) in W's residual | vacuous for every W1a facade case (rigid restraints: every published prescription is 0); the Theorem's step 2 needs exact prescriptions | the golden work |
| R7-M24 | uses est in place of B | Lemmas D and E, not a kill, are why the certified bound is required; R6's kill by HH-SLENDER-m40 was an indexing artifact (ROOT: not expected to kill it) | E-CHARGE (bit level) |
| R7-M27 (design precision) | drops the shift bound's backward-error and rounding terms (σ′ = σ) | no selection or honesty kill at the design's precisions, argued (§19) | the low-precision stress (P = 10 to 32, A3-0 Q15) and E-CHARGE |

### 22.7 Differences from emu7, and why each is honest (A3-0 Q1)

GEN is the bit oracle, written to R7's text in K4's operation order; emu7 is the selection-level cross-check. At A3b GEN's selections, attempts and reasons were compared with emu7's (`run_controls7`'s column for R7's rule, and `large7_10_100`) on every control both define, and agree; class counts are compared only where the layouts agree (plan §7). The differences:

1. **Directed roundings (R7 items 6 to 12; Q2).** K4 rounds the norms, θ_c, t₁ to t₃, C_q and W⁺ upward, and 1 − t_c, σ_c and σ′_c downward, as R7 prescribes; emu7 keeps the norms, θ, t and C as exact Fractions. K4's figures are therefore never below the exact ones: a check can only be more conservative, so the difference is availability only, and no control's outcome differs.
2. **‖ā_q S‖₁ rounded upward at every stage (Q2).** emu7 evaluates it through E's nearest stages. R7 item 6 says upward; the data are nonnegative and monotone, so K4's value is a certified upper bound. This bears on honesty, and R7's text governs.
3. **est_c (Q3).** K4 rounds each block sum once and then divides (three roundings); emu7 rounds the exact quotient once. est_c only chooses σ_c, and Lemma E holds for any σ_c > 0: availability only.
4. **The verification pass runs only on verification states (Q18).** emu7 bounds every solved state, so it counts two shifted factorizations where K4 counts one (`large7`'s 2 is 1 in K4). Only verification states enter the acceptance rule.
5. **Support-group rows and node magnitudes.** emu7's layout has none. K4's form is derived in §22.1 (Q12), and GEN emulates it.
6. **E-ESTIMATE's reference (Q10).** GEN's exact rational q\* and R\*(u_P), in place of emu7's P = 2048 pipeline: a stronger reference.
7. **Terminal encodings.** An E, or an ê, that does not encode ends the case `Unresolved(ResolutionScaleUnencodable)` (Q8; §22.4); R7 and emu7 treat it at the receipt as `unavailable (receipt_encoding)`. Nothing is published either way.
8. **The evidence's extremes.** K4 finds each reported worst ratio with a binary64 prefilter (rows within 2^13 ulps of the running approximate extreme, the approximations being within 2 ulps of the exact ratios) and evaluates the kept rows exactly; the reported value is the directed rounding of the exact extreme, as emu7's Fractions give. The acceptance tests themselves are decided exactly per row.

### 22.8 Revision 5a.3's figures in K4

- **E-HEADROOM:** 322 state pairs; the worst |q_P − q_2P| is 1.55·2^-P·ê (F-3-FREE at 512), against the allowance 2^8.
- **E-CHARGE:** 360 verification states bit-equal to GEN (21 more stop before the charge); **E-ESTIMATE:** 1,828 rows against GEN's exact solution.
- **Named figures** (asserted at R7's two figures, ROOT's A3b ruling): LEVER2's W/V at 512 of 2,681, 2.7e6 and 2.8e9, with the charge above its allowance; TILT-LEVER's charge ≥ 1.0e30 and θ ≥ 6.0e10; CHARGE-SLENDER's and HH-SLENDER-m40's C/allowance 1.6e-3 at 256; THETA-STUB-COUPLED's θ = 28 and C/allowance 5.6e-12 at 256; the HH-FOOL LOADED forms with B = Uc after three failed shifts; the shift on RF-LARGE at 10 members at every verification except CONT-AX; the probe set's worst C/allowance 7.4e-49.
- **DEMOTION2:** relative rows 7 → 4, b = 4.7e-6 (R7: b ≤ 4.7e-6); **EXACT-RIGID:** b = 1.07e-150 > 0 at 512 (R7: about 1.1e-150).
- **RF-LARGE:** at 10 and 100 members every frame is selected at 128 (as R7 states), and honest row by row against GEN's high-precision expectations, with G5a, from addendum 1 (at D the lane's 1e-9 predicate only, RV19-2); RF-LARGE-100's six frames take 1 to 4 s each in the debug suite, so no `--ignored` lane was needed (A3-0 Q13).

### 22.9 The honesty predicate tightened at D (ROOT's ruling at C)

*Superseded in part by addendum 1 (RV19-2): the check now uses GEN's 128-bit exact expectations and skips nothing; the text below is D's.*

**The predicate** (`models.rs` `compare_honest`, used by the controls test and the recovery test). Each selected row must satisfy the claim it publishes, against GEN's exact solution rounded once to binary64 (e), with the binary64 publication rounding stated:
- an `absolute_verified` row with bound b: |q_pub − e| ≤ b·(1 + 2^-22) at p = 128 and 256, and b·(1 + 2^-21) at p = 512 (R7 §5.2);
- a `relative_verified` row: |q_pub − e| ≤ 2^-64·max(|q_pub|, S\*_pub)·(1 + 2^-21) + ½ ulp(q_pub), the stop rule's relative bound with R7 §5.2's allowance for S\*_pub against S\*_2p and the charge's 2^-22 at 512, plus the publication rounding;
- an `input_derived` row: ½ ulp(q_pub) (it is its exact value rounded once);
- an `unpublishable` row publishes no value and is skipped;
- the test's derived keys (N and T from the end actions; Mb and Mbs from two components) take the claims of the rows they are formed from, plus 2^-50·|value| for the test's own binary64 arithmetic;
- every comparison also allows ½ ulp(e), the expectation's own rounding to binary64.

It replaces "1e-9 relative, or b for an absolute row", which no longer passes: a relative row off by 1e-9 now fails by a factor of about 2^22 (its allowance is dominated by the two half-ulps, about 2^-52 relative).

**Unmutated: no stop.** Every selected control with an expectation satisfies it (at D, 13 selected controls had none and were skipped, as were unpublishable rows: RV19-2): 99 controls, 5,490 rows, the worst at 0.28 of its allowance (SKEW-K1E-12's end.1.i.1, a relative row); the recovery test's 43 models pass. FK's full suite passes at D (§17).

**The evidence pass re-run for R7-M1, K4-M24 and K4-M37** (from clean archives of `8f8023a20` with D's three test files overlaid; `_run_records/c/evidence_tight.jsonl`). Ratios are |q_pub − e| over the allowance:

| Mutant | Controls that move | Caught as dishonest under the tightened predicate | At C (the loose predicate) |
|---|---|---|---|
| R7-M1 (drop V) | ASSEMBLY-SAT, EXACT-RIGID, F-2, F-2-CEIL, F-2-SPOS, M10-G, PRESCRIBED-TAIL, PRESCRIBED-TAIL-FREE, PT-A, PTF-A → 128; DEMOTION2, GS-TRANS-y345, M7-GS1, MIXED-2^-200, RIGID-UNLOADED still at 512, with changed attempts or classes | ASSEMBLY-SAT (9.0e15, R.1.0), F-2 (9.0e15, N.1), F-2-CEIL (9.0e15, N.1), and **F-2-SPOS, newly caught (16.0, N.1;** R7's "ratio 16") | ASSEMBLY-SAT, F-2, F-2-CEIL |
| K4-M24 (a combination accepted at its operands' precision) | CEIL5A3 → 512 | **CEIL5A3, newly caught (670, N.3)** | a selection change only |
| K4-M37 (Φ in the stop rule, not in the classification) | F-2-CEIL, F-3-FREE, F-3-ROT, GS-ROT-y345, GS-TRANS-y345, M10-ANISO, M7-GS1, RIGID-UNLOADED (at 512, with b from the unfloored S\*) | all eight (9.0e15; N.1 for F-2-CEIL, a reaction for the others) | all eight (with an unbounded ratio) |

- **Not caught, and why.** R7 counts PRESCRIBED-TAIL false under M1, and it is: at 128 its force rows are published as +0.0 with b = 0, while their truth is 2^-1091 N (PRESCRIBED-TAIL-FREE likewise). That truth is below binary64's smallest subnormal, so the expectation file holds 0.0, and a comparison of binary64 values (|0 − 0| ≤ 0) cannot see the claim fail. Unmutated, the case is selected at 512, where the floored b (about 2^-502·ê) covers the truth. Catching the false b = 0 needs the expectation to record that the exact value is nonzero (an underflow marker), which `models5a3.txt`'s binary64 encoding does not carry. The mutant is killed regardless (the controls test fails on its selection changes). PT-A and PTF-A (the operands, whose forces are exactly zero), EXACT-RIGID and M10-G move precision but stay honest, as R7 lists them (selection changes, not false claims).
- NONE's evidence lists N02, N03-RZ and N04 (models GEN leaves unresolved with pivot failures) in every run, the unmutated one included; they are not moves.

## Addendum 1: RV19's review, the fixes

**Basis.** RV19's review of head `7d8fa9c0e` (`REVIEW/K4_REVIEW.md`, `d906539e…`): FAIL, with 1 BLOCKING, 5 SHOULD-FIX and 7 NOTEs. ROOT's binding rulings, "K4: rulings on RV19's review" (numerics `e9f8ebe1d`), which include D1 revision 5a.3 amendment A1. RV19's probes, oracle and probed fix (`REVIEW/_run_records/k4_review/`) were read, and its OVF-ROT, TINY-S and GROUP-DIR models were reused as controls.

**Status: no stop.**
- Every selected control and combination of the controls test (117), and RF-LARGE's six 100-member frames, satisfy the new checks. [Scoped at addendum 2 (RV19-DN2): `models.txt`'s combinations were not in that set. Addendum 2 checks PRECISION-RULE, B1-C and B1-E directly; CEILING is never selected.]
- No control's outcome moved. GEN's `outcomes.txt` gains only the six new controls; every earlier line is byte-identical.
- Every edit is inside K4's write set: `FK/src/structural/retained/{adaptive,combine,wide_sum}.rs`, `K4T/`, and `T3/IMPLEMENTATION/K4/`.
- I made no Git writes.

### A1.1 RV19-1 (BLOCKING): O9 on the stop rule

**The fix** (`adaptive.rs`, `rule` and `scales_at`; RV19's probed fix):
- `rule` marks every row whose candidate value is nonzero and has no binary64 value (`to_binary64()` gives `Underflow` or `Overflow`).
- `scales_at` leaves those rows out of S\*_2p, as `classify_rows_floored` leaves them out of S\*_pub. So both scales are formed from the same rows, and R7 §5.2's premise holds again: S\*_pub lies within a relative 2^-64 + 2^-52 of S\*_2p.
- GEN's `scales_at_em` does the same.

**Users of `scales_at`, and why the fix only makes the rule stricter.**
- `scales_at` has one caller, `rule`, reached through `decide` (revision 5a.3's acceptance) and `stop_rule` (5a.2's rule, a test entry). Two test sites call it directly and pass the same skip: the every-quantity control, and DIRECTIONAL-SPAN's 5a.2 check, where no row is skipped.
- In `rule` the scales feed four things:
  - M_q = max(|q_2p|, S\*) of (a);
  - the allowance 2^-86·M_q of (d) at p = 512;
  - the floor, S\* := max(S\*, Φ), at 512;
  - the summary ratios.
- Leaving rows out can only lower each S\*, because it is a maximum over fewer rows, and Φ's floor is unchanged. So every allowance of (a) and (d) is at most what it was. V, (b), `uc`, θ and g do not read S\*. A skipped row still meets (a), against its own |q_2p| and the reduced S\*.
- **So a candidate the fixed rule accepts, the old rule accepted too.** The fix can only withhold. That is availability, never honesty.
- The classification is unchanged. Rows that underflow or overflow were already out of S\*_pub.

**Evidence.**
- **OVF-ROT-928** (RV19's model; GEN and `outcomes.txt`). A (1,2,0) member of section 2^-100 with an axial load 2^928·(1,2,0): its displacements overflow binary64, while its rotation is exactly 0.
  - It is now rejected by the stop rule at 128, 256 and 512, and ends `Unresolved(Ceiling)`. GEN agrees.
  - Before the fix, K4 selected it at 128 and published Rz = −2^901 as `relative_verified` (RV19).
- **OVF-ROT-900** beside it, where nothing overflows, is selected at 128 and is honest.
- **No other control moved.** GEN's regenerated `outcomes.txt` equals D's line for line, plus the six new controls. The controls test is token-equal to GEN.
- **The underflow side** (RV19 §1.4, the PRESCRIBED-TAIL pattern) is closed by the same skip, as RV19's probe of the fix found: a nonzero candidate value that underflows no longer sets S\*.
- **The reverted fix, RV19-1R, is killed** by the controls test at OVF-ROT-928 (§A1.8).

### A1.2 RV19-6: D1 revision 5a.3 amendment A1

**The rule.** Where 0 < S\* < 2^-988, each `absolute_verified` row's bound is

  b_row = fl↑(fl↑(2^-64·S\*) + fl↑(2^-53·|q_pub|) + 2^-1074).

It is unchanged where S\* ≥ 2^-988, and b = 0 is unchanged at S\* = 0.

**The derivation** (ROOT's ruling, restated):
- |q_pub − q\*| ≤ |q_pub − q_p| + |q_p − q\*|.
- Rounding to nearest gives |q_pub − q_p| ≤ 2^-53·|q_pub| + 2^-1075:
  - ≤ 2^-53·|q_pub| for a normal result, since half an ulp of the result is at most 2^-53 of it;
  - ≤ 2^-1075 for a subnormal one;
  - a nonzero q_p that rounds to zero is `Unpublishable` and carries no bound.
- The accepted candidate gives |q_p − q\*| ≤ 2^-64·S\*, within the factor R7 §5.2 already carries (1 + 2^-22, or 1 + 2^-21 at 512). That factor covers S\*_pub against S\*_2p, which RV19-1's fix makes true again, and the charge's 2^-22 at 512.
- fl↑(2^-64·S\*) ≥ 2^-64·S\*, fl↑(2^-53·|q|) ≥ 2^-53·|q|, and 2^-1074 > 2^-1075. So |q_pub − q\*| ≤ b_row·(1 + 2^-22), or (1 + 2^-21) at 512, with the upward rounding of the sum only adding to b_row.
- Where S\* ≥ 2^-988, an absolute row has |q| < 2^-34·S\*, whose rounding is below 2^-23·b (R7 §5.2), so the plain b holds. Relative rows cannot occur below 2^-988, and above it they are normal numbers.

**Checked against the code** (`adaptive.rs` `row_bound`, called by `classify`):
- `absolute_bound(S*)` is fl↑(2^-64·S\*), decided exactly as before.
- fl↑(2^-53·|q|) is `q / 2^53`, then one step up when the scaling back by 2^53 falls short of |q|. That scaling back is exact (only the division can round, into the subnormal range), so the comparison decides.
- The three terms are summed in one `ExactWideSum` and rounded upward once to binary64 by `directed_ratio(…, Up)` with denominator 1. That is the single fl↑ of the exact sum. Its span is at most about 2,100 bits, so no refusal is reachable, and the unreachable fallback is +∞, never a low bound.
- `classify` receives the classification's S\* (coupled, and floored by Φ at 512) and the published binary64 q.
- GEN's `row_bound` computes the same with Fractions and `fl_up`. `classification.txt` gains four targeted sets, and Rust matches every set bit for bit:
  - TINY-S-995's scale, with a subnormal and a zero row;
  - S\* just below 2^-988, where 2^-53·|q| is normal;
  - S\* at 2^-988 exactly, where the plain rule applies;
  - a coupled case.
- The unit test `amendment_a1_gives_each_row_its_publication_rounding_below_2_to_the_minus_988` pins five values:
  - q = S\* = 2^-995 gives `0x4008001`;
  - a zero row gives `0x8001`;
  - just below 2^-988 gives `0x200400001`;
  - at 2^-988 and at S\* = 0 the plain rule applies.

**Bound bits that change** (every selected control of the controls test compared with fl↑(2^-64·S\*)):
- PT-B, 26 rows: an operand prescribing 2^-1000;
- PTF-B, 45 rows: likewise;
- TINY-S-995, 24 rows.

No other control's bound moves. In `classification.txt`, five rows of the seeded sets change (each in a body and kind whose S\* lies below 2^-988), and so do the four targeted rows below 2^-988 (sets 402 and 403). `item_6a_floors_force_and_moment_by_phi…`'s zero row under a subnormal Φ now carries 2·2^-1074 instead of 2^-1074.

**TINY-S-995** (RV19's model, GEN and `outcomes.txt`) is selected at 128:
- Every row of its body is `absolute_verified` below S\* = 2^-988.
- Under A1 every row is within its claim against GEN's exact solution.
- Under the reverted amendment, **A1R**, the controls test fails on TINY-S-995: its worst row, end.1.i.2, is 819.2 times the allowance. RV19 measured 222 to 819 on seven rows (§A1.8).

TINY-S-900 beside it is selected at 128, with the plain rule.

**Routed:**
- D2's G5 checks b_row (D2's input list);
- RV19 checks the derivation and the code at its delta check;
- ROOT adopts the amendment into the design text.

### A1.3 RV19-2: the honesty check sees every row

**Exact expectations** (GEN). Every `expect` line of `models.txt` and `models5a3.txt` now carries, after the binary64 of the exact value (unchanged on every earlier line):
- `x:±<m>p<e>`: the exact value rounded once to 128 bits;
- `underflow` or `overflow`: ROOT's range marker, for an exact value outside binary64's range. The sign is the token's;
- `err:<e>`: for a model with irrational lengths or axes, which has no exact rational solution, a bound |x − q\*| ≤ 2^e.

**`solve_hp`.** It solves the intended model in decimal at 300 significant digits:
- every binary64 input is lifted exactly;
- each member is T^T k T with the Gram–Schmidt axes;
- symmetric elimination runs in RCM order, and recovery is as `solve_exact`'s.

It solves again at 240 digits, and err = 4·|x₃₀₀ − x₂₄₀|. Across the ROT frames, the other irrational models and OVF-ROT, err lies at least 398 bits below the model's largest value (RF-LARGE-TREE-n00010-ROT), and 600 to 800 bits elsewhere.

**Which models use it.** The 13 selected controls without expectations at D:
- N03-RX, the four HH-FOOL forms, HH-SLENDER-m40 and R115-SEED3;
- RF-LARGE's six 10-member frames (the three AX frames solve exactly);
- also RF-LARGE's three 100-member ROT frames and OVF-ROT.

None of the 13 is left without expectations. The models with none are:
- the mechanisms N02, N03-RZ and N04;
- `models.txt`'s combination PRECISION-RULE, which has no net case;
- the three TILT-LEVER controls, which are withheld (a pivot of the decimal solve falls below its singular test).

None of them is ever selected.

**`compare_honest`** (`models.rs`) checks each published value q against its claim a with the exact x:
- max(0, |q − x| − δ) ≤ a, decided exactly, with δ = 2^-127·|x| + 2^err;
- an `absolute_verified` row's a is b·(1 + 2^-22 or 2^-21), with no ½ ulp of an expectation.

**Nothing is skipped:**
- a published value with no expectation fails;
- an unpublishable row passes only if its exact value's range agrees:
  - `Underflow`: |x| − δ ≤ 2^-1075, an exact zero included;
  - `Overflow`: a truth of its sign that may reach 2^1024 − 2^970;
- an expectation that is neither a published row nor a derived key fails.

The controls test asserts that every selected control and combination it runs (every model and 5a.3 combination of the default lane) has expectations, and that the number checked equals its rows plus its derived keys. The recovery test asserts that every selected model has expectations.

**Results:**
- **The controls test:** 117 selected controls and combinations (RF-LARGE at 100 members excepted), 9,412 checks. The worst is 0.949 of an allowance, at B1-C-A u.0.3, the row and figure RV19's independent oracle reported.
- **RF-LARGE at 100 members** (their references tests, `honest_large`): G5a passes, and all six are honest, 3,013 to 3,163 checks each. The worst is 0.97, CONT-ROT's R.1.1.
- **The recovery test:** every models.txt model it selects, N03-RX now among them.
- **The 33 `Underflow` rows** RV19 counted (SPRING-CARRIED, GS-ROT-y345, M10-ANISO) pass their range checks.

**Statements corrected.** RETURN's status lines, §11's controls item, §12.6, §22.8 and §22.9, and CHANGE_RECORD, now say what the tests check, with D's narrower coverage stated where it was D's.

**The evidence pass under the new checks** (`_run_records/rv19/mutations/rv19_evidence.py`, `evidence.jsonl`). Each run takes K4's C-campaign edits on a clean copy of the fixed tree, and adds a print-only insertion to the controls test. That insertion lists every control whose outcome differs from GEN's, and every selected control that `compare_honest` rejects.

| Mutant | Caught as dishonest now | At D (binary64 expectations, skips) |
|---|---|---|
| R7-M1 (drop V) | ASSEMBLY-SAT (9.0e15, R.1.0), F-2 (∞, N.1), F-2-CEIL (∞, N.1), F-2-SPOS (16.0, N.1), **PRESCRIBED-TAIL (∞, N.1) and PRESCRIBED-TAIL-FREE (∞, N.1), now caught** | the first four (F-2 and F-2-CEIL at 9.0e15); both PRESCRIBED-TAIL forms missed |
| K4-M24 | CEIL5A3 (670, N.3) | the same |
| K4-M37 (Φ in the stop rule, not the classification) | D's eight (F-2-CEIL now ∞ at N.1; F-3-FREE, F-3-ROT, GS-ROT-y345, GS-TRANS-y345, M10-ANISO, M7-GS1 and RIGID-UNLOADED 9.0e15 at a reaction), **and PRESCRIBED-TAIL and PRESCRIBED-TAIL-FREE (∞, N.1) and R115-SEED3 (9.0e15, N.3), newly caught**: the tails through their exact values, R115-SEED3 because it had no expectation at D | eight (9.0e15) |
| RV19-1R (the reverted fix) | OVF-ROT-928, selected at 128 against GEN's withholding, with u(1, Rz) outside its `relative_verified` claim (9.0e15, u.1.5): RV19's false publication | — |

- **PRESCRIBED-TAIL and PRESCRIBED-TAIL-FREE** are the answer to ROOT's question at D. Their force rows publish +0.0 with b = 0 at 128. GEN's expectation now carries the exact value, 2^-1091 N, with its `underflow` marker, so the check sees 0 ≠ q\* with no allowance (ratio ∞).
- F-2 and F-2-CEIL now report ∞ where D's check gave 9.0e15. They publish N = 0 with b = 0 against a nonzero truth, and the allowance no longer includes half an ulp of a binary64 expectation.
- As at D, R7-M1 moves EXACT-RIGID, M10-G, PT-A and PTF-A without a false claim (selection changes), and DEMOTION2, GS-TRANS-y345, M7-GS1, MIXED-2^-200 and RIGID-UNLOADED keep 512 with other rejections. NONE lists only N02, N03-RZ and N04 (K4's geometry refusals, which GEN does not model).

### A1.4 RV19-3: combination operands with different stations or groups

`RetainedCombination::solve` now also requires every operand's stations and support groups to equal the first operand's (`combine.rs`); otherwise it returns `OperandsDiffer`. K4STF and the layout hold only a station's and a group's ids.

The test `operands_whose_stations_or_support_groups_differ_are_withheld` covers three cases:
- RV19's cantilever pair, station 1 at t = 0.25 and at t = 0.75: withheld;
- group 1 reused with a different spring set: withheld;
- equal operands combine: st.1.4 = 1.5, twice 0.75.

### A1.5 RV19-4: a support group holding directional springs

GROUP-DIR and GROUP-DIR-X (RV19's models) join GEN, `outcomes.txt`, E-UNIT (`scale.txt`), E-UC (`bounds.txt`), E-CHARGE (`charge.txt`) and E-ESTIMATE (`estimate.txt`):
- DIRECTIONAL-WELL with group 1 at the root, restraining its translations and holding the three rotational directional springs;
- group 2 at the tip, holding the translational one;
- X adds loads at the root's rotations.

Both are selected at 128 and honest. RV19-M6 (a group's E without its directional contributors) is killed (§A1.8).

### A1.6 RV19-5: ‖SĀS‖ as the larger of the two norms

`the_sas_norm_is_the_larger_of_the_one_and_infinity_norms` raises, one at a time, each off-diagonal free–free entry of N05's Ā at 256 by 2^20 in its transposed position. A column sum then exceeds every row sum on some entries. For every block it asserts that ‖SĀS‖ is the larger of the two norms, and it requires at least one case where the 1-norm is the larger. RV19-M2 (the ∞-norm only) is killed by it (§A1.8).

### A1.7 The NOTEs

- **N1** (§22.1 item 1): the count is ≤ 68·2^-P·E_group (69 with the coarser 2.5), not 20g + 49. Item 5's "to first order" is exact. Corrected in place.
- **N2** (§9 steps 11 and 12): route 2's uncertified step is step 9, not step 6; ρ is ‖Ā‖/‖|K|_contrib‖. Corrected in place.
- **N4** (`wide_sum.rs`): the release-mode `break` past the buffer now returns `SumRefusal::Span` whenever a bit or a carry remains. It is unreachable under the span check, and every caller ends its attempt on `Span`.
- **N6:** every `2f64.powi(n)` in K4's tests is gone:
  - `models.rs` uses `f64::from_bits` constants;
  - the rest use `support::pow2`, which builds 2^e from its bits.

  The two remaining `powi` strings in `adaptive_tests.rs` are the source scan's own list and its lexer control.
- **N3:** partly met. The claims are now checked at the expectations' 128-bit resolution, not binary64's. K4-M33 stays bit-exact.
- **N5** (a magnitude-row SD-G5 vector) and **N7** are recorded and not added.

### A1.8 Re-runs, mutants and records

Host rules as before: rustc 1.97.1, `--offline --locked`, `-j 4`, `RUST_TEST_THREADS=2`, at most two cargo jobs, and the memory guard (no kill).

- **K4's suite:** 125 of 125, the N5 streams included (443 s at 2 threads). The controls are token-equal to GEN, the six new controls among them.
- **FK's full suite** on the final tree: 392 passed, 0 failed: D's 389 plus the three new K4 tests, with the lib's 326 in 695.4 s at 2 threads. The run used `--nocapture`, so the controls test's claims line and RF-LARGE-100's lines are in the log. It repeats K4's suite on the final bytes.
- **The S11 site table:** 3 of 3.
- **rustfmt:** K4's files are clean.
- **`gen_k4_vectors.py --check`:** 23 of 23 OK (the 22 vector files and SHA256SUMS), 16 min 56 s, with FK's suite running beside it. The pinned inputs' sha256 hold.

**Mutants** (`rv19_mut.py`). Each runs from a clean copy of the candidate: `git archive 7d8fa9c0e` of FK with the 25 changed files copied over it, a fresh target, and exact edits with their counts. Every `structural::retained` K4 test runs except the N5 streams and the sum differentials, 114 tests.

| Mutant | Edit | Result | Killed by |
|---|---|---|---|
| NONE | the candidate | passes 114 of 114 | — |
| RV19-1R | the RV19-1 fix reverted (every row sets the stop rule's S\*) | killed | the controls test: OVF-ROT-928 selected against GEN (in the evidence pass, its Rz is 9.0e15 times outside its claim) |
| A1R | amendment A1 reverted (b = fl↑(2^-64·S\*) below 2^-988 too) | killed | the controls test: TINY-S-995, 819.2 times the allowance at end.1.i.2; the A1 unit test; the classification vectors (set 19); item 6a's test |
| RV19-M2 | ‖SĀS‖ = the ∞-norm only | killed | `the_sas_norm_is_the_larger_of_the_one_and_infinity_norms` |
| RV19-M6 | a support group's E without its directional contributors | killed | E-UNIT and E-CHARGE, on GROUP-DIR's group rows (the selections do not move) |

**The evidence pass** re-ran R7-M1, K4-M24 and K4-M37 (and RV19-1R, and NONE) under the new checks (§A1.3).

**Records** (`_run_records/rv19/`), with `_run_records/SHA256SUMS` regenerated over the whole folder:
- `gen/`: the four partial regenerations and the full `--check`;
- `tests/`:
  - K4's suite (run before `rustfmt` reformatted some test files, whitespace only; the first mutant copies predate it too);
  - FK's full suite on the final tree;
  - the S11 site table;
- `mutations/`: the harness, the evidence script, `results.jsonl`, `evidence.jsonl`, each run's logs, and each evidence run's controls log;
- `changed_files.txt`: every file changed against `7d8fa9c0e`, with its sha256.

## Addendum 2: RV19's delta check at `a5fa0eaf7`

**Basis.** RV19's delta check of `a5fa0eaf7` (`REVIEW/K4_REVIEW.md`, "Delta check at a5fa0eaf7") PASSES with 0 BLOCKING, 1 SHOULD-FIX and 4 NOTEs. ROOT's rulings on it (numerics `83909efd4`) ask for two items before merge: RV19-D4 and DN2.

**Status: no stop.**
- No control moves, and no expectation other than PRECISION-RULE's changes.
- Every edit is inside K4's write set. I made no Git writes.

### A2.1 RV19-D4: the RV19-1 skip is keyed on the candidate

- **The gap.** No test pinned which value the skip reads. RV19's mutant keys it on the verification value instead, and it survived 114 tests. The two keys differ only when a row's p and 2p values straddle binary64's overflow or underflow threshold.
- **The new vector.** `sd_g5_a_row_the_candidate_cannot_publish_sets_no_s_star` (`method_tests.rs`, beside SD-G5's) runs `decide` at P = 256 on two displacement rows. Row 0 straddles the threshold. Row 1's allowance, 2^-64·max(|q_2p|, S\*), shows whether row 0 set S\*.

  | Case | Row 0: candidate / verification | Row 1 | Verdict |
  |---|---|---|---|
  | Overflow | 2^1024 − 2^970 (the threshold: a tie that rounds to 2^1024, `Overflow`) / 2^1024 − 2^970 − 2^950 (rounds to f64::MAX) | Uy; verification 1, \|Δ\| = 2^-40 | Row 0 sets no S\*, so S\* = 1 and row 1 is rejected (`StopRule { index: 1 }`) |
  | Overflow, swapped | the two values exchanged | the same | Row 0 sets S\* ≈ 2^1024, and the case is accepted |
  | Underflow | 2^-1075 (a tie that rounds to zero, `Underflow`) / 2^-1075 + 2^-1200 (rounds to 2^-1074) | Rx through S\*(rot) = max(rot, tr/L_b), with L_b = 2^-60; verification 2^-1020, \|Δ\| = 2^-1080 | Row 0 sets no S\*(tr), so S\*(rot) = 2^-1020 and row 1 is rejected |
  | Underflow, swapped | the two values exchanged | the same | Row 0 sets S\*(tr) = 2^-1075, so S\*(rot) = 2^-1015 and the case is accepted |

  Keyed on the verification value, each of the four verdicts would turn over, since each pair straddles the threshold the other way. The mutant run stops at the first. The test also asserts each value's binary64 outcome.
- **The mutant.** RV19-D4 (RV19's edit, verbatim) ran from a clean copy: `git archive a5fa0eaf7` of FK, with the files this addendum changes copied over it, a fresh target and the counted edit. The result is **killed**: only the new vector fails, at its first case (the overflowing candidate is accepted), while the other 115 tests pass. The NONE control passes 116 of 116 (the 114 of addendum 1 and the two new tests).

### A2.2 DN2: PRECISION-RULE and the wording

- **GEN.** `models.txt`'s PRECISION-RULE (SKEW-K1E-28-AXIAL + SKEW-K1E-28), its one combination without a net case, now carries expectations: the combination solved as its own case (`combined_model`), exactly, with 128-bit tokens. No other line of `models.txt` changes.
- **The tests** (`combine_tests.rs`):
  - `precision_rule_is_selected_at_256_and_honest` combines the two operands. It asserts selection at 256, G5a, and `compare_honest` with every row checked.
  - `check_combo`, behind B1-C and B1-E, now also runs `compare_honest` and G5a. So every `models.txt` combination K4 selects is checked directly. CEILING's operands are withheld under 5a.3, so it is never selected.
- **The NOTEs DN1, DN3 and DN4** are recorded, as ruled; they affect no check.
- **The wording.** RETURN's addendum-1 status line ("every selected control and combination …", RV19's line 761) is scoped to what is tested, with the scope of addendum 1's check stated alongside it.

### A2.3 Re-runs and records

Host rules as before: `-j 4`, `RUST_TEST_THREADS=2`, at most two cargo jobs, and the memory guard (no kill).

- **FK's full suite** on the final tree, with `--nocapture`: 394 passed, 0 failed: addendum 1's 392 plus the two new tests, with the lib's 328 in 691.4 s at 2 threads. The controls test's claims line is unchanged: 117 controls, 9,412 checks, worst 0.949. It includes K4's suite, now 127 tests (the D4 vector and the PRECISION-RULE test added), with the controls token-equal to GEN.
- **`gen_k4_vectors.py --check`:** 23 of 23 OK (the 22 vector files, including the regenerated `models.txt`, and SHA256SUMS), 14 min 43 s, with FK's suite running beside it for 11 min 37 s of that. The pinned inputs' sha256 hold.
- **rustfmt:** clean on K4's files.

**Records.** `_run_records/rv19d/` holds:
- the mutant harness and results, with each run's logs;
- the GEN check;
- FK's full suite;
- the changed files with their sha256.

`_run_records/SHA256SUMS` is regenerated over the whole folder.
