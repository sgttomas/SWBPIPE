# P5 WORKING_ITEMS return

Run status: SUCCESS. DEL-05-03 / SOW-015 / OBJ-003 four-case pressure-reference investigation is complete as candidate evidence. Physical adoption remains Owner-gated.

## Recommendation

Adopt for the next Owner decision the exact circular-annulus, generalized-plane-strain straight-pipe reference described in the deliverable `INVESTIGATION_REPORT.md`: wall force `Nw` is the physical tensile wall resultant; effective force is separately `S=Nw+peAe-piAi`; endpoint actions and section cuts use explicit opposite-end transforms; closure topology controls wall transfer/support loads; and pressure deformation requires an authored constitutive contract. Prefer explicit temperature-aware `poisson_ratio`; derive it from `E,G` only under an accepted homogeneous-isotropic compatibility and precedence rule.

The four cases close as follows for internal pressure `P=pAi`:

- free closed: `Nw=P`, `S=0`;
- restrained closed: `Nw=2nuP-EAs alpha DeltaT`, `S=-(1-2nu)P-EAs alpha DeltaT`;
- pressurized free barrel with separate closures/open-end compensation: `Nw=0`, `S=-P`, with closure supports carrying `P`;
- combined thermal/pressure: the same relations with free thermal strain or `-EAs alpha DeltaT` under restraint.

The existing source has a limited effective-force-like straight recovery, mismatched endpoint/station signs, suppressed longitudinal pressure stress, no Poisson input contract, and no closure topology. The retained Option A can repair force bookkeeping while preserving those physical limits; Option C remains recommended because the stated goal is complete physics modeling.

## Evidence and validation

- Reproducible, production-independent arithmetic exactly matches `REFERENCE_RESULTS.json`.
- Primary MIT and SIMULIA publications were checked directly; URLs, checked sections, and applicability are in `SOURCES.json`. They support candidate formulation and are not project authority.
- One bounded read-only Agent 2 ran as `/root/pressure_reference/p5_pressure_check` with `gpt-5.6-sol`, high reasoning, `fork_turns=none`, no descendants, and returned SUCCESS. Manager validation accepted its result with terminology/material-contract normalization.
- The in-memory child brief preceded execution, but the durable mirror was persisted after the child started. This is recorded honestly and does not establish pre-launch durable evidence.
- SOW validation, source hashes, exact calculation reproduction, E1/current source continuity, JSON, LF, coverage, and write containment passed.

## Remaining Owner rulings

The Owner must choose the exact/thin reference, Poisson source/compatibility rule, pressure-boundary topology/defaults, public result types/migration, stress outputs/validity threshold, curved and expansion-joint treatment, and PKG09 tolerances. No blanket pressure-physics closure is claimed.

Canonical run evidence: `projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module/_run_records/PHYSICS_UI_EXECUTION_20260908/`.
