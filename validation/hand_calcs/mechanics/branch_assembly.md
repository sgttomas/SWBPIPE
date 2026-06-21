# Branch Assembly Three-Member Benchmark

Fixture ID: `MECH-BRANCH-ASSEMBLY-THREE-MEMBER`

This note supports the PRD section 16.2 branch-assembly solver benchmark. The
fixture is project-original public content derived from elementary frame-member
stiffness relationships. It is not a protected standards example, commercial
software benchmark, proprietary engineering value, code-specific branch
formula, professional approval, or release acceptance criterion.

Final tolerance policy, release thresholds, CI gate policy, benchmark
publication scope, external validation claims, and professional reliance remain
`TBD`.

## Geometry And Boundary Conditions

All quantities use the fixture-local unit basis from the mechanics benchmark
README.

- Header left anchor node: `N0 = (-2.0, 0.0, 0.0)`.
- Branch junction node: `N1 = (0.0, 0.0, 0.0)`.
- Header right anchor node: `N2 = (3.0, 0.0, 0.0)`.
- Branch tip node: `N3 = (0.0, 4.0, 0.0)`.
- Header elements: `N0-N1` and `N1-N2`.
- Branch element: `N1-N3`.
- `N0` and `N2` are anchored in all six degrees of freedom.
- `N1` and `N3` leave only global `UY` free.
- Applied load: `F = +90.0 N` at `N3:UY`.

The shared-node topology is the benchmark target: three frame members assemble
into one global stiffness matrix at the branch junction. The restrained degrees
of freedom reduce the expected response to a two-degree stiffness network.

## Section Values

The benchmark uses the shared mechanics fixture section:

- Elastic modulus: `E = 1200.0 Pa`.
- Shear modulus: `G = 500.0 Pa`.
- Area: `A = 2.0 m^2`.
- Second moment about local `y`: `Iy = 3.0 m^4`.
- Second moment about local `z`: `Iz = 4.0 m^4`.
- Torsion constant: `J = 1.5 m^4`.

## Hand Calculation

Branch axial stiffness:

```text
k_branch = E A / L_branch
         = 1200.0 * 2.0 / 4.0
         = 600.0 N/m
```

With rotations restrained at the branch junction, each header member contributes
lateral stiffness at the junction:

```text
k_left  = 12 E Iz / L_left^3
        = 12 * 1200.0 * 4.0 / 2.0^3
        = 7200.0 N/m

k_right = 12 E Iz / L_right^3
        = 12 * 1200.0 * 4.0 / 3.0^3
        = 2133.3333333333335 N/m

k_header = k_left + k_right
         = 9333.333333333334 N/m
```

The two-degree reduced stiffness equations are:

```text
[ k_header + k_branch   -k_branch ] [ u_junction ] = [ 0 ]
[ -k_branch              k_branch ] [ u_tip      ]   [ F ]
```

Therefore:

```text
u_junction = F / k_header
           = 90.0 / 9333.333333333334
           = 0.009642857142857142 m

branch_extension = F / k_branch
                 = 90.0 / 600.0
                 = 0.15 m

u_tip = u_junction + branch_extension
      = 0.15964285714285714 m
```

Header anchor reactions:

```text
R_left  = -k_left * u_junction
        = -69.42857142857143 N

R_right = -k_right * u_junction
        = -20.571428571428573 N

R_left + R_right = -90.0 N
```

## Expected Values

| Name | Value | Unit | Dimension |
|---|---:|---|---|
| `branch_axial_stiffness` | `600.0` | `N/m` | `linear_stiffness` |
| `header_lateral_stiffness` | `9333.333333333334` | `N/m` | `linear_stiffness` |
| `junction_uy_displacement` | `0.009642857142857142` | `m` | `length` |
| `branch_tip_uy_displacement` | `0.15964285714285714` | `m` | `length` |
| `branch_axial_extension` | `0.15` | `m` | `length` |
| `header_left_uy_reaction` | `-69.42857142857143` | `N` | `force` |
| `header_right_uy_reaction` | `-20.571428571428573` | `N` | `force` |
| `header_reaction_sum` | `-90.0` | `N` | `force` |

