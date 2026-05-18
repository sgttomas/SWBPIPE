---
doc_id: AGG-TP-PHYS-001-BOUNDARY-REVIEW
doc_kind: aggregation.boundary_review
status: completed
created: 2026-05-15
tranche: TP-PHYS-001
---

# TP-PHYS-001 Boundary Review

## Contract Boundary

Reviewed against `docs/CONTRACT.md` and `docs/IP_AND_DATA_BOUNDARY.md`.

Relevant invariant conclusions:

- `OPS-K-MECH-1`: TP-PHYS-001 remains inside 3D centerline/frame and related
  open-mechanics validation work.
- `OPS-K-MECH-2`: solver and stress changes compute mechanics only; rule-pack
  and professional judgment remain outside software output.
- `OPS-K-UNIT-1`: unit policy remains upstream/TBD where not already explicit;
  TP-PHYS-001 did not define canonical unit constants.
- `OPS-K-SOLVER-1`: solver-adjacent changes have deterministic tests.
- `OPS-K-IP-1` / `OPS-K-DATA-1`: no protected standards data, bundled
  allowables, SIF/flexibility tables, proprietary data, or private values are
  introduced by the tranche evidence.
- `OPS-K-AUTH-1`: no certification, sealing, professional approval, or
  code-compliance claim is introduced.

## `recover_stress_range` Review

The new `recover_stress_range` surface is reviewed as mechanics-only component
delta behavior.

Accepted boundary interpretation:

- It compares recovered component stresses between two unblocked mechanics
  states.
- It reports component-wise ranges only.
- Omitted optional pressure in both states remains omitted, not zeroed.
- Asymmetric optional pressure components block range output with findings.
- Blocked input states block range output.

Excluded interpretations:

- fatigue assessment;
- design-code stress range;
- allowable comparison;
- equivalent or principal stress;
- compliance check;
- professional acceptance.

## Benchmark Review

Mechanics and stress benchmark changes consume the current engine APIs and keep
fixture values invented/original public mechanics values. The stress benchmark
now calls `recover_stress_range` rather than duplicating component range logic.

## Preserved TBDs

The following remain open and are not resolved by this closeout:

- sparse solver selection;
- solver and benchmark tolerance policy;
- canonical calculation unit basis and conversion constants;
- release thresholds;
- CI gate policy;
- result-envelope/export integration;
- benchmark publication scope;
- broader publication policy;
- professional reliance.

## Verdict

PASS. TP-PHYS-001 is mechanics hardening and validation evidence only. It is
not a release, compliance, professional reliance, or lifecycle promotion
record.
