# N2 return — component-creation coverage seam

## Verdict

`SUCCESS` — implementation checks passed; the first fresh review found two P1 defects, the owner authorized their narrow remediation in `N2_AMENDMENT_V2.md`, directly affected native/Wasm/UI/build checks passed, and a new fresh 100%-diff review returned PASS with no actionable findings. The final evidence sweep was not run.

## Implemented bounded slice

- Retired the blanket completion-plan-A3 block by routing `insert_component_symbol` through `resolve_create_component`.
- Implemented strict **bend-only** component creation with explicit component identity, existing node, realized pipe reference, endpoint connectivity, radius quantity, angle quantity, plane orientation, geometry source, and provenance.
- Preserved `check_claimed_model_hash` before resolver execution and the single structured-operation mutation route.
- Added apply/diff support for appending a validated component record.
- Added explicit bend-creation emitting forms in both the viewport component tool and Property Inspector.
- Added focused native and UI end-to-end tests, including claimed-hash binding and missing/disconnected reference rejection.
- Canonicalized all persisted bend identity/connectivity/reference/provenance strings after validation.
- Reserved queued component target IDs across the viewport and Inspector surfaces, with a cross-surface regression.

## Changed files and frozen content hashes

- `core/model_operations/operation_applier/src/lib.rs` — `8d07a0cf1e9140258771e904e6f41714c17b1a1c5886e4a2588c99936b071dd7`
- `apps/desktop/src/features/component-creation/componentIntent.ts` — `d87e30d5633484e27cb22a92e1f445424be3ba985beffd1b31a4e2d824ec25dc`
- `apps/desktop/src/features/viewport/PipeViewport.tsx` — `93879b2371ac049f422913666a1009c181fb7f25028f44461d0e3ca302da93d7`
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx` — `dfcb356381a867c0592cbb9e3ca16aea69c69bebbb927804bc1853b942cc1337`
- `apps/desktop/src/App.test.tsx` — `2a250af5c08033a13ddb3853e3a977155741c77c28d802d7c7b212588422ce8d`

## Coverage disposition

- NORMATIVE-NOW row 15: **partial implementation evidence; bend slice passed**. Bend creation reached native/Wasm/UI end-to-end behavior and fresh review PASS, but the multi-kind row is not fully closed.
- Row 15 residual kinds: tee, reducer, valve, and flange each still require a kind-specific explicit geometry/connectivity resolver and emitting form.
- Row 14 expansion-joint creation remains residual and was not selected.
- DEL-07-09 contract/coverage surfaces were not written.

## Validation evidence

- Native operation applier: PASS — 78 unit + 1 canonical hash parity + 2 contract corpus.
- Wasm operation-engine build: PASS.
- Focused Inspector/viewport component creation: PASS — 2 tests.
- Full desktop Vitest: PASS — 29 files / 541 tests.
- Desktop TypeScript/Vite build: PASS.
- Scoped diff check: PASS.
- `tests/test_viewport_editor_contract.py`: environment failure before collection because the selected Xcode Python lacks `pytest`; not rerun under the stop rule.
- Pre-amendment fresh review: FAIL with two P1 findings; preserved in `N2_REVIEW_RETURN.md`.
- Post-amendment native canonicalization regression: PASS.
- Post-amendment Wasm build: PASS.
- Post-amendment focused cross-surface UI regression: PASS — 2 tests.
- Post-amendment desktop build: PASS.
- New fresh 100%-diff read-only review: **PASS**, no actionable findings; see `N2_REVIEW_RETURN_V2.md`.

## Remediated review findings

1. The resolver now reconstructs the persisted record from normalized validated fields; padded input is covered by a native regression.
2. Viewport and Inspector validity now includes queued component targets; the sibling surface remains disabled after the first queues `component:C-1`.

## Handoff

Return to `HELP_HUMAN` as a successful bounded bend-creation slice with residual kinds explicitly parked. No commit, shared closeout write, or final evidence sweep was performed.
