# Problem statement: exact consistent mass of a Timoshenko straight frame member

This statement is your only technical input. It contains mathematics, conventions and input values; it contains no program code. Derive everything else yourself.

## The member

A two-node straight frame member in three dimensions, between node i at x_i and node j at x_j. Its length is L = |x_j − x_i| > 0.

It carries:
- translational mass per length m̄, which acts in all three translations;
- rotary inertia per length ī, which acts on the section rotation of each bending plane;
- torsional inertia per length j̄.

Its bending stiffness is a Timoshenko beam with:
- Young's modulus E;
- shear modulus G;
- shear area A_s;
- second moments I_y and I_z.

All of E, G, A_s, I_y, I_z, m̄, ī and j̄ are independent prescribed inputs, constant along the member, in consistent units. The same ī applies to both bending planes, including when I_y ≠ I_z. Do not derive a replacement inertia, or any relation between these inputs, from a density or a section shape. This is a mathematical test case; it makes no claim to describe a particular physical cross-section.

## Member axes

- x̂ = (x_j − x_i)/L.
- ŷ is the unit vector along the component of a given reference vector v normal to x̂:

  ŷ = (v − (v·x̂)x̂) / |v − (v·x̂)x̂|.
- ẑ = x̂ × ŷ.

## Degrees of freedom

Each node has six, in this order:

  u_x, u_y, u_z, r_x, r_y, r_z.

These are translations and small rotations about the member axes. The element's 12 DOFs are node i's six, then node j's six.

## The mass, by definition

**Axial.** On (u_x at i, u_x at j): the consistent mass of linear interpolation with mass per length m̄.

**Torsional.** On (r_x at i, r_x at j): the consistent mass of linear interpolation with torsional inertia per length j̄.

**Bending, in each plane.** Use local coordinate s ∈ [0, L], transverse displacement w(s) and section rotation θ(s). The stiffness is the Timoshenko beam's:
- strain energy (1/2)∫ EI θ'² ds + (1/2)∫ G A_s (w' − θ)² ds;
- equilibrium with no distributed load: EI θ'' + G A_s (w' − θ) = 0, with G A_s (w' − θ) constant along the member.

The interpolation is the exact homogeneous solution of these equations. It is fixed by the four nodal values (w(0), θ(0), w(L), θ(L)), giving shape functions N_w,k(s) and N_θ,k(s) for k = 1..4.

The plane's 4×4 consistent mass on (w_i, θ_i, w_j, θ_j) is

  M_ab = m̄ ∫₀ᴸ N_w,a N_w,b ds + ī ∫₀ᴸ N_θ,a N_θ,b ds.

Define Φ_y = 12 E I_y / (G A_s L²) and Φ_z = 12 E I_z / (G A_s L²). The x–y plane uses I_z and Φ_z; the x–z plane uses I_y and Φ_y.

**The two planes:**
- **x–y plane.** w = u_y, θ = r_z, I = I_z. Its 4×4 sits on DOFs (u_y,i, r_z,i, u_y,j, r_z,j) as is.
- **x–z plane.** w = u_z, θ = −r_y, I = I_y. Its 4×4 sits on DOFs (u_z,i, r_y,i, u_z,j, r_y,j), with entry (a, b) multiplied by s_a·s_b, where s = (+1, −1, +1, −1).

All other entries of the 12×12 in member axes are zero.

**Global axes.** Let R be the 3×3 matrix whose rows are x̂, ŷ, ẑ, and T = diag(R, R, R, R). The global 12×12 is Tᵀ M_local T.

## What to produce

1. **A generator.** Standard-library Python 3 only, with no third-party package; for example fractions, decimal, json, hashlib, argparse, struct and math.isqrt. The derivation and every value stay in exact rational arithmetic.
   - Every operand is given below as an IEEE 754 binary64 bit pattern. Convert each to its exact rational value, and work in exact rational arithmetic throughout.
   - The cases are chosen so that L and the member axes are rational. Fail if a square root is not rational.
   - Derive the shapes from the equations above and integrate them exactly. Do not use a closed-form mass matrix from any source to produce the values.
2. **For each case, output:**
   - L, Φ_y and Φ_z;
   - both 4×4 plane matrices, labelled x–y plane and x–z plane, before the sign map and placement, each entry indexed (a, b) with a, b = 0..3 in the order (w_i, θ_i, w_j, θ_j);
   - the nonzero entries of the upper triangle of the global 12×12, diagonal included.
     - Each entry is indexed (r, c) with 0 ≤ r ≤ c ≤ 11. The DOFs are numbered from 0 in the stated order: node i's u_x, u_y, u_z, r_x, r_y, r_z as 0 to 5, then node j's as 6 to 11.
     - An upper-triangle entry that is not listed is exactly zero; the lower triangle follows by symmetry. An exactly zero value or remainder is written as the positive-zero bit pattern 0000000000000000.

   Give each value in two forms:
   - **Decimal:** forty significant digits in scientific notation (one nonzero digit before the point, thirty-nine after it, then the exponent). The last digit is rounded from the exact rational value to nearest, ties to even. An exact zero is written as zero.
   - **Pair (hi, lo):**
     - hi is the exact value rounded to binary64, to nearest with ties to even;
     - lo is the exact rational (value − hi), rounded to binary64, to nearest with ties to even;
     - write both words as 16-hex-digit IEEE 754 bit patterns.
3. **Self-checks your generator performs,** each exact. State in your return how each is computed.
   - Your shapes satisfy the equilibrium equations and the nodal conditions.
   - The stiffness from your shapes, through the strain energy above, is symmetric. It annihilates the two rigid-body motions of the plane: translation, and rotation with w = s and θ = 1.
   - As G A_s grows without bound (Φ → 0), your shapes tend to the Euler–Bernoulli shapes, in which θ = w'. Check this by evaluating your plane mass at Φ = 0 and comparing it with the mass you derive independently from the cubic Euler–Bernoulli interpolation.
   - For each global unit translation r_k (k = x, y, z: that unit translation at both nodes, no rotation), derive yourself the consistent nodal vector of a uniform unit load per length in that global direction, including its axial projection, both transverse projections and the nodal moments they produce, turned to global axes. The global mass times r_k equals m̄ times that vector, and r_kᵀ M r_k = m̄L.
   - The global matrix is symmetric and positive definite.

File names and any serialization detail not prescribed here are yours to choose, provided the output is deterministic: two runs produce identical bytes.

## The cases

The table below gives each case's values for orientation. The bit patterns after it are the operands; where the two differ, the bit patterns govern.

| Id | x_i | x_j | v | E | G | A_s | I_y | I_z | m̄ | ī | j̄ |
|---|---|---|---|---|---|---|---|---|---|---|---|
| T-AXIS-SLENDER | (0, 0, 0) | (3, 0, 0) | (0, 1, 0) | 2.03e11 | 7.8e10 | 5.540e-3 | 1.2847e-4 | 1.2847e-4 | 81.5 | 1.035 | 2.07 |
| T-SKEW-STUBBY | (1, 2, 3) | (1.1875, 2.25, 3.75) | (67, −19, 60) | 2.03e11 | 7.8e10 | 5.540e-3 | 1.2847e-4 | 1.2847e-4 | 81.5 | 1.035 | 2.07 |
| T-SKEW-UNEQUAL | (0, 0, 0) | (0.25, 0.375, 0.75) | (6, 2, −3) | 2.0e11 | 8.0e10 | 4.0e-4 | 4.0e-5 | 1.0e-5 | 50.0 | 0.25 | 0.5 |
| T-AXIS-NEAR-EB | (0, 0, 0) | (0, 0, 2) | (1, 0, 0) | 2.03e11 | 7.8e10 | 5.540e9 | 1.2847e-4 | 1.2847e-4 | 81.5 | 1.035 | 2.07 |
| T-RANGE-TINY-MASS | (0, 0, 0) | (1, 0, 0) | (0, 1, 0) | 1 | 1 | 12·2⁻⁴⁰⁰ | 1 | 1 | 2⁻⁷⁰⁰ | 2⁻⁷⁰⁰ | 2⁻⁷⁰⁰ |

Operand bit patterns:

### T-AXIS-SLENDER
- x_i: 0000000000000000, 0000000000000000, 0000000000000000
- x_j: 4008000000000000, 0000000000000000, 0000000000000000
- v: 0000000000000000, 3ff0000000000000, 0000000000000000
- E: 4247a1df17000000
- G: 423229298c000000
- A_s: 3f76b11c6d1e108c
- I_y: 3f20d6bce5a40934
- I_z: 3f20d6bce5a40934
- mbar: 4054600000000000
- ibar: 3ff08f5c28f5c28f
- jbar: 40008f5c28f5c28f
### T-SKEW-STUBBY
- x_i: 3ff0000000000000, 4000000000000000, 4008000000000000
- x_j: 3ff3000000000000, 4002000000000000, 400e000000000000
- v: 4050c00000000000, c033000000000000, 404e000000000000
- E: 4247a1df17000000
- G: 423229298c000000
- A_s: 3f76b11c6d1e108c
- I_y: 3f20d6bce5a40934
- I_z: 3f20d6bce5a40934
- mbar: 4054600000000000
- ibar: 3ff08f5c28f5c28f
- jbar: 40008f5c28f5c28f
### T-SKEW-UNEQUAL
- x_i: 0000000000000000, 0000000000000000, 0000000000000000
- x_j: 3fd0000000000000, 3fd8000000000000, 3fe8000000000000
- v: 4018000000000000, 4000000000000000, c008000000000000
- E: 42474876e8000000
- G: 4232a05f20000000
- A_s: 3f3a36e2eb1c432d
- I_y: 3f04f8b588e368f1
- I_z: 3ee4f8b588e368f1
- mbar: 4049000000000000
- ibar: 3fd0000000000000
- jbar: 3fe0000000000000
### T-AXIS-NEAR-EB
- x_i: 0000000000000000, 0000000000000000, 0000000000000000
- x_j: 0000000000000000, 0000000000000000, 4000000000000000
- v: 3ff0000000000000, 0000000000000000, 0000000000000000
- E: 4247a1df17000000
- G: 423229298c000000
- A_s: 41f4a35b10000000
- I_y: 3f20d6bce5a40934
- I_z: 3f20d6bce5a40934
- mbar: 4054600000000000
- ibar: 3ff08f5c28f5c28f
- jbar: 40008f5c28f5c28f
### T-RANGE-TINY-MASS
- x_i: 0000000000000000, 0000000000000000, 0000000000000000
- x_j: 3ff0000000000000, 0000000000000000, 0000000000000000
- v: 0000000000000000, 3ff0000000000000, 0000000000000000
- E: 3ff0000000000000
- G: 3ff0000000000000
- A_s: 2728000000000000
- I_y: 3ff0000000000000
- I_z: 3ff0000000000000
- mbar: 1430000000000000
- ibar: 1430000000000000
- jbar: 1430000000000000
