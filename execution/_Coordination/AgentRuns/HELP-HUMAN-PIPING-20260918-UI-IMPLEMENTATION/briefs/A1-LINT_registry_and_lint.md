# Sealed brief — A1-LINT: the claims registry and the root claims lint

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-18 before launch. Role: TASK (Type 2), implementer, working alone. Model requested: Claude Fable 5.1. Mechanism: Claude Code `Agent` tool, general-purpose type, background.

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

- `{REPO_ROOT}/tools/validation/validate_claims_language.py`
- `{REPO_ROOT}/tools/validation/test_validate_claims_language.py`
- `{WORKING_ROOT}/docs/claims_registry.md`

The owner directed the root tool edit explicitly (activation record §2, "Root lint"). ROOT writes the tranche manifest.

## The work

1. **Registry** (`docs/claims_registry.md`), as governed registry acts with a dated amendment note citing the DEC rows:
   - `BS-MATURITY` is retired (`DEC-105`): the entry stays as a retired entry that says it is retired, by which ruling, and that no live surface carries the sentence; ruled history is not rewritten.
   - `BS-ACCEPT` (`DEC-100`): its placement clause is retired for product surfaces; the canonical text and its four short variants are no longer placed in the product. Keep what `DEC-100` preserves: the "not authoritative"-family retirement stays enforced; PRD §19.3, §5.9, §21.2, §21.3, `docs/PROFESSIONAL_BOUNDARY.md` and `docs/CONTRACT.md` are untouched. Governance surfaces (the `ScopeOfWork.md` files) may still carry a registered variant, so decide from the registry's own structure whether the texts remain registered for non-product surfaces, and say what you decided.
   - `BS-IP` canonical text takes the name SWBPIPE (`DEC-101` (ii)); short variants that are name-free are unchanged.
   - §2 gains `DEC-102`'s display-form table with its three rules (token reachable in place; authority domain always shown; no label outside the table; `ENGINEER_ACCEPTED` stays reserved). Authority domains are in `docs/SPEC.md` §4.3.
2. **Lint** (`validate_claims_language.py`): `REGISTERED_TEXTS` follows the registry exactly (it is a hand transcription; the docstring says so). Remove the `MISSING_MATURITY_BANNER` anchor. Consider, and implement if sound, the inverse guard that `DEC-105` implies: a finding when the retired maturity sentence appears on a scanned live surface. Do the same reasoning for the acceptance sentence on product source (`apps/desktop/src` non-test files) only, because governance surfaces may keep it. Keep `RETIRED_PHRASE` and the litany rule unchanged. If the old `BS-IP` sentence must still suppress a litany finding on an unrenamed governance surface, keep it registered as a former text and say so.
3. **Lint tests**: update the constants and cases; the missing-banner case becomes a case for whatever replaced it. Every new guard has a firing case and a clean case.

## Checks you run

- `python3 -m pytest -q tools/validation/test_validate_claims_language.py` from `{REPO_ROOT}`.
- `python3 tools/validation/validate_claims_language.py` from `{REPO_ROOT}`. It scans the live tree, which your siblings are changing at the same moment: report its findings and classify each as yours, a sibling's pending work (the maturity sentence at `apps/desktop/src/App.tsx` near line 3282, `BuildReadinessPanel.tsx:75`, `ExportReviewPanel.tsx:658`; acceptance-sentence placements), or a real defect. ROOT re-runs it after integration.

## Return


Your final message is your return. ROOT retains it. Include: the model you are; every file you changed, grouped; every check you ran with its exact command and result; every string you deliberately left because it is identity, with file and line; anything in your scope you could not do and why; anything you noticed outside your scope that a sibling or ROOT must handle. End with the line: Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
