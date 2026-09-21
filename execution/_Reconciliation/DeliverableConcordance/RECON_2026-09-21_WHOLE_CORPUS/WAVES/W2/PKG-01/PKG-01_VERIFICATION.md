VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-01 verification: wave W2, DOUBLE sampling

- Verifier: a fresh TASK (Type 2) instance that checks evidence only. It never
  edited a ledger. Its parent is HELP_HUMAN Agent 0 of run
  `HELP-HUMAN-PIPING-20260921-RECONCILIATION`.
- Brief: `briefs/R2-VERIFIER_brief.md`, SHA-256 `47fb3c52…2dd5b00`. The hash
  was checked before work began.
- Evidence: read from the frozen checkout at
  `00115c71931bcae79909602d653740d3bb72dfa1`.
- Conventions applied: `CONVENTIONS.md` (Parts A–F), `CANONICAL_SITUATIONS.md`
  and `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`.
- Every judgment here is an agent judgment, not an owner ruling.

Ledgers verified. Each recomputed forward SHA-256 matches its SEAL file.

| Deliverable | Forward SHA-256 | Reverse SHA-256 | Rows (forward/reverse) |
|---|---|---|---|
| DEL-01-01 (ISSUED) | `993746f5c2248429fe1c646771f72a22af4bc8750e994be60755edc0e2432ab6` | `103fa03cb727f10bc00a06856205b28df068270dbc45dedb7576f37748c73c08` | 116 / 199 |
| DEL-01-02 | `b9ad8f8a2493df0e1106b79275f5101b2eac46dc24fee65213a5e628222899fd` | `de000f604fc328704679f2f1e80fcc93e2f5c7c4bf760d9f165f42b569122f32` | 72 / 199 |
| DEL-01-03 | `896bcdd0c5dbf30951ac97806814e5aafd9b5f639553a15eecee1aa93f79dfbe` | `173cf5428ebfa071d74539a48dbe92a7f8884dd5c9239744e99bd63a7654eff1` | 110 / 199 |
| DEL-01-04 | `2d4991ec0229f24281ff7facf62793d5a447dd70959edf72686b0b4ce7e13f5b` | `8fc612dad4f350b3de0f345523cfb08eeee8847eaf6723f4a79faf2ce3bf90e0` | 61 / 199 |

## 1. Sample selection

Each row falls into the first class that fits, in the order below. Within a
class, candidate `ClaimKey` strings were sorted by their SHA-256 and the lowest
fraction was taken.

For `ALIGNED` normative rows, rows with `LOW`/`MEDIUM` confidence, `NONE_FOUND`
verification or a `NONE` verification class were taken first. Almost every
aligned row in this package has `VerificationClass NONE`, so in practice the
hash order decided.

The brief's `DOUBLE` doubling applies only to rates marked \*. The reverse
rates (10% `NOT_MINE`, 20% F5) are not marked, so they were not doubled.
Coverage was widened anyway: 100% of F5 candidates were checked.

| Class | Rate used |
|---|---|
| A. `ACCEPTED_DIVERGENCE`, `AUTHORITY_CONFLICT`, `UNKNOWN`, `LIFECYCLE_REASSESSMENT_REQUIRED`, `INVARIANT` tier, `PROTECTED_CHECK`/`FROZEN_CONTRACT` | 100% |
| B. Every row of an ISSUED deliverable (DEL-01-01) | 100% |
| C. Unit with `SharedTextCount > 1` | 100% |
| D. `ALIGNED` with `GAP_WORDING_CHECKED:` or `OPEN_ACTION:` | 100% (none outside DEL-01-01) |
| E. `ALIGNED` with `PRODUCT_CALLER: NONE` | 50% (none in the package) |
| F. Other non-aligned rows | 50% |
| G. `ALIGNED` normative rows (`REQUIREMENT`/`ACCEPTANCE`/`EXCLUSION`) | 40% |
| H. Structural rows and inherited canonical (CS) rows | 20% |

Sampled keys for classes C, F, G and H. Classes A and B cover every DEL-01-01
row. The DEL-01-03 class A row is `STATUS#remaining/R02`.

- **DEL-01-02**
  - C: `CONTEXT#architecture-basis-injection`, `CONTEXT#objective-support`,
    `CONTEXT#decomposition-reference`, `CONTEXT#package-reference`,
    `STATUS#remaining`.
  - F: `SOW#CLM-012.s01`, `CONTEXT`, `SOW#CLM-023`,
    `CONTEXT#architecture-basis-injection.s01`, `SOW#CLM-012.s02`,
    `SOW#CLM-002`, `SOW#CLM-017`, `SOW#CLM-010.r11`, `SOW#CLM-010.r03`,
    `SOW#CLM-016`, `…/VER-001`, `…/AC-001`.
  - G: `SOW#CLM-009`, `SOW#CLM-010.r10`, `.r09`, `SOW#CLM-022`,
    `SOW#CLM-010.r02`, `.r08`.
  - H: `SOW#CLM-010`, `SOW#output-and-evaluation-matrix`, `SOW#CLM-026`.
- **DEL-01-03**
  - C: `CONTEXT#package-reference`, `SOW#CLM-030`, `SOW#CLM-005`,
    `CONTEXT#objective-support`, `CONTEXT#decomposition-reference`.
  - F: `CONTEXT#architecture-basis-injection.s01`, `SOW#CLM-012/…-REQ-04`,
    `CONTEXT`, `CONTEXT#architecture-basis-injection.s02`, `SOW#CLM-022.r04`,
    `SOW#CLM-017`, `STATUS#remaining/R01`, `SOW#CLM-021`,
    `SOW#CLM-012/…-REQ-08`, `SOW#CLM-020.s01`, `…/AC-001`,
    `CONTEXT#architecture-basis-injection`, `STATUS`, `SOW#CLM-003.r04`,
    `SOW#CLM-006.r11`.
  - G: `SOW#CLM-006.r06`, `.r12`, `SOW#CLM-004`, `SOW#CLM-006.r10`,
    `SOW#CLM-011`, `SOW#CLM-006.r03`, `SOW#CLM-022.r02`, `SOW#CLM-029`,
    `SOW#CLM-012/…-REQ-02`, `SOW#CLM-006.r05`, `.r04`, `SOW#CLM-022.r05`,
    `SOW#CLM-006.r09`, `SOW#CLM-012/…-REQ-06`,
    `SOW#purpose-and-objective-traceability/OUT-001`.
  - H: `SOW#CLM-025`, `SOW#output-and-evaluation-matrix`, `SOW#CLM-006`,
    `SOW#completion-and-reliance-basis-epistemology`.
- **DEL-01-04**
  - C: `CONTEXT#package-reference`, `STATUS#remaining`,
    `CONTEXT#preparation-notes`.
  - F: `CONTEXT#architecture-basis-injection.s01`,
    `CONTEXT#decomposition-reference`, `SOW#CLM-006`, `SOW`, `SOW#CLM-021`,
    `SOW#CLM-002`, `SOW#CLM-017`, `CONTEXT#architecture-basis-injection.s02`,
    `…/AC-001`.
  - G: `SOW#CLM-019`, `SOW#CLM-009`, `SOW#CLM-024`,
    `SOW#purpose-and-objective-traceability/OUT-001`.
  - H: `CONTEXT#objective-support`, `SOW#governing-values-and-decisions-axiology`,
    `CONTEXT#scope-coverage`.

The verifier also read every other row of all four ledgers. Where that turned
up something outside the sample, it is reported in §4 as out of sample and
kept out of the rates.

## 2. Per-deliverable results

| Deliverable | Rows | Sampled per class | Sampled total | Firm | Weak | Field | Sampled `ALIGNED` normative | Firm false alignments | Firm false-alignment rate |
|---|---|---|---|---|---|---|---|---|---|
| DEL-01-01 | 116 | A 18 · B 98 | 116 | 0 | 3 | 2 | 25 | 0 | 0.0% |
| DEL-01-02 | 72 | C 5 · F 12 · G 6 · H 3 | 26 | 0 | 0 | 1 | 6 | 0 | 0.0% |
| DEL-01-03 | 110 | A 1 · C 5 · F 15 · G 15 · H 4 | 40 | 0 | 0 | 2 | 15 | 0 | 0.0% |
| DEL-01-04 | 61 | C 3 · F 9 · G 4 · H 3 | 19 | 0 | 0 | 0 | 4 | 0 | 0.0% |

- Firm error rate on sampled rows is 0% for every deliverable, so no rerun is
  triggered.
- There is no firm error on any 100%-sampled class.

## 3. Package-level firm false-alignment rate

**0.0%**: 0 of 50 sampled `ALIGNED` normative rows. This is the scale-out gate
input; the gate is 5% or less.

## 4. Disagreements

### Weak disagreements (DEL-01-01; all 100%-sampled rows)

**W1. `DEL-01-01:SOW#CLM-003` (Attributes)**
- **Row:** `ALIGNED`. The "Product stance: source-available noncommercial" row
  matches SOW-001 and DIRECTIVE.
- **Found:** the same ledger disposes the product-posture sentence in
  `CLM-009.s01` as `AUTHORITY_CONFLICT`, because governing sources disagree:
  - `docs/PRD.md` L24 says "free and open-source";
  - `docs/DIRECTIVE.md` L73 (§4.1, not §6 as the SOW cites) and the PolyForm
    Noncommercial `LICENSE.md` say source-available noncommercial.
  - `CLM-003` states one side of that recorded conflict.
- **Right values:** either `AUTHORITY_CONFLICT · AUTHORITY_UNCLEAR ·
  PROJECT_BASELINE · ISSUED · RECORD · OWNER`, sharing the `CLM-009.s01`
  finding, or `ALIGNED` if the owner treats the PRD sentence as the stale
  side.
- **Why weak:** the conventions do not say whether a claim that matches one
  side of a recorded authority conflict is `ALIGNED`. Resolve it with the
  `CLM-009.s01` ruling.

**W2. `DEL-01-01:SOW` (SURFACE row)**
- **Row:** `LIFECYCLE_REASSESSMENT_REQUIRED · RENAME_OR_IDENTITY ·
  LOCAL_DESIGN · ISSUED · RECORD`, in `FG-DEL-01-01-02`, with
  `CANONICAL_DEPARTURE` from CP-04 under C6(d).
- **Found:**
  - The departure is written, and the literal C6(d) reading supports it.
  - The same ISSUED SOW keeps its revision and DAG pins (0.7, DAG-006) under
    CP-02 `STALE_REVIEW_OR_EVIDENCE`. Those pins are equally "true at issuance
    and overtaken later", and there the worker departed only on
    `BaselineClass` and `AuthorityNeeded`. The two classes of ISSUED residue
    are treated inconsistently.
  - The row now routes to R4 twice: on the ISSUED change path (`FG-02`) and in
    the rename class.
  - Batch mode cannot detect this, because CP pattern groups are keyed by
    disposition.
- **Right values:** either CP-04 as written with `BaselineClass=ISSUED`, or
  C6(d) applied to the ISSUED pins as well.
- **Why weak:** the conventions do not rank C6(d) against CP-02 and CP-04 for
  ISSUED text.

**W3. `DEL-01-01:SOW#CLM-010/REQ-01-01-05`**
- **Row:** `PARTIALLY_IMPLEMENTED · RECORD_DRIFT · LOCAL_DESIGN`.
- **Found:**
  - The requirement is that authority is recorded in public governance
    artifacts before it is treated as policy.
  - DEC-027 is recorded in `SOFTWARE_DECOMP.md` L618 and cited by
    `RELEASE_QUALITY_GATES.md` L183. The row's own Notes say that "no
    unrecorded authority is treated as policy".
  - `governance/MAINTAINERS.md` L35–L36 and L128 still show TBD, but that is a
    record lag against a *different* criterion. `AC-01-01-04` carries it
    correctly.
- **Right values:** `ALIGNED`, with the MAINTAINERS lag left on `AC-01-01-04`.
- **Why weak:** the row over-reports (conservative). This is not a false
  alignment.

### Field disagreements

**F-a. `DEL-01-01:SOW#CLM-004.r05`** (sampled; `LIFECYCLE_REASSESSMENT_REQUIRED`, MEDIUM)
- **Field:** `DecisionBasis`.
- **Row:** `DEC-027; DEC-057; DEC-089`.
- **Found:** those rulings do not bear on maturity labels or validation
  disclosure. The overtaking rulings are:
  - DEC-081 (BS-VALID, evidence-status labels), `SOFTWARE_DECOMP.md` L672;
  - DEC-105 (BS-MATURITY retired).
- R0 ruling item 4 named only roster, quorum, release authority and signing
  for the lifecycle-reassessment group. Putting this slot into
  `FG-DEL-01-01-01` extends the owner's group.
- **Right values:** `DecisionBasis` DEC-081 and DEC-105; either a separate
  `FindingGroup`, or FG-01 with the extension disclosed to the owner. The
  disposition stands.

**F-b. `DEL-01-01:SOW#CLM-014`** (sampled; `LIFECYCLE_REASSESSMENT_REQUIRED`)
- **Field:** `DecisionBasis`.
- **Row:** the same template, `DEC-027; DEC-057; DEC-089`.
- **Found:** the Notes rightly name DEC-081 BS-VALID for validation wording,
  but the field omits it.
- **Right values:** add DEC-081. The disposition stands.

**F-c. `DEL-01-02:SOW#CLM-010.r03`** (sampled; `STALE_SETUP_SPECIFICATION`)
- **Field:** `CauseTag`.
- **Row:** `DOC_BEHIND_CODE`.
- **Found:** the thing that moved on is a policy document
  (`docs/IP_AND_DATA_BOUNDARY.md` L80, the checklist's §8 default quarantine
  path), not code.
- **Right values:** `RECORD_DRIFT`. The disposition, tier and layer stand.

**F-d. `DEL-01-03:SOW#CLM-006.r10` and `.r12`** (sampled; `ALIGNED`)
- **Field:** the anchors in `ImplementationEvidence`.
- **Row:** `CONTRIBUTOR_CERTIFICATION_TEMPLATE.md#L103` and `#L104`.
- **Found:** those lines are the table header. The fields sit at L107
  ("Review disposition") and L108 ("Disposition rationale"). `.r11`, not
  sampled, is likewise cited at L102 where the field is at L106.
- **Right values:** the corrected anchors. The disposition is correct: the
  fields exist.

### Out-of-sample observations (not counted)

**DEL-01-02:SOW#CLM-011**
- **Field:** `CauseTag`.
- **Row:** `DOC_BEHIND_CODE`.
- **Found:** the license-TBD element in this row is the same situation as
  DEL-01-03 `CLM-003.r04`, which uses `SCOPE_REDIRECTED_BY_RULING`
  (2026-06-03 human license ruling). The quarantine-path element fits
  `RECORD_DRIFT`.
- **Suggested:** `SCOPE_REDIRECTED_BY_RULING` or `RECORD_DRIFT`.

**DEL-01-02:SOW#CLM-026/C-001**
- **Row:** `ALIGNED` ("no … owner is assigned; ruling TBD").
- **Found:** the same ledger treats DEC-027 and DEC-079 as recording the
  reviewer role (`C-003`, `STALE_REVIEW_OR_EVIDENCE`). By the same reading,
  DEC-027 assigns the escalation owner (the sole maintainer).
- **Status:** weak. The row is defensible, since the checklist keeps
  escalation owner as a per-record field.

### Checks that held

These were verified against the freeze and found correct:

- **`DEL-01-01:CLM-009.s01` (`AUTHORITY_CONFLICT`).** PRD L24 against
  DIRECTIVE L73 and `LICENSE.md` is confirmed. Project `README.md` L10 says
  "free and open-source", while `docs/README.md` L10 says "source-available
  noncommercial".
- **`DEL-01-01:…/AC-001` (`ACCEPTED_DIVERGENCE`, DEC-081).**
  - `git log` on the SOW shows only `3c7f6abfb` (conversion; SHA-256
    `23d92dde…`, the production that `RECON-I0-PKG01` PASS binds) and
    `8fac6631a` (D-48 Wave 2).
  - The diff is exactly 6 lines.
  - DEC-081 Wave 2 scopes "the 45 live `ScopeOfWork.md` litany occurrences →
    GF-TOKEN".
- **`OUT-001` and `VER-001` under CP-09.**
  - DEL-01-01: `EVIDENCE_MAP` lists `NONE_FOUND`; the worker ran the A3
    discovery step correctly.
  - DEL-01-02, 01-03 and 01-04: `AnyPassMatchesFrozen=NO` is confirmed.
- **FG-01 TBD rows.**
  - DEC-027 (L618), DEC-057 (L648), DEC-089 (L680) and DEC-079 (L670) say
    what the rows claim.
  - `CLM-005.r06` (legal review TBD) is correctly `ALIGNED`: DEC-079 keeps the
    legal instrument open.
- **`AC-01-01-05` (`PARTIALLY_IMPLEMENTED`).** The ISSUED closeout run record
  lists the edited files but states no protected-content outcome.
- **The F3 origin claims tested with `git log -S`.** Step 3's four-document
  naming is present at `7bee9ae41`. The TBD conditions were first added at
  `8b8d1781d`.
- **DEL-01-02 `CLM-010.r05` (`IMPLEMENTED_DIFFERENTLY · OTHER`).**
  - The checklist spells the value `unknown`.
  - `docs/TYPES.md` L129 defines `UNKNOWN_SOURCE`.
  - The row's reading is defensible.
- **DEL-01-02, 01-03 and 01-04 authority-block lags.** The artifacts
  (IP policy §1.1, checklist, `CONTRIBUTING.md` L36–L48, template, PB §2)
  still list roster, quorum and release authority as TBD after DEC-027, and
  pin 0.7 / DAG-007.
- **DEL-01-03 `REQ-08`.** `CONTRIBUTING.md` L18 ("accepts contributions only
  within the project boundaries") never says that intake is closed.
- **DEL-01-03 `STATUS#remaining/R02` (`UNKNOWN · AUTHORITY_UNCLEAR`).** PRD
  v0.4 has no legal-review requirement; its §17.5 is now "Missing Data
  Behavior".
- **DEL-01-03 `R03` (`DOCUMENTED_UNIMPLEMENTED`).** The export tool explicitly
  excludes `.github` (`tools/release/export_public_openpipestress.py` L133,
  L146).
- **DEL-01-03 `R01` (CP-07).** D-07b was ruled as DEC-079.
- **DEL-01-02 `CLM-012.s03` and DEL-01-04 `CLM-012`.** The GATE record
  `PR834_CI/harness-run.json` shows the self-check step as `success`, and the
  rows say "not rerun" (A6 held).
- **DEL-01-04 `CLM-012`.** `report_notice_template.md` quotes the PRD §19.3
  composite and names its change path.
- **DEL-01-04 `CLM-004` and `CLM-010` R05.** `analysis_status.schema.yaml`
  carries `bound_hashes` and invalidation.

No row uses context as authority. No `ACCEPTED_DIVERGENCE` rests on a merged
PR. No aligned-by-construction row was found (C6(a)). No row takes
`INVARIANT` or `PROTECTED_CHECK`, and F8 was applied consistently, with each
boundary named in Notes.

## 5. Batch consistency

**Validator runs in this session** (`PYTHONDONTWRITEBYTECODE=1`):
- Single mode with `--reverse`, `--inventory ROUTING/PKG-01_capabilities.csv`
  and `--notes-gap` passed for all four ledgers with 0 findings.
- `--batch` over the four PKG-01 forward ledgers passed with 0 consistency
  findings.
- A wider batch over all 44 sealed W1 and W2 forward ledgers, with
  `WAVES/W1/RESOLUTIONS.csv`, found 8 findings. None involves a PKG-01 row:
  - CP-04 tier in PKG-00 and PKG-02;
  - CP-03 cause in DEL-04-01;
  - CP-11 in DEL-00-07.

**Shared-body rows (100%).** These were consistent within PKG-01:
- the `decomposition-reference` (CS-01), `package-reference` (CS-06 DRIFT) and
  `architecture-basis-injection` (CS-04) bodies;
- the empty `STATUS#remaining` rows of DEL-01-02 and DEL-01-04, both
  `NOT_ASSESSED`;
- `CONTEXT#preparation-notes` (CS-02, `ALIGNED`);
- the heading-only `CLM-005` and `CLM-030` rows of DEL-01-03.

**Shared-situation conflict.** There is none mechanically. There is one
judgment-level inconsistency, W2: in the ISSUED SOW, rename residue went to
C6(d) while the pins stayed on CP-02. Batch mode cannot see it, because CP
groups are split by disposition.

## 6. Reverse pass

**Checked:**
- 100% of the 12 `CLAIMED_BY`, `PARTIAL` and `UNKEYED` answers (8, 3 and 1),
  plus all 12 `COVERS` answers.
- F5 on 100% of the 30 `NOT_MINE` answers whose `EntryPoints` hit a path the
  deliverable's forward ledger cites (7, 6, 9 and 8 per deliverable).
- A 10% hash-ordered sample of `NOT_MINE` answers: 78 of 772.

All agree:
- Every `CLAIMED_BY` matches a `deliverable_id` in the artifact's frontmatter
  (`CONTRIBUTING.md` and the certification template for DEL-01-03;
  `PROFESSIONAL_BOUNDARY.md` and `report_notice_template.md` for DEL-01-04) or
  the anticipated artifacts in `Deliverables.csv` (`MAINTAINERS.md`;
  `IP_AND_DATA_BOUNDARY.md` and the checklist).
- `LICENSE.md` is claimed by DEL-01-01 on `REQ-01-01-01`, which is defensible.
- Every F5 reason addresses its own capability.
- No sampled `NOT_MINE` answer names a PKG-01 artifact.

**Capabilities claimed by more than one PKG-01 deliverable:** none with
overlapping `CLAIMED_BY` or `PARTIAL`. The overlaps are coverage only:
- `RC-01-0001` `CONTRIBUTING.md`: 01-03 claims it, 01-02 covers it.
- `RC-01-0061` certification template: 01-03 claims it; 01-01 and 01-02 cover
  it.
- `RC-01-0135` checklist: 01-02 claims it; 01-01 and 01-03 cover it.
- `RC-01-0065` IP policy: 01-02 claims it, 01-03 covers it.
- `RC-01-0034` notice template: 01-04 claims it, 01-02 covers it.
- `RC-01-0031` `RELEASE_NOTES_TEMPLATE.md`: 01-01 and 01-04 both cover it and
  no PKG-01 deliverable owns it (a release deliverable's).

**Suspected missed claims:** no ownership misses. For R3, these are possible
`COVERS` relations:
- `RC-01-0192` (`docs/_Examples/` example-data policy): relates to DEL-01-02
  R6. It was answered `NOT_MINE` with a specific reason.
- `RC-01-0184` (`docs/architecture/code_neutral_analysis_boundary.md`,
  "prohibited public content"): all four answered with the template reason.
- `RC-01-0071` (`docs/contributor_guide/index.md`): DEL-01-03 gave a specific
  reason.

**UNKEYED:** `RC-01-0123` `.github/ISSUE_TEMPLATE/`. It was produced under
DEL-01-03's `TP-E6-ISSUETEMPLATES-001`, but no requirement key specifies it.
R3 owns the final unmapped set.

**Sampled against area rows:**

| Routing | Rows | `NOT_MINE` |
|---|---|---|
| `SAMPLE` | 35 × 4 = 140 | 140 (100%) |
| `AREA` | 164 × 4 = 656 | 636 (97.0%) |

- All 20 non-`NOT_MINE` answers are DOCS-area governance files.
- No `SAMPLE` row names a path the package's deliverables declare. One names
  an adjacent path: `RC-01-0038`, library-import provenance enforcement.
  DEL-01-02 and DEL-01-03 answered it with capability-specific `NOT_MINE`
  reasons, which is correct.
- The distribution shows no sign of anchoring.
- No answer is anchored on a `STATUS#remaining` key.

## 7. For the owner, stated plainly

1. **Product posture conflict (DEL-01-01 `CLM-009.s01`), verified.** Two
   governing sources disagree:
   - PRD v0.4 §1 (L24) calls SWBPIPE "free and open-source".
   - DIRECTIVE §4.1 (L73), ScopeLedger SOW-001 and the selected PolyForm
     Noncommercial license say source-available noncommercial.
   - Project `README.md` L10 repeats "free and open-source", while
     `docs/README.md` L10 says noncommercial.

   This needs your ruling. It also settles W1.
2. **The DEC-081 edit to the ISSUED SOW (DEL-01-01 `AC-001`) is recorded as
   an accepted divergence and needs your confirmation.** Commit `8fac6631a`
   changed 6 lines after the conversion parity PASS. DEC-081 authorizes the
   Wave 2 litany edits, but C6(d) points to lifecycle reassessment for ISSUED
   text overtaken by a later ruling. Please confirm that DEC-081 reached this
   ISSUED SOW.
3. **The ISSUED lifecycle-reassessment group `FG-DEL-01-01-01` (15 rows) goes
   beyond what your R0 item 4 named.** It adds maturity labels and
   validation-disclosure wording (`CLM-004.r05`, `CLM-014`), which DEC-081 and
   DEC-105 overtook, not DEC-027 or DEC-057. Those rows cite the wrong rulings
   (F-a, F-b).
4. **Rename residue in the ISSUED SOW is routed twice**: as an ISSUED
   lifecycle item (`FG-DEL-01-01-02`) and in the rename class. The ISSUED
   revision pins, meanwhile, stay ordinary stale pointers (W2). The
   conventions do not settle which rule wins for ISSUED text.
5. **The repository governance files lag DEC-027 and DEC-079.**
   - `MAINTAINERS.md`, `CONTRIBUTING.md`, `IP_AND_DATA_BOUNDARY.md`, the
     review checklist, the certification template and
     `PROFESSIONAL_BOUNDARY.md` still show roster, quorum and release
     authority as TBD, and pin revision 0.7 / DAG-007.
   - `CONTRIBUTING.md` reads as open for contributions and never says that
     external intake is closed.
   - These are recorded as record-lag findings (`LOCAL_DESIGN`, no decision
     needed).
6. **One item needs you to name a basis (DEL-01-03 Remaining R02).** It asks
   for a "§17.5 pre-release legal review", but that requirement exists only in
   the superseded PRD v0.1. The row is `UNKNOWN · AUTHORITY_UNCLEAR`.
7. **Process notes**, none affecting the verdict:
   - The worker left a notebook, `WAVES/W2/PKG-01/_WORKER_DEL-01-01_NOTES.md`,
     outside its named output files.
   - Because the deliverable folder paths contain spaces, which the validator
     rejects in evidence columns, deliverable-local evidence (SOW, `_STATUS`
     and run records) appears only in `ContextRefs`.
   - The manager ran the validator with `--repo-root .`. That is equivalent,
     because the validator resolves paths as git objects at the frozen commit.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
