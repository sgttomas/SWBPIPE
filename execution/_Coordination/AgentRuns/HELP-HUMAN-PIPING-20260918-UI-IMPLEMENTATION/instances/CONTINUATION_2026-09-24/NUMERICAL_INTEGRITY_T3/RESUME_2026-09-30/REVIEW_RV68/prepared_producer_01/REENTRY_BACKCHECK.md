# RV68-1 backcheck

**RV68-1 CLOSED on the supplied frozen boundary.** Whole-component acceptance
and final source coverage remain pending; RV68-2/3 and C4 observations remain.

ROOT supplied REENTRY_BOUNDARY.json, SHA-256
`cd7b91f60628904579c10c3541c90fc7e5b36de8421f813985273b6c1e34d984`.
The exact patch SHA-256 is
`e2c0c68bd3958f061bca39578d15ac22e6768a872115ce72c2cc2f6bd78e6f56`.
Both PP files were independently reconstructed from immutable `430bc4f798`;
their bytes match the manifest (157492 and 122731 bytes respectively).

PreparedCase now has a private sticky `proof_attempted` field. The method checks
it before repeat adapter work, enters the marker write before setting it, and
sets it before `begin_prepared_product`. Once proof entry occurs, success and
every proof/projection/prefinal/final refusal retain the marker. Refusals before
that point enter no draft. Production success/refusal wrappers keep PreparedCase
private and provide only borrowed capture access; they no longer return draft-
start ownership. Test-only methods bypass that privacy solely to verify the latch.

The old-source numerical-refusal test keeps the prior ProductProofFailure and
its two corrections separately, obtains the repeated-entry refusal and checks
unchanged ordinary bytes/counters. The success re-entry test likewise retains the
original passing proof with its work, rejects repetition and checks unchanged
committed bytes/counters. A final precommit accounting-fault test retains the
successful numerical proof while preserving original ordinary bytes.

The exact producing PP hashes occur in pp_retained_debug_02.json; it reports
stable source, exit0 and reap. The raw log independently reads 27 passed, zero
failed. This is inspected author execution evidence, not an RV68 Cargo rerun.

The full two-file delta was read. Fixed arrays/scans replace temporary support,
theta/resolution/bound Vec collections while preserving cardinality/identity
checks. Maximum midpoint Sub/Mul/Add are now separately counted in their existing
operation order; eight Number constructions are entered. Diagnostic formatted
work strings are test-only. These changes introduce no observed new blocker,
but they do not supply the pending complete C4 schedule.

Exact hashes and source/command association are in external
scratch/rv68_prepared_producer/REENTRY_AUDIT.json and the eventual bulk manifest.
No Cargo/model/solver/native lane, maintained-source, Git, index or API write was
used by this backcheck.
