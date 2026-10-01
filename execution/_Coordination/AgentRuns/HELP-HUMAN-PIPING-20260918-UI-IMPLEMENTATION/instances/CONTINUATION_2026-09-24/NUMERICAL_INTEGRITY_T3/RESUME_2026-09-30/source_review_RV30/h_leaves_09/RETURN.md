# RV30 h_leaves_09 — source12 independent four-leaf review

**H-R2 and H-F0 check out for the stated paths. The startup/name/population arithmetic is consistent, but one narrow source-evidence gap remains in the synchronization wrapper chain.** No concrete numeric undercount was demonstrated. Do not yet describe the only remaining premises as the two private request values; retain the forwarding/surviving-child warrant below as well. No complete E_max or admission acceptance follows.

Independent TASK Type2 RV30, native child `/root/rv30_k6c_kernel` of ROOT `/root` HELP_HUMAN Agent0; no delegation. Start2026-10-01 15:23:57 UTC; deadline15:43:57 UTC. Completion after checks is in VERIFICATION.json. The parent granted review of the sealed supplied sources only. I did not read another designer's proposal, open new library pages, run a probe or repair source. Instruction/skill origins remain at ../kernel_01/INSTRUCTION_BINDING.json.

Source12 seal `024339a4e87285bb001c0fe44fe3a4051c0e89165cf0cd38b54d01a2dd42f87c` and manager seal `b16bd3abd8a844a0593283e885448f4f73873f5ddcdfe7dee94d9deec005da9a` verify. All47 child payloads and all40 original installed HTML/decoded source pairs independently match. The three named product inputs match frozen40129 hashes. H_LEAVES hash is63ce58a4b2524c072ebb218a169216c6b4339e802433dad8c017f3d9a8bb3f8e.

## RV30-H12-F1 — P2, missing verification: preserve the two public synchronization facade links

**Location:** H_LEAVES:26,90-102,111-122. This is a source-evidence gap, not evidence that the library actually allocates an omitted object.

The supplied thread_info.rs:30,40,117 declares and locks **crate::sync::Mutex<()>**. The supplied mutex pages define **crate::sys::sync::Mutex** and its pal::Mutex pointee. Stdout's ReentrantLock uses sys::Mutex directly, but the registry's public Mutex wrapper lies between LOCK and that backend. Its representation and new/lock/unlock forwarding are not among the40 supplied pages. Therefore the packet alone has not traced the claim that registry locking adds the same single M_mutex request and no additional surviving wrapper child.

Likewise once_lock.rs:7,109-111 imports/contains **crate::sync::Once** and :542 calls call_once_force. The packet supplies the sys::sync::once selector/queue backend, but not the public Once forwarding layer. Its statement that completed stdout OnceLock initialization leaves no further registered child needs that small wrapper warrant.

**Effect:** the symbolic reduction to1028+2*M_mutex+L_info remains conditional on those forwarding/no-surviving-child facts. H-R2's already-initialized writer result and H-F0's numeric conversion result are unaffected. This does not justify inventing a third allocation or a numeric allowance.

**Minimum repair:** bind the two selected public wrapper source paths and show that their inline fields and reached forwarding paths introduce no additional registered allocation surviving the first H reset, with registry Mutex using the same selected sys::Mutex backend. This is a narrow source check, not a new private-layout measurement, runtime experiment, generic synchronization audit or inferred C/Rust layout. No missing wrapper page was opened in this review, as instructed. MUTEX_BRIDGE_GAP.json records both links.

## Checked H-R1 identities and counts

macOS does not match the supplied futex alternatives and reaches the Unix pthread selector (sys/sync/mutex/mod.rs:1-25). pthread::Mutex contains OnceBox<pal::Mutex>. Its get at19-27 constructs Box::pin(pal::Mutex::new()), calls init once, and OnceBox at47-72 retains the resulting pointer. Box::pin's actual default allocation is for the pal pointee; Pin/Box/OnceBox handles are not additional allocated wrappers.

On the specified single-main-thread/no-competing-initializer path, one OnceBox cannot accumulate losing-race allocations or one new box per lock. Later locks use the cached pointer. H's start.emit locks stdout before its first stage reset. The initialized stdout buffer and mutex are distinct persistent owners.

Thus M_stdout=M_mutex is source-supported for this selected ReentrantLock backend. The registry is a distinct identity, never an alias of stdout. The same-byte M_stack equality is the intended result once F1's public Mutex bridge is supplied. Neither private pointee size nor alignment is inferred from its one UnsafeCell<libc::pthread_mutex_t> field. Source elision could reduce a count; the proof is an upper, not a promise that an allocation call must occur.

The provided queue Once backend uses stack closure/result/state/waiter-queue data on the uncontended call path. F1 retains the missing public Once bridge; there is no proposal here to measure its wrapper.

## H-R2: initialized byte-slice write/flush adds no registered helper heap

I traced the supplied exact chain from H Line::emit's borrowed byte slice and dropped Results through StdoutLock, LineWriter, LineWriterShim, BufWriter, StdoutRaw, Unix Stdout and FileDesc.

LineWriterShim's newline search is core::slice::memchr; its splits and tail are borrowed slices. BufWriter copies only within the existing spare capacity, or writes directly. Flush uses a stack guard and in-place drain/truncate. There is no buffer reserve/grow or copied owned slice on this chain. The retained1024-byte stdout buffer remains counted separately.

Unix FileDesc::write calls libc::write with a borrowed pointer/length. The default write_all loop advances that slice, retries Interrupted and returns the error; raw flush returns Ok. EBADF handling inspects the error code and may replace it with an inline success. H does not format either Result.

For64-bit non-UEFI targets, the supplied io::Error path selects repr_bitpacked. OS errors encode a scalar code; WriteZero constants refer to static SimpleMessage. Their Drop dispatch does not execute the custom Box branch. Therefore successful writes, short/Interrupted retries, OS errors and static WriteZero errors add no registered helper allocation.

The source conclusion `T_stdio_emit=0; O_stdio_emit=0` is warranted **after stdout initialization and beyond its existing owners**, for this exact borrowed-byte/drop-result path. It excludes first-init allocations, arbitrary writers, custom io::Error::new payloads, panic handling and OS/System memory. It is not a success-only assumption.

## H-F0: shortest finite-f64 path and32788 bound

The supplied core/fmt/float.rs routes plain Debug without precision to shortest fixed or scientific output. The H format has no dynamic width/precision/alternate request. Fixed formatting uses min_precision1. Both paths use the caller-owned17-byte digit array and fixed4/6 Part arrays. Grisu's fallback Dragon uses inline Big32x40 storage; its clones copy fixed arrays rather than Vec/Box storage. Formatter streams signs, static zero runs and borrowed digits into the existing String. Part::Num has a stack5-byte array.

The checked registered conversion-helper result is therefore
`T_Debug_float=0; O_Debug_float=0`.
This does not claim zero destination-String allocation or a measured stack-frame size.

Independent integer derivation:
- sign<=1, digit count1..17;
- fixed e<=0:1+2+32768+17=32788; with frac_digits1 and k>=1 the additional fractional-zero branch is absent;
- fixed0<e<k: at most1+17+1=19;
- fixed e>=k: at most1+32767+1+1=32770;
- scientific:1+17+1+2+5=26; the i16 exponent adjustment is widened before subtraction and Part::Num is u16;
- signed zero<=4.

Hence32788 is a valid deliberately loose integer-type overapproximation for the stated finite default-Debug route. It is not an expected rendered length, a general precision/width bound or an observed allowance. The mechanism candidate is finite under the already reviewed original_rigid_witness guard; nonfinite formatting need not be assumed as an input enlargement.

All downstream source12 arithmetic independently matches:
D_ErrorSolve196808; ErrString retained393616;
escaped stage-line length1181161;
stage emit retained2362400;
outer-end extra2756016;
outer Line active-old1181161;
earlier ErrFmt active-old196808.

These retain the error String and escaped Line as separate owners and add only one current old realloc buffer. Successive construction/emission phases remain maxima, not a summed history. The known no-error708 branch is unchanged. Outcome, model, saved attempts, prefix list, stdout and private requests are outside these displayed extras.

## H-R0 beta, name and one-entry qualifications

The supplied direct entry path sets the main ThreadId before selected Unix init. main_thread storage is a static atomic; current_id caching does not create a Thread handle. The earlier registered-allocator exclusion for Thread/TLS infrastructure remains distinct from explicit Box/tree allocations in stack-overflow metadata.

On the stated non-unwinding direct entry:
- standard-fd repair, SIGPIPE setup, page-size lookup and Apple OS thread naming use scalar/static/stack arguments to libc;
- alternate stack/guard mappings use mmap/mprotect/sigaltstack and are outside K6Alloc's registered requests;
- Apple args init is empty; later parsed Args Strings are the already modeled separate caller owners;
- ordinary entry closure/catch-unwind/backtrace wrappers contain stack state and add no persistent owner on this non-panicking path.

The named exception is thread_info metadata. init at stack_overflow.rs:152-190 processes SIGSEGV/SIGBUS, but NEED_ALTSTACK gates setup once and guard_page_range.take consumes the sole range. make_handler(true):250-259 deliberately skips the non-main set_current_info route. Thus the direct startup cannot insert twice through those two branches.

At most one key is inserted into the initially empty BTreeMap<usize,ThreadInfo>. This is one leaf, not an internal node or an arbitrary-population tree. The source02 insertion contract applies; the private value specialization is different from every kernel/JSON tree already discussed.

with_current_name reads a preexisting handle's name if any, otherwise returns borrowed "main" when current/main IDs agree. In the specified direct startup without foreign initializers, no supplied earlier path creates a differently named Thread handle. The explicit Box::from name copy therefore has length at most4 (or no child). It requests the str payload bytes, not a fat-pointer/String wrapper size. Arbitrarily named pre-main handles or extra Rust threads are outside that stated source path and must not silently inherit this upper.

beta in{0,1} represents the guard/default-handler branch; choosing beta=1 is a conservative prelaunch substitution, not a measured observation. The leaf and name persist across H resets. Cleanup is after main; deletion cannot reduce an earlier sample, and the static mutex child survives entry removal independently. If the registry was not initialized before main, a cleanup-only lock is outside the completed H windows; it does not add a hidden stage owner.

## Conditional composition and minimum remaining premises

The distinct-owner expression is arithmetically correct:
`1024 + M_stdout + beta*(M_stack + L_info +4)`.
With beta=1 and F1's forwarding premise, equality of request-size parameters gives
`1028+2*M_mutex+L_info`.
The two mutex allocations remain distinct. Do not add source10's old R_live placeholder again for the same identities.

The minimum remaining **numeric** request facts are still:
1. M_mutex, the actual selected private pal::Mutex allocation request;
2. L_info, the actual first leaf request for BTreeMap<usize,ThreadInfo>.

Direct request sizes are sufficient for these byte terms; no private alignment value or inferred pthread representation is required to multiply a request count. Their means of binding belong to the separate authorized design work, which I have not read or influenced.

Additional nonnumeric premises are F1's two public facade links; preservation of direct single-main-thread/no-competing-initializer, no foreign named-handle path and completed-stage/nonpanic scope; actual compiler/stdlib/target and final-A1 reconciliation; H-I0 planned input/path/pass data; and H-C0 checked implementation/full integration. No new private-layout cells or blanket runtime study are requested by this review.

No value was substituted for M_mutex or L_info, and no612 baseline was used. No designer proposal, new library page, Ruby/Rust/build/test/probe/runtime/model/solver, source repair, tool development, Git/index operation, network/install or delegation occurred. Only the additive h_leaves_09 evidence subtree was written; all prior seals remain unchanged.
