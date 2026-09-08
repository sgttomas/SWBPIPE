# P5-A2-CHECK return

Run status: SUCCESS. Actual identity: `/root/pressure_reference/p5_pressure_check`. Configuration: `gpt-5.6-sol`, reasoning `high`, `fork_turns=none`. Agent 2 nondelegation was instruction/config asserted; the live inventory showed this instance with no descendants. The repository was read only and HEAD was `779dedb8670625b36af07b89fc5557470e47c50e`.

## Verdict

Retain manager Option C. The current implementation has coherent effective-force bookkeeping for a limited closed-end axial-bar model, but it is not a full pressure/thermoelastic reference. The earlier Option A is suitable only as an interim repair that reconstructs wall force once as `Nw=S+piAi-peAe`, preserves current displacement behavior, and discloses omitted Poisson coupling and topology limits.

## Independent result frozen before current-source comparison

With local `x:i->j`, define wall force `Nw` positive in tension and effective force `S=Nw+peAe-piAi`. A positive tensile section force has member endpoint actions `[-Nw,+Nw]`; recover the common cut scalar as `Ncut_i=-Fi_x`, `Ncut_j=+Fj_x`.

For an internal-pressure-only homogeneous isotropic Lamé cylinder, `P=pAi`, `A=P/As`, and generalized axial strain gives

```text
Nw = E As(epsilon_z-alpha DeltaT) + 2 nu P
S  = Nw-P
```

The four cases are:

- free closed: `Nw=P`, `S=0`, `epsilon_z=(1-2nu)P/(EAs)+alpha DeltaT`;
- restrained closed: `Nw=2nuP-EAs alpha DeltaT`, `S=-(1-2nu)P-EAs alpha DeltaT`, with complete-vessel supports acting inward by `-S` at each end;
- pressurized free barrel with remote/separately supported closures or modeled open-end compensation: `Nw=0`, `S=-P`, `epsilon_z=-2nuP/(EAs)+alpha DeltaT`, while each closure support carries `P`;
- thermal plus pressure: superpose the thermal terms above within the stated linear model.

Closure topology changes `Nw` and the nodal/support path; it does not erase the pressure term from `S` at a pressurized cut. A physical opening to ambient instead has local `p`, `P`, and `S` tending to zero.

For `Do=.168 m`, `t=.007 m`, `L=2 m`, `E=200 GPa`, `p=1 MPa`, candidate `nu=.3`: `P=18.626503 kN`. Free closed gives `Nw=18.626503 kN`, `S=0`, `sigma_z=5.260870 MPa`, and `DeltaL=.021043 mm`. Restrained closed gives `Nw=11.175902 kN`, `S=-7.450601 kN`, `sigma_z=3.156522 MPa`. Remote closure/free barrel gives `Nw=0`, `S=-18.626503 kN`, and `DeltaL=-.031565 mm`. Exact Lamé hoop stress is `11.521739 MPa` inner and `10.521739 MPa` outer. The mean-radius cap proxy is `20.358306 kN`, 9.2975% above bore thrust.

## Current comparison and remaining choices

Static inspection found the current straight-pipe pressure pair `[-P,+P]`, correction to effective-force-like actions, raw endpoint versus station sign inconsistency, suppression of longitudinal pressure stress when thrust is active, and no wired Poisson or closure-topology input. Existing `E,G` imply `nu=E/(2G)-1` only if an Owner accepts a homogeneous-isotropic compatibility and precedence rule.

Owner rulings remain on the exact versus named thin-wall reference; explicit versus contract-derived Poisson ratio; closure topology/default compensation; typed public wall/effective/cut/action outputs and migration; stress output level and approximation limits; curved/expansion-joint extension; and verification tolerances.

Primary checks used MIT OpenCourseWare pressure-vessel and structural-mechanics publications plus SIMULIA Abaqus documentation for the effective-force and open-end conventions. Full URLs and applicability are preserved in the manager evidence `SOURCES.json`.
