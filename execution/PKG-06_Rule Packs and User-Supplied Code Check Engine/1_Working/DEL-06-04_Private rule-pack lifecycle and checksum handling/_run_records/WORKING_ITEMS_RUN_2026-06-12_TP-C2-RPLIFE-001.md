---
run-id: WORKING_ITEMS_RUN_2026-06-12_TP-C2-RPLIFE-001
timestamp: 2026-06-12T19:40:00-0600
completed: 2026-06-12T20:10:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/core/rules
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C2-RPLIFE-001 — rule-pack backend seam (C2)

## Tranche and authority basis

- Tranche: completion-plan Phase C item C2, backend slice — "checksum/
  lifecycle surfacing from the existing lifecycle/checksum module"
  (`plans/PLAN_2026-06-10_prd_completion.md` §3 C2).
- Authority: `DEC-022` (frozen grammar, checksum binding), DEL-06-04
  requirements R-06-04-001..012, `DEC-019`/store-ledger precedent for the
  SQLite migration, OPS-K-PRIV-1/OPS-K-RULE-3/OPS-K-AUTH-1.
- This record lives at DEL-06-04 because the tranche's center of gravity is
  lifecycle/checksum surfacing; the new crate also consumes DEL-06-01
  (schema shape) and DEL-06-02 (evaluator AST) seams, and the store
  migration touches the DEL-02-05-owned local persistence surface (noted in
  that deliverable's MEMORY).

## Changes

### New crate `core/rules/rule_pack_document`

The production seam the C1 run record flagged as missing — the first
consumer of the three previously-orphan `core/rules` crates:

1. **Expression codec**: `decode_expression`/`encode_expression` between
   the node-tagged JSON encoding (conformance-corpus authority) and the
   evaluator's typed AST, with JSON-path breadcrumbs in decode errors for
   editor diagnostics. Refusal markers decode (corpus parity) but
   `expression_contains_refusal_marker` exposes them and document
   validation blocks them as authorable content.
2. **Checksum**: `compute_rule_pack_checksum` — RFC 8785 canonical bytes of
   the document minus its `checksums` member (`payload_excludes:
   ["checksums"]`, R-06-04-006: a checksum identifies what was hashed),
   grammar-version-bound via
   `ChecksumRecord::sha256_caller_supplied_jcs_bytes_with_grammar_version`
   (DEC-022). The document-level canonicalization label is `JCS` because
   this crate produces the canonical bytes itself; the lifecycle crate's
   caller-supplied marker remains its own internal posture.
3. **Document validation**: `validate_rule_pack_document` composes
   shape-critical findings (kind, grammar version presence/support),
   per-formula AST decode findings, declared-vs-recomputed checksum match
   status (`match` / `mismatch_review_required` / `declared_placeholder` /
   `not_computable`), and DEL-06-04 `validate_lifecycle` findings into one
   serializable envelope with a fixed professional-boundary notice.
   Mapping note: the schema's `lifecycle_status` vocabulary is wider than
   the crate enum; `validate_lifecycle` does not branch on that field, so
   unrepresentable tokens map to `Tbd` rather than being interpreted.

### Desktop shell (`apps/desktop/src-tauri`)

- Store **v10 migration** `store-v10-local-rule-packs-table`:
  `local_rule_packs (project_id, rule_pack_id, document_json,
  created_at_unix, updated_at_unix)` — project-scoped, local SQLite only;
  private packs never committed or transmitted (OPS-K-PRIV-1, PRD §17.3).
  Migration-evidence tests re-pinned for v10 (fresh open, legacy
  reconcile, v7 staging).
- New commands: `validate_rule_pack`,
  `compute_rule_pack_document_checksum`, `save_local_rule_pack` (drafts
  with blocking findings remain saveable; validation returned alongside),
  `open_local_rule_pack`, `list_local_rule_packs` (project-filtered
  index), `delete_local_rule_pack` (honest receipt).

### Example pack checksum stamping (DEL-06-05 surface)

- `examples/rule_packs/invented_demo.yaml` checksums block flipped from
  `deferred_to_DEL_06_04` placeholders to a stamped value
  `9910cecaff4ed7187e308fd9feee984a660b034890abea2baa2b6f665ee70f8b`
  (sha256, JCS, payload_scope rule_pack_payload, payload_excludes
  ["checksums"], verification_status verified, hash_basis "Canonical
  JSON/JCS-compatible").
- `schemas/rule_pack.schema.yaml` gained the optional
  `Checksum.payload_excludes` member (additive; DEL-06-01 surface).
- Downstream restamp: `examples/models/invented/fake_rule_pack_toy_model.json`
  raw-bytes ref checksum + project JCS hash (DEL-11-04 surface).

## Validation

- `core/rules/rule_pack_document`: **10/10** — 6 unit tests, corpus parity
  (all 69 conformance expressions round-trip through the production
  codec), and 3 example-pack tests: the evaluator computes the invented
  demonstration ratio (0.5, dimensionless `ratio`) from the decoded AST;
  the stamped checksum matches recomputation (golden); document validation
  reports no blocking findings and `publicly_exportable` true.
- `apps/desktop/src-tauri`: **37/37** (4 new: store round-trip per
  project, command-level validation clean/draft findings, checksum command
  golden match, draft saveability).
- `python3 -m pytest -q tests`: **359/359** (new cross-engine parity test:
  Python JCS recomputation equals the Rust-stamped value, and the
  grammar_version member is byte-present in the canonical payload).
- `cargo fmt --check` clean on both touched crates.

## Residuals and hand-offs

- **Editor GUI** (next tranche): frontend service + "Rule Packs" workspace
  section consuming these commands; browser mode gets an explicit
  `RULE-PACK-ENGINE-DESKTOP-ONLY`-style diagnostic (unit-catalog/report
  precedent); Playwright/Vitest evidence rides that tranche (H4).
- **Check-evaluation composition** and **load-case mapping vocabulary**:
  still routed to the C4 lead-up (unchanged from TP-C2-SCHEMA-001).
- **Cross-project private library storage** (C3): the v10 table is
  project-scoped by design; library-level packs are C3 scope with
  DEL-12-01/PKG-12 storage authority (conflict C-06-04-001 remains TBD).
- The lifecycle crate's `Canonicalization` enum still has no `Jcs` variant
  (it cannot prove canonicalization); if a future tranche moves proof into
  the crate, revisit finding PKG06-04-PKG02-001's ACCEPT_AS_IS basis.

## Boundary review

- Local-only: no cloud, daemon, network, or telemetry surface; user packs
  persist in the app-local SQLite store only.
- All repository values remain invented demonstration content.
- Validation statuses are software findings; the envelope carries a fixed
  notice that no professional, certification, sealing, authentication,
  approval, or code-compliance claim is made. No lifecycle state changed.
