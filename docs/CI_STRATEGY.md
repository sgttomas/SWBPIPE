# Piping CI

`harness` is the single required result. It calls the product workflows and
fails if any selected verification fails, is cancelled or never runs. Unselected
product jobs are explicitly skipped. Selection uses the complete PR diff;
missing diff information conservatively selects coverage.

On a relevant PR, desktop coverage is a production build and one Chromium
project running the R2 smoke and save/reopen journey. The full desktop Vitest
suite and browser matrix run only by explicit workflow dispatch.

Rust changes select their crate and transitive consumers through Cargo path
dependencies and cross-crate Rust source/fixture path literals. Their existing numerical
oracles remain unchanged. Shared or unknown numerical inputs select all crates;
Python-test-only edits do not run Cargo. Python PR coverage keeps security and constraint/source-block validation, then
adds qualification, schema and canonical-JSON families when their inputs change.
Edited tests are run directly; shared CI/input changes expand the selection. Full preview, schema and qualification
suites remain available on dispatch, alongside the full Cargo suite.

No automatic nightly or post-merge duplicate suite is required. Reuse passing
hosted results rather than repeating the same suite locally without a change or
unresolved concern. Failure diagnostics stay in CI artifacts. The optional
macOS release sweep remains available for release preparation.
