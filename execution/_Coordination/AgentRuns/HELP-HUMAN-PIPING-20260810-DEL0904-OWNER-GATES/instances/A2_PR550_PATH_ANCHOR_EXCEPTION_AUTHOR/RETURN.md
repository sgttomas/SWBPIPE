# Structured Return — A2-PR550-PATH-ANCHOR-EXCEPTION-AUTHOR

## Terminal verdict

`PASS — EXACT HASH-BOUND CONTROL EXCEPTION AUTHORED AND VALIDATED`

The completed V1 verifier brief remains byte-for-byte unchanged. The project
portability policy gained exactly one `control_path_exception` for that
CONTROL record. The exception acknowledges both machine-local anchors in the
one frozen file while preserving the tokenized V2 verifier as the terminal
acceptance basis.

## Identity and scope

| Field | Value |
| --- | --- |
| RunID | `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES` |
| ParentInstanceID | `WORKING_ITEMS-A1-PR550-PATH-ANCHOR-REPAIR` |
| ChildInstanceID | `A2-PR550-PATH-ANCHOR-EXCEPTION-AUTHOR` |
| Agent form | fresh non-delegating ephemeral Agent 2 author |
| Branch | `codex/piping-del0904-owner-gates-20260810` |
| Accepted HEAD | `1613ebfae29634242cf140d55b7309e945ba43f1` |

## Exact repair evidence

- V1 launch brief SHA-256:
  `d937f558ee2b38d6d1458790de5efd2d987d9a2ce584ba9cfec2869bebca4a41`.
- V1 launch brief Git blob:
  `f52c0bf13f4ece6ed2631a0a3c7c941c7a6451d5`.
- Policy SHA-256 before repair:
  `f0dacfee06bdd4aac177a962f4710edc99fda0e073e53e965c88fb4d238fade4`.
- Policy SHA-256 after repair:
  `67a7868278e775cc00de53087029e6290af3682554122d563a3af5dbaacb84b9`.
- Policy Git blob after repair:
  `9a9b2b9efcc4ce68d0903fa2dd8d0a7886c825d9`.
- Existing historical-role overrides are identical to `HEAD`; existing
  control exceptions are an identical prefix. The control-exception count
  changed only from four to five.
- The appended entry has the required repo-relative path, frozen SHA-256,
  `entry_type: control_path_exception`, `role: CONTROL`, reason, and
  authority. No V1 or V2 record was edited.

## Validation

| Check | Result |
| --- | --- |
| Schema-1 JSON parse and exact-one-entry semantic comparison to `HEAD` | `PASS` |
| `validate_path_anchors.py --text .` | `PASS` — zero findings across 1,298 final surfaces |
| Path-anchor semantic invariants | `PASS` — unacknowledged control 0; active unclassified 0; policy issues 0; acknowledged control 5 |
| Path-anchor and policy unit tests | `PASS` — 51/51 |
| `git diff --check` | `PASS` |
| V1 frozen identity after repair | `PASS` |
| Receipt cursor | `PASS` — Receipt 99 file unchanged, SHA-256 `fa18f6d1e9854a88df96a4b86386652ce7fa41296b1b47a6807e2d8f6ead9c57` |
| Staged paths | `PASS` — zero |
| Ignored paths | `PASS` — zero |

The unit tests ran with Python 3.13.14 and pytest 9.1.1 using `-B`,
`PYTHONDONTWRITEBYTECODE=1`, and `-p no:cacheprovider`; 51 tests passed. The
system developer-tools Python did not provide pytest, so it was not used for
the successful unit-test run. No dependency installation or network access
occurred.

The manager-authored V6 amendment and this instance's sealed launch brief
were pre-existing untracked control inputs at child start. They were read but
not edited. This child wrote only the portability policy and its own return
and status records.

## Exclusions and attestation

- No delegation occurred.
- No historical V1/V2 record, receipt, deliverable, register, owner ruling,
  evidence, lifecycle, or release surface was edited.
- No staging, commit, push, merge, fetch, rebase, reset, clean, deletion, or
  network action occurred.
- This repair changes portability classification only; it does not change the
  frozen run evidence or owner-gate semantics.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
