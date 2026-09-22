# C1 — Code contradicts the SOW: rule on intent before any fix

Packet writer: TASK P3, R3 integration, run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
This is a proposal. It repairs, rules and changes nothing.

`RUN` = `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.
`F:` = the freeze `projects/chirality-piping/` at `00115c71931bcae79909602d653740d3bb72dfa1`.

## 1. Decision

In eight subjects the frozen code does something an explicit SOW requirement
forbids or does not contemplate. For each subject, the owner rules whether the
code's behaviour is intended (the text catches up) or not (the code catches up).
A text catch-up must not run before the ruling, because it would silently
ratify unruled product behaviour.

**Holder: OWNER.** Subject 2 has an engineering part (the boundary-mapping
reading, T11 SS-01), which H3 carries. The DEL-08-04 rows are AuthorityNeeded
REVIEW. They share their substance with DEL-08-03 (OWNER), so this packet takes
all of them together.

The eight subjects:

| # | Subject | Deliverables | Rows |
|---|---|---|---|
| 1 | Export redaction since PR #307 (checksum, units, IDs, refs) | DEL-15-03 | 19 |
| 2 | Diagnostic class and remediation derived by adapters; `affected_object` replaced | DEL-08-03, DEL-08-04, DEL-00-06 | 9 |
| 3 | Bare numbers accepted at the import gate | DEL-03-07 | 3 |
| 4 | Desktop preview design-knowledge data not schema-shaped or validated | DEL-13-01 | 1 |
| 5 | Fixture-local tolerance loosened against the DEC-026 seed | DEL-09-01 | 3 |
| 6 | Runtime intent "schema validation" without JSON Schema | DEL-16-02 | 1 |
| 7 | Eleven-class symbolic path vocabulary against the SOW's six | DEL-12-01 | 4 |
| 8 | One result-row table against per-entity tables | DEL-17-06 | 1 |

## 2. Background

**Prior rulings and governing text.**
- OPS-K-UNIT-1 (`F:docs/CONTRACT.md:33`) requires imported values and exports
  to be unit-aware; OPS-K-DATA-2 forbids invented data. Both are cited as the
  normative source on the DEL-15-03 FG-02 rows.
- AB-00-06 (diagnostics) and SPEC §7–§8 govern diagnostic fields. DEL-00-06
  REQ-06-02 says every field except class is kept across crossings.
- DEC-026 (`F:execution/_Decomposition/SOFTWARE_DECOMP.md:617`; register row
  D-04, `_DECISIONS/_REGISTER.md:31`) makes fixture overrides tighten-only and
  treats loosening as a governance event.
- DEC-020 (`SOFTWARE_DECOMP.md:611`) is the operation-seam ruling behind
  DEL-16-02's runtime.
- The DEL-12-01 SOW step 1 keeps its six path classes "unless a human-approved
  decision replaces them" (T5A-C05 decision 4). No such decision was located.
- No ruling located adopts any of the eight behaviours. A merged PR carries
  owner intent (CONVENTIONS A2), but no ledger reads PR #307 as a ruling on the
  SOW's preserve requirements; the rows route the question to R4.

**What the code does (freeze, read-only).**
1. `F:core/handoff/exporter/workflow.py:129-133` routes the export payload
   through `control_route_export` with `route_id="REXC-CORE-001"` and
   `export_context="downstream_tool"`. Ledger Notes record the resulting
   `[REDACTED]` values on invented fixtures (DEL-15-03 FG-DEL-15-03-02).
   T12-C01 records that no product path calls this exporter.
2. `F:apps/desktop/src/features/report/reportPackageRequest.ts:61`
   (`diagnosticClass(severity)`), `F:apps/desktop/src/features/report/renderableReportInput.ts:91`
   (`diagnosticsForSections`) and `F:apps/desktop/src/features/result-export/resultExportAdapter.ts:138`
   (`buildCurrentResultExport`). `F:core/runner/headless/src/result_envelope_binding.rs:168`
   sets `affected_object` to a reference to the diagnostic itself.
3. The import unit gate inspects only `magnitude`-shaped values (DEL-03-07
   FG-DEL-03-07-01 Notes).
4. The desktop Knowledge path reads an invented preview file not in schema shape
   (DEL-13-01 REQ-13-01-011 Notes; FIELD correction points the verification
   evidence at `apps/desktop/src/services/previewService.test.ts`).
5. `F:validation/benchmarks/mechanics/src/lib.rs:3114`, `:3117`, `:3122` set
   expansion-loop relative tolerances of 5.0e-7; the DEC-026 analytic seed is
   1e-9.
6. `F:core/model_operations/operation_applier/src/lib.rs:2072`
   `check_intent_structure` checks required keys. The Python preview engine uses
   a JSON Schema validator (`F:core/model_operations/validation_preview/engine.py:192`).
7. `F:core/security/local_first_storage/controls.py:51-55` lists the policy's
   path classes (eleven in all, per ledger Notes).
8. The DEL-17-06 package ships one result-row table (nine 0.2 members), per
   ledger Notes and RemainingWork.

## 3. Options

Each subject is ruled separately. Options are as they stand in the evidence.

**Subject 1 — Export redaction (DEL-15-03).**
- (a) Redaction of invented or public handoff content is intended. Amend the
  SOW (REQ-002, REQ-004, REQ-005, AC-001 and the checksum-carry rows
  FG-DEL-15-03-03) through the change path. The no-recompute and no-JCS-claim
  parts hold either way.
- (b) Not intended. Fix the export route classification so invented/public
  content is preserved; restore label carry for checksums.
- Consequences: (a) authorises SOW revision and R5 record repair of 19 rows;
  (b) authorises a code-fix brief on the exporter route and then re-verification.
  Either way B7 matters: if B7 retires or ports the Python exporter, the code
  target of (b) moves.

**Subject 2 — Diagnostic class and remediation (DEL-08-03, DEL-08-04, DEL-00-06).**
- (a) Producer values must be carried through, with TBD where the producer has
  none, and `affected_refs` must reach `affected_object`. Code fix in the desktop
  adapters and the runner binding.
- (b) Adapter derivation is accepted; refine REQ-06-02 and the DEL-08-03/04
  requirements. R5 record repair.
- Consequences: (a) touches three desktop files and one runner file; (b) touches
  AB-00-06-derived requirement text in three deliverables. T11 SS-01 found five
  ALIGNED rows on the same surfaces (DEL-10-05, DEL-07-07, DEL-07-08); under (a)
  those rows need re-verification.

**Subject 3 — Bare numbers at the import gate (DEL-03-07).**
- (a) The gate must reject bare numeric values, or schema-validate material,
  section and component records. Code fix; engineering names the legitimately
  dimensionless fields (T8-K3).
- (b) Bare numbers stay accepted for named record kinds; restate the SOW rows.
- Consequences: (a) closes an INVARIANT gap (OPS-K-UNIT-1); (b) needs an owner
  basis for an exception to OPS-K-UNIT-1, which is a contract-level change.

**Subject 4 — Preview design-knowledge path (DEL-13-01 REQ-13-01-011).**
- (a) The preview path must carry schema-valid records validated against the
  design-knowledge schema. Code fix.
- (b) Accept a preview-only exception, recorded in the SOW.
- Consequence: depends on B9 (PKG-13 product status). If B9 retires the Knowledge
  surface, this subject lapses.

**Subject 5 — Tolerance (DEL-09-01).**
- (a) Adopt 5.0e-7 into the governed DEC-026 record with its recorded reason.
  This is the governance event DEC-026 requires.
- (b) Require the witness configuration that meets 1e-9; tighten the constants.
- Consequences: (a) is a DEC-026 record amendment and R5 repair of three rows;
  (b) is a code and fixture change. Neither asserts engineering adequacy.

**Subject 6 — Runtime schema validation (DEL-16-02 REQ-16-02-002).**
- (a) Validate runtime intents against the governing JSON Schema.
- (b) Restate the requirement for the intent-structure contract.
- Consequence: must be ruled with B3 (runtime edit contract, FG-DEL-16-01-01);
  a split answer would leave two contracts.

**Subject 7 — Path-class vocabulary (DEL-12-01).**
- (a) Ratify the policy's eleven-class vocabulary into the SOW.
- (b) Record why the SOW names differ, keeping six in the SOW.
- (c) Restore the six classes in code (in evidence only as the SOW's default;
  no ledger proposes it).
- The intent (symbolic, private by default) holds under every option.

**Subject 8 — Member inventory (DEL-17-06).**
- (a) Record the implemented single result-row table.
- (b) State that per-entity tables are still wanted (product scope; implementation
  work follows).

## 4. Evidence and reliability

| Source | Reliability | What it shows |
|---|---|---|
| Sealed forward ledgers, W3 PKG-15 (DEL-15-03), PKG-08, PKG-09, PKG-13, PKG-16 (W1), PKG-12, PKG-17; W2 PKG-00, PKG-03 | Worker judgment, verifier-sampled | Dispositions, Notes, RemainingWork quoted above |
| `RUN/R3/TASKS/T7_CLASSES.md` T7-C05; `T5A_CLASSES.md` T5A-C05 | R3 task proposal | Class grouping and options |
| `RUN/R3/TASKS/T8_CLUSTERS.md` T8-K3 | R3 task proposal | Unit-vocabulary reading; DEL-15-03 split out |
| `RUN/R3/TASKS/T11_METHOD.md` SS-01, R-03 | R3 task proposal | Diagnostic-field tension across 5+ rows |
| `RUN/WAVES/W3/W3_ASSESSMENT.md:68`, `:77`; `W2/W2_GATE_ASSESSMENT.md` code-change candidates | Agent 0 assessment | Owner items |
| Freeze code at the lines cited in §2 | Code reading (evidence, not authority, A1) | Behaviour; not executed |
| DEC-026, DEC-020, OPS-K-UNIT-1 | Ruled / contract text | Governing basis |

Verified by this writer at the freeze: the code lines in §2 items 1, 2, 5, 6
and 7. Known only from worker notes: the `[REDACTED]` output values, the
eleven-class count, the nine-member inventory, the bare-number acceptance path,
and the preview-file shape. No build or test was run by any R2/R3 agent.

## 5. Affected claims

**Classes and portions.**

| Class | Class rows | This packet's portion | Filter |
|---|---|---|---|
| T7-C05 | 27 | 27 (whole class) | `CLASS_ASSIGNMENTS.csv` `ClassID == T7-C05` |
| T5A-C05 (split with B6) | 17 | 14 | `T5A_CLASSES.csv` `ClassID == T5A-C05` and `DeliverableID != DEL-17-04` |

T5A-C05 portion keys (14): `DEL-16-02:SOW#CLM-010/REQ-16-02-002`;
`DEL-12-01:SOW#CLM-005`; `DEL-12-01:SOW#CLM-019`; `DEL-12-01:SOW#CLM-027`;
`DEL-12-01:SOW#CLM-032`; `DEL-15-03:SOW#CLM-006.r05`; `DEL-15-03:SOW#CLM-008`;
`DEL-15-03:SOW#CLM-011/DEL-15-03-REQ-008`; `DEL-15-03:SOW#CLM-012`;
`DEL-15-03:SOW#CLM-015`; `DEL-15-03:SOW#CLM-022`; `DEL-15-03:SOW#CLM-031`;
`DEL-15-03:CONTEXT#architecture-basis-injection.s03`; `DEL-17-06:SOW#CLM-006`.
The three `DEL-17-04` rows belong to B6.

**Rows by subject (41).**
- S1 DEL-15-03 (19): T7-C05 11 — `SOW#CLM-005`, `CLM-006.r04`,
  `CLM-011/DEL-15-03-REQ-002`, `CLM-011/DEL-15-03-REQ-004`,
  `CLM-011/DEL-15-03-REQ-005`, `CLM-013.r02`, `CLM-013.r03`, `CLM-019`,
  `CLM-020.r03`, `CLM-025`, `completion-and-reliance-basis-epistemology/AC-001`;
  T5A-C05 8 as listed above.
- S2 (9): `DEL-08-03:SOW#CLM-006`, `CLM-006.r02`, `CLM-012/DEL-08-03-REQ-002`,
  `CLM-020`, `CLM-026.r03`, `CLM-027`; `DEL-08-04:SOW#CLM-011.r05`,
  `CLM-025.r04`; `DEL-00-06:AB#normative-requirements/REQ-06-02`.
- S3 (3): `DEL-03-07:SOW#CLM-003.r06`, `CLM-009.r05`, `CLM-021.s02`.
- S4 (1): `DEL-13-01:SOW#CLM-009/REQ-13-01-011`.
- S5 (3): `DEL-09-01:SOW#CLM-015.r03`, `CLM-023`, `CLM-024.r05`.
- S6–S8: the T5A-C05 DEL-16-02, DEL-12-01 and DEL-17-06 keys above.

**Packages and deliverables.** PKG-00, 03, 08, 09, 12, 13, 15, 16, 17.
DEL-00-06, 03-07, 08-03, 08-04, 09-01, 12-01, 13-01, 15-03, 16-02, 17-06.

**Rows known only from `OtherCorrections` (unapplied).**
- DEL-15-03 `CLM-006.r04`, `REQ-002`, `CLM-019`, `AC-001`: FIELD tier
  PROJECT_BASELINE → INVARIANT (unit preservation).
- DEL-09-01 ×3: FIELD BaselineClass RULED_CRITERION → PROTECTED_CHECK.
- DEL-13-01 REQ-13-01-011: FIELD verification-evidence correction.
- DEL-16-02 REQ-16-02-002: OBSERVED — same finding as FG-DEL-16-01-01, proposed
  OWNERSHIP_ELSEWHERE · PROJECT_BASELINE.
- DEL-03-07 `CLM-021.s02`: OBSERVED cause split (POSSIBLE_DEFECT vs PARTIAL_SLICE).

**T8 against class routes (show both).**

| Keys | Class view | T8 view |
|---|---|---|
| DEL-03-07 `CLM-003.r06`, `CLM-009.r05`, `CLM-021.s02` | T7-C05 · OWNER_DECISION (effective AuthorityNeeded OWNER) | T8-K3 · CODE_FIX_CANDIDATE, AuthorityNeeded REVIEW; engineering names dimensionless fields |
| DEL-15-03 8 FG-02 rows in T8-K3 | T7-C05 · OWNER_DECISION | T8-K3 · OWNER_DECISION (agrees) |

These three DEL-03-07 rows appear in `R3/T8_ROUTE_DISAGREEMENTS.csv`. This
packet does not choose between the views. If the owner confirms the T8-K3
reading (C7), subject 3 leaves this packet and goes to H2 under REVIEW.

## 6. Risks

- **Undecided.** Export packages lose hashes and units that the handoff contract
  promises. Diagnostics show classes and remediation that no producer supplied,
  against the do-not-invent principle. Unitless values can pass the import gate
  (INVARIANT). A protected-check tolerance stays loosened without its governance
  event. A text catch-up done first would ratify all of this silently.
- **"Adopt the code" options.** Amending INVARIANT-backed requirements (S1, S3)
  moves a contract boundary and needs an explicit basis. S5(a) loosens a
  protected criterion by record.
- **"Restore the text" options.** Code briefs on an unreached exporter (S1) or
  PKG-13 surface (S4) may be wasted if B7 or B9 retire those paths.

## 7. Recommended routing

- S1–S3, S5–S8: no recommendation; owner's call.
- S4: no recommendation; sequence after B9.
- S6: no recommendation; rule jointly with B3.
- S2: the evidence supports treating the three deliverables together under one
  ruling (T7 observation 1; T11 SS-01). It does not support a preferred option.

## 8. On-ruling mechanism

| Option type | Authorises | Path |
|---|---|---|
| Adopt behaviour (S1a, S2b, S3b, S4b, S6b, S7a/b, S8a) | SOW text revision on the named deliverables, then R5 record repair of the listed rows | Ordinary change path; R5 under separate authorization. S3b also needs a contract amendment to OPS-K-UNIT-1 through the governance path |
| Restore text (S1b, S2a, S3a, S4a, S6a, S7c, S8b) | A code-fix brief per subject (exporter route; desktop diagnostic adapters and runner binding; import gate; preview path; operation applier; path policy; DEL-17-06 writer) | H2 candidate brief under a production brief, chirality-change PR path with independent review; rows re-verified later, never edited in place |
| S5a | A DEC-026 governed-record amendment with the reason, then R5 repair of three rows | Owner record in the register / DEC-026 record |
| S5b | A fixture and benchmark code change | H2 brief |

Nothing executes until the owner rules. R5 needs separate authorization (D-73).

## 9. Dependencies

- **Depends on:** B7 (canonical handoff path; the exporter is unreached,
  T12-C01) for S1; B9 (PKG-13 status, Knowledge panel) for S4; B3 (runtime edit
  contract) for S6; C7 (T8-K3 unit-vocabulary reading) for S3's route.
- **Blocks:** H2 briefs for the exporter, diagnostic adapters, runner binding,
  import gate, preview path; H4 rows of DEL-15-03, DEL-08-03/04, DEL-00-06,
  DEL-09-01, DEL-12-01, DEL-16-02, DEL-17-06 in these subjects (BlockedOnPacket C1).
- **Related:** B6 owns the DEL-17-04 part of T5A-C05. C6 carries the
  diagnostics-boundary method split; H3 carries T11 SS-01. A3 holds the
  separate DEL-12-01 private-data-root CP-10 item.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
packet describes records and evidence. It makes no claim of certification, code
compliance, professional approval or engineering acceptance.
