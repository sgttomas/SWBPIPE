# Actual headless Windows artifact/startup diagnostic

Experimental RT-V7 Later investigation. Native execution is pending until this
candidate's supplemental hosted Windows job succeeds. This directory ships in
the public export. It contains no production startup or allocator connection.

`run.ps1` is intended for the committed exact PR head on `windows-2025`, native
x64 OS/process, Rust 1.97.1 MSVC. It builds the existing headless runner unchanged,
with default features, its existing locked dependency graph and a fresh release
target directory. The controller and its PE tests use `rustc` directly: no new
manifest, lock, dependency or registry package. Compile/link/case output streams
live; raw runner JSON stays in hosted scratch and is never uploaded. Paths are
redacted in build/export output. Import and module observations accept only
bounded simple public names; unsupported names fail rather than print paths.

The workflow has a separate full committed public-export boundary/lint job. Its
unchanged exporter invokes the empty-dependencies protected-content Rust linter.
No exporter is run locally. Before export starts, that job requires every
fixture hash in the committed generated P7 scope. Final evidence must bind
`tools/release/provenance_scope.json` and every new file to the same candidate.
The existing exporter's fixed-size lint batches exceed Windows command capacity
for this tree; final export evidence awaits its separately owned batching repair.
No reduced scan or batching change is introduced by this diagnostic.
The exporter is not the separate DEC-058 owner scan or P7 scanner check.

The checked-in JSON preserves the actual Rust runner request metadata shape and
actual `invented_non_engineering_example` provenance. It copies the maintained
invented metadata shape. Its model derives from
`fixtures/model_operations/precision_connected_ui_model.json`: retain only the
axial nodal force and restrain the other five tip coordinates to keep this
startup/API exercise small. The maintained source model is unchanged. It
claims no engineering reference or numerical agreement. Metadata validation
changes both operation fields to `validate_input` and omits the solve payload;
that verb checks request metadata, not model validity. The missing-payload case
is a deliberate refusal. Malformed JSON is a one-character hosted scratch
input, not a falsely valid committed JSON file. P7 scope and routing outputs
are generated on integrated source by the parent. The published
`HeadlessRunnerRequest` definition excludes `tbd_decisions`, while actual Rust
requires it. These CLI fixtures follow the maintained source shape; they do not
claim conformance to that inconsistent schema definition or change it.

## Source-bound gate and custody

At the starting main revision `8e9392f10d22d217e34ffc5c28407024fed269d0`,
`core/runner/headless/src/bin/openpipestress-runner.rs` calls `parse_args`, then
`read_input` (named path uses `std::fs::read_to_string`), then
`execute_json_with_mode` only after the full string read returns. The controller
creates one local, byte-mode, outbound named pipe with remote clients rejected,
first-instance exclusivity and protected SDDL
`D:P(A;;GA;;;SY)(A;;GA;;;OW)` (System and object owner full access). The server
handle is noninheritable. Its unique name is not logged. The connected client
PID must equal the actual launched runner PID. No bytes or EOF are sent before
`INPUT_CONNECTED_PAYLOAD_WITHHELD`. This marker proves the named path was opened
inside `read_input`; it does not prove a pending ReadFile, quiescence, settled
startup, allocator or worker initialization. A still-live process is checked.
The pipe connection and write use bounded OVERLAPPED waits and cancellation;
the independent observer can terminate a case stuck in a native operation.

The script reads the pinned official Rust source over HTTPS in hosted Windows
and streams bounded relevant source context and hashes for review. Its token
checks are source-availability checks, not a semantic proof. The actual solve,
metadata and malformed/refusal cases must demonstrate completed buffered input
and broken-pipe EOF with this exact Rust/Windows artifact. If the std file path
or pipe behavior fails, the job fails; no runner edit or wrapper fallback is
allowed. A native success must be reviewed together with that source context.
Until then compatibility remains unverified.

One observer outside the Job launches one case suspended with creation-time Job
assignment, then resumes it. That case creates one unchanged runner suspended
inside a nested Job; Windows also retains inherited outer Job membership. Both
Jobs are kill-on-close, active-process limit 2 and job-memory limit 512 MiB. The
outer Job therefore contains at most case+runner; observer is outside. These are
diagnostic safety caps, never production admission equivalence. Any cap hit or
abnormal exit fails the observation; counters do not identify the cause alone.
No Job handle is inherited. The case inherits only NUL stdin, explicit duplicated
stdout/stderr and its ready event. Runner inherits only NUL stdin and the two
explicit stdout/stderr capture writers. The input server, read ends, process and
thread handles are absent from both handle lists. Ambient environment is retained
for normal native loading, never logged or qualified as a production environment.

The external observer keeps custody through case/runner faults. It terminates
remaining members, observes the Job's active-process count at zero and the case
process HANDLE signaled before `JOB_DRAINED`. Runner exit code is read only after
its HANDLE signals; exit code 259 is never used as a liveness predicate. The real
blocked-descendant control connects the unchanged runner, withholds all bytes
and EOF, signals its gate, verifies two active outer members, then deliberately
times out and kills both. It is a cleanup control, not a successful runner case.
Job/handle layouts and launch/drain patterns are adapted into standalone source
from committed `windows_arena_probe/src/win32.rs` and `arena_case_guard.rs`;
there is no include or inherited claim from their tests.

Each pipe phase waits at most 5 s (plus 1 s cancel); output completion 8 s,
runner exit 3 s, case observer 25 s plus bounded 5 s drain/exit. Blocked control
setup is 10 s plus a deliberate 200 ms timeout. Case bodies have the independent
observer backstop. Each input is at most 64 KiB; combined captured stdout/stderr
is at most 1 MiB. At most two bounded capture threads run in the case, outside
the runner. They are diagnostic overhead. The six serial cases form a finite
selection; workflow steps bound total time including a 45 minute fresh build
budget and a separate 20 minute exporter budget.

## Observations and limits

PE inspection parses the actual executable's PE32+ AMD64 headers, stack
reserve/commit, image/header/file sizes and regular/delay DLLs and imported
symbols/ordinals. It validates bounded raw/RVA/section/directory/string/thunk
ranges and checked arithmetic; unsupported/malformed input fails. Hosted
`rustc --test pe.rs` runs the exact nine-test inventory including every truncated
prefix of the synthetic bounded PE, empty/wrong-machine/overflow/bad-directory/
unbacked-import/unterminated-import/raw-bounds controls. Synthetic PE tests do
not count as actual runner inspection. The SHA256 is checked before/after all
cases, and QueryFullProcessImageNameW binds each actual launched process to the
candidate path. Cargo metadata is never PE evidence.

Markers are CREATED_SUSPENDED (pre-loader only), RESUMED,
INPUT_CONNECTED_PAYLOAD_WITHHELD, PAYLOAD_SENT_EOF, OUTPUT_COMPLETE,
EXIT_OBSERVED and JOB_DRAINED. Missing, duplicate or out-of-order required
markers fail; the deliberately blocked control requires only its three live
markers and the external drain marker. Wrong candidate, empty/unknown selection
and missing marker controls exercise the same predicates as actual witnesses.
Successful solves also require an identified completed job, present clean
request/result validations, empty runner/mechanics diagnostics, and nonempty
result references and rows. Ten mutations of the actual first successful output
must fail this same predicate: failed job, blocking diagnostics in each of four
nested locations, empty runner, missing request/result validation, empty result
references and empty mechanics rows. Those mutations are controls, not solver
or numerical qualification.

At the gated live phase, separate counters report GetProcessMemoryInfo private
usage, working set and peak working set; VirtualQueryEx sums private/image/mapped
commit/reserve; a Toolhelp snapshot counts threads; module basenames use bounded
PSAPI enumeration with three retries and explicit failure. These operations
race with normal runtime activity, occur sequentially with timing gaps, and
cannot establish a maximum, exact heap attribution, liability, physical backing
or Native/System bypass isolation. Module enumeration may still race even when
an API call succeeds. The main PE stack facts do not bound other thread stacks.
First fresh and repeated launches are observations, not true cold-cache proof
or a performance claim. No transient running samples or upper bounds are claimed.

This is not the desktop PE, same-app solver worker or parent general interactive
constructor path. RT-E1 reader/writer threads are parent liabilities and are not
observed by this headless artifact. R64, Nphysical/Ncommit/native/stack bounds,
production global allocator and numerical meaning remain unqualified. Retained
Windows registration remains Stale. Private/native/CRT/TLS, mappings, kernel IO,
helpers and future stack touch costs remain unknown. Next minimal evidence is
independent review of native source/artifact witnesses, followed by a separately
steered actual worker boundary and enforceable nonarena accounting.

Official basis: [PE/COFF](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format),
[Rust fs::read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html),
[pinned Rust fs source](https://github.com/rust-lang/rust/blob/1.97.1/library/std/src/fs.rs),
[pinned Windows fs source](https://github.com/rust-lang/rust/blob/1.97.1/library/std/src/sys/fs/windows.rs),
[pinned Windows handle source](https://github.com/rust-lang/rust/blob/1.97.1/library/std/src/sys/pal/windows/handle.rs),
[named pipe security](https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipe-security-and-access-rights),
[pipe operations](https://learn.microsoft.com/en-us/windows/win32/ipc/synchronous-and-overlapped-pipe-i-o),
[ReadFile](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile),
[client PID](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getnamedpipeclientprocessid),
[process attributes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute),
[Jobs](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects),
[memory counters](https://learn.microsoft.com/en-us/windows/win32/api/psapi/ns-psapi-process_memory_counters_ex),
[VirtualQueryEx](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualqueryex),
[modules](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodules),
[thread snapshots](https://learn.microsoft.com/en-us/windows/win32/toolhelp/traversing-the-thread-list),
[thread stacks](https://learn.microsoft.com/en-us/windows/win32/procthread/thread-stack-size).
