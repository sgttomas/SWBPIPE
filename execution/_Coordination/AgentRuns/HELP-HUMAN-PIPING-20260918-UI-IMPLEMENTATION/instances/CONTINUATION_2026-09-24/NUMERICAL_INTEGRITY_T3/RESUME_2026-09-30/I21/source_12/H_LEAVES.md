# Source12 H leaves — conditional source result

TASK I21; actual start2026-10-01 15:02:46 UTC; deadline15:27:46 UTC. Four leaves only. Frozen product40129; installed Rust1.97.1/aarch64-apple-darwin source documentation. RV30 h_caller_05 N1/N2 are additive adopted inputs; source10 and all earlier seals remain immutable. No Rust/product/source execution or host job.

## Finite findings (detailed proofs and final statuses below)

| Leaf | Source identity and current result | Exact remaining premise |
|---|---|---|
| H-R1 stdout mutex | std/sys/sync/mutex/mod.rs selects pthread for macOS Unix. pthread.rs:19-27 installs one Box::pin(pal::Mutex) through OnceBox; once_box.rs:47-72 retains the pointer. Stdout's wrapper is static; its child is a distinct persistent registered allocation initialized before the first stage. | The exact requested size_of::<std::sys::pal::sync::Mutex>() is private and unmeasured. No copied struct, inferred private layout, Thread probe or observed baseline substitutes for it. Source pal/unix/sync/mutex.rs:7-9 has one UnsafeCell<libc::pthread_mutex_t> field but repr(Rust) does not itself authenticate a numeric request here. |
| H-R2 byte write/flush | Direct chain and dropped OS/static-error destructor are source-bound below. Existing buffer never grows. Extra registered scratch and active-old are both0. | Independent review/final basis only for this exact initialized byte-slice path; no general error formatting or writer assertion. |
| H-F0 six finite default-Debug f64 leaves | Actual shortest conversion uses stack digit/Part/Big storage and streams into ErrString. D64<=32788 follows a deliberate finite i16-exponent overapproximation; extra registered helper scratch/active-old are0. | Only the existing destination String growth remains. Geometry40 and wrapped nonfloating100 stay RV30 N1. The bound is conservative source arithmetic, not typical rendered length. |
| H-R0 startup survivors | Direct entry adds at most one ThreadInfo tree leaf, one registry mutex child and4 name bytes; selected stdout has its distinct mutex child. These survive all stage resets and do not grow there. | Exact private ThreadInfo/tree and pal::Mutex request bytes remain. N2 excludes private Thread-handle/TLS infrastructure; it does not exclude explicit Box/tree owners in stack-overflow setup. |

The bound is conditional until exact retained registered owners and their requests are bound. Every new owner will be counted once, with initialization before the first H reset distinguished from later reachable growth. No612, guessed constant, safety factor or full E_max/admission acceptance.

LIBRARY_SOURCE.json preserves original HTML hashes plus the one-off decoded-source transformation and exact decoded hashes. The decoded files are evidence only; no reusable extractor or library audit is introduced.

## H-R1 — selected mutex request and lifetime

Installed std/sys/sync/mutex/mod.rs:1-25 excludes macOS from the futex alternatives and selects the Unix pthread implementation. Its Mutex holds OnceBox<pal::Mutex> (pthread.rs:8-9). get():19-27 uses Box::pin(pal::Mutex::new()), initializes it, then retains it. OnceBox:47-52 uses an existing pointer when initialized; :62-72 stores the new pointer. A losing initialization race can discard a new box, but the authorized single main-thread initialization has no competing initializer. No per-emission multiplication is justified.

alloc/boxed.rs:247-251,284-290,355-356 explicitly routes Box::pin through Global.allocate(T::LAYOUT). Define M_mutex as this actual pal::Mutex request size. pal/unix/sync/mutex.rs:7-9 identifies the private pointee as a struct containing UnsafeCell<libc::pthread_mutex_t>. Neither that repr(Rust) source nor a copied C struct establishes its numeric size/alignment; installed rustdoc has no libc source page in its source inventory. This is the precise unresolved private-layout cell; no numeric request is inferred.

For stdout, std/io/stdio.rs:645,717-721 is a static OnceLock/ReentrantLock/RefCell/LineWriter. ReentrantLock:284-299 uses current_id and reaches its mutex on the first lock. H main.rs:134-139 writes and flushes the start line at:558, before model/stages. Thus R_stdout_mutex <= M_mutex is retained at the first reset and all later samples. Static wrappers/guards are not additional heap allocations. Cached main current_id has no fresh Thread handle (RV30 N2); core/alloc/global.rs:119-133 separately excludes the named thread/TLS infrastructure from registered allocation.

The same M_mutex request describes the distinct mutex used by startup ThreadInfo. Count two identities when that registry is initialized, not one alias and not one per stage. Uncontended OnceLock initialization uses stack res/closure/OnceState/WaiterQueue (once_lock.rs:533-556; sys/sync/once/queue.rs:189-227); the selected backend is queue (once/mod.rs:26-34). It adds no persistent heap child on this path.

After initialization these mutexes do not resize. The stdout buffer still contributes the independently reviewed1024 bytes exactly once. These are retained request terms; they add no active-old growth during the measured stages.

All source allocation counts here are upper bounds on the stated direct path, not a claim that compiler optimization must preserve a counted call. core/alloc/global.rs:102-117 permits elision; elision can reduce these requests. No actual allocation count or private size is inferred from these source sites.

## H-R2 — reached dropped-result write/flush closes at zero extra request

The actual H edge is main.rs:134-139: out.write_all(self.0.as_bytes()) and out.flush(), both assigned to let _ and dropped. The existing Line String is caller-owned and already counted. No Display/Debug conversion of io::Error is reached.

The direct chain is:
- std/io/stdio.rs:846-864 StdoutLock borrows the existing LineWriter.
- buffered/linewriter.rs:191-209 calls LineWriterShim::write_all and existing BufWriter::flush.
- linewritershim.rs:268-295 splits the same borrowed slice at the last newline. It either writes completed lines directly or uses the existing buffer, then handles the borrowed tail. No owned copied slice is created.
- bufwriter.rs:195-264 flushes through a stack BufGuard and drains by copy_within/truncate; :414-446/:548-560 either copies within spare capacity or writes directly. It never grows the1024 buffer. :656-658 flushes it then calls the raw writer.
- StdoutRaw stdio.rs:140-159 forwards with handle_ebadf:203-207; sys/stdio/unix.rs:47-66 creates stack ManuallyDrop<FileDesc>, calls its write and returns Ok(()) on flush.
- sys/fd/unix.rs:344-352 calls libc::write on the borrowed bytes and wraps the scalar result via cvt. sys/pal/unix/mod.rs:242-243 and sys/io/error/unix.rs:30-42 read Apple __error's scalar errno.
- Default io::Write::write_all (io/mod.rs:1858-1869) loops over slices, retrying Interrupted. The zero-write error is static WRITE_ALL_EOF; BufWriter's zero-write error is another const_error! static. Error::is_interrupted:692-698 and sys/io/error/unix.rs:97-98 only inspect tags/codes.

On the selected64-bit target, io/error.rs:16-19 selects repr_bitpacked. from_raw_os_error:364-365 creates new_os; repr_bitpacked.rs:166-180 stores scalar tagged bits. const_error! (error.rs:187-190), from_static_message:304-305 and repr_bitpacked:202-204 refer to static message storage. Drop:228-235 calls decode_repr; the OS and static-message branches (:249-252,268-270) do not invoke the custom-Box branch. Thus returned/dropped errors on this chain have no registered children.

Conclusion for the already initialized stdout and the reached byte-slice API:
  T_stdio_emit = 0; O_stdio_emit = 0.
This means extra registered requests beyond the existing caller Line and persistent stdout owners. It does not assign zero to OS storage, arbitrary io::Error::new/custom errors, panic reporting, arbitrary writers, or a first initialization inside a differently ordered program. H's dropped OS/WriteZero Results are included, so this is not a success-only I/O assumption.

## H-F0 — finite default Debug bound and zero extra numeric heap

RV30 N1's40-byte geometry leaf and100-byte nonfloating wrapped bound are reused without a new geometry audit. Only six finite rigid-parameter f64 Debug leaves remain in the mechanism refusal.

core/fmt/float.rs:182-212 routes default Debug (no width/precision/sign flags) to shortest scientific or shortest fixed formatting, with fixed min_precision=1. :61-82 and:137-160 create a17-byte MaybeUninit digit array and4 or6 Part slots on the stack. flt2dec/mod.rs:138-143 fixes MAX_SIG_DIGITS=17; Grisu:456-466 returns a slice of that buffer or uses Dragon fallback. Dragon imports Big32x40 (:12), whose source is a fixed40-digit array plus scalar (bignum.rs:68-94,388); its clones copy inline storage. Dragon:260 returns the supplied buffer's slice. The source-backed statement is zero registered conversion-scratch requests; no exact stack-size measurement is needed.

Use the following deliberately loose but fully finite output bound, derived from the actual integer types rather than an unproved typical display length:
- sign is at most1 byte (flt2dec/mod.rs:316-335).
- shortest digit slice length k is1..17.
- fixed decimal conversion accepts an i16 exponent e and frac_digits=1 (:177-241). For e<=0, at most sign1 + "0."2 + (-e)<=32768 zeros + k<=17, hence32788. Its optional trailing-zero part is absent because k>=1. For0<e<k, length<=19. For e>=k, length<=1+32767+1+1=32770. Zero has length<=4.
- scientific conversion (:258-304) has at most sign1 +17 digits + decimal point1 + "e-"2 +5 exponent digits (Part::Num is u16), hence26.
Therefore D64_Debug <=32788. This uses an explicit finite exponent-type overapproximation; it neither admits arbitrary formatter width/precision nor treats32788 as a measured or expected string length. The actual magnitude switch could tighten it but is unnecessary for this bounded leaf proof.

Formatter::pad_formatted_parts (core/fmt/mod.rs:1981-1985) takes the width-zero path. write_formatted_parts:2031-2074 streams sign/static zero chunks/borrowed digits into the existing destination; Part::Num uses a stack [u8;5]. No auxiliary String or Vec is created by these helpers. So:
  T_Debug_float = 0; O_Debug_float = 0,
where the existing ErrString's own retained F(D) and active old request remain separately charged. Six calls are sequential; no sixfold scratch owner is manufactured.

Using RV30's narrowed grammar:
  D_ErrorSolve <= max(80 +6*32788,100) =196808.
  ErrString <= F(196808)=393616.
  L_stage_error <=313+6*196808=1181161.
  StageEmit_R <= F(1181161)+78=2362400.
  Outer_end extra beyond Outcome and persistent caller =393616+2362400=2756016.
  Outer_end active-old extra <=max(1181161,39,6)=1181161.
  Earlier ErrFmt active-old extra <=196808.

These constants are conservative source arithmetic for that error branch. The known no-error708 branch is unchanged. None includes the model, saved attempts, prefix list, Outcome/kernel, stdout buffer or unresolved mutex/tree requests. Use phase maxima, not a sum of successive error-construction and emission phases.

## H-R0 — direct startup survivor roster

For the direct Rust entry path on the selected main thread, rt.rs:111-119 records main_thread::set(current_id()) then calls selected Unix init. main_thread.rs:14-30 uses a static atomic ID. thread/current.rs:17-39,164-175 caches it; repeated ReentrantLock calls use the same ID. RV30 N2's registered-allocator exclusions remain in force.

Unix init (sys/pal/unix/mod.rs:25-51) calls only the named standard-fd sanitization, signal setting, stack-overflow setup, Apple args init and main-thread OS naming. The successful finite path has:
- fd reopening through scalar fcntl/open/dup and static "/dev/null" (:54-76,126-141): descriptor/OS storage, not a Rust Global owner.
- SIGPIPE setup via static/scalar flags and signal (:145-187): no registered retained child.
- Apple args init is empty (sys/args/unix.rs:165-172); later argv copy/drop is source10's already reviewed C_Args, not a new runtime vector.
- OS main naming uses a fixed stack C-char array (sys/thread/unix.rs:395-400,453-460) and pthread_setname_np, not a retained Rust name String.
- Page-size query is scalar libc::sysconf (sys/pal/unix/conf.rs:4-7). Main guard and alternate stack use mmap/mprotect/sigaltstack (stack_overflow.rs:205-244,518-549), outside K6Alloc's registered requests. Their OS memory is not converted into a heap allowance.

The concrete registered exception is stack-overflow metadata. stack_overflow.rs:152-190 installs at most one main entry: NEED_ALTSTACK gates the first default SIGSEGV/SIGBUS handler, and guard_page_range.take() prevents a second entry. It calls thread_info::set_current_info only if a guard range exists. thread_info.rs:34-45 declares:
  ThreadInfo { tid:u64, name:Option<Box<str>>, guard_page_range:Range<usize> },
  static LOCK:Mutex<()>,
  static THREAD_INFO:BTreeMap<usize,ThreadInfo>.
set_current_info:112-122 obtains a scalar OS ID, clones the current name, locks and inserts one key (errno address). thread/current.rs:212-235 uses cached/main IDs to provide the borrowed static "main" on this direct startup path; it does not create a Thread handle. Apple current_os_id is a scalar pthread_threadid_np call (sys/thread/unix.rs:370-379).

The cloned name is a distinct Box<str>, at most4 requested bytes. alloc/boxed/convert.rs:124-138 invokes Box::clone_from_ref; boxed.rs:792-794 uses Global, and:865-889 allocates Layout::for_value of the str once then copies it. It is not a16/24-byte wrapper allocation. A missing/unnamed current handle reduces the child to none; no arbitrary name is introduced by the direct startup source.

Define beta in{0,1} as whether that startup registry branch initializes. For prelaunch conservative source accounting, take beta=1 instead of observing it. Define L_info as the actual single leaf request for BTreeMap<usize,ThreadInfo>. Its key/value specialization is new; do not reuse a kernel or JSON-tree size. The source02 BTree insertion contract gives one leaf for the first insertion, no internal node. ThreadInfo's private repr(Rust) size/alignment and hence L_info remain unbound; no mirror type is measured.

The direct startup registered survivor envelope is:
  R_stack <= beta*(M_mutex + L_info +4).
The first mutex is the registry's own identity; R_stdout_mutex is a different one. The map's static header and spin-lock scalar are not heap children. No other thread entries are added on the authorized finite main-thread path. These initialized owners persist through all H resets. cleanup calls drop_handler after main, which removes the entry (stack_overflow.rs:196-202,284-304; thread_info.rs:125-132); cleanup cannot be subtracted from earlier samples. The static mutex child survives independently of entry removal.

Normal entry wrappers do not introduce another persistent Rust owner: rt.rs:171-206 borrows/calls the main closure, panicking.rs:500-548,575-582 stores closure/return in a stack union on the non-unwinding path, and sys/backtrace.rs:162-171 calls f() then black_box(()). This is a statement about the selected successful entry path, not arbitrary panic handlers, failed startup, foreign initializers or unrelated standard-library APIs. A startup failure that never reaches a reset supplies no completed H stage; it does not justify changing numerical fixture/refusal domains.

Within these four traced leaves, no later registered initialization/growth remains after start.emit and entry setup: mutexes use cached OnceBox pointers; stdout keeps fixed1024 capacity; ordinary write/flush adds zero; f64 writes grow only the already modeled destination. Thus their separate late-runtime active-old contribution is zero. This does not erase any independently unresolved caller/final-A1 obligation.

## Conditional composition and exact residuals

Replace source10's broad runtime placeholder for the traced direct path with distinct identities:
  C = C_model + C_Args +1024 + M_stdout
      + beta*(M_stack + L_info +4),
  M_stdout=M_stack=M_mutex as request-size parameters, but two allocations.
A valid conservative source upper takes beta=1, giving1028+2*M_mutex+L_info for these persistent library children. It is a symbolic source formula, not a caller-selected arbitrary constant. Never add the old R_live placeholder on top of the same identities.

H_source/H_solve/H_prefix keep source10's existing saved-attempt/prefix/kernel/outcome unions and its inner-versus-outer sample edges. Substitute the H-R2 and H-F0 results above; include only the currently growing ErrString/Line/integer/control buffer in movement. Source error75, geometry40 and nonfloating wrapped100 remain reviewed prior facts. H's33 models, repeats<=5 and all stage/refusal/prefix paths are preserved.

Remaining minimum cells:
1. M_mutex: exact actual private pal::Mutex request on the authenticated target; installed external libc type source is unavailable, and private Rust wrapper layout is not inferred.
2. L_info: exact actual BTree leaf request for keyusize,valueThreadInfo, including the private value's size/alignment. One-node population/name4/lifetime are source-bound; its bytes are not.
3. Source/target reconciliation: final A1 product basis and actual installed compiler/stdlib correspondence remain the existing acceptance dependency; this packet binds installed documentation/version/hashes and existing reviewed compiled basis, not a new build.
4. H-I0 actual planned model/pass/path byte descriptors and H-C0 checked implementation/review remain unchanged. No numeric prelaunch E_max or admission follows merely from resolving these four source leaves.

No arbitrary system baseline,612, safety factor, Thread-layout probe, new allocator or observer is used. No experiment is running. This is an additive source proposal for ROOT/RV30 review.
