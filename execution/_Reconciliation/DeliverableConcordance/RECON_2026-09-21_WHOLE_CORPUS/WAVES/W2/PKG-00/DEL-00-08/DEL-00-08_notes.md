# DEL-00-08 notes — Layered software test and acceptance strategy

Wave W2, package PKG-00, worker G2. Forward ledger 38 rows (30 required keys,
2 `.rNN` rows for the split realized-artifacts block, 6 `.sNN` sub-claims).
Reverse file 387 answers. Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`.

## Path aliases

- `AB` = `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/ArchitectureBasis.md` (cited only in `NormativeSource`/`ContextRefs`).
- Hosted CI workflow: repository-root `.github/workflows/piping-desktop-e2e.yml`; DEC-093 at SOFTWARE_DECOMP L684; D-65 ruling record in `_DECISIONS/`.
- Test-file census at the freeze (outside `execution/`): 84 Python test modules, 100 desktop unit test files, 21 Playwright specs, one `.test.mjs`.

## Judgment calls

- **CI-platform bullet (resolved-decisions.s01).** "Hosted CI deferred; the local sweep is the merge gate" was written 2026-07-15. D-65 (ruled 2026-08-19, DEC-093) accepted the hosted desktop E2E workflow as an alternative DEC-025 surface-4 path, the sweep tool carries that binding, and hosted checks ran on PR #834. STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · NO: a catch-up citing DEC-093 needs no decision. The DEC-059 public-repo clause is still accurate.
- **Test-file count (realized-artifacts.r01).** "On the order of 75-100 test files" against about 205 at the freeze: a stale count in prose, not a protected check. DOC_BEHIND_CODE.
- **Rename residue (SURFACE, CP-04).** `.opsproj` in the release-matrix bullet; `.opsproj` fields with the DEC-101 persistence obligation. Header pin moved to `AB.s01` (CP-02).
- REQ-08-02 ALIGNED MEDIUM: deterministic solver and rule verification exists (cargo tests, benchmark suites); no release exists yet, and the verification is not promoted to validation (A5).
- REQ-08-04 ALIGNED MEDIUM: the D-43 architecture-basis validator and the D-40-preserved PKG-00 lock review are the acceptance gates; neither claims release quality.
- REQ-08-05 ALIGNED MEDIUM: every CI/tooling choice in the frozen tree traces to a ruling; the basis's own stale record is judged on resolved-decisions.s01.
- Resolved-decisions `.s02`: the bullet labels DEC-026 as the tolerance ruling, which it is; coverage tooling itself is DEC-060. Not a finding.
- Open-holds block ALIGNED: one coverage telemetry artifact exists, short of the DEC-060 promotion trigger, so floors are rightly unset.

## Canonical departures

None.

## Convention friction

- CP-02 `AuthorityNeeded NO` versus the manifest hash binding: same friction as DEL-00-05.
- DEC-093 is not a "setup" change, so resolved-decisions.s01 uses STALE_REVIEW_OR_EVIDENCE (text first declared 2026-07-15, F3).

## UNKNOWN rows

None.

## Reverse pass

18 CONSTRAINS, 1 PARTIAL, 1 COVERS, 367 NOT_MINE. PARTIAL on RC-00-0285 (architecture-basis validator) under REQ-08-04: this member defines the PKG-00 acceptance gate the validator implements package-wide. Benchmark suites are CONSTRAINS REQ-08-02; hand calculations and witnesses are NOT_MINE (PKG-09 validation material). The reverse pass confirmed resolved-decisions.s01 (RC-00-0136, RC-00-0317) and did not change any sealed view.

## Batch consistency

Batch over the four G2 ledgers: PASS, 0 findings. Shared bodies with DEL-00-07 (required invariants, currency) and DEL-00-06 (STATUS#history) take matching treatment.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
