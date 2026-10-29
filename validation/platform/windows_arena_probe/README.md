# Windows arena and admission investigation

This dependency-free native-x64 probe exercises a controlled arena API, not a
global allocator, solver, Tauri/agent runtime connection or production worker
bound. Each arena reserves 4 MiB of virtual addresses; this is not a full-budget
lease. At most two native child workers exist, deliberate buffers stay below
64 MiB aggregate, and no case pressures the runner. Page/allocation granularity
comes from GetSystemInfo. Loader/libtest/OS process overhead is not qualified.

Fixed 256-entry slot metadata is committed inside the arena; fixed control
arrays have a separate page-rounded allowance. Data pages are admitted before
commit, counted once, and remain charged while retained free. A failed commit
keeps its page authority for retry. This probe conservatively retains authority
even after successful decommit until observed exit; it does not claim a
production reclamation/availability profile. Moving realloc asserts old+new
live overlap and preserves old data on failure. Fault hooks distinguish
synthetic headroom, synthetic other failures and real OS error codes.

The child protocol gives each separate growth operation its own absolute 2 s
window. Its headroom requests and a classified injected post-grant commit retry
share that same window; grants/replies/retries never restart it. Separate
physical/commit refusals, stale replies, cancellation/reset, nonheadroom errors,
persistent shortage, late commit failure and unrelated-operation deadlines have
named cases. Held authority, actual committed arena bytes and future liability
are separate. All realized credits start at zero, deliberately conservative.
Samples are scripted, not native host reservations or an OS error classifier.

Native children use creation-time Job assignment and restricted inheritance.
Cancellation/kill requests do not release authority; real process exit does.
A separate Rust case supervisor holds a creation-time outer Job around each
case and its arena descendants. It applies a 30 s case deadline, independently
drains that Job to zero active processes, and observes the case process exit
before emitting a required cleanup-confirmed marker. A negative control starts
a blocked arena worker then deliberately hangs its case; external Job cleanup
must end both. PowerShell's 45 s last backstop reports cleanup unconfirmed if
the Rust supervisor itself fails; it never invents a confirmed Job drain.
Internal child waits and Job drain also have short deadlines. The runner
streams diagnostics, verifies exact candidate/OS architecture, enumerates 12
required witnesses plus two internal entries, runs each witness exactly
once and checks missing-selection, child-failure and external-timeout negative controls. No ignored
or empty case can count as a pass. The workflow is supplemental, outside the
required harness, and uses a fresh target with Rust 1.97.1 MSVC on windows-2025.

The arena observation reports requested/live capacity, committed data, metadata,
retained empty pages, syscall count and reserved-span unused bytes. The last
includes uncommitted alignment gaps and must not be read as committed waste;
committed slack is reported separately. Small-block slab packing is compared
with per-allocation pages on a scripted pattern. Hosted timings are diagnostic,
not qualified performance. Real OS exhaustion and every cancellation race are
not established by synthetic faults or passing fixtures.

Hosted command (PowerShell7; EXPECTED_HEAD and fresh CARGO_TARGET_DIR required):

```powershell
./run.ps1
```

All compilation/execution is hosted. No local Cargo, guest access, acquisitions
or provisioning belongs to this task. Existing CAEPIPE guest is permanently
off-limits. R64 MiB remains unqualified for Windows; retained registration stays
Stale. See DEL-04-07 Design/windows-arena-probe.md for startup/bypass audit,
pending solver guard basis, threat model and remaining finite proof.
