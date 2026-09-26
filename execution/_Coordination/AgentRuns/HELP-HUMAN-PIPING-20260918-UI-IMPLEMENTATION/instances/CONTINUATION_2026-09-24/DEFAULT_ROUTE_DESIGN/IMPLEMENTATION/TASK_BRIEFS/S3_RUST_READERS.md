# TASK S3 — Rust readers, rule binding and admission

Read `_COMMON.md` first.

## Assignment
Implement DESIGN §5.6 reader checks, §5.7 standing/rule binding, and S1_INTERFACE §9–§10 in Rust.

1. `P/core/reporting/result_export/src/semantic_contract.rs`: `PREVIEW_PHYSICS_ID`; `preview_physics_contract()` (include the new table); header dispatch in `for_source_metadata` (profile `product_preview_mechanics_v1`; `contract_evidence` required for this id only) and `for_source` → new `validate_preview_physics_evidence`; the static `FRESH_IDENTITIES` set and `is_fresh_identity`; a `standing_reason(source) -> Option<&'static str>` (`PRECISION_1_HISTORICAL_SEMANTICS`, `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS`); `numerical_use_standing_with_context` returns `needs_recompute` for non-fresh ids and for mixed ordinary source-blocks-1 (keep this hunk minimal and at the top of the function body; T1 adds one early return for `load-reference-source-1` there and ROOT merges them by hand); `rule_binding_refusal`. Keep enumeration hunks small and adjacent to existing ones (T1 edits the same enumerations).
2. New `src/preview_physics_evidence.rs` with every S1 §9 check, plus `lib.rs` module line; `derivative.rs` copies `contract_evidence` for the preview id (and `validate_document` binds it).
3. `src/source_blocks.rs`: the ordinary-case standing (non-composite only). A partial receipt stays not eligible; pin it with a test.
4. `P/core/rules/rule_check_runner`: `refused_solver_results` (`#[serde(default)]`), reported as `RULE_INPUTS_INCOMPLETE` with the reason.
5. `P/core/runner/headless/**`: admission of the new id wherever ids are enumerated; a test pinning that no headless path binds solver rows to rules (DESIGN §5.7 item 4); fix tests that assert precision-1 producer output from a fresh solve (after S2a lands, fresh non-exact solves are preview-physics-1).
6. `P/apps/desktop/src-tauri/src/lib.rs` (cannot be compiled here — keep edits minimal, mechanical and self-evidently correct): `solver_result_row_value` calls `rule_binding_refusal` so both `resolve_solver_result_bindings` and `resolve_authored_solver_result_bindings` refuse with the reason and pass `refused_solver_results` to the runner; the rule gate (`qualify_rule_mechanics_with_context`) admits only fresh identities; the packaged self-test (~4309–4660, header pin ~4921) and native tests (~5058, ~5084, ~5150, ~5979) that pin producer identity move to preview-physics-1 where the solve is non-exact. List every src-tauri hunk in your return.

## Write boundary
`P/core/reporting/result_export/**`, `P/core/rules/rule_check_runner/**`, `P/core/runner/headless/**`, and in `P/apps/desktop/src-tauri/src/lib.rs` only the sites named in item 6. Nothing in `P/core/product_physics/**` (manager only).

## Tests to add (all must fail against a deliberately broken input)
Tamper tests per DESIGN §9.2 for the Rust reader: retired kind; retired code; dangling `result:` ref (refused) and a positive control where non-`result:` refs such as `hanger`, `DEC-046` and a user source-reference string are accepted; headline that does not govern / appears while coverage incomplete / wrong tie; value outside bounds; inconsistent support or combination magnitudes; combination row for a gated combination; missing withheld record; zero-filled withheld support; intensified row in a combination; precision-1 offered as fresh (needs_recompute); selected+ordinary source-blocks-1 → needs_recompute with reason (use the existing multicase / n05 / n06 source-block fixtures) while a physics-source-1 envelope with ordinary cases stays admitted; selected+failed stays not eligible; all-selected keeps standing; `rule_binding_refusal` refuses the summary row and headline ref, but a force row in the same envelope binds; runner reports refused inputs as incomplete with the reason. History: every existing precision-1/source-blocks-1/physics fixture, table-hash and verification test passes byte-unchanged.

## Checks
`cargo test` for `result_export`, `rule_check_runner` and `headless` (targeted, `CARGO_TARGET_DIR=/home/user/wt/t0r-s3`), and `cargo clippy --all-targets` on `result_export` if time allows. Report counts.
