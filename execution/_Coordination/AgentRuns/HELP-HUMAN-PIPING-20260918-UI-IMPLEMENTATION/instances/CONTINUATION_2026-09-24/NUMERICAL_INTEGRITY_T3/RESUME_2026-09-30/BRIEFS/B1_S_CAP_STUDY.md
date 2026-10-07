# B1-S: the cap and M study for multi-case (pricing only; nothing installed)

TASK (Type 2), dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance.** Your ID names a records folder and a role, with no memory of earlier sessions. Earlier holders of related roles (I65, who wrote DOMAIN and the G4–G6 pricing; I78, B0) left their work in records. Cite them, and assume nothing beyond them.

## Why

B1 widens the F2a D1 milestone from one load case to c cases.
- **The ceiling today:** D1's dense W3 maximum is already 0.8929 M, 28,389,922 B under 0.9 M at M = 4,026,531,840 B (QUAL §3). T-8, T-9 and T-11 keep every attempted case's Run and frozen candidate live until staging.
- **So at today's M,** c ≥ 2 is unlikely to fit at D1's model caps (DESIGN_v2 §6 and §9; RV105 N-10).
- **The owner's decision** of 2026-10-06 lets ROOT select M up to 6.0 GiB (6,442,450,944 B) under D-7.

This study is PLAN decision 9's read-only study, extended by DESIGN_v2 §6 steps 1–5. It proposes; **ROOT selects.**

## The basis

- **The design:** `R/I78/b0_contract_01/DESIGN_v2.md` §6 (the restated cap rows, the re-pricing approach and the classification by atom), §1.2 (T-1 to T-13) and §9.
- **The review:** `R/REVIEW_RV105/b0_01/`, N-10 and N-11.
- **The pricing record:**
  - QUAL (`T/IMPLEMENTATION/F2A_D1/copies/QUALIFICATION.md`) §3 and §6;
  - DOMAIN (`R/I65/u4_g2_01/DOMAIN.md`) §1–§4;
  - I65's G4–G6 records under `R/I65/`;
  - the TEXT tool and profile (`g5_profile.py`, `profile_tree.json`, as DESIGN_v2 §6 names them).
- **The code:** PP `retained_memory.rs` (`caps`, `cap_rows`, `family_clauses`, `late_observations`, `cap_priced_maximum`, `admission_bound`, `REGISTERED_PROFILES`).
- **The rulings:** D-7 (RR:8887), "Owner decision: ROOT may raise M up to 6.0 GiB without asking", and "B0 selected on DESIGN_v2; …".

## What to produce

1. **The pricing model in (c, l, Σl):**
   - add c and Σl to the census and the TEXT tool's variables;
   - classify every roster row and atom as per invocation, per requested case or per attempted case (|A| ≤ c);
   - restate each row's monotonicity lemma.

   Work in scratch copies; change no maintained file.
2. **The maxima.** Evaluate E_mov,max + R per mode (SparseInteractive and DenseScrutiny), by the same method as `cap_priced_maximum` and `admission_bound`:
   - for c ∈ {1, 2, 3, 4, 8} and |A| = c;
   - at D1's model caps;
   - with any model-cap trade you propose (for example, smaller n or l for larger c).

   Show each against 0.9 M at today's M and at 6.0 GiB. If you use a scratch Rust driver to call the real pricing functions, run it through `WT/tools/t3_cargo.sh` in a disposable archive. Pricing only: **no solver, native or at-scale runs.**
3. **The proposal:**
   - the target caps (C, l, L, and any model-cap trades);
   - the smallest M ≤ 6.0 GiB that fits them under 0.9 M in both modes, with the margin;
   - the rows that bind.

   If the target caps cannot fit at 6.0 GiB, say so: that goes to the owner (decision 17).
4. **What measurement must confirm before selection.** D-7 selects M by measurement. Name the in-build witnesses B1's re-qualification must run (G5 profile, G6 maximum, the multi-case stack witness), and what each must show. Don't run them.
5. **The text-error budget** (1.81 % of TAV_W today, QUAL §3): how c affects it.

## Rules

- **Pricing and reading only.**
  - Python with VENV against committed files and scratch copies.
  - Cargo only through `WT/tools/t3_cargo.sh`, for pricing drivers in a disposable archive.
- **Not allowed:** solver, native or at-scale runs; DEC-025; installs; Git writes.
- **Other T3 jobs share the lock.** Wait for it, and never kill another job.
- **Scratch** goes in `WT/scratch/<id>_b1_study/`. Nothing goes to the system temp directory.

## Output

- **The record:** `R/<id>/b1_cap_study_01/STUDY.md` plus SHA256SUMS, with `_run_records/` and placeholder paths only.
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/<id>_b1_study/records/` and say so.
- **Budget:** 4–6 h.
- **End your turn with:**
  - STUDY.md's sha256;
  - the proposed caps and M, with their margins per mode;
  - the binding rows;
  - the measurements needed;
  - anything for ROOT, or for the owner (above 6.0 GiB).
