# Proposed finite protected replay

This packet is source preparation only. ROOT reads the patches, RV29 reviews
exact semantics, and ROOT grants runtime separately. No current kill or
baseline pass is claimed.

1. Freeze accepted patch bytes and focused filters against source
   `40129a225d73860ac2a53da9a2fa73869df668f3`. Use a disposable archive under the
   execution grant, one mutant at a time. Compare pre/post hashes in INVENTORY.
   Do not execute the old broad mutation drivers or their verdict rules.
2. Existing guard must be active; ROOT allocates the host slot. Use toolchain
   1.97.1, RUSTUP_AUTO_INSTALL=0, CARGO_INCREMENTAL=0, offline/locked Cargo,
   one process at -j4, RUST_TEST_THREADS=2, an owned target. This is a proposal,
   not a Rust grant. Preserve every protected test, oracle, tolerance and limit.
3. Compile the unmutated finite target, then validate the exact filters. Use
   the built binary's `--list --exact <filter>` to prove nonempty selection,
   then `--exact <filter> --test-threads=2 --nocapture`. Every baseline must pass.
   Pin binary/build hashes and raw logs. Stop an affected path on a baseline
   false publication or unexpected outcome; do not edit observations.
4. Compile each finite patched source separately. A compile/link failure is
   an invalid experiment. Run the same exact filter and credit a kill only
   for the named semantic assertion, with actual code-path/row evidence.
   Publication-certificate rejection, unrelated failure and record/golden
   drift alone do not kill a historical fault. A surviving required fault or
   masked assertion returns a gap, never automatic equivalence.
5. Prioritize original A2 M1–M8 (M4a–d separately), direct scalar/bound/class
   checks for stale entries, and the assertion-preserving RV19-D4u route.
   Preserve historical R7 derivation guards, M14/M19 dispositions and
   RV23-N4 survivors. RV23C-M1 requires its separate non-budget cache route.
6. Additive diagnostic code in PROPOSED_ADDITIVE_DIAGNOSTICS.rs.txt is a
   reviewable proposal only. It was not installed or compiled and does not
   grant test implementation. Entries requiring these diagnostics stay open.
   K4-M24/D15 retain the real certificate; their pre-certificate verdict
   observation remains a gap. Do not fabricate an accepted certificate.
7. V-K uses its existing `seeded-faults` feature, with FK_SEEDED_FAULT set to
   exactly one registered ID in each fresh process (OnceLock). Its 15 faults,
   NONE, VK-UNKNOWN, source sites and focused test targets are retained in
   INVENTORY. Family record inequality alone is not a numerical kill. Keep
   the original registered semantic target and expose any narrower-filter gap.

Suggested Cargo compile command, only after grant:
`cargo test --offline --locked -j 4 --manifest-path <DISPOSABLE>/projects/chirality-piping/core/solver/frame_kernel/Cargo.toml --lib --no-run`.
For the independent S11 entry use the original core archive scope and
`--test s11_site_table`. For V-K use its recorded manifest and
`--features seeded-faults --test <target> --no-run`, then exact binary filters.
No full H/VR suite, solver experiments, generator, install, network, test
weakening or general mutation framework is proposed by this packet.
