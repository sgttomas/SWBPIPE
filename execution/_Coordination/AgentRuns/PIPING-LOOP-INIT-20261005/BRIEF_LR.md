# Brief LR: independent review of PR #1092 (piping LOOP_INIT binding form; T3 handoff as init steer)

TASK (Type 2), an independent reviewer for HELP_HUMAN (ROOT), run `PIPING-LOOP-INIT-20261005`. You return to ROOT and do not delegate. **You wrote none of the reviewed text**; LM drafted the LOOP_INIT and ROOT integrated it and wrote the T3 records.

## The candidate

- **PR:** https://github.com/sgttomas/chirality/pull/1092, branch `codex/piping-loop-init-binding-20261005`, head named in your dispatch.
- **How it is cut:** from main `6479bf110a`. It takes six instruction paths and `projects/chirality-piping/execution/` from the T3 integration branch (NUM).
- **Expected tree:** the PR's tree should equal NUM's at the head named in your dispatch.

## Basis

- **The owner's direction:** `RUN/OWNER_DECISIONS.md`. It relies on the App v4 decisions it cites (`V4RUN/OWNER_DECISIONS.md`, the two LOOP_INIT sections). The test: each LOOP_INIT sentence is specific to Piping, instructs, and is stated nowhere else. Also: no routers, no new current-state record type, no edition pins.
- **The precedent:** App v4's mapping and its review, `V4RUN/reviews/MR-LOOPINIT.md`. Use MR's method.
- **The method:** `RUN/BRIEF_LM.md`.
- **The records:** `RUN/LOOP_INIT_MAPPING.md`, `RUN/LOOP_INIT_PROPOSED.md`, `RUN/CONSISTENCY_EDITS.md` and ROOT's `RUN/RULINGS.md`.

## Review, in priority order

1. **The new `projects/chirality-piping/loop/LOOP_INIT.md`:**
   - each sentence passes the three-part test;
   - no load-bearing rule of the old file is lost. Re-run a substantial sample of the mapping's class (a) destination quotes (at least 40, across all destinations) against the files at the head, and check every class (b) and (d) row;
   - every path exists, and every workflow name is in `workflows/index.json`;
   - nothing machine-local and no current state;
   - the stage-gate constraint is accurate against `execution/_Coordination/_COORDINATION.md` and `docs/PRD.md` §24;
   - the `LOOP_RECEIPTS` statement is accurate;
   - ROOT's ruling 3, the changed last sentence of "When to read further", is sound.
2. **Consistency edits A1–A7:**
   - each one is minimal and true;
   - nothing else in live instructions becomes untrue. Search for statements about LOOP_INIT's role, steps or contents in `projects/chirality-piping/` (outside `execution/` records), `init/`, `docs/SPEC.md`, `workflows/` and `agents/`;
   - for A6: confirm that no active concordance run freezes DEL-11-05;
   - section B and C items are correctly left out.
3. **The tranche manifest:**
   - it passes `tools/validation/validate_instruction_tranche_manifest.py`, in CI mode and in diff mode against main with `--tranche PIPING-LOOP-INIT-20261005`;
   - its authorization quotes match `RUN/OWNER_DECISIONS.md`;
   - its notice disposition is truthful.
4. **T3 records:**
   - **The work graph's new section "T3 current route (numerical integrity)":** every node, owner-held choice, owner decision, ID and ruling heading matches T3's records. The ruling headings must exist verbatim in `ROOT_RULINGS_V1.md`; also check `PLAN.md` §§1–3, `BRIEFS/` and the old handoff text in Git history.
   - **The T3 row's status** points to it.
   - **The retired `ROOT_CURRENT.md`.**
   - **The ephemeral handoff:** accurate, and it restates no instruction.
   - **`HANDOFF_2026-10-05_PROMPT.md`:**
     - its init block is byte-identical to `projects/chirality-piping/init/dev-loop-init-prompt.md`, except for the steer line;
     - the steer is consistent with the new LOOP_INIT and project `AGENTS.md`, contradicts neither, and states nothing false;
     - each practice it lists has a basis in T3's rulings.
   - **The T3 ruling** "Owner direction: piping LOOP_INIT in the binding form; …" is accurate. RR is append-only over main: main's copy must be a byte prefix of the PR's.
5. **Publication and portability:**
   - GEN-8 on the head;
   - no machine-absolute paths in any changed living document;
   - no credentials or whole-host data in the delta;
   - the entry validator `tools/validation/validate_instruction_entrypoints.py` passes.

## Host and method

- **Your copy:** a `git archive` of the head into `WT/lr/`, or a read-only Git view. Logs go in `WT/scratch/lr_loopinit_01/`. Delete the copy afterwards.
- **What you may run:** the GEN-8 pytest, the two validators and `projects/chirality-piping/tests/test_ci_e2e_plan.py`. No cargo, native or other suites. No Git writes, no installs, no network beyond `gh` reads.
- **The memory guard** must be running (`pgrep -f memguard.sh`).

## Output

- **The report:** `RUN/reviews/LR-LOOPINIT.md` plus `RUN/reviews/SHA256SUMS`, with placeholder paths only. It contains:
  - a verdict, READY or REPAIR;
  - counts by severity: BLOCKING, MAJOR, MINOR, NOTE;
  - each finding with its evidence and fix;
  - a section per item.
- **Time box:** 90 minutes.
- **End your turn** with the verdict, the counts, one line per finding, the report's sha256, and anything ROOT must rule on.
