# I23 wrapped-error artifact18 return

Three actual private constructor requests are identified in the already bound normal-release vk_records executable (SHA2567c477a6bd129a1e2d5dd70c4f7557984615337a155ba8f4b0f0a91f08ddc861e):

| Actual private type | Requested bytes / alignment | Allocation call |
|---|---:|---|
| StringError local to alloc boxed From<String> |24 /8|0x10016fab4|
| std::io::error::Custom |24 /8|0x10016fad4|
| serde_json::error::ErrorImpl |40 /8|0x1001a91f0|

The two24-byte allocations are separately attributed in Error::new<&str>. The first receives the converted String fields and becomes the boxed-error data pointer. The second receives that pointer, its vtable and the error kind. Pinned source binds the first to Box<StringError> and the second to Box<Custom>. The earlier dynamic string-byte allocation is kept distinct. Error::io is a typed constructor whose pinned body allocates Box<ErrorImpl>; its compiled40/8 request and typed cleanup are explicit. No public-header, field-sum, private-layout mirror or untyped-constant inference is used.

The actual WriterFormatter caller and its serializer error forwarding are captured narrowly. These support constructor attribution only: this packet neither accepts nor completes I21 format_stream_17's topology/owner proof. No aggregate wrapped-error envelope or peak is asserted.

All facts are scoped to this inspected executable, its recorded seeded-faults normal release profile, Rust1.97.1/aarch64 build, and source/package bindings. Current binary/fingerprint hashes match the existing records. Final vk_scale source/cfg/profile/request-site correspondence remains separate, as does RV30 independent review. No runtime reach/frequency, E_max, K0 or admission acceptance is claimed.

Raw provenance, exact disassembly, per-request argument/type attribution, source excerpts and build/package bindings are under `_run_records`. Only this packet was written. No executable/test, Cargo/build/probe, package fetch, library scan, replacement tool, incompatible archive-reader retry, source/binary change, Git/index or delegation occurred.

Actual start:2026-10-01 19:39:05 UTC. Deadline:19:49:05 UTC. Completion is recorded after final checks.

Completed:2026-10-01T19:44:54Z. Final checks preserve the binary, fingerprints and consulted source/record hashes. Five bounded static instruction captures cover1608 bytes of named code. Three request pairs verified; no runtime.
