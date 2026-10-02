# RV29 VR accounting-record backcheck

**Refresh verified within ROOT's bounded ruling; affected-suite rerun outstanding.**
Reviewed41b01f712a6d113ca26086a51c829469672a6f88 against
b13a42ea3b56a492461e0c329ca75790b38818ba. The allowed disposition is NUM
2b3fc261cf088702278fb6fd2afbed46fae9823d, ROOT_RULINGS_V1.md's A1 VR records
addition. No Rust, generator, mutation or test rerun was performed by RV29.

## Independent field and charge comparison

Recursively compared every key, array length/order, value and value type in all
ten family JSONs plus invariance.json and parity.json. Exactly955 scalar fields
change:191 each at attempts.exact_sum_work, attempts.own_work, attempts.stop_rule_work,
attempts.stages.stop_rule and invocation_charged. All other fields are identical,
including outcomes, precisions, geometry, report counts, range/class/control and
not-covered evidence, source hashes, failed-attempt reasons and storage fields.
Invariance and parity files are byte-identical. The computed field counts and
old/new hashes independently agree with ROOT's FIELD_DIFF.json.

For all201 family-case records and every attempt, checked:

- new context work + exact-sum work equals the sum of its19 stage fields;
- stop_rule_work equals stages.stop_rule;
- nonnegative context delta + exact-sum delta equals both stop-rule deltas;
- sum of attempt deltas equals the invocation delta;
- full invocation charge equals own/context+sum work plus shared and verification
 shared work only when their respective built-here flags require charging.

No closure mismatch or changed nonaccounting field was found. Every ten-file
postimage exactly matches the preserved isolated generated output. The refreshed
SHA256SUMS validates all12 kernel_lane JSONs. The actual commit write set is
limited to these authorized observations/manifest and its review evidence packet.

## Unchanged criteria and generator binding

Compared178 unrefreshed tracked files in the preserved FK/sparse_direct/VR
archive directly to40129a225d73860ac2a53da9a2fa73869df668f3, the pre-refresh
revision and the refreshed revision. Every byte matches. This includes numerical
source, generator/example, tests, cases, fixed inputs/pins, Cargo manifests/locks
and unaffected observations. Only the explicitly regenerated observation files
and their manifest were excluded from that archive source equality check.
The preview build log points to those archive crates and runs the unchanged
release vk_records --write. Generated output equality is checked, not merely
inferred from a printed source label. No generator or criterion was edited.

Verified nine original-log SHA256s and their exact portable transformations,
plus the eleven-entry refresh evidence seal. VR's ten original family failures
are at tests/lane.rs:108, the final byte-record comparison. Source inspection
confirms it follows the unchanged row accounting, numerical failure/class,
not-covered, tally, outcome and control-discrimination assertions. Thus those
prior checks executed successfully for the failed family tests; a byte-record
refresh does not delete or weaken them. Their generator outputs are now the
committed observations, but the required affected-suite rerun on that committed
refresh is still unrun by this reviewer and not claimed passed.

## Conclusion and limits

This is the authorized inclusion of new certificate charges in derived accounting
observations. It does not rebaseline a numerical answer, tolerance, selected
precision/outcome or protected availability criterion. The independent absolute
certificate ledger and required mutants remain necessary; this field comparison
proves recording consistency and unchanged criteria, not every primitive's price.
No new finding blocks this records-only refresh. Full A1 closure still requires
its pending mutants, consumer result and exact-candidate gates. K6c/W1/F2a retain
their separately owned qualification boundaries.

CHECKS.json records all case closure results, file hashes, changed-path counts
and archive checks. BASIS.json preserves immutable origins/hashes and inherited
role/skill provenance. Only this owned additive packet was written.
