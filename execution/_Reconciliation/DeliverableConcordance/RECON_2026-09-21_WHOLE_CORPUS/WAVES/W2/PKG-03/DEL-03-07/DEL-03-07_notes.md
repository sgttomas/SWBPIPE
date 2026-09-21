# DEL-03-07 notes — W2 PKG-03, rerun 1 (worker G1)

Rerun after the package verifier returned `RERUN DEL-03-07`
(`PKG-03_VERIFICATION.md`, SHA-256 `f08a7f0d…bbd`). Before anything was
written, the first run's four files were moved unchanged into
`superseded_1/`. This run did not read them. Both passes were encoded fresh.
The verifier's DEL-03-07 findings were read only after the forward ledger was
sealed.

- Forward: 97 body rows, covering 53 required keys, all 37 `.rNN` keys of the
  six split blocks, and 7 `.sNN` sub-claims.
- Sealed at SHA-256
  `a549f6f896da0ee38225cf9f836d8729fad21075a9faad50515e9eeb6cdf53e0`.
- Reverse: 376 routed capabilities.

## Path aliases

- Deliverable-folder files (`ScopeOfWork.md`, `_STATUS.md`, `_CONTEXT.md`,
  `MEMORY.md`, `Review_Findings.csv`) have paths with spaces. Part D does not
  allow spaces in evidence columns, so these files are cited only in
  `ContextRefs` and named in Notes.
- The parity and claim-map records are cited at the repository root:
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…/manager-validation/DEL-03-07/`.
  There is no project copy.
- "Rust port" means
  `core/library_import/library_import_document/src/lib.rs`, crate
  `open_pipe_stress_library_import_document`.
- "Store gate" means `save_local_library` in
  `apps/desktop/src-tauri/src/lib.rs`, together with its test
  `local_library_store_gate_admits_accepted_and_refuses_quarantined_imports`.

## Judgment calls

1. **Unit gate and bare numbers (FG-DEL-03-07-01).** In both Python and Rust,
   the unit check runs only on objects that carry `magnitude`.
   - Material, section and component payloads get no schema check on this
     path. Only `hanger` does.
   - So a bare JSON number in a record can reach an accepted outcome with no
     unit finding.
   - Three rows state an absolute unit rule, and I disposed them
     `PARTIALLY_IMPLEMENTED`: CLM-003.r06, CLM-009.r05 and CLM-021.s02. All
     three are tier `INVARIANT` under F8, because the gap touches OPS-K-UNIT-1.
     Layer is `BASELINE`, since C5 has no units layer. Confidence is `MEDIUM`.
   - Two rows describe a method or test evidence that exists, so they stay
     `ALIGNED`: CLM-016 step 6 and CLM-017.r04.
   - I split CLM-021 into `.s01` (bullets 1–5) and `.s02` (bullet 6), because
     the parts take different dispositions (C1).
2. **Review-finding disposition text (FG-DEL-03-07-02).** Three rows still say
   the local review findings await human disposition: CLM-005.s02,
   CLM-012.s02 and CLM-017.r07.
   - Both findings are `ACCEPT_AS_IS`/`RESOLVED` since the Human Gate A ruling
     of 2026-06-05.
   - The text dates from 2026-06-04 (commit `1b62eb5b8`).
   - Disposition: `STALE_REVIEW_OR_EVIDENCE` · `RECORD_DRIFT`.
   - CLM-016 step 9 is conditional ("unless an authorized human review updates
     them"), so it stays `ALIGNED`.
3. **Revision pins (FG-DEL-03-07-03).** CLM-002.r08, CLM-006, CLM-010.r02 and
   CLM-015 pin rev 0.7.
   - Disposition: CP-02, `STALE_REVIEW_OR_EVIDENCE` · `BASIS_POINTER_STALE`.
   - The frontmatter pins commit `69ac259a`, which is rev 0.8. That pin is
     recorded on the SOW SURFACE row.
4. **SOW SURFACE: `STALE_REVIEW_OR_EVIDENCE` · `DOC_BEHIND_CODE`.** The SOW
   body is the verbatim June reconciliation text.
   - It describes only the Python slice. It does not mention:
     - the runtime Rust port;
     - the Tauri command;
     - the DEC-036 store gate;
     - the Libraries panel;
     - the hanger kind.
   - Items that list "reconciled evidence" (CLM-005.s01, CLM-012.s01, CLM-023)
     are true as written and stay `ALIGNED`, with `PRODUCT_CALLER: NONE`,
     because the Python module itself has only test callers.
5. **CLM-026.r01 vocabulary: CP-10.** The conflict table says the disposition
   vocabulary awaits a ruling. But the material schema's `RedistributionStatus`
   and `ReviewStatus` enums, and the checker's gates, have settled one, and no
   ruling covers it. Disposition: `IMPLEMENTED_DIFFERENTLY` ·
   `AUTHORITY_UNCLEAR` · `PROJECT_BASELINE` · `OWNER`.
6. **C2–C4 and the `TBD` attribute rows are `ALIGNED`.** Each accurately
   declares an open ruling, and the code settles none of them. Each carries
   `GAP_WORDING_CHECKED` where the scan required it.
7. **MEMORY SURFACE: `DECLARED_STATE` · `STALE_SETUP_SPECIFICATION` ·
   `DOC_BEHIND_CODE`.** The undated "Open Items" list reads as a current
   declaration. Its "UI/editor presentation … future GUI work" item is
   overtaken by `LibraryManagerPanel` (2026-06-13). The text is first present
   at `7bee9ae41` (F3).
8. **Architecture Basis Injection.** The keyed row keeps CS-04. A `.s01`
   sub-claim records the PKG-00 `SEMANTIC_READY` statement as
   `STALE_REVIEW_OR_EVIDENCE` · `RECORD_DRIFT`, because all eight PKG-00
   deliverables are `IN_PROGRESS` at the freeze.
9. **CLM-004, the solve-required condition.** Disposition: `ALIGNED`,
   confidence `MEDIUM`.
   - The checker adds no values, and says so.
   - CONTRACT names the solver and rule engine as the OPS-K-DATA-2 enforcement
     point.
10. **CLM-008, the GUI exclusion.** Disposition: `ALIGNED`. Ownership of the
    Libraries panel is left to the reverse pass, which answers `COVERS`, not
    ownership.
11. **Rename residue.** The SOW text carries no former name. The former name
    appears in code the SOW does not name as current:
    - the crate identifier `open_pipe_stress_library_import_document`;
    - the command envelope `document_kind`
      `openpipestress.library_import.validation`;
    - the Python module docstring;
    - fixture strings.

    CP-04 records residue where the deliverable names it, so no CP-04 row is
    recorded. MEMORY's mentions are dated history. R3 may still want the crate
    identifier and `document_kind` in the active-identifier cluster.

## Canonical departures

None. The seven CS rows inherit their values. CP-01, CP-02, CP-09 and CP-10 are
applied as written.

## Convention friction

- **No units layer.** C5 has no layer for unit-awareness invariants. I used
  `BASELINE`. The verifier's W5 notes that another group used `VALIDATION`.
- **CauseTag for the unit rows.** I used `POSSIBLE_DEFECT`: code appears to
  contradict a governing unit rule. The verifier's suggested values for the
  first run's rows were `PARTIAL_SLICE`. Both fit C7. I chose to surface the
  item as a candidate code defect. This is a difference, not an error I would
  correct, but R3 should settle one tag for the cluster (see DEL-03-08 RQ-003).
- **Spaces in paths.** Deliverable-folder paths cannot appear in evidence
  columns, which weakens evidence for record-type rows.

## UNKNOWN rows

None.

## Reverse pass

The reverse pass did not change my view of any sealed row.

| Answer | Capabilities |
|---|---|
| `CLAIMED_BY` | RC-03-0072 (Rust port and Tauri command), RC-03-0107 (Python checker), RC-03-0256 (Python README) |
| `UNKEYED` | RC-03-0121 (Rust README) |
| `PARTIAL` | RC-03-0198 (hanger import validation; the hanger extension is routed to DEL-07-09 by DEC-103 item 5) |
| `COVERS` | RC-03-0288 (Libraries panel), RC-03-0162 and RC-03-0109 (fixtures) |

Every capability whose `EntryPoints` hit a path cited in the forward ledger has
its own `NOT_MINE` reason (F5). That includes every other command in
`apps/desktop/src-tauri/src/lib.rs`, plus CONTRACT, the registers and the
material schema.

**After sealing I read the verifier's DEL-03-07 findings.**

- **D1.** My fresh CLM-021 split reaches the same result on principle 6
  (non-aligned, `INVARIANT`, `BASELINE`). Only the cause tag differs, as noted
  above.
- **W8.** I kept CLM-016 `ALIGNED` for the same reason the verifier did not
  re-dispose it.
- **X1.** I ran a post-seal self-check over every `ALIGNED` row whose
  implementation evidence cites the Python module but not the Rust port or
  the Tauri path.
  - Three such rows carry the marker: CLM-005.s01, CLM-012.s01 and CLM-023.
  - Three omit it: CLM-002.r09, CLM-011 and `CONTEXT#context-envelope`. This
    is an F7 field error I would correct by adding `PRODUCT_CALLER: NONE`.
  - Dispositions are unaffected: each claim is about the checker or a record,
    and the Rust port of the same contract has a product caller.

## Batch consistency

`--batch` over this forward file alone: PASS, 0 findings. The package batch
belongs to the manager.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Piping has
selected work through owner-steered work graphs since 2026-09-19.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). These
dispositions are agent judgments, not owner rulings. Nothing here states or
implies release, approval, compliance or certification.
