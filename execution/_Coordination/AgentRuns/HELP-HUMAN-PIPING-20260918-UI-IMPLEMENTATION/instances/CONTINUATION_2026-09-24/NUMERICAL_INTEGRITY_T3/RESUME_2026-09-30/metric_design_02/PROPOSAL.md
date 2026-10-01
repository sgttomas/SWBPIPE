# Two private H request facts — bounded design follow-up

**Conditional verification plan, no numeric binding.** The source-only result
is that the two private requests remain unknown; a narrowly attributed witness
could bind them without changing H's metric or prelaunch admission. It has not
been authorized or run. source12 is proposed source proof under independent
review, not an adopted complete startup theorem in this packet.

## Smallest recommended route

Use actual-type evidence if it is already preserved for the precise installed
standard-library artifact. Otherwise propose **one tiny standalone witness using
the unchanged existing K6Alloc and the public Mutex API**, with one earliest-main
startup snapshot and two fresh-mutex checks. No new allocator, event logger,
thread observer, guard, library patch, Thread probe or general runtime audit.
The startup part is eligible only after its exact attribution premises below
are independently established. If they cannot be established, return L_info
unbound; do not broaden the witness to explore runtime behavior.

Binding two type/request facts is different from transferring a measured process
baseline. Only M_mutex and L_info may enter the prelaunch source formula
`1028 + 2*M_mutex + L_info` (source12's conservative beta=1). Each is the byte
size of one named request from an identified compiled type. A startup current
value or residual without that attribution is not an admissible substitute.
The two mutex instances stay distinct allocations. The stdout1024 and name<=4
remain separate source facts. No reset, fixture, stage/prefix window, reported
metric, admission predicate/timing, or numerical claim changes.

## Direct source/layout route

The supplied sources establish where requests originate, but not their bytes:

- macOS selects sys/sync/mutex/pthread; `get` creates
  `Box::pin(pal::Mutex::new())` through OnceBox. pal::Mutex is a private default-
  representation struct containing UnsafeCell<libc::pthread_mutex_t>.
- The startup map is `BTreeMap<usize,ThreadInfo>`; its first insertion requests
  one actual `LeafNode<usize,ThreadInfo>` using Box::new_uninit_in. ThreadInfo
  and LeafNode both have default Rust representation. LeafNode contains11 keys
  and11 values plus parent/index/length fields. That list is not a layout proof.

A public size_of::<std::sync::Mutex<()>>() measures the outer wrapper, not its
private boxed pointee. A pthread C size alone does not authenticate the private
Rust wrapper. A mirror ThreadInfo or other BTree specialization is not the same
nominal type. Public Layout/size_of_val APIs cannot name or access these private
pointees through the existing stable interfaces. No numeric layout follows from
these source pages alone, and no copied repr(Rust) type is proposed.

An already-existing compiler layout record or debug-type entry is suitable only
if it resolves the **actual private type** and its instantiation from the exact
libstd artifact used by H, including target/cfg/data layout and type identity.
An already-existing code-generation record can instead expose the actual Layout
size supplied at the two named allocation call sites. It must distinguish the
leaf instantiation and pal mutex from unrelated constants or calls. In either
case bind the containing artifact hash and linkage to final H. Availability of
such records was not established here; no tool installation, private-symbol
exposure, std rebuild or disassembly programme is presumed. Stop after a bounded
lookup if ROOT elects this route and the records are absent/ambiguous.

## Public-Mutex witness: M_mutex

The newly consulted installed poison/mutex.rs proves the bridge:
Mutex<T> has inner sys::Mutex (:227-230); const new initializes it inline
(:350-352); lock calls self.inner.lock then makes the guard (:490-494);
guard drop unlocks (:741-747). The selected non-generic sys::Mutex::get uses
one pal::Mutex box. A fresh public Mutex<()> therefore reaches the same pointee
**provided** the witness and H resolve the same std crate/target/cfg/layout.
No payload clone, heap-owned protected value or worker thread is needed.

For a future approved witness, capture only stack scalars with existing
`current()` and `calls()`, and defer all printing/assertion diagnostics:

1. First-main startup snapshot occurs before this test (next section).
   Construct a fresh local Mutex<()>; new itself is inline initialization.
2. Immediately before first lock record C0,N0. Take a successful uncontended
   lock; while its guard and mutex remain alive record C1,N1.
3. Unlock and relock the **same instance**. The cached OnceBox must produce no
   new registered call or byte increase. Release the guard; then destroy that
   mutex and record C2,N2. Source shows the unlocked pointee is dropped, while
   a leaked locked guard could instead leak it and is forbidden here.
4. Candidate M=C1-C0 is usable only when M>0, N1-N0=1, no other registered
   owner changes in the interval, relock adds0, and destruction returns exactly
   to C0 without additional registered allocation. A second fresh instance
   should give the same positive request and lifecycle. This is corroboration,
   not a statistical search for a maximum.

All differences use checked arithmetic. Existing `calls` counts successful alloc,
alloc_zeroed **and realloc**; it does not count frees or identify call sites.
The source/compiled-path premise must therefore exclude other alloc/realloc/free
operations, lazy initialization, concurrent mutation and failure/panic paths in
these intervals. The successful pthread/poison path needs that small direct
check; source12 alone does not requalify every wrapper helper automatically.
Do not put printing, format!, test-framework bookkeeping, thread spawn, argv
parsing or even failure diagnostics inside the snapshots. Report failures after
capturing the data; never use assert_unchecked to assert an allocation happened.

## Earliest-main startup witness: L_info

There is no stable public API to create, reinitialize or expose the exact private
startup leaf. Do not invoke private init twice, alter signal handlers/guard pages,
or create threads merely to force a leaf. Ordinary unit tests are unsuitable:
the test harness has already allocated and may start threads.

A minimal standalone program can take C_entry,N_entry using the existing scalar
allocator accessors as its **first operations in Rust main**, before output,
argument collection or Mutex checks. Its ordinary Rust entry, linked std artifact,
startup signal/guard setup and allocator registration must match the source12
path. Source12 proves only an upper survivor roster; exact subtraction needs the
stronger following premises, which are **not established by this follow-up**:

A. Exactly the intended direct startup executes; no other registered allocations
   or frees/reallocations occur before the snapshot, including any constructor,
   instrumentation or wrapper activity. Dead startup allocations cannot simply
   be ignored when `calls` is used for attribution.
B. The stack-overflow registry branch runs (beta=1): not panic=immediate-abort;
   a guard range exists; the selected default signal-handler branch initializes
   it once; no prior registry entry or competing initializer exists. Merely
   observing nonzero current is not this proof. These conditions follow specific
   compiled startup/control-flow plus launch facts, not an arbitrary environment
   label such as macOS.
C. with_current_name selects the already-set main-thread identity and returns
   exactly the static string `main`, so its sole name request is4, not merely
   <=4. No preexisting differently named Thread handle may substitute.
D. The sole registered requests are that name4, one registry pal::Mutex of the
   positively bound M, and one leaf of the **actual private specialization**.
   All three survive to the snapshot; none was optimized away, merged, promoted,
   substituted or shrunk. Counters alone do not identify them.

Only with A-D and the fixed-artifact premise can the equation

```
N_entry = 3
C_entry = 4 + M_mutex + L_info
L_info = C_entry - 4 - M_mutex > 0
```

bind L_info. `peak==current==peak_move` at this point is a useful sanity check,
not proof of A-D. A positive residual with another call count is **unbound**;
zero/2 calls, a skipped branch, an unexpected owner or an ambiguous attribution
is not permission to choose beta=0 for the H bound. The conservative H source
formula must still allow beta=1. No observed612 value is used or expected.

If exact name length is proved only bounded0..4 while the registry mutex and
leaf are otherwise exactly attributable, `C_entry-M` is an upper bound on that
one leaf, not its exact request. This possible conservative variant must be
explicitly labeled and independently justified; the recommended plan seeks exact
facts and does not substitute this residual now. Unknown unrelated survivors
or optimizer elision defeat even that inference.

## Optimization and transfer: explicit hard gates

Pinned core/alloc/global.rs:102-117 says allocations may be eliminated/moved and
that allocation occurrence is not program behavior even with a counting
allocator. Safe measurements can report what happened, but source syntax,
black_box, repeat agreement or choosing debug mode cannot guarantee that the
required calls happened. A zero/missing call leaves the request unbound.

Positive counter deltas are usable only with actual call-site attribution.
Where source alone cannot exclude transformation/other calls, retain an existing
compiled request-size/call-site record or obtain a separately authorized bounded
inspection of just those paths. If no such evidence is available, stop with
that attribution gap; do not design logging into the allocator. This matters
especially for the pre-main aggregate, where there is no boundary around each
private operation. Runtime checks are corroboration, not a way around this gate.

Pin rustc1.97.1 full commit8bab26f4f68e0e26f0bb7960be334d5b520ea452,
aarch64-apple-darwin, actual libstd/alloc/core artifact identities and linkage,
relevant cfg/panic/target/layout settings and optimization/LTO assumptions.
The layout/request correspondence must be to final H, not merely another binary
with the same version string. The allocator source must be byte-identical to H's
existing registered K6Alloc, not a substitute System/malloc usable-size query.
Observation does not bind alignment; retain it separately if a future formula
requires alignment rather than just requested byte sizes.

Once those premises hold, the output facts concern fixed request sizes from
identified source sites and may be used *before* later H runs. They are not
assertions that every H startup must allocate the same aggregate. Source12's
beta<=1 and at-most-two distinct mutexes account for conditional occurrence;
omission reduces the counted source upper. Optimization's reduction of actual
requests cannot repair missing source facts or license an underestimated request.

## Bounded future verification plan and authority

No runtime authority is present in this turn. Suggested future grant is limited
to these two request facts after source12's independent review:

- Verify the sealed source/target basis and check once for existing exact-type
  layout/request records. If both facts are directly proved, no runtime witness
  is needed. A missing private record is an explicit result.
- Before authorizing an allocation witness, dispose A-D and the optimization/
  linkage premises, with a precise accepted basis. If these cannot be met by the
  named evidence, return L_info open; a Mutex-only result may still be useful.
- At most one minimal standalone witness build, followed by one controlled run
  containing two fresh Mutex checks and one entry snapshot. One further cold run
  may check repeat agreement if ROOT chooses it, but cannot close attribution.
  Use the existing host guard/build rules and an explicitly granted slot/target;
  no solver, library rebuild, maintained edit, Thread probe or installation.
- Retain source, binary/compiler/library hashes, exact commands/options, raw
  scalar observations, expected call/lifetime equations and failures. Have an
  independent checker validate attribution/transfer before integrating any facts.
- On mismatch, unavailable compiler/artifact, elision, additional calls or a
  skipped startup branch, stop. Return the exact unresolved predicate; no sweep,
  repeated tool-repair lane, arbitrary allowance or admission adjustment.

This route can remain wholly inside the existing requested-heap/prelaunch
contract as build-bound request evidence. ROOT's explicit permission to run the
witness is still necessary because the present grant forbids runtime work.
If no actual-type artifact or attributable witness can bind L_info, the owner-
held choice is whether to authorize another *specifically described* evidence
source, or retain this unresolved prerequisite. This proposal offers no weaker
metric or domain as a way to close it.
