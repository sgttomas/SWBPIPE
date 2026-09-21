# Conventions — RECON_2026-09-21_WHOLE_CORPUS

**Status: BOUND by the owner's R0 ruling** (`R0_CALIBRATION/R0_RULING.md`).
This file consolidates the candidate conventions
(`R0_CALIBRATION/CANDIDATE_CONVENTIONS.md`) with every amendment the owner
adopted: the dispositions in `R0_CALIBRATION/R0_REVIEW.md` §4 and the
ruling's items 2–4. Where this file and those sources differ, the ruling
governs, then this file. A later change needs a new owner ruling.

Origin labels:
- **OWNER**: the owner's words.
- **KERNEL**: the pinned method.
- **RULED**: the owner adopted Agent 0's or the reviewer's text at R0.

## Part A — Authority and evidence

**A1. Code is evidence, not authority (KERNEL).** Implementation and tests
show what exists. They never create or change scope.

**A2. Merged pull requests carry owner intent (OWNER, RULED).** Owner's
words: "Any merged PR is an extension of my intent, through a standing
instruction and my confidence in the existing structure to carry my broad
intent sufficiently that I don't need to review the PR once I understand what
that tranche of work is intended for.  So merged PRs are great references to
see what was done and why."

- Cite a merged PR and its description in `ContextRefs` as evidence of what
  was intended and why.
- A PR's merge state and commit are facts. They may be cited as evidence of
  what landed and when.
- A merged PR never supplies the permission that `ACCEPTED_DIVERGENCE`
  requires.
- When shipped behaviour departs from a `LOCAL_DESIGN` claim and a merged PR
  explains why, the likely resolution is that the deliverable catches up.
- A merged PR does not amend a `PROJECT_BASELINE` or `INVARIANT` claim. Such a
  divergence goes to R4 with the PR as context.

**A3. Authority versus evidence (RULED).**

- *Authority* (what is wanted) comes only from the governing sources in
  `RUN_BASIS.md`.
- *Context records* are AgentRuns records, owner-direction records, plans,
  design frames, handoffs, merged PRs, run records and `MEMORY.md`. They
  never create, remove or relax a claim. They never supply the permission
  that `ACCEPTED_DIVERGENCE` requires (except under A3a). They never decide a
  disposition by themselves.
- *Evidence* (what exists) may come from any record in the frozen tree.
  That includes AgentRuns and run records: parity reports, claim maps, review
  returns and validation outputs.
  - Cite evidence in the evidence columns.
  - `SourceReliability` states whether a human disposition covers it.
- Evidence may change a disposition. Context may only explain one.
- **Discovery step.** Before recording `NONE_FOUND` for a record-type claim
  (parity, claim map, review), run `git grep -l <DEL-ID>` over the frozen
  tree. Include root `execution/_Coordination/AgentRuns/`. Start with the
  evidence map in your brief.
- Rulings adopted by reference keep the `ADOPTED_BY_REFERENCE` flag in the
  `AdoptedByReference` column.

**A3a. Owner-direction records (RULED, ruling item 2).** A verbatim,
hash-bound owner decision in an owner-direction or adoption record may
support `ACCEPTED_DIVERGENCE` if it names the exact divergence. The row
writes `OWNER_DIRECTION_RECORD:<path>` in Notes. It goes to the owner at R4
for confirmation, not decision.

**A4. `## Remaining` is not authority (OWNER, RULED).** The owner's words
(Direction 1): "I waive the `## Remaining` boostrap seeding as a ruled
variance, because it's stale", clarified as meaning the current entries "are
stale and can't be referred to as an authority".

- Each Remaining item is its own unit, audited as declared state.
- It is never cited as evidence that work is open or done.
- Remaining items take `REMAINING_STATE_MISMATCH` when their text disagrees
  with the frozen code and evidence.
- A Remaining item whose text is accurate but records no open action is
  `ALIGNED` and carries `NO_OPEN_ACTION` in Notes, for the census.
- A governing declaration that delegates residuals to `## Remaining` is
  assessed as `DECLARED_STATE`.
  - Its delegation clause is recorded as not relied on.
  - The clause does not by itself make the row non-aligned.
- A `STATUS#remaining` key is never an ownership anchor in the reverse pass.
- The contract's "Remaining is executable truth" invariant and its final
  Remaining census apply at R6. D-73 does not authorize R6.

**A5. Verification is not validation (KERNEL).** A passing test shows
implementation behaviour. Engineering correctness needs validation evidence
where the claim class demands it: a benchmark, witness, oracle or vetted
source. A unit test is never promoted to validation.

**A6. Gate evidence (OWNER, RULED; D-73 addendum).**

- No suite is run.
- Cite suite-level gate evidence in `VerificationEvidence` as
  `GATE:<path relative to the run folder>`, for example
  `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`.
- Assert per-test pass status, or any validator or tool result, only when a
  frozen-tree record or a GATE record shows it.
- Otherwise, cite the last record and write `not rerun`, or dispose `UNKNOWN`.

**A7. Frozen-state binding (KERNEL).** Read every cited path from the
evidence checkout at `00115c719`. Every row carries `SourceStateSHA`.

## Part B — Sealed two-pass protocol (RULED)

**B1. Forward pass.**

- For every claim key issued to the deliverable, decide what the claim says.
- Find evidence using the deliverable's declared paths, the evidence map and
  the A3 discovery step.
- Assign all fields.
- The worker never sees another deliverable's ledger, or the reverse
  inventory, before sealing.
- Write, validate and hash the ledger, then report the hash. The ledger is
  then sealed. Corrections go through a fresh worker.

**B2. Reverse pass.** After sealing, answer every capability row routed to
the worker. Allowed answers:

| Answer | Meaning |
|---|---|
| `CLAIMED_BY` | The deliverable owns the capability |
| `PARTIAL` | The deliverable owns part of it |
| `COVERS` | A coverage or record relation, without ownership |
| `CONSTRAINS` | An architecture-basis constraint, without ownership |
| `UNKEYED` | The deliverable should own it but no issued key covers it; `ClaimKey` names the nearest key |
| `NOT_MINE` | Not this deliverable's |

- `ClaimKey` may hold a `;` list.
- `STATUS#remaining/*` keys are not allowed.
- The reverse pass never edits the forward ledger.

**B3. Why.** A sealed forward pass cannot be anchored by knowing which code
"ought" to belong to a deliverable. The reverse pass then tests ownership
independently. R3 owns the final unmapped set.

## Part C — Classification

**C1. Units (RULED).** Every issued key gets exactly one row.

- **SURFACE rows** assess the file as a whole declared state.
- **Pre-typed rows.** Where extractor v2 marks a unit `PreType=NON_NORMATIVE`
  (section wrappers and heading-only blocks), the row takes `NON_NORMATIVE` /
  `NOT_ASSESSED`.
  - Override the pre-type only if the unit truly carries a claim, and say why
    in Notes.
- **Identification tables** that carry status, phase or basis fields are
  `DECLARED_STATE`.
- **Blocks with children.**
  - A block whose substance is fully carried by its children is `CONTAINER` /
    `COVERED_BY_CHILDREN`.
  - A block that has children and substance of its own is assessed directly.
- **Sub-claims.** Table rows are issued as deterministic `.rNN` keys. Use a
  `.sNN` sub-claim only for normative prose that no issued key covers, and
  whenever parts of a block would take different dispositions.
- **Common defects.** A defect common to every item of a surface is recorded
  once on the SURFACE row. Items are then judged on their own substance.
- **History.** The history rule applies to `MEMORY`, `STATUS#history` and
  `CONTEXT#preparation-notes`.
  - Accurate history is `ALIGNED`.
  - Only text that still reads as a current obligation is assessed.
  - An undated current declaration inside a history surface is
    `DECLARED_STATE`.
- **Blanket supersession declarations** (for example the D-41-era "current
  declaration" blocks) are their own `DECLARED_STATE` rows. They never change
  sibling dispositions.
- **Verbatim-preserved blocks.** A block preserved verbatim by the SOW
  migration is assessed as a current claim. Cite the migration record in
  `ContextRefs`.
- **Duplicates.** A duplicate of another unit in the same deliverable writes
  `DUPLICATE_OF <key>` in Notes and takes the same disposition.
- **Canonical inheritance.** Where the claim keys give a `CanonicalSituation`,
  the row inherits the canonical disposition, cause, tier and layer. Depart
  from them only with a justification in Notes.

**C2. Claim type:** `REQUIREMENT`, `ACCEPTANCE`, `EXCLUSION`,
`DECLARED_STATE`, `REMAINING_WORK`, `CONTEXT`, `CONTAINER`, `NON_NORMATIVE`,
`HISTORY`.

**C3. Authority tier (RULED).** Required on every non-aligned row. The tier
is the authority of the claim's substance as it stands.

| Tier | Use it for |
|---|---|
| `LOCAL_DESIGN` | A deliverable-local design choice. Also pointer drift and setup residue that a deliverable catch-up would repair with no decision, even when the stale text names a ruling |
| `PROJECT_BASELINE` | Substance that conflicts with, or restates, a ruling or accepted scope |
| `INVARIANT` | Substance that restates a contract, boundary or validation invariant, including every `VERIFIED_NOT_VALIDATED` row. `DivergenceLayers` carries the subject |

**C4. Baseline class (RULED).**

- Values: `ISSUED`, `FROZEN_CONTRACT`, `PROTECTED_CHECK`, `RULED_CRITERION`,
  `OWNER_HOLD`, `NONE`.
- The class names the baseline of the artifact that diverges, not of the
  claim's deliverable.
- `NONE` is the default on non-aligned rows.
- An owner-ruled gate that is not a hold on this deliverable is
  `RULED_CRITERION`.
- `PROTECTED_CHECK` is for a protected check that was removed, weakened or
  contradicted. A stale test count is not a protected-check event.

**C5. Divergence layers (RULED).** A `;` list, no spaces.

| Layer | Meaning |
|---|---|
| `CLAIMS` | The claims-language boundary only (profile layer 1) |
| `VALIDATION` | Validation and provenance |
| `IP_DATA` | The IP and data boundary |
| `BASELINE` | An accepted baseline or protected criterion |
| `SECURITY` | Security and privacy |
| `LIFECYCLE` | Lifecycle state |
| `RECORD` | Documentation or declared-state drift, with no protected layer affected |
| `NONE` | No divergence layer; valid only on `ALIGNED`, `COVERED_BY_CHILDREN` and `NOT_ASSESSED` rows |

**C6. Disposition (KERNEL core plus Piping extensions, RULED rules).**

Vocabulary:
- **Kernel:** `ALIGNED`, `IMPLEMENTED_UNDOCUMENTED`,
  `DOCUMENTED_UNIMPLEMENTED`, `PARTIALLY_IMPLEMENTED`,
  `IMPLEMENTED_DIFFERENTLY`, `ACCEPTED_DIVERGENCE`,
  `LIFECYCLE_REASSESSMENT_REQUIRED`, `DEFERRED_AGENT_WORKFLOW`,
  `AUTHORITY_CONFLICT`, `UNKNOWN`, `STALE_INPUT`.
- **Piping:** `STALE_SETUP_SPECIFICATION`, `STALE_REVIEW_OR_EVIDENCE`,
  `VERIFIED_NOT_VALIDATED`, `REMAINING_STATE_MISMATCH`,
  `ENGINEERING_AUTHORITY_REQUIRED`.
- **Structural:** `COVERED_BY_CHILDREN` (only with `CONTAINER`),
  `NOT_ASSESSED` (only with `NON_NORMATIVE`).

Rules:

- **(a)** A claim satisfied only because the governed behaviour does not
  exist is not `ALIGNED`. Use `PARTIALLY_IMPLEMENTED` or
  `DOCUMENTED_UNIMPLEMENTED`, with the cause of the gap (for example
  `DEFERRED_BY_RULING`).
- **(b)** The disposition describes the claim's subject. Whether the
  declaration is accurate goes in Notes.
- **(c)** Use the stale and mismatch classes as follows:
  - `STALE_SETUP_SPECIFICATION` for setup-era origin text;
  - `STALE_REVIEW_OR_EVIDENCE` for later declarations, review states,
    revision pins and metadata (such as dates) that were overtaken;
  - `REMAINING_STATE_MISMATCH` for `STATUS#remaining/*` units only.
- **(d)** An ISSUED claim that was true at issuance but was overtaken by a
  later ruling is `LIFECYCLE_REASSESSMENT_REQUIRED` and carries a
  `FindingGroup` (ruling item 4). `IMPLEMENTED_DIFFERENTLY` is reserved for:
  - implementation that departs from a claim;
  - a declared-open hold settled in code with no ruling (cause
    `AUTHORITY_UNCLEAR`).
- **(e)** A ruling that schedules a catch-up ("at next amendment") supports
  `ACCEPTED_DIVERGENCE` until its trigger occurs. Record the trigger in
  `RemainingWork`. **Exception (ruling item 3):** DEC-101 does not reach
  deliverable Scope of Work files. Rename residue there is a finding with
  cause `RENAME_OR_IDENTITY`, never an accepted divergence.
- **(f)** `UNKNOWN` with `AUTHORITY_UNCLEAR` means the governing sources are
  silent.
- **(g)** `ACCEPTED_DIVERGENCE` requires a named governing ruling that permits
  the divergent state, or an A3a owner record.
- **(h)** `UNKNOWN` requires the smallest next check in `RemainingWork`.

**C7. Cause tag (RULED).** Required on every non-aligned row, one per row;
the cause of the remaining gap wins.

| CauseTag | Meaning |
|---|---|
| `DOC_BEHIND_CODE` | Code advanced; the text still describes the earlier behaviour |
| `SCOPE_GREW_BY_DIRECTION` | Implementation went beyond the claim under recorded owner direction or merged PRs |
| `SCOPE_REDIRECTED_BY_RULING` | A later governing ruling changed what is wanted |
| `REDESIGN_SUPERSEDED` | The interface redesign program replaced the described behaviour |
| `RENAME_OR_IDENTITY` | Product rename and identity residue (DEC-099 to DEC-109, SCA-010) |
| `CONTRACT_VERSION_ADVANCED` | Schemas or contracts moved to a new version; schema and contract versions only |
| `REPRESENTATION_MIGRATED` | The four-document kit was replaced by `ScopeOfWork.md` or `ArchitectureBasis.md` |
| `BASIS_POINTER_STALE` | Decomposition, DAG or revision pins, section references, and relocated code anchors with unchanged behaviour |
| `OWNERSHIP_ELSEWHERE` | The behaviour landed under another deliverable's area, or the binding is only tentative |
| `PARTIAL_SLICE` | A bounded slice landed; the rest has not |
| `NOT_STARTED` | Nothing implements the claim and no ruling defers it |
| `DEFERRED_BY_RULING` | A governing ruling defers it |
| `EVIDENCE_OVERTAKEN` | Review or verification records no longer bind the frozen bytes |
| `EVIDENCE_NOT_LOCATED` | Only with `UNKNOWN`; the evidence could not be found |
| `VERIFICATION_REMOVED` | A cited test existed and was deleted |
| `VALIDATION_GAP` | Implemented and verified; the required validation basis is missing |
| `RECORD_DRIFT` | Status, Remaining or process declarations disagree with other records |
| `AUTHORITY_UNCLEAR` | Governing sources conflict or are silent |
| `POSSIBLE_DEFECT` | The code appears to contradict a governing claim |
| `OTHER` | None of the above; explain in Notes |

**C8. Verification class (RULED).** A `;` list drawn from: `UNIT`,
`BROWSER_E2E`, `NATIVE`, `STATIC_CHECK`, `DOCUMENT_REVIEW` (human or agent
review records of documents), `NONE`.

**C9. Selectability (KERNEL field).** `NOT_APPLICABLE`. Since 2026-09-19,
Piping selects work through owner-steered work graphs, not `## Remaining`.
State this once in the notes.

**C10. Source reliability (RULED).**

- Values: `VETTED`, `REVIEWED`, `UNVERIFIED`, `NOT_APPLICABLE`.
- The field rates engineering and reliance-bearing sources:
  `ValidationEvidence` and cited human dispositions.
- Rows whose evidence is tests or run records are `NOT_APPLICABLE`, and so
  are declared-state prose rows.
- `REVIEWED` requires a named human ruling or disposition covering the cited
  record.

## Part D — Ledger schema (RULED)

RFC-4180 CSV, CRLF record endings, UTF-8. One file per deliverable per pass.

**Sentinel.** Each file ends with a final record whose first field is `#END`
and whose last field holds the body-row count.

**Forward ledger `<DEL>_forward.csv`:**

```
ClaimKey, DeliverableID, UnitKind, ClaimType, ClaimClass, ClaimSummary,
NormativeSource, DecisionBasis, DeclaredState, RecordedRemaining,
ImplementationEvidence, VerificationEvidence, VerificationClass,
ValidationEvidence, SourceReliability, LifecycleState, Disposition,
Confidence, CauseTag, AuthorityTier, BaselineClass, DivergenceLayers,
FindingGroup, CanonicalSituation, AdoptedByReference, ContextRefs,
RemainingWork, AuthorityNeeded, SelectableUnderCurrentLoop, Notes,
SourceStateSHA
```

- `UnitKind`: `SURFACE`, `BLOCK`, `ITEM`, `SUBCLAIM`.
- `ClaimClass`: `GOVERNANCE`, `SCHEMA`, `MECHANICS`, `WORKFLOW`, `GUI`,
  `REPORTING`, `INTEROP`, `VALIDATION`, `SECURITY`, `DOCUMENTATION`.
- `ClaimSummary`: at most 200 characters, in the worker's own words.
- Evidence columns hold a `;` list (no spaces) of tokens, or `NONE_FOUND` or
  `NOT_APPLICABLE`. A token is one of:
  - a path from the repository root (`projects/…`, `execution/…`, `docs/…`,
    `tools/…`, `agents/…`, `workflows/…`, `.github/…`, `_DomainEngines/…`);
  - `GATE:<run-relative path>`.

  Append `::case` or `#Lnn` where useful. Put free text in Notes, never in
  evidence columns.
- `FindingGroup`: the same ID on every row that repeats one finding
  (`FG-<DEL>-NN`), or empty.
- `CanonicalSituation`: the ID from the canonical situation table, or empty.
- `AdoptedByReference`: `YES` or empty.
- `AuthorityNeeded`: `NO`, `OWNER`, `ENGINEERING`, `SCOPE_CHANGE`, `REVIEW`,
  or a decision ID. Use `NO` when a deliverable catch-up with no decision
  would repair the row (typically `LOCAL_DESIGN`). Use `OWNER` when a choice
  is needed.
- `Confidence`: `HIGH`, `MEDIUM`, `LOW`.
- `CauseTag` and `AuthorityTier` are empty on quiet rows (`ALIGNED`,
  `COVERED_BY_CHILDREN`, `NOT_ASSESSED`).

**Reverse answers `<DEL>_reverse.csv`:** `CapabilityID, Answer, ClaimKey,
Reason`, ending with the sentinel.

**Notes `<DEL>_notes.md`:** path aliases, judgment calls, canonical-table
departures, convention friction, and the smallest checks for `UNKNOWN` rows.

## Part E — Fences for every worker (KERNEL and profile)

- Read-only everywhere except the worker's named output files.
- No builds, installs or test runs.
- Never present unreviewed equation artifacts from the external piping-design
  corpus as evidence (DEC-043).
- Never quote protected standards, vendor or private data.
- Never state or imply a release, approval, compliance or certification
  claim.
- Agent dispositions are never owner rulings.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
