# P3 decision packets — product intent, evidence, validation and method

Writer: TASK P3, R3 integration, run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
Proposals only; nothing here rules, repairs or executes.

| ID | Question | Holder | File |
|---|---|---|---|
| C1 | For eight subjects where code contradicts the SOW (DEL-15-03 export redaction since PR #307; diagnostic class, remediation and affected object; bare numbers at the import gate; the preview design-knowledge path; the DEC-026 fixture tolerance; DEL-16-02 runtime schema validation; the DEL-12-01 path-class vocabulary; the DEL-17-06 table inventory), is the code's behaviour intended? | OWNER (engineering part of the diagnostics subject in H3) | `C1_code-contradicts-sow-intent.md` |
| C2 | Does an agent reading or a denylist satisfy a claim naming a protected-content review, and should a bounded review round run, with PR #787's removed protected check restored or re-ruled? | OWNER (standard); WORKING_ITEMS (workflow: review) executes | `C2_protected-content-review-standard.md` |
| C3 | Keep or lift the held validation bases: the independent usability/security basis (D-68), external-prover activation (DEC-080), and the DEL-05-03 pressure reference hold? | OWNER; ENGINEERING after a hold lifts; EXTERNAL for procured or independent bases | `C3_validation-holds-and-prover-activation.md` |
| C4 | How far do the R18 guide-only exception and DEC-107 reach for DEL-11-01, and is DEL-11-04 R-DEL-11-04-002 kept or overtaken by the DEC-081/DEC-107 prohibition-list instruction? | OWNER | `C4_user-guide-exception-and-boundary-list.md` |
| C5 | Do single dimensional values coinciding with a published table fall under the IP boundary (PCF fixture), and how are the missing PCF and glTF fixture-provenance records supplied? | OWNER (interpretation); WORKING_ITEMS (workflow: review) for the dispositions | `C5_pcf-gltf-fixture-ip-provenance.md` |
| C6 | Fifteen method items (M1–M15): convention readings, gate and rerun rules, tool and routing-design gaps | OWNER (readings, gate rules); HELPS_HUMANS (workflow, validator, routing); ENGINEERING substance via H3 | `C6_method-items-for-adoption.md` |
| C7 | Confirm or replace six readings: SR-1 cause, unit-vocabulary tier, F1 on CONTEXT, the scope-item tier test, the acceptance-workflow cause, and the SCA-009 re-point on DEL-07-03 | OWNER (confirmation) | `C7_readings-for-owner-confirmation.md` |

## Class portions claimed by P3

| Class | Class rows | P3 portion | Packet |
|---|---|---|---|
| T7-C05 | 27 | 27 | C1 |
| T5A-C05 (split with B6) | 17 | 14 (not `DEL-17-04`) | C1 |
| T7-C09 | 31 | 31 | C2 |
| T5B-C02 | 12 | 12 | C2 |
| T7-C08 | 10 | 10 | C3 |
| T7-C07 | 7 | 2 (`DEL-05-03`) | C3 |
| T5B-C09 | 5 | 4 (`DEL-11-01`) | C4 |
| T6-C09 | 9 | 9 | C7 |

Keys decided but not claimed as class portions (their class route stays with a
handoff): `DEL-04-04:SOW#CLM-010/DEL-04-04-REQ-08` (T7-C06, C2);
`DEL-11-04:SOW#CLM-011/R-DEL-11-04-002` (T6-C03, C4); 11 DEL-17-07/08 rows in
T7-C09, T6-C05 and T6-C01 (C5). C7 also covers 132 T8 rows by cluster filter;
C3 covers the three T9 `STALE_VV:VNV-*` items; C2 covers the T9 item
`STALE_VV:SRE-2_protected_content_review_overtaken` (same 7 rows as the
T5B-C02 INVARIANT part).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
