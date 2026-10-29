# Windows worker fixture — incomplete (RT-V7, DEL-04-07)

This is a standalone investigation fixture. It does not implement the desktop
worker, a solver, Tauri, wasm, numerical references or retained registration.
The Windows registration remains Stale. The native process and custody cases
below are not yet implemented; the supplemental CI lane fails closed for them.
A green protocol model or accounting test cannot establish worker safety.

## Present scope

The deterministic supervisor model represents one fixture mutator. A stop
request sets one absolute two-second deadline; it does not wait for an ack.
Only a current-generation cooperative acknowledgement permits admission.
Each scripted attempt gets a new sample, tries preferred then minimal growth,
and cancellation dominates late acknowledgements and resumes. The model owns
its entire explicit fixture transition allowance before requesting a stop and
keeps it while parked. This is a model of the protocol only: no real pool lock,
release wakeup, allocator, process, page-touch loop or channel lock is tested.
The 50 ms sample interval is represented by injected time; no timing guarantee
is inferred from the host model.

On native Windows, a test samples `PROCESS_MEMORY_COUNTERS_EX2` and prints
private commit, private working set, total working set and shared commit at
untouched committed mapping, touched mapping, release, touched heap and freed
heap. Allocator retention and OS accounting can vary; the test does not demand
exact deltas, or treat commit as resident credit. The mapping and heap are each
4 MiB, sequentially held, and this fixture creates no worker. This accounting
exercise does not prove a production transition growth bound.

## Required before fixture completion

- A bounded real process protocol must cover allocator-blocked GROW before
  System allocation, checkpointed computation, and an intentionally
  uncheckpointed page-touch loop that cannot be mistaken for quiescence.
  Exercise the 50 ms/release wakeup, fresh samples under one pool lock,
  generation and stale acks, cancellation/reset in each phase, a pending ack
  while the supervisor continues to the original deadline, and allocator and
  channel-lock races without deadlock. Precreate handles, state and fixed
  buffers for any path described as allocation-free. A request event is not
  quiescence; no assumption covers production threads.
- Reserve a separate small safety cap before acknowledgement and while parked;
  cap aggregate intentional allocations at 64 MiB across at most two workers.
  Explain ownership and cleanup on every failure. Neither the cap nor the
  fixture proves bounds on production numerical, parse, native or serialization
  transitions or Mac allocator portability.
- Create workers with `PROC_THREAD_ATTRIBUTE_JOB_LIST` at creation, no
  breakaway and an explicit inherited-handle allowlist. Check observed process
  exit before logical lease release, including parent death while parked and
  cancellation/reset. A normal death observer may retain a process handle, not
  a Job handle. An adversarial retained/inherited Job handle requires a separate
  bounded external cleanup process and a survivor check. A kill request or EOF
  is not exit evidence. Job kill-on-close is triggered by the *last* Job handle.
- Native x64 CI must run all cases against the actual checked-out candidate,
  detect empty or missing selection, and propagate child/test failures. The
  current lane runs the model and accounting tests, then explicitly fails for
  the missing cases. It has read-only repository access, no credential
  persistence, and no build or result cache. Hosted Rust acquisition is confined
  to the job; no guest transfer or private inputs are involved.

The fixture does not qualify retained precision, cross-app transport, desktop
integration or release. DEL-07-11's desktop shell contract, DEL-10-04's release
platform matrix and DEL-04-08's retained pricing and registration declarations
therefore remain unchanged. Independent review and actual Windows execution are
still required; this document makes no acceptance or release decision.

## Review threat model

A reviewer must challenge a wrong candidate or platform giving a false green;
event delivery, stale ack or unchecked work mistaken for quiescence; restarted
deadline or blocked supervisor; allocator/channel deadlock; conflated commit
and working set or unreserved transition; lease release on kill request or EOF;
process escape or inherited Job lifetime; hangs or leaks; swallowed child/test
failure or empty selection; changed numerical references or production policy;
and secret, write or provisioning overreach. Independent adversarial review
belongs to the parent after a concrete implementation. New findings beyond this
model are recorded; a new kind within it goes to Agent 0.

## API basis

- [PROCESS_MEMORY_COUNTERS_EX2](https://learn.microsoft.com/en-us/windows/win32/api/psapi/ns-psapi-process_memory_counters_ex2), [GetProcessMemoryInfo](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getprocessmemoryinfo), [VirtualAlloc](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc), and [VirtualFree](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualfree).
- [Process creation attributes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute), [Job objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects), [TerminateProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess) and [WaitForSingleObject](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject).
- [SuspendThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-suspendthread) is primarily for debuggers, not synchronization. [Anonymous pipe operations](https://learn.microsoft.com/en-us/windows/win32/ipc/anonymous-pipe-operations) do not support overlapped operations. [CancelIoEx](https://learn.microsoft.com/en-us/windows/win32/api/ioapiset/nf-ioapiset-cancelioex) does not wait for completion; overlapped state and buffers must survive completion. Job hard limits fail commits; notification limits differ ([JOBOBJECT_EXTENDED_LIMIT_INFORMATION](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_extended_limit_information), [JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_notification_limit_information)).
