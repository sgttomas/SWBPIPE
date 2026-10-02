# Bounded additive discriminator proposal

The five functions retain their protected_01 names and existing fixtures.
`i23_phi_floor_only_at_verification_1024` observes StopDecision.floor directly
for verification precisions 256, 512 and 1024. Its exact 2^-438 expectation
comes from R7 item 6a and the existing Phi test, not PHI_SCALE_BITS.
`i23_r7_data_scope` and `i23_r7_g_scope` use CHARGE's fixed flags/theta/g facts.
`i23_r7_bound_choice` now compares B directly to fixed CHARGE tokens instead
of deriving its expectation from live Uc/S fields. `i23_r7_required_shift`
compares the scheduled count and S to fixed CHARGE/BOUNDS data. The two selected
RF-LARGE models each have one data block, need=1 and a first-try bound; therefore
BOUNDS's forced shift is the same scheduled shift in the accepted derivation.

For R7-M5, no existing direct verifier E/resolution assertion was established.
The existing scale_at test independently forms E through formation_scale,
which bypasses the mutant's verify_state w_abs site. The proposed sixth test,
`i23_r7_prescribed_e_matches_scale`, calls the real verifier before publication
and compares all 33 PRESCRIBED E rows (21 through 53) with SCALE's explicit
fixed tokens, then body 0's uncoupled force/moment resolution bits with
`4120facb09949713` and `40e2318280966d4e`. The SCALE record's independent
Fraction derivation and verify_em use the same abs(u) input including prescribed
DOFs, precision and rounding; the source excerpts establish this correspondence.
No expected value was obtained from runtime or the production E/bound path.

All numeric/model corpus hashes match the existing K4 SHA256SUMS. The source
warrants contain the exact selected record lines and the independent generator
formulas. The generator was read, never executed. Existing parameters, models,
oracles, assertions and tolerances remain unchanged.

The overlay appends one shared prepublication report helper and six tests to
the already mounted method_tests module. build_shared<4,8>(256,320),
solve_case_at<4,8>, build_verify_shared<4,8,8> and verify_state<4,8,8> follow
the existing helper path; q_w=448 fits the 8-limb width. The fixture's existing
Seeded RAII scope is preserved. No solve_case/publication certificate is needed
for these direct report assertions, so it cannot mask the named historical
fault. A failed helper/precondition is still not a kill.

OVERLAY_MAP.json identifies exact test filters, mapped semantic assertion
messages, the separately frozen mutant patch hashes and target pre/post hashes.
RV29 must review this proposal before any installation or runtime grant. At
that later stage each exact baseline filter must run one test and pass first;
only the designated numeric/rule assertion failure may count. Compilation,
shape/precondition/record-count failure, empty selection or generic failure
cannot supply a kill. No existing runtime subset or optional historical NOTE
was reopened here.
