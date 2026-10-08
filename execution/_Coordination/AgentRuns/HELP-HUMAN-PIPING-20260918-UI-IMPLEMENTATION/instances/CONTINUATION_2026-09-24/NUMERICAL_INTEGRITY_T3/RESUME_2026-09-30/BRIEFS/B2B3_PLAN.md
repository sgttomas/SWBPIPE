# B2/B3-P: the plan for F2a breadth after B1 (documents only)

TASK (Type 2), a planner dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance.** Your ID names a records folder and a role, with no memory of earlier sessions. Earlier holders of related roles left their work in records:
- I61 wrote the parent roadmap;
- I78 wrote B0's contract;
- I81 and I82 probed and studied B1;
- I84 planned B1.

Cite them, and assume nothing beyond them.

## Why

After B1 (multi-case at S3, without combinations), the owner's F2a order runs B2/B3, then B4 if ruled, then **PR-B2**, then B7 and B8 (RR "…the F2a order…"; work graph T3 section). B1 is in implementation now. **A concrete plan for what follows lets ROOT dispatch the moment PR-B1 merges.**

**The units** (I61 PLAN §2.1):
- **B2:** combinations, and preparation-only and mixed invocations. D1.4 widens to combinations and possibly components. It needs the C3 amendment (C3a's names are reserved: `operand_preparations`, `OperandPreparation`, `operand_preparation_failure`, `retained_precision_operand_preparation_v1`, `combination_operand`), readers ×3 for combination receipts, and carriers.
- **B3:** the promised exact routes. These are 0.3.0 exact with explicitly empty pressure regions, under the reserved `openpipestress.result_semantics/0.3.0/physics-retained-1`, and 0.3.0 `legacy_pressure_v1` with zero pressure on the ordinary route (DESIGN_NUMERICS §4.3). D1.3 changes (the namespace). **The new table binds RV78-N1's policies from its first version** (RR "I86's SW probe accepted; I83's B6 return verified and ruled; …", ruling 1).
- **B4:** cap growth above D1.9's caps, only if ruled. A 3–4 h study comes first.

## The basis

- **The parent plan:** `R/I61/u8_plan_01/PLAN.md` §2.1–§2.2 and decision 8 (two main-bound PRs).
- **B0's contract:** `R/I78/b0_contract_01/DESIGN_v2.md`, the whole of it, especially:
  - what B0 left for B2 and B3;
  - decisions 17–19 (owner-held, prepared);
  - the reserved names (RR "B0 selected on DESIGN_v2; …").
- **B1's plan and state:** `R/I84/b1_plan_01/PLAN_v2.md` (`c85786b7…`), including:
  - §9's cap shape and contingency;
  - the S3 study (`R/I82/b1_cap_study_01/`);
  - the rulings from R1 through the latest RR sections.

  B1's B2-relevant routed items include:
  - the explicit-row rule waits for B2 (PLAN_v2 §3.7);
  - RV87's sweep;
  - C3a.
- **The T6S consistency notes** (I74 PLAN §4.3), routed to the B2 and B3 briefs.
- **The owner's decisions:** M ≤ 12 GiB, and the target machines (32 GB workstations, 16 GB still practical). The owner-held list in the work graph's T3 section is authoritative.
- **The code at NUM's current maintained tree** (main `2007709549`), and B1's in-flight branches, read-only, for where B1 leaves things. Treat B1's code as moving.

## What the plan must give

1. **The units' boundaries.** Is B2 one unit or two (combinations; preparation-only and mixed)? Does B3 split by route? Recommend an order. For each unit give:
   - its contract changes (C3 amendment text, D1 clause changes);
   - its producer and reader changes;
   - its corpus and witnesses;
   - its write sets, by file;
   - its dependencies, including what each needs from B1's final state.
2. **Slices, owners and estimates,** as I84 gave for B1. Respect the host rule of one heavy job at a time and at most three implementers.
3. **Re-qualification:**
   - what PR-B2 re-qualifies once (G5, G6, the S1 witnesses, the challenge, the sweep and the explicit-row rule);
   - how B2's and B3's changes enter one registration;
   - the memory picture at M ≤ 12 GiB with combinations. I82's study priced B2's combination price beside three cases only roughly ("likely can't add a D1-cap combination beside three cases within 12 GiB"). Price it now with I82's evaluator, and say what cap shape B2 needs. If B4 is needed, say so.
4. **B4:** whether it is needed for B2 and B3 at all; if so, the study's question.
5. **Decisions.** Number each choice, with your recommendation and its decider. Never decide an owner-held one. Prepare decisions 17–19's packages if B2 or B3 needs them answered.
6. **Risks and stop rules,** as I84 gave for B1.
7. **What would change the owner's F2a order,** if anything.

## Rules

- **Documents and code reading only.** You may run read-only Python with VENV against committed files, including I82's `b1_eval.py`.
- **Not allowed:** cargo, vitest, native or solver jobs, installs, and Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`.
- **Writes:** absolute paths only. Scratch goes in `WT/scratch/<id>_b2b3_plan/`. Records hold placeholder paths only, never `~/`, `/Users/`, `/private/` or the worktree's name. No record folder is named `build`.

## Output

- **The record:** `R/<id>/b2b3_plan_01/PLAN.md` plus SHA256SUMS.
- **Budget:** 5–7 h.
- **End your turn with:**
  - PLAN.md's sha256;
  - the units and the recommended order;
  - the slice list with estimates;
  - B2's memory answer at M ≤ 12 GiB;
  - the numbered decisions;
  - anything that would change the order.
