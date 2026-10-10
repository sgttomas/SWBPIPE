# TP-EXPORT-006-B Contract Chain And TBD Review

## Inputs Read

- `AGENTS.md`
- `agents/AGENT_TASK.md`
- `docs/CONTRACT.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
- DEL-17-01 through DEL-17-09 four-document kits: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`
- DEL-17-01 through DEL-17-09 `_SEMANTIC_LENSING.md`
- DEL-17-01 through DEL-17-09 `MEMORY.md`
- Recent DEL-17 `_run_records` where useful for validation, parent fan-in, and remaining-TBD context
- DEL-17-01 `Source_Basis_Register.md`
- DEL-17-01 `CAEPIPE_Question_Dossier.md`
- Read-only coordination status evidence where useful for DEL-17 readiness context

## Verdict

PASS_WITH_WARNINGS

## Findings

| FindingID | Severity | DeliverableID | Location | Description | ProposedDisposition |
|---|---|---|---|---|---|
| TP-EXPORT-006-B-F001 | WARNING | DEL-17-06 | `Procedure.md` Records; `MEMORY.md`; local file inventory | `Procedure.md` says `Dependencies.csv` and `_SEMANTIC_LENSING.md` were intentionally not produced in the Phase A task, but the deliverable now contains those artifacts and `MEMORY.md` records their generation and validation. This is artifact-inventory drift, not a contract contradiction about export behavior. | In a future authorized document cleanup, update DEL-17-06 records/closeout wording to distinguish the initial P1/P2 phase from later semantic/lensing/dependency passes. Not blocking for DEL-17-03. |
| TP-EXPORT-006-B-F002 | WARNING | DEL-17-07 | `Procedure.md` Records; `Specification.md` Scope | `Procedure.md` says the deliverable should retain or produce PCF subset profile, target file, manifest, sidecar, diagnostics, loss report, and fixture provenance records, while the current Phase A scope says no code/schema implementation occurs. The surrounding procedure mostly frames those as later implementation work, but the Records wording is not phase-qualified. | In a future authorized document cleanup, qualify those records as future implementation records unless explicitly produced by a sealed implementation tranche. Not blocking for DEL-17-03. |
| TP-EXPORT-006-B-F003 | WARNING | DEL-17-08 | `Procedure.md` Records; `MEMORY.md`; local file inventory | `Procedure.md` lists only the initial Phase A run record and future records as `TBD`, while `MEMORY.md` records later semantic/lensing/P3/dependency work and multiple run records. This is audit-inventory drift after later passes, not an export-contract conflict. | In a future authorized document cleanup, refresh the Phase A records section to match post-P3 artifact reality. Not blocking for DEL-17-03. |
| TP-EXPORT-006-B-F004 | WARNING | DEL-17-09 | `Procedure.md` Records; `MEMORY.md` | `Procedure.md` says `MEMORY.md` is intentionally not updated in this phase, but `MEMORY.md` records TP-EXPORT-005 population and validation. This appears to be stale phase wording after parent/orchestrator work. | In a future authorized document cleanup, align the procedure records wording with actual memory/run-record practice. Not blocking for DEL-17-03. |
| TP-EXPORT-006-B-F005 | INFO | DEL-17-01 | `_SEMANTIC_LENSING.md` C-001/D-001/E-001 | DEL-17-01 lensing already recorded minor downstream-consumer and maintenance-check improvements: DEL-17-06 consumer coverage, broad downstream wording, and admitted-source freshness checks. These are useful source-basis hygiene items but no notable conflict or parse error was recorded. | Carry as source-basis maintenance candidates. Do not resolve inside this review note. Not blocking for DEL-17-03 because DEL-17-03 consumes DEL-17-01 only for boundary discipline, not CAEPIPE/PCF/glTF target behavior. |
| TP-EXPORT-006-B-F006 | INFO | DEL-17-05 | `_SEMANTIC_LENSING.md`; `Guidance.md` Conflict Table | DEL-17-05 has one recorded conflict about command-line versus batch-mode invocation profile and an unresolved parser-section coverage question. The four-document kit correctly preserves these as TBD/human-ruling items and does not convert them into compatibility or validation claims. | Keep in the consolidated TBD register. Blocks DEL-17-05 implementation details only, not DEL-17-03. |

## Consolidated TBD Register Candidates

| TBDID | DeliverableID | Category | Question | BlockingForDEL1703 | SuggestedDisposition |
|---|---|---|---|---|---|
| TBD-17-01-001 / CQ-17-01-001 | DEL-17-01 | CAEPIPE profile | Confirm first supported CAEPIPE version/profile and citation target. | No | Carry for DEL-17-04/05; do not let it become a DEL-17-03 native JSON blocker. |
| TBD-17-01-002 / CQ-17-01-002 | DEL-17-01 | MBF subset | Confirm initial MBF record families and required fields for first deterministic writer subset. | No | Carry for DEL-17-04 implementation brief and MBF writer acceptance. |
| TBD-17-01-003 / CQ-17-01-003 | DEL-17-01 | Stable identity | Confirm stable ID carrying inside MBF versus sidecar-only mappings. | No | Carry for DEL-17-04; DEL-17-03 can proceed with direct JSON identity or package-local ID map per DEL-17-02. |
| TBD-17-01-004 / CQ-17-01-005 | DEL-17-01 | CAEPIPE CSV parser | Confirm which CSV result sections are stable and useful for automated parser coverage. | No | Carry for DEL-17-05/06 parser and regression evidence; keep public fixtures invented or rights-cleared. |
| TBD-17-01-005 / CQ-17-01-006 | DEL-17-01 | PCF subset | Define conservative PCF subset and translator-default rejection/warning rules. | No | Carry for DEL-17-07; do not infer support from translator behavior. |
| TBD-17-01-006 | DEL-17-01 | GLB/glTF identity | Define GLB/glTF review-geometry identity metadata and sidecar policy. | No | Carry for DEL-17-08; native JSON identity policy remains governed by DEL-17-02/03. |
| TBD-17-03-001 | DEL-17-03 | Native JSON schema | Which concrete JSON schemas will bind package members? | No, if included in the sealed DEL-17-03 implementation scope | Treat as a required implementation-brief closure item before schema/API freeze. |
| TBD-17-03-002 | DEL-17-03 | Hashing | Which hash canonicalization helper will package writer code use? | No, if included in the sealed DEL-17-03 implementation scope | Bind to existing project canonical JSON/JCS-compatible policy before deterministic hash tests. |
| TBD-17-03-003 | DEL-17-03 | Fixtures | Which invented fixtures will exercise native JSON round trips? | No, if included in the sealed DEL-17-03 implementation scope | Define invented fixtures and provenance in the implementation tranche. |
| TBD-17-03-004 | DEL-17-03 | Writer authority | Which concrete schema and writer binding source will authorize native JSON implementation? | No, if included in the sealed DEL-17-03 implementation scope | Make the sealed implementation brief name write scope, schema target, package writer target, and validation evidence. |
| TBD-17-04-001 | DEL-17-04 | CAEPIPE MBF profile | Which CAEPIPE version/profile is the first target? | No | Blocks MBF target support wording only. |
| TBD-17-04-002 | DEL-17-04 | CAEPIPE MBF subset | Which MBF record families and required fields are in the first subset? | No | Blocks MBF writer acceptance only. |
| TBD-17-04-003 | DEL-17-04 | CAEPIPE MBF stable IDs | Can MBF carry stable canonical IDs directly? | No | Use sidecar fallback unless direct carrying is source-confirmed. |
| TBD-17-04-004 | DEL-17-04 | Diagnostics severity | Which unsupported entities block export versus produce non-blocking diagnostics? | No | Decide in MBF profile implementation tranche; keep uncertainty loss-reported. |
| DEL-17-05-PH-001 / CQ-17-01-004 | DEL-17-05 | External execution | What configuration surface and invocation profile should be used for a user-owned CAEPIPE executable? | No | Carry as DEL-17-05 implementation gate; public CI must skip without executable. |
| DEL-17-05-PH-002 | DEL-17-05 | Run directory | What exact filenames or manifest links define the run directory record shape? | No | Define in later harness/schema work. |
| DEL-17-05-PH-003 | DEL-17-05 | Parser coverage | What parser coverage register shape and section evidence threshold are required? | No | Carry for parser implementation; section support requires source or fixture evidence. |
| DEL-17-05-PH-004 | DEL-17-05 | Skip evidence | What exact evidence fields prove skip-without-executable behavior? | No | Carry for harness test implementation. |
| TBD-17-06-FIELDS | DEL-17-06 | Stress-neutral package layout | Exact CSV table names, JSON properties, manifest layout, ID-map layout, loss-report layout, units representation, comparison tolerances, and validation thresholds remain unresolved. | No | Carry for stress-neutral schema/writer tranche; keep comparison semantics diagnostic/audit-only until DEL-14 basis is consumed. |
| TBD-17-07-PROFILE | DEL-17-07 | PCF profile | First PCF target profile/version, entity-family classifications, translator-default handling, support/restraint preservation, and unsupported behavior severity remain unresolved. | No | Carry for PCF implementation tranche; current candidate subset is not a support claim. |
| TBD-17-08-METADATA | DEL-17-08 | Review geometry | Direct metadata versus sidecar policy, service boundary, geometry coverage, coordinate transform, canonical ID family list, viewer behavior, and fixture policy remain unresolved. | No | Carry for GLB/glTF profile/writer tranche; review geometry only. |
| TBD-17-09-SDK | DEL-17-09 | Adapter SDK | Concrete schemas, runtime/API surface, permission taxonomy, sandbox/grant records, target-admission artifacts, source-basis intake fields, validation checklist fields, and signoff format remain unresolved. | No | Carry for adapter SDK implementation/source-intake tranche; no additional target support claim is made now. |

## DEL-17-03 Readiness Assessment

No blocker was found that prevents recommending DEL-17-03 as the next implementation tranche. The contract chain is coherent:

- DEL-17-01 supplies source-basis and target-claim boundaries, with CAEPIPE/PCF/glTF unresolved behavior kept as TBD.
- DEL-17-02 supplies the common export package/profile/stable-ID/manifest/loss-report contract and requires unresolved target behavior to stay visible.
- DEL-17-03 consumes both and defines a native project-owned JSON package with manifest, model payload, stable ID map, loss report, validation report, and diagnostics.
- DEL-17-04 through DEL-17-09 generally carry DEL-17-01 and DEL-17-02 forward without silently resolving target-specific questions or making compatibility, release, code-compliance, formal-validation, or professional-acceptance claims.

DEL-17-03 is also the least target-coupled next implementation target. Its remaining TBDs are implementation-shaping questions for native schema/writer/hash/fixture work, not external vendor/source blockers. CAEPIPE MBF, external harness/CSV, PCF, GLB/glTF, and adapter SDK work all depend on target-specific TBD closure or later target-profile decisions; native JSON can establish the project-owned package substrate those later adapters consume.

## Recommendation

Recommend DEL-17-03 as the safest next implementation target, provided the sealed implementation brief explicitly scopes schema/member layout, writer binding, canonical JSON/JCS-compatible hashing, invented fixtures, validation checks, and the DEL-17-02 mandatory loss-report/stable-ID/manifest behavior. Do not resolve CAEPIPE/PCF/glTF/adapter SDK target TBDs in the DEL-17-03 tranche except as carried loss-report or boundary metadata.
