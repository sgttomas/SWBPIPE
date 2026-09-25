# RET-01 repair 01 backcheck

**Disposition: RET-01 remains open for the late-rejection accounting path. No focused-test clearance yet.** The first repair correctly sizes the eventual metadata comparisons, but it does not retain the charge for the admitted sizing traversal when the full reservation later rejects. A bounded two-stage debit repair is required. No arithmetic, source-ownership, projection-criterion or exact-sign defect is identified.

Candidate adapter fd78de18caf9f93b83495d1c7be3a5ca7340a03a16e6e2f5145aae4724c381f8 exactly matches the supplied frozen bytes and complete delta from 2328cabd. Kernel 6801610d is unchanged. [Source verification](_run_records/SOURCE_CHECK.json), snapshot, inputs and [origins](_run_records/ORIGINS.json) preserve the check. Same independent TASK reviewer and instruction/model basis; no source edit, Cargo/build/Node/native/heavy execution or descendants.

## Partial correction confirmed

summary_comparison_charge now includes both sides of decision IDs, work labels/errors, source strings and relevant containers. It uses checked multiplication/addition and a defined saturated rejected-size marker on overflow. It prevents summaries_match from comparing string contents unless the full reservation fits. On success or a later summary/nested-replay failure, the admitted comparison charge and reduced child limit are carried forward correctly.

The long-ID and public nested-error controls are relevant. No numeric oracle, accuracy limit, exact source binding or method eligibility changed.

## Remaining P2: admitted sizing walk is reported as zero work

Locations: structural_adapter.rs:615–652 and 668–675.

The O(1) container prefix is checked against the limit at 615–633, after which the function walks both work-attempt arrays and both decision arrays at 642–652 to inspect each string length. The prefix is never accepted/debited before those loops. A later byte reservation failure returns through 668–675 with charged=0 and the full size in rejected.

A deterministic source witness already exists in the new test: compute the full successful comparison charge C, then replay with C−1. For the adjacent short/long-ID reports, the container prefix and earlier byte additions fit. The sizing walk reaches the final dynamic entries and then rejects. The current test expects charged=0 despite that admitted traversal.

This is not a claim that charged must equal hardware instructions or elapsed work. The shared WorkReport contract explicitly says its conservative charges include bounded scalar checks and loops, and that rejected denotes an attempted reservation that was not executed. The repair return documents an atomic **comparison** reservation, but that narrower convention does not account for its separately admitted O(n) sizing pass. Reporting zero is truthful only about acceptance of the final comparison reservation; it is insufficient as complete per-call reservation accounting.

## Focused repair 02

1. Compute the fixed/container sizing prefix from O(1) length fields with checked arithmetic.
2. If it cannot fit, reject before traversing entries, with charged=0.
3. Once admitted, debit that prefix before walking labels/IDs. It covers the sizing pass and fixed/container comparison work.
4. Accumulate/admit the remaining dynamic-byte reservation under the remaining budget. On a late size/overflow rejection, retain the accepted prefix charge and report only the rejected remainder (using the declared saturation marker when necessary).
5. On full admission, carry prefix plus byte charge into summaries_match and give nested exact replay only the remaining budget. Retain accepted charges on all subsequent failures.

Update the new accounting controls: the historical 1088 limit can still reject before the larger container prefix with zero charge. The C−1 and sufficiently late nested-error rejection cases must retain a positive prefix charge. Keep sufficient-budget success and one-below-total nested failure controls. These are corrections to newly authored resource assertions, not changes to mechanical references or protected criteria.

Preserve repair 01 and this failed backcheck. After the two-stage repair is sealed, the same bounded backcheck can clear focused helper/adapter tests. Full NGR02, wire/persistence, public method and Current qualification remain outside this source backcheck.

