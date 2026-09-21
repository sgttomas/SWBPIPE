# DEL-02-04 notes — Plugin and extension domain contracts

Worker G2, gate wave W2, PKG-02. Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`.
Forward ledger sealed at SHA-256 `e7880c8c0a399ee77614cad0923f050761a19a9904a713bc81882280a39ca59c`
(142 rows, 93 required keys, 7 canonical; validator with `--notes-gap`: PASS, 0 findings).

## Path aliases

- The deliverable folder path contains spaces and commas
  (`projects/chirality-piping/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts/`).
  Part D forbids spaces in evidence columns, so deliverable-local records (`_STATUS.md`, `_CONTEXT.md`,
  `_REFERENCES.md`, `_DEPENDENCIES.md`, `MEMORY.md`, `_run_records/*`) are cited only in `ContextRefs`
  (full path) and named in Notes. Where a run record is evidence (the 2026-08-20 R6 N1 records reporting
  324 passed and the V16–V34 agent reviews), it appears in `ContextRefs`; `VerificationEvidence` carries the
  test files plus `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`.
- `VER` = `core/adapters/framework/plugin_verification.py`; `SCH` = `schemas/plugin_manifest.schema.yaml`;
  `EDC` = `docs/architecture/extension_domain_contracts.md` (cited with the project prefix).
- Parity: latest record cited is
  `execution/_Coordination/AgentRuns/SOW-PACKAGE-BATCH-ADOPTION-20260714-01/instances/WORKING-BATCH-ADOPTION/members/DEL-02-04/parity-2.md` (root `execution/`).

## Judgment calls

1. **Contract reading of the requirements.** DEL-02-04 is an `API_CONTRACT` whose scope (CLM-011)
   excludes a plugin loader or runtime. Requirements phrased as "plugin/adapter … shall …" are read as
   obligations on the contract. They are ALIGNED where the schema, contract text and the pure verifier
   enforce them, and those rows carry `PRODUCT_CALLER: NONE` (F7). The exception is where the claim names
   behavioural tests of something that does not exist (rule-pack-facing hooks): CLM-014/REQ-13 and
   CLM-015.r03 are PARTIALLY_IMPLEMENTED (CP-11), tier INVARIANT (security).
2. **Public fixture nonconformance (possible defect).** `fixtures/plugin_manifest/invented_manifest_no_bypass.json`
   does not conform to the canonical manifest schema: `manifest_kind` `openpipestress.plugin_manifest`
   vs const `open_pipe_stress_plugin_manifest`; `checksums` is a list where the schema requires an
   object; `plugin_id` and `entrypoint_id` fall outside the schema patterns. The fixture test checks only
   selected fields. This was found by reading both files; no validator was run. It is recorded on
   REQ-11 (PARTIALLY_IMPLEMENTED · POSSIBLE_DEFECT) and on CLM-014.r04 and CLM-015.r01 (FG-DEL-02-04-03).
3. **Landed schema vs setup "TBD" wording (FG-DEL-02-04-01).** Setup text that says field names, layout or
   the candidate-family list are TBD is overtaken by the landed draft schema:
   STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE. REQ-16 conditions resolution on a "human/architecture
   review", and none was found. I chose DOC_BEHIND_CODE over CP-10 because no governing ruling declared
   the hold. R4 may prefer CP-10 (IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR) for REQ-16.
4. **Registry and permission names not treated as settled.** The schema enumerates seven extension
   families and 14 permission tokens. The code labels them as schema categories or concept-level
   (`plugin_boundary.md`: "not a final permission taxonomy"), includes `TBD` enum members, and forces
   `grant_state` to `not_granted`. So OD-02-04-002/003 and REQ-17 are ALIGNED (the approval is still
   open). Only the stale candidate list (CLM-007.r03) is a finding.
5. **Import/export formats (FG-DEL-02-04-02).** At rev 0.12, OI-004 names the PKG-17 format set under
   SCA-004 and export schemas exist. Rows declaring formats TBD are therefore STALE_SETUP_SPECIFICATION ·
   SCOPE_REDIRECTED_BY_RULING. Confidence is MEDIUM because the adapter framework still lists
   `external_format_list` as unresolved for plugin adapters.
6. **REQ-12 (JCS hashing).** ALIGNED as a conditional contract obligation. The schema fixes the JCS basis
   for declared manifest checksums; no product path hashes plugin manifests. The verifier's pinned schema
   fingerprint uses sorted-key compact UTF-8 JSON. That matches JCS output for this ASCII-keyed,
   integer-only content, but it is not a general JCS implementation.
7. **STATUS#remaining/R01.** ALIGNED · NO_OPEN_ACTION. The held runtime, transport, capability and
   permission-persistence choices are separately governed (DEC-012; adapter framework
   REQUIRED_TBD_DECISIONS; PKG-10). They are not a DEL-02-04 action.
8. **Four-document preambles (CLM-001/009/017/025).** CP-01 with STALE_REVIEW_OR_EVIDENCE, not the origin
   class: they are document metadata (status, created date), which F3 keeps in STALE_REVIEW_OR_EVIDENCE.
   CP-01's text offers the origin class for "origin text". This is a disclosed reading, not marked
   `CANONICAL_DEPARTURE` in the rows.
9. **CONTEXT SURFACE row.** Assessed as a whole declared state: STALE_REVIEW_OR_EVIDENCE ·
   BASIS_POINTER_STALE (CP-02), because rev 0.7 pins recur across blocks. The SOW SURFACE row carries
   CP-04. The frontmatter pin gets its own `.s01` sub-claim.

## Canonical departures

None on CS-keyed rows (all seven inherited exactly). CS-04 is split as instructed: `.s01` (PKG-00 state:
all eight PKG-00 `_STATUS.md` files read IN_PROGRESS) and `.s02` (still-TBD list overtaken for
import/export formats).

## Convention friction

- Evidence tokens cannot hold deliverable-local paths (spaces). See Path aliases.
- F7's `PRODUCT_CALLER: NONE` marker. I applied it to requirement and acceptance rows that rely on the
  verifier or schema. I did not apply it to declared-state rows that cite the schema only to confirm a TBD
  or status statement. See "Reverse pass" below for the resulting list.
- Rename residue in code identifiers the SOW does not name (schema `$id` and title, `manifest_kind` const
  `open_pipe_stress_plugin_manifest`, "OpenPipeStress" in `extension_domain_contracts.md` and
  `plugin_boundary.md`). CP-04 records residue on the surfaces that carry it, and these surfaces are not
  DEL-02-04 surfaces, so they are listed here for R3 rather than keyed.

## UNKNOWN rows and smallest checks

None in this ledger.

## Reverse pass: effect on sealed rows (not edited)

- The routing file has **no capability row for `core/adapters/framework/plugin_verification.py`**, which
  is DEL-02-04's main implementation (the verifier with 97 test functions). It is presumably routed to
  another package. R3 should confirm it is attributed to DEL-02-04 and not left unmapped.
- F7 marker gap (an error I would correct): 26 ALIGNED rows whose implementation evidence is the schema
  or verifier lack `PRODUCT_CALLER: NONE`: SOW#CLM-005.r03, r05–r09, r11; CLM-007.r04; CLM-011; CLM-016,
  .r02, .r04; CLM-020; CLM-027; CLM-029; CLM-030; OD-02-04-001/002/003/005; CLM-032;
  STATUS#remaining/R01; CONTEXT#description, #anticipated-artifacts, #context-envelope; MEMORY. None of
  these claims is about app or runtime behaviour, so their dispositions stand.
- No other sealed disposition changed on reverse reading.

## Batch consistency

`--batch` over DEL-02-04 and DEL-02-05 forward ledgers: FAIL, 1 finding, in DEL-02-05
(`STATUS.s01`, CP-04 with tier PROJECT_BASELINE vs the majority LOCAL_DESIGN). The difference is
justified: CP-04 itself prescribes PROJECT_BASELINE for the `.opsproj` identifier. No DEL-02-04 row
was flagged.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work
through owner-steered work graphs, not `## Remaining`.

## Claim fence

These are agent dispositions, not owner rulings, and they state no release, approval, compliance or
certification. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
