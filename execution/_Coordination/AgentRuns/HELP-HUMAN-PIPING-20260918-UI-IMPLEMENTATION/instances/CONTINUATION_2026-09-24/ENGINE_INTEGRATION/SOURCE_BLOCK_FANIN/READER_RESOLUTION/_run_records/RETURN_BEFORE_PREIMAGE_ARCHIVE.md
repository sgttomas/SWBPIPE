# Reader conflict resolution — source ready

TASK `/root/physics_resume/physics_reader_join/physics_rust_export` resolved all
16 owned conflict hunks: Python compatibility9, Rust derivative2, semantic
dispatch4 and library wiring1. The already auto-merged Python `__init__.py`
needed no edit. Parent `/root/physics_resume` remains the facade owner; the
immediate caller is `/root/physics_resume/physics_reader_join`.

The first supplied basis was `../FANIN_01.json`, whose56 imported inputs bind the
immutable NUM `REVIEWED_SUBSET.json` at
`0b8c8d84431a94ccc401afad1e7d569858de55ca8c44667e7b294ad1ad47a71d`.
That manifest hash was checked before edits. The reviewed subset explicitly
excludes whole-tranche/native qualification and unreviewed transport/schema
wiring. No broader authority is inferred from the import. Before/after hashes
and exact conflicting preimages are retained in this directory; per-hunk
semantic dispositions are in `HUNK_RESOLUTIONS.json`.

## Resolved APIs

- Rust `semantic_contract::for_source_metadata` remains header-only. It selects
  precision-1, physics-1 or source-blocks-1 by the exact producer ID and requires
  the matching formulation profile. It never invokes a raw-row validator against
  a canonical envelope and does not establish numerical eligibility.
- Rust `for_source` sends physics-1 to the unchanged repaired physical validator
  and source-blocks-1 to `source_blocks::validate(source, None)`. The latter
  checks the imported statement/publication contract; absent independently
  captured context cannot yield positive source-block standing. Unsupported
  identities and mixed namespaces do not fall back to another table.
- Rust `numerical_use_standing` keeps its two-argument API. Its new imported
  companion `numerical_use_standing_with_context(source, requested_basis_refs,
  actual_invocation)` uses the reviewed source-block validator and exact case
  list for that method only. Existing precision/physics ordinary quality and
  Sensitive/not-assessed gates are unchanged. A serialized receipt or a
  self-recomputed digest does not recreate a private arithmetic receipt.
- Python `_source_contract(..., check_receipt=True)` mirrors the explicit
  three-ID dispatch. Its imported `check_receipt=False` source-block transport
  mode checks retained receipt shape/body hash only; it is not a numerical-use
  path. The physics branch still performs the complete unchanged physical
  validator and is not weakened by that source-block-specific metadata mode.
- Python `numerical_use_standing(source, requested_basis_refs,
  source_block_context=None)` retains the reviewed context requirement for
  source-blocks-1 and preserves the ordinary precision/physics path. Existing
  two-argument callers obtain `needs_recompute` for a structurally valid
  source-block carrier without independent context.
- Python analysis builders now select the correct table/hash for all three
  explicit IDs. Source-block records preserve `analysis_run.source_block_recovery`
  and validate it against the actual source. `__init__.py` already exports the
  imported validator and existing numerical-use function; no extra wiring was
  necessary.
- Rust derivatives retain physics `contract_evidence` and source-block
  `source_block_recovery` under their own identities and bind them back to the
  raw source during validation. In the two large formatting conflicts, the
  joined-vs-base semantic delta was exactly the physics copy/binding and legacy
  exclusion; those deltas were composed into the reviewed NUM common body.
  Row/value/hash assembly behavior was not replaced wholesale.

Receipt presence, including false/null/empty values, cannot downgrade a
source-block carrier into precision-1, physics-1 or legacy. Source-block raw
validation requires its own closed namespace and ordinary formulation; physical
raw validation rejects source-block evidence. No physics-source-1 table,
producer identity, profile or header relabeling was introduced. This union of
separate readers is not composite numerical/physical qualification.

## Narrow follow-on joins

1. `schemas/results.v0.3.schema.yaml` currently permits only precision-1 and
   physics-1 producer/semantic references. Add a distinct source-block branch
   with the pinned new ID/hash context, ordinary formulation and required closed
   `source_block_recovery` receipt. Its existing oneOf cannot be extended by
   adding an enum value alone. Keep physical evidence required only on the
   physical branch and forbid the foreign namespace on the other branches.
2. `schemas/analysis_run.v0.3.schema.json` similarly needs the explicit
   source-block semantic ID/hash pairing plus the retained receipt property on
   `analysis_run` under a matching conditional. The imported standalone
   `schemas/source_block_recovery.schema.json` supplies the closed shape; hashes,
   source/row bindings and numerical context remain code-level checks.
3. The present headless metadata gate calls `for_source_metadata`; it can now
   recognize a source-block header without constructing fake raw rows. Its
   producer/raw/schema/eligibility path still needs the parent's explicit join.
   Any numerical-use caller requiring source-block eligibility must pass the
   actual independently captured `{request, solver_mode}` invocation, including
   actual request materials and extra captured request fields, not a rebuilt
   model-only surrogate. Caller source/model/input/build authenticity remains a
   separate obligation.
4. Desktop/native changes were explicitly excluded from the imported subset.
   No browser/native or composite admission is supplied by this TASK. The
   independently repaired physics validators and table identities remain the
   accepted input to any later composite design/review.

## Verification and limits

`STATIC_CHECKS.json` records: zero conflict markers on all owned paths, Python
AST parsing pass, and unchanged hashes for all12 captured read-only
validator/table/schema inputs. `rustfmt --edition 2021 --config skip_children=true
--check` passed for the resolved Rust semantic and derivative modules after
formatting those owned files. Formatting parses syntax; it is not a Rust
compile/typecheck. No Cargo or Python suite was run during the parent's UI lane,
and no runtime pass is claimed. The imported source-block tests and the existing
physics/precision/derivative tests remain for the parent's next coordinated
validation, followed by independent review of the actual joined candidate.

Precision-1 table remains
`d75aacee175e178dbdeb256d89a65f4b375265f7da077725ee635af33df51d7e`;
physics-1 remains
`9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc`;
source-blocks-1 remains the imported
`5f299065f15a157bbedf9467a598994ae684c4ecb3f851bbcb291981ec550a9f`.
No test, tolerance, numeric oracle, physics guard, UI journey or table was edited.
