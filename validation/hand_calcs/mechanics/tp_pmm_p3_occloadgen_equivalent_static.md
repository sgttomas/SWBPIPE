# MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC

## Purpose

Invented mechanics benchmark for static-equivalent occasional-load
generation (tranche `TP-PMM-P3-OCCLOADGEN-001`, ruling `DEC-068` item 2).
It verifies that:

1. Seismic: user-entered per-global-axis g-factors and a user-entered
   gravity acceleration multiply the model's own computed mass
   distribution (metal + contents + insulation over the mill-tolerance
   effective wall) into uniform distributed load intensities.
2. Wind: user-entered pressure and shape parameters project onto the
   exposed diameter (outside diameter plus twice the insulation
   thickness) of user-marked spans only.
3. The generated primitive loads pass the equivalent-static boundary
   helper and lump 50/50 to span end nodes exactly as authored loads do.

Pure mechanics from user inputs: no code coefficient, exposure category,
importance factor, catalog value, or default is encoded. The gravity
acceleration is an explicit user-entered input in this fixture, not an
embedded physical constant.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, proprietary
  data, or private project data.

## Invented Inputs

All values are invented non-engineering numbers in the fixture-local
`PKG09-FIXTURE-UNITS-EXPLICIT-SI` basis.

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Outside diameter, `D_o` | 0.2 | m | length |
| Nominal wall thickness, `t_nom` | 0.01 | m | length |
| Mill tolerance, `m` | 0.00125 | m | length |
| Material density, `rho_m` | 7850.0 | kg/m^3 | density |
| Contents density, `rho_c` | 800.0 | kg/m^3 | density |
| Insulation thickness, `t_ins` | 0.025 | m | length |
| Insulation density, `rho_i` | 120.0 | kg/m^3 | density |
| Gravity acceleration (user-entered), `g` | 9.80665 | m/s^2 | acceleration |
| Seismic g-factor, global `X` | 0.3 | ratio | dimensionless |
| Seismic g-factor, global `Z` | -0.2 | ratio | dimensionless |
| Wind pressure, `p` | 480.0 | Pa | pressure |
| Wind shape factor, `C` | 0.7 | ratio | dimensionless |
| Wind direction | global `Y` | - | - |
| Span length, `L` | 3.0 | m | length |

## Computed Mass Distribution

```text
t_eff = t_nom - m = 0.00875 m          (mill-tolerance effective wall)
D_i   = D_o - 2 t_eff = 0.1825 m
A_metal    = pi/4 (D_o^2 - D_i^2) = 5.257258956241679e-3 m^2
A_contents = pi/4 D_i^2           = 2.6158667579656257e-2 m^2
D_ins = D_o + 2 t_ins = 0.25 m
A_ins = pi/4 (D_ins^2 - D_o^2)    = 1.767145867644258e-2 m^2

m' = A_metal rho_m + A_contents rho_c + A_ins rho_i
   = 41.26948280649718 + 20.926934063725005 + 2.1205750411731096
   = 64.31699191139529 kg/m
```

## Expected Generated Intensities

```text
w_seismic,X = m' * 0.3 * 9.80665  =  189.22026861836537 N/m  (global X)
w_seismic,Z = m' * -0.2 * 9.80665 = -126.14684574557693 N/m  (global Z)
w_wind,Y    = p * C * D_ins = 480.0 * 0.7 * 0.25 = 84.0 N/m  (global Y)
```

Wind acts on the user-marked span only; an unmarked span receives no
generated wind load. A seismic case with any pipe lacking a user-entered
material density is a blocking diagnostic, not a defaulted mass.

## Lumped End-Force Cross-Check

50/50 lumping of each uniform intensity over the span (the primitive-load
lumped conversion):

```text
F_end(w_seismic,X) = w_seismic,X * L / 2 = 283.83040292754805 N per end
F_end(w_wind,Y)    = w_wind,Y * L / 2    = 126.0 N per end
```

Cross-checks: the arithmetic above was recomputed independently in decimal
arithmetic from the closed forms; the benchmark fixture recomputes the same
closed forms in code, so agreement is at floating-point identity
(internal assertion epsilon 1.0e-9).

## Tolerance Policy

Per `DEC-024`/`DEC-026`, governed tolerance values remain `TBD` and the
fixture records no fixture-local `tolerance_policy` override; any future
override may only tighten the governed value.

## Boundary

No dynamics (response spectra, time history, modal content — disposition
stays with `D-12`), no code wind/seismic coefficient or catalog, no
default. No lifecycle, release, professional, certification, or
code-compliance claim is made.
