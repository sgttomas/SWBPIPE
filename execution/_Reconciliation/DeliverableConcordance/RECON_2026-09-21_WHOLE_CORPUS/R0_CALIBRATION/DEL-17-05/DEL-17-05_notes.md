# DEL-17-05 — R0 calibration notes

Forward ledger sealed at SHA-256
`ed0f14573a8ee6feb17be1a4219cce09cab1659e822ff14eb54761ebb6267e0e`
(113 rows: 104 issued keys plus 9 worker sub-claims). These notes are
calibration evidence about the candidate conventions. They are not findings
accepted by the owner. Standard claim fence applies (F-PIP-2; claims taxonomy
per DEC-081).

## Path aliases

- `NormativeSource` uses short forms relative to the deliverable folder:
  `SOW#Lnn` = `ScopeOfWork.md`, and `_STATUS.md#Lnn`, `_CONTEXT.md#Lnn` and
  `MEMORY.md#Lnn` likewise.
- `RR` in my reasoning = `<deliverable>/_run_records/`. Ledger cells always
  hold the full repository-relative path.
- `GATE_EVIDENCE/...` appears only in `ContextRefs`. It lives in the run
  folder, not in the frozen tree, so the validator's path check would reject it
  in an evidence column.
- Two ledger cells cite repository-root paths (`agents/AGENT_TASK.md`,
  `tools/validation/check_four_documents.sh`). The Piping project has no
  `agents/` folder and no `tools/validation/check_four_documents.sh`.
- `DEC-074` = the D-41 ruling family (PDU-016/050/055). "O10", as used in the
  deliverable, is not defined in any governing record I found. It appears only
  in the earlier concordance run's R5 records. I cite DEC-080 (prover
  activation is gated by the owner) as the governing gate instead.

## Judgment calls

- **The harness has no process launch.** `core/handoff/caepipe_external/run.py`
  has no subprocess or process-launch code. It builds records for skipped,
  parser-only and attempted runs. I read "harness shall be optional/disabled"
  requirements (REQ-001, CLM-006.s01) as met by construction and marked them
  ALIGNED at MEDIUM confidence. I marked the live-run procedure and tests
  PARTIALLY_IMPLEMENTED or DOCUMENTED_UNIMPLEMENTED, under DEFERRED_BY_RULING
  (DEC-080) where the gap is the live run itself.
- **The skip is data, not a test skip.** No test needs an executable, so none
  can skip. Skipping is recorded as a `skipped_no_executable` package. I rated
  REQ-012 and CLM-027 IMPLEMENTED_DIFFERENTLY rather than ALIGNED.
- **Record-shape requirements.** Where every field exists in the schema but
  nothing live fills it (REQ-004), I used ALIGNED. Where fields are missing from
  the closed schema (`additionalProperties: false`) I used
  PARTIALLY_IMPLEMENTED. The missing fields are the MBF profile id, the
  environment context (OS, permissions, launch context), the source model id and
  the coverage-register version.
- **The desktop panel is judged against the core contract.**
  `CaepipeExternalHarnessPanel.tsx` builds its own packet in TypeScript with
  invented values keyed to the user's model IDs. Its packet includes fields
  outside the schema, and it labels its privacy as `invented_public_example`.
  I did not treat that label as a breach of REQ-014 or CLM-018, because those
  rows are not user CAEPIPE output. I noted it on REQ-014 and REQ-016 (the panel
  makes up an MBF reference that passes the substring check).
- **CLM-019 (PDU-050 hold).** The declaration is accurate. I recorded the state
  underneath it (VERIFIED_NOT_VALIDATED, VALIDATION_GAP) rather than ALIGNED.
  See friction F4.
- **CLM-003 (Identification table).** I kept it normative (DECLARED_STATE,
  STALE_SETUP_SPECIFICATION) because its "Current phase" field is false at the
  freeze. C1 would have classed it NON_NORMATIVE.
- **CLM-031 is not a container.** Its own text (the PDU-016 check) differs from
  its child `VER-001` (a broad review list), so I assessed both.
- **Sub-claims.** I split CLM-006 into `.s01`–`.s06` because its table rows
  have mixed dispositions. I added CLM-007 `.s01`–`.s03` because only three of
  its reference rows are issued items. I assessed CLM-004, CLM-028 and CLM-035
  whole, with the gaps listed in `Notes`; splitting would have added about 35
  rows.
- **Status history.** C1 names only MEMORY as a history unit. I applied the same
  treatment to `STATUS#history`.
- **Selectability.** `SelectableUnderCurrentLoop = NOT_APPLICABLE` on every
  row (C9). Since 2026-09-19 Piping selects work through work graphs steered by
  the owner, not through `## Remaining`.

## Convention friction

1. **Section blocks overlap CLM blocks.** The four section-heading keys
   (e.g. `#deliverable-definition-ontology`, L18-121) span line ranges that are
   also issued as CLM keys whose parent is the surface. They are neither true
   containers nor plain headings. *Fix:* the extractor should issue section
   headings as NON_NORMATIVE by rule, or make the CLM keys their children.
2. **Partial itemisation.** CLM-007 has issued items for only some of its
   table rows, so it is not "fully carried by children". Yet C1 only allows
   `.sNN` sub-claims "where no numbered items" exist. *Fix:* allow `.sNN` for
   the parts of a partly itemised block that no item covers.
3. **No cause tag for a change of representation.** The four documents were
   replaced by `ScopeOfWork.md` (PR #235). This is neither code advancing nor a
   ruling. I used OTHER once (CLM-016/VER-001) and DOC_BEHIND_CODE elsewhere,
   which is inconsistent. *Fix:* add `REPRESENTATION_MIGRATED`, or state that
   DOC_BEHIND_CODE covers it.
4. **Accurate declarations of an unvalidated state.** C6 does not say whether a
   true "held as VERIFIED_NOT_VALIDATED" declaration gets ALIGNED (the text is
   right) or VERIFIED_NOT_VALIDATED (the state underneath). The same question
   comes up for accurate Remaining items (R01, R02: I used ALIGNED as A4
   directs). *Fix:* one sentence stating that the disposition describes the
   claim's subject, and that accuracy of a declaration goes in `Notes`. Or the
   reverse, but decide it.
5. **Blanket currency overrides.** The PDU-055 declarations (CLM-002/010/021/033)
   say the implemented slice "supersedes" setup wording without naming what
   changed. Read literally, they would make every stale row ALIGNED. I did not
   read them that way. *Fix:* state that a blanket supersession declaration is
   itself a DECLARED_STATE row and does not change sibling dispositions.
6. **Gate evidence has no evidence-column home.** A6 says to cite suite-level
   pass status from `GATE_EVIDENCE/`, but those paths fail the frozen-tree path
   check. *Fix:* allow a `GATE:` token prefix in `VerificationEvidence`, or name
   `ContextRefs` as the place for it.
7. **One VerificationClass per row.** Rows covered by both pytest and
   Playwright or Vitest lose information. *Fix:* allow a semicolon list.
8. **Deferred-by-ruling combined with partial work.** C7 asks for a single
   cause. Several rows are partly implemented, with the remainder deferred by
   DEC-080 (CLM-026, CONTEXT#description). *Fix:* allow a secondary cause, or
   state that the cause of the *remaining* gap wins.
9. **BaselineClass for owner gates.** I used `OWNER_HOLD` for the DEC-080
   activation gate. C4 does not say whether a ruled gate that is not a hold on
   this deliverable counts as OWNER_HOLD or RULED_CRITERION.
10. **Duplicate item keys.** `VER-001` appears under both CLM-016 and CLM-031
    with different content, and the output matrix cites "VER-001" ambiguously.
    The keys are unique because they are qualified by parent, but the SOW's own
    cross-references are not.

## Unit-grain observations

- CLM-level grain worked well for requirement tables (REQ, PH, VER items) and
  for CONTEXT and STATUS.
- It was too coarse for the "Construction", "Run-record field list" and
  "Principles" blocks, where one block mixes met and unmet sub-claims.
- It was too fine for the four PDU-055 currency declarations (identical text
  four times) and the four currency heading markers (CLM-001/009/020/032).
  Collapsing each set would drop 6 near-duplicate rows.
- About 17% of rows were structural (containers and headings).

## Smallest next check per UNKNOWN row

- `SOW#CLM-007.s02` (public CAEPIPE pages): check whether DEL-17-01
  `Source_Basis_Register.md` records retrieval dates or hashes for the four
  URLs. No network fetch was permitted.
- `SOW#output-and-evaluation-matrix/OUT-001`: read the SOW migration mapping
  rule in PR #235. Find out why CLM-009 (a heading) is the requirement
  reference and which VER-001 is meant, and whether any DEL-17-05 claim map or
  parity report exists.
- `CONTEXT#architecture-basis-injection`: check DEC-009 and `SOFTWARE_DECOMP.md`
  §1 for whether the Python `core/handoff` layer is an admitted home for export
  evidence builders, given the basis names a Rust core.

## Effect of the reverse pass

All 96 pilot capabilities are NOT_MINE. None is in the export or handoff area.
The closest links are CAP-SEC-003/004/005: DEL-17-05's writer consumes
`control_route_export` on route `REXC-CORE-007`. That consumption is not
recorded in any DEL-17-05 claim. The sealed forward rows (e.g. REQ-014) cite
the writer, not the control. This is a dependency without a claim, not an
ownership gap. It did not change any sealed disposition. The pilot inventory
contains no capability for `core/handoff/caepipe_external/` or the desktop
CAEPIPE panel, so the reverse pass could not test whether any row I claimed
would come back as someone else's capability. That coverage gap is worth
checking in the inventory design.
