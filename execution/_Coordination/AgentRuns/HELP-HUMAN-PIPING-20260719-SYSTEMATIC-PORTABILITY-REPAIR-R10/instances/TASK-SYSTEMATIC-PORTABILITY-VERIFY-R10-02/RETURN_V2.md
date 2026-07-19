# TASK Return V2 — Systematic Portability Final Adversarial Verifier R10-02

## Terminal Result

- `RUN_STATUS: SUCCESS`
- Objective verdict: `COMMIT-SAFE`
- `ControlSurface: FILE + BRIEF_AMENDMENT_002.md`
- `TaskProfile: NONE`
- `TaskSkill: NONE`
- `ScopePath: {REPO_ROOT}`
- `WriteAuthorization: RUN_RECORD_ONLY` — this managed instance's versioned
  `RETURN_V2.md` and `STATUS_V2.json` only
- `ToolPolicyCompliance: N/A` — no tool allowlist was declared

The V2 candidate satisfies the owner-approved systematic portability repair.
The V1 BLOCK remains correct immutable history; its defect is repaired by an
exact registered evidence-shape policy, not by another broad exception. The
candidate is eligible for R10 fan-in and Receipt 57. This verdict does not
stage, commit, push, accept DEL-09-04, or authorize R11 by itself.

## V1 Block Closure

The generic evidence-token regex was removed. Evidence classification is now
limited to these exact, case-sensitive governed shapes:

- `RETURN.md` and `RETURN_V<n>.md`, where `<n>` starts at 1 with no leading
  zero;
- `STATUS.json` and `STATUS_V<n>.json` under the same version rule;
- `HANDOFF_STATE.md`, `RUN_RECORD.md`, `INTERRUPTION_RECORD.md`, and
  `TOOL_ERROR_RECORD.md`;
- exact `_run_records/` directory structure;
- the four hash-bound P1 migration overrides.

Control matching executes before `_run_records/` evidence classification.
Thus `_run_records/LAUNCH_BRIEF.md` remains `CONTROL`.

Every V1 adversarial name independently classified `UNCLASSIFIED` and is
covered by a fixture that proves a machine path produces
`ABS_PATH_IN_UNCLASSIFIED_SURFACE`:

- `NOT_A_RETURN.md`
- `RETURN_INSTRUCTIONS.md`
- `HANDOFF_INSTRUCTIONS.md`
- `SECRET_SUMMARY.md`
- `ARBITRARY_RESULT.json`
- `UNREGISTERED_MANIFEST.yaml`

Case variants, version zero, leading-zero versions, cross-type extensions,
backup extensions, unknown extensions, absolute paths, dot segments,
duplicate separators, and backslashes also fail closed or are rejected.

## Ledger and Semantic Invariants

- Policy ledger: enabled; exactly four historical role overrides and three
  control exceptions; zero issues.
- Every entry has a normalized project-contained path, exact entry type and
  role, non-empty reason and authority, current machine-path hit, present
  regular-file target, and matching whole-file SHA-256.
- Live self-check portability findings: none.
- Piping semantic fact:
  `unacknowledged_control=0; active_unclassified=0; policy_issues=0; acknowledged_control=3`.
- App-dev and PEC semantic facts also report zero unacknowledged controls,
  active unclassified artifacts, and policy issues.
- Aggregate severity-count and exact GEN8 full-tree path baselines are absent;
  no equivalent brittle pin was found.
- Observable self-check severities remain `REVIEW=3`, `INFO=15`, `WARN=6`,
  and `NOT_APPLICABLE=2`; these unrelated totals are not portability gates.

## Sweep Disposition

- Superseded and non-admitted V1 sweep:
  `validation/evidence/sweeps/SWEEP_20260719T193438Z_dca98da8527f-dirty.json`,
  SHA-256 `367bc963039af0c6b74aec19273e3e781fd395a3ef64ab4c57c798c4d9dcd564`.
  It remains byte-exact historical evidence and is explicitly marked
  `INVALIDATED_BY_POST_SWEEP_CODE_CHANGE` in the governing amendment and
  versioned author record.
- Final admitted post-V2 sweep:
  `validation/evidence/sweeps/SWEEP_20260719T195631Z_dca98da8527f-dirty.json`,
  SHA-256 `4ac56dda2b2c5169f083f5f010166492aa0586fda622bd618fd5b9282324fa4a`.
- The admitted sweep ran from `2026-07-19T19:56:31Z` through
  `2026-07-19T19:59:36Z`, reports overall `pass`, and contains exactly the five
  expected passing surfaces: Cargo crate sweep, repository Python pytest,
  desktop Vitest, desktop Playwright E2E, and desktop production build.
- Its command inventory contains no install, download, fetch, or network
  command. No sweep was executed by this verifier.
- All implementation, test, documentation, policy, and Git-attribute files
  have modification times at or before the admitted sweep finish. Current V2
  hashes match the pre-sweep author bindings. Later amendments and terminal
  AgentRuns records are contained schema evidence outside code-under-test.

## Validation Evidence

- Focused plus full practitioner-harness and path-validator tests:
  `316 passed`.
- Full piping pytest: `505 passed`.
- Claims-language validator: PASS, `262` files.
- Path-anchor validator: PASS, `597` live surfaces before this terminal pair.
- Piping receipt and instruction-entrypoint validators: PASS.
- Controlled tracked/untracked JSON and JSONL parse: PASS, `652` JSON and `4`
  JSONL files before this terminal pair.
- `git diff --check` and `git diff --cached --check`: PASS.
- Independent raw-evidence proof: reproduction stderr ending in two newlines
  passed `git diff --cached --check`; malformed authored Markdown failed with
  exit `2` and a trailing-whitespace finding.
- Cleanup merge `525ef0903e68b536ff5b22f985263ca737a67986` is an ancestor of
  source commit `dca98da8527fc118d9bbdcc1e88ccdc7c96b863d`.
- Candidate delta before this terminal pair: `35` paths, all inside the
  amendment-complete fence, including exactly the two governed R10 sweeps.
- R3, R7, R8, P1, R9, all three existing DEL-09-04 reproduction bundles,
  DEL-09-04 status and memory, and `LOOP_RECEIPTS.md` have no Git diff.
- Every R10 control record present before this terminal pair used placeholders
  or repo-relative paths and contained zero machine-path hits.

## Key Hashes

- shared classifier: `bb046f041ed33e0d7b4cee797aa0da52920cc5fd98eb95fbb087f795b580f2be`
- adversarial test module: `e30f16608ffad2e3f3aa90490190215d631e278ff042d27c83d0fd3b025e5b9a`
- classifier documentation: `e56c6c7f07aa5d988f0c95915fa77d2adf8fd3068a6792ed437aa96ef94d21dd`
- portability ledger: `b95d1a12db73557224148719209805ec3c1c9641c4a3857bce440a1aee82a9c9`
- self-check: `18042b68eaade87c0e46449a9dfa552e9947f9fe49b66f62fe5baa0e6695a5f2`
- coordination check: `066d69d61a410cf1e14309e58c07e059aa397d44eb38754aad0d248cc152254f`
- path-anchor validator: `c23387180f4dc6b7d0f3744d70e4a9df77d34fce70613aa4950ff8d35b3f98ed`
- piping Git attributes: `789c2961a1eb692099c4d90b912de046de219a3e524821f3e38954e4ad65e891`
- preserved verifier V1 return:
  `666b15dad82aebea3ecfac15c6e4792862085810392d1246277326f4ad7bde8c`
- preserved verifier V1 status:
  `8aecf7c30aea5b4e7ac0bcd30a000d8695a283bd672d26931bc81bbb8fc1242d`

## Tools Used

- `python3 -m pytest`
- `python3 tools/validation/validate_claims_language.py`
- `python3 tools/validation/validate_path_anchors.py`
- `python3 tools/validation/validate_piping_loop_receipts.py`
- `python3 tools/validation/validate_instruction_entrypoints.py`
- direct read-only Python probes of classification, semantic facts, policy
  entries, hashes, timestamps, sweep JSON, JSON/JSONL, containment, and raw
  evidence Git behavior
- `git diff`, `git status`, `git ls-files`, `git merge-base`, `rg`, and
  `shasum`

## Outputs Produced

- This terminal `RETURN_V2.md`.
- This instance's terminal `STATUS_V2.json`.

## Missing

- none

## Needs Human Ruling

- none for R10 repair fan-in. Receipt 57 and the local repair commit remain
  parent/CHANGE responsibilities.

## Dependency Notes

- `COMMIT-SAFE` applies only to the systematic portability repair and its R10
  evidence fence.
- R11 must still begin from the clean committed repair source, revalidate
  offline prerequisites, and obey its separate sealed no-network/no-install
  authority.

## Proposed Changes

- none

## Preserved Boundaries

No repair, stage, commit, push, merge, network operation, install, download,
five-surface sweep, reproduction, lifecycle transition, receipt append,
acceptance, promotion, publication, or external effect was performed.
