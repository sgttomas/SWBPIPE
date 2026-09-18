# A1-DOCS — retained return

Retained by ROOT. What follows is the child's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed.

---

**Model:** Claude Sonnet 5.

**Brief verified:** SHA-256 of `A1-DOCS_sca010_core_docs.md` matched `2a45a9731a1953c8907a76a8fce15b2adf26e114da73c2af3a7e4758c392c17d`. Authority read whole: `SOFTWARE_DECOMP.md` §12 DEC-095–DEC-105, and all five SCA-010 bundle files.

## Files changed (grouped by work item)

**1. SCA-010 (PRD + notice template + pointer)**
- `docs/PRD.md` — 23 of 24 "OpenPipeStress"→"SWBPIPE" replacements (line 16's historical path `OpenPipeStress_PRD_v0.2.md` kept); header retitled, `**Product name:**`/`**Formerly:**` lines added, version 0.3→0.4, amended date and authority line updated per A004.
- `docs/report_notice_template.md` — 3 occurrences (lines 16, 41, 68).
- `execution/_ScopeChange/_LATEST.md` — pointer moved to SCA-010.

**2. Emitted report notice**
- `core/reporting/report_renderer/src/lib.rs` (line 719) — first word → SWBPIPE; "decision-support software" fragment kept.
- `core/reporting/report_renderer/tests/render.rs`, `core/reporting/report_package/tests/container.rs`, `core/reporting/pdf_emitter/tests/emit.rs` — report-title fixtures drop "(Technical Preview)" (these three crates share the same fixture pattern feeding the renderer's title; none had an assertion pinning the old text).

**3. DEC-100 in core**
- `core/solver/nonlinear_supports/src/lib.rs` — removed the acceptance short-variant sentence from `active_set_report_limitations()` (was free diagnostic prose, not schema/fixture-pinned — confirmed no serde derive, no schema, no external consumer); moved the test that pinned it (`active_set_report_preserves_nonconvergence_as_structured_diagnostic`) to assert the surviving "does not assemble" limitation instead.
- `core/solver/nonlinear_integration/src/lib.rs` — same removal from `assembled_loop_limitations()`; no test pinned it.

**4. Documents (DEC-105 + DEC-101)**
- `docs/user_guide/index.md`, `docs/README.md`, `docs/BUILD_AND_RELEASE.md`, `docs/RELEASE_NOTES_TEMPLATE.md`, `docs/contributor_guide/index.md`, `docs/developer_guide/index.md`, `docs/validation_manual/index.md`, project `README.md`, `CONTRIBUTING.md`, and the piping issue templates (`.github/ISSUE_TEMPLATE/{README.md,bug_report.yml,question.yml,documentation_feedback.yml,config.yml}`) — prose renamed to SWBPIPE. No maturity sentence ("Technical preview — not a released product.") was present in any of these files, so DEC-105 removal had nothing to do here (its only live-surface occurrences are `docs/claims_registry.md`, a sibling's file, and `docs/security/redaction_export_controls.md`, excluded by "Not now").

**5. DEC-103 item 9 (user guide, CAEPIPE)**
- `docs/user_guide/index.md` — the export-interoperability table row and the export prose (near old lines 65, 273–274) now read "the export of a `.mbf` model batch file" / "optional user-owned external-run evidence"; the two schema file names containing "caepipe" (`caepipe_mbf_export.schema.json`, `caepipe_external_run.schema.json`) were dropped from the visible table row rather than shown, per the brief's own instruction. Lines 43 and 186 (`HUMAN_APPROVED_FOR_PROJECT`) untouched; `tests/test_user_guide_status_wording.py` still passes.

**6. Packaging tools and Python tests**
- `tools/release/package_release_artifact.py` — two prose "OpenPipeStress" mentions → SWBPIPE (productName/identifier are read dynamically from `tauri.conf.json`, already "SWBPIPE"/"com.swbpipe.desktop" — a sibling landed that).
- `tools/release/generate_app_icon.py` — prose only, as instructed.
- `tests/test_release_packaging_script.py` — fixture identity dict now `{"product_name": "SWBPIPE", "identifier": "com.swbpipe.desktop", ...}`; end-to-end zip-name assertion updated to `SWBPIPE_0.1.0_aarch64-apple-darwin.app.zip`; a docstring example updated to `'SWBPIPE.app'`.

## Checks run

- `python3 -m pytest -q tests -n auto --dist loadscope` → initially 41 failed/7 errored, all traced to a pre-existing missing `jsonschema` dependency in this environment (unrelated to my edits — `pip show jsonschema` found nothing). Installed `jsonschema==4.26.0` from `requirements-dev.txt`'s pin, reran: **1105 passed, 0 failed, 18 skipped→0 (all collected), 304 warnings** (pre-existing `RefResolver` deprecation warnings, not mine). `tests/test_claims_language_surface.py` **passed** (the brief expected it might fail pending sibling integration; at this snapshot the sibling's registry/lint work is already consistent, so no failure occurred — reporting this as the actual, not assumed, result). `tests/test_release_packaging_script.py` and `tests/test_user_guide_status_wording.py` verified individually: all pass.
- `cargo test` (offline, `CARGO_NET_OFFLINE=true`), one per changed crate:
  - `core/solver/nonlinear_supports` → 22 passed.
  - `core/solver/nonlinear_integration` → 44 passed.
  - `core/reporting/report_renderer` → 8 passed.
  - `core/reporting/report_package` → 19 passed (7 + 12), including the `document_kind`-pinning tests, confirming `CONTAINER_DOCUMENT_KIND` was correctly left untouched.
  - `core/reporting/pdf_emitter` → 8 passed.
- My own searches: `grep -c SWBPIPE docs/PRD.md` = 23; `grep -n OpenPipeStress docs/PRD.md` shows exactly 2 remaining (line 4, the new Formerly line; line 16, the historical path, unchanged); the §19.3 notice in `docs/PRD.md` (×2), `docs/report_notice_template.md`, and `core/reporting/report_renderer/src/lib.rs` all start "SWBPIPE is decision-support…"; `docs/report_notice_template.md` has 3 SWBPIPE / 0 OpenPipeStress.

## Strings deliberately left as identity

- `docs/PRD.md:16` — `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` (historical file path, SCA-010's own exception).
- `docs/BUILD_AND_RELEASE.md:358` and `core/reporting/report_package/src/lib.rs:58` (`CONTAINER_DOCUMENT_KIND`) — `"OpenPipeStress Project Package"`: written into a saved manifest (`document_kind` field) and asserted by `assert_eq!` in `report_package/tests/container.rs`; a document-kind string, not display prose.
- `core/rules/rule_check_runner/src/lib.rs:78-80` (`PROFESSIONAL_BOUNDARY_NOTICE`) — contains the same acceptance phrase removed from the solver crates; explicitly named by the brief as staying (schema const).
- `tests/test_release_packaging_script.py:100` — `"artifact": "openpipestress.evidence_sweep_summary"`, a dotted artifact-kind identifier.
- `tests/test_release_packaging_script.py` docstring — `openpipestress-desktop` (binary name) kept; only the illustrative bundle-name text around it was updated.
- `docs/user_guide/index.md` — schema file names `caepipe_mbf_export.schema.json` and `caepipe_external_run.schema.json` (contain "caepipe"), omitted from the visible table row rather than shown, per item 5's own instruction.

## Judgment calls (not failures, flagging for ROOT)

- The brief says "the five `docs/*/index.md` guides," but only four non-excluded ones exist (`contributor_guide`, `developer_guide`, `user_guide`, `validation_manual`); the fifth `index.md` in the repo is `docs/architecture/adr/index.md`, which the same brief item excludes under "Not now: … `docs/architecture/**`." I renamed the four and left architecture/adr/index.md untouched.
- `{REPO_ROOT}/.github/ISSUE_TEMPLATE/**` does not exist; the piping project's issue templates live at `projects/chirality-piping/.github/ISSUE_TEMPLATE/**` (confirmed by that directory's own README as "the piping project's" templates). I treated that as "the piping issue templates" the brief names.
- Dropped "(Technical Preview)" from report-title test fixtures in `report_package` and `pdf_emitter`, not just `report_renderer`, since all three share the exact copy-pasted fixture string and the instruction reads as a general rule ("report titles drop `(Technical Preview)`").

## Noticed outside scope — for a sibling or ROOT

- `docs/claims_registry.md` and root `tools/validation/validate_claims_language.py`/its test are mid-edit by a sibling in this same worktree (visible in `git status`); untouched by me as instructed.
- `apps/desktop/**` (many files) is being actively edited by a sibling; untouched.
- Prose "OpenPipeStress" still appears, un-renamed, outside this tranche's named file list: `governance/MAINTAINERS.md`, `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`, and several `tools/release/*` files not named by item 6 (`coverage_telemetry_schema.json`, `run_release_gate_records.py`, `run_evidence_sweep.py`, `run_coverage_telemetry.py`, `release_gate_record_schema.json`, `check_release_readiness.py`, and `export_public_openpipestress.py` — the last one's filename itself contains the old name and is a file name, identity layer). Also untouched: test-fixture `contributor`/`source_name` provenance strings and doc comments across `report_package`, `report_sections`, `audit_manifest`, `result_export`, `report_generator`, `protected_content_linter`, and the PDF `/Title`/`/Producer` constants in `pdf_emitter/src/lib.rs` — none of these were named by items 2 or 3, so I left the sweep bounded to the brief's explicit list rather than expanding it.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
