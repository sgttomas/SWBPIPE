# NOTES — DEL-11-01 User guide skeleton (R2 wave W4)

PKG-11 / DEL-11-01, IN_PROGRESS. Frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger: 19 rows, 20 columns,
RFC-4180 CRLF. Discovery role/model: **GPT-5 discovery pilot**, no substitution.

## Histograms

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 10 |
| ACCEPTANCE | 0 |
| EXCLUSION | 2 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 1 |

| Disposition | Count |
|---|---:|
| ALIGNED | 13 |
| STALE_SETUP_SPECIFICATION | 4 |
| PARTIALLY_IMPLEMENTED | 1 |
| REMAINING_STATE_MISMATCH | 1 |

AuthorityNeeded recount: **NO 13 / OWNER 6**.

## Census and decisions

- Ten native UG requirements map one-for-one to fixed REQ IDs.
- No ACC rows: the Specification has verification hooks/table but no distinct acceptance
  section; mirroring requirement verification would double-weight the same claims.
- Two EXCs separate historical setup no-product-edit scope from permanent protected-content/
  professional-claim boundaries.
- DECL census is four kit docs + `_STATUS` + MEMORY. `docs/user_guide/index.md` is the
  implemented product artifact, not an owned README and not an addendum-1 census surface.
- The guide is structurally present and broad, but REQ-002 is PARTIALLY_IMPLEMENTED because
  its current inventory still calls revision 0.7 authoritative and marks several later ruled
  or implemented surfaces TBD. REM-001 records the missing `_STATUS` home for that refresh.
- REQ-004/009 SourceReliability is REVIEWED only because the named 2026-06-07 human
  disposition covers the exact external hash-bound status wording. Other technical/document
  rows remain weakest-leg UNVERIFIED.
- SECURITY REQ-008 is a bounded guide-content/prohibitive-boundary check with explicit
  `NOT_APPLICABLE` reasoning and NO routing; it does not claim legal clearance or
  protected-content sufficiency.
- Guidance DECL-003 is ALIGNED because its advisory and conditional principles remain
  current. Procedure DECL-004 remains STALE because its two named validation helpers are
  absent and its setup-state workflow no longer matches the IN_PROGRESS status.

## Checks and containment

Re-executed with `PYTHONDONTWRITEBYTECODE=1` and `pytest -p no:cacheprovider`:

- `test_user_guide_status_wording.py`: **1/1 PASS**.
- dependency schema validator: **VALID**, 29 columns, 20 rows.

Before/after ignored porcelain showed exactly the six disclosed allow-listed sets and no
seventh path; tracked porcelain stayed empty. No cargo, `py_compile`, clean, or frozen write.

Rev-0.7 drift is captured on Datasheet DECL-002 and product-guide REQ-002/REM-001; dated
MEMORY authority entries remain historical notes. The undated MEMORY head is current-shaped,
so its blanket TBD inventory takes DECL staleness. Bootstrap text appears only on DECL-005
and is excluded from residual/selectability analysis.

No material authority contradiction was found. No lifecycle, DAG, product, kit, review,
decision or dependency file changed. This ledger makes no release, legal-clearance,
professional-approval, certification, sealing, authentication or code-compliance claim.
Dispositions are agent judgments, never human rulings.
