# Windows worker probe (incomplete)

This standalone crate is an early fixture, not the SWBPIPE worker. The protocol
model tests injected time and samples; it does not spawn a worker, establish
quiescence, test allocator locks, or exercise a Job. On native x64 Windows the
accounting test reports process private commit, private working set, total
working set and shared commit before and after a small mapping and heap exercise.
The OS and allocator may retain or move pages; no exact delta is asserted and
commit is never counted as resident credit. Intentional live allocations are at
most 8 MiB, and the two 4 MiB allocations do not overlap.

The CI job deliberately fails until the native process/control and custody
cases in the design have been implemented. A passing model or accounting test
is not Windows worker safety or production qualification. No external Rust
dependencies are used; the lockfile is committed. Run `cargo test --locked
--target x86_64-pc-windows-msvc -- --nocapture` on native x64 Windows.
