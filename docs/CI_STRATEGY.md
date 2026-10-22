# Piping CI

`harness` is the single required result. It calls the product workflows and
fails if any selected verification fails, is cancelled or never runs. Unselected
product jobs are explicitly skipped. Selection uses the complete PR diff;
missing diff information conservatively selects coverage.

On a relevant PR, desktop coverage is a production build and one Chromium
project running the R2 smoke and save/reopen journey and the rotation-glyph
browser evidence (DEL-07-05:1). The browser matrix runs only by explicit
workflow dispatch.

The `piping-desktop` selection adds the full desktop Vitest suite
(`npm run test:desktop`) to that product job, before the build. It is selected
when the PR diff changes an input the suite reads or compiles into its wasm
engines:
- any shared CI input that selects the piping product;
- any project path that desktop source names, exactly or as a directory, by
  a project-relative string or a relative import;
- any other piping-selected input except Python and Rust code;
- Rust in the normal and build path dependencies of the two wasm engine crates
  that `apps/desktop/scripts/build-wasm-engine.mjs` builds, and files that
  their `src/` or `build.rs` read by `include*!` or `#[path]`.

Python- or Rust-only diffs outside those inputs do not select it. Docs that
desktop tests read select it, and with it the product job. An unreadable
source tree, crate or compile read selects the suite. The selection's reasons
are in the plan artifact. The browser reuse key includes whether the suite ran,
so a pass without it is never reused for a run that needs it. Dispatch runs the
suite.

Rust changes select their owning crate and transitive consumers through resolved
Cargo path dependencies, including dev, build, target and workspace dependencies.
An exact changed fixture or included source selects its actual readers and their
Cargo consumers; a fixture read does not create a supplier-crate dependency.
Literal compile reads are resolved from the source file. Dynamic runtime reads
have explicit exact-file declarations bound to source anchors, source scans and
Rust caller trees. The caller trees are bound by the `source_trees` digests in
`tools/hosted-ci-routing.json`. A change to any `.rs` file in a crate that has
runtime-read declarations therefore refreshes that crate's digest in the same
change. Without the refresh, `tests/test_ci_numerical.py` fails, and hosted
selection falls back to every crate. The required `harness` check does not catch
a missing refresh (it passed on #1511's first head, `c42f11bdf7`, which lacked
one), so run that test, or the full registered pytest, before asking for merge. Stale declarations, missing inputs, unresolved dependencies,
unknown paths and shared execution controls select all discovered crates. Existing
numerical oracles remain unchanged. Python coverage is gated independently;
Python-test-only edits retain their Python checks with zero Cargo crates. Python PR coverage keeps security and constraint/source-block validation, then
adds qualification, schema and canonical-JSON families when their inputs change.
Established shared test helper edits also select their direct importing tests;
an edited support module under `tests/`, such as a `tests/verifiers/` module,
selects every test importing it directly or through other test modules, read
from the tests' own imports so no non-test module names the verifiers;
and a declared byte-pinned fixture selects the tests that pin it;
full-fallback plans retain the broad Python cohort and directly edited tests.
Edited tests are run directly; shared CI/input changes expand the selection. Full preview, schema and qualification
suites remain available on dispatch, alongside the full Cargo suite.

Hosted builds are unregistered. The retained-memory profile is registered only
for the qualification host's exact build identity, so a hosted build reports
`Stale` and takes the ordinary route. A test that depends on registration
asserts both branches: the registered facts in the registered build, and `Stale`
with the unchanged ordinary route otherwise, as `retained_facade_tests.rs`'s
`registered()` does. When product_physics is selected, the numerical job also runs the six
retained W1 stack witnesses at the default debug profile, one process each: an
early warning, since only the qualification host's registered sweep is VER-004's
record (DEL-04-08 Design `stack-headroom-repair.md`). The hosted numerical job stops at the first failing crate,
so one failure hides the crates after it. Before pushing a change to retained
sources or registration-dependent tests, run the selected numerical suite
locally in the registered build and in an emulated unregistered build (a
build-identity-changing flag such as `RUSTFLAGS=--cap-lints=warn`, in its own
target directory).

No automatic nightly or post-merge duplicate suite is required. Reuse passing
hosted results rather than repeating the same suite locally without a change or
unresolved concern. Failure diagnostics stay in CI artifacts. The optional
macOS release sweep remains available for release preparation.

Successful numerical and core-browser results can be reused on PRs. The key
covers every tracked file available in the sparse product checkout, the runner
image/architecture, language runtime and selected numerical command set and the plan's classification, reasons and exact manifest identity. Selection is recomputed before execution or reuse. This
is intentionally conservative: source, fixture, test or workflow changes rerun;
changes confined to excluded execution records can reuse a pass. Cache restore
uses an exact key only, and records are saved only after successful execution.
A hit reports the original run and commit, explicitly saying tests were not
executed again. Manual runs bypass result reuse. Missing caches run normally;
this is separate from dependency/build-output caching.
