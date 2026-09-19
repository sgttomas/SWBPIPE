# Sealed brief — A1F-FOLLOWUPS: the bundle identifier, the acceptance text in emitted notices and documents, and one panel name

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-18 before launch. Role: TASK (Type 2), implementer, working alone; Type 2 does not delegate. Model requested: Claude Fable 5.1. Mechanism: Claude Code `Agent` tool, general-purpose type, background.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel` in the worktree you are started in; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`. Write no absolute machine path in any file you author.

## Authority

The owner's directions of 2026-09-18, recorded with their hashes: `{RUN}/instances/ROOT/OWNER_APP_ID_2026-09-18.md` and `{RUN}/instances/ROOT/OWNER_DIRECTION_2026-09-18_THREE_ITEMS.md`. Read both whole, and the rows `DEC-100`, `DEC-103` and `DEC-106` to `DEC-109` in `{WORKING_ROOT}/execution/_Decomposition/SOFTWARE_DECOMP.md` §12. They are the specification of this work. ROOT's readings in those records are labelled as ROOT's; you attribute nothing to the owner that the owner's words do not say.

## Limits

- Work only in the worktree you are started in, on the branch checked out. Run no git command that changes state. ROOT integrates and commits. Another worktree of this repository is in use by a sibling; never touch it.
- Write only inside your write scope. ROOT has already changed the identifier in `apps/desktop/src-tauri/tauri.conf.json`, two path names in the carry-forward tests of `apps/desktop/src-tauri/src/lib.rs` and `tests/test_release_packaging_script.py`; those edits are in your worktree, uncommitted, and are not yours to redo or revert.
- This is not the identity tranche. Do not rename document kinds, schema `$id` values, packages, crates, file or directory names, test ids, event names, identifiers that contain `caepipe`, download file names, or the four identifiers the owner kept.
- Remove; do not replace. Where the acceptance text goes, no new claims sentence of your own takes its place. Repair the prose around it so it still reads well. Keep every other statement in the sentence or paragraph: the content boundary, "human review remains required", what validation means, what a lint run is evidence of.
- Never weaken a test. An assertion that **required** the removed text moves with the text: it goes, or it pins what remains. An assertion that **guards against** the text (`not.toContain("responsible engineer")` and the like) stays. No behavioural assertion is deleted or loosened. Never alter a tolerance, an oracle or a benchmark limit.
- Never edit: the PRD (`docs/PRD.md`), `docs/CONTRACT.md`, `docs/PROFESSIONAL_BOUNDARY.md`, `docs/SPEC.md`, `docs/report_notice_template.md`, any `ScopeOfWork.md`, `docs/validation_manual/cases/**`, `{WORKING_ROOT}/execution/**`, `validation/evidence/**`, `plans/**`, `loop/**`, `docs/_history/**`, `docs/_ScopeChange/**`, and the claims lint at `{REPO_ROOT}/tools/validation/`. The PRD §19.3 report notice that the report renderer emits is a different sentence and stays.
- Copy rules: "Accept", never "Approve", as a control; none of certify, seal, approve, authenticate, comply, compliant or sign-off as a control; no other vendor's product named in rendered copy; Canadian spelling in new prose.
- Browser tests share the host. Every Playwright run and every dev or preview server goes through `sh {RUN}/tools/with_e2e_lock.sh <command>` with `PLAYWRIGHT_WORKERS=1`. The lock may make you wait for the sibling; that is expected. Stop servers you start.

## Write scope

Exactly the files named in the work below, plus test files that pin a string you change. If you find a carrier that is not named here, do not edit it: list it in your return.

## The work

**1. `DEC-106`, the bundle identifier.** `apps/desktop/src/features/build-readiness/BuildReadinessPanel.tsx` near 157 still says `com.swbpipe.desktop`; it becomes `com.chirality.swbpipe`, with any test that pins it. Confirm by search that no live file outside `execution/**` still carries `com.swbpipe.desktop`.

**2. `DEC-107` (i), the three emitted notices.** Drop the acceptance clause; keep the rest exactly:

- `core/rules/rule_check_runner/src/lib.rs` near 78, `PROFESSIONAL_BOUNDARY_NOTICE`, becomes: "Rule-check results are engineering decision-support information computed from user-supplied rules and data. Human review remains required."
- `core/rules/rule_pack_document/src/lib.rs` near 1207 becomes: "Document validation results are engineering decision-support information computed from user-supplied rule-pack content. Human review remains required."
- `core/reporting/report_package/src/lib.rs` near 600, the first boundary note, becomes: "This container is an artifact format, not an issuance act; package contents are engineering decision-support information." The two notes after it are unchanged.
- Pins that move: the assertion in `apps/desktop/src-tauri/src/lib.rs` near 7279 (pin the first clause and keep the "Human review remains required." assertion); the three fixture copies in `apps/desktop/src/features/rule-check/RuleCheckRunPanel.test.tsx` (take the new text); any unit test inside the three crates that pins the old text. Search `fixtures/`, `examples/` and `tests/` for stored copies of any of the three old texts; ROOT found none.

**3. `DEC-107` (ii), the schema.** In `schemas/rule_check_run_result.schema.json` the property `professional_boundary_notice` stops being a `const` and becomes `"type": "string", "minLength": 1`, with a description saying the wording is the producer's and is not pinned, so documents emitted under earlier wordings stay valid. Nothing else in the schema changes. In `tests/test_operation_result_schemas.py` the mirrored constant and its `const` assertion become an assertion of that type and minimum length; correct the comment that says the Python text mirrors the Rust constant.

**4. `DEC-107` (iii), the documents.** Remove every registered acceptance text (the registry's `BS-ACCEPT` canonical sentences and the four short variants) from these thirteen documents: `README.md`, `docs/README.md`, `docs/user_guide/index.md`, `docs/BUILD_AND_RELEASE.md`, `docs/RELEASE_NOTES_TEMPLATE.md`, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, `docs/_Examples/rule_pack_notice.md`, `docs/contributor_guide/index.md`, `docs/local_analysis/local_fea_handoff_guidance.md`, `docs/security/local_first_storage_policy.md`, `docs/security/redaction_export_controls.md`, `docs/validation_manual/index.md`, `docs/validation_manual/headless_runner_reproduction.md`. Sentences wrap across lines and sit in table cells; use `python3 {RUN}/tools/find_acceptance_texts.py` from `{WORKING_ROOT}` to find them and to confirm the end state. Judgment you owe on each placement: where the text is a clause of a longer sentence, keep the rest of the sentence; where a pointer such as "(see `docs/claims_registry.md`)" or "Per DEC-081" only served the removed text, remove it with the text; where a document **instructs** authors or agents to include the sentence (a template instruction, a checklist item), do not edit it: list it in your return with the line. Two assertions require a document to carry the sentence and move with it: `tests/test_user_guide_status_wording.py` and `tests/test_local_fea_handoff_contract.py`; every other assertion in those tests stays.

**5. `DEC-107`, the registry.** `docs/claims_registry.md`: add a dated amendment in the form the 2026-09-18 amendment already uses, citing `DEC-107`. `BS-ACCEPT`'s "Belongs on" names only what keeps it: the live `ScopeOfWork.md` files, as existing placements. The PRD, the anchor document `docs/PROFESSIONAL_BOUNDARY.md`, `docs/CONTRACT.md` and `docs/SPEC.md` state the boundary in their own words, carry no registered text (ROOT checked) and are untouched; say so. Its "Not on" names product surfaces, emitted notices and the project's user-, contributor- and agent-facing documents. §4's authoring directive says that `BS-ACCEPT` is placed on no new artifact. The registered texts themselves stay listed so the lint's list still matches. Ruled history is not rewritten. Then run `python3 {REPO_ROOT}/tools/validation/validate_claims_language.py` from `{REPO_ROOT}`; it must report VALID. If it does not, report why; do not edit the lint.

**6. `DEC-108` (ii), the panel name.** In `apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx` the heading and the accessible name become "External prover run"; the note near 117 and the two messages near 211 and 246 say "external prover run" where they now say "external run", with the grammar repaired. The pins move: `apps/desktop/src/App.test.tsx` near 2031 and `apps/desktop/e2e/r2-smoke.spec.ts` near 708, and the dist twin if one pins it. `DEC-108` (i) needs no change: "Evidence" is already the word in `statusLabels.ts`. `DEC-109` needs no change: no migration is written.

## Checks (from `{WORKING_ROOT}` unless stated)

- `cargo test --lib` in each of the three changed crates and in `apps/desktop/src-tauri`.
- `python3 -m pytest -q tests/test_operation_result_schemas.py tests/test_user_guide_status_wording.py tests/test_local_fea_handoff_contract.py tests/test_release_packaging_script.py`, then the whole `python3 -m pytest -q tests`.
- `npm run test:desktop`; `npm run build:desktop`.
- Both Playwright lanes through the lock, from `apps/desktop`: `PLAYWRIGHT_WORKERS=1 npm run test:e2e` and `PLAYWRIGHT_WORKERS=1 npm run test:e2e:dist`. Both rebuild the WASM engine first, which the notice change needs.
- `python3 {RUN}/tools/find_acceptance_texts.py --scope-of-work` from `{WORKING_ROOT}`: report its full output. The expected survivors are `docs/claims_registry.md` and test files whose only match is a guard against the text.
- From `{REPO_ROOT}`: `python3 tools/validation/validate_claims_language.py` and `python3 tools/validation/validate_path_anchors.py`.

## Return

Your final message is your return; ROOT retains it. Include: the model you are; that you verified this brief's SHA-256 before starting; every file you changed, grouped by the six items above, with the nine documents ROOT had not named to the owner listed apart from the four it had; each removed sentence quoted once with its file, so the owner can see what left; every check with its exact command and result; every placement you left and why; anything you noticed outside your scope. End with the line: Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
