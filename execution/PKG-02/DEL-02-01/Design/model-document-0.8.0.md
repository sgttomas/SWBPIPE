# Model document 0.8.0: the eccentric reducer offset (W-15 slice c)

**Basis:** main `2d4b1bb4bb` (2026-10-26).

**Status:** revision 2 of a design pass, PR #1649. It resolves Codex's design
review r1 and applies Agent 0's rulings of 2026-10-26. Built in #1649 on the
merge of main `b4b26cdd82` (#1630); §13 records what the build settled.

**Lineage:** it follows the 0.6.0 → 0.7.0 precedent of
[`model-document-0.7.0.md`](model-document-0.7.0.md) §2 (W-13 S1b, #1564). The
member it adds and its solve meaning are in DEL-03-05 Design
[`eccentric-reducer-offset.md`](../../../PKG-03/DEL-03-05/Design/eccentric-reducer-offset.md).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Paths are
relative to `projects/chirality-piping/`.

## 0. Rulings this design applies

**Rulings applied:**
- **OD-2** (owner, 2026-10-23, "I approve OD-1 through OD-5 as you
  recommended"): DEC-045 rigid and semi-rigid components, including W-15
  slice c's eccentric reducer offset (PRD §12.3).
- **OD-4** (same ruling): the desktop's model document is canonical, and new
  slots arrive in a new document version under the one-version rule
  (`docs/COMPLETENESS_MAP.md` §4.2).
- **One current version per format** (owner, 2026-10-10: "nothing needs to be
  preserved for backwards compatibility … one current version per format").
- **Owner, 2026-10-26, verbatim: "R-c1 A, 0.8.0 a2".**
  - R-c1 A fixes the offset convention (DEL-03-05 Design
    `eccentric-reducer-offset.md` §10).
  - 0.8.0 a2 governs the stored work (§3.2), as Agent 0 relayed it:
    - 0.7.0 project opens and saved states are refused by name;
    - recorded runs keep their existing read and compare behaviour, their
      own format version is unchanged, and no run gate is added;
    - the design, the user guide and the refusal message state the practical
      limit: history loads by the active project's id, so old runs are
      reachable only through a current project with the same identity, and
      comparison keeps its eligibility gate.
- **Agent 0, 2026-10-26:**
  - The offset is built first and the 0.8.0 ripple last, after #1625 and
    #1630.
  - There is one D3 turn.
- **Agent 0, 2026-10-27, `model_settings.shear_model`:** "add one optional
  member to 0.8.0 before #1649 merges ... Why now: every model-document version
  step strands older projects (0.7.0 opens are refused by name), so we take this
  field in 0.8.0 rather than spend another step on it." The member is §1.1.
- **Agent 0, 2026-10-27, setting the default (Codex r7 R7-1), option A:** "keep
  as built. An explicit user set of `euler_bernoulli` writes the member,
  matching the tolerance member's rule ("never written into a document the user
  did not author"). The product never writes the default unprompted, and clear
  restores absence." Agent 0 confirmed the precision: "results identical,
  model hash differs", because the document differs (§1.1). The regression
  `euler_bernoulli_solves_exactly_as_absent` (§12) holds it. This corrects the
  member's first specification, which said "The applier never writes the
  default".

## 1. Version 0.8.0

0.8.0 is 0.7.0 plus two optional members: `geometry.eccentric_offset` on a
component, a `VectorQuantity` `{x, y, z, unit}` of a length unit; and
`model_settings.shear_model`, the beam-theory choice described in §1.1.
- A 0.7.0 document whose version string alone is changed is a valid 0.8.0
  document with the same meaning.
- The solve reads the member only on a `reducer`, and only to check it against
  the node geometry, which governs (DEL-03-05 Design §3.2).
- No member is renamed or re-keyed (FR-MOD-003).

**Why a version** (P12 in DEL-03-05 Design §9: HELP_HUMAN, 2026-10-26, under
the owner's delegation). The schema's component geometry record is closed.
- On main the Rust record is open: a 0.7.0 build would silently drop the
  member.
- With #1630 the record is closed: a 0.7.0 build would refuse the member as
  `UNKNOWN_MODEL_FIELD`.

Either way, a document carrying the member is not a 0.7.0 document. The version
names the format.

### 1.1 `model_settings.shear_model`

0.8.0 also gains a second optional member, `model_settings.shear_model` (§0,
Agent 0, 2026-10-27): a string, one of `euler_bernoulli` or
`timoshenko_annulus_energy_v1`, the exact contract's vocabulary
(`pressure_contract.shear_model`).
- **Absent means Euler–Bernoulli.** This is the tolerance member's rule
  (`model-document-0.7.0.md` §3.8): an absent `model_settings`, or an absent
  member, means the default, and an authored `null` is refused. As Agent 0
  decided (§0, R7-1):
  - an explicit user set of `euler_bernoulli` writes the member, as the
    tolerance member's set writes an authored 5 mm;
  - the product never writes the default into a document the user did not
    author;
  - clear restores the absent form.
- **`euler_bernoulli`** solves exactly as absent: the result rows and the
  mechanics are byte-identical. The model hash differs, because the document
  differs.
- **Refusals**, named in `core/product_physics/src/model_settings.rs`. The solve
  (`validation::validate_model_settings`) and the applier's document validator
  give the same code and message.
  1. `MODEL-SETTING-SHEAR-MODEL-UNKNOWN`: any other value, a non-string or
     `null`.
  2. `MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT`: the member on a
     document that declares `pressure_contract`, for either value. That
     document's selection is `pressure_contract.shear_model`, so it has one
     source of truth.
  3. `MODEL-SETTING-SHEAR-MODEL-TIMOSHENKO-UNSUPPORTED` (the 0.8.0 interim):
     `timoshenko_annulus_energy_v1` on a pressure-free document. The solve
     refuses it by name before any route, and nothing is solved; the applier
     refuses authoring it. The document validator admits it as schema-valid, so
     such a document opens and is refused at solve. The numerical unit later
     removes this refusal from the solve and the applier.
- **Wire and authoring.** `ModelSettings.shear_model` decodes any present value,
  so the refusals above name it rather than a decode error.
  `PreviewModel::shear_model()` returns the effective choice (Euler–Bernoulli
  when absent); in 0.8.0 nothing consumes it beyond the refusals. The applier's
  `set_model_setting` and `clear_model_setting` accept
  `model_settings.shear_model`: set admits `euler_bernoulli`, writing it, and
  refuses the other cases by name; clear removes the member, and the record when
  it becomes empty. The top-level `shear_model` alias refusal (`lib.rs`
  `SHEAR_MODEL_WITHOUT_PRESSURE_CONTRACT`) is unchanged.

## 2. Code

**Constants:**
- `MODEL_SCHEMA_VERSION = "0.8.0"`, in `core/product_physics/src/model_format.rs`
  :13 and the desktop `apps/desktop/src/services/modelSchemaVersion.ts` :6.
- `RETIRED_MODEL_SCHEMA_VERSIONS` gains `"0.7.0"` (the Rust `[&str; 7]` and
  the TS list).

**Consumers that follow the constant, or are edited by token where they cannot
read it:**
- the desktop Rust evaluator (`model_document_version.rs`);
- the TS evaluator (`projectService.ts`);
- the applier's version checks.

**Readers that accept only the current version:**
- `result_export/src/physics_source.rs` :32, :838 and :1360
  (`ACTUAL_MODEL_SCHEMA`);
- `result_export/src/retained_precision.rs` :4866 and :4954, with its unit
  tests;
- their Python twins: `tests/verifiers/retained_precision.py`,
  `tests/verifiers/compatibility.py` and `tools/validation/qualification_gate.py`.

**Production literals not covered by the current guard** (Codex r1):
- `core/product_physics/src/pressure_runtime.rs` :227 ("model document 0.7.0
  accepts only 3.0.0/exact_pressure_v3 …") and :250 ("direct mechanics solves
  support exactly model version 0.7.0") take the constant.
- `tools/validation/qualification_physics_structure.py` :30 and
  `tests/verifiers/physics_source.py` :51 (`MODEL_VERSION = '0.7.0'`) follow
  the current version.
- `core/product_physics/src/retained_wire.rs` :43 `EXACT_DEFINITION_SHA256`
  changes with DEF-E (§7). It is bound by `retained_wire_tests.rs` :845, which
  is re-pinned to the regenerated definition.

**Audit beyond the named consumers.** At ripple time every file under the
project that contains `0.7.0` is listed and classified (§5): writer,
production text, test literal, current artifact, or historical exception. The
audit is part of the review evidence. Version-dependent text that does not
contain the token is found the same way, by searching for
`MODEL_SCHEMA_VERSION` and "current model". Examples are the `retained_memory.rs`
branch comments and `exact_admission.rs` :3.

**Tests that probe non-current versions** move so that they stay non-current:
- **`model_document_save.rs` :620** refuses `0.8.0`. It becomes `0.9.0`.
- **`model_document_version.rs` :171** lists `0.7.1`, `0.8.0` and `1.0.0` as
  newer. They become `0.8.1`, `0.9.0` and `1.0.0`. `" 0.7.0"` (:181) stays
  non-semver.
- **The other `0.7.1` "newer" probes** become `0.8.1`:
  - `src-tauri/src/lib.rs` :7806;
  - `projectService.test.ts` :72 and :107 (:72's target column becomes
    `0.8.0`);
  - `loadReferenceRoundTrip.test.ts` :50.
- **Unchanged:** `model_format.rs` :81 asserts only that `0.7.1` is not
  retired, which stays true. The `0.9.0` probes in `projectService.test.ts`
  :214 and the analysis-run tests (a different format) are already newer.

**Elsewhere:**
- The packaged self-test model (`src-tauri/src/lib.rs`, run from `main.rs`)
  moves to 0.8.0.
- **The wire.** `ComponentGeometryInput` gains `eccentric_offset:
  Option<VectorQuantity>` (`core/product_physics/src/lib.rs` ~507).
  - With #1630 the field takes
    `#[serde(default, deserialize_with = "model_fields::closed_optional_vector_quantity")]`,
    exactly as `center_of_gravity` does (#1630 `lib.rs` :624).
  - `model_fields.rs` declares the key on component geometry. The nested
    scan gains
    `nested_member(geometry, "eccentric_offset", fields.vector_quantity, &at, findings)`
    beside `center_of_gravity` (#1630 :528).
  - The `$defs`-bound key test's `VectorQuantity` set gains it (#1630 :807).
  - The scan's key-loop bound rises 31 → 32.
  - The tests are in DEL-03-05 Design §12, "Closed record".

## 3. A 0.7.0 document, and the stored work that holds one

### 3.1 Refused by name

The code is `MODEL_SCHEMA_VERSION_RETIRED`. Its message comes from
`retired_model_schema_message`:

> model document 0.7.0 is retired; re-author it as model document 0.8.0 (a pressure-free model omits pressure_contract). Nothing is migrated.

These surfaces call the version evaluator today, so they refuse a 0.7.0
document after the bump:

| Surface | What a 0.7.0 document gets |
|---|---|
| Solve (both routes, headless runner, wasm) | the blocking refusal above; nothing is solved |
| Desktop File › Open (`model_document_file.rs` :141) | `MODEL-FILE-SCHEMA-REFUSED`: "schema_version `0.7.0` refused (retired_schema): MODEL_SCHEMA_VERSION_RETIRED: …" |
| Local project store open (`lib.rs` ~2500) | "Model document refused for editing: status=retired_schema; source_schema_version=0.7.0; target_schema_version=0.8.0; …". The row is not rewritten. |
| Create and save (`check_model_document_for_persist`) | refused by the same text |
| Browser-preview store (`projectService.ts`) | the TS evaluator's `retired_schema` refusal |
| Saved model states (W-12, `model_states.rs` :2088, :2473) | Saving a 0.7.0 snapshot is refused. Opening or comparing a stored 0.7.0 state (`StateReadPurpose::Open` and `Compare`) is refused by `refuse_version`'s text. Rows are never rewritten. |
| Retained and source readers (`physics_source.rs`, `retained_precision.rs` and their twins) | refused: they accept only the current version |
| Model operations (`operation_applier` `run_with_context`: validate, apply and every batch step; wasm alike) | `MODEL_SCHEMA_VERSION_RETIRED` with the message above, blocking, before any change is checked or written (r6). Every other non-current version is refused there too, with the desktop's state (r7, below) |
| Python project persistence (`core/project_persistence/service.py`: validate, create, save, open) | `MODEL_SCHEMA_VERSION_RETIRED`, blocking, with the desktop's text: the message above, then §3.3's note; nothing is written (r6). Every other non-current version is refused too (r7, below) |
| Python preview service (`core/product_preview/service.py` `validate_preview_model`) | `MODEL_SCHEMA_VERSION_RETIRED`, blocking; another non-current version `PREVIEW_SCHEMA_VERSION_UNSUPPORTED` (r6) |
| CAEPIPE SI writer (`core/handoff/caepipe_mbf/si_writer.py`) | `MODEL_SCHEMA_VERSION_RETIRED` with the message above, status `refused`, no MBF text; another non-current version `MBF-SI-SCHEMA-VERSION`, now an error (r6) |

**Every non-current version (Codex r7).** The desktop's evaluator has four states, and the
applier now owns it (`operation_applier` `model_document_version.rs`; src-tauri
`model_document_version.rs` delegates to it). The TS evaluator and Python persistence
mirror it, and all three are checked against one table,
`fixtures/model_document/version_states.json`:
- `current`: the one current version;
- `retired_schema`, code `MODEL_SCHEMA_VERSION_RETIRED`: any other semver version not
  above the current one, one written with leading zeros (`00.8.0`) included. A listed
  retired version gets the message above; any other gets "model document {v} is not a
  supported version; re-author it as model document 0.8.0. Nothing is migrated.";
- `newer_than_supported`, code `MODEL_SCHEMA_VERSION_NEWER_THAN_SUPPORTED`: a semver
  version above the current one;
- `unsupported_schema`, code `MODEL_SCHEMA_VERSION_UNSUPPORTED`: a missing, non-string or
  non-semver version. Each part must be ASCII digits, as the TS and Python checks read it,
  so a signed part such as `+0.8.0` is unsupported in the native check too, where it had
  read as an older version.

Parts compare as digit strings in all three (Codex r8 R8-1, R8-2): leading zeros
dropped, then the longer part is larger, then digit by digit. Nothing converts a part to
a bounded integer, so a part of any length has one defined state everywhere. Python's
int conversion had raised on a part over 4,300 digits, and the native check had read a
part above `u64` as unsupported. The table carries leading-zero, `u64`-boundary and
4,301-digit cases.

The applier and Python persistence refuse each non-current state by its code, blocking,
before anything is written. The desktop's surfaces keep their texts.

Not gated, by design: the one document validator (`document_validator.rs`) judges a
document's model-data slots whatever its version, and every caller that admits a
document gates the version itself; the stored-document recovery export
(`stored_model_document_export.rs`) writes a refused project's or state's stored
bytes so that the user can re-author them; and the run and report readers that read
a recorded run's own model (§3.2).

### 3.2 Stored work under the owner's ruling "0.8.0 a2"

This is the first version step since W-12's saved states and recorded runs
(#1589, #1596).
- **Projects and saved states.** A saved project, or a saved model state,
  that holds a 0.7.0 model is refused by name on open, save and compare
  (§3.1). Nothing migrates it, and stored rows are never rewritten.
- **Re-authoring.** A 0.7.0 document whose `schema_version` alone is changed
  to 0.8.0 is valid with the same meaning (§1). Making that change is the
  user's re-authoring act, not a product migration.
- **Recorded runs** keep their existing read and compare behaviour. Their own
  format version is unchanged, and no model-document-version gate is added
  for them; their integrity checks stay. As found in Codex
  r1:
  - the run read (`analysis_runs.rs` `read_run_in_store_with`, :1785) checks
    the stored result, the manifest and the state summary's model hash, not
    the manifest model's version;
  - the ordinary preview-physics and pressure-2 model-bound readers
    (`u4b_evidence.rs` :478, :508; `reader_verdict.rs` :138) have no general
    model-version gate.

  Both stay as they are. 0.8.0 is purely additive (two optional members,
  absent from every 0.7.0 document), so a 0.7.0 run's results mean what they
  meant.
- **The recorded table basis (Codex r6, F1).** The ripple's token edit moved
  pressure-2's table digest (`3e2aac3c…` → `a47626ed…`), and a run record binds
  its table by digest, so an ordinary pressure-2 run recorded at 0.7.0 failed
  `ANALYSIS_SEMANTIC_CONTRACT_MISMATCH` and was refused for comparison. The two
  tables differ only in the admitted-families sentence's version token; every
  row, and so every row reading, is the same. The record validators
  (`analysisRunCompatibility.ts` and `compatibility.py`,
  `RECORDED_RUN_TABLE_BASES`) therefore read the 0.7.0-era digest as the
  recorded basis of exactly the current pressure-2 table: the record and each
  row must name it consistently, and any other digest is refused as before.
  The record builder and the export bind only the current table, and no record
  is rewritten. Regression: main's own pressure-2 record at `2e4748eada`
  (`fixtures/persistence/recorded_model_0_7_0`) reads, displays with no finding
  and is offered for comparison as a recorded revision; it failed before.
- **The practical limit.**
  - Run history loads by the active project's id
    (`apps/desktop/src/features/history/useModelStateHistory.ts` :231). A
    0.7.0 project cannot be opened, so its runs are reachable only through a
    current (0.8.0) project with the same project id.
  - Comparison keeps its eligibility gate
    (`apps/desktop/src/features/comparison/runComparison.ts` ~294).
  - Retained-precision and source results keep their existing limits: their
    readers refuse a non-current model (§3.1).

### 3.3 The refusal message and the user-guide text (implemented with the ripple)

**Every surface** keeps the core message, with version tokens from the
constants (`retired_model_schema_message`):

> model document 0.7.0 is retired; re-author it as model document 0.8.0 (a pressure-free model omits pressure_contract). Nothing is migrated.

**The desktop's project-store open and saved-state open and compare refusals**
(`refuse_model_document`, `model_states.rs` `refuse_version`, and the
browser-preview store) append the practical limit:

> Recorded runs keep their own format and existing read and compare behaviour; this change adds no model-document-version gate for recorded runs. Existing integrity checks and retained-precision/source-reader restrictions still apply. Run history is listed for the active project's id, so this project's runs are reached through a current project with the same project id, and run comparison keeps its eligibility rules.

**The user guide** (`docs/user_guide`, the section on opening projects) gains:

> **Projects saved before model document 0.8.0.** Model document 0.8.0 adds a reducer's optional eccentric offset and the optional `model_settings.shear_model` beam-theory choice (§1.1 of DEL-02-01 Design `model-document-0.8.0.md`). The desktop opens only the current model-document version, so a project or saved model state stored at 0.7.0 is refused by name when you open it, and nothing is migrated. A 0.7.0 document whose `schema_version` alone is changed to 0.8.0 has the same meaning in 0.8.0; making that change is your re-authoring step. Recorded runs keep their own format and existing read and compare behaviour; this change adds no model-document-version gate for recorded runs. Existing integrity checks and retained-precision/source-reader restrictions still apply. Run history is listed for the active project's id, so the runs of an older project are reached only through a current (0.8.0) project with the same project id. Comparing an older run with a newer one keeps the usual eligibility rules.

**Test plan** (with the ripple, through the desktop entry path, not only the
Rust read):
- a store holding a 0.7.0 project, a 0.7.0 state and an ordinary recorded run
  with intact hashes and bindings;
- the project and the state are refused by name, with the appended text;
- the project re-authored to 0.8.0 with the same identity lists the run, the
  run opens, and it compares with a new 0.8.0 run under the existing
  eligibility gate;
- a tampered hash is refused, and a retained or source result is refused as
  today.

### 3.4 Handoff packages (correction)

The desktop writes `.swbhandoff` packages and has no open path.
`tests/verifiers/handoff_package.py` `verify_handoff_package` (:330) checks a
package's internal consistency and hashes. It does not check the embedded
model's version. Two acts are distinct:
- **Verifying an old package's bytes.** This stays possible: a 0.7.0
  package's integrity still verifies.
- **Admitting its model as current.** This does not happen: any solve, open or
  store path refuses the embedded 0.7.0 model by name (§3.1).

This design changes neither.

## 4. The schema

`schemas/model.schema.yaml`:
- `schema_version` becomes `enum: ["0.8.0"]`, and the description names
  0.1.0–0.7.0 as retired.
- Component geometry gains
  `"eccentric_offset": {"$ref": "#/$defs/VectorQuantity"}`, described as a
  reducer slot that states the R-c1 convention.
- `$defs/ModelSettings` gains `shear_model` with
  `enum: [euler_bernoulli, timoshenko_annulus_energy_v1]` (§1.1). Both values
  are listed from the start, so the numerical unit needs no schema change. The
  record stays closed. Its description states §1.1's rule: absent means
  Euler–Bernoulli, an explicit set writes the member, and the product never
  writes the default into a document the user did not author.
- The schema stays structural. The kind restriction and the geometry check
  belong to the solve, as do the shear member's refusals 2 and 3.

`$defs/DesktopDocument` is unchanged.

## 5. Regenerated artifacts and the completeness guards

**The standard** (0.7.0 §2; precedent `f468da4651`):
- The version token, plus every binding digest that follows from it, is
  written by the maintained writers. No hand edits.
- No physical value, status or reader outcome moves.
- Old and new numerical leaves are compared, separately from the expected
  hash and byte changes.

**Intentional new content is kept apart.** The desktop emission golden and the
applier golden gain one reducer with `eccentric_offset` (DEL-03-05 Design §6).
That content is reviewed as an addition. The unchanged-leaf comparison covers
every other leaf.

**Positive identification of what is swept.** The guards are positive lists,
not inference from version alone.
- **Current artifact families.** Each family is named with its writer:
  - the bundled demo and product-preview fixtures, with their generation
    records;
  - the ui-foundation fixtures and manifest;
  - `t4_u5b_requests`;
  - the applier contract corpus;
  - the qualification packages and MANIFEST;
  - the reader corpora;
  - result envelopes;
  - persistence fixtures;
  - the emission goldens;
  - the retained successors (§7);
  - #1648's `fixtures/reports/invented/exact_report_package_requests/*`,
    if #1648 lands before the sweep, and the other lanes' new fixtures that
    have landed (§6).

  For each family, the review evidence records that its writer ran and that
  every output carries `schema_version` `0.8.0`, checked directly on the
  writer's output.
- **Historical and refusal exceptions.** The 0.7.0 documents that are meant
  to stay at 0.7.0 are listed explicitly, each with its reason:
  - SHA-bound frozen requests under `validation/**`, re-authored in memory by
    the shared helpers;
  - any 0.7.0 refusal control added by this ripple.
- **`tests/test_model_document_schema.py`.**
  - Adding 0.7.0 to its `RETIRED_VERSIONS` would silently classify an
    unswept, non-frozen 0.7.0 document as a `retired` refusal control
    (:217).
  - The ripple therefore adds a rule: a non-frozen document at 0.7.0 is
    classified `retired` only when it is on an explicit 0.7.0 exception
    list. Any other is `unclassified`, and the test fails on it.
  - The 0.1.0–0.6.0 classification is unchanged.
- **`tests/test_model_version_literals.py`.**
  - It scans desktop and applier production code only (:104).
  - The ripple extends its scan to the production sources of
    `core/product_physics/src` and `core/reporting/result_export/src`, and
    to the Python verifiers and validation tools that state a model version.
  - Explicit exceptions are allowed: the retired lists, and comments that
    record history.

**What the guards establish.** Every named family was regenerated at 0.8.0.
Every remaining 0.7.0 document is a listed exception. No unlisted literal
remains in the scanned production code. They do not establish that no other
version-dependent text exists anywhere. The audit (§2) is the evidence for
that, and is reviewed.

**Frozen references.** SHA-bound requests keep their bytes. The re-author
helpers (`tests/common/current_model.rs`, its lib twin and
`qualification_physics.reauthor_model`) gain 0.7.0 among the versions they
re-author by token. `twins.py` and the t4_u5b builder follow the current
version.

**The version token keeps its length** ("0.7.0" → "0.8.0"), so no
protected-byte count or length literal moves; only hashes do.

## 6. Fixtures added by other lanes

The ripple runs last, on the latest main, after #1625 and #1630 (Agent 0,
2026-10-26). It sweeps whatever has landed at 0.7.0 by then, including any
other lane's new tests and references (for example #1615's, if it has
landed) and #1648's report requests. **Agent 0 is told before any of #1648's files are
touched**, whether #1648 has landed or not.

## 7. The D3 chain

This repeats 0.7.0 §2's four steps.

**1. Price.** These pinned or priced files move:
- `model_format.rs`;
- `lib.rs` (the wire field and closed deserializer);
- `pressure_runtime.rs` (the :227 and :250 texts);
- `retained_wire.rs` (`EXACT_DEFINITION_SHA256`, which follows DEF-E);
- `physics_source.rs`, `retained_precision.rs` and `semantic_contract.rs`
  (the version tokens);
- `exact_admission.rs` (a comment);
- `rigid_component.rs` and `model_fields.rs` (DEL-03-05 Design §8);

`retained_wire.rs` and `pressure_runtime.rs` are named in the D3 review.

**2. Register.** The reviewed inputs are edited by token only, 0.7.0 → 0.8.0:
- DEF-E `scope.model_format`;
- the pressure-source-1 `supported_profile_limitations[0]`;
- the pressure-2 sentence "Admitted families under current model 0.7.0 and
  3.0.0/exact_pressure_v3: …", regenerated with `semantic_contract_tables.py`
  `derive` and `repin`.

TBD-002's sentence stays untouched. `retained_memory.rs`'s registration is
rebound to the new digests.

**3. Regenerate the successors publicly** on the registered build:
- `retained_precision_*successor*`;
- `retained_precision_cases.json`;
- the two `failed_ordinary` fixtures.

**4. Update the synthetic-bases writer**
(`tools/fixtures/retained_precision_synthetic_bases.py`).

**Then:**
- `compare` against main's registered tree, on the final merged source. The
  expected outcome is `identical_but_basis` for the token, but it is a
  hypothesis to verify, not an assumption.
- The offset's pricing is DEL-03-05 Design §8, and both are registered in the
  same turn.
- The AC-004 stack witnesses run under an exclusive lane granted by Agent 0.
- The registration gets independent review.

## 8. Authoring

The offset's applier, adoption checks, library apply, inspector and report are
covered in DEL-03-05 Design §4. The document validator does not validate
component geometry. For `model_settings.shear_model`, §1.1 adds the named
value and pressure-contract checks; on a pressure-free document, the validator admits Timoshenko as schema-valid
and the solve and applier refuse it during the 0.8.0 interim.

## 9. `apps/desktop/native-smoke/expected.json`

No value is expected to move:
- the file pins no version or hash;
- `run.mjs` hashes the files it opens;
- the newer-version case writes `9.0.0`;
- the refusal codes are unchanged;
- the demo moves by token only (rows 837 and standing unchanged), and the
  milestone is unchanged.

A native smoke run on the build head confirms this and reports the counts. Any
value that moves is reported to the owner and not edited.

## 10. Version-token text (old → new)

These change only to describe the version that follows from owner decisions
already made (OD-2's W-15 slice c and the one-version rule). That makes them
HELP_HUMAN's under the owner's grant of 2026-10-19, with the exact text in the
PR and an independent review. Line numbers are at the basis. Each edit
replaces only the quoted text.

1. **DEL-02-05 ScopeOfWork**
   - **Line 33.**
     - **Old:** `` `0.7.0`, defined by DEL-02-01 Design `model-document-0.7.0.md` under the owner's OD-4 ruling (2026-10-23), ``
     - **New:** `` `0.8.0`, defined by DEL-02-01 Design `model-document-0.7.0.md` and `model-document-0.8.0.md` under the owner's OD-4 ruling (2026-10-23) and W-15 slice c (OD-2), ``
   - **Line 34.**
     - **Old:** `` (`0.1.0`, `0.2.0`, `0.3.0`, `0.4.0`, `0.5.0`, `0.6.0`, or newer) ``
     - **New:** `` (`0.1.0`, `0.2.0`, `0.3.0`, `0.4.0`, `0.5.0`, `0.6.0`, `0.7.0`, or newer) ``
   - **Line 35.**
     - **Old:** `` `current` for `0.7.0`; ``
     - **New:** `` `current` for `0.8.0`; ``
   - **Line 37**, two edits.
     - **Old:** `` Current code: `0.7.0` is the one current version ``
     - **New:** `` Current code: `0.8.0` is the one current version ``
     - **Old:** `` Current implementation (W-13 S1b, #1564): `RETIRED_MODEL_SCHEMA_VERSIONS` in `model_format.rs` names `0.1.0`, `0.2.0`, `0.3.0`, `0.4.0`, `0.5.0` and `0.6.0`, and the `model_document_version.rs` tests refuse all six by name. ``
     - **New:** `` Current implementation (W-15 slice c, #<PR>): `RETIRED_MODEL_SCHEMA_VERSIONS` in `model_format.rs` names `0.1.0`, `0.2.0`, `0.3.0`, `0.4.0`, `0.5.0`, `0.6.0` and `0.7.0`, and the `model_document_version.rs` tests refuse all seven by name. ``
   - **Line 38** is a dated approval record and stays unchanged.
   - **Line 179.**
     - **Old:** `` and only `0.7.0` reports `current` ``
     - **New:** `` and only `0.8.0` reports `current` ``
   - **Insert after line 35** (the ruling's effect on stored work; through
     Codex, then Agent 0 under the wording grant):
     `> - Owner ruling, 2026-10-26: "R-c1 A, 0.8.0 a2". A project or saved model state holding a retired model-document version is refused by name on open; recorded runs keep their own format and their existing read and compare behaviour, with no model-document-version gate for recorded runs; existing integrity checks and retained-precision/source-reader restrictions still apply. Run history is listed for the active project's id, so a retired project's runs are reached through a current project with the same identity, and comparison keeps its eligibility gate (DEL-02-01 Design `model-document-0.8.0.md` §3).`
2. **DEL-04-08 ScopeOfWork**
   - **Lines 108 and 122.**
     - **Old:** `current model 0.7.0`
     - **New:** `current model 0.8.0`
   - **Line 262.**
     - **Old:** `DEF-E and the code admit current model 0.7.0 with`
     - **New:** `DEF-E and the code admit current model 0.8.0 with`
     - TBD-002's held sentence is unchanged.
   - #1648 edits other lines of this file, so Agent 0 is told before the edit
     (§6).
3. **DEL-03-01 ScopeOfWork line 145**
   - **Old:** `` is `0.7.0` and `0.1.0`–`0.6.0` are refused by name ``
   - **New:** `` is `0.8.0` and `0.1.0`–`0.7.0` are refused by name ``
4. **DEL-02-01 ScopeOfWork line 93** (current-implementation wording, under
   the owner's grant of 2026-10-23, after independent review)
   - **Old:** `` Current implementation (S1a and S1b, 2026-10-23; Design `Design/model-document-0.7.0.md` §2-§4): `schemas/model.schema.yaml` describes the desktop's model document, `document_kind` `openpipestress.product_preview.model` at version `0.7.0` (owner OD-4, 2026-10-23: the desktop's model document is canonical; 0.6.0 and older are refused by name). ``
   - **New:** `` Current implementation (S1a and S1b, 2026-10-23, and W-15 slice c; Design `Design/model-document-0.7.0.md` §2-§4 and `Design/model-document-0.8.0.md`): `schemas/model.schema.yaml` describes the desktop's model document, `document_kind` `openpipestress.product_preview.model` at version `0.8.0` (owner OD-4, 2026-10-23: the desktop's model document is canonical; the desktop open/save and mechanics solve paths refuse 0.7.0 and older by name). 0.8.0 adds the reducer's optional `geometry.eccentric_offset` (DEL-03-05 Design `eccentric-reducer-offset.md`) and the optional `model_settings.shear_model` (Design `model-document-0.8.0.md` §1.1). ``
5. **Reconciliation pointers** (not commitments):
   - the `deliverable.yaml` conditions DEL-02-05:128, DEL-04-07:7,
     DEL-10-05:48 and DEL-12-01:98;
   - `b7-plan.md` "Current model format";
   - `RELEASE_NOTES_v0.1.0_DRAFT.md`;
   - `docs/user_guide`.

No acceptance criterion changes in substance.

## 11. Coordination and sequencing

**No other lane plans 0.8.0 members.** #1615, #1618 and #1648 add no
model-document member and change no model version. The map's "0.8.0
coordination" refers to this unit. Any unit that needs a member before this
ripple lands joins 0.8.0 through Agent 0.

**Order** (Agent 0, 2026-10-26):
1. Build the offset on current main, with constants at 0.8.0 and focused
   tests.
2. The ripple, last, on the latest main after #1625 and #1630.
3. One D3 turn and registration for both.

## 12. Checks

- **Conformance.** Every current document validates at 0.8.0. A document
  carrying `eccentric_offset` validates in both profiles. The explicit 0.7.0
  exception list is enforced, and an unlisted 0.7.0 document fails (§5).
- **Writers.** Each family's writer output is checked directly for 0.8.0.
- **Refusal on every §3.1 surface**, one test each. `model_document_version.rs`
  and `model_format.rs` refuse all seven retired versions by name.
- **Ruling a2.** §3.3's test plan: projects and states refused with the
  appended text; old ordinary runs with intact hashes listed under a re-authored
  project, read and compared under the existing gate; a tampered hash refused.
- **Handoff.** An old package's integrity verifies, and its embedded model is
  refused by the solve and the open paths.
- **r6 surfaces.** One regression each, failing before its fix: the 0.7.0 pressure-2
  record read and recorded revision (TS and Python); the applier's validate, apply
  and batch; Python persistence validate, create, save and open; the preview
  service; the CAEPIPE SI writer. Every Python retired list equals
  `model_format.rs`'s (`test_model_version_literals.py`).
- **r7 surfaces.** Every state of the shared version table, at the applier
  (validate, apply and batch, for an offset edit and a model-setting edit;
  `tests/retired_model_version.rs`) and at Python persistence (validate, create,
  save and open; `test_project_persistence_service.py`), each failing before its
  fix. The applier, the native desktop, the TS evaluator and Python persistence
  each give the table's states.
- **`model_settings.shear_model` (§1.1).** Every existing document gives
  byte-identical results (`pressure_free_byte_identity`), and `euler_bernoulli`
  gives the byte-identical solve envelope (rows, mechanics and diagnostics) of its
  twin without the member, while the two documents, and so their model hashes,
  differ (`model_settings_shear_model.rs` `euler_bernoulli_solves_exactly_as_absent`). Each refusal is tested
  at solve, and refusals 1 and 2 at the document validator; the applier sets,
  clears and refuses (`operation_applier/tests/model_settings_authoring.rs`); a
  set member saves and reopens byte for byte (src-tauri `model_document_save.rs`).
- **Literals.** The extended literal guard passes, and the audit list is
  reviewed.
- **Leaf comparison.** 0 numeric, boolean or status differences outside the
  intentional golden additions, which are reviewed separately.
- **D3.**
  - `price.py run` and `compare` on the final source;
  - `generate --check`;
  - the `retained_wire_tests.rs` :845 pin;
  - the AC-004 witnesses under the exclusive lane.
- **Suites.**
  - the affected Rust crates;
  - `piping-pytest` under `t3_slot.sh`, `-n 4` at most;
  - `desktop-test`;
  - the native smoke compared with main;
  - the hosted `harness` on the exact head.
- **Independent review** of the regeneration and the registration.

## 13. As built (the ripple, #1649)

- **Refusal note.** §3.3's appended text is `RETIRED_PROJECT_RUNS_NOTE`
  (src-tauri `model_document_version.rs`, mirrored in `modelSchemaVersion.ts`).
  It is appended to a `retired_schema` refusal only: the project-store open,
  create and save refusal (`refuse_model_document`), the saved-state save, open
  and compare refusal (`model_states.rs` `refuse_version`), and the browser
  store and saved-state open. A newer or non-semver document's refusal is
  unchanged, since the note concerns stored work of a retired version.
- **Literal guard (§5).** The independent readers' current-version gates and
  texts (`physics_source.rs` 3, `retained_precision.rs` 2, `semantic_contract.rs`
  1) stay token edits, as §2 and §7 plan. They are the guard's one listed
  exception, `READER_TOKEN_SITES`, tied to the version it was taken at, so the
  next ripple fails until it re-checks each. The Python verifiers and tools name
  the version once each, as a module-level `MODEL_VERSION` that every gate and
  message reads; the guard checks that every such definition is current.
- **Committed 0.7.0 control (§5, §12 "Handoff").** One: the handoff witness
  package as the 0.7.0 producer wrote it,
  `fixtures/handoff/retired_0_7_0_witness.swbhandoff`. Its bytes verify; the open
  path and the solve refuse its model by name. It is the only entry of
  `EXPLICIT_RETIRED_CONTROLS["0.7.0"]`.
- **DEF-E's domain hash (§7).** DEF-E's token edit moves its formation-definition
  hash (`retained_wire.rs` `EXACT_DEFINITION_SHA256`, `bd3c7de7…` → `913d248b…`),
  which physics-retained-1's `product_formation_definitions` and the readers
  carry. The registration's four moved reviewed inputs are DEF-E (#7),
  physics-retained-1 (#8), pressure-source-1 (#9) and pressure-2 (#11), counted
  from 0.
- **Ledger control.** As at 0.6.0 and 0.7.0, the `COMMITMENT_DOMAIN_SEPARATION`
  ledger case copies the regenerated mixed source's functional-plan hash, so its
  base reader outcome stays the named refusal.

- **D3 (§7).** Priced and registered with the offset as DEL-04-08
  `retained-pricing-rules.md` §8.26, on main `289bd8d546`. `compare` against
  main's registered tree is `identical_but_basis` (1,586 coefficients), as §7
  expected. It needed `validate_profile`'s two formatted version texts to be
  zero on D1 (M08-VZ), and the offset check to read the tolerance after its
  first pass. The four reviewed inputs above move, and the successors and the
  synthetic bases were regenerated on the registered build.
- **D3 with `model_settings.shear_model` (§1.1).** The member's refusal is a
  second blocking diagnostic in `validate_model_settings`, reachable on D1 as
  the tolerance's is, so coefficients move: re-priced at `642e9737e4` and
  re-registered in §8.26 ("The shear-model member"), 92 coefficients higher and
  the merged check +98,134 B; the tightest native headroom is still positive
  (merged dense W3, +2,361,079,335 B). The member's decoder builds no value.
  The six AC-004 stack witnesses passed 6/6 on this build, at head
  `c83cf828cc` (registered source `95aacdf852`), under the exclusive lane, by
  `python3 tools/release/check_release_readiness.py --profile stack-witnesses
  --execute`. This is test evidence, not an acceptance.
