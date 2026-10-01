# RV28 selected-numerator execution receipt

Actual start read from clock: 2026-10-01 17:27:21 UTC.
Deadline: 2026-10-01 17:52:21 UTC. Final time is recorded in TIMING.json.
TASK /root/rv28_a1_design, parent /root, native followup_task. No delegation.
The shared host imposes a prompt write fence, not a separate OS sandbox.

Only <K6C>/<R>/design_review_RV28/selected_numerators_01/** was written.
The checker is a bounded evidence artifact, not maintained or reusable tooling.
It decodes stored bits and calculates exact rational inequalities; it does not
execute the solver, form a stiffness matrix, generate a model, emulate the
runtime arithmetic, or read expected outcomes as numeric inputs.

Source review used git show at the full immutable 40129 revision, with bounded
nl/sed views, plus cat/rg and a standard-library metadata inspection of existing
JSON. Read-only Git always used GIT_OPTIONAL_LOCKS=0.
The original proposal and source14/RV30 seals were checked independently and
left unchanged. Ten pinned source/design files and two fixture files are
hash-bound in BASIS.json; paths and argv are in COMMANDS.json.

Canonical exact run, exit 0:

    <VENV>/bin/python -B <OUT>/independent_check.py <K6C> <NUM> <OUT> <APP_WORKTREE> \
      > <OUT>/RUN.stdout.json 2> <OUT>/RUN.stderr.txt

OUT is the granted review directory. Python runtime identity is in BASIS.json.
The first successful run used the same arithmetic with APP_WORKTREE hardcoded
for reading instruction hashes; that path was then made a CLI argument for
portable, repository-relative evidence and the same checks reran successfully.
Both runs passed 2,876 assertions. No failed numeric check or numerical repair
occurred. The retained stdout is the canonical second run; stderr is empty.

Additional final source reconciliation reads returned:

    git rev-parse HEAD
    66c186743b4db005e942f241d7e72a30ae0365e5

    git rev-parse HEAD:projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/adaptive.rs
    5448ca262ab0346c138052e39b9a205b82739842

    git hash-object projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/adaptive.rs
    5448ca262ab0346c138052e39b9a205b82739842

All three ran read-only in <K6C>, with GIT_OPTIONAL_LOCKS=0, exit 0 and no stderr.
They confirm ROOT's clarification: K6C's different adaptive.rs is its older
committed source awaiting A1 integration, not an unexpected working-tree edit.
The numerical review uses immutable 40129, as assigned.

SHA256SUMS lists all packet files except itself. Future checks must write an
addendum or use an owned scratch OUT; do not overwrite this sealed review.
