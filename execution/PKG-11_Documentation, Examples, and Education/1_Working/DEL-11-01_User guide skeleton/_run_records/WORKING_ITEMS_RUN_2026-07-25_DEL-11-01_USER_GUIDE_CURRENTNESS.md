---
doc_id: WORKING-ITEMS-RUN-2026-07-25-DEL-11-01-USER-GUIDE-CURRENTNESS
doc_kind: execution.run_record
status: COMPLETED_PENDING_FINAL_VERIFIER
created: 2026-07-25
run_id: HELP-HUMAN-PIPING-20260725-CANDIDATE-BRIEFS-R18
instance_id: WI-PKG11-DEL1101-EXECUTION
package_id: PKG-11
deliverable_id: DEL-11-01
candidate_sha256: 15d54c72665085b1b68b03da807b03b0319e5767fc45053025d677e3a0a6a1d0
---

# DEL-11-01 user-guide currentness execution

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Authority and scope

The human project authority adopted
`execution/_Coordination/CANDIDATE_BRIEF_2026-07-25_DEL-11-01_USER_GUIDE_CURRENTNESS.md`
at the SHA-256 above through
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260725-CANDIDATE-BRIEFS-R18/instances/WI-PKG11-DEL1101-EXECUTION/OWNER_ADOPTION.md`.
The adoption requires revision-free pointer-based authority wording, grants a
guide-only exception to the DEL-11-01 Scope of Work, preserves the exact
four-path fence, and permits `DEL-11-01-REM-001` closure only after all
acceptance evidence passes without changing `**Current State:** IN_PROGRESS`.

The exact write fence comprised:

1. `docs/user_guide/index.md`;
2. the DEL-11-01 `_STATUS.md` (residual/history updated only after recovery
   and every remaining acceptance gate passed; `IN_PROGRESS` preserved);
3. this run record; and
4. the companion registered-check JSON.

The two run-record targets were absent at preflight. The guide and status
pre-write SHA-256 values were respectively
`1164ab5c0bed3cfdbce473b3c57ae4a3013d5b7a67961d430695e188cde77bd2`
and
`3af64df6d91e7b7d5f4232e65b296be0f03bdb5dd3097ec37487ed27123f4e97`.

## Accepted sources reviewed

- `execution/_DAG/_LATEST.md` — selects approved active `DAG-008` and records
  `DAG-007` as superseded.
- `execution/_DAG/DAG-008/APPROVAL_RECORD.md` — dependency-authority scope and
  separate lifecycle/residual boundary.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` — accepted decomposition
  surface, architecture basis, open issues, and decisions through `DEC-080`.
- `docs/PRD.md` — adopted product, privacy, reporting, professional-boundary,
  validation, milestone, and open-question basis.
- `docs/claims_registry.md` — `BS-IP`, `BS-ACCEPT`, `BS-VALID`, and
  `BS-MATURITY`.
- `software-workflow.json` and `docs/SOFTWARE_WORKFLOW_PROFILE.md` — affected
  check selection and registered-check contract.
- DEL-11-01 `ScopeOfWork.md`, `_STATUS.md`, `MEMORY.md`, and `_REFERENCES.md` —
  deliverable contract, exact residual, historical context, and guide-only
  exception boundary.
- Each path named in guide section 2, plus the directly relevant current
  implementation/evidence surfaces used below. Every section-2 path existed at
  review time.

## Before/after claim map

| ID | Guide surface | Before | After | Accepted evidence |
|---|---|---|---|---|
| CAM-01 | Opening authority paragraph | Named decomposition revision `0.7` and approved `DAG-007`. | Names the accepted decomposition path and the graph selected by `_DAG/_LATEST.md`, with no revision token. | `_DAG/_LATEST.md` lines 3-9; `DAG-008/APPROVAL_RECORD.md` Owner decision and Approval conditions; owner adoption qualification 1. |
| CAM-02 | Section 2 Solver mechanics row | Omitted `core/solver/sparse_direct` and said the production sparse-solver choice remained `TBD`. | Adds the live sparse module and states its bounded interactive preview/render default role while preserving unmeasured tolerance, release-threshold, and integration TBDs. | `SOFTWARE_DECOMP.md` `DEC-023`, `DEC-026`, `DEC-046`, `DEC-050`, and `DEC-053`; `core/solver/performance_harness/src/lib.rs` sparse-promotion evidence fields and current role strings. |
| CAM-03 | Section 2 GUI workflow row | Said final packaging remained `TBD`. | Records bounded packaged-journey evidence while preserving complete integration, supported-release acceptance, and broader-platform TBDs. | `apps/desktop/SMOKE.md`; `docs/BUILD_AND_RELEASE.md` §§1, 6, and 9; `SOFTWARE_DECOMP.md` `DEC-057`. |
| CAM-04 | Section 2 Reports row | Omitted the renderer and said preview/export runtime remained `TBD`. | Adds `core/reporting/report_renderer` and records the live deterministic HTML renderer, desktop preview, and native report-package save; preserves final styling, hash-bound PDF, portable replay, and release-template TBDs. | `core/reporting/report_renderer/src/lib.rs`; `apps/desktop/src/features/report/RenderedReportPanel.tsx`; `apps/desktop/src-tauri/src/lib.rs`; DEL-08-01 `MEMORY.md` 2026-06-11 and 2026-07-22 entries and current `_STATUS.md`. |
| CAM-05 | Section 3 opening | Treated binary/desktop packaging and release channels as wholly undecided. | Records the accepted v0.1 macOS Apple Silicon `.app`-zip/checksum policy and prospective GitHub Releases target while explicitly stating that no release exists; installation, storage roots, and publication remain TBD. | `SOFTWARE_DECOMP.md` `DEC-057` and `DEC-059`; `docs/BUILD_AND_RELEASE.md` §§1 and 6. |
| CAM-06 | Section 14 sparse/tolerance bullet | Said the production sparse library and tolerance policy were TBD. | Preserves only unmeasured tolerance entries, release-quality thresholds, and hardening beyond bounded sparse-default evidence. | `SOFTWARE_DECOMP.md` `DEC-023`, `DEC-026`, `DEC-046`, `DEC-053`. |
| CAM-07 | Section 14 API/adapter bullet | Treated adapter formats generally as TBD. | Preserves public transport, endpoint, plugin-loader, and target-field gaps while recognizing the accepted export families. | `SOFTWARE_DECOMP.md` `SCA-004`, `OI-004`, `OI-015`, and `AB-00-07`. |
| CAM-08 | Section 14 reporting bullet | Said report preview/export runtime remained TBD. | Preserves final styling, deterministic hash-bound PDF, portable replay, release-template, and redaction-breadth gaps after recognizing the live bounded runtime. | `SOFTWARE_DECOMP.md` `DEC-021`; DEL-08-01 `MEMORY.md` 2026-07-22 entry and `_STATUS.md`; `core/reporting/report_renderer/src/lib.rs`. |
| CAM-09 | Section 14 CI/signing bullet | Treated CI provider and release signing as wholly open. | Records conditional public sanitized-export GitHub Actions and accepted unsigned v0.1 posture; preserves activation, numeric floors, future signing/notarization, and policy details. | `SOFTWARE_DECOMP.md` `DEC-057`, `DEC-059`, `DEC-060`; `docs/BUILD_AND_RELEASE.md` §§1, 7, and 9. |
| CAM-10 | Section 14 contributor/authority bullet | Said maintainer quorum and release authority remained TBD. | Records closed external intake and sole-human-authority quorum/release authority while preserving the future legal instrument and other genuine governance TBDs. | `SOFTWARE_DECOMP.md` `DEC-027` and `DEC-079`. |

No other guide paragraph changed.

## Section 2 current-surface disposition matrix

| Row | Disposition | Evidence and reason |
|---|---|---|
| Model and units | `PRESERVE` | All three schema paths exist. `SOFTWARE_DECOMP.md` `AB-00-04`, `DEC-017`, and SCA-003 preserve SQLite as store/index with canonical JSON/JCS-compatible domain truth. |
| Analysis status | `PRESERVE` | Both schemas and `docs/architecture/analysis_status_semantics.md` exist; the guide vocabulary retains the mechanics/rule/human separation required by `AB-00-03` and PRD §21.3. |
| Solver mechanics | `CHANGE` | `core/solver/sparse_direct` exists; `DEC-023` selects it and `DEC-053` plus the performance-harness implementation establish the bounded interactive default. Residual tolerance/release/integration limits remain explicit. |
| Loads and stress recovery | `PRESERVE` | All four module paths exist. The note remains consistent with PRD §§11-12 and the protected/user-rule data boundary. |
| Libraries and provenance | `PRESERVE` | All five paths exist. The note matches PRD §§18 and 20 and `BS-IP`; no protected or proprietary values were added. |
| Rule packs | `PRESERVE` | All four paths exist. The note correctly states user ownership and explicit inputs/provenance/checksum/privacy controls; it does not repeat the retired grammar TBD. |
| GUI workflow contracts | `CHANGE` | The listed schema/modules/app exist. Current smoke and packaging evidence disproves a blanket packaging TBD, but lifecycle and broader-platform evidence do not support a release-complete claim. |
| Reports and result envelopes | `CHANGE` | All prior paths plus the current renderer exist. The renderer, desktop preview, and native save path are live bounded surfaces; final styling, hash-bound PDF, portable replay, and release-template integration remain unresolved. |
| Privacy and export controls | `PRESERVE` | All four paths exist. The note matches PRD §20 and the local-first/telemetry/redaction policy files. |
| Export interoperability | `PRESERVE` | All schemas and `core/handoff/` exist. The note matches SCA-004 and uses the `BS-VALID` short-variant posture: handoff evidence for external validation, not a validation outcome. |
| Interop and local analysis | `PRESERVE` | All listed paths exist. `AB-00-07`, `OI-004`, `OI-015`, and `DEC-078` preserve the public-transport, plugin, target-coverage, and local-FEA limitations. |

No section-2 row is blocked.

## Section 14 limitation/TBD disposition matrix

| Bullet | Disposition | Evidence and reason |
|---|---|---|
| End-user install/package steps and release channels | `CHANGE` | `DEC-057` selects the v0.1 matrix, artifact shape, unsigned posture, and prospective publication target; `DEC-059` keeps publication/CI activation conditional. The revised bullet preserves execution and installation gaps without erasing the decision. |
| Exact dependency versions and final GUI journey integration/packaging | `PRESERVE` | `SOFTWARE_DECOMP.md` §8.2 and `OI-002` retain exact dependency/platform integration details as implementation-level TBDs; bounded smoke evidence is not release acceptance. |
| Production sparse library, tolerance policy, and release thresholds | `CHANGE` | `DEC-023` resolves the sparse library and `DEC-026`/`DEC-046` resolve policy structure while leaving measured entries and release-quality thresholds open; `DEC-053` bounds the default path. |
| DEC-022 typed AST / DEC-037 writable text syntax | `PRESERVE` | `SOFTWARE_DECOMP.md` `DEC-022` and `DEC-037`, `core/rules/expression_evaluator/README.md`, and the evaluator implementation support the exact current statement. |
| OS storage roots, migration tooling, portable project export/copy | `PRESERVE` | `SOFTWARE_DECOMP.md` §8.2, `OI-011`, `DEC-017`, and `docs/security/local_first_storage_policy.md` keep these distinct from the accepted SQLite/JCS basis. |
| Public API transport, endpoint syntax, adapter formats, plugin loader | `CHANGE` | SCA-004 resolves export families while `AB-00-07`, `OI-004`, and `OI-015` retain public transport, plugin loading, and concrete target-field gaps. |
| Target field coverage, source confirmations, stable-ID carriage, loss taxonomy | `PRESERVE` | `OI-004`, `OI-015`, and `OI-017` preserve these deliverable-local gaps where not already accepted. |
| Report styling/layout, preview/export runtime, redaction UX | `CHANGE` | The bounded renderer/preview/native-save runtime exists; DEL-08-01 current residuals and `DEC-021` preserve the narrower final-policy/replay/release-template gaps. |
| Local FEA handoff format and external-tool execution | `PRESERVE` | `DEC-078` places local FEA implementation post-beta under DEL-10-03; the guidance/schema remain contract surfaces, not a completed external execution path. |
| CI provider, coverage thresholds, release signing, maintainer policy | `CHANGE` | `DEC-059`, `DEC-060`, and `DEC-057` resolve the conditional provider/tooling/unsigned-v0.1 postures while retaining activation, numeric-floor, future-signing, and policy gaps. |
| Contributor legal mechanism, maintainer/release authority, and other governance TBDs | `CHANGE` | `DEC-027` resolves sole-maintainer quorum and release authority; `DEC-079` keeps intake closed and the future legal instrument open. Legal-review authority, security contact, labels, acceptance workflow, and jurisdiction wording remain unresolved. |

No section-14 bullet is blocked. All genuine unresolved matters remain explicit.

## Boundary and structure review

- Required guide sections 1-15 remain present, including setup, project
  creation, modeling, solving, rule checks, results, reports, troubleshooting,
  limitations, and glossary.
- Required status vocabulary remains present:
  `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`,
  `USER_RULE_CHECKED`, `USER_RULE_FAILED`, `HUMAN_REVIEW_REQUIRED`, and
  `HUMAN_APPROVED_FOR_PROJECT`.
- The opening result statement preserves the canonical `BS-ACCEPT` language.
- Export sections preserve the accepted `BS-VALID` posture.
- Public/private and protected-content boundaries remain unchanged. No
  protected standards text, protected table, proprietary example/value,
  private project value, private rule-pack payload, credential, or secret was
  introduced.
- No installation command, dependency version, solver tolerance value,
  release claim, lifecycle claim, or professional acceptance claim was added.

## Checks and containment

The affected-check selector selected exactly `harness-pytest` and
`harness-self-check`.

The first registered-runner attempt failed for operational interpreter
resolution:

| Check | Status | Exit | Evidence |
|---|---|---:|---|
| `harness-pytest` | `FAIL` | 1 | `No module named pytest` |
| `harness-self-check` | `FAIL` | 2 | `PyYAML is required to read domain-engine profiles but is not importable in this interpreter` |

That first attempt did not supply acceptance evidence. Its accidentally
workspace-root-relative output was removed, and the failure was preserved at
the allowed companion JSON path pending the one authorized operational rerun.
No absolute machine interpreter path is retained in this record.

The final normalized recovery evidence is the companion
`WORKING_ITEMS_RUN_2026-07-25_DEL-11-01_USER_GUIDE_CURRENTNESS_CHECKS.json`.

The one owner-authorized operational rerun prefixed `PATH` only for that
runner invocation with the directory of the configured `python3`. It failed
with the same two results:

| Check | Status | Exit | Evidence |
|---|---|---:|---|
| `harness-pytest` | `FAIL` | 1 | `No module named pytest` |
| `harness-self-check` | `FAIL` | 2 | `PyYAML is required to read domain-engine profiles but is not importable in this interpreter` |

Per the adopted stop rule, no further acceptance checks or residual closure
were performed in that executor turn.

One fresh bounded check-recovery child then reran exactly the two registered
checks once using the configured workspace Python directory for that
invocation. It overwrote only the exact checks JSON and returned:

| Check | Status | Exit | Evidence |
|---|---|---:|---|
| `harness-pytest` | `PASS` | 0 | 311 tests passed |
| `harness-self-check` | `PASS` | 0 | registered self-check completed |

Final recovery JSON SHA-256:
`407239b7662f9e8eea78c0b33567487f98d6b3c8d2b5be02d0742f66d595abe7`.
Commands remain normalized to `python3`; no absolute interpreter path is
recorded.

After the status/history update, WORKING_ITEMS completed the remaining
acceptance checks:

- claims-language validation: `PASS` — 268 governed surfaces scanned;
- repository-root `git diff --check`: `PASS`;
- explicit four-path `validate_change_scope.py`: `PASS`, zero violations;
- required guide sections 1-15 and status vocabulary review: `PASS`;
- protected/private-content review: `PASS`; no protected standards content,
  proprietary example/value, private project value, private rule-pack payload,
  credential, or secret was introduced;
- current-state byte-identity check: `PASS` — the exact line remains
  `**Current State:** IN_PROGRESS`;
- guide SHA-256:
  `2cc930d657bed31ba65385f914aaa12528000b8515497b87d9962ae4283dc50f`;
- normalized recovery-check JSON SHA-256:
  `407239b7662f9e8eea78c0b33567487f98d6b3c8d2b5be02d0742f66d595abe7`.

## Residual, state, and unresolved items

`DEL-11-01-REM-001` is satisfied after all candidate acceptance evidence
passed. The deliverable remains `IN_PROGRESS`; no lifecycle transition is
performed. The guide's genuine product, packaging-execution, validation,
governance, and release-policy TBDs remain visible.

Rerun is required if an accepted pointer/source changes, any final check
fails, the four-path containment set changes, either run-record path is
replaced, or the exact current-state line changes.

## Effect

This execution refreshes documentation currentness and satisfies exactly
`DEL-11-01-REM-001`. It does not modify the Scope of Work, MEMORY, references,
decomposition, DAG/pointer, PRD, claims registry, source code, schema, tests,
examples, register, receipt, ruling, or shared R18 integration state. It has
no lifecycle or release effect. Standard claim fence applies (F-PIP-2; claims
taxonomy per DEC-081).
