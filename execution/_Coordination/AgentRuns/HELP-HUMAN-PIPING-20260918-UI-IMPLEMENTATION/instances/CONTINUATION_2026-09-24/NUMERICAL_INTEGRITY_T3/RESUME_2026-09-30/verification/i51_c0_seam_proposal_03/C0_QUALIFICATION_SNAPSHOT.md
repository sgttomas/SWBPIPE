# C0 sequencing miss and bounded correction

I51 prematurely executed the native solver before completing the selected C0
local helper/return-temporary and nested-copy/capacity schedule. ROOT caught this
in checkpoint review. FIRST_NATIVE.json, its command/log and C1_SOURCE.patch
remain unchanged. They prove the actual prepared-source p = 128 / P = 256
admission in both modes, not completed C0 accounting readiness.

Established before that run: actual preparation sizes (frame 3888, Endpoint 144,
NumericWork 400, WideContext 80, ExactWideSum 2144, PreparedAnnulus 56, conversion
outcome 24 bytes; alignment 8), 19 directed geometry entries, nine conversions,
27 initialized endpoints, 26 entered assignments, and ten top-level snapshot
capacity fields. Missing: the transitive helper/return-temporary lifetime schedule
and the broad cloned model/build/material/application graph's nested capacities
and entered copy/allocation prefixes. LibraryBoundary was not complete accounting.

Correction box received at verified 10:18:04 UTC; due 10:33:04 UTC. ROOT permits
only compilation and explicitly filtered layout/accounting checks, with no model
or native solve, until the concrete boundary is checked. Original component
milestones stay in force.

## Concrete capture dependency and proposed fix

Delete DeferredCaseInputs and all broad clone graphs. At the existing early
case_source hook, the prepared-only mode records just the actual case id and
hook-presence counter for later ordinary observation custody. Keep default
old diagnostic behavior unchanged. At a new late prepared-only hook immediately
before the final LoadCaseSolve return, all actual model/build/material/application/
boundary operands are still borrowed and source_case finalization is complete.
Require the existing one-case scope and source_selected = false; then execute
current old-to-old/source capture directly from those actual operands. In this
one-case scope, that final case-selection fact is the invocation coexistence
permit. A source-selected case records no W1 source/geometry/solve/draft work.
The final ordinary envelope is still checked before genuine new preparation.

This requires one additional exact file: PP/src/lib.rs, only the five-line late
observer hook in C0_PROPOSED_LIB_HOOK.patch. No arithmetic, solver, maximum,
ordinary routing, public entry or ordinary output change. The existing early
hook stays in place. The PP retained_product.rs prepared-only state changes
remain inside the granted fence. The proposed lib hunk is not applied without
ROOT's scope disposition. This eliminates the broad owner graph rather than
guessing its allocation total. Existing captured scalar/string/vector owners
and new prepared-source child lists still need explicit checked capacities and
copy prefixes; those are being corrected inside retained_product.rs.


## Frozen seam guards and scope proof

C0_SEAM_PROPOSAL.json in assigned scratch pins the exact three unapplied hunks
and their actual source base hashes. The lib hunk adds only the late observer
call. The callback hunk removes DeferredCaseInputs entirely and its broad clones;
the test hunk updates the diagnostic field from obsolete snapshot capacities to
actual late-hook count. It does not alter a physics expectation.

The existing normalized hook rejects combinations and unsupported profiles.
Prepared case_seen additionally checks exactly one model load case, an empty
combination list, matching actual case id and a previously unused hook. It
records prepared_one_case_seen only after counted checks and case-id storage.
The late callback rechecks those same facts, exact case id and case_calls = 1;
its independently counted prepared_late_calls must become exactly one. Thus the
passed source_selected flag describes the only case in this invocation. At the
chosen callsite source_selected was set from selected_source.is_some(), source
finalization has completed, and it is immutable through the final case return.
The later envelope has no other case or combination that can select exact-block.
A true flag refuses the prepared path before SourceParts/native preparation.
False is necessary but not sufficient: the positive early and late markers,
actual source capture success and post-envelope validation are still required.

A missing late hook leaves prepared_late_calls = 0 and the permit false; finish
refuses even if some other field is present. A duplicate hook records its entered
count and refuses before another source build. A failed old-to-old capture keeps
its error/work prefix and cannot become a PreparedCase. The final finish and
prepare_case still require the ordinary preview identity, solved status, no
source-block envelope, complete original observation metadata/value/presence,
actual source, one final hook and no old native call. No absence implies consent.
Default observer diagnostics use their old early source path and ignore the new
callback; observer=None performs neither callback body nor extra source work.
