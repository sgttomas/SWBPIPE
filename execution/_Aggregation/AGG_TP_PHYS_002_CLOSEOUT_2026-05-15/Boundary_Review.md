---
doc_id: AGG-TP-PHYS-002-BOUNDARY-REVIEW
doc_kind: aggregation.boundary_review
status: completed
created: 2026-05-15
tranche: TP-PHYS-002
---

# TP-PHYS-002 Boundary Review

## Contract Boundary

Reviewed against `docs/CONTRACT.md` and `docs/IP_AND_DATA_BOUNDARY.md`.

Relevant invariant conclusions:

- `OPS-K-MECH-1`: TP-PHYS-002 remains inside the 3D centerline/frame mechanics
  boundary.
- `OPS-K-MECH-2`: solver and diagnostics changes compute/report mechanics only;
  rule-pack evaluation and professional judgment remain outside software output.
- `OPS-K-UNIT-1`: unit policy remains upstream/TBD where not already explicit;
  TP-PHYS-002 did not define canonical unit constants or conversions.
- `OPS-K-SOLVER-1`: solver-facing changes have deterministic tests.
- `OPS-K-IP-1` / `OPS-K-DATA-1`: no protected standards data, bundled
  allowables, SIF/flexibility tables, proprietary data, or private values are
  introduced by the tranche evidence.
- `OPS-K-AUTH-1`: no certification, sealing, professional approval, or
  code-compliance claim is introduced.

## Integration Boundary

Accepted boundary interpretation:

- The frame kernel now has an explicit prescribed-displacement reduction path.
- Linear supports can add spring stiffness and reduce rigid/imposed
  displacement boundaries through the frame kernel.
- Primitive loads can lump explicit translational `ForcePerLength` element
  loads when element span/connectivity is supplied.
- Diagnostics can map frame, support, and primitive-load failures into existing
  code-neutral diagnostic records.
- The mechanics benchmark suite now includes an invented linear static
  integration fixture.

Excluded interpretations:

- sparse solver selection;
- production tolerance or release threshold approval;
- canonical project unit constants;
- pressure, thermal, wind, seismic, or code-procedure generation;
- nonlinear global solve integration;
- result-envelope, GUI, CLI, report, or persistence integration;
- rule-pack/code checking or allowable comparison;
- professional acceptance or code compliance.

## Preserved TBDs

The following remain open and are not resolved by this closeout:

- sparse solver selection;
- solver and benchmark tolerance policy;
- canonical calculation unit basis and conversion constants;
- release thresholds and CI gate policy;
- result-envelope/export integration;
- benchmark publication scope;
- broader publication policy;
- professional reliance.

## Verdict

PASS. TP-PHYS-002 is mechanics integration and validation evidence only. It is
not a release, compliance, professional reliance, or lifecycle promotion record.
