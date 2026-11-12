# Native launch guard

`tools/native_launch_guard.py` keeps scripted launches of the desktop application inside a
throwaway home, records custody of what each launch starts, and checks the consequences. One
object, `Run`, owns a launch from admission to its finish, and it is the only way the kit starts
the application. Besides it, the module starts only four fixed utilities in fixed invocation
shapes.

The agent journey controller (`tools/agent_journey_fixture.py`) and the agent live-session fixture
(`tools/agent_live_session_fixture.py`, every case) use it. The native-smoke runner (`run.mjs`) and
the other Node runners do not yet.

**The helper removes nothing.** It admits, confines, records custody, compares, lists the home and
marks the run finished in the kit's inventory. Every home a run leaves stays in place, named in the
inventory with its state, until its lane removes it with the separate removal command. That
command is not in this pull request: it comes with its own reading, and no launch is asked for
until it has had a clean one. "Run finished" never means "safe to remove"; that judgment belongs to
the removal command, made when it runs.

## The adversary model

**Inside the model**, and so repaired or refused:
- a caller's mistake: a wrong or missing argument, a forgotten step, or a second runner that does
  not know of the first;
- a concurrent or earlier run of this kit, or of another lane's runner;
- interruption at any point, by signal, kill or crash;
- the launched application and everything it starts, doing anything their own code can do, while
  running or after the kit believes they ended: writing anywhere they can reach, leaving unreadable
  folders, surviving their parent, renaming a path, deliberately or not;
- any file system state left by any of those.

**Outside the model**, and so recorded and not repaired: an unrelated, deliberately adversarial
process of the same account that forges or races the kit's own folders, registry or records. Any
state such a process could forge that an ordinary accident could also produce (a stale path, a
renamed folder, a second runner) is inside the model.

The claim is bounded. It is not absolute protection against every process of the account.

**What the helper adds to such an outside process's reach.** It removes nothing. Such a process
could make admission refuse, or make the inventory misreport the kit's own homes. It cannot make
the helper remove, open or write anything outside the kit's own folder and the run's evidence
folder. The removal command states its own limit when it comes.

## What the application is confined to

It is confined as to where it reads and writes its own files, and as to what environment it is
handed; nothing more. With no operating system boundary around the application itself, sockets
elsewhere in the account stay reachable to it, and home redirection cannot change that.

Every run's record carries both sentences. It also carries this one: a finished home may still be
written to by anything of the launch that is alive.

The helper launches on macOS only; on any other system it refuses to launch.

## Why both home variables

The system web view does not honour `HOME` alone. With only `HOME` set, it keeps its website data
under the account's real `Library`, in folders named for the product's bundle identifier or its
executable's name. With `CFFIXED_USER_HOME` set to the same throwaway home as well, the data goes
under the throwaway home's own `Library`. The kit refuses to launch without both.

## A run

```python
run = native_launch_guard.Run(executable, evidence_folder, "home", extra_names=[...])
run.begin()   # admission, baseline, home
process = run.start("app", [str(executable)], {DRIVER: "-", PLAN: plan_json, REPORT: RUN_OWNED},
                    web_view=True, frame=run.driver_frame("apps/desktop/native-smoke/agent-journey.js"),
                    key_line=None, close_stdin=True, stdin=subprocess.PIPE, stderr=subprocess.PIPE)
outcome = run.finish(retain=False)  # comparison persisted, home listed, run marked; nothing removed
```

### Before anything is created or read

Every location a caller supplies is checked first:
- **The application and the probes** must be absolute, canonical (no link on the path), regular
  files, and not under the account's `Desktop`, `Documents`, `Downloads` or `Library`.
- **The output and evidence folders** must be absolute, canonical, not under those folders and not
  under this repository.

The tools make their output folder, and the run opens its evidence folder, by walking from the
root through folder descriptors, each component opened without following a link. Every file the
run writes there goes through that descriptor, is created new and never through a link.

### The product's identity

The executable must be canonical and absolute, inside a bundle whose `Info.plist` names it. The
names the comparison watches are derived from the bundle: the bundle identifier, the executable's
name, and its underscore form by matching. If they cannot be established, the run refuses. A
caller may add names; it can never replace or remove them.

The executable is opened and held from `begin`, and its device, inode, size and time are recorded.
Before every start it must still be that file. The strict chooser lead is read through that held
descriptor.

### Admission, at `begin` and again before every start

- **System:** macOS.
- **Screen:** this account's console-user entries are found, agree with one another, are on the
  console, and carry no locked key; a locked key present with any value refuses. The screen saver
  and display sleep are not used, because neither could be established by a harmless read.
- **Slot:** an ancestor is the slot wrapper's lock program (`/usr/bin/lockf`), alive, of this
  account, with a slot lock file open. With `guard_dir`, the file must be in that folder. A lock
  program started by hand with such a file open would pass. Hosted runners refuse.
- **Strict chooser lead:** the executable's bytes contain the strict chooser variable's name. That
  is a lead and not a proof. The protection itself is the product's, in pull request 283; nothing
  of it is claimed until that is on main, and the comparison test reports "could not compare"
  until then.
- **The inventory** of this kit's homes (below). Every home is reported, by name and state, with a
  count. A launch is refused while:
  - any home, of whatever state, is found held: a recorded generation alive or with an unread start
    time, a member of a recorded group, an executable under the home or the private copy, a holder
    of a file there;
  - or the check cannot establish that none is: a listing that fails, a folder inside the home that
    cannot be listed, a path that no longer identifies the folder the kit made;
  - or a folder under the kit's homes is unknown custody (one the registry does not hold) and is
    held or cannot be established free;
  - or a registry entry cannot be read.

  A finished home with no holder found does not refuse by itself.

What admission established is kept in the run's record.

### The baseline

The listing below is taken before the home is made. An incomplete baseline refuses: an unreadable
or linked watched top folder, an unreadable product-named folder, a folder changed during the
walk, or a walk too deep.

### The home

The kit keeps its own folder, `swbpipe-launch-kit`, under the system's per-user temporary folder,
private to this account, with a `registry` and the `homes`. All are held open by descriptor.

The run writes its own record into the evidence folder first, including a token only it holds. It
then enters the home in the registry, and only then makes the home, with `tmp` and `work`,
recording their device and inode. A failure after the folder is made and before the entry is
complete leaves a `creating` entry in the inventory.

### `start`

- **Environment,** built entirely by the helper:
  - `PATH=/usr/bin:/bin`, fixed;
  - `LANG`;
  - `HOME` and `CFFIXED_USER_HOME`, both the throwaway home;
  - `TMPDIR`, the home's `tmp` (the bundled database library also reads it for its temporary files);
  - `SWBPIPE_NATIVE_SMOKE_STRICT_CHOOSER=1`;
  - the kit's own variables, each with a value the kit owns: the driver only `-`, the provider key
    only `stdin`, the report and the gate only `run-owned` (replaced by fixed files in the home's
    `work`), the gate mode only `handshake`, the plan only as JSON in its closed shape.

  Refused by name: injection, loader, `XDG_`, proxy and credential-like variables, and anything
  else not on the list.
- **Working directory:** the home's `work`.
- **Process group:** a new one, led by the application.
- **Streams and descriptors:** only `stdin`, `stdout` and `stderr`, each a pipe or nothing. Every
  other descriptor is closed. Passed descriptors, a pre-start callback, a new session, another
  environment or working directory are refused.
- **Standard input:** carries only frames the helper built.
  - The fixture's first frame is built from validated parts. Its state folder is made inside the
    home through the held descriptors. Its probes are checked regular files, not links and not
    under the protected folders.
  - The native-smoke driver is a file inside this repository, read through a verified descriptor
    walk.
  - The record keeps the hash of the bytes actually written to the pipe; a provider key line
    written before a frame is never recorded.
  - After the first frame, the live-session fixture sends only fixed path-free commands.
  - A path the product itself is handed through a chooser is the product's to refuse under the
    strict mode (pull request 283); nothing of that is claimed today.

### Custody across the death of a parent

What a launch starts is recorded as it goes, by identifier and start time together (a generation),
and persisted in the registry.

- **The mechanism.** The application is started as the leader of a new process group, and the
  group is recorded. A group's members can be listed by its identifier after its leader has ended
  and its members have been reparented, for as long as any member is alive.
- **Sampling.** While the run is open, the kit samples the process table every second. Every living
  process descended from a recorded generation, or in a recorded group, is recorded as a generation
  of its own; a descendant that made a group of its own has that group recorded too.
- **A missed sample** (a listing that fails, or an interval much longer than it should be) is
  recorded as missed, with its time.

**What escapes, and what the kit does about each:**

| Process | What happens | What the kit does |
| --- | --- | --- |
| A descendant that starts its own session or group | The product does this for its supplier (`embedded_agent/macos.rs`) and its solve worker (`solve_worker.rs`); both make a process group of their own. | Each is seen in a sample, through its parent, and its group recorded. The supplier's group is also attributed from the application's handshake. The solve worker runs the application's own executable, which lies under the private copy, so it is found by its executable too. |
| A descendant born and gone into a group of its own between two samples | Its parent may end before a sample sees it. | Not seen by the group or the samples. It is found only if its executable lies under the home or the private copy, or it holds a file or working directory under the home. |
| Processes the system starts on the application's behalf, the web view's helpers among them | They are not descendants of the application and are not in its group. | Found only while they hold a file or working directory under the home. A helper that holds the home's path only in memory cannot be seen. |

### `finish`

The comparison is made and persisted first. Then the home is listed into the evidence folder. Then
the run is marked in the inventory: `finished`, or `left` when the caller could not establish that
its processes ended, or the one holder check finds a holder alive or cannot exclude one. Last
comes the run's record. Nothing is removed.

Whatever raises on the way, the later steps still run. The terminate, interrupt and hang-up signals
are handled while a run is open; a signal arriving meanwhile is re-raised at the end. A run begun
and never finished is finished when the interpreter exits. A run killed outright stays `open` in
the inventory, and the holder check at the next admission decides whether it blocks.

**An unreadable folder inside the home** is a finding about the application. It is listed and
reported by count and by place within the home, in the run's record and in the inventory entry.
Because a holder inside it cannot be excluded, the home is marked `left`.

## The listing

Names, sizes and times only, by `lstat` through folder descriptors. Each folder entered is opened
without following a link and checked to be the folder that was seen.

- **Walked in full:** the `Library`'s `WebKit`, `Caches`, `HTTPStorages`, `Application Support`,
  `Preferences`, `Saved Application State` and `Logs`, and the system's per-user cache folder,
  which records name by what it is, never by its path.
- **One level deep, by name:** every other top folder of the `Library`, for a product-named child.
  Such a child folder is checked to be readable, without walking it.
- **Four folder times:** the home folder, `Desktop`, `Documents` and `Downloads`, by a single `lstat`
  each, never listed. A change is reported, not concluded from.
- **Matching:** a path component equal to a watched name, or beginning with one followed by a dot;
  after a plus sign, also one followed by a hyphen.

| Class | What it covers | Effect |
| --- | --- | --- |
| product | A created, modified or removed product-named entry; an unreadable product-named folder. | The run stops (exit 1). |
| web view helper cache | Changes inside `com.apple.WebKit.{Networking,WebContent,GPU}+<name>…` directly under the system's per-user cache folder. Whether both home variables redirect these is not established. Where they do not, it is a write outside the throwaway home that confinement does not reach: observed, not prevented. | The run stops, always (exit 6). |
| others | Any other entry created or modified in the window. | Counted by class and by top folder; never named, opened, edited or removed; never fails the run. |
| nothing | Nothing changed. | — |

**Fail closed.** The comparison fails (exit 5) for:
- an unreadable or linked watched top folder;
- a cache folder the system does not give;
- a folder changed during the walk;
- a walk too deep;
- a different count of unreadable non-product folders after the run than at the baseline.

The record says "N folders could not be listed and were not watched".

**Positive half.** After a run that opened a window, a file must be found under the throwaway
home's `Library/WebKit` or `Library/Caches`, or the run fails (exit 3). Any file there satisfies the
check; it does not identify the web view's own data.

**Window.** Two launches whose windows overlap cannot be told apart. The journey controller takes a
host-wide lock; other runners are not serialised by this helper.

A clean comparison establishes neither that nothing was read nor that nothing was written
elsewhere on the disk.

## Homes, and what a lane does with them

```sh
python3 -I tools/native_launch_guard.py homes
```

This prints every home of this kit on disk, by name and state, with a count:
- `finished` (not removed);
- `left` (a holder may be alive);
- `open` (a run killed outright);
- `creating`;
- `unknown custody`.

The homes hold test data only.

Each lane reports that count in its state. It removes its own homes with the removal command once
that command exists and has been read, and before the undertaking ends; none outlives it. Until
then, nobody removes a home by hand except the owning lane, after verifying by its own process
identifiers that nothing of the launch is alive.

## The strict chooser

The helper sets `SWBPIPE_NATIVE_SMOKE_STRICT_CHOOSER=1` on every launch. Once the product change
lands, a file chooser asked for with no answer queued ends the application with exit status 86,
before any system dialog. Nothing of the kit's uses 86.

**Limit:** queued answers are not matched to their operation. A download link in the web view opens
a dialog that no queued answer intercepts, so no journey presses one.

## The helper's own programs

`_read` starts only `/bin/ps`, `/usr/sbin/lsof`, `/usr/sbin/ioreg` and `/usr/bin/getconf`, each in
its fixed invocation shapes, with a minimal environment of its own. Anything else is refused.

The open-files lister's diagnostics are kept. Anything on its standard error, or a status other
than 0 or 1, is a failed listing. It also exits 1 when it found holders but not the named folder
itself open.

## Command line

```sh
python3 -I tools/native_launch_guard.py check-launch [--guard-dir <slot guard folder>]
python3 -I tools/native_launch_guard.py homes
```

| Exit status | Meaning |
| --- | --- |
| 0 | Clean; the inventory printed. |
| 1 | A product entry changed: stop. |
| 2 | Refused or unusable input. |
| 3 | No web view data in the throwaway home. |
| 5 | The comparison failed. |
| 6 | The web view helpers' cache folders changed under the system's per-user cache folder. |

A Node runner calls it with `execFileSync("/usr/bin/python3", ["-I", guardPath, ...])`. A Node
runner's own launches come under the helper in a later pull request.
