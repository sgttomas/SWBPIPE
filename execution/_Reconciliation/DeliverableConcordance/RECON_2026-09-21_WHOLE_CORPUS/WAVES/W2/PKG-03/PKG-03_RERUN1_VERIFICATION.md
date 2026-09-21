VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-03 rerun 1 verification — DEL-03-07 only (W2)

- Verifier: fresh, evidence-only TASK. Parent: HELP_HUMAN Agent 0 of run
  `HELP-HUMAN-PIPING-20260921-RECONCILIATION`. I am independent of both PKG-03
  managers, their workers and the first PKG-03 verifier. I did not read
  `PKG-03_VERIFICATION.md`, the `superseded_1/` files or the July 2026 run's
  ledgers.
- Brief: `briefs/R2-VERIFIER_brief.md`. Its SHA-256 matched the launch value
  (`47fb3c52…5b00`).
- Values: `{SAMPLING}=DOUBLE`. Evidence was read from the freeze checkout at
  `00115c71931bcae79909602d653740d3bb72dfa1`, and the checkout's HEAD was
  confirmed.
- Scope: `WAVES/W2/PKG-03/DEL-03-07/` with these files:
  - `DEL-03-07_forward.csv`, SHA-256 `a549f6f8…3e0`, which equals the SEAL
    hash;
  - `DEL-03-07_reverse.csv`, SHA-256 `428c474a…3a1`;
  - `DEL-03-07_notes.md`, SHA-256 `cf153897…2f2`.
- DEL-03-07 is `IN_PROGRESS`, not ISSUED, so the ISSUED 100% class does not
  apply.

## 1. Mechanical checks

| Check | Result |
|---|---|
| Single mode, `--notes-gap`, with `--reverse` and `--inventory ROUTING/PKG-03_capabilities.csv`, repo root = freeze | `PASS DEL-03-07: 97 forward rows, 53 required keys, 7 canonical, 0 findings` |
| The same run with repo root = the working checkout | PASS, identical |
| `--batch` over all eight PKG-03 forward ledgers, using the fresh DEL-03-07 ledger | `PASS batch of 8 ledgers: 0 consistency findings`. No `WAVES/W2/RESOLUTIONS.csv` exists, so none was passed |
| Keyed CS rows | 7 of 7 inherit `CANONICAL_ASSIGNMENTS.csv` values. There are no `CANONICAL_DEPARTURE`s |
| Cross-package shared bodies (my own check over every W1/W2 ledger) | All 8 shared-text keys agree with every sibling ledger row found |

`--batch` has a blind spot: it cannot compare `.sNN` sub-claims, because they
carry no `BodySHA256`. That is where the one shared-situation conflict below
sits (§5).

## 2. Sampling (deterministic)

Every row was assigned to its highest-rate class, with rates doubled under
`DOUBLE`. In each class, candidate keys were sorted by the SHA-256 of the
`ClaimKey` and the lowest `ceil(n × rate)` were taken.

| Class | Rate | n | Sampled | Keys sampled |
|---|---|---|---|---|
| INVARIANT tier (no AD/AC/UNKNOWN/LRR/PROTECTED_CHECK/FROZEN_CONTRACT rows exist) | 100% | 3 | 3 | CLM-009.r05, CLM-003.r06, CLM-021.s02 |
| `SharedTextCount > 1` | 100% | 8 | 8 | CLM-002.r04, CONTEXT#decomposition-reference, CONTEXT#package-reference, STATUS#remaining, CLM-002.r02, CONTEXT#preparation-notes, CLM-002.r08, CONTEXT#architecture-basis-injection |
| ALIGNED with `GAP_WORDING_CHECKED:`/`OPEN_ACTION:` | 100% | 9 | 9 | AC-001, CONTEXT#description, CLM-026.r04, CLM-005.s01, CLM-017.r06, CLM-026.r03, CLM-009.r02, CLM-021.s01, CLM-026.r02 |
| ALIGNED with `PRODUCT_CALLER: NONE` | 50% | 2 | 1 | CLM-012.s01 |
| Other non-aligned | 50% | 11 | 6 | CLM-017.r07, CLM-012.s02, CLM-005.s02, CONTEXT#architecture-basis-injection.s01, CLM-015, CLM-026.r01 |
| ALIGNED normative | 40% | 21 | 9 | CLM-003.r01, CLM-017.r01, output-and-evaluation-matrix/OUT-001, CLM-009.r04, CLM-009.r03, CLM-017.r04, CLM-004, CLM-017.r02, CLM-009.r06 |
| Structural and inherited canonical | 20% | 21 | 5 | CLM-001, CLM-003, CONTEXT#scope-coverage, CLM-019, CONTEXT#objective-support |
| Other quiet declared-state/context/history | — | 22 | 0 | — |

The sample is 41 of 97 rows.

I also checked the following rows outside the drawn sample. They are not
counted in the rates below:

- the three rows the worker self-flagged under F7: CLM-002.r09, CLM-011 and
  CONTEXT#context-envelope;
- the SOW and MEMORY SURFACE rows, because the owner items touch them.

## 3. Results table — DEL-03-07

| Rows | Sampled | Firm | Weak | Field | Sampled ALIGNED normative | Firm false alignments | **Firm false-alignment rate** |
|---|---|---|---|---|---|---|---|
| 97 | 41 (3 / 8 / 9 / 1 / 6 / 9 / 5 by class above) | 0 | 3 | 1 in sample, plus 3 confirmed outside it | 14 (9 from the normative class, plus 5 normative rows in the gap-wording class) | 0 | **0.0%** |

- The firm error rate on sampled rows is 0%.
- No 100%-class row has a firm error.

**Package-level firm false-alignment rate (scale-out gate input): 0.0%**
(0 of 14). The gate is ≤ 5%.

The package-level rate covers DEL-03-07 only. The other seven deliverables
belong to the first verifier's report.

## 4. Disagreements

### Weak

**W1. `DEL-03-07:SOW#CLM-026.r01`**

- **The row says:** `IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · PROJECT_BASELINE · NONE · RECORD · OWNER`, CP-10. Its Notes say "No ruling in the register or SOFTWARE_DECOMP section 12 approves these names".
- **What I found:**
  - The vocabulary is not settled by code alone.
    `projects/chirality-piping/docs/IP_AND_DATA_BOUNDARY.md` §4 (lines 60–71)
    defines it. The run's authority map lists that file as a governing
    source.
  - §4 defines `redistribution_status` as `public_permissive`,
    `private_only`, `unknown`, `protected_suspected`.
  - §4 defines `review_status` as `pending`, `accepted`, `rejected`,
    `quarantined`.
  - That table has been present since `7bee9ae41`, the same commit that
    introduced the C1 text (checked with `git log -S`).
  - The code extends the policy. `schemas/material.schema.yaml`
    `RedistributionStatus` (l.777–786) adds `rejected` and `TBD`, and
    `ReviewStatus` (l.817–825) adds `TBD`. The checker gates on the
    `rejected` redistribution value (`core/library_import/provenance_checker.py`
    l.219).
  - The IP policy file is marked `status: draft`.
  - The worker knew of the file: reverse answer RC-03-0267 calls it "a
    governing boundary document DEL-03-07 relies on". It is not cited on this
    row.
- **Why weak rather than firm:** the result depends on whether a draft
  governing policy's field table counts as "approved enum names". The
  conventions should settle that.
- **Right values:**
  - If the policy counts, CP-10 does not apply. The row becomes
    `STALE_SETUP_SPECIFICATION` (origin `7bee9ae41`, F3) · `RECORD_DRIFT` ·
    `LOCAL_DESIGN` (F8; boundary BS-IP named in Notes) · `NONE` · `RECORD` ·
    `NO`.
  - The schema's extension values (`rejected`/`TBD`) would then be a separate
    small item for R3.
  - If it does not count, the row stands. Its Notes should still cite §4 as
    context.
  - Either way, **R3 should add the IP_AND_DATA_BOUNDARY §4 citation**
    before the owner sees this item.

**W2. `DEL-03-07:SOW#CLM-004`**

- **The row says:** `ALIGNED`, confidence MEDIUM. It argues that the condition
  "Missing solve-required or rule-check-required values are findings, not
  silent defaults" holds, because the checker adds no values and CONTRACT
  places OPS-K-DATA-2 enforcement in the solver and rule engine.
- **What I found:**
  - The checker emits no finding for an imported record that lacks a
    solve-required value. `provenance_checker.py` has no completeness logic,
    and neither does the Rust port.
  - The store gate would save such a record.
  - The row cites no solver or rule-engine evidence for the downstream
    enforcement it relies on.
  - `docs/CONTRACT.md` l.27 does map enforcement to "GUI warnings;
    solver/rule engine validation", so the reading is defensible.
- **Right values:** it stays `ALIGNED` if restating a system invariant in a
  Conditions block is met by another component's enforcement. The row should
  then cite that enforcement. Otherwise it becomes `PARTIALLY_IMPLEMENTED` ·
  `OWNERSHIP_ELSEWHERE` · `INVARIANT` (OPS-K-DATA-2). The conventions should
  settle which (compare the F7 subject rule).

**W3. `DEL-03-07:CONTEXT#architecture-basis-injection.s01`**

This is a shared-situation conflict (see §5). DEL-03-07's own values are
defensible: F3 metadata exception, matching the W1 DEL-07-0x rows.

### Field

**X1. `DEL-03-07:SOW#CLM-017.r06`** (sampled)

- The claim's expected evidence is a fixture review. The row rests on the
  worker's own reading of the two fixtures and on `MEMORY.md`.
- It records `VerificationEvidence NOT_APPLICABLE` and `VerificationClass
  NONE`.
- Right fields: `VerificationClass DOCUMENT_REVIEW`, citing the fixtures and
  the MEMORY record. The disposition is unaffected.

**X2–X4. `SOW#CLM-002.r09`, `SOW#CLM-011`, `CONTEXT#context-envelope`**
(outside the sample; the worker self-disclosed them)

- Confirmed. Each row's implementation evidence names only
  `core/library_import/provenance_checker.py` (plus a register), which has
  test callers only. None carries `PRODUCT_CALLER: NONE` (F7).
- The dispositions stand: each claim is about the checker, its tests or a
  record.
- Right field: add `PRODUCT_CALLER: NONE` to Notes.

### Rows confirmed (selected reasoning)

**FG-DEL-03-07-01** (CLM-003.r06, CLM-009.r05, CLM-021.s02): confirmed
`PARTIALLY_IMPLEMENTED` · INVARIANT.

- `_validate_nested_values` returns early unless `"magnitude" in value`
  (`provenance_checker.py` l.296–321). The Rust `validate_nested_values` does
  the same (`library_import_document/src/lib.rs` l.425–489).
- Material, section and component kinds get no schema check. Only `hanger`
  does (l.107).
- The product path is the Tauri `validate_library_import` command and
  `save_local_library`, which is re-validated through
  `library_import_validation_value` in `apps/desktop/src-tauri/src/lib.rs`
  l.3376–3414 and 3676–3690. So a record carrying a bare JSON number is
  accepted and stored with no unit finding.
- The frozen fixtures carry no bare numerics; I checked both. No test
  exercises the case.
- Other fields:
  - `POSSIBLE_DEFECT` and `PARTIAL_SLICE` both fit C7. This is not an error.
  - Layer `BASELINE` is a convention gap: C5 has no units layer. R3 should
    settle one tag and one layer for the cluster.
- CLM-017.r04 and CLM-016 staying `ALIGNED` is consistent: they describe test
  evidence and a method that exist.

**FG-DEL-03-07-02** (CLM-005.s02, CLM-012.s02, CLM-017.r07): confirmed.

- `Review_Findings.csv` shows both findings `ACCEPT_AS_IS` / `RESOLVED`.
- `_STATUS.md` history records the 2026-06-05 gate.
- The text originates at `1b62eb5b8` (2026-06-04), which is after the initial
  migration, so `STALE_REVIEW_OR_EVIDENCE` is correct under F3 and under the
  review-state exception.

**Other rows confirmed:**

- FG-03 pins (CP-02).
- The OUT-001 matrix row under CP-09: the frozen SOW SHA-256 is `f87b738d…`,
  and `EVIDENCE_MAP` shows `AnyPassMatchesFrozen=YES`.
- The CLM-026.r02–r04 open-ruling rows. Their `GAP_WORDING_CHECKED` clauses
  are sound.
- The MEMORY SURFACE row: the "UI/editor presentation … future GUI work" item
  (MEMORY l.26) is first present at `7bee9ae41`, and `LibraryManagerPanel`
  shows import findings.
- The CS-06 OK rows and the structural rows.

## 5. Batch consistency and shared-situation conflicts

- The batch passes with 0 findings.
- There is one conflict the batch cannot see. The identical `_CONTEXT.md`
  Architecture Basis Injection body (`BodySHA256 696d7086…`, shared by all
  eight PKG-03 deliverables) makes the statement "PKG-00 … at `SEMANTIC_READY`
  supplies dispatchable architecture-basis constraints". Its `.s01` sub-claim
  is disposed three ways inside PKG-03:

| Deliverables | Disposition · Cause |
|---|---|
| DEL-03-07 | `STALE_REVIEW_OR_EVIDENCE` · `RECORD_DRIFT` |
| DEL-03-01, 03-02, 03-03 | `STALE_REVIEW_OR_EVIDENCE` · `SCOPE_REDIRECTED_BY_RULING` |
| DEL-03-04, 03-05, 03-06, 03-08 | `STALE_SETUP_SPECIFICATION` · `SCOPE_REDIRECTED_BY_RULING` |

- Tier (`LOCAL_DESIGN`) and layer (`RECORD`) agree across all eight.
- The text originates at `7bee9ae41`. The class turns on whether a PKG-00
  lifecycle-state declaration falls under F3's "review states … metadata"
  exception. The cause turns on whether the gap is the stale state
  (`RECORD_DRIFT`; all eight PKG-00 deliverables are `IN_PROGRESS` at the
  freeze) or the superseded gate (`SCOPE_REDIRECTED_BY_RULING`, as in CS-05).
- W1's DEL-07-01/02/05/07/08 used `STALE_REVIEW_OR_EVIDENCE` ·
  `RECORD_DRIFT`, which matches DEL-03-07.
- **Agent 0 should record one resolution in `WAVES/W2/RESOLUTIONS.csv`
  (F6).** It might also add a keyed CS row for this sub-claim, or extend
  `--batch` to compare `.sNN` rows whose parents share a body.

## 6. Reverse pass

- 376 capabilities:

| Answer | Count |
|---|---|
| `NOT_MINE` | 368 |
| `CLAIMED_BY` | 3 |
| `COVERS` | 3 |
| `UNKEYED` | 1 |
| `PARTIAL` | 1 |

- **100% review of `CLAIMED_BY` / `PARTIAL` / `UNKEYED` (5 rows), plus the 3
  `COVERS` rows:** all correct.
  - RC-03-0072 (CAP-COREB-047, Rust port and Tauri command): `CLAIMED_BY`.
    The crate declares itself the DEL-03-07 port (Tauri lib.rs l.3359).
  - RC-03-0107 (CAP-COREB-045, Python checker): `CLAIMED_BY`.
  - RC-03-0256 (CAP-COREB-048, `core/library_import/README.md`):
    `CLAIMED_BY`.
  - RC-03-0121 (CAP-COREB-049, Rust crate README): `UNKEYED`. It is correctly
    unkeyed; the SOW names neither the crate nor its README.
  - RC-03-0198 (CAP-COREB-046, hanger import validation): `PARTIAL`. The
    SCA-009 item 23 / DEC-103 item 5 routing is stated.
  - RC-03-0109, RC-03-0162 (fixtures) and RC-03-0288 (Libraries panel):
    `COVERS`. For the panel, CLM-008 excludes GUI.
- **F5.** 13 `NOT_MINE` rows have `EntryPoints` that hit a forward-cited
  path: 9 on Tauri `lib.rs`, plus `CONTRACT.md`, `material.schema.yaml`, the
  registers and `LibraryManagerPanel.tsx`. I checked all 13 (the brief
  requires at least 20%; the deterministic subset is RC-03-0247, 0038 and
  0094). Every reason addresses its own capability, so F5 holds.
- **`NOT_MINE` 10% sample (37 rows).** Every answer was right; the rows
  checked are recorded in scratch, which has since been deleted:
  RC-03-0273, 0174, 0119, 0149, 0020, 0163, 0028, 0077, 0361, 0241, 0131,
  0111, 0240, 0173, 0059, 0012, 0044, 0202, 0357, 0002, 0243, 0280, 0064,
  0179, 0257, 0078, 0185, 0255, 0116, 0221, 0067, 0096, 0321, 0254, 0055,
  0154, 0159.
- **Routing sample.**
  - SAMPLE rows: 26 (VIEW 7, SHELL 6, WSUI 5, COREC 4, PHYS 4), 26/26
    `NOT_MINE`.
  - AREA rows: 350, of which 342 are `NOT_MINE` (97.7%).
  - All 8 ownership or coverage answers are AREA rows in DEL-03-07's natural
    areas (COREB, DATA, FEATB).
  - Three SAMPLE rows hit a declared path (Tauri `lib.rs`): RC-03-0042, 0094
    and 0275. Each has a capability-specific reason, and each is correctly
    `NOT_MINE`.
  - The template-reason rate is similar: 23/26 in SAMPLE and 321/342 in AREA.
  - No sign of anchoring. As the brief notes, about half the sample is
    recognisable from its paths.
- **Capabilities claimed by more than one PKG-03 deliverable (involving
  DEL-03-07):** none. RC-03-0109 is claimed by DEL-03-02 and partially by
  DEL-03-03/04/05/06, but DEL-03-07 answers `COVERS` there.
- **Suspected missed claims:** none.
  - I searched every `NOT_MINE` capability mentioning provenance,
    redistribution, quarantine, library import, local libraries or hanger.
    All answers are reasoned and correct.
  - RC-03-0288 (the Libraries panel) is owned by no PKG-03 deliverable. It is
    an unmapped candidate for R3, since CLM-008 correctly excludes GUI here.
- **Anchored answers:** none found.

## 7. For the owner

1. **Possible product-code defect against OPS-K-UNIT-1 (FG-DEL-03-07-01;
   `INVARIANT`; owner decision).**
   - Both the Python checker and the Rust port used by the desktop app check
     units only on values shaped as `{magnitude, …}`.
   - A material, section or component record carrying a bare number passes
     with no unit finding, and `save_local_library` stores it.
   - No test covers this case.
   - The rerun and I agree on the disposition. Only the cause tag is a
     taxonomy choice for R3.
2. **CLM-026.r01 (vocabulary).** The worker routes this to you as "code
   settled the vocabulary with no ruling".
   - The governing `docs/IP_AND_DATA_BOUNDARY.md` §4 (status `draft`) already
     lists most of the names.
   - The schema and checker add `rejected` and `TBD` to that list.
   - The question for you is therefore narrower. Is the draft policy's list
     authoritative? And should the two extra values be accepted?
3. **Shared sub-claim conflict (§5):** one Agent 0 resolution is needed for
   the PKG-00 `SEMANTIC_READY` statement across PKG-03.
4. **Rename residue in active identifiers that the SOW does not name:**
   - crate `open_pipe_stress_library_import_document`;
   - `document_kind` `openpipestress.library_import.validation`.

   Correctly, no CP-04 row was written for these. They belong in R3's
   active-identifier cluster.
5. **Process note.** The rerun worker read the first verifier's DEL-03-07
   findings after sealing (its notes record this). The sealed hash predates
   that reading. I found no sign that the sealed rows were shaped by it.

Scratch files were created only in the session scratchpad, not in the run
folder, and were deleted before return. Nothing was built or run beyond the
ledger validator. There were no git writes.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). These are
agent judgments, not owner rulings. Nothing here states or implies release,
approval, compliance or certification.
