# Package Concordance Summary — PKG-17 (Export Format Interoperability)

> **Epistemic status: derivative, agent-authored, non-operative evidence**
> (R2 wave W5, run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from
> nine corrected W5 ledgers after independent high-effort fan-in and owning-
> pilot corrections. Accepted upstream evidence snapshot: frozen `main` at
> `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. This summary is not
> decomposition truth and makes no owner, engineering, lifecycle,
> professional, security, or release ruling.

## Census

Nine `IN_PROGRESS` deliverables cover export-source basis, common package and
profile contracts, native JSON, CAEPIPE MBF, external-run/CSV handling,
stress-neutral exchange, PCF, glTF/GLB, and an adapter SDK. The corrected
ledgers contain **375 claim rows** (30/63/28/39/37/35/57/49/37). Nine distinct
GPT-5 deliverable-pilot assignments ran in a rolling capacity-bounded schedule
with no more than three pilots active at once. Independent highest-capability
high-effort verification is recorded in `WAVES/W5/W5_VERIFICATION_PKG-17.md`:
pre-correction 5 SOUND / 4 DEFECTIVE and 153 PASS / 0 QUALIFIED / 8 FAIL
across 161 scoped rows. Four owning-pilot correction clusters closed at all
nine SOUND and 161 PASS / 0 QUALIFIED / 0 FAIL.

## ClaimType × Disposition (computed from corrected ledgers)

| ClaimType | total | ACCEPTED_DIVERGENCE | ALIGNED | PARTIALLY_IMPLEMENTED | REMAINING_STATE_MISMATCH | STALE_REVIEW_OR_EVIDENCE | STALE_SETUP_SPECIFICATION | VERIFIED_NOT_VALIDATED |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| REQUIREMENT | 163 | 0 | 150 | 13 | 0 | 0 | 0 | 0 |
| ACCEPTANCE | 102 | 0 | 87 | 13 | 0 | 0 | 1 | 1 |
| EXCLUSION | 35 | 0 | 31 | 0 | 0 | 0 | 4 | 0 |
| DECLARED_STATE | 54 | 0 | 31 | 0 | 0 | 0 | 23 | 0 |
| REMAINING_WORK | 21 | 3 | 13 | 0 | 4 | 1 | 0 | 0 |
| **Package total** | **375** | **3** | **312** | **26** | **4** | **1** | **28** | **1** |

All 375 rows are mechanically `SelectableUnderCurrentLoop=NO`. Review/TBD
rows are evidence observations or formally gated/permitted states rather than
new selectable work. There are no IMPLEMENTED_UNMAPPED, UNKNOWN,
AUTHORITY_CONFLICT, or lifecycle-reassessment rows.

## Package reading

PKG-17 has broad implemented contract and focused exporter/fixture/test
coverage, but 312/375 ALIGNED is not a maturity score. The 26 partial rows
concentrate in full RFC-8785/JCS fidelity, privacy/redaction defaults,
coordinate/loss-report certainty, conditional assumptions and reproducibility
references, full adapter/family coverage, provenance/manifest completeness,
identifier species, timestamp/generator policy, diagnostic completeness,
service seams, and GLB/visual validation. The CAEPIPE external-run acceptance
has no live external-prover validation, producing one
VERIFIED_NOT_VALIDATED row.

Twenty-eight setup/review surfaces are stale, including one overtaken validator-
path review finding. Four remaining-state mismatches preserve open/unhomed
findings; three ACCEPTED_DIVERGENCE rows retain named human-permitted
nonblocking findings. SourceReliability is UNVERIFIED 321 / NOT_APPLICABLE
54; Confidence is HIGH 333 / MEDIUM 42.

## Decision findings (routing, not rulings)

AuthorityNeeded is **NO 299 / OWNER 66 / ENGINEERING 10**. OWNER routes
format/profile/interface choices, currentness/homing, privacy policy, and
named human dispositions. ENGINEERING routes coordinate, tolerance, mechanics,
or validation questions. These are agent routing judgments, never owner or
engineering rulings. All 28 SECURITY rows use zero convention-6 markers;
bounded negative/security-source checks do not assert sufficiency.

## Verification and correction record

Pilots and verifier exercised contract schemas, fixtures, focused exporters,
validators, dependency mirrors, semantic/lens checks, and boundary surfaces
under addendum-9 controls. Corrections were limited to:

1. DEL-17-01 DECL-004 now records the non-executable mixed-cwd Procedure path
   basis as STALE_SETUP_SPECIFICATION/OWNER.
2. DEL-17-04 DECL-005 restores the byte-exact frozen bootstrap string.
3. DEL-17-05 excludes bootstrap metadata, treats two review findings as
   unhomed rather than status items, and normalizes two ValidationEvidence
   prefixes to `NONE_FOUND —`.
4. DEL-17-08 excludes bootstrap metadata and treats its review finding as
   unhomed rather than a status item.

Only the first cluster changed a disposition/authority histogram; all other
corrections were exact metadata or controlled-prefix repairs.

## Cross-ledger risks carried forward (R3)

1. **Common contract versus exporters:** DEL-17-02 owns common package/profile
   contracts; DEL-17-03..09 are consumers/implementers. R3 must not treat
   shared schemas or validators as duplicate ownership.
2. **Source versus implementation authority:** DEL-17-01 owns the external-
   source basis and open questions, not exporter implementation or vendor
   validation authority.
3. **Canonicalization:** JCS-compatible labels without RFC-8785 vectors recur
   across PKG-08, PKG-14, PKG-15, and PKG-17; deduplicate as one fidelity
   species by owning surface.
4. **Repeated review/TBD findings:** RF-001/RF-002 and format questions recur
   across profiles; aggregate the underlying disposition/currentness issue,
   not every consumer row.
5. **Validation boundary:** schema/fixture and deterministic-export tests do
   not establish live CAEPIPE execution, commercial-tool suitability,
   engineering coordinate correctness, or professional approval.
6. **Privacy/provenance breadth:** metadata fields and source-safe fixtures do
   not establish every runtime redaction, proprietary-source, manifest, or
   embedded-payload path.
7. **Format scope:** PCF and GLB/glTF are conservative/review-oriented subsets;
   their ALIGNED contract rows do not imply comprehensive format coverage.
8. **Displayed command paths:** successful corrected invocations do not make
   mixed-cwd Procedure commands executable; currentness is judged at the
   document surface.

## Fences

Frozen HEAD remained `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Ignored-aware porcelain before/after contained exactly the six addendum-9
allow-listed paths and no seventh path. Python used
`PYTHONDONTWRITEBYTECODE=1`, external pycache, and pytest
`-p no:cacheprovider`; no in-tree `py_compile` or lockless Cargo run occurred.
No product, deliverable, lifecycle, DAG, register, dependency, R4, or R5
change is authorized or performed by this derivative summary.
