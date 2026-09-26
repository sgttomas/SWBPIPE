# TASK R1 — independent review of the T0R producer (S1 + S2a/S2b)

You are a fresh-context, non-author reviewer (TASK, Type 2) under the T0R WORKING_ITEMS manager. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `projects/chirality-piping/AGENTS.md`. **Read-only:** do not edit any file in the repository and make no Git write. You may build and run tests with `CARGO_TARGET_DIR=/home/user/wt/t0r-r1` (delete it when done) and write scratch probes outside the repository.

## Candidate
Worktree `/home/user/wt/engine`, commits `cbec393f3` (S1 table + interface) and `dcbd3cad7` (producer), over main `45a5381c8`. Review the complete diff `git diff 45a5381c8 dcbd3cad7 -- projects/chirality-piping/core projects/chirality-piping/fixtures projects/chirality-piping/validation` plus the S1 records in `DEFAULT_ROUTE_DESIGN/IMPLEMENTATION/`. Other uncommitted files in the worktree belong to parallel TASKs (readers, Python, TS); they are out of scope for R1.

## Basis
`DEFAULT_ROUTE_DESIGN/DESIGN.md` revision 3 with `ROOT_SELECTION.md` amendments F-1 and N-A, `ROOT_RULINGS.md`, `IMPLEMENTATION/S1_INTERFACE.md`, the frozen `references.py` / `_run_records/references.stdout.txt`, and `../OWNER_SIF_DECISION_2026-09-26.md`.

## Look for (prioritise what can publish a silently wrong result)
1. Physics: support device laws and attribution (rigid residual, spring, constant effort, nonlinear); the certified maximum and its thermal axial term; headline selection and identity ties; the intensified measure (formula, which ends, arcs excluded, no default factor, never combined); arc labelling (chord-frame end rows, nominal basis on arc stress rows).
2. Combination gates and algebra: the three gates and their order; subtraction and range handling; support grouping and recomputed magnitudes; that no maximum, intensified, nonlinear, review or record kind is combined; that gated combinations have no rows.
3. Rendering contract: retired kinds and codes removed; every `result:` diagnostic ref and summary `result_ref` resolves (F-1); `contract_evidence` exactly per S1 §5; `HIGH_DISPLACEMENT_REVIEW` on the all-case value; blocked envelopes; source-blocks-1 output byte-unchanged when a case is source-selected; exact route unchanged.
4. SF-E imposed-displacement refusal: scope (pre-0.4 only), no solve from the cleaned clone, error fall-through.
5. Tests: whether each migrated test still checks something real (no protected criterion weakened; see comments marked `T0R`); whether the runtime tests genuinely read the frozen references and apply the 1e-9 criterion without new tolerances; mutation-control record `_run_records/S2_MUTATION_CONTROLS.txt`.
6. The S1 deviation (arc discriminator as a first-match `source_basis` variant, ROOT-accepted) and the table generator.
7. T1 merge surface: are the `lib.rs` hunks minimal and adjacent as DESIGN §10.2 intends?

## Return
One SendMessage to your manager: verdict (CLEAR / FINDINGS / BLOCKING); findings, each with severity (BLOCKING / SHOULD-FIX / NOTE), file:line, evidence (a probe or test output where possible) and a concrete repair; and what you ran, with counts.
