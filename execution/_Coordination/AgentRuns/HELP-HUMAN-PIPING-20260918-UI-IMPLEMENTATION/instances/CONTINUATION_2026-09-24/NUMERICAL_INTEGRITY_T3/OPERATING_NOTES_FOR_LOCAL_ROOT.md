# T3 operating notes for the next ROOT (HELP_HUMAN), 2026-09-28

This is a companion to `HANDOFF_2026-09-28_TO_LOCAL.md`. It records how the cloud ROOT ran T3 through the Type 0/1/2 roles, and what did and did not work. These are practice notes, not rulings. Where they touch a ruling, the ruling in `ROOT_RULINGS_V1.md` governs.

## 1. Shape of the delegation

- **ROOT (HELP_HUMAN, Type 0):** you. You own rulings, pushes, PRs, merges and branch restarts, and you speak to the owner.
- **The T3 manager (WORKING_ITEMS, Type 1):** one long-lived agent for all of T3. It:
  - writes briefs and addenda in `TASK_BRIEFS/`;
  - arbitrates the host (the "cargo token");
  - verifies and commits every TASK's work (hashes, SHA256SUMS, GEN-8, no machine paths, `git diff --check`);
  - drafts records (merge records, the work graph, ruling text you give it).
- **TASKs (Type 2):** implementers (I-numbers) and independent reviewers (RV-numbers). One slice per implementer, and a **fresh reviewer per slice**. They make no Git writes and do not delegate.
- **Spawning.** In the cloud host, subagents could not spawn subagents, so **ROOT spawned every TASK** from the manager's brief, and the manager then directed it. On the Mac, check whether nested delegation works. If it does, the manager may dispatch TASKs directly (root `AGENTS.md` allows this). Record the actual mechanism in run evidence either way.
- **Reviewers:**
  - Read the whole diff against the brief, design and rulings, and try to evade the tests with small patches.
  - Build only from `git archive <exact commit>`, never in the implementer's worktree.
  - Their verdict names the exact head.
  - A delta check covers later commits. If a main merge touches no piping product, `tools/` or `.github/` path, the reviewer confirms that, and an earlier DEC-025 sweep can stand for the new head.

## 2. Messaging mechanics (the main source of lost time)

- **Managers often end their turn before acting on a queued message.** If `SendMessage` reports "queued" and a hand-back then arrives that doesn't reflect your message, **resend it**, prefixed "resent, in case you finished before it arrived". Expect crossed messages, and don't treat a crossed hand-back as a reply.
- **Hand-backs repeat.** One informal message and one formal hand-back often cover the same event. Act once.
- **Ask for decisions in one message,** with the manager's recommendation and explicit options. Answer with a ruling plus conditions, and have the manager record it (see §5).
- **Keep the manager's authority clear.** It may take in-authority calls and report them for you to overrule, for example I8's private field or reading "no new field" as published output. Anything touching a ROOT ruling or pin comes to you.
- **Background agents can be lost** when the container restarts. Commit or push WIP often, and keep a byte snapshot of uncommitted drafts. A replacement TASK brief (see `I3R_KD5_REPAIR_RESUME.md`) resumed cleanly from disk.

## 3. Host arbitration

- **One named token holder,** granted and returned through the manager. TASKs do all reading, scanning, drafting and design first, then ask for a slot with a size estimate.
- **Before any handover of the host,** run `pgrep -af 'cargo (test|build|run)'`. Anchor wait and kill patterns so they cannot match the waiting shell, e.g. `pgrep -f 'python[0-9.]* .*run_evidence_sweep'`. One agent's unanchored `pkill` killed its own shell.
- **Timing runs and gate part 2** need a quiet host: nothing else heavy running.
- **DEC-025 sweep needs:**
  - a clean tree;
  - `node_modules` hardlinked in;
  - the authority targets (`core/serialization/canonical_json/target` and `core/units/target`) present;
  - a pruner that excludes those targets and `self_weight_wasm`.
- **On the Mac,** the one-job rule can relax for untimed work. Use a shared build cache and run gate part 1 cases in parallel, but keep timing runs and gate part 2 solo.

## 4. Pace: keep the pipeline full

- **While one slice builds,** have the manager draft the next briefs, and spawn the next implementer to read and design.
- **Spawn a parallel slice** when write sets are disjoint. Record the merge order (K1 was spawned alongside K2a and merges after it).
- **Run the sweep** while the reviewer reads, and give the reviewer the host after the sweep.
- **Open records PRs during long builds.** Keep numerics commits local while a records PR is under review, so the reviewed head stays put.

## 5. Epistemic discipline (ROOT's own errors this run)

ROOT made four rulings that later needed withdrawing or correcting:
- the M31b equivalence;
- the K2a product-reach claim, twice;
- the S11-G performance figure.

A manager premise needed a third K2a correction. Each came from ruling on a claim that had not been derived or run on the product. The lessons are recorded in `ROOT_RULINGS_V1.md`. In practice:
- **Before ruling on a numeric bound, a behavioural claim or a speed claim,** ask for the derivation, the product run or the interleaved run, and for someone other than its author to check it.
- **Rulings cite a RETURN section for numbers,** not restated figures (K2a's rule).
- **Never rewrite a ruling.** Supersede it in place with a pointer, and add a correction section. Reviewers check for silent rewrites.
- **Honour stop rules.** The most valuable findings came from TASKs that stopped instead of forcing a case: I6 on K2a's premise, and I3 and RV5 on M31b. A stop is a result, not a failure.

## 6. PR and merge mechanics

- **E2E dispatch:** `piping-desktop-e2e.yml` with a full, real 40-hex `target_base`, which must be an ancestor of the head. Copy it from `git rev-parse`; a mistyped one wastes a run. The pull_request run fails plan validation if main's tip is not in the head, so merge main first (a merge commit, never a rebase).
- **Merge:** a merge commit with `expectedHeadSha`, once review PASS, green CI, the sweep and the dispatch are all in. Then restart the designated branch from main and force-push it, since it is your own branch.
- **After each merge:**
  1. the manager writes `<SLICE>_MERGE/RECORD.md` with the real merge SHA (no placeholders);
  2. you push numerics;
  3. then a records PR with its own independent reviewer.
- **Watch main.** It moves often (PEC and App). Before trusting an earlier sweep, check that the merge touches no piping product, `tools/` or `.github/` path.
