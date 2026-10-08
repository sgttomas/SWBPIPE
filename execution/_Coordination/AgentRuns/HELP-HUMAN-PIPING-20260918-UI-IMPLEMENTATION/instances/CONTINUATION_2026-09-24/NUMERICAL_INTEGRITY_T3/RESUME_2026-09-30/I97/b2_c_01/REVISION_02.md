# I97 B2-C revision 02: option (ii), and RV118's addendum

TASK (Type 2), I97, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-08 UTC.

**The brief:** `R/BRIEFS/B2C_REVISION_02.md`, sha256 `d82dba65f45a01aecf46b5ce8c86bc8f456cd0ad7d77e907b125741850b80c56`, verified before reading. It is committed at NUM `592f487abe`, which is NUM's head.

**The basis, verified before reading:**

| Record | sha256 |
|---|---|
| My `REVISION_01.md` | `6f6a583f601fa4f6111001196eeb1488a71f674547ff12bb109fc7cddcea38a1` |
| RV115's `R/REVIEW_RV115/b2_kd_01/ADDENDUM_03.md` | `4bd3e234eeb16f36e7bb96c98725a8537379b3c4be47ba6d2270d4983505dc63` |
| RV118's `R/REVIEW_RV118/b2_c_01/ADDENDUM_01.md` | `774b06bdc21e0d7f999501894b8898caf70eccf850493882e3f1b906fb1bd164` |

The specification is two RR sections:
- "RV115 confirms S-4 (a)'s soundness with SA3-1; the combination's displacement magnitude becomes one exact rounding";
- "RV118 confirms B2-C revision 01; the exact rounding is formed in the kernel's projection (B2-K); I97 writes revision 02".

**What this record does.**
- **Sealed files are untouched:** CONTRACT.md, REVISION_01.md, `SHA256SUMS`, `SHA256SUMS.revision_01`, `statics/`, `statics/r1/`, `_run_records/` and `_run_records/r1/`.
- **The new files:** `statics/r2/` (the two revised statics), `_run_records/r2/` (scripts, outputs, test vectors and commands), and `SHA256SUMS.revision_02`, which covers them and this file.
- **Supersession.** Each section here names what it supersedes in REVISION_01 or CONTRACT.md. Everything else stands, with RV118's acceptance of all eight of REVISION_01 §9.2's choices.

**Notation** is CONTRACT.md's. Code is cited at NUM `592f487abe`, whose maintained tree (`P/fixtures`, `P/schemas`, `P/core`, `P/tests`, `P/apps`) equals CONTRACT.md's basis `cebff253d6` (`git diff --quiet`).

## 0. In brief

**B2-K's cost for (ii): 2.5–3.5 h, under the 4 h threshold** (§2.4). The branch reuses FK's existing exact sum and correctly rounded square root, so no new arithmetic primitive is needed.

| Item | Change | Section |
|---|---|---|
| 1. Option (ii) | A combination's `displacement_magnitude` is RN64, ties to even, of the exact 3-norm of its frozen published components. It is the same on every platform, G7 holds with a ≥ 21× margin, and availability is v0's (RV115). The reference implementation agrees with an independent oracle and with RV118's code on 32,027 triples | §1 |
| 2. A-1 | Formed in FK `final_case.rs` `ProductProofDraft::project`, for combination owners only (B2-K). B2-P keeps the observables stage. B2-K's brief gains four items | §2 |
| 3. SA3-1 | §1.1's coverage argument is restated for (ii): it is an upper bound, not availability parity | §1.4 |
| 4. NC-2 | `stages.observables` states the formula 64ε·max(\|p\|, MIN_POSITIVE) | §3 |
| 5. A-2 | S-3's parenthesis: only `scale_bits` is null | §4.1 |
| 6. A-3 | W-CB1z's full-gate PASS and `b2_c1_range_mechanics` are predicted. Where each is first observed, and what happens if it does not pass | §4.2 |
| 7. A-4 | m69's row sits away from G5c's class threshold. m50's `source_ref` names an existing CaseSource | §4.3 |
| 8. A-6 | The 19 in-domain shapes listed (r1's 18 merged the two subtraction orders) | §4.4 |
| 9. NC-3 | SC2 adds a TS `consistentNorm` test | §4.5 |
| 10. Statics | DEF-C r2, H and PTABLE r2 regenerated; SCHEMA unchanged | §5 |
| 11. Estimates | B2-K +2.5–3.5 h; B2-P unchanged | §6 |

**The statics** (`_run_records/r2/b2c_statics_r2.py`, run twice, byte-identical; its controls reproduce DEF-C r1, PTABLE r1 and the v0 J1 SCHEMA byte for byte):

| Static | sha256 | Status |
|---|---|---|
| **DEF-C r2, raw** (`statics/r2/retained_precision_prepared_combination_v1.json`, 11,165 B) | `3cebce55d1b31e0031628d7542a33a8debdfa37d2d258cb27dfbdfd1fc28db22` | was `6467c733…` |
| **DEF-C r2, H(`retained_precision_formation_v1`)** | `d3fde142aff9c05d709b2fc2a04add42e14c66be3e2b2ba82012da57edf3d957` | was `0c43cf42…` |
| **PTABLE r2** (`statics/r2/semantic_contract_v0_3_preview_physics_retained_1.json`) | `b2b4a54d610aa38c66f5d31921c2d8f3113313e33eb6933e45093ba6f1e3667c` | was `791c0a0d…` |
| **The J1 SCHEMA text** | `abf3225ca431342dd785072a1baad7715b7e8c19afd15b5777feebf06d48669e` | **unchanged** (v0 `statics/`) |

- **J1's reviewed inputs** become SCHEMA `abf3225c…`, PTABLE `b2b4a54d…` and DEF-C `3cebce55…`.
- **The PTABLE cascade** (RV114 §6's 12 files, constants only) is `c74742ce…` → `b2b4a54d…`.
- **J1's G0 constant lists** (RS, TS) carry DEF-C's H `d3fde142…`.

## 1. Option (ii): the combination's displacement magnitude (supersedes REVISION_01 §1.1)

### 1.1 The recipe (DEF-C r2 `rows.displacement_magnitude`, verbatim)

"finish the same node's frozen global_nodal_displacement_x/y/z rows; RN64, ties to even, of the exact 3-norm sqrt(x^2+y^2+z^2) of their frozen raw values (mm): each square and their sum formed exactly, the square root rounded once to binary64, the same bits on every platform; finite only (a norm beyond binary64's range refuses the row), canonical +0, SI by the projection's mm rule; formed in the kernel's projection for combination owners only; still certify dual physical norm and the unchanged 64-epsilon combination magnitude guard (base G7 combination_magnitudes), which the observables stage checks; not H of a lane norm hull and not a nested binary64 hypot"

**Exactly:**
1. Each component x is an exact dyadic n/2^k, so each square, and S = x² + y² + z², is an exact dyadic rational.
2. p = RN64(√S): the binary64 nearest to √S at the quantum of its binade (2^(e−52), or the subnormal quantum 2^−1074 below 2^−1022). It is rounded once, with ties to even.

**r1's other clauses carry over:**
- **Finite only:** a norm whose rounding is beyond binary64's range refuses the row, which is the combination's `facade_certificate`.
- **Canonical +0:** every all-zero triple, signed zeros included, gives +0. W-CB1z pins this.
- **SI by the projection's mm rule.**
- **The certificate** against the dual physical norm.
- **The unchanged 64ε guard,** checked by the observables stage.

**Ties to even, and why it must be stated (RV118 A-5).** An exact midpoint is reachable.
- Take x = (2²⁷+1)·2⁻⁶⁰ and y = (2⁵³+2²⁷)·2⁻⁶⁰ mm, with z = 0. That is RV118's a = 2²⁶+1, b = 2²⁶, with x = (a²−b²)·2⁻⁶⁰ and y = 2ab·2⁻⁶⁰.
- The exact norm is (2⁵³+2²⁷+1)·2⁻⁶⁰, a binary64 midpoint, and p is the even neighbour `0x1.0000004000000p-7`, which equals y.
- **The same pair with z = 2⁻¹⁰⁷⁴ must round up,** to `0x1.0000004000001p-7`. Here this host's nested `hypot` gives the even neighbour, and so would any method that rounds S to 1,024 bits before the root: S − m² = 2⁻²¹⁴⁸ lies far below.
- So an implementation must decide the side of the midpoint exactly (§2.2).
- **Subnormal results never tie.** Every component is a multiple of 2⁻¹⁰⁷⁴, so S is a multiple of 2⁻²¹⁴⁸. A subnormal-range midpoint m = (2j+1)·2⁻¹⁰⁷⁵ has m² = (2j+1)²·2⁻²¹⁵⁰, which is not such a multiple.

### 1.2 Why G7's guard holds by construction

**The guard** (RE `guarded`, PY `_consistent_norm`, TS `consistentNorm`) is |p − r| ≤ 64ε·max(|p|, MIN_POSITIVE), where r = hypot(hypot(x,y),z) of the same `value` bits, in the reader's library.
- p is within ½ ulp of the exact norm, and a faithful nested r within 2 ulps, so |p − r| ≤ 2.5 ulps (RV115), against an allowance of 64 ulps: **a margin of about 25×**.
- The inputs are the same: the recipe reads the frozen raw values the guard reads (NC-4).

**Measured** (`_run_records/r2/b2c_checks_r2.out.json` §studies). r is this host's `math.hypot`, or any variant with each call moved by up to 1 ulp, a superset of faithful:

| Triples | Guard pairs | Failures | Largest \|p − r\| | Largest share of the allowance |
|---|---|---|---|---|
| RV118's 20,000 (its seed 20261008) | 88,350 | 0 | 3 ulps of p | 3/64 |
| RV115's 9,000 published-component triples (seed 115003, replayed) | 43,000 | 0 | 3 ulps | 0.0469 |
| RV115's 3,000 guard triples (replayed; its worst triple is among them) | 13,145 | 0 | 3 ulps | 3/64 |
| 27 curated cases | 103 | 0 | 3 ulps | 3/64 |

The model moves each call up to 1 ulp around this host's own hypot, so it can slightly exceed RV115's 2.5-ulp bound for a faithful library. Its 3-ulp maximum equals RV118's measurement, and still leaves a margin of at least 21×.

### 1.3 The same bits on every platform; availability back to v0's

**Platform-independent.** p is a mathematical function of the three published bit patterns. Any correct implementation gives the same bits.
- FK's implementation (§2) uses only `Wide` and `ExactWideSum` arithmetic, which are integer-only and "bitwise reproducible on every platform" (`wide/multi.rs`).
- The retained route's published bits no longer depend on the producer platform's `f64::hypot`. Under r1, this host's nested hypot differed from p on 4–13 % of triples (`this_libm_nested_hypot_differs_from_p`: 1,082 of 20,000, 1,147 of 9,000 and 115 of 3,000).

**Availability (SA3-1; RV115's study, point enclosure).** These are failures per 3,000 rows at S* = n:

| Rows | v0 | (ii) |
|---|---|---|
| Comparable | 378 | 379 |
| One small component | 314 | 315 |
| Planar | 273 | 292 |

- At S* = 8192n, both fail none.
- So (ii) restores v0's availability for comparable and one-small rows, and is within 0.6 percentage points for planar rows.
- r1's figures were 487–505 with a correctly rounded hypot and up to 2,995 with an adversarial faithful one.

### 1.4 The certificate's coverage, restated (SA3-1)

**The bound.** The magnitude row is certified as every row is: the published value against the hull of the two lanes' norm enclosures, by `check_intervals` and `gate`. A refusal is the combination's `facade_certificate`. RV115 checked the bound in 9,000 of 9,000 trials:

h_mag ≤ √3·max_k h_k + |p − ‖x̂‖| + the SI rounding.

**Under (ii),** |p − ‖x̂‖| ≤ ½ ulp(p): one rounding, where r1 had two hypot roundings.

**This is an upper bound on the row's distance from the truth, not a pass guarantee.** The relative class's allowance has one round-to-nearest term (2⁻⁵³·|n|) besides its scale terms. So a row can still fail while its components pass. Under (ii) that happens about as often as under v0's own recipe (§1.3), not more.

**The two routes now form displacement magnitudes differently** (RV118 §6). The ordinary route keeps PP's nested hypot of the combined components (`lib.rs` `append_combined_vector_magnitude`, `:13328`), which passes G7 as today. The retained route forms (ii). Support magnitudes stay one rule on both routes: DEF-C's `support_magnitude` is DEF-O's, byte for byte.

### 1.5 The reference implementation (`_run_records/r2/b2c_checks_r2.py` `rn64_norm3`)

**The algorithm.** It uses integers only, with VENV's Python.
1. Write each component as n/2^k, and form N = Σ(n·2^(k_max−k))², so S = N/2^(2k_max).
2. Take e = ⌊log₂√S⌋ from `math.isqrt(N)`'s bit length.
3. With the quantum q = max(e − 52, −1074), form ⌊2√S/2^q⌋ by `math.isqrt` of an exactly scaled 4N. A sticky bit is kept from the root's remainder and from any bits shifted out.
4. Round once, ties to even. A result of 2^1024 or more is refused.

**The checks:**

| Check | Result |
|---|---|
| An independent oracle (Fraction midpoints: mid_lo² ≤ S ≤ mid_hi², even at equality, the overflow threshold (MAX + ½ulp(MAX))²) | Agrees on all 32,027 triples |
| RV118's own `rn64_sqrt`, imported unchanged from its record | Agrees bit for bit, tie flag included, on all 32,027 |
| A-5's midpoint | Equals RV118's `tie_example` (`0x1.0000004000000p-7`, tie) |

**The curated cases** include:
- the midpoint plus 2⁻¹⁰⁷⁴ and plus 2⁻⁶⁰⁰, which round up;
- the midpoint with one component an ulp lower, which rounds down;
- the eight signed-zero triples, all +0;
- the smallest subnormal, alone and three times;
- the largest subnormal three times;
- `MAX, 0, 0` → MAX;
- `MAX, MAX, 0` → refused;
- 2¹⁰²³ twice → finite;
- 1e308 three times, where this host's libm is one ulp off;
- 1e±200 triples, where an unscaled hypot would overflow or underflow;
- 1e300, 1e−300 and 2⁻¹⁰⁷⁴.

**`_run_records/r2/exact_norm_vectors.json`** holds 411 vectors: every curated case, 256 of RV118's triples and 128 of RV115's. Each gives the input and output binary64 bit patterns and the tie flag, for B2-K's tests and K-09's diagnose.

## 2. A-1: where (ii) is formed, and B2-K's cost (supersedes REVISION_01 §8's "B2-K is unchanged")

### 2.1 The site

**FK `final_case.rs` `ProductProofDraft::project`, for combination owners only, in lane K (B2-K).**
- `certify_final` refuses any row whose value bits differ from those the projection froze (`:1884–1894`, read by ROOT).
- PP supplies bits only for record rows (`ProductRowSpec.observed`).
- So PP can neither form nor replace a quantity row's value. B2-P keeps the observables stage (REVISION_01 §1.2), which only checks the guard.

### 2.2 The branch (KD §5.7 gains it)

**Pass 1.** `project` hull-projects every `Native` recipe, `DisplacementMagnitude` included (`:1799–1830`), and skips support magnitudes. For a combination owner (`product_owner` gives `NativeOwner::Combination`, KD §1.5), it also skips `Native(QuantityId::DisplacementMagnitude(node))`.

**Pass 2.** Beside `support_hypot` (`:1831–1853`), a new helper (suggested `displacement_norm`) forms each such row:
1. **The components.** Find the node's three `Native(QuantityId::Displacement(Dof))` translation specs: exactly one each, unit `Millimetre`, finite projected values. These are counted visits, as `support_hypot`'s are.
2. **The exact sum.** S = x·x + y·y + z·z in one `ExactWideSum`, by `add_product` three times. It is exact. `wide_sum.rs` bounds any sum of binary64 products at 6,244 bits, and a subnormal midpoint's square (step 4) reaches only two bits lower, far inside the 8,128-bit limit. S = 0 publishes +0.
3. **A nearest estimate.** y₀ = `to_binary64`(`WideContext::<16>::sqrt`(`sum.round`)). An `Overflow` outcome refuses the row ("finite only"). A `Subnormal` outcome is used as is.
4. **The exact side, decided once.**
   - Form the midpoints adjacent to y₀ exactly in `Wide`: y₀ ± half its quantum, the lower one half as far at a binade's first value, and ±2⁻¹⁰⁷⁵ for subnormals.
   - Take `signum`(S − m²) for each, again by `ExactWideSum` (`add_product` of x, y, z and of m, m negated).
   - If S lies beyond a midpoint, step y₀ to that neighbour. At equality, take the even one.
   - y₀ is within one step, because a 1,024-bit estimate can be wrong only within about 2⁻¹⁰²² of a midpoint. This is `directed::certificate::sqrt_owned`'s own pattern, "one nearest sqrt and the exact q·q − a side".
5. **The result** is canonical +0 at zero. The helper's `WideContext` and sum work are merged into the numeric work as the `sqrt` helper does (`w.wide.merge`, `w.sums.merge`), and it adds a counted `scalar_operations` entry as `support_hypot` does.

**What it reuses,** with no new arithmetic primitive:
- `ExactWideSum::add_product`, `signum` and `round` (`wide_sum.rs`, K4);
- `WideContext::sqrt` and `Wide::to_binary64`, each rounded once, ties to even (`wide/multi.rs`, K3);
- the pattern of `directed::certificate::sqrt_owned` and the row search of `support_hypot`.

**What does not change:**
- `certify_final`, `check_intervals`, `gate` and every case owner's path;
- `support_hypot`, used by cases and combinations alike;
- PP.

**The S11 site table.** The helper adds no `+=`, `.sum(` or `fold(` shape, using `WorkTotal::add` and `checked_add` as the existing code does, so it needs no new S11 row. If B2-K does add a counted fold, the row comes with it (S-12).

**The file list.** `final_case.rs` and the FKT tests are in S-13's list, so no stop fires.

### 2.3 B2-K's brief gains

1. **The branch, in KD §5.7** (§2.2): "`project`: for a combination owner, `DisplacementMagnitude` rows are formed in the second pass as RN64 of the exact 3-norm of the node's projected translations (DEF-C r2), not hull-projected."
2. **K-09's diagnose check.** For every combination displacement-magnitude row, the published bits equal RN64, ties to even, of the exact 3-norm of the published component bits.
   - The oracle computes it with `math.isqrt`, independently of production, as `rn64_norm3` does.
   - It also runs `exact_norm_vectors.json`, which includes A-5's midpoint and the midpoint plus 2⁻¹⁰⁷⁴.
3. **K-13's byte identity for case displacement magnitudes.** Every existing case-path test passes unchanged, with identical printed work counts. The branch never runs for a case owner.
4. **RV-K's review** of the branch as kernel code (RV115).
5. **Unit tests in FKT `product_final_case_tests.rs`:**
   - the vectors, including a refusal and the signed zeros;
   - a combination owner gets (ii), a case owner the hull projection;
   - a missing, duplicated or non-mm component is refused.

### 2.4 The cost in B2-K: 2.5–3.5 h, under the 4 h threshold

| Item | Agent hours |
|---|---|
| The pass-1 skip and the pass-2 helper (rows, exact sum, estimate, midpoint side, work merge) | 1–1.5 |
| Unit tests from the vectors, and the owner branch | 1–1.5 |
| KD §5.7's text and K-09's diagnose clause | 0.5 |
| **Total** | **2.5–3.5** |

**Why it is small.** Every arithmetic step exists, and the sole subtle part is the midpoint side, whose pattern `sqrt_owned` already has. The runs ride B2-K's existing ones (KD §9.2). RV-K's code round gains about 0.5 h.

## 3. NC-2: `stages.observables` (DEF-C r2, verbatim)

"base G7 combination_magnitudes on the frozen rows: one support-action row per support and component, and each displacement, force and moment magnitude p with |p-r| <= 64*eps*max(|p|,MIN_POSITIVE), r the binary64 hypot(hypot(x,y),z) of the same combination's published components; the case support coverage and guard; no preview case evidence, maximum or headline check; a failure is this combination's facade_certificate"

The stage's order and failure mapping are unchanged (REVISION_01 §1.2), as RV118 confirmed.

**DEF-C r2 against r1:** exactly two paths change, `rows.displacement_magnitude` and `stages.observables`. In addition:
- `support_magnitude` still equals DEF-O's;
- `operand_definition.sha256` is still H(DEF-O) `a7ed7ca0…`;
- the bytes are their own canonical form, ASCII, with no float.

**PTABLE r2 against r1:** only `product_formation_definitions` changes (DEF-C's H). N-7's scope text is r1's.

## 4. RV118's NOTEs and RV115's NC-3

### 4.1 A-2 (supersedes the parenthesis in REVISION_01 §2)

"The entry's other members follow a selected case's row of the same class: **`scale_bits` null, and `normalized_bits` from the row's value**, as for a selected case's record row." `normalized_bits` is never null. It is RS's `u64` from the row's value (`b1-r` `retained_precision.rs:36`, `:3348`).

### 4.2 A-3: two predicted must-pass bases (amends REVISION_01 §5.1)

| Base | Predicted | First observed | If it does not pass |
|---|---|---|---|
| **W-CB1z** (A + B, every combination row exactly 0) | Its full-gate PASS. I98 observed only the proxy, stopped by the case-only D6b. G5a–G5c's S = 0 path and G6–G8 on an all-zero combination are unobserved. Under (ii) its magnitudes are +0 | **B2-P's producer pin** on `r7_cb1.json` in both modes, through precommit with B2's RS reader. Then **SC2's 07o must-pass** under PY and TS | **Producer side** (not selected, or a certificate, observables or precommit refusal): the pin records the actual outcome, the base keeps its "cancellation pin" label with the observed disposition, its must-pass is restated under decision 20, and ROOT is told, because NB-3's cancelled-net property is then pinned on another disposition. **Reader side** (one reader refuses a successor the others accept): a reader defect for that lane, which SC2 declares under decision 20 |
| **`b2_c1_range_mechanics`** (the milestone with `[range(case), 2·case]`) | No probe ran it. 2·case's ledger is the case's scaled exactly by 2, so 2·case is predicted `retained_selected` | **B2-P's producer pin**, both modes: T-6′'s layout custody, then 2·case's freeze. Then SC2's must-pass | **If 2·case is not selected or not certified,** it becomes `retained_unavailable`. The base still pins S-2's layout and still serves m18 and m19 (two entries), and SC2 restates its must-pass (decision 20). **If T-6′'s custody refuses the layout,** that is a B2-P defect against REVISION_01 §3.2, fixed before SC2 |

### 4.3 A-4: m69 and m50 (amends REVISION_01 §5.3's rows)

**m69:** "One combination `displacement_magnitude` value × (1 + 2⁻⁴⁰), at a node whose magnitude n is a relative-class row of ordinary size, 2⁻²⁰·S ≤ |n| < max, where S is that combination's scale."
- Not the largest, so G5b's scale is unchanged.
- Far from G5c's class threshold |n| = 2⁻³⁴·S, so G5c's `absolute_verified` list is unchanged.
- The edit exceeds the 64ε allowance by at least 63.7× for every normal magnitude, under (ii) as under r1 (RV118).
- The designed failure stays G7, `COMBINATION_MAGNITUDE`.

**m50:** "the refused record's `source_ref` → 0, case A's existing CaseSource."
- G3's strict index then resolves.
- G3 (d) and (g) read prepared records only (REVISION_01 §4.3), so the edit reaches G5 `PRODUCT_ATTEMPT_MISMATCH`.

### 4.4 A-6: the in-domain shapes (amends REVISION_01 §3.2's count)

**There are 19,** listed by `b2c_checks_r2.out.json` §a6_shapes on the same append-order model. Revision 01's rule holds for every one, and v0's fails exactly the two marked:
- **c = 1, z = 1 (3):**
  - mechanics with distinct cases;
  - mechanics with a repeated case (C-1, rowless);
  - range(A).
- **c = 1, z = 2 (9):** each ordered pair of those three. **v0 fails** `range(A); mechanics, distinct cases` (`[range(A), 2·A]`) and `range(A); range(A)`.
- **c = 2, z = 1 (7):**
  - mechanics with distinct cases;
  - mechanics with a repeated case;
  - range(A), range(B) and range(A,B);
  - subtraction A−B and subtraction B−A.

**Why the counts differ.** r1's 18 counted subtraction once, since its operand order does not change the layout. RV118's 19 counts both orders. Range operand ids are sorted by the producer, so range(B,A) is range(A,B).

### 4.5 NC-3: SC2 adds a TS test (amends REVISION_01 §5.2's harness items)

**The test:** TS gains a unit test of `consistentNorm` (`previewPhysicsEvidence.ts`) on adversarial and subnormal triples, taken from `exact_norm_vectors.json`:
- A-5's midpoint and its plus-2⁻¹⁰⁷⁴ variant;
- the subnormal cases;
- the 1e±200 triples, where an unscaled hypot would overflow or underflow;
- 1e308 three times.

**What it asserts:**
- `consistentNorm` accepts p, the vector's RN64 value;
- it refuses p·(1 + 2⁻⁴⁰).

**Where it runs:** on every JavaScript engine that CI uses, which is Node's V8 for vitest. The shipped app's webview engines (JavaScriptCore; V8 through WebView2) are not in CI. That limit is recorded (RV115 NC-3 (a)). Flush-to-zero and x87 modes are not targets.

## 5. The statics, regenerated

`_run_records/r2/b2c_statics_r2.py` imports the sealed v0 `b2c_statics.py`, the sealed r1 `b2c_statics_r1.py` and I96's generators, all unchanged.
- **Its controls,** each asserted before anything is written:
  - DEF-C r1 is rebuilt from v0's `combination_definition` and r1's `definition_r1`, and equals the sealed `statics/r1/` DEF-C;
  - PTABLE r1 is rebuilt by v0's `ptable_revision` with r1's scope text, and equals the sealed `statics/r1/` PTABLE;
  - the J1 SCHEMA, by v0's `apply_j1`, equals the sealed v0 `statics/` text.
- **It then applies** §1.1's and §3's texts, and writes DEF-C r2 and PTABLE r2.
- **The two runs are byte-identical** (`diff -r`).

**SCHEMA does not move.** It carries no definition or table hash (checked: none of DEF-O's, r1's or r2's H occurs in it), and (ii) changes no id or member.

## 6. The estimates, restated (supersedes REVISION_01 §8)

Agent hours, without repair rounds.

| Slice | r1 | **r2** | What moves it |
|---|---|---|---|
| **B2-K** | 19–30 ("unchanged") | **21.5–33.5** | (ii)'s formation, its tests and K-09's clause (§2.4) |
| B2-A | 5.5–8.5 | 5.5–8.5 | — |
| B2-P | 28–40 | **28–40** | Unchanged. It keeps the observables stage. The recipe was never PP's, so B2-P loses nothing |
| B2 readers | 41–58 | 41–58 | — |
| SC2, B2's part | 9–13 | **9.5–13.5** | NC-3's TS test (0.25–0.5) |
| J1's package | 1–2 | 1–2 | Only the hashes move |

**Net:** about +3–4 h on revision 01. **Review:** RV115 confirms (ii)'s numerics and RV118 confirms this revision, about 1–2 h, and RV-K's code round gains about 0.5 h.

## 7. For ROOT

1. **B2-K's cost for (ii) is 2.5–3.5 h, under the 4 h threshold,** so I wrote the rest. (ii) stands as ruled.
2. **The confirmations RR orders:**
   - RV115 confirms (ii)'s numerics: DEF-C r2, H `d3fde142…`, §1, and the reference implementation;
   - RV118 confirms this revision.

   B2-C is then final for J1, with SCHEMA `abf3225c…`, PTABLE `b2b4a54d…` and DEF-C `3cebce55…`.
3. **One fact for B2-K's brief.** The midpoint side must be decided exactly. A 1,024-bit rounding of S before the root misses A-5's midpoint-plus-2⁻¹⁰⁷⁴ vector, where this host's libm also rounds the wrong way (§1.1). FK's `sqrt_owned` pattern does it.
4. **Routed elsewhere, not mine:** RV115's NC-1 (DEF-O's mm→SI double rounding) and the same property in support magnitudes, which SQ measures (RR).
5. **Nothing here needs an owner decision.** Public meaning before B8 is unchanged.

## 8. Execution record and limits

**Executed** with VENV's Python 3.13.14 only (`-B`, `PYTHONDONTWRITEBYTECODE=1`, `TMPDIR` in scratch). No host `python3`, cargo, install or Git write was used. `_run_records/r2/RUN_R2.md` has the commands with placeholders.
- `b2c_statics_r2.py`, twice, byte-identical;
- `b2c_checks_r2.py`, twice: the output and the vectors byte-identical.
- **It imports:**
  - my sealed r1 `b2c_checks_r1.py`, for the layout model;
  - RV118's sealed `rv118_a01_option_ii.py`, unchanged and by path, for the cross-check.
- **It replays** RV115's generator from its seed, copied with attribution.

**Read:**
- the brief; the three basis records in full; RR's two sections;
- FK `final_case.rs` (`project`, `support_hypot`, `project_hull`, `certify_final`, `sqrt`, `norm2`), `wide_sum.rs` (header and API), `wide/multi.rs` (header and API), `directed/certificate.rs` (`sqrt_endpoint`, `sqrt_owned`) and `recover.rs` (`QuantityId`);
- KD §5.7, §6, §7 (K-09, K-13), §9.1 (S-12, S-13) and §9.2;
- RV115's and RV118's scripts.

**Limits:**
- **Nothing was compiled or run in Rust or TS.** §2's site and cost are code readings.
- **The guard study models faithful libraries** as ±1 ulp per call around this host's `math.hypot`.
- **The availability figures are RV115's model,** not re-derived here.
