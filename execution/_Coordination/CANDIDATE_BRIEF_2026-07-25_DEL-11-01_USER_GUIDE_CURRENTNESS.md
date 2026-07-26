---
doc_id: CANDIDATE-BRIEF-2026-07-25-DEL-11-01-USER-GUIDE-CURRENTNESS
doc_kind: coordination.candidate_brief
status: CANDIDATE_NOT_ADOPTED
created: 2026-07-25
run_id: HELP-HUMAN-PIPING-20260725-CANDIDATE-BRIEFS-R18
node_id: M1
manager_instance: WI-PKG11-DEL1101
package_id: PKG-11
deliverable_id: DEL-11-01
selection_authority: HUMAN
frozen_basis: 2f8d35ceb30da734ca6dff24dcab36dded8c9b35
---

# CANDIDATE / NOT ADOPTED — DEL-11-01 user-guide currentness

This is a proposal-only execution brief. It is not an adopted brief, does not
release implementation, and grants no write authority. Owner adoption and the
gates below are prerequisites to any future execution.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 1. Bounded objective

Prepare a future documentation-only refresh that:

1. replaces the stale active-guide authority declaration naming `DAG-007` with
   wording grounded in the then-current accepted decomposition surface and
   approved DAG pointer;
2. backchecks the active guide's current-repository and `TBD` statements
   claim-by-claim against accepted live evidence, changing only statements that
   are demonstrably stale;
3. preserves the guide structure, public/private and protected-content
   boundaries, canonical claims language, genuine unresolved decisions, and
   historical records; and
4. closes `DEL-11-01-REM-001` only if every acceptance check passes while
   preserving `DEL-11-01` at `IN_PROGRESS`.

The objective is the exact residual recorded in
`execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/_STATUS.md`
under `## Remaining`. It is not a general documentation rewrite or a product
maturity pass.

## 2. Accepted basis for owner review

The future executor must re-read these live surfaces after adoption; this
candidate records their R18 preparation state but does not freeze mutable
working truth:

- `execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/ScopeOfWork.md`
  — `OUT-001`, `AC-001`, `VER-001`, `UG-REQ-001` through `UG-REQ-010`,
  and the current read-only treatment of the repository guide;
- the same deliverable's `_STATUS.md`, `MEMORY.md`, and `_REFERENCES.md`;
- `docs/user_guide/index.md`;
- `execution/_DAG/_LATEST.md` and
  `execution/_DAG/DAG-008/APPROVAL_RECORD.md`;
- `execution/_Decomposition/SOFTWARE_DECOMP.md`;
- `docs/PRD.md`;
- `docs/claims_registry.md`; and
- `software-workflow.json`.

Current evidence supporting a candidate:

- `_STATUS.md` records the exact residual to refresh the active guide against
  accepted current authority without overstating maturity or rewriting
  history.
- `docs/user_guide/index.md` currently names decomposition revision `0.7` and
  approved `DAG-007`.
- `execution/_DAG/_LATEST.md` names `DAG-008` as the approved active graph
  authority and identifies `DAG-007` as superseded.
- `DAG-008/APPROVAL_RECORD.md` limits DAG-008's effect to dependency authority;
  brief adoption, execution, lifecycle action, and residual removal remain
  separate acts.
- `docs/PRD.md` is the adopted PRD authority as amended through SCA-007.
- `docs/claims_registry.md` is the single source for product-surface boundary
  language and is enforced by the claims validator.

## 3. Stale-claim and preservation map

| ID | Live surface | R18 observation | Future disposition |
|---|---|---|---|
| SCM-01 | `docs/user_guide/index.md` authority paragraph | Names approved `DAG-007`, while `_DAG/_LATEST.md` names approved `DAG-008` and marks DAG-007 superseded. | `CHANGE`, after owner adoption, to the approved current pointer. |
| SCM-02 | The same paragraph's decomposition revision token | Names `0.7`; the live decomposition metadata says `0.9`, its amendment/gate text says `v0.10`, and the DAG-008 approval record cites decomposition truth `0.7`. | `STOP` until the owner names the accepted exact revision token or approves revision-free path-based wording. Do not infer or silently normalize. |
| SCM-03 | `docs/user_guide/index.md` current-surface table and `TBD` statements | These are currentness-sensitive but are not proved stale merely by DAG-008. | `BACKCHECK`; preserve every claim lacking accepted contrary evidence. No broad search-and-replace. |
| SCM-04 | `docs/user_guide/index.md` acceptance wording | The result boundary at the opening section matches `BS-ACCEPT`; the export section carries the accepted external-validation posture. | `PRESERVE` unless the current registry requires an exact listed variant. |
| SCM-05 | DEL-11-01 `ScopeOfWork.md` and `MEMORY.md` current declarations | They contain older DAG/revision tokens, but the live SOW says dated MEMORY/history stay unchanged and says the repository guide is read-only in the setup contract. | `OUT OF FENCE`; do not edit. Route any separate contract/current-declaration repair through its owning workflow. |
| SCM-06 | Project `AGENTS.md` D-56 current-authority wording | R18 treats D-56 as a separate owner packet and blocked node, independent of M1. | `OUT OF SCOPE`; do not read, edit, adopt, or treat D-56 as an M1 prerequisite. |

## 4. Proposed future execution write fence

If and only if the owner adopts this brief and resolves Section 7, the future
execution may write exactly these repository paths:

1. `projects/chirality-piping/docs/user_guide/index.md`;
2. `projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/_STATUS.md`;
3. `projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-25_DEL-11-01_USER_GUIDE_CURRENTNESS.md`; and
4. `projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-25_DEL-11-01_USER_GUIDE_CURRENTNESS_CHECKS.json`.

The `_STATUS.md` permission is limited to the exact
`DEL-11-01-REM-001` residual and one dated history entry. `Current State:
IN_PROGRESS` must remain byte-identical. The run-record paths are new-file
targets; if either already exists at execution time, stop and re-mint the
evidence paths rather than overwrite.

No other repository write is allowed. Check tools may use ordinary transient
state outside the repository, but generated repository output must remain
inside the four exact targets above.

## 5. Exclusions

- No edit to either root or project `AGENTS.md`; D-56 remains separate.
- No edit to `ScopeOfWork.md`, `MEMORY.md`, `_REFERENCES.md`, `_REVIEW.md`,
  decomposition, DAG artifacts or pointers, PRD, claims registry, registers,
  receipts, rulings, plans, source code, schemas, tests, examples, or product
  assets other than the one active guide named in the write fence.
- No blanket deletion of `TBD`, planned/future wording, limitations, or
  historical text.
- No new installation command, supported-platform assertion, dependency
  version, solver tolerance, release channel, API behavior, external-format
  behavior, or product feature claim without accepted file evidence.
- No protected standards content, proprietary example, private project value,
  or user-owned rule-pack payload.
- No lifecycle transition or release effect. The candidate and any later
  documentation refresh do not adopt D-56 or authorize Git or network action.

## 6. Required future method and acceptance evidence

The future execution return must include:

1. a before/after claim map keyed to each changed guide paragraph, with the
   accepted source path and section for every replacement;
2. evidence that SCM-01 is corrected to the approved current DAG pointer;
3. the owner's SCM-02 disposition, quoted or linked exactly;
4. a currentness review of every row in `## 2. Current Repository Surfaces` and
   every bullet in `## 14. Known Limitations And Current TBDs`, recording
   `PRESERVE`, `CHANGE`, or `BLOCK` with evidence;
5. confirmation that required guide sections and status vocabulary remain
   present and that no unsupported maturity claim was introduced;
6. claims-language validation against `docs/claims_registry.md`, preserving
   applicable `BS-ACCEPT` and `BS-VALID` wording;
7. a protected/private-content review and evidence that no prohibited content
   was introduced;
8. exact changed-path containment, final hashes for all four allowed targets
   that exist, and `git diff --check`;
9. registered-check JSON at the exact checks path; and
10. a run record that reports sources, changes, checks, unresolved items,
    rerun triggers, and the no-lifecycle/no-release effect.

`DEL-11-01-REM-001` may be removed or marked satisfied only after items 1-9
pass. Failure leaves the exact residual active and records the blocker without
changing lifecycle state.

## 7. Adoption gate

Execution remains `STOP` until the human project authority performs one
explicit adoption act that:

1. adopts this candidate by exact path and reviewed bytes;
2. authorizes the four-path future write fence;
3. explicitly resolves the live SOW clauses that otherwise keep
   `docs/user_guide/index.md` read-only for this deliverable, or routes a prior
   scope amendment through the owning workflow;
4. resolves SCM-02 by naming an accepted decomposition revision or approving
   revision-free path-based authority wording; and
5. confirms whether successful evidence may satisfy
   `DEL-11-01-REM-001` without changing `IN_PROGRESS`.

Silence, merge, file presence, DAG-008 approval, or this candidate's manager
validation does not satisfy the adoption gate.

## 8. Required checks

Run from `projects/chirality-piping` unless noted:

```text
python3 ../../tools/software_workflow/select_affected_checks.py software-workflow.json docs/user_guide/index.md "execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/_STATUS.md" "execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-25_DEL-11-01_USER_GUIDE_CURRENTNESS.md" "execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-25_DEL-11-01_USER_GUIDE_CURRENTNESS_CHECKS.json"
```

Expected selection is `harness-pytest` plus always-check
`harness-self-check`. Execute both through the registered runner and bind the
JSON output to the exact checks path:

```text
python3 ../../tools/software_workflow/run_registered_checks.py software-workflow.json --check harness-pytest --check harness-self-check --output "execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-25_DEL-11-01_USER_GUIDE_CURRENTNESS_CHECKS.json"
python3 ../../tools/validation/validate_claims_language.py --repo-root ../..
```

From repository root, also run:

```text
git diff --check
python3 tools/software_workflow/validate_change_scope.py . --allowed "projects/chirality-piping/docs/user_guide/index.md" --allowed "projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/_STATUS.md" --allowed "projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-25_DEL-11-01_USER_GUIDE_CURRENTNESS.md" --allowed "projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-25_DEL-11-01_USER_GUIDE_CURRENTNESS_CHECKS.json"
```

Any selected check not run, nonzero result, missing JSON, claims finding,
unexplained path, or diff-check failure is a blocker.

## 9. Stop and rerun triggers

Stop without implementation and return to the owner if:

- the adoption act is absent, incomplete, ambiguous, or does not resolve the
  SOW write boundary and SCM-02;
- `_DAG/_LATEST.md`, the adopted PRD pointer, claims registry, DEL-11-01 SOW,
  or exact Remaining item differs from the basis reviewed for adoption;
- any proposed statement lacks accepted evidence or conflicts with another
  current authority;
- any allowed target has concurrent or unexplained changes;
- a protected/private-content or claims-language finding appears;
- a required registered check or containment check fails; or
- closing the residual would require lifecycle judgment.

After any trigger, preserve the failed evidence, leave
`DEL-11-01-REM-001` active, re-read all Section 2 sources, re-select checks,
and require a new or amended owner authorization before rerun where scope,
authority, acceptance, or write ownership changed.

## 10. Candidate effect and readiness

This file is ready only for owner review as an R18 candidate. It does not
cross any Section 5 exclusion or Section 7 gate.
