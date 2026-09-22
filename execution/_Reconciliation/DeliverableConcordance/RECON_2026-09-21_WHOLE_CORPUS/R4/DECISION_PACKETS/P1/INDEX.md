# P1 decision packets — authority, baseline and architecture

R3 integration, TASK P1, run HELP-HUMAN-PIPING-20260921-RECONCILIATION. Proposals for the R4 gate; nothing here decides or changes anything.

| ID | Question | Holder | Rows (portion) | File |
|---|---|---|---|---|
| A1 | Do Python engines and contract builders under `core/` satisfy DEC-009's Rust core: port, permit or split (including the PKG-16 engines with no row)? | OWNER | 14 | [A1_dec009-python-engines.md](A1_dec009-python-engines.md) |
| A2 | Does "JCS-compatible" require RFC 8785 bytes, or may named paths hash labelled sorted-compact JSON (DEL-17-03 AC-001, MBF label, frozen profile contract version)? | OWNER | 12 | [A2_json-hash-basis.md](A2_json-hash-basis.md) |
| A3 | Do holds that code settled with no ruling need an owner ruling (CP-10) or record catch-up; and per topic, does the implemented choice close each of 13 holds? | OWNER (CP-10 amendment via HELPS_HUMANS if 1b) | 50 | [A3_cp10-holds-settled-in-code.md](A3_cp10-holds-settled-in-code.md) |
| A4 | One rename ruling: text sweep, code identifiers, and whether the four identifiers kept on 2026-09-18 are renamed or kept | OWNER | 85 | [A4_rename-identity-residue.md](A4_rename-identity-residue.md) |
| A5 | Target lifecycle state and workflow for PKG-00 (Direction 8), and how the injection statements are restated | OWNER (lifecycle workflow) | 82 | [A5_pkg00-semantic-ready-lifecycle.md](A5_pkg00-semantic-ready-lifecycle.md) |
| A6 | ISSUED DEL-01-01: reissue, amend or keep as history; confirm the DEC-081 Wave 2 edit; single route for `DEL-01-01:SOW` | OWNER; change via WORKING_ITEMS (workflow: scope-change) | 30 | [A6_issued-del-01-01-change-path.md](A6_issued-del-01-01-change-path.md) |
| A7 | Deleted export plan: restore, re-point or retire; and where the N7 intake evidence is | OWNER (CF-001 re-scope via WORKING_ITEMS, workflow: scope-change) | 22 | [A7_deleted-export-plan-and-n7-intake.md](A7_deleted-export-plan-and-n7-intake.md) |
| A8 | Product posture (FOSS vs source-available); pre-release legal review; palette landing and secret provider rows (follow B5, B12) | OWNER | 5 | [A8_product-posture-and-silent-sources.md](A8_product-posture-and-silent-sources.md) |
| A9 | D-41 "current declaration" blocks: re-pin or retire (one treatment for all) | OWNER (treatment choice inside R5) | 213 | [A9_d41-current-declaration-blocks.md](A9_d41-current-declaration-blocks.md) |
| A10 | Baseline rows overtaken: D-68 / DEL-00-05, SCA-004 / DEL-00-07, DEC-058 / DEL-08-05, DEC-051 / DEL-12-05, PB-TBD-003 release-label floor | OWNER | 8 | [A10_baseline-rows-overtaken-by-rulings.md](A10_baseline-rows-overtaken-by-rulings.md) |

**Totals.** 521 claim rows across 13 classes, each counted once. Subject-keyed items: A5 (T9-C07 ×9, T9-C12 SRE-5 ×1), A6 (T9-C10 ×1).

**Split-class portions (for Agent 0's cross-packet check).**
- T4A-C06 (14): A1 7 (DEC-009 ABI rows), A7 5 (export plan), A10 2 (DEL-09-05).
- T5B-C07 (22): A6 16 (DEL-01-01), A10 6 (non-DEL-01-01).
- T4B-C02 (5; not listed as split in the topic file): A6 4 (DEL-01-01), A2 1 (DEL-17-03 AC-001).
- T4B-C01 (86; not listed as split in the topic file): A4 85, A6 1 (`DEL-01-01:SOW`).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
