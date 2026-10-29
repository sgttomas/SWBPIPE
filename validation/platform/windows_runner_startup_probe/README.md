# PARTIAL actual headless Windows startup diagnostics

Experimental RT-V7 Later diagnostic. The selected statements are request-metadata
validation, two typed input refusals, input-open observations and independent
blocked-input cleanup. **Full solve response is BLOCKED/UNVERIFIED at the
unchanged 1 MiB capture cap.** A successful supplemental job establishes only the
four named partial cases. It does not discharge the two original solve witnesses
or their ten proposed live-output mutations.

`run.ps1` builds the existing `openpipestress-runner` entry unchanged with
Rust 1.97.1 MSVC, `--locked --release`, default features and a fresh target on
native x64 `windows-2025`. The controller and nine PE tests compile directly with
rustc; there is no new manifest, lock or registry dependency. Exact candidate,
architecture, lock, executable hash and launched process path are checked.
Compile/link/case diagnostics stream live. Raw captured JSON remains in hosted
scratch. Names and counters are sanitized; private paths and identities do not
enter maintained outputs.

The executable inventory is exactly:

- `validate_metadata`: a valid invented request, matching operation/reference
  metadata, clean actual metadata validation, exit 0 and no runner/mechanics result.
- `malformed_json`: exactly `{`, typed `HEADLESS_RUNNER_CLI_INPUT_JSON_INVALID`,
  `RUNNER_BLOCKING`/`blocking`, exit 2, correct affected input and no request
  validation, job or mechanics result.
- `missing_payload`: valid invented metadata without `solve`, typed
  `HEADLESS_RUNNER_SOLVE_PAYLOAD_MISSING`, `RUNNER_BLOCKING`/`blocking`, exit 1,
  correct request reference and no job or mechanics result.
- `blocked_timeout`: the actual CLI receives `validate-input` with its named
  input connected but no bytes/EOF. The observer times out and confirms exit
  and zero surviving Job members.

The separate blocked inventory names `solve_first`, `solve_repeat` and the ten
live solve-output mutations. None runs automatically or counts as diagnostic
success. The controller compares each input with its compiled fixture bytes
(or the single malformed byte) before creating the runner. Unknown selections,
wrong verbs and substitute payloads fail. Metadata and missing-payload fixtures
have no solve payload; both typed refusal paths return before mechanics.
`fixtures/solve.json` remains the exact failed diagnostic input/evidence basis,
not an active case or an admitted response fallback.

Metadata/refusal outputs have no mechanics rows. Their fixed source shapes have
at most 40 scalar leaves, paths below 128 ASCII bytes and messages below 512
bytes. Allowing 2 KiB per audit decision/finding pair plus payload/framing gives
a conservative bound below 128 KiB for these selected inputs, below the unchanged
1 MiB runtime guard. This source sizing is not native execution or a bound for
any solve/user input.

## Explicit diagnostic intent and validation

The three output-bearing cases add the existing documented
`--explicit-local-private-intent` flag. This is explicit test intent only for the
compiled, genuinely invented P7-labelled diagnostic records and their fixed
invented error. It grants no user-data, protected-write, `--output` path,
user-release or public-export authority. No payload-carried intent is trusted.
The argv changes; the flag's source/semantics and all existing warnings/audits
remain unchanged. The blocked-input case has no flag and never releases input.
No solve is retried under the flag.

Operation-specific predicates check the real controlled export, observed exit,
request/operation binding and exact refusal code/class/severity/source/affected
reference. Metadata checks bound input refs because the CLI does not echo them;
no output reference echo is invented. Null job/mechanics/result-validation fields
are required, rather than accepting a fabricated completed status. Audit decision
paths must cover every payload scalar; warning paths, findings and summary counts
must agree. Nonblocking privacy warnings are preserved and do not fail by
themselves. A refusal's local-private source-location warning is required.

After each of the three successful captured baselines, twelve mutations exercise
the same predicate: empty/malformed controlled object, missing payload, wrong
operation, exit, runner status, validation or refs, wrong/missing typed diagnostic,
missing audit, stripped warnings and unexpected privacy blocking. Wrong input
refs mutate the paired bound fixture context; other mutations alter the actual
captured output or observed exit. Expected total: **36 diagnostic mutations**,
not the ten unexecuted live solve mutations. Missing-marker, wrong-candidate,
empty/unknown-selection and blocked-inventory controls remain separate.

## Gate, custody and observations

The CLI parses args, calls `read_input` through `std::fs::read_to_string`, then
parses/executes only after complete input/EOF. A local byte-mode outbound named
pipe rejects remote clients, uses first-instance exclusivity and protected SDDL
`D:P(A;;GA;;;SY)(A;;GA;;;OW)` (System/object owner). The noninheritable server
verifies the client's actual runner PID while withholding every byte/EOF.
`INPUT_CONNECTED_PAYLOAD_WITHHELD` proves this source-bound input open only,
not pending ReadFile, quiescence, settled startup or allocator initialization.
Pinned official std source context/hashes are streamed; actual compatibility
requires the native diagnostic outputs and cleanup witnesses.

The independent observer launches the case with creation-time Job assignment;
the case creates the runner in a nested Job, retaining outer membership. Job
handles never inherit. Only named standard handles and the case's ready event
inherit. There is at most one runner and two outer Job processes. Kill-on-close,
active-process limit 2 and 512 MiB Job memory are diagnostic safety guards,
not production memory admission. Any guard hit fails the statement.

IO phases use bounded OVERLAPPED waits (5 s plus 1 s cancellation); capture has
an 8 s deadline, exit wait 3 s, outer case timeout 25 s and drain/exit 5 s. The
blocked control uses a 10 s gate setup and deliberate 200 ms timeout. Input stays
at most 64 KiB; combined capture stays at most 1 MiB. Capture threads belong to
the case, not the runner. EOF/kill requests do not release custody: process
HANDLE signalling precedes exit-code inspection (including 259), and independent
Job drain must observe active zero. Markers are ordered and checked once each:
CREATED_SUSPENDED, RESUMED, INPUT_CONNECTED_PAYLOAD_WITHHELD, PAYLOAD_SENT_EOF,
OUTPUT_COMPLETE, EXIT_OBSERVED, JOB_DRAINED. The blocked case stops at its gate
and requires the external termination/drain markers.

Actual PE32+ AMD64 parsing reports imports, delay imports, sizes and main-stack
headers with bounded RVA/raw/section/string/table checks. The exact nine-test
inventory includes truncation, wrong machine and malformed bounds controls.
Live gated samples separately report private usage, WS/peak WS, VA region types,
thread counts and module basenames with bounded failure/retry handling. These
are sequential observations with races/gaps, not maxima, exact heap attribution,
physical/commit liabilities or other-thread stack bounds. No cold-cache or
performance claim follows. This is not the desktop PE, same-app solver worker or
RT-E1 parent framing threads. R64 and Nphysical/Ncommit/native/stack bounds remain
unqualified; Windows retained registration remains Stale.

## Public boundary and remaining solve constraint

Every file here ships in the existing public export. Fixtures honestly retain
`invented_non_engineering_example`/`pending`; authoritative P7 generation binds
their exact bytes. The narrow project attribute rule preserves those bytes under
autocrlf and survives the public cutdown. Actual Rust request shape requires
`tbd_decisions`, which the published HeadlessRunnerRequest definition excludes;
this unit does not change that schema or claim full schema conformance.

The full raw-committed-candidate exporter remains required before fan-in, including
all new files and generated P7 entries. Its job is explicitly deferred until the
separate verified exporter batching repair is merged, then must be enabled and
pass; a skipped export is not boundary evidence or release authority. No exporter
source is edited here. Full solve representation cost remains an item in the
owning DEL-04-07 Design, cross-referenced to DEL-10-05 and #1742's source-qualified
F-1 observation. Actual failed runs and the maintained-carrier size illustration
are retained there; no limit, method, projection or serializer is changed.

Official basis: [PE/COFF](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format),
[pinned Rust fs source](https://github.com/rust-lang/rust/blob/1.97.1/library/std/src/fs.rs),
[pinned Windows fs source](https://github.com/rust-lang/rust/blob/1.97.1/library/std/src/sys/fs/windows.rs),
[pinned Windows handle source](https://github.com/rust-lang/rust/blob/1.97.1/library/std/src/sys/pal/windows/handle.rs),
[named pipe security](https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipe-security-and-access-rights),
[pipe operations](https://learn.microsoft.com/en-us/windows/win32/ipc/synchronous-and-overlapped-pipe-i-o),
[client PID](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getnamedpipeclientprocessid),
[process attributes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute),
[Jobs](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects),
[memory counters](https://learn.microsoft.com/en-us/windows/win32/api/psapi/ns-psapi-process_memory_counters_ex),
[VirtualQueryEx](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualqueryex),
[modules](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodules),
[thread snapshots](https://learn.microsoft.com/en-us/windows/win32/toolhelp/traversing-the-thread-list),
[thread stacks](https://learn.microsoft.com/en-us/windows/win32/procthread/thread-stack-size).
