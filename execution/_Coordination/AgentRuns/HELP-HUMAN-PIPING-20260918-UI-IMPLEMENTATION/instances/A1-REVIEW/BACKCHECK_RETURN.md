# A1-REVIEW — retained backcheck return

Retained by ROOT. What follows is the reviewer's backcheck message as the host stored it; nothing is changed. ROOT's disposition: the one trivial finding (ROOT's index said seven carry-forward tests; there are six) is corrected in `../../briefs/_INDEX.md`. The observations are recorded in `../../HANDOFF_STATE.md`; no code changed after the reviewed commit.

---

## Return — A1-REVIEW backcheck: corrections at `d7ff2e45a53096a324c25f6c0641bae5c6b36ebc`

**Model:** Claude Opus 5 (1M context), `claude-opus-5[1m]`. Role: TASK (Type 2), read-only, working alone; I delegated nothing, wrote no file in the repository, and ran no state-changing git command. No Playwright, no dev server.

**Reviewed:** `git diff 8143645eacff25e56d685259301795f0c3a482cb..d7ff2e45a53096a324c25f6c0641bae5c6b36ebc` in full — 6 files, 311 insertions, 50 deletions, spanning `7dd0d97b7` (A1-REVIEW sealing and index rows) and `d7ff2e45a` (the two corrections).

---

# Verdict: **FINDINGS** — one trivial, in a record, not in code.

**Both of my findings are properly closed.** Finding 1's replacement is materially better than what I asked for and I have no further finding against it. Finding 2 is complete. The retained return is faithful. The one thing left is a wrong number in `briefs/_INDEX.md`.

---

## Finding 1 — closed. The `VACUUM INTO` carry-forward is sound.

`projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:592–658`. This is not the fix I proposed (reordering the copies); it is the stronger one I named as the alternative, and it removes the defect class rather than narrowing it. I reviewed it as new code.

**Read-only open flags — correct.** `Connection::open_with_flags(&legacy_store, SQLITE_OPEN_READ_ONLY | SQLITE_OPEN_NO_MUTEX)` (lines 627–631) passes an explicit flag set rather than amending the default, so `SQLITE_OPEN_CREATE`, `SQLITE_OPEN_READ_WRITE` and `SQLITE_OPEN_URI` are all absent. No `CREATE` means a vanished legacy file errors instead of being conjured; no `URI` means a path that happens to begin `file:` is treated as a path. `VACUUM INTO` is read-only with respect to the source, so it is legal on this connection, and the source's journal mode is not rewritten — the test asserts `PRAGMA journal_mode` still reads `wal` afterwards through the still-open writer.

**Consistency — the actual defect is gone.** The whole point of the original finding was that two files read at different instants can disagree. There is now one read, inside one SQLite read transaction, and no sidecar is copied at all. I confirmed empirically (Python `sqlite3` 3.53.1, scratchpad, temporary directory) that a read-only connection plus `VACUUM INTO` with a **bound parameter** works and carries rows that are committed but still only in the `-wal` — the same thing the new test `new_absent_and_legacy_present_carries_uncheckpointed_wal_rows_and_leaves_legacy_byte_identical` asserts with `wal_autocheckpoint = 0` and the writer held open. That test is the right test for this function and it would have failed against the old implementation.

**Pre-cleanup — safe, and its stated warrant holds.** Lines 606–620 remove `temporary`, `<new_store>-wal`, `<new_store>-shm`, `<temporary>-wal`, `<temporary>-shm` and `<temporary>-journal`. The comment's justification is exactly right and I checked the control flow that makes it true: the pre-cleanup is reachable only after `legacy_project_store_source` returned `Some`, which it does only when `new_dir/PROJECT_STORE_FILE` does not exist (line 581). A write-ahead log or shared-memory file with no main database is not user data, so deleting it cannot lose anything — and deleting it is what closes the second half of my finding, the stale-sidecar resume. Removing `temporary` is also load-bearing in its own right: `VACUUM INTO` fails outright if the target exists, so without this the first killed attempt would have wedged the carry-forward permanently. A `remove_file` failure here returns before anything is created.

**Rename race check — unchanged and still adequate.** `if new_store.exists()` before `fs::rename` (lines 641–644) is still TOCTOU, but both racers would be writing byte-equivalent copies of the same source, so the benign outcome I noted last time is unchanged. It matters even less now: I re-confirmed that all seventeen `app_store_path` callers are synchronous `#[tauri::command]`s, which Tauri v2 runs on the main thread, and the two `async` commands do not touch the store path.

**Cleanup on failure — correct, including the exclusion.** Lines 646–656 remove every leftover *except* `<new_store>-wal` and `<new_store>-shm`. That exclusion looks odd until you trace the one path that needs it: the "a store appeared at the new location during the copy" error means some other writer now owns `new_store`, and those two sidecars would be its live files. Since the pre-cleanup already removed any pre-existing copies, anything at those paths at this moment is the interloper's. Skipping them is right, and removing them would have been the bug.

**Startup is never blocked.** `app_store_path` still logs and continues (lines 558–560); every failure path returns `Err` and is swallowed, leaving the user at an empty store in the new location with the legacy store intact. The `an_unreadable_legacy_store_fails_without_leaving_files_or_touching_the_legacy_store` test pins the non-database case and asserts both no residue and byte-identical legacy content.

**The legacy directory not being writable.** I measured this rather than reasoning about it. A WAL-mode database cannot be opened even read-only unless SQLite can create or write the `-shm` wal-index — in a directory chmod'd `0555` the open fails with `attempt to write a readonly database`, and that holds even when no `-wal` currently exists, because the WAL journal mode is persisted in the database header. So on a non-writable legacy directory the carry-forward returns `Err`, is logged, and the user gets an empty store. That is the safe direction and it destroys nothing, so it is not a defect. Two honest consequences follow, which I record below as observations rather than findings.

**Tests — they cover what they claim, with one arithmetic correction.** There are **six**, not seven: `new_store_present…`, `new_absent_and_legacy_present_carries_uncheckpointed_wal_rows…`, `both_absent_copies_nothing`, `leftovers_of_a_killed_attempt…`, `an_unreadable_legacy_store_fails…`, `legacy_directory_is_the_new_directorys_sibling_and_never_itself`. The module went 5 → 6 (one replaced, one added), which matches the lib total going 96 → 97. Coverage is genuinely better than before: the deleted `sidecars_are_copied_with_the_main_file` tested behaviour that no longer exists, and the two new tests exercise real SQLite databases with a live writer instead of `b"legacy"` byte blobs, check `PRAGMA integrity_check` on the result, and assert the legacy main file and `-wal` are byte-identical afterwards. The `assert_ne!` on the stale WAL in `leftovers_of_a_killed_attempt…` is looser than an `assert!(!exists())` would be but is correct as written (`VACUUM INTO` emits a delete-journal-mode database, so the read-only `marker_rows` open creates no `-wal` before that assertion runs). Not covered: the non-writable legacy directory, and a legacy store locked long enough to exhaust the 5-second `busy_timeout`.

**Ran:** `CARGO_NET_OFFLINE=true cargo test --lib` in `apps/desktop/src-tauri` → **97 passed, 0 failed**, all six carry-forward tests among them, no warnings. `STORE_SIDECAR_SUFFIXES` and `store_sidecar_path` are still live (the leftovers list uses both), so no dead code was left behind.

## Finding 2 — closed.

`docs/governance_harness/tranche_manifests/PIPING-SWBPIPE-A1-CLAIMS-LINT-20260918.yaml:57–59` adds exactly the three paths I named. I re-derived the check mechanically rather than eyeballing it: every one of the **107** paths changed by `origin/main...d7ff2e45a` — the candidate plus both record commits — is now covered by a `candidate_paths` entry, with none uncovered. The `m2_gate` authorization statement is untouched and still matches the activation record.

## Retained return — faithful.

`instances/A1-REVIEW/RETURN.md`, SHA-256 `c2a39462510105e111f1445acf19dbc10523ff7c9f2e9495ba3cc9afbc130914`, matching the value recorded in both `briefs/_INDEX.md` and `WORK_GRAPH.json`. It reproduces my final message under ROOT's standard four-line retention preamble, worded identically to the three children's. I spot-checked distinctive passages — the `lines 592–618` citation, the "safe read order is main file first, then WAL" sentence, the `lib.rs:843` and old `styles.css:3518` references, the `@media (max-height: 840px)` reasoning, the command table and the "What I did not check" list — all present and unaltered. The `{REPO_ROOT}`/`{RUN}` placeholders are the declared substitution. The sealed brief still hashes `98cf6915bf…50a`.

---

## The one finding

**Severity:** trivial. **Actionable before merge:** yes — one word.
**Location:** `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/briefs/_INDEX.md`, the A1-REVIEW retention paragraph (line 28): "*…renamed into place; **seven** unit tests; `cargo test --lib` 97 passed.*"
**What is wrong:** the carry-forward module holds **six** `#[test]` functions (`lib.rs:4370–4538`). I confirmed this three ways: by brace-matched count over the module, by the named test list in the runner output, and arithmetically — the module went from five to six (one test replaced, one added), which is why the lib total moved 96 → 97 rather than 96 → 98. The "97 passed" in the same sentence is correct; I measured it.
**Why it matters at all:** this is retained run evidence, and the count is the kind of thing a later inspection reads instead of re-running. It is the only reason this backcheck is not a PASS.
**Smallest correction:** change "seven unit tests" to "six unit tests". The same paragraph's characterization of Finding 1, of Finding 2, and of my 1440 × 899 judgment is accurate.

## Observations — not findings, no action required

- **The doc comment at lines 594–602 is very slightly broader than SQLite guarantees.** "The legacy store is never moved, deleted or modified" is true of the store's data — the test proves the main file and `-wal` are byte-identical — but a read-only open of a WAL-mode database does create or write the `-shm` wal-index in the legacy directory, and may leave it behind. If ROOT wants the comment to be exactly true, "its data and journal mode are never modified; the read needs write access to the legacy directory for SQLite's transient wal-index" would say it. I am explicitly not calling this actionable: the behaviour is correct, the fallback is safe, and the substance of the comment is right.
- **`<new_store>-journal` is absent from the pre-cleanup list** although `<temporary>-journal` is there. The same warrant would cover it for free (no main store exists, so a rollback journal there is not user data, and a hot journal beside the freshly renamed store is the one leftover class that could in principle be rolled back into it). I judge the scenario effectively unreachable — nothing in this app produces a delete-mode journal at that path without its main file, and SQLite validates a hot journal before replaying it — so this is offered as a cheap consistency tidy, not a defect.
- **The carried-forward store arrives in delete journal mode**, since `VACUUM INTO` writes a fresh database with the default. `apply_store_migrations` sets `PRAGMA journal_mode = WAL` on every open (`lib.rs:843`), so it returns to WAL on first use. Correct as-is; worth knowing.
- **Duration, not blocking.** On the one launch where a legacy store exists, the first store-touching command runs the `VACUUM INTO` on the main thread, plus up to the 5-second `busy_timeout` if the legacy store is locked. It cannot fail startup, and it is no worse in kind than the full file copy it replaces, but it is a first-launch pause proportional to store size.

## What I did not check

- **The rest of the candidate.** This backcheck covers only `8143645ea..d7ff2e45a`. My findings and judgments on the other 105 files stand from the retained return and were not re-derived; the correction commit does not touch them.
- **No Playwright, no dev server, no evidence sweep** — brief-excluded, as before. The e2e lanes remain unverified by me.
- **The non-writable-legacy-directory path in the product itself.** I established the SQLite behaviour by probe in my scratchpad with Python's `sqlite3` (3.53.1), not through the Rust code with rusqlite's bundled SQLite, and no test exercises it.
- **I did not exercise a legacy store large enough, or locked long enough, to reach the 5-second `busy_timeout`,** nor a real macOS `~/Library/Application Support` upgrade.
- **Ran, all read-only:** `CARGO_NET_OFFLINE=true cargo test --lib` (97 passed); `shasum -a 256` on the sealed brief and the retained return; a mechanical coverage check of `candidate_paths` against all 107 changed paths; a brace-matched test count over `legacy_store_carry_forward_tests`; and the WAL read-only permission probe in my scratchpad. I re-ran neither the claims lint nor the Vitest and pytest files, since the correction touches no scanned surface — `validate_claims_language.py` does not scan `src-tauri`, `.yaml` manifests or `execution/**`.

## Suitability

The code is suitable for fan-in now. Fix the one word in `_INDEX.md` and I have nothing further: no blocking, no major, no minor finding remains against `d7ff2e45a53096a324c25f6c0641bae5c6b36ebc`. Residual risk is unchanged from my first return except that the carry-forward's risk is materially reduced — what remains there is the untested non-writable-directory path, which fails safe, and the first-launch pause.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
