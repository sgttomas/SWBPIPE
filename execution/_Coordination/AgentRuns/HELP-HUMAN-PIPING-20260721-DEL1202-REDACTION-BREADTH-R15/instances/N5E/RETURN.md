# N5E Return — Fresh attempt-5 terminal verification

**Verdict:** `BLOCK`

## Material findings

1. **`REXC-CORE-001` no longer treats its existing blocking diagnostics as additive exposure gates, and A1 removed the assertions that would detect the bypass.**
   `core/handoff/exporter/workflow.py:113-119` generates and embeds workflow
   diagnostics, then passes the envelope directly to `control_route_export`.
   `core/security/redaction/route_control.py:101-111` derives `blocked` only
   from redaction-contract decisions (or lossless materialization), never from
   the source diagnostic severity. A fresh live probe removed the mapping
   record's required unit metadata and reproduced
   `EXP-UNIT-METADATA-MISSING`; the returned result was
   `blocked=False`, `payload is not None`, had zero blocking redaction
   findings, and exposed one diagnostic whose code and severity were merely
   replaced with `[REDACTED]`. Thus an input the workflow itself declares
   blocking still produces an outward payload. A1 changed
   `tests/test_handoff_export_workflow.py:451-507` from exact diagnostic-code
   and blocking-severity assertions to placeholder/count assertions and never
   asserts `export.blocked` or payload withholding. This violates candidate
   §3 steps 4-6, §5.1's additive-diagnostic gate, acceptance predicates 2 and
   5, and A1's explicit no-weaker-coverage condition.

2. **A3 is not expectation-only: it changes the H4 interaction flow.**
   The attempt-5 diff adds
   `page.getByTestId("rendered-report-render").click()` at
   `apps/desktop/e2e/r2-smoke.spec.ts:550` before the new blocked-output
   assertions. `BRIEF_AMENDMENT_A3.md` authorizes only expectation changes and
   explicitly prohibits E2E-flow changes. The remainder of the A3 diff is
   assertion replacement, but this new user action exceeds the sealed
   amendment authority.

## Independently reconciled evidence

- Fresh Python and in-memory-transpiled TypeScript probes covered `schema`,
  `nested_deliverable_id`, `target_family`, checksums, IDs, diagnostic codes,
  booleans, counts, unit names, suffix lookalikes, nested objects, and arrays
  across public-report, shared-model, and downstream contexts. Both projectors
  produced zero public classifications/includes and no raw leakage. TypeScript
  PCF structural fields were included only for the exact
  `DOTH-FORMAT-003` / PCF document / `DEL-17-07` / required-root case and were
  redacted for a wrong route, wrong deliverable, and opaque descendants.
- Source and focused tests confirm the Parser CSV is wrapped as an explicit
  private scalar and owns component-local intent state independent of Harness
  JSON. Before CSV intent it renders a disabled non-anchor with no `href`; after
  intent it materializes the exact CSV with warning evidence. The source copy
  remains untouched.
- Every `ControlledExportLink` now renders per-decision path,
  classification/action/reason and per-finding path/class/severity/reason
  before its anchor, without rendering decision values. Blocked controls have
  no anchor/href and retain an accessible disabled state.
- A2 is expectation-only. A1 is product-code-free but weakens the blocking
  diagnostic coverage described above. A3 contains the prohibited added
  interaction. N4E otherwise changed only its ten declared product/test paths;
  the authorized CAEPIPE panel path was unchanged.
- The route matrix has 31 unique RouteIDs. The five disposition rows assign
  all 31 exactly once (22 controlled sinks, 2 caller traces, 4 no-egress, 1
  existing control, 2 separate-governance), and all 31 have owner-map
  coverage.
- The five sweep artifacts match the recorded SHA-256 values exactly:
  attempts 1-4 remain
  `10fbc3c4...`, `d1620f2f...`, `b2e89383...`, and `67fe4d20...` and are
  superseded/non-acceptance; attempt 5 has the single acceptance-eligible
  artifact `SWEEP_20260722T083801Z_0c066652cd52-dirty.json`, status `pass`,
  SHA-256
  `3838165eae833faf04a5b2612f42e102224911582580bcab06bef4aa5757a12b`.
  No sweep was run by N5E.
- Terminal N4E evidence records pass and failed intermediate piping, desktop,
  H4, and harness records remain distinguishable. External H4 paths use
  portable placeholders; only mandatory TASK records contain local absolute
  scope/skill paths.
- The complete pre-return inventory was 213 tracked/untracked dirty paths:
  N4E's final 210 plus `N4E_FAN_IN.md`, the runtime-owned N5E brief, and this
  N5E TASK record. Scope validation passed, no dirty or ignored
  `test-results/` file exists, no path is staged, and protected-content/release
  tools, DEL-12-02 state/memory, receipts, lifecycle state, and HEAD remain
  unchanged from `0c066652cd527eb1559f715e914262d2bda42602`.

No repair, evidence sweep, state/lifecycle/receipt, or Git effect was
performed. W3 remains held. Remediation requires a new bounded implementation
attempt followed by a fresh verifier.
