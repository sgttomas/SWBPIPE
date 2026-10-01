# Replacement independent oracle — fresh TASK

Use A1_ORACLE.md and COMMON.md with these specific overrides. The prior worker
inadvertently saw matrix.json expected_rows before freezing its derivation and
has been stopped. Its interface suggestions and partial work are not accepted.

Your write scope is R/oracle_fresh/** in the A1 worktree and
<wt>/scratch/a1-oracle-fresh/**. Do not read the prior worker's oracle directories
or suggestions. Do not read response matrix.json, oracle.py, COMPARISON files,
expected-output files or B01/B02 numerical run output until independent truth
derivation and expected values are frozen in a hashed checkpoint.

Use response src/cases.rs and src/main.rs only to identify the primitive-source
definitions and output grammar. Independently derive all mathematical truth
from those source inputs, supported by accepted D1/D2 and source contracts.
Standard-library Python only; no Rust or solver run. Verify the published-claim
denominator directly from its accepted definition. If records conflict, state
the exact conflict and evaluate both predicates rather than silently selecting
one or importing the old comparator's choice.

Record consulted origins/hashes and actual ordering. Any unintended exposure
returns immediately. After the independent freeze, ask ROOT for the preserved
output path needed for comparison; this is a ROOT checkpoint, not a new owner
approval requirement. No other instruction or grant changes.
