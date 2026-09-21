# Candidate conventions and method addendum — R0

**Status: CANDIDATE.** Proposed by Agent 0 for calibration. Nothing here binds
until the owner rules it at the end of R0 (workflow invariant
"Human-calibrated conventions"; D-73 Item 2). Calibration workers apply these
candidates so that their fitness can be judged; calibration ledgers are
evidence about the conventions, not accepted findings, and every calibration
deliverable is re-encoded in the ordinary waves.

Each convention is labelled with its origin: **OWNER** (the owner's words,
recorded in the run record), **KERNEL** (restating the pinned method), or
**AGENT** (Agent 0's proposal).

## Part A — Authority and evidence

**A1. Code is evidence, not authority (KERNEL).** Implementation and tests
show what exists. They never create or change scope.

**A2. Merged pull requests carry owner intent (OWNER).** Owner's words: "Any
merged PR is an extension of my intent, through a standing instruction and my
confidence in the existing structure to carry my broad intent sufficiently
that I don't need to review the PR once I understand what that tranche of work
is intended for.  So merged PRs are great references to see what was done and
why." Proposed application: a merged PR and its description are cited in
`ContextRefs` as evidence of what was intended and why. Where shipped
behaviour departs from a `LOCAL_DESIGN` claim and a merged PR explains the
departure, the row records the PR, and the likely resolution is that the
deliverable catches up. A merged PR does not by itself amend a
`PROJECT_BASELINE` or `INVARIANT` claim (see C3); such a divergence goes to R4
with the PR as context.

**A3. Governing versus context (AGENT).** Governing: the authorities in
`RUN_BASIS.md` (PRD, CONTRACT, DIRECTIVE, SPEC, TYPES, boundary documents,
claims registry, decomposition and its §12 log, register rulings, accepted
scope changes, the approved DAG). Context: AgentRuns records, owner-direction
records, approved plans, design specifications and frames, handoffs, merged
PRs, run records, `MEMORY.md`. Context explains a divergence and drives
clustering. It never changes a claim's `Disposition`. Where a register ruling
adopts a plan or specification by reference (for example D-67 and D-68), the
worker cites the ruling as governing and the plan as context, and flags the
row `ADOPTED_BY_REFERENCE` in `Notes`; the owner decides at R4 how far such
adoption reaches.

**A4. `## Remaining` is not authority (OWNER).** Owner's words (Direction 1):
"I waive the `## Remaining` boostrap seeding as a ruled variance, because
it's stale", clarified as meaning that the current entries "are stale and
can't be referred to as an authority". Proposed application: each Remaining
item is its own unit (`STATUS#remaining/Rnn`) audited as declared state
(`ClaimType=DECLARED_STATE`). It is never cited as evidence that work is open
or done. Its disposition says whether its text matches the frozen code and
evidence. The contract's "Remaining is executable truth" invariant and its
final Remaining census (validity item 12) apply at R6, which D-73 does not
authorize; the owner reconciles them then.

**A5. Verification is not validation (KERNEL, profile §4).** A passing test
shows implementation behaviour. Engineering correctness needs validation
evidence (benchmark, witness, oracle, or vetted source) where the claim class
demands it. A unit test is never promoted to validation.

**A6. Gate evidence (OWNER, D-73 addendum).** No suite is run. Workers cite
tests by file and case name from the frozen code, and cite suite-level pass
status from `GATE_EVIDENCE/` (the PR #834 hosted CI and the 9d55 sweep). They
never assert per-test pass status beyond what those records show.

**A7. Frozen-state binding (KERNEL).** Every path cited is read from the
evidence checkout at `00115c719`. Every row carries `SourceStateSHA`.

## Part B — Sealed two-pass protocol (AGENT)

**B1. Forward pass.** For every claim key issued to the worker's deliverable,
decide what the claim says, find implementation and verification evidence in
the frozen code, and assign disposition and the other fields. The worker gets
path hints only (the deliverable's own declared paths). The worker writes the
forward ledger, computes its SHA-256, and reports the hash. The ledger is then
sealed. It may not be edited afterwards; corrections go through a fresh
worker.

**B2. Reverse pass.** After sealing, the worker receives implementation
capability rows from the reverse inventory. These rows carry no owner column.
For each row, the worker answers `CLAIMED_BY <ClaimKey>`, `PARTIAL <ClaimKey>`,
or `NOT_MINE`, with a one-line reason. The reverse pass never edits the forward
ledger. A capability the worker now sees its deliverable should claim, but no
key covers, is answered `PARTIAL` or `CLAIMED_BY` the nearest key, and the gap
is noted.

**B3. Why.** The forward pass cannot be anchored by knowing which code "ought"
to be the deliverable's. The reverse pass then tests ownership independently.
R3 owns the final unmapped set.

## Part C — Classification

**C1. Unit handling (AGENT).** Every key in `CLAIM_KEYS.csv` for the
deliverable gets exactly one row.

- `SURFACE` rows assess the file as a whole declared state, including
  staleness of setup-era wording. `MEMORY` is a single history unit: dated
  entries are history, never staleness findings.
- `BLOCK` rows whose substance is fully carried by their `ITEM` children take
  `ClaimType=CONTAINER` and `Disposition=COVERED_BY_CHILDREN`.
- Blocks with no normative content (identification tables, pure headings,
  navigation) take `ClaimType=NON_NORMATIVE`. Verifiers sample these heavily.
- Where a block holds several distinct claims and no numbered items, the
  worker may add sub-claim rows `<key>.s01`, `<key>.s02`, and so on. The parent
  row is then `CONTAINER`. Workers never mint keys other than `.sNN`
  sub-claims.

**C2. Claim type (AGENT, from the kernel and the earlier Piping practice):**
`REQUIREMENT`, `ACCEPTANCE`, `EXCLUSION`, `DECLARED_STATE`, `REMAINING_WORK`,
`CONTEXT`, `CONTAINER`, `NON_NORMATIVE`, `HISTORY`.

**C3. Authority tier (AGENT, profile §5 candidate).** Required on every
non-aligned row:

- `LOCAL_DESIGN`: the claim's authority is a deliverable-local design choice.
- `PROJECT_BASELINE`: the claim restates a ruled decision, accepted scope or
  baseline.
- `INVARIANT`: the claim restates a contract, specification or boundary
  invariant (professional boundary, IP/data, security and privacy, protected
  checks, engineering validation).

**C4. Baseline class (AGENT).** On rows touching a protected baseline:
`ISSUED`, `FROZEN_CONTRACT` (hash, result-semantics or schema versions),
`PROTECTED_CHECK` (tests, tolerances, oracles, limits), `RULED_CRITERION`
(for example D-68, D-72), `OWNER_HOLD`, or `NONE`.

**C5. Divergence layers (KERNEL, profile §4).** `DivergenceLayers` lists the
layers that apply: `CLAIMS`, `VALIDATION`, `IP_DATA`, `BASELINE`, `SECURITY`,
`LIFECYCLE`, or `NONE`.

**C6. Disposition (KERNEL core plus Piping extensions).**

- Kernel core: `ALIGNED`, `IMPLEMENTED_UNDOCUMENTED`,
  `DOCUMENTED_UNIMPLEMENTED`, `PARTIALLY_IMPLEMENTED`,
  `IMPLEMENTED_DIFFERENTLY`, `ACCEPTED_DIVERGENCE`,
  `LIFECYCLE_REASSESSMENT_REQUIRED`, `DEFERRED_AGENT_WORKFLOW`,
  `AUTHORITY_CONFLICT`, `UNKNOWN`, `STALE_INPUT`.
- Piping extensions: `STALE_SETUP_SPECIFICATION`, `STALE_REVIEW_OR_EVIDENCE`,
  `VERIFIED_NOT_VALIDATED`, `REMAINING_STATE_MISMATCH`,
  `ENGINEERING_AUTHORITY_REQUIRED`.
- Structural: `COVERED_BY_CHILDREN` (only with `CONTAINER`), `NOT_ASSESSED`
  (only with `NON_NORMATIVE`).

Two further rules:

- `ACCEPTED_DIVERGENCE` requires a named governing ruling that permits the
  divergent state. A merged PR alone gives context, not permission.
- `UNKNOWN` requires the smallest next check in `RemainingWork`.

**C7. Cause tag (AGENT).** Required on every non-aligned row. Choose the
single best cause; `OTHER` requires a sentence of explanation.

| CauseTag | Meaning |
|---|---|
| `DOC_BEHIND_CODE` | The code advanced within the claim's intent; the text still describes an earlier or setup-era state |
| `SCOPE_GREW_BY_DIRECTION` | Implementation went beyond the claim under recorded owner direction or merged PRs |
| `SCOPE_REDIRECTED_BY_RULING` | A later governing ruling changed what is wanted |
| `REDESIGN_SUPERSEDED` | The interface redesign program replaced the described behaviour |
| `RENAME_OR_IDENTITY` | Product rename and identity rulings (DEC-099 to DEC-109, SCA-010) |
| `CONTRACT_VERSION_ADVANCED` | Schemas or contracts moved to a new version (for example v0.2) |
| `OWNERSHIP_ELSEWHERE` | The behaviour landed under another deliverable's area, or the binding is only tentative |
| `PARTIAL_SLICE` | A bounded slice landed; the rest has not |
| `NOT_STARTED` | Nothing implements the claim and no ruling defers it |
| `DEFERRED_BY_RULING` | A governing ruling defers it |
| `EVIDENCE_OVERTAKEN` | Cited evidence or review no longer binds the frozen state |
| `VALIDATION_GAP` | Implemented and verified; the required validation basis is missing |
| `RECORD_DRIFT` | Status, Remaining or process declarations disagree with other records |
| `AUTHORITY_UNCLEAR` | Governing sources conflict or are silent |
| `POSSIBLE_DEFECT` | The code appears to contradict a governing claim |
| `OTHER` | None of the above; explain |

**C8. Verification class (AGENT):** `UNIT`, `BROWSER_E2E`, `NATIVE`,
`STATIC_CHECK`, `NONE`.

**C9. Selectability (KERNEL field, AGENT value).** The kernel ledger carries
selectability. Since 2026-09-19, Piping selects work through owner-steered work
graphs, not `## Remaining`. Proposed default: `NOT_APPLICABLE`, with the
reason recorded once in each ledger's notes.

**C10. Source reliability (from the earlier Piping practice).** Allowed values:
`VETTED`, `REVIEWED`, `UNVERIFIED`, `NOT_APPLICABLE`. Declared-state prose rows
are `NOT_APPLICABLE`. `REVIEWED` requires a named human ruling or disposition
covering the cited record.

## Part D — Ledger schema (candidate)

RFC-4180 CSV, CRLF line endings, UTF-8, header row as below. One file per
deliverable per pass. Each file ends with a final record whose `ClaimKey` is
`#END` and whose `Notes` field holds the row count; this record is the
sentinel.

Forward ledger `<DEL>_forward.csv`, one row per claim key:

```
ClaimKey, DeliverableID, UnitKind, ClaimType, ClaimClass, ClaimSummary,
NormativeSource, DecisionBasis, DeclaredState, RecordedRemaining,
ImplementationEvidence, VerificationEvidence, VerificationClass,
ValidationEvidence, SourceReliability, LifecycleState, Disposition,
Confidence, CauseTag, AuthorityTier, BaselineClass, DivergenceLayers,
ContextRefs, RemainingWork, AuthorityNeeded, SelectableUnderCurrentLoop,
Notes, SourceStateSHA
```

- `ClaimClass`: `GOVERNANCE`, `SCHEMA`, `MECHANICS`, `WORKFLOW`, `GUI`,
  `REPORTING`, `INTEROP`, `VALIDATION`, `SECURITY`, `DOCUMENTATION`.
- `ClaimSummary`: at most 200 characters, in the worker's own words. Never
  quote protected or private data.
- Evidence columns hold semicolon-separated repository-relative paths, with
  `::case` or `#Lnn` where useful, or `NONE_FOUND` or `NOT_APPLICABLE`.
- `AuthorityNeeded`: `NO`, `OWNER`, `ENGINEERING`, `SCOPE_CHANGE`, `REVIEW`,
  or a decision ID.
- `Confidence`: `HIGH`, `MEDIUM`, `LOW`.
- `CauseTag`, `AuthorityTier` and `BaselineClass` are empty on `ALIGNED`,
  `COVERED_BY_CHILDREN` and `NOT_ASSESSED` rows.

Reverse answers `<DEL>_reverse.csv`:

```
CapabilityID, Answer, ClaimKey, Reason
```

`Answer` is one of `CLAIMED_BY`, `PARTIAL` or `NOT_MINE`. The file ends with an
`#END` sentinel record.

Notes `<DEL>_notes.md`: a short account covering path aliases, judgment calls,
convention friction (what did not fit), and the smallest checks for
`UNKNOWN` rows.

## Part E — Fences for every worker (KERNEL and profile)

- Read-only everywhere except the worker's named output files.
- No builds, installs or test runs.
- Never present equation artifacts from the external piping-design corpus as
  evidence (DEC-043).
- Never quote protected standards, vendor or private data.
- Never state or imply a release, approval, compliance or certification claim.
- Agent dispositions are never owner rulings.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
