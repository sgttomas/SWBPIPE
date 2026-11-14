# Timoshenko straight mass: exact element references

These references serve DEL-04-09 REQ-001 and AC-012. They cover the consistent mass of a straight frame member whose bending stiffness is the Timoshenko element.

The mass is formed in each bending plane as M = m̄·∫N_wᵀN_w + ī·∫N_θᵀN_θ. Here N_w and N_θ are the interpolation the Timoshenko stiffness implies: the exact homogeneous static solution, of the interdependent-interpolation family of Friedman and Kosmatka, *Computers & Structures* 47(3), 1993. That interpolation is derived here, not copied. The axial mass m̄ and the torsional mass j̄ are linear. The result is assembled to 12×12 in member axes and turned to global axes.

## Files

| File | What it is |
|---|---|
| `generators/modal_timoshenko_mass_reference.py` | The generator. It uses the Python 3 standard library only, and reads, imports and runs no product code, product output, fixture or schema. |
| `references.json` | One record per case. Each record holds: the operands as binary64 bit patterns and values; the chord length, Φ_y and Φ_z; both plane matrices on (w_i, θ_i, w_j, θ_j); and the nonzero upper triangle of the global 12×12. Each value is given as a 40-digit decimal and as a double-double pair `hi lo`: hi is the value rounded once to binary64, and lo is the remainder rounded once. |
| `SHA256SUMS` | Hashes of this README, the generator and `references.json`, written by the generator. |

## Method

In each plane the generator works from the exact equations, with Φ = 12EI/(G·A_s·L²) and θ the section rotation:

- EI·θ'' + G·A_s·(w' − θ) = 0, with constant shear force;
- so θ = c₁ + c₂x + c₃x² and w' = θ − c₃ΦL²/6.

It solves the four coefficients from the nodal values exactly and integrates the products of the shape polynomials exactly over [0, L]. No closed-form mass entry is used.

Each case's coordinates and y reference make the chord length and member axes rational, so every value is exact until it is rounded for output.

## Conventions

These are the frame kernel's. They were checked against `core/solver/frame_kernel/src/lib.rs` (`timoshenko_phi`) and the module documentation of `core/solver/modal_eigen/src/reformation/mass.rs`, at main bbf13d190.

- The local DOFs at each node are u_x, u_y, u_z, r_x, r_y, r_z: node i's six, then node j's.
- The x–y plane acts on [u_y, r_z], with I_z and so Φ_z.
- The x–z plane acts on [u_z, −r_y], with I_y and so Φ_y. Its entry (a, b) carries the sign s_a·s_b, with s = (1, −1, 1, −1).
- The member axes are formed in this order:
  - x is the chord's unit vector;
  - y is the unit part of the y reference normal to x;
  - z is x × y.
- The 12×12 is turned to global axes as TᵀMT, with T = diag(R, R, R, R) and R's rows the member axes.
- One rotary inertia per length, ī, serves both planes, as the product forms it.

## Self-checks

Each check is exact; the build fails on any difference.

1. In each plane, the shapes reproduce the kernel's Timoshenko stiffness, EI/(L³(1+Φ))·[12, 6L, (4+Φ)L², (2−Φ)L²; …].
2. At Φ = 0 the plane mass is the Hermitian m̄L/420·[156, 22L, 54, −13L; …] plus the Rayleigh ī/(30L)·[36, 3L, −36, 3L; …].
3. For each global unit translation, the global mass times the rigid translation equals m̄ times the consistent uniform-load vector [L/2, L²/12, L/2, −L²/12] of each plane, turned to global axes. The total is m̄L.
4. The global matrix is symmetric and positive definite: every pivot of an exact symmetric elimination is positive.

## Cases

| Id | What it discriminates |
|---|---|
| `T-AXIS-SLENDER` | A chord along +x and an annulus, slender (Φ about 0.08). |
| `T-SKEW-STUBBY` | A chord along (3, 4, 12)/13, with a y reference that is not normal to it. Annulus, stubby (Φ about 1). |
| `T-SKEW-UNEQUAL` | A chord along (2, 3, 6)/7, with I_y four times I_z (Φ_y about 4, Φ_z about 1). This catches a plane swap. |
| `T-AXIS-NEAR-EB` | A chord along +z, with a very large A_s (Φ about 10⁻¹³): continuity with the Euler–Bernoulli element. |

The section values are illustrative binary64 inputs, not a product section derivation. A_s is given directly; the product derives it from the energy-matched annular shear factor, and that derivation is not referenced here.

## Check

From the package directory:

```sh
python3 -I -B generators/modal_timoshenko_mass_reference.py           # rebuild in memory and compare bytes
python3 -I -B generators/modal_timoshenko_mass_reference.py --write   # rewrite references.json and SHA256SUMS
```

## Limits

- These are element references. They do not reference:
  - the product's mass assembly, its mass per length or rotary inertia;
  - the shear-area derivation;
  - the integrity re-formation or the readers;
  - any frequency.
- The continuum Timoshenko references of AC-012 are separate.
