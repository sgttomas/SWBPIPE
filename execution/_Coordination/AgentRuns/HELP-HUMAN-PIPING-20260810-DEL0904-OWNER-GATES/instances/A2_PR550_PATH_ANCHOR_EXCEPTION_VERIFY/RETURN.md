# Structured Return — A2-PR550-PATH-ANCHOR-EXCEPTION-VERIFY

## Terminal verdict

`PASS — EXACT HASH-BOUND HISTORICAL-CONTROL REPAIR INDEPENDENTLY VERIFIED`

The completed V1 verifier brief remains byte- and Git-blob-identical. The
candidate portability policy appends exactly one lawful, hash-bound
`control_path_exception`, clears the PR #550 path-anchor defect, preserves the
tokenized V2 terminal-verifier provenance, and changes no execution meaning.
No unrelated repository-side drift was found.

## Identity and parentage

| Field | Value |
| --- | --- |
| RunID | `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES` |
| ParentInstanceID | `WORKING_ITEMS-A1-PR550-PATH-ANCHOR-REPAIR` |
| ChildInstanceID | `A2-PR550-PATH-ANCHOR-EXCEPTION-VERIFY` |
| Agent form | fresh non-delegating ephemeral Agent 2 verifier |
| Branch | `codex/piping-del0904-owner-gates-20260810` |
| Original accepted HEAD | `1613ebfae29634242cf140d55b7309e945ba43f1` |
| Accepted author return SHA-256 | `741c031758a963fa4f5e9a3ac0984b2923287194199943bc3576b870260556dd` |

## Exact delta and preservation proof

- Frozen V1 launch brief SHA-256:
  `d937f558ee2b38d6d1458790de5efd2d987d9a2ce584ba9cfec2869bebca4a41`.
- Frozen V1 launch brief Git blob:
  `f52c0bf13f4ece6ed2631a0a3c7c941c7a6451d5`.
- Candidate policy SHA-256:
  `67a7868278e775cc00de53087029e6290af3682554122d563a3af5dbaacb84b9`.
- Candidate policy Git blob:
  `9a9b2b9efcc4ce68d0903fa2dd8d0a7886c825d9`.
- The ten `historical_role_overrides` are field-identical and order-preserved.
  The four prior `control_path_exceptions` are field-identical and
  order-preserved. The exception count changes only from four to five.
- The one appended entry has the required repo-relative V1 path, exact frozen
  SHA-256, `entry_type: control_path_exception`, `role: CONTROL`, exact reason,
  and exact authority from the sealed author brief.
- `git diff HEAD` contains only the eight-line append to
  `projects/chirality-piping/validation/portability_policy.json`; every other
  repair record is a newly created file inside the frozen V6 run scope.
- Amendment V5 and the V1/V2 launch, return, and status records all match
  `HEAD`. The original 166-file branch payload from pinned base remains
  committed at exact HEAD and is not rewritten by this repair candidate.

## Required verification matrix

| Requirement | Result | Evidence |
| --- | --- | --- |
| Branch, HEAD, staging, ignored inventory | `PASS` | Branch and original HEAD are exact; staged paths 0; ignored paths 0. |
| V1/V2 provenance preservation | `PASS` | V1 brief exact at the required SHA-256/blob; V1 and V2 provenance files have zero diff from `HEAD`; Amendment V5 remains exact. |
| Exact policy delta | `PASS` | Schema 1 parses; all prior fields and ordering are identical; exactly one required exception is appended. |
| Live path-anchor validator | `PASS` | Pre-return scan checked 1,299 surfaces with findings 0, unacknowledged control 0, active unclassified 0, policy issues 0, acknowledged control 5. |
| Formerly failing live baseline | `PASS` | `test_live_gen8_semantic_portability_invariants`: 1/1 passed. |
| Focused policy tests | `PASS` | `test_surface_roles.py` plus `test_validate_path_anchors.py`: 51/51 passed. |
| Full practitioner-harness suite | `PASS` | 349/349 passed in 25.35 seconds. |
| Harness self-check | `PASS` | Exit 0; no BLOCK findings. Existing REVIEW/WARN/INFO facts are outside this repair delta. |
| Candidate hygiene and contracts | `PASS` | Candidate whitespace, `git diff --check`, claims-language (269 files), Receipt 99 contract, and policy JSON validity pass. |
| Exact containment | `PASS` | Pre-return candidate inventory has six entries, all in the exact V6 repair scope; this verifier adds only its `RETURN.md` and `STATUS.json`, producing the exact eight-path repair delta. |
| Prior owner-gate state | `PASS` | Receipt 99, owner ruling, final handoff, TM-PIP-037 closure manifest, DEC-025 summary, DEL-09-04 status, and all other tracked bytes match `HEAD`. |
| Current-main collision check | `PASS WITH NO-FETCH CAVEAT` | Locally observed `origin/main` is `43f89f96b6beaac87a448e281be512818eec8a60`; pinned base is its ancestor; both complete Piping tree objects are `36b91d558cde9277112a70cde8fa6b017cb8b07d`; Piping path delta and repair-path overlap are both zero. |

## Test execution and side-effect controls

Tests used Python 3.13.14 with pytest 9.1.1,
`PYTHONDONTWRITEBYTECODE=1`, Python `-B`, pytest
`-p no:cacheprovider`, and unique `--basetemp`/`TMPDIR` locations beneath an
external `mktemp -d` root. No test cache, bytecode, generated artifact, or
temporary output was directed into the repository. The successful commands
were:

- exact live GEN-8 baseline test: 1 passed;
- focused surface-role/path-anchor policy suites: 51 passed;
- full `tools/practitioner_harness` suite: 349 passed;
- `tools/practitioner_harness/harness.py self-check`: exit 0;
- path-anchor, whitespace, claims-language, receipt, JSON, hash, blob, and Git
  containment/currentness checks: all passed.

## Currentness caveat and non-effects

No fetch was authorized or performed. The locally observed `origin/main`
therefore establishes collision freedom only at the recorded local tracking
ref; CHANGE must refresh/revalidate integration currentness before export or
merge under its own authority.

This candidate changes only portability classification for one immutable
completed control. It does not alter execution history, the C-B/V-D/O-B/MR-A
owner rulings, TM-PIP-037 closure, Receipt 99, DEC-025 evidence, DEL-09-04
lifecycle, public-comparison selection, page promotion, release, reliance,
professional approval, or any other governed effect.

## Attestation

- I did not delegate.
- I did not repair author output or edit any historical V1/V2 record.
- I did not stage, commit, push, merge, fetch, rebase, reset, clean, delete,
  install dependencies, or access the network.
- I wrote only this verifier's `RETURN.md` and `STATUS.json`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
