# Datasheet: DEL-12-05 Security threat model

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-12-05 |
| Deliverable name | Security threat model |
| Package ID | PKG-12 |
| Package name | Security, Privacy, and Private Data Handling |
| Deliverable type | DOC_UPDATE |
| Scope item | SOW-040 |
| Objective | OBJ-010 |
| Context envelope | M |
| Anticipated product artifact | docs/security/threat_model.md |
| Setup artifact location | This deliverable folder only |
| Current setup state | SEMANTIC_READY |

Source basis: `_CONTEXT.md` sections "Description", "Anticipated Artifacts", "Scope Coverage", "Objective Support", and "Architecture Basis Injection"; `docs/_Registers/Deliverables.csv` row `DEL-12-05`; `docs/_Registers/ScopeLedger.csv` row `SOW-040`; `docs/_Registers/ContextBudgetQA.csv` row `DEL-12-05`.

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Primary purpose | Draft and maintain a threat model for private data, report sharing, plugins, imports, and supply chain exposure. | `_CONTEXT.md` Description; `docs/_Registers/Deliverables.csv` row `DEL-12-05` |
| Data protection objective | Protect private project, code, rule-pack, and component data in local-first workflows. | `docs/_Decomposition/SOFTWARE_DECOMP.md` objective `OBJ-010` |
| Governing scope | Private data handling controls, including redaction/export safeguards where reports or shared models may expose protected/private values. | `docs/_Decomposition/SOFTWARE_DECOMP.md` row `SOW-040` |
| Product posture | Local-first; no cloud service required for modeling, solving, rule checking, or reporting. | `docs/PRD.md` section 18.1 |
| Telemetry posture | Disabled by default; opt-in only if added; must not transmit private engineering data without explicit user action. | `docs/PRD.md` section 18.2; `docs/CONTRACT.md` `OPS-K-PRIV-2` |
| Rule-pack posture | Private or user-owned; versioned, checksummed, source-noted, and marked public/private. | `docs/SPEC.md` section 6; `docs/CONTRACT.md` `OPS-K-RULE-3` |
| Plugin/import posture | Adapters and plugins cannot bypass validation, unit checks, provenance, diagnostics, sandboxing, or report controls. | `_CONTEXT.md` Architecture Basis Injection `AB-00-07`; `docs/SPEC.md` section 1 |
| Report posture | Reports disclose provenance, warnings, limitations, and rule-pack identifiers without public protected content. | `docs/SPEC.md` section 8; `docs/IP_AND_DATA_BOUNDARY.md` section 7 |
| Professional boundary | Software and agents must not certify, seal, approve, authenticate, or declare code compliance for reliance. | `docs/CONTRACT.md` `OPS-K-AUTH-1`; `docs/DIRECTIVE.md` section 3 |

## Conditions

### In Scope

| Area | Threat-model coverage |
|---|---|
| Private project data | Local project files, model contents, result envelopes, report manifests, hashes, and user settings that may identify project engineering work. |
| Private code and rule-pack data | User-supplied rule packs, code-specific inputs, formulas, allowables, load combinations, rule-pack checksums, and source notices. |
| Private library data | Material libraries, component libraries, manufacturer/vendor values, owner standards, and project templates kept in user-controlled private paths. |
| Report sharing | Report export, report templates, bug reports, shared model packages, result tables, screenshots, and copy/export workflows that could disclose protected or private values. |
| Plugins and adapters | Import/export plugins, scripting/plugin APIs, FEA handoff adapters, private-library importers, and external-tool interfaces. |
| Supply chain | Application dependencies, plugins, adapter packages, build artifacts, packaging/signing choices, and provenance of imported data. |

### Out of Scope

| Area | Boundary |
|---|---|
| Legal sufficiency | This deliverable is not a legal opinion and does not determine redistribution rights. |
| Professional approval | This deliverable does not certify, approve, seal, or authenticate engineering work. |
| Cloud service design | Cloud operation is excluded unless separately authorized by the human project authority. |
| Detailed encryption implementation | Optional encrypted storage is noted as a product requirement; concrete key management remains `TBD`. |
| Exact plugin permission model | Plugin permissions, transport protocol, and external format list remain implementation-level `TBD`. |
| Protected standards content | No protected standards text, tables, figures, examples, formulas, allowables, or copied code data are included. |

## Construction

This setup document represents the planned content for `docs/security/threat_model.md` without writing that repo-level product artifact. It is organized as a maintainable threat-model basis for later architecture and implementation work.

### Threat Surfaces

| Surface | Representative exposure | Required model treatment |
|---|---|---|
| Local storage | Private project/rule/library files readable from user paths. | Record trust boundary, asset class, confidentiality risk, and storage-control questions. |
| Export/report | Reports, shared models, bug reports, result exports, and screenshots may carry private or protected values. | Record redaction, disclosure warning, provenance, and protected-content lint controls. |
| Telemetry | Diagnostic or usage collection could disclose private engineering data. | Record off-by-default posture and explicit opt-in/user-action requirements. |
| Plugins/adapters | Plugin, import/export, scripting, and FEA handoff flows could bypass validation or exfiltrate data. | Record no-bypass boundary, sandbox/permission questions, validation gates, and diagnostics. |
| Rule evaluator | Rule packs may contain private code data and expressions. | Record sandboxing, checksums, private/public metadata, and no arbitrary-code execution. |
| Supply chain | Dependencies, plugin packages, build artifacts, and importer libraries could be compromised or unreviewed. | Record provenance, dependency review, build reproducibility, and release-gate questions. |

### Asset Classes

| Asset class | Public/private posture | Notes |
|---|---|---|
| Public mechanics code | Public if original/permissive and protected-content clean. | Must not embed protected standards data. |
| Project model data | Private by default. | May include owner/project information, geometry, loads, results, and assumptions. |
| Rule packs | Private by default unless explicitly public/permissive. | Must carry source notice, redistribution status, version, and checksum. |
| Material/component libraries | Private by default unless provenance and redistribution rights are documented. | Public contribution requires provenance and review. |
| Reports and exports | Potentially private. | Must support warnings/redaction and avoid protected public templates. |
| Diagnostics and telemetry | Private-sensitive. | Must avoid private engineering/code data by default. |
| Secrets | Private. | Concrete storage and rotation controls remain `TBD`. |

### Control Themes

| Theme | Control intent | Source |
|---|---|---|
| Local-first default | Prevent hidden network dependency or hidden cloud storage for ordinary workflows. | `docs/PRD.md` 18.1; `docs/DIRECTIVE.md` section 4.2 |
| Explicit disclosure action | Require user intent before private data leaves local control. | `docs/PRD.md` 18.2-18.3 |
| Redaction and warning | Warn before export and redact private data from bug reports. | `docs/PRD.md` 18.3; `docs/_Decomposition/SOFTWARE_DECOMP.md` `SOW-040` |
| Provenance and redistribution | Record source, license/redistribution status, and review disposition. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; `docs/CONTRACT.md` `OPS-K-IP-2` |
| Sandboxed evaluation | Keep rule packs from executing arbitrary code. | `docs/SPEC.md` section 6; `docs/CONTRACT.md` `OPS-K-RULE-2` |
| No-bypass adapters | Ensure plugins/adapters cannot skip validation, provenance, diagnostics, sandboxing, or report controls. | `_CONTEXT.md` `AB-00-07`; `docs/SPEC.md` section 1 |
| Reproducible evidence | Bind hashes, checksums, versions, and warnings into reports and manifests. | `_CONTEXT.md` `AB-00-04`; `docs/SPEC.md` section 8 |

## References

| Reference | Use in this setup artifact |
|---|---|
| `INIT.md` | Bootstrap boundaries for open mechanics, protected data, professional responsibility, and local-first agent work. |
| `AGENTS.md` | TASK dispatch scope and bounded execution rule. |
| `docs/DIRECTIVE.md` | Product principles, stop rules, public/private data boundary, and professional-responsibility limits. |
| `docs/CONTRACT.md` | Invariants for IP, data, privacy, rule packs, reports, authority, and agents. |
| `docs/TYPES.md` | Canonical terms for protected standards data, user-supplied code data, private rule packs, and analysis statuses. |
| `docs/SPEC.md` | Layer responsibilities, rule-pack evaluator requirements, diagnostics, reports, and acceptance semantics. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data policy, provenance fields, quarantine rule, and report boundary. |
| `docs/PRD.md` | Security/privacy requirements, private-data handling, report prohibitions, and risk table. |
| `docs/_Decomposition/SOFTWARE_DECOMP.md` | PKG-12, SOW-040, OBJ-010, and AB-00 architecture-basis constraints. |
| `docs/_Registers/*.csv` | Machine-readable deliverable, scope, and context-budget rows. |

