# Independent NGR02 in-memory retention review

**Disposition: one actionable P2 accounting finding, RET-01, remains. Perform the bounded comparison-reservation repair and independent backcheck before clearing this candidate for the focused tests.** No arithmetic, source-ownership, lifetime or exact-sign defect was found in the remaining complete two-file diff. RET-01 is not a refutation of the retained numerical result.

Reviewed frozen files:

- frame_kernel/src/structural/exact_boundary.rs: 6801610d0bb515eab33603163423812b81bf94946d8c23b99c127b655d996a7f.
- nonlinear_integration/src/structural_adapter.rs: 2328cabde725a61334bbba0b9d37469388e402ea6dd7acdd9ad6347e5ca895a6.

Both actual files match frozen copies. The complete per-file diffs reproduce their changes from helper700338b9 and adapter79272e2f. [Source verification](_run_records/SOURCE_CHECK.json), [snapshots](_run_records/reviewed_sources) and [scope check](_run_records/scope_check.json) are preserved.

Same independent TASK Type2 /root/solver_manager/kernel_review, actual parent /root/solver_manager, delegated-harness-native, unchanged gpt-6-astra xhigh dispatch. Full TASK/software-code-review were reread; the same full Root/Piping/LOOP instruction basis remains recorded. The selected primary Revision 02 ROOT_SELECTION, EXACT_BOUNDARY and WIRE_TYPES were read and their selected hashes verified; applicable CONTRACT excerpts were consulted. [Origins](_run_records/ORIGINS.json) and [inputs](_run_records/INPUT_HASHES.json) record actual paths and hashes. No descendants, source/Git edits, Cargo/build/Node/native/browser or heavy tests. Only RETENTION_REVIEW evidence was written.

## RET-01 — P2: adapter replay does not charge the dynamic data it compares

Location: structural_adapter.rs:605–619, with the comparisons in summaries_match at lines 521–535.

replay_retained_against reserves only:

32 + 8*ordinary_displacements.len + 24*decisions.len + 24*projections.len.

It then calls summaries_match, which compares the complete ExactGapWork/work_snapshot (including nested attempt labels/error strings), decisions/decision_sources (including support-ID strings), source identity and other public/private summaries. The reservation does not grow with these dynamic strings or the work-attempt payload. In contrast, state_retention_charge explicitly accounts for their bytes when copying them.

A legal long support ID demonstrates the gap without changing any mechanics. In the existing 18-DOF/two-gap fixture, the prefix is always 1,088 units. A 4,096-byte ID contributes at least 4,109 compared work-label bytes and 4,096 compared decision-ID bytes, yet replay can enter the comparisons with a 1,088-unit budget. If that budget is exhausted afterward by the child replay, the already performed dynamic comparison remains unaccounted. This is a reservation/accounting defect, not an empirical performance claim.

[The retained reproduction recipe](_run_records/RET01_WITNESS.md) specifies the source-valid long-ID mutation, fixed-charge calculation, current predicted failure ledger and required tight-budget negative control. No Rust reproduction was executed here.

**Focused repair scope:** in the adapter, compute and admit a checked conservative comparison charge before summaries_match that covers all dynamically compared data, including nested work labels/errors and decision/source strings. Keep accepted and rejected reservations separate and pass only the remaining budget to nested replay. Add the long-ID/tight-budget control alongside a successful sufficient-budget replay. No change to exact arithmetic, source identity, contact decisions, projection 1e-9 criterion, wire admission or Current policy is needed.

## Reviewed implementation properties

- RetainedResponse privately owns its complete numerical Snapshot, admitted limits, actual constructor block witnesses, all displacement/reaction Ratio terms, contact operands/signs and projection records. The owned types contain their data rather than Context/Response borrows. Their constructors are private; public accessors expose read-only data.
- Actual a/b/c and positive determinant terms are captured during Context construction, before the witnesses are discarded. Singleton and two-dimensional SPD warrants follow the selected exact method. The original free partition, nonzero prescribed values and full K_fc contributions survive.
- Original aggregate K/f, ordered represented stiffness and identified force contributions, force-basis distinction, symmetry metadata and original IEEE source bits are copied. Expansion order/tails are preserved. Empty numerator is canonical mathematical zero, without a fabricated IEEE zero-propagation claim. No absent primitive engineering contributor identities are invented.
- Ratio comparison uses ordered term bits. Kernel replay bounds dimensions/term counts, rebuilds Context from the retained original source, checks independently expected source identity/bytes/force basis, regenerates minors and all ratios, and compares them before accepting contact/projection facts. It does not obtain a proof from the last term of an unvalidated imported array.
- Denominator positivity and expansion ordering come from regenerated checked arithmetic; empty/negative/changed denominator or term arrays cannot substitute. Contact DOF/sense/gap/prescribed references and normalized penetration/reaction operands are retained and recomputed.
- retain_with_work checks live same-response ownership, unique contact DOFs and complete unique displacement/reaction projection coverage. It reserves source/witness/ratio/leaf copies before cloning. Exact source/identity and projection criterion/basis/value mutations are covered by authored negative controls.
- Kernel replay shares one Work across source reconstruction, solve, contact evaluation and projection recomputation. The adapter debits its comparison prefix and forwards only the remaining limit; the defect is the incomplete size estimate, not a budget reset. Failed charged reservations stay charged and rejected unexecuted reservations remain separate.
- The adapter privately retains the actual ordinary LinearizedSolve, including its structural/equilibrium/sparse reports, separately from selected projected u/reactions and their equilibrium report. Public summaries are compared to those private facts. The ordinary report is not relabelled as an exact/projected response report.
- Existing exact arithmetic and projection formulas are unchanged. Constructor/replay charging was refactored; earlier 17 helper passes do not establish that the new 20 helper tests or new adapter test pass under the changed work accounting.

## Independent reference evidence

[Independent Fraction checks](_run_records/check_reference.py) and [output](_run_records/REFERENCE_CHECKS.json) reconfirm the retained adjacent witness:

- Exact u2=1/4−2^-56 publishes as 1/4.
- The exact reaction is −25/2^54.
- Exact action on the public displacement instead gives −25/2^53.
- The original free residual is exactly zero; K_fc and prescribed g1 are necessary.
- Singleton a=100 and the two-coordinate a=200, b=−100, c=100, determinant10000 are positive.
- An identified force tail 2^-60 remains mathematically distinct even when the aggregate rounds to 1.
- A different internally consistent source must fail comparison against the original independent source.
- A separate small ledger model confirms the required parent-prefix/child-remaining cap invariant and failed-reservation distinction.

These are independent rational/control-flow evidence, not executed Rust retention, replay or adapter solves. The new helper tests and adapter test have only been inspected in source.

## Deliberate scope boundaries

The kernel replay API accepts independently expected source, contacts, criterion and limits. The adapter convenience replay uses its private original contact/limit basis while accepting an independent numerical StructuralSystem/identity/force basis. Its success proves the retained original response and summary consistency; it does not alone bind a different current request's released-gap values or policy. Future invocation/currentness validation must use the complete independently expected contact/source contract, or the kernel's full expected-argument API. Replaying against one's own source is consistency, not independent current-source correspondence.

This delta retains successful completed responses with full projection coverage. Failure/unsupported attempt artifacts and reached-stage availability remain successor work; no preconstruction source/response is invented. That limit is distinct from the former loss of a successful temporary Context.

Wire schemas/import, immutable artifact persistence, source-owned invocation finalization, complete field-specific exact-versus-projected recovery, and the initial nonpassing method receipt remain outside this delta. Revision 02 does not promote this in-memory record to Current. The original NGR02 issue is therefore only partly addressed in source; full NGR02/public-method qualification remains open after this repair and runtime checks.

