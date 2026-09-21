VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-00 verification — wave W2 (DOUBLE sampling)

Fresh, evidence-only package verifier for run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION` (parent: HELP_HUMAN Agent 0).
Brief: `R2-VERIFIER_brief.md`, SHA-256
`47fb3c5225ac0a3c532d99a53ee3211129998f887dc9187b40ab898bb2dd5b00` (matched).
Evidence was read from the frozen checkout at
`00115c71931bcae79909602d653740d3bb72dfa1`. I made no build, test run, git
write or ledger edit. The manager's validator transcripts and the workers'
returns were read only to find the flags they raise; they are not evidence.
Every judgment here is an agent judgment, not an owner ruling. Part F
(F1–F8) was judged like any other rule.

## 0. Inputs and seals

| Ledger | Rows | Seal SHA-256 (matches file) |
|---|---|---|
| DEL-00-01 forward | 35 | `57a202470417a0f0552f132358b96dab12545627d624a6b92583979eafd3aaac` |
| DEL-00-02 forward | 43 | `9f20297ca0c95b2cdf4bd3283a97f3efe38329f611bb2540e2005a531be2477b` |
| DEL-00-03 forward | 37 | `1d77b6f40821a0796b40df3f2bb1ad12ea3b773ec1b506dd7df564ceecd072e3` |
| DEL-00-04 forward | 43 | `32db799d19e039df57d8e3b23c56e9f6d04e4ff95ad38f8bcd2b88fd0ca9574e` |
| DEL-00-05 forward | 38 | `03efacbc36b729bbde293934667975915cbf0ba739f75589c9e52d10b5cbd92c` |
| DEL-00-06 forward | 42 | `be09bb89741e2e0177ebde27a359b11a8d302f8ea8badc70711f0ac52d2bec9e` |
| DEL-00-07 forward | 40 | `1c0c7622002239215d44f7479c326491cf9b1ea914d8058e269a597efed81fd9` |
| DEL-00-08 forward | 38 | `e96c1ebf43e88308fccffd9dea439a3199cea62f2e3a0d0215d7c32b339d055b` |

- Each reverse file answers all 387 routed capabilities.
- I ran `validate_ledger_v2.py` in single mode (`--forward`, `--reverse`,
  `--inventory ROUTING/PKG-00_capabilities.csv`, `--notes-gap`) on all
  eight deliverables. All eight returned PASS with 0 findings.

## 1. Sample selection

Sampling follows `R0_REVIEW.md` §7, with the starred rates doubled. Each row
goes into exactly one class, taking the first that applies:

| Class | Rule | Rate |
|---|---|---|
| **A100** | `INVARIANT` tier; `ACCEPTED_DIVERGENCE`, `AUTHORITY_CONFLICT`, `UNKNOWN` or `LIFECYCLE_REASSESSMENT_REQUIRED`; `PROTECTED_CHECK` or `FROZEN_CONTRACT`; or an ISSUED deliverable | 100% |
| **S100** | The unit's `SharedTextCount` in `CLAIM_KEYS_V2.csv` is above 1. A `.sNN` or `.rNN` key takes its parent's count | 100% |
| **F100** | `ALIGNED` with `GAP_WORDING_CHECKED:` or `OPEN_ACTION:` in Notes | 100% |
| **PC50** | `ALIGNED` with `PRODUCT_CALLER: NONE` in Notes | 50% |
| **NA50** | Other non-aligned rows | 50% |
| **AL40** | `ALIGNED` rows of type `REQUIREMENT`, `ACCEPTANCE` or `EXCLUSION`. Weighted rows come first: `LOW`/`MEDIUM` confidence, `NONE_FOUND` verification, or verification class `NONE` | 40% |
| **ST20** | Structural, inherited-canonical and all other quiet rows | 20% |

How the rows were picked:

- Selection is per deliverable and per class.
- Within a class, candidates are ordered by the SHA-256 of `ClaimKey`; the
  lowest ceil(rate × n) are taken. In AL40, the weighted rows are ordered
  first.
- All eight deliverables are `IN_PROGRESS`. None is ISSUED.

| Class | Package rows | Sampled |
|---|---|---|
| A100 | 2 | 2 |
| S100 | 55 | 55 |
| F100 | 3 | 3 |
| PC50 | 1 | 1 |
| NA50 | 74 | 39 |
| AL40 | 46 | 21 |
| ST20 | 135 | 30 |
| **Total** | 316 | 151 |

Sampled keys (prefix each with the deliverable ID and a colon):

- **DEL-00-01 (18):**
  - AL40: `AB#normative-requirements/REQ-01-01`, `REQ-01-02`, `CONTEXT#description`
  - NA50: `AB.s01`, `AB#purpose.s02`, `AB#normative-requirements/REQ-01-04`, `AB#open-holds-and-routed-questions.s01`, `.s02`
  - S100: `CONTEXT#architecture-gate-rule`, `#decomposition-reference`, `#objective-support`, `#package-reference`, `#preparation-notes`, `STATUS#history`, `STATUS#remaining`
  - ST20: `AB#purpose`, `MEMORY`, `STATUS`
- **DEL-00-02 (20):**
  - A100: `AB#open-holds-and-routed-questions.s02`
  - F100: `STATUS#remaining/R01`
  - AL40: `REQ-02-02`, `REQ-02-04`
  - NA50: `AB.s01`, `AB#purpose.s02`, `AB#normative-requirements.s01`, `REQ-02-05`, `AB#open-holds-and-routed-questions.s01`, `.s03`, `CONTEXT`
  - S100: the same five `CONTEXT#…` keys as DEL-00-01
  - ST20: `AB#resolved-decisions-former-tbd-and-human-ruling-q`, `CONTEXT#scope-coverage`, `MEMORY`, `STATUS`
- **DEL-00-03 (18):**
  - AL40: `AB#normative-requirements`, `REQ-03-01`
  - NA50: `AB`, `AB#purpose.s02`, `REQ-03-02`, `AB#realized-artifacts.r02`, `AB#open-holds-and-routed-questions.s02`, `CONTEXT#description`
  - S100: the five `CONTEXT#…` keys, `STATUS#history`, `STATUS#remaining`
  - ST20: `AB#resolved-decisions-…`, `AB#realized-artifacts`, `CONTEXT#register-references`
- **DEL-00-04 (19):**
  - A100: `AB#open-holds-and-routed-questions.s01`
  - AL40: `AB#normative-requirements`, `REQ-04-01`
  - NA50: `AB`, `AB#purpose.s02`, `REQ-04-04`, `CONTEXT`, `CONTEXT#sca-003-storage-profile-injection.s02`
  - S100: the five `CONTEXT#…` keys, `STATUS#remaining`
  - ST20: `CONTEXT#context-budget-qa`, `#register-references`, `#sca-003-storage-profile-injection.s01`, `#scope-coverage`, `MEMORY`
- **DEL-00-05 (19):**
  - F100: `REQ-05-05`
  - AL40: `AB#normative-requirements.s01`, `.s02`, `REQ-05-03`
  - NA50: `AB#resolved-decisions-…`, `AB#open-holds-and-routed-questions.s02`, `AB#currency-and-provenance`, `CONTEXT`, `CONTEXT#anticipated-artifacts`
  - S100: the five `CONTEXT#…` keys, `STATUS#remaining`
  - ST20: `AB#purpose`, `AB#resolved-decisions-….r02`, `CONTEXT#context-budget-qa`, `STATUS`
- **DEL-00-06 (19):**
  - F100: `AB#resolved-decisions-….r02`
  - AL40: `AB#normative-requirements.s02`, `REQ-06-03`
  - NA50: `AB#normative-requirements.s01`, `REQ-06-02`, `AB#currency-and-provenance`, `CONTEXT`, `CONTEXT#anticipated-artifacts`
  - S100: the five `CONTEXT#…` keys, `STATUS#history`, `STATUS#remaining`
  - ST20: `CONTEXT#context-budget-qa`, `#description`, `#scope-coverage`, `STATUS`
- **DEL-00-07 (19):**
  - PC50: `REQ-07-02`
  - AL40: `REQ-07-04`, `AB#req-07-05-handoff-obligations-d-41-r5-t2b-2026-0.r01`, `.r02`, `.r04`
  - NA50: `AB`, `REQ-07-03`, `CONTEXT#anticipated-artifacts`
  - S100: `AB#required-invariants`, `AB#currency-and-provenance`, the five `CONTEXT#…` keys, `STATUS#remaining`
  - ST20: `AB#resolved-decisions-…`, `AB#realized-artifacts`, `CONTEXT#scope-coverage`
- **DEL-00-08 (19):**
  - AL40: `REQ-08-02`, `REQ-08-04`, `AB#interpretation-guidance`
  - NA50: `AB`, `AB.s01`, `CONTEXT#anticipated-artifacts`
  - S100: `AB#required-invariants`, `AB#currency-and-provenance`, the five `CONTEXT#…` keys, `STATUS#history`, `STATUS#remaining`
  - ST20: `AB#realized-artifacts`, `CONTEXT#description`, `CONTEXT#register-references`, `STATUS`

## 2. Per-deliverable results

Firm false alignment is counted among sampled `ALIGNED` normative rows
(`REQUIREMENT`, `ACCEPTANCE`, `EXCLUSION`), whatever their sampling class.

| Deliverable | Rows | Sampled per class | Firm | Weak | Field | Firm false alignment |
|---|---|---|---|---|---|---|
| DEL-00-01 ADR baseline | 35 | S100 7 · NA50 5 of 9 · AL40 3 of 6 · ST20 3 of 13 (18) | 0 | 1 | 0 | 0 of 3 = 0% |
| DEL-00-02 Module boundaries | 43 | A100 1 · S100 5 · F100 1 · NA50 7 of 14 · AL40 2 of 4 · ST20 4 of 18 (20) | 0 | 0 | 0 | 0 of 2 = 0% |
| DEL-00-03 Command-query-job | 37 | S100 7 · NA50 6 of 12 · AL40 2 of 3 · ST20 3 of 15 (18) | 0 | 0 | 1 | 0 of 2 = 0% |
| DEL-00-04 Persistence and versioning | 43 | A100 1 · S100 6 · NA50 5 of 9 · AL40 2 of 5 · ST20 5 of 22 (19) | 0 | 0 | 1 | 0 of 2 = 0% |
| DEL-00-05 GUI state | 38 | S100 6 · F100 1 · NA50 5 of 9 · AL40 3 of 6 · ST20 4 of 16 (19) | 0 | 0 | 1 | 0 of 4 = 0% |
| DEL-00-06 Diagnostics and envelopes | 42 | S100 7 · F100 1 · NA50 5 of 9 · AL40 2 of 5 · ST20 4 of 20 (19) | 1 | 0 | 1 | 1 of 2 = 50% |
| DEL-00-07 API and adapter map | 40 | S100 8 · PC50 1 · NA50 3 of 6 · AL40 4 of 10 · ST20 3 of 15 (19) | 0 | 1 | 3 | 0 of 6 = 0% |
| DEL-00-08 Test and acceptance strategy | 38 | S100 9 · NA50 3 of 6 · AL40 3 of 7 · ST20 4 of 16 (19) | 0 | 0 | 1 | 0 of 4 = 0% |

Rerun test:

- No deliverable's firm error rate on its sampled rows is above 10%.
  DEL-00-06 has the highest: 1 of 19 = 5.3%.
- The firm error falls in AL40, not a 100%-sampled class.
- The rerun criteria are therefore not met.

## 3. Package-level firm false-alignment rate

**1 of 25 sampled ALIGNED normative rows = 4.0%.** This is within the 5%
scale-out gate.

- The margin is one row. A second firm false alignment would give 8.0%.
- The only firm false alignment is DEL-00-06 `REQ-06-03` (§4, item F1).
- Counting the weak disagreement on DEL-00-07 `REQ-07-04` as well, the
  rate is 2 of 25 = 8.0%.

## 4. Disagreements

### Firm

**F1. `DEL-00-06:AB#normative-requirements/REQ-06-03`**

- **What the row says:** `ALIGNED`, MEDIUM. It claims that solver,
  rule-pack, GUI, CLI, report, storage and adapter outputs use result
  envelopes for nontrivial operations. Its storage evidence is
  `schemas/project_persistence.schema.yaml` and
  `tests/test_project_persistence_service.py`. Its adapter evidence is
  `schemas/adapter_framework.schema.yaml`. The Notes say "Breadth judged by
  schema presence and contract tests, not by tracing every operation".
- **What I found:**
  - The product storage commands return a typed payload on success and a
    bare `String` on error. None returns a diagnostics or result envelope.
    Examples in `apps/desktop/src-tauri/src/lib.rs`:
    - `save_local_project` (L2398–2401): `Result<LocalProjectEnvelope, String>`;
    - `save_local_rule_pack` (L3270–3274): `Result<LocalRulePackEnvelope, String>`;
    - `save_local_library` (L3676–3681): `Result<LocalLibrarySaveResult, String>`;
    - `delete_local_rule_pack` (L3335–3339): `Result<LocalRulePackDeleteReceipt, String>`.
  - The storage envelope the row cites belongs to the Python persistence
    service `core/project_persistence/service.py`. That service has no
    product caller: a search of the frozen tree finds only
    self-references and tests. The same holds for the Python adapter
    framework `core/adapters/framework/adapter_framework.py`.
  - F7 applies. The claim concerns what the product's layers output, which
    is app and runtime behaviour. A tested engine with no product caller
    does not satisfy such a claim. The row also lacks the required
    `PRODUCT_CALLER: NONE` marker.
  - The same package records this gap. DEL-00-03 `REQ-03-02` (sampled,
    agreed) is `PARTIALLY_IMPLEMENTED` because storage, rule-pack and
    library commands "return typed receipts or a bare error string".
- **Right values:** `PARTIALLY_IMPLEMENTED` · `PARTIAL_SLICE` ·
  `PROJECT_BASELINE` · `NONE`.
  - Layers: `RECORD`, matching DEL-00-03 `REQ-03-02`. `BASELINE` is
    defensible for both rows.
  - `AuthorityNeeded`: `NO`, or `OWNER` if the storage commands are to be
    exempted.
  - `RemainingWork`: decide whether storage, rule-pack and library commands
    must return the diagnostics envelope, or narrow REQ-06-03.
  - Notes: add `PRODUCT_CALLER: NONE` for the Python persistence and adapter
    engines.
  - On the alternative reading, REQ-06-03 would be a definition met by the
    schemas alone. The worker did not read it that way: the row judges
    implementation breadth. On that reading the row is still F7-deficient.

### Weak

**W1. `DEL-00-07:AB#normative-requirements/REQ-07-04`**

- **What the row says:** `ALIGNED`, MEDIUM. It claims that import/export
  format choices are recorded as TBD unless a human ruling is cited. The
  Notes defer "whether this basis's own open-hold text should now cite
  SCA-004" to `open-holds.s02`. That row is disposed `STALE_REVIEW_OR_EVIDENCE`
  · `SCOPE_REDIRECTED_BY_RULING` · `PROJECT_BASELINE` · `OWNER`, on the
  grounds that SCA-004 settles concrete export formats.
- **What I found:**
  - SCA-004 (decomposition v0.7, `SOFTWARE_DECOMP.md` L33) accepted concrete
    export formats: CAEPIPE MBF workflow, stress-neutral CSV/JSON,
    conservative PCF and GLB/glTF.
  - The basis still lists "External format list" as a genuinely open TBD
    (`ArchitectureBasis.md` L80).
  - If the worker's `.s02` reading is right, REQ-07-04's own rule ("TBD
    unless a human ruling is cited") is unmet for the export side. F1 says
    a gap recorded on another row does not make this row ALIGNED.
  - The accepted row AB-00-07 at revision 0.12 (`SOFTWARE_DECOMP.md` L444)
    still says "concrete import/export formats remain TBD". The governing
    decomposition therefore carries both texts.
- **Right values:** unresolved.
  - If SCA-004 governs the export side: `PARTIALLY_IMPLEMENTED` ·
    `SCOPE_REDIRECTED_BY_RULING` · `PROJECT_BASELINE` · `NONE` · `RECORD` ·
    `OWNER`.
  - If AB-00-07's TBD text governs: the row is `ALIGNED`, and `.s02`
    becomes the question.
  - Either way, the AB-00-07 and SCA-004 pair looks like an
    `AUTHORITY_CONFLICT` inside the decomposition, and the owner should
    see it (§7).

**W2. `DEL-00-01:AB#open-holds-and-routed-questions.s01`**

- **What the row says:** `STALE_REVIEW_OR_EVIDENCE` · `RECORD_DRIFT` ·
  `LOCAL_DESIGN` · `NONE` · `RECORD` · `NO`. It drops rule grammar, CI
  provider and coverage from the open list. The Notes say "Public API
  transport and state library remain open as stated".
- **What I found:**
  - Another worker in the same package disposed the same hold
    (component/state-management library) as `CP-10` · `IMPLEMENTED_DIFFERENTLY`
    · `AUTHORITY_UNCLEAR` · `PROJECT_BASELINE` · `OWNER`
    (DEL-00-05 `open-holds.s01`).
  - The evidence supports DEL-00-05. `apps/desktop/package.json` has no
    third-party state or component library, and state is split into React
    session hooks. The code has settled the choice with no ruling found.
  - C1 requires a `.sNN` split when parts of a block would take different
    dispositions.
- **Right values:** split the row. The overtaken items take the present
  fields. The state-library clause takes CP-10's fields. Only the tier
  and `AuthorityNeeded` of the state-library clause change. It is weak
  because whether "React built-ins, no library" counts as a choice is
  exactly what CP-10 routes to the owner.

### Field

**D1–D4. `CONTEXT#anticipated-artifacts` on DEL-00-05, DEL-00-06, DEL-00-07
and DEL-00-08** (all four sampled)

- **What the rows say:** `STALE_SETUP_SPECIFICATION` · `DOC_BEHIND_CODE` ·
  `LOCAL_DESIGN` · `NONE` · `RECORD` · `NO`.
- **What I found:**
  - The dispositions are right. The named documents were never created,
    and the text dates from `7bee9ae41` (F3).
  - The cause is wrong. The artifact column of `docs/_Registers/Deliverables.csv`
    was moved to `ArchitectureBasis.md` by the D-43 consolidation (commit
    `33fd81a85`; `D-43_pkg00_architecture_basis_consolidation.md` L79).
    No code advanced here.
  - The other worker in the package used `SCOPE_REDIRECTED_BY_RULING` for
    the identical situation (DEL-00-02, DEL-00-03 and DEL-00-04, not
    sampled). The batch check cannot see this split, because the bodies
    differ and no CP ID is written.
- **Right values:** cause `SCOPE_REDIRECTED_BY_RULING`. `REPRESENTATION_MIGRATED`
  is also defensible. Use one cause for all seven rows.

**D5–D6. `DEL-00-07:AB#req-07-05-handoff-obligations-d-41-r5-t2b-2026-0.r01`
and `.r02`**

- **What the rows say:** `ALIGNED`, citing
  `core/adapters/framework/adapter_framework.py`. `.r01` also cites
  `core/project_persistence`.
- **What I found:**
  - Neither Python engine has a product caller.
  - The claims concern adapters' handoff obligations, so F7 lets the
    engines satisfy them, and the dispositions stand.
  - F7 still requires `PRODUCT_CALLER: NONE` in Notes. The worker wrote the
    marker on `REQ-07-02` for the same engine, but not on these rows.
- **Right values:** add `PRODUCT_CALLER: NONE` to Notes. The unsampled
  parent block `AB#req-07-05-handoff-obligations-…` needs the same marker.

**D7–D8. `DEL-00-03:AB#normative-requirements` and
`DEL-00-04:AB#normative-requirements`**

- **What the rows say:** `ALIGNED`, HIGH. The only evidence is
  `docs/CONTRACT.md` ("The six OPS-K identifiers exist in the project
  contract").
- **What I found:**
  - The blocks carry boundary invariants (IP, data, authority) and the
    guardrail "software and agents must not claim code compliance".
    Checking only that the identifiers exist does not assess them.
  - The other worker judged the same substance by a contrary-evidence
    check at MEDIUM confidence: DEL-00-05 `.s01`, DEL-00-06 `.s02`,
    DEL-00-07 and DEL-00-08 `AB#required-invariants`. Those rows cite the
    claims-language scan and the protected-content linter test.
  - I found no contrary evidence, so `ALIGNED` stands.
- **Right values:** Confidence `MEDIUM`. Add the contrary-evidence check,
  as the other worker's rows do.

### Checked and agreed (summary)

- **Canonical rows:** all 56 inherited rows match `CANONICAL_ASSIGNMENTS.csv`
  field for field (CS-01, CS-02, CS-05, CS-06-OK and CS-07; 7 per
  deliverable). PKG-00 has no CS-04 assignment.
- **CP-02 pins:** every ArchitectureBasis header and currency block pins
  revision 0.9 and DAG-007. The frozen decomposition is 0.12
  (`SOFTWARE_DECOMP.md` L6), and `_DAG/_LATEST.md` names DAG-010.
- **CP-02 section pointer:** DEL-00-05 and DEL-00-06 cite a "§8.4" that
  does not exist.
- **CP-04 rename residue:**
  - `.opsproj` is named in DEL-00-01, 00-02, 00-07 and 00-08; those rows
    take `PROJECT_BASELINE`.
  - `openpipestress-runner` (Cargo `[[bin]]`, crate
    `open_pipe_stress_headless_runner`) is named in DEL-00-03, whose row
    takes the default `LOCAL_DESIGN`.
  - DEL-00-04, 00-05 and 00-06 name neither.
  - Each is recorded once, on the SURFACE row, with the DEC-101
    persistence note where required.
- **A100 rows:**
  - DEL-00-02 `open-holds.s02` is `UNKNOWN` · `AUTHORITY_UNCLEAR`. The
    governing sources are silent on where implementation run records
    belong. The records exist under
    `DEL-00-02…/_run_records/WORKING_ITEMS_RUN_2026-06-11_t3_wasm_enablement_*`.
  - DEL-00-04 `open-holds.s01` is `UNKNOWN` · `EVIDENCE_NOT_LOCATED`.
    Searching the frozen tree for "provider expansion" finds no referent in
    `projects/chirality-piping`; the only hits are in chirality-app-dev and
    unrelated AgentRuns. Both rows carry a smallest check in
    `RemainingWork`.
- **F100 rows:**
  - DEL-00-02 `STATUS#remaining/R01` correctly takes `OPEN_ACTION` to
    `REQ-02-05`.
  - DEL-00-05 `REQ-05-05` is correct. `MissingDataBlockingPanel.tsx` types
    all six SPEC classes, with `local_contract_class` beside each.
  - DEL-00-06 `.r02` is an accurate record of the TP-VERIFY-013E
    classification.
- **DEL-00-06 `REQ-06-02` (the worker's possible defect) is confirmed.**
  - `export_mechanics_diagnostic` in
    `core/runner/headless/src/result_envelope_binding.rs` L156–182 sets
    `affected_object` to `("preview_diagnostic", diagnostic.id)`. That
    drops the solve diagnostic's `affected_refs` list
    (`core/product_physics/src/lib.rs` L710–718).
  - It also supplies a fixed remediation string.
  - `IMPLEMENTED_DIFFERENTLY` · `POSSIBLE_DEFECT` · `PROJECT_BASELINE` ·
    `BASELINE` · `OWNER` is right. AB-00-06 (`SOFTWARE_DECOMP.md` L443)
    requires the affected object.
- **CP-10 rows:**
  - DEL-00-02 `.s03`: the project `package.json` declares an npm workspace
    with a committed lockfile, and no lint configuration exists.
  - DEL-00-05 `.s02`: undo storage is a 25-deep in-memory snapshot stack,
    per the worker's notes and the cited files.
- **F3 origins spot-checked:** DEL-00-04 `CONTEXT#sca-003….s02` first
  appears in `7bee9ae41` (`git log -S`).
- **DEL-00-03:**
  - `core/gui/viewport_editor` is a Rust crate with no `engine.py`
    (`realized-artifacts.r02`).
  - The runner verbs `solve`, `validate-input`, `export-results`,
    `run-benchmark` and `run-regression` exist
    (`openpipestress-runner.rs` L825–829).
- **Context used as authority:** none. Merged PRs, run records and
  workplans appear only in ContextRefs or Notes. The D-68 adopted-by-reference
  row carries its flag.

### Outside the sample (observations only; not counted)

- **`DEL-00-02:AB#resolved-decisions-….r04`**, raised by worker G1. The
  Notes say dense "stays default" under DEC-050. That is outdated: DEC-053
  promotes sparse interactive as the default preview path
  (`SOFTWARE_DECOMP.md` DEC-053 row). The `ALIGNED` disposition is
  unaffected, because the basis text says only "in-repo sparse
  skyline/profile direct solver".
- **`DEL-00-07:AB#open-holds-and-routed-questions.s02`**: see W1. AB-00-07
  at revision 0.12 still keeps the formats TBD, which argues for
  `AUTHORITY_CONFLICT` rather than `STALE_REVIEW_OR_EVIDENCE`.
- **FindingGroup gaps:** DEL-00-03 and DEL-00-04 leave `FindingGroup` empty
  on their CP-02 pin rows. The other six deliverables group them. This is
  cosmetic.

## 5. Batch consistency and shared situations

- **PKG-00 batch.** `validate_ledger_v2.py --batch` over the eight forward
  ledgers returns **FAIL, 1 finding**. There is no `WAVES/W2/RESOLUTIONS.csv`.
  - The finding: `DEL-00-03:AB` is CP-04 with tier `LOCAL_DESIGN`, while
    the other four CP-04 rows are `PROJECT_BASELINE`.
  - **Resolved by this verifier as CP-04's own variant split, not a
    conflict.** DEL-00-03 names only the runner binary, which takes CP-04's
    default fields. The other four name `.opsproj`, one of the four
    identifiers CP-04 gives `PROJECT_BASELINE`.
  - For DEL-00-07, which names both, the stronger `.opsproj` fields are
    used. I agree.
  - Agent 0 may record this pair in `WAVES/W2/RESOLUTIONS.csv` (F6).
- **Cross-package batch.** Over all 31 sealed W2 forward ledgers the batch
  returns 7 findings. Four involve PKG-00: the four `.opsproj` CP-04 rows
  against a W2-wide `LOCAL_DESIGN` majority. They are the same variant
  split. The validator should learn CP-04's variants, or workers should
  write the variant in `CanonicalSituation`, for example `CP-04-OPSPROJ`.
- **Shared bodies.** All 55 S100 rows agree across every sealed ledger
  (W1 and W2) that shares their `BodySHA256`:
  - `STATUS#remaining` (empty; 36 units, 21 ledgered): all `NOT_ASSESSED`;
  - `STATUS#history` (4): all `ALIGNED`;
  - `CONTEXT#decomposition-reference` (71 units, 36 ledgered): CS-01;
  - `CONTEXT#preparation-notes` (70 units, 35 ledgered): CS-02;
  - `required-invariants` and `currency-and-provenance`
    (DEL-00-07 and DEL-00-08): consistent.
- **Shared-situation conflicts the batch cannot see:** the
  anticipated-artifacts cause split (D1–D4) and the state-library split
  (W2). Both are between the package's two workers.

## 6. Reverse pass

- **Coverage.**
  - Every CLAIMED_BY (1), PARTIAL (1) and CONSTRAINS (133) answer was
    checked. There are no UNKEYED answers.
  - The 19 COVERS answers were also read.
  - Every positive answer names an existing forward key, and none uses a
    `STATUS#remaining` key.
  - A 10% SHA-ordered sample of NOT_MINE answers was checked: 295 of 2,942.
  - The F5 check covered 20% of the NOT_MINE answers whose `EntryPoints`
    hit a path the deliverable's own forward ledger cites: 111 of 555.
- **F5.** All 111 sampled overlap answers give a capability-specific
  reason. No answer in the sample is wrong.
  - Worker G1 (DEL-00-01 to 00-04) generated every NOT_MINE reason from a
    pattern. Each quotes the capability, names the overlapping path and
    why the forward ledger cites it, then states that the member owns no
    module.
  - I judge that this meets F5 (worker G1 asked for this judgment). The
    reason addresses the capability and the overlap. For a
    documentation-only member, "owns no module" is the true ground.
  - The 264 DEL-00-02 overlaps come mostly from citing whole directories
    (`apps/desktop`, `core`, `schemas`) as forward evidence. That evidence
    choice, not ownership, inflates the F5 population.
  - Worker G2 (DEL-00-05 to 00-08) wrote bespoke reasons for overlap rows
    and area templates for the rest. Templates are permitted where paths
    do not overlap.
- **Answer distribution: SAMPLE rows against AREA rows.**

  | Routing | Answers | NOT_MINE | Other answers |
  |---|---|---|---|
  | SAMPLE | 184 | 178 (96.7%) | 6 |
  | AREA | 2,912 | 2,764 (94.9%) | 148 |

  - The six SAMPLE positives are all CONSTRAINS, and each is plausible:
    - CAP-PHYS-037 (runner record validation): DEL-00-03, 00-06 and 00-07;
    - CAP-COREC-001 (audit-manifest hashing): DEL-00-04;
    - CAP-COREC-036 (export SDK admission): DEL-00-07;
    - CAP-COREC-006 (report protected-content block): DEL-00-08.
  - The distributions match. I see no sign of answers driven by path
    recognition beyond what is inherent.
- **Capabilities claimed by more than one PKG-00 deliverable.**
  - None has two CLAIMED_BY answers. The only CLAIMED_BY is RC-00-0099
    (ADR surface), by DEL-00-01, which is correct.
  - 38 capabilities draw positive answers from two or more members. All of
    these are CONSTRAINS or COVERS, which is expected for cross-cutting
    architecture.
  - RC-00-0285 (architecture-basis validator) draws COVERS from seven
    members and PARTIAL from DEL-00-08. That is consistent with REQ-08-04,
    which asks DEL-00-08 to define PKG-00 acceptance gates.
- **Suspected missed claims.**
  - **Routing gap (R3):** CAP-PHYS-039 is not routed to PKG-00. It builds
    the canonical result-export document in
    `core/runner/headless/src/result_envelope_binding.rs`, the code that
    carries DEL-00-06's REQ-06-02 defect. DEL-00-06 could not answer it;
    CONSTRAINS on REQ-06-01 and REQ-06-02 would fit.
  - **Minor:** DEL-00-06 answers NOT_MINE on RC-00-0098 (issues drawer
    listing operation and model diagnostics) and RC-00-0133 (diagnostic to
    entity resolution), with the GUI area template. It answers CONSTRAINS
    on the comparable blocking-data panel RC-00-0199.
  - **Minor:** RC-00-0173 (planning registers) is COVERS from DEL-00-05 to
    00-08 and NOT_MINE from DEL-00-01 to 00-04. This is a worker-level
    split.
- **Anchored answers.** None found.
  - Reverse reasons cite forward findings, for example "the REQ-03-02 gap",
    which the protocol allows after sealing.
  - No forward row shows knowledge of the reverse inventory.

## 7. What the owner must see

1. **The scale-out gate input for PKG-00 is 4.0%, within the 5% gate, by
   one row.**
   - The single firm false alignment is DEL-00-06 REQ-06-03. It claims
     that storage outputs use result envelopes, but the product storage
     commands return typed payloads and bare error strings.
   - DEL-00-03 REQ-03-02 in the same package already records that gap.
   - The contested rows in §4 need a fresh correction worker or entries in
     `WAVES/W2/RESOLUTIONS.csv`. Sealed ledgers are not patched.
2. **Confirmed code defect candidate (DEL-00-06 REQ-06-02).**
   - When a solve diagnostic crosses into the 0.2 result-export envelope,
     its list of affected objects is replaced by the diagnostic's own ID,
     and a fixed remediation string is added
     (`result_envelope_binding.rs` L156–182).
   - The accepted AB-00-06 row requires the affected object.
   - The same function hard-codes provenance `contributor: "OpenPipeStress"` (L177; also L114 and L402 in the same file),
     the former product name. This active code string is not named by any
     PKG-00 text, so no PKG-00 row records it.
3. **The decomposition contradicts itself on export formats.**
   - AB-00-07 at revision 0.12 says "concrete import/export formats remain
     TBD".
   - SCA-004, accepted and recorded in v0.7, names the PKG-17 export
     formats.
   - DEL-00-07's rows treat SCA-004 as governing. The owner should say
     which text governs.
4. **Open holds the code has settled with no ruling (CP-10, routed to the
   owner):**
   - package manager and workspace: npm workspaces (DEL-00-02);
   - state-management library: React built-ins, no library (DEL-00-05);
   - undo/redo storage: in-memory snapshots, 25 deep (DEL-00-05);
   - severity taxonomy (DEL-00-06).
   DEL-00-01 still treats the state library as open (W2).
5. **No PKG-00 row carries `INVARIANT` tier.**
   - The IP and data requirements (REQ-01-05, REQ-04-05) and the carried
     OPS-K invariant blocks are all `ALIGNED`.
   - The 100% class therefore holds only two `UNKNOWN` rows. R4 boundary
     routing receives nothing from this package.
6. **Every pin and pointer fix changes hash-bound bytes.**
   - Each ArchitectureBasis file is hash-bound in the package
     `CONSOLIDATION_MANIFEST.md`, which the architecture-basis validator
     checks. The validator also requires the setup 0.7 pin in `_CONTEXT.md`.
   - A catch-up edit must therefore re-record the manifest hash and keep
     the validator consistent.
   - `AuthorityNeeded NO` still holds, because no decision is needed.
7. **Worker-raised item outside PKG-00 (for the DEL-02-05 verifier).**
   - `core/project_persistence/service.py` labels its canonicalization
     `SORTED_COMPACT_JSON`, stated as "not RFC 8785/JCS" (L7, L28, L241).
     Its test asserts that "JCS" is absent (L446).
   - AB-00-04 requires JCS-compatible hashing.
   - The service has no product caller.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
