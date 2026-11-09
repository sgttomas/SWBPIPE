# T2.integrity: the step ledger and its replacements (DEL-04-09 Design §5.3.8)

Source: main `bf1bdd2dbf1f1a357cbdeb01d78e41e502c5bd3e`, read with `git show`; every file:line below is at that commit. "`adapter`" is `core/solver/nonlinear_integration/src/structural_adapter.rs`, and "product `lib.rs`" and "product `skew.rs`" are under `core/product_physics/src/`. Status: **proposed**. It bounds `T2.integrity` (`core/product_physics/src/modal/resource.rs`) from source under two premises, which Agent 0 adopted as pricing premises (§1.3), and five replacements that are specified here (Part II) and not built. Nothing here changes P0, adopts a weight, or authorizes an edit; P0 still refuses every budgeted request (modal-analysis.md §5.3.8). Part I is the ledger; Part II the replacements.

**Summary.** `T2.integrity` is the M03-INTEGRITY work of the static screen's solve on a zero force (`stiffness.rs:193`–`:211`). It excludes the Cholesky k-loop (`T2.factor`) and `cholesky`'s entry, scan and allocation steps (`T2.entries`, `structural.rs:2067`–`:2104`). It is bounded on both branches, under two premises and five replacements, on a target with a fused multiply-add instruction; on x86_64, `f64::mul_add` is a library routine and the bound waits on a decision (§12, item 2). On the plain path (`stiffness.rs:207`–`:209`) the charge is quadratic in n_g, plus C(3,400 + 105ℓ̄ + 2β̄) for the exact contribution audits, 350,793 per node for the rigid-body screen, and a fixed formation cost per element (§6). The restraint-basis branch (`:203`–`:205`) adds K-D5's formation check, which re-forms every element four times at 128-bit precision (≈ 7.1·10⁵ steps per element plus 4κ^F), the coordinate transforms, and the W4 constrained-body screen (§9). P0 needs these new dimensions: E_el, C, G, Z̄, δ̄, ℓ̄, β̄, λ̄_A and Λ_m on every request, and ν, s_k, Λ_s, β̂, w̄, Z_ν, U and m_A with a basis (§2). The premises are P-fp (core float and integer methods are one operation each, except those §1.3 names) and P-fmt (`format!` is one step per output byte) (§1.3). The replacements are R1 (`SparsePattern::find`), R5 (`frame_member_identities`' hash sets), R6 (`powi`) and R8 (128-bit integer divisions in the wide arithmetic) on the plain path, and R3 (eleven sorts and searches) on the basis branch. Each replacement needs an oracle test against a verbatim copy of the old code, the named suites, and `price.py` showing every coefficient equal (R5: equal or lower). R1, R3a–b, R4, R5, R6 and R8 also edit a file that `tools/retained_pricing` pins by full-file hash, so each needs a reviewed re-pin (Part II, "Full-file pins").

---

# Part I. The ledger

## 1. Unit, premises and lemmas

### 1.1 The unit

An *integrity step* is `T3.kernel`'s step, defined at `resource.rs:185`–`:197` (Design "The support terms, bounded"):

> "A kernel step is at most 64 operations. Operations are counted literally: every read of a named value (a variable, parameter, field or element), every arithmetic operation (+, −, ×, ÷, √), absolute value, negation and logical `!`, every comparison or predicate (`is_finite`, `is_nan`, `is_normal`, `&&`, `||`, a match or `?` test; `max`/`min` count a comparison and a select), and every binding or assignment (…). Excluded, and only these: loop control, index arithmetic (…), bounds checks and `debug_assert`s. Where a count was unclear it is rounded up. A body of k operations is charged ⌈k/64⌉ steps."

This ledger applies it with four readings. Each can only raise a count.
- Every loop iteration is a body, charged at least one step.
- A callee's straight-line code may be charged as its own body (⌈a⌉ + ⌈b⌉ ≥ ⌈a + b⌉).
- A fixed-size array fill (`[x; n]`) costs one step per element, and so does a copy of a fixed-size array. A copy of a struct, enum or tuple costs one step per scalar field, an array field counting one per element. This covers every copy made by dereferencing or cloning (`*matrix`, `*squares`, `*self`, `..*self`, `.clone()`, a by-value receiver called through a reference, such as `is_some_and` on `&mut Option<[f64; 3]>`), and every array or aggregate read by value out of borrowed data (`self.a_i`, `element.node_i.coordinates`, `self.node_i`, `v[1]` of a `&[Wide2; 3]`). An element of an array may itself be an aggregate (a `Wide2`, a tuple); it is one element of that array. A by-value read of an owned local, whether a move or a copy, is one read of a named value, as the Design counts it. So is a scalar field. A return is one operation.
- A straight line not counted operation by operation is charged 2 × its token count (identifiers, literals and operators), callees included ("2×tok"). One token carries at most two operations (a compound assignment, a `?`, an implicit return or a shorthand field).

### 1.2 Premises already adopted

`resource.rs:198`–`:203`, reused unchanged:

> "A core slice or `Vec` element primitive (`copy_within`, `fill`, `swap`/`swap_with_slice`, `resize`, iterator adapters, `push` with amortized growth) is one step per element (…). Allocator calls are charged in no term. `f64::sqrt` is one operation (IEEE 754's correctly rounded square root)."

This ledger reads the iterator consumers (`any`, `all`, `find`, `position`, `count`, `fold`, `sum`, `max`, `min_by`, `max_by`), slice `contains`, slice `==`, `copy_from_slice`, `String::clone`, `clone`/`to_vec` of a slice, `vec![x; n]` fills, `extend`, and drop glue per element as element primitives. The plain-path sites are listed in §11.

### 1.3 Premises adopted for this ledger

The Design's adopted text names √ and nothing else from `std`'s float or integer methods. Its arithmetic list "(+, −, ×, ÷, √)" is an enumeration. Agent 0 adopted these two as pricing premises on 2026-11-09:
- **P-fp.** Each of these is one operation: `f64::mul_add` where the target has a fused multiply-add instruction (IEEE 754 fusedMultiplyAdd, correctly rounded); `to_bits`/`from_bits`; `next_up`/`next_down`; `total_cmp`; `signum`; `clamp` (two comparisons and selects, as `max`/`min`); the integer operators `&`, `|`, `^`, `<<`, `>>`, ×, + and − on `u64`, `u128`, `i64`, `i128`, and their `from`/`as` conversions between integer types; `leading_zeros`; `is_power_of_two`; `overflowing_add`/`overflowing_sub`/`wrapping_sub`/`saturating_add`. *Rationale:* these are core float and integer methods with √'s standing, an IEEE operation or a fixed instruction sequence with no loop or library call. Plain-path uses: `structural.rs:852`, `:1838`; `correct_norm/src/lib.rs:79`, `:86`, `:96`, `:104`, `:162`; `exact_sum.rs:58`–`:66`, `:72`, `:164`, `:201`; `connector.rs:770`; `tie_rod.rs:451`; `load_ledger.rs:539`, `:617`; `rigid_body.rs:147`, `:175`. Basis-branch uses: `retained/wide.rs:314`–`:334`, `:505`–`:629`; `wide/multi.rs:707`; `restraint_basis.rs:190`, `:261`; `rigid_body.rs:583`, `:611`, `:1063`–`:1080`; `adapter :1861`.
- **Excluded from P-fp by name**, because one operation would under-count a software path:
  - `f64::powi`. `2.0_f64.powi(k)` is not an IEEE basic operation. It lowers to `__powidf2`, a library loop whose source is outside the repository, which is the standing the Design refused for `binary_search`. R6 replaces it with a bit-built power at the same bits.
  - `f64::mul_add` on a target without a fused multiply-add instruction. `std` says it "may be more performant than an unfused multiply-add if the target architecture has a dedicated fma CPU instruction". x86_64's baseline target features have none, so on x86_64 (the Windows build is `x86_64-pc-windows-msvc`, `.github/workflows/piping-windows-core.yml:64`) it lowers to the platform's `fma` routine. On aarch64 it is the base-ISA `FMADD` and stays under P-fp. §12, item 2 holds the decision.
  - `div_ceil` on `u128` (`wide/multi.rs:711`) lowers to `__udivti3`, a library division loop. `div_euclid` and `rem_euclid` on `i128` (`retained/wide.rs:818`, `wide/multi.rs:196`–`:197`, `:578`) are 128-bit divisions, `__divti3`/`__modti3` unless the optimizer folds the constant divisor. R8 replaces each with bit operations or an in-repo loop at the same bits.
- **P-fmt.** `format!` over fixed text with a bounded payload costs one step per output byte. *Rationale:* `std` documents no cost for formatting, and one step per output byte is the conservative reading, since writing each byte is at least one operation. Plain-path sites: `structural_adapter.rs:2262` (`symmetry_basis`, < 512 B); product `lib.rs:4880` (`member_prefix`, λ_p + 9 + ⌈log₁₀(λ_p + 1)⌉ B, where λ_p is the pipe's `element_id` bytes); `structural.rs:1959` (an error path, < 512 B). Basis-branch sites, all on fail-closed paths: `adapter :2286`, `:2317` (< 64 + λ_A B, where λ_A is the arc's id bytes); `formation_check.rs:443`–`:449` (< 128 B: `WideError`'s texts, `retained/wide.rs:177`–`:197`); `formation_check/skew.rs:30` (< 64 B).
- **No `HashSet` premise.** `std` documents no worst-case cost for it. R5 replaces it.

### 1.4 Lemmas

**Lemma A (ExactAccumulator carries).** Over any k `add_word` calls on one accumulator, the loop at `exact_sum.rs:70`–`:76` runs at most 4k + Φ₀ iterations. Φ counts limbs equal to `u64::MAX` across both magnitudes; Φ₀ is its value at the start.
- *Proof.* A call runs only when `add != 0` (`add_shifted` skips zero parts, `:96`). Let t be its iterations. Iteration 1 adds `add` to limb `word`. Each later iteration adds `add = u64::from(carry) = 1` (`:74`), and it runs only because the previous one carried. For iterations 2 to t − 1, adding 1 carried, so the limb was `u64::MAX` and became 0: those destroy t − 2 all-ones limbs. Only iterations 1 and t can create one, so ΔΦ ≤ 2 − (t − 2) and t ≤ 4 − ΔΦ. This holds for t = 1 (ΔΦ ≤ 1) and when `get_mut` fails at iteration t (`:71`; that limb is unchanged). Summing, Σt ≤ 4k − (Φ_end − Φ₀) ≤ 4k + Φ₀. Only `add_word` writes limbs; `subtract` (`:113`) writes a fresh array.
- *Consequences.* `new` gives Φ₀ = 0 (`:205`–`:210`). An `add` or `add_product` makes at most 3 `add_word` calls (`:95`–`:99`), so ≤ 12 carry iterations each, amortized over the accumulator's lifetime. A clone (`load_ledger.rs:536`) starts at Φ₀ ≤ 136, and every call creates at most 2, so a source that took k calls has Φ ≤ 2k.
- *Per-call worst case.* Each call stops at limb 68 (`get_mut(..).ok_or`), so at most 68 iterations. This is the fallback if the amortization is not accepted: `add` ≤ 209 and `add_product` ≤ 210 steps.

**ExactAccumulator bodies**, counted literally:
- `new` (`:205`–`:210`): two `[0; 68]` fills, so **137**.
- `add` (`:213`–`:227`): 19 operations, plus `decompose` (`:57`–`:67`) 31 and `add_shifted`'s straight line (`:81`–`:94`, `:100`) 39, so 89 operations (2 steps). Then 3 loop iterations (`:95`–`:99`) and 12 carries. That is **17 amortized**.
- `add_product` (`:231`–`:248`): 145 operations (3 steps), plus 3 iterations and 12 carries, so **18 amortized**.
- `add_product(a, 0.0)`: **2**. It returns at `mb == 0` (`:237`) after two `decompose`s (83 operations).
- `is_zero`: **69** (`compare`, `:251`–`:253`).
- `round` = `round_scaled` (`:284`–`:299`): `compare` ≤ 68, `subtract` 68 fill + 68 iterations, `project` 2 straight + `find` ≤ 68 + `window` 64 (each iteration with `bit`, ≤ 31 operations) + `any_below` ≤ 69, and 2 call bodies. That gives **ρ = 412**.
- A clone: **137**.

**Lemma B (radix loops).** `radix_scale` (`structural.rs:664`), `exact_radix` (`:924`) and `physical_residual_record` (`:1625`) run ≤ 5 iterations each.
- *Proof.* Each loop steps by `exponent.clamp(-512, 512)` (`:665`, `:927`, `:1626`) toward zero. In `exact_radix` a step that succeeds keeps the value finite, nonzero and exact (`:929`); in `radix_scale` it keeps it normal (`:668`). A nonzero finite binary64's bits lie within positions [−1074, 1023], and 5·512 > 2,097, so at most 4 full steps succeed. The next iteration either takes the remaining partial step, which ends the loop, or returns `Err`.
- `physical_residual_record` has no zero test. Its exponent is a row exponent built from `binary_exponent` ∈ [−1074, 1023] (`:674`–`:682`), so it lies in [−2147, 2047], and ⌈2,147/512⌉ = 5.
- A call is ≤ **6 steps**: the prologue, plus 5 bodies of ≤ 30 operations each.

**Lemma C (Expansion).**
- `Expansion::add` (`structural.rs:808`–`:834`) pushes at most one `low` per old term plus the final q, so it returns ≤ ℓ + 1 terms. After k adds the length is ≤ min(k, X̄), with X̄ = 2,099 (`resource.rs:177`).
- `add` costs ≤ ℓ + 3 steps, the caller's iteration included.
- `add_product` (`:835`–`:857`) costs ≤ 34 + 2ℓ: two `binary_exponent`, four `exact_radix` at ≤ 6 each, one `mul_add` and two adds.
- `round` (`:899`), that is `exact_rounded_sum` over ℓ terms, costs ≤ **ρ_E(ℓ) = 551 + 17ℓ**: 137 for `new`, 17 per add, 412 for `round`, and 2.
- `exact_scalar` costs ρ_E(ℓ) + 2ℓ + 6.

**Lemma D (`correct_norm::norm3` takes ≤ 2 iterations).** The loop at `correct_norm/src/lib.rs:85`–`:108` runs at most **2 iterations** and makes at most **4** `midpoint_sign` calls.

*Notation.* u = 2⁻⁵³. The function sorts so that x₀ ≥ x₁ ≥ x₂, with e = `exponent(x₀)` ∈ [−1074, 1023]. The scaled values are yᵢ = xᵢ·2⁻ᵉ, S = Σyᵢ², and R = √S.

*Step 1: exact scaling.*
- y₀ ∈ [1, 2) is exact. `scale` (`:147`–`:157`) multiplies by at most two powers of two. When |k| > 1000 the first factor scales up, or scales a value ≥ 2¹⁰⁰¹ down, to a normal result.
- If the exact y₁ ≥ 2⁻⁶⁰, then x₁ ≥ 2^(e−60). Every intermediate is normal, so y₁ is exact.
- If the exact y₁ < 2⁻⁶⁰, rounding is monotone and the bound 2^(e−1060) or 2⁻⁶⁰ is a normal power of two. So the computed y₁ < 2⁻⁶⁰ too. The test at `:62` therefore takes the early return exactly when the exact y₁ < 2⁻⁶⁰.
- That return is right. y₁, y₂ < 2⁻⁶⁰ give √(x₀² + x₁² + x₂²) < x₀(1 + 2⁻¹²⁰). That is below x₀ + ½ulp(x₀), and subnormal x₀ has a wider gap. No loop runs.
- The same argument makes y₂ exact whenever the exact y₂ ≥ 2⁻¹²⁰ (`:65`–`:70`). Below that it is replaced by 0 with `sticky`.

*Step 2: exact parts.*
- Each kept yᵢ ≥ 2⁻¹²⁰, so `square` (`:160`–`:163`) gives yᵢ² = pᵢ + qᵢ exactly. The error is a multiple of 2⁻³⁴⁴, so it is representable.
- `two_sum` (`:166`–`:170`) is exact.
- So S = s + L exactly, with L = t₀ + t₁ + q₀ + q₁ + q₂ and S ∈ [1, 12).
- With |qᵢ| ≤ u·pᵢ, |t₀| ≤ u·s₁ and |t₁| ≤ u·s: |L| ≤ 3.01·u·S.

*Step 3: the candidate's error.* The candidate is `low` (4 additions, depth 3), `root₀ = RN(√s)`, then `root = RN(root₀ + RN(RN(fma(root₀, −root₀, s) + low)/(2·root₀)))` (`:77`–`:79`).
- `low` = L + λ with |λ| ≤ γ₃·3.01uS ≤ 9.04u²S.
- |R − root₀| ≤ |√S − √s| + u√s ≤ 2.52uR.
- The exact Newton step from root₀ is within (2.52uR)²/(2root₀) ≤ 3.18u²R of R.
- The computed numerator is within 16.1u²S of S − root₀². This does not assume that the fma is exact (it is, for root₀ = RN(√s), but nothing here relies on that). Divided by 2root₀ that is ≤ 8.06u²R.
- The quotient's rounding is ≤ u·2.53uR.
- So the sum before the last rounding is within **η ≤ 16u²R = 2⁻¹⁰²R** of R. The source's comment "about 2⁻¹⁰⁰ relative" (`:74`) is consistent with this.
- After the last rounding, |root − R| ≤ ½·ulp(root) + η, and η < 2⁻⁴⁹·ulp(R).

*Step 4: at most one grid step from the target.* Let D be the destination grid: binary64 with its subnormals, extended above MAX by spacing 2⁹⁷¹, as `:89` states. The loop's target t is a nearest point of D to N = 2ᵉ√S. Its tie rule (`:92`, `:103`) breaks exact ties up when `sticky` is set and to even otherwise. Write c = 2ᵉ·root, η′ = 2ᵉη, and r₀ = RN_D(c).
- `scale(root, e)` (`:80`) rounds once to D: one multiply, or an exact scaling up followed by one rounding.
- **Uniform spacing g** (the normal range inside a binade, the subnormals, and the transition at 2⁻¹⁰²² where both sides are 2⁻¹⁰⁷⁴): D's spacing is at least the scaled grid's, so |c − N| ≤ ½g + η′. Then |r₀ − t| ≤ 1.5g + η′ < 2g, and two grid points that close are at most one step apart.
- **Binade boundary P = 2^k:** the spacing is G below and 2G above.
  - t = P: N ∈ [P − ½G, P + G], c ∈ [P − G − η′, P + 2G + η′], so r₀ ∈ {P − G, P, P + 2G}.
  - t = P − G: c ≤ P + ½G + η′, so r₀ ≤ P; and c ≥ P − 2G − η′, so r₀ ≥ P − 2G.
  - t = P + 2G: c < P would force N < P + ½G + η′, a contradiction. So c ≥ P − η′ and r₀ ≥ P, and c ≤ P + 4G + η′.
  - The same holds at x₀'s own binade. root < 1 is impossible, since root ≥ R − ½ulp − η > 1 − 2⁻⁵³.
- **Overflow:** if c ≥ MAX + 2⁹⁷⁰, then r₀ = ∞ is clamped to MAX (`:81`–`:83`). There |c − N| ≤ ½·2⁹⁷¹ + η′, so N ≥ MAX − η′, so t ∈ {MAX, overflow}. The first iteration returns MAX, or `INFINITY` at `:93`–`:95`.

*Step 5: the walk.*
- `midpoint_sign` (`:113`–`:117`) is exact. rs = `scale(r, −e)` is exact (rs ∈ [1, 4)). The half-gap h is a power of two in [2⁻⁵³, 2⁻¹], so 2·rs·h and h² are exact. `square` and `exact_sign`'s TwoSums are exact.
- So each test compares S with a midpoint exactly. The midpoints increase with r, so the walk never reverses.
- At an exact tie the two neighbours differ by one in their bits, so exactly one is odd, and the up and down rules (`:92`, `:103`) agree.
- The iterations are therefore |idx(r₀) − idx(t)| + 1 ≤ **2**, where idx is a point's position on D.
- An upward move costs 1 `midpoint_sign`, and a downward move or a return costs 2, so there are at most 4.

No repository test counts the iterations. `correct_norm`'s own tests (`lib.rs:210`–`:294`) are hand cases; `load_case_algebra/src/unsigned_tests.rs:318` compares a random sample with `norm3`; `stress_recovery/tests/u4b_section_bits/` compares `norm2` bit for bit with an exact `Fraction` reference. Part II R7 states the optional witness.

*Steps.* The two array literals, `squares` (`:84`) and `exact_sign`'s argument (`:116`), are charged one step per element, as fills.
- `exact_sign` (`:175`–`:200`): the `[0.0; 10]` fill (10); the prologue and the match (1); 10 outer bodies of ≤ 19 operations, each reading one element of `terms` through `&term`; and inner iterations of ≤ 35 operations, at most Σ_{t<10} t = 45 of them (the length after t terms is at most t). It copies no array. That is **66**.
- `midpoint_sign` (`:113`–`:117`): `*squares` copies six elements out of borrowed data (6); the ten-element argument array (10); `square`, the bindings and the ten element expressions, 44 operations (1); and `exact_sign`. That is **83**.
- `norm3`: straight line ≤ 320 operations counted literally, charged 6; the `squares` literal (6); 4 `scale` loop iterations (each `scale` call runs at most one, since |k| ≤ 1,074); and per loop iteration 3 straight steps (≤ 125 operations) + 1 `scale` iteration + 2 × 83. That gives **ν̄₃ = 6 + 6 + 4 + 2 × 170 = 356**.
- `norm2` (`:34`) adds one call: **ν̄₂ = 357**. The ledger charges 357 for both.

**Lemma E (128-bit wide arithmetic).** `Wide2` (a 128-bit significand) costs, charged 2×tok over each worst path with its callees (`retained/wide.rs:258`–`:498`, `:505`–`:629`, `:636`–`:919`), plus the copies of §1.1 on that path:

| Operation | Path | Tokens | Copies | Steps |
|---|---|---|---|---|
| `add` | wrapper, `add_exact_operands`, 2 `is_zero`, `cmp_magnitude`, 2 `sig`, `shr_sticky`, 2 U256 `sub` + `is_zero`, `round_pack` (+ 7 helpers) | 979 | `shr_sticky`'s `(*self, false)` (`:544`) and `shl`'s `*self` (`:529`, in `round_pack`): U256, 2 each | **35** |
| `sub` | `add` + `neg` | 994 | as `add`, plus `neg` | **39** |
| `mul` | wrapper, `mul_exact_operands`, 2 `is_zero`, 2 `sig`, `mul_u128`, `round_pack` | 659 | `shl` 2 | **23** |
| `div` | prologue 600 tok (19) + 129 iterations of `divide_significands` | | `shl` 2 in the prologue | **279** |
| `sqrt` | prologue 572 tok (18) + 130 iterations of `sqrt_significand` | | `shl` 2 in the prologue; `Ok(*a)` (`:810`) only on the zero path | **410** |
| `from_f64` (`lift`) / `mul_pow2` / `cmp_value` | | 143 / 98 / 181 | `mul_pow2`: `..*self` (`:376`), exponent and two limbs, 3; or `Ok(*self)` (`:373`) on the zero path, 4 | **5 / 7 / 6** |
| `neg`, `abs` / `is_zero` | | ≤ 29 | `..*self` (`:353`, `:360`), 3 | **4 / 1** |
| `split_binary64` / `add_product_to` | 476 tok + 3 iterations / split + ≤ 3 accumulator `add_product` | | none | **18 / 74** |
| `at_least_one_twentieth` | with `mul_u128`, `shl` | 227 | `shl` 2 | **10** |
| `to_binary64` (`wide/multi.rs:725`–`:766`) | with helpers, after R8 | ≤ 360 | none | **12** |

- The division iteration (`:756`–`:764`) is 55 operations counted literally, with `shl` 21 and `or_low` 11 inlined. Its shift is by 1, so `shl`'s `0 => *self` arm is not taken. It is charged 2.
- The square-root iteration (`:795`–`:803`) is 180 operations counted literally. It is charged 3.
- So no operation's charge exceeds 1.22 times its charge without copies (`sub`, 32 → 39), except `neg`, `abs` and `mul_pow2`, which add 3 each. §9.3 uses this.
- `atan_of_positive` (`:954`–`:999`) costs ≤ 9,245:
  - ≤ 5 reductions of 793 (`:957`–`:968`: `mul`, `add`, `sqrt`, `add`, `div`, the test 10 and 1), plus a final test of 10;
  - 44 before the series (the `*t` copy at `:956`, 4; `mul`; the `[Wide2::ZERO; 15]` fill; 2);
  - ≤ 15 series terms of 312 (`mul`, `neg`, `from_f64`, `div`, 1);
  - a tail of ≤ 14 adds of 36;
  - 42 to finish (`add`, `mul_pow2`).
- `included_angle` costs ≤ 9,630: `atan_of_positive` and ≤ 1.22 × 315 for the rest.
- **τ̄**, the cost of `twice_atan2_nonnegative` (`structural/arc_angle.rs:36`–`:58`), is ≤ 9,557: `atan_of_positive`, `div` 279, two `from_f64` 10, `mul_pow2` 7, `to_binary64` 12 and the `atan_positive` wrapper 4. It is charged **9,600**.

**Lemma F (`body_scales`' union-find, `formation_check.rs:1151`–`:1157`).** Each `root` call runs ≤ N − 1 iterations.
- *Proof.* `parent` starts as the identity (`:1150`), so it is a forest whose roots point to themselves. `parent[ra] = rb` (`:1180`) links two distinct roots, so it creates no cycle. Path halving (`:1153`) replaces a pointer by an ancestor. Each iteration moves x strictly toward its root, so the iterations are at most the depth, and the depth is at most N − 1.

## 2. Dimensions

P0 holds N, n_g = 6N, n̄ = n̄_f, pipes, A, K_c, T_r and S (`modal_resource_bound_dimensions`, `resource.rs:1026`–`:1082`), and forms `twelve` = pipes + A + K_c + T_r at `:1048`. N is nodes, n_g global DOFs, n̄ the bound on free DOFs, A arcs, K_c connectors, T_r tie rods and S springs. Each new dimension is computable in `modal_resource_bound_dimensions` from `prepared.built` and `prepared.boundary` in O(model) time without allocation, except δ̄, which takes one array of N counters (Agent 0's decision, §12).

| Symbol | Meaning | Bound from source | P0 computes |
|---|---|---|---|
| n | free DOFs of the solved system | n ≤ n̄ | n̄ |
| p | prescribed DOFs | p = n_g − n exactly: `validate` refuses an incomplete or overlapping map (`structural.rs:757`–`:772`), and `stiffness.rs:202` passes every restrained DOF. So p ≤ n_g | charged as n_g |
| F_e | frame elements | F_e ≤ pipes: `frame_elements.push` (product `lib.rs:9052`) runs at most once per pass of the pipe loop that ends in `pipes.push` (`:9062`), and `continue` (`:9046`) skips both | pipes, or `built.frame_elements.len()` |
| E_el | 144-entry elements (edges) | F_e + K_c + T_r + A: one `element()` each (`adapter :1766`, `:1790`, `:1809`, `:1820`) | `twelve` |
| C | physical contributions | 144E_el + S on the plain path (`:1909`–`:1920`, `:1833`); + 9s_k with a basis (`:1849`–`:1866`) | `144·twelve + springs + 9·skewed`. P0 already forms these products for D_K (`:1049`) |
| G | contribution groups | E_el + S (+ s_k) (`:1922`, `:1839`, `:1865`) | `twelve + springs (+ skewed)` |
| Z, Z̄ | stored pattern entries; their bound | Z ≤ Z̄ = min(n_g², 36(N + 2E_el) + S). The pattern is `from_connectivity` (`sparse.rs:83`–`:131`). A node's list is itself plus its partners, deduplicated (`:96`–`:101`), so Σ\|list\| ≤ N + 2E_el. A node gives 6 rows of 6·\|list\| (`:123`–`:125`), and a spring-only node at most 6 diagonals (`:114`–`:116`) | O(1) |
| δ̄ | most contributions at one stored entry | max over nodes a of deg(a) + σ(a). deg(a) is the elements with a as an end; σ(a) is the springs on a's DOFs plus the skew blocks at a. An entry in block (a, b) receives one contribution per element joining a and b (a ≠ b) or meeting a (a = b), one per spring on its diagonal, and nine per skew block at a | one pass over `frame_elements`, `connectors`, their `tie_rods()`, `curved_bend_elements`, `boundary.springs` and `skew.springs()`, incrementing a counter per node: O(N + E_el + S) time and one allocation of N counters, charged in P0's own account (§12) |
| ℓ̄ | longest expansion before an add | min(δ̄ − 1, X̄) ≤ min(δ̄, 2,099) (Lemma C) | O(1) |
| β̄ | steps per pattern search | ⌊log₂ n_g⌋ + 2, after R1 (row width ≤ n_g) | O(1) |
| λ̄_A | longest arc component id | `CurvedBendMacroBuild::component_id` bytes, cloned at `stiffness.rs:197`; λ_A ≤ λ̄_A for each arc | `max` of `component_id.len()`: O(A). `String::len` is a field read |
| Λ_m | member prefixes' bytes | Σ over straight pipes of `member_prefix`'s output, `"member:{len}:{id}:"`: λ_p + 9 + ⌈log₁₀(λ_p + 1)⌉ ≤ λ_p + 29 | Σ over `built.pipes` of `element_id.len() + 29`: O(pipes). It over-counts arcs and replaced spans, which is sound |
| ν | constrained nodes (one per skewed stop) | `RestraintBasis::nodes`; one stop per node, else `build` refuses (`restraint_basis.rs:766`–`:772`) | `skew.stops().len()` |
| s_k | skewed springs | `RestraintBasis::springs` | `skew.springs().len()` |
| Λ_s | skew support ids' bytes | `RestraintBasis::clone` copies each stop's and spring's `support_id` (`adapter :1216`) | Σ `support_id.len()` over stops and springs |
| β̂ | basis-branch search and sort depth | ⌊log₂(n_g + S + 1)⌋ + 2 ≥ every LB and HS depth on the branch (Part II, "Common algorithms") | O(1) |
| w̄ | widest pattern row | min(n_g, 6(1 + δ̄)): a node's list is itself plus its partners (`sparse.rs:96`–`:101`) | from δ̄ |
| Z_ν | entries in the 3ν translation rows at constrained nodes | min(Z̄, 18ν(1 + δ̄)) | from δ̄ |
| U | bodies W4 screens | min(N, K_c + A): each holds a connector or arc edge (`adapter :1970`–`:1982`); a rod lies on its connector's nodes | O(1) |
| m_A | most arcs on one node pair | ≤ A | O(A²) compares, or A |
| C′, ℓ̄′ | coordinate contributions; their longest expansion | C′ ≤ 36C (`restraint_basis.rs:936`–`:940`; μ = 36 at `resource.rs:1077`); ℓ̄′ = min(36δ̄, 2,099) | O(1) |

**No identifier cap exists on this path.**
- The model schema's `Id` is `{"type": "string", "pattern": "\\S"}` with no `maxLength` (`schemas/model.schema.yaml:1238`–`:1242`). The schema's only `maxLength` is the tags' 64 (`:2771`).
- `normalize_required_id` only trims and refuses empty or "TBD" ids (`nonlinear_integration/src/lib.rs:3519`–`:3531`).
- The retained route's "l ≤ 128" (`retained_memory.rs:1574`, `tools/retained_pricing/base/u6_owners_model.py:11`) is a cap that the retained route's own gates check against observed facts (`retained_memory.rs:3602`–`:3606`), not a validation of the model.
- The desktop app's `MAX_MODEL_DOCUMENT_BYTES` = 64 MiB (`apps/desktop/src-tauri/src/model_document_file.rs:27`) bounds a file it opens, not `run_modal_dense`'s input.
- So λ̄_A, Λ_m and Λ_s are measured.

## 3. The plain path, in call order

1. `stiffness.rs:194`–`:202`: the arc slots (`from_macro_element`, one per arc), the spring list and the prescribed list. Then `:208`: `frame_member_identities` (product `lib.rs:4890`–`:4908`) and `with_frame_members` (`adapter :784`–`:787`, a `to_vec` of the strings).
2. `SparseAssemblyEvidence::new` (`adapter :680`–`:698`) → `build` (`:701`–`:745`) → `EvidenceParts::new` (`:1712`–`:1924`). Every `slot`/`scatter_slot` call goes through `SparseEvidenceStore::index`, whose `self.pattern.find(row, col)` at `:1657` reaches `SparsePattern::find` (`sparse.rs:185`–`:191`). Its `binary_search` at `:190` is the plain path's only search (R1).
3. `solve_assembled` (`adapter :1077`–`:1105`):
   - `unscaled_evidence` and `check_pattern`;
   - `geometry` → `BodyEvidence::geometry` with `w4: None` (`:791`–`:799`, `:1941`–`:2038`) → `assess_rigid_body` (`rigid_body.rs:34`–`:231`);
   - `symmetry_basis`, `to_dense` (`sparse.rs:309`) and `dense_symmetry_view` (`adapter :846`).
4. `prepare_assembled_structural` → `prepare_bound` (`structural.rs:1492`–`:1608`): `validate`, radix scaling, `exact_scaled_rhs` (`:1461`–`:1491`), the skew audit, and `audit_contributions` (`contribution_sums`, `contribution_differences`).
5. `solve_prepared` (`adapter :2736`) → `factor_structural_cholesky` → `validate_prepared`, `cholesky`. On a factor error, `negative_pair_witness` (`structural.rs:2504`–`:2573`).
6. `finish_structural` → `finish_checked_factor` (`:1894`–`:2036`): `validate`, the Hager estimate (`:1817`), the solves, `evaluate_original_residual_bound` (`:1654`–`:1751`), `audit_intended_action` (`:1144`–`:1212`), `audit_load_fidelity` (`:1243`–`:1303`) and the report. The refinement loop refuses at `:2013` (`if attempts == 3 || worst >= prior`) and counts at `:2034`. No formation source is set (`adapter :1093`–`:1104`), so K-D5 does not run.
7. The drops of the dense views, sums, differences and the discarded solution.

Two things are off this path:
- `sparse.rs`'s sorts and dedups (`:74`–`:75`, `:100`–`:101`) belong to K's formation (`from_connectivity`, reached from `assemble_sparse_stiffness`). §5.3.8 does not charge it: "work outside §5.3.8's table (K's formation, …)" (`resource.rs:367`).
- `adapter :2124`–`:2125` (W4's sort and `binary_search`) runs only with `w4: Some`, which is the restraint-basis branch (§9).

**P-zero (source facts).** The force is `LoadLedger::new().finish(n_g)` with no term (`stiffness.rs:158`–`:159`), and every prescribed value is 0.0 (`:202`). So:
- every `exact_scaled_rhs` is +0.0, and its couplings' `add_product(−k, 0.0)` return at `mb == 0` (`exact_sum.rs:237`);
- the triangular solves give +0.0 (`structural.rs:2048`–`:2059`), so u = 0;
- the first residual pass passes (r = 0, d = 0, guarded 0, `:1723`–`:1725`), so no correction solve runs;
- every product against u = 0 returns at its zero test (`:841`, `:1613`);
- no load-fidelity row is flagged (denominator 0 and residual zero give ratio 0, `:1390`–`:1400`), so `row_sources` is not reached;
- `prescribed_coupled` is false (`:1457`), so the residual's exact numerator is not formed.

§8 gives the general force.

## 4. The ledger (P-zero, plain path)

- **V** = `validate` (`structural.rs:707`–`:774`) = **(5n_g² + 15n_g + 8)/2**. The parts are:
  - dimension scan n_g;
  - finiteness n_g² + n_g;
  - symmetry evidence n_g² + 2n_g (`:726`–`:730`, the operation-count row scan included);
  - pairs n_g(n_g − 1)/2 bodies of ≤ 40 operations, plus n_g outer bodies (`:736`);
  - `seen` n_g;
  - the free and prescribed maps ≤ n_g + 1;
  - `any` n_g;
  - straight code 3.
- **CS** = `contribution_sums` (`:937`–`:959`) = **n_g² + 2n_g + n + G + 3 + C(1,124 + 35ℓ̄)**. The parts are:
  - the `sums` fill, n_g² + n_g;
  - `ElementAbsorption::new` (`:994`–`:1009`), G + n_g + n + 3;
  - per contribution: 2 for the body, its checks, the binding and `record` (`:1016`–`:1048`, 66 operations); ρ_E(ℓ) for `old = round`; ℓ + 3 for `add`; ρ_E(ℓ + 1) for the second `round`. That is 2 + (551 + 17ℓ) + (ℓ + 3) + (568 + 17ℓ) = 1,124 + 35ℓ.
- **Σ** = one factor solve (`:2042`–`:2061`, with the dispatch `:2121`) = **n² + 2n + 2**.

| Group | Lines | Trip counts | Steps |
|---|---|---|---|
| PREP, call preparation | `stiffness.rs:194`–`:202`, `:208`; product `lib.rs:4890`–`:4908`; `adapter :784`–`:787` | A × `from_macro_element` (κ_arc, §5.5); S; p. The pipe loop: 2 membership tests (scans of A and K_c with R5), `next`, `pipe.frame_element()` (κ_pe, §5.6), `==`, `member_prefix` (P-fmt), `push`. The `to_vec` of F_e strings | A(64,300 + 7λ̄_A) + S + p + pipes(233 + A + K_c) + 2Λ_m + F_e + 3 |
| AE, evidence build | `adapter :711`–`:745`, `:1722`–`:1924` | `pattern.clone()` n_g + 2Z + 2 (its dimension and transpose fields included); four `vec![_; Z]`; `visit` Z (`:1682`–`:1694`); `build`'s checks (`:711`–`:744`). Then connectors' `force_scaled` (`:1725`–`:1728`); `coordinates` N; `frames.to_vec` F_e; arc slots; the frame, connector, rod and arc loops (§5); `element()` per element (§5.1); springs S(1 + β̄) + `to_vec` S; scatter C(1 + β̄) | 7Z + n_g + N + 8 + AE_el + E_el(190 + 144β̄) + S(2 + β̄) + C(1 + β̄) |
| VIEW | `adapter :1085`–`:1092`; `sparse.rs:309`–`:318`; `adapter :846`–`:858`, `:2256`–`:2263` | `check_pattern` (derived `PartialEq`, slice compare) n_g + 2Z + 2; `symmetry_basis` E_el + ≤ 512 (P-fmt); `to_dense` n_g² + 2n_g + Z (outer row bodies, `sparse.rs:314`); the view 2n_g² + 3n_g + Z (outer row bodies, `adapter :850`) | 3n_g² + 6n_g + 4Z + E_el + 518 |
| GE, `geometry` | `adapter :1941`–`:2038`; `rigid_body.rs:58`–`:231` | per node: `seen`, seed, BFS pop, `any`, collect (with its `Option<[f64; 3]>` copy out of borrowed data, `adapter :1991`, 4), validation 3, `relative` (`norm3`, `:81`) ν̄₃ + 4, scaling 4; per body, `origin` (`:72`) 3. These per-node and per-body constants lie inside 7,716. The BFS pops each node once and scans every edge: N·E_el. `body.contains` ≤ 2N·E_el. The ground filter ≤ 2N(p + S). Per qualified body, `assess_rows`: ≤ 7,256 + 960ν̄₂, of which Jacobi is ≤ 64 sweeps × 15 pairs (`:135`, `:138`–`:139`) at 7 + ν̄₂ each (`norm2`, `:147`). Per ground row 3,875: 3,840 in the sweeps (three sums and the rotation, 4 per pair), 21 for its formation (18 counted: the body, `motion_rows` 1 with its `relative[node]` copy at `:96` 3, the norm 6, the scaling 6 and the push), 6 for its clone, 6 for `singular` and 1 for `count` (`:108`–`:173`) | N(ν̄₃ + 960ν̄₂ + 7,716) + 3N·E_el + 2N(p + S) + 3,875(p + S) |
| MECH (refusal only) | `rigid_body.rs:196`–`:299`; `adapter :2020`–`:2027` | one body at most. Candidates ≤ 7 + 2N, built in 55 + 8N (`rigid_body.rs:198`–`:216`); each 16 (the witness prologue, `:238`–`:241`) + ≤ 5,414 per node (`original_rigid_witness`: expansions ≤ 12 terms and 6 `exact_scalar` calls, Lemma C) + g, where g ≤ p + S is the body's ground rows; the direction copy n_g + 7N (`adapter :2023`–`:2025`) | (7 + 2N)(5,800N + p + S + 16) + 15N + n_g + 55 |
| PB, `prepare_bound` without AC | `structural.rs:1496`–`:1607`, `:1461`–`:1491` | V; exponents n; `a` n² + n; `rhs` n. Per free row: n `radix_scale` at 6, then `exact_scaled_rhs`: `new` 137, `accumulate_dof` 1, p couplings at 2 (`add_product(−k, 0.0)`), `round` 412, `round_scaled` 412, `is_zero` 69 (`:1487`, value 0), checks 2. Symmetry evidence n_g² + 2n_g + 2. Skew loop n(n − 1)/2 at ≤ 14 (two `radix_scale` when skewed). `basis.to_owned()` ≤ 512 (P-fmt bytes) | V + 14n² + 1,036n + 2n·p + n_g² + 2n_g + 517 |
| D, `contribution_differences` | `:1050`–`:1061` | `sums.to_vec()` n_g + n_g² + Σℓ; n_g outer bodies (`:1055`); n_g² adds at ℓ_e + 3, where ℓ_e is entry e's expansion length; Σℓ_e ≤ C | 4n_g² + 2n_g + 2C |
| AC, the rest of `audit_contributions` | `:1071`–`:1139` | rounding loop n_g + n_g² (outer row bodies, `:1072`); per nonzero difference (≤ Z: a contribution off the pattern is refused at `adapter :1658`–`:1660`) 1 + 2ℓ + 2ℓ′, where ℓ′ ≤ ℓ + 1 is the difference's length; perturbation n + n² × 8 (radix 6 + 2) + 7 per term (Σℓ′ ≤ C + Z); prescribed right-hand side n(p + 2), whose `add_product` against 0.0 returns at `:841` | n_g² + 8n² + n_g + 14Z + 12C + n(p + 5) + 3 |
| G1, `validate_prepared` | `:775`–`:799` | V; n + n²; n; n(n − 1)/2 + n | V + 1.5n² + 2.5n + 4 |
| HC, Hager | `:1795`–`:1874` | `square_and_finite` n + n²; norm n + n²; x n; ≤ 5 iterations (`:1817`) of 2Σ + 9n + 5; `alt` n + Σ + 3n + 5. So ≤ 11 solves | 13n² + 75n + 53 |
| R, one residual pass | `:1654`–`:1751` | V; u n_g; prescribed `any` p. Per free row: the exponent loop over all n_g columns (dense row, `:549`–`:551`); `radix_scale` 6; `new` 137 (`:1694`, always built); `prescribed_coupled` p; the second loop n_g at ≤ 2; three `physical_residual_record`s at 6 (`:1735`, `:1736`, `:1744`); and 2 | V + n_g + p + n(3n_g + p + 163) + 1 |
| K, `audit_intended_action` | `:1144`–`:1212` | CS. Per free row: two `sums_row` passes of n_g; `exact_radix` 6 + 6; `add` 1; three records at 6; and 2. Parts at ≤ 3 (Σ over rows ≤ C). The `round` at ρ_E(0) = 551 is not reached (`is_zero`, `:1193`) but is charged as margin | CS + 2n·n_g + 3C + 583n + 2 |
| L, `audit_load_fidelity` | `:1243`–`:1303`, `:1341`–`:1412` | V; u n_g; `by_dof` n_g; terms 0; CS; `restrained` n_g + p. Rows over all n_g: `parts` n_g + \|parts_i\|; `audit_load_row` = `new` 137 + `round` 412 + 4\|parts_i\| + `exact_radix` 6 + 6 + `add` 1 + `residual.round()` ρ_E(0) = 551 + 2; `extend` 1. Σ\|parts_i\| ≤ C | V + CS + n_g² + 1,118n_g + p + 5C + 2 |
| FIN, finish without R, K, L | `:1906`–`:2011`, `:2146`–`:2159` | V; HC; Σ; y's finiteness `any` n (`:1925`); u n_g + p + 2n; `fold` n; `all` n; `any` n; `scale_exponents` n; `factor.pivots.clone()` n (`:2156`); `gate_symmetry` basis ≤ 512; rounding `to_vec` ≤ 2Z + 2C; 12. One loop pass (`:2013`, `:2034`) | V + HC + Σ + n_g + p + 8n + 2Z + 2C + 524 |
| WIT (factor failure only) | `:2519`–`:2573`, `:2461`–`:2502` | G1; n(n − 1)/2 pairs × 4 cells at ≤ 29; one verification (P-witness): G1 + n² + 9n + n_g + 24 | 2V + 21.5n² + 12n + n_g + 50 |
| DROP | | 4 n_g×n_g expansion arrays; 8n_g row vectors; 4n; rounding records twice (2Z); member strings twice (2F_e); arc slots twice (2A); connector rod vectors K_c | 4n_g² + 8n_g + 4n + 2Z + 2F_e + 2A + K_c + 10 |

WIT's literal count is 2V + 18.5n² + 1.5n + n_g + 35. The row charges the larger form.

## 5. Per-element formation constants (plain path)

All loops here run over fixed ranges (`0..12`, `0..6`, `0..3`, `0..64`, `0..129`, `0..130`) or are capped (`ATAN_MAX_REDUCTIONS` = 5, `ATAN_MAX_TERMS` = 15, `retained/wide.rs:137`, `:139`).

### 5.1 The common `element()` and `node()`

- `element` (`adapter :1895`–`:1924`): straight 1 step. `element_dof_map` (`frame_kernel/src/lib.rs:87`–`:95`) is a 12 fill + 6 iterations. Then 12 outer + 144 inner iterations (≤ 42 operations each, `slot` included) + 144 finds. That is **E = 190 + 144β̄**.
- `node` (`adapter :1882`–`:1894`): **8 steps**. `slot.is_some_and(..)` copies the `Option<[f64; 3]>` through the reference (4), `old != point` compares three elements (3), and the rest is 1. Each caller also passes a coordinate array: a frame's `element.node_i.coordinates` is read out of borrowed data (3), and a connector's or rod's `node_i()` copies its `FrameNode` (4).

### 5.2 Frame, κ_f = 12,100 per frame element

All at `frame_kernel/src/lib.rs` unless stated.

| Part | Lines | Steps |
|---|---|---|
| `local_stiffness()`: `properties` → `length` (`:683`–`:698`, `:559`–`:562`); the straight line, Timoshenko the worst case (`:732`–`:855`, `:864`–`:908`, `:994`–`:1051`, `:2172`–`:2180`, `:2060`–`:2101` non-loop), 2×tok over 4,250 tokens | | 133 |
| its `[[0.0; 12]; 12]` fill (`:843`) | | 144 |
| `add_terms` × 2 (`:2090`–`:2096`): 4 + 16 iterations each | | 42 |
| the row check (`:856`–`:858`): 12 + 144 iterations | | 156 |
| the copies out of `&self` in `properties()` (`:696`): `self.section` 8 and `length()`'s two coordinate arrays 6 (`:684`) | | 14 |
| **κ_ls**, `local_stiffness` | | **489** |
| `orientation()` (`:700`–`:702`, `:622`–`:636`, `:595`–`:620`, helpers `:2182`–`:2247`): straight (2×tok) ≤ 135; `validate_finite_vector` loops 2 × 3 + 3 × 3 + 3; the copies out of `&self`, `local_x_axis()`'s coordinates 6 and `self.y_reference` 3 | | **169** |
| `transformation_matrix()` (`:638`–`:648`): fill 144, 2 iterations, 4 `copy_rotation_block` × 12 (`:2140`–`:2146`), each passed `self.local_axes` out of `&self` (4 × 9), 1 | | **231** |
| `structural::transform_roundoff` (`structural.rs:2422`–`:2457`): three fills 432; loop 1: 12 + 144 + 1,728 iterations (body 56 operations: `checked_product` 23, `checked_value` 9 × 2, `:684`–`:706`); `gamma` 1 (`:649`–`:652`); loop 2: 12 + 144 iterations, 1,728 at 2 steps (81 operations); the `[[48; 12]; 12]` fill 144; 1 | | **6,074** |
| `node` × 2, with `element.node_i.coordinates` read out of borrowed data (2 × (8 + 3)) | | 22 |
| `global_stiffness()` (`:708`–`:716`): κ_ls + `orientation` + `transform_global_stiffness` (`:1614`–`:1620`) = `transformation_matrix` + `multiply_transpose_left` (`:2119`–`:2138`: fills 288 + 2 × (12 + 144 + 1,728) + 1 = 4,057); row check 156; 1 | | **5,103** |
| the loop body (`adapter :1755`–`:1776`), `frames.to_vec` | | 2 |
| **κ_f** | | 12,090, charged **12,100** |

### 5.3 Connector, κ_c = 306,200 per connector

All at `frame_kernel/src/connector.rs` unless stated.

| Part | Lines | Steps |
|---|---|---|
| `force_scaled` (`adapter :1727`; `:439`–`:454`): a `clone` of the connector (89 element values ≤ 100); 6 + 36 iterations of `force_scaled_value` (`lib.rs:1122`–`:1134`); 1. Rods are §5.4 | | 143 |
| `b()` (`:366`–`:395`): `chord` (`:351`–`:360`, `:827`–`:847`) 11, plus its four arrays read out of `&self` (`node_i.coordinates`, `node_j.coordinates`, `a_i`, `a_j`) 12; straight 2; `skew(self.a_i)`, `skew(self.a_j)` 6 (`:370`–`:371`); fills 18 + 72; 9 + 3 iterations; per (axis, c) 1 + 2 `exact_dot3` (`:775`–`:781`: 137 + 3 × 18 + 412 + 2 = 605); × 9 | | 11,038 (inside the next two rows) |
| `global_stiffness` (`:400`–`:420`): `b()`; fill 144; 12 outer; 78 entries × (`new` 137 + 6 + 36 × (`triple` 1 + 2 × 18, `:762`–`:773`) + `round` 412 + 2 = 1,889) | | 158,536 |
| `force_scaled_matrix` (`lib.rs:1138`–`:1150`): `*matrix` 144 + 12 + 144 + 1 | | 301 |
| `formation_roundoff` (`:459`–`:488`): `b()`; fill 144; 12; 78 entries × (`new` 137 + 6 + 36 × 21 [`product_upward` × 2 (`load_ledger.rs:611`–`:621`, 2 steps each) + `add` 17] + `round_upward` 827 (`load_ledger.rs:530`–`:543`: ρ + clone 136 + an `add` on the clone ≤ 5 + 12 + 136 + `signum` 68 + 2) + 3 = 1,729); the `[[1; 12]; 12]` fill 144 | | 146,200 |
| `force_scaled_matrix` (bounds), node × 2 with `node_i()`'s copy (2 × (8 + 4)), body | | 326 |
| **κ_c** | | 305,944 (the rows above sum to 305,506), charged **306,200** |

### 5.4 Tie rod, κ_r = 64,440 + 2ν̄₃ per rod

All at `frame_kernel/src/tie_rod.rs` unless stated.

| Part | Lines | Steps |
|---|---|---|
| `force_scaled` in the connector's (`connector.rs:447`–`:450`; `:321`–`:325`): `*self` 31 + 3; `collect` 1; `flat_map` 1 (`adapter :1799`) | | 36 |
| `direction_and_length` (`:245`–`:264`): `difference` 7 (`:549`–`:565`), plus its four arrays read out of `&self` (`:246`–`:251`) 12; `all` 3; `degenerate` (`:490`–`:530`): 3 × (`new` 137 + 2 `add` + `signum` 68 + a second `new` and 2 `add` + `round_upward` 827 + 1) + `new` 137 + 3 `add` + `round_scaled` 412 + `add_product` 18 + `signum` 68 + `new` 137 + 4 `add_product` + `signum` 68 + 3 ≈ 4,680, plus its copies out of `rod` (`:494` 6, `:509` 6) 12; `norm3` ν̄₃; 5 | | 4,719 + ν̄₃ (inside `c()`) |
| `try_c` (`:274`–`:286`), that is `c()`: the above + `offset_cross` × 2 (`:470`–`:482`: 3 × (605 + 1) + 5), each passed `self.o_i` or `self.o_j` (2 × 3) + fill 12 + 3 + 1 | | 8,387 + ν̄₃ (inside the next rows) |
| `global_stiffness` (`:291`–`:304`): `c()`; fill 144; 12; 78 × (137 + 37 + 412 + 2) | | 54,407 + ν̄₃ |
| `force_scaled_matrix` × 2, node × 2 with `node_i()`'s copy (2 × (8 + 4)), body | | 628 |
| `formation_roundoff` (`:330`–`:349`): `c()`; fill 144; 12; 78 × 7; fill 144 | | 9,233 + ν̄₃ |
| **κ_r** | | 64,359 + 2ν̄₃ + 36 (the rows above sum to 64,304 + 2ν̄₃), charged **64,440 + 2ν̄₃** (65,154 at ν̄₃ = 357) |

### 5.5 Arc, κ_arc = 64,300 + 7λ̄_A in PREP, κ_a = 439 + λ̄_A in AE

τ̄ = **9,600** (Lemma E): `twice_atan2_nonnegative` (`structural/arc_angle.rs:36`–`:58`) through the 128-bit arithmetic of `retained/wide.rs`. λ is the arc's id bytes, λ ≤ λ̄_A.

The parts, all at `curved_bend/src/lib.rs` unless stated:
- `objective_arc` (`:186`–`:285`): two `exact_rounded_dot` (604, 622), `twice_atan2` τ̄, 2 Gram–Schmidt iterations, validations 9, straight 4. That is **1,245 + τ̄**.
- `geometry()` (`:378`–`:380`) passes `objective_arc` three arrays read out of `&self`: `chord()`'s two coordinate arrays (`:374`) and `self.y_reference`, 9.
- `global_stiffness` (`:484`–`:488`) = 7,401 + τ̄ = **17,001**:
  - `geometry()`, that is `objective_arc` and its copies 9;
  - `self.chord()` for `assemble_macro_stiffness`, 6;
  - `tip_stiffness_global` (`:502`–`:506`) 5,320, made of `FrameOrientation::new` 20, `flexibility_of` (`:420`–`:468`, ≤ 8 `Series::eval` (`arc_integrals.rs:34`–`:45`, ≤ 13 coefficients + 5 powers + 1) + fill 36 + 21 iterations + 5) ≈ 230, `invert_symmetric6` (`:1092`–`:1117`: 6 × frame_kernel `solve_dense(6)` (`lib.rs:1995`–`:2058`) ≤ 700 + dense build 42 + 15 + 6) ≈ 4,360, and `rotate_tip_to_global` (`:1132`–`:1154`) ≈ 710;
  - `assemble_macro_stiffness` (`:1173`–`:1211`) 820.
- `curved_formation` (`adapter :2761`–`:2883`) = **18,280 + 3τ̄**: `global_stiffness`, `geometry` (with its copies 9), `tip_stiffness` (`geometry()` + `flexibility_of` + `invert_symmetric6`), and four `stage` closures (`:2797`–`:2828`: fills 72, 6 + 36 iterations, 216 iterations at 3 steps (≈ 165 operations), 36 tails) of 799 each, plus fills, transposes and the 42-iteration copy.
- `from_macro_element` (`nonlinear_integration/src/lib.rs:224`–`:244`): `normalize_required_id` × 2 (`:3519`–`:3531`, ≤ 3λ + 5 each), `global_stiffness`, `Self::new` (`:188`–`:220`, 157), and `curved_formation`. That is **25,850 + 4τ̄ + 6λ**, plus the `component_id.clone()` at `stiffness.rs:197` (λ).
- **κ_arc** = 25,850 + 4τ̄ + 7λ̄_A = 64,250 + 7λ̄_A, charged **64,300 + 7λ̄_A**.
- **AE per arc** (`adapter :1740`–`:1749`, `:1811`–`:1828`): the slot (`element_id` clone λ, `global_stiffness` copy 144, 5) + two fills 288 + 1. That is **κ_a = 439 + λ̄_A**.

### 5.6 Straight pipe in PREP, κ_pe = 220

`StraightPipeElement::frame_element` (`straight_pipe/src/lib.rs:506`–`:513`): `self.node_i`, `self.node_j` and `self.y_reference` read out of `&self` (11); `frame_section` (`:460`–`:480`) ≤ 9 (2×tok); and `FrameElement::new` (frame_kernel `lib.rs:660`–`:681`) with `properties` ≤ 10 + 14 for its copies and `orientation` 169, plus checks 4. That is 217, charged **220**. The pipe loop adds `next`, `==` (derived `PartialEq` on `FrameElement`, product `lib.rs:4900`: its three `[f64; 3]` fields compared as element primitives 9, and its scalar fields 1), `push` and the loop body: 233 per pipe, plus A + K_c for R5's scans.

So **AE_el = 12,100F_e + 306,200K_c + 65,154T_r + (439 + λ̄_A)A** and **PREP_el = (64,300 + 7λ̄_A)A + (233 + A + K_c)·pipes + 2Λ_m + F_e**.

## 6. The closed form (plain path)

n ≤ n̄, p ≤ n_g. V appears 5 times (PB, G1, R, L, FIN). CS appears 3 times (AC's `audit_contributions`, K, L). HC and Σ enter through FIN.

**Sum of the rows (success path):**

c(T2.integrity) ≤ 29.5n_g² + 37.5n² + 5n·n_g + 4n·p + 29Z + C(3,397 + 105ℓ̄ + 2β̄) + 1,183.5n_g + 1,881.5n + 4p + 3G + 191E_el + 3S + N + 2F_e + 2A + K_c + 1,676 + GE + AE_el + PREP_el.

The β̄ terms are AE's E_el·144β̄ + Sβ̄ + Cβ̄ = 2Cβ̄, since C = 144E_el + S on this path. C's constant 3,397 is CS's 3 × 1,124 = 3,372, plus AE 1, D 2, AC 12, K 3, L 5 and FIN 2.

**Charged** (rounded up; p ≤ n_g, n·p ≤ n·n_g):

> c̄(T2.integrity) = 32·n_g² + 38·n̄² + 9·n̄·n_g + 29·Z̄ + C·(3,400 + 105·ℓ̄ + 2·β̄) + 1,200·n_g + 1,900·n̄ + 3G + 191·E_el + 3S + N + 2F_e + 2A + K_c + 1,700
> + GE + MECH + WIT + AE_el + PREP_el,

where:
- GE = 350,793·N + 3N·E_el + 2N(n_g + S) + 3,875(n_g + S), with ν̄₃ + 960ν̄₂ + 7,716 = 350,793 at ν̄₂ = ν̄₃ = 357;
- MECH = (7 + 2N)(5,800N + n_g + S + 16) + 15N + n_g + 55;
- WIT = 2V + 21.5n̄² + 12n̄ + n_g + 50 ≤ 5n_g² + 16n_g + 22n̄² + 12n̄ + 58 (factor failure only, under P-witness);
- AE_el = 12,100F_e + 306,200K_c + 65,154T_r + (439 + λ̄_A)A;
- PREP_el = (64,300 + 7λ̄_A)A + (233 + A + K_c)·pipes + 2Λ_m + F_e;
- Z̄ = min(n_g², 36(N + 2E_el) + S), ℓ̄ = min(δ̄, 2,099), β̄ = ⌊log₂ n_g⌋ + 2 (§2).

Every coefficient is positive, so the bound is monotone in every dimension. MECH, WIT and the success path are exclusive; charging all three is still an upper bound. P0 may charge F_e as pipes.

**Order of growth.** The work is quadratic in n_g, and linear in C with constant 3,400 + 105ℓ̄. Per node it carries GE's 350,793 (the Jacobi screen's 960 `norm2` calls per body, with bodies ≤ N). Per element it carries the exact formations: a connector costs ≈ 3·10⁵ and an arc ≈ 6.4·10⁴. Nothing is cubic, so `T2.factor` (≈ n³/6) dominates at large n.

**Hand check (SEC-A, N = 2, one frame).** n_g = 12, n̄ = 6, S = 0, E_el = F_e = pipes = 1, A = K_c = T_r = 0, G = 1, C = 144, Z̄ = 144, ℓ̄ = 1, β̄ = 5.
- The charged polynomial is 544,658. Its C term is 144 × 3,515 = 506,160; n_g gives 14,400 and n̄ 11,400.
- GE is 748,140 (701,586 + 6 + 48 + 46,500).
- AE_el is 12,100, and PREP_el is 234 + 2Λ_m.
- The success path is ≈ 1.31·10⁶ (1,305,132 + 2Λ_m). MECH (128,005) and WIT (1,834) raise the charge to ≈ 1.43·10⁶.

## 7. Premises, with the source that makes each true

| Premise | Where | Source |
|---|---|---|
| Hager ≤ 5 iterations, ≤ 11 solves | `structural.rs:1817` | `for _ in 0..5 {` with two solves (`:1818`, `:1831`) and one after (`:1858`) |
| Refinement ≤ 4 passes and ≤ 3 corrections; under P-zero 1 and 0 | `:2013`, `:2034` | `if attempts == 3 \|\| worst >= prior {` … `attempts += 1;` |
| Jacobi ≤ 64 sweeps × 15 pairs | `rigid_body.rs:135`–`:139`, `:565`–`:569` | `for sweep in 0..64 {`, `for p in 0..6 { for q in p + 1..6` |
| One body at most runs a candidate search | `adapter :2020`–`:2034`, `:2095`–`:2108` | every status but `Restrained` returns `Err` |
| Radix loops ≤ 5 | `structural.rs:664`, `:924`, `:1625` | Lemma B |
| Carries ≤ 4 per `add_word` call, amortized; Φ₀ = 0 fresh | `exact_sum.rs:70`–`:76`, `:205`–`:210` | Lemma A |
| `norm3` ≤ 2 iterations, ≤ 4 `midpoint_sign` | `correct_norm/src/lib.rs:62`, `:66`, `:74`–`:83`, `:85`–`:108` | Lemma D |
| Wide loops capped | `retained/wide.rs:137`, `:139`, `:755`, `:794`, `:957`–`:987` | `0..129`, `(0..130u32).rev()`, `ATAN_MAX_REDUCTIONS = 5`, `ATAN_MAX_TERMS = 15` |
| `scale_pow2` ≤ 2 iterations | `rigid_body.rs:1083`–`:1089` | steps of ≤ 1,023 on an exponent in [−1,074, 1,023] |
| Union-find `root` ≤ N − 1 | `formation_check.rs:1151`–`:1157` | Lemma F |
| W4 candidates ≤ 391 + \|b\|, `k` ≤ 64 | `rigid_body.rs:651`–`:704`, `:902` | `for k in 1..=64_u32` |
| W4 has no ties | `adapter :2140` | `let ties: Vec<[usize; 2]> = Vec::new();` |
| Expansion length ≤ min(adds, 2,099) | `structural.rs:813`–`:832` | Lemma C |
| Σℓ_e ≤ C; differences nonzero only on the pattern | `adapter :1655`–`:1661`; `sparse.rs:309`–`:318` | "assembly entry outside the stiffness pattern" |
| F_e ≤ pipes | product `lib.rs:9046`, `:9052`, `:9062` | frames are pushed in the pipe loop that pushes each pipe once |
| Pattern rows strictly increasing | `sparse.rs:73`–`:76`, `:99`–`:102`, `:123`–`:125`, `:133` | `sort_unstable(); dedup();` then `other * DOF_PER_NODE + k` over sorted unique nodes, k < 6 |
| P-witness: one verification | `structural.rs:2510`–`:2518`, `:2561`–`:2568` | the guard repeats the verifier's arithmetic on the same four cells bit for bit, so a pair that passes the guard is a witness and returns. Without it, verifications ≤ n(n − 1)/2, each G1 + n² + 9n + n_g + 24: still bounded, but quartic |
| Load-fidelity rows unflagged under P-zero | `structural.rs:1390`–`:1400` | denominator 0 and residual zero give ratio 0 ≤ target |
| P-zero | `stiffness.rs:158`–`:159`, `:202` | §3, §9.1 |

## 8. Without P-zero (any force; the basis for T1.integrity)

τ is the number of force terms, and τ_i those of row i. An `ExactAccumulator` add costs 17 or 18 amortized (Lemma A); an `Expansion::add_product` on a residual ≤ 34 + 2X̄; an `Expansion::round` ρ_E(X̄) = 551 + 17X̄ (Lemma C). With τ terms, the following change:
- **PB rows:** + 17τ; couplings at 18 each.
- **AC prescribed rows:** + (C + Z)(35 + 2X̄) + n·X̄.
- **R:** per nonzero entry 36; `round_scaled` 412 per coupled row; 17τ; ≤ 4 passes and ≤ 3 correction solves (n + Σ + 3n each).
- **K:** + C(57 + 2X̄) + n(17X̄ + 583).
- **L:** + C(58 + 2X̄) + τ(50 + 2X̄) + 17n_g·X̄. Flagged rows add `row_sources`: τ_i clones of source ids and one sort (R4).
- **FIN:** `format!("{error:?}")` on an audit error (P-fmt).
- **Basis branch:** K-D5's corrections run their solves, its loads are re-formed, and `pow2` and `scale_up` (R6) are reached. That is not ledgered here.

T1's solves enter through `solve_structural_dense_binary64`/`factor_structural_cholesky` (`resource.rs:151`–`:155`), possibly over `AssemblyEvidence`. That path is not ledgered here.

## 9. The restraint-basis branch (`stiffness.rs:203`–`:205`)

It runs when `built.skew` is `Some`. The partition passed is `axis_restrained` (`:164`–`:167`), so p = |`axis_restrained`|. The gate's prescribed coordinates number p + ν ≤ n_g.

### 9.1 P-zero on coordinates

- `transform_force` of the zero ledger has no term (`restraint_basis.rs:1137`), so the coordinate force f′ = 0.
- `coordinate_values` gives 0 at every axis coordinate and pivot (`:1177`–`:1186`).
- K-D5's coordinate displacement `v` is 0. `add_weighted` returns at `x == 0.0` (`formation_check/skew.rs:66`). So the residual `rho` stays zero, `correction` returns zeros at `formation_check.rs:807`–`:809`, the corrections `w` and `w2` are 0, the bound `psi` is 0, and `scale_up(0, _)` returns at `:2554`.
- `source.loads` is the zero ledger's, which is empty (`adapter :1315`). So `load_closure` and `straight_frames_bound` are O(1), and `member_frame_index` (`formation_check.rs:1357`) is not reached.

### 9.2 Path and ledger

In call order:
1. **`skew::solve_coordinates`** (product `skew.rs:176`–`:198`):
   - `new_with_restraint_basis` (`adapter :1191`–`:1218`): the node-count check; `blocks` = s_k × `spring_block` (`restraint_basis.rs:1019`–`:1030`, 9 iterations);
   - `build` with node blocks: §4's AE, plus `adapter :1849`–`:1866` per skew spring. C gains 9s_k and G gains s_k;
   - `basis.clone()`: `axis_restrained` p, restrained coordinates ≤ p + ν, nodes ν, stops ν, springs s_k, and Λ_s bytes;
   - `frame_member_identities` as PREP;
   - `curved_sources` collect A (`skew.rs:196`).
2. **`solve_skew_with_formation_check`** (`adapter :1235`–`:1367`):
   - the prescribed map p; `direction_rows`: `ground_rows` (`restraint_basis.rs:1515`–`:1534`), p + ν + s_k;
   - `geometry` with W4 (`adapter :1287`–`:1298`): §4's GE with direction rows, plus W4's `screen` per unqualified body (§9.4);
   - `transform_stiffness` (`restraint_basis.rs:1083`–`:1117`), `reduce` (`:1208`–`:1224`), `transform_contributions` (`:921`–`:951`) and `transform_roundoff` (`:957`–`:991`);
   - `formation_source` (`adapter :2267`–`:2321`) and the dense view (`:1317`–`:1332`);
   - `prepare_formation_checked_structural` (`structural.rs:1432`–`:1438`), which is `prepare_bound` with C′ and ℓ̄′;
   - `solve_prepared` and the finish with C′, where FIN runs K-D5's formation check (§9.3).

| Group | Lines | Trip counts | Steps |
|---|---|---|---|
| PREP, AE, VIEW | as §4 | the same calls (`skew.rs:186`–`:196`) | as §4, with C and G including 9s_k and s_k |
| NB, node blocks | `adapter :1200`–`:1205`, `:1849`–`:1866`; `restraint_basis.rs:1019`–`:1030`, `:380`–`:382` | per skew spring: `spring_block` 631 (`norm_squared`: `new` 137, 3 `add_product`, `round` 412; `direction.scaled()` and `norm_squared(self.scaled)` read `[f64; 3]` out of borrowed data, 2 × 3), copy 10, 3 outer + 9 × (2 + β̂), group 1 | s_k(664 + 9β̂) |
| RBC, basis clone | `adapter :1216` | 2p + 4ν + 2s_k + Λ_s + 5 | as stated |
| SRC, curved sources | `skew.rs:196` | A copies | A |
| PRO, prologue | `adapter :1243`–`:1286`, `:1300` | prescribed check p; `ground_rows` + filter 2(p + ν + s_k); values p | 4p + 2ν + 2s_k + 5 |
| GE′, frame screen with direction rows | `adapter :1999`–`:2018`; `rigid_body.rs:108`–`:126` | GE, plus per body (ν + s_k)(1 + \|b\|) row scans; 3,884 per direction row (its formation is 30: GE's 18, the `[0.0; 6]` fill at `:113` 6 and its 6 iterations) | GE + 2N(ν + s_k) + 3,884(ν + s_k) + (p + S) |
| W4, constrained-body screen | `adapter :2086`–`:2250`; `rigid_body.rs:381`–`:849` | §9.4 | W4 |
| TS, `transform_stiffness` | `restraint_basis.rs:1065`–`:1117` | `check_node_blocks` ν(7 + 3β̂ + 2w̄); `to_vec` Z; per touched entry `transformed_entry` 966 + 11β̂ (`new` 137, `congruence_terms` 35 + 2β̂, ≤ 9 × (`get` β̂ + 2 + `add_triple` 40), `round` 412); pattern clone n_g + 2Z + 2 | ν(7 + 3β̂ + 2w̄) + 3Z + n_g + Z_ν(966 + 11β̂) + 6 |
| RD, `reduce` | `:1125`–`:1189`, `:1208`–`:1224`; `sparse.rs:851`–`:960` | `transform_force` n_g(18 + β̂) + `finish(n_g)` 621n_g; `coordinate_values` (p + ν)(β̂ + 2); `reduce_assembled_sparse_system` 5n_g + 4Z + 4(p + ν) + 554n̄ | n_g(650 + 2β̂) + ν(6 + β̂) + 4Z + 554n̄ + 12 |
| TC, `transform_contributions` | `:921`–`:951` | per contribution: 2 `translation`s, then 1 push or ≤ 9 pairs of `exact_triple_parts` (`:269`) + 4 pushes | C(108 + 6β̂) + 3 |
| TR, `transform_roundoff` | `:957`–`:991` | `!=` n_g + 2Z + 2; two `to_vec` 2Z; per touched entry 105 + 11β̂ (`congruence_terms`, ≤ 9 `find`s) | n_g + 4Z + 3ν + Z_ν(105 + 11β̂) + 4 |
| FS, `formation_source` | `adapter :2267`–`:2321` | frames F_e; members F_e + Λ_m; connectors 100K_c + 31T_r; per arc a scan of A, ≤ m_A macro `global_stiffness` (17,001) + 145, a push, or a P-fmt name ≤ 64 + λ_A | 2F_e + Λ_m + 100K_c + 31T_r + A(A + 17,146m_A + λ̄_A + 66) + S + 3 |
| DV, dense view | `adapter :1317`–`:1332` | `symmetry_basis` E_el + 512; `to_dense` n_g² + 2n_g + Z; two fills and the copy 2n_g² + 3n_g + Z | as VIEW, which it replaces |
| Gate | `adapter :1333`–`:1347`; `structural.rs:1432`–`:1438` | PB, CS, D, AC, G1, HC, R, K, L, FIN, WIT as §4, with C′ for C, ℓ̄′ for ℓ̄, G + s_k for G | §4's rows |
| KD5 | `structural.rs:1968`–`:1978`; `formation_check.rs:489`–`:535`; `formation_check/skew.rs:271`–`:441` | §9.3 | KD5 |
| DROP′ | | DROP, plus K′ (the coordinate stiffness), f′'s `by_dof` n_g, the source's members and connectors, the basis clone's strings | DROP + 4n_g + 2F_e + 2K_c + 2A + 2ν + 2s_k + 20 |

### 9.3 K-D5 on coordinates (KD5)

`check_at` (`formation_check.rs:507`–`:530`) → `skew::evaluate`. FIN runs it unless the case is already Sensitive (`structural.rs:1964`–`:1969`). It is charged always.

**Element re-formation at 128-bit precision** (Lemma E units), once per element per `intended_stiffness_pass` (`formation_check.rs:727`–`:750`). The "base" column counts each row with Lemma E's operation charges without their copies (`add` 31, `sub` 32, `mul` 21, `div` 277, `sqrt` 408, `neg` and `abs` 1, `mul_pow2` 4). Lemma E's copies raise every operation by at most 1.22 times, except `neg`, `abs` and `mul_pow2`, which add 3 each. So each row is charged ≤ 1.22·base + 3m + c, where m counts its `neg`, `abs` and `mul_pow2` calls and c its other copies under §1.1.

| | Lines | Main parts of the base | Base | m | c | Steps |
|---|---|---|---|---|---|---|
| κ^F_f, frame | `:2694`–`:2827`, `:2609`–`:2678` | `chord_axes` 3,388; leaves 712; `coefficients` × 2 (Timoshenko) 3,350; fill and patterns 226; `rotate` 16 blocks × 3,034 + 148 = 48,700 | 56,379 | 16 (`pair` 4, the two bending patterns 12, `:2798`–`:2817`) | 9 (`frame_matrix`'s three arrays out of `e`, `:2695`) | 68,840, charged **68,900** |
| κ^F_c, connector | `:2835`–`:2936` | `connector_b` 3,972; lifts 258; `kb` 72 × 331; `out` 144 × 331; fills 216 | 75,996 | 30 (`half` 3, `skew` 9, `qc.neg()` 18) | 95 (`node_i()`/`node_j()` 8, `offsets()` 6, `skew`'s by-value `v[k]` reads 3 × 12, `axes()` 9, `stiffness()` 36) | 92,901, charged **93,000** |
| κ^F_r, tie rod | `:2942`–`:2996` | `tie_rod_c` 3,010; 12 × 23 + 78 × 24; fill 144 | 5,229 | 6 (`:2965`–`:2966`) | 23 (`axes()` 9, `local_offsets()` 6, `node_i()`/`node_j()` 8) | 6,421, charged **6,500** |
| κ^F_a, arc | `:3100`–`:3414` | `curved_geometry_values` 11,076 (`included_angle` 9,304); flexibility 21 × 4,034 (4 `quad` of 692); `invert6` 44,121; axes 3,170; `mul6` × 2 23,916; `rotate` 48,700; rest 1,108 | 216,845 | 152 (geometry `mul_pow2` 4; `included_angle` 17; flexibility `mul_pow2` 4; `h` 3; the coupled blocks 72; `unit_load_actions` 7; `invert6` `abs` 30 and `mul_pow2` 15) | 53 (`e`'s three arrays 9; `*radius` 4, `:3319`; `*t` 4; `invert6`'s `let mut m = *f`, 36 elements, `:3372`) | 265,060, charged **265,100** |

Σκ^F = 68,900F_e + 93,000K_c + 6,500T_r + 265,100A.

**The evaluation, in order:**
1. **SA, `subtract_action`** (`skew.rs:85`–`:131`), three times (`:340`, `:379`, `:401`). Each costs Σκ^F + E_el(6,500 + 576β̂) + S(50 + 4β̂) + s_k(2,577 + 36β̂) + 4. Per element entry, `entry` is ≤ 44 + 4β̂: `q_row(i)` 8 + β̂, then ≤ 3 × (1 + `q_row(j)` + ≤ 3 `add_weighted` returns); 144 entries give ≤ 6,336 + 576β̂. Per skew spring, `skew_spring_matrix` (`skew.rs:36`–`:54`: 3 `add`, 15 `mul`, 6 `div`) costs 2,164, and its `spring.direction.authored()` argument reads an array out of borrowed data, 3.
2. **CORR, `correction`**, twice: 71n + n_g + 2 each (`scaled_exponent` returns at `is_zero`, `:1119`).
3. **JUDGE** (`skew.rs:345`–`:375`):
   - `lift` n_g + ν(1,684 + 3β̂);
   - `physical_free` n_g(2 + β̂) (R3k);
   - **BS, `body_scales`** ≤ 9N² + 2N·E_el + 2E_el + 28N (Lemma F; its edges are E′ = F_e + K_c + A ≤ E_el, `formation_check.rs:1159`–`:1174`);
   - the loop n_g(26 + β̂).
4. **Norms and σ** (`:383`–`:407`): 31n + 79n (two `mul_pow2` per row, `:386`–`:387`), and the threshold's `mul_pow2` 7.
5. **EPS, `epsilon_e`** (`skew.rs:136`–`:267`):
   - Σκ^F;
   - per element 18 + 144 × (4,826 + 4β̂), where the `formation` closure is `new` 137, `split` 18, 4 adds, and ≤ 9 pairs × 507 (`exact_abs_scaled_up` 482);
   - per skew spring 46,300 + 36β̂ (`skew_spring_matrix` 2,164 and its argument 3 included);
   - S;
   - the assembly part: fill 137n, per row a reset of 139n plus a scan of C′, 17C′ adds, and 1,226 per pair (two `exact_abs_scaled_up`, a projection `new`, the `mul_pow2` at `:253`, a split and adds), so 137n + 1,365n² + n·C′ + 17C′;
   - folds 4n.
6. Rows, `psi`, the index fill and the tail: 139n + n_g + 330, plus λ̄_A for a refused-check detail.

Summed: Σκ^F 3 + 1; E_el 3 × 6,500 + 694,962 ≤ 714,500; S 3 × 50 + 1; s_k 3 × 2,577 + 46,300 = 54,031; n 142 + 110 + 137 + 4 + 139 = 532 ≤ 533; n_g 2 + 29 + 1 ≤ 33; constants 12 + 4 + 7 + 330 ≤ 460.

> **KD5** = 4Σκ^F + E_el(714,500 + 2,304β̂) + S(151 + 12β̂) + s_k(54,031 + 144β̂) + 1,365n̄² + n̄·C′ + 17C′ + 533n̄ + n_g(33 + 2β̂) + ν(1,684 + 3β̂) + BS + λ̄_A + 460.

### 9.4 W4, the constrained-body screen

`geometry` calls `screen` for every unqualified body (`adapter :1979`–`:1982`). Such a body then skips the frame screen (`:1986`). Per body b, with \|b\| nodes, g_b ground rows, σ_b ≤ \|b\| sub-bodies sb and L_b links (its edges passed to `objective_sub_bodies`, Σ L_b ≤ E_el):
- **`W4Context::body`** (`:2117`–`:2218`):
  - `to_vec` \|b\|, then **R3a** HS(\|b\|);
  - the rods sum K_c, and `curved_points` \|b\|;
  - the edge loop over **all** E_el edges, at 1 + 2β̂ each (**R3b**, two `local`s);
  - per arc edge in the body (each arc once overall): a clone λ_A, and `w4_curved_source` (`:2226`–`:2250`), which scans A and runs ≤ m_A macro `global_stiffness` at 17,146 (17,001 + 145 bit compares);
  - points 2\|b\|;
  - grounds (p + S)(1 + β̂) (**R3b**).
- **`objective_sub_bodies`** (`rigid_body.rs:381`–`:418`): ≤ 4\|b\| + 7L_b + Σ HS(\|sb\|) (**R3c**) + 3σ_b.
- **`assess_constrained_bodies`** (`:494`–`:705`), restrained path:
  - `reduce_constrained_body` (`:709`–`:849`): 33\|b\| + Σ HS(\|sb\|) (**R3d**) + HS_e(σ_b) (**R3e**) + HS(0) (**R3f**; no ties, `adapter :2140`) + HS_g(g_b) (**R3g**) + 4σ_b + 10g_b + 10. A ground row is ≤ 10: `match *ground` copies a directional row's `direction` out of borrowed data (`:751`, 3), its two scans 6, and the body 1. With σ_b > 1 and no ties it returns `INVALID` at `:830`, which is cheaper;
  - `rounded3` 1,756\|b\|;
  - rows 45g_b;
  - Jacobi 1,920g_b + 7,168 (`:565`–`:600`);
  - the rest 81.

Here HS(w) = 3w(⌊log₂ w⌋ + 1) + 2, and the keyed sorts cost HS_k(w; κ, c) = HS(w) + 2w(⌊log₂ w⌋ + 1)(2κ + c), with κ one key's computation and c one comparison of two keys (Part II, "Common algorithms"). R3e's key costs κ = 1 and c = 1, so HS_e(w) = 9w(⌊log₂ w⌋ + 1) + 2. R3g's costs κ = 7 and c = 4, so HS_g(w) = 39w(⌊log₂ w⌋ + 1) + 2. ⌊log₂ w⌋ + 1 ≤ β̂ − 1 for every w on the branch. Summed over the U bodies (Σ\|b\| ≤ N, Σg_b ≤ p + S):
- per body, constants 10 + 7,168 + 81 + HS(0) 2, plus the HS constants of R3a, R3e and R3g (2 + 2 + 2): 7,267 ≤ 7,280;
- per node, 1 + 1 + 2 + 4 + 33 + 1,756 + 7 (σ_b ≤ \|b\|) = 1,804, plus 2 + 2 for R3c–d's constants, and 18(β̂ − 1) for R3a, R3c, R3d (3 each) and R3e (9): ≤ N(1,809 + 18β̂);
- per ground row, 10 + 45 + 1,920 + 39(β̂ − 1) for R3g: ≤ (n_g + S)(1,988 + 39β̂).

> **W4** ≤ U·[E_el(1 + 2β̂) + (n_g + S)(1 + β̂) + K_c + 7,280] + N(1,809 + 18β̂) + 7E_el + (n_g + S)(1,988 + 39β̂) + A(λ̄_A + A + 17,146m_A + 7).

**Refusal paths (each at most once, exclusive):**
- **W4-MECH** (`rigid_body.rs:632`–`:705`, `:865`–`:1012`) = (391 + N)(174,400N + 137(n_g + S) + 84,000) + n_g + 7N + 10,300. Each candidate is a prefilter 7g_b, `null_translations` 530\|b\| + 2g_b, and `canonical` ≤ 64 × (1,308 + 2,716\|b\| + 2g_b).
- **MECH′** (frame screen with direction rows) = (7 + 2N)(5,800N + n_g + S + 2,761(ν + s_k) + 16) + 15N + n_g + 55. A direction row costs ≤ 2,761 per candidate (`rigid_body.rs:271`–`:278`, ≤ 30 `add_product`s on a growing expansion).

### 9.5 R3's eleven sites (every one on this branch)

Each is counted at its replacement's cost (Part II R3). LB is ≤ β̂ steps.

| | Site | Calls (P-zero) | Steps charged in |
|---|---|---|---|
| R3a | `adapter :2124` `nodes.sort_unstable()` | 1 per W4 body | W4: Σ HS(\|b\|) ≤ 3N(β̂ − 1) + 2U |
| R3b | `adapter :2125` `nodes.binary_search` | 2E_el + p + S per W4 body | W4: U(2E_el + n_g + S)β̂ |
| R3c | `rigid_body.rs:414` `body.sort_unstable()` | 1 per sub-body | W4: ≤ 3N(β̂ − 1) + 2N |
| R3d | `rigid_body.rs:725` `body.sort_unstable()` | 1 per sub-body | W4: ≤ 3N(β̂ − 1) + 2N |
| R3e | `rigid_body.rs:728` `sort_unstable_by_key(body[0])` | 1 per W4 body | W4: HS_e, ≤ 9N(β̂ − 1) + 2U |
| R3f | `rigid_body.rs:748` `ties.sort_unstable()` | 1 per W4 body, on 0 ties | W4: 2U |
| R3g | `rigid_body.rs:789` `sort_unstable_by_key(GroundRow::key)` | 1 per W4 body | W4: HS_g, ≤ 39(n_g + S)(β̂ − 1) + 2U |
| R3h | `restraint_basis.rs:861` `binary_search_by_key` (through `translation`) | ≤ 22C + 4Z_ν + 2n_g + 3ν: TC 6C; TS and TR 2Z_ν each; `transform_force` n_g; KD5's three SAs 12C, EPS 4C, JUDGE n_g, `lift` 3ν | the β̂ terms of TC, TS, TR, RD and KD5 |
| R3i | `restraint_basis.rs:1070` `first.binary_search` | 3ν | TS: 3νβ̂ |
| R3j | `restraint_basis.rs:1178` `axis_restrained.binary_search` | p + ν | RD: (p + ν)β̂ |
| R3k | `formation_check/skew.rs:348` `axis_restrained().binary_search` | n_g | KD5 (JUDGE): n_gβ̂ |

### 9.6 The closed form (basis branch, P-zero)

> **c̄_basis(T2.integrity)** = 32·n_g² + 38·n̄² + 9·n̄·n_g + 29·Z̄ + C′·(3,400 + 105·ℓ̄′) + 2β̂·C + 1,200·n_g + 1,900·n̄ + 3(G + s_k) + 191·E_el + 3S + N + 2F_e + 2A + K_c + 1,700
> + GE + MECH′ + W4-MECH + WIT + AE_el + PREP_el + Δ_basis,

> **Δ_basis** = 275,600F_e + 372,000K_c + 26,000T_r + 1,060,400A
> + E_el(714,600 + 2,304β̂) + U·E_el(1 + 2β̂) + C(108 + 6β̂) + C′(n̄ + 17)
> + 1,365n̄² + 1,087n̄ + n_g(700 + 4β̂) + (n_g + S)(1,990 + 39β̂) + U(n_g + S)(1 + β̂)
> + 11Z̄ + Z_ν(1,071 + 22β̂) + S(153 + 12β̂) + s_k(54,701 + 153β̂) + ν(1,708 + 7β̂ + 2w̄) + (2N + 3,884)(ν + s_k)
> + 9N² + 2N·E_el + N(1,837 + 18β̂) + A(2A + 34,292m_A + 2λ̄_A + 76) + 4F_e + 102K_c + 31T_r + U(K_c + 7,280)
> + Λ_m + Λ_s + λ̄_A + 518.

How the rows map to it:
- The first line is §6's charge with the gate's C′ and ℓ̄′. The gate's rows give C′(3,396 + 105ℓ̄′); AE's C(1 + β̄) and its other β̄ terms give C + (2C − 9s_k)β̄ ≤ C + 2β̂C. P0 charges C′ = 36C ≥ C, so C′·3,400 covers C′·3,396 + C.
- Δ_basis's first line is KD5's 4Σκ^F.
- E_el: KD5 714,500 + W4 7 + BS 2 ≤ 714,600; 2,304β̂ from KD5. U·E_el from W4. C from TC. C′(n̄ + 17) from KD5.
- n̄: KD5 533 + RD 554 = 1,087. n_g: RBC 2p + PRO 4p + GE′ p (7, with p ≤ n_g) + TS 1 + RD 650 + TR 1 + KD5 33 + DROP′ 4 = 696 ≤ 700; 4β̂ from RD and KD5. (n_g + S) and U(n_g + S) from W4.
- Z̄: TS 3 + RD 4 + TR 4. Z_ν: TS and TR. S: GE′ 1 + FS 1 + KD5 151. s_k: NB 664 + RBC 2 + PRO 2 + KD5 54,031 + DROP′ 2 = 54,701; 9β̂ + 144β̂ = 153β̂. ν: RBC 4 + PRO 2 + TS 7 + RD 6 + TR 3 + KD5 1,684 + DROP′ 2 = 1,708; 3β̂ + β̂ + 3β̂ = 7β̂; 2w̄ from TS. (2N + 3,884)(ν + s_k) from GE′.
- 9N² + 2N·E_el + 28N from BS; N(1,809 + 18β̂) from W4. A: SRC 1 + W4 (A + 17,146m_A + λ̄_A + 7) + FS (A + 17,146m_A + λ̄_A + 66) + DROP′ 2. F_e: FS 2 + DROP′ 2. K_c: FS 100 + DROP′ 2. T_r: FS 31. U(K_c + 7,280) from W4.
- Λ_m from FS, Λ_s from RBC, λ̄_A from KD5. Constants: RBC 5 + PRO 5 + TS 6 + RD 12 + TC 3 + TR 4 + FS 3 + KD5 460 + DROP′ 20 = 518.

- P0 charges C′ = 36C (μ = 36), ℓ̄′ = min(36δ̄, 2,099), U = min(N, K_c + A), and m_A = A unless it computes m_A.
- GE, WIT, AE_el and PREP_el are §6's. MECH′ and W4-MECH are §9.4's.
- Every coefficient is positive.
- **Order of growth.** The branch adds ≈ 7.1·10⁵ + 2,304β̂ per element, plus 4κ^F (2.8·10⁵ per frame, 1.1·10⁶ per arc). The largest product term is n̄·C′ ≤ 36·n̄·C. It is still not cubic.

## 10. Allocation sizes (requested element counts; resident memory not stated)

Plain path:

| Allocation | Lines | Size |
|---|---|---|
| contributions / groups | `adapter :1731`–`:1732`, pushes | ≤ C × 24 B (amortized growth to < 2C) / G × 8 B |
| coordinates / edges / spring_ground | `:1733`–`:1735` | N × 32 B / E_el × 24 B / ≤ S × 8 B |
| formation primitives | `:1736`–`:1752` | F_e frames, K_c connectors (with rods), A slots (12×12 f64 + id), S springs |
| pattern clone + 4 slot vectors | `:724`–`:728` | 8(n_g + 1) + 16Z, then 4 × 8Z |
| curved / springs / prescribed lists | `stiffness.rs:194`–`:202` | A slots, S × 16 B, p × 16 B |
| `frame_member_identities` | product `lib.rs:4891`–`:4894` | 2 `HashSet<usize>` (A, K_c keys), which R5 removes; F_e strings (Λ_m bytes) |
| `seen`, `body`, `coordinates`, `ground`, Jacobi | `adapter :1943`–`:1998`; `rigid_body.rs:73`–`:128` | N bools; ≤ N per body; 24 B × \|b\|; ≤ p + S; 48 B × g_b (× 2) |
| dense K view / two symmetry views | `sparse.rs:311`; `adapter :848`–`:849` | 8n_g² + 24n_g each (3 of them) |
| `sums`, `differences` (built 3× / 1×) | `structural.rs:944`, `:1054` | 32 B × n_g² + their term vectors (Σ ≤ C / C + Z f64). `Expansion::add` allocates a fresh vector on every call (`:813`) |
| `rounding` records | `:1076` | ≤ Z × (56 B + two term vectors) |
| `exponents`, `a`, `rhs` | `:1498`, `:1518`–`:1519` | 4n, 8n² + 24n, 8n |
| `ElementAbsorption` `free` (× 3) | `:1001` | n_g bools |
| `ExactAccumulator` (stack) | `exact_sum.rs:205` | 1,088 B each, transient |
| solves, Hager vectors | `structural.rs:2047`, `:1814`–`:1849` | n f64 per solve (≤ 11 + 1 + 3), x, signs, alt |
| u, residual rows, correction | `:1931`, `:1669`, `:2020` | n_g f64; ≤ n `ResidualRow`; n |
| load fidelity | `:1253`, `:1261`, `:1272` | n_g empty vectors, n_g bools, parts ≤ Σ_j ℓ_ij per row (ℓ_ij: entry (i, j)'s expansion length) |
| report clones | `:1995`–`:2002` | n `PivotEvidence`, n i32, rounding deep copy |
| refusal directions | `:1502`, `adapter :2023`, `structural.rs:2490`, `:2563` | n_g f64 / n f64 |

Basis branch, beyond the plain path:
- K′ values Z and its pattern clone;
- `transform_contributions` ≤ C′ × 24 B (`restraint_basis.rs:926`, then `extend`);
- `transform_roundoff` 2Z (`:968`–`:969`);
- f′'s ledger, n_g `by_dof` vectors;
- the reduction, n_g + n + p + ν;
- the dense view, 3 × (8n_g² + 24n_g);
- K-D5: `rho` and `epsilon_e`'s `sums` are n̄ × 1,088 B each; `index` n_g; `body_scales` ≈ N × 152 B + E′ × 64 B; `physical_free` and `lift` n_g;
- W4 per body O(\|b\| + L_b + g_b), with ≤ 3\|b\| `Expansion`s of ≤ 9 terms;
- the basis clone, p + 2ν + s_k + Λ_s B.

## 11. Foreign steps (file:line)

| # | Step | Where | Calls | Disposition |
|---|---|---|---|---|
| F1 | `slice::binary_search` | `sparse.rs:190` (`SparsePattern::find`), through `adapter :1657` from `:1666`, `:1677`; basis: `restraint_basis.rs:978` (TR) and, through `SparseStiffness::get` (`sparse.rs:299`), `restraint_basis.rs:1108` (TS) | 2C; basis + ≤ 18Z_ν | R1 |
| F2 | `HashSet` collect, `contains` | product `lib.rs:4891`, `:4892`, `:4896` (`frame_member_identities`, from `stiffness.rs:208`) | A + K_c inserts, 2·pipes lookups | R5: `std` documents these only as expected O(1), so they have no worst-case constant even as a foreign step |
| F3 | `f64::powi` | `structural.rs:666`, `:928`, `:929`, `:1627`; basis: `formation_check.rs:1139`, `:2559` | each radix iteration; the basis sites are unreached under P-zero | R6 |
| F4 | float and integer methods | §1.3 | many | P-fp, adopted |
| F5 | `format!` | §1.3 | 1 / pipes / ≤ 1 on the plain path; fail-closed paths on the basis branch | P-fmt, adopted |
| F6 | sorts and searches, basis branch | the eleven R3 sites (§9.5) | §9.5 | R3 |
| F7 | `f64::mul_add` on a target without a fused multiply-add instruction (x86_64) | `structural.rs:852`; `correct_norm/src/lib.rs:79`, `:162`; `connector.rs:770`; `tie_rod.rs:454`; basis: `restraint_basis.rs:190`, `:261` | every `Expansion::add_product`, `norm2`/`norm3` and `triple` | §12, item 2 |
| F8 | 128-bit integer division | `retained/wide.rs:818` (`sqrt`); `wide/multi.rs:196`–`:197` (`word_at`, from `to_binary64`), `:711` (`half_quantum_over`); `:578` (`Wide<L>`'s `sqrt`, off this path) | every wide `sqrt` and `to_binary64`, so τ̄ on the plain path | R8 |
| — | `correct_norm`'s ulp loop | `correct_norm/src/lib.rs:85` | N + ≤ 960 per body | closed by Lemma D (repository source, ≤ 2 iterations) |

Element primitives on the plain path (§1.2; no replacement):
- `.sum()`: `structural.rs:1080`, `:1082`, `:1118`, `:1822`, `:1840`, `:1863`; `rigid_body.rs:121`, `:140`–`:142`, `:171`.
- `max_by`/`min_by`: `structural.rs:1838`; `rigid_body.rs:175`.
- `contains`/`position`: `adapter :1973`, `:1997`.
- `windows`: `structural.rs:996`.
- The derived pattern `PartialEq`: `adapter :1068`.
- The clones, `to_vec`s and `vec!` fills listed in §10.

Off both paths:
- `sparse.rs:74`–`:75` and `:100`–`:101`: sort and dedup in K's formation (`from_connectivity`, `assemble_sparse_stiffness`, `sparse.rs:735`), outside §5.3.8's table (`resource.rs:367`) (R2);
- `sparse.rs:154`, `:779`, `:787`, `:795`, `:816`, `:1426`: finds in K's formation and transposes;
- `sparse.rs:234`: `from_pattern_and_contributions`, only from `from_contributions` (`:250`), not on the modal path;
- `sparse.rs:751`: `#[cfg(any(test, feature = "mutation-controls"))]` only;
- `sparse.rs:299` from `stiffness.rs:241`: the pencil's K_ff loop, modal work outside every §5.3.8 term;
- `SparseInteractive`, `sparse.rs:1188`–`:1945` (sorts and searches at `:1188`, `:1589`, `:1601`, `:1648`, `:1838`, `:1945`);
- `row_sources`, `structural.rs:1306`–`:1307`: T1, and a flagged load-fidelity row, which P-zero excludes (R4);
- `restraint_basis.rs:754`, `:776`, `:819`, `:1199`–`:1200`: basis construction and `judge_prescribed`, not called from `solve_coordinates`;
- `formation_check.rs:1357`: `member_frame_index` has no caller on this path (§9.1);
- `formation_check.rs:2528`: a constant 2⁻⁹⁰⁰ in a load re-formation, unreached without loads.

## 12. Open items and decisions

**Decided by Agent 0** (2026-11-09; each was provisional on the independent review raising no objection to it, and it raised none):
- **P-fp and P-fmt are adopted as pricing premises** (§1.3, with each one's rationale and its exclusions by name).
- **δ̄ is computed with one array of N counters in P0.** P0 charges that allocation in its own account: one `vec![0; N]` of `usize` (8N bytes) and ≤ 2N + E_el + S + s_k steps (the fill, one iteration per element, spring and skew spring, and the maximum scan).
- **R5 is approved in principle.** It is implemented with R1, R3 and R6 after the pressure-elongation unit merges, with its re-pins, and a D3 turn if coefficients fall.

**Open:**
1. **Implementation.** R1, R3, R5, R6 and R8 are specified, not built; R8 also awaits Agent 0's approval. Until they land, F1–F3 (and F6 with a basis) and F8 keep T2.integrity unbounded. Each also needs the re-pins listed in Part II.
2. **`mul_add` on x86_64** (F7; §1.3 excludes it there by name). On x86_64 builds T2.integrity is unbounded on every request until one of these is chosen:
   - build x86_64 with the `fma` target feature (FMA3), so `mul_add` is one instruction with the same bits; this sets a hardware floor (FMA3 CPUs) and is a build and support choice;
   - an in-repo correctly rounded fused multiply-add (the exact 106-bit product as a `u128`, the addend aligned, one rounding), the same bits by IEEE 754's definition, priced from source, and reviewed as a change of numerical code;
   - price the platform's `fma` routine as a foreign step, the standing the Design declined for `binary_search`.
   *Recommendation:* the target feature, if the hardware floor is acceptable; otherwise the in-repo routine.
3. **The weights.** The charged coefficients (§6, §9.6) are not yet adopted as P0's weights for `T2.integrity`.
4. **An independent literal audit of this ledger** is still required before adoption. It is a numerical-resource claim. The 2×tok constants (§5, Lemma E, §9.3) and the hand-rounded constants (`flexibility_of`, `invert_symmetric6`, `solve_dense(6)`) are sound over-counts under §1.1, not literal counts. Agent 0 accepted three small points, none of which moves a charge:
   - κ_c and κ_r state totals above the sums of their listed parts (by 438 and 91 steps, §5.3–§5.4). The charges keep the larger totals.
   - K's and L's per-row parts, as listed in §4, sum to one step more than their rows charge (584 against 583, and 1,116 against 1,115 per row). K's unreached `round` (551) covers its row under P-zero. The charged coefficients 1,900n̄ and 1,200n_g keep 18.5n̄ and 12.5n_g of slack over the row sums, so the charge holds either way.

---

# Part II. Priced in-place replacements for the foreign steps (specification only)

**Nothing here is built.** Agent 0 approved R5 in principle, to be implemented with R1, R3 and R6 after the pressure-elongation unit merges (Part I §12); R8 awaits its approval. Units, premises, lemmas and dimensions are in Part I §1–§2.

"Result-identical" means the same returned values, the same `Err` variants at the same first failing input, and the same panics, for every input the callers can pass. The review's threat model is "a replacement that changes any static result or retained coefficient". So each replacement states:
- the premise that makes its result identical, quoted from source;
- what it does to retained pricing: coefficients, loop classification and pins.

### Common rules

- Every replacement allocates nothing new; it uses indices and swaps only. Only R5 removes allocation sites (two `HashSet`s).
- Each new `while`/`for` header is a loop to loopscan (`base/loopscan.py:15`). On a priced route it needs a header classification with its bound and "no site inside". A loop with no allocation and no text site multiplies nothing, so a classified header leaves coefficients equal.
- **Line neutrality.** Keep each touched function's header line, and add no line above a keyed line. A helper goes at the end of its file, below every key. This keeps the linemap carry trivial.
- **Full-file pins.** `tools/retained_pricing` pins seven of the touched files by SHA-256 of their bytes. `price.py` raises `ValueError` on any byte change until the pin is re-registered. Each pin's digest matches its file at bf1bdd2.

  | File | Pin | Raised as |
  |---|---|---|
  | `frame_kernel/src/structural.rs` | `base/b7a_formation_model.py:52` | "B7a formation owner source changed" (`:113`) |
  | `frame_kernel/src/structural/sparse.rs` | `base/b7a_formation_model.py:64` | the same |
  | `nonlinear_integration/src/structural_adapter.rs` | `base/b7a_domain_model.py:237`; `base/w1_reductions_model.py:113` | "B7a positive-domain owner source changed" (`:338`); "W1 reductions source/lifetime pin changed" (`:138`) |
  | `frame_kernel/src/structural/formation_check.rs` | `base/b7a_domain_model.py:237`; `base/w1_reductions_model.py:110` | the same two |
  | product `lib.rs` (compared through `ce_fallback_model.py`'s `s6_bytes`) | `base/b7a_domain_model.py:237`; `base/w1_reductions_model.py:105` | the same two |
  | `structural/retained/wide.rs` | `base/w1_member_load_model.py:100` | "W1 stage2 source/owner pin changed" (`:196`) |
  | `structural/retained/wide/multi.rs` | `base/w1_member_load_model.py:101` | the same |

  `rigid_body.rs`, `restraint_basis.rs` and `formation_check/skew.rs` carry no full-file pin, only `o10_rules.py` text guards. A re-pin follows the existing D3 pattern: the reviewed hunks are recorded in the pin's comment, and the new digest replaces the old. Replacements whose coefficients do not move may share one D3 turn (standing delegation, "Proportionate review").

### Common algorithms

**LB, lower-bound search** on `s[lo0..hi0)`, ascending by key k(·):

```
let (mut lo, mut hi) = (lo0, hi0);
while lo < hi { let mid = lo + (hi - lo) / 2; if k(&s[mid]) < key { lo = mid + 1 } else { hi = mid } }
// found iff lo < hi0 && k(&s[lo]) == key
```

- **Steps.** ⌊log₂ w⌋ + 1 iterations of ≤ 10 operations each, plus the call: ≤ ⌊log₂ w⌋ + 2, where w = hi0 − lo0.
- **Equivalence.** On a slice whose keys are pairwise distinct, `binary_search`/`binary_search_by_key` return `Ok(k)` for the unique matching k, or `Err(_)`; `std`'s caveat for multiple matches cannot arise. LB returns the first index with key ≥ the target, so it finds the same unique k exactly when one exists. Every caller below reads only `Ok`'s index, `.ok()` or `.is_err()`, never `Err`'s insertion point.

**HS, in-place heapsort** (bottom-up `sift_down` with `swap`) on `&mut [T]` of length w:
- comparisons ≤ 2w + 2w⌊log₂ w⌋;
- steps HS(w) ≤ 3w(⌊log₂ w⌋ + 1) + 2, where comparing two elements is one step;
- keyed: where a comparison computes two keys, or compares more than a scalar, each comparison also costs 2κ + c, with κ one key's computation and c one comparison of two keys, both in steps counted literally under §1.1 at that site. So HS_k(w; κ, c) = HS(w) + 2w(⌊log₂ w⌋ + 1)(2κ + c). Each keyed site states its κ and c (R3e, R3g, R4);
- no allocation. Precomputing the keys would need a buffer of w keys, a new allocation, so it is not used.

HS is unstable. It is used only where elements with equal keys are indistinguishable, or where every order gives the same result, so stable `sort`, `sort_unstable` and HS give the same outcome.

**DD, in-place dedup.** Keep a write index k; for each i, if k == 0 or v[i] ≠ v[k − 1], swap v[k] and v[i] and increment k; then `truncate(k)`. That is w + 1 steps. It keeps the first of each run, as `Vec::dedup` does, and the capacity is untouched.

---

### R1. `SparsePattern::find` (`sparse.rs:185`–`:191`), the plain path's only search

**Replace** `self.row(row).binary_search(&col).ok().map(|k| start + k)` with LB over `self.columns[self.row_starts[row]..self.row_starts[row + 1]]`, returning `Some(lo)` when found. Keep the `row >= self.dimension → None` guard at `:186`–`:188`.

**Premise: rows strictly increasing.** `from_sorted_rows` documents "Rows must be sorted, deduplicated and structurally symmetric" (`:133`). Its callers satisfy it:
- `from_positions`: `row.sort_unstable(); row.dedup();` (`:74`–`:75`);
- `from_connectivity`: `list.sort_unstable(); list.dedup();` (`:100`–`:101`), then `other * DOF_PER_NODE + k` for sorted unique `other` and k < 6 (`:123`–`:125`);
- `from_dense`: j ascending (`:263`–`:266`);
- `from_pattern_values` (`:281`) reuses a pattern.

**Callers covered by this one edit:** `adapter :1657` (from `:1666`, `:1677`); `restraint_basis.rs:978` and, through `SparseStiffness::get` (`sparse.rs:299`), `restraint_basis.rs:1108`; K's formation and transposes (`:154`, `:779`, `:787`, `:795`, `:816`, `:1426`); `:234`; `:751` (test only). The `.expect(..)` panics see the same `None`.

**Steps:** β̄ = ⌊log₂ n_g⌋ + 2 per call; 2C calls on the plain path, plus ≤ 18Z_ν on the basis branch (≤ 9 per touched entry in TS and in TR, Part I §9.2).

**Static results.** An oracle test (test-only) compares a verbatim copy of the former `find` with the new one by exact `Option<usize>`. Cases: empty rows, width 1, a column below the first, above the last, between entries and absent, `row == dimension`, `row > dimension`, and every stored entry of the 35 population models' formed K patterns. Plus the suites:
- `cargo test -p open_pipe_stress_frame_kernel` (sparse, absorption, `restraint_basis_tests.rs`);
- `-p open_pipe_stress_nonlinear_integration` (dense/sparse parity, `actual_adapter_*`);
- `-p open_pipe_stress_product_physics` (`rt_a3_nonlinear/`, `s11f/rf_cancel_cases.json`, `u4b_preview_shear_moduli.json`, and `modal/tests_resource.rs` on the 35 models);
- result_export's committed `*-dense_scrutiny.raw.json` and `*-sparse_interactive.raw.json`, unchanged, including the `normalized_residual: -0.0` values that `structural.rs:1188`–`:1190` cites.

**Pricing.**
- **Re-pin** `b7a_formation_model.py:64` (sparse.rs).
- `fnkey("find", FK_ + "structural/sparse.rs")` (`rules/o10_rules.py:3410`, `:3422`, `:3429`) resolves only while sparse.rs has exactly one `fn find`. So keep the name, add no second `fn find`, and keep it at line 185.
- The new header `while lo < hi` needs an `O10_LOOPS_LAST` entry with that exact header: bound "⌊log₂(row width)⌋ + 1; no site inside", limited to `structural/sparse.rs:find`.
- `price.py run <head>` completes with no unmapped header; `price.py compare` shows every coefficient equal both ways (`a_le_b`); `summary.json`'s `linemap.pruned` lists no rule; AC-004 7/7 on the composed binary.

### R2. Pattern sorts (`sparse.rs:74`–`:75`, `:100`–`:101`): K's formation, not T2

Replace `sort_unstable(); dedup();` with HS + DD.
- **Premise:** `usize` elements, so equal values are indistinguishable.
- **Steps:** 3w(⌊log₂ w⌋ + 1) + w + 3 per list.
- **Evidence:** R1's suites, `SparseStorageCounts`, an oracle on random `usize` lists, classified headers (one shared helper at the end of sparse.rs serves both), and R1's re-pin. Coefficients equal.

It is needed only if Agent 0 brings K's formation under a term. `resource.rs:367` places it outside §5.3.8's table.

### R3. The restraint-basis branch's eleven sorts and searches

All lie on `stiffness.rs:204`'s branch; none is on the plain path. Each is result-identical under the quoted premise. Calls and totals are Part I §9.5.

| | Site | Replace | Premise (quoted) | Steps per call | Calls (P-zero) |
|---|---|---|---|---|---|
| R3a | `adapter :2124` `nodes.sort_unstable()` | HS | `nodes = body.to_vec()` (`:2123`), and `body` holds distinct nodes: `if !seen[other] { seen[other] = true; body.push(other); }` (`:1963`–`:1966`), seeded once per unseen node (`:1945`–`:1949`). The `usize`s are distinct, so the order is unique | HS(\|b\|) | 1 per W4 body (≤ U) |
| R3b | `adapter :2125` `nodes.binary_search(&global).ok()` | LB, `Some(lo)` when found | `nodes` sorted (R3a) and distinct | ⌊log₂\|b\|⌋ + 2 | 2E_el + p + S per W4 body: 2 per edge (`:2142`), 1 per ground (`:2212`) |
| R3c | `rigid_body.rs:414` `body.sort_unstable()` | HS | `usize` elements (and distinct: `if !seen[other] { … }`, `:408`–`:411`) | HS(\|sb\|) | 1 per sub-body |
| R3d | `rigid_body.rs:725` `body.sort_unstable()` | HS | `usize` elements: equal values are indistinguishable | HS(\|sb\|) | 1 per sub-body |
| R3e | `rigid_body.rs:728` `bodies.sort_unstable_by_key(\|body\| body[0])` | HS by `body[0]` | equal keys mean two sub-bodies share a node; the owner loop then returns the constant `Err(INVALID)` in either order (`if node >= n \|\| owner[node] != usize::MAX { return Err(INVALID); }`, `:730`–`:737`); with distinct keys the order is unique; an empty body returned `Err(INVALID)` before the sort (`:721`–`:723`) | HS_e(σ_b) = HS_k(σ_b; 1, 1) = 9σ_b(⌊log₂ σ_b⌋ + 1) + 2: the closure `body[0]` is 3 operations, charged as its own body (κ = 1), and two `usize` keys compare in one step (c = 1) | 1 per W4 body |
| R3f | `rigid_body.rs:748` `ties.sort_unstable()` | HS | `[usize; 2]` elements | HS(0) = 2 here (`let ties: Vec<[usize; 2]> = Vec::new();`, `adapter :2140`). With ties it would be HS_k(r; 0, 2), each comparison two element steps | 1 per W4 body |
| R3g | `rigid_body.rs:789` `rows.sort_unstable_by_key(GroundRow::key)` | HS by `key` | `key` is `(self.node, self.kind, self.axis.is_none(), self.axis.unwrap_or(0), self.direction.map(f64::to_bits))` (`:443`–`:451`), which fixes every field of `GroundRow { node, kind, axis, direction }` (`:433`–`:438`) bit for bit, so equal keys are identical rows | HS_g(g_b) = HS_k(g_b; 7, 4) = 39g_b(⌊log₂ g_b⌋ + 1) + 2. κ = 7: `self.direction` is copied out of `&self` (3), `map(f64::to_bits)` is 3 element steps, and the rest is ≤ 64 operations (1). c = 4: the four scalar fields compare in one step, and the `[u64; 3]` field compares as an element primitive (3). A bottom-up heapsort of 8 ascending DOF rows makes 27 comparisons, 27 × 18 = 486 steps of keys, inside HS_g(8) = 1,250 | 1 per W4 body |
| R3h | `restraint_basis.rs:861` `self.nodes.binary_search_by_key(&node, \|n\| n.node)` | LB by `n.node` | `build` orders stops by `order.sort_by_key(\|&k\| (stops[k].node, k))` (`:754`), walks one node group at a time (`:758`–`:765`), and refuses a node with two stops (`:766`–`:772`, `:808`–`:810`), so `nodes` is ascending, one per node | ⌊log₂ ν⌋ + 2 | ≤ 22C + 4Z_ν + 2n_g + 3ν |
| R3i | `restraint_basis.rs:1070` `first.binary_search(&(base + t)).is_err()` | LB, found test | `first = pattern.row(base)`, strictly increasing (R1's premise) | β̄ | 3ν |
| R3j | `restraint_basis.rs:1178` `self.axis_restrained.binary_search(&dof)` | LB, `Ok(lo)`/`Err` | `build` refuses `axis_restrained.windows(2).any(\|p\| p[0] >= p[1])` (`:739`–`:743`), so it is strictly increasing; only `Ok`'s index is read (`:1179`) | ⌊log₂ p⌋ + 2 | p + ν |
| R3k | `formation_check/skew.rs:348` `basis.axis_restrained().binary_search(dof).is_err()` | LB, found test | as R3j | ⌊log₂ p⌋ + 2 | n_g |

All per-call costs are ≤ β̂ (Part I §2).

**Static results.**
- Per site, an oracle against a verbatim copy, on random distinct inputs, duplicates where the premise allows them, empty, singleton and reverse-ordered inputs.
- R3e's oracle includes two sub-bodies with the same first node. R3g's includes rows equal in every field and ±0.0 directions (distinct by bits).
- Suites: `restraint_basis_tests.rs`; the adapter's `skew_tests`, `k5_tests` and `kd5_tests`; `rigid_body` tests; the product skew fixtures; `product_physics/tests/formation_check_runtime.rs`; and the static and modal population runs.

**Pricing.**
- **R3a, R3b edit structural_adapter.rs: re-pin** `b7a_domain_model.py:237` and `w1_reductions_model.py:113`.
- SKW-D1's text guards (`o10_rules.py:6243`–`:6330`) must hold. `_skw_geo` spans `BodyEvidence::geometry`, which R3a–b leave unedited; `W4Context::body` lies below it.
- R3c–R3g: `objective_sub_bodies` and `reduce_constrained_body` are on priced routes (`base/loop_bounds.g4.json:1091`–`:1106`). Their new headers need classification (bounds: body size, sub-bodies, ties r, rows), and the existing keys must carry. Keep the `fn` header lines, with one HS helper at the end of `rigid_body.rs`.
- R3h keeps `node_basis`' name and signature (`o10_rules.py:6286` pins `basis.node_basis(linear.node_index).is_some()` in product code).
- R3h–R3k: `restraint_basis.rs` and `formation_check/skew.rs` have no full-file pin. `test_855_sk1_skew.py:64` locates `skew.rs:evaluate` by name, so keep it.
- `price.py compare` equal, plus AC-004.

### R4. `row_sources` (`structural.rs:1304`–`:1309`): T1, and T2 only on a flagged row

HS (by `String` byte order), then DD.
- **Steps:** HS_k(l; 0, 1 + λ̄_s) + DD with each `==` at ≤ 1 + λ̄_s, where l is the row's terms and λ̄_s its longest source id: comparing two `String`s is a slice comparison, one element step per byte compared. It is T1's work, and T2's only on a flagged row.
- **Premise:** equal `String`s are byte-equal, and each element is `t.source.clone()` (`:1305`), whose capacity equals its length. So even the surviving object's capacity is the same.
- **Pins kept byte-identical:** `base/b7a_owner_choices.py:35`, `:285`–`:310`; `base/b7a_formation_model.py:82`, `:91`; `base/text_args.g4.json:556`; `base/loop_bounds.g4.json:1121`; `tests/test_b7a_owner_choices.py:138`.
- The `fn row_sources` header stays at line 1304. **Re-pin** `b7a_formation_model.py:52` (structural.rs). New headers classified (bound l, no site). Coefficients equal, plus AC-004.

### R5. `frame_member_identities` (product `lib.rs:4890`–`:4908`): **recommended**

**Why not count the `HashSet`s.** `std` documents `HashSet`/`HashMap` insert and lookup only as *expected* O(1), amortized for insert, under randomized hashing, with no worst case. Even a premise reading `std`'s documentation as a foreign step would give no upper bound. There is no honest constant to adopt.

**Replace, line for line** (the line count is unchanged):
- `:4891`: `let arcs = &built.curved_bend_elements;`
- `:4892`: `let replaced = &built.connector_records;`
- `:4896`: `if arcs.iter().any(|bend| bend.pipe_index == index) || replaced.iter().any(|record| record.span_index == index) {`

**Premise.** The function reads the sets only through `contains(&index)` for `index` in `0..built.pipes.len()` (`:4895`–`:4896`). Membership is the same predicate, so `members` is the same `Vec<String>`, and the early `return Vec::new()` paths fire on the same pipes.

**Steps.** pipes × (A + K_c) element steps (Part I §4, PREP). A merge cursor would give pipes + A + K_c, but it would need `curved_bend_elements` ascending by `pipe_index` and `connector_records` by `span_index`, which is not established here.

**Allocation.** Two `HashSet<usize>` are removed. Nothing is added.

**Pricing.**
- **Re-pin** `b7a_domain_model.py:237` and `w1_reductions_model.py:105`. Both compare lib.rs through `ce_fallback_model.py`'s `s6_bytes`, so the projection must still apply to the edited bytes.
- **U8-FM** (`base/u6_owners_model.py:222`–`:226`) pins four exact texts: `fn frame_member_identities(built: &BuiltModel) -> Vec<String> {`, `members.push(member_prefix(&pipe.element_id))`, `let mut members = Vec::with_capacity(built.frame_elements.len());` and `format!("member:{}:{member}:", member.len())`. R5 leaves all four untouched, so it does not trip.
- No pricing rule names `frame_member_identities`' set lines (grep of `tools/retained_pricing`). The two `.any(` closures are new loops to loopscan's `ADAPT` pattern. They need classification: bounds A and K_c, no site.
- Coefficients may move **down** if `price.py` prices the two `HashSet` collects through its generic container rules (`base/callgraph_g5.py:183` lists `HashSet`). Then it is a D3 re-registration with a reviewed lower tree, not a coefficient-equality claim. If `compare` shows equality, it can share a D3 turn.
- Evidence: product tests, the K-D5 member-identity tests (an empty list fails closed, `:4888`–`:4889`), the population models, and AC-004.

### R6. `2.0_f64.powi(step)`: **recommended**

**Sites.**
- Plain path: `structural.rs:666`, `:928`, `:929`, `:1627`.
- Basis branch: `formation_check.rs:1139` (`pow2`, in `correction`) and `:2559` (`scale_up`). Both are unreached under P-zero but reached for a general force.
- `formation_check.rs:2528` is a constant 2⁻⁹⁰⁰ in a load re-formation, off this path.

**Replace** each power with the bit-built power of the same exponent, written inline on the same line; a helper would add a line above later keys. The exponent's sign differs between sites:

| Site | Function | Now | Replacement |
|---|---|---|---|
| `structural.rs:666` | `radix_scale` | `value *= 2.0_f64.powi(step);` | `value *= f64::from_bits(((step + 1023) as u64) << 52);` |
| `structural.rs:928` | `exact_radix` | `let next = value * 2.0_f64.powi(step);` | `let next = value * f64::from_bits(((step + 1023) as u64) << 52);` |
| `structural.rs:929` | `exact_radix` | `… \|\| next * 2.0_f64.powi(-step) != value {` | `… \|\| next * f64::from_bits(((1023 - step) as u64) << 52) != value {` |
| `structural.rs:1627` | `physical_residual_record` | `value *= 2.0_f64.powi(step);` | `value *= f64::from_bits(((step + 1023) as u64) << 52);` |
| `formation_check.rs:1139` | `pow2` | `value *= 2.0_f64.powi(step);` | `value *= f64::from_bits(((step + 1023) as u64) << 52);` |
| `formation_check.rs:2559` | `scale_up` | `value = mul_up(value, 2.0_f64.powi(step));` | `value = mul_up(value, f64::from_bits(((step + 1023) as u64) << 52));` |

Every site clamps `step` to [−512, 512] on the line above (`structural.rs:665`, `:927`, `:1626`; `formation_check.rs:1138`, `:2558`), so `step + 1023` and `1023 − step` lie in [511, 1,535], a normal biased exponent. The same form exists as `correct_norm`'s `pow2` (`lib.rs:141`–`:144`) and `rigid_body.rs:1074`–`:1079`. The `:929` line becomes 109 columns. No CI job runs `cargo fmt --check`, and structural.rs already holds 24 lines over 100 columns, so it is not reflowed and the line count holds.

**Premise.** 2^k and 2^−k for |k| ≤ 512 are normal binary64 values, so each bit pattern is exact. `powi(2.0, k)` is also exact: `__powidf2`'s square-and-multiply on 2.0 forms only powers of two in [2⁻⁵¹², 2⁵¹²], and one reciprocal of a power of two, and constant folding gives the same value. So every product and check (`next * 2^-step != value`, `structural.rs:929`; `mul_up`, `formation_check.rs:2559`) sees identical operands.

**Steps.** One operation (P-fp's `from_bits`, the shift and the add or subtract) instead of a foreign loop.

**Static results.**
- A whole-function oracle (test-only): verbatim copies of `radix_scale`, `exact_radix`, `physical_residual_record`, `pow2` and `scale_up` (with `mul_up`) at bf1bdd2 against the new ones. `Result`s compare by `Ok` bits or the same `Err` and message; `f64`s by `to_bits`. Values: ±0.0, ±`f64::from_bits(1)`, ±`MIN_POSITIVE`, ±1.0, ±1.5, ±`MAX`, ±∞, NaN, and 10⁴ random finite values over every binade. Exponents: every integer in [−2,200, 2,200], and `i32::MIN` and `i32::MAX` for `radix_scale` and `exact_radix`. Named cases: `exact_radix(1.0, 1)` is `Ok(2.0)` and `exact_radix(1.0, -1)` is `Ok(0.5)`. A power built with the wrong sign at `:929` returns `Range` on the first.
- An oracle comparing each power's `to_bits` over all k ∈ [−512, 512], for both signs.
- The `exact_operation_count::radix_iteration` counters (`structural.rs:926`) unchanged.
- `normalized_residual_retains_soft_physical_cancellation`, `krev03_offdiagonal_expansion_retains_lost_low_tail`, `nonfinite_and_unrepresentable_range_block`, every dense and sparse fixture, and the K-D5 suites (`formation_check_tests.rs`, `formation_check_timoshenko_tests.rs`, `formation_check_runtime.rs`).

**Pricing.**
- No loop, site or line is added. No pricing rule names `powi`, `scale_up`, `radix_scale`, `exact_radix` or `physical_residual_record` (grep of `tools/retained_pricing`).
- **Re-pin** `b7a_formation_model.py:52` for structural.rs. The formation_check.rs sites also need `b7a_domain_model.py:237` and `w1_reductions_model.py:110`. B7a's leaf-seam check (`b7a_domain_model.py:310`–`:322`) is unaffected.
- `price.py compare` equal, plus AC-004.

**Why not a premise.** `powi` is a library loop, not an IEEE operation. Premising it is the same kind of claim the Design declined for `binary_search`. R6 is a bit-identical, line-neutral edit.

### R7. `correct_norm::norm3`'s ulp loop: **not needed**

Part I Lemma D proves ≤ 2 iterations from source. No source changes.

If Agent 0 wants a regression witness, a test-only counter can record the iterations across `correct_norm`'s tests and a random sample, asserting ≤ 2.

A defensive cap is not recommended. A cap at 2 that refuses would add a new refusal path, a changed static result in principle, that Lemma D makes unreachable. It would only add review surface.

### R8. 128-bit integer divisions in the wide arithmetic: **recommended**

§1.3 excludes these from P-fp: a 128-bit division lowers to `__divti3`, `__modti3` or `__udivti3`, library loops, unless the optimizer folds a constant divisor. Each is replaced inline on its own line, except one helper at the end of `wide/multi.rs`.

| Site | Now | Replacement | Premise |
|---|---|---|---|
| `retained/wide.rs:818` (`sqrt_exact_operand`) | `if e.rem_euclid(2) == 0` (`e: i128`) | `if e & 1 == 0` | in two's complement, x mod 2 (Euclidean, in {0, 1}) is the low bit for every `i128` |
| `wide/multi.rs:196` (`word_at`) | `start.div_euclid(64)` (`start: i128`) | `start >> 6` | the arithmetic shift is ⌊x/64⌋, which is Euclidean division by a positive divisor |
| `wide/multi.rs:197` | `start.rem_euclid(64) as u32` | `(start & 63) as u32` | x − 64⌊x/64⌋ is the low six bits |
| `wide/multi.rs:578` (`Wide<L>`'s square root, off this path) | `(e - base).rem_euclid(2) == 0` | `(e - base) & 1 == 0` | as `:818` |
| `wide/multi.rs:711` (`half_quantum_over`) | `numerator.div_ceil(d)` (`u128`) | `ceil_div_u128(numerator, d)` | a long division in base 2¹⁰ with `u64` `/` and `%`. d < 2⁵⁴ (`debug_assert!(k >= 1 && k < 1 << 52)`, `:703`, d = 2k), so a remainder times 2¹⁰ plus a digit fits a `u64`. The numerator 2^(52+t) has ≤ 106 bits, so ≤ 11 digits. The helper returns q + (r ≠ 0), which is `div_ceil`'s value |

**Steps.** One operation each, except `ceil_div_u128`: ≤ 11 iterations and its call, ≤ 13. `to_binary64` reaches it only on its subnormal path, so `to_binary64` is ≤ 25 there and 12 otherwise. τ̄ (Lemma E, ≤ 9,557 + 13) stays within its 9,600.

**Static results.**
- An oracle (test-only) against verbatim copies of `sqrt_exact_operand`, `word_at` and `half_quantum_over` at bf1bdd2: exponents over [−4,096, 4,096] and ±(2⁶² − 1), `start` over [−4,096, 4,096] and ±(2⁶² − 1), and every k = 2^j and 2^j ± 1 for j < 52 plus 10⁵ random k in [1, 2⁵²), compared by value.
- The retained-wide suites (`tests/retained_wide/wide_tests.rs`, the `multi` tests), `arc_angle`'s tests, and the K-D5 suites.

**Pricing.**
- **Re-pin** `w1_member_load_model.py:100` (wide.rs) and `:101` (multi.rs). Both are full-file SHA-256 pins, raised as "W1 stage2 source/owner pin changed" (`:196`); each digest matches its file at bf1bdd2.
- `callgraph_rules.g4.json` keys wide.rs and multi.rs by line (`:246`–`:272`, `:316`–`:324`, and multi.rs `:1052` onward). The inline edits keep every line, and the helper goes below the last key.
- The helper's `for` header needs a loop classification: bound 11, no site inside.
- `price.py compare` equal, plus AC-004.

### Premises rather than replacements

Agent 0 adopted both as pricing premises (Part I §1.3); modal-analysis.md §5.3.8 records them. The Design's adopted text (`resource.rs:185`–`:203`) names √ only.
- **P-fp.** These are IEEE operations and core integer operators, with √'s standing. Where the target has a fused multiply-add instruction, replacing `mul_add` with Dekker's TwoProduct would rewrite `Expansion::add_product`, `correct_norm`, `connector`, `tie_rod`, `restraint_basis` and the wide arithmetic for no change in value. The methods §1.3 excludes by name are replaced (R6, R8) or wait on a decision (`mul_add` on x86_64, Part I §12, item 2).
- **P-fmt.** The three plain-path sites are priced TEXT sites. `o10_rules.py:419` pins `format!("member:{}:{member}:", member.len())` with `full=True`, and U8-FM pins the same text. Rewriting them would move text coefficients and trip both pins. The premise is the coefficient-neutral route.

### Decisions

Decided by Agent 0 (2026-11-09):
1. **P-fp and P-fmt** are adopted as pricing premises.
2. **δ̄** is computed with one array of N counters in P0, its allocation (8N bytes) and its ≤ 2N + E_el + S + s_k steps charged in P0's own account. P0 already allocates the `BTreeSet` at `resource.rs:1038`.
3. **R5** is approved in principle. It is implemented with R1, R3 and R6 after the pressure-elongation unit merges, with its re-pins (`b7a_domain_model.py:237`, `w1_reductions_model.py:105`), and a D3 turn if `price.py compare` shows lower coefficients.

Open for Agent 0:
4. **`mul_add` on x86_64** (Part I §12, item 2): the `fma` target feature, an in-repo fused multiply-add, or a priced foreign step.
5. **Approve R8**, which the `div_ceil`, `div_euclid` and `rem_euclid` exclusions require.

Beyond these, the open items are implementation and the weights (Part I §12).

### Summary

| R | Site | Premise in one line | Pricing |
|---|---|---|---|
| R1 | `sparse.rs:190` (+ `adapter :1657`, `restraint_basis.rs:978`, `:1108`) | rows strictly increasing (`sparse.rs:74`–`:75`, `:100`–`:101`, `:123`–`:125`, `:263`–`:266`) | re-pin `b7a_formation_model.py:64`; one header; equal |
| R2 | `sparse.rs:74`, `:100` | `usize`s indistinguishable | K formation only; R1's re-pin |
| R3 | `adapter :2124`–`:2125`; `rigid_body.rs:414`, `:725`, `:728`, `:748`, `:789`; `restraint_basis.rs:861`, `:1070`, `:1178`; `formation_check/skew.rs:348` | distinct by `seen`; equal keys give the same `Err` or identical rows; ascending unique keys | re-pin `b7a_domain_model.py:237` and `w1_reductions_model.py:113` (R3a–b); headers; equal |
| R4 | `structural.rs:1306`–`:1307` | equal `String`s byte- and capacity-equal | re-pin `b7a_formation_model.py:52`; equal |
| R5 | product `lib.rs:4891`, `:4892`, `:4896` | same membership predicate | re-pin `b7a_domain_model.py:237`, `w1_reductions_model.py:105`; U8-FM does not trip; equal or lower |
| R6 | `structural.rs:666`, `:928`, `:929`, `:1627`; `formation_check.rs:1139`, `:2559` | 2^k exact for \|k\| ≤ 512 | re-pin `b7a_formation_model.py:52` (and `b7a_domain_model.py:237`, `w1_reductions_model.py:110` for formation_check.rs); equal |
| R7 | none | Lemma D | none |
| R8 | `retained/wide.rs:818`; `wide/multi.rs:196`–`:197`, `:578`, `:711` | x mod 2ⁿ is the low n bits and ⌊x/2ⁿ⌋ the arithmetic shift; an in-repo long division gives `div_ceil`'s value | re-pin `w1_member_load_model.py:100`, `:101`; one header; equal |
