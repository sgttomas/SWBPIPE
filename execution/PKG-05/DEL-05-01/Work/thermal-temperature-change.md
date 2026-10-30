# Thermal temperature change: typed and condition-derived ΔT

The pressure-free route (PF) takes a pipe's thermal load as a `thermal`
primitive with dimension `temperature_change` (ΔT), read by
`build_thermal_element_loads` (PP `lib.rs` ~11986). Model document 0.9.0 adds
a second source: a load case's `condition_use.temperature` resolves each
pipe's ΔT from its operating-condition set (DEL-02-01 Design
`model-document-0.9.0.md` §1.5-§1.8). This note records how the two routes
relate. Agent 0 decided the policy on 2026-10-27 under the owner's delegation
(#1703, OC-5).

Paths are relative to `projects/chirality-piping/`. PP is
`core/product_physics/src`. Standard claim fence applies (F-PIP-2).

## 1. The two routes

- **Typed ΔT (unchanged).** An authored ΔT keeps today's path: the units
  catalog's interval conversion to kelvin, `value × factor`. The factor is 1
  for K and degC, and fl(5/9) for degF and degR. 0.9.0 does not change this
  route, so no existing result or frozen reference moves.
- **Condition-derived ΔT (new in 0.9.0).** The exact difference of the condition
  temperature and the set's installation temperature, each read as its canonical
  decimal, rounded once to binary64 kelvin (DEL-02-01 Design §1.5.1). The
  lowered case carries it as a `thermal` primitive in K, so the rest of the solve
  is the typed route's.

## 2. Disclosure: the degF/degR cross-route difference

For K and degC inputs typed with up to 15 significant digits, the two routes
agree exactly: a typed decimal ΔT equals the condition route's ΔT for the same
temperatures. For degF and degR they can differ in the last bits, because the
typed route rounds the factor 5/9 before multiplying.

Example: a typed 7 degF ΔT gives 3.8888888888888893 K (`400f1c71c71c71c8`),
while 77 degF − 70 degF through conditions gives 3.888888888888889 K
(`400f1c71c71c71c7`). The two are one ulp apart, and the results that follow
from them can differ by similarly small amounts.

## 3. Remaining item

Make typed degF/degR interval conversion correctly rounded. It is not done now
because it would move existing results; any effect on
`apps/desktop/native-smoke/expected.json` or other frozen references is the
owner's to decide.
