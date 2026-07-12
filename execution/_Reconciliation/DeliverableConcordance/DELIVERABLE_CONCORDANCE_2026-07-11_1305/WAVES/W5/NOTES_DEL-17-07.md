# NOTES — DEL-17-07 Conservative PCF subset exporter (R2 W5)

Frozen evidence: `main` @
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Deliverable:
PKG-17 / DEL-17-07, lifecycle `IN_PROGRESS`. Role/model: **GPT-5
deliverable-grained owning pilot**.

Unqualified deliverable documents refer to
`execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/`
at the frozen SHA; other paths are relative to `projects/chirality-piping/`.

## Census and histograms

The exact adopted 20-column ledger contains **57 rows**.

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 23 |
| ACCEPTANCE | 24 |
| EXCLUSION | 1 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 3 |
| **Total** | **57** |

| Disposition | Count |
|---|---:|
| ALIGNED | 53 |
| STALE_SETUP_SPECIFICATION | 3 |
| REMAINING_STATE_MISMATCH | 1 |
| **Total** | **57** |

Confidence is HIGH 54 / MEDIUM 3. AuthorityNeeded is NO 51 / OWNER 6.
SourceReliability is UNVERIFIED 51 / NOT_APPLICABLE 6. All 57 rows are
mechanically non-selectable.

The 24 ACCEPTANCE rows preserve seven Verification items, nine Diagnostic and
Loss-Report Acceptance Scaffold rows, and eight Documentation bullets. The
declaration census is the four-document kit plus `_STATUS.md` and `MEMORY.md`;
no deliverable-owned README exists. Three remaining rows preserve the formal
Guidance conflict and both Review_Findings rows. Semantic proposals and
candidate-family repetitions are not double-counted as independent residuals.

## Self-flags for package fan-in

- **DECL-001/002/004 — STALE_SETUP_SPECIFICATION.** Specification, Datasheet,
  and Procedure retain Phase A/future profile, writer, fixture, or test prose
  that does not fully describe the frozen implemented builder/schema/three-
  fixture/seven-test slice. The substantive contracts remain usable; only the
  declaration surfaces are stale.
- **REM-002 — REMAINING_STATE_MISMATCH, MEDIUM, OWNER.** RF-001 is OPEN with
  `HumanDisposition=TBD`, proposes carrying Phase A wording for CHECKING and
  refreshing before issuance/publication, and has no current `_STATUS.md`
  home. This ledger makes no disposition or refresh.
- **REM-001/003 — overlapping target-profile TBDs.** CF-001 and RF-002 both
  cover profile/version; RF-002 additionally covers import behavior,
  support/restraint semantics, and direct ID carriage. ALIGNED means the TBDs
  are correctly visible, not resolved.
- **ACC-004 and unit rows — evidence grain.** Unit disclosure and desktop
  conversion witnesses establish deterministic metadata/conversion evidence,
  not engineering suitability, downstream import interpretation, or solver
  validation.
- **REQ-011 and ACC-003 — bounded foundation.** Deterministic invented
  straight-pipe PCF output is not PCF completeness, target-profile closure,
  or real downstream compatibility.
- The sole `_STATUS.md` remaining item is the byte-exact D-41 bootstrap. It is
  present only on DECL-005 and excluded from residual, gate, and selectability
  analysis.

## Verification and addendum-9 containment

Re-executed in the frozen worktree with `PYTHONDONTWRITEBYTECODE=1`, external
`PYTHONPYCACHEPREFIX=/tmp/d41-del1707-pycache`, and pytest cache disabled:

- `python3 -m pytest -q -p no:cacheprovider projects/chirality-piping/tests/test_pcf_export_package.py`
  — PASS, 7 tests.
- dependency validator — VALID, 29 columns, 21 rows.
- root four-document and minimum-fileset checks — PASS.
- root semantic-matrix and lens-register validators — VALID.

The current dependency row count is 21; the June 4 formal review's 14-row
count is historical pre-refresh evidence. No Cargo command or in-tree
`py_compile` ran. Ignored-aware porcelain before and after contained exactly
the six known allow-listed paths and no seventh path. Tracked porcelain
remained empty; the frozen worktree was not modified or cleaned.

## Cross-ledger and R3 risks

1. **Source versus exporter ownership:** DEL-17-01 owns PCF source/TBD basis,
   DEL-17-02 common export contracts, and DEL-17-07 the conservative package
   builder/profile/sidecar/loss slice.
2. **Profile-TBD duplication:** CF-001, RF-002, DEL-17-01 TBD-005 and multiple
   candidate-family TBDs describe linked decisions, not separate defects.
3. **Straight-pipe grain:** the frozen fixture proves only the declared
   invented straight-pipe foundation. Candidate elbows, tees, reducers,
   flanges, valves, supports, boundaries, and material mappings remain
   classified/TBD rather than implemented support.
4. **Desktop versus Python:** desktop unit witness visibility shares this
   deliverable but does not change Python schema support breadth or establish
   downstream import behavior.
5. **Review/status drift:** `_REVIEW.md` retains then-current CHECKING wording;
   frozen `_STATUS.md` is IN_PROGRESS under later lifecycle correction.
6. **External evidence:** frozen citations/guards were checked, not live
   vendor content, a redistributed PCF specification, or target compatibility.
7. **Dependency state:** pending/TBD satisfaction cells authorize no
   dependency, DAG, lifecycle, or source-authority edits.

## Fences

All dispositions are agent judgments, never human, owner, vendor,
engineering, professional, security, legal, validation, compatibility, or
release rulings. Writes were limited to this CSV and notes. No product,
deliverable, lifecycle, status, dependency, DAG, register, review, decision,
R4, or R5 surface changed. No approval, certification, sealing,
authentication, code-compliance, professional reliance, legal clearance,
security assurance, target compatibility, solver validation, or external
validation claim is made.
