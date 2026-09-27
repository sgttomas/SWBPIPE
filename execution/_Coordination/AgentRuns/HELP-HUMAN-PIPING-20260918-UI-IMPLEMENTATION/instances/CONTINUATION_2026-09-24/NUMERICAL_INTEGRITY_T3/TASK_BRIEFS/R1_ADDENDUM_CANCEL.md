# R1 addendum — RF-CANCEL (cancelling load contributions)

Addendum to [R1_REFERENCES.md](R1_REFERENCES.md), under the same independence rules and write set (`T3/REFERENCES/**`). ROOT made this family top priority (V1-S11). Add it to `references.py`, `references.json` and `README.md` as family RF-CANCEL.

## Cases

Linear-elastic Euler–Bernoulli frame, invented properties (N-series section unless stated), with stated units.

1. **Nodal cancellation on one DOF.** A cantilever tip DOF carries three nodal contributions `(G, n, −G)` in that authored order, with net `n`. Use net `n = 0.3 N` with gross `G` = 1e5, 1e6, 1e7 and 1e8 N, and the extreme `(1e80, 1e-8, −1e80)`. Give the same cases on a moment DOF in N·m.
2. **Order dependence.** The same contributions in the orders `(G, −G, n)` and `(n, G, −G)`.
3. **Cancellation with a response elsewhere.** A second, ordinary load on another DOF of the same body, so the correct response is not a pure scaling of the net load.
4. **Element-equivalent cancellation at a shared node.** Two adjacent spans with uniform transverse loads of opposite sign whose fixed-end moments nearly cancel at the shared node, plus a small nodal moment there. State the exact response of the implemented theory.

## For every case

- The exact response: nodal displacements and rotations, support reactions, and member invariants, all from the exact net load.
- **A stated scale for each quantity, derived and justified.** Say whether the gross or the net load governs it, because the comparison `|obs − exp| ≤ 1e-9 · max(|exp|, scale)` depends on it. Where the choice is contestable, give both and say which one you recommend.
- **Negative controls:** the response to the net load obtained by summing the binary64 inputs left to right in each listed order (compute it with Python floats in that order), and the response with the small contribution dropped.

Standard-library Python only; no product code. Report the family's case and value counts in the README and in your SendMessage summary.
