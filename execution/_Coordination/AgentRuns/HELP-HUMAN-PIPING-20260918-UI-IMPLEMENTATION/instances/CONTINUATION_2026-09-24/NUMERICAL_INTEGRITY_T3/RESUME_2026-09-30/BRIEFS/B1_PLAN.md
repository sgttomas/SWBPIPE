# B1-P: B1's implementation plan (documents only)

TASK (Type 2), a planner dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance.** Your ID names a records folder and a role, with no memory of earlier sessions. Earlier holders of related roles (I61, who planned U8; I74, who planned T6S; I78, B0; I81, the probe; I82, the study) left their work in records. Cite them, and assume nothing beyond them.

## Why

B1 widens the F2a D1 milestone from one load case to multi-case, without combinations. It is the largest unit left before PR-B1: about 37–59 h agent time and 12–17 h review (DESIGN_v2 §9).
- The contract is selected (B0).
- The witnesses are established (I81's probe).
- The cap and M target is chosen (I82's study, P1).

ROOT needs a concrete plan before dispatching implementers. U8 and T6S were planned the same way (I61's PLAN; I74's PLAN).

## The basis

- **The contract:** `R/I78/b0_contract_01/DESIGN_v2.md`, the whole of it: T-1 to T-13, §1.3 and §1.4, §2 (D38), §3 (F-1 text B and the G8/G5 alignment), §6 and §7. Its selection is RR "B0 selected on DESIGN_v2; C3a's names reserved; B1 opens with a probe and the cap/M study".
- **The probe:** `R/I81/b1_probe_01/PROBE.md`, with RR "I81's B1-0 probe verified; W-C2's case C established; the re-basing ruled".
- **The study:** `R/I82/b1_cap_study_01/STUDY.md`, with RR "I82's study: P1 adopted as B1's target; B1's plan dispatched". The target is P1, with its two tiers and M = 5.25 GiB, selected finally at B1's G6 by measurement, with at least a 5 % text-error budget and the fallback ladder.
- **The parent plan:** `R/I61/u8_plan_01/PLAN.md` §2 (the breadth units, the re-qualification obligations and decision 8's two main-bound PRs) and §5.2.
- **The milestone record:** `T/IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md` and `copies/QUALIFICATION.md`, including §11, the re-qualification carry: RV87's sweep or an explicit-row rule.
- **The method:** U8's plan and its briefs (`R/BRIEFS/U8_COMMON.md`, `I68_U8_PROBE_AND_WITNESSES.md`, `I72_U8_PASS_B.md`); T6S's plan (`R/I74/t6_slice_plan_01/PLAN.md`); and the T3 gate set (RR "T3's gate set and Git rules, consolidated after the handoff was made ephemeral").
- **B6, which runs beside B1** (`R/BRIEFS/B6_READER_ITEMS.md`). It is the corpus's single writer before B1's snapshot. Assume its corpus edits land first, and plan B1's re-pin on top of them.

## What the plan must give

1. **Slices.** For each slice give:
   - its owner role;
   - its exact write set, with the files named;
   - its content and acceptance;
   - its dependencies;
   - its estimate.

   Cover:
   - the producer's n-case transaction in PP: T-1 to T-13, T-4 keyed on the verdict, decision 21, `NoTriggeredCase`, the cap rows per tier, `REGISTERED_PROFILES`' `threshold_bytes`, G-B and G-C per tier;
   - the three readers: R-D38, F-1 text B, the G8 alignment, RS's `not_required` rule, and T-12's base readers for several notices;
   - the corpus re-pin, on the L = 0 bases, with the W-C2 pins, D38's synthetic pin and F-1's entries;
   - the witnesses: W-C2's three cases, the re-based W-C1 and W6 on case C, W2b's replacement under its stop rule, the `NoTriggeredCase` pins, and W2 staying in A;
   - the carried notes for B1 in DESIGN_v2 §7, including RV97 R2-N-2, I77's citations and the T6S consistency checks.
2. **Re-qualification,** per PLAN §2.1 and QUAL §11, and I82 STUDY §6:
   - G5 (TEXT and profile per tier);
   - G6 (`admission_bound`, the pure maximum, and the registration diff with the new M);
   - the S1 stack witnesses at R/16 in both modes;
   - the challenge peaks;
   - RV87's sweep or the explicit-row rule;
   - the full 40-manifest suite before the freeze;
   - Pass B, since B1 changes the D1 call graph.

   Say what each must show, and which slice or reviewer produces it.
3. **Order and parallelism.** Order the slices so independent work runs at once, under the host rule of one heavy job at a time. Name the points where ROOT rules: for example, M's final selection after G5 and G6, and W2b's stop rule.
4. **Reviews.** Name the independent reviewers' scopes for each slice and the oracles they need. Plan for fresh IDs; ROOT assigns them.
5. **PR-B1's packaging.** Say whether B6 travels with B1 (PLAN §2.1) or goes separately. B6 touches no D1 code, so it could go first with lighter gates. Give a recommendation and its reason. Give PR-B1's gate set.
6. **Decisions.** List every choice the plan makes, with your recommendation and its decider. The owner-held list in the work graph's T3 section is authoritative; never decide those. M ≤ 6.0 GiB is ROOT's (owner, 2026-10-06).
7. **Risks and stop rules:**
   - memory: the margin at P1, and the ladder;
   - W2b;
   - the reader alignment's re-pin cascade;
   - the stack depth with several cases.

## Rules

- **Documents and code reading only.** You may run read-only Python with VENV against committed files.
- **Not allowed:** cargo, vitest, native or solver jobs, installs, Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`.
- **Scratch** goes in `WT/scratch/<id>_b1_plan/`. Nothing goes to the system temp directory.

## Output

- **The record:** `R/<id>/b1_plan_01/PLAN.md` plus SHA256SUMS, with placeholder paths only.
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/<id>_b1_plan/records/` and say so.
- **Budget:** 4–6 h.
- **End your turn with:**
  - PLAN.md's sha256;
  - the slice list with estimates;
  - the numbered decisions;
  - the PR-B1 recommendation;
  - anything that would change the breadth order or the owner's F2a order.
