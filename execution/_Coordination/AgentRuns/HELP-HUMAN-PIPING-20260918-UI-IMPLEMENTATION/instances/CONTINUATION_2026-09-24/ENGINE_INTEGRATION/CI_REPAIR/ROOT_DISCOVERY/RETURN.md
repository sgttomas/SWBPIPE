# ROOT browser discovery for the PR905 repair checkpoint

HELP_HUMAN (ROOT), resumed session of 2026-09-25, before the browser repair was applied. Candidate `5b1ccd356c0d9656f7b9f42c6558f7a4c73f9295` (PR905 head `e65001ad` plus the preserved record and lockfile repairs). Hosted source run 36139859999 had passed accessibility and shard 1, failed five cases in shard 4 and cancelled shards 2 and 3, so the complete failure set was unknown. Environment and exact commands: [_run_records/environment.txt](_run_records/environment.txt).

| Observation | Result | Record |
|---|---|---|
| Local reproduction of the five hosted failures, both projects and all `ui-foundation.spec.ts:677` variants | 10 of 18 failed: the hosted five plus their `chromium-desktop` counterparts and the light-theme 1280x800 variants the cancelled shards never ran | [repro-18-cases-5b1c.log](_run_records/repro-18-cases-5b1c.log) |
| Same 18 cases on current main `aa312755e` in the same environment | 18 of 18 passed, so the failures belong to PR905, not to the local Chromium | [main-aa312755-baseline-18-cases.log](_run_records/main-aa312755-baseline-18-cases.log) |
| Complete source suite on `5b1ccd356`: 499 identities, both projects, one worker | 466 passed, 20 skipped, 13 failed | [full-source-5b1c.log](_run_records/full-source-5b1c.log), [outcomes](_run_records/full-source-5b1c-outcomes.json), [raw JSON (gzip)](_run_records/full-source-5b1c.json.gz) |
| `full-cohort-controller.spec.ts:468`, compact, isolated rerun | Passed; its original failure text was `ENOSPC: no space left on device` during a shared-disk exhaustion window | [cohort468-compact-isolated-rerun-5b1c.log](_run_records/cohort468-compact-isolated-rerun-5b1c.log) |
| Practitioner self-check on a clean `5b1ccd356` checkout | Exit 0 after the clone was unshallowed. The earlier COMMIT_NOT_FOUND receipt findings came from the shallow clone. Remaining REVIEW/WARN findings are outside Piping | [harness-self-check-5b1c-unshallow.log](_run_records/harness-self-check-5b1c-unshallow.log) |

The 12 real failures fall in five spec files and four causes, which the [browser repair](../BROWSER_REPAIR/RETURN.md) diagnoses and repairs:

- A sourced status chip was checked with a helper that assumed title and token are equal: `gui-workflow-validation.spec.ts:107` ×2 and `linear-authoring.spec.ts:60` ×2.
- Assertions written for the old fixture "solve" remained after the honest browser refusal: `r2-smoke.spec.ts:338` ×2 and `r2-smoke.spec.ts:1680` ×2.
- A real narrow-column layout defect let the result-filter summary cover the search input: `ui-foundation.spec.ts:677` at 1280x800 ×4.

No other source spec failed. These are local development observations: local Chromium 1194 is older than the Chromium 1223 bundled with the CI-pinned Playwright 1.60.0. Hosted CI on the frozen candidate remains the required check.
