# I19: implement slice KF3 (W1a at scale: a certified bound that cannot be formed is unavailable, not an attempt stop; partial stage work recorded)

> This is an implementation TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host" and "Platform calibration", `I8R_K1_RESUME.md:24-50`) override `_COMMON.md`'s host section, and apply to you in full.

## Roles

- ROOT (HELP_HUMAN) dispatches you directly, as a background subagent, and is your return path.
- Make no Git writes and no index operations. ROOT commits.
- Record the delegation mechanism in RETURN.

## Paths and bases

- `P/`, `T3/`, `FK`, `K4R` (`FK/structural/retained/`) and `K4T` (`FK/tests/retained_k4/`) are as in `I18_KF1_IMPLEMENTATION.md`.
- **The base is main `0f5d8c7b4`** (K4 and KF1 merged). ROOT records the actual base at spawn. T3's records are read in `<wt>/numerics`.

## Purpose

At 10,000 members, W1a publishes nothing on five of R1's six RF-LARGE frames.
- **The evidence:**
  - K6b's W1-T4, row 247;
  - V-K's B, tier V3: CHAIN-AX and -ROT, TREE-AX and -ROT, and CONT-ROT end `Unresolved(ExactSumSpan)`, while CONT-AX is selected at 128 and passes.
- **Where it stops, the same on all five:**
  - the 128 candidate is rejected by its verification;
  - the 256 verification's shared build then stops with `Span`, after its bounded and wide formation, and before its `uc` stage records any work (`verify.rs:471-485`: `gamma_m`, then `uc_bounds`).
- **The rulings:** "K6b: slot K6B-S3 stopped at W1-T4's first row …" and "V-K: B prepared; ExactSumSpan recorded …" in `ROOT_RULINGS_V1.md`.

ROOT's reading:
- `uc_bounds` (`bound.rs:405-420`) runs directed recurrences (`u_pass`, `nl_pass`) over the factor's profile. On a long chain the comparison-matrix bound grows geometrically, so an exact sum exceeds `ExactWideSum`'s span, and the whole attempt stops.
- **But R7's certified bound is B = min(Uc, S) per block** (Lemmas D and E), and S is independent of Uc. A bound that cannot be formed can be taken as +∞ with every step still certified.

Separately, the verification's shared build records no stage for the partial work of a stage that stops (`verify.rs:471-485`), although the charged total is right. F2a will publish that evidence.

## Scope

1. **Diagnose first** (checkpoint 0): which exact sum spans, in which function, and why, on RF-LARGE-CHAIN-n10000-AX and on a smaller model that reproduces it, if one exists. State the growth law.
2. **An unavailable bound is +∞.** If Uc (Lemma D) cannot be formed for a block because its exact arithmetic exceeds the sum's span, or overflows, it is unavailable for that block, and B = S where S is available. Likewise for S: if its shift fails, B = Uc, as today.
   - The attempt stops only if neither bound is available for some block that needs one.
   - The unavailability is recorded in the evidence, per block.
   - **Check R7 §5's text and K4's code for every other reader** of Uc, S and B (θ, g, the charge, E, ê, Φ, the classification and the evidence). Each must be consistent with B = min over the available bounds.
   - **This is D1 revision 5a.3 amendment A2,** a ruling ROOT records. Your RETURN derives that it changes no honesty step.
3. **Partial stage work.** On every error path of every build that records stages (the verification's shared build, and any other you find), record the partial stage's work, so that the stages sum to the charged total on every path, completed or stopped.
   - K6b's parity check then holds with equality everywhere. Say so, so K6b can tighten it back after KF3.
4. **Nothing else changes.** Every control's outcome, every published row, class and bound, and every golden work count is unchanged, except where a control previously ended in an unavailable-bound stop. List every such control and its new outcome.

## Write set

- `K4R/{bound,verify,adaptive}.rs`, as needed.
- `K4T/`: tests, GEN (`gen_k4_vectors.py`) where it models Uc or S, and the vectors.
- `T3/IMPLEMENTATION/KF3/`.

Anything else is a stop.

## Required tests

- **Unit tests:** a block whose Uc cannot be formed yields B = S, with the evidence recorded. A block where neither is available stops the attempt as today.
- **A constructed model,** small enough for CI, whose outcome moves from an unavailable-bound stop to selected under KF3. It must be honest against GEN's exact solution under `compare_honest`, with G5a passing.
- **The partial-stage identity** on every error path. This includes a budget stop inside `uc`, as K6b's test does.
- K4's full suite, KF1's tests and FK's full suite. The controls are token-equal to GEN, with GEN updated to mirror the rule if it models Uc. `gen_k4_vectors.py --check`.
- **Evidence at scale**, run as an example and not in CI: RF-LARGE-CHAIN-n10000-AX through W1, with its outcome, and its honesty against R1 where it publishes. Run it in a slot ROOT grants.
- **Mutants:** Uc's unavailability treated as 0 (not +∞), B taking the maximum, the stop dropped when neither bound is available, and the partial stage left unrecorded. The NONE control passes.

## Gates (ROOT runs the PR)

- An independent reviewer, directed to the honesty argument for B = min over the available bounds, and to every reader of Uc, S and B.
- Hosted CI with the full-SHA dispatch.
- DEC-025 with a fresh sweep target.
- GEN-8.
- **Kernel only:** `retained` is private on main, so T9 and the gate are not run.

## Checkpoints

- **0: the diagnosis and plan,** with no code.
- **A:** the change, the tests, the suites and the constructed model.
- **B:** the scale evidence, in ROOT's slot.
- **D:** RETURN, CHANGE_RECORD, `_run_records/` and SHA256SUMS.

**Stop and report** on any change to a published result outside the listed controls, on any honesty step that depends on the change, on an edit outside the write set, or on a surviving mutant.

## Host

- One cargo job at `-j 4`, `RUST_TEST_THREADS=2`.
- Your own target, `<wt>/kf3-target`.
- The memory guard running.
- No building during a timed slot ROOT grants to another slice.
