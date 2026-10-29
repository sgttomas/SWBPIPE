# Bounded Windows worker fixture (RT-V7, DEL-04-07)

This is standalone verification tooling. It changes no desktop worker, numerical
method/reference, Tauri/wasm route, engineering data format, memory/wait policy,
retained coefficient or registration. Windows retained registration remains
Stale. Fixture execution cannot establish the subsequent source-path checkpoint
and transition-growth proof for a production solve or model check.

## Ownership, caps and custody

The controller holds a 2 MiB payload/control allowance before creation and while
acknowledgment is pending or the mutator is parked, releasing only after observed
process exit. The allowance covers a single 512 KiB mutable buffer, one shared
4 KiB page, a 64-byte post-grant allocation and bounded control state. Control
and release helper threads request 128 KiB stacks. The separate accounting test
uses a 4 MiB mapping and then a 4 MiB heap allocation. Cases execute serially and
create at most two child actors (caretaker plus mutator); aggregate deliberate
fixture buffers/stacks stay below 8 MiB, below the 64 MiB assigned cap. Loader,
libtest and OS implementation memory are not claimed to fit this payload
allowance, nor used as production budget evidence. No test pressures the host.

`native_fixture.rs` self-spawns the same libtest executable through the single
launcher in `win32.rs`. `PROC_THREAD_ATTRIBUTE_JOB_LIST` assigns the child to its
Job during suspended creation; explicit inherited handle lists include only the
shared page, control events/mutex, selected observer capability and standard
streams. There is no create-then-assign fallback or breakaway flag. Membership is
checked before resume. X64 ABI sizes/offsets are checked at the FFI boundary.
Child stdout is NUL so a child libtest summary cannot mimic a parent pass; child
stderr remains live for failures. The parent checks exit status.

`Fixture`/`Process` guards terminate and wait on genuine process handles on
failure/unwind. The outer cleanup Job is drained to zero active members before
reservation release. Startup/normal exit waits are bounded at 5 s; workload
loops end after 8 s, caretaker waits after 10 s, and termination/drain checks
are bounded at 5 s. Unexpected failure to confirm cleanup aborts the case
process with its kill-on-close Job, rather than claiming zero survivors.

In the parent-death witness, the observer keeps a real worker process handle,
but no inner Job handle. It kills the caretaker while the worker is parked and
observes last-inner-handle termination. In the adversarial variant the observer
intentionally duplicates/retains the inner Job, observes the worker survive the
caretaker, then externally terminates and waits for it. An independently held
outer cleanup Job covers the child actors even if inner-handle setup or a test
assertion fails. This tests retained-handle custody; it does not claim arbitrary
untrusted descendant code or every possible handle-escape pattern is qualified.

## Protocol and deadline cases

| Native case | Witness and limit |
| --- | --- |
| `native_grow_blocked_control_and_cancellation` | The fixture mutator holds the channel mutex and announces GROW before any underlying 64-byte System allocation. A precreated helper acknowledges the known blocked state without acquiring that mutex. Cancellation precedes late grant/resume; allocation stays zero. This is a controlled allocator-path fixture, not a global production allocator hook. |
| `native_grow_grant_only_allocates_after_current_resume` | Grant alone cannot bypass the current park epoch; the real tiny allocation follows the current resume and successful exit. |
| `native_grow_waits_for_transient_or_persistent_headroom` | The real GROW fixture traverses fresh scripted samples, a release wakeup, preferred/minimal selection and the original absolute 2 s deadline. Persistent shortage leaves allocation zero for the whole window. |
| `native_cancel_reset_across_worker_phases` | Running, GROW-blocked, pause-requested and parked states; the same terminal flag represents fixture cancellation/reset. No product reset command is exercised. |
| `native_admission_fresh_samples_release_and_absolute_deadline` | Each injected sample, preferred/minimal verdict and fixture ledger update shares one mutex. Release is a wakeup hint, not admission. Successive epochs reject reuse of the preceding ack; delayed ack cannot restart the 2 s window. |
| `native_uncheckpointed_work_fails_quiescence_but_supervisor_reaches_deadline` | An intentionally uncheckpointed bounded page-touch loop continues progressing after an event; no current ack appears, supervisor reaches its original deadline, then externally cleans up. Missing quiescence is the expected negative observation, not production wait equivalence. |
| `native_parent_death_kills_parked_worker_before_lease_release` | Caretaker death, last-inner-Job-handle behavior and observed worker exit with allowance still held. |
| `native_retained_job_handle_requires_external_cleanup` | Deliberate inner Job retention prevents automatic death; external cleanup observes exit before allowance release. |

Four model cases in `lib.rs` separately check generation, preferred/minimal,
injected ticks/release, cancellation and deadline behavior. Expiry without an
ack is `MissingQuiescence`, distinct from a parked headroom wait expiring. The
model is not a native process/pool witness. Current fixture checkpoints cover
only the known mutator/control code. They do not suspend arbitrary threads or
establish bounded acknowledgment for numerical loops, native calls, parsing,
copying, hashing, serialization, allocator caches or pending I/O in production.

The accounting case samples `PROCESS_MEMORY_COUNTERS_EX2`: private commit,
private working set, total working set and shared commit, at untouched committed
mapping, touched mapping, release, touched heap and freed heap. API failure is a
failure; no exact OS delta or retained-heap behavior is required. No residency
credit is derived from commit, and no Windows counter is asserted equivalent
to macOS physical footprint or its lifetime peak.

## Runner and evidence interpretation

`run.ps1` requires native x64 Windows and x64 PowerShell, Rust 1.97.1 MSVC and
optionally an exact expected Git candidate. The supplemental
`piping-windows-worker-fixture.yml` provides that candidate, `windows-2025`, a
fresh runner-local target, no credential persistence and read-only contents
permission. It uses the existing product sparse-checkout pattern to avoid
historical execution-record MAX_PATH. There is no result/build cache or guest
provisioning. Each of 13 exact required cases must enumerate once and produce
one passing/non-ignored libtest summary; the internal child entry is excluded
from the witness count. Output is streamed at compile/link enumeration and
per-case execution/return; child failure diagnostics remain visible.

Execution evidence belongs in the candidate PR/hosted run. A host cross-check
or four model passes do not establish Windows execution. The crate's committed
Cargo.lock contains no external packages; no registry acquisition delta or
common-guest transfer is introduced. Hosted Rust toolchain acquisition remains
confined to the existing hosted environment. Owner release, acceptance, risk,
purchase and guest provisioning decisions are unaffected.

DEL-04-07 owns this fixture and gains project-relative code/check pointers.
Its governing memory-budget Design, ScopeOfWork and needs stay unchanged.
DEL-07-11 shell/cancellation interfaces, DEL-10-04 release matrix/required
harness and DEL-04-08 retained pricing/registration remain unchanged; this
fixture fulfills none of their remaining Windows product qualifications.

## Independent review threat model

Challenge wrong candidate/platform false greens; event delivery, stale ack or
unchecked work mistaken for quiescence; reset deadline or blocked supervisor;
allocator/channel deadlock; commit/working-set conflation or unreserved
transition; lease release on kill request/EOF; process escape or inherited Job
lifetime; hangs/leaks; swallowed child/test failures or empty selection;
changed references/production policy; secret/write/provisioning overreach.
Independent review uses at most three adversarial tooling rounds. Findings
outside this model are recorded; a new kind inside it goes to Agent 0. No
fixture result permits silently shortening the governing headroom wait.

## Official API basis

- [PROCESS_MEMORY_COUNTERS_EX2](https://learn.microsoft.com/en-us/windows/win32/api/psapi/ns-psapi-process_memory_counters_ex2), [GetProcessMemoryInfo](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getprocessmemoryinfo), [VirtualAlloc](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc), [VirtualFree](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualfree).
- [Process creation attributes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute), [CreateProcessW](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw), [Job objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects), [TerminateProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess), [WaitForSingleObject](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject).
- [SuspendThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-suspendthread) is debugger-oriented, not a synchronization substitute. [Anonymous pipe operations](https://learn.microsoft.com/en-us/windows/win32/ipc/anonymous-pipe-operations) do not support overlapped I/O; [CancelIoEx](https://learn.microsoft.com/en-us/windows/win32/api/ioapiset/nf-ioapiset-cancelioex) does not wait for completion. These fixture events/shared state do not adopt a new external transport. Job limits fail commits and are not the production allocator-grant policy.
