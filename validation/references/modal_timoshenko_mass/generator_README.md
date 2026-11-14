# Source-blind exact Timoshenko consistent mass

Derived only from `../PROBLEM.md` (SHA-256
`ab4a949c2533603d91d27cdeff561207d0339997dbe1e53bbc783ca8cc00784d`).
No closed-form mass matrix was used to produce or check any value.

## Files

- `generator.py`: the generator. Standard-library Python 3 only (`argparse`, `fractions`, `json`, `math.isqrt`, `os`, `sys`). Every value is a `fractions.Fraction` from the binary64 operands to the output; no float is formed.
- `timoshenko_consistent_mass.json`: the output for PROBLEM.md's five cases.
- `timoshenko_consistent_mass_addendum1.json`: the output for the three cases added by `../ADDENDUM-1.md` (SHA-256 `40b5954acac00205fe05db8fa167251f82b5c8a21850eb86e8df74e614cc1d91`). It has the same layout, with `"format": "swbpipe-source-blind-timoshenko-consistent-mass-addendum1/1"` and an added `"addendum_sha256"`.
- `generator_v1.py`: the generator before the addendum, kept for review. `generator.diff` is the unified diff from it to `generator.py`.
- `run.sh`: runs `host_check.py`, then the generator inside `t3_slot.sh`. It passes its arguments through and exits with the generator's status.
- `host_check.py`: the pre-run host check (timing-window flag; live exclusive holds in the guard log).

## Use

```sh
python3 -I -B generator.py           # rebuild both files in memory, compare with the written files; exit 0 iff both byte-identical
python3 -I -B generator.py --write   # write both files
python3 -I -B generator.py --output PATH --output-addendum1 PATH2 [--write]   # use other paths
```

Both modes run every self-check on all eight cases, and any failure raises and exits non-zero. The output is deterministic: fixed case order, insertion-ordered keys, `json.dumps(indent=1, ensure_ascii=True)`, one trailing newline, ASCII.

## Output layout

```
{
 "format": "swbpipe-source-blind-timoshenko-consistent-mass/1",
 "problem_sha256": "<PROBLEM.md digest>",
 "cases": [
  {
   "id": "T-AXIS-SLENDER",
   "operands": { ...the bit patterns from PROBLEM.md, verbatim... },
   "L":     VALUE,
   "Phi_y": VALUE,
   "Phi_z": VALUE,
   "plane_xy": [ {"a": 0, "b": 0, ...VALUE}, ... 16 entries, a-major ],
   "plane_xz": [ ... 16 entries ... ],
   "global_upper_nonzero": [ {"r": 0, "c": 0, ...VALUE}, ... r-major, then c ],
   "self_checks_passed": [ "...", ... ]
  }, ...
 ]
}
```

`VALUE` is `{"decimal": D, "hi": H, "lo": W}`:

- `D` has forty significant digits in scientific notation, `d.ddd…ddde±XX`: one nonzero digit, a point, 39 digits, then `e`, a sign and at least two exponent digits. The last digit is rounded from the exact rational to nearest, ties to even. An exact zero is written `"0"`. No listed entry is zero, so this applies only where a value happens to be zero.
- `H` is the exact value rounded to binary64 (nearest, ties to even), as 16 lowercase hex digits.
- `W` is the exact rational `value − hi` rounded the same way. An exactly zero remainder is `0000000000000000`.
- Underflow, for an in-range `Phi_y` or `Phi_z` (μ ≤ |Φ| ≤ (2⁵³ − 1)·2⁹⁷¹ with μ = 2⁻¹⁰⁷⁴): hi is Φ rounded to nearest-even, and never zero in this range. lo is the exact residual Φ − hi rounded to nearest-even. A nonzero residual that rounds to zero is written as the zero carrying the residual's sign: `0000000000000000` if positive, `8000000000000000` if negative. Examples: 3μ/2 gives hi `0000000000000002`, lo `8000000000000000`; 5μ/4 gives hi `0000000000000001`, lo `0000000000000000`.
- Underflow, for every other value (matrix entries, L): the generator refuses (raises) rather than emit a nonzero hi or lo that rounds to zero. No value in either output file triggers this refusal.

**Shear parameter outside binary64 (ADDENDUM-1).** This applies to `Phi_y` and `Phi_z` only. If |Φ| > (2⁵³ − 1)·2⁹⁷¹ (the largest finite binary64) or 0 < |Φ| < 2⁻¹⁰⁷⁴ (the smallest positive subnormal), the value is `{"decimal": D, "numerator": N, "denominator": M}`. Here N and M are decimal-integer strings in lowest terms with M > 0, and there is no `hi` or `lo`. The bounds themselves count as in range. Strings are used so that no JSON parser can round the integers. Everything else keeps the (hi, lo) form. Among the eight cases the rule applies only to T-RANGE-PHI-HUGE (Φ = 2²⁰⁰⁰) and T-RANGE-PHI-TINY (Φ = 2⁻²⁰⁰⁰).

The plane matrices are labelled `plane_xy` (x–y plane: w = u_y, θ = r_z, I_z, Φ_z) and `plane_xz` (x–z plane: w = u_z, θ = −r_y, I_y, Φ_y). They are given before the sign map and placement, indexed (a, b) in the order (w_i, θ_i, w_j, θ_j).

`global_upper_nonzero` lists every nonzero entry (r, c), 0 ≤ r ≤ c ≤ 11, of Tᵀ M_local T. The DOFs are numbered u_x, u_y, u_z, r_x, r_y, r_z at node i (0–5), then at node j (6–11). An unlisted upper entry is exactly zero, and the lower triangle follows by symmetry.

## Derivation

**Axes.** L = √(d·d), with d = x_j − x_i; x̂ = d/L; ŷ is the normalized part of v normal to x̂; ẑ = x̂ × ŷ. Each square root is taken exactly: the reduced numerator and denominator must both be perfect squares (`math.isqrt`), or the generator fails. RRᵀ = I is checked exactly.

**Bending shapes.** Q = G A_s (w′ − θ) is constant and EI θ″ + Q = 0, so θ″ is constant. With β = EI/(G A_s) = Φ L²/12 the general solution is

  θ = c₁ + c₂ s + c₃ s²,  w = c₀ + c₁ s + c₂ s²/2 + c₃ (s³/3 − 2β s).

The four nodal values (w(0), θ(0), w(L), θ(L)) give a 4×4 linear system in c. It is solved exactly once per unit nodal vector, giving N_w,k and N_θ,k as exact polynomials in s.

**Plane mass.** M_ab = m̄ ∫₀ᴸ N_w,a N_w,b ds + ī ∫₀ᴸ N_θ,a N_θ,b ds, integrated exactly through the polynomial antiderivative.

**Axial and torsional mass.** These use linear interpolation N₁ = 1 − s/L, N₂ = s/L, integrated exactly with m̄ (u_x) and j̄ (r_x).

**Placement and global axes.** The x–y plane goes on DOFs (1, 5, 7, 11) as is. The x–z plane goes on (2, 4, 8, 10), each entry (a, b) multiplied by s_a s_b with s = (+1, −1, +1, −1). The global matrix is Tᵀ M_local T with T = diag(R, R, R, R), where R has rows x̂, ŷ, ẑ.

## Self-checks (all exact rational equality)

1. **Equilibrium and nodal conditions** (each plane, each shape). The polynomial EI N_θ″ + G A_s (N_w′ − N_θ) is identically zero. The polynomial G A_s (N_w′ − N_θ) has zero derivative. The nodal values of shape k are the unit vector e_k.
2. **Stiffness.** K_ab = ∫ EI N_θ,a′ N_θ,b′ + G A_s (N_w,a′ − N_θ,a)(N_w,b′ − N_θ,b) ds is integrated exactly from the shapes. Checks: K = Kᵀ; K·(1, 0, 1, 0) = 0 (translation); K·(0, 1, L, 1) = 0 (rotation, w = s, θ = 1).
3. **Euler–Bernoulli limit.** The same Timoshenko construction is run with Φ = 0 (β = 0). Each shape satisfies θ = w′ exactly. Its plane mass equals, entry for entry, a mass derived on a separate path: monomial cubic w(ξ) in ξ = s/L on [0, 1], θ = (1/L) dw/dξ, Hermite shapes from their own nodal system, and the integrals m̄ L ∫ H_a H_b dξ + (ī/L) ∫ H_a′ H_b′ dξ. This runs once per case with that case's L, m̄ and ī; at Φ = 0 the two planes coincide.
4. **Rigid translations against an independent load vector.** For k = x, y, z, the local load components are q = R e_k, which gives the axial and both transverse projections. The nodal vector is built without any shape function:
   - **Axial.** ∫ N q_x ds over the linear interpolation.
   - **Each bending plane.** The clamped Timoshenko beam under uniform load q is solved exactly: (G A_s (w′ − θ))′ = −q, EI θ″ + G A_s (w′ − θ) = 0, and w = θ = 0 at both ends. Then f = (Q(0), EI θ′(0), −Q(L), −EI θ′(L)). This equals ∫ q N_w,a ds for exact homogeneous shapes: integrate the energy bilinear form by parts on each argument, with the clamped solution vanishing at the nodes and each shape homogeneous. The clamped solution's own equilibrium and end conditions are checked exactly.
   - **Assembly.** The x–z vector takes the same sign map s, F is placed like M, and it is turned to global axes by Tᵀ F.

   Checks: M_global r_k = m̄ Tᵀ F (all 12 components), and r_kᵀ M_global r_k = m̄ L. Here r_k is the unit translation in global direction k at both nodes.
5. **Symmetry and definiteness.** M_global = M_globalᵀ, and an exact LDLᵀ elimination without pivoting gives 12 pivots, all > 0. By Sylvester's criterion all leading principal minors are positive, so the matrix is positive definite.

The encoders are also self-tested on known values before the cases run: binary64 round trips including subnormals and the largest finite value, binary ties to even in both directions, and decimal rounding including ties to even and the carry to a new decade.
