# Independently frozen current-normal expectations V1

Status: `FROZEN_BEFORE_F4_DIAGNOSTIC_FAN_IN`
Basis: invented M1-N-008 stiffness/load definition only; no production output used
Units: stiffness `N/m`, force/reaction `N`, displacement `m`, friction coefficient dimensionless
Scope: returned-state current-normal consistency under already accepted D-35 / DEC-067; no path/load-step history, zero-reference adoption, direction-policy change, or new acceptance tolerance

The 45-degree rotation of local translational stiffnesses `(100, 200)` gives the global in-plane terms `Kxx=150` and `Kxy=-50`. With `y=0` and `Fy=-10`, the signed normal reaction magnitude is

`N = |-50*u - Fy| = 10 - 50*u`

for the positive-load branch and remains positive for both cases below. The returned sliding state must satisfy tangential equilibrium and `Rf = -sign(u)*0.3*N` using the same returned state.

| Fx | assumed/verified slip sign | equilibrium | exact u | exact N | exact Rf |
|---:|---:|---|---:|---:|---:|
| `+10` | positive | `150u = 10 - 0.3(10-50u)` | `7/135` = `0.05185185185185185` | `200/27` = `7.407407407407407` | `-20/9` = `-2.2222222222222223` |
| `-10` | negative | `150u = -10 + 0.3(10-50u)` | `-7/165` = `-0.04242424242424243` | `400/33` = `12.121212121212121` | `40/11` = `3.6363636363636362` |

For each sign, both initial states (`Sticking`, `Sliding`) and both existing solve modes (`SparseInteractive`, `DenseScrutiny`) are expected to either return the same current-normal-consistent tuple above or fail visibly as non-converged/blocked. A stable `Sliding` label alone is insufficient. This expectation does not claim an adopted static reference for other cases or a general friction-history guarantee.
