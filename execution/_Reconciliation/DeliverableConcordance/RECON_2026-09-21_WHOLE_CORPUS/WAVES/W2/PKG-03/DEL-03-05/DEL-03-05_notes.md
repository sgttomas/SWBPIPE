# DEL-03-05 — worker notes (W2, PKG-03, group G2)

Rigid component models for valves, flanges, reducers and specialty items.
Forward ledger: 72 rows (59 required keys; the optional `.rNN` block CLM-010
split into all 11 rows R01–R11; 2 `.sNN` sub-claims). Frozen state
`00115c719`. Agent judgments only; nothing here is an owner ruling.

## Path aliases

- Project-root tokens resolve under `projects/chirality-piping/`; project
  documents are cited as `projects/chirality-piping/…`.
- SOW-STAGE2 parity/claim-map records are repository-root AgentRuns paths.
- Tests are cited as `<file>::<test name>`.
- Deliverable-local records sit under a folder name with spaces and commas;
  they are cited in `ContextRefs` and described in Notes (Part D forbids
  spaces in evidence columns).
- Suite-level pass status: `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`
  (not rerun; A6).

## Judgment calls

1. **CLM-010 split.** R01–R11 take different dispositions, so the block is
   split into all its `.rNN` rows and the block itself is `CONTAINER`.
2. **Product-path slice absent from the SOW (FG-DEL-03-05-01).** The
   2026-06-21 TP-R4-D3-RIGIDVIS-001 tranche (DEC-045) landed rigid inputs,
   unit checks and diagnostics in product physics, rigid-family creation in the
   applier, and invented preview `component:C-130`. SOW SURFACE, CLM-003 (its
   public-data row says public artifacts carry "invented diagnostics only") and
   CLM-009 (scope limited to schema evidence) are `STALE_REVIEW_OR_EVIDENCE ·
   DOC_BEHIND_CODE`. Rigid components remain `mechanics_geometry_only`, so the
   solver-treatment TBDs are accurate.
3. **CLM-004 → STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING ·
   LOCAL_DESIGN.** Its ban on invented weights/COGs in public artifacts is
   overtaken by DEC-045 ("public examples remain schema-shape-only or
   invented"), and C-130 now carries an invented weight and COG. F8: the IP/data
   boundary (no proprietary, vendor or protected values) holds, so the tier is
   the gap's.
4. **R10 → DOCUMENTED_UNIMPLEMENTED · NOT_STARTED · LOCAL_DESIGN ·
   VALIDATION · ENGINEERING.** No COG convention exists. The code captures a COG
   vector at creation but does not consume it in mechanics, and the applier
   keeps COG edits unsupported pending a payload ruling, so the hold is not
   settled in code (not CP-10). Remaining R01 points `OPEN_ACTION` here (F2).
5. **R07 → PARTIALLY_IMPLEMENTED · PROJECT_BASELINE · BASELINE · REVIEW**: the
   named architecture review of adapter/plugin no-bypass was not located; basis
   is AB-00-02/07 only (compare DEL-03-04 RQ-006, whose basis names OPS-K-IP-3).
6. **R08 → ALIGNED (MEDIUM).** Project persistence is deterministic, versioned
   and delegates to `model.schema.yaml`, whose Component definition governs
   component records; round-trip tests are envelope-level (recorded with a
   `GAP_WORDING_CHECKED` clause).
7. **R11 → ALIGNED.** The provenance validator named in the hook exists as the
   shared library-import checker (DEL-03-07).
8. **CLM-013 → UNKNOWN (INVARIANT, IP_DATA;RECORD, REVIEW)**, following the W1
   resolution for DEL-07-02 CLM-026: it lists a protected-content review of
   fixtures and public examples and adapter/API no-bypass checks as current
   signals, and neither was located.
9. **CLM-014 → UNKNOWN (LOCAL_DESIGN):** reducer/flange/valve fixture
   provenance notes not located.
10. **CLM-016 → CP-02.** Its "REQ-008" pointer does not resolve in this SOW
    (R08 is persistence); the evidence itself is true.
11. **CONTEXT#anticipated-artifacts → PARTIALLY_IMPLEMENTED:** no reducer or
    flange fixture record exists (one rigid record, one preview valve). Context
    Envelope note states a constraint ("no data tables") → DECLARED_STATE,
    `ALIGNED`.
12. **Parity:** a PASS record matches the frozen SOW, so matrix OUT-001 and
    VER-001 are `ALIGNED` (CP-09).
13. **STATUS:** Last Updated equals the latest history date; CP-05 does not
    apply.
14. **Origins (F3):** CLM-003 public-data row, CLM-009, CLM-020 and CLM-021
    from 1b62eb5b8 → `STALE_REVIEW_OR_EVIDENCE`; CLM-004 and CLM-022 from
    7bee9ae41 → `STALE_SETUP_SPECIFICATION`; CLM-016 from 0aad0068e.

## Canonical departures

None. CS rows inherited unchanged.

## Convention friction

- The no-spaces rule keeps `Review_Findings.csv` and run records out of
  evidence columns; CLM-011 and CLM-030 are REVIEWED on the Gate C human ruling
  recorded in MEMORY (cited in `ContextRefs`).
- No layer fits a mechanics-convention gap; R10 uses VALIDATION as the nearest.
- **Custody note for the verifier:** my scratch builder in this folder
  disappeared at about 16:57 local time, after this ledger was sealed, without
  my action. The sealed file was re-hashed afterwards and matches the seal
  (`b231806a…`).

## Smallest checks for UNKNOWN rows

- `SOW#CLM-013`: search review records for a protected-content review of the
  public component fixtures and preview `component:C-130`, and for an
  adapter/API bypass acceptance check on component data; if none exists, run
  them.
- `SOW#CLM-014`: search the deliverable folder and fixture sidecars for
  reducer/flange/valve fixture provenance notes; if absent, author them or drop
  the item.

## Reverse pass

376 capabilities answered: PARTIAL 3 (RC-03-0073, RC-03-0109, RC-03-0294),
COVERS 7 (RC-03-0072, RC-03-0107, RC-03-0167, RC-03-0321, plus RC-03-0259
persistence envelopes and RC-03-0181 model schema for R08, and RC-03-0373 the
transform for the CLM-016 scalar-path evidence), NOT_MINE 366, each
path-overlapping NOT_MINE with a specific reason (F5). The reverse pass did not
change my view of any sealed row.

## Batch consistency

`--batch` over DEL-03-04, DEL-03-05 and DEL-03-06: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
