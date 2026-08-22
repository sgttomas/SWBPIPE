# WORKING_ITEMS return — WI-PKG12-001

**Status:** SUCCESS
**Package / deliverable:** PKG-12 / DEL-12-01
**Selected node:** N1 local-first runtime route binding
**Lifecycle:** `IN_PROGRESS` (unchanged)

## Validated closure

The current Python handoff routes, desktop controlled-export/report projectors, desktop report save service, and native report-package persistence boundary now enforce governed local-first route evidence. Unknown routes/contexts fail closed; local-private writes require exact Boolean wrapper intent; malformed values remain blocked before directory creation or persistence.

Changed product/test paths are the exact 13 paths and SHA-256 values frozen in `FROZEN_HASH_INVENTORY_ATTEMPT_002.md`. Deliverable-local truth changed only in `_STATUS.md`, `MEMORY.md`, and `_run_records/WORKING_ITEMS_RUN_2026-08-21_LOCAL-FIRST-RUNTIME-BINDING.md`.

## Acceptance evidence

- Focused Python local-first/redaction: `35 passed`.
- Focused CAEPIPE malformed-intent writer regressions: `5 passed`, `11 deselected`.
- Desktop focused tests, build, Rust focused tests, formatting, diff, and scope containment: PASS.
- Pinned full Piping: `913 passed`; the existing exact-pin Python environment with `jsonschema==4.26.0`; `_run_records/piping-pytest-pinned-v3.json`.
- Registered host DEC-025: PASS across Rust, Python, desktop Vitest (`575 passed`), Playwright development (`22 passed`), production-dist (`2 passed`), and production build; `_run_records/evidence-sweep-pinned-host-v3.json`.
- Final coordination harness after record fan-in: PASS (`350 passed` plus self-check); `_run_records/harness-final.json`. The first portability-only failed attempt is preserved as `_run_records/harness-final-attempt-1.json`.
- Review attempt 1 failed on malformed truthy Python intent and is preserved immutably. Repair and fresh attempt 2: PASS, 13/13 paths and 100% diff; `children/TASK-PKG12-REVIEW-001/ATTEMPT_002_RETURN.md`.

## Residuals

- Duplicated Python/TypeScript route allowlists require coordinated maintenance; unknown additions fail closed.
- Native persistence relies on wrapper-owned IPC evidence and validates its complete tuple before write.
- LFSP-REQ-011 implementation-dependent families and owner dispositions RF-001/RF-002 remain open; no owner decision blocks this node.
- No payload inspection, network/cloud/telemetry, storage-root, physical-container, or plugin-runtime selection was introduced.

Runtime summary: NOT_REQUIRED (bounded short activation; no telemetry contract activated).
