# T3 audit resumption records merged — PR1068

ROOT (HELP_HUMAN, Agent 0) merged [PR1068](https://github.com/sgttomas/chirality/pull/1068)
at 2026-10-01T06:38:11Z as `546e05a159a58f5ceffe1d315373bc2f31982ea8`.
Candidate: `729e80b5c2a33b278623a850d9c75c25372b688c`.
ROOT fetched and checked main `a38617d08bfbff6ea0a15b1cb129a16cbc82b2fc`
immediately before `gh pr merge --merge --match-head-commit` at the exact candidate.
GitHub's merged response is preserved in `_run_records/PR1068_MERGED.json`.

## What reached main

Records only: audit rulings, response PR1066 disposition, original raw-evidence
preservation and provenance, bounded dispatch records, and the C17 BLOCKING
escalation with a durable pin to its independent diagnostic evidence. This accepts
no repair or unfinished response design. The owner identified and stopped the
response's supporting-tool drift; the historical records retain their bytes.

## Pre-merge gates

RV27's independent review and backchecks are in `../../REVIEW/RESUME_RECORDS_RV27/`.
The initial SHOULD-FIX was repaired and confirmed. The final `escalation_729e80b5`
and `gatecheck_729e80b5` reviews pass on the actual candidate with no unresolved
merge-blocking finding; the latter binds all gate results below and their limits.

- Hosted full-SHA dispatch 36819769945 succeeded at this candidate, target_base
  a38617d. Its preserved plan has full coverage and numerical_required=true.
  All PR checks succeeded or were skipped as selected. Earlier cancelled dispatches
  are not counted as passes.
- GEN-8 passed on the final head. The final practitioner suite recorded 419 passes
  (RV27 final gatecheck, Final-head binding).
- DEC-025 ran on the M5 Max at the exact final head under the existing guard.
  Original fail-fast and every no-fail-fast log remain at
  `<wt>/scratch/sweep_resume_records_729e80b5/`; KF2 remains the baseline.
  RV27 independently compared all 40 manifest keys. The same three known Mac
  failed-test names remain. The original self_weight_wasm compiler failure ran
  no tests; a named isolated, locked, unchanged-source check passed 14 tests.
  `_run_records/isolated_DISPOSITION.json` and execution binding preserve that
  limited substitution; zero effective count/failure-name differences remain.
  The original fail/incomplete sweep is not rewritten as a clean pass. There was
  no immediate pre-isolated HEAD sample; the setup/history/postcheck binding and
  its limitation were independently reviewed. This does not claim tool repair.
- Python, wasm, Vitest and production build completed successfully, as detailed
  with counts and original logs in RV27's final gatecheck.
- T9/both-entry were not required for this records-only delta; no product source changed.

`_run_records/MANIFEST.json` hashes the copied gate/merge facts. Original scratch,
all original suite logs and preserved audit raw evidence remain; no prune occurred.
No numerical closure follows from these records gates.

## State after merge

AUD-T3-01 remains BLOCKING; F2a reliance on A1 remains held. The independently
reviewed bare-b correction is selected for bounded implementation on the A1 branch.
I22's initial four-file source draft is frozen but uncompiled/unverified. K6c has
source accounting only, no accepted E_max. ROOT integrated main into the numerics
branch with a merge commit, preserving the durable C17 addition and later design
selection; this record, the merged ruling and graph update were made together.
