---
run-id: WORKING_ITEMS_RUN_2026-06-14_TP-C3-LIBREFRATIFY-001
timestamp: 2026-06-14T11:30:00-0600
completed: 2026-06-14T12:20:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
write-authorization: HUMAN_RULING_DEC-038_AND_COORDINATION_LOOP
---

# WORKING_ITEMS_RUN TP-C3-LIBREFRATIFY-001 — ratify `library_value_ref` + bump rule-pack schema_version to 0.3.0 (DEC-038)

## Tranche and authority basis

- The human project authority **ratified** the additive `library_value_ref`
  rule-pack schema member (added as PROPOSAL by `TP-C3C4-LIBREF-001`, companion
  to DEC-031) and ruled that the **DEC-033 additive-minor versioning policy
  applies to the rule-pack `schema_version`**. Recorded as **`DEC-038`** in
  `execution/_Decomposition/SOFTWARE_DECOMP.md` §12. This is a **non-`D-XX`**
  ratification (it is not a `D-01..D-12` register row); it was presented as a
  standing-PROPOSAL ratification and ruled "Ratify + bump to 0.3.0".
- This tranche is the **bounded implementation the ruling authorized**.

## Ruling content (DEC-038)

1. `library_value_ref` is a permanent part of `schemas/rule_pack.schema.yaml` as
   drafted (optional on `RequiredInput`, `additionalProperties:false`, required
   {`library_kind`∈{material,section,component}, `library_id`, `record_id`,
   `slot_id`}; reference-only; value resolved at run time and never embedded —
   IP boundary).
2. Versioning: because `RequiredInput` is `additionalProperties:false`, a strict
   validator predating the member **rejects** a pack that uses it
   (forward-incompatible while new code still reads old packs) — textbook
   minor-bump semantics — so the rule-pack schema version moves **0.2.0 → 0.3.0**,
   adopting the DEC-033 additive-minor policy for the rule-pack `schema_version`.

## Changes

### Decision record
- `SOFTWARE_DECOMP.md` §12 — new **`DEC-038`** row; `_DECISIONS/_REGISTER.md` is
  unchanged (no `D-XX` row exists for this standing PROPOSAL).

### Canonical demo/example packs (byte-identical pair)
- `examples/rule_packs/invented_demo.yaml` and
  `fixtures/product_preview/invented_demo_rule_pack.json`: top-level
  `schema_version` and the `metadata.schema_version` echo 0.2.0 → 0.3.0;
  `rule_pack_checksum.value` re-stamped `9910cec…` → `60c7ba2…` (recomputed via
  `compute_rule_pack_checksum`, RFC 8785 canonical bytes of the document minus
  the `checksums` member). The two files remain byte-identical (the fixture is a
  copy of the example). `metadata.rule_pack_version` (author content version) is
  **unchanged** — the demo rules did not change; `grammar_version` stays 1.0.0.

### New-draft editor template
- `apps/desktop/src/services/rulePackService.ts` `buildDraftRulePackDocument`:
  `schema_version` + `metadata.schema_version` 0.2.0 → 0.3.0 so new private
  drafts declare the current schema version; `rule_pack_version` stays 0.1.0
  (a fresh draft's content starts at 0.1.0).
- `apps/desktop/src/features/rule-packs/RulePackManagerPanel.test.tsx`: the
  draft-template assertion 0.2.0 → 0.3.0.

### Transitive: invented model example hash reference
- `examples/models/invented/fake_rule_pack_toy_model.json` embeds a
  `project.rule_pack_refs[].checksum.value = sha256:<sha256 of the rule-pack file
  bytes>` and a JCS `project` hash over its content
  (`tests/test_invented_example_models.py` recomputes and asserts both). Because
  the rule-pack file bytes changed, both were re-stamped: the rule-pack-ref digest
  `4c744…→63228…` and the project hash `a1533d9…→fe0c008…` (recomputed via the
  repo `core.project_persistence.canonical_json`). This transitive reference was
  caught by the five-surface sweep's pytest surface (the whole `tests/` suite),
  not the targeted `tests/test_rule_pack_schema.py` run — a concrete instance of
  why the commit-then-sweep-at-HEAD gate exists.

### Not touched (deliberate)
- The unrelated **model-document** schema version (also 0.2.0, DEC-033,
  `projectService.ts` `SUPPORTED_MODEL_SCHEMA_VERSION`) — a different versioning
  axis. `RuleCheckPanel.tsx`'s synthesized minimal pack at `schema_version`
  0.1.0 (a valid older declaration) is left as-is. Test-internal packs declaring
  0.2.0 remain valid (0.2.0 still conforms) and were not bumped.

## Evidence

- `cargo test -p open_pipe_stress_rule_pack_document
  example_pack_stamped_checksum_matches_recomputation`: re-passes against the new
  stamp (the test recomputes from the bumped pack and asserts the stamped value).
- Five-surface DEC-025 sweep: see the committed sweep summary. The bumped packs
  still schema-conform (pytest rule-pack-schema; `schema_version` pattern accepts
  0.3.0); the wasm/Vitest/Playwright surfaces consume the re-stamped fixture; the
  production build is unaffected.

### UI evidence posture (H4)
The user-observable change is that a new draft pack and the bundled demo pack
declare `schema_version` 0.3.0 — a version-string change, no behaviour change and
no new UI surface. It is covered by the Vitest draft-template assertion and the
Playwright e2e lanes (which load the re-stamped demo fixture). No new SMOKE
journey beyond the recorded TP-MAC-160 note.

## Boundary compliance

Local-only; status-vocabulary-only. No private values embedded (the ratified
member stays reference-only). No lifecycle, release, professional, certification,
sealing, authentication, or code-compliance claim — DEC-038 is a schema-member
ratification and a version-policy disposition, recorded as a `DEC`; deliverables
stay `CHECKING`.

## Open decisions awaiting human ruling

- None outstanding for the rule-pack ↔ library reference area: `library_value_ref`
  is ratified (DEC-038); D-02b is ruled (DEC-037). Remaining C3/C4 residual is a
  richer C4 run-panel library/record/slot picker (implementation scope, not a
  human decision).
