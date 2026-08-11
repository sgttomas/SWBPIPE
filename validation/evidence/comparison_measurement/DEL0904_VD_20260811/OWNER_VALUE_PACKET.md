# V-D comparison measurement and owner value packet

**Status:** `OWNER-READY / NON-AUTHORITATIVE / NUMERIC RULING REQUIRED`

**Basis:** commit `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`; owner-selected V-D; immutable
R14 stress/nonlinear outputs and DEC-053 sparse observation; current exact
mechanics capture in this directory.

This packet selects no public comparison number. It makes no policy,
promotion, runner repair, case edit, register, lifecycle, reliance, release,
or Git change.

## 1. Current 25-fixture capture

The exact current mechanics run requested all 25 inventory fixtures and
returned:

| requested | matched | mismatched | fail-closed blocked | recorded values | observed | unobserved |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 25 | 11 | 0 | 14 | 206 | 91 | 115 |

The runner exit was `1` because blocked cases fail closed. The enclosing
local-private wrapper itself was not blocked. The raw runner output SHA-256 is
`e41f8545cf6fc0603cb41a0bdc08d90726f59ce57e13ad6809036589db3ded8f`.

The historic July-20 census remains exactly 13 blocks / 109 unobserved values.
The proved fourteenth current block is
`MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION`, with six
unobserved recorded values: one `second_moment_area` (`m^4`), two `stress`
(`Pa`), and three `rotation` (`rad`). Its public oracle exists, but the
headless benchmark binding has no value-addressable arm for it. This is an
implementation/binding block, not a data, tolerance, or fixture block.

All 14 current blocks have the same primary classification:
implementation/binding. No tolerance choice can unblock any of them.

## 2. Per-kind measurement

`measured zero floor` is the maximum absolute delta among exact-zero recorded
references. `nearest nonzero` is the smallest observed nonzero reference and
its delta; it is context, not a scale sweep or positive-floor measurement.
Stress rows come from the immutable R14 committed 3-case/11-value output.

| kind / unit | observed | max relative delta at nonzero reference | exact-zero rows | measured zero floor | nearest nonzero reference; delta |
| --- | ---: | ---: | ---: | ---: | --- |
| discrete count / `count` | 5 | `0` | 1 | `0 count` | `1`; `0` |
| ratio / `ratio` | 4 | `0` | 0 | `TBD` | `0.25`; `0` |
| displacement / `m` | 6 | `1.2390881971262908e-16` | 1 | `0 m` | `0.014 m`; `1.734723475976807e-18 m` |
| force / `N` | 42 | `4.440892098500626e-16` | 2 | `0 N` | `2 N`; `0 N` |
| force per length / `N/m` | 0 | `TBD` | 0 | `TBD` | `TBD` |
| length / `m` | 7 | `0` | 0 | `TBD` | `0.004745144915236896 m`; `0 m` |
| linear stiffness / `N/m` | 2 | `0` | 0 | `TBD` | `600 N/m`; `0 N/m` |
| mass per length / `kg/m` | 0 | `TBD` | 0 | `TBD` | `TBD` |
| moment / `N-m` | 20 | `1.7763568394002505e-15` | 2 | `1.7763568394002505e-15 N-m` | `1 N-m`; `1.7763568394002505e-15 N-m` |
| rotation / `rad` | 5 | `2.0016040107424698e-16` | 0 | `TBD` | `0.004333333333333333 rad`; `8.673617379884035e-19 rad` |
| second moment area / `m^4` | 0 | `TBD` | 0 | `TBD` | `TBD` |
| stress / `Pa` | 11 | `0` | 1 | `0 Pa` | `1 Pa`; `0 Pa` |

The current mechanics observations are numerically unchanged from the
historic 91 values. The new DEC-092 fixture expands the unobserved population;
it does not add a measured comparison. Nonlinear regression remains five
exact count/categorical projections. Sparse remains the immutable nine-row
aggregate record discussed in the design companion.

## 3. Numeric options

The comparison predicate in every option is the proposed inclusive DEC-026
pair/floor form:

```text
abs(observed - reference) <= max(atol_kind, rtol_kind * abs(reference))
```

### VD-N1 — exact measured boundary, incomplete

Use each measured maximum relative delta and exact-zero floor shown above.
Leave every absent zero-floor measurement and every wholly unobserved kind
`TBD`. This admits all 102 currently observed mechanics-plus-stress values,
but it is a boundary fit with no headroom and is not a complete release table.

### VD-N2 — measured 10× envelope, incomplete

Multiply nonzero measured members by ten. The resulting positive members are:

| kind | `rtol` | `atol` |
| --- | ---: | ---: |
| displacement | `1.2390881971262908e-15` | `0 m` |
| force | `4.440892098500626e-15` | `0 N` |
| moment | `1.7763568394002505e-14` | `1.7763568394002505e-14 N-m` |
| rotation | `2.0016040107424698e-15` | `TBD rad` |

Measured-zero members remain zero; absent zero measurements and wholly
unobserved kinds remain `TBD`. This also admits all 102 observed values but is
still incomplete and the factor ten is designed headroom, not a measurement.

### VD-N3 — unit-scale projection, complete analytic numbers but inferred

Use a single designed `rtol=2.0e-14` for every continuous kind and
`atol_kind = rtol * S_kind`, where `S_kind` is the smallest nonzero recorded
reference in the current 25-fixture mechanics inventory plus the committed
three-case stress output. Keep counts exact.

| kind | `atol_kind` |
| --- | ---: |
| ratio | `1.8e-17 ratio` |
| displacement | `2.1617230691217002e-18 m` |
| force | `4.0e-14 N` |
| force per length | `2.5e-14 N/m` |
| length | `9.490289830473791e-17 m` |
| linear stiffness | `5.0e-12 N/m` |
| mass per length | `1.2863398382279059e-12 kg/m` |
| moment | `2.0e-14 N-m` |
| rotation | `8.666666666666666e-17 rad` |
| second moment area | `2.10800867055875e-19 m^4` |
| stress | `2.0e-14 Pa` |

`2.0e-14` is just over eleven times the largest observed continuous relative
delta. This table admits all 102 observed values, but values for blocked-only
kinds and most absolute floors are design inferences, not measurements.
Current sparse admission cannot be proved because its stored absolute maxima
mix units.

### VD-N4 — retain the numeric hold and authorize evidence completion

Select no public number yet. Preserve VD-N1 through VD-N3 as
non-authoritative options and separately authorize the missing runner
observation mappings, near-zero scale probes, and kind-separated sparse raw
metrics. Return with a new capture before numeric promotion.

## 4. Recommendation

**Recommend VD-N4.** V-D proved the current population, but did not cure the
reason V-D was needed: three analytic kinds still have no observed counterpart,
seven kinds lack an exact-zero observation, and the sparse absolute metrics
cannot be unit-separated from committed evidence. VD-N3 is the only complete
analytic numeric table, but its completeness comes from design inference
rather than measurement. Promoting it now would erase the distinction the
owner explicitly required V-D to establish.

If the owner prefers a bounded partial policy rather than another evidence
tranche, VD-N2 is the best measured option, with its `TBD`s preserved and no
claim that it is the final complete release table.

## 5. Capture-side cleanup gate

The first Cargo invocation created ignored
`projects/chirality-piping/core/runner/headless/Cargo.lock`, a regular
non-symlink file of 10,114 bytes with SHA-256
`7a3bd7e0df41a07e5c503aa312734e95fa6625afcd8b12f1f7994bd7a75b2e66`.
It was absent at application start. It is preserved untouched because this
Agent 2 has no deletion authority. Terminal zero-ignored-drift requires a
separate exact cleanup gate; this does not change the scientific capture.
