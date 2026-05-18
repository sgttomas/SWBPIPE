---
amendment_id: SCA-001
doc_kind: scope_change.brief
created: 2026-04-30
status: executed
---

# SCA-001 Brief

## Request

Use `PKG-00 - Software Architecture Runway` `SEMANTIC_READY` content as an architecture-basis candidate for sealed brief injection without treating `PKG-00` as `ISSUED`.

## Human Rulings

- `SCA-001` is the intended amendment ID and interpretation.
- Gate 1 validation and Gate 2 impact assessment were accepted.
- Gate 3 decomposition/register amendment preview was approved.
- Gate 4 propagation plan was approved.
- Downstream propagation updates all 65 `PKG-01` through `PKG-12` `_CONTEXT.md` files now.
- Stack/runtime/framework/API/persistence/test choices that are imperative from `PKG-00` may be resolved; implementation-specific choices remain `TBD`.

## Resolved Baseline

- Runtime/UI: Rust core/application services; Tauri 2 desktop shell; TypeScript/React/Vite GUI; Three.js viewport where 3D viewport-facing.
- Schema/API: JSON Schema 2020-12; schema-first command/query/job/result envelopes.
- Persistence/hash: versioned JSON-schema-governed persistence; canonical JSON with JCS-compatible hashing for JSON payloads; manifest hashes for non-JSON/binary assets.
- Tests: Cargo tests, Vitest, Playwright, validation gates, protected-content/provenance gates, deterministic solver/rule verification gates.

## Remaining TBD Boundary

Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider, coverage thresholds, performance thresholds, migration framework, and physical project package/container remain implementation-level TBD unless later human approval or a sealed brief resolves them.

## Gate 3 Reference URLs

- Tauri 2: https://v2.tauri.app/
- React: https://react.dev/
- Vite: https://vite.dev/guide/
- Three.js: https://threejs.org/docs/
- JSON Schema: https://json-schema.org/specification
- JSON Canonicalization Scheme RFC 8785: https://www.rfc-editor.org/info/rfc8785
- Vitest: https://vitest.dev/
- Playwright: https://playwright.dev/
- Cargo workspaces: https://doc.rust-lang.org/nightly/cargo/reference/workspaces.html
