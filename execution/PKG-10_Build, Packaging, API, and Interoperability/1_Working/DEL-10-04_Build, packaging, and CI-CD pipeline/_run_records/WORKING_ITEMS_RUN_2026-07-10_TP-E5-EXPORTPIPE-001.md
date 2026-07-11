# WORKING_ITEMS Run Record - TP-E5-EXPORTPIPE-001

Date: 2026-07-10
Agent: WORKING_ITEMS (bounded implementation tranche)
Plan item: Phase E / E5 — the sanitized-export pipeline whose build `DEC-059`
names "a new, explicitly scheduled Phase E work item"
(`execution/_Decomposition/SOFTWARE_DECOMP.md` §12; requirements spec:
D-05b packet §4.1 guarantees G1-G7,
`execution/_Coordination/_DECISIONS/D-05b_public_export_ci_activation.md`).
Deliverable: DEL-10-04 Build, packaging, and CI-CD pipeline.
Deliverable basis: the completion plan maps E5 ("CI + release implementation
... public sanitized-export CI activation") to DEL-10-04
(`plans/PLAN_2026-06-17_prd_completion.md:203`), and DEL-10-04 already holds
the DEC-025 sweep/release-tooling run records (TP-SWEEP-001,
TP-C4-SWEEPHARDEN-001). DEL-09-05 (release quality gates) was considered and
rejected: this tranche builds packaging/export tooling, not gate checklist
content. Note: the concurrent TP-E7-SCANEXT-001 tranche (merged PR #152)
recorded under DEL-08-05; no MEMORY.md collision expected on DEL-10-04, but
merge care applies if another E5 tranche lands concurrently.

## Scope

Built the DEC-059 sanitized public-export pipeline (build + self-test surface
only) that produces a candidate public OpenPipeStress tree from the private
monorepo project:

- `tools/release/export_public_openpipestress.py` — stdlib-only Python
  entrypoint (zero new dependencies, DEC-023 posture). Plan-only by default
  (no writes); `--execute --target <dir>` stages into an explicit
  caller-named directory outside the source root; `--verify <dir>` re-hashes
  an existing export against its manifest.
- `tests/test_export_public_openpipestress.py` — 20 self-tests (profile
  shape, selection, sanitize, boundary check, lint-gate classification, and
  cargo-backed integration runs against synthetic invented trees staged into
  pytest temp directories).

### G1-G7 implementation (re-derived from the D-05b packet §4.1)

- **G1 (boundary content excluded):** inclusion allowlist (ROOT_FILES /
  ROOT_DIRS); `execution/`, `plans/`, `loop/`, `init/`, `_harness/`,
  `AGENTS.md`, `validation/evidence/`, `tools/coordination/`, `_run_records`
  dirs, and generated/build surfaces stay out by construction; everything
  not allowlisted stays out (fail toward exclusion).
- **G2 (recorded D-20 scan before any push):** supported, not claimed — the
  pipeline emits the staged tree + manifest + record + report the
  DEC-058 owner-signed scan (AC-4) consumes; no scan/clearance happens here.
- **G3 (deterministic, manifested, commit-bound):** sorted walk;
  byte-identical re-runs (self-tested); `export-manifest.csv` with
  `path,size_bytes,sha256` per staged file; `export-record.json` binds the
  export to the canonical monorepo commit + dirty state.
- **G4 (mechanical in-pipeline checks):** a `boundary_findings` pass
  (forbidden path segments/names/suffixes, secret-material filename
  patterns, private-key block content, private-absolute-path residue after
  the sanitize pass, non-allowlisted binaries, quarantine paths) plus the
  DEL-08-05 protected-content lint over every staged lintable text member,
  via the engine's `protected_content_lint_cli` binary — the same DEC-058
  check (b) surface `run_release_candidate_scan.py` consumes; the engine is
  consumed as-is, no linter edits. Reviewed expected findings are exact
  (path, code)/(path, kind) ledgers with justifications, still recorded in
  the report for the owner scan; any unexpected finding fails the run. An
  unavailable engine fails the gate (fail closed).
- **G5 (no secrets, evidence-only):** no network, signing, or publication
  credential surfaces anywhere in the tool; secret-material patterns are
  boundary findings.
- **G6 (§7 phase comparability):** the staged tree carries the
  provider-neutral `docs/BUILD_AND_RELEASE.md` §7 command basis
  (tools/release scripts, tests, sweep entrypoint) unmodified and ships no
  CI workflow (`.github` excluded — CI activation stays a separate
  owner-gated act under DEC-059).
- **G7 (drift managed):** commit binding in the record plus `--verify`
  re-hash drift detection (added/removed/changed), self-tested via
  tamper-then-verify.

## Evidence

Files added (this deliverable's surface):

- `tools/release/export_public_openpipestress.py`
- `tests/test_export_public_openpipestress.py`

Real-tree dry run (`--execute` into a session scratchpad temp directory;
nothing exported or published): 892 files staged (14.8 MB); exclusions
excluded_subtree=263 (validation/evidence sweep artifacts,
tools/coordination, generated app surfaces), skip_dir_name=274 (target/,
node_modules/, _run_records, caches); top-level entries kept out by
construction: AGENTS.md, _harness, execution, init, loop, plans; sanitize
pass 0 files (the one private-absolute-path carrier, a stray
`core/units/_run_records` task record, is now excluded); boundary check
pass (1 reviewed expected finding — the local-first storage policy test's
forbidden-prefix fixture data — 0 unexpected); lint gate pass over 513
staged text members (5 reviewed expected blocking findings: the invented
DEL-08-05 lint fixtures and the docs/TYPES.md prohibition prose; 0
unexpected blocking; 1 UnknownProvenanceReviewRequired standards-table
signature warning on `validation/hand_calcs/mechanics/
expansion_loop_curved_bend_thermal.md`, routed to human review); overall
gate result: pass. Determinism check: two consecutive runs produced
byte-identical manifests and staging trees; `--verify` clean, and drift
detected after deliberate tamper.

## Validation

- `python3 -m pytest tests/test_export_public_openpipestress.py`:
  **20/20 pass** (includes cargo-backed integration tests against the real
  `protected_content_lint_cli` engine).
- Repo-root `python3 tools/practitioner_harness/harness.py self-check`:
  exit 0.
- Repo-root `python3 -m pytest tools/practitioner_harness`: 263 passed,
  1 skipped.
- DEC-025 five-surface sweep at committed clean HEAD
  (`python3 tools/release/run_evidence_sweep.py --execute`): recorded in the
  follow-up sweep commit on this branch (artifact name in the commit and
  PLAN_COMPLETION_LOG proposal).

## Boundaries

Pipeline build and self-tests only (DEC-059 hard fences, verbatim basis:
"No CI is activated, no export pipeline is run [for real], no public
repository is created"): no public repository was created, nothing was
pushed or published, no CI workflow file was added anywhere (GitHub Actions
on the private monorepo remains prohibited absent a recorded
`docs/BUILD_AND_RELEASE.md` §7 authorization, DEC-025), and the linter/scan
surfaces (`core/reporting/protected_content_linter`,
`tools/release/run_release_candidate_scan.py`) were consumed as-is with no
edits. The export act itself remains owner-gated behind the three DEC-059
prerequisites (pipeline meeting G1-G7; a D-20/DEC-058-ruled owner-signed
green scan of the staged export before any public push; public-repo
location/naming per the D-06 ruling, DEC-057). A green pipeline run is
development evidence only — not a release-readiness claim, clearance,
release claim, professional approval, certification, sealing,
authentication, or code-compliance determination. DEL-10-04 lifecycle state
is unchanged.

## Residual Handoff

- The exported tree is not claimed to pass the five-surface sweep by itself:
  `check_release_readiness.py --profile skeleton` requires
  `execution/_DAG/_LATEST.md`, which is deliberately excluded, and some
  pytest surfaces read excluded coordination files. Making the export
  self-verifying (or trimming the exported test set) is CI-activation
  prerequisite work under the D-05b/D-06 rulings, not this tranche.
- The include/exclude profile is v1 design (D-05b packet §4 named it
  unresolved design work): owner review of the allowlist — notably
  `docs/_Registers`, `docs/_ScopeChange`, and `provenance/build-artifacts`
  — belongs to the pre-publication D-20 scan pass.
- The one standards-table-signature warning (hand-calc benchmark doc) is
  routed to the owner's D-20 review; it is a heuristic warning, not a
  blocking finding.
