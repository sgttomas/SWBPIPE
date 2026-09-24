# M31 — straight annular static shear deformation

Design/reference selection, 2026-09-24, HELPS_HUMANS `/root/correctness_design`. `P/` means `projects/chirality-piping/`. This is a new bounded design under the active correctness/reference-quality direction; it changes no product, public version or shared numerical policy. Origins, actual reference arithmetic and independent review are under this subtree. This contract supersedes no pressure, curved-element or joint mechanics by implication.

## 1. Selected theory and section definition

Select a **first-order, small-displacement, prismatic Timoshenko beam with energy-normalized annular shear rigidity**, alongside an explicit Euler–Bernoulli mode. For the two transverse planes of an isotropic circular annulus, `Ksy=Ksz=Ks`. Retain ordinary EA axial and GJ torsional response. Shear flexibility is independent of fitting flexibility and stress-intensification factors.

For metal annulus inner radius a, outer radius b, `0<a<b`, homogeneous isotropic E and nu, with `E>0` and `-1<nu<1/2`, define:

```
m=a/b; A=pi(b-a)(b+a)
I=A(a²+b²)/4; Iy=Iz=I; J=2I
G=E/[2(1+nu)]
D=(7+14nu+8nu²)(1+m²)² + 4m²(5+10nu+4nu²)
kappa_E=6(1+nu)²(1+m²)²/D
Ks=kappa_E G A = 3 E A (1+nu)(1+m²)²/D
```

Ks has units N, but is a **section shear rigidity parameter**, not an applied force. `Ashear=kappa_E A` is effective shear area, not metal area, pressure bore area or mass area. Do not replace A in EA, weight, pressure or axial stress with Ashear. The shear factor must use the same selected mechanical annulus as the element's A/I; nominal, corroded, manufacturing-deduction and stress-section bases remain explicit under M30.

This choice matches Saint-Venant shear strain energy per unit length, `Us=V²/(2Ks)`, for the homogeneous annular section. It is supported by the independent stress-field integration in `INDEPENDENT_REFUTATION.md` and the author-hosted 2013 Steinboeck/Kugi/Mang paper. Its generalized displacement/rotation variables are the selected energy-normalized 1D variables; do not identify them with every literal cross-sectional average or point displacement of a 3D clamped body.

The common Cowper annular coefficient is a **different displacement-average normalization**:

```
kappa_C=6(1+nu)(1+m²)² / [(7+6nu)(1+m²)²+(20+12nu)m²].
```

It is not an algebra error, but it is not the selected energy-matched coefficient. Do not silently substitute it, a solid-circle factor, 5/6, 0.9 or a thin-wall constant. A future named alternative must state its normalization and result meaning. Dynamic dispersion/frequency-matched factors likewise do not automatically replace a static energy basis.

Useful exact checks: `m→1` gives kappa_E→1/2; `m→0` gives `6(1+nu)²/(7+14nu+8nu²)`. At nu=0 the Cowper and energy coefficients coincide for every m, so nu=0 alone cannot distinguish them. To evaluate positive D without cancellation near auxetic/thin limits, its equivalent form is `(7+14nu+8nu²)(1-m²)²+48m²(1+nu)²`. Shared numerical range/finite guards still apply; no new physical wall cutoff is introduced.

E/nu/G must share the selected material and temperature basis. Prefer the already designed isotropic E/nu→G contract; no fabricated nu=0.3 and no silent `nu=E/(2G)-1` conversion of legacy independent constants. Every use of a shared material must respect the same declared constitutive authority. Interpolate E and nu on their common selected basis before deriving G and kappa; do not independently interpolate G/kappa from incompatible points. Missing necessary basis permits edit/save but blocks this requested theory.

## 2. When Euler–Bernoulli remains appropriate

Euler–Bernoulli is the shear-rigid limit of this selected 1D theory. It remains an explicit legitimate approximation when transverse shear's influence on the quantities of interest is negligible under a justified comparison, and the underlying beam assumptions hold. No universal L/D cutoff answers that question: section ratio, E/G, loading, restraints, continuity and the quantity being assessed matter.

For a cantilever with transverse tip force F, the deflection contributions are `db=FL³/(3EI)` and `ds=FL/Ks`; their ratio is `3EI/(KsL²)`. For a full uniform transverse load w, the tip ratio is `4EI/(KsL²)`. For a member with both end rotations fixed, lateral translation stiffness depends on `phi=12EI/(KsL²)`. These are distinct boundary-value problems. The report's universal stiffness multiplier must not become an application accuracy rule.

For a general model, report the chosen element theories and physically meaningful comparison evidence: matched EB/Timoshenko solves where appropriate, signed reaction/displacement changes, and `Ub=integral(M²/(2EI))dx`, `Us=integral(V²/(2Ks))dx` for the actual selected states. An energy fraction is sensitivity evidence, not a bound on every local displacement or reaction. A zero-load/bending-zero case must not produce a fabricated ratio. Refining an element mesh cannot supply missing EB shear compliance, and the element subdivision length must not become a physical applicability cutoff.

Neither theory qualifies very short end regions, concentrated-load bearing zones, restrained warping, local attachment/weld stresses, ovalization/Brazier behavior, elbow flexibility, local shell buckling, pressure-stiffening or large deformation. Timoshenko shear does not repair a straight-chord bend or turn a stocky component into a valid 1D idealization. Those require a curved/shell/solid or section-warping model with independent evidence. Normal circular stress recovery remains its named beam approximation; `V/(kappa A)` is not the actual maximum transverse shear stress field.

## 3. Exact static element, signs and work

In the local x-y plane use transverse displacement v=uy and section rotation theta=rz, right-handed signs. Define effective shear strain `gamma=v'-theta`, curvature `theta'`, shear `V=Ks gamma`, and moment `M=EI theta'`. The selected strain energy is

```
U=1/2 integral_0^L [EI(theta')² + Ks(v'-theta)²] dx.
```

Section rotation theta is **not** centerline slope v'. A positive cantilever tip force gives positive v, theta and gamma; root reaction force and moment are negative. In the x-z plane use `v=uz`, `theta=-ry` and transform conjugate moment signs consistently with the existing frame convention.

For d=[vi,thetai,vj,thetaj], the exact prismatic no-distributed-load stiffness is

```
phi=12EI/(Ks L²)
K=EI/[L³(1+phi)] *
 [ 12,          6L,              -12,          6L
   6L, (4+phi)L²,               -6L, (2-phi)L²
  -12,         -6L,               12,         -6L
   6L, (2-phi)L²,               -6L, (4+phi)L² ]
```

Use this exact static relation, or an algebraically equivalent force-based/mixed formulation, for the first implementation. Do not use a fully integrated equal-order linear v/theta interpolation: it locks as shear rigidity increases. Avoid introducing reduced-integration hourglass modes while fixing locking. The four-DOF block has only its two physical rigid modes; the complete positive-stiffness 3D frame has six.

A convenient independent derivation for the unloaded span is

```
Delta=vj-vi-L(thetai+thetaj)/2
V0=Delta / [L³/(12EI)+L/Ks]
M0=EI(thetaj-thetai)/L + V0 L/2
f=[-V0,-M0,+V0,M0-V0L].
```

Rigid motion `d=[t,omega,t+L omega,omega]` gives zero strain/action. Pure curvature `d=[0,0,cL²/2,cL]` gives V0=0, moment EI*c and energy EI*c²*L/2 regardless of Ks. A rigid rotation of all reference geometry, frames, loads and supports preserves energies and transforms force/moment/displacement vectors. The kinematics are infinitesimal, not a finite-rotation or stress-stiffness formulation.

## 4. Loads, recovery and field reconstruction are part of the change

Changing only `local_stiffness` is insufficient. Derive the exact load part from the same statics/compatibility relation. For distributed transverse load w(x) and distributed couple c(x), in the sign convention above:

```
V'=-w; M'=-V-c; theta'=M/EI; v'=theta+V/Ks.
V(x)=V0-integral_0^x w(s)ds
M(x)=M0-integral_0^x [V(s)+c(s)]ds
theta(x)=thetai+integral_0^x M(s)/EI ds
v(x)=vi+integral_0^x [theta(s)+V(s)/Ks]ds.
```

Solve the two linear compatibility equations v(L)=vj and theta(L)=thetaj for V0,M0. For piecewise uniform loads, integrate the polynomials exactly on each span. Point force/couple events produce their signed jumps; define one-sided station values and avoid counting endpoint point loads both as nodal and interior loads. For unsupported load shapes, block explicitly instead of using stale EB equivalents.

The endpoint action is `f_e=[-V(0),-M(0),V(L),M(L)] = Kd-feq`; compute `feq=-f_e(d=0,loads)`. The source equivalent used for global assembly must also be used in end-force recovery. Recover section V/M by equilibrium, and v/theta by the same integrated solution including the load particular part. Do not reconstruct Timoshenko displacement using EB Hermite interpolation alone. Pressure/thermal axial eigenloads stay under their existing separate ownership; shear deformation does not alter their pressure convention or justify new pA terms.

Full-span uniform w happens to retain `feq=[wL/2,wL²/12,wL/2,-wL²/12]` for this prismatic symmetric model. That coincidence does not validate using EB formulas for partial loads. For uniform w over the distal half `[L/2,L]`, exact equivalent values are:

```
feq = [wL(3+4phi)/(32(1+phi)),
       wL²(5+8phi)/(192(1+phi)),
       wL(13+12phi)/(32(1+phi)),
      -wL²(11+8phi)/(192(1+phi))].
```

Both this vector and the wrong EB vector can balance total force/moment; displacement, rotation and compatibility checks must distinguish them. Straight normal-stress extrema still follow signed equilibrium resultants, with the M14 resultant objective; this change must not revert that contract or intensify shear/axial/torsion using a fitting flexibility factor.

## 5. Frozen reference cases and negative controls

The independent return supplies the coefficient stress-energy derivation and executable reference arithmetic. Freeze those expectations before product changes; a test computing its expected answer through the new production element is not independent evidence.

**S-A, rational element controls.** At EI=1 N*m², Ks=12 N, L=1 m, phi=1 (an abstract element fixture, not a pipe material):

| Load on a fixed-root cantilever | Tip v, m | Tip theta, rad | Additional check |
|---|---:|---:|---|
| +1 N tip force | 5/12 | 1/2 | tip slope=7/12; root Fy=-1 N, Mz=-1 N*m |
| +1 N*m tip couple | 1/2 | 1 | V=0 and response independent of Ks |
| +1 N/m over full span | 1/6 | 1/6 | root Fy=-1 N, Mz=-1/2 N*m |
| +1 N/m over distal half | 53/384 | 7/48 | root Fy=-1/2 N, Mz=-3/8 N*m; feq=[7/64,13/384,25/64,-19/384] |

The distal-half EB vector `[3/32,5/192,13/32,-11/192]` is a required killed mutation: used with the selected Timoshenko stiffness it gives wrong tip v=9/64=54/384 m while retaining theta=7/48 rad. Thus checking rotation and global force/moment balance alone misses it. Also kill theta=v' (wrong tip rotation), applying kappa to GJ, swapping the two bending-plane signs and replacing metal area with Ashear in EA.

**S-B, physical pipe companions.** Use ro=.1 m, ri=.09 m, E=200 GPa, nu=.3, G=1000/13 GPa, A=.0019pi m², I=.0000085975pi m⁴, J=2I. The selected coefficient is `kappa_E=33.219654/66.137512`; kappa_C is `1277679/2397284` and must not produce the selected expected values. At F=100 N and L=1,2,10 m compare:

```
v_tip=100 L³/(3 E I)+100 L/Ks
theta_tip=100 L²/(2 E I)
gamma_tip=100/Ks
root reaction [-100 N, -100L N*m].
```

These cover a measurable shear contribution and a slender companion without asserting that L/D alone establishes 3D accuracy. Keep force/rotation small, retain the beam-model/end-effect qualification, and repeat with reversed load and a spatially rotated model. The short member is a correct 1D model oracle, not a claimed exact shell/solid specimen. An independent 3D comparison, if later required, must use compatible load introduction, warping and observed-displacement definitions.

**S-C, coefficient and dimensional controls.** Verify analytic `m→0`, `m→1`, nu=0 and at least two nonzero nu values. Use a genuinely hollow ratio such as .9 to reject a solid-circle substitution. Compare the closed coefficient with independent integration of the Saint-Venant shear stress square. Normalize coherent m/mm, N/kN, Pa/MPa inputs before deriving geometry/material/Ks; same physical model must give same result. For the S-B force-loaded cantilever with zero prescribed motion and no other elastic restraint, doubling E at fixed nu doubles EA/EI/GJ/Ks, halves displacement and leaves reactions unchanged. A general such scaling oracle requires every contributing stiffness to scale together. In S-E, doubling only beam E while holding the spring k fixed changes relative stiffness and therefore spring/root reactions; retain that as a counterexample to universal reaction invariance. Changing nu must affect both G and the selected coefficient coherently.

**S-D, locking, subdivision and null modes.** For fixed E/I/L, increase Ks so phi approaches zero; the exact matrix and responses approach EB smoothly, never a spurious infinitely stiff transverse response. Mesh the same homogeneous member into 1,2,4,8 exact-static elements with consistent applied loads; retained-node displacements, reactions and resultants must agree within the governing arithmetic comparison. Do not change the physical applicability label merely because the elements become shorter. Recover exactly six free-body rigid modes and no extra zero-energy mode; verify the pure-curvature patch and energy/virtual-work identities. Existing numerical policy owns rounding/rank tolerances—this design introduces none.

**S-E, indeterminate response and field observations.** Verify fixed-guided lateral translation delta with end rotations zero: force `12EI*delta/[L³(1+phi)]`, paired end moments from the exact matrix, and their whole-body balance. Unlike a statically determinate cantilever, an indeterminate model can redistribute moments/reactions when Ks changes. A concrete two-element system is a fixed-root cantilever of length L with a linear ground spring k at interior node a and transverse tip force F. Independent flexibility/compatibility gives:

```
c_aL=a²(3L-a)/(6EI)+a/Ks
c_aa=a³/(3EI)+a/Ks; c_LL=L³/(3EI)+L/Ks
u(a)=F c_aL/(1+k c_aa); spring reaction R=-k u(a)
u(L)=F c_LL+R c_aL
theta(L)=(F L²+R a²)/(2EI)
root force=-(F+R); root moment=-(F L+R a).
```

At EI=1, Ks=12, L=1, a=1/2, F=1 and k=12 in consistent SI units: `R=-7/8 N`, `u(a)=7/96 m`, `u(L)=37/128 m`, `theta(L)=25/64 rad`, root force `-1/8 N` and moment `-9/16 N*m`. The corresponding EB model gives `R=-5/6 N`, `u(L)=71/288 m`, `theta(L)=19/48 rad`, root force `-1/6 N`, root moment `-7/12 N*m`. These large normalized motions are an algebraic fixture, not a physical small-motion specimen. Use the S-B section with L=2 m, a=1 m, k=1000000 N/m and F=100 N for a small-motion companion evaluated from the same exact formulas: approximately R=-14.8775370812 N, u(a)=1.48775370812e-5 m, u(L)=4.78723378122e-5 m, theta(L)=3.56464923927e-5 rad, root force=-85.1224629188 N and root moment=-185.122462919 N*m. Full independently computed values, not these presentation decimals, are retained in the reference arithmetic. This tests actual assembly/support attribution and load redistribution. Inspect interior slope, section rotation, shear strain and load-boundary jumps, not only endpoint displacements.

## 6. Interface, ownership and implementation sequence

The implementing owner should use one explicit per-element beam-theory discriminator, with initial supported meanings `euler_bernoulli` and `timoshenko_annulus_energy_v1`. The latter derives Ks from the qualified isotropic annulus and material basis. Unknown/missing required basis blocks a requested Timoshenko solve; no default rectangle factor or silently inferred nu. Retain historical EB results/inputs, disclose their model approximation, and record an explicit theory change as a model change invalidating Current results. No version is allocated here; ROOT integrates the namespace through the current shared contract.

A named generic user-supplied shear-rigidity or computed-section matrix is a possible later explicit mode, with its own source/normalization. It is not an undocumented override of the selected coefficient. Curved elements, joints, branches, rigid components and multi-material/noncircular sections cannot silently consume this annulus law. Mixed-model results must show each actual element basis; a model containing EB curved members is not wholly shear-deformable. Pressure-stiffened or nonlinear tangents require their existing owning formulations.

| Source owner/seam | Necessary connected change |
|---|---|
| `core/solver/frame_kernel/src/lib.rs`: FrameSection/FrameProperties, `local_stiffness`, `FrameElement` transforms/assembly | Add an explicit theory/rigidity path or equivalent new typed constructor; preserve EB API behavior. Use exact static bending blocks with correct plane signs; axial/torsion remain unchanged. Rebase against the active numerical branch rather than editing shared files concurrently. |
| `core/solver/straight_pipe/src/lib.rs`: `StraightPipeSectionProperties`, `frame_element`, `equivalent_nodal_loads_with_spans`, `add_spanned_uniform_equivalent_load`, `add_point_equivalent_load`, end and station recovery | Carry theory through stiffness, full/partial/point-load equivalents and displacement/rotation reconstruction. Preserve existing correct cut/action signs. Avoid old Hermite-only load/recovery assumptions in the new path. |
| `core/product_physics/src/lib.rs` / `validation.rs`: section/material derivation, `build_model`, `add_uniform_element_loads`, `straight_local_uniform_loads`, `corrected_local_forces_for_axial_effects`, station/summary publication | Resolve actual per-case E/nu/geometry basis before constructing the element; assemble the selected lane in dense/sparse consistently. Keep theory, kappa, Ks, section/material and load provenance. Validate unsupported/mixed compositions truthfully. |
| Desktop types/typed operations/material/pipe properties, result interpretation; native/headless/persistence/export consumers | Author/display theory and basis, preserve full precision and data hashes, invalidate on edits, and round-trip saved/reopened models/results. Section rotation and slope must never share a misleading label. New parameter/evidence result kinds require the existing semantic-registry process. |

First freeze/refute the section and exact-element references. Then implement the primitive and complete load/recovery path in one coherent bounded slice; only afterward enable authored product consumption with its consumer/source evidence. Diagnostic disclosure for existing EB can land independently where it is factual, but it does not close the missing shear capability. Validate actual native review/apply/solve/results/save/reopen and headless/core path agreement; browser mechanics fixtures cannot stand in for native solver evidence. ROOT owns build/CI/full-diff review and shared-resource leases.
