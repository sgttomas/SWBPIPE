# NOTES — DEL-11-02 Developer guide for solver and rule packs (R2 W4)

DEL-11-02 / PKG-11, IN_PROGRESS; frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Role/model: **GPT-5 discovery
pilot**, no substitution. Ledger: 31 rows, 20 columns, RFC-4180 CRLF.

ClaimType census: REQUIREMENT 14; ACCEPTANCE 8; EXCLUSION 2; DECLARED_STATE
6; REMAINING_WORK 1; no IMPLEMENTED_UNMAPPED. Dispositions: ALIGNED 24;
PARTIALLY_IMPLEMENTED 2; STALE_SETUP_SPECIFICATION 4;
REMAINING_STATE_MISMATCH 1. AuthorityNeeded: NO 24 / OWNER 7. All rows are
non-selectable: `_STATUS.md` contains only the D-41 bootstrap, and the omitted
guide-currentness residual has no recorded gate or status item.

The 14 native requirements and eight distinct Verification bullets are
re-keyed contiguously. Two exclusions preserve the durable public/protected/
professional boundary and the historical setup-session write boundary. The
mandatory declaration census is four kit docs + `_STATUS.md` + MEMORY; no
deliverable-owned README exists. The bootstrap is copied only in DECL-005
`RecordedRemaining` and excluded from residual/gate/selectability analysis.

`docs/developer_guide/index.md` implements the requirement set at guide-contract
grain: architecture/no-bypass map, solver mechanics, rule-pack schema and
sandbox, data/privacy/provenance boundaries, diagnostics, evidence families,
and contribution checklist. REQ-014/ACC-008 are ALIGNED at setup-run grain:
the setup session respected its fence, while the final guide was created later
under a separately authorized tranche. This follows the W3 setup-run-grain
calibration; later implementation drift belongs on DECL rows.

All four kit documents retain overtaken factual instructions after the guide
landed, so DECL-001..004 are STALE. In particular, Guidance and Procedure still
call the DEC-022 canonical grammar TBD, and Procedure prefers decomposition
revision 0.7 and conflates the selected project license with the still-open
contributor legal mechanism. Status and MEMORY are current and ALIGNED. The
product guide repeats revision 0.7 and the false grammar-TBD, so REQ-012 and
ACC-007 are PARTIALLY_IMPLEMENTED. Because `_STATUS.md` has no non-bootstrap
home for that currentness work, REM-001 records the mismatch with no recorded
item, source, or gate and remains non-selectable.

No test, cargo, pytest, generator, or compilation was re-executed. Recorded
Tranche A and 2026-06-07 review/run evidence is attributed and marked not
re-executed by W4. Technical rows use weakest-leg UNVERIFIED; DECL prose uses
NOT_APPLICABLE. SECURITY requirement/acceptance rows state deterministic
sandbox/content boundaries without claiming sufficiency or legal clearance.

Fan-in corrections preserve the eight-row acceptance grain and setup-run-grain
treatment of REQ-014/ACC-008 while recording the material currentness defect:
REQ-012/ACC-007 are partial, DECL-004 is stale on its overtaken revision,
grammar, and license instructions, and REM-001 owns the unrecorded product-guide
currentness work.

Frozen `git status --short --ignored=matching` before/after showed exactly the
six allow-listed pre-existing paths and no others. Frozen state was not cleaned
or modified. Writes were limited to this CSV and notes. No lifecycle, DAG,
register, decision, product, R4, or R5 surface changed. Dispositions are agent
judgments, not human rulings. No material authority conflict was found.
