# Guidance: DEL-17-09 Export adapter SDK and additional targets

## Purpose

Use DEL-17-09 as the boundary guide for future export adapters and additional targets. Its job is to keep extension work possible without allowing adapter code, target names, or community contribution pressure to weaken the PKG-17 source-basis, stable-ID, loss-report, protected-content, privacy, diagnostics, or professional-boundary controls.

This guidance is contract-level only. It is not an implementation guide, public API promise, compatibility statement, or target support announcement.

## Principles

- Start from DEL-17-02 package/profile/stable-ID/manifest/loss-report contracts before considering any adapter-specific behavior.
- Admit target behavior only from public, official, project-owned, user-provided-with-rights, or otherwise rights-cleared sources.
- Keep candidate targets in a non-support state until source basis, target version basis, redistribution posture, validation expectations, and unresolved TBDs are recorded.
- Treat every export as potentially lossy; require manifest, ID-map, diagnostics, and loss-report evidence even when the target file is produced.
- Keep adapter work behind the same no-bypass controls as first-party export code: schema validation, unit checks, provenance, diagnostics, private-data controls, protected-content screening, report boundaries, and professional-boundary wording. SourcePath: `docs/SPEC.md`; SectionRef: 4.5 Plugin and extension domain contracts.
- Prefer explicit `TBD`, `unsupported`, `omitted`, `approximated`, or `delegated` classifications over broad compatibility or support language.

## Considerations

### Target admission

Additional targets should move through conservative admission states:

| State | Meaning |
|---|---|
| Candidate | A target has been named for possible future review, but no support is claimed. |
| Source-basis pending | Public or rights-cleared source evidence has not yet been admitted. Target behavior remains `TBD`. |
| Source-basis admitted | Source evidence has been recorded with boundary notes, but support still depends on a later sealed implementation/validation deliverable. |
| Contract-ready | The target profile can reference source-basis IDs, required fields, loss categories, stable-ID behavior, and validation obligations. |
| Implementation-gated | Future code/test work may be dispatched under a separate sealed deliverable. DEL-17-09 does not perform that work. |

Do not use these states as release, compatibility, vendor-certification, or professional acceptance states.

### Adapter SDK surface

The SDK surface should be described as obligations and review checkpoints until a later implementation brief authorizes code. Useful obligations include source basis, profile metadata, stable identity, package member declaration, loss reporting, diagnostics, protected-content screening, privacy classification, and professional-boundary wording. Exact method names, schemas, endpoints, loader mechanisms, and package structure remain `TBD`.

Vocabulary note:

| Term | Use in this deliverable |
|---|---|
| Adapter SDK contract | The normative obligation set for future exporters: source basis, target profile intake, validation, stable identity, loss reporting, diagnostics, privacy, protected-content screening, and professional-boundary controls. |
| SDK surface | The future runtime/API methods, schemas, loader mechanics, and package structure. This remains `TBD` until later sealed implementation work. |
| Target profile contract | The per-target declaration of target family, target version basis, source-basis IDs, unit and coordinate policy, supported/unsupported behavior, loss policy, and validation policy. |
| Target registry contract | The admission record for candidate/source-basis/contract-ready/implementation-gated target states. It is not a release, compatibility, or professional-acceptance registry. |

### Additional targets

Additional targets may include plant-design exchanges, solver-neutral packages, geometry exports, or user-owned APIs only when the target brings a source basis and rights posture. A target name alone is not evidence. Common industry practice is not enough to close target behavior gaps.

### External execution

If a future adapter involves a downstream executable or hosted service, that execution must remain optional and user-owned unless a later human authority explicitly approves a different boundary. A successful external run may be recorded as regression or handoff evidence; it must not be described as professional acceptance, formal validation, code compliance, or vendor certification.

External-run evidence is admissible only as bounded technical evidence when the user controls the executable/service, has the legal right to run it, preserves run artifacts, and records the target version, profile version, manifest or run record, and limitations. Exclude or quarantine external-run evidence when licensing, redistribution, private-data, protected-content, provenance, or target-version basis is unclear. Treat external-run evidence as non-authoritative context when it supports regression or handoff review but does not close source-basis, code-compliance, professional-acceptance, or formal-validation questions. SourcePath: `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`; SectionRef: CAEPIPE Scripted Validation Harness and Validation Strategy. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 6. Private/user-supplied data boundary.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Community extensibility vs. governance control | Favor a narrow admission contract with required evidence and loss reporting. A target can remain useful while still carrying explicit limitations. |
| Broad target list vs. source quality | Keep unsupported targets as candidates. Admit fewer targets with stronger source basis instead of naming many targets as supported. |
| Adapter convenience vs. stable identity | Require stable ID maps and sidecars where target artifacts cannot carry canonical IDs directly. |
| Fast file writing vs. auditability | Validation, manifests, diagnostics, and loss reports are part of the export outcome, not optional extras. |
| Target-specific options vs. local code-checking | Pass-through target options may be recorded as target configuration, but shall not become OpenPipeStress code-checking logic. |

## Examples

Acceptable contract-level example:

- A future target registry entry names a target as `Candidate`, records source-basis status as `TBD`, states that no support is claimed, and lists required evidence before implementation can be dispatched.

Acceptable public template example:

- An invented adapter checklist uses synthetic target names and invented model references to show where source basis, stable ID behavior, loss categories, and boundary notes would be recorded.

Not acceptable:

- A public fixture copied from a vendor sample, standards table, protected format manual, private project file, or licensed report without documented redistribution rights.
- A statement that a named target is compatible, supported, validated, code-compliant, accepted for professional use, or release-ready before source basis and later sealed validation work exist.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No current conflict detected. | NA | NA | NA | NA | TBD |
