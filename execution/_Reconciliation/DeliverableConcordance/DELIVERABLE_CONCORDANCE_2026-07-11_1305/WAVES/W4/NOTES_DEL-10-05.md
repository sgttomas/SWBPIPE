# NOTES — DEL-10-05 Headless CLI and structured I/O analysis runner (R2 W4)

DEL-10-05 / PKG-10, IN_PROGRESS; frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Role/model: **GPT-5 discovery
pilot**, no substitution. This is an ordinary W4 re-encoding; R0 artifacts are
calibration only. Ledger: 20 rows, 20 columns, RFC-4180 CRLF.

## Histograms and census

ClaimType: REQUIREMENT 9; ACCEPTANCE 1; EXCLUSION 1; DECLARED_STATE 6;
REMAINING_WORK 3. Disposition: ALIGNED 12; PARTIALLY_IMPLEMENTED 2;
STALE_SETUP_SPECIFICATION 3; ACCEPTED_DIVERGENCE 2;
STALE_REVIEW_OR_EVIDENCE 1. Selectability: YES 7 / NO 13 (REQ-002, REQ-004,
ACC-001, DECL-005, and the three ungated residual rows).

The nine native R-01..09 requirements become contiguous REQ-001..009. One
distinct TP-RUNNER-015 Verification bullet-set claim becomes ACC-001; the
durable bounded-surface exclusion becomes EXC-001. Addendum-1 census is four
kit documents + `_STATUS.md` + MEMORY; no deliverable-owned README exists.
Three REM rows map exactly to the non-bootstrap status items. The D-41
bootstrap appears only in DECL-005 `RecordedRemaining` and is excluded from
residual/gate/selectability analysis.

R0's two IMPLEMENTED_UNMAPPED rows are not carried forward: the desktop
headless-runner panel is mapped to DEL-10-05 in R1, and optional full-envelope
payload validation is an internal named slice within the mapped runner crate,
below addendum-8 material-surface grain.

## Adjudication and calibration

REQ-001 and REQ-004 remain PARTIALLY_IMPLEMENTED at full requirement grain:
persisted-project/service breadth and downstream export payload binding remain
open. REM-001/002 are ACCEPTED_DIVERGENCE because DEC-065 explicitly permits
structured export/benchmark/regression stubs when downstream binding is not
implemented. REM-003 remains STALE_REVIEW_OR_EVIDENCE: the committed witness
has 822 result refs while the frozen status/Receipt 9 records the live runner
at 830; no human decision permits treating the stale witness as current.

DECL-001 consolidates the Specification's future-runner framing and overtaken
“exact schema files TBD” statement into one mandatory surface row. Datasheet is
current and ALIGNED. Guidance and Procedure retain future/setup-only facts
overtaken by DEC-065/TP-RUNNER-015 and are STALE. `_STATUS.md` and MEMORY are
ALIGNED. Dated rev-0.7/DAG-006 MEMORY text is historical, not current-state
staleness. Technical evidence uses weakest-leg UNVERIFIED; DECL prose uses
NOT_APPLICABLE.

No test, cargo command, pytest, generator, or compilation was re-executed.
Ledger cells preserve recorded-pass citations and explicitly say not
re-executed by the W4 pilot. SECURITY REQ-007 uses the convention-6 marker and
OWNER routing because fixture provenance/protected-content review sufficiency is
deferred; REQ-008 is the deterministic local/no-transmission row and does not
use the marker. No security-sufficiency conclusion is made.

Fan-in self-flags: the one-row ACC census intentionally treats the TP-RUNNER-015
bullet set as one distinct acceptance basis; REQ-001/004 bounded-slice grain;
DEC-065's permission scope on both stub residuals; and REM-003's
STALE_REVIEW_OR_EVIDENCE rather than a remaining-state mismatch.

## Containment and fences

Frozen `git status --short --ignored=matching` before/after showed exactly the
six allow-listed pre-existing ignored paths and no others. The frozen tree was
not cleaned or modified. With no execution, copy-out cargo, pytest-cache, and
bytecode mitigations were not invoked. Writes were limited to this CSV and
notes. No lifecycle, DAG, register, decision, product, R4, or R5 surface
changed. Dispositions are agent judgments, never human rulings. No material
authority conflict was found.
