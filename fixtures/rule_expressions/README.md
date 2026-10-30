# Rule Expression Fixtures (TP-C1-GRAMMAR-001)

Golden fixtures for the frozen rule-pack expression grammar ruled by
**DEC-022** (`execution/_Decomposition/SOFTWARE_DECOMP.md` §12; D-02 Option A
in `execution/_Coordination/_DECISIONS/D-02_rule_pack_expression_grammar.md`).
The grammar is the typed AST in `core/rules/expression_evaluator`
(`expression_language: open_pipe_stress_declared_expression`, grammar version
`1.0.0`). No text syntax exists at this freeze (deferred to ruling D-02b).

| Directory | Contents | Executed by |
|---|---|---|
| `conformance_corpus/` | One JSON case per file pinning evaluator semantics for every frozen function kind, dimension algebra, and blocking diagnostic. | `cargo test` in `core/rules/expression_evaluator` (`tests/conformance_corpus.rs`). |
| `checksum_binding/` | Invented JCS-style payload bytes plus a golden-hash manifest pinning the DEC-022 property that `grammar_version` is bound inside the JCS-hashed `rule_pack_checksum` payload. | `cargo test` in `core/rules/rule_pack_lifecycle` (`grammar_version_binding_fixture_corpus_is_stable`). |

## Provenance (IP policy)

| Field | Value |
|---|---|
| data_origin | All expressions, variable ids, table rows, magnitudes, and unit references in these fixtures are **invented values** authored for this repository. |
| provenance_class | `PUBLIC_DOMAIN_OR_ORIGINAL` |
| contributor_certification | Nothing here derives from protected standards content, vendor-proprietary data, or real project data. No code-specific allowables, SIFs, flexibility factors, or load-combination defaults appear anywhere; numeric values are arbitrary invented scalars used only to exercise evaluator mechanics. |
| engineering_status | Non-engineering test data. These fixtures make no release-readiness, professional-approval, certification, sealing, or code-compliance claim. |
| review_status | pending human review |

## Freeze rule

These fixtures are the freeze artifact for grammar v1.0.0: any grammar change
(new function, changed semantics, changed diagnostics) requires a
corresponding corpus extension and a `grammar_version` bump, and a grammar
version bump without a corpus extension fails review (DEC-022 / D-02 §5.5).
