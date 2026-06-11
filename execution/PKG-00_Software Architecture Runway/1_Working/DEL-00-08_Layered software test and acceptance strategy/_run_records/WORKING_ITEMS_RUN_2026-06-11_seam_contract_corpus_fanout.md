# WORKING_ITEMS Run Record — Contract Corpus Fan-Out Summary (TP-SEAM-CORPUS-001)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona).
- Tranche: `TP-SEAM-CORPUS-001` under
  `plans/PLAN_2026-06-11_operation_seam_unification.md` §3 T1.
- Deliverable context: DEL-00-08 (layered software test and acceptance
  strategy) — fan-out summary. Primary record and full evidence:
  `../../../PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-11_seam_contract_corpus_fanin.md`.

## Relevance To This Deliverable

A new cross-engine contract-test layer now exists for the editor-operation
seam: one checked-in corpus (`fixtures/model_operations/contract_corpus/`,
44 invented cases) executes inside both engines' normal suites —
`cargo test` (Rust integration test) and `npm test --workspace apps/desktop`
(Vitest) — with no separate command to forget. The layer asserts semantic
parity and a single corpus-harness canonical hash of applied documents
(hash-stability guarantee), and enforces its own coverage floor
programmatically (all 10 operation kinds accepted; named block classes;
dynamic field paths accepted and blocked). Under plan T3/T4 the same corpus
becomes the native↔wasm parity layer after the TypeScript engine retires.

## Evidence

`cargo test` 36/36 (applier crate); `npm test --workspace apps/desktop`
105/105 (7 files; baseline 58/5); Playwright 1/1 unchanged; desktop
production build green. Boundary review in the primary record. No lifecycle
state change.
