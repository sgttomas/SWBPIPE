# NOTES — DEL-11-02 Developer guide for solver and rule packs (R2 W4)

DEL-11-02 / PKG-11, IN_PROGRESS; frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Role/model: **GPT-5 discovery
pilot**, no substitution. Ledger: 30 rows, 20 columns, RFC-4180 CRLF.

ClaimType census: REQUIREMENT 14; ACCEPTANCE 8; EXCLUSION 2; DECLARED_STATE
6; no REMAINING_WORK and no IMPLEMENTED_UNMAPPED. Dispositions: ALIGNED 26;
STALE_SETUP_SPECIFICATION 4. AuthorityNeeded: NO 26 / OWNER 4. All rows are
non-selectable because `_STATUS.md` contains only the D-41 bootstrap.

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

All four kit documents retain setup/future-only factual framing after the guide
landed, so DECL-001..004 are STALE. Status and MEMORY are current and ALIGNED.
The guide itself says decomposition rev 0.7 with DAG-007 while frozen current
decomposition is rev 0.8; this is disclosed as authority-pointer drift and does
not change the substance dispositions. No current non-bootstrap residual is
recorded; the guide's upstream TBD list remains explicit and homed to its
owning implementation/governance surfaces.

No test, cargo, pytest, generator, or compilation was re-executed. Recorded
Tranche A and 2026-06-07 review/run evidence is attributed and marked not
re-executed by W4. Technical rows use weakest-leg UNVERIFIED; DECL prose uses
NOT_APPLICABLE. SECURITY requirement/acceptance rows state deterministic
sandbox/content boundaries without claiming sufficiency or legal clearance.

Fan-in self-flags: eight-row acceptance grain; setup-run-grain treatment of
REQ-014/ACC-008; all-four-kit STALE choice despite still-valid principles; and
rev-0.7/DAG-007 guide pointer drift treated as a note rather than an extra
non-census declaration.

Frozen `git status --short --ignored=matching` before/after showed exactly the
six allow-listed pre-existing paths and no others. Frozen state was not cleaned
or modified. Writes were limited to this CSV and notes. No lifecycle, DAG,
register, decision, product, R4, or R5 surface changed. Dispositions are agent
judgments, not human rulings. No material authority conflict was found.

