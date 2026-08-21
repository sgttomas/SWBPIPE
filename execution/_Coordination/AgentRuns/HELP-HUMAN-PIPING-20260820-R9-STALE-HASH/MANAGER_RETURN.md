# WORKING_ITEMS manager return — PKG-05 / DEL-05-04 N1

## Verdict

`IMPLEMENTATION_COMPLETE` / `VALIDATED_UNCOMMITTED_CHANGES`

N1 closes the exact DEL-05-04 PDU-037 runtime stale-hash residual in product
code and deterministic native/Wasm evidence. Mandatory independent review is
PASS with zero actionable findings. Publication closeout is not complete:
`FINAL_CLEAN_COMMIT_DEC025_REQUIRED`.

## Coverage and accepted outputs

- Authoritative runtime: `core/model_operations/operation_applier/src/lib.rs`
  now gates a supplied claimed model hash before preview/application.
- Meaningful comparison basis: exact `sha256` / `rfc8785_jcs` /
  `model_payload` metadata and a lowercase 64-hex `sha256:` value.
- Matching: truthful `claimed_model_hash_matches_current_backend_model` and
  ordinary validate/apply behavior.
- Stale/mismatch: stable blocking
  `OP-CLAIMED-MODEL-HASH-MISMATCH`, blocked apply, no applied model/hash, no
  user-application acceptance reuse.
- Malformed: stable blocking
  `OP-CLAIMED-MODEL-HASH-METADATA-INVALID` with schema blocked.
- Unsupported comparison metadata: stable blocking
  `OP-CLAIMED-MODEL-HASH-METADATA-UNSUPPORTED` with schema blocked.
- Absent/null claim: existing before-state staleness behavior and truthful
  no-claim binding status preserved.
- Output contract: `schemas/operation_outcome.schema.json` and its focused
  schema test carry the five truthful terminal statuses.
- Cross-engine evidence: case 15 proves a matching claim; new cases 79–81
  prove stale, malformed, and unsupported claims in native, adapter/Wasm, and
  direct-Wasm lanes. Both runners enforce exactly 81 cases. Original accepted
  cases 01–03 are byte-identical to HEAD.
- Deliverable effect: the exact `_STATUS.md ## Remaining` item is removed;
  lifecycle remains `IN_PROGRESS`. MEMORY and the deliverable-local run record
  carry implementation evidence and boundaries.

## Validation

| Check | Result |
|---|---|
| Focused Rust claimed-hash gate | PASS — 1 |
| Full operation-applier crate | PASS — 76 unit + 1 canonical hash + 2 corpus |
| Focused operation-outcome schema pytest | PASS — 5 |
| Wasm build + focused adapter/direct-Wasm parity | PASS — 171 |
| Full desktop Vitest | PASS — 539 |
| Desktop production build | PASS |
| Full piping pytest with pinned `requirements-dev.txt` in isolated uv environment | PASS — 902 |
| Repository practitioner-harness self-check | PASS — exit 0; pre-existing out-of-scope findings only |
| Full practitioner-harness pytest | PASS — 350 |
| Original corpus cases 01–03 vs HEAD | PASS — byte-identical |
| `git diff --check` | PASS |
| Fresh TASK + software-code-review | PASS — 100% of 19 frozen files, 19/19 hashes, zero findings |

The first direct local-Python full pytest attempt lacked `jsonschema` and
reported 813 passed / 30 failed / 5 collection errors / 17 skipped. The same
registered surface rerun with the pinned requirements passed 902; this is a
superseded environment-dependency attempt, not an unresolved product failure.

After review PASS, a dirty-candidate DEC-025 invocation was started but stopped
during the cargo surface on `HELP_HUMAN` direction because the required gate is
the clean commit-bound run, not a full dirty-worktree expenditure. It produced
no accepted sweep summary and makes no DEC-025 pass claim.

## Accepted child return

- Child: `task_code_review`
- Role/skill: `TASK + software-code-review`
- Frozen scope: 19/19 files, 100%
- Return: `REVIEW_RETURN.md`
- Status: `REVIEW_STATUS.json`
- Verdict: PASS; zero blocking and zero non-blocking actionable findings.
- Model attribution: inherited GPT-5-based Codex runtime; exact variant not
  exposed; no substitution.

## Notices, decisions, blockers, and waivers

- Notices: none cross-package.
- Decisions: none; N1 v2 was an Agent 0 in-place brief/write-fence amendment,
  not a new node or acceptance change.
- Engineering blockers: none.
- Publication blocker: `FINAL_CLEAN_COMMIT_DEC025_REQUIRED`.
- Waivers: none.
- Human corpus-review disposition: case 15's regenerated expectation and new
  cases 79–81 remain pending their own human corpus-review entry. This does not
  invalidate the runtime/code-review result and is not lifecycle acceptance.

## Derivative and runtime status

- Accepted upstream basis: DAG-009, DEC-020, R5, Receipt-120, activation base
  `cd823be3badd034c86390f2707dcf01952c782f0`, exact DEL-05-04 Remaining item.
- Corpus expectations and AgentRuns records are derivative evidence bound to
  that basis; they do not substitute for decomposition truth or lifecycle
  acceptance.
- Runtime telemetry ledger: `NOT_REQUIRED` for this single-node, non-batch
  activation. Manager/child status and exact checks are recorded directly.

## Requested Agent 0 action

Route CHANGE to create the scoped N1 candidate commit from these validated
uncommitted changes. Then run the complete clean commit-bound DEC-025 sweep,
including host/CI surface 4, and resume closeout only if all five surfaces pass.
Do not treat the interrupted dirty-candidate attempt as gate evidence.

Standard claim fence applies (F-PIP-2; DEC-081 claims taxonomy).
