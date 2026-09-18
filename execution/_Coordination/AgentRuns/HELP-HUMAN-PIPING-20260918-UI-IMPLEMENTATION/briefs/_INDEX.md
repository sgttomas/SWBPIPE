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

Return of A1-REVIEW, retained at `../instances/A1-REVIEW/RETURN.md` `c2a39462510105e111f1445acf19dbc10523ff7c9f2e9495ba3cc9afbc130914`; model that ran: Claude Opus 5; reviewed `8143645eacff25e56d685259301795f0c3a482cb`, all 105 files; verdict FINDINGS: no blocking, no major, one minor, one trivial, both accepted. Finding 1 (the store carry-forward copied the write-ahead log before the main file and could pair a stale sidecar with a fresh main file) went to A1-APP as correction 2: the carry-forward now opens the legacy store read-only and produces the new store with `VACUUM INTO` a temporary file renamed into place; seven unit tests; `cargo test --lib` 97 passed. Finding 2 (the manifest's candidate paths omitted three changed paths): ROOT added them. The reviewer judged the 1440 × 899 test change an honest preservation of the characterized geometry.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
