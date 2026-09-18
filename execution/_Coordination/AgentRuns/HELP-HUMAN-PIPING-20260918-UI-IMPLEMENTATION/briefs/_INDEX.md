# Sealed briefs — index

Run `HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`. Every brief is written, hashed and listed here before its child is launched. Each row records the mechanism, the model requested, the role and the return. The model that actually ran is recorded with the return. Returns are retained with SHA-256. Type 2 children do not delegate.

| Brief | SHA-256 | Sealed | Model requested | Role | Return |
|---|---|---|---|---|---|
| `A1-LINT_registry_and_lint.md` | `3db45afb67197ce57b7dc52c6e871a4c15edf8f3f670b7fbe19d785789491cad` | 2026-09-18T19:41Z | `fable` (Claude Fable 5.1) | TASK implementer; the registry and the root claims lint | `../instances/A1-LINT/RETURN.md` |
| `A1-APP_desktop_visible_rename.md` | `19fe7ca236cd9681e799e138637cdc442e9274707dd313c3f9d54312ce1f6480` | 2026-09-18T19:41Z | `fable` (Claude Fable 5.1) | TASK implementer; `apps/desktop/**` | `../instances/A1-APP/RETURN.md` |
| `A1-DOCS_sca010_core_docs.md` | `2a45a9731a1953c8907a76a8fce15b2adf26e114da73c2af3a7e4758c392c17d` | 2026-09-18T19:41Z | `sonnet` (Claude Sonnet 5) | TASK implementer; SCA-010, core text, documents, packaging tools | `../instances/A1-DOCS/RETURN.md` |

Tranche A1, mechanism for all three: Claude Code `Agent` tool, general-purpose type, background, in ROOT's worktree on branch `codex/swbpipe-a1-visible-rename-20260918`, at the same time, with disjoint write scopes by path. ROOT integrates and commits; the children run no state-changing git command.


Returns of Tranche A1, retained by ROOT 2026-09-18 (each is the child's final message as the host stored it):

- A1-LINT: `../instances/A1-LINT/RETURN.md` `5c3a4a7f21702a5dec253a63dafd5a234f36b9a146370c54f02d8c0ce7962e0d`. Model that ran: Claude Fable 5.1. It kept the `BS-ACCEPT` texts registered for non-product surfaces, removed the `MISSING_MATURITY_BANNER` anchor and added two findings, `RETIRED_MATURITY_SENTENCE` (every scanned surface) and `RETIRED_ACCEPTANCE_SENTENCE` (product source only). Open point it raised: `DEC-102` requires a domain for every label and `docs/SPEC.md` §4.3 has none for the two evidence labels.
- A1-DOCS: `../instances/A1-DOCS/RETURN.md` `297131b66278d4faac97c100e7eff8c5a74e7c66574ffa35eb09715122bec4d4`. Model that ran: Claude Sonnet 5. PRD 23 of 24 replacements with the historical path kept; the three notice surfaces agree; 1,105 Python tests passed after it installed the pinned `jsonschema` development dependency, which this host lacked. Correction 1 from ROOT (by the host's message-to-agent mechanism): the PDF document metadata title and producer take the name; no test pins a literal digest; the crate's tests pass.
- A1-APP: `../instances/A1-APP/RETURN.md` `7d6fe1e6acaa01aefbd63a2fbd9a748bd8115f28003b942fbf26a8dd54082563`. Model that ran: Claude Fable 5.1. 1,184 unit tests, the build, and 187 Playwright tests passed (10 skipped). It changed one functional e2e test's window height from 920 to 899 after removing the footer, leaving endpoints, tolerances and assertions unchanged; the reviewer is asked to judge that. Correction 1 from ROOT: the new bundle identifier moves the local data directory, so the app now copies (never moves) an existing store from the former identifier's directory when the new location has none; five unit tests; 96 Rust tests pass.

ROOT's checks at integration: every changed path is inside a child's write scope or is ROOT's (the run directory and the tranche manifest `{REPO_ROOT}/docs/governance_harness/tranche_manifests/PIPING-SWBPIPE-A1-CLAIMS-LINT-20260918.yaml`); the claims lint reports VALID over 321 files with the new guards; the lint's 24 tests pass; the manifest corpus validates.

| Brief | SHA-256 | Sealed | Model requested | Role | Return |
|---|---|---|---|---|---|
| `A1-REVIEW_code_review.md` | `98cf6915bfe37c32fd17bf49ebf873e4b6a5a5297f2ac997794f9e2e8c8e750a` | 2026-09-18T20:16Z | `opus` (Claude Opus 5) | TASK with `software-code-review`, read-only, fresh context | `../instances/A1-REVIEW/RETURN.md` |

A1-REVIEW reviews candidate `8143645eacff25e56d685259301795f0c3a482cb`. Mechanism: Claude Code `Agent` tool, general-purpose type, background.

Return of A1-REVIEW, retained at `../instances/A1-REVIEW/RETURN.md` `c2a39462510105e111f1445acf19dbc10523ff7c9f2e9495ba3cc9afbc130914`; model that ran: Claude Opus 5; reviewed `8143645eacff25e56d685259301795f0c3a482cb`, all 105 files; verdict FINDINGS: no blocking, no major, one minor, one trivial, both accepted. Finding 1 (the store carry-forward copied the write-ahead log before the main file and could pair a stale sidecar with a fresh main file) went to A1-APP as correction 2: the carry-forward now opens the legacy store read-only and produces the new store with `VACUUM INTO` a temporary file renamed into place; six unit tests (ROOT first wrote seven; corrected after the backcheck); `cargo test --lib` 97 passed. Finding 2 (the manifest's candidate paths omitted three changed paths): ROOT added them. The reviewer judged the 1440 × 899 test change an honest preservation of the characterized geometry.

Backcheck of A1-REVIEW at `d7ff2e45a53096a324c25f6c0641bae5c6b36ebc`, retained at `../instances/A1-REVIEW/BACKCHECK_RETURN.md` `900e60cb156ca12122a26f1457f9b40ab6f8e35dadb0fd58895fdd9fb9040581`: both findings closed; no finding against the code; one trivial finding against this index (the test count), corrected above. The commit that carries this record changes no code.

First DEC-025 sweep, on `626fa91ebea74f63edee7a55c4b7502237fc4624`: failed at surface `desktop_playwright_e2e`, dist lane, one test of 53 (`e2e/ui-foundation-dist.spec.ts`, "decorative viewport overlays pass real canvas gestures while view controls stay interactive"; expected > 757.40625, received 754.971875); later surfaces not run. The summary is retained as it ran at `{WORKING_ROOT}/validation/evidence/sweeps/SWEEP_20260918T204011Z_626fa91ebea7.json`, following the tracked precedent for failed sweeps. The test is the dist twin of the source-lane test already run at 1440 x 899; ROOT's candidate missed it because the children ran the source lane only.

A1-APP correction 3, retained at `../instances/A1-APP/CORRECTION_3_RETURN.md` `7eff45017f95f2eb6ff5fad6311714c94d935005a285943ed02e67d4c8f8cb12`; model that ran: Claude Fable 5.1. Same cause confirmed from the error context; window height only, comment identical to the source twin; endpoints, tolerances and assertions untouched; six other height-920 sites examined and left; `PLAYWRIGHT_WORKERS=1 npm run test:e2e:dist` 53 passed. Commit `07ece3f9d3d7fdc0da4ceca1293ed5d634c9f687`.

Backcheck 2 of A1-REVIEW over `626fa91eb..07ece3f9d`, retained at `../instances/A1-REVIEW/BACKCHECK_2_RETURN.md` `9ebc85954c231b76f32987b923e63e15a7e35d4af88c56fa21e52a20f8b16dd7`; model that ran: Claude Opus 5; verdict PASS, no finding. The reviewer re-derived the 21 px figure from the dist lane's reported shortfall and found it agrees with the stylesheet derivation. One observation outside scope, not a finding: the twins' wheel-target assertions differ in tolerance at `origin/main` already; carried to `HANDOFF_STATE.md`. The reviewer ran nothing; the observed pass is the child's run and the second sweep.

Tranche A1 merged: PR #800, merge commit `7866f0a3c2c846cf071f735ea9b263a44fb00ca9`; required CI passed on head `dd0c6d177e1d4d189d307335a2f8762f08c3a467`; passing DEC-025 sweep on `a762301be23b9f614a83ce50faa60c18db826a38` (`../_run_records/CLOSEOUT_CHECKS.json`).

## Tranche B

| Brief | SHA-256 | Sealed | Model requested | Role | Return |
|---|---|---|---|---|---|
| `B1-TOKENS_design_tokens.md` | `dd0e1da42fa32b20b9bbf24950deeef4e7ef94702354f5fc7e0418f47d27b71f` | 2026-09-18T22:01Z | `fable` (Claude Fable 5.1) | TASK Type 2, implementer | pending |
| `B-SHELL_shell_lane.md` | `00c98ec0f77363d3bf498c4729185b4761f77e7178853277996352b4b59c4b75` | 2026-09-18T22:01Z | `fable` (Claude Fable 5.1) | WORKING_ITEMS Type 1, lane manager | not launched; launches after B1 merges |
| `B-CANVAS_canvas_lane.md` | `df89f5b622a63f206ccc0f879e7255642e4b9ee16174faf84e539ab51bc12084` | 2026-09-18T22:01Z | `fable` (Claude Fable 5.1) | WORKING_ITEMS Type 1, lane manager | not launched; launches after B1 merges |

Mechanism for all three: Claude Code `Agent` tool, general-purpose type, background. Shared tool `../tools/with_e2e_lock.sh` `9defd858c86509765108b83f5b336cede8b04e9d13facb4f21d56dcf98e197d1`: the Playwright configurations use fixed ports and reuse a listening server, so concurrent runs from two worktrees would test each other's build; every browser run and ROOT's sweeps go through the lock. ROOT's departures from the plan, both recorded in the briefs: B1 runs first and flat because both lanes depend on it; the product owns a copy of `tokens.json` and the generator's logic instead of importing from the design run's folder. ROOT, not the lane managers, dispatches the independent reviews. Each lane manager works in its own git worktree named in ROOT's launch message.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
