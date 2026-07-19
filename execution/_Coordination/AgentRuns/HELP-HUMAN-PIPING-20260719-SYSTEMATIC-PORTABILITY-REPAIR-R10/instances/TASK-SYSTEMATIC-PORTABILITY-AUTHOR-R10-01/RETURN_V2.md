# TASK Return V2 — Systematic Portability Author R10-01

## Terminal Result

- `RUN_STATUS: SUCCESS`
- Objective verdict: `READY_FOR_ADVERSARIAL_VERIFICATION`
- Existing sweep applicability: `INVALIDATED_BY_POST_SWEEP_CODE_CHANGE`
- `ControlSurface: FILE + BRIEF_AMENDMENT_002.md`
- `TaskProfile: NONE`
- `TaskSkill: NONE`
- `ScopePath: {REPO_ROOT}`
- `WriteAuthorization: ALLOWED_WRITE_TARGETS`
- `ToolPolicyCompliance: N/A`

The verifier V1 block was correct. Author remediation V2 removes the broad
evidence-token inference and adds the complete requested adversarial coverage.
The original author `RETURN.md` and `STATUS.json`, verifier `RETURN_V1.md` and
`STATUS_V1.json`, and the existing sweep remain byte-identical.

## Amendment 002 Acknowledgment

`PLAN_AMENDMENT_002.md` and `BRIEF_AMENDMENT_002.md` were read and applied.
The same author identity was re-opened. No delegation occurred. No second
sweep was invoked or created.

## Remediation

- Removed generic evidence tokens `RETURN`, `HANDOFF`, `STATUS`, `RESULT`,
  `SUMMARY`, and `MANIFEST` from filename inference.
- Evidence now requires an exact registered name: `RETURN.md`,
  `RETURN_V<n>.md`, `HANDOFF_STATE.md`, `STATUS.json`, `STATUS_V<n>.json`,
  `RUN_RECORD.md`, `INTERRUPTION_RECORD.md`, or `TOOL_ERROR_RECORD.md`; the
  only directory-shaped evidence rule is exact `_run_records/`.
- Versioned names are case-sensitive, start at version 1, reject leading zero,
  and bind return records to Markdown and status records to JSON.
- Control classification now executes before `_run_records/` evidence handling,
  preserving mandatory control precedence.
- Misleading names, case variants, extension variants, arbitrary results or
  manifests, and non-normalized paths remain `UNCLASSIFIED` or are rejected.

## Adversarial Regression Evidence

Each verifier example is asserted `UNCLASSIFIED` and, with a machine path,
produces `ABS_PATH_IN_UNCLASSIFIED_SURFACE`:

- `NOT_A_RETURN.md`
- `RETURN_INSTRUCTIONS.md`
- `HANDOFF_INSTRUCTIONS.md`
- `SECRET_SUMMARY.md`
- `ARBITRARY_RESULT.json`
- `UNREGISTERED_MANIFEST.yaml`

Additional tests cover exact registered records, case mismatches, cross-type
versioned suffixes, version zero, leading-zero versions, backup and unknown
extensions, absolute paths, dot/dot-dot segments, duplicate separators,
backslashes, and a control record nested beneath `_run_records/`.

## Validation Evidence

- Focused role/ledger/coord/path/live set: `81 passed`.
- Full practitioner harness plus path-validator tests: `316 passed`.
- Full piping pytest: `505 passed`.
- Final piping self-check: no findings.
- Piping semantic fact: `unacknowledged_control=0; active_unclassified=0; policy_issues=0; acknowledged_control=3`.
- Path-anchor validator: PASS across `590` live surfaces.
- Claims language: PASS, `262` files scanned.
- Piping receipt contract and instruction-entrypoint validator: PASS.
- `git diff --check`: PASS.
- All six verifier examples independently probed as `UNCLASSIFIED`.
- Prior R7, R8, P1, and reproduction-bundle paths remain unchanged from HEAD.
- Sweep inventory remains `282`; exactly one R10 sweep exists and no second
  sweep was created.

## Existing Sweep Applicability

The existing sweep
`projects/chirality-piping/validation/evidence/sweeps/SWEEP_20260719T193438Z_dca98da8527f-dirty.json`
remains truthful immutable evidence of the V1 candidate and retains SHA-256
`367bc963039af0c6b74aec19273e3e781fd395a3ef64ab4c57c798c4d9dcd564`.
It is not applicable as final evidence for V2 because
`surface_roles.py`, its adversarial tests, and documentation changed after the
sweep. Amendment 002 prohibited a second sweep, so a replacement sweep requires
subsequent parent authority and must not be inferred from the non-sweep checks.

## Key V2 Hashes

- shared classifier: `bb046f041ed33e0d7b4cee797aa0da52920cc5fd98eb95fbb087f795b580f2be`
- adversarial test module: `e30f16608ffad2e3f3aa90490190215d631e278ff042d27c83d0fd3b025e5b9a`
- practitioner-harness documentation: `e56c6c7f07aa5d988f0c95915fa77d2adf8fd3068a6792ed437aa96ef94d21dd`
- preserved author V1 return: `7a0ae216001d1c7be8d0f82d061a0236764a108a42d18fa651526b16b37c07ee`
- preserved author V1 status: `f911aa6aa246a56b707be02be45461429f1dc80ec1716e3778f828d1c4380809`
- preserved verifier V1 return: `666b15dad82aebea3ecfac15c6e4792862085810392d1246277326f4ad7bde8c`
- preserved verifier V1 status: `8aecf7c30aea5b4e7ac0bcd30a000d8695a283bd672d26931bc81bbb8fc1242d`

## Tools Used

- `python3 -m pytest`
- `python3 tools/practitioner_harness/harness.py`
- `python3 tools/validation/validate_claims_language.py`
- `python3 tools/validation/validate_path_anchors.py`
- `python3 tools/validation/validate_piping_loop_receipts.py`
- `python3 tools/validation/validate_instruction_entrypoints.py`
- direct read-only Python role, semantic, JSON, and sweep-count probes
- `git diff`, `git status`, `sha256sum`

## Outputs Produced

- Corrected shared classifier and expanded adversarial tests.
- Updated classifier documentation.
- This versioned `RETURN_V2.md` and `STATUS_V2.json`.

## Missing

- A post-remediation five-surface sweep applicable to V2; explicitly forbidden
  in this amendment and therefore not produced.

## Needs Human Ruling

- none; parent workflow authority must sequence any replacement sweep before
  claiming complete R10 fan-in.

## Dependency Notes

- Verifier V2 may assess the corrected implementation and non-sweep evidence.
- R10 cannot rely on the V1 sweep as final V2 evidence. Receipt 57, repair
  commit finalization, and R11 remain gated by parent disposition of the stale
  sweep evidence.

## Preserved Boundaries

No stage, commit, push, merge, network, install, download, reproduction,
lifecycle transition, receipt, DEL-09-04 state change, or external effect
occurred. All prior managed records and reproduction bundles remain immutable.
