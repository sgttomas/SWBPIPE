# RV19-D4u current diagnostic proposal

The original RV19 script changes O9 membership from the candidate to its
verifier, then removes the two overflow verdict assertions in
`tests/retained_k4/method_tests.rs`. Those original edits remain recoverable
in INVENTORY.json with the original script hash. They are not authorized test
changes and neither deletion appears in the emitted patch.

`patches/RV19__RV19-D4u.patch` is an explicitly reconstructed underflow-only
source fault. It uses verifier membership only when candidate or verifier
has `Binary64Outcome::Underflow`; overflow and normal pairs keep candidate
membership. Both old overflow verdict assertions therefore remain meaningful
and unchanged. The complete ordinary RV19-D4 patch is retained separately.

The existing exact filter is
`structural::retained::adaptive::method_tests::sd_g5_a_row_the_candidate_cannot_publish_sets_no_s_star`.
Its original overflow assertions at lines 1643–1649 must pass. The intended
failure is the first underflow assertion at lines 1664–1667: the tiny candidate
must cause `Some(Rejection::StopRule { index: 1 })`, whereas the underflow-scoped
fault returns None. The opposite straddle at 1670–1673 remains present.
An early overflow failure, compilation failure or unrelated assertion is not
underflow evidence. This route was proposed by RV29 through ROOT and rederived
from the source and original test; mapping acceptance and execution are pending.
