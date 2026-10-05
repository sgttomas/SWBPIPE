# Init prompt for the next T3 session (ephemeral, 2026-10-05)

The owner pastes the block below as the first message of the new session, started in the T3 integration worktree (see [the handoff](HANDOFF_2026-10-05_TO_NEXT_ROOT.md)). It is `projects/chirality-piping/init/dev-loop-init-prompt.md` with the steer filled in. Only the steer is specific to this run.

```
<init-prompt>
Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`.

Set `WORKING_ROOT` to
`{REPO_ROOT}/projects/chirality-piping`.

Read `{REPO_ROOT}/AGENTS.md`.
Read `{REPO_ROOT}/agents/AGENT_HELP_HUMAN.md`.

Act as `HELP_HUMAN` for `{WORKING_ROOT}`.

Read `{WORKING_ROOT}/loop/LOOP_INIT.md` and follow it within the owner's
steering and live authority.

Steer (this run): Continue T3, numerical integrity, precision and scale, of the piping undertaking `HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`. Act as its ROOT (Agent 0), under the owner's standing delegation for T3 ("carry on with T3 in the manner you see fit"), and apply `coordinated-knowledge-work` in proportion to the work.

1. This session runs in the T3 integration worktree, so `{REPO_ROOT}` is that checkout. It is on branch `codex/piping-numerical-integrity-20260926` (NUM). Its maintained source equals main's, and it carries T3 records that are not yet on main.
2. The work graph is `{WORKING_ROOT}/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md`. Read its T3 row and its "T3 current route" section. They carry T3's route, the owner-held choices, the decisions and rulings in force, the next IDs and the next safe action.
3. T3's records are in `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`. Read its `HANDOFF_2026-10-05_TO_NEXT_ROOT.md` once, for orientation and the machine-local host details. Read other T3 records when the work needs them.
4. First, verify the host and the heads:
   - the memory guard is running;
   - the worktrees are `numerics` and `sweep-skewpin`;
   - whether main moved;
   - the free disk space.
   
   Give the owner a short status that references the graph. If main moved, absorb it into NUM first, using the dry run in §4 of I61's plan.
5. Then start U8 with I68 Part 1, the probe, and rule on its outcomes before Part 2. Run S-I1 (I73, reviewed by RV99) and the T6 successor-output slice plan (I74) alongside. After that, follow the graph's order through F2a's breadth, B7, B8, S-I2, F2b and F3.
6. Hold to these T3 practices. Their basis is in the T3 rulings the graph lists:
   - **Heavy builds and tests** run on this Mac only while the memory guard runs, one cargo job at a time across all T3 work.
   - **DEC-025, native and solver-at-scale jobs:** TASKs run none of them. ROOT runs DEC-025 with `run_dec025.sh`, and a run counts only when its `meta.txt` ends with `ALL-DONE`.
   - **Before any freeze,** run the full 40-manifest suite.
   - **Product PRs, which come first for S-I1 and the T6 slice,** take T3's full gate set. The ruling "T3's gate set and Git rules, consolidated after the handoff was made ephemeral" lists it: independent complete-diff review, hosted CI with the full-SHA dispatch, GEN-8, an exact-head Mac DEC-025 against a fresh main baseline, and Pass B where the D1 call graph is touched. Merge with `--merge --match-head-commit`.
   - **Product PRs** are cut compactly from main, with maintained-source equality to the NUM head.
   - **Never merge NUM itself into main,** because its history holds redacted originals.
   - **Records reach main through records-only PRs,** after each main merge and before any handoff:
     - cut the PR from main, taking NUM's execution files;
     - run GEN-8 on the exact head and screen for whole-host data first;
     - gate it with the automatic CI and an independent review;
     - squash-merge it with `--match-head-commit`.
   - **IDs and returns:** give implementers and reviewers separate, fresh IDs (the next are I75 and RV101), and verify every return yourself.
   - **Owner-held choices:** prepare the decisions the graph lists as owner-held, and never decide them.
   - **After merges,** remove regenerable build output with `t3_cleanup.py`: gather, then plan, then apply.
7. Keep the graph's T3 section current as the work moves. At a handoff, write only an ephemeral note, plus a steer like this one.
</init-prompt>
```
