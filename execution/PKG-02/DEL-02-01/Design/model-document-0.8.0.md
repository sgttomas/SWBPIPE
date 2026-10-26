# Model document 0.8.0: the eccentric reducer offset (W-15 slice c)

**Basis:** main `2d4b1bb4bb` (2026-10-26). Design pass, not built. It follows
the 0.6.0 → 0.7.0 precedent: [`model-document-0.7.0.md`](model-document-0.7.0.md)
§2, built in W-13 S1b, #1564. The member it adds and its solve meaning are
DEL-03-05 Design [`eccentric-reducer-offset.md`](../../../PKG-03/DEL-03-05/Design/eccentric-reducer-offset.md).
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

Paths are relative to `projects/chirality-piping/`.

## 0. Rulings this design applies

- **OD-2** (owner, 2026-10-23, "I approve OD-1 through OD-5 as you
  recommended"): DEC-045 rigid and semi-rigid components, including W-15
  slice c's eccentric reducer offset (PRD §12.3).
- **OD-4** (same ruling): the desktop's model document is canonical, and
  `schemas/model.schema.yaml` describes it. New slots arrive in a new document
  version under the one-version rule (`docs/COMPLETENESS_MAP.md` §4.2, OD-4
  consequence).
- **One current version per format** (owner, 2026-10-10: "nothing needs to be
  preserved for backwards compatibility … one current version per format"):
  0.8.0 becomes the only version the product reads and writes. 0.7.0 is
  refused by name with the re-author diagnostic, like 0.1.0–0.6.0. Nothing is
  migrated.

## 1. Version 0.8.0

0.8.0 is 0.7.0 plus one optional member, `eccentric_offset`, on the component
`geometry` record: a `VectorQuantity` `{x, y, z, unit}` of a length unit.
- A 0.7.0 document whose version string alone is changed is a valid 0.8.0
  document with the same meaning.
- The solve reads the member only on a `reducer`, and only to check it against
  the node geometry (DEL-03-05 Design §3.2). No other member is added, renamed
  or re-keyed, so FR-MOD-003 holds.

**Why a version (P12 in DEL-03-05 Design §9).** The component geometry record
is closed in the schema (`additionalProperties: false`, ~355).
- On main the Rust component record is open: unknown keys are ignored, so a
  0.7.0 build would silently drop the member.
- #1630 (draft, in the D3 queue) closes the nested input records, including
  component geometry. A 0.7.0 build would then refuse the member by name, as
  `UNKNOWN_MODEL_FIELD`.

In either case a document carrying the member is not a 0.7.0 document. The
version names the format, as at 0.7.0, so the member arrives at 0.8.0. Adding
it to 0.7.0 in place was considered and rejected: two different closed formats
would share one version string.

## 2. Code

- **Constants.**
  - `MODEL_SCHEMA_VERSION = "0.8.0"` (`core/product_physics/src/model_format.rs`
    :13; desktop `apps/desktop/src/services/modelSchemaVersion.ts`:6).
  - `RETIRED_MODEL_SCHEMA_VERSIONS` gains `"0.7.0"` (Rust `[&str; 7]`; TS list).
  - The module documentation names 0.8.0 as current.
- **Shared consumers.** These follow the constant, as in 0.7.0 §2:
  - the desktop Rust evaluator (`src-tauri/src/model_document_version.rs`);
  - the TypeScript evaluator (`projectService.ts`);
  - the applier's version checks;
  - the readers that accept only the current version. Their literal gates
    become the current version: `result_export/src/physics_source.rs` :32,
    :838, :1360 (`ACTUAL_MODEL_SCHEMA`), and `retained_precision.rs` :4866,
    :4954 with their unit tests. Where the crate cannot read the constant,
    the literal is edited by token. The Python twins follow
    (`tests/verifiers/retained_precision.py`, `tests/verifiers/compatibility.py`,
    `tools/validation/qualification_gate.py`).
- **Guard.** `tests/test_model_version_literals.py` already fails on a literal
  model version outside the constant definitions and the retired lists. The
  ripple is complete when it passes.
- **Tests that use 0.8.0 as a non-current version** move to a version that
  stays non-current:
  - `src-tauri/src/model_document_save.rs` :620 refuses `0.8.0`; it becomes
    `0.9.0`;
  - `src-tauri/src/model_document_version.rs` :171 lists `0.7.1`, `0.8.0`
    and `1.0.0` as newer. They become `0.8.1`, `0.9.0` and `1.0.0`, because
    `0.7.1` would now be an older, unsupported version. `" 0.7.0"` in the
    unsupported-version test (:181) still has a leading space and stays
    non-semver.
  - The other "newer" probes at `0.7.1` must move above 0.8.0, for example
    to `0.8.1`:
    - `src-tauri/src/lib.rs` :7806;
    - `projectService.test.ts` :72 and :107 (the target column at :72 becomes
      `0.8.0`);
    - `loadReferenceRoundTrip.test.ts` :50.
  - `model_format.rs` :81 asserts only that `0.7.1` is not retired. That
    stays true, so the test is unchanged; `0.8.1` may be added beside it.
  - The `0.9.0` probes are already newer and stay as they are:
    `projectService.test.ts` :214, and the analysis-run schema tests, which
    are a different format.
- **The packaged self-test model** (`src-tauri/src/lib.rs`, run from
  `main.rs`) moves to 0.8.0.
- **Texts that name the version** take the constant. The blank project name
  and the diagnostics listed in 0.7.0 §2 "Texts and emitted data" already do.
- **The wire.** `ComponentGeometryInput` gains
  `eccentric_offset: Option<VectorQuantity>`
  (`core/product_physics/src/lib.rs` ~507). If #1630 has landed,
  `model_fields.rs` declares the key on component geometry, and its
  `$defs`-bound key test gains it in the `VectorQuantity` set (#1630 :807).

## 3. A 0.7.0 document is refused by name

The code is `MODEL_SCHEMA_VERSION_RETIRED`. The message is built by
`retired_model_schema_message`:

> model document 0.7.0 is retired; re-author it as model document 0.8.0 (a pressure-free model omits pressure_contract). Nothing is migrated.

| Surface | What a 0.7.0 document gets |
|---|---|
| Solve (both routes, headless runner, wasm) | the blocking `MODEL_SCHEMA_VERSION_RETIRED` with the message above. Nothing is solved. |
| Desktop File › Open (`model_document_file.rs` :141) | `MODEL-FILE-SCHEMA-REFUSED`: "schema_version `0.7.0` refused (retired_schema): MODEL_SCHEMA_VERSION_RETIRED: …" |
| Local project store open (`open_project_in_store`, `lib.rs` ~2500) | "Model document refused for editing: status=retired_schema; source_schema_version=0.7.0; target_schema_version=0.8.0; MODEL_SCHEMA_VERSION_RETIRED: …". The stored row is not rewritten. |
| Create and save (`check_model_document_for_persist`) | refused by the same text. A 0.7.0 document is never written. |
| Browser-preview store (`projectService.ts`) | the TS evaluator's `retired_schema` refusal, with the same message |
| Saved model states (W-12, `model_states.rs` :2088, :2473) | saving a 0.7.0 snapshot is refused. Opening or comparing a stored 0.7.0 state (`StateReadPurpose::Open` and `Compare`) is refused by `refuse_version`'s text. Rows are never rewritten. |
| Recorded runs | a run whose state holds 0.7.0 cannot be compared, through its state read. The run row is kept as stored. |
| Saved result exports and report packages | the model-bound readers refuse an embedded 0.7.0 model (`ACTUAL_MODEL_SCHEMA` and its twins). They are never re-read as current. |
| Handoff packages (`.swbhandoff`) | The desktop only writes them; it has no open path. A package saved earlier keeps its 0.7.0 model. The Python verifiers treat it as retired. |

**The user's stored work (consequence for Agent 0 to relay).** This is the
first version step since W-12's saved states and recorded runs (#1589,
#1596). Under the one-version rule, every saved project, state and run that
holds a 0.7.0 model is refused on open after the update, and nothing migrates
it. A 0.7.0 document whose `schema_version` alone is changed to 0.8.0 is valid
with the same meaning (§1). Making that change is the user's re-authoring act,
not a product migration. The user guide and release notes say so.

## 4. The schema

`schemas/model.schema.yaml`:
- `schema_version` `enum: ["0.8.0"]`, and its description names 0.1.0–0.7.0
  as retired;
- component geometry gains `"eccentric_offset": {"$ref": "#/$defs/VectorQuantity"}`,
  with a description stating the R-c1 convention and that it is a reducer slot.
  The schema stays structural: the kind restriction and the geometry check are
  the solve's (DEL-03-05 Design §3.2, §3.4).

`$defs/DesktopDocument` is unchanged: the member is optional. The
conformance pytest (`tests/test_model_document_schema.py`) validates a
committed document carrying it.

## 5. Regenerated artifacts

**The standard** is 0.7.0 §2's, with precedent `f468da4651`:
- the version token plus every binding digest that follows from it, written by
  the maintained writers, with no hand edits;
- no physical value, status or reader outcome moves;
- each regeneration is checked by comparing old and new numerical leaves
  separately from the expected hash and byte changes.

**The scope** is every committed current document and everything that binds
one. At the basis that is 276 files carrying a 0.7.0 `schema_version`, and
about 430 files carrying the token anywhere, the same families as #1564's 553
files:
- the bundled demo, the product-preview fixtures and their generation records
  (`demo_fixture_generation.json`, `preview_physics_fixture_generation.json`);
- the ui-foundation fixtures and their manifest;
- `core/runner/headless/tests/t4_u5b_requests`;
- the applier contract corpus;
- the qualification packages and MANIFEST;
- the reader corpora;
- result envelopes (`model_hash`);
- persistence binding hashes and test literals;
- the desktop emission golden and the applier golden.

**Frozen references.** SHA-bound requests under `validation/**` keep their
bytes. The shared re-author helpers (`tests/common/current_model.rs`, its lib
twin, `qualification_physics.reauthor_model`) gain 0.7.0 among the versions
they re-author by token. `twins.py` and the t4_u5b builder follow the current
version.

**The version token keeps its length** ("0.7.0" → "0.8.0"). No protected-byte
count or length literal moves; only hashes do. This differs from a sentence
edit such as #1615's.

## 6. Fixtures added by other lanes

Every document committed at 0.7.0 before the ripple runs is swept by it. Open
PRs add such documents, including #1615's W-16 tests and references and
#1648's exact report-package requests. The ripple is therefore regenerated on
the latest main immediately before its D3 turn (§11), and
`tests/test_model_version_literals.py` together with the conformance pytest
find any document left behind.

## 7. The D3 chain

This repeats the four steps of 0.7.0 §2 "Retained and source successors".

**1. Price.** These pinned files move:
- `model_format.rs`;
- `lib.rs` (the wire field);
- `physics_source.rs`, `retained_precision.rs` and `semantic_contract.rs`
  (the version tokens);
- `exact_admission.rs` (a comment);
- `rigid_component.rs` (DEL-03-05 Design §8);
- `model_fields.rs`, if #1630 has landed.

**2. Register.** The reviewed inputs are edited by token only, 0.7.0 → 0.8.0,
with no other wording change:
- DEF-E `scope.model_format`;
- `supported_profile_limitations[0]` of the pressure-source-1 table;
- the pressure-2 sentence "Admitted families under current model 0.7.0 and
  3.0.0/exact_pressure_v3: …" (`semantic_contract_v0_3_pressure_2.json`). The
  table is regenerated with `semantic_contract_tables.py` `derive` and
  `repin`.

The physics-retained-1 `accuracy_classification.scope` sentence is TBD-002 and
stays untouched. `retained_memory.rs`'s registration is rebound to the new
digests.

**3. Regenerate the successors publicly** on the registered build, with the
maintained writers:
- the `fixtures/results/retained_precision_*successor*` files;
- `retained_precision_cases.json`;
- the two `failed_ordinary` fixtures.

**4. Update the synthetic-bases writer.** It hard-codes the version step
(`tools/fixtures/retained_precision_synthetic_bases.py`).

The re-pricing is expected `identical_but_basis` for the token, as in S1b
(§8.17). The offset's own pricing is DEL-03-05 Design §8. Both are registered
in one turn, then the AC-004 stack witnesses run under an exclusive lane
granted by Agent 0, and the registration gets independent review.

## 8. Authoring

The applier field rule, creation and inspector are DEL-03-05 Design §4. The
document validator gains no rule. The emission goldens gain one reducer
carrying the member, so the conformance pytest exercises it in both profiles.

## 9. `apps/desktop/native-smoke/expected.json`

No value is expected to move. The file pins no version or hash. `run.mjs`
hashes the files it opens. The newer-version case writes `9.0.0`, which is
still newer than 0.8.0. The refusal codes are unchanged. The demo moves by
token only, so its rows (837) and standing do not change, and neither do the
milestone probe and rows. A native smoke run on the build head confirms this
and reports the counts. Any value that moves is reported to the owner and not
edited.

## 10. Version-token text (old → new)

These change only to describe the version that follows from an owner decision
already made (OD-2's W-15 slice c and the one-version rule). They are
therefore HELP_HUMAN's under the owner's grant of 2026-10-19, with the exact
old and new text in the PR and an independent review. Line numbers are at the
basis. Each edit replaces only the quoted text.

1. **DEL-02-05 ScopeOfWork:**
   - **Line 33.**
     - **Old:** `` `0.7.0`, defined by DEL-02-01 Design `model-document-0.7.0.md` under the owner's OD-4 ruling (2026-10-23), ``
     - **New:** `` `0.8.0`, defined by DEL-02-01 Design `model-document-0.7.0.md` and `model-document-0.8.0.md` under the owner's OD-4 ruling (2026-10-23) and W-15 slice c (OD-2), ``
   - **Line 34.**
     - **Old:** `` (`0.1.0`, `0.2.0`, `0.3.0`, `0.4.0`, `0.5.0`, `0.6.0`, or newer) ``
     - **New:** `` (`0.1.0`, `0.2.0`, `0.3.0`, `0.4.0`, `0.5.0`, `0.6.0`, `0.7.0`, or newer) ``
   - **Line 35.**
     - **Old:** `` `current` for `0.7.0`; ``
     - **New:** `` `current` for `0.8.0`; ``
   - **Line 37.**
     - **Old:** `` Current code: `0.7.0` is the one current version ``
     - **New:** `` Current code: `0.8.0` is the one current version ``
     - **Old:** `` Current implementation (W-13 S1b, #1564): `RETIRED_MODEL_SCHEMA_VERSIONS` in `model_format.rs` names `0.1.0`, `0.2.0`, `0.3.0`, `0.4.0`, `0.5.0` and `0.6.0`, and the `model_document_version.rs` tests refuse all six by name. ``
     - **New:** `` Current implementation (W-15 slice c, #<PR>): `RETIRED_MODEL_SCHEMA_VERSIONS` in `model_format.rs` names `0.1.0`, `0.2.0`, `0.3.0`, `0.4.0`, `0.5.0`, `0.6.0` and `0.7.0`, and the `model_document_version.rs` tests refuse all seven by name. ``
   - **Line 38** is a dated approval record. It stays unchanged.
   - **Line 179.**
     - **Old:** `` and only `0.7.0` reports `current` ``
     - **New:** `` and only `0.8.0` reports `current` ``
2. **DEL-04-08 ScopeOfWork:**
   - **Lines 108 and 122.**
     - **Old:** `current model 0.7.0`
     - **New:** `current model 0.8.0`
   - **Line 262** (TBD-002 status).
     - **Old:** `DEF-E and the code admit current model 0.7.0 with`
     - **New:** `DEF-E and the code admit current model 0.8.0 with`
     - TBD-002's held sentence is unchanged.
   - #1648 edits other lines of this file (216, AC-006). The edits do not
     overlap.
3. **DEL-03-01 ScopeOfWork line 145** (REQ-03-01-007).
   - **Old:** `` is `0.7.0` and `0.1.0`–`0.6.0` are refused by name ``
   - **New:** `` is `0.8.0` and `0.1.0`–`0.7.0` are refused by name ``
4. **DEL-02-01 ScopeOfWork line 93** (current implementation; reconciliation
   wording under the owner's grant of 2026-10-23, after independent review).
   - **Old:** `` Current implementation (S1a and S1b, 2026-10-23; Design `Design/model-document-0.7.0.md` §2-§4): `schemas/model.schema.yaml` describes the desktop's model document, `document_kind` `openpipestress.product_preview.model` at version `0.7.0` (owner OD-4, 2026-10-23: the desktop's model document is canonical; 0.6.0 and older are refused by name). ``
   - **New:** `` Current implementation (S1a and S1b, 2026-10-23, and W-15 slice c; Design `Design/model-document-0.7.0.md` §2-§4 and `Design/model-document-0.8.0.md`): `schemas/model.schema.yaml` describes the desktop's model document, `document_kind` `openpipestress.product_preview.model` at version `0.8.0` (owner OD-4, 2026-10-23: the desktop's model document is canonical; 0.7.0 and older are refused by name). 0.8.0 adds the reducer's optional `geometry.eccentric_offset` (DEL-03-05 Design `eccentric-reducer-offset.md`). ``
   - The paragraph's next sentence, "0.7.0 adds optional slots …", stays as
     history of what 0.7.0 added.
5. **Reconciliation pointers** (not commitments):
   - the `deliverable.yaml` conditions naming 0.7.0: DEL-02-05:128,
     DEL-04-07:7, DEL-10-05:48 and DEL-12-01:98;
   - DEL-04-08 Design `b7-plan.md` "Current model format";
   - `RELEASE_NOTES_v0.1.0_DRAFT.md`;
   - `docs/user_guide`.

No acceptance criterion changes in substance.

## 11. Coordination and sequencing

- **No other lane plans 0.8.0 members.**
  - The open solver-lane PRs, #1615 (W-16 a) and #1618 (the W-16 stage 2
    design), and the reports lane's #1648 add no model-document member and
    change no model version. Their diffs against main carry no `0.8.0` and no
    `MODEL_SCHEMA_VERSION` change.
  - The map's "0.8.0 coordination after slice a's D3 turn" refers to this
    unit.
  - Any unit that needs a member before this ripple lands joins 0.8.0 through
    Agent 0 rather than making 0.9.0. W-13 S1b did the same for "the
    rigid-element unit's fields".
- **#1630** closes the component record. Whichever lands second adds
  `eccentric_offset` to `model_fields.rs`'s declared set and the key-loop bound
  (§2).
- **Order.**
  1. Build the offset itself (DEL-03-05 Design §3, §4, §7) on the current
     main, with code constants at 0.8.0 and focused tests.
  2. Regenerate the version ripple (§5–§7) last, by the maintained writers, on
     the latest main. This happens after the D3 queue ahead of it (#1625,
     #1630, #1615) has landed and immediately before this unit's D3 turn.
  3. Run one registration for both.

  The ripple touches nearly every file the in-flight lanes regenerate:
  successors, generation records, persistence fixtures, report requests and
  routing digests. Doing it last means writers regenerate those files once,
  instead of resolving them by hand in merges.

## 12. Checks

- **Conformance.** The schema conformance pytest covers every committed
  document at 0.8.0, a document carrying `eccentric_offset` in both profiles,
  and retired-version documents excluded by rule.
- **Refusal on every surface** (§3), one test each:
  - the solve, the headless runner and wasm;
  - native File › Open;
  - store open;
  - create and save;
  - the browser-preview store;
  - state save, open and compare;
  - the model-bound result readers.
  `model_document_version.rs` and `model_format.rs` refuse all seven retired
  versions by name.
- **Literal guard.** `tests/test_model_version_literals.py` passes.
- **Leaf comparison** of every regenerated artifact: 0 numeric, boolean or
  status differences, with the moved tokens and digests counted.
- **D3.** `price.py run`, `compare` against main's registered tree,
  `generate --check`, and the AC-004 witnesses under the exclusive lane.
- **Suites.** The affected Rust crates, `piping-pytest` (under `t3_slot.sh`,
  `-n 4` at most), `desktop-test`, and the native smoke compared with main.
  The hosted `harness` runs on the exact head.
- **Review.** Independent review of the regeneration and the registration.
