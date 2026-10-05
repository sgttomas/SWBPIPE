# I38 — native reply overlap source result

**A finite synchronous-section bound is conditionally available, but no complete
native aggregate allowance follows from the current source plus I30 alone.** The
actual poll command is synchronous, not a suspended Tokio command future. Its
owned result runs directly into synchronous `IpcResponse::body`. Therefore a
qualified finite bound on simultaneous dispatch sections can bound small reply
owners as well as large ones. That dispatch/target premise, retained request
contexts, and the emergency-buffer proof remain explicit qualification cells.
K large slots and first-party sequential retry do not establish them.

This is a proposed source/profile condition for ROOT/P4/P5 review, not implemented
F2a APIs, an accepted numeric ceiling, native availability, or policy. Receipt:
2026-10-02 23:35:00 UTC. Early checkpoint sent to ROOT at the 23:36:50 clock;
original checkpoint/cutoff/hard-return limits: 23:45:00 / 23:55:00 /
2026-10-03 00:00:00 UTC. See `ORIGINS.json` for actual completion timing.

## What the actual source establishes

P means `projects/chirality-piping`. Source files inspected in NUM are byte-equal
by scoped Git diff to main `49034a940f3f8cd3f3da4d4cbc839943b808063d`; resumed
record basis is `52a57416b40f9b669303c3ed7ca052ee0ba729be`. A later observed HEAD
was `081cd0e6552547e0b1e094956f4deb458606b3db`; no Git mutation was performed here.

- `P/apps/desktop/src-tauri/src/lib.rs:1834–1839` uses plain
  `#[tauri::command] fn poll_preview_mechanics_job`, returning
  `Result<SolveJobStatusReport,String>`. Both direct-solve and cancellation
  commands are also synchronous. A JS `await invoke(...)` is not evidence of
  a Rust async command. The background solve uses `std::thread::spawn`; it
  publishes a Value to the registry and does not own the poll response.
- Pinned tauri-macros 2.6.1 defaults to Blocking (`wrapper.rs:47–54`). Its
  blocking wrapper calls the command, `blocking_kind`, and `kind.block`
  directly (`:404–435`). Tauri 2.11.1 `command.rs:249–256` converts the Result
  synchronously, and `ipc/mod.rs:265–275,392–400` calls `body()` before the
  responder. There is no intervening await, executor yield or task queue here.
- The registry mutex protects lookup/clone only (`lib.rs:1731–1749`); it drops
  before serialization. It cannot alone limit how many copies survive outside
  the lock. I30's future guard must still live in the non-Serialize response
  wrapper until body transfer or actual Drop.
- Tauri's default IPC script first uses custom-protocol fetch on macOS and can
  fall back to postMessage (`scripts/ipc-protocol.js:19–84`). Both must be
  covered. Wry's postMessage delegate is declared MainThreadOnly and directly
  invokes its handler. Its custom-protocol callback also directly calls the
  registered function, but enters from a native WebKit callback. The loaded
  source does not supply a complete native callback-concurrency/reentrancy
  contract for every target/entry path. Single configured window is not such
  a proof; neither is an asynchronous-responder API a task-count limit.

Exact already-local Tauri/macros/runtime-Wry/Wry/serde_json/Tokio sources were
read. Each retained source hash was checked against its cached `.crate` archive;
archive SHA-256 values match native Cargo.lock checksums. No dependencies were
installed, compiled, run or fetched. The remaining premise is target/entry-path
qualification, not missing Rust implementation of the inspected chain.

## Conditional finite bound and smallest faithful proposal

Let H be a source/profile-proved finite upper bound on simultaneously active
reply-construction sections for this qualified native caller. Each section starts
before the first reply-specific allocation and ends only when its owned JSON body
has transferred or all reply allocations have actually dropped. Require:

1. All included entry paths (custom protocol, postMessage fallback, any native
   direct caller) execute the current synchronous command-to-body chain; no
   wrapper/control escapes into a queue, registry, worker, deferred serializer,
   await, `block_in_place`, or reentrant callback while retaining that reply.
2. The target/runtime establishes a finite number of concurrently executing
   sections and a finite nesting bound. For a proven single, nonreentrant native
   dispatch lane, the structural implication is one section, not a chosen
   memory allowance. This packet does not qualify that platform premise.
3. If H is also used to cover request/id/header/resolver/context bytes, its scope
   must run from their actual Rust acquisition through their actual Drop. It
   cannot end merely when body transfer occurs: the macro's InvokeMessage and
   protocol request context can still be on the stack while the responder runs.
   Any such overlap outside H needs its own bound and term.
4. I30's proposed small control/emergency owner remains live through body
   transfer/Drop, with finite field lengths and proved buffer/error paths.
   No ordinary Tauri Err fallback escapes this scope uncharged.

Under these premises, if l large replies and d denied/reply-local-error replies
are simultaneously owned, `l ≤ K` and `l + d ≤ H`, hence `d ≤ H` independently
of K. Allocation preemption on one of finitely many threads does not violate
this argument: that thread still owns its section. The premise forbids nesting
or handoff that would create extra retained owners, not ordinary preemption.

**Smallest proposal: add this explicit synchronous-dispatch qualification to the
native P4/P5 caller profile and keep the existing command-to-body segment
synchronous when implementing I30.** It adds no endpoint, queue, retry policy,
ordinary-request throttling, or accepted platform claim. P5 must identify and
verify the actual callback lane/nesting premise and request-context census for
the selected build. If those cannot be established, native aggregate
qualification stays open; report the missing H/context term instead of choosing
an arbitrary value.

A later async refactor requires a new proof. A fixed Tokio worker count alone
bounds only executing nonyielding sections: queued/suspended futures may retain
requests/resolvers, and `block_in_place` can hand a worker core to another thread.
The loaded default Tokio constructor also derives worker count from its builder
profile, not this product's reply policy. It is not the present poll route and
supplies no current denial-count answer. A small-reply semaphore acquired *inside*
an unbounded population of command futures leaves the waiting requests; rejecting
its losers creates another error-reply population. Neither is a complete repair.

## Exact aggregate addition / remaining term

For each requested-live and moving-reallocation phase metric k, retain the
existing resident/caller terms and write the native ownership sum as:

`E_native(k,t) = R(k,t) + S(k,t) + Q(k,t)
                 + Σ[large owners i] L_i(k,t)
                 + Σ[small denial/error owners j] D_j(k,t)`

R includes the retained job Value/lease and the applicable existing registry
backing census; S includes shared reply/control metadata and guard headers not
already charged. Q is still-owned invocation/request/argument/resolver/transport
context storage not already assigned to an L or D owner, including any pending or
post-body/pre-Drop context overlap inside the accepted window. Partition actual
allocations once; shared payloads are not duplicate allocations. L must cover an
admitted reply's success **and** reachable error/partial-buffer/emergency phases.
D must cover each separately owned denied/reply-local-error request, including its
actual input/ID widths if assigned there. Ordinary/exact/unknown-job/argument
errors keep their behavior and need their separate caller census when included;
arbitrary error strings are not finite-width W1 controls.

If uniform qualified maxima L_k and D_k and the full H premise are established,
a conservative reply-only upper is `K*L_k + H*D_k`; a tighter partitioned form is
`max_{0≤l≤min(K,H)} [l*L_k + (H-l)*D_k]`. Add the independently derived R, S and Q
terms. These formulas select no numeric coefficient and do not replace the
per-phase capacity/moving-overlap derivation. If H covers only the reply section,
Q remains separately bounded. Without either proof, the precise missing term is
`sup_t Σ D_j(k,t)` plus any unbounded `Q(k,t)`, not zero and not K times D.

Serializer output growth, old/new Vec overlap, clone/header capacities, fixed
error-code set, native target/layout/feature pins and an actually available
emergency construction are still P4/P5 obligations. `serde_json::to_string` builds
an ordinary growing Vec and turns it into String; Result-returning serialization
does not by itself make allocation failure recoverable. No process-RSS or
external-queue bound is asserted.

## Custody and verification limits

No changes to I30/RV41's Busy same-job retry, original capture/job/generation
ownership, synchronous final registration/claim, cancellation receipts, or
ordinary error processing are proposed. Busy still makes no result clone and
neither consumes a terminal claim nor changes the stored completed job. Stale UI
retrieval/cancellation does not free Rust allocations; only their actual
transfer/Drop does. Rejected cancellation of a completed record preserves later
retrieval. The table currently retains terminal records; response transfer does
not release the resident Value.

The source ledger names current and proposed owners separately. This return is
source review only. Wrapper/macro, emergency, cancellation, abandonment and
concurrency tests remain unrun; no native qualification follows. No new test,
runtime, build, UI/native interaction, host tool, install, source edit,
Git/index/API mutation or delegation occurred. Only this new packet was written;
startup `_01` remains unchanged. ROOT can route this bounded result to independent
review without accepting any allowance or protocol amendment.
