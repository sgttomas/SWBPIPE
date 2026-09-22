# DEL-10-04 notes: Build, packaging, and CI/CD pipeline

Worker: W3 PKG-10 G2 (TASK). Brief `R2-WORKER_brief.md` (SHA-256 `2d793d0a…0db141`, verified).
Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`. The forward ledger is sealed in
`DEL-10-04_SEAL.txt`. DEL-10-04 was not an R0 pilot.

## Counts

Forward: 128 rows (71 required keys, 32 `.rNN` rows, 25 `.sNN` rows).
- 58 ALIGNED, 29 STALE_SETUP_SPECIFICATION, 13 STALE_REVIEW_OR_EVIDENCE.
- 16 COVERED_BY_CHILDREN, 9 NOT_ASSESSED.
- 1 each of PARTIALLY_IMPLEMENTED, IMPLEMENTED_DIFFERENTLY and REMAINING_STATE_MISMATCH.
- No UNKNOWN rows.

Reverse: 387 capabilities. 14 CLAIMED_BY, 2 PARTIAL, 6 COVERS, 2 UNKEYED, 363 NOT_MINE.

## Path aliases

- `D4/` = `projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/`.
- The parity records are cited as repository-root `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/...`, which is where they live.
- Release tools and docs are cited in the explicit project form (`projects/chirality-piping/tools/release/...`, `projects/chirality-piping/docs/...`), because the repository root has its own `tools/` and `docs/`.
- `.github/workflows/piping-desktop-e2e.yml` and `.github/actions/setup-piping-e2e/action.yml` are repository-root files.

## Judgment calls

- **Split blocks.** CLM-003, CLM-004, CLM-005 and CLM-015 are split into all their `.rNN` rows, because their items take different dispositions. `.sNN` sub-claims are used where parts differ and no `.rNN` keys exist: CLM-006, CLM-008, CLM-013, CLM-014, CLM-016, CLM-020, CLM-021, CLM-028, CLM-031, the architecture-basis injection and MEMORY. Each split parent is CONTAINER.
- **Rulings that overtook setup-era TBDs (FG-DEL-10-04-02).** CI provider, platform matrix, installer, signing and publication were ruled after setup:
  - DEC-025 (hosted CI deferred; the local sweep is the merge gate).
  - DEC-059 (public-export GitHub Actions at first publication).
  - DEC-093 (the private-repo e2e workflow is accepted as the surface-4 alternative).
  - DEC-057 (v0.1 matrix aarch64-apple-darwin, `.app` zip, GitHub Releases target).
  - DEC-089 (policy-only future signing).

  The text is F3 origin text (7bee9ae41), so these rows are STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN. Coverage and performance thresholds are still unruled numerically (DEC-060 has no floor), so the rows limited to them are ALIGNED.
- **REQ-10-04-01 is PARTIALLY_IMPLEMENTED · DEFERRED_BY_RULING.**
  - Build: the DEC-025 sweep. Packaging: DEC-057 mechanics. CI: the DEC-093-accepted private e2e workflow. All exist.
  - The public-export CI (DEC-059) and any publication (CD) wait for the first public publication.
  - Tier PROJECT_BASELINE (it restates SOW-032). BaselineClass RULED_CRITERION (the DEC-059 gate). Layer BASELINE.
  - Remaining R01 is ALIGNED with `OPEN_ACTION` pointing to this row (F2).
- **R02 is CP-07.** D-06b was ruled 2026-07-25 (DEC-089), but the item still waits on it: REMAINING_STATE_MISMATCH · RULED_CRITERION.
- **Possible defect: CLM-013.s02, SURF-011 BuildReadinessPanel (FG-DEL-10-04-06).**
  - The panel is rendered in the product and hard-codes `bundle_active: false`, `bundle_icon_count: 0` and `installer_format: "TBD"`. It also emits `BUILD-READINESS-BUNDLE-INACTIVE`.
  - `tauri.conf.json` has had `bundle.active: true`, `targets: ["app"]` since TP-E5-PACKAGING-001 (2026-07-10).
  - The panel also lists the matrix, signing, checksum publication and publishing as TBD, although DEC-057 and DEC-089 ruled them.
  - Disposition: IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · PROJECT_BASELINE · AuthorityNeeded REVIEW.
  - The routing note for RC-10-0302 independently says the panel's values are hard-coded and can drift.
- **Anticipated artifacts rows (CLM-004.r02, CONTEXT#anticipated-artifacts) are ALIGNED (MEDIUM).** CI workflows, packaging scripts and a release notes template all exist. The hosted workflow's ownership is tested in the reverse pass (claimed; see below).
- **CLM-015.r10 and CLM-013.s03 are ALIGNED.** Section 7 still frames hosted CI as future, but the rows' claims are about the mapping's agreement with the live scripts and configs, and that agreement holds. The hosted workflow's setup action uses the same provisioning.

  Observation for R3, not a keyed claim: `docs/BUILD_AND_RELEASE.md` §2, §7 and §9 do not mention DEC-093, DEC-089 or the existing `piping-desktop-e2e.yml`. §2 still says GitHub Actions on the private monorepo is prohibited unless §7 records a private-data-handling authorization. The guide lags the rulings. No SOW key states the guide's currency, so this is not a ledger row.
- **CLM-032 (setup conflict table) is ALIGNED as HISTORY.** The later D-42 attribution conflict does not falsify the setup-pass statement.
- **CONTEXT#description is ALIGNED (MEDIUM).** "Supported platforms" is read as the DEC-057 platform, and one local release-artifact record exists. Publication is a separate release-authority act.
- **The CONTEXT SURFACE and STATUS SURFACE rows are ALIGNED** on their own header substance. `_STATUS` "Last Updated" equals the newest history entry, so CP-05 does not apply.
- **MEMORY.s01.** The undated "Remaining TBDs" section is a current declaration inside a history surface (C1), so it is judged separately.

## Canonical departures

None. CS rows use their assigned fields.

The following CP situations were applied:
- CP-03 with CP-02 fields: CLM-002, CLM-010, CLM-025.
- CP-02: CLM-007, CLM-014.s01 (INIT.md is missing at the freeze; SOFTWARE_DECOMP pinned at 0.7).
- CP-01: CLM-015.r01, CLM-016.s01, CLM-020.s01, CLM-021.s01, CLM-022, CLM-023.
- CP-04: the SOW SURFACE row, default variant.
- CP-07: R02.
- CP-09: VER-001 and the output-matrix OUT-001. PASS records bind `1b7eb7b1…`; none matches the frozen `156a761b…`.

## Convention friction

- **CP-04 scope.** The SOW text says "OpenPipeStress boundaries" and "OpenPipeStress governance artifacts". The active identifiers it names through its evidence (root package name `openpipestress-workspace`, panel `document_kind` and download filename) carry the former name. All of this is recorded once on the SURFACE row, with the default fields, because none is one of the four special identifiers.
- **SOW frontmatter pin.** The `decomposition_basis` pin (`SOFTWARE_DECOMP.md@4d153302`) has no key. It is noted on the SURFACE row and not dispositioned separately.
- **F3 cutoff.** It classifies most of this SOW as setup-era. AC-001 (2026-07-14) and the CLM-013 parts are post-migration and take STALE_REVIEW_OR_EVIDENCE where stale.

## UNKNOWN rows

None.

## Reverse pass

Answers:
- **CLAIMED_BY:**
  - the evidence sweep, packaging, root package.json, readiness checker, release notes template, SURF-011 and the build guide;
  - the export pipeline and the icon generator;
  - the Piping e2e workflow, its setup action, the cache workflow, the e2e planner and CI_STRATEGY.md. Taken together these are the CI/CD workflows of SOW-032, and DEC-093 routes the CI surface-4 tooling to DEL-10-04.
- **PARTIAL:** the Tauri config (bundler only) and the wasm build script (F-4 atomic rider only).
- **COVERS:**
  - gate records (DEL-09-05);
  - coverage telemetry (DEC-060);
  - the release-candidate scan (DEL-08-05 per DEC-058);
  - the dependency-register validator (restored for the readiness profile);
  - the Playwright configs;
  - the preview example.
- **UNKEYED:** `package-lock.json` (nearest key CLM-013.s01) and `requirements-dev.txt` (nearest CLM-005.r05).
- **NOT_MINE with specific reasons (F5):** the App.tsx shell capabilities, governing documents, the shared product_preview fixtures, the lint CLI and frontend build config.

**Did the reverse pass change my view of anything sealed?** It confirms CLM-013.s02: the routing note for RC-10-0302 independently records that the panel's values are hard-coded and can drift. It also shows that the CI workflow family (RC-10-0272, 0232, 0085, 0353, 0346) has no DEL-10-04 key naming it beyond REQ-10-04-01 and the anticipated-artifacts row. If R3 wants a narrower key, these are candidates. No sealed row would change.

Disclosure: when I hashed my own forward files, a shell glob also printed the SHA-256 of the three other PKG-10 workers' forward files. I did not read their contents.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-10-04 and DEL-10-05: PASS, 0 consistency findings. The shared body hashes (package reference, decomposition reference, preparation notes) take identical canonical fields.

## Selectability

`SelectableUnderCurrentLoop` is NOT_APPLICABLE on every row (C9). Since 2026-09-19 Piping selects work through owner-steered work graphs.

## Verification basis

No suite was run. Test-pass statements rest on `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (all five surfaces PASS, Python 1,138 with no failures, product bytes identical to the freeze) and on frozen run records, which were not rerun.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Nothing here states or implies release, approval, compliance or certification. Agent dispositions are not owner rulings.
