# NOTES — DEL-11-04 Invented educational example models (R2 wave W4)

PKG-11 / DEL-11-04, IN_PROGRESS. Frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger: 16 rows, 20 columns,
RFC-4180 CRLF. Discovery role/model: **GPT-5 discovery pilot**, no substitution.

## Histograms

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 7 |
| ACCEPTANCE | 0 |
| EXCLUSION | 2 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 1 |

| Disposition | Count |
|---|---:|
| ALIGNED | 10 |
| STALE_SETUP_SPECIFICATION | 5 |
| REMAINING_STATE_MISMATCH | 1 |

No acceptance rows were minted: the setup Verification list restates the requirements and
session gates rather than defining a distinct acceptance section. DECL census is four kit
docs + `_STATUS` + MEMORY; no owned README exists.

## Key judgments

- Two material fixtures exist: mechanics-only toy span and fake-rule-pack toy model. Both
  validate against the full model contract, carry concrete hashes/provenance, persist/round-
  trip, and carry non-reliance notices. No unmapped row is warranted.
- REQ-003 and REQ-006 are ALIGNED on independent fixture/test evidence, but their load-bearing
  PKG-02 findings remain `TECHNICALLY_ADDRESSED_PENDING_HUMAN`; addendum 13 caps confidence
  at MEDIUM and routes OWNER.
- REM-001 records those two pending dispositions as omitted from `_STATUS`; it does not
  perform or infer the human act.
- SECURITY REQ-001 and EXC-002 scope the convention-6 marker to content-review sufficiency.
- Five stale DECLs are the four future/setup kit docs and MEMORY's undated head. Later dated
  MEMORY entries correct schema/checksum facts, but the undated current-shaped text itself
  remains false, so the corrected-in-file distinction does not make it ALIGNED.
- Bootstrap is copied only on DECL-005 and excluded from residual/selectability analysis.

## Checks and containment

Re-executed with bytecode disabled and pytest cache disabled:

- `tests/test_invented_example_models.py`: **7/7 PASS**.
- dependency schema validator: **VALID**, 29 columns, 17 rows.

Before/after ignored porcelain reproduced exactly the six disclosed allow-listed sets, no
seventh path; tracked porcelain stayed empty. No cargo, `py_compile`, clean, or frozen write.

No material contradiction was found. No lifecycle, DAG, product, fixture, kit, finding,
decision or dependency file changed. Educational/regression evidence is not represented as
engineering validation or real-project suitability. No release, certification, sealing,
professional-approval or code-compliance claim is made. Dispositions are agent judgments.
