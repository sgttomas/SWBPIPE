# I23 private request artifact08 return

Proposed artifact-specific request evidence found; independent adoption and
final-H correspondence remain open. Start **2026-10-01 15:34:14 UTC**;
return **2026-10-01T15:42:20Z** (486.8 seconds, within12 minutes).

The exact reviewed layout08 diagnostic binary matches SHA256
63ea631507802f2a0ec939299b5dba2b7bdfb08728cb432be45d20c9e6938607.
Its recorded release1.97.1/aarch64 build and current fingerprints were checked.
Only the two named functions were inspected:

- Actual pal::Mutex OnceBox initializer: at0x100039058, __rust_alloc receives
  **64 bytes, alignment argument8**, followed by pal::Mutex::init and pointer
  storage; the losing-race cleanup uses matching64/8 deallocation.
- Actual ThreadInfo registry set_current_info: the null-root branch at
  0x100023e30 reaches0x100023ee8, where __rust_alloc receives **544 bytes,
  alignment argument8**. It then stores the root with height0 and inserts the
  first key/value. Source fixes this map as BTreeMap<usize,ThreadInfo> and
  new-tree creation as an actual LeafNode, not another specialization.

These are compiled request arguments in that diagnostic artifact, not a new
runtime measurement or a source-field/mirror/public-wrapper inference. No
aggregate baseline was used. No claim is made about call occurrence on every
startup, final-H linkage, E_max or acceptance. REQUEST_FACTS.json carries exact
symbols, ranges, call addresses, instruction witnesses and remaining conditions.

**Tool limit, separately:** installed nm cannot read the Rust LLVM22 libstd
object (Unknown attribute kind105; Apple LLVM reader). Exact error preserved;
that archive lookup was stopped without retry or replacement. Bounded DWARF
queries yielded namespaces/inlined names but no actual nominal layout DIE.
The separately authorized reviewed executable was readable by existing tools,
so its two call sites supply evidence despite the archive-reader limitation.

Current libstd/alloc/core rlib/rmeta (and std dylib) paths/hashes are pinned in
ARTIFACT_BINDING.json. Correspondence of those current files to the historical
link is qualified: reviewed toolchain/full commit/target and nominal crate
identities support it, but no contemporaneous linker input hash list was found.
The missing BUILD_IDENTITIES filename was not fabricated; existing TARGET_FILES,
RUSTC_INFO and RV30 compiler/binding records supplied the concrete build basis.
ROOT selected this diagnostic explicitly, not a final-H artifact.

No executable, Cargo, Rust build, probe, install, replacement, std rebuild,
flag/config/source/permission change, network, Git/index or delegation occurred.
No existing object was copied or extracted. Prior/source13/RV30 active work is
untouched. RV30 must inspect the exact source/call-site attribution before any
fact is integrated or transferred. SHA256SUMS seals the portable packet; exact
raw tool outputs remain hash-bound in owned scratch.
