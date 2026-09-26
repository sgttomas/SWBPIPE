#!/bin/sh
# REVIEW_B: regenerate the WP3 delete-control golden from the base revision's applier.
# Run in a scratch copy of the candidate (git archive of 6ded3e347), from WORKING_ROOT.
set -e
A=core/model_operations/operation_applier
cp fixtures/model_operations/load_reference_delete_control.json "$SCRATCH/golden_committed.json"
git -C "$REPO" show d8f0dc4f7:projects/chirality-piping/$A/src/lib.rs > $A/src/lib.rs   # base lib.rs (no load_state_authoring)
rm $A/src/load_state_authoring.rs $A/tests/load_state_authoring.rs
(cd $A && LOAD_REFERENCE_DELETE_CONTROL_BLESS=1 cargo +1.97.1 test --locked --offline -j 2 --test load_state_delete_control)
cmp fixtures/model_operations/load_reference_delete_control.json "$SCRATCH/golden_committed.json" && echo GOLDEN_IDENTICAL
