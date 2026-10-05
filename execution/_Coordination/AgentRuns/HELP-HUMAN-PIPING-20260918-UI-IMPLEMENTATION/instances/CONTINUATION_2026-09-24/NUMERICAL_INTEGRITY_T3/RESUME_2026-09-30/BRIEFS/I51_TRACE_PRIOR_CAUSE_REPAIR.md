# I51 — preserve the prior capture failure (RV74-F1)

Same TASK Type 2 under ROOT. This is a narrow repair of frozen source
7018513af3d6574be382d5c3aaee01e36cb0b28e; no new scope or clock reset.
Original source cutoff 2026-10-03 16:57:40 UTC and seal deadline 17:17:40 remain.

RV74-F1 is SHOULD-FIX before fan-in. prepare_owned_case currently detects an
existing capture error, creates a generic custody/permit error and overwrites
the original cause. Its preceding adapter.require can likewise replace a prior
cause when a sticky adapter fault accompanies it. The new typed prefix must
retain the actual prior owned cause and work. ROOT read these exact source paths.

Write only CODE/P/core/product_physics/src/retained_product.rs and
retained_product_tests.rs. Preserve the prior error without cloning/parsing its
text, fabricating a source/Run, clearing sticky work, changing ordinary output
or entering helpers. Preserve actual old operational result/work and the
captured_prefix/prelude distinction. A new genuine prelude error in the absence
of any prior cause still reports itself. Trace/copy accounting stays honest.

Add a direct typed assertion of the original cause/variant/payload. Cover a
prior cause alone and with an accompanying sticky adapter fault, verifying that
both the cause and fault/work survive. Keep no new helper/evaluator/native entry,
original ordinary bytes, and existing numerical controls. Do not merely assert
that some error exists. Earlier sealed source/evidence remains unchanged.

The Cargo lane is now yours: RV74's original-candidate confirmations finished
and all processes are reaped. One additional focused PP private-suite invocation
is authorized for this repair, under existing target/cache/guard, locked/offline,
four jobs/two threads and 1200-second wall. This is a justified extra command for
a newly discovered defect, not a silent reset of the completed eight-command grant.
No FK/other source or test changes, tools, installs, descendants or Git/index/API
writes. Freeze/release promptly; RV74 owns independent delta confirmation afterward.

Own only NEW CODE/R/I51/prepared_trace_repair_08 and
WT/scratch/i51_prepared_trace_repair_08 for this repair's compact evidence.
Use anchors from I51_PREPARED_TRACE; explicitly WT is
/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3.
Return the exact small delta, updated source hashes, actual checks and preserved
original-cause/work evidence. No public activation or acceptance follows.

