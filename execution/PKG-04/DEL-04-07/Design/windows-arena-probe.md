# Windows arena investigation (RT-V7 / DEL-04-07)

Maturity: experimental standalone probe; execution evidence is candidate-bound
in its PR. This is not a production allocator/helper/solver/Tauri connection,
a selected replacement for the governing memory policy, or retained qualification.
No final retry wording is adopted here. Budgets, fresh admission/waits,
cancellation/confirmed-exit/result custody and no numerical-mode fallback stay
unchanged. Windows retained registration stays Stale; R64 MiB is unqualified.

## Implemented boundary and cases

The native arena uses documented VirtualAlloc reserve/commit and VirtualFree
decommit/release, 256 in-arena slot descriptors, separate fixed control-page
allowance and checked sizes/alignment. Reserve-only VA is not leased backing.
The authority bitmap distinguishes admitted from committed pages; a failed
commit cannot ask for the same page twice. Live blocks, retained pages and
metadata are not double-counted. Free retains page charges; successful decommit
changes actual committed state, but this conservative probe still holds granted
authority until process exit. That over-reservation is a cost, not a production
reclamation design. No Job commit limit substitutes for admission.

The ledger holds two workers' authority and committed arena bytes separately,
with zero realized credits initially for both resource checks. Physical and
commit headroom are independent scripted inputs, not live OS free-memory
samples. Each verdict and mutation is under one mutex. The child protocol has
epoch-matched replies, cancellation/reset and typed outcomes. A growth operation
has one original absolute 2 s window across headroom attempts, grant and a
positively classified injected commit-headroom failure. A separate later
operation gets its own window. Unclassified OS errors remain distinct; the
probe does not establish a production GetLastError classifier.

| Witness | What it checks |
| --- | --- |
| arena::tests::native_arena | Native page/granule and metadata sizes, checked overflow, high alignment, zeroing/reuse, packed small slots, moving realloc overlap and old contents, repeated fault authority, retained pages and failed/successful decommit. |
| ledger::tests::ledgers_and_cancellation | Independent physical/commit shortage, preferred/minimal targets, two-worker debt, actual/future separation, overflow, cancellation/reset/stale generation and no early release. |
| ledger::tests::retry_classifier | Physical/commit admission and explicitly synthetic retryable commit shortage only; address/limit/unknown/cancel/stale/unclassified OS faults do not retry. |
| worker::native_recovery | Real child rejects a stale grant, traverses physical and commit shortage, then post-admission synthetic commit failure/recovery; old data and granted page authority persist. |
| worker::native_persistent_headroom | Full original 2 s shortage, typed terminal result and held authority. |
| worker::native_nonheadroom_no_retry | Typed synthetic other failure, zero retries, old contents intact. |
| worker::native_cancel_and_reset | Cancellation after admission but before commit; reset rejects old-generation grant; no added committed page. |
| worker::two_native_workers | Two real children hold liabilities under one pool mutex; cancellation dominates late replies; one observed exit does not release the other lease. |
| worker::blocked_child_termination | External termination while blocked; lease stays held until process handle signals. |
| worker::child_failure_negative_control | A deliberately failing child must yield nonzero status. |
| worker::native_separate_deadline | An unrelated earlier expired operation does not consume the next growth window. |
| worker::native_commit_retry_deadline | Delayed grant and classified commit fault still expire at the original deadline, not a new retry window. |

The runner verifies the complete 14-entry inventory (12 witnesses and two
internal child/timeout-control entries), executes every witness once/nonignored, and checks missing-selection and forced-timeout cleanup negative controls. Compiler/link, case, child failure and
cleanup diagnostics stream immediately. Native OS/process x64, Rust1.97.1 MSVC,
exact candidate, fresh runner-local target and --locked dependency-free crate
are checked. The supplemental workflow remains outside required harness and
uses product sparse checkout for historical MAX_PATH. No guest/local-heavy
route or registry acquisition is needed.

At most two arena workers and two 4 MiB VA arenas exist, with a case controller
and an external Rust supervisor. The supervisor owns the case's creation-time
outer Job independently of the case and arena workers. On timeout/abort/error,
it drains the Job to zero active processes and observes case exit before emitting
the cleanup marker required by PowerShell. The timeout negative control first
establishes a real blocked arena descendant, then confirms external termination
of both case and descendant. It introduces no additional arena worker. If the
supervisor cannot confirm cleanup, the case fails without a confirmation marker;
PowerShell's 45 s last backstop labels such cleanup unconfirmed and kill-on-close
remains a backstop, not invented observation. Metadata/control/buffers are
bounded and deliberate memory stays below 64 MiB. Loader, framework and native
process memory is not claimed covered by that payload allowance. Creation-time
Job lists, handle allowlists, process-exit waits and bounded Job drain are
adapted from committed fixture source226de967 (now integrated on main as
#1737); no dependency on a closed contribution checkout is retained.

Cost output separates requested/live capacity, committed data, metadata,
retained empty pages, committed slack, VA-span unused bytes and syscall count.
VA-span unused includes uncommitted alignment gaps; it is not physical waste.
Scripted small-slot packing exposes per-allocation-page overhead. A classified
post-admission commit retry revalidates physical and commit headroom under the
parent lock with a zero-delta request: existing authority is neither lost nor
granted twice, and the original growth deadline is retained. Counter or
hosted timing observations are not a qualified resource/performance profile.

## Production non-arena/startup audit (read-only)

Source basis initially main31084e1f792b0612013e508b30e2a5dabdc151e7; the
worker/backend/core memory paths were identical to1560bbf7 in that scoped diff.
Integrated main c5c1c8f089702572bbdeddc288284cbc6dedfd58 contains
#1735/#1737 and #1741's routing repair and is merged into the probe candidate; their
fixture evidence does not qualify this arena or a complete Windows worker.
No Windows desktop PE was built or examined in this tranche, so actual imports,
loaded modules and PE stack reservations remain unknown rather than inferred
from the package lock.

| Boundary | Source-established fact | Bound/remaining decisive evidence |
| --- | --- | --- |
| Same app executable | src-tauri/src/main.rs dispatches --swbpipe-solve-worker before run/Tauri setup. The worker still links the desktop crate. | This is not a loader fence. A hosted artifact import/delay-load/module inventory is required before claiming no UI/native startup cost. |
| Cargo/native surface | Locked desktop declares Tauri2.11.1/runtime-wry2.11.1, wry0.55.1, tao0.35.2, WebView2 COM0.38.2, dialog/rfd0.16 and bundled rusqlite0.32.1/libsqlite3-sys0.30.1. The lock contains multiple platform branches. | These are source dependencies, not an actual Windows PE import list. A later minimal Rust worker comparison can reduce the boundary without changing engineering methods; none is ported here. |
| Global allocation | worker_alloc.rs429–491 delegates to System; Mac envelopes charge live requests and moving overlap, not all retained native heap. | The arena proves only its API. Direct System/native/CRT/TLS/RNG/mapping allocations must be audited or bounded; Rust global allocation replacement alone cannot cover them. |
| Startup/control | solve_worker.rs212–253 creates raw parent watchdog before enable; raw request is copied before arm, under a parent starting grant. | Initial Windows liability must be bounded before process creation and control setup. The Mac64 MiB allowance is not a Windows result. |
| Threads/stacks | Mac watchdog requests64 KiB. Ordinary shared entry passes no retained profile (PP lib.rs2608), so it cannot enter retained reserved-stack dispatch3534. Parent supervisor/blocking threads are outside the child. | Windows main/other stack reserve/commit needs the actual PE/header and thread settings. Charge future stack-touch capacity; no default stack size or absence of loader-created threads is assumed. |
| IO/native operations | Child control uses fixed frames; result bytes already exist before send. Native pipes and runtime/math/hash initialization can bypass the Rust allocator. | Bound outstanding operations, their user/kernel/section liabilities and helper threads. No general bound follows from sampled private commit or working set. |
| Parent pool purposes | Intake, interactive commands, retained results/custody and worker grants share the parent pool; separate report rendering is not this worker. | A future dual-resource ledger must include all relevant purposes with resource-appropriate credits, not only arena workers. No current pool schema or behavior changes here. |

The next finite closure is an actual worker artifact import/stack/native-call
inventory, cold/warm startup/error observations, and enforced non-arena bounds
before process creation; counters expose omissions but are not the proof. If
the same-app boundary cannot be bounded, compare a minimal helper sharing the
same Rust methods. No production helper/global allocator or source-wide
checkpoint modification is authorized by this investigation.

RT-E1's announced raw_value framing is a PARENT interactive path, not a child
allocation. The locked Tauri2.11.1 official Cargo.toml already declares normal
serde_json features=[raw_value]; explicitly declaring it in desktop Cargo makes
the direct dependency intention visible rather than introducing a new package
or that feature into this normal graph. Existing ingress.rs module docs already
name raw_value/unbounded_depth and exclude preserve_order/arbitrary_precision.
No local Cargo feature-resolution run was done. The direct flag alone does not
create a new feature-only layout change in that graph; new frame wrappers,
owned buffers and lifetimes still need their actual path's admission accounting.
Borrowed RawValue can reference bytes and owned RawValue can retain a full frame;
neither restores original model number lexemes already lost upstream to Value
or another serialization. The coming production parser/commands and their
permission/reach review remain RT-E1's work, not this probe's qualification.

## Solver guard and declarations

Pending solver input9b73234e50f5ac151ad9f8a80c4c961c5a488170 adds a checked
pre-case nonlinear dense guard in both modes:88*n_global²+16*n_free² against
fixed floor(0.9*12GiB)=11,596,411,699 B, about1759 all-free nodes; overflow
refuses. This is a provisional COMPONENT estimate, not a complete worker
bound, and does not scale with larger desktop budgets. Preserve/reconcile that
text and #1739 numerical pointers when they land; do not use this guard to
price arena/non-arena liabilities or alter its limit.

After merging #1741, every declared scan was regenerated from the integrated
source using its actual anchor/exclusion/name-or-suffix policy. The two project
inventories add this probe's six .rs files (external guard included) and
Cargo.toml; no inputs are removed. Existing fixture/numerical entries remain.
The full source_read_dependencies graph validates 41 manifests and 1,359 read
inputs. All registered crate-tree digests were recomputed and matched; reader
and source-anchor hashes likewise matched, so none was blindly repinned. The
two source inventories over product_physics also received canonical ordering.
Both routing suites pass under OPENPIPESTRESS_NO_CARGO_BUILD=1 (65 tests and
87 subtests). The selected Root software_workflow suite also passes (76 tests,
80 subtests, one existing PytestReturnNotNoneWarning), using the existing venv
on PATH for its nested Python commands and the same no-Cargo guard. The remaining selector-rule gap recorded by #1741 is unchanged;
a green harness is not substituted for these explicit checks.

DEL-04-07 gains this Design and additive code/check pointers. Its ScopeOfWork,
needs and governing memory-budget Design are unchanged. Ready-first integration
must preserve both prior #1737 fixture and #1739 numerical/temperature pointers.
DEL-04-08 qualification stays Stale; DEL-07-11 shell and DEL-10-04 release/required
harness obligations are not discharged. No governing wording was adopted.

## Independent review threat model and official basis

Prior worker-fixture threats apply: wrong candidate/platform false green;
stale/event/unchecked work mistaken for quiescence; reset deadline/blocked
supervisor/deadlock; release on kill request/EOF; escape/inherited Job lifetime;
hang/leak; swallowed failure/empty selection; production/reference/policy or
secret/write/provisioning overreach. Additional threats: underpriced metadata,
retention, realloc or decommit; physical/commit liability conflation; unclassified
allocation failure; stale grants/credits; overflow/alignment and case suppression.
Use at most3 tooling rounds. Record out-of-model findings; route a new kind
inside the model to Agent0. Repair small findings without new delegates.

Official references: [VirtualAlloc](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc), [VirtualFree](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualfree), [GetSystemInfo](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsysteminfo), [process attributes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute), [Job objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects), [thread stacks](https://learn.microsoft.com/en-us/windows/win32/procthread/thread-stack-size), [Rust System](https://doc.rust-lang.org/std/alloc/struct.System.html), [Tauri2.11.1 manifest](https://github.com/tauri-apps/tauri/blob/tauri-v2.11.1/crates/tauri/Cargo.toml), [serde_json1.0.149 RawValue](https://docs.rs/serde_json/1.0.149/serde_json/value/struct.RawValue.html).
