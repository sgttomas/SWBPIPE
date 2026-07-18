# Piping standing-loop instruction separation — run record

Status: COMPLETE — review disposition applied; shared-validator/root-catalog dependencies remain external to this PR
Date: 2026-07-17; corrective disposition 2026-07-18
Examined-Through: `b495fe19b470b68a87a791708c1b21bf75951900`
Receipt-Basis: `Receipt-53`
Execution: session agent plus a no-write Agent-1-to-Agent-2 child-depth probe
Authority: owner direction below

## Owner direction (verbatim)

> The owner has approved the following instruction-separation direction for standing development loops:
>
> - The default open-ended development-loop entry role is HELP_HUMAN.
> - `init/init-prompt.md` should only bootstrap the repository, establish the working root, select HELP_HUMAN, and point to the loop.
> - Agent selection below HELP_HUMAN, delegation mechanics, work graphs, fan-out/fan-in, and HELP_HUMAN → WORKING_ITEMS → TASK routing belong in canonical `AGENTS.md` and `AGENT_*.md` instructions—not in project launchers, LOOP_INIT, or standing workplans.
> - The development loop should retain project goals, discovery requirements, authority and owner gates, project-specific constraints, validation, receipts, evidence traceability, and closeout requirements.
> - TASK must not become a top-level chat entry.
> - Preserve genuinely piping-specific governance requirements. Do not remove a rule merely because it mentions delegated evidence; separate project-specific evidence obligations from generic orchestration mechanics.
>
> Please scrutinize and revise the chirality-piping instruction surfaces accordingly.
>
> Required procedure:
>
> 1. Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`.
>
> 2. Read:
>    - `{REPO_ROOT}/AGENTS.md`
>    - `{REPO_ROOT}/agents/AGENT_HELP_HUMAN.md`
>    - `{REPO_ROOT}/agents/AGENT_WORKING_ITEMS.md`
>    - `{REPO_ROOT}/agents/AGENT_TASK.md`
>    - `{REPO_ROOT}/projects/chirality-piping/AGENTS.md`
>    - `{REPO_ROOT}/projects/chirality-piping/init/init-prompt.md`
>    - `{REPO_ROOT}/projects/chirality-piping/loop/LOOP_INIT.md`
>    - the newest standing `WORKPLAN_*.md` selected by that loop
>    - any piping-local validator or instruction surface directly referenced by those files.
>
> 3. Trace every piping-local statement that selects agents or specifies generic orchestration. Classify it as:
>    - canonical agent doctrine that should be removed from the piping loop;
>    - a genuinely piping-specific execution/evidence constraint that should remain;
>    - an ambiguity or governance gap requiring owner direction.
>
> 4. Use this default piping launcher:
>
>    <init-prompt>
>    Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`.
>
>    Set `WORKING_ROOT` to
>    `{REPO_ROOT}/projects/chirality-piping`.
>
>    Read `{REPO_ROOT}/AGENTS.md`.
>    Read `{REPO_ROOT}/agents/AGENT_HELP_HUMAN.md`.
>
>    Act as `HELP_HUMAN` for `{WORKING_ROOT}`.
>
>    Read `{WORKING_ROOT}/loop/LOOP_INIT.md` and follow it: pursue the loop's
>    inherent goals as far as live authority permits.
>
>    Steer (this run): <none>
>    </init-prompt>
>
> 5. Remove piping-local repetition of generic hierarchy and delegation mechanics. In particular, do not retain named WORKING_ITEMS/TASK routing merely to explain how HELP_HUMAN operates. Do not replace removed text with a new summary of the same architecture.
>
> 6. Preserve all piping-specific:
>    - Step 0 discovery requirements;
>    - accepted-state and source-tracing rules;
>    - domain-engine or bridge fences;
>    - owner gates;
>    - validation commands;
>    - receipt contracts;
>    - branch/PR requirements;
>    - project-specific run-evidence or provenance obligations.
>
> 7. Do not edit:
>    - root `init/init-prompt.md`;
>    - root `AGENTS.md`;
>    - canonical `agents/AGENT_*.md`;
>    - chirality-app-dev files;
>    - shared domain-engine files.
>
>    The primary agent will integrate the root launcher catalog centrally. Report the exact root piping-launcher replacement that your project-local change requires. Treat root/project launcher interchangeability as a declared cross-PR dependency.
>
> 8. Work branch-first on a piping-scoped branch, keep all writes within `projects/chirality-piping/**`, validate the affected instruction surfaces with applicable repository validators, run `git diff --check`, and open a PR. Do not merge it.
>
> 9. Do not create a new standing document merely to record the analysis. Use only existing governed run-record or receipt surfaces when the piping loop requires them.
>
> 10. In your return, provide:
>     - the classification of removed versus retained material;
>     - all changed paths;
>     - any unresolved ambiguity;
>     - validation evidence;
>     - the PR URL;
>     - the exact root `init/init-prompt.md` piping section the primary agent should apply.

## Classification and disposition

### Removed as canonical agent doctrine

- Piping-local role inventory and role-selection table.
- Work-graph, concurrency, fan-out/fan-in, sibling-communication, managed-child,
  and generic hierarchy mechanics.
- Named HELP_HUMAN → WORKING_ITEMS → TASK routing and proposed TASK-profile
  selection guidance.
- Generic representation-migration dispatch topology already governed by the
  canonical manager instructions.
- Agent-named Git-closeout routing; the project-specific closeout checklist remains.

### Retained as piping-specific requirements

- Required HELP_HUMAN default entry in the project launcher.
- Path anchors and active-checkout containment posture.
- `DEC-043` engineering-corpus/equation reliability boundary.
- Project software-check profile, canonical dependency-type/DAG-007 rules,
  write fences, unrelated-worktree handling, and protected-data boundary.
- Bounded-execution evidence bindings to Piping `DeliverableID`, `PackageID`,
  scope register, contract, acceptance basis, and explicit write scope.
- All loop discovery, accepted-state tracing, owner gates, DEC-082/DEC-083
  disposition-class triage, validation, receipt, branch/PR, lifecycle,
  domain-engine, claims, evidence, and closeout requirements.
- Piping receipt provenance: actual model/capability attribution by role and
  explicit disclosure of any mid-task model/capability change; silent
  substitution is not valid closeout. Because D-44 freezes the receipt rules'
  prefix bytes, the live post-freeze closeout constraint is restored in
  `LOOP_INIT.md` and evidenced by Receipt 54's `Owner-Direction` record; the
  frozen Rule 8 text, bytes, and hash remain unchanged.

### Cross-PR dependencies and gaps

- Root `init/init-prompt.md` must replace its piping launcher with the exact
  project-local launcher in this tranche. PR #268 closed unmerged on 2026-07-18,
  so a replacement central PR is required.
- `tools/validation/validate_instruction_entrypoints.py` and its tests encode
  the superseded requirement that generic orchestration phrases remain in
  project `AGENTS.md` and `LOOP_INIT.md`; the shared validator must be updated
  centrally. This is an implementation dependency under the owner direction,
  not an unresolved owner-policy choice.
- Generic model selection, capability-tier fallback, and no-silent-substitution
  doctrine are not present completely in canonical agent surfaces. Their
  canonical addition is a separate governance proposal outside this PR's
  protected-path fence; the piping provenance obligation remains live here in
  the interim.

## Review disposition and child-depth evidence (2026-07-18)

The owner accepted the doctrinal resolution: *"I accept your third option on
Finding 1: it is the doctrinally correct reading, and I withdraw the carve-out
proposal."* The HELP_HUMAN launcher therefore does not grant project execution
permissions. Loop execution, validation, receipt, and Git obligations are
workflow obligations discharged by canonical downstream execution. For
`DEC-082`/`DEC-083`, the loop session holds the Step-2 judgment; the writing and
independent verification are executed downstream, and both seats are named in
the rationale artifact and receipt. This topology change is an intended
consequence of the approved HELP_HUMAN default, not an unresolved role
collision. Required child depth unavailable in a harness means lawful deferral,
not inline substitution.

Child-depth evidence is harness-specific:

- **App managed-delegation harness — PASS (accepted prior run evidence).** The
  parent is `HELP_HUMAN` in
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/ORCHESTRATION_PLAN.md`;
  `instances/WORKING-A2-PKG06/STATUS.json` records a terminal Agent 1
  `WORKING_ITEMS` manager with twelve child manifests; for example,
  `children/VERIFY-DEL-06-01/STATUS.json` records the terminal Agent 2 verifier
  and its parent instance. This is an executed depth-2 return, not a brief-only
  representation.
- **Current platform-native harness — PASS (live no-write probe).** Agent 1 node
  `/root/depth2_probe_manager` spawned Agent 2 node
  `/root/depth2_probe_manager/depth2_probe_executor`, collected `PROBE_PASS`,
  and returned the first line of `loop/LOOP_INIT.md` with SHA-256
  `41c2146a5139368b74691e63ae91a81c97136fcc8843c026f3b46db3f3d2ef66`.
  The session independently reproduced the hash. Neither child changed files;
  the only porcelain entry remained the pre-existing untracked
  `.claude-worktrees/`.

## Near-miss candidate for paired corpus review

**N5 candidate — repeal by misclassification.** A project-specific,
owner-directed executor-provenance convention was classified as canonical
duplication even though no complete canonical successor existed; deleting it
therefore silently converted an owner-ruled convention into repeal. The item
fails the delegation class test because repeal changes normative content and
requires an owner act. This record preserves the lesson as a proposal only; it
does not edit the frozen Shared-Block v1, D-49/D-50 packets, or local N1–N4
corpus. Addition to the paired permanent corpora requires their governed
owner-mediated amendment path.

## Changed paths

- `projects/chirality-piping/AGENTS.md`
- `projects/chirality-piping/init/init-prompt.md`
- `projects/chirality-piping/loop/LOOP_INIT.md`
- `projects/chirality-piping/loop/WORKPLAN_2026-07-17_piping_loop.md`
- this run record
- `projects/chirality-piping/loop/LOOP_RECEIPTS.md` at closeout

## Validation

- `python3 tools/validation/validate_path_anchors.py .`: PASS.
- `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .`:
  PASS before and after Receipt 54.
- `python3 tools/validation/validate_claims_language.py --repo-root .`: PASS.
- Targeted validation plus full piping pytest: PASS, 499 tests.
- `python3 tools/practitioner_harness/harness.py self-check`: exit 0; unchanged
  known inventory (`INFO=15`, `NOT_APPLICABLE=2`, `REVIEW=27`, `WARN=6`).
- `git diff --check`: PASS.
- `python3 tools/validation/validate_instruction_entrypoints.py .`: EXPECTED
  DEPENDENCY FAILURE on obsolete project-local orchestration phrase
  requirements. No failing phrase is retained merely to satisfy the stale
  validator; central validator/test alignment is required before this PR can
  become merge-ready.
- No executable product code changed; the DEC-025 five-surface code-touching
  sweep is not triggered.
