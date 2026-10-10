# Source Basis Register: DEL-17-01

## Register Purpose

This register records the admitted source basis for PKG-17 export-format work. It is source evidence and boundary control, not implementation authority by itself.

## Public and Official Source Evidence

These sources are accepted as public/official evidence for documented interfaces only. They do not authorize target-specific exporter coverage, bundled commercial execution, professional acceptance, or protected-content reproduction.

| Source ID | Source Path | Best-Effort Location | Admitted Evidence | Boundary |
|---|---|---|---|---|
| CAEPIPE-IMPORT-MBF | `https://www.sstusa.com/docs/users_manual/import_mbf_print.htm` | `IMPORT MBF`; `Command line operation`; `Format of .MBF` | CAEPIPE can import `.mbf` model batch files, command-line use can produce CSV output, and the page lists MBF section keywords and record descriptions. | Does not define an OpenPipeStress writer subset, does not grant permission to copy examples wholesale, and does not turn CAEPIPE code options into OpenPipeStress local code-checking logic. |
| CAEPIPE-EXPORT-DATA | `https://www.sstusa.com/docs/users_manual/export_data_from_caepipe.htm` | `Export data from CAEPIPE` | CAEPIPE documents model/result export surfaces including CSV/TEXT print-to-file, PDF print, MBF, PCF, material-library ASCII, time-history CSV, hanger-report ASCII, and 3D plant-design exports. | Does not prove automated parser stability for every exported section; parser coverage remains explicit downstream scope. |
| CAEPIPE-EXPORT-MBF | `https://www.sstusa.com/docs/users_manual/export_mbf.htm` | `EXPORT MBF` | CAEPIPE can export Layout-window model data to `.mbf`, including current/latest-version MBF and a 6.xx compatibility option. | Does not imply OpenPipeStress can reproduce all CAEPIPE model constructs or support every MBF version/profile. |
| CAEPIPE-BATCH | `https://www.sstusa.com/docs/users_manual/running_caepipe_caepipe_3d__in_batch_mode.htm` | `RUNNING CAEPIPE/CAEPIPE 3D+ IN BATCH MODE`; `Run the Analysis and Output Results in CSV Format`; `Note` | CAEPIPE/CAEPIPE 3D+ batch-mode documentation describes using MBF as an executable input argument and outputting CSV results, with argument/path constraints. | Harness remains optional, user-owned, license-bound, and environment-specific; no bundled executable or license-bypass behavior is admitted. |
| CAEPIPE-PCF | `https://www.sstusa.com/pdfs/PCF.pdf` | `PD2CAEPIPE - Plant Design-to-CAEPIPE Translator`; `Reference`; `PCF to CAEPIPE component Mapping` | Official/public PCF translator documentation records translator inputs, defaults, mapping behavior, and limitations such as defaulted or externally mapped values. | Supports conservative subset and loss-report planning only; hidden translator behavior and external mapping databases are not reproduced. |
| GLTF-2.0 | `https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html` | `glTF 2.0 Specification`; `2.4 glTF Basics`; `3.2 Asset`; `3.3 Indices and Names`; `3.4 Coordinate System and Units`; `4 GLB File Format Specification` | Khronos glTF 2.0 defines asset structure, versioning, indices/names, coordinate/units conventions, scenes/nodes, and GLB container semantics for visual exchange. | Review geometry only; not solver geometry equivalence, stress-model proof, or professional acceptance evidence. |

## Accepted Project References

These sources are accepted project-basis references. They are governance, planning, or workflow sources rather than public target-format evidence.

| Source ID | Source Path | Best-Effort Location | Admitted Use | Boundary |
|---|---|---|---|---|
| PLAN-EXPORT-INTEROP | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` | location TBD within plan | Strategy, target priority, boundary framing, and downstream workflow intent. | Not a substitute for target-format field details or vendor/developer-team answers. |
| CONTRACT | `docs/CONTRACT.md` | `1.9 Provenance and Epistemic Integrity`; `1.10 Write Scope and Snapshots` | Binding invariants for provenance, unknown/TBD treatment, claim strength, authority, and write scope. | Project governance, not legal advice or professional acceptance. |
| IP-DATA | `docs/IP_AND_DATA_BOUNDARY.md` | `2. Public repository may contain`; `3. Public repository must not contain`; `5. Quarantine rule`; `6. Private user data`; `7. Report boundary` | Protected-content and public/private data boundary policy. | Requires human/legal review when redistribution rights or protected-content status are uncertain. |
| SPEC | `docs/SPEC.md` | `2. Deliverable Folder Layout`; `3. _STATUS.md`; `5. _DEPENDENCIES.md`; `8. _MEMORY.md`; `11. Snapshot and Pointer Conventions` | Technical and agentic workflow context for deliverable files, dependencies, memory, lifecycle, and snapshots. | Does not override SCA-004/DAG-005 approval boundaries. |
| TYPES | `docs/TYPES.md` | `4. Agent Classification`; `5. Deliverable Lifecycle States`; `10. Epistemic Ontology` | Type 2 authority limits, lifecycle vocabulary, and epistemic labels for FACT/ASSUMPTION/PROPOSAL/TBD. | Does not authorize lifecycle transition or human ruling. |
| DAG-005 | `execution/_DAG/DAG-005/APPROVAL_RECORD.md` | `Approval Conditions`; `Immediate Follow-Up Boundary` | Active graph authority boundary for PKG-17 coordination and explicit downstream obligations. | Does not dispatch Type 2 work, promote candidate rows, change lifecycle state, or create release/code-compliance claims. |

## Finding Register

| Finding ID | Statement | Source IDs | Best-Effort Source Location | Evidence Type | Downstream Use |
|---|---|---|---|---|---|
| F-17-01-001 | MBF is the first CAEPIPE-focused target because admitted sources identify it as a text model exchange path and the plan prioritizes it before PCF. | PLAN-EXPORT-INTEROP, CAEPIPE-IMPORT-MBF, CAEPIPE-EXPORT-MBF | PLAN location TBD; CAEPIPE `IMPORT MBF`; CAEPIPE `EXPORT MBF` | Source-grounded strategy with plan-location TBD | DEL-17-04 |
| F-17-01-002 | CAEPIPE external execution can be planned only as an optional harness around a user-provided executable and permitted environment. | PLAN-EXPORT-INTEROP, CAEPIPE-IMPORT-MBF, CAEPIPE-BATCH, IP-DATA | PLAN location TBD; CAEPIPE `Command line operation`; CAEPIPE `RUNNING CAEPIPE/CAEPIPE 3D+ IN BATCH MODE`; IP-DATA `6. Private user data` | Source-grounded boundary with plan-location TBD | DEL-17-05 |
| F-17-01-003 | CAEPIPE CSV/text outputs can support regression/handoff parsing, but parser coverage must be explicitly scoped. | CAEPIPE-EXPORT-DATA, CAEPIPE-BATCH, PLAN-EXPORT-INTEROP | CAEPIPE `Export data from CAEPIPE`; CAEPIPE batch `Run the Analysis and Output Results in CSV Format`; PLAN location TBD | Source-grounded with TBD coverage | DEL-17-05, DEL-17-06 |
| F-17-01-004 | PCF translation is broader interoperability with mapping/default caveats; downstream work should be conservative and loss-report driven. | PLAN-EXPORT-INTEROP, CAEPIPE-PCF | PLAN location TBD; PCF PDF `Reference`; `PCF to CAEPIPE component Mapping` | Source-grounded boundary with plan-location TBD | DEL-17-07 |
| F-17-01-005 | GLB/glTF should be review geometry only and must preserve identity assumptions through file metadata or sidecars as needed. | GLTF-2.0, PLAN-EXPORT-INTEROP | glTF `2.4 glTF Basics`; `3.2 Asset`; `3.3 Indices and Names`; `3.4 Coordinate System and Units`; PLAN location TBD | Source-grounded boundary with plan-location TBD | DEL-17-08 |
| F-17-01-006 | All target-specific export claims must distinguish exported, omitted, approximated, unsupported, TBD, and delegated behavior. | PLAN-EXPORT-INTEROP, CONTRACT, TYPES | PLAN location TBD; CONTRACT `1.9 Provenance and Epistemic Integrity`; TYPES `10. Epistemic Ontology` | Project-governed requirement with plan-location TBD | Relevant downstream PKG-17 exporters and harnesses |

## Protected and Private Boundary Constraints

| Boundary ID | Constraint | Source Path | Best-Effort Location | Downstream Effect |
|---|---|---|---|---|
| B-17-01-001 | Do not include protected standards text, protected tables, standards-derived examples, code allowables, SIF/flexibility tables, or owner criteria in public DEL-17 artifacts. | `docs/IP_AND_DATA_BOUNDARY.md`; `docs/CONTRACT.md` | IP-DATA `3. Public repository must not contain`; CONTRACT `1.9 Provenance and Epistemic Integrity` | Export schemas may provide private/import slots, but public examples must remain invented, public-domain, permissively licensed, or TBD. |
| B-17-01-002 | Do not bundle CAEPIPE binaries, commercial examples, proprietary model files, copied report templates, hidden binary-format behavior, or license-bypass instructions. | `_CONTEXT.md`; `docs/IP_AND_DATA_BOUNDARY.md`; `CAEPIPE_Question_Dossier.md` | `_CONTEXT.md` `Package Exclusions`; IP-DATA `3. Public repository must not contain`; dossier `Question Boundary` | External-run harnesses remain user-owned and license-bound; questions must avoid proprietary internals. |
| B-17-01-003 | Treat target code/check options as pass-through target configuration unless a separate public rule-pack design admits local logic. | `docs/IP_AND_DATA_BOUNDARY.md`; `docs/CONTRACT.md`; CAEPIPE MBF public page | IP-DATA `3. Public repository must not contain`; CONTRACT `K-CLAIM-1`; CAEPIPE `Format of .MBF` / option sections | Downstream MBF work must avoid converting target solver options into OpenPipeStress code-compliance behavior. |

## TBD Register

| TBD ID | Question | Affected Deliverables | Closure Route |
|---|---|---|---|
| TBD-17-01-001 | Confirm first supported CAEPIPE version/profile and citation target. | DEL-17-04, DEL-17-05 | CAEPIPE developer/support clarification or explicit project profile decision. |
| TBD-17-01-002 | Confirm initial MBF record families and required fields for the first deterministic writer subset. | DEL-17-04 | Public documentation review plus CAEPIPE clarification where public docs are insufficient. |
| TBD-17-01-003 | Confirm stable ID carrying strategy inside MBF versus sidecar-only mappings. | DEL-17-02, DEL-17-04 | MBF docs review and downstream profile design; no assumption that comments/user fields are available until confirmed. |
| TBD-17-01-004 | Confirm which CSV result sections are stable and useful for automated parser coverage. | DEL-17-05, DEL-17-06 | CAEPIPE documentation/support clarification and fixture smoke tests when a licensed executable is available. |
| TBD-17-01-005 | Define conservative PCF subset and translator-default rejection/warning rules. | DEL-17-07 | PCF translator doc review and downstream loss-report contract; external mapping database behavior remains private/user-controlled unless public redistribution is cleared. |
| TBD-17-01-006 | Define GLB/glTF review geometry identity metadata and sidecar policy. | DEL-17-08 | glTF spec review and downstream export-profile contract; solver-fidelity claims remain excluded. |
