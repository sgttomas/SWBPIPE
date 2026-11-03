# RT-A3 reducer 2a analytical references

These invented references cover a collinear concentric reducer represented by
one mapped body member in its own pressure region, joined to adjacent regions
at implicit wall-transferring junctions. Governing source is `2656dcb33328`:
DEL-04-07 `Work/route-unification-r4.md` §00 C-3/C-4 and §3.1;
`Work/pressure-contract.md`, “Tees and pressure junctions”; DEL-03-05
`Work/rigid-component-stiffness-scaling.md` §§2.1/2.4/2.7; and DEL-05-03
`Work/section-bases-and-timoshenko.md` §§1/2.1.

Status: frozen analytical candidate. Product agreement and independent scrutiny
are separate checks. Self-consistency does not establish product conformance,
acceptance, release or eccentric reducer readiness. RS-R1 is eccentric and
remains outside this reference set and pending increment 2b.

## Independent basis

The standard-library generator imports, reads and runs no product code, output,
standard, catalog or vendor input. All input numbers are invented. It derives
axial responses by two exact formulations: series compatibility/effective-force
statics written left-to-right, and a separately assembled oriented axial
stiffness system eliminated over rational numbers. They agree exactly on the
ten intended-admission cases. The omitted-cap mutation control uses the second
formulation and independent equilibrium checks. There are 373 exact checks,
including nodal/global equilibrium, constitutive response, pressure linearity,
body scale effects and closure resultants.

Every area/force is a rational coefficient of π; displacements and stresses
are rational. Machin's identity and exact alternating-series remainders enclose
π with rational endpoints. Binary64 values are emitted only when both interval
endpoints round identically. Decimal arithmetic supplies display text only.

The three-member model has a large upstream section, a separately mapped body,
and a small downstream section. Stiffness is `s E A_nominal/L`; the Poisson
pair remains `2νpA_i`, with the reduced pressure bore used once. Wall force is
`N_wall = N_mechanical + 2νpA_i`. Each nominal/reduced axial stress divides that
one wall force by its own metal area; Lamé rows use each basis's own annulus.
The body's rows describe its equivalent reference section, not taper/body
stress. Descriptive end sizes supply no mechanics or bore value.

## Cases and limits

Free capped ends with a junction anchor give exactly zero anchor reaction
(the corrected T-3 boundary condition). Far anchors with free junctions exercise
compatibility and load redistribution, including body scale 2, reversed member
orientation, an intermediate body bore, and signed vacuum pressure. A distinct
all-nodes-held case shows the different support reactions under those complete
boundaries; it does not restore the superseded T-3 anchor claim. The unequal
pressure case is D-1's warned closure. The omitted-cap case is a mutation
control that no admitted product input may silently produce.

No bending, eccentric-offset, varying-bore taper, nonlinear or hydrotest device
law is established. Axial values are shear-model independent on this exact
collinear geometry; transverse translations and rotations are unloaded zero.

## Reproduce

```sh
python3 -I -B generators/rt_a3_reducer_reference.py --check
python3 -I -B generators/rt_a3_reducer_reference.py --summary
shasum -a 256 -c SHA256SUMS
```

`--write` intentionally regenerates `references.json` and `SHA256SUMS`; normal
execution only checks their bytes. A disagreement is investigated against the
independent basis and review, never repaired by matching a product output.
