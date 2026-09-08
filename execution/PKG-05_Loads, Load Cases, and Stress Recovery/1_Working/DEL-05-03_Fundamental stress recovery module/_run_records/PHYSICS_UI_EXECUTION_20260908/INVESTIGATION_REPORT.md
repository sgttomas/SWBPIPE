# DEL-05-03 four-case pressure-reference investigation

Status: COMPLETE CANDIDATE INVESTIGATION. This is derivative engineering evidence for Owner vetting. It does not adopt a production model, alter a public result contract, close all pressure physics, or create code-compliance acceptance.

## Decision-ready recommendation

Use the following reference interpretation for the next implementation brief:

1. `Nw` is the physical axial resultant carried by the pipe wall, positive in tension. Fundamental axial wall stress is `sigma_z = Nw / As` exactly once.
2. `S` is a separately named effective axial force. For a section containing internal pressure `pi` and external pressure `pe`, define `S = Nw + pe Ae - pi Ai`; for this internal-only investigation, `S = Nw - P` with `P = p Ai`. Closure/support topology sets the wall-force and nodal-load path; it does not remove the internal-pressure term from `S` while the section remains pressurized.
3. Raw element endpoint actions and section cuts are distinct. For local axis `x` from `i` to `j`, positive wall tension has endpoint action pair `[-Nw,+Nw]`. Convert to the tension-positive section scalar with `Ncut_i=-Fi_x` and `Ncut_j=+Fj_x`. Station and endpoint stress recovery must consume the same section-cut convention.
4. Use exact circular-annulus geometry and the axisymmetric Lamé/generalized-plane-strain solution as the independent straight-pipe reference oracle. It uses actual pressure area `Ai=pi ri^2` and wall area `As=pi(ro^2-ri^2)`. A runtime thin-wall reduction may be adopted separately, but every such result must identify its reference surface and approximation class.
5. Pressure deformation requires an explicit Poisson/material contract. Prefer an authored dimensionless `poisson_ratio` at the same temperature basis as `E` and `G`. Infer `nu=E/(2G)-1` only under an explicitly selected homogeneous-isotropic compatibility rule; the mere presence of `E` and `G` does not establish that rule.
6. Pressure-boundary topology must state whether end thrust transfers to the pipe wall (`closed_transferred`), bypasses it through separately supported closures (`remote_closure`), or is compensated at an open modeled boundary. It must also identify the pressure/effective area and its provenance. Expansion-joint effective area is a load-path input and must not be reused as wall membrane area.
7. Version or add typed result rows for `wall_axial_force`, `effective_axial_force`, `element_end_action`, and `section_cut`. Do not silently reinterpret the existing generic axial-force/stress IDs.

This recommendation completes the requested reference investigation. Physical adoption, compatibility migration, model validity limits, and curved/EJ extension remain Owner choices listed below. The preparation child’s Option A remains a lawful limited fallback: it can repair wall/effective-force bookkeeping while intentionally preserving the pressure-strain omission. E1 manager-selected Option C is the basis of this report.

## Independent derivation

The fresh Agent 2 checker froze this derivation before reading current source or the accepted E1 pressure packet. The manager independently reproduced the arithmetic with `calculate_reference.py`, which imports no production module. Current-output comparison appears only after this derivation.

Let `ri`, `ro`, `As=pi(ro^2-ri^2)`, `Ai=pi ri^2`, internal pressure `p`, `P=pAi`, axial strain `epsilon_z`, Young’s modulus `E`, Poisson ratio `nu`, coefficient `alpha`, and uniform temperature change `DeltaT`. Tension is positive.

For a homogeneous isotropic linear-elastic circular cylinder away from end discontinuities, let internal/external pressures be `pi,pe`, define `Pc=pi Ai-pe Ae`, and use the Lamé radial and hoop fields

```text
A = (pi ri^2-pe ro^2)/(ro^2-ri^2) = Pc/As
B = (pi-pe) ri^2 ro^2/(ro^2-ri^2)
sigma_r(r)     = A - B/r^2
sigma_theta(r) = A + B/r^2
sigma_r + sigma_theta = 2A
```

Generalized axial strain therefore gives

```text
epsilon_z = [sigma_z - nu(sigma_r+sigma_theta)]/E + alpha DeltaT
sigma_z   = E(epsilon_z-alpha DeltaT) + 2nu A
Nw        = E As(epsilon_z-alpha DeltaT) + 2nu Pc
S         = Nw - Pc = Nw + pe Ae - pi Ai
```

These equations separate three matters that the current result surface blends: wall constitutive force, fluid pressure across a section, and the external closure/support path.

### Four required cases

| Case | Boundary equilibrium | Exact wall/effective result | Axial strain and support path |
|---|---|---|---|
| Free closed pipe | Attached closures transfer `P` to the wall; no external axial reaction | `Nw=P`, `S=0`, `sigma_z=P/As` | `epsilon_z=alpha DeltaT+(1-2nu)P/(EAs)`; no support reaction |
| Axially restrained closed pipe | `epsilon_z=0`; attached closure thrust and supports share end equilibrium | `Nw=2nuP-EAs alpha DeltaT`; `S=-(1-2nu)P-EAs alpha DeltaT` | Each complete-vessel end support acts inward with magnitude `P-Nw=-S`; pressure-only gives `(1-2nu)P` |
| Pressurized free barrel with separately supported closures or modeled open-end compensation | No cap thrust transfers to the wall | `Nw=0`, `sigma_z=0`; for the still-pressurized section `S=-P` | `epsilon_z=alpha DeltaT-2nuP/(EAs)`; remote closure supports each carry `P`, while the free barrel carries no axial support load |
| Thermal plus pressure | Superpose uniform thermal strain within the same linear model | Free closed remains `Nw=P,S=0`; restrained closed follows the second row with the thermal term | Free strain and restrained support load are the algebraic sums shown above |

For a restrained open/free-barrel variant, pipe supports must provide `Nw=2nuP-EAs alpha DeltaT` while separate closure supports independently carry `P`. This is intentionally separate from the required free-barrel remote-closure/open-boundary-compensation case. A physically open terminal venting to ambient has local pressure tending to zero, so `P` and `S` also tend to zero there.

### End and support signs

For local `+x` from `i` to `j`:

| Quantity | End i | End j | Meaning |
|---|---:|---:|---|
| Positive tensile wall-force endpoint action | `-Nw` | `+Nw` | member endpoint vector |
| Tension-positive section cut recovered from endpoint action | `-Fi_x=+Nw` | `+Fj_x=+Nw` | common section scalar |
| Closed-end pressure action on the complete vessel | `-P` | `+P` | fluid-on-closure force |
| Restrained closed support-on-vessel action | `+(P-Nw)` | `-(P-Nw)` | inward actions; vector sum with pressure and wall transfer closes at each end |

If a product reports node-on-element rather than element-on-node action, every endpoint vector sign reverses. A result schema must state which one it publishes. A scalar support magnitude is insufficient evidence for directional equilibrium.

## Numerical worked reference

Inputs are the existing audit geometry plus explicit candidate constitutive values: `Do=.168 m`, `t=.007 m`, `L=2 m`, `E=200 GPa`, `p=1 MPa`, candidate `nu=.3`. The mixed illustration additionally uses candidate `alpha=1.2e-5 /K`, `DeltaT=75 K`. These are invented/reference values, not adopted material or acceptance data.

Derived geometry: `ri=.077 m`, `ro=.084 m`, `rm=.0805 m`, `Ai=.0186265028431 m^2`, `As=.00354057492060 m^2`, `P=18.6265028431 kN`, and `t/ri=.0909091`.

| Case | `Nw` kN | `S` kN | `sigma_z` MPa | `epsilon_z` microstrain | axial displacement mm | end support path |
|---|---:|---:|---:|---:|---:|---|
| Free closed, pressure only | +18.626503 | 0 | +5.260870 | +10.521739 | +0.021043 | none |
| Restrained closed, pressure only | +11.175902 | -7.450601 | +3.156522 | 0 | 0 | complete-vessel supports inward 7.450601 kN each |
| Free barrel, open/remote closures | 0 | -18.626503 | 0 | -15.782609 | -0.031565 | pipe none; each remote closure support carries 18.626503 kN |
| Free closed, pressure + heat | +18.626503 | 0 | +5.260870 | +910.521739 | +1.821043 | none |
| Restrained closed, pressure + heat | -626.127584 | -644.754087 | -176.843478 | 0 | 0 | complete-vessel supports inward 644.754087 kN each |

The mixed numeric row is an arithmetic oracle for the stated linear model. It does not establish material linearity, allowable stress, or fitness at that stress.

## Cap-area, thin-wall, and thick-wall comparison

The exact closed-end axial mean stress is cap equilibrium over actual metal area: `P/As=5.260869565 MPa`. It is uniform in the ideal Lamé cylinder. The exact hoop field varies through the wall: `11.521739130 MPa` at `ri` and `10.521739130 MPa` at `ro`; its diametral-ligament average is `11.000000000 MPa`. A mean-radius cap proxy `p pi rm^2=20.358306 kN` exceeds the physical bore thrust `p pi ri^2=18.626503 kN` by `9.2975%` and is not interchangeable with it.

| Candidate convention | Longitudinal MPa | Hoop MPa | Relation to exact reference |
|---|---:|---:|---|
| Exact annulus/Lamé | 5.260870 | inner 11.521739; outer 10.521739 | reference oracle for stated assumptions |
| Thin wall at inner radius | 5.500000 | 11.000000 | axial +4.5455% versus exact; hoop equals diametral-ligament average |
| Thin wall at mean radius | 5.750000 | 11.500000 | axial +9.2975% versus exact; hoop -0.1887% versus exact inner-surface value |

The mean-radius formulas mix an approximate enclosed area with an exact annulus area if their longitudinal result is combined with `P=pAi`. The closeness of one hoop value does not cure that axial mismatch. No suitability threshold or acceptance tolerance is adopted here.

## Frozen expectations before current comparison

Proposed exact test expectations for a future authorized implementation are:

- pressure-only free closed: `Nw=P`, `S=0`, zero support reaction, and displacement `(1-2nu)PL/(EAs)`;
- pressure-only restrained closed: `Nw=2nuP`, `S=-(1-2nu)P`, zero displacement, and opposing support vectors of magnitude `(1-2nu)P`;
- pressure-only open/remote closure with free barrel: `Nw=0`, `S=-P`, displacement `-2nuPL/(EAs)`, zero pipe support reaction, and separate closure reactions `P`;
- mixed free and restrained rows exactly as the equations and numeric table above;
- `p=0`, `nu=0`, and `DeltaT=0` reductions; signed pressure/temperature scaling within the adopted linear range;
- reversed member orientation: endpoint vectors swap/sign-transform while `Ncut`, `Nw`, and physical support equilibrium remain invariant;
- two collinear pressurized segments: internal virtual end-load cancellation with continuous wall force and only physical terminal boundary actions;
- endpoint and station stress parity after applying `Ncut_i=-Fi_x`, `Ncut_j=+Fj_x`;
- mutation checks that fail if `P` is omitted from wall-force recovery, added twice to axial stress, or omitted from effective force;
- exact inner/outer/average thick-wall stress outputs or explicitly labelled thin-wall outputs, with no tolerance accepted until the model/validity decision is made;
- curved-bend and expansion-joint cases held to their separately accepted geometry/load-path models rather than assuming the straight Lamé reduction applies unchanged.

The historical TP-PHYS-008 case remains a limited axial-bar witness. It lacks explicit `nu` and circular wall geometry, so its `Fthermal+P=12 N` fixed/fixed expectation must not be relabelled as a full constitutive oracle. If its `E=1000 Pa` and `G=400 Pa` were later accepted as homogeneous isotropic data, they would imply `nu=.25`; that inference is not made by the current contract.

## Current implementation comparison

No relevant source bytes changed between E1 basis `35249acc...` and current source `779dedb...` for `core/product_physics`, `stress_recovery`, or `straight_pipe`. Static inspection therefore confirms the prepared mismatch remains current without executing production to derive the reference:

- `build_pressure_thrust_loads` uses `pAi` for ordinary pipe and a separate authored effective area for mapped expansion joints; `add_pressure_thrust_loads` applies the straight pair `[-P,+P]`.
- `corrected_local_forces_for_axial_effects` removes both the thermal and pressure equivalent pairs from stiffness recovery, producing the effective-force-like straight result. The product does not add the `2nuP` constitutive term.
- Fixed/fixed straight pressure therefore yields raw endpoint rows `[+P,-P]`. The current station path converts the i-end action to a j-side section convention, so it yields constant `-P`, while endpoint stress recovery passes the raw endpoint components directly and yields opposite signed stresses at the two ends. This is a sign/reference mismatch, not a physical wall-stress field.
- Pressure longitudinal output is suppressed whenever pressure thrust is active. The generic stress crate can compute only the thin mean-radius values `p rm/t` and `p rm/(2t)`; enabling the longitudinal row would produce `5.75 MPa`, not the exact cap-area wall mean `5.260869565 MPa`, and can double-count when axial wall force is also recovered.
- Current `MaterialInput` and its temperature points contain `E`, `G`, and optional `alpha`, but no `nu`; no field declares homogeneous isotropy or E/G/nu precedence. The PRD says material records shall support Poisson ratio, so the missing runtime field is an implementation/schema gap, not evidence that `nu` may be silently inferred.
- Current primitive pressure loads target an element but do not declare open/closed/remote closure topology. The current closed-end pair can cancel at ordinary internal nodes, but it cannot distinguish physical terminal cap transfer from open-end compensation or separately supported closures.
- The curved-bend macro recovers the correct limited-model wall membrane force `+P` by cap/wall equilibrium, but its free extension remains `PL/(EAs)` and therefore omits the same Poisson coupling. Its present wall-force meaning also differs from the straight effective-force-like recovery.

Historical `pressure_dense.json` and `pressure_fixed_dense.json` are retained only as evidence of prior endpoint output: free gives zero axial rows and `0.052609 mm`, while fixed gives `[+P,-P]` endpoint rows and `[+5.260870,-5.260870] MPa` endpoint axial stresses. Their interpolated zero station values predate the current section-equilibrium path and are not claimed as current behavior.

## Choices still requiring Owner vetting

1. **Physical model:** adopt the exact annulus/generalized-plane-strain straight-pipe reduction recommended here; adopt a named thin-wall reduction; or explicitly retain limited Option A as an interim compatibility repair. The first is recommended for the independent oracle because the geometry is already available and `t/ri=.0909` makes radial stress material to axial strain.
2. **Poisson data:** add explicit temperature-aware `poisson_ratio` as authoritative (recommended), or derive it from E/G only when an explicit homogeneous-isotropic contract and consistency/precedence rule are present. Any mismatch tolerance remains unset.
3. **Pressure boundary:** choose required closure-transfer/open/remote-closure fields, defaults for legacy models, terminal compensation semantics, and ownership across multi-element pressure regions.
4. **Public compatibility:** add versioned typed rows while preserving legacy rows (recommended), or break/reinterpret existing generic axial force/stress IDs. Existing endpoint action signs cannot be silently relabelled as wall stress.
5. **Stress output level:** choose exact inner/outer/radial plus mean outputs, or one explicitly labelled thin-wall membrane value; choose the validity indicator and any approximation threshold.
6. **Curved and special components:** decide whether the straight generalized eigenstrain is applied to bends as a local membrane correction after separate proof, and define expansion-joint effective-area versus pipe-wall-area behavior. No blanket extension is supported here.
7. **Verification policy:** approve exact arithmetic expectations, publication fixtures, and tolerances in the owning PKG09/Owner process. The numbers here are candidate expectations, not release acceptance.

## Closure

The four requested cases, free bodies, constitutive equations, signs, exact/thin geometry comparison, numerical examples, proposed tests, current-interface gaps, and decision forks are complete. Primary sources were checked directly and are recorded in `SOURCES.json`; they support the candidate formulation but do not become accepted project physics automatically. No source, test, threshold, public output, register, status, DAG, pointer, or authority surface was changed.
