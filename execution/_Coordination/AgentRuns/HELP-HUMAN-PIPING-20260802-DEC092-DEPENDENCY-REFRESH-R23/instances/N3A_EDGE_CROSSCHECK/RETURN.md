# N3A exact edge/evidence crosscheck — return

## Verdict

PASS on frozen Git basis
`23d15899fd0acf5d1d0513f3fe396438375c9e25`.

## Validated result

- All 24 candidate execution rows have target-maturity PASS.
- Exactly 12 also have exact consumer-integration PASS and close.
- Exactly 12 fail the consumer gate and remain unchanged.
- Five named holds remain unchanged; one anchor normalization is
  evidence-consistent.
- Coverage: 17/17 target-status files, 11/11 consumer-evidence files, 24/24
  candidate rows, 5/5 holds, 1/1 anchor, and all 1,480 DAG rows.
- DAG-009 differs from approved DAG-008 at exactly 13 IDs and 39 cells: only
  `SatisfactionStatus`, `LastSeen`, and `Notes` for each accepted ID.
- Every changed candidate field equals its current accepted local row;
  `EstimateImpactClass` and `ConsumerHint` remain inherited from DAG-008.
- The 12 failed candidates and five holds are raw-row byte-identical in their
  current local registers and ordered-field identical between DAG-008 and
  DAG-009.
- No missing evidence, hash mismatch, row-order drift, extra delta,
  contradiction, or accepted-closure ambiguity was found.

The complete 30-ID disposition and evidence map is preserved at
`execution/_Evaluation/DAG009_CANDIDATE_EVALUATION_2026-08-02_R23/EDGE_DISPOSITIONS.csv`.

## Basis hashes

- DAG-008 edges: `dde5d2d0601290d085a30833bccb582ff6518ba7dcdd9adebe6e03ba49719005`
- DAG-009 candidate edges: `4293cbe39ff794f74da7031c2f0e2706003fadb666ca4d85f0e7d3ec25baa9cc`
- Target evidence CSV: `78ee5982b7ac49eb85b9511095e22b23eee32d8e209a33a2b85e2b8574e8bf36`
- N2A brief: `0153f55e51bd48329b1effb3392adcefb04fc6adb81dcaa215cd1ec282f83dd9`
- N2B brief: `13c0ab842fe548a58d26c9d996f94d2e7d6a7621a8a88dff4d6b4ef2bdeabc6b`
- N2C brief: `8706fffcac8e55c2aef607740577c28c1350c9c058c0f215b830dbc163a17b4b`
- N2D brief: `b88a13de01129d88a5153a32e8d2ddb991811fbda002c48aa9059a628e06ae1d`
- N2E brief: `d043a5a72ef7f8ceaa78f9e4eeb26962717511d8b6115498d077b34feab88d6a`

Execution was read-only: no writes, repairs, delegation, Git mutation, or
network use.
