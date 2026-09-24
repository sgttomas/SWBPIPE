# Focus repair reproduction and validation

Use repository-relative desktop package root `projects/chirality-piping/apps/desktop`. `CANDIDATE.json` binds the four maintained files. Actual machine/executable/temporary paths and launch logs are classified under `_run_records`; no CI policy, browser config or OS preference was changed.

Environment: Node 24.18.0; npm 11.16.0; Vitest 4.1.10; TypeScript 5.9.3; Playwright 1.60.0. Available macOS engines were installed Chrome 153.0.8010.54 and bundled headless Chromium 148.0.7778.96. Hosted witness identifies Linux HeadlessChrome 148.0.7778.96. Same engine version does not make the local probe a Linux run.

Diagnosis used the existing representative hosted trace/context, complete all-six CI log, and read-only action/event extraction. Three retained probe script versions document the progression: first native focus events; second requested outerHTML/command-line enrichment but failed on unavailable CDP command-line access; third retained that field as unavailable and used actual launch logs while preserving complete event/outerHTML data. Product behavior was never replaced by the probes. The minimal control was instrumented twice, so its paired wrapper log entries denote one native mutation, not two; the product event sequence was instrumented once.

Manager acknowledged the exact port5174 diagnostic reservation before server/probe launch and separately acknowledged the two sequential backcheck commands. Port5175 and native resources were not used.

- `npm run dev -- --port 5174` started the owned source server after free-port inspection.
- Repository-root Node invocations ran the retained `focus-probe*.mjs` scripts. No product writes preceded `DIAGNOSIS_RETURN.md` and manager activation.
- `npx vitest run src/features/workspace/table/OverflowRail.test.tsx src/features/workspace/table/EngineeringTable.test.tsx --reporter=verbose`: initial new synthetic tests failed because jsdom refuses blur() after disabling. The fixture now explicitly models the observed post-mutation BODY focus and restores the body's original tabindex; it remains identified as synthetic. Final result: 58 passed. No assertion was dropped.
- `npx tsc -b`: passed.
- Browser backchecks used `e2e/b4-table-editing.spec.ts --grep 'classic scrollbar compact' --project chromium-desktop --workers 1 --max-failures 1 --reporter=list,json`, first with the bundled148 executable supplied through the existing environment override, then with installed153. Both: six passed. The original immediate focus assertions remain unchanged; status focus and Enter/Space endpoint assertions were added.
- The final browser runs exercise actual native disable behavior, retained draft/Undo guards, all existing classic geometry/count/density cases and keyboard/pointer endpoints. They do not substitute for the required hosted rerun or native WebKit/Tauri witness.

Raw reports, exact commands, failed attempts and check status are under `_run_records`. The canonical enriched probe is losslessly gzip-compressed with original-byte identity in PROBE_ARCHIVE.json; the uncompressed local duplicate is not needed in Git. The representative CI trace ZIP and context are byte-identical copies, not a regenerated trace. No whole CI artifact directory needs tracking.
