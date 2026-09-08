# M9-C1 independent component derivation

Status: `DERIVATIVE_CANDIDATE_NOT_ACCEPTED_AUTHORITY`
Package / deliverable: `PKG-09 / DEL-09-01`
Input fixture: `fixtures/product_preview/invented_preview_model.json`, SHA-256 `986c055944776ca8d0d849d552678e1f2bfb6c4559bbb9ff8a1928157a4f871c`
Independence fence: equations, inputs, cases, signs and expected results were frozen without reading product output, fixture expected-result constants, manager calculations, or production source. No `domains/piping-design` equation artifact was used.

## Applicability and case boundary

The computed branch is a small-displacement, linear-elastic, prismatic 3D Euler-Bernoulli centerline frame. It uses the fixture's four pipe segments, circular section, `E=200 GPa`, `G=77 GPa`, anchor and guide restraints, retained `SH-140` `UZ` spring with `k=42000 N/m`, the `NL-140` one-way `UY` support, and the `NL-130-FRIC` `UZ` friction action with `mu=0.01` and current `S-130/UY` reaction. Units are N, m, Pa and rad.

Both fixture load cases are evaluated without either pressure primitive:

| Case | P-120 global Z load | N-140 global Y force | P-120 thermal interval | Pressure |
|---|---:|---:|---:|---|
| `load:L-100` | -190 N/m over 4.4 m | +350 N | +12.5 degC | omitted |
| `load:L-200` | -95 N/m over 4.4 m | +125 N | none | omitted |

`CE-120.constant_load`, the spring installed/cold/hot loads, and `C-130.geometry.weight` are metadata rather than primitive loads in this branch. Bend, branch and valve modifiers are excluded because their interfaces say `mechanics_geometry_only`. The expansion-joint user stiffness is excluded from the numerical branch because the fixture does not specify a second connector node, rigid offsets, insertion length, or whether its six stiffnesses replace P-130, act in series, or act in parallel. That is an exact topology blocker for a component-stiffness-inclusive answer.

The retained spring uses the explicit zero-reference branch

\[
R_{s,z}=-k_s(u_{140,z}-0).
\]

The fixture does not define a force-free reference displacement or say that installed/cold/hot load is preload. Any preload branch is therefore blocked pending that physical choice.

## Coordinates, axes and signs

Global coordinates are the fixture's right-handed `X,Y,Z`. Every node has global DOFs

\[
\mathbf d_n=[U_X,U_Y,U_Z,R_X,R_Y,R_Z]^T.
\]

For an element from `i` to `j`, `e_x=(x_j-x_i)/L`. The declared `y_reference` is projected normal to `e_x` and normalized as `e_y`; `e_z=e_x cross e_y`. With `R=[e_x^T;e_y^T;e_z^T]`, local components are `v_l=R v_g`. The 12-DOF transformation is `T=diag(R,R,R,R)`, so

\[
\mathbf d_l=T\mathbf d_g,\qquad K_g=T^T K_l T,\qquad \mathbf f_g=T^T\mathbf f_l.
\]

The resulting triads are:

| Element | `e_x` global | `e_y` global | `e_z` global |
|---|---|---|---|
| P-100 | (+X) | (+Z) | (-Y) |
| P-110 | (+Y) | (+Z) | (+X) |
| P-120 | (+X) | (+Z) | (-Y) |
| P-130 | (+Z) | (+Y) | (-X) |

The section values calculated from `Do=0.168 m`, `t=0.007 m`, `Di=Do-2t` are

\[
A={\pi\over4}(D_o^2-D_i^2)=0.003540574920595698\;m^2,
\]
\[
I_y=I_z={\pi\over64}(D_o^4-D_i^4)=1.1493591335983794\times10^{-5}\;m^4,
\quad J=I_y+I_z=2.2987182671967588\times10^{-5}\;m^4.
\]

## Local 3D frame stiffness

Local ordering is

\[
[u_i,v_i,w_i,\theta_{xi},\theta_{yi},\theta_{zi},u_j,v_j,w_j,\theta_{xj},\theta_{yj},\theta_{zj}].
\]

The axial and torsional submatrices on `[u_i,u_j]` and `[theta_xi,theta_xj]` are

\[
{EA\over L}\begin{bmatrix}1&-1\\-1&1\end{bmatrix},\qquad
{GJ\over L}\begin{bmatrix}1&-1\\-1&1\end{bmatrix}.
\]

For bending displacement `v` about local `z`, on `[v_i,theta_zi,v_j,theta_zj]`,

\[
{EI_z\over L^3}
\begin{bmatrix}
12&6L&-12&6L\\
6L&4L^2&-6L&2L^2\\
-12&-6L&12&-6L\\
6L&2L^2&-6L&4L^2
\end{bmatrix}.
\]

For `w` bending about local `y`, positive `theta_y` produces negative `w` slope. On `[w_i,theta_yi,w_j,theta_yj]`,

\[
{EI_y\over L^3}
\begin{bmatrix}
12&-6L&-12&-6L\\
-6L&4L^2&6L&2L^2\\
-12&6L&12&6L\\
-6L&2L^2&6L&4L^2
\end{bmatrix}.
\]

Assembly adds the ground spring stiffness directly to global `K[N-140,UZ;N-140,UZ]`. Prescribed rigid-support DOFs are eliminated at zero displacement.

## Consistent loads and thermal strain

For constant local distributed force `q=[qx,qy,qz]`, the consistent local nodal vector is

\[
\mathbf f_q=
[q_xL/2,q_yL/2,q_zL/2,0,-q_zL^2/12,q_yL^2/12,
 q_xL/2,q_yL/2,q_zL/2,0,q_zL^2/12,-q_yL^2/12]^T.
\]

For P-120, global `-Z` is local `-y`. The L-100 vector has `v_i=v_j=-418 N`, `theta_zi=-306.533333333333 N m`, and `theta_zj=+306.533333333333 N m`. A P-120-only cantilever control gives fixed-end action `Vy=+836 N`, `Mz=+1839.2 N m`, free-tip `v=-0.00387247455550761 m`, and `theta_z=-0.00117347713803261 rad`. A scalar zero-reference spring under the same `-836 N` resultant gives `u=-0.0199047619047619 m` and spring action `+836 N`.

The P-120 thermal primitive uses the initial axial strain `epsilon0=alpha DeltaT` and equivalent vector

\[
\mathbf f_T=[-EA\epsilon_0,0,0,0,0,0,+EA\epsilon_0,0,0,0,0,0]^T.
\]

It has zero resultant force and moment. In the solved L-100 branch it produces the free P-120 axial growth `alpha DeltaT L=0.00066 m` at N-130 relative to restrained N-120.

## End and station recovery

The reported local element equilibrium-end vector is

\[
\mathbf a_e=K_l\mathbf d_l-\mathbf f_{q,l}-\mathbf f_{T,l}.
\]

This sign convention is the vector assembled into global nodal equilibrium. For a station `x` from end i, the JSON reports the force and moment exerted by the right portion on the left segment:

\[
\mathbf C_F(x)=-(\mathbf a_{i,F}+\mathbf qx),
\]
\[
\mathbf C_M(x)=-\mathbf a_{i,M}+x(\mathbf e_x\times\mathbf a_{i,F})+{x^2\over2}(\mathbf e_x\times\mathbf q).
\]

Values are reported at `x/L = 0, 0.25, 0.5, 0.75, 1`. This definition makes the station endpoints exactly auditable against the end vectors: `C(0)=-a_i` and `C(L)=a_j` for the complete element.

## Rigid and unilateral support actions

With applied consistent/nodal vector `F`, prescribed friction force `Ff` for a sliding branch, and spring included in `K`, the action on the structure at a constrained DOF is

\[
\mathbf R_c=(K\mathbf u-\mathbf F-\mathbf F_f)_c.
\]

For `NL-140`, the fixture says only `active_when: negative_reaction`. To state a complete complementarity check, the calculated interpretation blocks positive UY motion:

\[
g=-u_{140,y}\ge0,\quad \lambda=-R_{140,y}\ge0,\quad g\lambda=0.
\]

Thus active means `u140,y=0` and `R140,y<=0`; open means `R140,y=0` and `u140,y<=0`. The alternative open solves give `u140,y=+0.00534914262604520 m` (L-100) and `+0.00177576289599474 m` (L-200), which violate this gap-side interpretation. The active reactions are negative, so the active branch is internally consistent. If the product intends the opposite gap side, the fixture must say so.

## Current-normal Coulomb friction

Let tangential positive direction be global `+Z` at N-130. The same solved state supplies

\[
N=|R_{S-130,UY}|.
\]

For the static zero-reference interpretation, stick imposes `u_t=0` and is admissible only when `|R_f|<=mu N`. Slip requires

\[
R_f=-\mu N\,sign(u_t),\qquad u_t\ne0.
\]

No prior nonlinear iterate or prior-normal force enters this check. Because the linear frame gives `R_N(f)=a+bf` for a prescribed tangential friction force `f`, each assumed slip sign `s` and normal-reaction sign `rho` is solved without iteration:

\[
f={-\mu s\rho a\over1+\mu s\rho b}.
\]

All four `(s,rho)` branches are evaluated and checked for displacement sign, normal sign, opposing action, Coulomb equality, and unilateral-support admissibility. Exactly one branch is admissible for each pressure-free load case. The stick trials are strongly inadmissible:

| Case | Stick reaction `Rf` | Current `mu N` | Stick margin `muN-|Rf|` |
|---|---:|---:|---:|
| L-100 | +395.474076169 N | 0.0852008191 N | -395.388875350 N |
| L-200 | +197.737038085 N | 0.0426004095 N | -197.694437675 N |

The admissible zero-reference sliding results are:

| Quantity | L-100 | L-200 |
|---|---:|---:|
| N-130 `UZ` | -0.00664392679739 m | -0.00332196339870 m |
| N-140 `UZ` | -0.00664305996255 m | -0.00332152998128 m |
| `S-130/UY` action | -52.3732819873 N | -26.1866409937 N |
| `N=abs(S-130/UY)` | 52.3732819873 N | 26.1866409937 N |
| Friction `UZ` action | +0.523732819873 N | +0.261866409937 N |
| `NL-140/UY` action | -297.626718013 N | -98.8133590063 N |
| `SH-140/UZ` action | +279.008518427 N | +139.504259214 N |
| N-130 `UX` | +0.000660000000000 m | 0 m |

The friction force opposes the negative UZ slip and equals the bound formed from the same returned S-130 normal action. `EXPECTED_RESULTS.json` contains every signed nodal displacement, rigid/support/spring/friction action, element-end vector, station-cut vector, rejected branch, and raw balance residual.

## Separate force and moment balance

Force balance sums physical nodal and distributed loads, rigid support actions, the ground-spring action, and sliding friction. Thermal equivalent loads are excluded from this physical-load sum because they have zero global force and moment resultant. Moment balance is independently formed about global origin from every force at its point plus rigid rotational support actions.

| Case | Global force residual `[X,Y,Z]` N | Global moment residual `[X,Y,Z]` N m |
|---|---|---|
| L-100 | `[0, 0, -2.60644e-10]` | `[-6.24569e-10, 1.97414e-9, 0]` |
| L-200 | `[0, 0, -1.30152e-10]` | `[-3.12000e-10, 9.86162e-10, 0]` |

These are raw double-precision closure observations, not pass/fail tolerances. No numerical acceptance threshold is proposed.

## Exact blockers and forks

1. **Expansion-joint topology:** the fixture has stiffness values but no unambiguous two-ended mechanical topology, axis/reference insertion rule, or series/replacement/parallel choice. No unique component-stiffness-inclusive matrix exists.
2. **Spring reference/preload:** the fixture says stiffness is solver-consumed but does not select a force-free displacement or identify installed/cold/hot load as an applied preload. Only the zero-reference stiffness-only branch is calculated.
3. **Friction history:** `initial_state: sliding` does not define committed slip displacement, load-step order, reversal state, or prior converged normal. The current-normal static zero-reference branch is calculated; a path-dependent answer is not unique.
4. **One-way gap side:** negative active reaction is directly checkable, but the allowed displacement side is not explicit. Open feasibility is evaluated only under the stated block-positive-UY interpretation.
5. **Acceptance tolerance:** no fixture or accepted decision gives a comparison tolerance; the package reports expected values and raw residuals only.

## Public reference applicability

- [OpenSees 3D elastic beam-column documentation](https://opensees.github.io/OpenSeesDocumentation/user/manual/model/elements/elasticBeamColumn.html) confirms the standard `A,E,G,J,Iy,Iz` parameterization of a 3D elastic frame element. It is used only as public formulation context.
- [OpenSees linear coordinate transformation](https://opensees.github.io/OpenSeesDocumentation/user/manual/model/geomTransf/Linear.html) confirms node-defined local x, reference-vector construction of transverse axes, and stiffness/force transformation. This derivation explicitly states its own `y_reference` projection convention.
- [OpenSees beam uniform load documentation](https://opensees.github.io/OpenSeesDocumentation/user/manual/model/pattern/PlainPatternloadcommands/eleLoad.html) confirms local 3D uniform beam-load components. The consistent vector above is independently integrated from cubic Hermite/linear axial shape functions.
- [COMSOL tangential contact theory](https://doc.comsol.com/6.4/doc/com.comsol.help.sme/sme_ug_theory.06.088.html), already cited by accepted E1, is used only to support the distinction between static stick/slip closure and incremental history. No COMSOL penalty or surface-contact formulation is adopted.

These public references do not constitute owner-vetted engineering acceptance, and no protected standard or proprietary benchmark was used.
