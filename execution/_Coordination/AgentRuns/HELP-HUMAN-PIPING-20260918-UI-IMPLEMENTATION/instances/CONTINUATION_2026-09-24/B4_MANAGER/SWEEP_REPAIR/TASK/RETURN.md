# Clean-sweep repair TASK return

Diagnosed the two independent failures, received bounded manager activation, and completed the minimal three-file repair. Source frozen for independent backcheck: `244767aeb8a29cb924aa0d5b085987cadd7a2149296baf83c3f07ccf7f2d8b11`. Complete file hashes are in `CANDIDATE.json`; product basis is c57, with ROOT's subsequent document-only HEAD recorded separately. No App/controller/setup-wide adapter, Git, native/CUA or unrelated source changes were made.

## Repair

- Existing compact queued status now carries the canonical `entity-grid-queued-message` marker, preserving genuine status text/role and the mutually exclusive noncompact marker. The unchanged App test now reaches and passes both operation-ID/unit-payload assertions.
- Info and Details cleanup first require a callable target `hidePopover` before querying native `:popover-open`. Without the API there is no native popover to dismiss. The supported branch still uses native state, hide and focus/event order exactly as before. No catch, global selector patch, fabricated fallback disclosure or asynchronous state mirror was introduced.
- The prior closed-selector unit shim was removed. Tests verify absent-API cleanup never calls the unsupported selector, supported closed/open cleanup preserves native ownership/focus behavior, and a deliberate supported-state-query exception propagates out of the product handler. Component doubles are explicitly branch tests, not native-support claims.

## Evidence

Independent diagnosis remains in `DIAGNOSIS_RETURN.md`: queue-only reproduction failed without popover errors while real compact feedback was present; project Save reproduction passed its assertion but exited with two unhandled selector errors. Standalone jsdom has no show/hide API and throws for the selector. Those failed logs are preserved.

After repair: unchanged queue/payload case **1 passed**; the five source sweep handler reproductions **5 passed**, no unhandled errors; focused components **68 passed**; final TypeScript and whitespace checks passed. The initial typecheck caught a new-test-only unsupported `exact` option; removal preserved exact string name matching and the three new/updated unit cases passed again. Product source did not change after the 68-case pass.

Existing real Chromium Details/Info journeys **4 passed**, covering Model/narrow Both traversal, focus/enum/editor ownership and both-density classic-gutter Info behavior. No new browser test or global launch configuration changed. The final browser JSON report records zero unexpected/skipped/flaky cases.

No clean full sweep, native WebKit/Tauri witness or merge acceptance is claimed by this return. ROOT's failed c57 sweep remains failed; ROOT must run its next required clean-candidate gates after review/integration. The guard does not claim unsupported environments can display native popovers.

## Payload and resources

Canonical: `RETURN.md`, `DIAGNOSIS_RETURN.md`, `PROVENANCE.json`, `DIAGNOSIS_SOURCES.json`, `CANDIDATE.json`, `COMMANDS.md`, and all raw logs/probe/report/command/check files under `_run_records`. Temporary traces/screenshots are noncanonical. All focused commands finished; owned Vite stopped with expected exit 130 and listener checks found neither 5174 nor 5175 occupied. No native process/bundle was touched.

The three maintained paths are ModelTree.tsx, EngineeringTable.tsx and EngineeringTable.test.tsx under Piping desktop. Source/API lifecycle behavior outside the exact guard remains unchanged. Await parent independent backcheck and next clean sweep scheduling.
