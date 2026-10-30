# Declared-problem oracle: the frozen per-entry K_local bound

**Status: frozen statement, committed before any product dump, envelope or
other product output was produced, opened or compared.** It is not changed
after outputs are seen. If it is later believed mis-derived, that is reported
separately with its reason; the O4 implementation manager decides.
`bound.json` beside this file is its machine-checkable form, and
`oracle.py` reads that file and refuses to run if its derived counts disagree
with the formulas restated in `bound.json`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Every
model is invented. No code SIF, flexibility formula, table or allowable
appears (F-PIP-1), and no `domains/piping-design/` artifact is used (DEC-043).

## 1. What is bounded

For every frame of a selected retained-source case, the normalized private
payload declares a binary64 local stiffness matrix K̂ (`local_matrix_bits`)
and its formation inputs (`formation_inputs`, with `shear_area_bits` on a
Timoshenko frame). The oracle re-forms the exact matrix K* in rational
arithmetic from those formation inputs, taken as the exact values of their
binary64 bit patterns, and requires, for every one of the 144 entries,

    |K̂[r][c] − K*[r][c]|  ≤  ρ_N(r,c) · S(r,c)

with ρ_k = (1 − ε)^(−k) − 1, ε = 2^−53 (binary64, round to nearest), the count
N and scale S of the entry's role given below, and K̂'s entry taken exactly.
ρ_k is evaluated exactly in rationals (a half-integer k uses a rational upper
bound of the square root), so the comparison is exact.

### 1.1 The exact reference K* (DEL-05-03 U8 §3.3)

With E, G, A, I_y, I_z, J, A_s and L the formation inputs, per bending plane
with I (I_z for v = u_y, θ = r_z; I_y for v = u_z, θ = −r_y):

    φ     = 12·E·I / (G·A_s·L²)            (Timoshenko; φ = 0 for Euler–Bernoulli)
    k_vv  = 12·E·I / (L³·(1 + φ))
    k_vθ  =  6·E·I / (L²·(1 + φ))
    k_θθ  = (4 + φ)·E·I / (L·(1 + φ))
    k_θθ' = (2 − φ)·E·I / (L·(1 + φ))
    k_a   = E·A / L        (axial; A is the metal area, never A_s)
    k_t   = G·J / L        (torsion; κ never touches GJ)

DOF order per node is (u_x, u_y, u_z, r_x, r_y, r_z), node i then node j
(indices 0–5 and 6–11), in the member's local frame. The entries are placed
where the two-node frame places them (U8 §3.3: "placed exactly where
`add_bending_z` / `add_bending_y` place the Euler–Bernoulli coefficients"),
with the bending-plane conventions above (θ = −r_y in the x–z plane flips the
sign of every v–r_y coupling):

| Entries (and their symmetric partners) | Value |
|---|---|
| (0,0), (6,6) / (0,6) | k_a / −k_a |
| (3,3), (9,9) / (3,9) | k_t / −k_t |
| (1,1), (7,7) / (1,7) | k_vv(I_z) / −k_vv(I_z) |
| (1,5), (1,11) / (5,7), (7,11) | k_vθ(I_z) / −k_vθ(I_z) |
| (5,5), (11,11) / (5,11) | k_θθ(I_z) / k_θθ'(I_z) |
| (2,2), (8,8) / (2,8) | k_vv(I_y) / −k_vv(I_y) |
| (2,4), (2,10) / (4,8), (8,10) | −k_vθ(I_y) / k_vθ(I_y) |
| (4,4), (10,10) / (4,10) | k_θθ(I_y) / k_θθ'(I_y) |
| every other entry | structural zero |

K* is symmetric; each entry (r,c) with r > c takes the value of (c,r). A
structural zero must be declared exactly ±0 (bound 0). The layout is part of
the reference: a declared matrix in another layout fails, and the harness
validation on Euler–Bernoulli frames (section 4) exposes such a fault as a
harness fault rather than an O4 fault.

### 1.2 The length L

- If `formation_inputs` declares a binary64 length leaf, L is that leaf, taken
  exactly, and its count is ℓ = 0.
- Otherwise L is the exact Euclidean distance between the member's two
  declared binary64 node positions (the payload's, or else the request's when
  every coordinate converts to SI exactly), carried as a rational enclosure of
  the square root narrower than 2^−200 relative, and the comparison takes the
  worst point of that enclosure. Its count is ℓ = 3.5 (section 2.2).
- If neither is available exactly, the frame is reported **not judged** for
  this bound; nothing is guessed.

## 2. Derivation

### 2.1 Counting rule (o4-source-extension.md §4.4)

x̂ = x*·π_k means π_k ∈ [(1 − ε)^k, (1 − ε)^−k] for a count k that may be
fractional. Then |π_k − 1| ≤ ρ_k exactly, for fractional k too.
- A correctly rounded binary64 operation (+, −, ×, ÷, √) contributes one, as
  (1 + δ) or 1/(1 + δ) with |δ| ≤ ε. An exact operation contributes zero; a
  fused multiply-add only removes roundings. Exact leaves have count zero.
- Ratios multiply and divide by adding counts (π_a·π_b and π_a/π_b are
  π_(a+b)).
- A correctly rounded square root halves its argument's count and adds one.
- A positive sum RN(a + b), a, b > 0, of counts k_a and k_b has count
  max(k_a, k_b) + 1: the computed-to-exact ratio before the rounding is the
  mediant of the two operands' ratios, so it lies between them (the §4.4
  mediant argument). The constants 1, 4 and 2 are exact (count 0).
- **Order independence.** A product or quotient of f factors, evaluated in any
  association and commutation, with any sharing of intermediates, uses exactly
  f − 1 binary multiplicative operations. Its count is therefore at most
  (f − 1) plus the sum of its factors' counts (a factor occurring n times, such
  as L in L³, contributes n times its count), whatever FK's operation order.
  The constants 12 and 6 are counted as factors whose multiplication rounds
  (12 = 3·2², 6 = 3·2; multiplication by 3 rounds). Every operation in U8
  §3.3's named K2a chain is counted as a correctly rounded operation; an
  operation that happens to be exact only lowers the true count.

### 2.2 The length count ℓ

With a declared length leaf, ℓ = 0 against that leaf. Without it, from binary64
node coordinates: each coordinate difference rounds once (1); each square
doubles that and rounds (3); two rounded positive additions give
max(3, 3) + 1 = 4 and then max(4, 3) + 1 = 5; the correctly rounded square
root halves that and adds its own rounding: 5/2 + 1 = 3.5. This is the plain
chain; §4.4's `ExactWideSum` chain (2.5) is lower, so ℓ = 3.5 covers both.

### 2.3 Counts per coefficient

Leaves E, G, A, I, J, A_s have count 0; L has ℓ.

| Quantity | Factors f | Count |
|---|---|---|
| φ = 12·E·I/(G·A_s·L·L) | 7 | k_φ = 6 + 2ℓ |
| 1 + φ, 4 + φ (positive sums) | — | k_φ + 1 = 7 + 2ℓ |
| k_vv = 12·E·I/(L·L·L·(1+φ)) | 7 | 6 + 3ℓ + (7 + 2ℓ) = **13 + 5ℓ** |
| k_vθ = 6·E·I/(L·L·(1+φ)) | 6 | 5 + 2ℓ + (7 + 2ℓ) = **12 + 4ℓ** |
| k_θθ = (4+φ)·E·I/(L·(1+φ)) | 5 | 4 + ℓ + 2·(7 + 2ℓ) = **18 + 5ℓ** |
| k_θθ' = (2−φ)·E·I/(L·(1+φ)) | 5 | absolute form below, **18 + 5ℓ** |
| k_a = E·A/L, k_t = G·J/L | 3 | **2 + ℓ** |

The (4 + φ) and (1 + φ) errors are correlated through the one φ̂; adding
their counts ignores that and is conservative.

**k_θθ', the cancelling entry.** d = 2 − φ has no relative bound. With
φ̂ = φ*·π_(k_φ), |φ̂ − φ*| ≤ ρ_(k_φ)·φ*, and d̂ = RN(2 − φ̂) (or exact):

    |d̂ − d*| ≤ ρ_(k_φ)·φ* + ε·|d̂|,  |d̂| ≤ (|d*| + ρ_(k_φ)·φ*)/(1 − ε)
    ⇒ |d̂ − d*| ≤ (ρ_(k_φ)·φ* + ε·|d*|)/(1 − ε) ≤ ρ_(k_φ+1)·(φ* + |d*|),

since ρ_k/(1 − ε) ≤ ρ_(k+1) and ε/(1 − ε) = ρ_1 ≤ ρ_(k+1). The rest of the
entry is the product d̂·M* ·π_m with M* = E·I/(L·(1 + φ*)) and
m = 4 + ℓ + (k_φ + 1) = 11 + 3ℓ (five factors d, E, I, L, 1 + φ). Then

    |k̂ − k*| ≤ M*·(|d̂ − d*|·(1 − ε)^−m + |d*|·ρ_m)
             ≤ M*·(φ* + |d*|)·(ρ_(k_φ+1)·(1 − ε)^−m + ρ_m)
             =  ρ_(k_φ+1+m) · (φ* + |2 − φ*|) · E·I/(L·(1 + φ*)),

using ρ_a·(1 − ε)^−m + ρ_m = ρ_(a+m) exactly. So N = (7 + 2ℓ) + (11 + 3ℓ) =
18 + 5ℓ, and the scale is S = (φ* + |2 − φ*|)·E·I/(L·(1 + φ*)) ≥
2·E·I/(L·(1 + φ*)) > 0. It holds at φ* = 2 exactly, where k* = 0 and the bound
is absolute.

**Euler–Bernoulli frames** (φ = 0; U8 §3.3: "The Euler–Bernoulli branch is
the existing code"), judged with the same rule: k_vv = 12·E·I/L³ (f = 6:
**5 + 3ℓ**), k_vθ = 6·E·I/L² (f = 5: **4 + 2ℓ**), k_θθ = 4·E·I/L and
k_θθ' = 2·E·I/L (f = 4: **3 + ℓ**), k_a and k_t (**2 + ℓ**). All are relative
(S = |k*|).

## 3. The resulting bounds (ε = 2^−53)

ρ_k is about k·ε = k·1.110223e−16.

| Role | S | N (ℓ = 0) | ≈ ρ_N | N (ℓ = 3.5) | ≈ ρ_N |
|---|---|---|---|---|---|
| Timoshenko k_vv | \|k*\| | 13 | 1.44e−15 | 30.5 | 3.39e−15 |
| Timoshenko k_vθ | \|k*\| | 12 | 1.33e−15 | 26 | 2.89e−15 |
| Timoshenko k_θθ | \|k*\| | 18 | 2.00e−15 | 35.5 | 3.94e−15 |
| Timoshenko k_θθ' | (φ* + \|2 − φ*\|)·E·I/(L·(1 + φ*)) | 18 | 2.00e−15 | 35.5 | 3.94e−15 |
| Euler–Bernoulli k_vv | \|k*\| | 5 | 5.6e−16 | 15.5 | 1.72e−15 |
| Euler–Bernoulli k_vθ | \|k*\| | 4 | 4.4e−16 | 11 | 1.22e−15 |
| Euler–Bernoulli k_θθ, k_θθ' | \|k*\| | 3 | 3.3e−16 | 6.5 | 7.2e−16 |
| k_a, k_t (both theories) | \|k*\| | 2 | 2.2e−16 | 5.5 | 6.1e−16 |
| structural zero | — | — | 0 (exactly ±0) | — | 0 |

**What it discriminates.** A wrong Timoshenko recipe moves an entry by a
relative amount of order φ (for example 4 + 2φ for 4 + φ, A for A_s in φ, or a
dropped 1 + φ), which exceeds 4e−15 wherever φ is above about 1e−14. The bound
is independent of FK's operation order within U8 §3.3's chain (section 2.1).
A recipe that restructures the algebra (for example forming k_vv through
G·A_s·L² + 12·E·I) is not what U8 §3.3 states; its count is not derived here.

## 4. Supplementary rules stated before outputs

These are exact or coarse screens, frozen with the bound, and reported apart
from it:
1. **Harness validation.** The Euler–Bernoulli zero-allowance originals are
   judged by every rule. A failure there is a harness fault, not an O4 fault.
2. **Shear-area leaf.** On a Timoshenko frame, A_s equals, bit for bit,
   RN(κ̂·Â_n) with κ̂ in U8 §3.1's fixed binary64 order (no FMA) from
   r_o = D/2, r_i = fl(D/2 − t) on the nominal wall and the member's ν, and
   Â_n the declared area leaf; where ν, D or t is not available exactly, this
   rule is reported not judged.
3. **Section leaves (coarse screen).** The declared A, I and J of the stiffness
   section are within 2^−40 relative of the exact annulus of (D, t) on the
   nominal wall (π enclosed rationally), with J = 2I exactly in the annulus.
   Each stress check's parameters are within 2^−40 relative of the exact
   annulus of its own basis (nominal: t; reduced: t_r = fl(fl(t − c_m) − c_a)),
   with r_o = D/2 exactly and Z = I/r_o. This screen is not a derived rounding
   bound; it separates the bases (they differ by percent) and catches the
   wall t − c_m in stiffness.
4. **Projections and stress checks** (task items (b) and (c)) are judged
   exactly: an interval contains the exact rational value of its functional,
   endpoints taken exactly; a stress check's published value is recomputed from
   its action and its own parameters as `oracle.py` states, and its action is
   the exact functional's enclosure.

## Addendum (after independent review), 2026-10-19

This addendum adds to the frozen text above and replaces nothing. The counts
in sections 2 and 3 and in `bound.json` are unchanged.

**The finding.** An independent review (repair round 1) observed that section
2.1's order-independence claim holds for association, commutation and sharing
within one factor tree, but not for algebraic rewrites that add an operation.
Its example is E·A/L formed as (E·A)·RN(1/L), with the reciprocal of the length
rounded separately. That is three roundings, and a random search found an
operand set where the result exceeds ρ₂ by a factor of 1.0079.

**The scope, from the specification text.** DEL-05-03 U8 §3.3 states the
formation as the expressions φ = 12EI/(G·A_s·L²), k_vv = 12EI/(L³(1+φ)),
k_vθ = 6EI/(L²(1+φ)), k_θθ = (4+φ)EI/(L(1+φ)) and k_θθ' = (2−φ)EI/(L(1+φ)).
It adds "Axial EA/L and torsion GJ/L are unchanged" and "Each intermediate is a
named K2a checked formation", and names (2 − φ) and its product as
intermediates. The text names no reciprocal length and no other rewrite: no
1/L, 1/L², 1/(1+φ) or G·A_s·L² + 12EI form appears for any K_local entry.
o4-source-extension.md §4.4's description of the retained kernel's
`over_length`, RN(RN(E·I)/L̂), also divides by the length rather than
multiplying by a rounded reciprocal. This derivation reads only that text; no
product output entered it.

**The stated exclusion.** The bound covers every evaluation of U8 §3.3's
expressions as written: any association and commutation of each expression's
factors, any sharing of intermediates, fused multiply-adds, and operations
that happen to be exact. It does not cover algebraic rewrites that introduce
operations the expressions do not contain. Examples are a separately rounded
reciprocal (multiplying by RN(1/L), RN(1/L²) or RN(1/(1+φ)) in place of the
division), a rewrite of (1+φ) through G·A_s·L² + 12·E·I, or a recombination
of entries such as k_θθ' = k_θθ − φ·EI/(L(1+φ)). Such a chain has its own
count, which this unit does not derive. If the product's chain is a rewrite,
the bound can fail on a correct formula. That would be reported as a scope
finding with this reason, not as a formula defect, and the counts would be
re-derived from the rewritten chain's own text.
