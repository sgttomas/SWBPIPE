# I6: implement slice K2a (checked formation)

This is an implementation TASK. Read `_COMMON.md` first; this brief overrides it where they differ.

## Purpose

K2a is the next kernel slice after K-D5 (the order is S11-K → K3a → K-D5 → **K2a** → K1 → K2b → K5; `DESIGN.md` revision 5a.2 §6). It closes the **formation silent zero** (ROOT's S11 decisions, `ROOT_RULINGS_V1.md`; V1's S11B-9 and FORM-C).
- Today `local_stiffness` forms each coefficient in binary64 (for example `g * j / length`, `12.0 * e * iy / length3`).
- An underflow becomes an exact 0, which passes the finiteness check and silently removes a member's torsion or bending stiffness.
- A partial underflow makes the element inconsistent. In R1's RF-RANGE LEF-small, 6EI/L², 4EI/L, 2EI/L and GJ/L are exactly 0, while 12EI/L³ is a normal value 3.5 % wrong, from a least-subnormal intermediate.

After K2a, any such formation is **refused with a named reason**, never published. Its realistic reach is nil (it needs section products below about 2.2e-308 in SI units). It is still a silent path, and T3 closes it.

## Basis (read in this order)

1. `T3/DESIGN_NUMERICS/DESIGN.md` revision 5a.2 (sha256 `fb62ef4a…`; hash-pinned, do not edit):
   - §4.7 "Formation range", step 1, checked formation;
   - the K2a row of the §6 slice table;
   - the "Why K2a is separate from S11-K" note;
   - §7.3 mutation 18.
2. `T3/ROOT_RULINGS_V1.md`: ROOT's S11 decisions (D-S11-1 to D-S11-4, the formation silent zero, K2a as its own slice), and the K-D5 outcome sections.
3. `T3/REFERENCES/README.md` and `references.json` (`c0f14201c`): the RF-RANGE family, in particular the LEF-small vector −(200, 300, 600) and the LEF-large vector +(200, 300, 600) on the three bases.
4. `T3/REVIEW/_run_records/r2_backcheck/probe_r2_formation.*`: V1's FORM-C probe.
5. K-D5's `FK/structural/formation_check.rs` on main, for how K-D5 re-forms straight frames from primitives. K2a must not change K-D5's behaviour.

## Base, worktree and branch

- ROOT creates `<k2a-worktree>`, on a new branch from **main after K-D5 (PR #1017) merges**. It contains S11-K, K3a, S11-F, S11-G and K-D5.
- K2a lands as **its own PR to main with full gates**: a complete-diff independent review, hosted CI with the surface-4 dispatch, a clean DEC-025 sweep, and the committed-fixture diff with its stop rule.
- Make no Git writes and no index operations. The manager commits.

## Write set (from the K2a row)

- **`P/core/solver/frame_kernel/src/lib.rs`, `local_stiffness` only.** The line numbers in the design (`:699-768`) predate S11-K, K3a and K-D5; re-locate it on your base.
  - Form each coefficient with **checked operations on every intermediate product and quotient, not only the final coefficient.** Any intermediate formed from nonzero finite operands that is zero, subnormal or non-finite is refused with a new `FrameKernelError::NumericalRange { name }`, naming the coefficient and the intermediate.
  - An exactly zero operand, for example a zero modulus, is not a range error; keep today's handling of it.
  - Keep the operation order bit-identical, so every normal coefficient has exactly today's bits.
- **`P/core/solver/diagnostics/src/lib.rs`:** the one exhaustive match outside FK (at `:285-362` in the design; re-locate it) gains the `NumericalRange` mapping, with no new diagnostic code if the existing range or formation code fits. If a new code seems needed, stop and ask the manager.
- **Enumerate every caller** of `local_stiffness` and every match on `FrameKernelError` (ROOT's recorded lesson), by lexer scan, into `_run_records/callers.txt`. That includes K-D5's `formation_check.rs`, curved_bend, and anything else that forms frame stiffness.
- **Tests** in the touched crates, and records in `T3/IMPLEMENTATION/K2A/**`.
- **Not in scope:** formation-time scaling (K2b, SCALE-W) and its facade wiring (F1). Until those land, checked formation **refuses**. That is the design's stated interim.

## Tests (from the K2a row; all required)

- **Every existing suite passes and is byte-identical,** including the committed-fixture diff (expected unchanged: no committed intermediate is zero or subnormal). **Any committed-byte change stops the work** and is reported to the manager with its site before anything is regenerated.
- **RF-RANGE LEF-small** (all three bases): refused with the named reason. The test shows the silent path on the unchecked formation first (the exact zeros, and the 3.5 %-wrong 12EI/L³, computed in the test), then the refusal. This is the paths-differ precondition.
- **RF-RANGE LEF-large** (all three bases): refused with the named reason, where today it overflows to a blocked envelope.
- **A partial-underflow control:** 12EI/L³ normal while 4EI/L underflows. Refused.
- **A subnormal-intermediate control:** a coefficient that would be subnormal but nonzero. Refused.
- **Every other RF-RANGE vector keeps every coefficient normal and is unaffected:** for example (0, −1000, 0) and ±(−120, 500, 260). Show them byte-identical.
- **The gate:** run the both-entry no-Passed-breach gate over the frozen references against main's **empty** lists (`GATE/S11_EXCEPTIONS.json`, `GATE/FORMATION_EXCEPTIONS.json`). Zero trusted breaches are allowed. The LEF cases moving to a refusal is expected; record each standing change with its cause. Use the two-part method (the known dense timeouts last, on a quiet host), following ROOT's K-D5 gate ruling.
- **Mutation 18:** revert to unchecked formation, or check only the final coefficient. The underflow, partial-underflow and subnormal-intermediate controls must fail at a **behavioural** assertion. Add your own mutants: drop the check on one intermediate, or treat a subnormal as acceptable.
- **K-D5 interaction:** a case K-D5 re-forms (straight frames in `formation_check.rs`) keeps its K-D5 outcome for every normal formation. A refused formation is refused before K-D5 runs, and that is expected. Pin both.

## Disclosure and return

- **`CHANGE_RECORD.md`,** following `.agents/skills/chirality-change/SKILL.md`. It states:
  - which cases change standing (the LEF cases, now refused), and why;
  - that no value changes for any normal formation;
  - the fixture result;
  - that the refusal is the design's interim until K2b and F1;
  - that there is no in-band marker.
- **`RETURN.md`,** with logs under `_run_records/`, SHA256SUMS and no machine paths. It covers:
  - the files and line counts;
  - each write-set item and test;
  - the callers;
  - the per-crate counts;
  - the mutation table;
  - the fixture diff;
  - the gate;
  - what was not done.
- Send the manager a SendMessage summary. Message at once if the stop rule triggers or a design item cannot be implemented as specified.

## Running things

- `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, and your own `CARGO_TARGET_DIR` under `<wt>`.
- **The cargo token is the manager's;** ask before any build. One heavy job at a time. Hold cargo while a DEC-025 sweep runs.
- Wait and kill patterns must not match your own shell.
- Keep free disk above about 8 GB.
- The authority targets are prerequisites, never scratch.
- **Mutation runs:** use a clean target per mutant (fresh mtimes; `tar -m`; remove the target), and run a no-mutation control first.
- **Never rewrite committed, hash-bound evidence;** add new files instead.
- Skip no tests and raise no timeouts.

## Addendum 1 (2026-09-27): product reach, after I6's stop report

This follows ROOT_RULINGS_V1, "K2a product reach: correction (ROOT)".
- **The partial-underflow test:** keep it, restated. Main refuses the case as NUMERICAL_INTEGRITY_UNRESOLVED (M03 `Range`) on both variants and in both modes, and K2a refuses it earlier, by name (`NumericalRange`). The precondition pins main's actual M03 refusal. Drop any claim that main publishes a wrong value.
- **Evidence:** keep the L = 2^-39 all-final-coefficients-normal pair and the threshold scan in `_run_records/product_reach/`.
- **CHANGE_RECORD and RETURN:**
  - State the benefit as ROOT restated it: a formation-layer guarantee independent of M03, meaning a named, earlier refusal plus protection for every `local_stiffness` consumer outside M03's check. List those consumers from the caller scan (for example K-D5's re-formation and curved_bend).
  - State the cost: exact-zero cases on physically absurd inputs, where main was accurate, are now refused.
  - Derive the bound (an exact zero moves a published value by at most about 2^-60 relative) step by step, with each inequality and its source. K2a's reviewer checks it.
- **The spring-carried test** stays as ruled.
- **Order:** phase 2 onward (mutations, suites, T9, the two-part gate) resumes when the manager returns the cargo token, after F1a's DEC-025 sweep and RV6's slot.
