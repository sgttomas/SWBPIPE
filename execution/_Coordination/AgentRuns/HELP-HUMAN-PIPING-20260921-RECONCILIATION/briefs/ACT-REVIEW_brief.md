# Brief — ACT-REVIEW: independent review of the D-73 activation diff

Parent: HELP_HUMAN Agent 0 (run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`).
Role: TASK (Type 2), fresh context, **read-only**. Do not delegate. Do not
edit, stage, commit, push, or create any file in the repository. Do not run
builds or test suites. You may run read-only `git` and file-reading commands
and the read-only validators named below.

## Candidate

Repository root: the current working directory (a git worktree). Review the
complete diff `00115c71931bcae79909602d653740d3bb72dfa1..5438c1c98` (two
commits on branch `claude/chirality-piping-reconciliation-7f7e70`). All
changed paths are under `projects/chirality-piping/`.

## What the change is meant to be

The recorded activation of a whole-corpus deliverable reconciliation of the
Piping project: a PROPOSAL packet (D-73), the owner's ruling record, a
register row flip, a `DEC-110` codification row, a new project profile
`docs/RECONCILIATION_PROFILE.md`, and a run record (owner directions with
verbatim transcript extracts and hashes, the approved Agent 0 plan, a work
graph, a handoff). It must activate nothing beyond what the owner ruled and
write nothing outside its stated scope.

## Requirements to check the candidate against

Read these first:

- Repo-root `AGENTS.md`; `projects/chirality-piping/AGENTS.md`;
  `projects/chirality-piping/loop/LOOP_INIT.md`.
- `workflows/reconciliation/WORKFLOW.md`,
  `workflows/reconciliation/resources/contract.md`,
  `workflows/reconciliation/resources/method.md`,
  `docs/DELIVERABLE_CONCORDANCE_METHOD.md` (§5–7).
- Conventions of existing records in
  `projects/chirality-piping/execution/_Coordination/_DECISIONS/`
  (e.g. `_REGISTER.md` header, `D-64_RULING_2026-08-04.md`,
  `D-72_RULING_2026-09-18.md`, `D-41_concordance_activation.md`) and
  `SOFTWARE_DECOMP.md` §12 rows `DEC-105`..`DEC-109`.

## Look for defects and unsupported claims, including

1. **Authority.** Does any file state or imply an owner ruling, adoption or
   effect the verbatim owner text does not support? Is agent reading clearly
   separated from owner words? Does anything broaden scope beyond Items 1–6 as
   ruled (in particular: R5/R6 not authorized; method extensions not adopted)?
2. **Verbatim and hash integrity.** For every quoted owner block in
   `OWNER_DIRECTIONS.md` and `D-73_RULING_2026-09-21.md`, recompute SHA-256 of
   the quoted bytes (content between the `~~~~` fences, excluding the fence
   lines and the newline after the opening fence and before the closing
   fence) and compare with the stated hash and byte count. Recompute the
   SHA-256 of `instances/ROOT/ENTRY_BRIEF_2026-09-21.md` and `PLAN.md` against
   `WORK_GRAPH.json`. Recompute the five pinned method files' blob IDs and
   SHA-256 at `00115c719` against the packet's Item 2 table. Recompute the
   profile's SHA-256 at the candidate against the ruling record.
3. **Consistency.** Do the packet, ruling record, register row, `DEC-110`,
   profile, work graph and handoff agree on scope, freeze SHA, phases, surfaces,
   concurrency and model parameters? Does the freeze-tree claim (Piping tree
   identical to the PR #834 merge `620ff6387`) hold?
4. **Format and conventions.** Register row column count and placement;
   `DEC-110` row column structure matching neighbours; ruling-record
   frontmatter; claim-fence sentence; repository-relative paths only (no
   absolute or machine-specific paths); no protected or private data.
5. **Method fidelity.** Does the profile weaken any kernel or contract
   invariant or evidence distinction? Is the no-seeding variance correctly
   characterized against kernel §6 and the contract's "Activation before
   dispatch" invariant?
6. **Validators.** Run and report:
   `python3 tools/validation/validate_claims_language.py` and
   `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .`
   (with `PYTHONDONTWRITEBYTECODE=1`).

## Return

A single message:

- first line `VERDICT: PASS` (no actionable finding) or `VERDICT: FINDINGS`;
- then numbered findings, each with file and line, severity (`BLOCKING`,
  `ACTIONABLE`, `MINOR`), what is wrong, the evidence, and the smallest fix;
- then the hash recomputations as a table (item, stated, recomputed, match);
- then validator outputs;
- last line `END-OF-RETURN`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
