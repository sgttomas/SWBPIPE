# Modal result fixtures (DEL-04-09 I3a)

Maintained test fixtures of the experimental modal result family `openpipestress.modal_result` 0.1.0 (`execution/PKG-04/DEL-04-09/Work/modal-analysis.md` §5.3). They are not run evidence, and every document carries the labelled **TEST-LOCAL, EXPERIMENTAL** parameter set it was formed with. All models are invented.

| File | What it is |
|---|---|
| `semantic_contract_modal_1.json` | The semantic table `openpipestress.result_semantics/modal-1` |
| `modal_<name>.json`, `modal_<name>.request.json` | A document and the request it binds, projected from a real `ModalOutcome` of a reference model |
| `modal_corpus.json` | The reader corpus: tampered relations, the standing each targets and the verdict both readers return |

## Generation

The documents and the table are generated deterministically from their requests by the product's own projection:

```bash
cd core/product_physics
<cargo wrapper> test --lib modal::tests_document::write_modal_fixtures -- --ignored
```

`modal::tests_document::fixtures_are_the_projection_of_their_requests` requires the committed bytes, so a change to the projection, the kernel or a request shows as a difference there. The corpus expectations come from the Rust reader and are reviewed against each entry's `intent` before they are committed:

```bash
cd core/reporting/modal_result
<cargo wrapper> test --test corpus write_modal_corpus_expectations -- --ignored
```

The Rust reader (`core/reporting/modal_result/tests/corpus.rs`) and the Python reader (`tests/test_modal_evidence.py`) are each held to every expectation, so they agree on every decision.
