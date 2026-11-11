# Autosave and backup copies (survey item D5)

D5 is adopted under the owner's 2026-10-27 delegation (`loop/LOOP_INIT.md`).
Workflow: the user can turn on a timed autosave of the open project, and keep a
backup of the last saved version in a local folder they choose. Purpose: no work
is lost to a crash. PRD §20.4 (Local-First Operation) governs it. Risk class:
persistence.

## 0. Locality and decisions

Agent 0 accepted on 2026-10-28, under the owner-accepted release target (exit
criterion 10), the three choices this design proposed: the native settings file (§2),
backup rotation defaulting to All (§4) and the backup taken from the stored
document (§4). It also set autosave on by default (§2).

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
session's model hash. It is a model document of the current version (0.9.0),
admitted by the open path's own checks (§5), so it re-opens through Open Model
Document unchanged. No new model-document member is added (§7).

**The name.** `<slot>-<written_unix_millis, 13 digits>-<model hash, 64 hex>.json`.
`slot` is the first 32 hex digits of the SHA-256 of the document's `project.id`
(as `project_id_from_model` reads it, `project:untitled-local` when absent).
The name therefore carries no project name or id. It does carry the record's
own hash, so a read checks the bytes against the name with no second file.
Writing a new name each time, and removing only records strictly older than
it once it is durable (§5), means that every moment has at least one complete
record. No file is replaced in place, except when one writer rewrites a
record that fails to verify under the same name.

**What it never touches.** An autosave does not open the store. It never writes
the stored project's row, its envelope hash or revision, its model states or
its recorded runs (current store v14). It never overwrites the user's
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

The slot is a consistency check, not authentication. It shows that a record's
document belongs to the project its name records. It cannot show who wrote
it: anything that can write the app's data directory can write a record that
passes every check. Recovery therefore always remains an explicit offer the
user accepts or declines (§3).

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
the model hash differs from the slot's newest record, and only when no project
operation or prompt owns the session. `unchanged` (nothing written) needs that
newest record to read back as its hash; a record truncated or altered under its
name does not count, and a new record is written.

**Retention.** A slot normally keeps only its newest record. After a new
record is written, read back and made durable (§5), the slot's records strictly
older than it, by written time, are removed. Only names that parse as that
slot's records are removed, regular files only (never a symlink), in that
directory only. A record that another window wrote later is kept. This pruning
replaces retained records automatically; only applying a recovery needs the
user. When Save Local Project (or New Local Project) lands, the desktop
discards the slot's records whose model hash is the saved model's, since the
store now holds exactly that model. A save never removes a record with any
other hash, whatever its time: only the hash proves that a record's work is
in the store, and another window may write newer edits around the save, or
the user may have kept the record for later.
These are the app's own recovery records in its data directory, not files the
user saved. The settings text says that autosaves are removed once saved.

## 2. Timing and the preference

Autosave is on by default, every 5 minutes. The user can choose 1, 2, 5, 10,
15 or 30 minutes, or turn it off, and it writes only while there are unsaved
edits (Agent 0, 2026-10-28, under the owner-accepted release target:
"on by default", so that a crash loses at most one interval, as exit
criterion 10 of `docs/RELEASE_TARGET_v1.0.md` states). For as long as it is
off, a one-line notice that a crash can lose unsaved work stays in the
toolbar, taken from the settings and apart from the status line, so no later
status replaces it (§8, slice 2). The existing
preference mechanism is the webview's UI
preferences (`features/workspace/uiPreferences.ts`, localStorage key
`chirality.desktop.ui-preferences.v1`: theme, density, layout). That record is
not changed. The autosave and backup settings are a native settings file,
`<app local data>/autosave-settings.json`:

```json
{"document_kind": "swbpipe.desktop.autosave_settings", "version": 1,
 "autosave_minutes": 5, "backup_folder": null, "backup_keep": null,
 "backup_local_private_intent": false}
```

`autosave_minutes` is 1 to 30, or 0 for off. A missing file reads as the
default, 5 minutes. It is written by the same atomic writer. An unreadable,
unknown-version or out-of-range file reads as the default, with a status line
naming it, and is rewritten only when the user changes a setting. No slice-1
code holds a settings default; slice 2 adds the file with these values. It is native
because the backup folder is a filesystem path that only the native folder
chooser produces. No native command takes a write path from the webview
(every existing save path comes from a native chooser), and the settings keep
it so. The interval lives in the same record, so one place governs both.

Backup changes and interval changes preserve each other's settings under
`autosave-settings.lock`. Backup configuration first takes `swbpipe-backup.lock`,
then the settings lock; interval changes take only the settings lock. A backup
holds the backup lock from its settings read through its last effect. An
acknowledged Disable therefore precedes every later backup's authorization.
A legacy folder without recorded intent stays unused and is shown as off.
Existing finite `backup_keep` values stay readable and are not rewritten on read
or interval changes; they produce `BACKUP-ROTATION-UNAVAILABLE` instead of a
backup. Disable preserves that value; explicitly enabling All replaces it with
null. Settings-read refusals remain refusals rather than becoming an off state.
A configuration whose response is lost has an unknown outcome, never "not
changed": the window re-reads the settings, says the outcome is unknown, and
keeps Disable available until a later definite outcome, because the change may
already have committed.

The timer is one webview interval. At most one autosave is in flight, and a
tick that finds one running is skipped. A refused autosave shows a status line
(`Autosave not written: <code>: <message>`), keeps the previous record and
retries at the next tick. A successful one shows `Autosaved at <time>`.

## 3. Recovery

At startup, after the most recent project opens, and whenever a stored project
is opened, the desktop lists the records (`list_project_autosaves`: names and
sizes only, no content read). It pairs each record with the stored project
whose id has the same slot. Every record whose model hash is not shown to be
the stored project's model stays recoverable: a wall clock that was set back
can make newer work look older, so the times never hide a record. At these
moments the dialog is offered for a record with no stored
project (a model never saved), and for one written, by this Mac's clock,
after that project was last saved, or within the same second. A record the
clock calls older is not offered then; a status line says that it is kept,
states both times, and names Recover autosave…, which is in the Appearance
popover beside the interval and in the project panel. Recover autosave…
offers every record not shown to be its stored project's model, newest first,
whatever the times. The texts compare the record with the stored document
"by this Mac's clock", by their written and last-saved times, and say which
is newer, that the same second cannot tell, or that there is no stored
project; they never silently prefer either. The offer is a modal dialog
showing:
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

## 4. Nondeleting backup of the last saved version

Agent 0 selected a bounded nondeleting unit. Automatic rotation remains
unfinished: checking an entry before a name-based removal does not establish
that only the verified file is removed under concurrent interference. No
weaker deletion promise or residual-risk acceptance is inferred. This unit
does not complete RT-U3's remaining rotation or release-volume qualification.

**Stored source.** After a verified Save Local Project response, the session
starts an independent native request with the saved project id and full model
hash. The store is opened read-only without migration (current store v14).
Its compact stored model text is strictly parsed once per member, canonicalized
as RFC 8785, checked against the stored hash and expected saved hash, and
required to name the explicit project id. Current model 0.9.0/open checks and
native disposition screening apply, including referenced libraries. A stale
store model or protected/quarantined/rejected disposition refuses the backup.
Only canonical model bytes are copied; queued batches, proposal/review context
and solve results are excluded. Backup completion never delays, rolls back or
changes the successful save. Superseded saves/sessions publish no backup status.

**Opt-in and folder.** The Appearance control offers All only and requires
explicit private-copy intent before the native folder chooser. Before enabling,
read-only `fgetattrlist` on the held folder queries `ATTR_VOL_CAPABILITIES`
(with `ATTR_VOL_INFO`); both the validity and capability bits for
`VOL_CAP_INT_CLONE` must be present in `VOL_CAPABILITIES_INTERFACES`.
Unsupported/unknown results refuse without probe files or settings changes.
This selects macOS volumes reporting clone support, not a filesystem-name
whitelist; actual clone-capable same-filesystem volume qualification remains
open. Other platforms refuse this route. Each backup repeats capability and
actual write/durability checks; the enable-time result is no future guarantee.
The folder's final component must not be a symlink. Each operation reopens the
configured path and holds its resulting directory handle; it does not claim
that external software cannot replace that folder between operations.

**Identity and publication.** The name remains
`<stem>.<slot8>.<UTC yyyymmddThhmmssZ>.<hash12>.swbpipe-backup.json`.
The stem is derived from project id. Short slot/hash parts only filter names;
full expected hash verification establishes an unchanged copy. A collision is
never overwritten. A new temporary regular file is created exclusively with
mode 0600, written and synced. macOS `fclonefileat` clones its still-held source
FD to one absent destination entry relative to the held folder FD. The source
name is never resolved again. No named-source rename/link or direct-write
fallback is permitted. Unsupported/unknown capability, unsupported clone or
cross-filesystem refusal remains explicit.

**Verification and durable result.** Final reads open one component relative
to that folder, without following links; inspected/opened identity, type, size,
mode 0600 and full expected hash must agree. Observed same-size mutation and
name changes during read-back refuse verification. Final-file sync and bounded
folder sync must complete, followed by a contained recheck, before
`written`/`unchanged` reports verified success. The recheck requires the name
to still refer to the synced file, unchanged (same device, inode, size and
modification and change times), with the full expected hash: identical bytes
in a substituted file, or rewritten in place, are not verified. An existing same-hash copy also
needs these durability checks. These checks do not prevent later external
mutation; advisory locks do not enforce same-user custody.

**No deletion and truthful failures.** No backup-copy path removes any file,
including temporary, partial, failed or unverified copies. Storage grows; the
control says this before enabling. A known refusal before final creation says
so and names any retained temporary file. A created but unverified final copy,
an existing copy whose durability cannot be verified, and an unknown publication
outcome have distinct diagnostics. Transport/worker failure is an unknown
outcome, never proof that no copy exists. Every older and unrelated entry stays
untouched. Automatic rotation requires a separately selected interference model
and supported promise before implementation.

## 5. Integrity

This section describes autosave records. Backup publication and its nondeleting
failure handling are in §4; they do not use the shared atomic writer.

- **Content.** Each autosave record is the canonical text of a current
  (0.9.0) model document with its SHA-256. Before writing, the native side
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
- **One transaction per slot.** The newest-record check, the write, the
  read-back and the pruning run while the slot's exclusive lock is held. The
  lock is `flock(LOCK_EX)`, through std `File::try_lock`, on `<slot>.lock` in
  the records directory, polled every 10 ms for at most 5 s
  (`AUTOSAVE-LOCK-BUSY` after that). An flock lock belongs to the open file
  description, so two threads that each open the file, two windows and two
  processes exclude each other on macOS, while separate slots never contend.
  The kernel releases it when the file closes, also when a process crashes.
  The lock file is never removed, because removing it would let a later opener
  lock a different file. After it is opened, `fstat` must show a regular file.
  Once it is locked, its name must still name that file (`fstatat`), or the
  file is opened and locked again, at most 8 times. Residual: the lock
  coordinates SWBPIPE's own writers. Something that can rewrite the app's data
  directory can still split it, as it can write records, which is why every
  record is verified and recovery is an explicit offer. Two openers creating it at once can see `ENOENT` from
  `O_CREAT` on macOS; that open is retried, at most 16 times. Even without the
  lock, pruning only strictly older records keeps the newest record of every
  writer.
- **Durability before pruning.** Every write, after its read-back, syncs the
  parent of `autosave/`, whether the directory was created now, already
  existed, or was created by a concurrent writer. Durability is never inferred
  from the directory existing. When that sync, or the directory sync after the
  rename, fails, is suppressed or times out, the write reports
  `saved_durability_uncertain` with `AUTOSAVE-PRUNE-DEFERRED`, and every older
  record is kept. The next durable write prunes them.
- **Resource failures never remove a record.** A check of a record against its
  hash has three results. It matches; it verifiably differs (its bytes hash
  otherwise, or it exceeds the cap); or it cannot be made (a refused
  reservation, a permission or I/O error, an entry that vanished, changed or is
  not a regular file). Only a record that verifiably differs may be replaced,
  or, when it is the one just written, removed. A check that cannot be made
  writes nothing and removes nothing: `AUTOSAVE-VERIFY-UNAVAILABLE` for the
  `unchanged` check or an existing entry of the same name, and
  `AUTOSAVE-READBACK-FAILED` for a read-back, whose new record is kept
  unverified. An existing valid record of the same name that is not the newest
  is refused `AUTOSAVE-NAME-IN-USE`. The previous valid record is pruned only
  after the new one has verified and is durable.
- **Failure.** A failed write leaves the store unread and untouched, the
  previous record in place and the failure reported. A crash before the rename
  leaves at most a `.swbpipe-autosave-*.tmp` file, which no listing or recovery
  ever reads.
- **Read-back.** After the rename, the file is streamed through SHA-256 in
  64 KiB blocks and compared with the model hash. On a mismatch
  (`AUTOSAVE-READBACK-MISMATCH`), the new file alone is removed and the
  previous record is kept. On a read error (`AUTOSAVE-READBACK-FAILED`),
  nothing is removed: the new file is kept unverified, and no older record is
  pruned.
- **Containment.** Every read, including the read-back and the `unchanged`
  check, opens the records directory `O_DIRECTORY | O_NOFOLLOW`, so a symlinked
  `autosave/` is refused (`AUTOSAVE-DIRECTORY-UNAVAILABLE`). It then resolves
  each name once, relative to that handle. A record is first inspected with
  `fstatat(AT_SYMLINK_NOFOLLOW)`: it must be a regular file within the cap.
  It is opened `O_NOFOLLOW | O_NONBLOCK`, and `fstat` on the descriptor must
  show a regular file with the inspected device, inode and size. A record
  swapped for a symlink, replaced or grown after inspection is refused
  (`AUTOSAVE-UNREADABLE`). The read is hard-capped at the inspected size, which
  is the size reserved; the buffer is allocated once, and a longer or shorter
  file is refused. Enumeration keeps one entry at a time for writes and
  pruning. A listing keeps at most 1,024 newest entries in a bounded heap,
  reserved first (512 bytes an entry), and counts the rest.
- **Read for recovery.** The name must parse, which also refuses path
  separators. Under the containment above, the file's SHA-256 must equal the
  hash in its name (`AUTOSAVE-INTEGRITY`). Then the content checks above run
  again, and the document's slot must be the name's slot.

## 6. Memory

Slice 2 adds these native commands under the DEL-04-07 ingress rules
("Model-sized interactive commands"):

| Command | Arguments | Lease |
|---|---|---|
| `write_project_autosave` | `{canonicalText, claimedModelHash}`, a model | `command_ingress::admit(…, ArgsDecoding::Moved)`: envelope(raw) + M + 4 KiB, held until return. Within it (`PoolReadMemory` seeded with the lease): the text's parse (`parse_bytes`), its canonical copy (`is_jcs_text`), a fixed 64 KiB read-back block and the response's exact length. |
| `read_project_autosave` | `{fileName}` | `admit_within` the history commands' 256 KiB. The file's load (`loaded_text_bytes(len)`) is reserved before it is read, then its parse, its canonical copy and the response (`respond_metered`). |
| `list_project_autosaves`, `discard_project_autosave` | none, or `{fileName}` | `admit_within` 256 KiB. The listing is at most 1,024 names; more are counted, not listed. |
| `write_project_backup` | `{projectId, expectedModelHash}` | `admit_within` 256 KiB. Before effects, a fixed completion reservation covers `exact_text_bytes(MAX_RESPONSE_BYTES) + 2 × MAX_RESPONSE_BYTES` for bounded diagnostics, their copies and serialization. Initial command/SQLite memory (1.5 MiB) precedes store open; model load/parse/canonicalization and referenced-library screening are metered. Read-back plus bounded name scan reserves 64 KiB + 4 KiB. The completion reservation stays through IPC construction; no response admission follows publication. |
| `get_autosave_settings`, `set_autosave_settings` | scalars | `admit_within` 4 KiB; existing autosave accounting. |
| `configure_project_backup` | `{enabled, backupKeep, explicitLocalPrivateIntent}`; enabling requires null keep (All) | `admit_within` 4 KiB, plus the same preadmitted completion cover before chooser/settings effects. |

The slice-1 functions take `&dyn ReadMemory` and reserve these terms
themselves: the parse, the canonical copy, the read-back block, the file's
load and the listing's entries. The commands only seed the memory from their
lease. Slice 2 still has to verify the admission of `write_project_autosave`
(envelope + materialization + 4 KiB) against its measured peak, with the
counting-allocator tests the other model-sized commands use.

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
| 1 | This design. `autosave.rs`: slot and name, the per-slot locked write transaction with read-back, durability-gated pruning of strictly older records, a bounded listing and a contained, verified read. The writer's I/O seam, including its directory sync, shared at crate visibility. | Rust: exact bytes and hash; `unchanged`; injected disk full (ENOSPC) at write; real permission denied (read-only directory); injected interrupted rename; crash-before-rename temp ignored; read-back mismatch removes only the new file; store file untouched in every case; tampered and renamed records refused on read; non-current and malformed documents refused; pruning removes only the slot's own strictly older records. Two writers on one slot always leave a valid record; a write waits for its slot's lock and another slot does not; a failed directory sync keeps older records; creating the directory syncs its parent; an `unchanged` record that does not verify is rewritten; a listing beyond its limit is bounded and reserved; a symlinked directory, a record swapped for a symlink and a record that grows are refused; the capped read never reads past its cap. A failed parent sync keeps older records on a later write too; a same-time retry whose check cannot be reserved keeps the only record; a read-back that cannot be reserved keeps new and older records; a corrupt record of the same name is replaced; a writer started between read-back and prune waits (forced by a hook) and a record survives; a lock file that is not a regular file is refused. | Independent review of the module (persistence) before slice 2 builds on it. |
| 2 | Commands of §6 for autosave and settings; the desktop timer, status lines, recovery offer and discard; discard after a landed save. Required (Agent 0, 2026-10-28): (a) turning autosave off shows a one-line notice that a crash can lose unsaved work; (b) recovery compares the autosave with the stored document and says which is newer before it offers Open, Keep for later or Discard, never silently preferring either. | Rust (`autosave_commands_tests.rs`): each command admitted under its §6 rule (the write with its whole body, leased envelope + M + 4 KiB; the record commands within 256 KiB; the settings commands within 4 KiB); each command's named arguments; the write runs in memory seeded with its lease, and its counting-allocator peak is within what it held; a record listed, read verified and discarded in the app data directory; a discard never follows or removes a symlink; the settings default (5 minutes, on) and the file's exact text; each bad settings file (not JSON, not an object, another kind, an unknown member, a folder that is not a path, too large, a directory, a symlink, version 2, no version, 7, 31, -1, 2.5, "5" or no minutes, keep 4) reads as the default with a status naming it and is not rewritten; a settings read that cannot be reserved reads as the default; setting the interval writes the file atomically and refuses values that are not choices. Vitest (`features/autosave`): on by default every 5 minutes and writes the canonical text on that interval; (a) the off notice, and no writes while off; (a) the off notice stays while off through later status lines (Keep for later, a write landing after autosave was turned off); a settings file that could not be used shows its status line; writes only while edited, only on the interval, never while the session is busy; one in flight; the refusal line and a retry at the next tick; after a landed save, only that project's records holding the saved model's hash are discarded, and a record with another hash (kept for later, or written by another window just before or after the save) is kept; the offer shown only for a newer record whose model differs; (b) which is newer by this Mac's clock, both directions, and the same-second and no-stored cases; a differing record the clock calls older (a clock set back) is stated as kept at start and offered by Recover autosave…, which also says when there is nothing to recover; what the record is, what it replaces and that the stored project stays unchanged; never applied by itself (Keep for later focused, modal hold taken and released, focus returned); Open hands the record to the session's open; Discard asks once more; an unreadable record offers Discard only; the pairing rule. Session (`autosave.session.test.tsx`): Open autosave goes through the unsaved-changes guard (Cancel keeps the edits; Discard edits adopts the record, read verified again, with no stored base); Save Local Project discards the project's autosave holding the saved model once it lands, and keeps one with another hash. A quit while the recovery offer is open closes nothing: the offer takes the shared modal hold, which the close guard (#1717) refuses on. | Independent review. Native smoke stays with the owner. |
| 3 | Nondeleting backup: native folder capability preflight, explicit intent/All, stored canonical model after a verified save, FD-source absent-target clone, verification/durability and truthful retained-output diagnostics. Rotation remains unfinished. | Maintained `backup_tests.rs`: capability validity/refusal without probes; legacy intent/finite settings; Disable ordering/read-memory refusal; stored hash/identity/duplicate/screening refusal; completion admission and allocator peak; actual macOS clone/open path; source-name substitution; EEXIST/unsupported clone without fallback; partial/final/temp preservation; source/final mutation; final-name/symlink swaps; same-content replacement or in-place rewrite during sync; source/final/folder durability failure. Frontend tests cover opt-in, All-only/legacy finite state, save/backup independence, unverified/unknown outcomes, a lost configuration response and stale sessions. The native tests establish only the owned host temporary filesystem; release-volume qualification remains open. | Final independent persistence/permissions and exact changed-text review after the actual S6/CE predecessor, plus guarded targeted execution. |

Each test must fail without its wiring. The slice-1 evidence is in its PR.
