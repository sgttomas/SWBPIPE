---
doc_id: R17-DEL1005-W-PKG10-RETURN
doc_kind: coordination.package_return
status: ready_for_owner_adoption
created: 2026-07-23
---

# WORKING_ITEMS package return — pre-adoption

## Coverage

- Package: PKG-10 only.
- Selected deliverable: DEL-10-05 only.
- Stages completed: serial reconnaissance, candidate synthesis, independent
  refutation, v2 repair, fresh refutation.
- Candidate: v2 of
  `CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001`.
- Exact matrix: 24 unique rows.
- Fresh refutation: N3B `COMMIT-SAFE`.

## Selected design

- Extract the accepted desktop-owned report-package DTO/conversion once into a
  PKG-08-owned shared core wire adapter.
- Keep desktop and runner as thin linked-product-physics identity binders.
- Replace only the runner's export-results stub.
- Return exact package bytes and manifest/member evidence in a bounded
  controlled JSON projection.
- Keep runner JSON `--output` separate from desktop picker/atomic `.opsproj`
  save.
- Classify exact `$.report_package` once, avoiding per-byte decisions while
  preserving existing local-private vocabulary.

## Validation plan

All six registered checks, focused Rust/Python tests, deterministic witnesses,
native runner ZIP/hash/cardinality proof, packaged-native desktop save
regression proof, frozen collateral hashes, claims/path/JSON/receipt/scope/diff
checks, exactly one terminal DEC-025 sweep, and fresh N5.

## Blockers and requested action

No technical or dependency blocker remains. Human adoption is mandatory
because the exact matrix includes three PKG-08/shared paths and the new bounded
aggregate package classification.

Requested HELP_HUMAN action: present the exact adoption language from
`HANDOFF_STATE.md`. Do not release N4 until the human adopts v2.

No product, test, schema, deliverable, dependency, DAG, stage, receipt, sweep,
Git, lifecycle, release, issuance, or merge effect occurred.

