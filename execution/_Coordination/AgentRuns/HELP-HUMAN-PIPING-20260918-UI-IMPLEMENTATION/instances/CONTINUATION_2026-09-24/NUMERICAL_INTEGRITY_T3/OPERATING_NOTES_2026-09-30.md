# T3 operating notes (2026-09-30): how ROOT ran T3 on the Mac, and what it learned

> **Practice notes, not rulings.** Where these touch a ruling, `ROOT_RULINGS_V1.md` governs.
>
> This succeeds `OPERATING_NOTES_FOR_LOCAL_ROOT.md` (2026-09-28), which stays valid except where the handoff's §7 or this file differs (RV26-N7). There is no separate T3 manager now: ROOT did that work. And from K5 (2026-09-29) on, DEC-025 ran on each slice's exact final head. On 2026-09-28, M03's and K3's sweeps were carried to a later head that changed only tests and records, as their merge records disclose (RV26 delta D1). Their merge records justify each carry-over: M03's later head changed only records, and K3's changed FK tests and records, with FK's suite re-run at the head (RV26 C1). The 2026-09-28 notes' carry-over rule covers only a main merge that touches no piping product path. Running the sweep on the exact head is simpler to audit. It complements `HANDOFF_2026-09-30_AUDIT_PAUSE.md`. The handoff gives the **state** and the **mechanics** (commands, paths, gates). This file gives the **practice**: how the work was actually run day to day, the judgment calls, what went wrong and why, and what a successor would otherwise have to re-learn.
>
> **Sources.** Most statements here cite committed records. Some rest on ROOT's session experience only. Examples: the resent grant, token usage, the number of concurrent subagents, the owner's words in §9, and parts of §4's timings and §8's quirks. They are not contradicted by the records, but they cannot be checked there (RV26-N5, N6).
>
> **Readers:** the owner's auditor, and any agent that resumes T3 as ROOT. Placeholders follow the handoff's §0 (`<wt>`, `<VENV>`, `T3/`, `P/`).

## 1. The shape of the work

- **One ROOT, no manager.** On the Mac, ROOT (HELP_HUMAN, one Claude Code session) dispatched every TASK directly as a background subagent. ROOT also did the manager's work: it verified and committed every change, ran every PR, gate and merge, and wrote every ruling and merge record.
  - This worked, with up to four subagents at once.
  - The cost is ROOT's own attention and context. Long sessions were compacted several times, and the rulings file and the records were what made that survivable.
- **The next free numbers are I22 and RV27** (I21 is reserved for K6c; RV26 reviewed records PR #1063).
- **Implementers are numbered I<n>; reviewers RV<n>.** There is one fresh reviewer per slice PR and per records PR, and the same reviewer confirms its own findings' fixes.
  - The numbering continues: I21 is reserved for K6c. Take the next free numbers from `TASK_BRIEFS/` and `REVIEW/`.
- **Everything durable goes in the repository.** Rulings, briefs, run records and reviews are committed. A message to an agent is not a record until the ruling it carries is committed where the agent can read it. That was the owner's instruction in this session; it is not otherwise recorded.

## 2. The rhythm of one slice, and what ROOT checks at each step

The sequence below is what KF1, V-K, K6b, KF3 and KF2 followed. Every "ROOT checks" line is something that, at least once, caught a real problem or would have.

1. **The brief** (`TASK_BRIEFS/I<n>_<SLICE>_IMPLEMENTATION.md`). I19 (KF3), I20 (KF2) and I21 (K6c) are good templates. The sections are:
   - roles;
   - paths and the base;
   - the purpose, with the evidence and **ROOT's reading**, a hypothesis for the implementer to confirm or refute;
   - a numbered scope;
   - a write set ending "anything else is a stop";
   - required tests, including named mutants;
   - the gates;
   - checkpoints 0, A, B and D, with stop conditions;
   - the host rules.
2. **Spawn.**
   - Create the worktree with `git worktree add -b codex/piping-<slice>-<date> <wt>/<slice> <main SHA>`, and record the base in a "spawn" ruling.
   - Spawn the agent with a prompt that points at the brief and the paths, says "this turn is checkpoint 0 only; no code, no build", and restates the Git rule.
3. **Checkpoint 0 (the plan).**
   - **ROOT reads the plan's central argument itself,** not just the summary. KF2's equality proof and KF3's A2 honesty argument were checked line by line before any code.
   - ROOT answers the plan's numbered questions in **one** ruling section, then commits the plan on the branch.
4. **Checkpoint A (the code).** ROOT checks:
   - `git status`: only write-set paths are changed;
   - each file's sha256 against the report;
   - **the core diff, read by ROOT.** KF2's guard was compared cell for cell with the verifier;
   - that each records folder's SHA256SUMS verifies, run from inside the folder;
   - a machine-path scan.

   Then ROOT commits ("By I<n>; ROOT committed.") and pushes.
5. **Checkpoint B (measurements or heavy gates).** ROOT grants a slot explicitly, then **checks that the work started**: a process, a scratch folder or a log growing (§3).
6. **Checkpoint D (records),** checked as at A.
7. **The PR:**
   - merge main in first if it moved;
   - open the PR (body: What, then Gates still to run);
   - dispatch the full-SHA E2E, and bind the PR (the desktop app's PR bar usually binds it on its own);
   - spawn the reviewer (§5).
8. **Review fixes.**
   - ROOT rules first, with the ruling committed on numerics, then sends the fix list to the same implementer. `SendMessage` to its agent ID resumes it.
   - ROOT commits the fix, re-dispatches CI, and asks the **same** reviewer for a confirmation of the new head.
   - If ROOT then merges main into the branch, it also asks for a **merge check**: path-by-path parentage, the remerge diff, and the suites on a clean archive.
9. **The gates on the final head:**
   - CI green with the dispatch;
   - DEC-025 on that exact head;
   - GEN-8 in a clean working tree;
   - **then `git fetch` and check that main has not moved** (handoff §7.2 item 6).
10. **Merge and record:**
    - `gh pr merge --merge --match-head-commit <sha>`;
    - `<SLICE>_MERGE/RECORD.md` with `dec025/` and SHA256SUMS;
    - a "<slice> merged" ruling;
    - the work graph's state, updated at every merge, not in batches.

## 3. Reading agent reports: what to trust

- **Trust, but recompute what matters.**
  - **Suite counts.** DEC-025's per-crate change must equal the slice's added `#[test]`s, counted with `git diff <main> <head> -- <crate> | grep -c '^+\s*#\[test\]'`. It did every time (KF3 +15 and +1; KF2 +16, 1 ignored), and that makes the sweep comparison mean something.
  - **Merge resolutions.** For each resolved file, main's delta and the resolution's delta should differ only by the ruled change (KF3's `c0473301e`, a resolved conflict; KF2's `522167ac6`, a clean auto-merge checked the same way).
  - **Figures a ruling will rest on.** Open the source table before writing the ruling (§6).
- **Reviewers can be wrong too, and implementers catch it.** RV24 counted 8 recovered runs where there were 4, and I20 caught it; RV24 then confirmed. Treat every count as a claim.
- **The return path is noisy:**
  - hand-backs sometimes arrive twice;
  - a completion notice can say "stopped with background work still running", although the report is already in;
  - a message "queued" to a running agent can land after it has ended its turn, and it then never sees it. I20's B grant had to be resent. **After granting work, look for evidence that it started.**
- **Implementers' mutant diffs must be recorded** (the patch text, not just a description). RV23 had to rebuild five of I19's mutants from descriptions. Ask for `_run_records/**/mutants/<id>.diff` in every brief.

## 4. The host and the clock (observed on this Mac: 18 cores, 128 GiB, no swap partition; macOS showed about 1 GiB of dynamic swap allocated, about 0.2 GiB in use, RV26-N3)

- **Rough durations.** Plan around these; they were observed with other work running.

  | Step | Time |
  |---|---|
  | DEC-025 | about 36–37 min (K6b, KF3, KF2) |
  | FK's full debug suite at `RUST_TEST_THREADS=2` | about 11–13 min |
  | `gen_k4_vectors.py --check` | about 15 min |
  | hosted CI's numerical cargo job | 17–24 min |
  | a slice review | about 40–80 min |
  | a records review | about 40 min |
  | a confirmation or delta check | 5–25 min |

  KF2's B (T9, gate part 1 at 884 × 2, part 2 and src-tauri): its recorded runs span about 28 minutes (02:48–03:16Z). Gate part 1 took about 6.4 min per side, and part 2 about 7.5 min. [This line said "a few hours" in its first draft, a ROOT figure error in this very file (RV26-S1).]
- **Overlap freely, except for timing.**
  - Two or three slices building, plus a reviewer, plus a DEC-025 sweep, ran together safely.
  - ROOT started a slice's sweep while its reviewer was still confirming the same head, and would have re-run it had the confirmation failed.
  - Keep timed measurements and gate part 2 to a quiet host.
  - vitest's workspace test can time out at 30 s when the load is above about 8. Re-run it quiet; never wave it through.
- **The memory guard** (`<wt>/guard/memguard.sh`) SIGKILLs anything naming `<wt>` under 35% free memory. No kill was logged this week, and KF3's 10,000-member runs peaked at about 3 GB. Still, **never materialize a dense matrix at 10,000 members or more.**
- **Usage.** The host's completion notices reported roughly 0.3–0.8 M tokens per subagent run. A slice from brief to merge needed an implementer across 4–5 runs plus a reviewer across 2–3. Budget for that when the owner's quota matters.

## 5. Reviewers: how to get real findings

- **The prompt lists review items in priority order.** It names the claim to break ("the equality holds for every value of the type, errors included") and says: **build independent oracles** (exact rationals in Python's `fractions`, GEN, probes of your own), don't reuse the implementer's tests, and try single-edit mutants.
- **The reviewers repeatedly found real issues that the implementer's tests missed.** Examples (the last is ROOT's own, not an implementer's):
  - RV19-1, a false publication;
  - RV22-1, a relaxation applied to completed builds;
  - RV23-1, refusals dropped on a stop;
  - RV24-1, three regressions surviving the committed tests;
  - RV25-S1 and S2, ROOT's own figures.
- **Reviewers build from `git archive <exact commit>`** in their own `<wt>/rv<n>*/` folders, and delete them afterwards. From RV23 on they did; some earlier reviewers' folders remain (handoff §9). They write their report **uncommitted** into the numerics worktree, and ROOT commits only their paths (`git add <their files>`), because several agents write to that worktree at once.
- **Records reviewers** check four things:
  - scope;
  - append-only against main (in-place brackets are listed and checked);
  - every SHA256SUMS;
  - merge records against GitHub, and a sample of ROOT's figures against their sources. **This is where ROOT's own errors were caught.**
- **The reviewer prompts for RV14 to RV26 were given inline, and not committed** (RV26-N8, C2). `TASK_BRIEFS/` holds reviewer briefs only up to RV13. Their shape, to reuse:
  1. Role and independence: "You did not write the code; don't rely on the implementer's tests as oracles."
  2. The candidate: PR, exact head SHA, base, implementer.
  3. Absolute paths: `<wt>`, T3/, `<VENV>`.
  4. What to read first: `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md`, the Mac host rules, the brief, the slice's rulings sections, the slice's RETURN and plan.
  5. Review items in priority order, each naming the claim to break and the evidence to produce: independent oracles, a differential harness against a verbatim copy, single-edit mutants, and re-running a sample of the gate.
  6. Host and method: `git archive` into `<wt>/rv<n>/`, own target, one cargo job at `-j 4`, the memory guard, no Git writes.
  7. Output: `T3/REVIEW/<SLICE>_REVIEW.md` in the format of the last review (a verdict, counts, and a findings table with file:line, evidence and remedy), records in `T3/REVIEW/_run_records/<slice>_review/` with README and SHA256SUMS, placeholder paths only; ROOT commits.
  8. "End with the verdict, the counts, one line per finding, the report's sha256, and anything ROOT must rule on."

  Confirmations and delta checks reuse the same agent (`SendMessage`), listing exactly what changed and what to check.

## 6. ROOT's own errors, and the fix that works

- **This week ROOT made at least seven figure or claim errors in rulings** (handoff §7.2 item 1), plus one in this file's first draft (RV26-S1). **It also made several process slips** (RV26-S2):
  - rulings left only in messages, and recorded late (`ROOT_RULINGS_V1.md` :1152, :1539, :1894);
  - asking TASKs to prepare merges, an index operation their rules forbid (:2457; handoff §7.2 item 2);
  - the work graph lagging the rulings (§7.2 item 7);
  - merging without checking a main move (§7.2 item 6).
- **Common cause:** compressing a table from a RETURN into one sentence loses its qualifiers:
  - the unit (MiB/1,000 written as GB, twice);
  - the basis of a comparison (`k6_observe`'s fixed term against `vk_scale`'s measured peak);
  - which size a range belongs to (65–188 s spans two sizes).
- **The corrections themselves went wrong twice.** The S1 correction said E_max "has not been shown to fail" a phase that, by construction, it does not bound (RV25-D1). And a ruling said fixes to new text were bracketed when they had been reworded (RV25-D2).
- **What works:**
  - **cite, don't restate:** "KF3 RETURN §13" rather than a number;
  - when a number must appear, **copy it from the source in the same turn, with its unit and basis**;
  - **never write "nothing relied on it" or "no effect"** without the check;
  - **correct a ruling with an in-place bracket, never a rewrite.** Text that is new in an unmerged PR may be reworded, but say so in the PR body;
  - **before correcting, read the source's own conclusion sentence**, not just its table.

## 7. Judgment calls, and how they were made

- **SHOULD-FIX findings are fixed before merge, even test-only ones,** where the fix costs less than a day. This week that covered RV20-1, RV21-1/2, RV22-1/2/3, RV23-1, RV24-1, RV25-S1/S2 and RV26-S1/S2. Earlier in the same week, RV11, RV13, RV14, RV16, RV17, RV18 and RV19's SHOULD-FIX findings were fixed before merge too (RV26-N1).
  - The practice kept "review PASS" meaning something.
- **A NOTE is left as a NOTE** unless one of these applies:
  - a test is cheap and protects a ruled decision (RV23-N1's precedence test);
  - it corrects a published claim or a figure (RV25's NOTEs).
- **Prefer fixing the evidence to narrowing the claim** when a later slice will consume the evidence. RV23-1 was fixed because F2a will publish `bound_refusals`.
- **Split a change out** when it would alter published bytes or classes beyond what the slice promised. KF2's dense-screen change was honest, but it moved report bytes and dense classes, so it is routed to a separate slice, not yet scheduled, with an owner-facing note.
- **Coupled slices use "whichever merges second adapts":** the export shared by K6b and V-K, and K6b's parity check tightened by KF3. Neither slice waits for the other.
- **A stop is a result.** The most useful findings came from stops, such as K6b's backstop, K6B-S3's parity stop and V-K's THIN. Diagnose and route; don't force past a stop.
- **When a finding belongs to an earlier slice's code,** such as KF3-B2's E_max omission, route it to that slice's follow-up (K6c) rather than widening the current slice. Record the reason.

## 8. The environment's quirks (Claude Code on macOS, zsh)

- **zsh:**
  - quote globs passed to tools (`--include='*.rs'`);
  - a bare `echo ====` fails, through zsh's `=` expansion;
  - macOS `cat` has `-e`, not `-A`;
  - `sed -i ''` needs the empty suffix.
- **Removals:** the host's safety check refuses `rm` on a variable path such as `rm $D/$J`. Use literal absolute paths, or `"${D:?}"/"${J:?}"`.
- **Long jobs** (DEC-025) run as background shell tasks, which notify when they finish. Don't poll them. **Never read a subagent's `.output` transcript file**; it will overflow the context.
- **The session scratchpad is not persistent.** Anything a successor needs, such as the DEC-025 suite-comparison script, goes into the records (`HANDOFF_2026-09-30_TOOLS/`).
- **`gh` reads:**
  - `gh run list --commit <sha>` shows a head's CI;
  - `gh pr checks <n>` shows pass and skip counts;
  - `gh run view <id> --json jobs` shows each job's start and end times.
  - A dispatch's `target_base` must be a full SHA that is an ancestor of the head. Copy it from `git rev-parse origin/main`.

## 9. Working with the owner

- **The owner prefers the cleanest path over speed,** and said so in this session ("not in a rush"; not otherwise recorded). Offer a stop point when it's clean; don't push to finish.
- **At closure-type points** (a pause, a handoff, anything hard to reverse), the owner may ask to hear intentions before any action; they did so in this session. Then state the plan and wait.
- **Status answers:** say plainly what is complete, what remains and what has not started (for example, T4 onwards), with numbers.
- **Answers meant for other agents** must be committed where those agents can read them, not only said in chat.
- **Surface your own errors plainly,** in the records and to the owner. The owner's stated aim for the audit is to find issues early, not to catch anyone out. Candor makes that possible.
- **Owner decisions:** keep the list short, explicit and "not needed yet" until it is needed. Don't ask what the rulings already let ROOT decide.

## 10. If I were starting again

1. Check main immediately before every merge. Make it part of the merge command, not memory.
2. Keep a small "figures" table in each ruling that cites a RETURN, copied with units and basis, instead of prose numbers.
3. Update the work graph in the same commit as each merge ruling.
4. Ask every implementer to record mutant diffs, and every brief to name the file that holds them.
5. Put tools ROOT relies on (comparison scripts and the like) into the records the day they are written.
6. Open records PRs more often (after every one or two merges). This one carried 1,031 files, which made its review heavier than it needed to be.
