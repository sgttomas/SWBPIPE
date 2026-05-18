# Source Basis Register: DEL-17-01

## Register Purpose

This register records the admitted source basis for PKG-17 export-format work. It is source evidence and boundary control, not implementation authority by itself.

## Admitted Sources

| Source ID | Location | Admitted Use | Boundary |
|---|---|---|---|
| PLAN-EXPORT-INTEROP | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` | Strategy, target priority, boundary framing, and downstream workflow intent. | Not a substitute for target-format field details. |
| CAEPIPE-IMPORT-MBF | `https://www.sstusa.com/docs/users_manual/import_mbf_print.htm` | Public evidence for MBF import/model input and command-line result behavior. | Does not define OpenPipeStress writer coverage by itself. |
| CAEPIPE-EXPORT-DATA | `https://www.sstusa.com/docs/users_manual/export_data_from_caepipe.htm` | Public evidence for CAEPIPE export/print model and result data surfaces. | Does not guarantee stable parser coverage for every result section. |
| CAEPIPE-EXPORT-MBF | `https://www.sstusa.com/docs/users_manual/export_mbf.htm` | Public evidence for MBF model-data export from CAEPIPE. | Does not imply OpenPipeStress can reproduce all CAEPIPE model constructs. |
| CAEPIPE-BATCH | `https://www.sstusa.com/docs/users_manual/running_caepipe_caepipe_3d__in_batch_mode.htm` | Public evidence for optional batch/external execution planning. | Harness remains user-owned and license-bound. |
| CAEPIPE-PCF | `https://www.sstusa.com/pdfs/PCF.pdf` | Public evidence for PCF translator behavior, mappings, and defaults. | Supports conservative subset planning, not hidden translator reproduction. |
| GLTF-2.0 | `https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html` | Public exchange-format reference for review geometry. | Review geometry only; not solver validation. |
| CONTRACT | `docs/CONTRACT.md` | Binding invariants for IP, data, authority, reports, agents, and Type 2 execution. | Project governance, not legal advice. |
| IP-DATA | `docs/IP_AND_DATA_BOUNDARY.md` | Protected-content and public/private data boundary policy. | Requires human/legal review when rights are uncertain. |
| SPEC | `docs/SPEC.md` | Technical and agentic workflow context. | Does not override SCA-004/DAG-005 approval boundaries. |

## Finding Register

| Finding ID | Statement | Source IDs | Evidence Type | Downstream Use |
|---|---|---|---|---|
| F-17-01-001 | MBF is the first CAEPIPE-focused target because admitted sources identify it as a text model exchange path and the plan prioritizes it over PCF. | PLAN-EXPORT-INTEROP, CAEPIPE-IMPORT-MBF, CAEPIPE-EXPORT-MBF | Source-grounded strategy | DEL-17-04 |
| F-17-01-002 | CAEPIPE external execution can be planned only as an optional harness around a user-provided executable and permitted environment. | PLAN-EXPORT-INTEROP, CAEPIPE-IMPORT-MBF, CAEPIPE-BATCH, IP-DATA | Source-grounded boundary | DEL-17-05 |
| F-17-01-003 | CAEPIPE CSV/text outputs can support regression/handoff parsing, but parser coverage must be explicitly scoped. | CAEPIPE-EXPORT-DATA, PLAN-EXPORT-INTEROP | Source-grounded with TBD coverage | DEL-17-05, DEL-17-06 |
| F-17-01-004 | PCF translation is broader interoperability with mapping/default caveats; it should be conservative and loss-report driven. | PLAN-EXPORT-INTEROP, CAEPIPE-PCF | Source-grounded boundary | DEL-17-07 |
| F-17-01-005 | GLB/glTF should be review geometry only and must preserve identity assumptions through file metadata or sidecars as needed. | GLTF-2.0, PLAN-EXPORT-INTEROP | Source-grounded boundary | DEL-17-08 |
| F-17-01-006 | All target-specific export claims must distinguish exported, omitted, approximated, unsupported, and delegated behavior. | PLAN-EXPORT-INTEROP, CONTRACT | Project-governed requirement | DEL-17-02 through DEL-17-09 |

## TBD Register

| TBD ID | Question | Affected Deliverables | Closure Route |
|---|---|---|---|
| TBD-17-01-001 | Confirm first supported CAEPIPE version/profile and citation target. | DEL-17-04, DEL-17-05 | CAEPIPE developer/support clarification or explicit project profile decision. |
| TBD-17-01-002 | Confirm initial MBF record families and required fields for the first deterministic writer subset. | DEL-17-04 | Public documentation review plus CAEPIPE clarification where public docs are insufficient. |
| TBD-17-01-003 | Confirm stable ID carrying strategy inside MBF versus sidecar-only mappings. | DEL-17-02, DEL-17-04 | MBF docs review and downstream profile design. |
| TBD-17-01-004 | Confirm which CSV result sections are stable and useful for automated parser coverage. | DEL-17-05, DEL-17-06 | CAEPIPE documentation/support clarification and fixture smoke tests when a licensed executable is available. |
| TBD-17-01-005 | Define conservative PCF subset and translator-default rejection/warning rules. | DEL-17-07 | PCF translator doc review and downstream loss-report contract. |
| TBD-17-01-006 | Define GLB/glTF review geometry identity metadata and sidecar policy. | DEL-17-08 | glTF spec review and downstream export-profile contract. |

