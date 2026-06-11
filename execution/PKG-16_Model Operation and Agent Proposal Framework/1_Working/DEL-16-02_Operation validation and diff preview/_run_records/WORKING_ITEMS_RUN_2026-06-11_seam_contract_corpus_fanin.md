# WORKING_ITEMS Run Record — Operation Contract Corpus Fan-In (TP-SEAM-CORPUS-001)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), hardening-lane tranche under
  `plans/PLAN_2026-06-11_operation_seam_unification.md` §3 T1 and the
  `execution/_Coordination/_COORDINATION.md` Application Integration And
  Issuance Loop (selection rationale recorded in that plan §8).
- Tranche: `TP-SEAM-CORPUS-001` (cross-engine contract corpus +
  boundary-hygiene rider).
- Deliverable context: DEL-16-02 (operation validation and diff preview) —
  primary record. Fan-out summaries: DEL-16-03, DEL-00-08. Rider record:
  DEL-10-03.

## Execution Note — Interrupted TASK Run Superseded By This Record

The tranche was dispatched to a bounded TASK worker
(`TASK_RUN_2026-06-11_1414.md`, this folder). The session environment lost
its connection mid-run: the worker authored the corpus, both runners, and
the rider fix, but was killed before its own QA/closeout phase. That record
is finalized `FAILED` (run lifecycle only — interruption, not artifact
rejection). WORKING_ITEMS independently re-verified every artifact from a
clean state and completed the closeout; this record carries the tranche's
closure evidence.

## What Landed

- `fixtures/model_operations/contract_corpus/` — README (provenance block
  per IP policy, comparison rules, engine-identity exclusion allowlist with
  per-field reasons, corpus-harness canonical-hash definition with number
  range guards, `CORPUS_BLESS=1` regeneration workflow with human-review
  requirement, quarantine convention, full case inventory) + 44 invented
  case files: 15 accepted paths (all 10 operation kinds, a validate-only
  path, a claimed-hash echo path) and 29 blocked paths spanning 19 distinct
  blocking codes — a superset of the plan's named block-class floor.
- `core/model_operations/operation_applier/tests/contract_corpus.rs` — Rust
  corpus runner (contract reference): semantic-outcome projection,
  order-insensitive diagnostic records `{code, severity, blocking,
  affected_refs}`, parsed-JSON deep equality (numbers by f64 value), the
  corpus-harness canonical sha256 (ECMAScript-`ToString`-compatible number
  rendering; loud out-of-range assertions), input-model immutability
  assertion, bless mode, and a coverage-floor test (10 kinds accepted, 10
  named block codes, dynamic paths `primitive_loads.N.magnitude.value` and
  `terms.N.factor` each accepted and blocked).
- `apps/desktop/src/services/operationContractCorpus.test.ts` — Vitest
  runner executing the same 44 files through `operationService` browser mode
  under identical comparison rules (harness hash reuses `canonicalJson` from
  `hashService.ts`), plus the same coverage-floor test.
- Rider (see DEL-10-03 record):
  `LocalFeaHandoffPanel.tsx` fixture-ID fallback removed; explicit
  `LOCAL-FEA-RESULT-SUMMARY-REF-MISSING` finding;
  `LocalFeaHandoffPanel.test.tsx` added (2 cases).
- `apps/desktop/SMOKE.md` entry TP-MAC-109.
- **`operationService.ts` is unchanged.** The TypeScript mirror reproduced
  all 44 recorded Rust-reference outcomes on first execution — zero
  alignment fixes, zero quarantined cases. This is direct evidence the
  mirror was semantically faithful at corpus scope on 2026-06-11, and it
  retroactively strengthens the probative value of the existing
  browser-based test evidence, as the plan anticipated (§8).

## Validation Evidence (run independently by WORKING_ITEMS at fan-in)

- `cargo test` (`core/model_operations/operation_applier`): 36 passed
  (34 lib + 2 corpus integration tests), 0 failed.
- `npm test --workspace apps/desktop`: 105 passed across 7 files (baseline
  before tranche: 58 across 5), 0 failed.
- `npm run test:e2e:desktop`: 1 passed, unchanged (`r2-smoke.spec.ts`).
- `npm run build --workspace apps/desktop`: TypeScript check and Vite
  production build passed with the existing chunk-size warning.
- Fan-in audit: byte-level inspection confirmed both runners use identical
  NUL-separated diagnostic sort keys (an apparent space/NUL mismatch during
  review was investigated with `od -c` and ruled a rendering artifact);
  comparison-rule documentation in crate runner, Vitest runner, and README
  is mutually consistent; the git-status delta was limited to the brief's
  authorized write targets.

## Boundary Review

- All corpus data is invented, provenance `PUBLIC_DOMAIN_OR_ORIGINAL`,
  recorded in the corpus README with contributor certification;
  `review_status: pending human review`.
- Plan §5 freeze rule honored: no new operation kinds or field rules entered
  the TS engine; no engine source files were modified — this tranche changes
  *what is verified*, not *what is accepted*.
- No network/daemon/cloud surfaces; no protected standards content; no
  code-specific allowables, SIFs, flexibility factors, or load-combination
  defaults anywhere in the corpus; no release-readiness,
  professional-approval, certification, sealing, authentication, or
  code-compliance claims. No lifecycle state change.

## Residuals

- Corpus README `review_status` awaits a human review disposition (IP policy
  contribution review); the invented-data certification is recorded.
- Plan T3 extends the corpus run to the wasm engine (three-way parity);
  plan T4 retires the TS lane and converts the corpus to native↔wasm parity
  + regression fixtures.
