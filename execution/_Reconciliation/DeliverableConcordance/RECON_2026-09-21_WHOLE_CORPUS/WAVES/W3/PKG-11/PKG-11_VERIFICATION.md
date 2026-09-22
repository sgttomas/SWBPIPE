VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-11 verification (wave W3, sampling STANDARD)

- Verifier: a fresh, evidence-only TASK (Type 2). Parent: HELP_HUMAN Agent 0,
  run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`. I am independent of the
  PKG-11 manager and its workers (G1 `a7198beae8c3edc38`, G2
  `ab4f5789455d9240b`).
- Brief: `briefs/R2-VERIFIER_brief.md`. I checked its SHA-256
  `47fb3c5225ac0a3c532d99a53ee3211129998f887dc9187b40ab898bb2dd5b00`, and it
  matches.
- Evidence: the read-only freeze. `git rev-parse HEAD` there gives
  `00115c71931bcae79909602d653740d3bb72dfa1`.
- Rules applied: `CONVENTIONS.md` (Parts A to F, with Part F judged like any
  other rule), `CANONICAL_SITUATIONS.md` and
  `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`. I used `WAVES/W1/RESOLUTIONS.csv`
  and `WAVES/W3/RESOLUTIONS_DRAFT_PART1.csv` as precedent only.
- Seals: for all five forward ledgers, the recomputed SHA-256 equals the SEAL
  record: DEL-11-01 `cd1f14a1…`, 02 `fc15e5ad…`, 03 `c5c5eb51…`,
  04 `e3f5de21…` and 05 `64f4c972…`.
- I read the manager's transcripts and the workers' returns only for the
  flags they raise. They are not evidence about the claims.
- Every disposition below is an agent judgment, not an owner ruling.

## 1. Verdict basis

- No deliverable's firm error rate on sampled rows exceeds 10%. The highest
  is 3.0%, on DEL-11-04.
- Two firm errors fall in a 100%-sampled class: the shared-body rows
  `CONTEXT#architecture-basis-injection.s02` in DEL-11-04 and DEL-11-05.
  - Each changes only the stale class, from `STALE_REVIEW_OR_EVIDENCE` to
    `STALE_SETUP_SPECIFICATION`.
  - Neither changes the tier (`LOCAL_DESIGN`) or the owner routing
    (`AuthorityNeeded NO`).
  - The rerun rule is therefore not triggered.
- **The package-level firm false-alignment rate is 1/50 = 2.0%.** That is
  within the 5% scale-out gate.
- The one firm false alignment is DEL-11-03 `CLM-014.r20`. The ledger holds
  the same "invented examples only" rule two ways: it applies CP-11 to
  REQ-11-03-08 and CLM-028, but leaves this row `ALIGNED`.
- "Contested" refers to the three firm and three weak rows in §4, plus the
  shared-body conflict in §5. None of them warrants a rerun.

## 2. Per-deliverable tables

Sample classes follow the brief's sampling table. The first matching class
wins.

- **A100:** `INVARIANT`, `ACCEPTED_DIVERGENCE`, `AUTHORITY_CONFLICT`,
  `UNKNOWN` or `LIFECYCLE_REASSESSMENT_REQUIRED` rows, and rows carrying
  `PROTECTED_CHECK` or `FROZEN_CONTRACT`. No PKG-11 deliverable is ISSUED.
- **SH100:** the unit has `SharedTextCount > 1` in `CLAIM_KEYS_V2.csv`. A
  `.sNN` row inherits its parent's count.
- **F2F4:** an `ALIGNED` row with `GAP_WORDING_CHECKED:` or `OPEN_ACTION:` in
  Notes.
- **PC25:** an `ALIGNED` row with `PRODUCT_CALLER: NONE`.
- **NA25:** other non-aligned rows.
- **N20:** other `ALIGNED` normative rows (`REQUIREMENT`, `ACCEPTANCE` or
  `EXCLUSION`).
- **S10:** structural rows (`COVERED_BY_CHILDREN`, `NOT_ASSESSED`), inherited
  canonical rows and SURFACE rows.
- **Not in any class:** other quiet rows (aligned `DECLARED_STATE`, `HISTORY`
  and `CONTEXT` rows with no canonical ID). The counts are 11, 24, 17, 17
  and 21. The sampling table gives them no rate.

Selection method: within each deliverable and class, I sorted candidate keys
by SHA-256 of the full `ClaimKey` and took the lowest ceil(rate × n). The
list is deterministic and was not weighted by confidence. The brief's sort
rule governs over its "weighted toward" wording.

"False-align rate" means firm false alignments among the sampled `ALIGNED`
normative rows, in any class.

| DEL | Rows | Candidates (A100/SH100/F2F4/PC25/NA25/N20/S10) | Sampled (A100/SH100/F2F4/PC25/NA25/N20/S10) | Firm | Weak | Field | Sampled ALIGNED normative | Firm false-align rate |
|---|---|---|---|---|---|---|---|---|
| DEL-11-01 | 119 | 4/20/1/0/24/38/21 | 42 (4/20/1/0/6/8/3) | 0 | 0 | 1 | 13 | 0/13 = 0% |
| DEL-11-02 | 137 | 0/20/2/0/31/40/20 | 40 (0/20/2/0/8/8/2) | 0 | 1 | 0 | 10 | 0/10 = 0% |
| DEL-11-03 | 139 | 0/7/5/0/39/51/20 | 35 (0/7/5/0/10/11/2) | 1 | 0 | 0 | 15 | 1/15 = 6.7% |
| DEL-11-04 | 107 | 1/15/0/23/30/1/20 | 33 (1/15/0/6/8/1/2) | 1 | 1 | 1 | 7 | 0/7 = 0% |
| DEL-11-05 | 114 | 0/19/0/0/37/22/15 | 36 (0/19/0/0/10/5/2) | 1 | 1 | 0 | 5 | 0/5 = 0% |
| **PKG-11** | 616 | | 186 | 3 | 3 | 2 | 50 | **1/50 = 2.0%** |

Firm error rate on all sampled rows:

| DEL | Firm error rate |
|---|---|
| DEL-11-01 | 0% (0/42) |
| DEL-11-02 | 0% (0/40) |
| DEL-11-03 | 2.9% (1/35) |
| DEL-11-04 | 3.0% (1/33) |
| DEL-11-05 | 2.8% (1/36) |

The selected keys, by deliverable and class (listed in ledger order, not draw order):

- **DEL-11-01:**
  - A100: CLM-010.s02, CLM-014.s02, CLM-017, CLM-018.r05.
  - F2F4: CLM-011/UG-REQ-008.
  - SH100: all 20 shared-body rows. These are CLM-013.r01/.r05,
    CLM-018.r04, CLM-020.r01/.r03–.r06, CLM-021, CLM-028 and the CONTEXT
    blocks (scope-coverage, objective-support, context-envelope,
    context-budget-qa, package-reference, decomposition-reference,
    architecture-basis-injection and its .s01/.s02, preparation-notes).
  - NA25: CLM-007, CLM-014.s01, AC-001, CLM-020.r02, OUT-001, MEMORY.s01.
  - N20: CLM-004.r01/.r03/.r07, CLM-006.r03, UG-REQ-002, UG-REQ-009,
    CLM-012, CLM-026.
  - S10: CLM-001, CLM-022, STATUS#remaining.
- **DEL-11-02:**
  - F2F4: REQ-11-02-011, REQ-11-02-012.
  - SH100: 20 rows. These are CLM-003.r02–.r04/.r07–.r09, CLM-020.r01,
    CLM-026, output-and-evaluation-matrix and its OUT-001, and the ten
    CONTEXT blocks.
  - NA25: CLM-002, CLM-023, CLM-003.r10, REQ-11-02-014, CLM-021, VER-001,
    CLM-033.r05, CLM-034.
  - N20: purpose OUT-001, REQ-11-02-006/-008/-013, CLM-018.r01,
    CLM-020.r07, CLM-027, CLM-032.
  - S10: CLM-003, MEMORY.
- **DEL-11-03:**
  - F2F4: CLM-004.r02/.r05, CLM-006.r01/.r06, STATUS#remaining/R01.
  - SH100: CONTEXT scope-coverage, package-reference,
    decomposition-reference, architecture-basis-injection and its .s01/.s02,
    preparation-notes.
  - NA25: CLM-005.s01, CLM-006.r07/.r09, AC-001, CLM-020, CLM-021,
    CLM-026.r02, CLM-028, OUT-001, CONTEXT (surface).
  - N20: REQ-11-03-11, CLM-014.r04/.r10/.r11/.r17/.r18/.r20/.r22/.r23,
    CLM-026, CLM-026.r08.
  - S10: CLM-014, governing-values-and-decisions-axiology.
- **DEL-11-04:**
  - A100: R-DEL-11-04-002.
  - SH100: CLM-003.r03–.r06/.r08/.r09, CLM-007.r01/.r02, and the CONTEXT
    blocks (scope-coverage, objective-support, package-reference,
    decomposition-reference, architecture-basis-injection and its .s01/.s02).
  - PC25: CLM-004.r05/.r08, CLM-006.s02, R-DEL-11-04-004, CLM-029,
    CONTEXT#description.
  - NA25: CLM-006.s01, CLM-015, CLM-019, CLM-020.r04, CLM-021.s02, VER-001,
    CLM-030, STATUS#remaining/R01.
  - N20: purpose OUT-001.
  - S10: CLM-021, CONTEXT#preparation-notes.
- **DEL-11-05:**
  - SH100: 19 rows. These are CLM-003.r02–.r05/.r07, CLM-027, CLM-033,
    output-and-evaluation-matrix and its OUT-001, STATUS#remaining, and the
    CONTEXT blocks (scope-coverage, objective-support, context-budget-qa,
    package-reference, decomposition-reference, architecture-basis-injection
    and its .s01/.s02, preparation-notes).
  - NA25: CLM-002, CLM-009, CLM-010, REQ-11-05-02, REQ-11-05-03, CLM-016,
    CLM-018, CLM-019, STATUS (surface), CONTEXT#anticipated-artifacts.
  - N20: CLM-005.r05, CLM-025, CLM-029, CLM-032.r03, CONTEXT#description.
  - S10: CLM-007, completion-and-reliance-basis-epistemology.

## 3. Package-level firm false-alignment rate

**2.0%** (1 of 50 sampled `ALIGNED` normative rows). The scale-out gate is 5%
or less, so the package passes it.

One out-of-sample row has the same kind of error: DEL-11-03 `CLM-004.r01`,
which the worker flagged itself (§4, "Out-of-sample"). If it were counted,
the rate would be 2/51 = 3.9%, still within the gate.

## 4. Disagreements

### Firm

**F-1: DEL-11-03:SOW#CLM-014.r20** (N20). A protected-content checklist row:
"Invented examples: permitted only when clearly non-code and non-engineering
reliance."

- **Row says:** `ALIGNED`. The evidence cites `centerline_analysis.md#L175`
  and `#L183`. Notes: "Holds in the note as it stands."
- **Found:**
  - The theory note (`projects/chirality-piping/docs/theory/centerline_analysis.md`,
    264 lines) contains no example. L175 describes the kinds of verification
    fixture that are useful, and L183–L187 bar copying textbook examples.
    Neither is an example.
  - The same ledger disposes the same rule, REQ-11-03-08 ("public examples,
    if any are introduced in future work, shall use invented non-code
    values") and CLM-028 ("future examples, if used, must be invented"), as
    `DOCUMENTED_UNIMPLEMENTED · NOT_STARTED`, CP-11. Its reason: "the note
    contains no examples, so the requirement holds only because the governed
    content does not exist."
  - This row states the same permission. It holds only because no examples
    exist (CP-11, C6(a)).
  - The row sits in a split table. Its siblings r17, r18, r19, r22 and r23
    are prohibitions on content that the existing note really does keep, so
    `ALIGNED` is right for them. r20 differs because it governs content that
    is absent.
- **Right values:** `DOCUMENTED_UNIMPLEMENTED · NOT_STARTED · LOCAL_DESIGN ·
  NONE · RECORD`, with `CanonicalSituation CP-11` and `AuthorityNeeded NO`,
  as REQ-11-03-08.
- If R3 or the owner prefers to read conditional "if used" permissions as
  aligned, then REQ-11-03-08 and CLM-028 move instead. Either way, the
  ledger cannot keep both readings.

**F-2: DEL-11-04:CONTEXT#architecture-basis-injection.s02** (SH100). "Still
TBD" list.

- **Row says:** `STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING ·
  LOCAL_DESIGN · NONE · RECORD`. The summary names only the rule-expression
  grammar as ruled.
- **Found:**
  - The "Still TBD" line is byte-identical in all five PKG-11 `_CONTEXT.md`
    files; the SHA-256 of the line is `a22f16b744b7…` in each.
  - `git log -S` on the freeze gives origin `7bee9ae41` (the initial
    migration).
  - The list is not a revision pin, a review or readiness state, or
    metadata. F3 therefore gives `STALE_SETUP_SPECIFICATION`. That is the
    class DEL-11-01, 02 and 03 assign to the same text.
  - The summary and RemainingWork are also too narrow. They cover the
    grammar (DEC-022) but not the other decided items that the G1 rows name:
    the solver library (DEC-023, the in-repo sparse skyline solver) and the
    project package or store (DEC-028; SCA-003 and AB-00-04).
- **Right values:** `STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING ·
  LOCAL_DESIGN · NONE · RECORD`, `AuthorityNeeded NO`. RemainingWork should
  refresh the list for the solver library, the grammar and the
  package/container.
- **Effect:** no tier or routing change.

**F-3: DEL-11-05:CONTEXT#architecture-basis-injection.s02** (SH100). The
same text and the same error as F-2, with the same right values.

### Weak

**W-1: DEL-11-02:SOW#CLM-018.r01** (N20). "A sealed DEL-11-02 brief and
explicit write scope are required before editing the final guide."

- **Row says:** `ALIGNED`. It lists three guide edits made under bounded
  briefs: Tranche A, the 2026-06-07 refresh, and D-41 PDU-006.
- **Found:** `git log` on `projects/chirality-piping/docs/developer_guide/index.md`
  shows two more edits that the row does not mention:
  - `1b32c8096` (2026-06-22, "approve dag 007 graph authority"), which
    changed the DAG pointer;
  - `8143645ea` (2026-09-18, SWBPIPE Tranche A1), which renamed the product.
  - Both were program-level edits made under owner-ruled work. Neither was
    made under a DEL-11-02 brief.
- **Why weak:** under one reading, programme-level edits made under their own
  authorized scope satisfy "a sealed brief and explicit write scope". Under
  a literal reading, the requirement names a DEL-11-02 brief. The
  conventions do not settle which reading applies. At the least, the Notes
  should list every edit.

**W-2: DEL-11-04:SOW#CLM-011/R-DEL-11-04-002** (A100). "Example artifacts
must state that they are … not suitable for engineering reliance,
certification, approval, sealing, or code-compliance claims (PRD §21.2)."

- **Row says:** `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · NONE ·
  CLAIMS`, `AuthorityNeeded NO`. RemainingWork: "Add an explicit
  certification, approval, sealing and code-compliance non-claim to both
  fixture notices … or narrow the requirement."
- **Found:**
  - Both fixtures under `projects/chirality-piping/examples/models/invented/`
    state that the content is invented, non-code, non-project, educational
    and not suitable for engineering reliance. Neither lists the other four
    nouns. The row's facts are right.
  - `docs/claims_registry.md` §4 (DEC-081, extended by DEC-107) directs
    agents: "Do not compose ad-hoc prohibition lists". It also retires
    "multi-noun prohibition litanies on product surfaces". The first
    RemainingWork option would put such a litany into two public fixtures
    and the fixture test.
  - Against that, DEC-081 Wave 2 (`8fac6631a`) edited this very SOW row, only
    to append "(PRD §21.2)", and kept the requirement. That is evidence the
    ruling's executors treated it as a preserved requirement.
- **Why weak:** `PARTIALLY_IMPLEMENTED · INVARIANT` is defensible on the
  requirement as it stands. Whether the unmet element is still wanted, or
  was overtaken by the DEC-081/DEC-107 authoring directive, is a
  claims-language question that the conventions do not settle. If it was
  overtaken, the row becomes a stale requirement with cause
  `SCOPE_REDIRECTED_BY_RULING` and tier `PROJECT_BASELINE` (the substance
  conflicts with a ruled directive).
- **Field error in the same row (counted under Field):** RemainingWork
  offers two courses of action, so a choice is needed. By the Part D
  definition, `AuthorityNeeded` should be `OWNER`, not `NO`.

**W-3: DEL-11-05:SOW#CLM-025** (N20). Purpose: "make the first hour of
contribution work predictable: read the governing documents, …"

- **Row says:** `ALIGNED`.
- **Found:** step 1 of the guide's First-Hour Path
  (`projects/chirality-piping/docs/contributor_guide/index.md`, table row 1)
  links `../../INIT.md`. That file was removed by `9c4caf8fd` on 2026-07-04,
  and `projects/chirality-piping/INIT.md` is absent at the freeze.
  - The same ledger disposes REQ-11-05-01 and CLM-028 `PARTIALLY_IMPLEMENTED`
    for exactly this dead link.
  - Steps 2 to 10 are sound.
- **Why weak:** this row's Notes do not record the gap, so F1 is not
  engaged, and its purpose statement largely holds. Under a strict reading,
  the reading path starting at a dead link is an unmet element of
  "predictable".

### Field (the disposition stands; a field is wrong)

- **D-1: DEL-11-01:SOW#CLM-007** (NA25). RemainingWork proposes replacing
  INIT.md "with init/init-prompt.md". That file does not exist at the freeze;
  `projects/chirality-piping/init/` holds only `dev-loop-init-prompt.md` and
  `taskmgmt-init-prompt.md`. The replacement should name a path that exists.
- **D-2: DEL-11-04:SOW#CLM-011/R-DEL-11-04-002.** `AuthorityNeeded NO`
  should be `OWNER` (see W-2).

### Out-of-sample (not counted in the rates)

- **DEL-11-03:SOW#CLM-004.r01. Firm under F1, and the worker flagged it
  itself.**
  - The row is "Documentation purpose: explain the lineage from classical
    piping flexibility analysis to modern global centerline/frame
    implementation."
  - It is `ALIGNED`, yet its own Notes record that the detailed lineage is
    deferred to `TBD-public-history`, "judged on CLM-010.s02". CLM-010.s02
    is `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE`.
  - F1: "even when the same gap is also recorded on another row."
  - Right values: `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · LOCAL_DESIGN ·
    NONE · RECORD`, as CLM-010.s02.
- **DEL-11-01:SOW#CLM-005.r02 versus CLM-011/UG-REQ-005. The worker flagged
  this itself.**
  - Both source-basis cells cite the removed INIT.md.
  - CLM-005.r02 is non-aligned (CP-02 with an F3 departure), while
    UG-REQ-005 is `ALIGNED`. DEL-11-03 handles the same pattern as `ALIGNED`
    with `GAP_WORDING_CHECKED`.
  - The error runs toward non-alignment, not toward false alignment.
- **DEL-11-03:SOW#CLM-027.** The trade-offs block includes "invented
  examples" and is `ALIGNED`, assessed as a whole. It may carry the same
  CP-11 element as F-1 and should be read with it.

### Worker-raised items I judged (no disagreement)

- **DEL-11-01 A3a rows** (CLM-010.s02, CLM-014.s02, CLM-017, CLM-018.r05;
  FG-DEL-11-01-01). All four are `ACCEPTED_DIVERGENCE · OWNER`, and I
  confirm them.
  - `…/HELP-HUMAN-PIPING-20260725-CANDIDATE-BRIEFS-R18/instances/WI-PKG11-DEL1101-EXECUTION/OWNER_ADOPTION.md`
    carries a verbatim owner message between markers. I recomputed its hash:
    386 bytes, SHA-256 `1ebd357d…77acfa`, which matches the bound hash.
  - The message names "the exact four-path fence, a guide-only SOW
    exception".
  - DEC-107 (iii) separately directs edits to `docs/user_guide/index.md` and
    leaves the live `ScopeOfWork.md` untouched.
  - The rows correctly route to R4 for confirmation of reach. The guide
    predates both acts: it was created 2026-05-09 and edited 2026-06-07.
- **DEL-11-02 STATUS#remaining/R01.** `REMAINING_STATE_MISMATCH ·
  RULED_CRITERION` (CP-07) is right. DEC-079 rules the trigger and defers
  the instrument choice to activation, and intake stays closed under DEC-027.
- **DEL-11-04 STATUS#remaining/R01.** `DOCUMENTED_UNIMPLEMENTED ·
  AuthorityNeeded REVIEW` is right. It is the F2 second branch: no governing
  row carries the two pending PKG-02 dispositions.
- **DEL-11-04 embedded rule-pack snapshot.** Confirmed. In
  `fake_rule_pack_toy_model.json`, L1034–L1042 pin 0.1.0 with the checksum
  value `TBD-invented-rule-pack-checksum-not-computed`, while L943 has
  0.2.0. It is explicit, so R-DEL-11-04-005 stays `ALIGNED`.
- **DEL-11-05 INIT.md dead link.** Confirmed (see W-3).
- **CP-02 → STALE_SETUP_SPECIFICATION departures for INIT.md pointers.**
  These are correct under F3, because a file pointer is not a revision pin,
  review state or metadata. Each carries `CANONICAL_DEPARTURE:`.

### Other checks with no finding

- **CP-09 rows.** I checked all six sampled CP-09 rows (OUT-001 and VER-001) against
  `EVIDENCE_MAP.csv`. For every PKG-11 deliverable, a PASS record exists and
  `AnyPassMatchesFrozen=NO`. `STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN`
  is right.
- **Four-document residue (CP-01), PDU-055 declarations (CP-03), CS-01, CS-04,
  CS-06 and CS-07.** These conform.
- **User-guide content rows.** I read the frozen guide in full (352 lines)
  against CLM-004.r01/.r03/.r07, CLM-006.r03, UG-REQ-002, UG-REQ-009,
  CLM-012 and CLM-026. Each holds.
- **UG-REQ-008.** I checked `git diff e60ef7c34..00115c719` on the guide. It
  shows renames, deleted sentences and CAEPIPE vendor mentions replaced by
  the generic `.mbf` wording, and adds no content. This supports its
  `GAP_WORDING_CHECKED` clause.
- **DEL-11-04 PC25 rows.** These are claims about the fixtures themselves,
  so F7 is satisfied without a product caller.
- **R-DEL-11-04-004 and CLM-006.s02.** These hold in the fixture JSON.
- **Run-record validator claims.** CLM-013.r05, CLM-020.r03 and CLM-020.r04
  cite frozen records: the 2026-06-16 record shows "Result: VALID; 29
  required columns; 20 data rows". Each row says "not rerun", as A6
  requires.

## 5. Batch consistency and shared situations

- **Batch mode:** `validate_ledger_v2.py --batch` over the five forward
  ledgers returns `PASS … 0 consistency findings`, both with and without
  `--resolutions WAVES/W1/RESOLUTIONS.csv`.
- **Single mode:** run with `--notes-gap`, `--reverse <DEL>_reverse.csv` and
  `--inventory ROUTING/PKG-11_capabilities.csv`, every ledger passes with 0
  findings. The counts are 119/137/139/107/114 forward rows and 267 reverse
  rows each.
- **Shared-situation conflict the batch does not see:**
  `CONTEXT#architecture-basis-injection.s02`.
  - G1 (DEL-11-01, 02, 03) assigns `STALE_SETUP_SPECIFICATION`; G2
    (DEL-11-04, 05) assigns `STALE_REVIEW_OR_EVIDENCE`. The text is identical
    and dates from `7bee9ae41`.
  - The batch check does not flag it, because `.sNN` sub-claims are minted
    by workers and have no `BodySHA256` of their own.
  - Resolution: `STALE_SETUP_SPECIFICATION` (F-2, F-3).
  - A validator gap follows from this: minted sub-claims of a shared-body
    parent are not compared across ledgers. Agent 0 may want the batch mode
    to compare same-suffix `.sNN` keys under a shared parent.
- **`.s01` (PKG-00 at `SEMANTIC_READY`):** consistent across all five, as
  `STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT`.
- **Other keys with the same name but different dispositions:**
  `CONTEXT#anticipated-artifacts`, `MEMORY`, `STATUS#remaining` and
  `STATUS#remaining/R01`. The texts differ per deliverable, so these are not
  conflicts.

## 6. Reverse-pass findings

- **Answer distribution.** Each deliverable answered all 267 capabilities.
  - DEL-11-01, 02 and 03 each have 1 `CLAIMED_BY` and 266 `NOT_MINE`.
  - DEL-11-04 has 1 `CLAIMED_BY`, 2 `COVERS` and 264 `NOT_MINE`.
  - DEL-11-05 has 1 `CLAIMED_BY`, 2 `PARTIAL` and 264 `NOT_MINE`.
  - There are no `UNKEYED` or `CONSTRAINS` answers.
- **Every non-`NOT_MINE` answer was checked (100%, eight rows).** Each is
  right, and each cited `ClaimKey` exists in the forward ledger:
  - RC-11-0173 → DEL-11-01 `CLAIMED_BY`: the user guide.
  - RC-11-0107 → DEL-11-02 `CLAIMED_BY`: the developer guide.
  - RC-11-0091 → DEL-11-03 `CLAIMED_BY`: the theory note.
  - RC-11-0015 → DEL-11-04 `CLAIMED_BY`: `examples/models/invented/`.
  - RC-11-0176 and RC-11-0207 → DEL-11-04 `COVERS`: the `docs/_Examples`
    policy and the DEL-06-05 demo pack, which the fixtures reference without
    owning.
  - RC-11-0186 → DEL-11-05 `CLAIMED_BY`: the contributor guide.
  - RC-11-0085 (`CONTRIBUTING.md`) and RC-11-0131
    (`AGENTIC_DEVELOPMENT_WORKFLOW.md`) → DEL-11-05 `PARTIAL`. DEL-11-05
    owns the tutorial link, and it re-created the workflow map on
    2026-06-07. Contribution policy and coordination authority sit with
    governance.
- **`NOT_MINE`, 10% per deliverable (27 each, 135 in total).** I found no
  error. Where entry points do not overlap forward citations, the template
  reasons ("implementation or governance capability outside this
  DOC_UPDATE deliverable") are adequate.
- **F5, at least 20% of the `NOT_MINE` answers whose entry points hit a
  forward-cited path.**
  - Hits per deliverable are 11/10/5/11/10; I checked 3/2/1/3/2.
    - RC-11-0192 (dependency validator) in DEL-11-01, 02 and 03.
    - RC-11-0252 (docs README) in DEL-11-01 and 05.
    - RC-11-0015 (invented examples) in DEL-11-01.
    - RC-11-0127 (CONTRACT) in DEL-11-02.
    - RC-11-0264 (model schema) and RC-11-0173 (user guide) in DEL-11-04.
    - RC-11-0144 (rule-pack schema) in DEL-11-04 and 05.
  - Every reason names the specific capability and why the citation is not
    ownership. F5 is met.
- **Sampled rows compared with area rows.**
  - Of the 267 capabilities, 234 are routed `AREA` (DATA 70, SOLVER 70,
    DOCS 52, CHECKS 42) and 33 are `SAMPLE`.
  - All five deliverables answered all 33 sampled rows `NOT_MINE`. That is
    100%, against 98.7–99.6% `NOT_MINE` on the area rows.
  - None of the 33 sampled rows' entry points or tests names a path a PKG-11
    deliverable declares (the four guides, `examples/models/invented/`,
    `CONTRIBUTING.md`, `AGENTIC_DEVELOPMENT_WORKFLOW.md` and
    `_COORDINATION.md`). No sampled-row evidence check was triggered.
  - The sampled rows are shell, UI, core and physics capabilities. A PKG-11
    worker would recognize most of them from their paths as foreign (the
    brief notes this is inherent), so the comparison has little power here.
    It shows no anomaly.
- **Capabilities claimed by more than one PKG-11 deliverable:** none.
  RC-11-0015 is claimed only by DEL-11-04, and DEL-11-01 answers it
  `NOT_MINE` with a specific reason.
- **Suspected missed claims:** none. I read every DOCS-area capability's
  answers across the five deliverables (52 area rows plus the
  examples-related DATA rows). No other capability is a PKG-11 anticipated
  artifact. The validation manual, security policies and governance files
  belong elsewhere.
- **Anchored answers:** none detected. The `CLAIMED_BY` reasons cite
  register rows, front matter and commits. They do not rely on routing
  knowledge.

## 7. For the owner

1. **Four A3a rows need R4 confirmation (DEL-11-01, FG-DEL-11-01-01).** The
   R18 owner record is verbatim and hash-bound, and it grants "a guide-only
   SOW exception". The guide was edited under DEL-11-01 before that record
   (2026-05-09, 2026-06-07). The owner confirms how far the exception
   reaches, or directs a SOW catch-up.
2. **R-DEL-11-04-002 versus the claims registry (W-2).** The ledger's first
   suggested fix is to add a certification, approval, sealing and
   code-compliance non-claim to the public fixtures. That would compose the
   kind of multi-noun prohibition list that `docs/claims_registry.md` §4
   (DEC-081, DEC-107) tells agents not to compose. Yet DEC-081 Wave 2 kept
   the requirement. An owner reading is needed before anyone acts on the
   row: amend the requirement, or add the litany.
3. **Rename residue in active artifacts, beyond the SOW surface rows (out of
   sample; for R3 clustering).**
   - `projects/chirality-piping/docs/theory/centerline_analysis.md` still
     says "OpenPipeStress" 13 times. It was not renamed on 2026-09-18. It
     also pins decomposition revision 0.7 in its source table. The DEL-11-03
     SOW surface row records this.
   - `projects/chirality-piping/docs/developer_guide/index.md` names the
     DEC-022 canonical grammar identifier `open_pipe_stress_declared_expression`.
     That identifier is active in `core/rules/expression_evaluator/src/lib.rs`
     and `schemas/rule_pack.schema.yaml`.
     - Under the owner's R0 addendum, active code identifiers are residue.
     - It is not one of the four identifiers CP-04 names, and it appears
       not to be recorded in DEL-11-02's ledger: the SOW surface note says
       only "The guide already carries SWBPIPE."
     - Renaming it would change a frozen grammar identifier, so it is an R4
       code-change candidate, not a deliverable edit.
4. **A validator gap (§5).** Batch mode does not compare minted `.sNN`
   sub-claims under shared-body parents. The one such conflict in this
   package (F-2, F-3) was found only by hand.
5. **Existing R4 item, noted and not decided.** In the still-TBD rows, SCA-004
   names the export formats while AB-00-07 at revision 0.12 still says TBD.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
