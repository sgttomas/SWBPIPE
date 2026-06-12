# WORKING_ITEMS Run Record — H5 RFC 8785 Canonical Number Rendering (TP-H5-JCSRENDER-001)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), executing completion-plan §3
  hardening-lane row H5 by explicit human direction for this session
  (recorded per coordination loop step 3: hardening selected ahead of
  current-stage plan items because the human named H5 as the tranche).
- Owning deliverable: DEL-08-02 (audit manifest and model hash — the hash
  evidence surface this tranche aligns with the DEC-010 "JCS-compatible
  hashing" baseline).
- Sub-worker: one bounded TASK run for the headless-runner/audit-manifest
  subscope — record at
  `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/_run_records/TASK_RUN_2026-06-11_h5_headless_jcs_alignment.md`.

## What changed

- **New shared crate `core/serialization/canonical_json`** — RFC 8785 (JCS)
  canonical JSON text: object keys sorted by UTF-16 code units (§3.2.3),
  ECMAScript `Number::toString` number rendering (§3.2.2.3; `-0` → `0`),
  `JSON.stringify`-identical string escaping via serde_json (§3.2.2.2).
  Digit selection uses `ryu` (already in the dependency closure through
  serde_json): development measurement found Rust std's formatter breaks
  exact shortest-representation ties differently from ECMAScript (7
  divergences in 20k random doubles, e.g. `2153998317200749.25` →
  std `…749.3` vs ECMA/ryu `…749.2`); a one-shot fuzz then matched ryu
  against node `JSON.stringify` on 112,220 random/laddered doubles with
  zero mismatches (scaffolding deleted after the run; RFC 8785 Appendix B
  bit-pattern vectors stay pinned as unit tests, node-verified 2026-06-11).
- **One documented divergence from a strict double-only JCS:** serde
  preserves i64/u64 exactly, so raw-text integers beyond 2^53 keep full
  precision instead of collapsing to the nearest IEEE double. Outside the
  I-JSON envelope RFC 8785 assumes; unreachable through any JS lane
  (`JSON.parse` rounds first); pinned by corpus case
  `number-beyond-2-53-integer-kept-exact` and documented in
  `fixtures/canonical_hash/README.md`.
- **Engine.** `operation_applier::canonical_json` is now a re-export of the
  shared crate (local serde-sorted `sort_json` deleted);
  `BACKEND_CANONICALIZATION` = `rfc8785_jcs` (was the honest
  `serde_json_sorted_keys_not_rfc8785`). All three JSON-text producers —
  JS `JSON.stringify` transport, raw-file serde parse, engine-internal
  serde rendering — now converge on identical canonical bytes for
  identical values (within f64-exact range).
- **Both H1 refutations resolved; harness renderers retired.** The corpus
  ECMA renderer twins (`canonical_json_ecma` + `canonical_number` in the
  Rust runner; `ecmaCanonicalJson` + local `corpusCanonicalHash` in the
  Vitest runner) and the plain-decimal number-range constraint
  (`|v| < 1e15`, `|v| ≥ 1e-6`) are deleted. The corpus hash is the
  engine's own hash in every lane.
- **Corpus tightened to full byte-equality.**
  `model_basis.backend_model_hash` joined the compared semantic
  projection; `applied_model_backend_hash` must byte-equal the blessed
  `applied_model_canonical_sha256` in every lane (native file-fed, wasm
  adapter, wasm direct). Engine-identity exclusions now reduce to
  `application_route` and diagnostic id/source/prose.
- **Re-blessed fixtures.** `fixtures/canonical_hash/cases.json` (extended
  14 → 20 cases: float-integral convergence, −0, ECMA notation
  boundaries, shortest-tie-to-even, beyond-2^53 integer, UTF-16 key
  sort; floor raised 12 → 20) and all 44 contract-corpus cases
  (`CORPUS_BLESS`), diffs reviewed: label + backend-hash fields as
  expected, no semantic outcome changed.
- **Headless runner (TASK subscope).** Local serde-sorted twin deleted;
  checksums render through the shared crate; label `"JCS"` (previously
  aspirational) → `rfc8785_jcs`, now true. Schema enum + Python contract
  pin updated to match (`schemas/headless_runner.schema.yaml`,
  `tests/test_headless_runner_contract.py`).
- **audit_manifest checked (H5 check item).** `CanonicalJson` carries
  caller-supplied number strings and performs no number rendering — no
  behavior change needed; doc comment now points callers at the shared
  crate as the project's RFC 8785 number renderer.
- **Label unification.** Every surface labeling engine-computed hashes
  carries `rfc8785_jcs`: `hashService.ts`, `types.ts` literal types,
  `previewService.ts` analysis-run hash refs, A7
  `renderableReportInput.ts` checksums, test assertions
  (`hashService.test.ts`, `App.test.tsx`, `renderedReport.test.tsx`),
  corpus case_15 claimed-evidence input, src-tauri test evidence inputs.

## One-time hash migration (sequenced per the H5 plan note)

Every recorded hash over canonical text changed in this tranche's
re-blessed fixtures (canonical_hash corpus, 44 contract-corpus cases).
Sweep findings for other recorded `sha256:` values: PKG-17 export-package
fixtures and `fixtures/results`/`fixtures/analysis_boundary` records are
produced by their own writers/canonicalizations (not the changed
renderers) and recompute nothing through the engine seam — unaffected,
verified by the full green test sweep. User-local saved projects carry
`computed_local_preview` hash evidence that recomputes on the next
save/solve through the wasm engine; stored claims are echoed, never
equality-evaluated (binding_status semantics unchanged), so no stored-data
migration step exists at the technical-preview stage.

## Residual hand-offs (recorded, not silently absorbed)

- `core/loads/primitive_loads` `BoundaryCanonicalization::JsonJcs` still
  maps to the string `"JCS"` (vocabulary token on declared boundary-record
  refs; no hashes computed). Unify when that vocabulary next opens.
- `core/reporting/result_export` test fixtures and
  `fixtures/analysis_boundary`/`fixtures/results` invented records carry
  caller-supplied `"JCS"`/`"jcs_compatible"` labels on placeholder or
  self-consistent values — label vocabulary only, no producer behind them.
- Python persistence truth-labels (`canonical_json_jcs*` in
  `core/project_persistence/service.py`, `core/analysis_runs/records.py`)
  predate this tranche and describe Python-side declared truth forms, not
  the engine renderer.
- Claimed-vs-backend hash equality evaluation (binding semantics) stays
  deliberately unchanged; with both lanes now on one canonicalization, a
  future tranche could evaluate equality — product behavior change, not
  H5 scope.

## Evidence (2026-06-11, this host)

- `core/serialization/canonical_json`: 8 unit tests (RFC 8785 Appendix B
  bit patterns, ECMA notation branches, convergence, big-int exactness,
  UTF-16 key sort, JSON.stringify escaping, composite document) green.
- `operation_applier`: 34 unit + canonical-hash parity (20 cases) +
  contract corpus (44 cases ×2 suites) green at re-blessed expectations.
- Desktop Vitest: 172/172 (8 files) against a freshly rebuilt wasm
  artifact — corpus byte-equality asserted across adapter and direct wasm
  lanes.
- Headless runner 11/11; audit_manifest 13/13; src-tauri 28/28; root
  pytest 353/353 (headless schema contract updated in the same change).
- DEC-025 five-surface sweep at the committed HEAD: summary in
  `validation/evidence/sweeps/` bound to the closing commit (recorded
  there after commit; see the sweep artifact for the result line).
- UI evidence posture (H4): no Playwright spec extension — no interactive
  desktop behavior changed; the user-visible deltas are hash values and
  the `rfc8785_jcs` label inside evidence packets, pinned by the unit
  suites (`App.test.tsx`, `hashService.test.ts`, `renderedReport.test.tsx`)
  and the cross-lane corpus; existing e2e specs assert behavior, not hash
  vocabulary.

## Boundary review

Local-only; invented fixture values; no protected standards content; no
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claims. Hash labels remain
`computed_local_preview`; `rfc8785_jcs` names a serialization form, not a
compliance vocabulary. Nothing here creates evidence reliance.
