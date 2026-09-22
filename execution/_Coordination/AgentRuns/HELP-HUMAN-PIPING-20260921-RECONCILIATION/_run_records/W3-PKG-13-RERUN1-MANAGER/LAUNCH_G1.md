# Sealed launch message — W3 PKG-13 worker G1 (RERUN, cycle 1)

You are a TASK (Type 2) worker in wave W3 (rolling queue) of run
HELP-HUMAN-PIPING-20260921-RECONCILIATION. Your parent is the WORKING_ITEMS
rerun manager for PKG-13. Do not delegate.

Read your brief first and follow it exactly:
/Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/briefs/R2-WORKER_brief.md
(SHA-256 2d793d0a7669618d6eca6f1c8cb66c180ea0edb038a9605d0c0e29556b0db141).
If the hash does not match, stop and report.

Values:
- {DELS} = DEL-13-02
- {PKG} = PKG-13
- {WAVE} = W3
- {FREEZE} = /private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/freeze (read-only checkout of 00115c71931bcae79909602d653740d3bb72dfa1; never write there)
- {REPO} = /Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2

## This is a rerun of a defective deliverable

This launch invokes the worker brief's **rerun clause** for DEL-13-02.
Before writing anything else, move the existing DEL-13-02 files
(`DEL-13-02_forward.csv`, `DEL-13-02_SEAL.txt`, `DEL-13-02_reverse.csv`,
`DEL-13-02_notes.md`) unchanged into
`RUN/WAVES/W3/PKG-13/DEL-13-02/superseded_1/`. Move only; never edit, delete
or reuse them, and do not read their contents to build your new ledger. Then
encode DEL-13-02 afresh: both passes, sealed, under CONVENTIONS Part F.

The defect is the independent package verifier's finding, not a validator
failure. The verifier (`RUN/WAVES/W3/PKG-13/PKG-13_VERIFICATION.md`, SHA-256
4ac0682fc54b02fb933e7ba76b6dc0027a1c5be83c0f95e6d11becff587c2db2) returned
RERUN DEL-13-02 because `DEL-13-02:SOW#CLM-019.r04` ("unit-bearing values are
unit-aware or blocked as TBD") was disposed ALIGNED although the ledger's own
requirement row records that a `quantity` parameter can carry a bare string
with no unit metadata (Part F rule F1). The verifier also names CLM-003,
CLM-009, CLM-012/R-13-02-005 and CLM-024 outside its sample. Judge those rows,
and every other row, on the evidence under the bound conventions.

You may read that verification report's DEL-13-02 findings only **after your
forward ledger is sealed**, not before; the forward pass must be your own
judgment. If reading it afterwards changes your view of anything sealed, say
so in the notes; do not edit.

Your carry-forward notebook, if you keep one, is
`RUN/WAVES/W3/PKG-13/_WORKER_DEL-13-02_NOTES.md`. Do not read the other
PKG-13 deliverable folders or `_WORKER_DEL-13-01_NOTES.md` (the first run's
material).

The brief's consistency check (batch) for your own work covers your one
forward file; your parent runs the package batch over all four ledgers.

This wave runs under CONVENTIONS Part F: validate the forward ledger with
`--notes-gap` before sealing, and the reverse file with `--notes-gap` as
the brief states. Deliverable-local evidence paths may be cited in evidence
columns (paths with spaces resolve at the freeze).

API-overload practice: keep scratch files in uniquely named `_scratch_*`
paths inside your own deliverable folder and delete them before sealing;
write drafts to disk early and keep each generation short.

Routing file (read only after your forward ledger is sealed):
/Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2/projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/ROUTING/PKG-13_capabilities.csv

Model: opus; reasoning "high (inherited)".

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
