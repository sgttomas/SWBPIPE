# OWNER DECISION INTERFACE — DEL-09-04 Held Gates

**Status:** `OWNER-READY PREPARATION — NO RULING APPLIED`

**Base:** `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`

**Independent verification:** `PASS`

The preparation supports four independently selectable decisions across
three packets. The recommendation is non-binding: **C-B + V-D**,
**R14 O-B**, and **MR-A**.

## Gate 1 — convergence and public comparison values

The requested “DEC-046 release tolerance values” crosses two accepted axes.
DEC-046 governs nonlinear active-set convergence; DEC-026 governs
result-versus-reference verification. One must not silently authorize the
other.

### Gate C — DEC-046 convergence

| Choice | Values | Evidence consequence |
| --- | --- | --- |
| `C-A` | changed-support threshold `0 count`; caps one-way/gap/lift-off/friction `2`, multi-support `3` | exact observed cap envelope; admits 22/22 accepted assembled observations |
| `C-B` (**recommended**) | threshold `0 count`; cap `4` for all classes | preserves accepted validation values with bounded headroom; admits 22/22 |
| `C-C` | threshold `0 count`; cap `6` for all classes | admits 22/22 but adds unevidenced cap headroom |
| `C-D` | defer/decline | release values remain `TBD` |

### Gate V — separately authorized DEC-026-derived comparison policy

| Choice | Values/basis | Current admissions and limitation |
| --- | --- | --- |
| `V-A` | exact measured per-kind relative maxima and zero-reference absolute floors | mechanics 91/91 and 11/11; stress 11/11 and 3/3; nonlinear 5/5; sparse 9/9; blocked mechanics 0/109 and 0/13; force-per-length and mass-per-length remain `TBD` |
| `V-B` | every nonzero V-A member multiplied by disclosed owner-selected `10×` headroom | same current admissions; same two unmeasured kinds `TBD` |
| `V-C` | proposed `1.0e-9` relative plus unit-bearing `1.0e-9` absolute floors, exact discrete semantics; narrow sparse precedent | same current admissions; projection is an inference from internal precedent, not an existing public table; absolute sparse parity remains ungated |
| `V-D` (**recommended**) | authorize only a current 25-fixture capture, near-zero per-kind floor measurement, and unit-normalized sparse design | no public comparison number is selected; return to owner with fuller basis |
| `V-E` | defer/decline | no public comparison policy selected |

All 13 cases in the immutable July 20 mechanics census are primarily blocked
by missing public runner observation/binding, not data, tolerance, or fixture
quality. Every C/V choice unblocks **zero**. Current source contains 25
fixtures; an additional DEC-092 fail-closed block is likely, but no committed
current-base whole-suite capture proves it.

Exact tables, units, sparse conjunctive criteria, algebra, 13-row census, and
option-by-option admissions are in
`instances/A2_DEC046_REMEDIATION/PACKET_V2.md`.

Recommended exact ruling form:

> **DEC-046 RELEASE-CONVERGENCE VALUES — C-B.** Authorize a new,
> release-scope governed convergence record with
> `{relative_residual_field=0 count, absolute_residual_floor=0 count,
> max_iterations=4}` for one-way, gap, lift-off, friction, and
> multi-support/multi-DOF. The inclusive criterion requires final changed-
> support count zero. This does not select verification-comparison values,
> unblock mechanics cases, or create an acceptance, lifecycle, reliance, or
> release act.
>
> **DEC-026-DERIVED PUBLIC COMPARISON VALUES — V-D.** Authorize only a bounded
> current-25-fixture observable capture, per-kind near-zero floor measurement,
> and unit-normalized sparse comparison design, returning to me before any
> numeric value is ruled. Do not promote V-A, V-B, or V-C now.

The owner may select any other C and V identifiers, amend exact text, or defer
one axis independently.

## Gate 2 — R14 clean-checkout reproduction

All P1-P16 and all 74 indexed bundle checksums verify for
`REPRO_DEL0904_20260720T074714Z_a5235340aae3`. The exact source is
`a5235340aae3c41cf227f5617e593b268936f6b3`; the procedure remains
byte-identical. Runner/dependency, product-physics, mechanics-inventory,
contract-test, workflow-profile, DAG, and toolchain changes fire the recorded
rerun triggers. The historic result remains valid but is not current-base
reproduction evidence.

Options are `O-A` unqualified source-pinned acceptance, `O-B` qualified
source-pinned acceptance (**recommended**), `O-C` defer pending a fresh
current-base bundle, and `O-D` decline.

Recommended exact ruling form:

> **R14 REPRODUCTION RULING — ACCEPT, QUALIFIED TO PINNED R14 SOURCE.** I
> accept the immutable bundle
> `REPRO_DEL0904_20260720T074714Z_a5235340aae3` as adequate
> `INTERNALLY_VERIFIED` evidence that the actor-neutral clean-checkout run at
> source commit `a5235340aae3c41cf227f5617e593b268936f6b3`, under procedure
> SHA-256 `fa714cf44d5c3e8a54ff6e2f6883676b81e01755e2e07d36a5bd118576b299c1`,
> satisfied P1-P16. This acceptance is source-pinned: it does not assert
> reproduction at current base `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`,
> because the recorded rerun triggers have fired. The evidence label remains
> `INTERNALLY_VERIFIED`; a fresh governed bundle is required before any
> current-head reproduction claim. Record this ruling verbatim. No tolerance,
> case-page, GUI-evidence, reliance, lifecycle, stage, issuance, release,
> publication, professional-acceptance, or external-prover effect is
> authorized.

The full bundle/source inventory and non-assertions are in
`instances/A2_R14/PACKET.md`.

## Gate 3 — MAINTAINER_REVIEWED case pages

The exhaustive inventory contains 64 registered/generated/indexed pages: 21
mechanics, 15 stress, and 28 nonlinear. All remain `DRAFT_EVIDENCE`; no
qualifying page/hash-bound maintainer disposition exists. Sixty-three pages
also carry superseded runner language, and the aggregate manual/strategy
statements need a bounded DAG-009/decomposition-0.11 currency correction.
Therefore no lawful promotion slate exists today.

Options:

- `MR-A` (**recommended**): adopt bounded `MR-W1` truthful-candidate and
  owner-review preparation; promote zero pages now.
- `MR-B`: amend the plan with exact changes; promote zero pages now.
- `MR-C`: defer/decline; all 64 remain `DRAFT_EVIDENCE`.

Exact ruling form:

```text
MAINTAINER_REVIEWED BASIS RULING, 2026-08-10: MR-A
```

`MR-A` would authorize a later bounded documentation/witness-preparation
brief, followed by a separate page-specific owner review and a separately
authorized deterministic application. It does not itself promote a page.
The exact 64-row inventory and MR-W1 phases are in
`instances/A2_MAINTAINER_REVIEW_REMEDIATION/PACKET_V2.md`.

## Future integration boundaries

- `TM-PIP-037` remains `DEFERRED`. Its trigger is conjunctive: an owner
  disposition for both the final value gate and the named R14 bundle. Only a
  subsequent TASK_MANAGEMENT session may propose or apply its disposition;
  WORKING_ITEMS must not infer closure from these packets.
- DEL-09-04 Remaining bullet 1 may lose only its `MAINTAINER_REVIEWED`
  residual after actual reviewed-page promotion, not after `MR-A` preparation.
  The GUI-workflow residual remains. R14 acceptance alone removes nothing.
- Remaining bullet 2 may change only to the extent of the exact ruled and
  later applied tolerance values. `V-D`, `V-E`, or any defer leaves the public
  comparison-value residual open; a C ruling alone cannot close Gate V.
- The GUI workflow, export/CAEPIPE cluster, D-61 reliance hold, lifecycle,
  publication/release, professional approval, case repairs, and every other
  status/decomposition/DAG/PRD/claims surface remain unchanged.

## Owner response requested

Return one exact identifier or amended text for each gate:

1. DEC convergence: `C-A`, `C-B`, `C-C`, or `C-D`.
2. DEC comparison: `V-A`, `V-B`, `V-C`, `V-D`, or `V-E`.
3. R14: `O-A`, `O-B`, `O-C`, or `O-D`.
4. Maintainer basis: `MR-A`, `MR-B` with amendments, or `MR-C`.

No application begins until the owner rules and supplies the next bounded
authority.
