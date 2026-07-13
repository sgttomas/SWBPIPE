# R5 Tranche Plan — D-41 Deliverable Concordance

> **Derivative execution control, not independent authority.** This plan
> partitions all 77 rows and 532 affected claims in
> `PROPOSED_DELIVERABLE_UPDATES.csv` under D-41/DEC-074. The verbatim authority
> is `execution/_Coordination/_DECISIONS/D-41_R4_RULING_2026-07-12.md`.
> Deliverable-local `_STATUS.md ## Remaining` items are the work-selection
> surfaces. This artifact neither changes lifecycle state nor supplies a
> validation outcome, review disposition, scope expansion, or ISSUED-baseline
> change.

## Partition proof

- Update IDs assigned: 77 / 77; unique: 77; missing: 0; duplicates: 0.
- Affected claims assigned: 532 / 532.
- Tranche claim totals: 10 + 48 + 39 + 40 + 13 + 43 + 311 + 4 + 24 = 532.
- Exact claim membership remains the semicolon-delimited
  `AffectedClaimIDs` field of each assigned update row.

## Sequenced tranches

| Tranche | PDUs / claims | Scope and principal write targets | Sequencing and validation |
|---|---:|---|---|
| T1 — ownership and attribution | 3 / 10 | PDU-009, PDU-061, PDU-077. D-41/D-42 records; DEL-00-05/DEL-07-02 delegation; DEL-07-05 Remaining; non-conflicted receiving kits/status; shared-infrastructure boundary. | First. Stop SURF-011/021 as `AUTHORITY_CONFLICT`; re-extract attribution/claims for the other six surfaces. |
| T2 — canonical schema, units, mechanics | 19 / 48 | Serializers, canonical units, migration/schema handling, stable IDs, numeric normalization, production-oracle binding, fixtures/tests, owning kit/status/run records. | After T1. Establish E1/E2/E4/E6 before downstream integration. RFC-8785 vectors or narrowed labels; schema tests; numeric witnesses; production-path oracle tests. |
| T3 — privacy, redaction, security reach | 9 / 39 | Privacy/redaction helpers at selected adapter/plugin/CLI/report/export seams; negative/no-bypass tests; DEL-12 and consumer kit/status/run records. | After relevant T2 contracts. O7 precedes E5. No whole-product security claim. Formal review closure stays gated. |
| T4 — application, interop, reporting/export | 11 / 40 | Application-service homes, result envelopes, reporting/export content, JSON glTF/conservative PCF behavior, workflow wiring, benchmark-envelope path. | After T2/T3. Preserve O11 scope; no GLB/broader geometry. Focused conformance, round-trip, fixture, output, and workflow tests. |
| T5 — GUI behavior and validation | 4 / 13 | Existing scoped GUI gaps, telemetry affirmative-action behavior, usability/interaction evidence, corresponding DEL-07/DEL-12 state records. | After T3 policy seams and T4 services. No new GUI scope. Unselected measurable-contrast target remains held. |
| T6 — evidence, review, Remaining homes | 5 / 43 | Fixtures/validation records, originating review surfaces, consumer references, sole Remaining homes. | After technical tranches. Technical repair never closes formal review. Optional live CAEPIPE remains profile-gated. |
| T7 — corpus currentness | 9 / 311 | Specifications, datasheets, guidance/procedures, declarations, stale evidence pointers, `_STATUS.md`, `MEMORY.md`, and `_run_records/**`. | Last operative tranche so prose reflects landed behavior; preserve dated history/residuals; no lifecycle transitions. |
| T8 — governed change paths | 2 / 4 | PDU-051 and PDU-057 only. | HOLD. PDU-051 requires SCOPE_CHANGE; PDU-057 preserves ISSUED DEL-01-01 under O4. |
| T9 — controlling deferrals | 15 / 24 | PDU-062..076. | Backcheck only; no threshold promotion, lifecycle change, signing/publication, broader format scope, or deferred-mechanism activation. |

## Machine-checkable update assignment

```csv
Tranche,UpdateIDs
T1,"PDU-009;PDU-061;PDU-077"
T2,"PDU-002;PDU-003;PDU-011;PDU-013;PDU-014;PDU-015;PDU-023;PDU-024;PDU-025;PDU-029;PDU-030;PDU-031;PDU-032;PDU-033;PDU-034;PDU-035;PDU-044;PDU-047;PDU-048"
T3,"PDU-016;PDU-017;PDU-018;PDU-019;PDU-026;PDU-027;PDU-028;PDU-043;PDU-049"
T4,"PDU-001;PDU-004;PDU-010;PDU-012;PDU-020;PDU-021;PDU-022;PDU-036;PDU-038;PDU-039;PDU-040"
T5,"PDU-008;PDU-041;PDU-042;PDU-045"
T6,"PDU-007;PDU-037;PDU-046;PDU-050;PDU-060"
T7,"PDU-005;PDU-006;PDU-052;PDU-053;PDU-054;PDU-055;PDU-056;PDU-058;PDU-059"
T8,"PDU-051;PDU-057"
T9,"PDU-062;PDU-063;PDU-064;PDU-065;PDU-066;PDU-067;PDU-068;PDU-069;PDU-070;PDU-071;PDU-072;PDU-073;PDU-074;PDU-075;PDU-076"
```

## Authority conflicts and held choices

### D-42 O3 conflicts — do not execute pending owner ruling

1. SURF-011 build-readiness → DEL-09-05 contradicts DEL-09-05's explicit
   no-GUI/checklist-only boundary and the panel's own DEL-10-04 packet
   identity.
2. SURF-021 export-unit disclosure → DEL-17-02 contradicts DEL-17-02's
   contract-only/no-GUI implementation boundary. It may consume DEL-17-02's
   common unit policy without being owned there.

Telemetry → DEL-12-03 and root `package.json` → DEL-10-04 pass accepted-scope
inspection. `core/product_preview` and the three named tools pass the shared
governed-infrastructure classification. Consumer-level conformance obligations
remain intact.

### Other unselected policies/review outcomes

These update rows remain wholly or partly held because DEC-074 did not invent
their exact policy or formal disposition: PDU-001, PDU-004, PDU-007, PDU-015,
PDU-019, PDU-025, PDU-031, PDU-034, PDU-035, PDU-046, PDU-050, PDU-051,
PDU-057, and PDU-060. Technical work may proceed only around the specifically
unselected portion. In particular:

- formal REVIEW sufficiency/disposition is not inferred;
- live CAEPIPE remains behind O10's user-owned profile gates;
- measurable contrast/readability targets are not invented;
- exact taxonomy, alias/parser, timestamp/generator, diagnostic/dimensionless,
  and readiness-policy values are not silently selected;
- scope-change and ISSUED-baseline paths remain human-governed.

## Closeout contract per operative tranche

Each tranche seeds or updates the owning deliverable's `_STATUS.md ##
Remaining`, preserves or appends `MEMORY.md` and `_run_records/**`, runs
focused tests plus applicable DEC-025/harness checks, re-extracts affected
claims, records surviving residuals, and ends in a scoped commit/push/PR
checkpoint. No tranche self-merges.
