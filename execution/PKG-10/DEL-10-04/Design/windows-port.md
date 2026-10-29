# Windows port: feasibility and native-service boundaries

Status: initial implementation design for RT-V7; native desktop support is
unestablished. The owner authorized development alongside macOS v1.0.
macOS-first release and all current exit criteria remain in force. This design
adds no Windows release commitment, retained-memory registration, accepted
limitation or release authority. Agent 0 resolves cross-project contracts and
assignment boundaries before the next tranche.

## Evidence and environment

Source inspected: `1560bbf7f7add4637a20572a2b0d404ef121af63`. The first
increment adds supplemental core CI and this design; it changes no production
Rust, frontend, numerical reference, memory coefficient or persistence path.

Read-only discovery on 2026-10-28 found an Apple Silicon macOS host with
128 GiB RAM, Parallels 27.0.2, and a running Windows 11 Pro Arm64 guest
(10.0.26200) assigned 32 GiB. `prlctl exec` can execute Windows PowerShell.
The guest reports WebView2 156.0.4314.50 in its runtime registration. Rust,
Cargo, rustup, Node/npm, Git, cl, CMake and pwsh are absent from that process's
PATH; Visual Studio Installer is absent and Windows SDK is not registered.
These checks do not rule out tools in other locations. No guest installation,
download, VM change, account inspection or unrelated-data modification was done.
WebView2 registration does not establish an exercised Tauri application.

The App implementation manager is the sole common-guest provisioning writer.
Before any common-guest download, that lane supplies exact files, official
sources, sizes and subsequent bootstrapper payloads for owner authorization.
SWBPIPE's proposed requirements, **not installed tools**, are:

- VS Build Tools 2022 17.14 Current (17.14 has no LTSC channel), C++ x64/x86 tools and Windows SDK
  10.0.26100; verify the exact components and Arm-hosted x64 build path.
- Rust 1.97.1 MSVC, `x86_64-pc-windows-msvc` and
  `wasm32-unknown-unknown`; use an Arm-native host toolchain if the selected
  linker/tool host supports it, or label an x64-hosted toolchain as emulated.
- Node 24 LTS, Git 2.x (exact patch versions supplied by provisioning), and
  `wasm-bindgen-cli` **0.2.123**, matching the pinned crate dependency.
  The provisioner proposes the prebuilt Windows x64 archive (7,554,935 bytes),
  avoiding a separate Cargo installation dependency tree; it remains a download
  proposal, not an authorized or installed prerequisite.

CMake, PowerShell 7 and winget are not established requirements. Windows
PowerShell already supports discovery. Use a separate guest-native NTFS
SWBPIPE checkout and Cargo target; the shared Mac mount is visible but is not
the persistence qualification filesystem. Do not share generated wasm output
with another checkout or app. No VM reset, power change or duplicate installer
belongs to the SWBPIPE lane. Native x64 Windows runners supply separate evidence;
Parallels Arm and x64 emulation cannot substitute for that qualification.

Host Rust is 1.97.1 (`8bab26f4f`, LLVM 22.1.6), host
`aarch64-apple-darwin`. Only the Windows x64 Rust standard library was added on
the host, outside the guest. With `CARGO_TARGET_DIR` set to a separate scratch
directory, these exact checks passed:

```sh
cargo check --locked --target x86_64-pc-windows-msvc --tests --manifest-path projects/chirality-piping/core/solver/curved_bend/Cargo.toml
cargo check --locked --target x86_64-pc-windows-msvc --lib --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml
```

The same `check --tests` command passed for `solver/frame_kernel`,
`solver/sparse_direct` and `serialization/canonical_json` by substituting those
crate paths. These checks type-check Windows code; they neither link nor run
Windows executables. On macOS the existing frozen curved-bend formation witness
passed (one test):

```sh
cargo test --locked --manifest-path projects/chirality-piping/core/solver/curved_bend/Cargo.toml formation_bits_are_frozen -- --exact platform_bits_tests::formation_bits_are_frozen
```

The desktop `cargo check --locked --target x86_64-pc-windows-msvc --lib
--manifest-path projects/chirality-piping/apps/desktop/src-tauri/Cargo.toml`, in
a second scratch target directory, failed in bundled `libsqlite3-sys`:
Apple clang could not find the Windows C header `stdlib.h`. No Windows C SDK or
linker is configured on this host. This failure occurs before the desktop Rust
crate can establish its own compilation status; the source blockers below are
inspection findings, not compiler diagnostics from that run. Installing the
Rust target alone is insufficient. No check was bypassed to get past SQLite.
No exclusive AC-004 stack witness or generated-wasm build was run.

## One source tree, narrow native boundaries

Keep React/Three.js, engineering algorithms, model/operation schemas, canonical
bytes, hashes and Rust core common. Preserve the existing Tauri command names,
owning services, admission rules and stale-result guards. Introduce native
implementations at the actual OS operations, without a second numerical engine
or a broad rewrite of `lib.rs`.

| Boundary and proposed owner | Existing constraint / Windows implementation work | Evidence needed before enabling |
| --- | --- | --- |
| Host sampling and admission: DEL-04-07 (`host_memory.rs`, `memory_pool.rs`) | Keep `HostMemory::sample() -> HostSample` and the shared pool policy. Windows sampling must bound physical availability **and commit availability**, timestamp samples and retain Unknown/refusal for failed or stale reads. `GlobalMemoryStatusEx`, process memory counters and low-memory notifications are candidate inputs; define their units, freshness and conservative pressure mapping before use. macOS reclaimable-page arithmetic is not a Windows formula. | Failed/partial API reads, low physical memory, low commit, racing leases, cancellation and a fresh low-memory sample during growth. No fabricated normal sample. |
| Worker custody and allocator channel: DEL-04-07 / DEL-07-11 (`solve_worker.rs`, `worker_alloc.rs`, `solve_jobs.rs`) | A Windows worker is created suspended **already inside** a non-inherited kill-on-close Job Object, using the supported extended-startup `PROC_THREAD_ATTRIBUTE_JOB_LIST` creation attribute. Creation/assignment failure refuses the job before computation; there is no create-then-assign fallback, which would orphan a suspended child if the parent died between calls. Resume only after owned pipes and custody are established. Explicit pipe handle allowlisting replaces Unix fd scans; the allocator's fixed frames, caps and allocation-free refusal stay shared. Own handles at pointer width, close unused ends, and use bounded parent I/O with cancellable reads and a process-exit wait. Parent death, cancellation and startup failure must drain/terminate the whole job and report any unconfirmed survivor. | Parent death before/after assignment, cancellation while blocked, broken/partial frames, cap refusal, child/descendant escape, process-handle reuse, startup races and bounded teardown. Native x64 plus separately labelled Arm/emulation runs. |
| Allocator/ingress/retained route: DEL-04-07 / DEL-04-08 | `worker_alloc.rs` assumes Mac size classes and 16 KiB pages; `ingress.rs` has Mac allocator witnesses. Windows working set, private commit and peak counters measure different things from `phys_footprint`. Define a conservative resident/commit accounting envelope and process reserve, then qualify the actual allocator/page/build identity. Keep Windows retained evidence `Stale` and the ordinary route until its own registration is justified. | Allocation-size/page-boundary and release/cached-heap probes, large ingress/response transport accounting, footprint/commit growth and stack headroom with no concurrent builds. No Mac registration reuse or changed reference outputs. |
| Files and recovery: DEL-02-05 / DEL-08-01 / DEL-15-01 (`autosave*.rs`, `atomic_report_package_save.rs`, `handoff_package_publish.rs`, `handoff_package.rs`) | Split low-level directory/file identity, contained open/enumeration, locks, publish-no-replace, replace-if-unchanged and sync operations from shared policy. Use Windows owned handles, volume/file IDs, explicit sharing flags, reparse-point inspection and handle-bound checks. Preserve each caller's containment strength; the generic atomic writer currently declares best-effort non-adversarial containment, while autosave/handoff have stronger handle-relative checks. Path normalization alone is insufficient. | Reparse/junction and parent/destination swaps, file replacement while locking, sharing violations, disk full, interrupted publish, read-back mismatch, restart/recovery and bounded sync/unknown outcomes. See persistence detail below. |
| Build, packaging and native UI: DEL-10-04 / DEL-07-11 / DEL-07-06 | Keep macOS config/scripts; add a Windows Tauri config overlay and Windows `.ico` only when desktop compilation is viable. Windows installer selection (NSIS/MSI), WebView2 bootstrap/offline policy, signing and distribution remain explicit decisions. Windows build lock/process custody needs creation-time identity and Job Objects; do not replace `/bin/ps` with a PID-only liveness check or remove fail-closed locking. Audit stamp dep-info parsing, drive letters, case and backslashes. | Build interrupt/parent death and stale-lock survivor checks; WebView2 actual launch; native dialogs/menus and Ctrl accelerators, focus, DPI/multiple monitors, Three.js GPU/WebGL, printing and save/reopen. Browser Chromium passes alone do not cover these. |

The current worker cannot be made safe by adding `cfg(windows)` around imports:
non-Unix allocator reads/writes return false, non-Mac footprint returns zero,
and non-Mac host samples refuse admission. The worker's **poll-driven growth
wait is an Agent 0 seam**: macOS stops a child with SIGSTOP while waiting, then
SIGCONT. Windows has no equivalent supported whole-process suspension policy
that can simply be substituted here. Prefer a cooperative safe point plus
conservative reservation, or refuse/terminate growth that cannot safely wait;
Agent 0 must settle the policy and its governed memory-design wording. Do not
call an undocumented suspend API or silently remove the two-second wait.

The backend integration owner retains `lib.rs`, Cargo/config and shared module
routing. Future worker, filesystem, build and UI contributors receive disjoint
files; numerical/persistence/permission slices get targeted tests and independent
scrutiny. No shared App library is a prerequisite.

## Persistence and durable outcomes

Autosave uses unconditional `std::os::fd` / Unix `MetadataExt`, `openat`,
`O_NOFOLLOW`, `fstatat` and directory enumeration. Handoff publication also uses
Unix directories and Mac exclusive/swap renames. Atomic saves refuse replacement
on non-Mac targets. `autosave_commands.rs`, `native_result_download.rs` and
handoff readers need the same audit; fixing only one writer does not make
recovery usable.

Preserve canonical validation, the newest valid recovery copy, read-back hashing,
lock identity and conditional replace semantics. Define outcomes separately for
published-and-durable, published-with-durability-unknown, refused-before-publish,
and failed-with-cleanup-warning. Never turn a directory-flush failure into a
claim of durability; test actual NTFS behavior and report the limit. A candidate
`ReplaceFileW` / handle-based rename does not by itself establish same-identity
replacement, no-overwrite publication, crash durability or containment. Directory
package replacement needs an explicit recovery protocol where Windows cannot
supply the Mac swap primitive; do not use delete-then-rename as an equivalent.
Network shares, cloud-sync directories, long/UNC paths, alternate data streams,
case collisions and reserved device names require explicit handling/refusal
according to the selected operation's contract.

The current local store is bundled rusqlite in `lib.rs` (`open_project_store`,
WAL and immediate transactions). Canonical payload bytes and application-service
ownership remain governed by `docs/architecture/persistence_contract.md`.
Windows SQLite must be tested for crash/reopen, locking, busy/error outcomes and
sidecar handling. App's Unix-VFS internal identity inspection is not a SWBPIPE
implementation contract and must not be copied as one.

## App integration seam

Current SWBPIPE producers/consumers inspected are `operationService.ts` ->
`nativeIngress.ts` -> Tauri `validate_model_operation` / `apply_model_operation`
in `lib.rs`, project-save revision checks in `projectService.ts` / `lib.rs`,
and the foreground JSON CLI
`core/runner/headless/src/bin/openpipestress-runner.rs`. These are local app
services and a local process interface. `api/api_boundary_contract.yaml` leaves
public transport/endpoint binding TBD; `docs/architecture/extension_domain_contracts.md`
does not select a loader or grant persistence. None of these paths establishes
a live App-v4-to-SWBPIPE transport, registration/readiness handshake or durable
external apply receipt. The historical D-30 package-consumption record and App
handoff notes do not establish current runtime connectivity.

Before actual cross-app implementation, Agent 0s must bind the current App
producer and SWBPIPE consumer at exact revisions: opt-in access, authoritative
workspace/source/model/revision identity, readiness and capability refusal,
stale-base rejection, cancellation/parent death, and durable receipt versus
unknown outcome after a lost response. A successful pipe write or child exit is
not proof of an applied domain operation. Reuse their adopted transport after
tracing it; Windows alone does not justify a new one. SWBPIPE retains model,
engineering, acceptance and protected-write semantics; App retains its own
agent execution/credential semantics.

## Increment, declarations and next assignments

The supplemental `piping-windows-core.yml` runs existing core tests on native
x64 Windows with explicit MSVC target and pinned Rust, then checks the engineering
facade. It has no secret access, publication, guest provisioning or desktop
runtime claim. It is outside the required `harness`; selected existing checks
remain required. A green result cannot qualify memory, persistence or native UI.

Neighborhood queries for DEL-10-04, DEL-07-11, DEL-04-07, DEL-04-08 and
DEL-02-05 reported no structural errors; `impact DEL-10-04` includes the shell,
solver/memory, GUI and report/interop consumers. The graph reports relevance,
not fulfilled dependencies. DEL-10-04's existing macOS release baseline and
Windows evidence-gated condition remain true. DEL-07-11 shell dispatch/security,
DEL-04-07 admission/custody, DEL-04-08 qualification, DEL-02-05 recovery,
DEL-08-01 publication, DEL-15-01 model-save and DEL-07-06 native UX obligations
are unchanged. Their ScopeOfWork and needs conditions are left unchanged; this
candidate design and supplemental CI do not satisfy the remaining Windows
conditions. DEL-10-04's existing project-relative code/check pointers are unchanged; the
Root workflow is referenced through `docs/CI_STRATEGY.md`. RT-V7's
post-v1.0 revisit wording needs Agent 0 reconciliation with the owner's parallel
development steer; no Must condition or macOS exit criterion is changed here.
For Agent 0's exact-text review, the proposed row reconciliation is:

Old:

> | RT-V7 | Windows build | Later (revisited immediately after v1.0) | macOS Apple Silicon only (DEC-057) | — |

Proposed (not applied by this Type 1 owner):

> | RT-V7 | Windows build | Later (revisited immediately after v1.0) | Release baseline: macOS Apple Silicon only (DEC-057); Windows development authorized alongside macOS v1.0; native x64 desktop unqualified | DEL-10-04 Design/windows-port.md |

This distinguishes authorized development from release qualification. The
owner's steer is “Windows does not become a Must v1.0 release condition through
this assignment.” Agent 0 owns adoption of the Later-row wording under §5.

Planning estimates below are engineering work days for bounded implementation
and focused repair, **not elapsed delivery promises**, and exclude owner release
acts, licensing/download authorization and external test access:

1. Provision and prove a guest-native checkout/toolchain, run native-x64 core
   CI and separately labelled guest checks: 1–2 days after provisioning.
2. Build-lock custody/stamp portability and a compiling desktop skeleton with
   explicit unavailable services: 2–4 days. Do not hide runtime refusals.
3. Host/worker/memory and ingress envelope qualification: 5–9 days, dependent
   on the poll-wait decision and actual Windows allocator/commit behavior.
4. Contained save/autosave/recovery and handoff publication: 5–10 days, with
   crash/fault tests and independent persistence review; Windows directory
   replacement may require a larger recovery design.
5. Native UI, packaging prototype and Windows native journey coverage: 3–6
   days after the relevant services work; printing/GPU/DPI are hardware-sensitive.
6. Windows retained-route qualification: separately scheduled after ordinary
   memory/custody works, initially 2–5 days; stack/allocator evidence can expand it.

Worker and persistence work can proceed with disjoint writes once their seams
are settled. First ordinary desktop feasibility is approximately 16–31 engineering
days excluding retained qualification, with overlap possible but integration and
review still serial. This tranche is groundwork; it is not Windows completion.
