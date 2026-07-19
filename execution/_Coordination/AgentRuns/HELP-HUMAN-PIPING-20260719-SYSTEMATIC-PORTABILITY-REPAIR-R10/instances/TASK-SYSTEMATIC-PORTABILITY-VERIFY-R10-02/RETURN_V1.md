# TASK Return V1 — Systematic Portability Adversarial Verifier R10-02

## Terminal Result

- `RUN_STATUS: SUCCESS`
- Objective verdict: `BLOCK`
- `ControlSurface: FILE`
- `TaskProfile: NONE`
- `TaskSkill: NONE`
- `ScopePath: {REPO_ROOT}`
- `WriteAuthorization: RUN_RECORD_ONLY` — this managed instance's
  `RETURN.md` and `STATUS.json` only
- `ToolPolicyCompliance: N/A` — no tool allowlist was declared

The candidate is not `COMMIT-SAFE`. The shared classifier still contains a
broad filename-token evidence exemption, so unknown or instruction-shaped
AgentRuns artifacts can be classified as `EVIDENCE` and carry machine paths
without an actionable portability finding. Receipt 57 is not eligible.

## Blocking Discrepancy

`tools/practitioner_harness/surface_roles.py:90-91` defines
`EVIDENCE_TOKEN_RE` over the generic tokens `RETURN`, `HANDOFF`, `STATUS`,
`RESULT`, `SUMMARY`, and `MANIFEST`; lines 137-139 then classify every matching
AgentRuns filename as evidence. Independent probes returned:

| Adversarial filename | Observed role |
|---|---|
| `NOT_A_RETURN.md` | `EVIDENCE` |
| `RETURN_INSTRUCTIONS.md` | `EVIDENCE` |
| `HANDOFF_INSTRUCTIONS.md` | `EVIDENCE` |
| `SECRET_SUMMARY.md` | `EVIDENCE` |
| `ARBITRARY_RESULT.json` | `EVIDENCE` |
| `UNREGISTERED_MANIFEST.yaml` | `EVIDENCE` |

This is a fail-open basename allowlist rather than a structural registry of
governed record shapes. It contradicts the approved requirements that unknown
AgentRuns artifacts fail closed, historical overrides remain the only bounded
migration mechanism, and evidence not be inferred from arbitrary filenames.
It also contradicts the module's own statement at lines 118-120 that only
enumerated structural records are evidence. The existing near-match tests at
`test_surface_roles.py:53-65` cover `RETURNED.md` and a `.bak` suffix but do not
cover token-delimited misleading names.

Required correction before a new verifier run: replace the generic evidence
token regex with exact registered record names or equally bounded structural
schemas, preserve control precedence, and add regression cases for all six
adversarial names above carrying machine-absolute paths.

## Independently Recomputed Passing Evidence

- Candidate delta before verifier terminal records: `23` paths; every path was
  inside the amended author fence, including exactly one new sweep JSON.
- Prior R3, R7, R8, P1, R9 managed paths and all three existing DEL-09-04
  reproduction bundles: `git diff --exit-code HEAD` PASS.
- Cleanup merge `525ef0903e68b536ff5b22f985263ca737a67986` is an ancestor of
  source commit `dca98da8527fc118d9bbdcc1e88ccdc7c96b863d`.
- Shared-consumer wiring exists in self-check, coordination checking, and
  path-anchor validation; no independent replacement classifier was found.
- Ledger structure: exactly four historical role overrides and exactly three
  control exceptions. All seven paths are normalized, present, hash-exact,
  contain a current machine path, and load with zero policy issues.
- Live self-check portability findings: none. Piping semantic fact:
  `unacknowledged_control=0; active_unclassified=0; policy_issues=0; acknowledged_control=3`.
  Overall observable severities were `REVIEW=3`, `INFO=15`, `WARN=6`, and
  `NOT_APPLICABLE=2`; these totals are not pinned by the repaired tests.
- Exact aggregate GEN8 path-set and total-severity pins were removed; unrelated
  evidence growth is tested semantically.
- Full practitioner harness plus path-validator tests: `280 passed`.
- Full piping pytest: `505 passed`.
- Claims-language validator: PASS, `262` files.
- Path-anchor validator: PASS, `586` live surfaces.
- Piping receipt validator and instruction-entrypoint validator: PASS.
- Controlled tracked/untracked JSON and JSONL parse: PASS, `648` JSON and `4`
  JSONL files. The first unrestricted attempt reached ignored `node_modules`
  JSON5-style package configs and was correctly discarded as outside governed
  source; no dependency files were changed.
- `git diff --check` and `git diff --cached --check`: PASS.
- Independent temporary-Git proof: checksum-governed reproduction stdout
  ending in two newlines passed with `diff`, `merge`, and `text` all unset;
  malformed authored Markdown failed with exit `2` and a trailing-whitespace
  finding.
- Exactly one R10 sweep exists:
  `validation/evidence/sweeps/SWEEP_20260719T193438Z_dca98da8527f-dirty.json`;
  SHA-256 `367bc963039af0c6b74aec19273e3e781fd395a3ef64ab4c57c798c4d9dcd564`,
  overall `pass`, and all five expected surfaces `pass`. It truthfully records
  the dirty repair tree. No second sweep was run.
- Every R10 control record present at verification used placeholders or
  repo-relative paths and contained zero machine-path hits.
- DEL-09-04 status and memory, Receipt 56, and the candidate brief remained
  unchanged. `LOOP_RECEIPTS.md` SHA-256 remained
  `2ad81234ee45ce396bd02a254c0a7e687e6b04ea430c6d5dfdf7a67035173dce`.

## Key Candidate Hashes

- shared classifier: `8da35a9db719ed96a4fa8b163fffe0480ed4b63daaa8a70940e6d2867f358c69`
- self-check: `18042b68eaade87c0e46449a9dfa552e9947f9fe49b66f62fe5baa0e6695a5f2`
- coordination check: `066d69d61a410cf1e14309e58c07e059aa397d44eb38754aad0d248cc152254f`
- path-anchor validator: `c23387180f4dc6b7d0f3744d70e4a9df77d34fce70613aa4950ff8d35b3f98ed`
- portability ledger: `b95d1a12db73557224148719209805ec3c1c9641c4a3857bce440a1aee82a9c9`
- piping Git attributes: `789c2961a1eb692099c4d90b912de046de219a3e524821f3e38954e4ad65e891`
- main portability tests: `24db0c2b54fbeda7d36877b9225f8ae96d545ea66b5be2928a0ffbd1212a460b`

## Tools Used

- `python3 -m pytest`
- `python3 tools/validation/validate_claims_language.py`
- `python3 tools/validation/validate_path_anchors.py`
- `python3 tools/validation/validate_piping_loop_receipts.py`
- `python3 tools/validation/validate_instruction_entrypoints.py`
- direct read-only Python probes of `surface_roles`, JSON/JSONL, ledger hashes,
  semantic invariants, containment, sweep structure, and temporary Git behavior
- `git diff`, `git status`, `git ls-files`, `git merge-base`, `git check-attr`,
  and `shasum`

## Outputs Produced

- This terminal `RETURN.md`.
- This instance's terminal `STATUS.json`.

## Missing

- A bounded classifier implementation that cannot infer evidence from
  arbitrary token-bearing filenames.
- Adversarial regression coverage for misleading evidence-token filenames.

## Needs Human Ruling

- none; the accepted policy already decides the discrepancy. Repair and a
  fresh adversarial verifier are required before Receipt 57.

## Dependency Notes

- This BLOCK prevents R10 fan-in, Receipt 57, a repair commit, and the sealed
  R11 reproduction.
- No implementation repair, stage, commit, push, merge, install, download,
  network operation, reproduction, lifecycle transition, or external effect
  was performed.

## Proposed Changes

- Replace broad evidence-token matching with an explicit, reviewable registry
  of exact governed evidence record shapes.
- Add fail-closed tests for the six adversarial names above, then dispatch a
  fresh verifier against the complete corrected R10 record set.
