# Compact repair TASK return

SR-1 and SR-2 repaired and source frozen for independent backcheck. Aggregate repair-source digest: `e31a1920e479bd8c386025a4c189b81980d34349d61c7eb69eb2ae383eec686d`. `CANDIDATE.json` records the three changed file hashes against prior candidate `1c8f04039b5f41812ebd879fc80d810392123072`. No App/controller/style/geometry/operation/benchmark changes, Git mutation, delegation, native/CUA use, or prior canonical evidence edits were performed by this TASK. Parent-managed prior-evidence portability changes are separate.

## Findings and repair

- **SR-1 confirmed before repair:** in narrow Both, Tab moved from Details to Sort Label; Shift+Tab moved to Review changes. Escape collapsed the drawer and left Details open in both paths. Exact observations remain in `_run_records/reproduction.json`.
- **SR-1 repair:** ModelTree's React bubble boundary owns an otherwise-unhandled Escape while its compact Details popover is open. Descendant editor and enum handlers run first and retain their existing preventDefault/stopPropagation ownership; the shell drawer runs afterward. Dismissal restores the Details trigger. True focus exit from ModelTree dismisses Details without restoring or stealing focus. Compact/Tree teardown unmounts its controls. No global listener or capture interception was introduced.
- **SR-2 repair:** select accessible name is now `Grid family`, containing visible `Family`. Source and dist role locators were updated, with explicit accessible-name assertions.

Maintained changes are limited to `src/features/model-tree/ModelTree.tsx`, `e2e/b4-table-editing.spec.ts`, and `e2e/ui-foundation-dist.spec.ts` under Piping desktop.

## Checks bound to this repair

- 65 ModelTree/EngineeringTable component tests passed against the final repaired product source. Product code did not change afterward; subsequent changes corrected new browser-test interaction assumptions only.
- Final TypeScript check passed; diff whitespace check passed.
- Two connected keyboard journeys passed: Model at 1280×800 and narrow Both at 1024×768. Coverage includes immediate, forward-Tab, backward-Tab Escape; trigger focus restoration; open drawer preservation; actual native `p` typeahead selecting Pipes; direct editor Escape before Details; enum popup Escape, editor cancel, then Details Escape in order; true focus exit without stealing focus; compact/noncompact and Tree round trips; retained input identity and drafts; no leaked Details controls; and normal narrow-drawer Escape after Details is closed.
- Two existing minimum-host scenarios passed in opposite densities (comfortable Model and compact narrow Both), preserving their full geometry, editor, Queue/Clear, history and pointer-hit assertions plus the repaired family name. Both retained the 127px host contract and nonzero usable rows. No unaffected broad suite was repeated.
- First two new keyboard attempts failed on assumed native-select arrow/menu sequences, after the leading Details Escape paths passed. Read-only isolation demonstrated the same arrow behavior in a minimal HTML select; actual printable `p` typeahead worked in the product. The final regression verifies that working keyboard transition and uses the native-select route to return Nodes. Raw failures and diagnosis are retained. This does not claim OS-native popup arrow operation was witnessed; that remains for native evidence.

## Evidence, resources, and limits

Canonical repair payload is this `RETURN.md`, `PROVENANCE.json`, `CANDIDATE.json`, `COMMANDS.md`; `unit.log`, `typecheck.log`, `typecheck-final.log`; `browser-r1.log`, `browser-r2.log`, `browser-r3.log`, `geometry-backcheck.log`; the three diagnosis/reproduction `.mjs` files; and all raw reports/observations/commands under `_run_records`. Raw JSON reports were moved byte-identically into `_run_records` under the parent's portability direction, without sanitization. Final successful report attachments preserve keyboard and geometry observations. No new screenshots or trace ZIPs are required; temporary browser trace outputs are noncanonical.

All owned browser checks completed; Vite on port 5174 was stopped (expected exit 130), and listener inspection found no remaining listener. Shared native/CUA resources were never acquired. Return resource ownership to the parent now.

This is focused Chromium/component evidence, not native WebKit/Tauri or practitioner acceptance. The full dist appearance matrix was not rerun; its only maintained change is the accessible-name assertion/locator. Independent repaired-candidate backcheck and parent-scheduled clean broad/native gates remain outstanding. No claim is made about the separate historical Provenance pointer issue or long-feedback native exposure.
