# WORKING_ITEMS RUN - TP-E7-SCANEXT-001

## Scope

- Package: PKG-08 Reporting, Audit, and Reproducibility.
- Primary deliverable: DEL-08-05 Report protected-content linter.
- Tranche: `TP-E7-SCANEXT-001` — the DEC-058 (D-20 ruling, register row
  `D-20`) scanner-extension tranche: agent-lawful scanner extension and
  record tooling only. The scan act itself is RESERVED to the owner
  (sole maintainer and release authority, DEC-027): nothing is scanned,
  cleared, or published under this tranche.
- Authority: `DEC-058` (`execution/_Decomposition/SOFTWARE_DECOMP.md` §12);
  packet `execution/_Coordination/_DECISIONS/D-20_protected_content_scan.md`
  §9 procedure text as written.

## Primary Evidence

- DEL-08-05 engine extension (DEC-058 check (b)):
  `core/reporting/protected_content_linter/src/lib.rs` now carries
  standards-table signature detection — a standards designator name or
  standards-clause label (e.g. allowable-stress / stress-intensification /
  flexibility-factor table labels) within eight lines of a dense numeric
  grid (>= 3 consecutive rows of >= 4 numeric fields at >= 60 percent
  numeric density, with plain-text, CSV/pipe, JSON-shaped, and simple
  markup-wrapped rows recognized). Matches fail toward
  `UnknownProvenanceReviewRequired` (severity `Warning`, class
  `ProvenanceWarning`, review route `HumanIpReview`, policy `OPS-K-IP-3`)
  — human review, never silent pass, and never a protected-content
  determination. Detection tokens are designator names and generic clause
  labels only; no standards text, table values, or other protected content
  is embedded. Recorded provenance does not suppress the shape check.
- Engine CLI surface:
  `core/reporting/protected_content_linter/src/bin/protected_content_lint_cli.rs`
  — reads caller-named files, runs the engine, prints deterministic JSON
  findings (`--provenance-mode engine|external`); zero new dependencies
  (crate remains dependency-free per the DEC-023 posture).
- Release-candidate scan runner and record emitter (DEC-058 checks (a)-(e)):
  `tools/release/run_release_candidate_scan.py` (stdlib-only) — requires
  every artifact class AC-1..AC-6 to be scanned or explicitly
  `not_applicable` with a reason (never silently omitted); walks per-class
  inventories with per-artifact sha256; runs the DEL-08-05 engine CLI over
  lintable members; runs the `docs/IP_AND_DATA_BOUNDARY.md` §4
  provenance-manifest check (blocks on `unknown`/`TBD`/missing
  certification); invokes `check_release_readiness.py --profile security`;
  checks quarantine hygiene; and emits
  `validation/evidence/releases/SCAN_<candidate>_<utc>_<commit12>.json`
  UNSIGNED with the owner sign-off block explicitly pending
  (`signed: false`, `status: pending_owner_review`, tooling never signs).
  Skipped or failed checks record `not_run`/`findings_present` and the
  disposition fails toward owner review. Plan-only by default; `--execute`
  required to write.
- `validation/evidence/releases/` is left absent: per DEC-058 the scan act
  and record creation for a real candidate are the owner's; the self-test
  emits records only into pytest temp directories.

## Validation

- `cargo test --manifest-path core/reporting/protected_content_linter/Cargo.toml`
  passed 15/15 (4 pre-existing + 11 new standards-signature tests:
  positive designator/clause/JSON/markup lookalikes, negative
  prose/plain-grid/sparse/substring/window cases, stable ordering,
  provenance-independence).
- `python3 -m pytest -q tests/test_release_candidate_scan.py` passed 14/14
  (record naming/fields, six-class completeness, checksums, byte
  determinism given fixed inputs, unsigned sign-off block, fail-toward-
  review on skipped checks, provenance blocking, quarantine hygiene,
  normalized findings, no release-readiness claim, engine-CLI integration
  positive and clean-content negative).
- Repo-wide `python3 -m pytest -q tests` passed 401/401.
- Repo-root `PYTHONDONTWRITEBYTECODE=1 python3
  tools/practitioner_harness/harness.py self-check` exited 0 (pre-existing
  REVIEW/WARN findings only; none touch this tranche's files).
- Practitioner-harness pytest passed 263 passed, 1 skipped.
- DEC-025 five-surface sweep at the committed clean head `1d160589cbea`:
  attempt 1 (`SWEEP_20260711T012456Z_1d160589cbea.json`, not committed)
  failed environmentally at `desktop_vitest` — `vitest: command not found`
  because the fresh worktree had no `node_modules`; remediation was
  `npm ci --prefer-offline` from the existing lockfile (no dependency
  changes); attempt 2 passed all five surfaces —
  `validation/evidence/sweeps/SWEEP_20260711T012810Z_1d160589cbea.json`
  (`overall: pass`).
- All test fixtures are invented lookalikes (designator names plus
  obviously synthetic sequential values) created inline or in temp
  directories; no protected standards data was used or embedded.

## Boundary

- F-PIP-1 held: local-only, no network services.
- F-PIP-2 held: the tooling documents checks; it does not clear, sign,
  gate, or publish. No release-readiness, certification, legal-clearance,
  professional-approval, sealing, authentication, or code-compliance claim
  is made or implied; `clean_scan_is_clearance` remains `false` everywhere.
- Owner-reserved acts untouched: no scan of any real release candidate was
  run, no record was emitted into `validation/evidence/releases/`, no
  finding disposition was recorded, no sign-off block was completed.
- No protected standards content entered the repository, including as test
  fixtures; no new external dependencies were added; no lifecycle
  transition occurred.
