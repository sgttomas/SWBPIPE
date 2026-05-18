# Guidance: DEL-12-04 Secret and private-library handling

## Purpose

This deliverable gives downstream implementation work a bounded, local-first privacy control surface for private libraries and credential references. It exists so OpenPipeStress can support user-owned rule packs, material libraries, component libraries, owner design bases, and project models without treating private or protected content as public repository data.

Sources: `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` SOW-029, SOW-040, and OBJ-010; `docs/PRD.md` sections 17.3 and 18.3.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Local-first by default | Private-library handling should not require cloud operation, cloud storage, telemetry, or external network access. | `docs/PRD.md` section 18.1 |
| References over secrets | Store opaque references to credentials and private library locations, not usable secret material. | ASSUMPTION from `docs/PRD.md` section 18.3 and plugin permission controls |
| Private unless proven public | Unknown provenance or redistribution status should not be promoted to public-safe. | `docs/IP_AND_DATA_BOUNDARY.md` sections 4 and 6 |
| Fail closed at boundaries | Imports, exports, report generation, bug reports, telemetry, plugin access, and public contribution flows should block, redact, warn, or quarantine when privacy status is uncertain. | `docs/IP_AND_DATA_BOUNDARY.md` section 5; `docs/architecture/plugin_boundary.md` Private Data Handling |
| Metadata without disclosure | Reports and exported payloads may reference private rule packs or libraries by identity/version/checksum/source note without exposing protected formulas or private values. | `docs/PRD.md` sections 15.2 and 17.3 |
| No-bypass architecture | Adapters and plugins are translators; they cannot bypass validation, provenance, privacy, diagnostics, checksums, rule sandboxing, or report controls. | AB-00-02, AB-00-06, AB-00-07; `docs/architecture/extension_domain_contracts.md` No-Bypass Rules |

## Considerations

- A private library registry should be treated as a control surface, not a data dump. It should carry enough metadata to validate, warn, hash, redact, and route assets without exposing private values.
- The exact physical project package/container and local secret provider remain `TBD`; this deliverable should not lock the product to one platform mechanism without human approval.
- Secret handling tests should use non-sensitive sentinel markers and fixture-only paths, not realistic tokens, passwords, API keys, owner data, material tables, or private component values.
- Optional encrypted storage for private libraries is called out by the PRD but not fully specified. Treat encryption defaults, key lifecycle, recovery, and platform storage as human/architecture decisions still requiring a later implementation brief.
- A checksum identifies a referenced artifact or payload. It is not permission to expose the artifact and does not make protected/private content public.
- Public examples may use invented data only. They should not be made more realistic by copying standards tables, proprietary formulas, commercial examples, or private owner data.

## Trade-offs

| Tension | Conservative posture |
|---|---|
| Detailed registry fields vs. unresolved schema layout | Specify required metadata semantics now; leave exact schema file placement and code-generation details `TBD`. |
| User convenience vs. accidental disclosure | Prefer explicit warning, redaction, quarantine, or permission grant over silent inclusion. |
| Report reproducibility vs. private-content exposure | Include identity/version/checksum/source-note references; omit formulas, protected values, and private library contents by default. |
| Plugin extensibility vs. privacy boundary | Use denied-by-default permissions and no-bypass rules. |
| Local encryption expectations vs. platform variability | Record optional encrypted storage as required consideration, not as a completed design. |

## Examples

The following are non-sensitive behavior examples only. They intentionally avoid real credentials, real private paths, real project data, and protected standards content.

| Scenario | Expected behavior |
|---|---|
| A user registers a private material library | The registry marks it private, records provenance and redistribution status, stores an opaque local reference, and prevents default telemetry/export inclusion. |
| A private rule pack is referenced in a report | The report can show rule-pack identity, version, checksum, source note, warnings, and missing-input findings without embedding protected formulas or private values. |
| A plugin requests private-library read access | The request is denied unless a grant exists; denial emits a diagnostic instead of bypassing the application service boundary. |
| Imported data appears to be copied protected content | Import/publication stops, the record is marked suspected, and human review is required before public use. |
| A test fixture needs credential behavior | The fixture uses a non-sensitive placeholder reference and asserts that no usable secret value is persisted or emitted. |

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| DEL-12-04-CF-001 | Exact local secret provider and encrypted-storage default are not specified. | `docs/PRD.md` section 18.3 | `docs/architecture/plugin_boundary.md` Remaining TBDs | Specification Requirements; Procedure Steps | Keep provider/encryption mechanics `TBD`; require opaque secret references and no real secrets in artifacts now. | TBD |
