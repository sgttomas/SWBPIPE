# I42 — RV56-F1 repair return

**Implemented and frozen for ROOT commit and same-RV56 backcheck.** Only
`frame_kernel/src/structural/retained/adaptive.rs` and
`frame_kernel/tests/retained_k4/source_bridge_tests.rs` changed. The source basis
is ROOT-supplied `4afbac6e613203f93a499b780082b28be37f6c2c`; review basis is
NUM `f2b9a89356981c246da2bfbeeecd35193c09d20d`. All maintained paths above are
under `projects/chirality-piping/core/solver/`.

The sharper helper now returns its original numeric result and its own exact
0/5 operation count. It records five immediately after the fifth unchanged
binary64 statement, before subsequent checks. The checked-radius helper retains
that count outside its fallible closure, including excessive-radius rejection.
Legacy signatures project the same single execution. The view collects actual
work before propagating the radius result; an original radius error survives
simultaneous accounting loss, while non-exact work prevents successful extraction.
The former class-based inferred increment is removed. No numeric expression,
predicate, class, radius, public reason, bridge theorem or solve schedule changed.

| Focused check | Result |
|---|---:|
| Existing nine bridge controls plus three repair controls, debug | 12/12 |
| Same controls, optimized | 12/12 |
| Existing private radius identity | 1/1 |
| Existing independent sharper rounding/decimal boundary | 1/1 |
| Unchanged S11 inventory/scanner | 3/3 |

The new controls verify early metadata/class/absence/shape/invalid-input refusal
at zero operations, post-ceiling radius refusal at five, successful relative
validation at five, full successful view at the unchanged 145, legacy result/error
parity, and simultaneous checked overflow with preserved RadiusClassMismatch.
The RV56 radius=1.0 private corruption path is covered through row, view and bridge
returns. Seeded corruption/accounting controls are not claims of natural reach.

All five credited source freezes match the final maintained hashes. The 109
original I42 native numeric/work/state trace lines match repaired debug and
optimized output byte-for-byte. Original I42 packet payload hashes match its
sealed inventory; this repair plan and original packet were not edited.
S11 required no new disposition and was not changed. No check failed in this
repair run. One reviewer-control read initially used the source checkout instead
of NUM; the absent-path read was corrected before consulting those controls.

Execution: the existing TASK `/root/i42_source_bridge` directly under ROOT,
with no descendants. First grant clock witness was 2026-10-03 01:10:48 UTC.
Commands were sequential, locked/offline, four jobs/two threads, existing isolated
I42 manifest target and 1200-second walls. All owned commands exited; guard 5387
was live and no Cargo/rustc remained at source freeze/lane release,
**2026-10-03T01:14:54.951916Z**. Source expansion ended before the 20-minute cutoff.
No Git/index/API/dependency/instruction/other maintained writes occurred.

`_run_records/FINAL_MAINTAINED_HASHES.json` binds the two-file source;
`BASELINE.json` binds its input and grant; per-command freezes, argv, environment,
times, exits and stdout/stderr retain the actual runs. `REVIEW_ORIGINS.json` binds
the read reviewer records. Existing Root/TASK/Piping and selected diagnosis-skill
origins remain recorded in PLAN and the original I42 packet. The only new runtime
owners are two small private fixed spent-return records; no source-sized storage
or resource qualification changes. Original PP/final-row/availability/profile
limitations remain. Independent backcheck and integration belong to ROOT/RV56.
