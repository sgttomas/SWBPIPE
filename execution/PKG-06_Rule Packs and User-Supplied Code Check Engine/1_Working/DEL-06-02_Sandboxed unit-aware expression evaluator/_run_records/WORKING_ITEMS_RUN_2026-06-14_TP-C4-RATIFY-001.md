---
run-id: WORKING_ITEMS_RUN_2026-06-14_TP-C4-RATIFY-001
timestamp: 2026-06-14T22:10:00-0600
completed: 2026-06-14T22:35:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
write-authorization: HUMAN_RULING_DEC-039
---

# WORKING_ITEMS_RUN TP-C4-RATIFY-001 — ratify the two C4 additive rule-pack schema members + bump schema_version 0.3.0 → 0.4.0 (`DEC-039`)

## Tranche and authority basis

- Authority: explicit human project-authority ruling on 2026-06-14 ("I approve
  adding the new schema members"), ratifying both PROPOSAL additive members
  presented in the `TP-C4-ACCEPTREL-001` and `TP-C4-SOLVERREF-001` run records.
  Recorded as `DEC-039` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12.
- Pattern: the `DEC-038` / `TP-C3-LIBREFRATIFY-001` ratification of
  `library_value_ref` (the same bounded "ratify + bump + re-stamp" follow-up).
- Members ratified (both already in `schemas/rule_pack.schema.yaml`; no schema
  shape change, only lifecycle PROPOSAL → permanent):
  - `acceptability_relation` on `CheckDefinition` (four ordering relations;
    absent → `less_than_or_equal`; unrecognized token blocks) — `TP-C4-ACCEPTREL-001`.
  - `solver_result_ref` on `RequiredInput` (`{result_id}`, authored in-pack
    solver-result-row reference; canonical, unresolvable → blocks) — `TP-C4-SOLVERREF-001`.

## Versioning decision (single minor bump for both)

Under the `DEC-033` / `DEC-038` additive-minor policy for rule packs, each member
is an `additionalProperties:false`-bearing addition a strict pre-member validator
would reject — a minor-bump signal. The two were ratified in one event, so this
is **one** minor release introducing both: rule-pack `schema_version`
**0.3.0 → 0.4.0** (not 0.4.0 then 0.5.0; no intermediate 0.4.0 pack ever
existed). `grammar_version` stays 1.0.0 (frozen, `DEC-022`); author content
version `metadata.rule_pack_version` is unchanged (the demo content did not
change and uses neither optional member).

## Changes (version bump + checksum re-stamp + transitive hash re-sync — no rule/grammar content change)

- `examples/rule_packs/invented_demo.yaml`: top + `metadata` `schema_version`
  0.3.0 → 0.4.0; `rule_pack_checksum.value` re-stamped
  `60c7ba2b… → c2b4ddce48cd07bed4bb2ff0be3a81d3755365e255896a0bbf583725a3fa47bd`.
- `fixtures/product_preview/invented_demo_rule_pack.json` (the preview fixture
  twin; canonically identical payload): identical bump + identical re-stamped
  checksum.
- `apps/desktop/src/services/rulePackService.ts` `buildDraftRulePackDocument()`:
  top + `metadata` `schema_version` 0.3.0 → 0.4.0.
- `apps/desktop/src/features/rule-packs/RulePackManagerPanel.test.tsx`: draft
  `schema_version` assertion 0.3.0 → 0.4.0.
- `examples/models/invented/fake_rule_pack_toy_model.json` (transitive): this
  invented model hash-references the demo rule-pack **file bytes** (a
  `canonicalization: NONE` rule-pack-ref checksum), and carries its own JCS
  project hash. Because the demo-pack bytes changed, both were re-synced:
  rule-pack-ref `63228952… → b63a4d270a743a51a4a8f8e2c2668fd164230c74d367cbc8c7b782751da468b8`
  (the new raw-byte digest) and the project hash
  `fe0c0086… → 5afe2b433cf3bf99b2905f08262f64b18d5e1643a8af1c994e45958d4ddaac7f`
  (recomputed over the canonical project minus its hashes). This is the same
  transitive step `DEC-038` performed (commit `a0625a0d5`); the full-suite pytest
  surface caught it (`test_invented_example_models.py`).
- Decision/plan surfaces: `DEC-039` recorded; completion-plan C4 row
  "PROPOSAL awaiting ratification" wording cleared to RATIFIED for both members;
  completion log entry added.

(`apps/desktop/src-tauri/src/lib.rs:4780` `schema_version: "0.3.0"` is a
*project/model* document version in a DEC-019 migration test — not a rule pack —
and is intentionally untouched.)

## Evidence

- `python3 -m pytest tests`: **359 pass** — incl. `test_rule_pack_schema.py` (5;
  the JCS checksum-parity test recomputes the demo checksum and matches the
  re-stamped 0.4.0 value) and `test_invented_example_models.py` (the transitive
  toy-model rule-pack-ref raw-byte digest + JCS project hash re-synced above; this
  surface caught the coupling on the first sweep).
- `cargo test` `rule_pack_document`: **10 pass** (6 unit + 1 corpus + 3 demo) —
  the demo stamped-checksum and validation golden tests recompute and match.
- `cargo test` `rule_check_runner`: **18 pass** (incl. the 3 invented-demo runs)
  — backward compat at 0.4.0.
- Targeted desktop Vitest (`rule-packs`, `rule-check`, `rulePackService`,
  `ruleCheckService`): **95 pass** — the draft-template version test now asserts
  0.4.0; the run-rule-checks/ruleCheckService tests load the re-stamped demo
  fixture and validate at run time.
- Five-surface DEC-025 sweep: green summary committed alongside (see the
  evidence commit).

## Boundary compliance

Metadata-only change (a version label and the hash that binds it). No new pack
content, no rule/grammar change, no professional/compliance/certification/
sealing/authentication/approval/code-compliance claim. Deliverables stay
`CHECKING`. Git/test evidence is source-control hygiene only.

## Open decisions awaiting human ruling

- None newly opened by this tranche. Pre-existing, unrelated `NOT_PREPARED`
  register items remain: **D-06**, **D-11**, **D-12**, **D-10b**, **D-04b**,
  **D-05b**, **D-07b**. The two R3-exit blocking residuals (F-4 packaged human
  journey; A3 authoring-journey usability) remain human-gated.
