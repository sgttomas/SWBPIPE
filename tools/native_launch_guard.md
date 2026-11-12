# Native launch guard

`tools/native_launch_guard.py` keeps scripted launches of the desktop application inside a
throwaway home that the kit makes and removes, and then checks the consequences. One object,
`Run`, owns a launch from admission to removal, and it is the only way the kit starts the
application; the module has no other start. The agent journey controller
(`tools/agent_journey_fixture.py`) and the agent live-session fixture
(`tools/agent_live_session_fixture.py`, every case) use it. The native-smoke runner (`run.mjs`)
and the other Node runners do not yet.

What the application is confined to, and nothing more: it is confined as to where it reads and
writes its own files, and as to what environment it is handed. Observation of the consequences
comes second. With no operating system boundary around the application itself, sockets elsewhere
in the account stay reachable to it, and home redirection cannot change that. Every run's record
carries both sentences.

The helper launches on macOS only; on any other system it refuses to launch.

## Why both home variables

The system web view does not honour `HOME` alone. With only `HOME` set, it keeps its website data
under the account's real `Library`, in folders named for the product's bundle identifier or its
executable's name. With `CFFIXED_USER_HOME` set to the same throwaway home as well, the data goes
under the throwaway home's own `Library`, and removing the home removes it. The kit refuses to
launch without both.

## A run

```python
run = native_launch_guard.Run(executable, evidence_folder, "home", names=...)
run.begin()                       # admission, baseline, home
process = run.start("app", [str(executable)], {DRIVER: "-", PLAN: plan_json, REPORT: RUN_OWNED},
                    web_view=True, stdin=subprocess.PIPE, stderr=subprocess.PIPE)
...
outcome = run.finish(retain=False)  # comparison persisted, home removed, second look
```

### `begin`

1. **Evidence folder.** It must be absolute, a real folder with no link on its path, and neither
   under the account's `Desktop`, `Documents`, `Downloads` or `Library` nor under this repository.
2. **Admission**, again before every start:
   - **System:** macOS.
   - **Screen:** the system registry's console-user entries for this account are found, agree
     with one another (conflicting or ambiguous entries refuse), are on the console, and carry no
     locked key; a locked key present with any value refuses. Two further signs are not used: the
     screen saver, because which process runs while it is active could not be established by a
     harmless read; display sleep, because the display's power value while asleep could not be
     established by a harmless read.
   - **Slot:** an ancestor of the process is the slot wrapper's lock program (`/usr/bin/lockf`),
     alive, of this account, with a slot lock file (`cargo_job.lock` or `cargo_job.slotN.lock`)
     open, read with the system's open-files lister. With `guard_dir`, the lock file must be in
     that folder; the record says whether it was given. The kit takes no lock and touches nothing
     of the wrapper's. **Limit:** a lock program started by hand with a file of that name open
     would pass. Hosted runners have no slot wrapper, so they refuse.
   - **Strict chooser lead:** the executable's bytes contain the strict chooser variable's name.
     This is a lead and not a proof; the tie is the comparison test against the product's own
     constants, which reports "could not compare" until the product change is on main.
   - **Left homes:** the inventory below. Every left home is reported by name; the launch is
     refused only when the kit cannot establish that no process started from a left home is alive.

   What admission established is kept in the run's record.
3. **Baseline.** The listing below. An incomplete baseline refuses the launch: an unreadable or
   linked watched top folder, an unreadable product-named folder, a folder changed during the
   walk, a walk too deep.
4. **The home.** The kit keeps its own folder, `swbpipe-launch-kit`, under the system's per-user
   temporary folder (`getconf DARWIN_USER_TEMP_DIR`), private to this account, holding a
   `registry` and the `homes`. It writes the run's own record (with a token only the run holds)
   into the evidence folder, enters the home in the registry, and only then makes the home with
   its `tmp` and `work` folders, recording their device and inode. A failure after the folder is
   made and before the entry is complete leaves the entry in the inventory. A caller cannot name
   the home or supply a record: the registry is the authority.

### `start`

- **Environment**, built entirely by the helper, never from the caller's:
  - `PATH=/usr/bin:/bin` (fixed), `LANG`;
  - `HOME` and `CFFIXED_USER_HOME`, both the throwaway home;
  - `TMPDIR`, the home's `tmp` (the bundled database library also reads it for its temporary files);
  - `SWBPIPE_NATIVE_SMOKE_STRICT_CHOOSER=1`;
  - the kit's own variables a caller passes, each with a value the kit owns: the driver only on
    standard input (`-`); the provider key only as the marker `stdin`; the report and the gate
    only as `run-owned`, which the helper replaces with fixed files inside the home's `work`; the
    gate mode only `handshake`; the plan only as JSON in its closed shape (no other key, every
    value of its type, an http(s) endpoint, no string that looks like a path).

  Refused by name: the injection variables, `DYLD_`, `LD_` and `XDG_` variables, proxy variables,
  credential-like names, and anything else not on the list.
- **Working directory:** the home's `work`, whatever the caller's own.
- **Process options:** only `stdin`, `stdout` and `stderr`, each a pipe or nothing; every other
  descriptor is closed, and passed descriptors, a pre-start callback, a new session, another
  environment or working directory are refused.
- **Identity:** the process handle, and its identifier and start time together (its generation),
  are recorded at once, in the run and in the registry; a start time that cannot be read leaves
  the generation unestablished, which withholds removal.
- **The solve worker.** The journey's solve starts a solve worker: the application's own
  executable, with its environment cleared and not sandboxed. It inherits the application's
  working directory, inside the throwaway home, but not the home variables, `TMPDIR` or the strict
  chooser variable; its confinement depends on a product change carried in another pull request.

### `finish`

Comparison made and persisted first, then the home's removal, then a second look, then the run's
record. Whatever raises on the way, the later steps still run. The terminate, interrupt and hang
up signals are handled while a run is open, so these steps run and the signal is re-raised
afterwards; an interrupt during the waits skips nothing. A kill that cannot be caught leaves the
home to the inventory.

## The listing

Names, sizes and times only, by `lstat` through folder descriptors. Each folder entered is opened
without following a link and checked to be the folder that was seen, so neither a top folder that
is a link nor a folder swapped for a link during the walk leads anywhere.

- **Walked in full:** the `Library`'s `WebKit`, `Caches`, `HTTPStorages`, `Application Support`,
  `Preferences`, `Saved Application State` and `Logs` (the application's log folder lies there),
  and the system's per-user cache folder (named in records by what it is, never by its path).
- **One level deep, by name:** every other top folder of the `Library`, for a product-named child.
- **Four folder times:** the home folder, `Desktop`, `Documents` and `Downloads`, by a single
  `lstat` each, never listed. A change is reported, not concluded from.
- **Matching:** a path component equal to a watched name (the bundle identifier, the executable's
  name, and the executable's name in its underscore form) or beginning with one followed by a
  dot; after a plus sign, also one followed by a hyphen.

| Class | What it covers | Effect |
| --- | --- | --- |
| product | A created, modified or removed product-named entry; an unreadable product-named folder. | The run stops (exit 1). |
| web view helper cache | Changes inside `com.apple.WebKit.{Networking,WebContent,GPU}+<name>…` directly under the system's per-user cache folder. Whether both home variables redirect these folders is not established; where they do not, it is a write outside the throwaway home that confinement does not reach: observed, not prevented. | The run stops, always (exit 6). |
| others | Any other entry created or modified in the window: counted per folder, named only directly under `Preferences` and `Preferences/ByHost`, never opened, edited or removed. | Never fails the run. |
| nothing | Nothing changed. | — |

**Fail closed.** An unreadable or linked watched top folder, a cache folder the system does not
give, a folder changed during the walk, or a walk too deep fails the comparison (exit 5). An
unreadable folder that is not product-named is counted, never named, and the record says "N
folders could not be listed and were not watched"; a different count after the run than at the
baseline fails the comparison.

**Positive half.** A file must be found under the throwaway home's `Library/WebKit` or
`Library/Caches` after a run that opened a window, or the run fails (exit 3). Any file there
satisfies it; it does not identify the web view's own data.

**Window.** Two launches whose windows overlap cannot be told apart. The journey controller takes
a host-wide lock; other runners are not serialised by this helper.

**What was watched,** stated in every report: nothing outside these folders and the throwaway
home. A clean comparison establishes neither that nothing was read nor that nothing was written
elsewhere on the disk.

## Removal

Before the throwaway home is removed, no process the kit started or can attribute to the launch
is alive. The check (macOS only; on any other system it cannot run, so the home is withheld)
counts as a possible writer:

- a process the run started and has not seen end;
- a recorded generation still alive (identifier and start time both matching), or one whose start
  time could not be read;
- a descendant of such a generation;
- a process in a group attributed to the run;
- a process whose executable lies under the home or the private copy;
- a process with a file or its working directory open under the home.

An incomplete or unparsable listing of processes or open files refuses; it never reads as an empty
result. The record says what coverage was achieved.

**The residual risk, in the record's words:** a system started web view helper that holds the
home's path only in memory, with no open file and no working directory there and an executable
that is not ours, cannot be seen; if one wrote after removal it would recreate a folder under the
system's temporary location; nothing of the account's own data is touched; the left home
inventory finds it at the next admission.

Removal itself goes through the kit's own folder descriptors, never a path resolved again: the
home is opened without following a link, its device and inode are checked after opening, it is
emptied one name at a time through folder descriptors that follow nothing (links inside are
unlinked, never followed; depth is bounded), and the emptied folder is checked again before its
name is removed. Any mismatch refuses and is recorded. Some seconds later the kit looks again; a
path that has reappeared is recorded as **reappeared** (exit 7), stays a left home, and the
SWBPIPE Agent 0 must be told.

## Left homes

Every registry entry not removed, and every folder under the kit's `homes` that the registry does
not hold, is reported by name at every admission (`left-homes` prints the same). A left home is
removed only by the run that made it, by its token, after the same writer check
(`remove-left-home --record <the run's record>`); nobody removes another's, and ownership is never
judged by a folder's name.

## The strict chooser

`SWBPIPE_NATIVE_SMOKE_STRICT_CHOOSER=1` is set on every launch. Once the product change lands, a
file chooser asked for with no answer queued ends the application with exit status 86, before any
system dialog; nothing of the kit's uses 86. **Limit:** queued answers are not matched to their
operation, so a wrong chooser can consume a queued answer (writing inside the throwaway home). A
download link in the web view opens a dialog that no queued answer intercepts, so no journey
presses one.

## The helper's own programs

Every program the helper runs (`/bin/ps`, `/usr/sbin/lsof`, `/usr/sbin/ioreg`,
`/usr/bin/getconf`) is named by its absolute path and given the helper's own minimal environment,
never the caller's.

## Command line

```sh
python3 -I tools/native_launch_guard.py check-launch [--guard-dir <slot guard folder>]
python3 -I tools/native_launch_guard.py left-homes
python3 -I tools/native_launch_guard.py remove-left-home --record <the run's record.json>
```

| Exit status | Meaning |
| --- | --- |
| 0 | Clean; a home removed. |
| 1 | A product entry changed: stop. |
| 2 | Refused or unusable input. |
| 3 | No web view data in the throwaway home. |
| 4 | The home was not removed (withheld or failed). |
| 5 | The comparison failed. |
| 6 | The web view helpers' cache folders changed under the system's per-user cache folder. |
| 7 | The home's path reappeared after removal. |

A Node runner calls it with `execFileSync("/usr/bin/python3", ["-I", guardPath, ...])`; a Node
runner's own launches come under the helper in the second pull request.
