# DEL-01-01 calibration notes (R0)

Worker: TASK (Type 2), run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
Forward ledger: sealed, SHA-256
`98afd6467482d17ec4074827d45dbf2e90d3291ed1475ea729ef088351dd4c1f`, 85 rows
(68 issued keys plus 17 `.sNN` sub-claims). Reverse: 96 capabilities, all
`NOT_MINE`. These are calibration evidence, not owner rulings. Standard claim
fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Path aliases

- All evidence paths in the ledger are repository-relative at `00115c719` and
  were read from the frozen checkout. The deliverable folder is spelled out in
  full, including its commas and spaces. No short alias appears in the CSV.
- `NormativeSource` uses `<path> Lnn-mm` (with a space) for the claim's own
  location. Evidence columns use `#Lnn`.
- Paths such as `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/...`
  (the H1 conversion evidence) are at the **root** level, not under
  `projects/chirality-piping/`. The validator's path-existence check does not
  cover the root `execution/` prefix, so these were checked by hand.
- Gate evidence (`GATE_EVIDENCE/PR834_CI`) is cited only in `ContextRefs`,
  because it is in the run folder, not the frozen tree.

## Judgment calls

1. **TBD values overtaken by later rulings.** The SOW (ISSUED 2026-06-03)
   records maintainer roster, quorum, release authority, human project
   authority and signing as TBD. DEC-027 (2026-06-11) ruled the first four.
   DEC-057 and DEC-089 ruled signing. The SOW anticipated this ("TBD until
   recorded"), so I tagged these rows `RECORD_DRIFT`, not
   `SCOPE_REDIRECTED_BY_RULING`. I disposed of the SOW value rows as
   `IMPLEMENTED_DIFFERENTLY` with `AuthorityNeeded=SCOPE_CHANGE`, because
   the deliverable is ISSUED. I disposed of the SOW surface as
   `LIFECYCLE_REASSESSMENT_REQUIRED`. The same finding appears in 8 rows
   (CLM-004.s02, CLM-005.s02, CLM-005.s03, CLM-014, CLM-023.s02, CLM-024.s04,
   CLM-025.s01, C-01-01-002). Legal-review TBD is still accurate (DEC-079).
2. **`governance/MAINTAINERS.md` lags DEC-027.** Roster and release
   authority are still TBD there. L44 says no one may state a final authority
   until the TBD is resolved. Meanwhile `docs/RELEASE_QUALITY_GATES.md` L183
   cites DEC-027. I recorded this as `PARTIALLY_IMPLEMENTED`, INVARIANT tier
   (OPS-K-GOV-2), on REQ-01-01-05, and as PROJECT_BASELINE on AC-01-01-04. The
   candidate fix is a bounded edit to MAINTAINERS.md, not a change to the
   ISSUED text. MAINTAINERS.md also points to decomposition 0.7 and DAG-007
   (CLM-024.s01).
3. **Posture conflict.** CLM-009 says "public/free/open-source posture". PRD
   L24 still says "free and open-source" even though PRD L44 says that framing
   was superseded. SOW-001 and the selected PolyForm Noncommercial license say
   source-available noncommercial. This is `AUTHORITY_CONFLICT` for the owner.
4. **Rename.** The name OpenPipeStress persists in the SOW, `LICENSE.md` and
   `MAINTAINERS.md`. DEC-101 says other governance documents take the new name
   at their next amendment, so I raised no rename finding. It is noted in
   Notes only.
5. **AC-001 "exact issued baseline".** Parity held at conversion
   (RECON-I0-PKG01 PASS). DEC-081 Wave 2 then edited five lines of the ISSUED
   SOW. I marked this `ACCEPTED_DIVERGENCE` on DEC-081, at MEDIUM confidence.
   The profile (layer 4) routes ISSUED changes through scope change, but
   DEC-081 says it is "not an SCA". The owner should confirm whether DEC-081
   reached ISSUED text.
6. **Representation conversion (PR #230, H1-APPROVAL-001)** removed the four
   documents that CLM-018, CLM-019 and CLM-020 still name. I disposed of these
   as `STALE_SETUP_SPECIFICATION` with cause `OTHER`.
7. **Source reliability** is `NOT_APPLICABLE` on every row. No row relies
   on an engineering source. Governance records are declared-state prose.
8. **Selectability (C9)** is `NOT_APPLICABLE` on every row, because work is
   selected by owner-steered work graphs, not `## Remaining`. DEL-01-01 has no
   Remaining section.
9. **BaselineClass** is `ISSUED` on every non-aligned row, because every SOW
   claim is part of the ISSUED baseline. That is uninformative for this
   deliverable (see friction F6).

## Convention friction

- **F1. Section headings versus CLM blocks.** The extractor parents CLM
  blocks to the SOW surface, not to their section heading. The four section
  blocks (`#deliverable-definition-ontology` and others) therefore have no
  keyed children. C1 only allows `CONTAINER` when substance is carried by
  ITEM children. I used `CONTAINER` / `COVERED_BY_CHILDREN` by line-range
  containment. *Fix:* the extractor sets ParentKey from the section heading,
  or C1 names section headings as `NON_NORMATIVE`.
- **F2. No disposition for declared-state metadata drift.** An example is
  `_STATUS.md` Last Updated 2026-06-03 with a 2026-07-16 history line. Another
  is the SOW Identification table saying IN_PROGRESS for an ISSUED
  deliverable. `REMAINING_STATE_MISMATCH` is limited to Remaining. I used
  `STALE_REVIEW_OR_EVIDENCE` and `STALE_SETUP_SPECIFICATION`, and both mislead.
  *Fix:* widen `REMAINING_STATE_MISMATCH` to `DECLARED_STATE_MISMATCH`.
- **F3. Identification tables are not always non-normative.** C1 sends them
  to `NON_NORMATIVE`, but CLM-002 carries lifecycle and basis state. *Fix:*
  say that an identification table carrying status or basis fields is
  `DECLARED_STATE`.
- **F4. No cause tag for representation migration.** The four-document to
  ScopeOfWork change (H1) forced `OTHER` three times. *Fix:* add
  `REPRESENTATION_MIGRATED`.
- **F5. No cause tag for a superseded authority revision.** Decomposition
  0.7 became 0.12 and DAG-006/007 became DAG-010. I used
  `EVIDENCE_OVERTAKEN`, which is about evidence, not basis pointers.
  `CONTRACT_VERSION_ADVANCED` is close but is framed around schemas. *Fix:*
  add `BASIS_REVISION_ADVANCED`, or widen `CONTRACT_VERSION_ADVANCED`.
- **F6. BaselineClass on an ISSUED deliverable.** Every row gets `ISSUED`,
  which hides the distinction between a SOW that is correctly frozen and a
  governance file that lags (MAINTAINERS.md is not ISSUED). *Fix:* let
  BaselineClass name the baseline of the divergent artifact, not only the
  claim's.
- **F7. No verification class for human document review.** Governance
  deliverables are verified by review runs. I put review records in
  `VerificationEvidence` with `VerificationClass=NONE`, which reads as a
  contradiction. *Fix:* add `DOCUMENT_REVIEW`.
- **F8. Validator path coverage.** `PATH_TOKEN_RE` does not check root
  `execution/` paths. *Fix:* add `execution/` and `plans/` prefixes.
- **F9. A later ruling that fills a slot the text anticipated.** This is
  neither "scope redirected" nor plain "record drift". C7 could state which
  applies. I chose `RECORD_DRIFT`.
- **F10. A single cross-cutting finding repeated over many rows.** The
  DEC-027 finding appears 8 times, which inflates counts. *Fix:* add a
  `FindingGroup` column, or let later rows reference the first.

## Unit grain

For the SOW the grain was mostly right. It was too coarse for the table
blocks without numbered items (CLM-004, CLM-005, CLM-009, CLM-023, CLM-024,
CLM-025): each mixed aligned and overtaken content, so I minted 17 sub-claims.
Tables whose rows carry their own IDs (the decision surface, the conflict
table) could be extracted per row. The migration items (OUT-001 twice, AC-001,
VER-001) are useful because they carry the conversion's parity claim. Heading
blocks CLM-001, 008, 015 and 021 add 4 empty rows each time. The `_CONTEXT.md`
per-heading grain is fine. `MEMORY.md` as one unit is right.

## UNKNOWN rows

None.

## Did the reverse pass change my view?

No. All 96 pilot capabilities are solver, security, adapter, model-tree or
workspace product behaviour. DEL-01-01 is a policy-only DOC_UPDATE that
excludes product and GUI behaviour. The nearest touches are the telemetry
controls (CAP-SEC-008 and CAP-SEC-009) and adapter quarantine (CAP-ADAPT-002).
They enforce at runtime the boundaries that DEL-01-01 states as review policy,
but they belong to the security and IP deliverables. No sealed row needs
revisiting. The inventory has no governance-document capabilities, so the
reverse pass could not independently test this deliverable's ownership of
`governance/MAINTAINERS.md`, `LICENSE.md` or `docs/README.md`. A later
inventory should include documentation and governance surfaces.
