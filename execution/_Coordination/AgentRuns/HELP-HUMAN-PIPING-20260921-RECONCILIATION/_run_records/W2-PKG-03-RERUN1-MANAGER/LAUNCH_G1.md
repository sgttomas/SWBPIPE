# Sealed launch message — W2 PKG-03 rerun worker G1 (verifier-triggered rerun, cycle 1)

You are a TASK (Type 2) worker in gate wave W2 of run
HELP-HUMAN-PIPING-20260921-RECONCILIATION. Your parent is the WORKING_ITEMS
rerun manager for PKG-03 (W2-PKG-03-RERUN1-MANAGER). Do not delegate.

Read your brief first and follow it exactly:
/Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/briefs/R2-WORKER_brief.md
(SHA-256 2d793d0a7669618d6eca6f1c8cb66c180ea0edb038a9605d0c0e29556b0db141).
If the hash does not match, stop and report.

Values:
- {DELS} = DEL-03-07
- {PKG} = PKG-03
- {WAVE} = W2
- {FREEZE} = /private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/freeze (read-only checkout of 00115c71931bcae79909602d653740d3bb72dfa1; never write there)
- {REPO} = /Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2

## This is a rerun

You are rerunning a defective deliverable. Invoke the worker brief's rerun
clause: **before writing anything**, move the existing DEL-03-07 files
(`DEL-03-07_forward.csv`, `DEL-03-07_SEAL.txt`, `DEL-03-07_reverse.csv`,
`DEL-03-07_notes.md`, and anything else in that folder) unchanged into
`RUN/WAVES/W2/PKG-03/DEL-03-07/superseded_1/`. Move only; never edit, delete,
read for reuse, or copy rows from them. Then encode DEL-03-07 afresh: both
passes, sealed, under CONVENTIONS Part F.

The defect is a verifier finding, not a validator failure. The package
verifier (`RUN/WAVES/W2/PKG-03/PKG-03_VERIFICATION.md`, SHA-256
f08a7f0d2092db06ca6ebba0d951f2c22264ea2a0c3e81e7ace9ce85ef817bbd) returned
RERUN DEL-03-07 because `DEL-03-07:SOW#CLM-021` was `ALIGNED` while its own
Notes recorded an unmet element of the claim (F1).

Your forward pass must be your own judgment. Read that verification report's
DEL-03-07 findings only **after** your fresh forward ledger is sealed, not
before. Do not read the superseded files at any point. Do not read or modify
the previous worker's notebook `RUN/WAVES/W2/PKG-03/_WORKER_DEL-03-07_NOTES.md`;
it belongs to the first run. With one deliverable you do not need a
carry-forward notebook; if you want one, name it
`RUN/WAVES/W2/PKG-03/DEL-03-07/_scratch_notebook.md` and delete it before
returning. Do not read other deliverables' folders.

Gate-wave rules: validate the forward ledger with `--notes-gap` before
sealing, and the reverse file with `--notes-gap` as the brief states. For the
consistency check, run batch mode over your own forward file only; your
manager runs the package batch.

Routing file (read only after your forward ledger is sealed):
/Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2/projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/ROUTING/PKG-03_capabilities.csv

In your return, add one line confirming the move: which files went into
`superseded_1/`.

Model: opus; reasoning "high (inherited)".

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
