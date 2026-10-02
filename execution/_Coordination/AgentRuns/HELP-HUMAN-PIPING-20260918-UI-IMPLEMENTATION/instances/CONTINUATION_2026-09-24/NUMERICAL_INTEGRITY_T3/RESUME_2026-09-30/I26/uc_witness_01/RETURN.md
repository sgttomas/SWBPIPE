# I26 — source-valid Uc overlap witness

**Exact M05 is killed by the new independent numeric assertion.** NONE compiled
and passed all ten focused kernel-envelope tests. Applying the unchanged original
M05 patch compiled successfully, left the nine existing tests passing, and failed
only `six_axis_springs_expose_uc_c_overlap_in_verification_build`. No protected
assertion, tolerance, estimator, kernel, loader or historical record changed.

P means `projects/chirality-piping`; H means `P/core/solver/performance_harness`;
FK means `P/core/solver/frame_kernel`; R is this packet's resumed T3 root.
The sole maintained edit is `H/tests/k6c_envelope.rs`. The frozen candidate was
`40179f1da5961cfef7edeaf7be4eb675379f79b5`; read-only comparison verified its whole
core subtree equals the declared `10315a8167c47f43aa41402beb88ed2c70e62cf2` basis.
ROOT receives this test and packet for integration and fresh independent review;
this implementation return does not grant acceptance or close the wider mutant
programme.

The witness constructs one invented finite node with six positive unit axis
springs, one per unconstrained DOF, using the public `PrimitiveSource::new` path.
It checks that all member, directional, constraint, load, station and support
collections are empty and verifies the six free DOFs and one source body.
`SparsePattern::from_positions` consumes the actual spring DOFs; every resulting
row is asserted to contain its diagonal alone. Consequently the six free rows
are six singleton blocks and have six profile entries under any RCM order. This
uses the same spring-position rule as FK `retained/assemble.rs::Structure::new`
and the component/skyline definitions in `bound.rs::free_blocks` and
`factor.rs::order_free`. The H member-graph counting shortcut is intentionally
inapplicable to this spring-only shape and is not used. Public `layout` and
`encoding` validate 13 quantities and 164 encoded bytes: six displacements, one
magnitude and six spring actions; six header bytes, eight four-byte list lengths,
one 24-byte node and six 17-byte spring records. These are checked source facts,
not scalars chosen merely to pass descriptor validation. No solve is run.

The expected owner values were calculated before running the controls from the
accepted Uc lifetime: `verify.rs::build_verify_shared` keeps six 16-byte refusal
slots and a six-u32 row map while `bound.rs::uc_bounds` retains `c` across
`nl_pass`'s `at`, `bt` and `ct`. Under the named conditional reference profile,
the additional owners require `96 + 24 + 24*w` bytes for Wide widths 48, 80 and
144. Requested and moving bounds are equal for these fixed-size allocations.
The test subtracts each schedule's identical prepaid retained prefix solely to
isolate this owner; it does not use that prefix as an oracle or claim to verify
its value. There is no full estimator reimplementation or evidence-file reader.
The member/widened competitors are empty. M05 drops `c` from the first alternative;
the later `c/ct/u/n_l` alternative then dominates, leaving a 24-byte deficit.
Thus neither a different phase maximum nor another build alternative hides this
exact mutation.

| Verification build above retained prefix | NONE expected and passed (requested/moving bytes) | Exact M05 actual (requested/moving bytes) |
|---|---:|---:|
| Full, 256 | 1272 / 1272 | 1248 / 1248 |
| Full, 512 | 2040 / 2040 | 2016 / 2016 |
| Full, 1024 | 3576 / 3576 | 3552 / 3552 |
| Selected128, 256 | 1272 / 1272 | 1248 / 1248 |

Both controls ran the same `cargo test --release --locked --offline -j 4 --test
k6c_envelope -- --test-threads=2`, from the immutable candidate's owned core
archive plus the exact maintained test postimage, with installed Rust 1.97.1,
auto-install disabled, incremental disabled and a fresh owned target. Existing
M5 guard 5387 was checked before each Cargo run. Raw argv, environment, tool
versions, logs, patches, source/test/binary hashes and restoration checks are
under `_run_records`. The normal and mutant binary hashes differ. The complete
test diff and before/after postimages are retained there. The earlier survivor
packet's 38 files are byte-identical; its historical result remains unchanged.

The archived estimator was restored and all 564 archived core files were checked
against the candidate plus the sole test postimage. The maintained core has no
other difference. Last compiled artifacts remain M05 diagnostic artifacts, not
normal binaries. Cargo work is complete and the granted lane is released.

Numerical scope is the source-valid conditional owner identity and the exact
NONE/M05 discrimination above. It is not runtime allocation measurement, solver
qualification, engineering acceptance or a wider envelope proof. Supporting
checks are the focused ten-test suite, exact patch identity, byte preservation,
and clean diff whitespace. M06–M12, H33 reruns, separately owned VR controls and
all solver/performance jobs remain unrun. No follow-on is automatic. Fresh review
and ROOT integration remain outstanding; there is no remaining witness gap.
The packet seal is `_run_records/SHA256SUMS`.
