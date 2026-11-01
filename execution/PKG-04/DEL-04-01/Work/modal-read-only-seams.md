# Read-only seams for DEL-04-09 (modal analysis)

DEL-04-09's slices add two read-only seams to DEL-04-01's code. DEL-04-09's design governs their use (`execution/PKG-04/DEL-04-09/Work/modal-analysis.md` §3.3 and §3.4). This record lets DEL-04-01's owner see what was added and why. It changes no DEL-04-01 behaviour and asserts no acceptance. Standard claim fence applies (claims taxonomy per DEC-081).

## `sparse_direct`: the factorization observation accessor
- **Added:** `ProfileFactorization::lower_factor_entries()` and `ProfileFactorization::pivots()`. Both are borrowing iterators over the strictly lower factor's stored entries, row by row in ordered position, and over the pivots d_j.
- **Guarantees:** no mutation and no allocation. Tests show solve results bit-identical before and after iterating, and zero allocations under a counting allocator.
- **Consumer:** DEL-04-09's Sturm-count premises. They need the componentwise backward-error bound γ_n‖|L||D||Lᵀ|‖, the growth factor and the negative-pivot count.

## `frame_kernel`: the element re-formation seam
- **Added:** `structural::element_reformation` with `reform_straight`, `reform_bend`, `reform_connector` and `reform_tie_rod`. They re-form element stiffness at K-D5's `FORMATION_PRECISION` as a pure function of caller-supplied double-double operands: section properties, moduli and flexibility factors. There is no D5C1 judgment and no state.
- **What changed in existing code:** `frame_matrix` and `curved_matrix` now read their section values through two small traits. For the existing binary64 types each value is the exact lift in the original order, so K-D5's results are bit-for-bit unchanged. The frame_kernel library suite (732 tests) passes, and the seam matches K-D5 bit for bit for a straight (Euler–Bernoulli and Timoshenko), a bend, a connector and a tie rod with the same operands.
- **Consumer:** DEL-04-09's modal formation check (check 1), which supplies section properties it re-derives itself from OD and wall.

## Pricing
Both crates are priced (`tools/retained_pricing/base/crate_dirs.txt`). The change that adds the seams re-registers D3 under DEL-04-08's rules and re-runs the AC-004 stack witnesses if D3 moves them.
