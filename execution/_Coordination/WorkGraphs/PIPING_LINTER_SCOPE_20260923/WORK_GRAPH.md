# Work graph — DEL-08-05 public lint surface boundary

## Intent and selected route

- Stable run identity: `PIPING_LINTER_SCOPE_20260923`.
- Owner steering: demonstrate one limited, useful Piping development undertaking; Agent 0 selected the DEL-08-05 default public-surface boundary correction on 2026-09-23. This graph is an execution route, not a scope, acceptance, lifecycle or release decision.
- Intended result: the protected-content linter scans only configured public paths by default, while its established renderer, PDF and explicit-file CLI callers retain their intended coverage. Private surfaces stay opt-in. The run ends after a substantive implementation/evidence PR and one bounded closeout/MEMORY PR merge, each with actual-candidate review and checks.
- Scope: `DEL-08-05` source, focused tests, affected caller configuration, graph and records. Leave release-candidate scan execution and owner sign-off, CI severity selection, adapter runtime binding, protected standards, manual, and issued baselines untouched.
- DAG route: approved `execution/_DAG/DAG-011/` gives `DEL-08-05` the owner. It lists `DEL-01-02` as a PENDING prerequisite and `DEL-08-01` as a PENDING interface. This slice uses the existing IP/private-data boundary and existing report-renderer interface; it does not change their maturity or claim full deliverable completion.

### Warranted authorization rule

`DEL-08-05` Scope of Work CLM-010/REQ-001 and its verification fixture require only authorized public template/example paths to be scanned by default; CLM-006 allows other *explicitly configured* public report surfaces, and CLM-033 OQ-08-05-002 names `LintConfiguration` plus typed `LintTarget` as the current mechanism. A public surface tag alone is insufficient. `DEL-01-02` preserves human IP/provenance review and quarantine; this path selection makes no legal disposition.

Default `lint_targets` will require a public-typed target inside a configured normalized root, or an exact configured target path. A private-typed target requires the existing explicit `scan_private_surfaces` opt-in. Component-aware matching excludes sibling prefix aliases and traversal. The report renderer constructs `report_renderer://section/...` and `report_renderer://document` in-memory outputs, and the PDF emitter constructs `pdf_emitter://document`; their caller configuration will include those virtual prefixes. The CLI's documented positional file arguments are explicit local file selection. The DEC-058 runner feeds it text/JSON files selected from owner-specified artifact classes AC-1–AC-6; the public export tool feeds selected staged files. The CLI will configure only those exact named file paths after reading them, with no blanket absolute-file root. This preserves existing caller contracts without conferring authority from a public tag or absolute spelling alone. The owner-only release scan/signature remains held.

## Deliverable scope

| Deliverable / basis | Existing surface | Trial outcome | Work |
|---|---|---|---|
| `DEL-08-05`, SOW-043 / `ScopeOfWork.md` | Rust linter `LintConfiguration.public_surface_roots` is populated but ignored by `should_scan_target`; skipped count includes all rejects. | Enforce configured target boundary, preserve explicit callers, add regression evidence and honest skip accounting. | W1, V1, P1, C1, M1, F1 |

## Work

| ID / outcome | Scope and owner | Needs | Completion check | State / result |
|---|---|---|---|---|
| W1 path boundary | WORKING_ITEMS; linter and three direct callers, focused tests | Accepted scope and caller contracts above | Out-of-root public tag skipped; component alias/traversal excluded; legitimate virtual and exact CLI targets scanned; private opt-in preserved | COMPLETE in working candidate; Rust linter, renderer and PDF tests pass; exact file CLI and private opt-in regressions added. |
| V1 independent verification | WORKING_ITEMS coordinates fresh-context reviewer; tests and DEC-025 sweep | Frozen W1 candidate | Focused and applicable full checks, clean-candidate sweep, independent review with no unresolved blocker | ACTIVE; focused tests pass. Full project pytest comparison and clean-candidate sweep/review remain. |
| P1 substantive PR | W1/V1 plus graph/pointer and needed slice documentation | W1 and V1 | PR merged under Root standing Git authorization; no full DEL acceptance claim | PLANNED |
| C1 bounded closeout | WORKING_ITEMS; `DEL-08-05` against Scope of Work, DAG and governance | P1 integrated | `chirality-root:bundled:workflow:bounded-reconciliation` comparison; warranted edits or supported no-change; Task Management only if exceptional intake qualifies | PLANNED |
| M1 run index | `DEL-08-05/MEMORY.md` | C1 | Terse date/result/central evidence and PR pointer | PLANNED |
| F1 final PR | C1/M1 and graph readiness | Final candidate review and required checks | Final PR merged, actual merge verified | PLANNED |

## Current state and recovery

- Checked basis: clean `codex/piping-loop-trial` at main `2a9b00fe9e67a4d98017833dace867a01867c591`; approved DAG-011 per `_DAG/_LATEST.md`.
- Selected method: `chirality-root:bundled:workflow:construct-local-work-graph` (`workflows/construct-local-work-graph/WORKFLOW.md`, SHA-256 `060f7153fb8a1b75825b23a799c58de45b6538b5e95d758f777079925b121c47`). Git closeout uses `.agents/skills/chirality-change/SKILL.md`, SHA-256 `2b490e172436417896c1cd25dbcd543c676e3473aa58b7663985d75785ff7dba`.
- Supplied instruction origins/hashes at start: Root `AGENTS.md` `1bb670ca339a990b153cf033dac2d8e29ca71bdea0accee4200e6dd1feed3d57`; Piping `AGENTS.md` `60d9af6492e415f68ba91176530316545ddc25e4c0fddc6fb3192beb10d10100`; `agents/AGENT_WORKING_ITEMS.md` `9ae4bea25bd95750a6878a9d53fbbbd7cd058d72c652a36baf4aa90601799665`; init prompt `b4ad6c6d4bdbd52d76f12361457c454ddd55785f425b4756680e18aeda8d304a`; loop entry pre-pointer `ccdf2daf326a169f4296511b31be4daa5e366d54aca9a5e725b79d9f72794492`.
- Next safe action: commit W1, run required checks against its clean candidate, obtain independent review, then integrate P1. The 2026-09-23 full Python run before W1 commit had seven failures; one was caused by the new CLI regression source triggering the release-tool-only guard and was repaired with a passing focused rerun. Six other exact test IDs reproduced in a detached clean `2a9b00fe` worktree with the declared Python dependencies: `test_local_first_user_control_default_is_explicit`, four `test_release_readiness_script` cases, and `test_live_architecture_basis_contract_passes`. The base also had 15 headless-runner setup errors from unavailable registry DNS. They are existing selected-base failures, not evidence that this candidate's required profile passed. CI and clean-candidate checks still decide merge readiness.
- Local/unmerged work: early graph/pointer commit `4fa66679613a44734209b562a07cfa75ce677132`; W1 code, schema, fixture and tests are uncommitted pending frozen-candidate verification. No Type 2 workers or shared native resources.
- Graph maintainer and integration owner: WORKING_ITEMS Type 1 under HELP_HUMAN Agent 0. No Type 2 dispatch.
