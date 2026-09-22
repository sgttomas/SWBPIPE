# CFB-01 — PKG-00 architecture-basis records: layer responsibilities, transaction boundaries, job control, migration status

**Candidate brief (H2). Not executed.** Area: Architecture basis (PKG-00). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Author the per-layer responsibility map and module ownership table for the eleven named layers (DEL-00-02); write the transaction-boundary definition for solve runs and adapter calls and record cancellation/progress (or synchronous execution) for report and export jobs (DEL-00-03 REQ-03-03/03-04); carry or justify per-record migration status for stored rule-pack and library records and locate or add save/open round-trip coverage for rule-pack references, diagnostics and provenance (DEL-00-04). Mostly document work on an architecture-basis record; any code change is limited to the named round-trip test.

## Affected claims

12 claim rows on 3 deliverable(s): DEL-00-02, DEL-00-03, DEL-00-04.

Classes (portion in this brief / class total): T6-C02 12/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-01"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-00-02:AB#normative-requirements.s01` | T6-C02 | NONE | — | — | As FG-DEL-00-02-01: the AB-00-02 row names the layers but defines no per-layer responsibilities. |
| `DEL-00-02:AB#normative-requirements/REQ-02-01` | T6-C02 | NONE | — | — | Define each of the eleven layers' responsibilities (the SPEC section 1 overview covers GUI, services, domain core and adapters only). |
| `DEL-00-02:AB#purpose.s01` | T6-C02 | NONE | — | — | Author the layer-to-responsibility map and module ownership table (or cite where they live); dependency-direction and adapter rules already exist. |
| `DEL-00-02:CONTEXT#description` | T6-C02 | NONE | — | — | As FG-DEL-00-02-01: module ownership and per-layer responsibilities. |
| `DEL-00-03:AB#normative-requirements/REQ-03-03` | T6-C02 | NONE | — | — | Write the transaction-boundary definition (the anticipated application-services document was never created) covering solve runs and adapter calls. |
| `DEL-00-03:AB#normative-requirements/REQ-03-04` | T6-C02 | NONE | — | — | Add or locate cancellation and progress for report and export jobs, or record that they run as synchronous commands. |
| `DEL-00-03:AB#open-holds-and-routed-questions.s02` | T6-C02 | NONE | — | — | Record the open elements found on REQ-03-02 to REQ-03-04 where the owning loop tracks work. |
| `DEL-00-03:AB#purpose.s02` | T6-C02 | NONE | — | — | Close or record the gaps on REQ-03-02, REQ-03-03 and REQ-03-04 (envelopes on storage commands, transaction boundaries, job control for report and export). |
| `DEL-00-03:CONTEXT#description` | T6-C02 | NONE | — | — | As REQ-03-03 and REQ-03-04: transaction boundaries and job control beyond solve. |
| `DEL-00-04:AB#normative-requirements/REQ-04-02` | T6-C02 | NONE | — | — | Carry migration status for stored rule-pack and library records (they store a versioned document but no per-record migration status), or record why the store ledger suff… |
| `DEL-00-04:AB#normative-requirements/REQ-04-04` | T6-C02 | NONE | — | — | Confirm or add product save/open round-trip coverage for rule-pack references, diagnostics and provenance metadata. |
| `DEL-00-04:AB#purpose.s02` | T6-C02 | NONE | — | — | Close or record the open elements on REQ-04-02 and REQ-04-04, and name the provider-expansion residual. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-00-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-00/DEL-00-02/DEL-00-02_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`, `projects/chirality-piping/docs/SPEC.md`, `projects/chirality-piping/api/api_boundary_contract.yaml`.
- DEL-00-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-00/DEL-00-03/DEL-00-03_forward.csv`; ImplementationEvidence cited: `core/model_operations/operation_applier/src/lib.rs`, `apps/desktop/src-tauri/src/lib.rs`, `apps/desktop/src-tauri/src/atomic_report_package_save.rs`, `schemas/headless_runner.schema.yaml`, `apps/desktop/src/features/solve/SolvePanel.tsx` ….
- DEL-00-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-00/DEL-00-04/DEL-00-04_forward.csv`; ImplementationEvidence cited: `apps/desktop/src-tauri/src/lib.rs`, `apps/desktop/src-tauri/src/model_document_migration.rs`, `schemas/rule_pack.schema.yaml`, `core/project_persistence/service.py`, `apps/desktop/src/services/hashService.ts` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Each of the eleven AB-00-02 layers has a stated responsibility and owning module, or a cited location.
- A transaction-boundary and job-control statement exists for solve, adapter, report and export operations, and the DEL-00-03 open-holds list names the elements still open.
- A save/open round-trip test (or located existing test) covers rule-pack references, diagnostics and provenance metadata.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8/T12 reading needs an owner or review decision.

## Notes and open views

- REQ-03-02 (diagnostics envelope on storage commands) is carried by CFB-14, not here.
- PKG-00 lifecycle (SEMANTIC_READY) questions are A5; these briefs do not change lifecycle state.

## Dependencies

A5 (lifecycle context only), CFB-14.

