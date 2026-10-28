# Autosave and backup copies (survey item D5)

D5 is adopted under the owner's 2026-10-27 delegation (`loop/LOOP_INIT.md`).
Workflow: the user can turn on a timed autosave of the open project, and keep a
backup of the last saved version in a local folder they choose. Purpose: no work
is lost to a crash. PRD §20.4 (Local-First Operation) governs it. Risk class:
persistence.

## 0. Locality

This design lives in DEL-02-05, because DEL-02-05 owns the native project
persistence it extends: the local SQLite store and its create, open and save
commands (`apps/desktop/src-tauri/src/lib.rs`), the one model-document version
check (CLM-003) and the open path's document checks
(`model_document_file.rs`). DEL-12-01 owns the symbolic path classes and states
that "current native placement/persistence is in DEL-02-05". Its condition on
DEL-02-05 (deterministic round trip, one version check, no stored document
rewritten) is what this design must keep. The autosave directory is a
`USER_CHOSEN_PROJECT_PATH`-class location in the app's local data directory.
The backup folder is a user-chosen local path holding project models, which are
private by default (DEL-12-01 CLM-026).

## 1. What autosave writes, and where

**The record.** An autosave is one file in `<app local data>/autosave/`, the
sibling directory of the store file `openpipestress-projects.sqlite3`. Its
bytes are exactly the session's model document as unchecked RFC 8785 (JCS)
canonical text, with no trailing newline: the W-6a bytes that Save Model
Document and a saved model state hold. The file's SHA-256 is therefore the
session's model hash. It is a model document of the current version (0.8.0),
admitted by the open path's own checks (§5), so it re-opens through Open Model
Document unchanged. No new model-document member is added (§7).

**The name.** `<slot>-<written_unix_millis, 13 digits>-<model hash, 64 hex>.json`.
`slot` is the first 32 hex digits of the SHA-256 of the document's `project.id`
(as `project_id_from_model` reads it, `project:untitled-local` when absent).
The name therefore carries no project name or id. It does carry the record's
own hash, so a read checks the bytes against the name with no second file.
Writing a new name each time and then removing the slot's older records means
that every moment has at least one complete record. No file is ever replaced
in place, so the writer's macOS-only replace claim is not needed on any
platform.

**What it never touches.** An autosave does not open the store. It never writes
the stored project's row, its envelope hash or revision, its model states
(store v12) or its recorded runs (store v13). It never overwrites the user's
saved project or any file the user saved. A failed autosave leaves the store
and the previous record exactly as they were.

**Saved model states.** A model state is a named, immutable record the user
chose to keep, with a record hash, in the store. An autosave is none of these.
It is not listed in the history, is not a comparison basis, a run basis or a
handoff basis, and never becomes a state by itself. After recovery the user can
save a state from the recovered model with the existing Save State.

**Hash and ownership.** The record names its model hash only. On recovery the
document is opened as Open Model Document opens a file: the session has no
stored base. Save Local Project of a recovered model onto a stored id is
therefore refused `PROJECT-ID-EXISTS`, and the existing replace prompt shows
the stored revision and asks (HELP_HUMAN rulings 2026-10-10). Recovery never
replaces the stored project silently.

**Session content (checked against `workspaceSession.ts` and `write_project_rows`).**
- Applied edits are in the model document, so they are captured.
- Queued batches are operations not yet applied to the model. A project save
  folds them into its `editor_intents` context. With the agent proposal, the
  editor intents and the selected review target, they are envelope slots, not
  model-document members, so an autosave does not hold them. The recovery
  offer says so.
- Solve results: Save Local Project does persist the retained
  `mechanics_result` and `analysis_run` in the project row. An autosave does
  not. Results are not model-document members, and a recovered model has no
  solve standing, so it must be solved again. Recorded runs stay in the store,
  untouched.

**When.** The timer (§2) writes only while the session's `modelEdited` is
true (the unsaved-changes guard's one notion of "edited"). It writes only when
the model hash differs from the slot's newest record (`unchanged` otherwise),
and only when no project operation or prompt owns the session.

**Retention.** A slot keeps only its newest record. After a new record is
written and read back, the slot's older records are removed: names that parse
as that slot's records, regular files only (never a symlink), in that
directory only. When Save Local Project lands with the same model hash, the
desktop discards the slot's records, since the store now holds that model.
These are the app's own recovery records in its data directory, not files the
user saved. The settings text says that autosaves are removed once saved.

## 2. Timing and the preference

The interval is the user's choice: Off, 1, 2, 5, 10, 15 or 30 minutes. It is
Off by default. The existing preference mechanism is the webview's UI
preferences (`features/workspace/uiPreferences.ts`, localStorage key
`chirality.desktop.ui-preferences.v1`: theme, density, layout). That record is
not changed. The autosave and backup settings are a native settings file,
`<app local data>/autosave-settings.json`:

```json
{"document_kind": "swbpipe.desktop.autosave_settings", "version": 1,
 "autosave_minutes": 0, "backup_folder": null, "backup_keep": null}
```

It is written by the same atomic writer. An unreadable, unknown-version or
out-of-range file reads as Off, with a status line naming it. It is native
because the backup folder is a filesystem path that only the native folder
chooser produces. No native command takes a write path from the webview
(every existing save path comes from a native chooser), and the settings keep
it so. The interval lives in the same record, so one place governs both.

The timer is one webview interval. At most one autosave is in flight, and a
tick that finds one running is skipped. A refused autosave shows a status line
(`Autosave not written: <code>: <message>`), keeps the previous record and
retries at the next tick. A successful one shows `Autosaved at <time>`.

## 3. Recovery

At startup, after the most recent project opens, and whenever a stored project
is opened, the desktop lists the records (`list_project_autosaves`: names and
sizes only, no content read). It pairs each record with the stored project
whose id has the same slot. It offers a record when the record's model hash
differs from the stored project's, and the record was written after that
project was last saved. A record with no stored project (a model never saved)
is offered too. The offer is a modal dialog showing:
- what it is: the project name and id (read from the verified record), when it
  was autosaved, its size and its short model hash; and that queued batches,
  proposals, review context and solve results are not in it;
- what it would replace: the working model now open (and whether it has
  unsaved edits); and that the stored project `<name>`, last saved at
  `<time>`, stays unchanged until the user saves and confirms the replace
  prompt.

The choices are Open autosave, Keep for later (the default, which closes the
dialog and shows a status line) and Discard. Discard asks once more, then
removes that record. Open goes through the unsaved-changes guard, then reads
the record verified (§5) and adopts it as Open Model Document adopts a file
(webview shape check and validator), with no stored base. Nothing is applied
automatically. A record that fails verification is shown as unreadable, with
its code, and offers Discard only.

## 4. Backup of the last saved version

**What.** Once Save Local Project has landed, and only when backups are on,
the desktop asks the native side to write a backup. The backup is the stored
project's model document read back from the store, which is the last saved
version, not the session's copy. It is read as the recovery export reads it
(`stored_model_document_export`: store opened read-only and not migrated, a
metered read, the 64 MiB cap, each member once). Its SHA-256 must equal the
stored row's model hash value. It is screened for dispositions as the export
is: `protected_suspected`, `quarantined` and `rejected` refuse the backup with
`PROTECTED_CONTENT_BLOCKED`. A backup copies private data to a user folder, so
it needs the explicit local-private intent. That intent is recorded when the
user turns backups on and chooses the folder.

**Folder.** It is chosen through the native folder chooser
(`blocking_pick_folder`), which starts in the documents folder, and kept in the
settings file. Each backup checks that the folder still exists, is a directory
and is not a symlink, which is the atomic writer's own parent check.

**Name.** `<stem>.<slot, first 8 hex>.<UTC yyyymmddThhmmssZ>.<model hash, first 12 hex>.swbpipe-backup.json`.
`stem` is Save Model Document's suggested stem
(`suggested_model_document_file_name`). The slot part keeps two project ids
that share a stem apart. The file is an ordinary model document, so it opens
with Open Model Document.

**Write.** It uses the atomic writer under its own format (temporary prefix
`.swbpipe-backup-`), then reads the file back and checks its hash (§5). If the
newest backup of the project in that folder already holds the same hash,
nothing is written (`unchanged`).

**Rotation.** "Keep the newest N backups of each project" is the user's
choice: All (the default), 3, 5, 10, 20 or 50. With All, nothing is ever
deleted. With N set, and only after a new backup has been written and read
back, the project's backups in that folder beyond the newest N are deleted, by
the timestamp in their names. Only names that parse as that project's backup
pattern, regular files and the folder itself are considered: no recursion, no
symlinks, no other file. Lowering N deletes nothing at once; the next backup
applies it. The setting's text says exactly this. A deletion that fails is a
warning naming the file and never fails the backup.

**Refusal.** A missing, unwritable or full folder, a read-back mismatch, a
screening refusal or a memory refusal refuses the backup by name. The save
already landed and is unchanged. The status line reads
`Saved. Backup not written: <code>: <message>`. A backup never blocks, delays
or rolls back a save.

## 5. Integrity

- **Content.** Each record and each backup is the canonical text of a current
  (0.8.0) model document with its SHA-256. Before writing, the native side
  applies the cap (`MAX_MODEL_DOCUMENT_BYTES`), parses the text, requires it to
  be its own JCS form and requires its SHA-256 to equal the claim
  (`AUTOSAVE-HASH-MISMATCH`). It then applies the open path's checks
  (`check_model_document`: kind, current version, the W-13 validator's blocking
  findings) and the desktop shape check, so every record re-opens. A retired,
  newer or unparseable version is refused by the open path's own
  `MODEL-FILE-SCHEMA-REFUSED`. Nothing is migrated, defaulted or normalized.
- **Atomic write.** The extension-neutral writer
  (`atomic_report_package_save.rs`) writes a same-directory temporary file,
  flushes it, fsyncs it, renames it and syncs the directory within a bound. Its
  stages report `AUTOSAVE-WRITE-FAILED`, `-FLUSH-FAILED`, `-FILE-SYNC-FAILED`
  and `-RENAME-FAILED`, each with the OS error kind and code. The temporary
  file is removed on failure.
- **Failure.** A failed write leaves the store unread and untouched, the
  previous record in place and the failure reported. A crash before the rename
  leaves at most a `.swbpipe-autosave-*.tmp` file, which no listing or recovery
  ever reads. A failed directory sync after the rename is reported as
  `saved_durability_uncertain`.
- **Read-back.** After the rename, the file is streamed through SHA-256 in
  64 KiB blocks and compared with the model hash. On a mismatch
  (`AUTOSAVE-READBACK-MISMATCH`) or a read error (`AUTOSAVE-READBACK-FAILED`),
  that new file alone is removed and the previous record is kept.
- **Read for recovery.** The name must parse, which also refuses path
  separators. The file must be a regular file within the cap, and its SHA-256
  must equal the hash in its name (`AUTOSAVE-INTEGRITY`). Then the content
  checks above run again, and the document's slot must be the name's slot.

## 6. Memory

Slice 2 adds these native commands under the DEL-04-07 ingress rules
("Model-sized interactive commands"):

| Command | Arguments | Lease |
|---|---|---|
| `write_project_autosave` | `{canonicalText, claimedModelHash}`, a model | `command_ingress::admit(…, ArgsDecoding::Moved)`: envelope(raw) + M + 4 KiB, held until return. Within it (`PoolReadMemory` seeded with the lease): the text's parse (`parse_bytes`), its canonical copy (`is_jcs_text`), a fixed 64 KiB read-back block and the response's exact length. |
| `read_project_autosave` | `{fileName}` | `admit_within` the history commands' 256 KiB. The file's load (`loaded_text_bytes(len)`) is reserved before it is read, then its parse, its canonical copy and the response (`respond_metered`). |
| `list_project_autosaves`, `discard_project_autosave` | none, or `{fileName}` | `admit_within` 256 KiB. The listing is at most 1,024 names; more are counted, not listed. |
| `write_project_backup` | `{projectId}` | `admit_within` 256 KiB. The stored text's read as the recovery export's (`StoreLibraryReads`, 1.5 MiB before the store opens), the screening and the read-back block. |
| `get_autosave_settings`, `set_autosave_settings`, `choose_backup_folder` | scalars | `admit_within` 4 KiB. |

The slice-1 functions take `&dyn ReadMemory` and reserve exactly these terms,
so the commands only seed the memory from their lease.

## 7. Boundaries

- **No cloud.** Nothing is sent anywhere. The autosave directory is in the
  app's local data directory. The backup folder is wherever the user chooses.
  The chooser starts in the documents folder, which some systems sync; the
  setting's text says that backups hold the project's private data and go where
  the user chooses. No folder is suggested as a default.
- **No new model-document member.** The record's identity (slot, time, hash)
  is in its file name. The settings are a separate native file.
- **Private data stays local.** Autosave names carry no project name or id.
  Backup names carry the same stem Save Model Document suggests. Nothing goes
  into telemetry, the repository or an issue bundle. Tests use invented
  documents in temporary directories.
- **No acceptance or issuance.** An autosave or a backup is a recovery copy of
  the user's model. It is not a state, a run, a review record or a handoff, and
  nothing in its name, status or text implies acceptance, approval or issuance.
- **Out of scope.** Rule-pack and code-check data are not touched. A rule pack
  stays in its own store table.

## 8. Slices

| Slice | Content | Tests | Gate |
|---|---|---|---|
| 1 | This design. `autosave.rs`: slot and name, verified atomic record write with read-back and own-slot pruning, listing, verified read. The writer's I/O seam shared at crate visibility. | Rust: exact bytes and hash; `unchanged`; injected disk full (ENOSPC) at write; real permission denied (read-only directory); injected interrupted rename; crash-before-rename temp ignored; read-back mismatch removes only the new file; store file untouched in every case; tampered and renamed records refused on read; non-current and malformed documents refused; pruning removes only the slot's own records. | Independent review of the module (persistence) before slice 2 builds on it. |
| 2 | Commands of §6 for autosave and settings; the desktop timer, status lines, recovery offer and discard; discard after a landed save. | Rust: command arguments and leases. Vitest: Off by default; writes only while edited and on the interval; one in flight; refusal display; offer shown only for a newer record; shows what it replaces; never auto-applied; Open goes through the guard; Discard confirms. | Independent review. Native smoke stays with the owner. |
| 3 | Backup: folder chooser, settings, write after a landed save, rotation, refusal display. | Rust: backup equals the stored text and hash; rotation with All deletes nothing; N deletes only that project's pattern, never symlinks or other files; missing, read-only and full folder refusals; screening refusal. Vitest: settings, refusal line after a landed save. | Independent review (rotation deletes files). |

Each test must fail without its wiring. The slice-1 evidence is in its PR.
