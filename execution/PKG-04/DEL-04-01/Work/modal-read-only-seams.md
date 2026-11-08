# Read-only seams for DEL-04-09 (modal analysis)

DEL-04-09 uses two read-only seams in DEL-04-01's code. Its design governs their use (`execution/PKG-04/DEL-04-09/Work/modal-analysis.md` §3.3 and §3.4). Standard claim fence applies (claims taxonomy per DEC-081).

## `sparse_direct`: the factorization observation accessor
- **Interface:** `ProfileFactorization::lower_factor_entries()` and `ProfileFactorization::pivots()` are borrowing iterators over the strictly lower factor's stored entries, row by row in ordered position, and over the pivots d_j.
- **Guarantees:** no mutation and no allocation. Tests show solve results bit-identical before and after iterating, and zero allocations under a counting allocator.
- **Consumer:** DEL-04-09's Sturm-count premises. They need the componentwise backward-error bound γ_n‖|L||D||Lᵀ|‖, the growth factor and the negative-pivot count.

## `frame_kernel`: the element re-formation seam
- **Interface:** `structural::element_reformation` provides `reform_straight`, `reform_bend`, `reform_connector` and `reform_tie_rod`. They re-form element stiffness at K-D5's `FORMATION_PRECISION` as a pure function of caller-supplied double-double operands: section properties, moduli and flexibility factors. There is no D5C1 judgment and no state.
- **Compatibility:** `frame_matrix` and `curved_matrix` read their section values through two traits. For the existing binary64 types each value is the exact lift in the original order, so K-D5's results are bit-for-bit unchanged. At implementation the frame_kernel library suite (732 tests) passed, and the seam matched K-D5 bit for bit for a straight (Euler–Bernoulli and Timoshenko), a bend, a connector and a tie rod with the same operands.
- **Consumer:** DEL-04-09's modal formation check (check 1), which supplies section properties it re-derives itself from OD and wall.

## Pricing
Both crates are priced (`tools/retained_pricing/base/crate_dirs.txt`). DEL-04-08 Work §8.39 records the source-bound seam proof: all 1,602 coefficients remain equal to integrated reducer2a, so the registered generated block stands. The changed debug wrapper/accessor depth received six passing AC-004 source-bound headroom witnesses on 2026-11-02; §8.39 records the exact source/binary binding and closure. Coefficient identity alone supplied no stack evidence.
