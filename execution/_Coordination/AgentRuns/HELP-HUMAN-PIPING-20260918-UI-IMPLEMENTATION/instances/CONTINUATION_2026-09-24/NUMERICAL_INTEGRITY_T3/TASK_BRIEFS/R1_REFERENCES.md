# R1 — T3 independent references (VP-ORACLES and VP-ROBUST)

Reference author TASK. Read `_COMMON.md` first.

## Purpose

Derive and freeze, before any T3 implementation, the hand-derived references that will judge the general accuracy method, range support and sparse scale. They must be independent of product code, and each must come with negative controls: the specific wrong answers a defective implementation would produce.

## Independence

- **Do not read product source** (`P/core/**`, `P/apps/**`) or product test expectations. Do not import or call production code.
- You may read: `STAGE0_MAP.md`, `STAGE1_PLAN.md`, `CORRECTNESS_DESIGN/NUMERICAL_REFERENCE.md`, `CORRECTNESS_DESIGN/CONTRIBUTION_PRECISION/{CONTRACT,INDEPENDENT_REFUTATION}.md`, and the existing independent reference package `P/validation/benchmarks/numerical_integrity/{README.md,COVERAGE.md,generate.py,fixtures.json,test_reference.py}` for conventions and to avoid duplicating frozen cases. You may read `DEFAULT_ROUTE_DESIGN/references.py` for its presentation style.
- Use Python's standard library only (`fractions`, `decimal`). Exact rational arithmetic is preferred. Where π enters, carry it symbolically or at 100 digits and state the reference's own accuracy.

## Formulation

The product implements a small-displacement, linear-elastic Euler–Bernoulli space frame (no shear deformation), with rigid and linear-spring supports and nodal loads. Write references in that theory. State for every case: node coordinates (m), member connectivity, section (use the N-series section unless a case needs otherwise: E = 200 GPa, G = 80 GPa, OD = 0.2 m, ID = 0.18 m, so EA = 380000000π N, EI = 1719500π N·m², GJ = 1375600π N·m²), restrained DOFs and spring constants, and loads. All values are invented; no material or component data.

Express expectations in convention-free quantities so no product frame convention leaks in: global nodal displacements and rotations, global support reactions (force and moment per restrained DOF or spring), and per-member invariants (axial force, torque, and bending-moment magnitude `hypot(My, Mz)` at each end and at stated stations).

## Required families

Each family needs a stated purpose, the exact expressions or derivation, the frozen values, the reference's own accuracy, a **zero scale** for every expected zero (derived from the case's load and geometry scales, with the derivation), and negative controls.

1. **RF-CHAIN, order > 2 soft chains.** Series chains of 3, 5 and 10 torsion or axial members with a soft grounding spring, generalizing N05 (`k/a` from 1e-4 down to 1e-12 relative). Closed form by series compliance. Include the relative motions (element twists or extensions) as expected quantities, not only nodal values.
2. **RF-SKEW, skewed members.** N05-class soft restraints on members along `(3,4,0)/5` and `(1,2,2)/3`, with the spring axis along the member, and with it off-axis. Use rational orthogonal rotations (from Pythagorean quadruples) so rotated coordinates are exact rationals.
3. **RF-WEAK, weak coupling.** Two stiff subsystems joined by a soft spring, loaded on one side, so the far response is proportional to the coupling. Include a 3D case with bending–torsion coupling (for example an L-shaped two-member frame with a soft root rotational spring).
4. **RF-LARGE, scale.** Statically determinate trees or chains whose exact answer is available at any size (n = 10, 100, 1000 and 10000 members), axis-aligned and rotated. Plus at least one indeterminate family with a closed form (for example a continuous beam under nodal loads, by the three-moment equation). These serve M32 parity and memory runs as well as accuracy.
5. **RF-INVARIANCE.** Transformations of RF-1 to RF-4 base cases with exactly transformed expectations: general rotation, translation to large origin offsets (1e3 and 1e6 m), node and member relabelling, and the same physics authored in m/N/Pa and in mm/N/MPa (moments in N·m and N·mm).
6. **RF-RANGE.** The same problems scaled exactly by powers of two (lengths, stiffnesses and loads, separately and together) toward both ends of the binary64 normal range, with exactly scaled expectations. The PHYS-R4 fixture (OD 4e-77 m, wall 1e-77 m, length 1 m, E 1 Pa, ν 0.1, pressure 4.7e-170 Pa, fixed, no other loads) already has a frozen stress reference in the physics records; do not re-derive it. Add loaded frame cases at comparably extreme geometry and stiffness scales, where the solve itself is exercised, and state what a correct solve publishes.
7. **RF-ZERO, near-zero budgets.** Cases whose exact outputs include structural zeros (symmetry, pure torsion with zero bending, unloaded directions), each with its derived zero scale, and one case where a tiny nonzero value sits below a naive absolute floor but above the stated scale.
8. **RF-FINITE, finite-accuracy budgets.** For each case above where input rounding matters (irrational coordinates such as 1/3, soft modes), compute the exact solution both for the intended rational inputs and for the binary64-rounded inputs (decode each input with `Decimal.from_float`), and report the relative difference per quantity. Where that difference exceeds 1e-9, say the case cannot discriminate at 1e-9 on intended inputs and freeze the represented-input expectation as the reference basis, as NP-A does.
9. **RF-MECH, negative controls that must be refused.** Genuine mechanisms in larger and skewed systems (for example translation-pinned nodes along a skew line leaving rotation about that line free, and a disconnected sub-assembly inside a large model), each with its null motion stated. A correct implementation refuses them; "recovering" an answer is a failure.

**Negative controls for every family**, derived explicitly: a lost soft contribution (the answer if `a + k` rounds to `a`), relative motion recovered by subtracting two rounded global rotations, a dropped coupling, a wrong frame transform or sign, an omitted prescribed-value term, a first-case or wrong-unit value, and so on, whichever apply.

## What not to do

- Do not change or regenerate any existing frozen reference.
- Propose no tolerance. Comparisons will use the unchanged `|obs − exp| ≤ 1e-9 · max(|exp|, scale)`.
- Do not claim physical validation: these are references for the implemented theory.

## Write set

`T3/REFERENCES/**` only:
- `references.py` (standard library only; deterministic; `python3 references.py` regenerates everything);
- `references.json` (the frozen values, as decimal strings at stated precision, with case ids, units, zero scales, reference accuracy and negative controls);
- `README.md` (the return: families, derivations, independence statement, limits, and what you read and ran);
- `_run_records/` (`references.stdout.txt`, `SHA256SUMS` over the files above, and the Python version used).

## Return

`T3/REFERENCES/README.md`, then a SendMessage summary to the manager with the case count, value count and file hashes. The references are frozen only when ROOT selects them after the independent refutation (V2).
