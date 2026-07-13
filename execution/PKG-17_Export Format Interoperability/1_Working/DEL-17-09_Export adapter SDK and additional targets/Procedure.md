# Procedure: DEL-17-09 Export adapter SDK and additional targets

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-17-09-DECL-004`.

## Purpose

This procedure defines how future work should use the DEL-17-09 adapter SDK and target-admission contract. It is an operational contract for source intake, target admission, adapter review, validation planning, and closeout. It does not implement adapters, schemas, code, tests, public API endpoints, runtime loaders, or target-specific writers.

## Prerequisites

Before a future target adapter or additional target can move beyond candidate status:

1. Read the DEL-17-01 four-document kit, `Source_Basis_Register.md`, and `CAEPIPE_Question_Dossier.md` when the target is affected by PKG-17 source-basis rules.
2. Read the DEL-17-02 four-document kit and use its export package, profile, stable-ID, manifest, and loss-report contract objects.
3. Confirm the target has public, official, project-owned, user-provided-with-rights, or otherwise rights-cleared source evidence.
4. Confirm protected standards data, proprietary examples, private project data, copied vendor files, and license-bypass material are not being introduced.
5. Confirm runtime permission, sandbox, public API transport, package layout, concrete schema layout, external execution, and target-specific validation remain `TBD` unless a later sealed brief authorizes them.

## Steps

### 1. Register a candidate target

1. Record the target name only as a candidate.
2. Record source-basis status as `TBD` until source evidence is admitted.
3. Record that no support, compatibility, validation, release, code-compliance, or professional-acceptance claim is made.
4. Record any known IP, license, privacy, or redistribution concern as a blocker or `TBD`.
5. Record the target-admission decision artifact as `TBD` unless a later sealed brief names a registry file, run record, or decision log.

### 2. Admit source basis

1. Classify each source as public official documentation, public specification, project-owned material, user-provided rights-cleared material, private-only material, or rejected/quarantined material.
2. Reject or quarantine suspected protected standards content, proprietary examples, copied target files, private project data, or unclear-redistribution material.
3. Record source location, license or redistribution basis, admitted use, boundary notes, and affected target behavior.
4. Mark unresolved or version-sensitive target behavior as `TBD`.
5. Record the source-basis admission owner and ruling record as `TBD` unless a later sealed brief assigns that responsibility.

### 3. Draft the target profile contract

1. Reference DEL-17-02 package/profile/stable-ID/manifest/loss-report concepts.
2. Declare target version basis or `TBD`.
3. Declare unit policy, coordinate policy, source-basis IDs, stable-ID behavior, loss-report policy, and external execution policy where applicable.
4. Classify each entity or behavior family as exported, omitted, approximated, delegated, unsupported, or `TBD`.
5. Preserve sidecar mapping requirements where target artifacts cannot carry canonical IDs.

### 4. Review adapter boundary obligations

1. Confirm adapter operations cannot bypass schema validation, unit checks, provenance checks, diagnostics/result envelopes, protected-content screening, private-data controls, report controls, solver boundaries, rule-pack sandboxing, or professional-boundary wording.
2. Confirm any filesystem, network, process, storage, private-data, or rule-pack access remains denied unless a later approved runtime/sandbox design grants a bounded capability.
3. Confirm target options are treated as pass-through configuration and not as OpenPipeStress code-checking logic.

### 5. Prepare validation checklist

The checklist for any later target implementation should cover:

| Checklist topic | Expected evidence artifact |
|---|---|
| Source-basis completeness | Source-basis record or dossier entry with source location, source-basis ID, license/redistribution posture, admitted use, and review disposition. |
| Target version basis | Target profile record naming the target version basis or marking it `TBD`. |
| Units and coordinate conventions | Profile field, validation record, or review note confirming unit and coordinate policy; unresolved cases remain `TBD`. |
| Stable ID mapping and sidecars | ID-map artifact, sidecar reference, or manifest entry; omitted mappings require reasons. |
| Export package member inventory | Manifest or package-member list with emitted files, diagnostics, reports, hashes where applicable, and declared omissions. |
| Manifest and hash policy | Manifest record identifying deterministic text/JSON hashes or explaining why a hash is unavailable. |
| Loss-report categories | Loss report with affected canonical IDs, category, severity, target artifact if any, reason, source-basis reference, and downstream implication. |
| Diagnostics and severity routing | Validation report or diagnostics envelope preserving warnings, blockers, and target limitations. |
| Privacy and protected-content screening | Review note or contribution disposition showing public/private/protected-content status and quarantine/rejection when needed. |
| External execution ownership when applicable | Run manifest or review note showing user-owned executable/service, target version/profile version, preserved run artifact, and non-authoritative status. |
| No compatibility, code-compliance, release, formal-validation, or professional-acceptance wording | Boundary review note confirming prohibited claims were not introduced. |

### 6. Dispatch later implementation separately

1. Do not implement adapter code from this Phase A contract.
2. Dispatch future implementation only under a sealed deliverable with explicit write scope.
3. Keep each target-specific writer, parser, harness, fixture, or schema change inside its approved deliverable scope.
4. Preserve all unresolved target behavior as `TBD` until the later deliverable has source and validation evidence.

Candidate downstream dispatch notes, not dependency declarations:

| Future work item | Candidate owner surface from current sources | Gate before dispatch |
|---|---|---|
| Concrete schema layout | DEL-17 target implementation deliverables or DEL-10 adapter framework work, as later scoped. | JSON Schema 2020-12 and package/profile contract alignment confirmed. |
| Runtime/API surface and permission taxonomy | DEL-10-01 public API/plugin boundary and DEL-10-02 import/export adapter framework, as later scoped. | Runtime grants, sandbox capability approval, filesystem/network/process access, and package layout remain `TBD` until named by later authority. |
| Sample adapter and adapter test harness | Later adapter SDK implementation tranche under PKG-17 or PKG-10. | Invented or rights-cleared fixtures only; no implementation from this Phase A contract. |
| Target-specific source dossiers | DEL-17-01 source-basis workflow or a later target-specific source-intake deliverable. | Public, official, project-owned, user-provided-with-rights, or otherwise rights-cleared source basis admitted. |
| Target-specific fixtures and validation tests | Later target implementation deliverables such as native JSON, CAEPIPE MBF, PCF, GLB/glTF, or additional adapters. | Fixture rights, target version basis, loss-report obligations, and validation evidence path recorded. |

## Verification

For this Phase A deliverable, verification consists of:

- four-document structure check for `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`;
- manual source-grounding review against local `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, DEL-17-01, DEL-17-02, project governance docs, and the export interoperability plan;
- write-scope review confirming only authorized DEL-17-09 files were edited;
- boundary review confirming no implementation code, schemas, public API promise, compatibility claim, target support claim, protected standards data, proprietary examples, release claim, code-compliance claim, formal-validation claim, or professional claim was introduced.
- acceptance signoff record for these reviews remains `TBD`: reviewer role, signoff format, and required approval artifact are not named by the available sources.

## Records

This Phase A run should leave:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_STATUS.md` safe state update from `OPEN` to `INITIALIZED` when applicable
- `_run_records/TASK_RUN_*.md`

Future target-admission runs should add records only under their own sealed write scope. Expected decision artifacts remain `TBD` unless a later brief names them; candidates include a target registry record, source-basis admission record, validation checklist record, and run record. `MEMORY.md` is intentionally not updated in this phase.
## D-41 R5 T2A canonicalization check (2026-07-12)

Before accepting adapter package hashes, execute the exact-byte/fixed-hash vector, stable-order test, mutation test, schema/fixture validation, and explicit no-JCS assertion.

## D-41 R5 T4 PDU-004 hold check

Inventory only fields and categories emitted by the current builder. Record distinct mechanics/rule-check categories and reviewer/signoff/approval fields as absent and owner-unselected. Do not add placeholder schema fields or normalize existing evidence into a readiness or acceptance record.
