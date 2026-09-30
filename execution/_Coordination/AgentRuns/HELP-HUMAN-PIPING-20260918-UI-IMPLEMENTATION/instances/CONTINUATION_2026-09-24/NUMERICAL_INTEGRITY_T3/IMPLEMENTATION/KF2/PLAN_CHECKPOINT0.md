# KF2 checkpoint 0: the diagnosis and the plan (I20)

This is checkpoint 0 only. It contains no code change, no build and no measurement. KF3's timed slot B was running at spawn, so I ran no cargo, no rustc and no heavy Python. Every time and ratio below is a prediction from reading, or an existing record with its source. The measurements come at A.

## 0. Record

- **Delegation:** I20 is a Type 2 TASK, started by ROOT (HELP_HUMAN, Agent 0) as a background subagent of ROOT's Claude Code session, with the brief `TASK_BRIEFS/I20_KF2_IMPLEMENTATION.md`. ROOT is the return path, and I20 does not delegate.
  - The write boundary (`T3/IMPLEMENTATION/KF2/` in `<wt>/kf2`, and no Git writes) is set by the brief and kept by me. I know of no host mechanism that enforces it.
- **Basis:**
  - product source at main `78f55f927`, in `<wt>/kf2` (branch `codex/piping-kf2-20260930`, clean);
  - T3's records at `<wt>/numerics` HEAD `32eba6f99`. The brief is unchanged since `6d9832db2` (sha256 prefix `25b5d316fe2a9ae8`), and the only later change is ROOT's KF3-B ruling.
- **Read:**
  - Root `AGENTS.md`, `agents/AGENT_TASK.md`, `TASK_BRIEFS/_COMMON.md` and `I8R_K1_RESUME.md:24-50`;
  - the I20 brief, "KF2: spawn" and "KF1: spawn" in `ROOT_RULINGS_V1.md`, and the Gates of `I13_F1B_IMPLEMENTATION.md`;
  - K6's RETURN §8.4–8.5, and V-K's RETURN with `_run_records/a1/mech_parity_time.{out,rs.txt}`;
  - F1b's gate-2 part-1 result (`IMPLEMENTATION/F1B/_run_records/gate2_130445db2/part1/result_part1_candidate.json`);
  - the Mac calibration's RECORD and T9 list;
  - R1's `references.py` RF-LARGE, RF-CHAIN and RF-MECH builders.
- **Source read, all at `78f55f927`:**
  - FKS: `:250-420`, `:540-760`, `:1186-1379`, `:1440-1548`, `:1800-2215`, `:2600-2660` and `:2840-2975`;
  - FKP: `:1870-2015`;
  - `FK/structural/sparse/tests.rs`: `:1075-1237`;
  - `P/core/solver/frame_kernel/tests/s11_site_table.rs` (below, `FKT/s11_site_table.rs`, the brief's `FK/tests/s11_site_table.rs`);
  - PP's site test (`P/core/product_physics/tests/s11f_site_test.rs`, rules 6 and 8);
  - SA: `:686-730` and `:2005-2021`;
  - SD: `P/core/solver/sparse_direct/src/structural.rs:25-40`;
  - H: `src/k6/staged.rs:455-490`, `src/k6/models.rs:415-470` and `src/bin/k6_observe/main.rs:1-60`;
  - PP: `lib.rs:1089-1107`, `:1391-1440`, `:2365-2385` and `:3795-3970`;
  - the desktop app's `P/apps/desktop/src-tauri/src/lib.rs:1553-1850` (below, `APP`);
  - the headless runner's `P/core/runner/headless/src/lib.rs:65-80` and `:336-345`.
- **Not done:** any build, test, probe or timing. There is no code in the worktree. Line numbers are at `78f55f927`.
- **One check was run:** a few lines of standard-library Python (binary64, the same operation sequence as V), to confirm §8's exact-tie construction. It is not a product measurement.

## 1. ROOT's reading: confirmed on the witness; the screen is probable, not proven

- **The witness is O(n⁴)** (§2). Each of its n(n−1)/2 pairs allocates an n-vector, re-validates the prepared system and its source (O(N² + n²)), and scans all n² cells of the direction. Only four of those cells can be nonzero.
- **An O(n²) witness that returns the same bits, errors included, exists** (§3–§4). I propose a *verified prefilter*: an O(1) evaluation of each pair's four cells, with exactly `verify_negative_direction`'s arithmetic in its order, decides whether that pair would be a witness. The one witness pair is then passed to the unchanged `verify_negative_direction`, which builds the published value.
- **The screen:** I15's hypothesis is consistent with the model structure, and I predict the refusing rows' figures from it (§7). The operation counts are certain from the code: 11,992 and 11,994 in the dense factor, against 16 and 14 for the rows' natural-order profile.
  - Both refusing pivots are predicted to lie between the two screens. That still needs A's measurement.
  - A profile-based count is honest (with proof, §7.3). But changing it changes the published bytes of essentially every dense report, as well as some classes. I recommend a separate slice.

## 2. The old witness's cost, term by term

Let n be the number of free DOFs (`prepared.matrix.len()`) and N the total number of DOFs (`source.force.len()`). There are P(n) = n(n−1)/2 pairs, visited at FKS:2200-2201. Per pair, today's code (`negative_pair_witness`, FKS:2195-2215, calling `verify_negative_direction`, FKS:2152-2193) does the following:

| Term | Work per pair | Source |
|---|---|---|
| (a) The direction `v` | 1 allocation, plus n zero writes | FKS:2202 |
| (b) `validate(source)` | the row lengths N; force and stiffness finiteness, N + **N²**; on SA's dense route, the symmetry evidence's roundoff finiteness **N²** and count rows N; the symmetry audit, N(N−1)/2 pairs of 2 reads (**N² − N**); the `seen` map, 1 allocation plus O(N) | FKS:608-675 |
| (c) The rest of `validate_prepared` | n rows; prepared finiteness **n²**; rhs n; prepared symmetry, n(n−1)/2 pairs of 2 reads (**n² − n**) | FKS:676-700 |
| (d) The direction check | n reads | FKS:2158-2160 |
| (e) The quadratic-form scan | **n²** cell tests (`matrix[i][j] == 0.0` first) | FKS:2164-2179 |
| (f) The pair's own terms | at most 4 evaluated cells, O(1) | FKS:2168-2177 |

- **Per pair:** R(n, N) = 3N² + 3n² + O(N) reads on SA's dense route (with symmetry evidence), or 2N² + 3n² + O(N) without evidence (FK's and SD's dense entries). There are also 2 allocations.
- **In all (no witness found):**
  - T_old(n) = c_r · P(n) · R(n, N) ≈ 3 c_r · n⁴ with N ≈ n, where c_r is the cost of one read;
  - plus c_a · 2P(n) for the allocations, which is O(n³) and negligible.
- **Calibration** (existing record; no new run): V-K's `mech_parity_time.out` records RF-MECH-DISC-CHAIN100 in dense mode at 312.40 s (release, Mac).
  - That model has 105 nodes, N = 630 and n = 624. Its witness found no pair (the outcome is the factor's `NumericallyUnresolved`), so it visited all 194,376 pairs: 1.61 ms per pair.
  - With R = 2.36 M reads per pair, that gives c_r ≈ 0.68 ns (0.82 ns if that path had no symmetry evidence).
- **Predicted time at the N10 size:** the two N10 models have N = 6,006 and n = 6,000 (N0's six DOFs are restrained), so P = 17,997,000 pairs.
  - The per-pair cost is 3·6,006² + 3·6,000² ≈ 2.16 × 10⁸ reads, ≈ 0.147 s.
  - **T_old ≈ 2.65 × 10⁶ s ≈ 31 days.** Scaling the calibration by n⁴ gives the same: 312.4 s × (6,000/624)⁴ ≈ 2.67 × 10⁶ s.
  - **This is a lower estimate.** At n = 624, each of the three n² matrices takes 3 MB and stays in cache. At 6,000 each takes 288 MB, and every pair streams about 1.7 GB.
  - **At K6's kill** (1,712.7 s and 1,733.7 s in the witness), the search had covered about 11,600 of 18.0 M pairs (0.06%).

## 3. The new witness: a verified prefilter (plan sketch, not code)

`negative_pair_witness` keeps its signature, its doc comment and its callers, and becomes a one-line delegation, as K1's sparse pair does:

```text
pub fn negative_pair_witness(prepared) -> Result<Option<StructuralError>, StructuralError> {
    negative_pair_witness_counted(prepared).map(|(witness, _, _, _)| witness)
}

/// KF2 doc: the search it replaces, pair for pair; each pair's four cells in
/// verify_negative_direction's order; only a witness pair is verified.
fn negative_pair_witness_counted(prepared)
    -> Result<(Option<StructuralError>, usize /*visited*/, usize /*evaluated*/, usize /*verified*/), StructuralError>
{
    validate_prepared(prepared)?;                       // once (FKS:2198 today)
    let system = prepared.source;
    let n = prepared.matrix.len();
    for i in 0..n { for j in 0..i {                     // today's order
        visited += 1;
        let sign = if prepared.matrix[i][j] >= 0.0 { -1.0 } else { 1.0 };   // today's rule
        let (mut energy, mut magnitude, mut terms) = (0.0, 0.0, 0);
        for (a, b, da, db) in [(j, j, sign, sign), (j, i, sign, 1.0), (i, j, 1.0, sign), (i, i, 1.0, 1.0)] {
            if prepared.matrix[a][b] == 0.0 { continue; }
            let original = radix_scale(system.stiffness[system.free_dofs[a]][system.free_dofs[b]],
                                       prepared.scale_exponents[a] + prepared.scale_exponents[b])?;
            let term = checked_product(checked_product(da, original)?, db)?;
            energy = checked_value(energy + term)?;
            magnitude = checked_value(magnitude + term.abs())?;
            terms += 1;
        }
        evaluated += terms;
        let allowance = 64.0 * gamma(3 * terms + 2) * magnitude;
        if energy < -allowance {
            verified += 1;
            let mut v = vec![0.0; n]; v[i] = 1.0; v[j] = sign;      // allocated once
            if let Some(witness) = verify_negative_direction(prepared, &v)? {   // today's body
                return Ok((Some(witness), visited, evaluated, verified));
            }
        }
    }}
    Ok((None, visited, evaluated, verified))
}
```

- **Why this shape.** Each pair's body is today's body (the `vec!`, `v[i]`, `v[j]`, `verify_negative_direction(..)?` and `return`), behind an O(1) guard that runs today's four-cell arithmetic.
  - The published value (`direction`, `energy`, `allowance`), and the mapping's `radix_scale` error if one occurs, are produced by the unchanged verifier on the same `v`.
  - **So the guard carries only one obligation:** it may never skip a pair on which the verifier would return `Some` or `Err`, and it must return the verifier's error where the verifier would.
  - **A guard that is too permissive** only costs one extra O(n²) verification. The result is unchanged, because a `None` from the verifier continues the loop as today.
- **No force read.** The new function reads no force token (`.force[`, `.force.` and the others; PP site test `:487-497`), so PP's rule-6 list is unchanged.
  - The alternative, building the mapped direction in the helper as the sparse `negative_verdict` does, reads `system.force.len()`. It would add a rule-6 row in PP, which is outside the write set.
- **The one helper** is `negative_pair_witness_counted`. It is private to `structural`, so tests in its descendant modules can reach it. It holds the loop and the pair evaluation inline, which gives one site-table row (§10).

## 4. The equality argument

**Definitions.**
- p is any value of `PreparedSystem`: every one `prepare_bound` builds, and also any one a test in the crate builds or corrupts.
- O is today's pair order: (1,0), (2,0), (2,1), (3,0), …, the loops at FKS:2200-2201.
- For a pair (i, j), s = −1 if `p.matrix[i][j] >= 0.0`, else +1, and v = e_i + s·e_j (zeros elsewhere), built exactly as at FKS:2202-2208.
- V is `verify_negative_direction`. OLD is today's function, and NEW is §3's.

**L1: validation.**
- `validate_prepared` (FKS:676-700, with `validate` at :608-675) reads only p and its source through shared references. Neither type has interior mutability, and its one allocation (`seen`) is local.
- So it returns the same value at every call.
- OLD's first call, at FKS:2198, is NEW's only pre-loop call. If it is `Err`, both return it before any pair. If it is `Ok`, every call inside V is `Ok`, so removing the per-pair re-validation changes nothing. An allocation failure aborts; it is not an `Err`.

**L2: the direction check.** v has length n = `p.matrix.len()`, and its entries are in {0.0, 1.0, −1.0}. So V's check at FKS:2158-2160 always passes.

**L3: the terms, their order, `magnitude` and `terms`.**
- V's double loop (FKS:2164-2179) skips (a, b) unless `p.matrix[a][b] != 0.0`, `v[a] != 0.0` and `v[b] != 0.0`. Here v[a] ≠ 0 exactly when a ∈ {j, i}.
- So V evaluates exactly the sublist of [(j,j), (j,i), (i,j), (i,i)] whose `p.matrix` entry is nonzero, in that order: V's loop order, since j < i.
- For each (a, b), V performs the same five operations as NEW, with the same operands:
  1. `radix_scale(stiffness[free[a]][free[b]], exp[a] + exp[b])`;
  2. `checked_product(v[a], ·)`;
  3. `checked_product(·, v[b])`;
  4. `energy = checked_value(energy + term)`;
  5. `magnitude = checked_value(magnitude + term.abs())`;
  
  and `terms += 1`. Here (v[a], v[b]) = (s, s), (s, 1), (1, s), (1, 1), which are NEW's (da, db).
- Each operation is a deterministic function of its operands:
  - IEEE-754 binary64 `+` and `×`, which rustc/LLVM do not contract or reassociate;
  - `radix_scale`'s power-of-two steps;
  - `abs`;
  - and `checked_*`, whose messages are fixed per function.
- So both runs reach the same first failing operation, with the same `StructuralError::Range(msg)`, or both finish with bit-identical (`energy`, `magnitude`, `terms`).
- The `i32` exponent sum is the same expression. A debug-build overflow, reachable only on a corrupted p, panics at the same cell in both.

**L4: the verdict.** Both compute `64.0 * gamma(3 * terms + 2) * magnitude`, the same left-associated expression on the same bits (FKS:2180), and test `energy < -allowance`. If that is false, V returns `Ok(None)` without any further operation (FKS:2190-2192).

**Theorem.** For every p, NEW(p) = OLD(p): the same `Err`, the same `Ok(None)`, or the same first `NegativeEnergy`, with bit-identical `direction`, `energy` and `allowance`.

**Proof, by induction over O.** Before any pair, both are in the same state (L1). At pair (i, j):
- **If NEW's evaluation returns `Err(e)`,** V would return `Err(e)` before its verdict (L2, L3), so OLD returns `Err(e)`.
- **Otherwise the verdicts agree** (L4).
  - If there is no witness, OLD's V returns `Ok(None)`, and both continue.
  - If there is a witness, NEW makes OLD's call V(p, v) itself and returns the same way: `Ok(Some(w))` returns, and the mapping's `Err` (FKS:2183-2185) propagates.
  - A `None` there is impossible by L3–L4. Even if it happened, NEW would continue as OLD does.
- **If O ends,** both return `Ok(None)`. ∎

**The errors a pair can raise, in the order they occur.** Per cell:
1. `radix_scale`: `Range("nonfinite radix input")`, which cannot occur after `validate`, or `Range("radix scaling loses normal range")`;
2. the first `checked_product`: `Range("product overflow or underflow")`;
3. the second `checked_product`: the same message;
4. the energy's `checked_value`: `Range("arithmetic outside normal range")`;
5. the magnitude's `checked_value`: the same message.

Then, on a witness only, V's mapping `radix_scale`.

Which of them a system built by `prepare_bound` can reach:
- **The product underflow is reachable.** `radix_scale(x, 0)` returns a subnormal x unchanged, and `prepare_bound` stores it. Example: K = [[1, 5e-324], [5e-324, 1]]. The dense factor fails first (`checked_quotient`), and the witness then returns `Range("product overflow or underflow")` at cell (0, 1).
- **The energy overflow is reachable.** Example: [[1, 1e308], [1e308, 1]]. At cell (1, 0), the energy −1e308 − 1e308 → −∞ fails before the magnitude.
- **A subnormal partial energy** is possible in principle.
- **The `radix_scale` and mapping errors are reachable only on a corrupted p.**
  - `prepare_bound` has already evaluated the identical `radix_scale` for every cell (FKS:1291-1294).
  - Its exponents lie in [−511, 537], so ±1 always maps into range.

**Zero couplings: NEW visits them, with no skip.**
- **A skip would be sound on the product's domain.** `prepare_bound` (FKS:1263-1379) is the only constructor of `PreparedSystem` (FKS:1365). The one later write is `formation` (FKS:1207). It refuses d ≤ 0 (FKS:1272-1286) and sets e = −⌊E(d)/2⌋, so every prepared diagonal a_kk = `radix_scale(d_k, 2e_k)` is in [1, 4). Projection leaves the diagonal alone (FKS:1333-1362).
  - So for a zero coupling, V evaluates only (j,j) and (i,i): the terms a_jj and a_ii are exact (the same `radix_scale` call as `prepare_bound`'s, times s² = 1).
  - The energy is then in [2, 8) > 0 ≥ −allowance, so the pair is neither a witness nor an error.
- **Why NEW still visits them:**
  - the dense loop must read `matrix[i][j]` for every pair anyway, so a skip saves only the two diagonal evaluations, a constant factor;
  - with no skip, the equality holds for every value of the type, including corrupted ones, not just on `prepare_bound`'s invariants;
  - so the brief's mutant 5 becomes a mutant of an *added* skip (§9, M5).

**Allocation.** The loop allocates only `v`, and only on a witness verdict. Since that verdict equals V's (L4), there is exactly one such allocation, and then a return. V's own `seen` and `mapped` are allocated once, on that call.

**At 6,006 DOFs,** no oracle can run OLD to completion. Equality there rests on this argument and on the differential test (§8), as the brief states.

## 5. The new witness's cost

- **Pre-loop:** one `validate_prepared`, R_v ≈ 2N² + 2n² (+N² with evidence) reads, about 0.1–0.2 s at n = 6,000 by §2's rate.
- **Per pair:**
  - O(1): one sign read, four zero tests, then 2 evaluated cells (a zero coupling, the diagonals) or 4 (a stored coupling);
  - each evaluated cell does two index lookups, one stiffness read, one `radix_scale` (one `powi` step when the exponent sum is nonzero), two `checked_product`s and two `checked_value`s;
  - then one `gamma` (a division) and one comparison.
- **On a witness:** one V call, O(N² + n²).
- **In all:** T_new = O(N² + n²), which is O(n²) for N ≈ n.
- **At 6,006:**
  - **Predicted ≈ 0.5–1 s** in release (18.0 M pairs at ~20–40 ns, plus 0.1–0.4 s of validation), against ≈ 2.7 × 10⁶ s today;
  - the N10 matrices are positive definite, so they find no pair and visit all of them;
  - A measures this.
- **Memory:** unchanged apart from dropping the per-pair allocations.

## 6. Sharing `pair_energy`, or a dense twin: **a dense twin**

The reasons:
1. **The obligation is against V's own loop body.** The twin is that body, restricted to four cells, with identical expressions (`free_dofs`, `stiffness` and `scale_exponents` indexing), so a reviewer reads it line by line against FKS:2168-2177.
   - A shared helper would take four (represented, original, exponent sum, da, db) cells from two storage schemes. It would move the arithmetic away from both verifiers.
2. **Sharing edits FKP,** which holds the default product route's witness. It would bring the sparse witness into KF2's product-reaching change set and gate, with no gain in O(·) or safety.
3. **Under design §3,** the dense verdict and direction come from V. The only part that could be shared is the four-cell sum. The sparse side's verdict (`negative_verdict`) reads the force length, and so is a different shape.
4. **Precedent:** K1 kept twin verifiers (`verify_negative_direction` and `verify_sparse_negative_direction`, "the same terms in the same order"), each with its own site-table row.

The cost is about 12 lines of arithmetic parallel to `pair_energy`. It is covered by §8's differential test and by K1's existing dense-against-sparse witness test (`sparse/tests.rs:1145-1199`).

## 7. The dense pivot screen

### 7.1 The mechanism (code facts)

- **The two factors charge different counts.**
  - The dense `cholesky` (FKS:1829-1871) screens its pivot at row i with `operation_count = 2j + 2 = 2i + 2` (FKS:1863).
  - The skyline (`ProfileFactor::factor_structural_profile`, FKS:1991-2025, with the screen at :2015-2021) charges `2(i − first_i) + 2`.
- **The screen** is `pivot ≤ 64·γ(m)·scale` → refuse (FKS:1523-1548). `scale = |a_ii| + Σ_k |l_ik|²` in the dense factor.
- **The refusals are at the last node's DOFs.**
  - In the Schur-complement reading, row i's pivot is the stiffness of DOF i with the earlier DOFs condensed and the later ones clamped.
  - An interior node is attached to its clamped successor through a whole element, so its pivots are O(element stiffness).
  - Only the last node, which has no successor, sees the whole structure's flexibility. That matches K6's refusals at 6001 and 6002 of 6006.

### 7.2 The refusing rows (counts are certain; pivots predicted from R1's models)

The two models:
- **CHAIN-n01000-ROT:** 1,001 nodes, members of 3 m along Q3·x = (1, 2, −2)/3, N0 fixed. SEC_N: E = 200 GPa, OD 0.2 m, ID 0.18 m, so EA/L = 3.979e8 and 12EI/L³ = 2.401e6.
- **TREE-n01000-AX:** P0, then P_k and B_k interleaved (`H/src/k6/models.rs:438-462`), so the last node is B500, on a branch along +z. Its UZ is the branch's axial DOF.

| | CHAIN-n01000-ROT | TREE-n01000-AX |
|---|---|---|
| Refusing global DOF (K6 §8.4) | 6001 = N1000.UY | 6002 = B500.UZ |
| Ordered index i (N0 / P0 restrained) | 5,995 | 5,996 |
| **Dense count 2i + 2** | **11,992** | **11,994** |
| **Dense screen 64γ(m)·scale** | **8.521e-11 · scale** | **8.522e-11 · scale** |
| The row's first nonzero, f_i (natural order) | N999.UX, 5,988 (Q3 couples all 12) | P500.UZ, 5,990 (an axial coupling only) |
| **Profile count 2(i − f_i) + 2** | **16** | **14** |
| **Profile screen** | **1.137e-13 · scale** | **9.95e-14 · scale** |
| Predicted pivot (physical) | 5·12EI/L_tot³ = 1.20e-2 N/m (guided tip; L_tot = 3,000 m; UX condensed gives 1 + d_y²/d_x² = 5) | ≈ 12EI/L³ over the 1,500 m spine, with the branch's EI/L rotational restraint at P500: 1.9e-2 N/m |
| a_ii | 3.979e8·(4/9) + 2.401e6·(5/9) = 1.782e8 | EA/L = 3.979e8 |
| **Predicted pivot / scale** (scale ≈ 2a_ii, since Σl² = a_ii − pivot) | **≈ 3.4e-11** | **≈ 2.4e-11** |
| Verdict | dense refuses (3.4e-11 ≤ 8.5e-11); profile passes (×300) | dense refuses; profile passes (×240) |

- Radix equilibration scales the pivot and the scale alike, so the ratio is scale-invariant.
- **Cross-checks from the same reading:**
  - CHAIN-AX's tip UY has pivot/scale ≈ k_b/(2·12EI/L³) = 5e-10 > 8.5e-11, so it passes. K6 records no dense refusal on CHAIN-AX.
  - Interior rows pass.
- **The sparse path** uses RCM with its own `first` (SD:25-29). It passes both models (K6: Sensitive). A records its count at the corresponding row.
- **Accuracy of the pivot:** a backward error of γ(bw)·|L||Lᵀ| (about 1e-15 relative to element entries) moves the tip Schur complement by about 1e-15 × λ_max(element) ≈ 4e-7 N/m, against 1.2e-2. So the pivot has about 4–5 correct digits: the refusal comes from the count, not from the pivot's accuracy.
- **Measurement at A,** if ROOT wants it before ruling:
  - a scratch probe under `<wt>/scratch/i20/`, never committed, that replays the dense factor's pivot loop on `PreparedSystem::matrix()` (public) for the two models;
  - it prints i, pivot, scale, 2i + 2, f_i and both screens at the refusing row;
  - it takes 65–86 s per model, one at a time, with the memory guard.

### 7.3 Is a profile-based count honest? **Yes**

**Lemma Z.** Let f_i = min{k ≤ i : a_ik ≠ 0.0} in the prepared matrix's natural order. Here a_ii ∈ [1, 4), so f_i ≤ i, and −0.0 counts as zero. Then for every k < f_i, the dense factor's l_ik ∈ {+0, −0}.

*Proof, by induction on k.*
- At step (i, k), `sum` starts at a_ik = ±0.
- For every m < k, the term is `checked_product(l_im, l_km)` = ±0, because l_im = ±0 by hypothesis.
- ±0 − ±0 = ±0.
- `checked_quotient(±0, l_kk)` is ±0, because l_kk > 0 (its pivot passed). It is `Ok`, since a = 0 disables the underflow test (FKS:593-600).
- No `checked_*` fails on zeros. ∎

**At the pivot (j = i),** for k < f_i:
- `term = l_ik·l_ik = +0`;
- `sum − (+0) = sum` exactly (also for sum = −0);
- `scale + 0 = scale` exactly.

So the dense factor's computed pivot and scale are bit-identical to those of the same loop run over k ∈ [f_i, i) only. The first 2f_i operations are exact, contribute δ = 0, and round nothing.

**Conclusion:**
- γ(2(i − f_i) + 2) bounds the pivot's own evaluation under the same model the skyline already uses: identity order, minimal envelope.
- The inherited error in L is covered, in both factors alike, only by the heuristic factor 64.
- The operation counts 2i + 2 charge exact zeros.
- I found no design text that requires the dense factor to charge the full row. K4's retained factor also uses the profile count (`FK/structural/retained/factor.rs:580`).

### 7.4 The proposal, and what it would change (for ROOT's ruling)

- **The change:** in `cholesky`, compute f_i once per row, an O(n) scan (O(n²) in all, against the factor's O(n³)), and charge `2 * (i - f_i) + 2` at the pivot.
- **It is monotone:** pivots, scales and L are unchanged, and only the pass or fail decision moves. So no dense pass becomes a refusal, and a dense refusal becomes a publication or a later refusal.
- **Every dense report's bytes change.**
  - `PivotEvidence.operation_count` and `.screen` change on every row with f_i > 0, which is essentially every row of every model.
  - The report is published whole in the structural-evidence diagnostic (`PP:1106-1107`, `{:?}` of `StructuralReport`, and `PP:3946`, `:3957`).
  - **So T9's 56 dense outputs and every dense part-1 envelope that publishes a report change bytes.** This is a published-byte change across the dense mode, not only a change of class.
- **The requests whose dense class is expected to change** (dense refused → published, as sparse):

| Request (both entries) | Where | Dense today | Sparse | Prediction |
|---|---|---|---|---|
| RF-LARGE-CHAIN-n01000-ROT | part 2 | refused (the factor; the witness never ends) | Sensitive | publishes (§7.2) |
| RF-LARGE-TREE-n01000-AX | part 2 | refused, the same | Sensitive | publishes (§7.2) |
| RF-CHAIN-T-n10-r1e-12 | part 1 | refused_blocked | Sensitive | likely publishes: the N10.RX Schur ≈ k = 2.16e-6 against a = GJ/0.75 = 5.76e6, so pivot/scale ≈ 1.9e-13; dense 64γ(118) = 8.4e-13 refuses, profile 64γ(14) = 9.9e-14 passes (margin only ×1.9). R1 says this case "must be solved" (`REFERENCES/references.py:2549`) |
| RF-CHAIN-A-n10-r1e-12 | part 1 | refused_blocked | Sensitive | likely publishes: the same structure, 64γ(112) against 64γ(14) |
| RF-SKEW-T-PIN-OFF-122-r1e-12 | part 1 | refused_blocked | Sensitive | a candidate; A must measure it |
| RF-WEAK-W-L-r1e-12 | part 1 | refused_blocked | Sensitive | a candidate; A must measure it |

- **Source:** F1b's gate 2 at `130445db2`, part 1, where these are the only dense-refused and sparse-published pairs. CONT-n10000's dense refusals are F1b's memory guard, not the factor.
- **RF-SKEW-T-CANT-OFF-122-r1e-12** (dense Sensitive, sparse refused) keeps its dense class, by monotonicity.
- **Not yet enumerated:** T9's 56 dense outputs. None is known to refuse at the screen. The A probe can list them.
- **Recommendation: ROOT splits the screen out of KF2 and declines it here.** It is its own slice, with an owner-facing note, because it:
  - changes published bytes across the dense mode, so T9 and part 1 cannot be byte-identical;
  - changes at least two, and likely up to six, request classes;
  - has an honest proof (§7.3), which the new slice's review can check independently.
- **KF2 stays byte-identical.** After KF2, the N10 pair ends with the factor's refusal and not a timeout (§11), and the class disagreement remains until that slice.

## 8. Tests

**The in-crate module,** located per Q1. It needs private access: the counted helper, `validate_prepared`, the struct's fields for corrupted systems, and verbatim reference copies.

- **T1: the reference copies.** `reference_verify_negative_direction` and `reference_negative_pair_witness` are verbatim copies of FKS:2150-2215 at `78f55f927`, renamed. They are test-only.
- **T2: the comparator.** It compares bits, not `==`:
  - the `Result` and `Option` shapes and the `StructuralError` variant and message;
  - for `NegativeEnergy`, `energy.to_bits()`, `allowance.to_bits()` and every `direction[k].to_bits()`;
  - and `Debug` equality as a second check.
- **T3: the differential test** (NEW against the reference, on every system below). Systems are built through `prepare_structural` unless marked "corrupted". A splitmix64 generator sits inline, with fixed seeds (FK has no dev-dependencies).
  - **Sizes:** n ∈ {2, 3, 4, 5, 8, 13, 21, 34, 48, 64}, plus 80 and 100 for no-witness systems. There are free maps that are not sorted, prescribed DOFs (N > n), and prescribed values.
  - **Witness position:**
    - at the first pair (1, 0);
    - at the last pair (n−1, n−2);
    - in the middle;
    - none (positive definite banded, and dense-random diagonally dominant);
    - two or more witnesses, where the first must win.
  - **Zero couplings next to a witness:** +0.0 and −0.0 couplings before and after the witness in the same row and column.
  - **Both coupling signs** at the witness (s = −1 and s = +1).
  - **Ties at the allowance's edge.**
    - **Exact ties are reachable** on built 2×2 systems K = [[1, t], [t, b]], with every sum exact, when:
      - 64γ(14) = 2⁻⁹⁶·(7·2⁵⁰ + 12) exactly, so fl(64γ(14)·M) lands on the energy's 2⁻⁵² grid for a computable set of M = μ·2⁻⁵⁰ ∈ [4, 8), namely q·2⁻⁵² with q ∈ [1792, 2048), q ≡ 0 mod 8 and μ even;
      - t = (4μ + q)·2⁻⁵⁴ and b = (4μ − q)·2⁻⁵³ − 1.
    - The test finds them by an integer search and requires the reference to give energy == −allowance on at least one.
    - **Checked at checkpoint 0** (standard-library Python, the same sequence): 9 (q, μ) pairs tie exactly, and 64γ(14) equals 2⁻⁹⁶·(7·2⁵⁰ + 12). For example, q = 1808 gives t = 1.0089285714286702, b = 1.017857142856939 and energy = −allowance = −4.014566457044566e-13.
    - **Its sides are b ± 1 ulp:** the nearest energies that can be reached. Energy lives on a 2⁻⁵² grid for any built pair, because the diagonals are ≥ 1, so "one ulp of energy" (2⁻⁹³ there) cannot be reached. The sides are one ulp of the input, and they straddle the verdict: in the example, b − 1 ulp is a witness, b ties, and b + 1 ulp is not a witness.
    - **The scan family:** k = 1 + m·2⁻⁵¹ with unequal diagonals whose random mantissas make the sums round, which gives more pairs at the edge.
    - **A self-check of discriminating power,** in the style of V-K's discriminating controls. The test computes the reversed cell order's sums itself and asserts that the family contains a pair whose reversed-order verdict differs from the reference's. The exact-tie pairs alone do not discriminate order, because their sums are exact in either order.
  - **Errors in a pair's terms:**
    - the energy overflow ([[1, 1e308], [1e308, 1]]-type blocks);
    - the product underflow (a subnormal coupling at exponent sum 0);
    - each placed before a would-be witness (both return that `Err`) and after one (both return the witness).
  - **Corrupted (built, then edited, as `krev04` does at FKS:2944-2974):**
    - a shape corruption, where both return `Err(InvalidInput(..))` without panicking;
    - a source with a negative diagonal behind a zero coupling, where the witness sits on a zero-coupling pair;
    - an exponent edit that makes a zero-coupling pair's diagonal `radix_scale` overflow;
    - balanced exponents (+1100 and −1100) that make the witness's mapping `radix_scale` fail after its verdict;
    - a source coupling that is nonzero where `prepared.matrix` is zero (projection);
    - an all-zero-cell pair (magnitude 0: energy 0 against −0);
    - a pair with two failing cells of different kinds: (j,j) fails in `radix_scale` (an exponent edit), and (i,i) fails in `checked_product` (a subnormal source diagonal at exponent 0). Built systems cannot give two different first errors in the two orders.
- **T4: the count invariants,** on every T3 system with an `Ok` result:
  - `verified == 1` exactly when the result is `Some`, and 0 otherwise;
  - `visited == n(n−1)/2` on `Ok(None)`;
  - `evaluated ==` the number of nonzero cells over the visited pairs (computed independently);
  - `evaluated ≤ 4·visited`.
- **T5: the cost in debug.** RF-LARGE-CHAIN-n00100-AX's stiffness is built with FK's own frame assembly and the R1 section (n = 600). The witness is counted, with no factor: visited = 179,700, `evaluated` = 2·visited + 2·(stored couplings), and verified = 0. It asserts counts, not time.
- **T6: the cost in release (`#[ignore]`, run at A with `--release -- --ignored`; not in CI).**
  - A 1,001-member chain has N = 6,012 and **n = 6,006**. It is built with FK's frame assembly and prepared, with no factor.
  - The witness is timed, with the counts above. The memory is about 0.6 GB.
  - **The bound: witness < 30 s in release.** The prediction is ≈ 1 s, and the bound sits far below the factor's 65–86 s and the 1,800 s kill.
- **T7: N10's two models in release** (not in CI; Q2).
  - H's unchanged `k6_observe`, built in release from `<wt>/kf2`, runs RF-LARGE-CHAIN-n01000-ROT and RF-LARGE-TREE-n01000-AX in dense mode, one at a time, with the memory guard.
  - These are the model and stage map that produced K6 §8.4. K6's test E ties them to SA's entry.
  - It records the outcome (the factor's refusal, or a `NegativeEnergy`), the factor's time and the witness's time from the stage lines.
- **The suites:**
  - FK's full suite, with `FKT/s11_site_table.rs` and its one new row;
  - NI (which contains SA), SD (a caller: SD:37 and `k1_tests.rs:371`), PP (with `s11f_site_test`) and H (`--all-targets`, with `k6_alloc`);
  - all against the Mac main baseline;
  - `gen_k4_vectors.py --check`, expected unaffected.

## 9. Mutants

Each mutant gets a clean `git archive` copy with one edit, under `<wt>/kf2-mut/<id>/`, and its own target, deleted afterwards. The NONE control runs first. They run one at a time (this brief allows one cargo job at `-j 4`), with the KF2 module and FK's existing witness tests (`krev04`, `k1_negative_witness_*`) and FK's site table.

| ID | Mutation | Expected kill |
|---|---|---|
| NONE | none | all pass |
| M1 (brief 1) | skip the first pair | T3: a witness at the first pair |
| M2 (brief 2) | the four cells in reverse order, (i,i), (i,j), (j,i), (j,j) | T3: the rounding scan family, with a verdict flip ensured by T3's self-check; and a corrupted pair whose (j,j) cell fails in `radix_scale` while its (i,i) cell fails in `checked_product`, so the first error's message depends on the order |
| M3 (brief 3) | the prefilter's allowance with `3 * (terms + 1) + 2` (too large) | T3: the tie's "one below" side is missed |
| M3′ | `3 * terms + 1` (too small): result-equivalent under §3 by design | T4: `verified == 1` on a non-witness edge pair |
| M4 (brief 4) | return the last witness (keep searching) | T3: two or more witnesses |
| M5 (brief 5) | skip zero-coupling pairs, the sparse witness's skip. It is equivalent on every built system (§4) | T3 corrupted: a witness or an overflow on a zero-coupling pair |
| M6 | sign rule `>= 0.0` → `> 0.0` | T3 corrupted: a zero-coupling witness's direction sign |
| M7 | the verdict `<` → `<=`: result-equivalent (V decides) | T4: the exact tie (verified 1, no witness) and the magnitude-0 pair |
| M8 | drop the pre-loop `validate_prepared` | T3 corrupted shape: a panic or a different `Err` |
| M9 | the prefilter is always true (today's O(n⁴) search restored): result-equivalent | T4 and T5 counts |
| M10 | the cell zero test removed (every cell evaluated) | T3 corrupted: a nonzero source coupling behind a zero `prepared.matrix` entry (the terms count) |

All must be killed. **A surviving mutant is a stop.** For M3′, M7 and M9, the kill is by count, because §3 makes a too-permissive guard change no result. That is the design's safety property, and the count tests make it observable.

## 10. The site table

**The site table needs one row.**
- The new function holds six scanned matches:
  - `visited += 1`, `terms += 1`, `evaluated += terms` and `verified += 1`;
  - the self-assignment folds `energy = checked_value(energy + ..)` and `magnitude = checked_value(magnitude + ..)` (`FKT/s11_site_table.rs:439-560`).
- Unlisted accumulations fail the test (`:599-613`).
- **The declared, additive row,** after `verify_negative_direction`'s row (`:164`):
  - `("FK/structural.rs", "negative_pair_witness_counted", 6, "KF2: integer: visited-pair, evaluated-term and verification counts and the term count; energy and magnitude sums of a pair direction (stiffness quadratic form), as verify_negative_direction")`.
- **No other row changes:**
  - `negative_pair_witness` keeps no match;
  - `verify_negative_direction` (3) is unchanged;
  - PP's site test has no FK rule-8 scope (`RULE8_FILES`, `:570-579`), and rule 6 sees no force token (§3).

## 11. Budget, cancellation, and what part 2 should show

- **No budget or cancellation check is needed inside the witness.** At O(n²) it predicts about 1 s at 6,006 DOFs and about 2 s at K6's 8,190-DOF ceiling. That is below the dense factor, whose O(n³) takes 65–86 s at 1,000 members and 166–188 s at the ceiling (K6 §8.5).
  - A check there would also be unobservable: no cancellation token reaches FK.
- **Where the product's dense route observes cancellation:**
  - only at two cooperative checkpoints in the desktop app:
    - before the solver starts (`APP:1688-1692`);
    - before publication, where the result is discarded (`APP:1711-1717`);
  - `APP:1587` sets `cancellation_scope = "cooperative_checkpoints_not_preemptive"`;
  - the solve itself (`APP:1697-1699` → PP → SA's `solve_assembled_with_formation_check`, `SA:688-725` → `solve_prepared`, `SA:2005-2020` → the witness at `SA:2017`) runs to completion on its thread;
  - `cancel_solve_job` (`APP:1753-1797`) only marks `cancellation_requested_awaiting_cooperative_checkpoint`;
  - the headless runner records `cancellation_requested: false` throughout (for example at `P/core/runner/headless/src/lib.rs:831-832`).
  - **So today a cancelled N10 dense job keeps its thread and about 3.4 GB for weeks.** After KF2 it reaches the publication checkpoint in about factor time.
- **Proposed, not in KF2:** an owner-facing note. Dense scrutiny's cancel takes effect only after the solve. The longest remaining step is the dense factor (up to about 3 minutes at the ceiling on this Mac). A cooperative checkpoint in the factor would be a product change for its own slice.
- **Part 2's expectation (B):** each of the four N10 runs ends after prepare, the factor and about 1 s, with the factor's refusal (`NumericallyUnresolved`, DOF 6001 or 6002). The matrices are positive definite, so the witness finds no pair.
  - The refusal is a pivot refusal, not a range trigger, so F1b's W2 attempt (PP:1391-1398) should not engage.
  - **Any later product step on these models is reached for the first time at B.** If one runs long, it is a new finding (stop and report), not a KF2 claim.

## 12. The write set, the host and the order at A

**The files:**
- **FKS:** `negative_pair_witness`, a delegation with its doc comment kept, and `negative_pair_witness_counted`, new, private;
- **FKT/s11_site_table.rs:** one row (§10);
- **the in-crate test file** (Q1);
- **`T3/IMPLEMENTATION/KF2/`.**

No FKP, NI, SA, SD, PP or H edit, and no screen change.

**The host at A** (after ROOT's word):
- `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0` and `CARGO_INCREMENTAL=0`;
- `--offline --locked`, one cargo job at `-j 4`, `RUST_TEST_THREADS=2`;
- target `<wt>/kf2-target`, the memory guard running;
- rustfmt (stable) on the changed files only;
- no dense materialization at 10,000 members or more (the largest dense system here is T6's 6,012 total DOFs).

**The order at A:**
1. the compile;
2. T1–T5 and the site table;
3. FK's full suite;
4. T6 and T7 in release;
5. the NI, SD, PP and H suites;
6. the mutants;
7. the optional §7.2 probe, if ROOT asks.

## 13. Questions and positions for ROOT

- **Q1, the test location.** The tests need private access, so they must be in-crate. My position is (a):
  - **(a)** a new file `FK/structural/kf2_witness_tests.rs`, with one `#[cfg(test)] mod kf2_witness_tests;` line in FKS beside `s11f_tests` and `s11k_tests` (FKS:2651-2654). The line is test-only, compiled out of every product build, and stripped by both site scanners. It is one line of FKS outside `negative_pair_witness`, so it needs your word.
  - **(b)** Put the tests in `FK/structural/sparse/tests.rs`, next to K1's witness tests. That needs no FKS line, but the brief ties that file to a shared helper, which I do not propose.
- **Q2, N10's two models.** My position is to use H's unchanged `k6_observe` (T7) rather than a new FK example or ignored test.
  - FK cannot build SA's route for these models without H, or without duplicating H's builders and SA's formation, contributions and symmetry evidence.
  - `k6_observe` is the tool behind K6 §8.4, its stages equal SA's entry, and it is not in CI.
- **Q3, the screen.** It is diagnosed and proposed with a proof (§7). My position is that ROOT declines it for KF2 and splits it out, with an owner-facing note. KF2 stays byte-identical.
- **Q4, the budget.** None, in KF2 or elsewhere for the witness (§11). The dense factor's cancellation note is for the owner.
- **Q5, the site-table row** named in §10. Please authorize it.
- **Q6, SD as an additional caller** (SD:37, and SD's `k1_tests.rs:371`). It is not named in the brief. The signature is unchanged, and SD's suite is added at A.
- **Q7, the helper.** The brief's "one private helper for the pair evaluation" is realized as `negative_pair_witness_counted`, which holds the loop, the counts and the pair evaluation inline. That keeps one helper and one site-table row. Please confirm this reading.

**Stops at A,** as the brief lists them:
- a result difference in T3;
- a committed-byte or T9 change;
- a part-1 envelope change;
- an edit outside the write set;
- a surviving mutant;
- a SIGKILL from the memory guard, reported with the log and never retried blindly.
