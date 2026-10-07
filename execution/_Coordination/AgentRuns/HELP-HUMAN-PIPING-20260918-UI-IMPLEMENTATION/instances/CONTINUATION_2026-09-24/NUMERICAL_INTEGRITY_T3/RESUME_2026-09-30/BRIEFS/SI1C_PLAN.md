# SI1c-P: a plan for point-path booleans over non-finite intermediates (documents only)

TASK (Type 2), a planner dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance.** Your ID names a records folder and a role, with no memory of earlier sessions. Earlier holders of related roles (I73, who implemented S-I1; I79, SI1b; RV99 and RV104, their reviewers) left their work in records. Cite them, and assume nothing beyond them.

## Why

On the ordinary point path, a boolean rule formula over a carried NaN or infinite intermediate is decided, and it can pass. Interval mode reads the same formula as U (indeterminate).
- **I79's reproducer:** with stress `x` = 1e300 and `inf = 1e300·x`, the point path gives `true`, with no finding, for each of `not((inf − inf) > 100)`, `(inf − inf) ≠ 100` and `inf ≥ 100`. Interval mode reads all three `Indeterminate` (`non_finite_enclosure`).
- **Through the runner's boolean mapping,** such a check would read `USER_RULE_CHECKED`. A quantity formula is safe, because its non-finite value blocks at the synthesized compare.
- **SI1b left it alone,** because changing it changes results for inputs that do not panic.

T3-SI1c owns it (RR "RV103 passes #1103; …", item 1, and "#1106 merged: T3-SI1b is on main"). It carries two notes from RV104:
- **N-4:** a NaN or ±inf caller value is reported as `MissingRequiredValue` while `bound_inputs` says `supplied: true`. A NaN or +inf slot limit is reported as a units-metadata `RULE_EVALUATOR_ERROR`. Both block, but name the wrong cause.
- **N-5:** the doc comments on `Expression::Logical` and `Select` say "diagnostics in either operand always surface". In fact evaluation stops at the first blocking operand.

The work graph's row: "A plan first; whether a pass-to-block change alters public meaning is decided with the plan (owner-held if it does). S-I2's planning accounts for it."

## The basis

- **The code:**
  - `P/core/rules/expression_evaluator/src/lib.rs`: the point path, `eval_expression`, the boolean and comparison arms, and `Logical` and `Select`;
  - `P/core/rules/rule_check_runner/src/lib.rs`: the boolean mapping, `run_one_check`'s binding, and the limit block;
  - the interval path S-I1 added, as the reference for what U means.

  All are read at main `025c1cf326`.
- **The records:**
  - `R/I79/si1b_01/RETURN.md` §4 and its probe (`_run_records/probe/`);
  - `R/REVIEW_RV104/si1b_01/REVIEW.md`: N-4, N-5, and probes 12–14 and 17;
  - `T/IMPLEMENTATION/SI1B/CHANGE_RECORD.md` §3;
  - `T/IMPLEMENTATION/S_I1/` and `R/I73/s_i1_01/` (S-I1's design of U and its point-mode invariants);
  - `R/I61/u8_plan_01/PLAN.md` §3, and its decision 14 (S-I2's constraints).
- **Who reads rule-check results:** find every product reader of the runner's outcomes (the desktop's Rule-check panel, exports, reports and any others). Say what each would show before and after each option.

## What the plan must give

1. **The exact set.** Which point-path forms over non-finite intermediates are decided today, and how each reads in interval mode:
   - comparisons, `not`, `and` and `or`, `Select`, and any other boolean-valued arm;
   - NaN against ±inf;
   - intermediates produced inside the formula against caller-supplied ones (which N-4 already blocks).

   Give code references, and a table of form, today's point result and interval result.
2. **Options for the remedy,** each with its rule, what it blocks, the finding code and message, and its effect on every input that does not reach a non-finite intermediate (required: byte-identical). At least:
   - (A) the point path blocks with `NonFiniteInput` whenever a comparison operand is non-finite;
   - (B) only NaN operands block, and comparisons with ±inf stay decided;
   - (C) the behaviour is kept and stated (documentation and a declared limit).

   Say how each aligns with interval mode's U.
3. **Public meaning.** For each option, does it change public meaning? A check that read `USER_RULE_CHECKED` would block instead. Prepare the owner's decision as a concrete, reviewable package: the question, the options, your recommendation and its reason, and what each reader would show. Decide nothing that is owner-held.
4. **N-4 and N-5.**
   - **N-4's wording:** the cause named for non-finite caller values and limits. Note whether it changes outputs (it does), and whether it rides with the remedy.
   - **N-5's doc:** a comment-only change.
5. **The slice.** Give:
   - the write set, with files named, and the tests, including a differential method like I79's and RV104's (base against candidate, with only the intended lines differing);
   - mutants;
   - the gate set (the product gate set, and whether Pass B applies: the rules crates were outside PP's closure for SI1b);
   - the reviewer's scope and oracles;
   - estimates.
6. **S-I2.** Say how each option affects S-I2's binding of solver bounds into rules.
7. **Decisions.** List each choice, with your recommendation and its decider. The owner-held list in the work graph's T3 section is authoritative.

## Rules

- **Documents and code reading only.** You may run read-only Python with VENV against committed files, and read I79's and RV104's probe outputs.
- **Not allowed:** cargo, vitest, native or solver jobs, installs, and Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`.
- **Waits:** you should need none. If you do wait on anything, use one wait per job, and end it when the job's process has gone.
- **Scratch** goes in `WT/scratch/<id>_si1c_plan/`. Nothing goes to the system temp directory.

## Output

- **The record:** `R/<id>/si1c_plan_01/PLAN.md` plus SHA256SUMS, with placeholder paths only.
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/<id>_si1c_plan/records/` and say so.
- **Budget:** 3–5 h.
- **End your turn with:**
  - PLAN.md's sha256;
  - the exact set, in brief;
  - the options, with your recommendation;
  - whether the owner must decide, and the question you would put;
  - the slice and its estimate.
