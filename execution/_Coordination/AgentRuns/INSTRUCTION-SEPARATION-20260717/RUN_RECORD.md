# Piping standing-loop instruction separation — run record

Status: COMPLETE — implementation prepared; shared-validator/root-catalog dependencies remain external to this PR
Date: 2026-07-17
Examined-Through: `b495fe19b470b68a87a791708c1b21bf75951900`
Receipt-Basis: `Receipt-53`
Execution: single session agent; no child sessions
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
- Piping-local subagent model/capability-tier assignment convention.
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

### Cross-PR dependencies and gaps

- Root `init/init-prompt.md` must replace its piping launcher with the exact
  project-local launcher in this tranche.
- `tools/validation/validate_instruction_entrypoints.py` and its tests encode
  the superseded requirement that generic orchestration phrases remain in
  project `AGENTS.md` and `LOOP_INIT.md`; the shared validator must be updated
  centrally. This is an implementation dependency under the owner direction,
  not an unresolved owner-policy choice.

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
- Targeted validation plus full practitioner-harness pytest: PASS, 297 tests.
- `python3 tools/practitioner_harness/harness.py self-check`: exit 0; unchanged
  known inventory (`INFO=15`, `NOT_APPLICABLE=2`, `REVIEW=27`, `WARN=6`).
- `git diff --check`: PASS.
- `python3 tools/validation/validate_instruction_entrypoints.py .`: EXPECTED
  DEPENDENCY FAILURE on seven obsolete project-local orchestration phrase
  requirements. No failing phrase is retained merely to satisfy the stale
  validator; central validator/test alignment is required before this PR can
  become merge-ready.
- No executable product code changed; the DEC-025 five-surface code-touching
  sweep is not triggered.
