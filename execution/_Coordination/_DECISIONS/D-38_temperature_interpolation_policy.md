# D-38 - Temperature Interpolation Policy For Material Property Bases

**Status:** PROPOSAL
**Date prepared:** 2026-07-10
**Decision ID:** D-38
**Prepared by:** agent (TP-PMM-P3-MODULUSBASIS-001 tranche worker), at owner
direction via the D-36 ruling ("the schema's open
`temperature_interpolation_policy` question is resolved inside this tranche
or minted as its own `D-XX` if optioned" —
`execution/_Coordination/_DECISIONS/D-36_RULING_2026-07-09.md`)
**Basis:** `DEC-068` (D-36 item 1, per-load-case modulus basis). The tranche
shipped the conservative lawful floor — exact basis selection only — and
mints this row because interpolation is a genuine method fork practitioners
expect, not a detail an agent may decide.

## 1. Decision Statement And Scope

When a load case names a modulus basis (a user-assigned temperature-case
id) and the analysis temperature or basis does not exactly match a stored
user-entered temperature point, may the software derive intermediate
property values (elastic modulus, thermal expansion coefficient) between
the user's stored temperature points — and if so, by what declared method?

Out of scope under any option: shipping any temperature-property curve,
catalog, or code table (all values remain user-entered, F-PIP-1 / PRD
§6.1–6.2); dynamics (D-12); any lifecycle/release/professional/
certification/code-compliance claim.

## 2. Verified Facts (Checked 2026-07-10)

- `schemas/material.schema.yaml` stores user-entered temperature-indexed
  property values (`MaterialPropertyValue.temperature_ref`) and lists
  `temperature_interpolation_policy` as an open-decision topic in its
  `OpenDecision.topic` enum.
- TP-PMM-P3-MODULUSBASIS-001 implemented **exact selection only**: a load
  case's `modulus_basis_ref` must match a stored point id on every material
  used by a pipe segment; an unmatched reference is a blocking diagnostic
  (`MODULUS_BASIS_UNRESOLVED`), and no intermediate value is derived
  (`core/product_physics/src/lib.rs`, `materials_for_modulus_basis`).
- Range results record their per-state basis verbatim
  (`core/loads/stress_recovery`
  `recover_stress_range_with_modulus_basis`; combination
  `modulus_basis_records` in `schemas/model.schema.yaml`).

## 3. Why This Is Human-Gated

Interpolation between user-entered points synthesizes property values the
user never typed. Whether that synthesis is acceptable, what method is
declared (linear in temperature is the common practitioner expectation),
how out-of-range temperatures are treated, and how synthesized values are
provenance-marked all change numerical results and sit on the
"software-derived value" side of the project's no-defaults posture
(K-AUTH-1; D-GOV-04; PRD §6.2).

## 4. Options

### 4.1 O-A — Exact selection only, permanently

The shipped behavior becomes the ruled policy: bases must match stored
points exactly; anything else stays blocking. Users wanting intermediate
temperatures enter intermediate points themselves.

- Pro: zero synthesized values; simplest provenance story; already shipped
  and evidenced.
- Con: workflow friction (users must pre-compute intermediate values by
  hand); diverges from common practitioner-tool expectations.

### 4.2 O-B — Declared linear interpolation between adjacent user points (recommended)

A load case may alternatively name a user-entered solve temperature; the
software linearly interpolates E and α between the two adjacent stored
temperature points, records the two source points and the method string in
provenance on every derived value, and blocks (never extrapolates) when
the temperature falls outside the stored range or when fewer than two
points bracket it. Exact-id selection remains available and unchanged.

- Pro: matches practitioner expectation; linear-in-temperature is a
  declared elementary method, not code content; blocking-not-extrapolating
  preserves the no-defaults fence at the range edges.
- Con: first software-derived material values in the project; needs its
  own witness/benchmark evidence bar and provenance wording review.

### 4.3 O-C — Defer (keep exact selection, leave the topic open)

Keep the shipped exact-selection floor and leave the schema topic open for
a later program phase.

- Pro: no new decision surface now.
- Con: the open decision continues to fire as an unresolved topic; users
  hit the friction in O-A without a recorded permanent disposition.

## 5. Recommended Disposition (PROPOSAL — non-binding)

**O-B**, with the evidence bar: hand-calc witness for an interpolated
hot/cold pair, benchmark fixture per `DEC-024`/`DEC-026` (tighten-only),
provenance strings naming both source points and the method, and blocking
tests at and beyond the stored range edges. Until ruled, exact selection
remains the shipped behavior.

## 6. On-Ruling Mechanism

On ruling, append the `DEC-XX` codification row to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 citing this packet,
update the `_REGISTER.md` D-38 row to RULED with the pointer, and execute
the selected option as a bounded tranche under
`plans/PLAN_2026-07-09_physical_model_mechanics.md` conventions
(branch-first, owner merges). If O-A or O-C is selected, the only code
change is documentation/provenance wording; if O-B, the tranche adds the
declared-interpolation path behind the evidence bar above.

## 7. Human Ruling And Disposition

*(unruled — this section is completed by the human project authority)*
