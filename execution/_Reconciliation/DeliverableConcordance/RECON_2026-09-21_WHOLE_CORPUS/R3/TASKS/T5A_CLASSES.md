# T5A — DOC_BEHIND_CODE classes (documents behind the product)

T5A covers the 576 rows of `R3/CORPUS_CLAIMS.csv` with `Divergent = YES` and
effective CauseTag `DOC_BEHIND_CODE`. The population is uniform in its
signature. Every row is `LOCAL_DESIGN`, BaselineClass `NONE`, layer `RECORD`
and lifecycle `IN_PROGRESS`. Every row except one (DEL-16-02 REQ-002) has
AuthorityNeeded `NO`, and every row is `ProductCallerNone = NO`. Dispositions
are `STALE_SETUP_SPECIFICATION` 418, `STALE_REVIEW_OR_EVIDENCE` 127,
`IMPLEMENTED_DIFFERENTLY` 20, `REMAINING_STATE_MISMATCH` 6,
`PARTIALLY_IMPLEMENTED` 4 and `IMPLEMENTED_UNDOCUMENTED` 1. 542 rows are
Scope of Work units. Most of the population, 506 rows (C01–C03), is text
catch-up. The deliverable records still describe a setup session, an open
TBD or an earlier declared boundary, and the frozen code has moved on. That
work routes to an R5 record repair with no substantive decision. Five smaller
classes stay separate because they need something other than a text edit:
- C04: one ruled product seam is missing from two records;
- C05: code departs from text on points the evidence reserves for the owner;
- C06: local semantic departures that need a reviewer's adopt-or-restore reading;
- C07: stale Remaining items;
- C08: rows whose own cause or vehicle is contested in the adopted resolutions.

T5A classifies and proposes routing only. It changes no row value and
decides nothing.

Classification rule (reproducible, in this order):
1. named contested-cause keys → C08;
2. named owner-reserved keys and FindingGroups FG-DEL-15-03-03 and FG-DEL-17-04-02 → C05;
3. FindingGroups FG-DEL-16-02-01 and FG-DEL-16-03-01 → C04;
4. `REMAINING_STATE_MISMATCH` → C07;
5. other `IMPLEMENTED_DIFFERENTLY`, `PARTIALLY_IMPLEMENTED` and `IMPLEMENTED_UNDOCUMENTED` rows → C06;
6. a setup-framing keyword pattern on ClaimSummary (for example setup session or pass, write scope or boundary, Phase A, deliverable-local, SEMANTIC_READY, "implements no") → C01;
7. the remaining `STALE_SETUP_SPECIFICATION` rows → C02;
8. the remaining `STALE_REVIEW_OR_EVIDENCE` rows → C03.

## Class table

| Class | Name | Rows | Pkgs | Dels | Owning authority | Route |
|---|---|---|---|---|---|---|
| T5A-C01 | Setup-session framing overtaken | 205 | 11 | 45 | REVIEW | R5_RECORD_REPAIR |
| T5A-C02 | Setup-era TBDs and future-tense product text answered by implementation | 207 | 11 | 52 | REVIEW | R5_RECORD_REPAIR |
| T5A-C03 | Post-migration declarations overtaken | 94 | 12 | 25 | REVIEW | R5_RECORD_REPAIR |
| T5A-C04 | Model-operation records omit the ruled runtime seam | 19 | 1 | 2 | REVIEW | R5_RECORD_REPAIR |
| T5A-C05 | Code departs from text on owner-reserved points | 17 | 4 | 5 | OWNER | OWNER_DECISION |
| T5A-C06 | Local semantic departures (adopt or restore) | 10 | 4 | 7 | REVIEW | REVIEW |
| T5A-C07 | Stale Remaining items | 6 | 4 | 6 | REVIEW | R5_RECORD_REPAIR |
| T5A-C08 | Cause or vehicle contested in adopted resolutions | 18 | 9 | 16 | REVIEW | REVIEW |
| | **Total** | **576** | | | | |

The full population of each class is in `T5A_CLASSES.csv`. Classes of 20
rows or fewer also list their keys below.

## T5A-C01 — Setup-session framing overtaken (205)

**Description.** These blocks and items describe the 2026-04/05 setup
session rather than the product:
- the deliverable's status as a "draft setup artifact";
- statements that "this setup pass implements no" code, tests or schemas;
- write boundaries limited to the deliverable folder, and "no files outside" checks;
- Phase A closeout and scope checks;
- setup-validity acceptance criteria that end in SEMANTIC_READY;
- setup procedure purposes.

Each was true of the setup run. Implementation, and in most cases product use,
has since happened, so read as current declarations they are overtaken. The
repair is to mark them historical or retire them, not to restate their
subject.

**Signature.** DOC_BEHIND_CODE · LOCAL_DESIGN · `STALE_SETUP_SPECIFICATION`
196, plus `STALE_REVIEW_OR_EVIDENCE` 9. The 9 are setup-status fields kept in
that class by the F3 exception. By type: DECLARED_STATE 141, REQUIREMENT 34,
ACCEPTANCE 16, EXCLUSION 8, CONTEXT 6.

**Packages.** PKG-02, 04, 06, 07, 08, 09, 10, 11, 12, 14 and 17.

**Deliverables (45).** DEL-02-01; DEL-04-01…05; DEL-06-01…05; DEL-07-01, 03, 04, 06, 07, 08; DEL-08-01…06; DEL-09-01…03; DEL-10-01…05; DEL-11-01…05; DEL-12-02, 05; DEL-14-01, 02; DEL-17-05…09.

**Owning authority.** REVIEW. The only question is whether the retired or
historical text is accurate. R5 itself needs a separate owner authorization
(D-73).

**Route.** `R5_RECORD_REPAIR`.

**On-ruling mechanism.** An R5 ruling would authorise, per deliverable, a
Scope of Work or Context text revision through the ordinary repository change
path (the chirality-change PR record). The revision marks setup-session
statements historical, or removes them, and leaves the governed subject
untouched. No lifecycle state changes. Where a row names SEMANTIC_READY as a
completion target, the revision may drop the setup target. It must not advance
or set a lifecycle state, which F3 (Direction 8) reserves for the owner in its
own workflow.

**Risk if unrepaired.** Low for product behaviour. The records are
misleading, though: a reader or agent taking the SOW literally would conclude
that no code exists, or that edits outside the folder are forbidden.
Acceptance criteria written for the setup run cannot be applied to the
implemented deliverable.

**Representative keys.**
- `DEL-07-01:SOW#CLM-019.r02`. The write scope is limited to the deliverable folder "for this setup session". The implementation lives under `apps/desktop/src`, `schemas/` and `core/gui/viewport_editor` (ledger Notes).
- `DEL-08-04:SOW#CLM-005`. A setup-session condition that no schema, exporter or tests are modified. The schema, crate and desktop writer exist, and the text has been present since 7bee9ae41 (ledger Notes).
- `DEL-11-05:SOW#CLM-022`. Setup validity ends in SEMANTIC_READY. The guide was published as `docs/contributor_guide/index.md` (DEV-001 Tranche M, bfb3931), and the deliverable is IN_PROGRESS (ledger Notes).

**Exceptions named.**
- WEAK: `DEL-09-01:SOW#CLM-003.r09`. The adopted disposition changed from STALE_SETUP_SPECIFICATION to STALE_REVIEW_OR_EVIDENCE.
- FIELD (evidence fields only; cause and disposition stand): DEL-10-04 CLM-004.r01, CLM-006.s01, CLM-011, CLM-015.r04, CLM-018, CLM-019, CLM-026 and CLM-030; DEL-10-05 CLM-018, CLM-027.r02 and CLM-028.
- OBSERVED (DecisionBasis empty, no disposition change): DEL-10-05 CLM-009.s01 and CLM-017.
- SR-1 adjacency: `DEL-04-01:CONTEXT`, `DEL-04-02:CONTEXT` and `DEL-04-03:CONTEXT`. Each whole-file Context row cites a "PKG-00 at SEMANTIC_READY" statement among its overtaken parts. That element is the SR-1 contested cluster (T8), so its repair follows the SR-1 reading.

**Boundary with C02.** The split from C02 is a keyword rule on ClaimSummary.
Both classes carry the same route and mechanism, so a row on the wrong side of
the boundary has no routing consequence. Only the repair verb differs: C01
retires or marks historical, and C02 restates.

## T5A-C02 — Setup-era TBDs and future-tense product text answered by implementation (207)

**Description.** Setup-era text, first present at 7bee9ae41 (F3), about the
product itself, of two kinds:
- open items that implementation has since fixed: "exact field names and layout TBD", "library TBD", open questions and "future" dependencies;
- future-tense requirements, construction notes and verification expectations for an artifact that now exists.

The notes mostly record implementation-level choices settled in code:
- schema field names and layouts;
- in-repo canonicalizers;
- fixture contents;
- first-coverage selections.

Engineering values that remain unset (tolerances, thresholds, timing budgets)
are recorded as still TBD, so they are not part of the gap. Some acceptance
checks cannot pass as written, because they assert that an element "remains
TBD" when it has since been fixed.

**Signature.** DOC_BEHIND_CODE · LOCAL_DESIGN · `STALE_SETUP_SPECIFICATION`
207. By type: DECLARED_STATE 145, REQUIREMENT 41, ACCEPTANCE 12, CONTEXT 8,
EXCLUSION 1.

**Packages.** PKG-02, 03, 04, 06, 07, 08, 09, 10, 11, 14 and 17.

**Deliverables (52).** DEL-02-01…05; DEL-03-07; DEL-04-01…06; DEL-06-01, 02, 03, 05; DEL-07-01…08; DEL-08-01…06; DEL-09-01, 02, 03, 05; DEL-10-01, 03, 04, 05; DEL-11-02…05; DEL-14-01…05; DEL-17-01, 02, 06, 08, 09.

**Owning authority.** REVIEW.

**Route.** `R5_RECORD_REPAIR`.

**On-ruling mechanism.** An R5 ruling would authorise a Scope of Work text
revision per deliverable, through the ordinary change path. Each settled TBD
is replaced with the implemented construct and its code anchor. Each open item
the notes say is still open stays an explicit TBD. Acceptance checks that
assert "remains TBD" are restated so that they can be evaluated. The repair
records what was built. It does not ratify an engineering value or create an
acceptance.

**Risk if unrepaired.** Moderate for records. Stated TBDs misdirect work
selection and review, because they present settled interfaces as open. Checks
that cannot pass leave the deliverable without a usable completion basis.
Implementation-level choices made in code stay unrecorded in the owning
deliverable.

**Representative keys.**
- `DEL-02-04:SOW#CLM-007.r01`. The plugin interface spec is "a draft target", with its layout TBD. The spec landed as `schemas/plugin_manifest.schema.yaml` plus the extension domain contracts (ledger Notes, and the file present at the freeze).
- `DEL-06-01:SOW#CLM-012.r04`. The canonicalization library is TBD. It was settled in code by in-repo canonicalizers: the Rust `rule_pack_document` canonicalizer, and Python `core/project_persistence/service.py:240` `canonical_json` (freeze).
- `DEL-17-06:SOW#CLM-015/DEL-17-06-VER-005`. The check confirms that table columns, JSON shape and manifest layout "remain TBD". They are now fixed, so the check as written cannot pass (ledger Notes).

**Exceptions named.**
- WEAK: DEL-07-05 CLM-005.r03 and CLM-033.r01; DEL-07-07 CLM-005.r06 and CLM-032/OI-07-07-001, -002 and -003. The adopted disposition changed from STALE_REVIEW_OR_EVIDENCE to STALE_SETUP_SPECIFICATION.
- FIELD (evidence fields only): DEL-10-04 CLM-016.s02 and CLM-031.s03; DEL-10-05 CLM-011.r03.
- `DEL-10-04:SOW#CLM-031.s03`. Its notes say the template placement "was settled in practice; no ruling on it was located". The repair should record the placement as settled in practice, not as ruled.

## T5A-C03 — Post-migration declarations overtaken (94)

**Description.** Text first declared after the initial migration. It
includes:
- D-41 R5 PDU declarations (2026-07-12);
- SOW-migration (2026-07-14) current-boundary statements;
- "current evidence boundary" and "implementation surface" statements;
- prerequisite and readiness statuses;
- evidence-state metadata such as "evidence TBD";
- completion criteria (AC-001) that enshrine TBDs the code has since closed.

Each was accurate when written. Later tranches then landed more, for example:
- R15 redaction route binding;
- runner verbs executing suite payloads;
- the DEC-053 sparse default;
- arc pressure thrust (R14-W1-T3);
- product user-load integration.

The text therefore understates the implementation.

**Signature.** DOC_BEHIND_CODE · LOCAL_DESIGN · `STALE_REVIEW_OR_EVIDENCE` 94.
By type: DECLARED_STATE 72, ACCEPTANCE 9, REQUIREMENT 5, CONTEXT 5,
EXCLUSION 3.

**Packages.** PKG-00, 03, 04, 05, 07, 08, 10, 11, 12, 13, 16 and 17.

**Deliverables (25).** DEL-00-04, 08; DEL-03-01, 04, 05, 06, 07; DEL-04-01…03; DEL-05-05; DEL-07-01, 02, 05, 06, 08; DEL-08-06; DEL-10-05; DEL-11-04; DEL-12-01…03; DEL-13-02; DEL-16-01; DEL-17-02.

**Owning authority.** REVIEW.

**Route.** `R5_RECORD_REPAIR`.

**On-ruling mechanism.** An R5 ruling would authorise SOW, Context or
Architecture-Basis text revisions through the ordinary change path. Each
revision restates the declared boundary, evidence state or prerequisite status
against the frozen implementation, and keeps open items open. Completion
criteria are revised to drop closed TBDs. Where the notes route an element to
a FindingGroup with an owner part, that element keeps its own route, and the
revision edits only the overtaken declaration. For example, AC-001 in
DEL-12-01 defers to FG-DEL-12-01-01/02/03.

**Risk if unrepaired.** Moderate for records. Reviews and agents read an
understated product boundary. That can prompt duplicate work, for example on
redaction routes already bound under R15. It also keeps completion criteria
anchored to closed questions.

**Representative keys.**
- `DEL-12-02:SOW#CLM-034`. "The helper is metadata-only and does not integrate report/export runtime routes." R15 (2026-07-22) bound the 31-route inventory to `core/security/redaction/route_control.py` and its TypeScript mirror (ledger Notes, and the file present at the freeze).
- `DEL-04-01:SOW#completion-and-reliance-basis-epistemology/AC-001`. The criterion lists arc pressure thrust and sparse policy as unresolved. Both closed (R14-W1-T3 on 2026-07-19, and DEC-053). The owner-gated mechanics-assessment item stays open (ledger RemainingWork).
- `DEL-10-05:SOW#CLM-002.r06`. The runner posture describes stub verbs. Since 2026-07-19 and 2026-07-23, run-benchmark and run-regression execute suite payloads, and export-results produces a report-package projection (ledger Notes).

**Exceptions named.**
- FIRM: `DEL-03-06:SOW#completion-and-reliance-basis-epistemology/AC-001`. The adopted disposition changed from ALIGNED to STALE_REVIEW_OR_EVIDENCE. The OtherCorrections add FindingGroup FG-DEL-03-06-01, AuthorityNeeded NO, and note that "SCOPE_REDIRECTED_BY_RULING also defensible".
- OBSERVED (DecisionBasis empty, no disposition change): DEL-10-05 CLM-005.s01, CLM-012 and AC-001.

## T5A-C04 — Model-operation records omit the ruled runtime seam (19)

**Description.** The DEL-16-02 (validation and diff preview) and DEL-16-03
(audit trail) Scopes of Work present Python engines as the deliverables'
implementation:
- `core/model_operations/validation_preview/engine.py`;
- `core/model_operations/audit_trail/engine.py`.

Both engines have test-only callers. The SOWs also declare operation
application, persistence and the envelope contracts "outside this slice" or
TBD. At the freeze, the operations the GUI actually performs run through the
Rust operation applier (`core/model_operations/operation_applier`). Its
outcome envelope is published (`operation_outcome.schema.json`). The ledgers
record that DEC-020 placed application in the applier seam, and that SCA-003
ruled the container. The records need to name the product seam. The Python
rows themselves stay accurate as descriptions of the Python modules.

**Signature.** DOC_BEHIND_CODE · LOCAL_DESIGN · `STALE_REVIEW_OR_EVIDENCE`
19. By type: DECLARED_STATE 14, REQUIREMENT 3, ACCEPTANCE 2. FindingGroups
FG-DEL-16-02-01 (W-3) and FG-DEL-16-03-01.

**Packages.** PKG-16.

**Deliverables.** DEL-16-02 (10) and DEL-16-03 (9).

**Owning authority.** REVIEW. The direction is already ruled (DEC-020,
SCA-003).

**Route.** `R5_RECORD_REPAIR`.

**On-ruling mechanism.** An R5 ruling would authorise SOW revisions for
DEL-16-02 and DEL-16-03 through the ordinary change path. Each revision
documents the applier and receipt/ledger seam and its DiffPreviewRow and
OperationOutcome contract, adds applier verification targets, and restates
the TBD rows. The revision has two preconditions, so that the repaired text
does not choose an engine the owner has yet to choose:
1. DEL-16-02 CLM-026 ("record the runtime/schema divergence as a conflict for ruling") waits on the owner decision in T5A-C05 on REQ-16-02-002 (FG-DEL-16-02-02, linked to FG-DEL-16-01-01).
2. The Python-versus-Rust status of the Python engines is part of the T8 DEC-009 cluster.

**Risk if unrepaired.** Moderate. A reviewer reading the SOW would validate
the test-only Python path and miss the product path. The runtime gap on
schema validation (C05) is invisible in the deliverable's own conflict table.

**Representative keys.**
- `DEL-16-02:SOW#CLM-006`. The Python constructs exist as stated, but the validator and preview used at runtime are the Rust applier (ledger Notes; `core/model_operations/operation_applier/src/lib.rs` at the freeze).
- `DEL-16-03:SOW#CLM-020`. It treats operation application as awaiting a future decision, which DEC-020 placed in the applier seam (ledger Notes).
- `DEL-16-02:SOW#CLM-026`. The trade-off table omits the runtime divergence on schema validation (ledger Notes).

**Population.** `DEL-16-02:SOW#CLM-005`; `DEL-16-02:SOW#CLM-006`; `DEL-16-02:SOW#CLM-012`; `DEL-16-02:SOW#CLM-013`; `DEL-16-02:SOW#CLM-019`; `DEL-16-02:SOW#CLM-022`; `DEL-16-02:SOW#CLM-026`; `DEL-16-02:SOW#CLM-010/REQ-16-02-005`; `DEL-16-02:SOW#CLM-010/REQ-16-02-007`; `DEL-16-02:SOW#completion-and-reliance-basis-epistemology/AC-001`; `DEL-16-03:SOW#CLM-005`; `DEL-16-03:SOW#CLM-006`; `DEL-16-03:SOW#CLM-009`; `DEL-16-03:SOW#CLM-013`; `DEL-16-03:SOW#CLM-016`; `DEL-16-03:SOW#CLM-020`; `DEL-16-03:SOW#CLM-026`; `DEL-16-03:SOW#CLM-028`; `DEL-16-03:SOW#completion-and-reliance-basis-epistemology/AC-001`.

**Exceptions named.** None. No row in C04 has a resolution row.

## T5A-C05 — Code departs from text on owner-reserved points (17)

**Description.** Here the code does something the text forbids or does not
contemplate, and the evidence reserves the choice for the owner. The owner
items can come from the W3 assessment's owner list, from an AuthorityNeeded
OWNER correction, or from the SOW's own requirement for a human-approved or
separate authorization that was not located. The DOC_BEHIND_CODE cause
presumes the code is the intended state. For these rows that presumption is
the open question, so a text catch-up must not proceed before a ruling.

**Signature.** DOC_BEHIND_CODE · LOCAL_DESIGN. Dispositions:
`IMPLEMENTED_DIFFERENTLY` 15 and `STALE_REVIEW_OR_EVIDENCE` 2 (DEL-15-03
CLM-008 and the architecture-basis .s03 row).

**Packages.** PKG-12, 15, 16 and 17.

**Deliverables.** DEL-12-01 (4), DEL-15-03 (8), DEL-16-02 (1), DEL-17-04 (3) and DEL-17-06 (1).

**Owning authority.** OWNER.

**Route.** `OWNER_DECISION`.

**Decisions and options as they stand in the evidence:**
1. **DEL-15-03 checksum carry vs export redaction** (8 rows, FG-DEL-15-03-03; the W3 owner item "the DEL-15-03 exporter redacts fields that the SOW requires it to preserve (PR #307)").
   - Option (a): ratify redaction and restate the checksum-carry wording.
   - Option (b): restore label carry in the export payload.

   The no-recompute and no-JCS-claim parts hold either way. Evidence: `tests/test_handoff_export_workflow.py:105` (freeze), the ledger Notes and RemainingWork ("once FG-DEL-15-03-02 is settled"), and the W3 assessment's owner items.
2. **DEL-17-04 GUI exclusion** (3 rows, FG-DEL-17-04-02; the W3 owner item "GUI panels contradict the SOW GUI exclusions").
   - Option (a): authorise the mounted MBF export panel under DEL-17-04 and lift the exclusion.
   - Option (b): assign the panel to another owner through the scope-change workflow.
   - Option (c): withdraw the panel.

   Evidence: `apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx:186` carries `deliverable_id: "DEL-17-04"`. The panel is mounted from `App.tsx` (freeze) and was added 2026-06-08 (TP-MAC-50). No separate authorization was located.
3. **DEL-16-02 REQ-16-02-002 runtime schema validation** (1 row, OBSERVED; OtherCorrections give tier PROJECT_BASELINE; AuthorityNeeded OWNER).
   - Option (a): validate runtime intents against the governing JSON Schema.
   - Option (b): restate the requirement for the intent-structure contract.

   Take this jointly with FG-DEL-16-01-01. Evidence: `core/model_operations/operation_applier/src/lib.rs:2072` `check_intent_structure`, against the Python `Draft202012Validator` at `core/model_operations/validation_preview/engine.py:192` (freeze).
4. **DEL-12-01 symbolic path-class vocabulary** (4 rows). The SOW's step 1 says to preserve its six classes unless a human-approved decision replaces them. The policy and guard use an eleven-class vocabulary, and no such decision was located.
   - Option (a): ratify the policy vocabulary into the SOW.
   - Option (b): record why the SOW names differ.

   Evidence: `core/security/local_first_storage/controls.py:53` (freeze) and the ledger Notes. The intent, symbolic and private by default, is preserved.
5. **DEL-17-06 member inventory** (1 row). The package ships one result-row table (nine 0.2 members), not per-entity tables.
   - Option (a): record the implemented inventory.
   - Option (b): state that per-entity tables are still wanted.

   This is a product-scope choice. Evidence: the ledger Notes and RemainingWork.

**On-ruling mechanism.** Each is an R4 owner decision packet. On an
"adopt the implementation" ruling, R5 authorises the SOW text revision
through the ordinary change path. On a "restore the text" ruling, a
`CODE_FIX_CANDIDATE` brief goes to engineering. For DEL-17-04, a
reassignment goes through the scope-change workflow instead.

**Risk if unrepaired.**
- DEL-15-03: exports may drop context a downstream receiver is promised.
- DEL-16-02: the runtime reports "schema_validation" without JSON Schema validation.
- DEL-17-04: an unauthorised GUI surface sits against a shall-not.
- DEL-12-01 and DEL-17-06: vocabulary and inventory disagree with the governing text.

A text-only catch-up done first would silently ratify unruled product
behaviour.

**Representative keys.** `DEL-15-03:SOW#CLM-015`, `DEL-17-04:SOW#CLM-003`
and `DEL-16-02:SOW#CLM-010/REQ-16-02-002`. The evidence is given above.

**Population.** `DEL-16-02:SOW#CLM-010/REQ-16-02-002`; `DEL-12-01:SOW#CLM-005`; `DEL-12-01:SOW#CLM-019`; `DEL-12-01:SOW#CLM-027`; `DEL-12-01:SOW#CLM-032`; `DEL-15-03:SOW#CLM-006.r05`; `DEL-15-03:SOW#CLM-008`; `DEL-15-03:SOW#CLM-011/DEL-15-03-REQ-008`; `DEL-15-03:SOW#CLM-012`; `DEL-15-03:SOW#CLM-015`; `DEL-15-03:SOW#CLM-022`; `DEL-15-03:SOW#CLM-031`; `DEL-15-03:CONTEXT#architecture-basis-injection.s03`; `DEL-17-04:SOW#CLM-003`; `DEL-17-04:SOW#CLM-008`; `DEL-17-04:SOW#CLM-015`; `DEL-17-06:SOW#CLM-006`.

**Exceptions named.** OBSERVED: `DEL-16-02:SOW#CLM-010/REQ-16-02-002`. The
resolution records the same finding as FG-DEL-16-01-01, with different
fields: OWNERSHIP_ELSEWHERE · PROJECT_BASELINE is proposed as the consistent
reading. Its effective cause stays DOC_BEHIND_CODE.

## T5A-C06 — Local semantic departures: adopt or restore (10)

**Description.** The implementation meets the claim's intent through a
different construct, or the contract text omits part of the implemented
boundary. None carries an owner marker or a W3 owner item. Cases:
- **Vocabulary.** DEL-09-05 criterion outcomes are pass, fail or TBD with a reason, instead of four gate-level outcomes.
- **Credential and transmission.** In DEL-12-04, credentials are separate record kinds, and the transmission default is a derived guard posture rather than a field.
- **Stricter than the text.** In DEL-12-03, the guard rejects a whole telemetry event instead of excluding a field.
- **Contract criteria.** In DEL-07-01, DEL-07-02 and DEL-11-02, AC-001 does not describe the implemented boundary.
- **Unlisted modules.** DEL-07-02 CLM-019 omits modules that self-identify with the deliverable.
- **Threat model.** In DEL-12-05, the model was not revised after the export-route redaction changes, although its own trigger list requires it.

**Signature.** DOC_BEHIND_CODE · LOCAL_DESIGN. Dispositions:
`IMPLEMENTED_DIFFERENTLY` 5, `PARTIALLY_IMPLEMENTED` 4 and
`IMPLEMENTED_UNDOCUMENTED` 1.

**Packages.** PKG-07, 09, 11 and 12.

**Deliverables.** DEL-07-01, DEL-07-02 (2), DEL-09-05 (2), DEL-11-02, DEL-12-03, DEL-12-04 (2) and DEL-12-05.

**Owning authority.** REVIEW.

**Route.** `REVIEW`.

**On-ruling mechanism.** A reviewer under the review workflow confirms, per
row, whether the implemented semantics are adopted. The default reading in
the ledgers is to adopt, since each RemainingWork is a text catch-up. The
adoption is then carried out as an R5 record repair. For DEL-12-05, the
review is the threat-model review its own trigger list requires, recorded as
a review outcome (ledger RemainingWork). If a reviewer finds that the text
should prevail, the row becomes a `CODE_FIX_CANDIDATE` brief. Escalate to
OWNER only if the reviewer finds a product-level choice.

**Risk if unrepaired.** Low to moderate:
- vocabulary drift between the SOW and the records;
- a stale threat model after export-surface changes, where the ledger records no breach shown;
- contracts that do not describe what exists.

**Representative keys.**
- `DEL-09-05:SOW#CLM-006`. The four gate outcomes against the per-criterion pass, fail and TBD-with-reason in the checklist and gate-record schema (ledger Notes).
- `DEL-12-03:SOW#CLM-005.r05`. The code rejects the whole event (`telemetry_forbidden_field`). The ledger reads this as not a privacy weakening.
- `DEL-12-05:SOW#CLM-036`. `threat_model.md` was last changed 2026-06-06 (d2defca8a), before the export redaction changes (ledger Notes).

**Population.** `DEL-07-01:SOW#completion-and-reliance-basis-epistemology/AC-001`; `DEL-07-02:SOW#CLM-019`; `DEL-07-02:SOW#completion-and-reliance-basis-epistemology/AC-001`; `DEL-09-05:SOW#CLM-006`; `DEL-09-05:SOW#CLM-021`; `DEL-11-02:SOW#completion-and-reliance-basis-epistemology/AC-001`; `DEL-12-03:SOW#CLM-005.r05`; `DEL-12-04:SOW#CLM-006.r08`; `DEL-12-04:SOW#CLM-006.r09`; `DEL-12-05:SOW#CLM-036`.

**Exceptions named.** None with resolution rows. `DEL-07-02:SOW#CLM-019`
notes former-product-name residue in an unlisted module. That residue belongs
to the R4 rename class (T4B), not here.

## T5A-C07 — Stale Remaining items (6)

**Description.** `STATUS#remaining/*` items whose description of the landed
baseline, or of what is still open, disagrees with the frozen code. Cases:
- D-68 and table-editing slices missing from the "landed" list (DEL-07-01 and DEL-07-02);
- report-package binding that has landed (DEL-08-06);
- the export-results stub clause (DEL-09-04);
- test families described as absent (DEL-12-01);
- a viewer description the code no longer matches (DEL-07-05). This drift was deliberate since PR #787, per the ledger.

**Signature.** DOC_BEHIND_CODE · LOCAL_DESIGN · `REMAINING_STATE_MISMATCH` 6.
All have Remaining = YES.

**Packages.** PKG-07, 08, 09 and 12.

**Deliverables.** DEL-07-01, DEL-07-02, DEL-07-05, DEL-08-06, DEL-09-04 and DEL-12-01.

**Owning authority.** REVIEW. DEL-12-01 R01 needs an owner or reviewer to
choose which store tests satisfy LFSP-REQ-011 (ledger RemainingWork). DEL-09-04
R01 keeps the owner-gated MAINTAINER_REVIEWED promotion open.

**Route.** `R5_RECORD_REPAIR`.

**On-ruling mechanism.** An R5 ruling would authorise `_STATUS.md` Remaining
revisions through the ordinary change path. Each revision restates the landed
baseline and keeps the genuinely open items. This is a declared-state edit,
not a lifecycle transition. T9 carries the Remaining census as a cross-view.
Remaining no longer selects work (C9), so the risk is to records only.

**Risk if unrepaired.** Low. The records understate the landed work and, in
DEL-07-05, describe a viewer behaviour the product deliberately removed.

**Representative keys.**
- `DEL-09-04:STATUS#remaining/R01`. The runner has bound export-results to the report-package payload since 2026-07-23 (f82bb28e2), with four committed witnesses (ledger Notes).
- `DEL-07-05:STATUS#remaining/R02`. `apps/desktop/src/features/results/ResultsPanel.tsx:114` renders `GoverningRatioState ratioCount={0}` (freeze), so the governing-ratio state always reads unavailable.
- `DEL-12-01:STATUS#remaining/R01`. The store (DEL-02-05) and the route test families exist, while the 2026-08-21 record still calls them open.

**Population.** `DEL-07-01:STATUS#remaining/R06`; `DEL-07-02:STATUS#remaining/R08`; `DEL-07-05:STATUS#remaining/R02`; `DEL-08-06:STATUS#remaining/R01`; `DEL-09-04:STATUS#remaining/R01`; `DEL-12-01:STATUS#remaining/R01`.

**Exceptions named.** None with resolution rows.

## T5A-C08 — Cause or vehicle contested in adopted resolutions (18)

**Description.** Rule 3 keeps these rows visible as their own class. The
underlying text is overtaken, as in C01–C03. What is contested, in an adopted
resolution row, is whether DOC_BEHIND_CODE is the right cause, or whether the
key is the right vehicle. The synthesis rule does not apply those corrections,
because none of them sets a Disposition. As a result the effective cause
stays DOC_BEHIND_CODE, and the rows sit in T5A's partition. The groups:
- **Cause corrected by FIELD, not applied.**
  - DEL-00-05, 06, 07 and 08 `CONTEXT#anticipated-artifacts` should be SCOPE_REDIRECTED_BY_RULING, "use one cause for all seven anticipated-artifacts rows".
  - `DEL-01-02:SOW#CLM-010.r03` should be RECORD_DRIFT. The moved item is a policy document, not code.
- **Vehicle corrected by FIELD.** In `DEL-06-04:MEMORY` and `DEL-06-05:MEMORY`, the MEMORY row should read HISTORY · ALIGNED. The finding belongs on a new, unminted `MEMORY.s01` key.
- **CONTESTED.**
  - `DEL-03-04`, `DEL-03-05` and `DEL-03-06:CONTEXT#architecture-basis-injection.s02` have a cause split across groups, DOC_BEHIND_CODE against SCOPE_REDIRECTED_BY_RULING, "Agent 0 to resolve corpus-wide". This is the architecture-basis `.sNN` blind spot.
  - `DEL-12-02:SOW#CLM-037/REXC-OI-002`: the F3 origin test is ambiguous between item identity and the assessed sentence ("conventions to settle").
  - `DEL-14-05:SOW#CLM-026.s01`: the repair must not text-edit what the owner has yet to confirm (FG-DEL-14-05-03).
- **OBSERVED, where R3 may harmonise the cause.**
  - `DEL-01-02:SOW#CLM-011`: SCOPE_REDIRECTED_BY_RULING or RECORD_DRIFT is suggested.
  - `DEL-08-04:SOW#CLM-003.r08`: RECORD_DRIFT in G1 against DOC_BEHIND_CODE in G2 for the same setup-status field.
  - `DEL-10-01:SOW#CLM-004` and `DEL-10-03:SOW#CLM-004`: members of the W-3 SEMANTIC_READY lifecycle-target cause question, adjacent to SR-1.
- **RESOLVED_PAIR.** `DEL-04-01:SOW#CLM-014` and `CLM-021` "keep as sealed", and "R3 may harmonize cause" with DEL-04-04 and DEL-04-06 (SCOPE_REDIRECTED_BY_RULING).

**Signature.** Effective DOC_BEHIND_CODE · LOCAL_DESIGN. Dispositions:
`STALE_SETUP_SPECIFICATION` 15 and `STALE_REVIEW_OR_EVIDENCE` 3. By type:
DECLARED_STATE 15, HISTORY 2, REQUIREMENT 1.

**Packages.** PKG-00, 01, 03, 04, 06, 08, 10, 12 and 14.

**Deliverables (16).** DEL-00-05…08; DEL-01-02; DEL-03-04…06; DEL-04-01; DEL-06-04, 05; DEL-08-04; DEL-10-01, 03; DEL-12-02; DEL-14-05.

**Owning authority.** REVIEW. Agent 0's R3 integration and the T8 cluster
readings do the harmonising. One element goes to OWNER: the F3 origin
question on DEL-12-02 is a change to a RULED convention.

**Route.** `REVIEW`.

**On-ruling mechanism.** Agent 0 harmonises the cause in R3 integration. For
the architecture-basis `.s02` and SEMANTIC_READY-adjacent rows, it follows the
T8 reading. The outcome is recorded as a proposed R4 item, with no change to a
row value. An owner ruling on CONVENTIONS F3 would settle the origin test
(item or sentence). The DEL-06-04 and DEL-06-05 vehicle fix needs a key
minted for `MEMORY.s01` in any later ledger revision. Once the cause is
settled, the text repair itself follows the C01–C03 R5 record repair. The
exception is DEL-14-05 CLM-026.s01, which waits for the owner's
FG-DEL-14-05-03 confirmation.

**Risk if unrepaired.** Partition and packet risk rather than product risk:
- If Agent 0 applies the FIELD cause corrections, 5 rows leave the T5A partition for T5B (SCOPE_REDIRECTED_BY_RULING) or T4B (RECORD_DRIFT).
- If it applies the vehicle correction, 2 rows leave the divergent set, and their finding has no minted key.
- Inconsistent causes across sibling rows would split one R4 item into several.

**Representative keys.**
- `DEL-03-04:CONTEXT#architecture-basis-injection.s02`. The Still-TBD list is partly settled: lockfiles, hosted CI (gate evidence PR834_CI) and the export schema set. The cause is CONTESTED.
- `DEL-00-05:CONTEXT#anticipated-artifacts`. `docs/architecture/gui_state_model.md` was never created, and the register has moved to `ArchitectureBasis.md` (ledger Notes). FIELD corrects the cause to SCOPE_REDIRECTED_BY_RULING.
- `DEL-06-04:MEMORY`. The undated Remaining TBDs block calls the canonicalization library TBD. `rule_pack_document` has used the `canonical_json` crate since 2026-06-12 (ledger Notes). FIELD corrects the vehicle.

**Population.** `DEL-00-05:CONTEXT#anticipated-artifacts`; `DEL-00-06:CONTEXT#anticipated-artifacts`; `DEL-00-07:CONTEXT#anticipated-artifacts`; `DEL-00-08:CONTEXT#anticipated-artifacts`; `DEL-01-02:SOW#CLM-010.r03`; `DEL-01-02:SOW#CLM-011`; `DEL-03-04:CONTEXT#architecture-basis-injection.s02`; `DEL-03-05:CONTEXT#architecture-basis-injection.s02`; `DEL-03-06:CONTEXT#architecture-basis-injection.s02`; `DEL-04-01:SOW#CLM-014`; `DEL-04-01:SOW#CLM-021`; `DEL-06-04:MEMORY`; `DEL-06-05:MEMORY`; `DEL-08-04:SOW#CLM-003.r08`; `DEL-10-01:SOW#CLM-004`; `DEL-10-03:SOW#CLM-004`; `DEL-12-02:SOW#CLM-037/REXC-OI-002`; `DEL-14-05:SOW#CLM-026.s01`.

## Coverage

- **Population.** 576 rows of `R3/CORPUS_CLAIMS.csv` with `Divergent = YES` and effective `CauseTag = DOC_BEHIND_CODE`. The launch message states 576.
- **Classified.** `T5A_CLASSES.csv` has 576 body rows, and the `#END` count is 576. By class: C01 205, C02 207, C03 94, C04 19, C05 17, C06 10, C07 6, C08 18.
- **Check.** A read-only Python script re-read `CORPUS_CLAIMS.csv` and `T5A_CLASSES.csv`. It confirmed:
  - the key set equals the population set (0 missing, 0 extra);
  - no key is duplicated;
  - every DeliverableID matches the corpus row;
  - every ClassID is one of the eight classes;
  - the class counts sum to 576.
- **Resolution rows.** All 46 population rows with a non-empty `ResolutionClasses` are named in their class's exceptions: CONTESTED 5, OBSERVED 10, FIELD 21, WEAK 7, RESOLVED_PAIR 2, FIRM 1.
- **Inputs.** Ledger Notes and RemainingWork came from the sealed `WAVES/W*/PKG-*/DEL-*/DEL-*_forward.csv` files (no `superseded_<n>/`). Resolutions came through `CORPUS_CLAIMS.csv` `OtherCorrections`, and the adopted `RESOLUTIONS.csv` files were not re-merged. No draft resolutions file (`RESOLUTIONS_DRAFT*.csv`) was read. Freeze citations are at 00115c71931bcae79909602d653740d3bb72dfa1. No other task's files were read.

## R3 observations

These are observations, not corrections. Effective values stand.

1. **Unapplied cause corrections affect the partition.** Five population rows carry FIELD text that proposes a cause other than DOC_BEHIND_CODE: DEL-00-05, 06, 07 and 08 anticipated-artifacts, and DEL-01-02 CLM-010.r03. The two DEL-04-01 RESOLVED_PAIR rows also allow harmonising. The synthesis rule applies value fields only when a Disposition is set, so these stay in T5A. If Agent 0 applies them, the T4A–T7 exactly-once partition must be recounted (T5A would lose up to 5 rows, or 7 with the DEL-04-01 pair).
2. **DEL-15-03 cause.** The DOC_BEHIND_CODE · LOCAL_DESIGN reading on the 8 FG-DEL-15-03-03 rows presumes the redacting export code is the intended state. The W3 assessment lists the same fact as an owner product item, and the ledger RemainingWork defers the restatement until FG-DEL-15-03-02 is settled. T5A routes them to OWNER_DECISION (C05) rather than to text repair.
3. **Test-only engines not flagged under F7.** The 19 C04 rows (DEL-16-02 and DEL-16-03) record in Notes that the SOW-named Python engines have "test-only callers", but all 19 are `ProductCallerNone = NO`. The F7 marker describes the row's own implementation evidence, so this may be correct. A T12-style unreached-engine view built only from the flag would still miss these two engines.
4. **Ownership signal in DEL-17-04.** The DEL-17-04 GUI rows (C05) and DEL-07-02 CLM-019 (C06: unlisted modules that self-identify with the deliverable) are ownership signals carried under DOC_BEHIND_CODE. T3 or T11 may see the same surfaces from the capability side.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
note describes records and evidence. It makes no claim of certification, code
compliance, professional approval or engineering acceptance.
