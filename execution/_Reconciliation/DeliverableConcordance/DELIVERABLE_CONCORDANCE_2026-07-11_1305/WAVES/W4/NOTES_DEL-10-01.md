# Notes — CLAIM_CONCORDANCE_DEL-10-01 (R2 wave W4)

Deliverable: DEL-10-01 “Public API and plugin boundary” (PKG-10).
Discovery pilot: GPT-5 at the standard discovery capability tier, as assigned
at dispatch. Dispositions are agent judgments, never owner or engineering
rulings.

Discovery basis: FROZEN worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Encoding follows
`R1_CONVENTIONS.md` Parts A–D, pinned plan §§6–8, W1–W3 package-summary
calibration, and the W4 addendum-9 mitigation in `RUN_BASIS.md`.

Run-level path alias: unqualified kit/current-state files resolve under
`execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-01_Public API and plugin boundary/`;
other paths are repo-`projects/chirality-piping`-relative.

## Histograms

Row count: 27.

| Disposition | Count |
|---|---:|
| ALIGNED | 21 |
| PARTIALLY_IMPLEMENTED | 2 |
| STALE_SETUP_SPECIFICATION | 3 |
| REMAINING_STATE_MISMATCH | 1 |

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 15 |
| ACCEPTANCE | 1 |
| EXCLUSION | 4 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 1 |

Mechanical selectability: zero `YES`. The D-41 bootstrap is copied byte-exact
only into DECL-005 and excluded. The unrecorded pending finding uses the
no-record defaults (`NONE_RECORDED`, `NO`) without inventing a selectable item.

## Evidence checked and re-executed

- Full DEL-10-01 kit, status, memory, context, dependencies, review/finding,
  and run records.
- `api/api_boundary_contract.yaml`, `docs/architecture/plugin_boundary.md`,
  `schemas/plugin_manifest.schema.yaml`, and the two direct static scripts.
- R1 inventory/surface/verification mappings, including `SURF-002` and
  `PY-15`. R1 correctly says `test_api_boundary_contract.py` has zero
  pytest-collected tests; this pilot executed it directly and does not cite a
  pytest pass.
- `PYTHONDONTWRITEBYTECODE=1 python3 tests/test_api_boundary_contract.py`:
  PASS (37 assertions).
- `PYTHONDONTWRITEBYTECODE=1 python3 tests/test_plugin_manifest_schema.py`:
  PASS (52 assertions).
- Dependency validator: PASS, 29 columns and 14 data rows.

Before and after execution, ignored-aware porcelain showed exactly the six
run-disclosed allow-listed sets and no new path: project `.pytest_cache/`, the
two reporting `Cargo.lock` files, two state-comparison `__pycache__/` sets, and
`validation/benchmarks/nonlinear/target/`. No pytest, cargo, or in-tree
`py_compile` operation was used for this deliverable.

## Encoding judgments and self-flags

1. REQ-002 is partial: the registry covers import/export, solve, results,
   rule-pack, report, and validation, but has no explicit model-creation or
   load-case-definition operation family.
2. REQ-010 is partial: the envelope distinguishes model/rule/mechanics/human
   states and external human acceptance correctly, but lacks an explicit
   invalid-input status/diagnostic class.
3. Three kit declarations are STALE because they still say a repo-level API
   artifact is out of scope/deliverable-local only. The frozen tree contains
   DEL-10-01-owned `api/api_boundary_contract.yaml`, its plugin doc, and tests.
   Procedure remains ALIGNED at historical setup-run grain.
4. The API/plugin contract is evaluated at contract grain. ALIGNED rows do not
   claim runtime server, adapter, plugin-loader, transport, or permission
   enforcement exists.
5. Convention-6 SECURITY marker is scoped only to REQ-012’s deferred
   permission/sandbox-enforcement sufficiency; it is not blanket-applied to
   all SECURITY rows.
6. REQ-014 cites pending-human review evidence, so addendum 13 caps confidence
   at MEDIUM and routes OWNER. Its substance rests independently on current
   JCS/binary-manifest fields and direct tests. The pending finding is also
   encoded separately as REM-001 because it appears in no `## Remaining`.
7. No deliverable-owned README exists, so addendum-1 census is six surfaces.
   The repo-level plugin boundary is a governed implementation document, not a
   README census surface; its facts are cited on requirement/declaration rows.
8. No `IMPLEMENTED_UNMAPPED` surface is warranted: the API contract is mapped
   by R1 `SURF-002`; unit/model/plugin schemas are mapped shared dependencies.

No material contradiction was found. No lifecycle, DAG, product, deliverable,
decision, or repair surface was modified.
