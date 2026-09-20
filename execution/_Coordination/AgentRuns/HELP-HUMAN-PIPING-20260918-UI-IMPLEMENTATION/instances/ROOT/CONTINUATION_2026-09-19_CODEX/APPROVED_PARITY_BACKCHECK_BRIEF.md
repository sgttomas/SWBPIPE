ROOT to existing independent TASK b3_code_review (Astra/xhigh). Read-only;
no writes, Git mutation, browser/native/hosted execution or delegation. Small
CI-policy unit/collection probes permitted. Owner now explicitly approves
parity with existing source test and bounded reruns; exact active-chat text and
scope are in OWNER_DIST_PARITY_APPROVAL.md. Review actual applied frozen
7a37a8f2b2d4f55460ebeaa7d201bae4d840cf9b against efaef7e7fd50af3763c0e8a53bc23ee838ad6f32.

Applied dist spec must equal the previously reviewed proposed hash
f6def858ca800f0f65fa338a92847e6ad8e0b11052dc6063f56d31d809cca4f3.
All source/product bytes, other limits/oracles/geometry/endpoints must remain.
CI exception now permits the named dist file only when its committed blob has
that exact SHA256 (not arbitrary edits to the file), plus raw failure binary
evidence under the exact ROOT B3_DIST_ROUNDOFF directory. Review helper/hash/fallback
behavior and the two added regression tests; current30unit passes retained in
_run_records/parity-rerun/ci-policy-tests.log. Docs name the one-time owner ruling
and distinguish combined delta verification from a fresh full sweep.

Full corrected dist53 and remaining production build are running in wt2 on clean
7a37 with pinned Chromium; do not claim them passed yet. Original065 failedsweep
remains failed; its unaffected Rust/Python-domain/unit/full-source successes are
reused only with exact final input comparison. Changed CI verification policy is
separately checked; no assertion or general gate beyond this ruling changes.
Return actionable findings or PASS with exact applied/hash/scope/authority and
verification limits. Final actual candidate collection/CI/merge still pending.
