---
doc_id: TP-MAC-02
doc_kind: implementation.plan
status: implemented
created: 2026-05-09
---

# TP-MAC-02 Physics-First Application Plan

This tranche shifts the product preview from fixture-only mechanics output to a
bounded linear-static physics loop over invented public example data.

## Scope

- Add `core/product_physics` as the product-preview mechanics adapter.
- Keep `core/product_preview` as the schema-adjacent fixture and validation
  layer.
- Use explicit invented mechanics inputs only: material moduli, pipe dimensions,
  support restraints, and primitive load cases.
- Return displacement, reaction, element-force-derived stress summaries, and
  diagnostics in a stable preview result envelope.
- Preserve the professional boundary: no compliance, certification, sealing,
  protected-table, allowable, or autonomous mutation claims.

## Implemented Notes

- `core/product_physics` now validates explicit mechanics inputs before solve:
  missing primitive loads, missing pipe orientation, invalid units,
  duplicate/empty IDs, missing invented/cleared provenance, and unsupported
  references block with diagnostics.
- The preview result envelope includes displacement magnitudes, support reaction
  resultants, local element force/moment components, open-formula stress
  summaries, diagnostics, professional-boundary fields, and non-mutation status.
- Local force/moment result items carry metadata for component, element-local
  coordinate system, endpoint location, recovery basis, and sign convention.
- The desktop preview groups computed results by displacement, reaction, force,
  moment, and stress, and review-only proposals target computed result context.
- The computed fallback fixture can be regenerated from the product-physics
  adapter with `npm run generate:product-preview-mechanics`.
- Browser smoke selectors and the checklist are recorded in
  `apps/desktop/SMOKE.md`; the live browser smoke passed on 2026-05-10.
- The desktop preview has a read-only report-packet panel that consumes
  computed result IDs and diagnostic/proposal status while preserving the
  professional-boundary language.

## Verification

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- Existing solver crate tests.
- Desktop build and tests.
- Product-preview Python tests.
- `npm run generate:product-preview-mechanics`
- `apps/desktop/SMOKE.md`
- `git diff --check`
