# Native launch guard

`tools/native_launch_guard.py` confines every scripted launch of the desktop application to a
throwaway home that the kit makes and removes, and then checks the consequences. Every runner
that starts the application executable uses it: Python runners import it and call its `start`,
and other runners call its command line.

Protection comes first by confinement, built in, and then by observation:

1. **Confinement.** The helper builds the whole launch: the environment from an allowlist, never
   from the caller's own; both home variables set to the throwaway home; `TMPDIR` and the working
   directory inside it; the strict chooser variable on every launch.
2. **Observation.** The real `Library` is compared before and after, the throwaway home is listed,
   and four home folders' times are read. Nothing else on the disk is watched, and the record
   says so.

## Why both home variables

The system web view does not honour `HOME` alone. With only `HOME` set, it keeps its website data
under the account's real `Library`, in folders named for the product's bundle identifier or its
executable's name. That data is local storage, caches, statistics and security databases, and a
person's own copy of the application keeps its data in the same folders. With `CFFIXED_USER_HOME`
set to the same throwaway home as well, the data goes under the throwaway home's own `Library`,
and removing the home removes it. The kit refuses to launch without both.

## The launch

- **The home.** `make-home` creates it under the system's own per-user temporary folder
  (`getconf DARWIN_USER_TEMP_DIR` on macOS) as `swbpipe-<label>-<random>`, with `tmp` and `work`
  inside it. It records the device and inode of the home, `tmp`, `work` and every folder up to the
  temporary folder in `<label>-record.json`, in the caller's evidence folder outside the home.
  **Interface change:** a caller can no longer name the home; the kit makes it, and every later
  command takes the record.
- **The environment** is built entirely by the helper:
  - `PATH=/usr/bin:/bin`, `LANG`;
  - `HOME` and `CFFIXED_USER_HOME`, both the throwaway home;
  - `TMPDIR`, the home's `tmp`;
  - `SWBPIPE_NATIVE_SMOKE_STRICT_CHOOSER=1`;
  - the kit's own named variables a caller passes (driver, plan, report, gate, gate mode, and the
    provider-key marker, whose only allowed value is `stdin`: the key itself comes on standard
    input).

  Nothing is read or copied from the caller's environment. A caller's variable is refused by
  name: the injection variables, any `DYLD_`, `LD_` or `XDG_` variable, proxy variables,
  credential-like names (key, token, secret, password, credential, auth), and any other name not
  on the list.
- **The working directory** is the home's `work` folder, whatever the caller's own.
- **Every start goes through `start`,** the preflight included. It takes neither an environment
  nor a working directory from its caller. It checks the screen and the slot, builds the
  specification, checks it again, and returns the handle. Because the preflight starts the same
  executable with the same environment, the frameworks it loads resolve their per-user locations
  to the throwaway home too.
- **Before any start** the kit refuses unless:
  - **Screen.** The system registry's console-user entry for this account is found, is on the
    console, and carries no locked key. Anything it cannot read or parse refuses. Two further
    signs are not used:
    - the screen saver, because which process runs while it is active could not be established
      by a harmless read;
    - display sleep, because the display's power value while asleep could not be established by
      a harmless read.
  - **Slot.** An ancestor of the process is the slot wrapper's lock program (`/usr/bin/lockf`),
    alive, of this account, with a slot lock file (`cargo_job.lock` or `cargo_job.slotN.lock`)
    open, read with the system's open-files lister. The lock program runs its command only while
    it holds that lock. With `--guard-dir`, the lock file must be in that folder; the record says
    whether it was given. The kit takes no lock and touches nothing of the wrapper's.
    **Limit:** a lock program started by hand with a file of that name open would pass. Hosted
    runners have no slot wrapper, so they refuse.

## The strict chooser

`SWBPIPE_NATIVE_SMOKE_STRICT_CHOOSER=1` is set on every launch. When the application is built
with the matching mode, a file chooser asked for with no answer queued ends it with exit status
86, before any system dialog; nothing else of the kit's uses 86. The test compares these two
constants with the product's own copy in `native_smoke.rs`, and reports "could not compare"
(skipped, never passed) where that copy is absent.

**Limit:** queued answers are not matched to their operation, so a wrong chooser can consume a
queued answer (writing inside the throwaway home). A download link in the web view opens a dialog
that no queued answer intercepts, so no journey presses one.

## Around the run

- **The comparison.** The real `Library` is listed by names, sizes and times only, under
  `WebKit`, `Caches`, `HTTPStorages`, `Application Support`, `Preferences` and
  `Saved Application State`. No file there is opened, no link is followed, and nothing is
  written, removed or changed. Wait about 20 seconds after the application exits before
  comparing, because the web view's helpers may still write.

  | Class | What it covers | Effect |
  | --- | --- | --- |
  | product | A created, modified or removed entry, at any depth, whose path has a component equal to a watched name (the bundle identifier, the executable's name) or beginning with one followed by a dot; or a product-named folder that could not be listed after the run. | The run stops (exit 1). |
  | others | Any other entry created or modified in the window. The operating system's own services and other software write such records whenever an application opens a window. | Never fails the run. |
  | nothing | Nothing changed. | — |

  The others are counted per folder and named, with size and time, only directly under
  `Preferences` and `Preferences/ByHost`. They are never opened, edited or removed.
- **Failing closed.** If one of the six folders cannot be listed, or a folder becomes unreadable
  during the run, the comparison fails (exit 5). A folder unreadable both before and after is
  counted in the report as a stated limit: a product-named entry inside it would not be seen.
- **Four folder times.** The modification times of the home folder, `Desktop`, `Documents` and
  `Downloads` are read before and after by a single `lstat` each. Nothing inside them is listed or
  opened. A change is reported with the run and not concluded from.
- **Positive half.** The throwaway home's own `Library` must hold web view data under `WebKit` or
  `Caches`, or the run fails with its own status (exit 3). That tests that the redirection
  worked; the listing tests that nothing leaked.
- **Window.** The report names the window it compared. Two launches whose windows overlap cannot
  be told apart, so launches through this kit are serialised.
- **What was watched.** The report states, in one sentence, that nothing outside the real
  `Library`'s six folders and the throwaway home was watched. A clean comparison establishes
  neither that nothing was read nor that nothing was written elsewhere on the disk.

## End of every run

This applies to a pass, a failure, a timeout, an exception and an interrupt alike. `remove-home`
writes the home's listing (names, sizes, times; bounded) to `<label>-listing.json` in the evidence
folder, outside the home, then removes the home and nothing else.

- The record is checked twice: once before anything else and again immediately before removal.
  The home, `tmp`, `work` and every folder up to the temporary folder must have the same device
  and inode, be real folders, and the temporary folder must still be the system's answer. A link,
  a replaced folder or a home the kit did not make is refused.
- Links inside the home are unlinked, not followed, and the path is verified gone afterwards.
- A failed listing is recorded and the home is still removed.
- Keep what a failed run needs (reports, journals) outside the home.

`cleanup` is one of `removed`, `withheld`, `failed`, `refused` or `not created`.

### When a home is withheld

A home is not removed while anything that could still write to it may be alive. In that case it
is listed, left in place and named, and the run fails (exit 4). The check runs inside
`remove-home`, and no caller can skip it. If the check cannot run, the home is withheld too.

The check combines four tests, because no single one covers the web view's helpers, which the
system starts on the application's behalf outside its process group and without its environment.
A process of this account counts as a possible writer if any of these holds:

- its executable lies under the home or a private copy the caller names (the application, its
  engine worker, the supplier);
- it descends, by parent process id at the moment of the check, from a process the run started;
- it is in a process group the run started;
- it has a file or its working directory open under the home, from the system's open-files
  lister, read-only. This test is what finds the web view's helpers.

On Linux a fifth test also looks for either home variable set to the home in a process's
environment. On macOS another process's environment cannot be read without privilege.

**What the check cannot see:** a process that has outlived the application, been re-parented away
from it, runs an executable outside the private copy, and holds nothing open under the home at
that moment. The 20-second wait before the comparison stays for that reason.

The kit stops only processes it started itself, by their handles. Removing a home left by an
earlier run belongs to the runner whose run left it; until commands for that arrive, a withheld
home is named in the run's record.

## Command line

```sh
python3 -I tools/native_launch_guard.py check-launch [--guard-dir <slot guard folder>]
python3 -I tools/native_launch_guard.py make-home --evidence <evidence folder> [--label home]
python3 -I tools/native_launch_guard.py spec --record <record.json> [--set NAME=VALUE]...
python3 -I tools/native_launch_guard.py snapshot --name <bundle id> --name <executable name> --out <before.json>
python3 -I tools/native_launch_guard.py compare --before <before.json> --record <record.json> [--out <report.json>]
python3 -I tools/native_launch_guard.py remove-home --record <record.json> --evidence <evidence folder> \
    [--started PID]... [--group PGID]... [--private PATH]... [--processes-not-gone]
```

`spec` prints the environment and working directory a non-Python runner must use exactly; it
checks the screen and the slot only through `check-launch`, which such a runner calls first.

| Exit status | Meaning |
| --- | --- |
| 0 | Clean: nothing changed, or others only; a home made, specified or removed. |
| 1 | A product entry changed: stop. |
| 2 | Refused or unusable input. |
| 3 | No web view data in the throwaway home. |
| 4 | The home was not removed (withheld or failed); the output names the path. |
| 5 | The comparison failed: a folder could not be listed. |

The application itself ends with 86 when a chooser is asked with no answer queued.

A Node runner calls it with `execFileSync("/usr/bin/python3", ["-I", guardPath, ...])`. Isolated
mode means no `PYTHONPATH`, no user site and no script directory on the path; the tool needs only
the standard library. If `python3` is absent or the call fails, the runner does not launch.
