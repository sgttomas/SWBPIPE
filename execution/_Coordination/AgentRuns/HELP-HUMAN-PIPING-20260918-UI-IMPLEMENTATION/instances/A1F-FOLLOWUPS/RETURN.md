A1F-FOLLOWUPS return

Retained by ROOT. What follows is the child's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed.

---

All six items are done and every check passed. I am Claude Fable 5.1 (`claude-fable-5-1`). Before starting I verified the brief with `shasum -a 256`; it gave `5b08732e0e667a50e3b838b78941d1358febea65e93379464070d3e553b24cfa`, which matches. I then read both owner records whole and the rows `DEC-100`, `DEC-103` and `DEC-106` to `DEC-109`.

I worked alone in `{REPO_ROOT}` on `codex/swbpipe-a1-followups-20260918`. I ran no state-changing git command, wrote nothing under `execution/**` and did not delegate. ROOT's three uncommitted edits are as I found them; `apps/desktop/src-tauri/src/lib.rs` now holds ROOT's two path names plus my one assertion. Paths below are relative to `{WORKING_ROOT}`. My last edit was at 16:54:33 and the first check started after it, so every result is for the final bytes.

## Files changed, by item

**1. `DEC-106`, the bundle identifier**
- `apps/desktop/src/features/build-readiness/BuildReadinessPanel.tsx` line 157 now says `com.chirality.swbpipe`.
- No test pins this literal.
- A git and filesystem search finds no live file outside `execution/**` that still carries `com.swbpipe.desktop`.

**2. `DEC-107` (i), the three emitted notices**
- `core/rules/rule_check_runner/src/lib.rs`, `core/rules/rule_pack_document/src/lib.rs` and `core/reporting/report_package/src/lib.rs` now carry exactly the brief's texts. The report package's two later notes are untouched.
- `apps/desktop/src-tauri/src/lib.rs` near 7279 now pins the first clause, full stop included, and keeps the "Human review remains required." assertion. The pinned text is: "Document validation results are engineering decision-support information computed from user-supplied rule-pack content."
- `apps/desktop/src/features/rule-check/RuleCheckRunPanel.test.tsx`: the three fixture copies take the new text, and I added one guard (judgment 5).
- Nothing inside the three crates pinned the old text. `fixtures/`, `examples/` and `tests/` hold no stored copy.

**3. `DEC-107` (ii), the schema**
- `schemas/rule_check_run_result.schema.json`: only `professional_boundary_notice` changed, to `"type": "string", "minLength": 1` with the description the brief asked for.
- `tests/test_operation_result_schemas.py`: the mirrored constant and its incorrect "Mirrors…" comment are gone. The test now asserts type and minimum length, with a corrected comment.

**4. `DEC-107` (iii), the documents**
- The four ROOT named to the owner: `README.md`, `docs/user_guide/index.md`, `docs/BUILD_AND_RELEASE.md`, `docs/RELEASE_NOTES_TEMPLATE.md`, with `tests/test_user_guide_status_wording.py`.
- The nine ROOT had not named:
  - `docs/README.md`
  - `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
  - `docs/_Examples/rule_pack_notice.md`
  - `docs/contributor_guide/index.md`
  - `docs/local_analysis/local_fea_handoff_guidance.md`
  - `docs/security/local_first_storage_policy.md`
  - `docs/security/redaction_export_controls.md`
  - `docs/validation_manual/index.md`
  - `docs/validation_manual/headless_runner_reproduction.md`
- `tests/test_local_fea_handoff_contract.py` belongs with the nine.

**5. `DEC-107`, the registry**
- `docs/claims_registry.md` gains "Amendment, 2026-09-18 (`DEC-107`)" in the existing amendment's form.
- "Belongs on" now names only the live `ScopeOfWork.md` files, as existing placements. It says the PRD, the anchor document, `docs/CONTRACT.md` and `docs/SPEC.md` use their own words, carry no registered text and are untouched. I confirmed that with the tool's matcher; the four came back with nothing.
- "Not on" names product surfaces, emitted notices and the user-, contributor- and agent-facing documents. §4 places `BS-ACCEPT` on no new artifact.
- The registered texts and the earlier amendment are unchanged.

**6. `DEC-108` (ii), the panel name**
- `apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx`: the accessible name and heading are "External prover run".
- The note reads "External prover run evidence is parser-only…". The two messages read "…an external prover run was not attempted." and "An external prover run is optional and user-owned."
- Pins moved in `apps/desktop/src/App.test.tsx` near 2031 and `apps/desktop/e2e/r2-smoke.spec.ts` line 708. No dist twin pins the name.

## What left, with HEAD line numbers

Two sentences recur below, so I name them once:
- **S1** is "Results are engineering decision-support information."
- **S2** is "Acceptance, professional judgment, and any certification, sealing, or code-compliance determination remain with the responsible engineer and project authority."

**Emitted notices**
- Both rule notices lost "; acceptance and professional judgment remain with the responsible engineer".
- The package note lost ", and acceptance and professional judgment remain with the responsible engineer and project authority".

**The four named documents**
- `README.md` 39: "; acceptance and professional judgment remain with the responsible engineer (see `docs/claims_registry.md`)".
- `docs/user_guide/index.md`:
  - 43–46: S1 and S2 with "(`docs/claims_registry.md`)".
  - 65 and 282–283: "acceptance stays with the responsible engineer".
  - 186: S2.
  - 247–248: "; acceptance and professional judgment remain with the responsible engineer".
- `docs/BUILD_AND_RELEASE.md`:
  - 26–28: S2 with its pointer.
  - 178–179: the short clause.
  - 336–338, an adapted form: "; acceptance and professional judgment for any project-specific piping calculation remain with the responsible engineer".
- `docs/RELEASE_NOTES_TEMPLATE.md`:
  - 36–39: S1 and S2.
  - 86–88, an adapted form: "; acceptance and professional judgment for any project-specific piping calculation stay with the responsible engineer (`docs/claims_registry.md`)". "Competent human review remains required." is kept.

**The nine**
- `docs/README.md` 56–58: the short clause with "(boundary vocabulary: `claims_registry.md`, DEC-081)".
- `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` 337–339: "Per DEC-081, " then S2 then " (PRD §21.2)".
- `docs/_Examples/rule_pack_notice.md` 19–21: S2.
- `docs/contributor_guide/index.md` 29–31: S2.
- `docs/security/local_first_storage_policy.md` 29–31: S2.
- `docs/security/redaction_export_controls.md` 140–142: S2.
- `docs/local_analysis/local_fea_handoff_guidance.md` 20–24: " — engineering decision-support information for review by the responsible engineer" and S2.
- `docs/validation_manual/index.md`:
  - 25–27: S2.
  - 32–33: ", and acceptance and professional judgment remain with the responsible engineer".
  - 203: ", and acceptance stays with the responsible engineer".
- `docs/validation_manual/headless_runner_reproduction.md` 23–24: ", and acceptance and professional judgment remain with the responsible engineer".

## Checks

Commands ran from `{WORKING_ROOT}` unless stated.

| Command | Result |
|---|---|
| `cargo test --lib --offline` in `core/rules/rule_check_runner` | 13 passed |
| `cargo test --lib --offline` in `core/rules/rule_pack_document` | 6 passed |
| `cargo test --lib --offline` in `core/reporting/report_package` | 7 passed |
| `cargo test --lib --offline` in `apps/desktop/src-tauri` | 97 passed, including the moved assertion's test |
| `python3 -m pytest -q` on the four named test files | 25 passed |
| `python3 -m pytest -q tests` | 1105 passed |
| `npm run test:desktop` | 73 files, 1184 tests passed |
| `npm run build:desktop` | exit 0 |
| From `apps/desktop`, through the lock: `PLAYWRIGHT_WORKERS=1 npm run test:e2e` | 374 passed, 20 skipped, exit 0 |
| From `apps/desktop`, through the lock: `PLAYWRIGHT_WORKERS=1 npm run test:e2e:dist` | 53 passed, exit 0 |
| From `{REPO_ROOT}`: `python3 tools/validation/validate_claims_language.py` | VALID, 321 files scanned |
| From `{REPO_ROOT}`: `python3 tools/validation/validate_path_anchors.py` | PASS, 8837 surfaces |

- **Rust.** `--offline` fetched nothing, and no `Cargo.lock` changed.
- **Extra Rust run, not in the brief.** Full `cargo test --offline` in the three crates and in `core/runner/headless`, which consumes the report package, passed.
- **The skips.** All 20 are environment-gated `test.skip` cases in `e2e/ui-foundation/full-cohort-controller.spec.ts`. The R2 smoke test that reaches the renamed panel passed in both source-lane viewport projects.
- **The lock.** I never had to wait for the sibling. Ports 5174 and 5175 were free before and after each lane, and afterwards the lock was released with no server left running.
- **What the lanes exercise.** The WASM engine is built from `core/model_operations/operation_applier`, which does not depend on the three notice crates. So the browser lanes do not exercise the new notice text; the crate and `src-tauri` tests do.
- **Lint unit tests, read-only.** `python3 -m pytest -q tools/validation/test_validate_claims_language.py` from `{REPO_ROOT}`: 24 passed.

Full output of `python3 {RUN}/tools/find_acceptance_texts.py --scope-of-work`:

```
apps/desktop/src/App.test.tsx: short-B loose=1
docs/claims_registry.md: canonical-1st,canonical-2nd,short-A,short-B,short-C loose=3
tests/test_local_fea_handoff_contract.py: canonical-2nd loose=1
tests/test_user_guide_status_wording.py: canonical-2nd loose=1
4 live file(s) outside execution and frozen history
live ScopeOfWork.md files carrying a registered text: 25 of 93
```

The three surviving test files are guards against the text. Before my edits the tool listed 22 files.

## Judgment calls, open to correction

1. **The two document-pinning assertions.** Instead of deleting them I inverted them, to `not in`. The user-guide one now checks whitespace-normalized text. They are guards against the sentence's return, and every other assertion in both tests stays. If you want plain removal, drop those lines.
2. **Registry pointer kept in the validation manual.** In `docs/validation_manual/index.md` I kept "(boundary vocabulary: `docs/claims_registry.md`, DEC-081)", re-attached to the `BS-VALID` statement that remains, because it did not serve only the removed text.
3. **Registry pointer removed from the docs index.** In `docs/README.md` the same kind of pointer served only the removed clause, so it went. That index now has no pointer to the registry.
4. **Adapted forms removed.** Two placements are adapted, not verbatim, registered texts; both are marked above. I removed them because the tool's loose match defines the end state.
5. **The FEA guidance appositive.** I removed all of " — engineering decision-support information for review by the responsible engineer", not only the registered tail, so no truncated variant remains.
6. **Guard added in the rule-check panel test.** The fixtures no longer contain "responsible engineer", so the existing `not.toContain("responsible engineer")` guard stopped proving the notice is not rendered. I added `not.toContain("Human review remains required")`; the panel renders no such text.
7. **Extra schema assertions.** The schema test also asserts `"const" not in notice` and `"enum" not in notice`.

## Placements left, and why

- `docs/claims_registry.md` records the texts.
- The guard-only assertions stay.
- 25 of 93 live `ScopeOfWork.md` files still carry a registered text, left by direction.
- The PRD, `docs/CONTRACT.md`, `docs/PROFESSIONAL_BOUNDARY.md` and `docs/SPEC.md` are never-edit files and carry no registered text.
- I found no placement in the thirteen documents that instructs authors to include the sentence. `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` 274–275 points to the registry generically and stays correct.
- One instructing carrier is outside my scope and I did not edit it: `tools/validation/validate_claims_language.py`.
  - Line 309 advises "use BS-ACCEPT from docs/claims_registry.md (DEC-081)".
  - Lines 21–23, 61–63 and 95 say documents may still carry the text.
  - Both now disagree with the registry's §4.

## Noticed outside scope, not edited

- The "External-run evidence" wording also sits in `apps/desktop/src/features/export-review/ExportReviewPanel.tsx` 963 and 990, and in `apps/desktop/src/features/report-lint/ReportLintPanel.tsx` 425 and 808. The user guide says "external-run evidence" in prose as well.
- The schema's top-level description has two problems:
  - It still carries the prohibition litany that `DEC-081` retired.
  - Its line citations were already wrong at HEAD, and everything after line 80 moves one more line. The property's own "78-80" citation is correct again.
- `apps/desktop/src/App.test.tsx` 17136 holds an ad hoc fixture notice, "…requiring responsible-engineer review."; it is not a registered text.
- `.github/ISSUE_TEMPLATE/*.yml` say the responsible engineer supplies code-specific values; these are not acceptance texts.
- Several of the nine documents still use the name "OpenPipeStress"; that belongs to the identity tranche.

My logs and helper scripts are in the session scratchpad only; none sit in the repository.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
