# Sealed brief — A1-DOCS: SCA-010, the report notice, core text and user-facing documents

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-18 before launch. Role: TASK (Type 2), implementer, working alone. Model requested: Claude Sonnet 5. Mechanism: Claude Code `Agent` tool, general-purpose type, background.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel`; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`; `{DESIGN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`.

## Authority

The owner authorized implementation on 2026-09-18 (`{RUN}/instances/ROOT/ACTIVATION_2026-09-18.md`). The acts you execute are the owner's rulings codified as `DEC-100` to `DEC-105` in `{WORKING_ROOT}/execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (rows near line 690; read them whole before you start, they are the specification of this work) and scope change `{WORKING_ROOT}/execution/_ScopeChange/SCA-010_2026-09-18_1400/`. `DEC-099` is superseded by `DEC-105` and is not executed. `DEC-104` (the Checked mark) is not in this tranche.

## Limits that bind every child of this tranche

- Work only in the git worktree you are started in, on the branch that is checked out. Run no git command that changes state (no add, commit, checkout, stash, reset). ROOT integrates and commits.
- Write only inside your write scope. Two sibling children are writing other paths in the same worktree at the same time; files outside your scope that look dirty are theirs. Do not touch, fix or revert them.
- This tranche is the **visible** rename. The identity layer is a later tranche. Do **not** rename: `openpipestress.*` document-kind or artifact-kind strings; schema `$id` values; npm package names, crate names, binary names, library names; directory or file names; the window global `__openPipeStressUiDiagnosticsV1`; DOM event names; environment variables; `openpipestress_jcs_ijson_v1`; `openpipestress_result_semantics_v0_2`; the `.opsproj` extension; the store filename `openpipestress-projects.sqlite3`; identifiers and test ids that contain `caepipe`. If a string is compared by code, stored in a saved document, hashed, or asserted by a schema `const`, it is identity, not display: leave it and list it in your return.
- Never edit frozen history: `{WORKING_ROOT}/execution/**` (except where your scope names a path), `validation/evidence/**`, `validation/hand_calcs/**`, `validation/witness/**`, `plans/**`, `loop/**`, `provenance/**`, `docs/_history/**`, `docs/_ScopeChange/**`, `apps/desktop/SMOKE.md`.
- Never weaken a test to make it pass. A test that pinned removed or renamed display text moves to the new text; a test of behaviour is not deleted. Never alter picking tolerances, oracles or benchmark limits.
- Copy rules: "Accept", never "Approve", as a control; none of certify, seal, approve, authenticate, comply, compliant or sign-off as a control; Canadian spelling in new prose; no new claims sentence of your own in place of a removed one.
- Type 2: you work alone and do not delegate.

## Write scope

Under `{WORKING_ROOT}`: `docs/**` except `docs/claims_registry.md` (a sibling's), `docs/_history/**` and `docs/_ScopeChange/**` (frozen); `core/**`; `tests/**`; `tools/release/**`; `README.md`, `CONTRIBUTING.md`, `governance/**`; `execution/_ScopeChange/_LATEST.md` only. And `{REPO_ROOT}/.github/ISSUE_TEMPLATE/**` only where a file is the piping project's. Nothing under `apps/`.

## The work

1. **SCA-010, exactly as its bundle says.** Read all five files of the bundle first; `Amendment_Actions.csv` rows A001 to A006 are the edit list. `docs/PRD.md`: 23 of the 24 occurrences of the old name become SWBPIPE; **line 15 stays** (it is a path to a preserved historical file); the header changes per A004 (version 0.3 to 0.4, amended date, authority line, the `**Formerly:**` line, "Working title" becomes "Product name"). `docs/report_notice_template.md` lines 16, 41 and 68. `execution/_ScopeChange/_LATEST.md` moves to SCA-010. Count the replacements and report the count.
2. **The emitted report notice.** `core/reporting/report_renderer/src/lib.rs` near line 719 emits the §19.3 notice: its first word becomes SWBPIPE so the PRD, the template and the renderer agree. Keep the fragment "decision-support software" that the claims lint anchors on. Update Rust tests that pin the notice or a report title; report titles drop "(Technical Preview)".
3. **`DEC-100` in core.** `core/solver/nonlinear_supports/src/lib.rs` (near 343 and 1380) and `core/solver/nonlinear_integration/src/lib.rs` (near 671) carry an acceptance short variant. First determine whether the string is written into a result document that a schema or a fixture pins. If it is free diagnostic prose, remove the sentence and move its test. If it is a pinned data contract, leave it and report it. **Do not change** `core/rules/rule_check_runner/src/lib.rs` `PROFESSIONAL_BOUNDARY_NOTICE` or `schemas/**`: that is a schema `const` and stays.
4. **`DEC-105` and `DEC-101` in documents and tools.** Remove the maturity sentence from user-facing documentation. The name becomes SWBPIPE in prose in: `docs/user_guide/index.md`, `docs/README.md`, `docs/BUILD_AND_RELEASE.md`, `docs/RELEASE_NOTES_TEMPLATE.md`, the five `docs/*/index.md` guides, `docs/validation_manual/index.md`, the project `README.md`, `CONTRIBUTING.md`, the piping issue templates. **Not now:** the governance documents' titles and bodies (`DIRECTIVE.md`, `CONTRACT.md`, `SPEC.md`, `TYPES.md`, `PLAN.md`, `PROFESSIONAL_BOUNDARY.md`, `IP_AND_DATA_BOUNDARY.md`, `AGENTIC_DEVELOPMENT_WORKFLOW.md`) take the name at their next amendment; `docs/validation_manual/cases/**`, `docs/architecture/**`, `docs/security/**` and `LICENSE.md` likewise. Commands, paths, package names and binary names inside documents are identity and stay (for example `openpipestress-runner`).
5. **`DEC-103` item 9 in the user guide.** `docs/user_guide/index.md` (near lines 65, 273, 274): the vendor's name goes; the export is "the export of a `.mbf` model batch file". Schema file names in a table are identity: if the only way to keep the row is to show a file name containing the vendor's name, describe the row without the file name and report it. Lines 43 and 186 (the `HUMAN_APPROVED_FOR_PROJECT` row) are pinned by `tests/test_user_guide_status_wording.py` and are preserved.
6. **Packaging tools and Python tests.** `tools/release/package_release_artifact.py` reads the product name and identifier from `tauri.conf.json`, which a sibling is changing to `productName` "SWBPIPE", `identifier` `com.swbpipe.desktop`, window title "SWBPIPE". Update any literal expectation and `tests/test_release_packaging_script.py` (near 211, 431, 433) to those values. `tools/release/generate_app_icon.py` prose only.

## Checks you run (from `{WORKING_ROOT}`)

- `python3 -m pytest -q tests -n auto --dist loadscope` (fall back to no `-n` if xdist is absent). A failure in `tests/test_claims_language_surface.py` is expected until ROOT integrates the siblings' work; report it and classify it. Every other failure is yours to explain.
- `cargo test` in each crate you changed, offline (`CARGO_NET_OFFLINE=true`).
- Your own searches proving the PRD count, that line 15 is unchanged, and that the three notice surfaces agree on the first word.

## Return


Your final message is your return. ROOT retains it. Include: the model you are; every file you changed, grouped; every check you ran with its exact command and result; every string you deliberately left because it is identity, with file and line; anything in your scope you could not do and why; anything you noticed outside your scope that a sibling or ROOT must handle. End with the line: Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
