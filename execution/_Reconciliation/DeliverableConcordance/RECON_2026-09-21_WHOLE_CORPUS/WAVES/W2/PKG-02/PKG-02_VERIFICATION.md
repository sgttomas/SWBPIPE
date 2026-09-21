VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-02 verification — wave W2, run HELP-HUMAN-PIPING-20260921-RECONCILIATION

Fresh, evidence-only package verifier (TASK, Type 2). The parent is HELP_HUMAN
Agent 0. Brief `briefs/R2-VERIFIER_brief.md`, SHA-256 `47fb3c52…dd5b00`,
checked and matching. Sampling was `DOUBLE`. Evidence was read from the freeze
checkout at `00115c71931bcae79909602d653740d3bb72dfa1`. No ledger was edited, no
build or test was run, and no git write was made. Every value here is an agent
judgment, not an owner ruling.

Sealed ledgers checked: the recomputed forward SHA-256 equals the seal for all
five deliverables.

| DEL | Forward SHA-256 (seal = recomputed) |
|---|---|
| DEL-02-01 | `4ad0dd3344aac58916377ec3bff85e468c5d7084ba058711446c1853a4fbdbdd` |
| DEL-02-02 | `6a253791c9f7a01abccc60023a9719d4ed7fc12d0a8ea4fee74ef736bc25a109` |
| DEL-02-03 | `eef1d2b60bcbe49acdb0d58b626828910dc2a44bb39d03561ffa8774b99fde2f` |
| DEL-02-04 | `e7880c8c0a399ee77614cad0923f050761a19a9904a713bc81882280a39ca59c` |
| DEL-02-05 | `28b9f6f6d12f32ca0927786f1aa186b16c674822e73395657190662eb9029eb1` |

## 1. Verdict basis

- No deliverable exceeds a 10% firm error rate on its sampled rows. The
  highest are DEL-02-02 at 3.4% and DEL-02-05 at 2.7%.
- No firm error falls on a 100%-sampled class. No firm error changes a tier
  toward `INVARIANT` or changes owner routing.
- So no rerun is required. The four firm rows and the weak and field rows
  below are contested and should go into `WAVES/W2/RESOLUTIONS.csv` (F6).

## 2. Sample selection (deterministic)

Candidates in each class were sorted by the SHA-256 of the claim key and the
lowest fraction was taken, rounding up. Each row falls in the first class it
matches, in the order shown.

| Class | Rule (DOUBLE) | Candidates | Sampled |
|---|---|---|---|
| C1 | `INVARIANT` tier, `ACCEPTED_DIVERGENCE`, `AUTHORITY_CONFLICT`, `UNKNOWN`, `LIFECYCLE_REASSESSMENT_REQUIRED`, `PROTECTED_CHECK`/`FROZEN_CONTRACT`: 100% | 16 | 16 |
| C2 | Unit with `SharedTextCount > 1` (including `.sNN` of such a unit): 100% | 38 | 38 |
| C3 | `ALIGNED` with `GAP_WORDING_CHECKED:` or `OPEN_ACTION:`: 100% | 8 | 8 |
| C4 | `ALIGNED` with `PRODUCT_CALLER: NONE`: 50% | 33 | 17 |
| C5 | Other non-aligned: 50% | 183 | 92 |
| C6w | `ALIGNED` normative, weighted (LOW/MEDIUM confidence, `NONE_FOUND` verification or `NONE` class): 80% | 42 | 34 |
| C6 | Other `ALIGNED` normative: 20% (C6w + C6 = 47 of 106, 44% ≥ the doubled 40%) | 64 | 13 |
| C7 | Structural and inherited canonical rows: 20% | 85 | 17 |
| C8 | Other `ALIGNED` rows (declared-state, context, history): 10%, supplementary; not in the brief's table | 129 | 13 |
| C9 | Worker-flagged row not otherwise sampled (DEL-02-02 `SOW#CLM-020`): added, disclosed | 1 | 1 |

That is 249 of 598 forward rows. No deliverable is ISSUED. The four
DEL-02-05 rows the worker flagged were already in the deterministic sample.

## 3. Per-deliverable results

"Firm FA rate" is the firm false-alignment rate among sampled `ALIGNED`
normative rows (`REQUIREMENT`, `ACCEPTANCE`, `EXCLUSION`).

| DEL | Rows | Sampled | Per class (C1/C2/C3/C4/C5/C6w/C6/C7/C8/C9) | Firm | Weak | Field | Firm rate | ALIGNED normative sampled | Firm FA rate |
|---|---|---|---|---|---|---|---|---|---|
| DEL-02-01 | 83 | 38 | 1/5/2/0/17/4/6/3/0/0 | 0 | 3 | 0 | 0.0% | 12 | 0.0% |
| DEL-02-02 | 119 | 58 | 8/5/5/0/22/8/4/3/2/1 | 2 | 2 | 0 | 3.4% | 15 | 13.3% (7.1% without C9) |
| DEL-02-03 | 71 | 27 | 2/8/0/0/12/2/2/1/0/0 | 0 | 0 | 0 | 0.0% | 4 | 0.0% |
| DEL-02-04 | 142 | 53 | 2/8/0/17/16/0/0/5/5/0 | 0 | 2 | 3 | 0.0% | 17 | 0.0% |
| DEL-02-05 | 183 | 73 | 3/12/1/0/25/20/1/5/6/0 | 2 | 10 | 11 | 2.7% | 22 | 9.1% |

Each row is counted once, under its most serious class.

## 4. Package-level firm false-alignment rate (scale-out gate input)

- **4.3%** (3 of 69) on the deterministic sample. This is the gate input
  (gate ≤ 5%): **inside the gate, with little margin.**
- **5.7%** (4 of 70) if the worker-flagged DEL-02-02 `CLM-020` is added. That
  row was not drawn by the selection rule, so it biases the estimate upward.
  It is reported so the owner sees it.

## 5. Disagreements

### Firm (would re-dispose)

1. **DEL-02-02:SOW#CLM-020** (C9, worker-flagged; `REQUIREMENT`).
   - *Row:* `ALIGNED`. It rests on the validation-preview engine importing
     `core/units/schema_vocabulary.py`.
   - *Found:* the requirement says Python adapter and application validation
     "SHALL derive the canonical `DimensionId` vocabulary … rather than
     maintaining a parallel literal set." Three Python validation modules
     still keep literal sets:
     - `core/gui/pkg02_boundary.py:10` (`CANONICAL_DIMENSIONS` tuple, used for
       membership checks at lines 253 and 273);
     - `core/constraints/validation/engine.py:64` (used at 742);
     - `core/model_transform/physical_to_analytical/contract.py:34` (used at
       531).

     `tests/test_operation_validation_preview.py:119-126` asserts that the GUI
     and constraint mirrors equal the schema enum. That checks parity but
     preserves the parallel sets. The block's own caveat ("does not assert
     system-wide B2/B3 completion") does not narrow the SHALL.
   - *Right values:* `PARTIALLY_IMPLEMENTED` · `PARTIAL_SLICE` ·
     `LOCAL_DESIGN` · `NONE` · `RECORD` · `NO`.
2. **DEL-02-02:SOW#CLM-035** (C6w; `REQUIREMENT`, principles table).
   - *Row:* `ALIGNED`. Its Notes say "Adapter uniformity is judged on U-002."
   - *Found:* the principle "Make adapters boring" requires imports, exports,
     plugins and adapters to pass through the same unit validation. U-002 is
     `UNKNOWN · EVIDENCE_NOT_LOCATED` in the same ledger. Under F1 an element
     recorded as not shown is not `ALIGNED`, even when another row also
     records it.
   - *Right values:* `UNKNOWN` · `EVIDENCE_NOT_LOCATED` · `PROJECT_BASELINE` ·
     `NONE` · `BASELINE`, with U-002's smallest check in `RemainingWork`.
3. **DEL-02-05:SOW#CLM-019.r04** (C6w; `REQUIREMENT`).
   - *Row:* `ALIGNED`. Its Notes say "Rule-pack reference diagnostics are
     judged on … REQ-02-05-024."
   - *Found:* the claim lists rule-pack diagnostics among the validate-project
     outputs. REQ-02-05-024 (`PARTIALLY_IMPLEMENTED`) records that
     `validate_project_persistence_envelope` in
     `core/project_persistence/service.py` never inspects `rule_pack_refs`.
     The implementation evidence also has no product caller, and the row
     lacks the F7 marker.
   - *Right values:* `PARTIALLY_IMPLEMENTED` · `PARTIAL_SLICE` ·
     `LOCAL_DESIGN` · `NONE` · `RECORD` · `FG-DEL-02-05-06` · `NO`, plus
     `PRODUCT_CALLER: NONE`. The worker's own reverse notes reach the same
     view.
4. **DEL-02-05:SOW#CLM-030.r03** (C6w; `ACCEPTANCE`).
   - *Row:* `ALIGNED`. Its Notes say "populated rule-pack coverage is judged on
     r08."
   - *Found:* the check requires rule-pack metadata to survive the round trip.
     The fixture `fixtures/persistence/invented_persisted_preview_project.json`
     has empty `rule_pack_refs`, and r08 is `PARTIALLY_IMPLEMENTED` ("no
     populated reference round trip"). So the element holds only by
     construction (CP-11) and is recorded as open elsewhere (F1).
   - *Right values:* `PARTIALLY_IMPLEMENTED` · `PARTIAL_SLICE` ·
     `LOCAL_DESIGN` · `NONE` · `RECORD` · `FG-DEL-02-05-06` · `NO`.

Two unsampled rows use the same F1 deferral:
- DEL-02-05 `SOW#CLM-005.r02`;
- DEL-02-05 `SOW#CLM-014/REQ-02-05-002`.

They are not counted here. Agent 0 may resolve them with the group.

### Weak (a reading the conventions should settle)

- **DEL-02-01:SOW#CLM-011/REQ-02-01-09** and **DEL-02-01:SOW#CLM-017** (the
  JCS scope reading).
  - The rows read "JCS-compatible where hashed" and "confirm the canonical
    JSON/JCS-compatible basis" as schema-compatibility obligations. On that
    reading they are `ALIGNED`.
  - The model schema's `Checksum.canonicalization` admits
    `SORTED_COMPACT_JSON`, `NONE` and `TBD` besides `JCS`.
  - REQ-09 cites `schemas/project_persistence.schema.yaml` as evidence. That
    schema's description says "no RFC 8785/JCS claim is made", and its hash
    is const `SORTED_COMPACT_JSON`.
  - A broader reading makes both rows `PARTIALLY_IMPLEMENTED` in
    FG-DEL-02-05-03's terms. The conventions do not say which deliverable's
    claims carry the Python persistence hash gap.
- **DEL-02-01:SOW#CLM-024** (prerequisites). The row keeps "SCA-001 TBDs are
  not silently resolved" `ALIGNED`, while CLM-005.s01 (CP-10) records that
  the layout TBD was settled in code with no ruling. Whether a procedural
  prerequisite can be "unmet" is not settled.
- **DEL-02-02:SOW#CLM-014/U-009** and **U-015**: cross-ledger inconsistency.
  - Both are requirements carrying a stale "layout/tooling TBD" or
    "categories TBD" clause. They stay `ALIGNED` and defer the clause to
    CLM-015.r11, r03 and r07.
  - Worker G2 disposed the same shape in DEL-02-05 REQ-02-05-004 as
    `STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE`.
  - The two workers diverge on whether an overtaken TBD clause inside a met
    requirement makes the row non-aligned.
- **DEL-02-04:SOW#CLM-012/DEL-02-04-REQ-18** (no telemetry or private data to
  plugins by default). It is `ALIGNED` on the contract reading. No plugin
  runtime exists (DEC-012 held), so part of the claim holds by construction.
  REQ-13 was disposed as CP-11 for the same reason.
- **DEL-02-04:SOW#CLM-012/DEL-02-04-REQ-12** (JCS for manifest hashes). The
  verifier's only hash, the pinned schema fingerprint, uses sorted-key
  compact JSON. The row itself says this, and says it coincides with JCS for
  this content only.
- **DEL-02-05:SOW#CLM-014/REQ-02-05-008**, **REQ-02-05-011** and
  **REQ-02-05-018** (worker-flagged).
  - The "representable" and "expressible" wording supports a contract
    reading. On that reading the Python service meets the claim.
  - The desktop product store's `open_local_project` and `save_local_project`
    return `Result<…, String>` (`apps/desktop/src-tauri/src/lib.rs:2337`,
    `2398`), so product failures are not structured diagnostics.
  - F7 would make these rows `PARTIALLY_IMPLEMENTED` if the subject is
    product persistence. They also lack the F7 marker.
- **DEL-02-05:SOW#CLM-030.r10**. Its Notes say "Structured diagnostics on both
  paths". That is inaccurate for product open and save failures (see the
  previous item). The disposition holds only on the service-contract reading.
- **DEL-02-05:SOW#CLM-006** (GUI exclusion). The worker disclosed this
  judgment. Several desktop panels list DEL-02-05 among their refs.
- **DEL-02-05:SOW#CLM-022.r02**. The claim requires "canonical hash
  behaviour", and the Notes defer JCS fidelity to REQ-02-05-005, which is
  `PARTIALLY_IMPLEMENTED`.
- **DEL-02-05 FG-DEL-02-05-03 rows** (sampled: `SOW#CLM-005.r05`,
  `SOW#CLM-014/REQ-02-05-005`, `CONTEXT#context-envelope`,
  `CONTEXT#sca-003-storage-profile-injection`): a shared-situation conflict
  with DEL-02-02.
  - The diverging artifact is the same: the Python persistence hash basis.
  - DEL-02-02 FG-DEL-02-02-01 gives it `BaselineClass FROZEN_CONTRACT` and
    layers `RECORD;BASELINE`. DEL-02-05 gives it `BaselineClass NONE` and
    layers `BASELINE;RECORD`.
  - C4 names the class of the diverging artifact. A hash-byte contract pinned
    by exact-byte and fixed-SHA tests fits `FROZEN_CONTRACT`.
  - The dispositions also differ: `IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR`
    against `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE`. That difference is
    defensible, because DEL-02-02's claims restate the non-JCS basis while
    DEL-02-05's restate AB-00-04.
  - The tier (`PROJECT_BASELINE`) and owner routing agree.

### Field (right disposition, wrong field)

All of these lack `PRODUCT_CALLER: NONE` (F7). Their implementation evidence
is only the Python persistence service or schema, or the plugin verifier or
manifest schema. Neither has a product caller: I confirmed that no non-test
code imports `core/project_persistence` or `plugin_verification.py`.

- DEL-02-04, 3 rows:
  - `SOW#CLM-005.r09`;
  - `SOW#CLM-007.r04`;
  - `SOW#CLM-031/OD-02-04-003`.
- DEL-02-05, 11 rows:
  - `SOW#CLM-005.r03`;
  - `SOW#CLM-014/REQ-02-05-022`, `REQ-02-05-023`;
  - `SOW#CLM-016.r09`;
  - `SOW#CLM-017`;
  - `SOW#CLM-022.r01`, `.r06`, `.r07`;
  - `SOW#CLM-030.r01`, `.r07`;
  - `SOW#CLM-039.r05`.

These match the workers' own lists (26 rows in DEL-02-04 and 46 in DEL-02-05
across the full ledgers). R3 should apply the marker to the full lists from
`RESOLUTIONS.csv`, because F7 clustering depends on it.

### Rows checked and upheld (notable)

- DEL-02-01 CLM-019 `PARTIALLY_IMPLEMENTED` holds.
  `tests/test_model_schema.py::check_jsonschema_validation` validates
  `normalized_*_fixture_for_current_schema()`, not the committed bytes.
- DEL-02-03 CLM-003 and CLM-011 hold. The boundary `Diagnostic` in
  `schemas/analysis_boundary.schema.yaml` has no `code` field; it requires
  `diagnostic_id`, `diagnostic_class`, `severity`, `source`, `message` and
  `provenance`.
- DEL-02-04 REQ-11 `POSSIBLE_DEFECT` holds.
  `fixtures/plugin_manifest/invented_manifest_no_bypass.json` has
  `manifest_kind` `openpipestress.plugin_manifest` against the schema const
  `open_pipe_stress_plugin_manifest`, and `checksums` as a list against
  `$ref ChecksumSet`.
- The DEL-02-02 FG-DEL-02-02-01 rows hold. There is no relaxing ruling: D-41
  activated the concordance program, and T2A was a repair tranche.
  `docs/SPEC.md` lines 140-143, 359 and 629 still state the JCS basis.
  - Cause `AUTHORITY_UNCLEAR` could also be read as `POSSIBLE_DEFECT`.
  - Routing is unchanged either way.
- DEL-02-05 CLM-029, CLM-030.r11 and CLM-031 `UNKNOWN` hold. My own A3 grep
  found no fixture data-boundary review record.
- The origin checks hold. `git log -S` over the whole frozen history, not
  path-limited, confirms `7bee9ae41` origin for the setup strings I spot
  checked, such as "not an implemented schema file" and "Exact schema file
  layout, `$id` URI".
- The DEL-02-02 STATUS surface is correctly `ALIGNED`: Last Updated is
  2026-09-05, which equals the latest history entry.
- The DEL-02-02 R02, R03 and R04 `OPEN_ACTION` targets (U-001, U-016, U-010)
  are all `PARTIALLY_IMPLEMENTED`, so F2 is satisfied.

## 6. Batch consistency and shared situations

- `validate_ledger_v2.py --notes-gap` single mode, with `--reverse` and
  `--inventory`, run against both the repo root and the freeze root: **PASS,
  0 findings, on all five ledgers.**
- `validate_ledger_v2.py --batch` over the five forward ledgers, with the
  freeze as repo root: **FAIL, 1 finding.** No `WAVES/W2/RESOLUTIONS.csv`
  existed, so `--resolutions` was not passed.
  - **DEL-02-05:STATUS.s01** (CP-04, tier `PROJECT_BASELINE`) against the CP-04
    majority (`LOCAL_DESIGN`).
  - *Resolution: the row is correct.* CP-04 itself prescribes
    `BaselineClass NONE`, tier `PROJECT_BASELINE` and layer `RECORD` for
    `.opsproj`. The majority rows are prose residue on SOW surface rows under
    CP-04's defaults.
  - This is not a departure, so no `CANONICAL_DEPARTURE:` marker is needed.
    Agent 0 should record it as resolved in `RESOLUTIONS.csv`.
- **Shared bodies:** all 38 shared-body rows in the package agree on
  disposition, cause, tier and layers within each body hash. I also compared
  them across every sealed W1 and W2 ledger in the run folder and found no
  conflicting row.
- **Shared situation without a keyed row:** the Python persistence hash basis
  (FG-DEL-02-02-01 against FG-DEL-02-05-03) differs in `BaselineClass` and
  layers. See §5, weak. It needs one resolution.

## 7. Reverse pass

Answers checked:
- 100% of the non-`NOT_MINE` answers: 21 `CLAIMED_BY`, 8 `PARTIAL` and 16
  `COVERS` (no `UNKEYED` or `CONSTRAINS`).
- 123 `NOT_MINE` answers, 10% of 1,225, selected deterministically.
- F5: 17 of the 81 `NOT_MINE` answers whose `EntryPoints` hit a path the
  deliverable's own forward ledger cites (21%). All 17 give a
  capability-specific reason. So do the other 64, which I read without
  counting them.

Distribution by routing (answers across the five deliverables):

| Routing | Answers | NOT_MINE | CLAIMED_BY | PARTIAL | COVERS |
|---|---|---|---|---|---|
| AREA | 1,065 | 96.4% | 1.9% | 0.5% | 1.2% |
| SAMPLE | 205 | 96.6% | 0.5% | 1.5% | 1.5% |

The distributions are close. SAMPLE rows lean slightly toward `PARTIAL` and
`COVERS`, which fits the recognisable PKG-02 paths in their entry points
(`hashService.ts`, the display-conversion commands, the e2e persistence spec,
`model_document_migration.rs`). There is no sign of anchored answers beyond
that inherent effect.

- **Claimed by more than one PKG-02 deliverable:** none as `CLAIMED_BY`.
  - RC-02-0249 (`docs/SPEC.md`) is `PARTIAL` for DEL-02-02 (SPEC §4) and for
    DEL-02-03 (§4.3). These are distinct sections, so they are consistent.
  - RC-02-0054 (`model.schema.yaml`) is `CLAIMED_BY` DEL-02-01 and `COVERS`
    DEL-02-02. That is consistent.
- **Suspected missed claims (routing gaps; confirmed against
  `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`):**
  - CAP-COREC-053, the plugin manifest verifier
    `core/adapters/framework/plugin_verification.py` and DEL-02-04's main
    implementation, is routed to PKG-04/05/06/08/10/12/14/15/17 but **not to
    PKG-02**.
  - CAP-SHELL-032, the desktop local project store commands
    `create_local_project`, `open_local_project` and `save_local_project` and
    DEL-02-05's product path, is routed to PKG-00/03/07/10/12, **not PKG-02**.
  - CAP-SHELL-048 (`atomic_report_package_save.rs`, the `.opsproj` writer) is
    **not routed to PKG-02** either.
  - R3 should check whether DEL-02-04 owns CAP-COREC-053 and DEL-02-05 owns
    CAP-SHELL-032, so they are not left unmapped.
  - RC-02-0188 (`core/gui/pkg02_boundary.py`) has three `COVERS` answers and no
    owner in PKG-02, which is expected for GUI contract code. It carries the
    literal dimension set in firm item 1.
- **Anchored answers:** none found. Reasons cite the forward row and the
  capability's own entry point, not a template.

## 8. For the owner

1. **The package false-alignment rate is close to the gate.** It is 4.3% on
   the deterministic sample and 5.7% with the worker-flagged row. The four
   firm errors are all F1 deferrals: an `ALIGNED` row that points its own
   unmet element to another row. Workers still apply F1 unevenly in wave 2.
2. **JCS hash-basis divergence, for R4.** The Python persistence service
   (`core/project_persistence/service.py`) and
   `schemas/project_persistence.schema.yaml` hash with `SORTED_COMPACT_JSON`
   and disclaim RFC 8785/JCS.
   - AB-00-04, DEC-010, DEC-017 and `docs/SPEC.md` require a JCS-compatible
     basis. The desktop product path uses RFC 8785.
   - No ruling relaxes the baseline.
   - The Python service has no product caller.
   - `docs/architecture/persistence_contract.md` still says the schema records
     JCS.
   - DEL-02-02 and DEL-02-05 record this with different baseline classes;
     that needs one resolution.
3. **Parallel literal dimension sets remain in Python validation**
   (`core/gui/pkg02_boundary.py`, `core/constraints/validation/engine.py` and
   `core/model_transform/physical_to_analytical/contract.py`), against the
   D-41 R5 T2B requirement. A parity test locks the mirrors in rather than
   removing them.
4. **Choices settled in code without a ruling (CP-10, OWNER):**
   - the DEL-02-03 status-axis split (FG-DEL-02-03-01);
   - DEL-02-01 schema layout and fixture organisation;
   - DEL-02-02 QuantityKind categories and schema layout.
5. **Implementation with no product caller.** DEL-02-04 (plugin verifier) and
   the Python half of DEL-02-05 have no product caller. About 70 aligned rows
   lack the F7 marker. Several DEL-02-05 runtime claims are met only on the
   contract reading (§5, weak).
6. **Invariant gaps:**
   - no fixture data-boundary review record for the persistence fixture
     (DEL-02-05, three `UNKNOWN` rows, `REVIEW`);
   - no independent unit-conversion witness (DEL-02-02 R05, `ENGINEERING`);
   - no staleness rule for human acceptance references (DEL-02-03 CLM-003).
7. **Rename residue in active identifiers is mostly in notes, not rows:**
   - schema `$id`s on `openpipestress.org` (model, analysis boundary, plugin
     manifest);
   - the crate `open_pipe_stress_units`;
   - the manifest and document kinds;
   - the store file `openpipestress-projects.sqlite3`, one of the four
     identifiers named in CP-04;
   - `.opsproj`, which is keyed.

   R3 should collect these from the notes files.
8. **Possible defects:**
   - the public plugin manifest fixture does not conform to its schema;
   - `tests/test_model_schema.py` validates normalised copies of the committed
     domain fixtures, not the committed bytes.
9. **Routing gaps:** see §7 (CAP-COREC-053, CAP-SHELL-032 and CAP-SHELL-048
   are not routed to PKG-02).

Scratch files (`_verify_scratch_1/`) were deleted before return.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
