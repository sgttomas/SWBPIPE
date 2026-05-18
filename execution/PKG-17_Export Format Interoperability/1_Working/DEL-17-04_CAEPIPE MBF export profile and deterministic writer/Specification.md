# Specification: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

## Normative Scope

DEL-17-04 shall define a source-bounded CAEPIPE MBF export profile and deterministic writer contract/design for later implementation. It shall consume DEL-17-01 source authority and DEL-17-02 package/profile/stable-ID/loss-report requirements.

This deliverable shall not implement the MBF writer, schemas, tests, fixtures, CAEPIPE parser, external run harness, public API, GUI behavior, release claims, CAEPIPE compatibility claims, code-compliance claims, or professional-acceptance claims.

## Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-04-REQ-001 | The CAEPIPE MBF profile shall declare target version basis as `TBD` until source-confirmed. |
| DEL-17-04-REQ-002 | The first MBF record-family subset shall remain `TBD` until source evidence or human-approved clarification closes it. |
| DEL-17-04-REQ-003 | The writer contract shall not silently approximate unsupported physical or analytical model entities. |
| DEL-17-04-REQ-004 | The writer contract shall require a loss report for omitted, approximated, delegated, unsupported, and TBD behavior. |
| DEL-17-04-REQ-005 | Stable canonical ID carrying inside MBF shall remain `TBD`; sidecar mapping shall be required when direct carrying is not source-confirmed. |
| DEL-17-04-REQ-006 | Pass-through target options shall remain target configuration metadata and shall not become OpenPipeStress local code-checking logic. |
| DEL-17-04-REQ-007 | Later implementation shall use invented fixtures only. |

## Verification Requirements

| Check | Requirement |
|---|---|
| Source trace | CAEPIPE-specific statements cite DEL-17-01 source IDs or remain `TBD`. |
| Boundary trace | No proprietary examples or protected data are introduced. |
| Loss trace | Unsupported and TBD behavior is visible in the loss-report contract. |
| Dependency trace | `Dependencies.csv` validates as v3.1. |

## Acceptance Criteria

| Criterion | Acceptance treatment |
|---|---|
| Source basis mapping | Each CAEPIPE-specific requirement is traced to DEL-17-01, DEL-17-02, an admitted public MBF reference, or an explicit `TBD` closure path. |
| Target version closure | REQ-001 remains unresolved until the first CAEPIPE version/profile is source-confirmed or human-approved as a bounded profile decision. |
| Record subset closure | REQ-002 remains unresolved until MBF record families and required fields are enumerated from admitted evidence. |
| Stable ID preservation | REQ-005 is accepted only when direct MBF carrying is source-confirmed or the writer uses manifest-referenced sidecar mapping. |
| Loss category coverage | REQ-004 is accepted only when exported, omitted, approximated, delegated, unsupported, and `TBD` behavior are all visible in the loss report. |
| Diagnostic classification | Blocking versus non-blocking diagnostics remain `TBD` until the later profile tranche defines unsupported-entity severity. |
| Fixture provenance | REQ-007 is accepted only with invented fixtures and no vendor, proprietary, protected, or owner/project examples. |

## Downstream Use

DEL-17-05 may consume this deliverable later for CAEPIPE external run harness and CSV parser work. DEL-17-05 remains blocked until DEL-17-04 has committed evidence.
