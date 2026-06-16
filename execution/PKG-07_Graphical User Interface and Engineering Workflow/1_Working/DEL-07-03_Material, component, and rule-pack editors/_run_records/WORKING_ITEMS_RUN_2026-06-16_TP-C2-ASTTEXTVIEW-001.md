# WORKING_ITEMS Run Record — TP-C2-ASTTEXTVIEW-001

**Date:** 2026-06-16
**Persona:** WORKING_ITEMS
**Primary deliverable:** DEL-07-03 — Material, component, and rule-pack editors
**Supporting deliverable:** DEL-06-01 — Rule-pack schema
**Plan lane:** Phase C2 rule-pack editor GUI

## Scope

Add the `DEC-037`-permitted read-only one-way AST-to-text preview to the
desktop rule-pack expression composer. This tranche is display-only and
frontend-scoped.

## Evidence

- `ExpressionComposer` now renders a labeled `rule-pack-expression-text-preview`
  panel for the selected formula.
- The preview is read-only, contains no `input`, `select`, or `textarea`
  controls, and is never parsed. The authored/checksum-bound rule-pack member
  remains the structured DEC-022 AST.
- The renderer covers authored grammar nodes and displays explicit placeholders
  for refusal markers or unrecognized future nodes instead of silently
  interpreting them.
- `RulePackManagerPanel` boundary copy now reflects the ruled posture:
  read-only AST-to-text preview permitted under `DEC-037`; no writable
  expression text syntax or parser.

## Validation

- `npm test --workspace apps/desktop -- ExpressionComposer.test.tsx` — passed,
  21/21.
- `npm run test:e2e --workspace apps/desktop -- --grep "rule-pack manager drafts privately"` —
  passed, 2/2.
- `npm run build --workspace apps/desktop` — passed with the existing Vite
  chunk-size warning.
- `npm test --workspace apps/desktop` — passed, 18 files / 388 tests.
- `npm run test:e2e --workspace apps/desktop -- --workers=1` — passed, 10/10.
- In-app Browser at `http://127.0.0.1:4180/` — opened a rule-pack draft and
  verified preview text `user_required_input_1`, boundary copy containing
  `DEC-037`, and `editableControlsInPreview=0`.
- `python3 tools/release/run_evidence_sweep.py --execute` — passed on the
  dirty tree and wrote
  `validation/evidence/sweeps/SWEEP_20260616T031013Z_b431a1676620-dirty.json`.

## Boundary Review

No schema, evaluator, parser, writable expression text syntax, protected
standard content, private project data, network/telemetry path, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance claim changed.
