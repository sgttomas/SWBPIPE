# Supplemental route screening

Original source: `BACKCHECK/R6_2026-09-22/CLAIM_DISPOSITIONS.csv`, same unresolved outcomes, PKG00–06, OriginalRoute other than R5_RECORD_REPAIR. All 336 screened; 27 carrier overlays and 309 original rows retained. See RETURN.md for subject examples and SUPPLEMENTAL_ACCOUNTING.csv for changed carriers.

| Original route | Screened | No additional carrier disposition (original row retained) |
|---|---:|---:|
| OWNER_DECISION | 114 | 102 |
| CODE_FIX_CANDIDATE | 84 | 83 |
| REVIEW | 114 | 100 |
| blank legacy route | 16 | 16 |
| ENGINEERING_AUTHORITY | 5 | 5 |
| SCOPE_CHANGE_HANDOFF | 3 | 3 |

Retained rows include concrete product/evidence acts, reserved decisions, issued-change paths and review parity tasks. They are not relabeled as no-change resolved. Mixed carrier overlays explicitly preserve product/review residuals, including manager SPEC repairs for DEL-02-02:SOW#CLM-018 and DEL-06-01:SOW#CLM-014.
