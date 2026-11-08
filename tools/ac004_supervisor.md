---
name: swbpipe-ac004-supervisor
description: Use the bounded AC-004 supervisor within the actual host grant.
metadata:
  kind: instruction
  surface: tool
---
# AC-004 deadline supervisor

`ac004_supervisor.py` runs one AC-004 payload on the T3 host's exclusive lane under a
deadline that covers payload teardown and lock release. It is DEL-04-08 qualification
tooling. It changes no registered profile, pin, budget, solver, memory registration,
admission rule or acceptance.

Having the tool does not authorize a run. Each actual AC-004 run on the shared host still
needs its own clearance. This tool does not change who grants it.

## Use

```sh
python3 tools/ac004_supervisor.py --guard-dir /Users/ryan/dev/chirality-t3/guard \
  [--budget 60] [--reserve 3] [--term-grace 1] [--trace-file PATH] -- PAYLOAD...
```

| Option | Meaning |
|---|---|
| `--budget` | Seconds from arming to the release of the last lock. Must be finite, greater than 0 and no more than the 60 s ceiling. Default 60. |
| `--reserve` | Seconds kept back for teardown. The payload is stopped at `budget - reserve`. Default 3. |
| `--term-grace` | Seconds between SIGTERM and SIGKILL. It falls inside the reserve. Default 1. |
| `--trace-file` | A new file (`O_EXCL`) that receives the JSON trace as well as stderr. |

The inputs must satisfy `term-grace < reserve < budget <= 60`. Invalid, non-finite,
repeated or unknown inputs are refused before any lock is taken.

The 60 s overall ceiling and the selected inner harness's own 55 s target are separate. Run
the payload with its internal 55 s target. The supervisor then enforces the outer 60 s, with
teardown and release included.

## Protocol

The supervisor follows `t3_exclusive.sh` and `t3_exclusive_stage2.sh` (hashes `c6891d51…`
and `09a2ba88…`, read for protocol only) and interoperates with them and with `t3_slot.sh`:

1. Preflight, all before queueing. The guard directory and its four lock files must exist as
   regular files, not symlinks. No `exclusive.pending` may exist. The payload must resolve
   to an executable. The kernel process-group query must work. `pgrep -f
   <guard>/memguard.sh` must find the memory guard (exit 97 if it does not, as the wrappers
   do).
2. Take `cargo_job.lock` and wait for it without a time limit.
3. While holding slot 1, create `exclusive.pending` with `O_EXCL`. It contains the
   supervisor's own live PID. If a marker already exists, refuse; a foreign marker is never
   opened, overwritten or removed. A short PID write fails the run. Cleanup identifies the
   created inode, so it also removes an owned partial write without reading marker contents.
4. Take `cargo_job.slot2.lock` and `cargo_job.slot3.lock`, waiting without a time limit.
5. Poll `cargo_job.slot4.lock` without blocking. The time of the attempt that succeeds is
   taken before that attempt, so the deadline is **armed no later than the final lock's
   acquisition**. The wait for existing slot holders to drain is not timed.
6. Check that each held descriptor is still the inode at its path; a replaced lock file is
   refused. Remove our own pending marker. Fork the sentry and start the payload only if
   the payload deadline has not passed; both controller and sentry check before launch.
7. Stop the payload at `armed + budget - reserve` if it has not ended. Send SIGTERM and
   SIGCONT to the owned group, then send SIGKILL repeatedly until the kernel reports no
   live member of the group. TERM grace is capped or skipped on a delayed wake-up to leave
   `reserve - term-grace` seconds for KILL, verification, reap and release. Reap the sentry.
8. Remove our own pending marker if it is still there, close every lock descriptor and
   record the release time. Lock files are never unlinked; this matches `lockf -k`.

Classification, the JSON trace and the trace file come after release. They are outside
the bound.

### Lock flavour

The supervisor takes locks with `flock(2)`. On macOS `/usr/bin/lockf` (BSD) excludes and
is excluded by `flock(2)` locks on the same file. The tests exercise both directions with
the real utility: a `lockf` holder delays the supervisor on each of the four slots, and
`lockf -s -k -t 0` reports 75 on every slot while the payload runs and 0 afterwards. On
Linux the tests use `flock(1)` or a Python `flock` holder. The supervisor does not rely on
`fcntl.lockf`/`flock` equivalence; it uses the flavour that BSD `lockf(1)` uses.

### Ownership and containment

- **The sentry.** A forked child of the controller is the process-group leader (pid =
  pgid, set by both parent and child before any signal or inspection). It starts the
  payload in that group with stdin from `/dev/null`, reports the spawn and the wait status
  over a pipe, and otherwise waits. The controller never reaps the sentry until the group
  has no live member. So the pgid cannot be reused while it is still being signalled, and
  no group-reuse race arises.
- **Membership** comes from the kernel: `sysctl kern.proc.pgrp` on macOS, `/proc/*/stat` on
  Linux. The supervisor does no `ps` or `pgrep` name matching. It signals only its own
  group and never signals any other process. Linux ignores only a process that disappears
  during inspection (`ENOENT`/`ESRCH`); other read errors and malformed records fail
  inspection and prevent verified containment or success.
- **Lock descriptors** are opened `O_CLOEXEC`. The payload is started with `close_fds`, so
  neither the payload nor its descendants inherit a lock descriptor. The tests check this
  in a child and in a grandchild. The sentry holds copies through `fork`, which is
  deliberate (see the next item).
- **Controller death.** The sentry holds the read end of a lifeline pipe. If the controller
  dies abruptly (SIGKILL, crash, memguard), the lifeline reaches EOF. The sentry then sends
  one `killpg(SIGKILL)` to its own group, itself included. The sentry's inherited lock
  descriptors keep the slots held until that signal is delivered, so the locks are not
  released while the payload still runs unsupervised.
- **Signals.** SIGINT, SIGTERM and SIGHUP interrupt only the untimed queue waits. Inside
  the bounded region they are recorded and lead to the same stop sequence, with exit
  `128 + signal`. The payload runs in a background process group, so a terminal Ctrl-C
  reaches only the controller.

## Exit codes and outcomes

| Exit | Outcome | Meaning |
|---|---|---|
| 0 | `success` | Payload exited 0 within the budget. The group was verified empty with no stray, and release happened within the budget. |
| payload's | `payload_failed` | Payload exited nonzero, or `128 + n` if signal `n` killed it. |
| 123 | `strays_contained` | Payload exited 0 but left live descendants. They were killed, and the run is not a success. |
| 124 | `timeout` / `deadline_exceeded` | Payload stopped at the deadline, or release came after the budget. |
| 125 | `containment_unverified`, `internal_error`, `payload_status_unknown`, `cleanup_failed` | No success can be claimed. |
| 126 | `spawn_failed` | The payload could not be executed. |
| 128+n | `cancelled` | Signal `n` received. |
| 64 / 75 / 78 / 97 | `refused` | Bad usage or input / existing pending marker / bad guard configuration or replaced lock / memory guard absent. |

An unsuccessful or incomplete run never exits 0. The trace's `containment.verified`,
`release_within_budget` and `payload` fields give the reason.

## Trace

The trace is a single JSON line on stderr, also written to `--trace-file` if given. It
gives the queue wait, the queue-relative and armed-relative monotonic marks (each lock,
pending published and removed, `armed`, sentry fork, payload spawn and end, TERM, KILL,
contained, reaped, released), `armed_to_release_s`, `release_within_budget`, the payload
disposition, `strays_at_exit`, the stop reason and the containment record. The payload argv
appears only as a count and a SHA-256, and no environment is included. It is the
supervisor's own trace, not a lock log. The supervisor writes nothing to
`cargo_jobs.log`.

## What is bounded, and the limits

The bounded region runs from `armed` to the close of the last lock descriptor. It contains
only local operations: `fstat`/`stat`/`lstat`, an owned pending-file unlink,
`fork`, pipe reads with timeouts, `kill(2)`, kernel membership queries, `waitpid` of a
SIGKILLed child, and `close`. It does no blocking log or receipt write. The payload's
stdout and stderr are inherited, so a blocked output pipe stalls only the payload, and the
deadline then stops it.

The supervisor is not hard real-time and does not contain arbitrary hostile process trees.
Specifically:

- **Leaving the group.** A payload component that calls `setsid`/`setpgid` (for example
  Python's `start_new_session=True`) leaves the owned group, and is neither seen nor
  signalled. The payload, including any inner harness, must keep every process in its
  inherited group. Fresh inspection before actual use must confirm this.
- **Unkillable members.** If a member cannot be killed, because it is in an
  uninterruptible kernel wait or is a setuid process that refuses our signal, the
  supervisor keeps the locks and keeps signalling past the budget. It reports
  `overrun: true` and `deadline_exceeded` rather than release slots with a live member.
  If the controller is then also killed, the locks drop with that member still alive.
- **Double failure.** If something outside kills the sentry while the payload runs,
  controller-death protection is lost. While the controller lives it still contains the
  group. If the controller is also killed abruptly, the payload is unsupervised.
- **Death before the sentry exists.** If the controller dies abruptly while queued, the
  kernel releases its locks, but the `exclusive.pending` marker remains with a dead PID.
  `t3_slot.sh` ignores a marker whose PID is not alive. The next supervisor run refuses,
  with exit 75, until the owner removes the stale marker.
- **Timing.** Machine sleep, extreme CPU starvation, and a root or hostile same-user
  process (which could join or signal the group, or replace files) are outside the model.
- **Memory guard.** The guard is checked once at start, as in the wrappers. It is not
  re-checked when the locks are acquired.
- **Platforms.** Only macOS and Linux are supported. On macOS the `kinfo_proc` layout is
  validated at preflight by finding the supervisor's own process in its own group.

## Tests

```sh
python3 tools/ac004_supervisor_test.py
```

The tests use standard-library `unittest`, with no pytest and no Cargo. Each test makes a
private temporary guard directory, lock files and fake memory guard, and uses only
self-limiting fake payloads. Each test also has its own bounded cleanup, which kills only
processes the test started. It registers sentry, payload and fake-descendant kernel birth,
session and group identities independently of the supervisor's completion trace. Every
fake Python fork registers its child before either branch continues, so cleanup retains
ownership even when the sentry and main have both died. Cleanup verifies the recorded
identities before signalling a private group. Disabled controller-death kill regressions
exercise both a live main and a lone orphan, including inherited output pipes, with a
separate exact-identity fallback for the negative control itself. Delayed-controller, short-write,
foreign-FIFO and process-inspection-error controls cover the review repairs. No test reads
or touches the host's real guard.
