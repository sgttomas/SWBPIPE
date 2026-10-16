# RF source-method extension: exact blocks of order up to 4, exact-congruence member transforms and metered work

**Status: design, not implemented.** No product code, test, fixture, schema, table, generator or reference has changed. The basis is main `dff1ff2a086ad681a8174b46b4d31833327452dd` (B7b, merged at `64c22d5510`, plus the post-T3 records).

This is the second post-T3 shared epoch in [b7-plan.md](b7-plan.md) ("After B7b", item 2). It is implemented after T4-U6/U8 merges, from merged main, under its own independent review. Section 10 lists what to re-check against U6/U8 at that point.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Commission** (b7-plan, B7b table, HELP_HUMAN 2026-10-12): "Extend the retained-source exact method's coverage under its own design, taken before any code: exact blocks of order up to 4 and non-axis-aligned member transforms first; a subnormal final reserve only with a sound outward-rounded recipe and underflow/overflow controls. Work and memory are accounted positively and priced. D1's limits (exact-boundary limits, `PHYSICS_SOURCE_WORK_LIMIT`, 256 DOFs) stay unchanged unless separately ruled. The J7 RF census is its consuming check: every newly selected case must meet the frozen criterion, and no reference or tolerance changes."

## Summary

- **Expected new coverage** (measured on a scratch prototype): 32 of the 161 in-family cases become directly selected, and each meets the frozen criterion on every reference row.
  - 28 are RF-CANCEL cases, whose blocks have order 4 (section 2). Their ordinary route passes its checks in both modes, so the public route attempts no recovery. Like RF-ZERO-TINY, they are method-only selections, and their public envelopes do not change.
  - RF-SKEW-T-PIN-OFF-345-r1e-04/-08/-12 and RF-MECH-LINE345-RX-COMPANION have non-axis-aligned members, carried by the exact congruence (section 3).
  - RF-SKEW-T-PIN-OFF-345-r1e-08 and -r1e-12 are sensitive on the ordinary route in both modes, so their four public envelopes newly publish `pressure-source-1`. This is the first public RF selected-source coverage, and the census binds it (section 6).
- **What stays out.** Every other case keeps a typed first cause:
  - an exact block of order above 4 (89 cases);
  - the 4 subnormal-reserve cases;
  - the 29 HASH-001 refusals;
  - the 6 ordinary axis-gate refusals.

  No previously selected case loses selection, and no first cause changes except as section 6 states. This was measured on the census's single-case invocations. Section 5.2 states the cross-attempt exception, which fails closed.
- **Two rulings applied** (HELP_HUMAN, 2026-10-16):
  - the subnormal final reserve is excluded (section 4);
  - metered charging becomes the single work-charging rule for exact attempts (section 5). Existing work counters move and are re-pinned.
  - Under that rule, one committed source case also changes outcome for work alone: `load_reference_fallback_uz` now passes the replay screen it used to fail, and publishes selected. The implementation keeps that fixture's role, or re-states its consumers (section 5.5).
  - No strict-gap outcome changes in any committed suite (measured).
- **No change** to a reference, criterion, tolerance, expected case, exclusion, D1 limit, semantic table or model format.
- **D3** is estimated at about 8 MB more in the phases that price the live source attempt and its receipt replay (derived, section 9). That fits if U6/U8's registration leaves at least about 10 MB in each such phase.
- **No ruling is outstanding.** Section 11 lists what stays reserved.

## 1. The census facts

### 1.1 How they were measured

Three throwaway probes ran in the scratch worktree `/Users/ryan/dev/chirality-t3/scratch/rf-design-probe`, a detached checkout of `dff1ff2a08`. Every build ran through `tools/t3_cargo.sh` with `CARGO_TARGET_DIR=/Users/ryan/dev/chirality-t3/targets/rf-design-probe`, from `projects/chirality-piping/core/product_physics` or `core/solver/frame_kernel`. Every Python step ran through `tools/t3_slot.sh python3 -I`. The evidence lives outside the repository and may not survive a reboot.

- **P1, operands and exact analysis** (`/private/tmp/rf-design-probe/`).
  - A scratch test appended to `j7_rf_census.rs`, run as `RF_DESIGN_PROBE_OUT=/private/tmp/rf-design-probe/operands.jsonl test --lib rf_design_probe_dump`, dumps the census's own `Owned::actual` operands for all 161 in-family cases. It records the partition, springs and identified loads, and per member the binary64 `local_stiffness()`, `transformation_matrix()` and `global_stiffness()`. It also records the output of a mirror of `transform_roundoff` that keeps every final reserve.
  - An independent Python analysis (`rf_probe/analyze.py`; standard library and exact rationals only, no producer code) rebuilds the exact free-DOF graph, classifies the transforms and solves every case exactly.
  - Outputs: `operands.jsonl`, `analysis.json`, `tables.txt` and `mirror_check.txt`.
- **P2, prototype** (`/private/tmp/rf-design-proto/`, source diff `prototype.diff`).
  - A scratch implementation of sections 2 and 3, under the legacy charges, runs the census's direct method on all 161 cases (`RF_PROTO_OUT=/private/tmp/rf-design-proto/measure.jsonl test --lib rf_proto_measure`).
  - Section 3.6 describes the shortcut it takes for the Context's congruence source.
  - Outputs: `measure.jsonl`, `summary.json`, `census_diff.jsonl` and the logs.
- **P3, metered charging.** P2's prototype with metered `Work::mul` (section 5), run twice:
  - `/private/tmp/rf-design-proto/metered/`: for "extended" attempts only (`RF_PROTO_METERED=1`), with a per-pair constant of 4. This was an intermediate step, superseded by the ruling.
  - `/private/tmp/rf-design-proto/uniform/`: the ruled rule. It applies to every exact attempt with the per-pair constant 96 (`RF_PROTO_UNIFORM=1`). The order cap is scoped per entry (section 2.1), and a scratch counter records operations and radix iterations.
  - The uniform run covers the census measurement (`measure.jsonl`, `compare.json`, `summary.json`), the leaf diff of the 24 source controls (`controls_leafdiff.json`), strict-gap endings under both rules (`strictgap_*.json`) and these suites under both rules: frame kernel lib, nonlinear lib and integration, product-physics lib, and the product-physics integration targets `kd5_preview_maximum_enclosure`, `physics_source_runtime`, `pressure_source_join`, `s11f_site_test` and `source_block_recovery` (`suite_*.log`).
  - The extended-only run's source is `metered/prototype.diff`. The uniform run's source was saved from the scratch tree after the run, as `uniform/prototype.diff` (tracked files) and `uniform/prototype-untracked.tar` (the probe scripts).
  - The "baseline" logs are the same scratch tree run with main's behaviour. The `result_export` suite was not run, because it reads committed fixtures, which no probe changes.
  - Every measured run ended normally. The last job's rc=101 (10:13:53Z) is its own test failures, classified in section 5.5; it was not an interruption.

Controls that passed (measured):
- **P1.**
  - The live first cause equals the pin in 161/161 cases.
  - The mirror of `transform_roundoff` agrees with the original on Ok/Err for 535/535 members, and on every reserve bit of the 518 accepted members. RF-RANGE-CHAIN-E-1000's row 1/column 1 reserve is `0002e5d8f51c523b`, as b7-plan states.
  - The independently rebuilt graph gives the pinned first order above 2 in 94/94 cases.
  - RF-ZERO-TINY passes 25/25 rows, and all 72 of its exactly computed quantities, rounded once, equal the product's dumped values bit for bit.
  - Every exact solve has zero residual on every free row.
- **P2, under the legacy charges.**
  - Orders 1 and 2 are byte-preserved: `tests/pressure_source_join.rs`'s `join_preserves_every_source_publication_byte` and `pressure_source_controls_keep_exact_work_and_current_bytes` pass unchanged.
  - RF-ZERO-TINY still charges 742,625 with an identical response digest.
  - The 52 block cases whose first component is above 4 keep identical errors and work.
  - `source_recovery::tests` pass 10/10.
- **P2 and P3, for every selected case.** Every published displacement and member end action contains P1's exact value inside its certified interval, and lies within 2 ulps of the correctly rounded value.
- **Mode independence.** `dense_scrutiny` spot checks (six in P2, three in P3) equal `sparse_interactive` in outcome, work and response digest.
- **P3 (uniform).**
  - The frame-kernel lib passes 611/611, with the legacy-entry tests unchanged.
  - The nonlinear lib (151) and its integration tests (4) pass identically under both rules.
  - No multiplication anywhere in the census or its replays is charged below its counted operations (section 5.4).

### 1.2 The 94 exact-block cases

The census pins only the *first* free component above order 2, in BFS order over the free DOFs (`exact_boundary.rs`), not the largest. P1 rebuilt every component (measured):

| Distribution over the 94 cases | Orders |
|---|---|
| First order above 2 (what the census pins) | 3:1, 4:41, 6:17, 7:4, 8:8, 10:8, 11:10, 12:1, 24:3, 60:1 |
| Largest order | 4:28, 6:17, 7:3, 8:8, 10:11, 12:5, 15:2, 20:16, 24:3, 60:1 |

| Family | Component orders, in BFS order |
|---|---|
| RF-CHAIN-T/A n03 | [4,3,6,6] or [4,6,6,3] |
| RF-CHAIN-T / -A n05 | [6,5,10,10] / [6,10,10,5] |
| RF-CHAIN-T / -A n10 | [11,10,20,20] / [11,20,20,10] |
| RF-WEAK-W-AX / -W-L / -W-3D | [4,8,8,4] / [7,6] / [24] |
| RF-LARGE axis-aligned | CHAIN [10,20,20,10]; TREE [60]; CONT [10,15,15,1,1,1,1,1] |
| RF-INVARIANCE | PINTOR [6,6,6]; LFRAME [8,7], relabelled [7,8]; CONT4 [8,12,12,1,1,1,1], relabelled first 12 |
| RF-RANGE-*-F-960 | as their bases |
| RF-ZERO-SYM / -PLANAR | [3,6,6,3] / [6,6] |
| RF-FINITE | [10,20,20,10] |
| RF-CANCEL | [2,4,4,2] |

An order cap of 4 admits exactly the 28 cases whose every component has order at most 4. All of them are RF-CANCEL:
- RF-CANCEL-{F,M}-G1e{5,6,7,8}-{GnG,GGn,nGG} (24 cases);
- RF-CANCEL-{F,M}-G1e8-GnG-{ORTHO,INPLANE} (4 cases).

The other 66 keep the typed first cause `UnsupportedBlock`. In 14 of them the reported order changes, because the first component above 4 is now reported:
- RF-CHAIN-T/A-n03 (10 cases): 4 → 6;
- RF-WEAK-W-AX (3 cases): 4 → 8;
- RF-ZERO-SYM: 3 → 6.

### 1.3 The 27 transform cases

Every non-identity transform is blockdiag(R,R,R,R) with a single 3×3 R, and no R is exactly orthogonal (measured, 70 members):

| Member direction | Members | Used by | Rows of exactly unit norm | det R − 1 | max \|RRᵀ − I\| |
|---|---|---|---|---|---|
| (3,4,0) | 10 | SKEW-*-OFF-345 (9), MECH-LINE345-RX-COMPANION | the (0,0,1) row only | +1.78e-16 | 1.78e-16 |
| (1,2,2) | 12 | SKEW-*-OFF-122 (9), ZERO-TORSION, RANGE-SKEW-F-960, RANGE-SKEW-E-1000 | none | −2.72e-16 | 2.56e-16 |
| (1,2,−2) | 33 | LARGE-*-n00010-ROT (25), INVARIANCE-CONT4-ROT-Q3 (8) | none | −2.72e-16 | 2.56e-16 |
| (2,1,2), (2,−2,−1) | 3, 2 | LARGE-TREE-n00010-ROT branches | none | −2.72e-16 | 2.56e-16 |
| (1,8,−4) | 9 | INVARIANCE-CONT4-ROT-Q9 (8), FINITE-ROT-Q9-NONMULT's first member | none | −1.07e-16 | 3.0e-16 |
| a non-small rational direction | 1 | FINITE-ROT-Q9-NONMULT's second member (one entry is 2⁻⁵⁵ where the exact value is 0) | none | +1.84e-16 | 1.97e-16 |

Two exact stiffness sources were compared for these members (measured):
- **R1, the rounded global contributions.** These are today's `global_stiffness()` entries, which the ordinary route assembles. R1 is *exactly asymmetric* in 12 cases: all nine OFF-345 cases and MECH-LINE345 (4 entry pairs each), INVARIANCE-CONT4-ROT-Q9 (18 pairs) and FINITE-ROT-Q9-NONMULT (25 pairs). The Context's exact symmetry check would refuse them before the block search (`Invalid("exact source asymmetry")`).
- **R2, the exact congruence** Σ scatter(Tᵀ K_local T), formed exactly from the binary64 bits of T and K_local. R2 is exactly symmetric in every case.

R1 and R2 give identical block profiles in all 27 cases. Only four of the 27 have every component of order at most 4:
- RF-SKEW-T-PIN-OFF-345-r1e-04/-08/-12, profile [4,2];
- RF-MECH-LINE345-RX-COMPANION, profile [3,2].

These four are the only cases that need both extensions: a block of order 3 or 4, and a congruence transform. Every other transform case has a component above order 4:

| Cases | Component orders |
|---|---|
| SKEW-*-CANT-OFF-345 | [5,4] |
| SKEW-*-CANT-OFF-122 | [9] |
| SKEW-T-PIN-OFF-122 | [6] |
| ZERO-TORSION | [6] |
| RANGE-SKEW-F-960 | [9] |
| FINITE-ROT-Q9-NONMULT | [12] |
| INVARIANCE-CONT4-ROT | [36] |
| LARGE-CHAIN-ROT, LARGE-TREE-ROT | [60] |
| LARGE-CONT-ROT | [45] |

### 1.4 The four range cases

The ruling in section 4 settles these: they stay out with their first cause, `checked_value_range`. Section 4 records their facts.

### 1.5 Does an exact solve meet the frozen criterion?

P1 decided every reference row exactly with the census's own predicate: `|obs − exp| ≤ 1e-9·max(|exp|, scale)`, with the RF-CANCEL row scales and the mm and N·mm exponents, and `magnitude_predicate` for bending magnitudes. It applied the predicate to the exact rational solution and to its binary64 rounding, and the two agree on every row (measured).

- **Every case the extension selects passes.**
  - All 28 RF-CANCEL cases pass every row (38/38), and so does RF-ZERO-TINY (25/25).
  - Under R2, the four transform cases pass: PIN-OFF-345 28/28 at all three spring ratios (worst 5e-7 of the tolerance) and MECH-LINE345 26/26.
  - Under R1, PIN-OFF-345-r1e-08 and -r1e-12 fail: 16/28, worst 131 and 1.3e6 times the tolerance, at R.N0.UZ.
- **Eight cases fail under every block or transform limit, and all are outside an order-4 cap**: RF-WEAK-W-L-r1e-08/-12 ([7,6]), RF-SKEW-T-CANT-OFF-345-r1e-08/-12 ([5,4]), RF-SKEW-T-CANT-OFF-122-r1e-08/-12 ([9]), RF-RANGE-SKEW-F-960 and RF-RANGE-SKEW-E-1000 ([9]).
  - **Cause.** In the product's binary64 K_local, the coefficients 12EI/L³, 6EI/L², 4EI/L and 2EI/L are each rounded separately. That leaves every member with a nonzero exact rigid-rotation residual, between 1.02e-17 and 1.05e-16 of its stiffness per member, across all 126 analysable cases. A soft spring with k/a ≈ 1e-8 or 1e-12 magnifies the residual by about a/k.
  - **Diagnostic.** A source R3 forms K_local exactly from the same binary64 E, G, A, I, J and L, and keeps the binary64 T. It passes all 126 cases, so the transform's non-orthogonality is not the cause.
  - **Consequence for later work.** The method certifies the exact solution of its declared binary64 source and cannot know the reference. An extension above order 4 would therefore select cases that miss the frozen criterion, unless it declines them or changes local formation. That would be a later unit under its own design; nothing here depends on it.

### 1.6 Out of scope

These keep their first causes; none makes a source attempt:
- the 29 HASH-001 cases (unsafe integral float, refused before solve);
- the 6 ordinary axis-gate cases (`PIPE_ELEMENT_INPUT_INVALID` at model build);
- the 55 cases outside D1's source family, which stay outside.

## 2. Exact blocks of order up to 4

### 2.1 Where the cap applies

`exact_boundary::Context` has two producers:
- the source method, through the functional entry `Context::prepare_with_budget` and its replay `RetainedFunctionalSet::replay_against`;
- the nonlinear strict-gap qualification, through the legacy `Context::new` (`nonlinear_integration/src/structural_adapter.rs`) and the legacy `RetainedResponse::replay_against`.

The order cap becomes a private Context parameter, set by the entry: **4 on the functional entry and its replay, 2 on the legacy entries.** A replayed Context uses the cap of the entry that produced it. The legacy `RetainedResponse::replay_against` keeps its `dofs.len() > 2` witness check (`exact_boundary.rs`, "retained minor size"). The strict-gap qualification's block scope therefore does not widen, and these tests stay unchanged:
- nonlinear `strict_gap_unsupported_blocks_and_mixed_families_do_not_gain_qualification`;
- frame-kernel `unsupported_singular_negative_asymmetric_range_and_budget_are_explicit` and `original_exact_graph_does_not_use_absorbed_stored_coupling`.

Section 5's charging rule does reach the strict-gap path.

### 2.2 Block search and refusal

The block search, its BFS order and its exact zero test are unchanged. Blocks are still processed in the existing single loop, so blocks that precede the first refused component are still prepared, as today. The first component whose order exceeds the entry's cap is refused with the existing `Error::UnsupportedBlock { order }`. Orders 1 and 2 keep their code and witness fields exactly.

### 2.3 The positive-definiteness warrant

Take a block of order o ∈ {3, 4}, with a[p][q] = k[block[p]][block[q]] as exact expansions. The existing exact symmetry check guarantees a[p][q] = a[q][p].
- The existing first check, `sign(a00) == Positive`, stays.
- Δ2 = a00·a11 − a01², Δ3 (the leading 3×3 determinant) and, for o = 4, Δ4 = det A must each be exactly positive; otherwise the result is `Error::NotPositiveDefinite`.

All leading principal minors positive is Sylvester's criterion, which is necessary and sufficient for a symmetric matrix to be positive definite. A positive det A alone is not enough: diag(1, −1, −1, 1) has a positive determinant. Exact signs come from the expansions' last terms, as for order 2.

### 2.4 The exact arithmetic

All arithmetic uses the existing `Expansion` operations through `Work`. There is no division.
- **Minors.** Cache the 2×2 minors M(r1,r2; c1,c2) = a[r1][c1]·a[r2][c2] − a[r1][c2]·a[r2][c1]. Read every entry from the upper triangle, so that a minor and its transpose are one computation. That gives 6 distinct minors for o = 3 and 18 for o = 4 (measured on the prototype).
- **Δ3** is the first-row cofactor expansion over cached minors of its other two rows.
- **Δ4** is the Laplace expansion by rows {0, 1}: the sum over column pairs c < d of (−1)^(1+c+d)·M(0,1; c,d)·M(2,3; complement of {c,d}, ascending).
- **Adjugate.** For p ≤ q, adj[p][q] = (−1)^(p+q) times the minor of A that deletes row q and column p, and adj[q][p] = adj[p][q].
  - For o = 3 that minor is a cached 2×2 minor.
  - For o = 4 it is a 3×3 determinant, expanded along its first remaining row over cached minors, with adj[3][3] = Δ3.
  - The cofactor sign is applied as the sign of the summed product, with no separate negation.
- **Solve.** Let h be the block's existing right-hand side (f minus the prescribed couplings). Then num_p = Σ_q adj[p][q]·h_q, and u[block[p]] = num_p / Δo.
  - Every DOF of the block takes a clone of the **same** Δo expansion as its denominator.
  - **The shared denominator is a correctness precondition.** `functionals::evaluate` combines a block's numerators before any ratio sum, and it takes `displacement[block[0]].denominator` for the whole block without checking (`functionals.rs`, `evaluate`). A block whose DOFs carried different denominators would publish wrong values, not merely cost more work.
  - So for every block of order ≥ 3, `evaluate` checks that all the block's denominators carry bit-identical terms, charging the comparison, and refuses with `Invalid` otherwise. Blocks of order ≤ 2 keep today's path.
- **Where it runs.** The minors, Δ3, Δ4 and the adjugate are computed once while the Context is prepared (stage Preparation) and kept privately in the Context. Solve forms h and adj·h. This costs about 1 MB per Context at the D1 caps (section 9), and avoids recomputing the cofactors in Solve and again in every replay.
- **Reactions.** `solve_charged` forms r_i = −f_i + Σ_j k_ij u_j with `ratio_add`, which multiplies denominators. For a block of order 3 or 4, that would raise its denominator to the power of the block order.
  - Columns j in a block of order ≤ 2 keep the existing per-DOF loop, in the same order.
  - For each block of order ≥ 3 that row i couples to, the numerator Σ_{j∈block} k_ij·num_j is accumulated exactly. After the existing loop, one `ratio_add(r, Ratio{numerator, Δo})` per such block follows, in block order.
  - The existing check that every original free row's reaction is exactly zero stays, and it is not vacuous: an error in the adjugate or in its signs fails it.

### 2.5 Certified bounds and enclosures

Nothing new is certified approximately. The displacements and reactions are exact ratios, and the positive-definiteness warrant is exact. Publication uses the existing `project` and functional projection unchanged:
- an exact-identity shortcut when it applies;
- otherwise, the outward interval over all four signed quotient corners of the enclosed numerator and denominator, with relative limit 1e-9 (`RELATIVE_LIMIT`).

On the 28 selectable RF-CANCEL cases, the largest certified relative bound is 6.59e-16 (measured).

### 2.6 Witness and replay

`BlockWitness` keeps `dofs` and `a` (a00's terms) for every order, and keeps `b`, `c` and `determinant` unchanged for o ≤ 2. Two optional fields are added and used only for o ≥ 3:
- `upper`: the terms of the o(o+1)/2 upper-triangle entries, row-major;
- `minors`: the terms of Δ2 … Δo.

Both fields enter `same_bits` and `scalar_count` only when present, so order-1 and order-2 witnesses and their replay comparisons are unchanged.

The retained-witness size checks (`functionals::retained_charge` and the functional replay) admit `dofs.len() ≤ 4`. The size bound is (o(o+1)/2 + o)·E + o for o ≥ 3, where E is `expansion_terms`, and the existing 4E + 2 for o ≤ 2. Replay recomputes the Context, so a changed retained minor or entry is refused as "functional retained block/minor mismatch" (`RetainedFunctionalSet::replay_against`).

### 2.7 Work and expansion lengths

Every exact operation is charged through `Work` under section 5's rule. Witness construction precharges `block.len()` plus the witness's scalar lengths, as order 2 does.

Measured on the prototype for the 28 selectable RF-CANCEL cases:
- **Under the legacy charges:** selection costs 3,417,588 to 6,377,841, and Evaluation is the largest stage. Selection plus the receipt's same-ledger functional replay costs 6,835,938 to 12,756,933, and only 8 of the 28 fit within 8,000,000.
- **Under section 5's rule (P3, uniform):** selection costs 370,674 to 538,389, and selection plus replay 742,110 to 1,078,029. For example, CANCEL-F-G1e5-GnG selects at 429,395 and replays within 859,737.
- **Expansion lengths:** no expansion-length budget is reached. The longest expansion in any case is 35 terms (an `add`) against `expansion_terms` = 256.

### 2.8 Limits

These are unchanged:
- 256 DOFs (`DENSE_SOURCE_DOF_LIMIT`, `Limits::dofs`);
- 16,384 source terms and 16,384 descriptor units;
- 256 expansion terms;
- `PHYSICS_SOURCE_WORK_LIMIT` = 8,000,000 per case, and 64,000,000 per invocation.

The order cap rises from 2 to 4 on the functional entry only. That is the commissioned extension.

### 2.9 Publication and the receipt replay

**No new rule.** The publication paths stay as they are:
- On the 0.4 route, the existing replay screen (`SelectedSourceRecovery::reserve_captured_replay`: charged ≤ limit − charged) runs before selection, and a join that still cannot finalize republishes ordinarily (ROOT CP3 SF-1R).
- On the pre-0.4 route, a selected case is replayed on its own ledger before publication. If finalization fails, the envelope gets the pre-0.4 path's named refusal, the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`. DEL-04-07 `pressure-contract.md` records "the pre-0.4 path retains its named refusal", and `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` pins it.

Extending the screen to the pre-0.4 route would change that recorded behaviour. No measured selection needs it:
- under section 5's rule, the largest new selection is RF-SKEW-T-PIN-OFF-345-r1e-08, at 2,832,853, and 5,667,343 with its functional replay (P3);
- every new selection fits within 8,000,000 including replay.

What still has to be established at implementation: that the finalization reservations of the two new public publications (section 6) fit in the remaining margin of about 2.3 million. Those reservations are 24E + F + P (b7-plan, D5/MJ9), where E is the encoded request's bytes, a few tens of kilobytes for these one-member requests (derived). If they did not fit, the pre-0.4 named refusal would apply and the census binding in section 6 would fail. That would be an implementation finding, not a reason to loosen anything.

## 3. Non-axis-aligned member transforms

### 3.1 What the source is

The certificate concerns "the declared binary64 element/load operands" (`source_recovery.rs`).
- For a member whose transform is a signed permutation, the declared local stiffness and transform give global entries ±k exactly, so the rounded global contributions *are* the declared source.
- For any other transform, the ordinary route's global entries fl(Tᵀ fl(K T)) are rounded, so they are not the declared source.

**The exact source of a non-axis-aligned member is the exact congruence Tᵀ K_local T,** formed exactly from the member's declared binary64 K_local (`local_stiffness()`) and T (`orientation().transformation_matrix()`). This is R2 in section 1.

The rounded global contributions cannot be the source:
- they are exactly asymmetric for (3,4,0) and (1,8,−4) members;
- they fail the frozen criterion where R2 meets it (section 1.5).

The certified response is the exact solution of the equations formed exactly from the declared local matrices, transforms, springs and loads. It is not the exact solution of the ordinary route's rounded aggregate, and it says nothing about primitive geometry. Section 7 makes the receipt state this.

### 3.2 Source closure in `prepare_sources`

A member whose transform passes `signed_permutation` keeps today's path and closure exactly. Any other member must pass three checks:
1. **Structure.** T must be blockdiag(R,R,R,R): the four 3×3 diagonal blocks are bit-identical, and every off-block entry is +0.0 or −0.0. Otherwise the decline is "actual transform is not a block rotation". The message still contains "transform", so `failure_fields` keeps the receipt code `unsupported_source_closure`.
2. **Formation determinism.** The member's 144 assembly contributions must equal its `global_stiffness()` bits at its DOF map; otherwise `SourceMismatch`. This ties the ordinary aggregate to the same declared operands.
3. **Unit precheck.** The member's descriptor units, counted from its structure as `descriptors_charge` counts them, are added to a running total before any descriptor is built. If the total would exceed 16,384, the decline is `Budget` (section 3.5).

These stay unchanged:
- the operand identity checks (frame and pipe K_local and T bit-equal);
- the "complete ordered stiffness fold" check against the actual aggregate;
- the identity record, which already contains the bits of K_local and T.

### 3.3 The Context's source

The functional entry takes one more argument: the list of congruence members, each with its DOF map, K_local, T and the index range of its 144 rounded contributions. The legacy entries take none.
- **Coverage.** The rounded contributions stay in `StructuralSystem::contributions`. The existing coverage check against the aggregate passes through its ordered-fold alternative, which is how the aggregate was formed.
- **The exact entries k.** For a congruence member, k uses the exact entries G_ij = Σ_k Σ_l T[k][i]·K_local[k][l]·T[l][j] instead of its rounded contributions. Zero factors are skipped, and the entries are formed in the two-stage order Tᵀ·(K·T), which P3 measured as value-equal to the direct triple sum in every member and cheaper. Other contributions and springs enter k as today.
- **Checks.** The existing exact symmetry check, the block search and the positive-definiteness warrant apply to this k.
- **Replay and retained copies.** The Snapshot stores the congruence operands (K_local and T: 288 values per member, plus the map and contribution range), `matches` compares them bit for bit, and replay re-forms them.
  - `Snapshot::copy_charge`, `functionals::retained_charge` and `source_preflight` charge and size-check those operands, before any copy, as they do contributions.
  - The functional replay (`RetainedFunctionalSet::replay_against`) and `check_binding_with_budget` take the expected congruence list, and `SelectedSourceRecovery::replay_against` and `check_binding_against` rebuild it from the current invocation through `prepare_sources`.

### 3.4 Recovery rows

- **Member end actions** of a congruence member: rows[r] = Σ_p K_local[r][p]·(T·u_e)_p. Each row's terms are the products [K_local[r][p], T[p][j]] for every nonzero pair, merged per global DOF into one `AffineTerm` with several products. Section rows follow from these rows unchanged, through the existing [sign, fraction, length] products.
- **Rigid-support action rows.** For a congruence member they use its exact entries, as triple products [T[k][i], K_local[k][l], T[l][j]], in place of its rounded contributions. Signed-permutation members and springs are as today.

A member's exact global end action is Tᵀ·rows: the 12 local end actions, rotated back exactly by its declared T. Exact equilibrium holds at every node between the members' exact global end actions, the support actions and the loads, because Tᵀ·(K_local·T·u_e) is the member's exact congruence contribution. It holds today for signed permutations in the same way.

### 3.5 Work and units

The congruence formation happens inside the Context (section 3.3). It is therefore charged on the attempt's ledger in stage **Preparation**, under section 5's rule; the prototype charged it in SourceClosure, so its stage attribution differs from the design's.

Measured per member under the uniform rule (P3):

| Member direction | Two-stage Tᵀ·(K·T), as specified | Direct triple sum |
|---|---|---|
| (3,4,0) | 29,416 | 33,608 |
| (1,2,2) or (1,2,−2) | 84,268 | 102,076 |
| (1,8,−4) | | about 102,400 |

The two forms gave value-equal entries in every member. The same members charge 924,176 and about 2.6 million under the legacy charges, with the direct sum. The census totals in 3.6 were measured with the direct sum, so they carry about 4,000 to 18,000 per member more than the specified form.

Descriptor units grow, derived from `descriptors_charge`'s counting rule and K_local's sparsity:
- A congruence member's end rows carry up to 12 terms, each with a two-atom product, where today they carry 2 to 4 single-atom terms. Its section rows add the [sign, fraction, length] atoms to those terms.
- That comes to about 2,900 units per member, against about 820 today.
- Each rigid support the member touches adds about 800 to 950 units. Each of its 6 support rows has at most 12 terms, each with at most 3 triple products of 3 atoms (1 + 3·4 = 13 units), so at most about 160 units per row (derived; the independent review counted about 800 per support).
- The unchanged 16,384-unit law therefore admits roughly four or five congruence members per model.
- Section 3.2's unit precheck declines a larger model by `Budget`, before descriptor #1 is built, so its memory stays within the unit law (section 9).

This is a coverage limit of the unchanged D1 limits, not a new rule.

### 3.6 The prototype's shortcut, and the measured outcome

The prototype implemented 3.3 with a shortcut. It replaced each congruence member's rounded contributions by the exact terms of its G_ij, and bypassed the coverage check only on the entries those members touch. That was 8 to 10 entries per single-member case and 34 to 258 per multi-member case; no other entry was bypassed. All 27 transform cases passed both closure checks of 3.2.

The shortcut does not build a receipt, so the public publication path for a congruence member was not exercised. The implementation verifies it (sections 6 and 12).

Measured at the unchanged 8,000,000:

| Cases | Legacy charges (P2) | Section 5's rule (P3, uniform) |
|---|---|---|
| SKEW-T-PIN-OFF-345 r1e-04/08/12 | `Budget` in Preparation; 30.9M / 33.4M / 32.0M needed | selected [4,2], 28/28. Selection 2,571,226 / 2,832,853 / 2,708,316; with replay 5,143,867 / 5,667,343 / 5,418,108 |
| MECH-LINE345-RX-COMPANION | `Budget` in Evaluation; 15.3M needed | selected [3,2], 26/26. Selection 1,167,411; with replay 2,336,114 |
| LARGE-CHAIN/TREE/CONT-n00010-ROT | `Budget` in SourceClosure (about 27.0M needed) | `UnsupportedBlock` 60 / 60 / 45 (about 1.75M) |
| INVARIANCE-CONT4-ROT-Q3/Q9 | `Budget` in SourceClosure (about 21.6M needed) | `UnsupportedBlock` 36 (about 1.41M) |
| the other 18 | `UnsupportedBlock` 5, 6, 9 or 12 | the same errors, at lower work (for example ZERO-TORSION 172,089, FINITE-ROT-Q9-NONMULT 365,141) |

Under the uniform rule no transform case ends in `Budget`. All 23 that do not select carry `UnsupportedBlock` with an order above 4.

## 4. The subnormal final reserve: excluded (HELP_HUMAN ruling, 2026-10-16)

**Ruling.** HELP_HUMAN, 2026-10-16: "the subnormal final reserve is excluded from the RF source-method extension."
- The four range cases keep their typed first cause, `checked_value_range`.
- No reference, criterion or guard changes; `structural.rs::checked_value` and `transform_roundoff` are untouched.
- RF-RANGE-THIN-B, which would need a sound reserve recipe *and* new power-of-two-scaled exact arithmetic, may become a later unit under its own design. It is not scheduled.

**Basis** (P1, measured on `dff1ff2a08`; evidence in `/private/tmp/rf-design-probe/`):
- **A sound reserve recipe alone selects none of the four.** Each block's exact products underflow below 2⁻¹⁰⁷⁴. The smallest are 2⁻¹⁹⁶⁰ for RANGE-CHAIN-E-1000, RANGE-CONT-E-1000 and RANGE-SKEW-E-1000; RANGE-THIN-B's 2×2 determinant terms are 2⁻²⁰³⁵. `Expansion::add_product` and `exact_radix` refuse any product that loses bits (derived from the code).
- **RANGE-CHAIN-E-1000 and RANGE-CONT-E-1000** also have blocks of order 10 and 15 (profiles [6,5,10,10] and [10,15,15,1,1,1,1,1]).
- **RANGE-SKEW-E-1000** has one (1,2,2) member and an order-9 block, and its exact solve misses the frozen criterion (explained below).
- **RANGE-THIN-B** has one identity member and profile [1,2,2,1]. It is the only case that could become selectable: its exact solve passes 25/25, but it needs both changes named in the ruling.
- **The reserve has no consuming meaning on the source path today.** `AssemblyEvidence::new` computes it for every frame (`transform_roundoff`; 36 subnormal, 4 normal and 104 zero reserves per identity member in these cases, except RANGE-CHAIN-E-1000's member M2, with 28 subnormal and 12 normal), and the source path then discards it: the exact Context receives no symmetry evidence.

**Why RANGE-SKEW-E-1000 misses the criterion** (recorded for a later unit).
- **The case.** It is RF-SKEW-T-CANT-OFF-122-r1e-08 with E and G scaled by 2⁻¹⁰⁰⁰, with a soft root torsional spring at k/a ≈ 1e-8.
- **R2's disagreements.** The exact solve of its declared source (R2) disagrees with the reference on 9 of 31 rows:
  - th.N0.RX and th.N1.RX, at 172 times the tolerance;
  - u.N1.UY and u.N1.UZ, at 122 times;
  - the three spring actions S.N0.*, at 57 and 14 times;
  - Mb.M1.i and Mb.M1.mid, at 61 times.
- **R1** disagrees on 12 rows, at up to 1.17e4 times the tolerance.
- **R3 passes.** R3, with K_local formed exactly from the same binary64 E, G, A, I, J and L and with the binary64 T, passes 31/31; its worst row is at 3.9e-7 of the tolerance.
- **The cause.** The disagreement comes from the separately rounded local coefficients. They leave a rigid-rotation residual of 6.5e-17 relative, and the soft spring magnifies it about 10⁸ times. It does not come from the subnormal range or from T.
- **The ordinary-range twin.** RANGE-SKEW-F-960, the same geometry at ordinary range, fails in the same way.

## 5. Work accounting: metered charging as the single rule (HELP_HUMAN ruling, 2026-10-16)

**Ruling.** HELP_HUMAN, 2026-10-16:
- **Not today's charges.** Keeping today's charges "would ship the commissioned transform support with zero census coverage". The `Work::mul` precharge "is sized for 256-term expansions when the longest expansion reached is 35 terms. That is a stale estimate, not a sound requirement."
- **One rule.** "Adopt metered charging, as one rule … apply it to every exact attempt, not only 'extended' ones. Two charging regimes keyed on an 'extended' flag would be a permanent seam." Existing work counters "move deliberately and are re-pinned".
- **Limits unchanged.** `PHYSICS_SOURCE_WORK_LIMIT`, the exact-boundary limits, the 256-term refusal and the 256-DOF limit all stay unchanged.
- **Soundness from structure.** "Soundness has to rest on structure, not on the census." The census result is a maintained control, an undercharge mutant is killed by a named test, and the independent review checks the argument.
- **Strict-gap outcomes.** Strict-gap qualifications that the rule newly allows are "capability gained under the same sound rule". They are listed with evidence, and DEL-04-01 is named.
- **Independent replay.** The independent replay of the rule is a test-side oracle, not the readers: "the readers check the ledger's structure and limits, and they don't redo the solve."

**Basis:** P2 and P3 (section 1.1), measured in `/Users/ryan/dev/chirality-t3/scratch/rf-design-probe` at `dff1ff2a08`, with evidence in `/private/tmp/rf-design-proto/`, `…/metered/` and `…/uniform/`.

### 5.1 The rule

`Work::mul(a, b)` processes each term pair (x, y) of its product loop, in order:
1. it refuses with `Budget` if `out.len() + 2 > E` (the existing expansion-length refusal; E = `expansion_terms` = 256). A pair therefore proceeds only while len(out) ≤ E − 2;
2. it charges **C₂ = 96**, which covers the two-product, including its exponent reads and radix scalings (5.3);
3. it runs `Expansion::add_product`'s existing sequence unchanged and unreordered, with one charge inserted immediately before each of its two `add` calls, each after that part's radix scaling: 6·len(out) + 1 before adding the scaled lo, and 6·len(out) + 1 again before adding the scaled hi. Here len(out) is the output length at the moment of each charge.

Because nothing is reordered, values, lengths and the error raised by a `Range` failure are exactly those of today's `add_product`.

The existing up-front precharge, pairs·(12E + 128), is removed. `Work::add`, `sum`, `scalar` and every explicit `w.charge` are unchanged.

The rule applies wherever `exact_boundary`'s `Work` multiplies: both Context entries, every `Response` and `RetainedResponse` method, the functional set, and so the source method and the strict-gap qualification alike.

### 5.2 Soundness

The argument rests on structure. The census is its control.
1. **Charge before execution.** Every primitive step is charged before it executes.
   - The per-pair constant is charged before the two-product.
   - Each grow-expansion add is charged before it runs, by the same formula `Work::add` uses: 6·len + 1, that is len two-sum steps of 6 arithmetic operations each, plus the final term (5.3 states the unit). The formula is evaluated on the actual current length.
   - `Work::charge` refuses before any work when the next total would pass the limit.
2. **The constant covers the two-product.** See 5.3.
3. **Executed work is bounded by the limit.** Each charge is at least the operations of the step it precedes, and a refused charge's step never executes. So the operations executed in an attempt never exceed its charged total, and the charged total never exceeds the limit.
4. **Refusal is typed and publishes nothing partial.** A refusal records its amount in `rejected` (D6's checked total) and returns `Error::Budget`. The partial `out` is a local and is dropped. No `Ratio`, `Response` or projection is built from it. The attempt ends in the existing typed `RecoveryFailure` (or strict-gap `Unsupported`), and the case publishes its ordinary result as today.
5. **Never above the legacy charge, step by step.**
   - *Per pair.* A pair proceeds only while len(out) ≤ E − 2 (5.1, step 1). Its first add is therefore charged at length at most E − 2 and its second at most E − 1, so a metered pair costs at most C₂ + (6(E − 2) + 1) + (6(E − 1) + 1) = 12E + 80 for C₂ = 96. The legacy charge is 12E + 128 per pair, for every E. With the coarser bound used as the headline, **2·(6·256 + 1) + 96 = 3,170 ≤ 3,200 = 12·256 + 128**.
   - *Same arithmetic.* Every other charge is unchanged, and the arithmetic, its values and its lengths are identical under both rules (5.1). So at every point of an attempt's execution, its metered running total is at most the legacy running total for the same executed prefix.
   - *Conclusion within one attempt.* Under a fixed limit, an attempt that completes under the legacy charges completes under the metered rule, with a total at most its legacy total. A `Budget` refusal can only disappear, never appear.
   - *Not a claim about totals.* An attempt that the legacy rule refuses can, under the metered rule, run further and charge *more* than the legacy rule charged before refusing. The per-attempt conclusion does not imply that every metered total is at most every legacy total.
6. **The cross-attempt exception.** Several attempts can share one ledger:
   - the source budget of an invocation (`SourceRecoveryBudget::case_limit`: min(8,000,000, 64,000,000 − charged));
   - the strict-gap run budget (`remaining_run_work`, debited by each call's charge).

   A strict-gap iteration that newly qualifies also changes the inputs of later iterations. So an earlier case that now runs further, or newly selects or qualifies, can leave a later case a smaller limit. In a heavy multi-case invocation, a previously selected later case could then lose selection.

   That outcome **fails closed**: a typed `Budget` decline and an ordinary publication, with no reference, criterion or limit touched. No committed suite exhibits it. The census runs single-case invocations, and the strict-gap endings are identical under both rules (5.5). Section 6's statement that no previously selected case loses selection is measured on those single-case invocations.

### 5.3 The per-pair constant

**The cost model (declared).** The unit is one floating-point arithmetic operation: +, −, ×, a fused multiply-add, and `2.0_f64.powi(k)` counted as one. Comparisons, `clamp` and integer exponent arithmetic are not counted. This is `Work::add`'s existing unit: its 6 per two-sum step counts the step's six arithmetic operations (`structural.rs`, `Expansion::add`) and none of its four comparisons. The implementation's operation counter and its oracle (5.4) both use this model.

**The count.** `Expansion::add_product(a, b, 0)` performs, per pair (`Work::mul` always passes exponent 0):
- two `binary_exponent` reads: integer bit operations, 0;
- four `exact_radix` scalings: of a by −ea and b by −eb, at most 3 iterations each, since |ea|, |eb| ≤ 1074 (subnormals included) and each iteration moves at most 512; and of lo and hi by ea + eb, at most 5 iterations each, since |ea + eb| ≤ 2148. That is at most 16 iterations, each with two `powi` and two multiplications (4), so at most 64;
- the product and its fused residual: 2.

The worst case is 66. C₂ = 96 covers it, with a margin of 30.

A stricter model also fits: P3's counter charged 6 per two-sum step, 2 per exponent read, 5 per radix iteration and 2 for the product, which gives a worst case of 86 ≤ 96. The monotonicity of 5.2 (5) allows any C₂ ≤ 126 (≤ 144 with the per-pair bound), so the choice of model cannot break it.

**Measured.** P3 recorded each pair's actual radix iterations over the 232,225 metered pairs of the census attempts and their replays:
- the largest count is 6 iterations, against the bound of 16;
- no pair needs more than the constant covers;
- under the stricter model, the smallest per-pair slack (charged minus counted) is 62, so under the declared model it is at least that.

### 5.4 The control and the oracle

- **The maintained control.** An operation counter in the frame kernel counts every executed two-sum step, two-product operation and radix iteration, under 5.3's model. A census-wide test asserts charged ≥ counted for every exact attempt and every replay.
  - The counter sits behind a dev-only cargo feature of the frame kernel (for example `exact-operation-count`), which `core/product_physics`'s and the frame kernel's test builds enable through their dev-dependencies. It adds a thread-local counter and no field to any priced type, and the registered build does not enable it.
  - A plain `#[cfg(test)]` counter would not work: it is not compiled when product_physics's tests run.
  - P3 measured, over all 161 cases and their replays, 34,423,472 charged against 17,406,962 counted operations in metered multiplications, under the stricter model. The smallest slack is 72 per call and 62 per pair, and no call was charged less than it executed.
- **The independent oracle.** A frame-kernel test reimplements grow-expansion and the two-product on binary64, including zero elimination, independently of the producer's `Expansion`. With that implementation it replays each product's actual output-length sequence, recomputes the charge of each `mul`, `add` and `sum`, and compares the totals exactly with the producer's `WorkReport`, in MJ9's style and without the producer's counters.
  - The cases cover varied operand lengths, cancellations that shorten expansions, the expansion-length refusal, and a limit set between two steps.
  - Operand lengths alone do not determine the charge, because zero elimination depends on the values. That is why the oracle replays the arithmetic.
- **The readers.** The Rust reader and the Python verifier keep their work checks, which are the ledger's structure and limits: per case, charged plus reserved at most the limit at most 8,000,000; the invocation sums at most 64,000,000 and consistent with the publication charge. They do not redo the solve, and they cannot recompute arithmetic charges.

### 5.5 Consequences

All measured on P3's uniform run, against main's behaviour in the same scratch tree.

**Census.**
- **Categories:** 32 newly selected; RF-ZERO-TINY selected as before; 14 order reclassifications and 23 transform reclassifications (section 6); 56 with an unchanged error; and 35 that make no attempt (HASH-001 and the axis gate).
- **Exceptions: none.** On these single-case invocations, no previously selected case loses selection. No first cause changes beyond those classes. Every non-selected case's error is unchanged apart from its work numbers. Section 5.2 (6) covers multi-case invocations.
- **RF-ZERO-TINY** now charges 126,845, with 253,699 including replay, against 742,625 today. Its response digest and rows (25/25) are unchanged.

**Strict-gap qualification.** The strict-gap path ends identically under both rules: 1,943 exact attempts in the nonlinear suites and 40 in the product-physics lib.
- Neither rule ends any attempt in `Budget`.
- The two strict-gap replays that a test drives into `Budget` do so under both rules.
- Nothing is newly qualified, so no DEL-04-01 K-D5 outcome changes (`kd5-nonlinear-active-set.md`).
- The rule does widen what the strict-gap helper can complete within its 2,000,000 per-call limit, for models that today exhaust it.

**Suites.**
- **Frame kernel lib:** 611/611 pass.
- **Nonlinear:** lib 151 and integration 4 pass, identically under both rules.
- **Product-physics lib:** 20 failures, each explained:
  - the 10 census family tests: coverage and work;
  - the 2 registration law tests (`retained_memory::law_tests`): any priced-source change stales the registered profile, and they fail in main-behaviour mode of the same tree too. They are D3's re-registration (section 9);
  - `cached_source_context_preserves_current_public_serializations`: work digits;
  - `retained_facade_tests::b3b_rv123_s2_unavailable_exact_case_binds_def_e`: its W1 route-E successor carries a source decline whose first cause becomes `UnsupportedBlock` instead of "not a signed permutation". This is section 3's reclassification, together with the hashes that bind it, in both modes;
  - five `source_receipt::load_state_fallback_tests`, and `s11g_tests::t13_committed_fallback_uz_is_byte_identical`: below.
- **Product-physics integration:**
  - `pressure_source_join`'s two control tests: work counters, in the 6 leaves below;
  - `s11f_site_test`'s `rule_6_kernel_functions_that_touch_the_force_are_the_listed_sites`: a source-audit list of frame-kernel functions, which the prototype changed by renaming one. The implementation keeps that list current.

**Outcomes that change for work alone.** Both cases below lose a `Budget` refusal under the lower charges, as 5.2 (5) predicts.
- **The five fallback tests** (`source_receipt/load_state_fallback_tests.rs`). Each constructs a "budget cliff": extra loads or copied cases at the public 8,000,000 and 64,000,000 limits, or a private `SourceRecoveryBudget` limit such as 7,000,000. Those cliffs were tuned to the legacy charges. Under the uniform rule the cliff is not reached and the selected join finalizes (`pressure-source-1` in place of the expected `pressure-1`). The SF-1 and SF-1R fallback paths themselves are unchanged.
  - **Re-derivation.** Each scenario's limits or sizes are recomputed from the uniform charges, so that the same path is reached.
  - A scenario that uses a private limit sets a new private limit.
  - A scenario defined at a public limit is first scaled: more extra loads or copies, within D1's caps.
  - If the cliff cannot be reached within those caps (the uniform charges are about 5 to 8 times lower), the scenario is re-stated with a private limit. It is then paired with a positive control showing that the public limit now admits the case.
  - The implementing PR lists every such change to what a test establishes.
- **The committed `core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-*.raw.json`** (0.4 load-reference route, both modes).
  - Today its source attempt is declined at the replay screen ("captured replay reservation", `Budget`, charged 4,061,151 against the 4,000,000 threshold), and it publishes its ordinary result.
  - Under the uniform rule it passes the screen and publishes the selected resolved `pressure-source-1`. Its retained maximum then takes the retained-source basis (`retained_section_functionals_binary64` in place of `j_side_section_equilibrium_binary64_enclosures`). The published values would be the source method's certified projections (relative bound ≤ 1e-9), the existing 0.4 boundary keeps the evidence numerically ineligible, and no reference covers the case.
  - **The fixture's role.** It is the committed *sensitive ordinary pressure-1* raw of a 0.4 case, and several consumers depend on that role:
    - T13 (`s11g_tests.rs`), which concerns the K-D5 ordinary maximum declaration on a Sensitive case;
    - `result_export/tests/reader_verdict.rs` (route `"pressure"`), `base_reader_outcomes.rs`, `tests/base_reader_corpus.py`, `fixtures/results/base_reader_outcomes.json`, `fixtures/results/loadReferenceReaderCases.cases.json` and `load_reference_source_contract.rs` (`lr-fallback`);
    - the desktop's `loadReference.resultsSessionState.test.ts` ("sensitive ordinary pressure-1 … never Current"), `LoadReferenceStatesBlock.test.tsx` and `baseReaderInputs.ts`;
    - `test_ci_numerical.py`.
  - **Repair.** The implementation keeps that role with a sensitive 0.4 request whose source attempt declines for a reason independent of work (a named source-family decline). T13's preconditions must still hold: Sensitive, and R-b at end i. A committed request cannot lower its case limit through the public producer, so a lower limit is not an option.
  - Only if no such request preserves T13's preconditions does the fixture become the selected publication, with every consumer above re-stated in the implementing PR.

**Not run under the uniform rule.** These suites were not run, so their evidence is still to come at implementation:
- `result_export`, the Python verifiers, the desktop and headless;
- six product-physics integration targets that touch source recovery: `exact_admission_seam`, `formation_check_runtime`, `f1b_w2_runtime`, `load_reference_state_runtime`, `load_reference_state_runtime_extension` and `retained_precision_admission`.

**Committed bytes.** Each of the 24 raw source controls differs in exactly 6 leaves, out of 2,948 to 5,067: 3 work counters, 2 hashes binding them and 1 decline text embedding work (`controls_leafdiff.json`). Their 30 derived documents and J2's bases follow from them. The retained corpus `retained_precision_cases.json` is expected to be unchanged (derived, not run): its two decline messages are source-closure refusals (the excluded subnormal reserve), which perform no multiplication, so their work counters do not move. Its pack step regenerates it at implementation, and any byte difference must be explained.

## 6. The consuming check: the J7 RF census

The census (`source_recovery/j7_rf_census.rs`, pinned in `fixtures/results/j7_rf_census_expected.json`) stays the consuming check. No reference, criterion, tolerance, expected case or exclusion changes:
- The frozen references (`validation/references/t3_r1/`, SHA-256 `7b176dbb…`), the 216-case classification, the 161-case family and the 55 exclusions stay byte-identical. So does the predicate, `|obs − exp| ≤ 1e-9·max(|exp|, scale)` with `magnitude_predicate`.
- The assertion that every directly selected case passes every reference row (`method_comparison`, `assert_eq!(p3, n3)`) stays, and now binds every newly selected case.

**First-cause classes.** Every case that does not select keeps a typed first cause. The classifier's classes become:
- `exact_block_order_above_4`: `UnsupportedBlock` with order above 4;
- `transform_not_block_rotation`: the decline in section 3.2 (0 cases, measured);
- `budget`: `Exact(Budget)`, at any stage (0 cases, measured);
- `checked_value_range`, `hash001_unsafe_integral_float`, `ordinary_axis_gate` and `selected_method_only`, unchanged.

Any other decline still panics as unclassified. `exact_block_order_above_2` and `transform_not_signed_permutation` disappear, because no case can reach them any more.

**Pinned counts.** `j7_rf_census_classification_and_pin` pins the new counts, measured on P3 (161 in total):

| Class | Cases |
|---|---|
| `selected_method_only` | 31: the 28 RF-CANCEL, RF-SKEW-T-PIN-OFF-345-r1e-04, RF-MECH-LINE345-RX-COMPANION, RF-ZERO-TINY |
| `selected_published` (new: selected, and published as `pressure-source-1`) | 2: RF-SKEW-T-PIN-OFF-345-r1e-08 and -r1e-12 |
| `exact_block_order_above_4` | 89: the 66 block cases that stay out and the 23 transform cases that stay out |
| `checked_value_range` | 4 |
| `hash001_unsafe_integral_float` | 29 |
| `ordinary_axis_gate` | 6 |
| `budget`, `transform_not_block_rotation` | 0 |

The selected/declined split and every decline class are measured (P3, `compare.json`). The split of the 33 selections into `selected_method_only` and `selected_published` is derived:
- from the pinned ordinary qualities, which say whether the public route attempts recovery;
- from the measured selection-plus-replay totals.

The public path itself was not exercised for a congruence member (section 3.6).

`j7_rf_census_classification_and_pin` today hard-codes RF-ZERO-TINY as the only selected case. It is generalised to pin both selected classes and their members.

**Method-only selections.** All 28 RF-CANCEL cases are `checks_passed` on the ordinary route in both modes. The public route attempts no recovery for them (`needs_source_recovery`), so, like RF-ZERO-TINY, they are method-only selections with unchanged public envelopes. This is the expected outcome.

**New public coverage.** RF-SKEW-T-PIN-OFF-345-r1e-08 and -r1e-12 are sensitive on the ordinary route in both modes. They newly publish `pressure-source-1`, in four envelopes. The census assertion that no frozen RF case publishes a source-recovery identity becomes a binding of exactly those four entries. For each, the census checks that:
- the identity is `pressure-source-1`, and the receipt carries level `complete_identified_declared_member_congruence`;
- the Rust reader accepts the envelope (`physics_source` raw read with the captured invocation);
- the envelope is byte-identical to the corresponding J2 base (section 7). J2 carries both requests in both modes, and the independent Python verifier and wasm read J2, so that is how all four envelopes reach the Python verifier: the Rust census cannot run Python;
- every published source row equals the direct method's projected value bit for bit, and every reference row meets the frozen criterion (the direct method's `method_comparison`);
- the receipt's work ledger is within its limits, and its finalization fitted (section 2.9).

**Other public bytes.** These change only where a public attempt's decline message embeds a changed failure:
- its typed error, for the 14 reclassified block cases and the transform cases whose ordinary route is sensitive or unresolved;
- its work counters, for every public decline, under section 5.

No `results_sha256`, identity, source attempt or numerical quality changes except in the four new publications.
- *Measured* on P2, under legacy charges: 37 entries (19 cases) differ, only in `diagnostics_sha256` and `envelope_sha256`.
- *Derived* for section 5's rule: every public decline's message also changes in its work numbers. P3 did not re-measure the public envelopes.
- *Not measured:* P2's congruence shortcut builds no receipt, so the four new publications are first produced by the implementation, which must establish them as above.

**Re-pinning.** The pin is regenerated once, deliberately (`SWBPIPE_J7_RF_CENSUS_WRITE=1`). The review compares it with the previous pin entry by entry, and every difference must fall in a class stated here.

**Text updated at implementation, not by this design PR:**
- b7-plan's J7 census paragraph (a Design condition naming "an exact block of order above 2 … 94 cases" and "Public RF selected-source coverage is zero");
- the census module's doc comment (`j7_rf_census.rs`, which names RF-ZERO-TINY as the only directly selected case).

Both change with the implementation, to the counts and coverage it establishes.

## 7. Receipt, Rust reader and Python verifier (v3 only)

The changes apply to the current `PRESSURE-SOURCE-1` receipt only; v2 is gone.
- **Schema** (`schemas/physics_source_recovery.schema.json`, reviewed input #1):
  - the `source.free_blocks` items' `maxItems` goes from 2 to 4;
  - `source.level` becomes an enum of `complete_identified_represented_contributions` (meaning and bytes unchanged; every member is a signed permutation) and `complete_identified_declared_member_congruence` (at least one congruence member);
  - `failure.block_order` and the `work` definition keep their shape.
- **Producer** (`source_receipt/source.rs`):
  - the published-block coverage check admits blocks of up to 4 DOFs (`b.len() > 4` refuses);
  - `level` is chosen as above;
  - the normalized-source payload gains `congruence_members` (the element ids, in member order) only when the list is non-empty. Without congruence members, the payload and `normalized_source_sha256` keep their structure, and each frame's `local_matrix_bits` and `transform_bits` are already in it;
  - `failure_fields` keeps its mapping: a transform decline maps to `unsupported_source_closure`, and `Budget` to `budget`.
- **Rust reader** (`core/reporting/result_export/src/source_blocks.rs`) and **Python verifier** (`tests/verifiers/source_blocks.py`):
  - `FAILURE_BLOCK_ORDER` requires `block_order > 4` for `unsupported_block`;
  - block length and the level enum are checked through the schema;
  - their work checks are unchanged (section 5.4).

  **Stored receipts.** A stored pressure-source-1 result that recorded an `unsupported_block` refusal of order 3 or 4 becomes unreadable: the receipt carries no version that would set it apart, and `FAILURE_BLOCK_ORDER` refuses it.
  - This applies the owner's direction of 2026-10-10 (one current version; "nothing needs to be preserved for backwards compatibility"): the current reader judges a receipt against the current producer's contract, and such a result is re-run.
  - No committed fixture or corpus carries such a receipt (checked by the independent review).
  - The alternative is to keep the readers at `> 2` and rely on the producer never to emit an order of 3 or 4. That would leave a receipt the current producer cannot produce readable as valid, so it is not taken.

  Neither reader can re-derive the exact solve, the transform closure or the charges, and neither does today. They check structure, partition, projections, hash bindings and the work ledger, and they gain nothing that would share producer code. The Python verifier stays independent and reads the J2 corpus unmodified.
- **J2** (`fixtures/results/pressure_source_reader_corpus.json`, generated by its PP test):
  - three genuine bases are added, in both modes:
    - the census's RF-SKEW-T-PIN-OFF-345-r1e-08 and -r1e-12 requests, re-authored as the census re-authors them (congruence publications; section 6 binds its four envelopes to them);
    - an invented order-4 model whose ordinary route is sensitive, which the implementation chooses and pins. It is authored with an explicit `euler_bernoulli` shear model and zero mill tolerance and corrosion allowance, which the readers require after U6/U8 (section 10);
  - reader mutations are added: a 5-DOF block (schema), `unsupported_block` with `block_order` 3 or 4 (`FAILURE_BLOCK_ORDER`), and an unknown `level`. Native Rust, wasm and the Python verifier read all of them. The desktop reads through the Rust reader (wasm), so no TypeScript changes.
- **Tables.** No semantic table changes. Limitation 1 of pressure-source-1 names "the existing bounded source adapter load, transform, block and 256-DOF limits" without giving their values, and the receipt's `level` carries the source basis.

## 8. Mutants

Each mutant is introduced into a scratch copy of the implementation and must be killed by the named test. Tests are new unless marked existing.

| ID | Fault | Killed by |
|---|---|---|
| RF-1 | Functional-entry order cap of 3 or 5 instead of 4 | `order_four_blocks_select_and_order_five_declines` (FK: tridiag(−1,2,−1) of order 4 gives u = (4,3,2,1)/5 exactly; order 5 gives `UnsupportedBlock{5}`); the census pin (RF-CANCEL selected, CHAIN-n03 reports 6) |
| RF-2 | Legacy entry admits order 3 | the existing FK tests `unsupported_singular_negative_asymmetric_range_and_budget_are_explicit` and `original_exact_graph_does_not_use_absorbed_stored_coupling`, and nonlinear `strict_gap_unsupported_blocks_and_mixed_families_do_not_gain_qualification` |
| RF-3 | Warrant checks only det A > 0, or skips Δ3 | `order_four_warrant_checks_every_leading_minor` (a symmetric 4×4 with Δ1, Δ2, Δ4 > 0 and Δ3 < 0 is `NotPositiveDefinite`) |
| RF-4 | Adjugate transposed, or a cofactor sign flipped | `order_three_and_four_solutions_are_exact` (SPD blocks with distinct entries, compared with independently computed rationals); the Context's zero-reaction check |
| RF-5 | A block's DOFs get distinct denominator expansions (which `evaluate` would otherwise turn into wrong values) | `order_four_block_dofs_share_one_denominator` (bit-identical denominators); `evaluate_refuses_a_block_with_mixed_denominators` (the 2.4 check refuses a mutated denominator with `Invalid`) |
| RF-6 | Reaction grouping applied to order ≤ 2 blocks | the re-pinned source controls (`pressure_source_controls_keep_exact_work_and_current_bytes`) and RF-ZERO-TINY's re-pinned work |
| RF-7 | `upper` or `minors` left out of `same_bits` | `order_four_replay_refuses_a_mutated_minor` |
| RF-8 | Retained witness bound loosened or omitted for o ≥ 3 | `retained_order_four_witness_size_is_bounded` |
| RF-9 | T accepted with unequal or non-diagonal blocks | `congruence_requires_identical_rotation_blocks` |
| RF-10 | Rounded global contributions used as a congruence member's exact source | `congruence_source_is_the_exact_symmetric_congruence` (k equals an independent exact Tᵀ K T, and a (3,4,0) member selects); the census rows of PIN-OFF-345-r1e-08/-12 |
| RF-11 | Member end rows formed without T, or from rounded globals | `congruence_end_actions_equilibrate_exactly` (at every node, the sum of the members' exact global end actions equals supports plus loads) |
| RF-12 | Formation-determinism check skipped | `congruence_closure_refuses_a_changed_rounded_contribution` |
| RF-13 | Unit precheck skipped | `congruence_unit_precheck_declines_before_descriptor_construction` |
| RF-13b | Unit precheck undercounts (D3's memory bound depends on it) | `congruence_unit_precheck_equals_descriptors_charge` (the precheck's count equals `descriptors_charge`'s count of the built descriptors, for every congruence census case and constructed models of 1 to 5 members) |
| RF-14 | Pre-0.4 publication behaviour changed (for example a screen added) | the existing `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` |
| RF-15 | Producer keeps `b.len() > 2` | `order_four_selection_publishes_a_pressure_source_receipt` (J2's order-4 base publishes, and Rust and Python read it) |
| RF-16 | Reader or verifier keeps `block_order > 2` | J2's `block_order` 3 and 4 mutations (Rust and Python) |
| RF-17 | Congruence case published with level `…represented_contributions` | `congruence_receipt_declares_declared_member_congruence` |
| RF-18 | Transform decline text loses "transform" | `transform_declines_map_to_unsupported_source_closure` |
| RF-W1 | Per-pair constant dropped or reduced (undercharge) | `metered_mul_charges_match_the_independent_oracle` (exact equality with the independent grow-expansion oracle of 5.4); the maintained charged ≥ counted control |
| RF-W2 | Pre-add charge taken after the add, or on the length after it | `metered_refusal_precedes_the_refused_step` (at a limit set between two steps, the refusal happens before the add and nothing partial escapes); the oracle |
| RF-W3 | Legacy precharge kept on some entry (two regimes) | the oracle run through every entry (functional, legacy, `Response`, `RetainedResponse` and strict-gap) |
| RF-W4 | Expansion-length refusal dropped from the metered loop | `metered_mul_keeps_the_expansion_term_refusal` (a product that would pass E terms refuses with `Budget`) |

## 9. Blast radius and D3

**Code.**
- Frame kernel (DEL-04-01): `structural/exact_boundary.rs` (cap per entry, order-3/4 arithmetic, witness, reactions, congruence source, metered `mul`) and `exact_boundary/functionals.rs` (retained bounds, replay), with new tests.
- Product physics (DEL-04-07):
  - `source_recovery.rs` (closure, descriptors, unit precheck);
  - `source_receipt/source.rs` (blocks, level, payload);
  - `source_recovery/j7_rf_census.rs` (classes, public binding, its module doc);
  - `source_receipt/load_state_fallback_tests.rs`, `s11g_tests.rs` (T13) and `retained_facade_tests.rs` (B3B RV123 S2), whose scenarios or pins move (section 5.5).
  - `lib.rs`'s publication paths are unchanged (section 2.9).
- Readers (DEL-05-02): `result_export/src/source_blocks.rs` and `tests/verifiers/source_blocks.py`.
- Nonlinear (`core/solver/nonlinear_integration`, owned by DEL-04-04): the code does not change. Its strict-gap path is charged by the new rule:
  - its work counters move;
  - no committed outcome changes (measured, section 5.5);
  - models that exhaust today's 2,000,000 per-call limit can complete.

  DEL-04-04 is therefore affected, and so is DEL-04-01, as the owner of the frame-kernel changes and of the K-D5 nonlinear design (`kd5-nonlinear-active-set.md`), which HELP_HUMAN's ruling names.
- `tools.deliverables impact DEL-05-02` also lists DEL-04-07, DEL-07-03 (material, component and rule-pack editors) and DEL-11-01 (user guide skeleton). RF changes no editor or guide surface; the desktop reads receipts through the Rust reader.

**Reviewed inputs.** Only #1, `physics_source_recovery.schema.json`, changes. No semantic table changes.

**Priced layouts.**
- `BlockWitness` gains two optional vectors (about +48 B).
- The Context gains a private adjugate cache and the cap.
- The Snapshot and `RetainedResponse` carry the congruence operands.
- `Sources` carries the congruence list.
- `exact::Limits`, `Work` and `AttemptBudget` keep their layouts; the charging rule needs no field.

**TEXT.**
- New or changed literals: the transform decline (40 B, replacing 44 B) and the receipt level (46 B).
- The census module's new sites, at multiplicity 0, are listed under J3's guarded rule.
- Decline messages that embed work counters keep their classes (a usize Debug is at most 20 B).

**Pricing tool** (`tools/retained_pricing`; new labels argued in [retained-pricing-rules.md](retained-pricing-rules.md)): T07's model gains these rows:
- the witness bound with `upper` and `minors`. It stays within the existing 4·EXP per block slot: at most 3.5 expansions per DOF at order 4, plus about 80 B of inner headers per DOF;
- the Context adjugate cache: at most 2.5 expansions per DOF;
- the order-4 preparation and reaction transients: about 26 expansions;
- the congruence k-children: at most 36 terms per member contribution, against today's 4Z + 2C rule;
- the Snapshot's congruence operands: 2,304 B per member per snapshot.

The new loops also need rules, or the run stops: the block arithmetic (bounded by order 4), the congruence formation (144 entries × 9 products), the reaction grouping (bounded by blocks) and the unit precheck.

**Fixtures, corpora and pins.**
- *From coverage:* the census pin; J2 (three new bases in both modes, and mutations); the four new public envelopes, pinned in the census and byte-equal to J2's PIN-OFF-345 bases; and, at implementation, b7-plan's J7 census paragraph and the census module doc (section 6).
- *From section 3's reclassification:* the W1 route-E successor pinned by `retained_facade_tests::b3b_rv123_s2_unavailable_exact_case_binds_def_e`. Its decline text and the hashes binding it move, in both modes.
- *From the uniform rule:* every committed artifact that carries a source attempt's work counters, or a decline message containing them:
  - the 24 raw source controls (`fixtures/product_preview/pressure_source/**`; measured: 6 leaves each, all work-derived) and their 30 derived documents (`fixtures/results/pressure_source_load_state_*`);
  - J2's existing bases;
  - MJ6/MJ9's captured bases (`tests/pressure_source_join.rs`);
  - the public-serialization pin in `source_receipt/pressure_source_context_tests.rs`;
  - `core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-*.raw.json`, whose outcome changes for work alone. It is either replaced by a work-independent sensitive 0.4 request, or becomes the selected publication with its consumers re-stated: T13, the reader-verdict and base-reader corpora, `loadReferenceReaderCases`, `load_reference_source_contract.rs`, the desktop load-reference tests and `test_ci_numerical.py` (section 5.5);
  - the five `load_state_fallback_tests` scenarios, whose limits are re-derived.
- Every regenerated byte difference must be a work counter, a hash binding one, a decline text embedding one, or one of the outcome changes listed in section 5.5. Each file is regenerated by its own writer.
- *Expected unchanged:*
  - the nonlinear suites and their pins (measured: identical under both rules), and DEL-04-01's K-D5 nonlinear outcomes;
  - the retained corpus `fixtures/results/retained_precision_cases.json`, and through it the R1 and R2 reader records and the desktop WASM inputs (derived, section 5.5).
- U6/U8 regenerates several of the same families for its own reasons (its design, section 9). RF regenerates them once more, on top of U6/U8's bytes.

**Unchanged:** references, criteria, tolerances, tables, D1 limits, the model format, the 0.4 eligibility boundary and the desktop's code. The desktop's load-reference tests are affected only through `load_reference_fallback_uz`, as above.

**D3** (derived; the actual-source re-registration replaces these figures). In the phases that price the live source attempt (T07) and its receipt replay (T25), at the D1 caps (32 nodes, so N = 192, and m = 32 members):
- **Order 4: about +2.1 MB.** The adjugate cache is about 0.98 MB in each of two live Contexts. The witnesses, transients and accumulators add about 0.1 MB.
- **Congruence: about +5.5 MB.**
  - The exact k-children are bounded at 36 terms per member contribution, with doubling capacity: about +2.4 MB in each of two Contexts.
  - The operand copies in three snapshots add about +0.2 MB.
  - Descriptor #1 is now bounded by the unit law through the precheck, at most about +0.5 MB above its construction-law bound.
- **TEXT: negligible** (under 1 KB per invocation).
- **Total: about 8 MB.** The charging rule does not affect memory.

At B7b's registration, the tight merged dense W3 phase had 24,554,418 B of headroom. U6/U8 is re-registering. Its design reports merged dense W3 at +390.6 MB with its levers ("measured", on scratch pricing of `dff1ff2a08`). It estimates X1 as then the tight phase, at about +53 MB; that figure is not measured. Both figures exclude several items its design lists as not yet priced.

**Assumption.** RF fits at M = 12 GiB, with the unchanged 0.9 M margin and 5 % TEXT reserve, if U6/U8's registered profile leaves at least about 10 MB in every phase that prices T07 or T25. If it leaves less, RF has one lever of its own: recomputing the adjugate in Solve frees about 2 MB at the cost of work. Otherwise RF returns to HELP_HUMAN with the measured shortfall. Decision 22 is not proposed for change.

## 10. Interactions with T4-U6/U8

U6/U8 (`codex/piping-t4-u6u8-20261016`, head `334dda5eb0` when this was written; design DEL-05-03 `section-bases-and-timoshenko.md`) is unmerged. This design is based on main `dff1ff2a08`. Each interaction below is re-checked on merged main before implementation starts.

1. **`prepare_sources`.** U6/U8 adds the O4 interim declines (Timoshenko, nonzero mill tolerance, nonzero corrosion allowance) after the D7 declines and before the dimension closure. RF's congruence closure and unit precheck sit in the member loop, after them. A Timoshenko member therefore never reaches the congruence. The census re-authors every case to Euler–Bernoulli with zero allowance, and the census inputs carry no mill tolerance. *Re-check:* on merged main, every census case's first cause and P1's operand dump are unchanged.
2. **FK layouts.** `FrameSection` gains `beam_theory`, and `FrameElement` grows from 136 to 152 B (re-priced by U6/U8). RF reads `local_stiffness()`, `orientation()` and `global_stiffness()`, which U6/U8 states are bit-for-bit unchanged for Euler–Bernoulli. *Re-check:* the census operands are bit-identical, and T07's `Xc("FrameElement", m)` uses U6/U8's atom.
3. **Reviewed input #1.** U6/U8 changes `PhysicsGeometry` and adds geometry definitions. RF changes `source.free_blocks` and `source.level`. The definitions are disjoint (measured: `source`, `failure`, `case`, `work` and `projection` are byte-identical on both branches), and RF's re-registration binds the merged bytes.
4. **Tables.** U6/U8 revises the pressure-source-1, pressure-1 and physics-retained-1 tables. RF changes none.
5. **Readers.** U6/U8 edits `source_blocks.rs` and `source_blocks.py` for per-basis row coverage. RF edits their `FAILURE_BLOCK_ORDER` lines. The hunks are disjoint.
6. **The census pin.** U6/U8 regenerates it, because its public digests move with the dual-basis stress rows. RF's regeneration is applied on top of U6/U8's pin, and its diff must stay within section 6's classes. RF's direct-method outcomes do not depend on U6/U8: the source is Euler–Bernoulli with zero allowance. *Re-check:* repeat P3's measurement on merged main. The four new public envelopes then carry U6/U8's dual-basis rows and its geometry shape, and their receipts must read valid under U6/U8's readers.
7. **The uniform rule's regenerated families** (section 9) are the same families U6/U8 regenerates. Their bases move with U6/U8 first.
8. **D3.** U6/U8 adds U6-DH, U6-RB and the route-specific row premise, and re-prices the FK layouts. RF's delta adds to whatever headroom U6/U8 registers (section 9). *Re-check:* U6/U8's registered per-phase headroom against RF's estimate, before RF's implementation begins.
9. **b7-plan.** U6/U8 edits "Current model format". RF adds only the two link lines (the B7b table row and "After B7b" item 2). The hunks are disjoint.
10. **Readers and authoring after U6/U8.**
    - After U6/U8 the pressure-source-1 readers require `euler_bernoulli` and zero allowances. J2's new invented order-4 base is authored that way (section 7).
    - With Timoshenko as the authoring default (U8), default-authored documents are declined by name before they reach RF's paths, until the O4 epoch.
    - U6/U8 also edits `source_receipt/pressure_source_context_tests.rs` and `retained_memory.rs`, both of which RF re-pins. Their re-pinning starts from U6/U8's bytes.
11. **The later O4 epoch** (Timoshenko and corrosion-allowance source extension) will lift U6/U8's declines. Its Timoshenko local matrices then enter RF's congruence unchanged. The congruence handles any declared K_local, and the O4 epoch must keep RF's closure, witness and charging rules.

## 11. Rulings

**Applied** (HELP_HUMAN, 2026-10-16; sections 4 and 5):
- the subnormal final reserve is excluded;
- metered charging is the single work-charging rule, with the per-pair constant 96, an independent test-side oracle and a maintained control;
- strict-gap qualifications the rule newly allows are accepted, with evidence.

**Decided in this design, for the independent review to check:**
- the exact source of a non-axis-aligned member is its exact congruence (3.1);
- the order cap is scoped per entry (2.1);
- no new publication rule; the pre-0.4 path keeps its named refusal (2.9);
- the receipt states the source basis through `level`, and no table changes (7);
- a stored receipt recording an order-3 or 4 block refusal is no longer readable and is re-run, under the owner's one-current-version direction (7).

**Stated for HELP_HUMAN's information.** The ruling expected that no previously selected case loses selection. The census shows that for its single-case invocations. On a shared invocation or strict-gap run ledger, the cross-attempt exception of 5.2 (6) can in principle cost a later case its selection, and that outcome fails closed. No committed suite exhibits it.

**Not proposed, and reserved:**
- any change to D1's limits (`PHYSICS_SOURCE_WORK_LIMIT`, the exact-boundary limits, 256 DOFs);
- any change to a ScopeOfWork commitment, acceptance criterion, frozen reference, criterion or tolerance;
- Decision 22 (M, the 0.9 M margin, the 5 % reserve, the budget model);
- G10, B8 and everything under b7-plan's "Held by the owner".

None is needed. If U6/U8's registered headroom leaves RF short, the shortfall returns to HELP_HUMAN (section 9).

**For a later unit, not this epoch:**
- extending above order 4 meets the 8 criterion failures of section 1.5;
- RF-RANGE-THIN-B needs a sound reserve and scaled exact arithmetic.

## 12. Implementation and verification

**Order of work,** from merged main after U6/U8:
1. Re-check section 10's points.
2. Frame kernel: the charging rule, its oracle and its control. Re-pin every work counter it moves (section 5.5), and nothing else.
3. Orders 3 and 4.
4. The congruence source.

   After steps 3 and 4, every order-1/2, signed-permutation control must be byte-identical to its step-2 re-pin. Otherwise the rule's own work changes would mask RF-6.
5. Product physics: closure, descriptors, unit precheck and receipt.
6. The readers, J2 and the census.
7. The regenerated families.
8. The pricing rules and the actual-source D3 re-registration at M = 12 GiB, with the unchanged 0.9 M margin and 5 % TEXT reserve.

**Checks.**
- The mutants of section 8, each killed by its named test.
- The suites `select_affected_checks.py` selects for the changed paths, including `frame_kernel`, `nonlinear_integration`, `product_physics` (with `pressure_source_join`, the census and the six integration targets section 5.5 lists as not yet run), `result_export`, headless, the desktop's load-reference tests, and the Python verifier tests.
- The census and J2 in native Rust, wasm and Python.
- The hosted `harness` on the exact head.

**Independent scrutiny,** because this changes numerical meaning and work accounting, by agents separate from the implementer:
- the order-3/4 arithmetic and warrant;
- the congruence source and its recovery rows;
- section 5.2's soundness argument for the charging rule, together with the oracle;
- the regenerated byte diffs, which may only be work-derived;
- the D3 registration.
