# T3 host cleanup, 2026-10-05 (ROOT, at the owner's direction)

**The owner's direction:** "remove what you can safely remove from built targets and scratch, etc. That's a lot of disk space and we need to do periodic cleanup of what can be safely removed."

This supersedes, for regenerable material only, the earlier no-prune practice (HANDOFF_2026-10-03: "do not prune"). Evidence that exists only on this disk is kept.

## What "safe" meant, and the tool

**Removed:** only what is regenerable from Git or by rebuilding.
- **cache:** a Cargo target directory (its root holds `CACHEDIR.TAG` or `.rustc_info.json`).
- **treecopy:** a copied source tree (a `projects/` directory holding `chirality-piping/core`), meaning `git archive` copies and mutant copies of committed revisions made for runs and reviews. Every review brief already required deleting these afterwards.
- **worktree:** a Git worktree under the T3 root that `git worktree remove` accepts as clean, and whose HEAD is contained in `origin/main` or the integration branch, with its branch pushed as-is.

**Kept:**
- every log and output file;
- `guard/`, `preserved-evidence/` and `tools/`;
- any path containing a file tracked in its worktree;
- NUM (`numerics`) and the DEC-025 tree (`sweep-skewpin`).

**The tool** is `WT/tools/t3_cleanup.py`, with a copy in `host_tools/t3_cleanup.py.txt`.
- **`plan`** is read-only and lists candidates with sizes.
- **`apply`:**
  - runs only from a plan file;
  - refuses while any cargo, rustc, pytest or vitest process runs;
  - re-checks each path's class and tracked status immediately before removal;
  - skips a path modified in the last 2 h;
  - restores owner write permission inside a candidate when a copy was made read-only;
  - logs one JSON line per action.

**Two defects were found and fixed before or during the run:**
1. **The first plan included committed record folders inside worktrees,** such as review `reviewed_sources/projects` snapshots. The tracked-file guard was added before anything was applied.
2. **Read-only copies stopped `rmtree`.** Owner-write restoration was added, and later the parent-mode restoration for the final removal.

## Result

**Measured on the volume:** used space went from 1.2 TiB to 695 GiB, freeing about 0.5 TiB (`df`).
- `WT/targets` went from 293 GB to 0.3 GB.
- `WT/scratch` went from 149 GB to 44 GB.
- The top-level `*-target` directories are gone.

**By class** (`cleanup/SUMMARY.json`; its byte counts double-count Cargo hard links):

| Class | Removed | Not removed |
|---|---|---|
| cache | 509 | 11 skipped as recently modified (the current DEC-025 target and caches inside `sweep-skewpin`) |
| treecopy | 398 | 59 emptied, each leaving an empty `projects/` inside a read-only parent (39 MB; the tool now handles this); 1 skipped as recent |
| worktree | 25 | — |

**Afterwards, five more worktrees went, by hand:**
- **Three with `git worktree remove`:** `f2a-carriers`, `f2a-carriers-ts` and `f2a-u7`. Each was clean once its untracked `node_modules` symlink was unlinked.
- **Two with `git worktree remove --force`,** after their unique untracked files were copied into `_run_records/preserved_untracked/` (SHA256SUMS there):
  - **`f2a-readers`:** 38 untracked files. 33 were byte-identical in NUM; 4 reader records were preserved. The 38th was the `node_modules` symlink, unlinked.
  - **`sweep-k1`:** one sweep JSON that differs from `K1_MERGE`'s committed copy, preserved.

**Only two worktrees remain under the T3 root:** `numerics` (NUM) and `sweep-skewpin` (the DEC-025 tree).

**The full action log** is `cleanup/manifest_20261005.jsonl`, with paths relative to the T3 root.

## What stays on disk (evidence only here; about 44 GB in `WT/scratch`)

The largest items:
- `u9_gate` (12 GB): G6 both-entry gate outputs;
- `i13` (12 GB) and `i20` (8.3 GB): earlier kernel and implementation run outputs;
- `i14` (1.7 GB);
- `gate_base_e7d930d49_full` and `gate_base_e7d930d49`: KF2's gate inputs, preserved by earlier ruling;
- `rv12` (1.3 GB);
- the DEC-025 sweep folders `sweep_*` and `u9_dec025`;
- the reviewers' scratch `rv*_*`.

`WT/preserved-evidence` (168 MB) holds AUD-T3-04's raw evidence. Decisive summaries of all of it are committed. **Removing any of it is the owner's call, not the tool's.**

## Periodic cleanup (for every later ROOT)

**All T3 scratch lives in `WT/scratch`.** That excludes the parent checkout's `scratch/`, a worktree's own folder, and the system temp directory. Briefs say so, and step 0 below moves anything that strayed.
**This is a documented procedure with a host tool, not a registered workflow.** If the owner wants a reusable workflow, it is made with `create-workflow` and registered by the owner.

**When:** after each merge to main, and whenever free space falls below about 400 GiB, but never while a cargo, test or DEC-025 job runs.

**How:**
0. **Gather stray scratch** into `WT/scratch`:
   ```
   python3 WT/tools/t3_cleanup.py gather
   ```
   That is a dry run listing each stray entry and its destination. Then:
   ```
   python3 WT/tools/t3_cleanup.py gather --apply --log WT/tools/cleanup_logs/gather_<date>.jsonl
   ```
   - **Strays** are a `scratch/` folder directly in the parent checkout (the folder that holds `.claude/`), or an untracked one directly in a worktree under WT.
   - **On a name collision** the destination gets a `_from_<origin>` suffix.
   - **Every file's sha256 is verified after the move.** Nothing is deleted. `plan` also prints a STRAY line while any remain.
1. Plan, read-only:
   ```
   python3 WT/tools/t3_cleanup.py plan --out WT/tools/cleanup_logs/plan_<date>.json
   ```
2. **Read the plan:** the totals, and the kept-worktree reasons it prints. Confirm that no entry lies inside a worktree you still need for running work.
3. Apply:
   ```
   python3 WT/tools/t3_cleanup.py apply --plan <plan> --log WT/tools/cleanup_logs/manifest_<date>.jsonl
   ```
4. Record the result in the next ruling.

**Deleting logs or evidence** is never part of the periodic cleanup; ask the owner.

## Addendum, 2026-10-05: stray scratch gathered (owner direction)

**The owner directed:** "move the scratch records to the common folder and ensure the cleanup work instructions … indicate to do so also."

**The strays:** the parent checkout's untracked `scratch/` (77 MB, 1,347 files) held two earlier T3 assignments' working folders, written there before this session.
- **`rv84_u4_g3_01`** moved to `WT/scratch/rv84_u4_g3_01`.
- **`i54_direct_container_profile`** collided with an existing `WT/scratch` folder of the same run. That folder has all 134 stray paths, and only `INPUTS.json` differs. The stray was moved whole to `WT/scratch/i54_direct_container_profile_from_parent_checkout`.

**Verification:** all 1,347 files' sha256 were verified after the move (`cleanup/strays_moved_20261005.sha256`). The emptied `scratch/` was removed, and the parent checkout is clean.

**The tool gained `gather`,** which was tested on a throwaway probe. The committed copy is updated in `host_tools/`.

**The periodic cleanup then ran after #1084's merge:** 17 caches (the last DEC-025 runs) and 3 tree copies, about 11.9 GB (`cleanup/manifest_20261005c.jsonl`).

## Addendum 2, 2026-10-05: RV100's N-5

**RV84's two tree copies were removed.** Of the 1,347 gathered files, 1,193 belonged to `rv84_u4_g3_01/srcb1f/projects` and `rv84_u4_g3_01/src5ae/projects`. The periodic cleanup removed them about a minute after the move, as regenerable `treecopy` entries; both source commits are on origin.

**The hash list's paths are relative to the stray root,** which is the parent checkout's `scratch/`. Lines under `i54_direct_container_profile/` map to `WT/scratch/i54_direct_container_profile_from_parent_checkout/`.

**The move was made by hand,** before `gather` existed, so there is no gather log. The hash list and the first addendum record it.

## Addendum 3, 2026-10-05: after #1088's merge

**Before the cleanup:**
- `gather` found no stray scratch.
- The merged records worktree `records-pr2` was removed (clean, and pushed as-is).

**The cleanup:** `plan` listed 16 caches, about 9.6 GB: today's DEC-025 build outputs in `sweep-skewpin` and `sweep-skewpin-target`.
- **The age guard was set to zero** (`apply --min-age-hours 0`). The caches were under 2 hours old, but no build or test was running.
- **All 16 were removed** (`cleanup/manifest_20261005d.jsonl`).

**After:** only `numerics` and `sweep-skewpin` remain, and 1.1 TiB is free.
