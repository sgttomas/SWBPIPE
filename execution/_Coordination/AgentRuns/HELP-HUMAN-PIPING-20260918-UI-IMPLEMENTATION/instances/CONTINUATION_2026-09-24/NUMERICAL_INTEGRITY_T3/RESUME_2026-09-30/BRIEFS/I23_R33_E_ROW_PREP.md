# I23 — isolate the existing R33 N05/p128 E-row witness

Existing I23 gets one 10-minute source-only preparation. Use immutable40129,
the unchanged D2 patch, the fixed SCALE corpus and existing e_unit helper in
scale_tests.rs. Prepare an append-only disposable test selecting exactly N05
at128 through e_unit, with an exact one-case coverage check. The qualifying
failure remains the original E-row numeric/digest assertion with that named
case/precision; corrections, g, A-bar, body, setup/refusal or other failures
are not substitutes. Original tests, oracle/corpus and helper stay identical.

Read the actual helper/records and prove the selection and source mechanism;
do not invent a specific wrong digest from current output or require a made-up
fault token. The original correct digest/tokens remain the frozen oracle.
One witness from the originally named N05/N06,p128/256 set is selected;
unrun alternatives receive no credit. No new model, precision or expected value.

Prepare two fresh independent116-file copies (baseline/future D2) under
<wt>/scratch/i23/r33_e_row, same overlay, D2 production fault UNAPPLIED,
empty targets under <wt>/a1-diagnostic-target/r33_e_row and sibling logs.
Return code/diff/source/record/patch hashes, exact filter/qualifier and future
baseline/fault/control proposal. Write only A1 R/I23/r33_e_row_12 and assigned
scratch/empty targets. No Rust/build/test/solver, maintained change, Git/index
or delegation. RV29 review precedes runtime; stop at actual10-minute boundary.
