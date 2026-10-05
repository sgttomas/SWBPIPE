# C0 fixed preparation helper and return-temporary schedule

This closes the formerly omitted source-level helper/return ownership schedule;
it is not an optimized stack measurement, RSS/profile qualification or all-in
work debit. The original premature C1 record remains unchanged. ROOT's prospective
exception permitted the exact filtered `i51_c0_layout_and_accounting_only` test:
it passed, with no model, PrimitiveSource, factor or native solve constructed.
The command, source digest/delta and raw output are pinned in assigned scratch.

## Measured layouts

`C0_LAYOUT_MEASUREMENTS.json` carries all 21 actual size/alignment pairs.
Frame 3888/8; preparation work 528/8; NumericWork 400/8; WideContext16 80/8;
ExactWideSum 2144/16; Endpoint 144/8; Enclosure 288/8; scalar/direct/wide
Result<Endpoint> 144/8; endpoint pair and its Result 288/8;
round-detail Result<(Endpoint,usize,bool)> 160/8; Magnitude[128] 1024/8;
trimmed[130] 1040/8; scaled[17] 136/8; Double[32] 256/8;
significand[16] 128/8; Binary64Outcome 24/8; annulus result 64/8;
AnnulusPreparationSpent 592/8. No unqualified 32-Endpoint ceiling remains.

## Explicit live-owner and call schedule

The peak is the maximum over the following fixed call stacks, with their caller
owners still live. Count argument/return slots separately for a conservative
source-storage bound; compiler elision is not assumed. Each entry names the
actual source function and its value-bearing locals, including returned values.
Scalar indices/signs/exponents/booleans and borrowed slices/references stay in
the listed function's fixed control frame; no variable-size owner is hidden there.
Use checked Layout/size/alignment arithmetic when composing a target memory
permit. This schedule does not assign a guessed numerical total to ABI spills.

| Phase / source frame | Live values and returned storage |
|---|---|
| prepare_product_annulus / SectionPreparationWork::new | one preparation Work; its construction result; two input f64s; eventual Result<PreparedAnnulus> and Spent return. NumericWork is a field of Work, not a second resident owner except its construction/move slot. |
| SectionPrepFrame::zero → caller f | local zero Enclosure, partially initialized 27-Endpoint return frame, and caller frame destination. The returned slot is counted even if optimized into its destination. Thereafter one 27-Endpoint frame remains live throughout geometry and conversions. |
| lift/pos_lift / Wide16::from_f64 / shift | input f64 or Endpoint borrow; `[u64;16]` significand construction, Endpoint result/error slots; shift returns an Endpoint copy. The persistent field assignment and temporary result both count. No heap. |
| sub_pair | borrowed operands; partial Enclosure containing the completed lower Endpoint while the upper scalar runs; scalar Result<Endpoint>; returned Enclosure/result slot. |
| positive interval mul/div/add | two **by-value Enclosure arguments**; positivity-check self/result copies; partial output Enclosure while second endpoint runs; returned Enclosure/result. These argument and return copies are outside the 27-Endpoint frame. |
| NumericWork::scalar → scalar_owned | one entered operation; one fresh WideContext16 and one ExactWideSum, moved to scalar_owned; conservative argument/return slots retained in layout accounting. scalar_owned holds its Result<Endpoint>; the caller/Enclosure output destination is separate. Both context and sum work are collected before every return, including failure. |
| directed add/sub → round_toward → step (only if wrong side) | round_toward's nearest Endpoint and step's out Endpoint; sum/context are borrowed, not duplicated. `step` additionally uses exact ONE via add_wide_scaled. Each active helper's Result<Endpoint> return slot is distinct. Calls are sequential, not recursive. |
| directed mul → ExactWideSum::add_product → WideContext::two_product → two_product_rounded | `s,e` pair retained while adding each term; product Double[32], residual Double[32], and conditional low Double[32]; rounded s/e endpoints and pair-result slots. `round_detail` adds significand[16] plus its 160-byte result. The lower/upper calls are sequential. |
| directed div → WideContext::div → div_rounded | quotient Double[32], remainder significand[16], round_detail significand/result and quotient Endpoint q. The exact q*b-a check subsequently uses the same add_product chain above; q remains live. If wrong-sided, step runs with q live. No unbounded retry. |
| ExactWideSum::add_wide / add_wide_scaled → add_raw | `parts()` copies `[u64;16]` (it is **not a borrow**). add_wide_scaled additionally owns scaled[17]. add_raw owns trimmed[130]; shift_up mutates the existing sum arrays and allocates no second sum. These temporaries overlap their calling pair/nearest/q values. |
| ExactWideSum::round → net → WideContext::from_integer → round_detail | net Magnitude[128] return, caller magnitude slot, then round_detail significand[16] and result. The sum's positive/negative magnitudes remain live; no second ExactWideSum is created here. |
| pi / shift_interval / positive / min/max | fixed pi constant arrays are static; make/from_parts argument `[u64;16]` and Endpoint result slots are counted, then the returned Enclosure. shift_interval and comparison helpers take/copy their declared endpoints/Enclosures. No runtime pi series, heap or widening retry. |
| nine binary64 conversions | Endpoint borrowed; Binary64Outcome return. Wide::to_binary64 reads the significand slice and holds fixed sign/quantum/index/kept/round/rest scalars; it creates no significand array. Nine calls are explicit: eight geometric interval endpoints plus c. Unequal/invalid outcomes stop with original error and work. |
| success/error exit | field/bit result, original numeric cause wrapper and work move into AnnulusPreparationSpent. Error causes no longer allocate a formatted String. Lower/earlier successful fields and all spent context/sum work survive. |

The helper graph is finite and nonrecursive. Width is fixed at 16 limbs; exact
sum capacity is 128 limbs and trimmed term capacity130. Source span/exponent
limits remain unchanged. The possible two-product residual-copy branch and the
net magnitude return/caller overlap are deliberately included. Fixed control
locals and ABI spills are not advertised as a measured exact stack peak.

## Entered-copy/accounting boundary

The live command records the 19 actual directed entries, nine conversion entries,
27 initialization slots and 26 persistent Endpoint assignment entries. Existing
AttemptWork/SumWork own arithmetic and their instrumented limb work. By-value
Enclosure arguments, `parts()` significand copies, result/move storage and helper
control work above are named auxiliary operations; their source-level presence
and storage are not priced as zero or falsely merged into an all-in LME total.
The layout-only overflow test proves that conversion-count overflow is sticky and
a second attempted conversion performs no new conversion entry. New source-side
buffer copies require independent checked capacities/prefixes; these cannot be
inferred from the arithmetic frame or LibraryBoundary counter.

## Snapshot dependency — not hidden by this schedule

The old broad DeferredCaseInputs graph is **not** qualified by these measurements.
Its elimination is the exact frozen late-hook proposal in C0_SEAM_PROPOSAL.json,
awaiting ROOT scope review. Actual scalar/string/vector source capture remains
through the previously explicit adapter path; prepared source support-child
copies are to use explicit checked reserve/copy loops rather than `.clone()`.
No further solver/product run is permitted until ROOT checks the complete C0
correction and the authorized source changes are applied and checked.
