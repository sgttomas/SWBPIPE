# Exact named-input feasibility witness

This is one finite analytical control, not a product or model execution.
Its exact fractions, complete per-row values/errors/predicates and input hash
are in scratch/control_results.json, located and hashed by BULK_MANIFEST.json.
Replay control.py in a **fresh** output directory after copying only that script;
it reads the immutable capture at its pinned absolute location. Do not overwrite
the sealed bulk result. Python standard-library Fraction/integer arithmetic is
the independent checker, not a proposed production arithmetic engine.

## Inputs and exact mechanics

The I50 capture log is 222045 bytes, SHA256
`d762d4dcc137e0bff9e739fb0a829fa086055ea9492f5b55f765ac5801c6dde7`.
Its two records carry the same actual request and source. RV66 independently
decoded their canonical K4 source/ledger and bound these normalized inputs.
The control additionally checks the actual captured dictionary identities,
coordinates, material, loads, springs and OD/wall bits.

- x_i=(0,0,0), x_j=d=(1,2,2), L=3, y_reference=(1,0,0).
- Exact axes are e1=d/3, e2=(4,-1,-1)/(3 sqrt(2)), e3=(0,1,-1)/sqrt(2).
  Dot products establish orthonormality and e1 cross e2=e3.
- D is binary64 `3fc999999999999a`; t is `3f847ae147ae147b`.
  They are not exact decimal 0.2 and 0.01. There is no wall reduction here.
- E=200000000000 Pa and G=80000000000 Pa, independently supplied base values.
- alpha is binary64 `3f73a92a30553261`; the actual tip moments are
  (alpha,2alpha,2alpha)=T e1, where
  T=16602069666338595/1152921504606846976 N*m.
- N0 translations are prescribed zero. Its rotational springs have exact
  stiffnesses (144,1000000,1000000) N*m/rad. No other load law is present.

Define theta0=(alpha/144,2alpha/1000000,2alpha/1000000). For either positive
section law J, the exact state is

    u0 = 0;                   theta_root = theta0;
    u1 = theta0 cross d;       theta_tip = theta0 + T*d/(G*J).

The rigid rotation contributes no member deformation. The added tip rotation is
pure torsion, with twist TL/(GJ), so the member carries T alone. Root spring
moments are -(alpha,2alpha,2alpha), root force is zero. All bending, axial and
transverse-shear actions vanish. These fields satisfy compatibility, beam laws
and equilibrium. With positive axial/bending/torsional coefficients, positive
three rotational springs and the three fixed root translations, zero total
strain energy implies a zero rigid motion; the state is unique. Thus different
positive A/I values do not generate bending in this case. This derivation is not
an implementation recognizer or a fixture-specific production shortcut.

At actual endpoint nodal actions, end_i torsion is -T and end_j is +T. Section-cut
stations use +T. Stress reverses end_i's nodal sign to the j-side convention;
all five torsional-shear stress locations therefore have +T*c/J.

## Source geometry and new actual primitives

For c=D/2, r=c-t, P=t(D-t), Q=c²+r² and g=PQ,

    A=pi*P; I=pi*g/4; J=pi*g/2; Z=pi*g/(4*c).

P=c²-r² and g=c⁴-r⁴ exactly. All denominators are strictly positive.
The control independently computes 128-term even alternating sums for atan(1/5)
and atan(1/239), bounds each next remainder, and uses
pi=16 atan(1/5)-4 atan(1/239). Tangent algebra and the angle in (0,0.8) prove the
identity. The rational interval has width <2^-600 and is rounded outward to a
2^-384 dyadic interval of width 2^-384. The production proposal reuses B1C's
reviewed 512-bit pi endpoints; no runtime series is proposed.

Both rigorous endpoints of each section interval round to the same binary64:

| Prepared property | New bits | Descriptive value |
|---|---|---:|
| A | `3f7872fa3a37ac13` | 0.005969026041820607 m² |
| Iy=Iz=I | `3efc52664442210a` | 2.7009842839238254e-05 m⁴ |
| J | `3f0c52664442210a` | 5.401968567847651e-05 m⁴ |
| Z | `3f31b37feaa954a6` | 0.0002700984283923825 m³ |
| c | `3fb999999999999a` | 0.1 m |

E/G stay unchanged; J_new=2 I_new exactly for these normal values. New operational
ka=RN64(RN64(E*A_new)/3) and kt=RN64(RN64(G*J_new)/3) are recorded with their bits.
Z_new is directly rounded source Z; represented stress still covers both this
actual operational modulus and exact I_new/c. Zero bending makes that distinction
numerically dormant here, not removed from the proposed certificate.

## All final rows and their source targets

For native quantities, enclose both the above K_new solution and the source
solution with real-pi J. Source and K agree exactly except tip rotations and
torsional stress. Norms use integer-square-root bounds at 256 fractional bits;
the checker verifies each squared endpoint inequality exactly.

| Row family | Count | Target and interpretation |
|---|---:|---|
| Nodal component displacements/rotations | 12 | Above global u/theta; three prescribed root translations InputDerived |
| Displacement magnitude | 2 | Norm of each state's translation, independently enclosed |
| Attributed support components | 24 | Three rigid reaction laws, three individual spring-action laws, eighteen empty laws; values from the precise support/axis ownership |
| Support force/moment magnitude | 8 | Norm for its own support group; here zero or the exact absolute single nonzero moment |
| Member endpoint actions | 12 | End_i/end_j signed node-on-element components, only torsion nonzero |
| Member station actions | 18 | Three exact quarter/mid/three-quarter j-side cuts, only torsion nonzero |
| Member section stresses | 20 | Five sites × N/A, My/Z, Mz/Z, T*c/J; cover K_new represented denominators and source annulus |
| Circular normal-stress maximum | 1 | max(|N/A|+hypot(My,Mz)/Z)=0 over the whole member; torsional shear excluded |
| Mode, and actual dense parity presence | 1 / 2 | NonQuantity, actual captured completed producer records; no mechanical scale |

No wall, pressure, intensified, combination or retired-summary row is invented.
There are 97 mechanical rows and 98/99 total rows, matching the actual captures.

For each mechanical SI hull [l,h], the control proposes one deterministic raw
value y=RN64((l+h)/(2a)). Here a=1/1000 for mm, 10^6 for MPa, and 1 for rad/N/N*m/Pa.
It then computes actual normalization n=RN64(y/1000) or RN64(y*10^6), respectively.
Every RN operation is checked against the exact rational midpoints of adjacent
binary64 values, including ties-to-even. No nearest-rounded error is compared.

    H=max(|n-l|,|n-h|); Hraw=max(|y-l/a|,|y-h/a|).

All zero projections are +0. This is not a new structural-zero exemption.

## Coupled final scales, classes and predicates

The control scans every final normalized row, excludes InputDerived and
NonQuantity, and takes exact maxima for translation, rotation, force and moment.
It computes L_b by the accepted binary64 extent operations (here exactly 3), then
tr=max(S_tr,RN(3*S_ro)), ro=max(S_ro,RN(S_tr/3)),
fo=max(S_fo,RN(S_mo/3)), mo=max(S_mo,RN(3*S_fo)), using original operands.
Stress scale is RN(RN(fo/A_new)+RN(k*RN(mo/Z_new))).

| Final kind | Scale bits | Descriptive scale |
|---|---|---:|
| translation | `3f1a378ea78c5ce9` | 0.00010000999635583246 m |
| rotation | `3f017a5f1a5d9346` | 3.333666545194415e-05 rad |
| force | `3f73a92a30553261` | 0.0048 N |
| moment | `3f8d7dbf487fcb92` | 0.0144 N*m |
| component stress k=1 | `404b0f1c3b53ceea` | 54.118049064561845 Pa |
| circular maximum k=2sqrt(2) | `4062f327f1759218` | 151.59862587893508 Pa |

The numerical witness is for p=128/256 selection; it imposes no p512 floor.
If the future new-K solver instead selects p512, actual verification E and floor
bits must be used and all scales/classes/gates recomputed; these listed bits must
not be copied into that run. This is an explicit unexecuted producer premise.

Classification uses |n|<RN64(2^-34*S), with the unchanged S<2^-988 rule. Absolute
rows use RU64(2^-64*S); no named scale is small or zero. Relative rows require
the exact allowance 2^-64 max(|n|,S)(1+2^-21)+2^-53|n|+2^-1074, the separately
executed five-step A1 binary64 allowance, 10^9 H<=|n| and 10^9 Hraw<=|y|.
The 194 mechanical rows across both modes pass all applicable predicates. Each
mode has 69 Absolute, 25 Relative and 3 InputDerived rows. The smallest fractional
sharper margin is 0.38994634518 at root RX. The smallest absolute-row margin is
about 5.42155276596e-24 m at the zero root displacement magnitude.
As a labelled slack control, enlarging every non-input physical interval by
2^-100 in its SI unit still passes all its predicates. This is a finite tolerance
reserve for the proposed certificate, not evidence of an actual residual width.

All eight support magnitude/component comparisons are exact equalities in this
candidate, stronger than their unchanged 64epsilon observable guard. All normal
action polynomial coefficients are +0, so the actual coefficient enclosure can
be [0,0], global upper/gap zero, subdivision zero and midpoint +0, with the
existing coefficient-scope labels. Complete-domain coverage and the maximum's
result/entity/unit aliases are required. The displacement headline aliases N1's
magnitude, raw bits `3fb821e751965334`; the stress headline aliases the zero
circular-normal maximum. No headline claims the maximum torsional stress.

G5a uses actual new-K verification evidence, not a chosen scale. The control
computes its candidate lower/sanity demands using the complete published rows
and actual new operational ka/kt. The new evidence's coupled upper must exceed
53050.36890729408 N and 96.11431291987076 N*m. Its zero-resolution rules, ownership,
stop list and work also remain mandatory. This analysis neither supplies that
live evidence nor fabricates E values. It establishes row/class/scale feasibility;
actual solver/certificate availability and custody remain the next experiment.
