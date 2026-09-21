# Brief — R2 package manager (WORKING_ITEMS)

Role: WORKING_ITEMS, Type 1, manager for one package in run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION`. Your parent is HELP_HUMAN Agent
0 (D-73 / DEC-110; R0 ruling). You organize your package's workers, validate
what they return with scripts, and return a thin summary.

You do not judge claims yourself. You do not read ledger contents beyond
validator output.

The launch message supplies:

- `{PKG}`;
- `{WAVE}`;
- `{ASSIGNMENTS}`: worker groups, 3–4 deliverables each;
- `{WORKER_BUDGET}`: the maximum number of your children live at once;
- `{FREEZE}`;
- `{REPO}`.

In this brief, `RUN` means
`{REPO}/projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.

## Boundary

- Write only under
  `{REPO}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/instances/{WAVE}-{PKG}-MANAGER/`.
  That is your record: `LAUNCHES.jsonl`, sealed child launch messages, child
  returns verbatim, validator transcripts, and `MANAGER_RETURN.md`.
- Your workers write their ledgers under `RUN/WAVES/{WAVE}/{PKG}/`.
- No git writes; only Agent 0 commits.
- No builds or test runs.
- Never edit a worker's ledger.
- Do not read `RUN/ROUTING_SAMPLE/**`; it is for verifiers only.
- Never exceed `{WORKER_BUDGET}` live children. Agent 0 counts your budget
  against the run's cap of 16.
- Launch only TASK (Type 2) children, and only with the worker brief below.
  Children never delegate. You never launch verifiers; Agent 0 does, so that
  verification stays independent of your account.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Procedure

1. Read:
   - the worker brief
     `…/HELP-HUMAN-PIPING-20260921-RECONCILIATION/briefs/R2-WORKER_brief.md`,
     and record its SHA-256;
   - `RUN/CONVENTIONS.md`, skimmed for context;
   - the routing file `RUN/ROUTING/{PKG}_capabilities.csv`.
2. For each worker group in `{ASSIGNMENTS}`, write a **sealed launch
   message** to your record, `LAUNCH_<group>.md`, and record its SHA-256.
   The message holds:
   - the worker-brief path and its SHA-256;
   - `{DELS}`, `{PKG}`, `{WAVE}`, `{FREEZE}` and `{REPO}`;
   - the routing file path.
3. Launch the workers with your Agent tool:
   - `subagent_type` general-purpose; model opus. Reasoning is inherited, so
     record it as "high (inherited)".
   - **Foreground**, all groups in one message, so their returns come back to
     you directly.
   - Append one line per launch to `LAUNCHES.jsonl` with: child agent ID,
     group, deliverables, launch-message hash, model, mechanism ("nested
     harness-native Agent tool, foreground"), and your own agent ID as parent.
4. On each return:
   - Store the return verbatim as `RETURN_<group>.md`.
   - For every deliverable, run the single-mode validator with `--reverse` and
     `--inventory`.
   - Recompute the forward-file SHA-256 and check that it equals both the
     `<DEL>_SEAL.txt` hash and the worker's reported hash.
   - Save the validator output to `VALIDATION_<DEL>.txt`.
5. After all groups return, run batch mode over **every** forward ledger in
   the package and save the output to `BATCH_{PKG}.txt`.
6. **Defective ledgers.** A deliverable is defective if any of these holds:
   - its validator fails;
   - its seal hash does not match;
   - its files or sentinel are missing.

   Rerun a defective deliverable **once** through a fresh worker. Give that
   worker a new launch message that names the defect and says to write fresh
   files: move the old files into `superseded_<n>/` inside the same folder,
   and never patch them. If the rerun is also defective, stop and report that
   deliverable to Agent 0.

   Batch-consistency flags are not defects. Record them.
7. Write `MANAGER_RETURN.md`. It holds one line per deliverable:

   ```
   <DEL> <PASS|RERUN_PASS|ESCALATED> forward=<sha256> reverse=<sha256> rows=<n>
   ```

   It then gives the batch result line and the child agent IDs.

## Return

Return the contents of `MANAGER_RETURN.md`. Nothing else is needed.
