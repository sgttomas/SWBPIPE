# Ordinary artifact transfer — reviewed-fact correspondence candidate

This packet binds observations to two unpatched ordinary release binaries from
maintained81c038. It supplies an artifact-correspondence candidate for independent
review; it does not accept a complete runtime bound or a later input/launch.

| Artifact | SHA256 | Bytes |
|---|---|---:|
| H k6_observe |48ae299e044240288486b82f15fd2a40c431803c21474ce5ad0372e6704c0561|2,861,152|
| VR vk_scale |c3309cf68be2e6b5bd6ad14a873f4585f6bf664fccb89982b304152c8aac647a|2,772,288|

Exact artifact paths, source/target inventories, complete commands and raw output
are under _run_records. The immutable archive SHA256 is
26b48ec62ea3f21849c288369f2ae245130fecde4fc69e7c9fa643562592a725.
All299 source files retain their prebuild bytes and read-only modes after both
builds. No diagnostic overlay exists in this ordinary archive.

## Build and source correspondence

H completed once, exit0, ten logged target compiler invocations. VR completed once,
exit0, nineteen logged compiler invocations (including three host dependency build
scripts) and three logged build-script executions. Both commands used installed
Rust1.97.1, target aarch64-apple-darwin, release/offline/locked/-j4, incremental0
and auto-install0. Targets were fresh and separate. H/VR/FK actual --cfg feature
sets are empty. serde_json activates default/float_roundtrip/std plus the existing
fast_arithmetic64 cfg; serde_core activates std only. No --test, mutation feature
or custom rustflag is present. Allowed --check-cfg feature names are kept distinct.

COMPILE_ARGV and CARGO_FINGERPRINTS retain exact effective settings. Target invocations
explicitly record opt-level3, embed-bitcode=no and strip=debuginfo. Panic,
overflow-checks, debug-assertions, LTO and codegen-units are not explicit target
arguments; no omitted default is labeled an observed flag. Host build-script
debug-assertions=off is separately explicit. Ordinary release interpretation and
same recorded release-profile identity remain distinguishable from explicit argv.

The installed cargo/rustc bytes match the accepted1.97.1 records, including rustc
full commit8bab26f4f68e0e26f0bb7960be334d5b520ea452 / LLVM22.1.6 identity. All59
target-sysroot files and the tool binaries match before/after. Both optional
.rustc_info.json caches are absent; no cache was fabricated or compiler query
added. Compiler identity uses actual hashes, invocation/fingerprint evidence and
the existing same-binary version/source record.

The two authenticated serde archive hashes and116 installed packaged-file hashes
match their accepted binding. Fifty-six selected installed/library source pages
match; the ArcInner source also matches its accepted hash. Full dep-info and32
Cargo fingerprint files are retained. The two changed type-containing files have
only the reviewed capture/wrapper additions:121 layout04 rows retain whole source
file identity,17 retain exact declarations/tuple anchors,21 standard/serde
expressions retain the bound library/compiler/feature basis. The five layout08
type expressions retain their originating source hashes. These are source/type
correspondence checks, not159+5 fresh layout observations. No old overlay ran.

Both ordinary images dynamically name only libSystem.B through otool. Actual
--extern identities, target inputs, source paths and final nominal std symbols are
retained. The normal commands emit no separate linker map/response or complete
per-standard-object link-input list. The59-file contemporaneous installed inventory
is not mislabeled as a list of59 linked inputs; top-level compiler driver hashes
are postbuild identity only. This limit does not erase the final direct request
arguments below.

## Direct requests in the ordinary images

All addresses below are actual calls to the image's __rustc::__rust_alloc.
Immediately preceding w0/w1 writes establish size/alignment and zero-extend to
x0/x1. Allocation-failure/deallocation argument order is not substituted. Complete
typed function ranges and surrounding branch/data flow are preserved; these are
compiled requests, not assertions of execution count or process peaks.

| Exact kernel/runtime fact | H allocation call(s) | VR allocation call(s) | Size/alignment |
|---|---|---|---|
| BTreeMap<u32,usize>, leaf/root |0x100154444|0x100152720|144/8|
| Same geometry map, internal |0x100158330;0x100158504|0x10015660c;0x1001567e0|240/8|
| BTreeMap<(RuleTest,u32,Kind),BoundedExtremeTracker>, leaf/root |0x100153508|0x100151ff4|1072/8|
| Same tracker map, internal |0x100157504;0x100157818|0x1001557e0;0x100155af4|1168/8|
| BTreeSet<(RuleTest,u32,Kind)> with actual SetValZST, leaf |0x100157b4c|0x100155e28|104/8|
| Same holding set, internal |0x100157d30;0x100157ed8|0x10015600c;0x1001561b4|200/8|
| OnceBox<pal::unix::sync::mutex::Mutex> initializer |0x1001d5e28|0x1001d44e8|64/8|
| BTreeMap<usize,ThreadInfo> first registry leaf |0x1001b6f44|0x1001b4250|544/8|

Geometry and tracker root routines contain their exact nominal key/value types,
test the no-tree branch and install the returned leaf with height0 and first
entry. Their typed insert_recursing routines directly expose leaf and internal
requests; holding likewise names the actual SetValZST and Leaf specialization.
Existing accepted node constructor/topology sources supply the branch attribution,
without a mirror or leaf-alignment guess. Allocation alignment8 is now directly
observed in these ordinary sites; the historical layout08 report itself stays
unchanged and did not supply that alignment.

For the registry, H's root-pointer branch at0x1001b6e8c selects0x1001b6f38,
VR's at0x1001b4198 selects0x1001b4244. The544 request is followed by root/height0
installation, node length1, null parent, first key/value stores and population
increment. Nonempty544/640 requests and name allocation are separate branches.
For mutex, the nominal OnceBox initializer allocates64/8, initializes the private
pal pointee and installs/compares the same pointer; losing-race deallocation is
separate. The accepted Mutex/Once forwarding and source-side single-main,
beta<=1, first registry entry/name<=4 assumptions remain necessary.

| Exact VR specialization | Fresh leaf call | Leaf/alignment | Internal/alignment and warrant |
|---|---|---:|---|
| String,serde_json::Value |0x10006b0a0|632/8|728/8, inherited accepted repr(C) containing-layout rule|
| &str,&cases::Row |0x10002e684|280/8|376/8, same rule|
| QuantityId,PublishedRow |0x100014bd4|848/8|944/8 directly at0x100014eb8 and0x1000152bc|
| String,(f64,f64) |0x10002e490|456/8|552/8, inherited rule|
| (usize,usize),(f64,usize) |0x1000155cc|368/8|464/8 directly at0x100015784 and0x100015988|
| String,String |0x10002e35c|544/8|640/8, inherited rule|
| usize,actual SetValZST |0x1000169b4|104/8|200/8 directly at0x100016ae8 and0x100016c7c|
| (u32,u32),actual SetValZST |0x100016398|104/8|200/8 directly at0x1000164e4 and0x10001667c|

The typed bulk constructors name actual Case capture, floor::scales_from and
lane::value_controls callers. Their leaf allocation is distinguished from the
controls' earlier dynamic Vec/sort allocation. Publication/allowance/set split
functions name actual K,V and marker::Leaf. The two set rows are independently
bound ordinary requests; neither is promoted from its prior debug executable.

Four internal pairs above are explicitly source-composed, not direct final call
observations: the already accepted actual InternalNode repr(C) rule,12 thin pointer
edges and freshly observed leaf alignment8 give the existing +96-byte result.
No default repr(Rust) leaf is reconstructed. The old source proof and its hashes
remain the warrant. The separate tuple3 spring set remains uninspected and has no
new private request fact; only the accepted fixed-input reach/emptiness premise
may omit it.

| VR handled-error owner | Ordinary call | Direct request |
|---|---|---:|
| StringError |0x100196714 (empty-string alternative0x100196770)|24/8|
| io::Custom |0x100196734|24/8|
| serde_json::ErrorImpl |0x1001ce428|40/8|

Typed Error::new<&str> allocates the backing string separately, stores its
capacity/pointer/length into the first24-byte object, then stores that data pointer,
vtable and ErrorKind in a second24-byte object. Error::io boxes its Io payload and
zero position fields into40 bytes; cleanup names ErrorImpl. WriterFormatter passes
the literal length9 to the typed constructor at0x100067c58; its w0=40 is ErrorKind,
not the40-byte ErrorImpl request. The compact escaped helper forwards the error,
and the Value serializer calls/tail-branches to Error::io at0x100068194/0x10006806c.
The accepted coexistence/move/drop theorem remains separate from these requests.
No general panic, fatal-allocation or foreign-startup envelope is supplied.

The requests route to H's registered allocation entry0x100003e78 or VR's
0x10000c814. Their captured bodies account requested bytes against their own
CURRENT/CAP state and update peaks; refusal names the correct H/VR allocator.
The separate realloc shims route to nominal K6Alloc/VkAlloc implementations.
Their source accounting remains unchanged; no allocator experiment was run.

## Fresh public reporter and boundary

Exactly one unchanged public_layout20 compile and one run succeeded against the
new ordinary VR/FK rlibs, with all ten expected labels, actual type names,
sizes and alignments. PUBLIC_LAYOUT_BINDING/EXECUTION retain source/compiler/rlib/
reporter hashes, exact argv/environment, times and raw stdout/stderr. Its source
hash remains7f4e0aa8a7a05f50bd325f015a81c1f70c291355ec09193010877f386a601dc6.
This is the only standalone diagnostic execution; H and vk_scale were never run.

No required private-request cell in the finite selected set was inaccessible in
these images. The four source-composed internal pairs, retained layout04/08 type
facts and unavailable link-map/cache fields are explicitly distinguished above.
Unknown general tuple3 requests are not silently assigned a layout.

Final exact inputs, normal argv/argv0/cwd/compiled-manifest paths, H surviving
Strings/repeat/prefix context, VR input history, runner prepass, chronological
admission and any measured witness remain unexecuted and unqualified. This packet
does not authorize them. Independent review and ROOT disposition precede reliance;
all prior normal/handled-error and fixed-roster scope restrictions still apply.

