# RV30 h_request_bindings_10 — source13 and two existing native request sites

**Close RV30-H12-F1. The two proposed request facts are usable for the exact reviewed layout08 executable: M_mutex = 64 bytes with allocation-alignment argument 8, and the first registry leaf L_info = 544 bytes with allocation-alignment argument 8.** These are two separate conclusions. They do not establish correspondence to final H, a complete E_max, admission or implementation acceptance.

TASK Type2 RV30 /root/rv30_k6c_kernel, direct native child of ROOT /root HELP_HUMAN Agent0; no descendants. Actual start 2026-10-01 15:45:00 UTC; fixed deadline 16:05:00 UTC. Completion and seal follow in VERIFICATION.json. The supplied grant and carried instruction hashes are in GRANT.json. Product basis remains 40129a225d73860ac2a53da9a2fa73869df668f3.

## 1. Source13 closes the two missing public facade links

All 12 source13 payloads match seal ec5754a160d6b8ccd1e30aa0091d4eb150ba6ccca974e8c04755f623a601b2d0. All 3 manager payloads match seal 22b2513e3c2a7eef31f17c0f83fb9e493b2d4f2c3a0226b3e30020ccf6ddd76c. I independently decoded all six original installed HTML pages using HTMLParser, skipping line anchors, and matched the supplied decoded bytes. All eight retained source12 helper originals/decodes also match. Source12's 47 sealed payloads remain intact.

The exact public Mutex path is sync/mod.rs:235–240 -> poison.rs:68 -> poison/mutex.rs. Mutex<()> contains only inline sys::Mutex, poison::Flag and unit UnsafeCell data (mutex.rs:227–230); new at350–351 initializes those fields. lock at490–494 calls inner.lock before the inline borrowed guard is made. MutexGuard at277–280 contains a borrowed Mutex and inline poison::Guard; Drop at741–747 calls poison.done and inner.unlock. sys/sync/mod.rs:9 re-exports the selected backend; the macOS Unix selector reaches pthread::Mutex. Consequently the registry wrapper adds no Box or second wrapper-owned lazy child beyond its existing OnceBox<pal::Mutex> allocation.

ThreadInfo's LOCK.lock() Result is stored without unwrap or formatting (thread_info.rs:117,127). PoisonError<T> owns T inline; Flag/Guard are scalar atomic/bool state, and map_result moves the same guard through Ok or Err. Dropping either Result releases the held guard and backend lock. It does not drop the static LOCK or its cached pal child. In non-unwind mode Flag::get is false, so PoisonError::new's abort-mode panic is not reached from this lock path. In unwind mode the panic-status read is a static atomic / const-initialized TLS Cell read; thread::panicking forwards directly. The supplied core GlobalAlloc contract excludes registered allocation by TLS infrastructure. No error string or heap wrapper was omitted on this facade path.

The exact public Once path is sync/mod.rs:193 -> sync/once.rs. Once contains only sys::Once (35–37); new at83–84 forwards. call_once_force at216–226 either returns on the completed test or passes a borrowed adapter closure around a stack Some(f) into inner.call(true,...). OnceLock at109–113 contains that public Once; initialize at542 reaches call_once_force. The selected Unix queue backend receives ignore_poisoning=true, skips its existing-poison panic branch, invokes the callback once in the initialization branch, then returns. Thus take().unwrap() is not a new reached panic allocation on this path. No callback Box, heap-bearing public Once child or public wrapper destructor adds a request.

The repaired consequence is exactly zero additional retained/transient/active-old registered bytes from these facade layers. The already modeled callback owners remain counted. Backend pthread failures, arbitrary recursive/concurrent callbacks, general panic machinery and foreign initialization are not proved allocation-free by this closure; their prior scope qualifications remain. H-R2's initialized writer path and H-F0's float path are unchanged.

The source-side distinct-owner expression remains:

    C_library = 1024 + M_stdout + beta*(M_stack + L_info + 4), beta in {0,1}
    conservative = 1028 + 2*M_mutex + L_info

M_stdout and M_stack are distinct identities with the same selected private request parameter. This closes the prior forwarding/no-surviving-child evidence gap without inventing or measuring public wrapper layout.

## 2. Two direct allocation arguments in the unchanged executable

All 12 I23 payloads match seal 7c39a134ab9be73bdfb778f7982f50a77ec59299b431a011bf1d1ed08759f186. I independently hashed the 469520-byte executable at WT/k6c-layout08-target/aarch64-apple-darwin/release/examples/i21_kernel_layout: 63ea631507802f2a0ec939299b5dba2b7bdfb08728cb432be45d20c9e6938607.

I ran the existing /usr/bin/objdump only over the two exact already-selected function ranges (228 and 3296 code bytes). Both commands exited 0 with empty stderr, and their normalized complete output is byte-identical to I23's sealed output. EXECUTABLE_RECHECK.json records argv and raw/normalized hashes; the local mutex_callsite.txt and registry_callsite.txt retain the matched output. No executable was launched, no third function was disassembled and no libstd archive reader was retried.

| Exact site | Argument/setup and attribution | Narrow usable fact |
|---|---|---|
| 0x100039058 in nominal OnceBox< std::sys::pal::unix::sync::mutex::Mutex >::initialize< pthread::Mutex::get closure > | w0=0x40 at39050; w1=8 at39054; bl __rust_alloc at39058. The returned pointer is populated, passed to nominal pal::Mutex::init at39074 and compared/stored into OnceBox at3907c. Losing-race deallocation at3909c separately passes the pointer,64,8. | This compiled pal pointee allocation requests64 bytes/alignment8. Public Mutex size, libc field sums and a mirror type were not used. |
| 0x100023ee8 in nominal std::sys::pal::unix::stack_overflow::thread_info::set_current_info | Root pointer loaded at23e2c; cbz at23e30 selects23edc. w0=0x220 at23ee0, w1=8 at23ee4, bl __rust_alloc at23ee8. stp x0,xzr at23ef8 installs the root with height0;23f00 sets node length1,23f04 installs null parent/key,23f0c–23f14 store the value and23f24 updates map population. | The compiled empty-root first leaf for BTreeMap<usize,ThreadInfo> requests544 bytes/alignment8. This is not a nonempty split/internal-node branch or the separate name allocation. |

Argument interpretation is direct: the supplied alloc/alloc.rs declares __rust_alloc(size: usize, align: Alignment) and alloc(layout) passes layout.size(), layout.alignment(). The AArch64 w0/w1 immediate writes zero-extend the two scalar argument registers x0/x1. The call targets are identified as __rust_alloc, rather than handle_alloc_error (whose lowered argument order differs) or a deallocator. The selected source pages were independently matched to their installed HTML hashes and decoded hashes.

The mutex function's nominal symbol identifies the private pointee and initializer. For the registry, attribution combines the exact set_current_info symbol with its single source registry declaration BTreeMap<usize,ThreadInfo> (thread_info.rs:34–45), insertion at112–122, and the compiled null-root/height0/first-entry branch. The supplied BTree node source makes a new tree via new_leaf and LeafNode::new's Box::new_uninit_in. This is not an exact nominal DWARF type-size record, and no generic node field layout was inferred. The source-to-artifact correspondence has the build qualification below.

## Artifact and build reliance limits

The existing reviewed layout08 build records remain the basis for source/compiler correspondence: source40129's227-file closure plus the reviewed diagnostic overlay/example (228 total), rustc1.97.1 full commit8bab26f4f68e0e26f0bb7960be334d5b520ea452, LLVM22.1.6, aarch64-apple-darwin release opt-level3 and strip=debuginfo. The example declares the unchanged H K6Alloc through its actual alloc.rs module. The prior review checked its source, allocator, lock/dependency closure and all10 effective compile argv. This backcheck rehashed12 cited prior records and independently matched the current H, FK and example fingerprints: features[], rustflags[], shared recorded profile2040997289075261528. It does not repeat the earlier228-source audit or claim to inspect a final-H executable.

The seven current installed std/alloc/core artifact hashes match I23. They are only current byte-identity facts. No contemporaneous list of linker-input hashes authenticates those bytes as the historic link inputs. Matching toolchain/full commit and nominal symbols support the qualified source attribution, not a silently backdated package-link proof. The recorded dylib list only names libSystem.B; a current libstd dylib hash does not establish that it was linked into the executable.

The LLVM22-versus-Apple-reader libstd archive error remains preserved in I23/LIBSTD_READER_LIMIT.txt. I read that error and did not retry nm, replace a reader, extract objects, issue DWARF queries or broaden inspection. I23's bounded DWARF queries did not find exact nominal layout DIEs; this review makes no such claim. No designer witness proposal was consumed or executed.

These facts establish allocation-request arguments present in this fixed artifact, not whether either site actually executes in every run, a measured aggregate baseline, caller population, arbitrary private alignment, or constants already adopted by final H. For illustration only, substituting the two artifact values in the already conditional symbolic expression gives1028+2*64+544=1700; this arithmetic is not a final-H library bound or full E_max. No612 subtraction or baseline reconciliation was used.

## Remaining premises and actual checked boundary

No new blocking defect was found in the scoped facade repair or the two artifact facts. H12-F1 is closed additively; prior sealed reports remain unchanged.

Before transferring the request facts to final H, retain explicit correspondence of its relevant selected standard-library specializations/compiled request sites, target/cfg/profile and allocator/build basis. The present executable facts alone do not supply that correspondence; current sysroot hashes alone are insufficient. This is the remaining request-binding premise, not a reason to introduce another mirror, new allocator or runtime probe.

The source-side direct single-main/no-competing-initializer, beta<=1, one registry entry, name<=4, owner lifetimes and completed-stage/nonpanic qualifications remain as reviewed in h_leaves_09. H-I0's planned input/path/pass descriptors, H-C0's checked estimator/integration, final-A1 reconciliation and all inherited unclosed kernel/VR/caller cells remain open. This backcheck neither derives whole-process error/startup behavior nor reopens serde/formatting/solver/kernel allocation formulas. No implementation or admission acceptance follows.

Only this additive review subtree was written. No scratch file, Git/index operation, Rust/Cargo/build/test, numerical/runtime/probe, source repair, network/install, allocator/observer change, tool development or delegation occurred. Two existing executable disassembly commands are the only native artifact inspections. One one-off evidence script ended with KeyError while printing heterogeneous metadata after both successful disassemblies and their equality records had already been written; a subsequent corrected metadata read succeeded. It was not an artifact/runtime failure and did not cause a disassembly or runtime retry.
