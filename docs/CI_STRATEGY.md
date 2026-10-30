# Piping CI

`harness` is the single required result. It calls the product workflows and
fails if any selected verification fails, is cancelled or never runs. Unselected
product jobs are explicitly skipped. Selection uses the complete PR diff;
missing diff information conservatively selects coverage.

`piping-windows-core.yml` supplies supplemental RT-V7 feasibility evidence on
`windows-2025` with Rust 1.97.1 and `x86_64-pc-windows-msvc`. It runs the existing
frame-kernel, curved-bend (including frozen formation bits), sparse-direct and
canonical-JSON tests, then checks the product-physics library. Its PR path filter
covers core sources, fixtures, frozen references, schemas and ancestor
Cargo/toolchain configuration; manual dispatch
is also available. It is separate from `harness` while Windows remains outside
the Must v1.0 release conditions. It checks native x64 OS/toolchain identity and
uses a runner-local target directory. This establishes only the named core
tests and compilation: desktop memory controls, persistence, WebView2, native
journeys, packaging and retained-memory qualification need their own Windows
evidence. Parallels Windows on Arm and emulated x64 runs are labelled separately.
The proposed native-service boundaries and remaining work are in DEL-10-04
`Design/windows-port.md`.

On a relevant PR, desktop coverage is a production build and one Chromium
project (`chromium-desktop`) running core specs: the R2 smoke and save/reopen
journey, the rotation-glyph browser evidence (DEL-07-05:1) and the result
overlays. The selection adds a group of specs when the PR diff changes one of
the group's paths. Open, New and Replace of an edited project add
`ui-foundation`, `c3-viewport-visibility`, `gui-workflow-validation` and
`linear-authoring`. Their paths are `apps/desktop/src/features/project-storage/`
(the replace and unsaved-changes prompts and guard), `workspaceSession.ts` (the
Open, New and Replace commands), `services/projectService.ts` (the project
store they read and write), and the four specs themselves. A shared CI input, an
unmatched project input or full coverage adds every group. The core specs,
groups and paths are declared in `piping_browser_journeys` in
`tools/hosted-ci-routing.json`. The product job runs the plan's spec list, and
the plan's reasons name the path and group behind each added spec. The browser
matrix runs only by explicit workflow dispatch.

The `piping-desktop` selection adds the full desktop Vitest suite, run in two
shards (`vitest --shard=1/2` and `2/2`) as jobs beside the product job, under
the same aggregate. It is selected
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
so a pass without it is never reused for a run that needs it. The product job
and both shards each compute and publish that key. The record is saved only
after all of them succeed, and only when every one published the same key; if a
runner image differed between them, nothing is saved. Dispatch runs the
suite.

The `piping-desktop-rust` selection runs the desktop shell crate's Cargo tests
(`apps/desktop/src-tauri`: store, command and persistence tests) in their own
job under the same aggregate: `cargo test --locked --manifest-path
apps/desktop/src-tauri/Cargo.toml` from the project, with the default test
threads. The job runs on `macos-26`, the product platform (macOS Apple Silicon),
which needs no webview packages and runs the macOS memory and worker paths the
tests cover; it builds no frontend. It restores the main-scoped
`piping-desktop-rust` dependency cache that `shared-build-cache.yml` warms with
`cargo test --no-run`. It is selected when the PR diff changes:
- any shared CI input that selects the piping product, or an unmatched input;
- any file under `apps/desktop/src-tauri/`;
- a Cargo control the job's `cargo test` reads: `.cargo/` configuration or a
  `rust-toolchain` file in the project or a directory above the crate, or a
  project-level `Cargo.toml` or `Cargo.lock` (Cargo reads configuration from its
  working directory and its ancestors; repository-root files are shared inputs);
- Rust or a `Cargo.toml` or `Cargo.lock` in a crate it compiles: its normal,
  build and dev path dependencies, and their normal and build path
  dependencies, transitively;
- any other `Cargo.toml` or `Cargo.lock` under the project, matched in any
  letter case: the shell's W-18b reach scan finds manifests by name
  case-insensitively and reads every project package's dependencies through
  `cargo metadata`, so a new or changed manifest can change its result;
- a file that the crate or those dependencies read by a literal `include*!` or
  `#[path]` in `src/` or `build.rs`, or that a declared runtime read in their
  `src/` or `build.rs` names;
- a project file or directory that the crate's Rust names by a path-like
  string literal (a format template names the directory before its first
  slot), which is how its tests read fixtures at run time.

Other Python, Rust, desktop TypeScript and docs changes do not select it. An
unreadable crate, compile read or stale runtime-read declaration selects it.
Dispatch runs it. The selection's reasons are in the plan artifact.
The literal scan finds fixture paths written as one path-like string or format
template. A test that assembled a fixture path from fragments (`join` of
separate parts, `concat!`) or took it from the environment would not be found,
so such a reader should name its path as one literal.

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
one), so run that test, or the full registered pytest, before asking for merge.
New or removed Rust files and Cargo manifests also refresh the declared runtime
`scan.files` inventories, including project-wide scans that cover fixture-only
crates with no runtime-read declaration of their own. Validate the regenerated
inventories with `numerical_ci.source_read_dependencies` and run both
`tests/test_ci_numerical.py` and local `tools/software_workflow/test_hosted_ci.py`
before integrating such changes. Stale declarations, missing inputs, unresolved dependencies,
unknown paths and shared execution controls select all discovered crates. Existing
numerical oracles remain unchanged. Python coverage is gated independently.

The standalone required repository job runs the bounded tool/routing suite and
`tests/test_ci_numerical.py` on every PR, including Rust and Cargo manifest
additions/removals. This closes the former monorepo's routing-self-test selection
gap. The local `tools/tools-test-routing.json` records this bounded estate; it
does not import unrelated Root tool suites. Numerical inventory/source-digest
refresh and product-selection failure propagation remain required.

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

GEN-13, the claims-language check
(`tools/validation/validate_claims_language.py`, SWBPIPE surfaces only), runs
in hosted CI as a step of Repository checks (`governance-harness.yml`). The
step is selected by the `claims_language` output of
`tools/run_affected_tests.py --dry-run`, which is true when a changed path
matches the validator's own `input_patterns()` (`--list-inputs` prints them):
`apps/desktop/src/`, `docs/`, every `execution/PKG-*/DEL-*/ScopeOfWork.md`, the
anchor surfaces `docs/PRD.md` and
`core/reporting/report_renderer/src/lib.rs`, `docs/claims_registry.md`, and the
validator itself. A dispatch run (`--all`) or an unresolvable base selects it
too; the selection step treats a schedule event as it does dispatch, though the
workflow has no schedule trigger. If the selector cannot load the validator, it selects
the step as well, so a broken validator fails closed rather than going unchecked. Other
changes skip it. The patterns are lexical paths; the repository has no tracked symlinks
into these surfaces, and adding one would need its target covered too. A finding fails Repository checks, and so
the `harness` result.

No automatic nightly or post-merge duplicate suite is required. Reuse passing
hosted results rather than repeating the same suite locally without a change or
unresolved concern. Failure diagnostics stay in CI artifacts. The optional
macOS release sweep remains available for release preparation.

Successful numerical and core-browser results can be reused on PRs. The key
covers every tracked file available in the sparse product checkout, the runner
image/architecture, language runtime and selected numerical command set and the plan's classification, reasons and exact manifest identity. The browser key also covers the selected Chromium project and spec list, so a pass without a group's specs is never reused for a run that selects them. Selection is recomputed before execution or reuse. This
is intentionally conservative: source, fixture, test or workflow changes rerun;
changes confined to excluded execution records can reuse a pass. Cache restore
uses an exact key only, and records are saved only after successful execution.
A hit reports the original run and commit, explicitly saying tests were not
executed again. Manual runs bypass result reuse. Missing caches run normally;
this is separate from dependency/build-output caching.
