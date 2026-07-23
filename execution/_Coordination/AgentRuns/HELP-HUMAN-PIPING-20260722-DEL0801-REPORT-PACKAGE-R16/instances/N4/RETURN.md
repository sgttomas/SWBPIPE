# N4 Return — IMPLEMENTED

N4 completed the adopted DEL-08-01 candidate v5 implementation within the
exact N4 rows of `WRITE_MATRIX.csv`. The result is ready for fresh N5
verification. N4 does not declare `COMMIT-SAFE`.

## Implemented scope

- Existing report-package assembly is bound to the desktop report surface and
  native File > Save Report Package command without adding or changing a runner
  verb.
- Current-session report, result-export, audit-manifest, and
  state-comparison/handoff inputs are assembled into the established package
  schema through the existing redaction contract.
- The UI exposes sanitized decisions, findings, blocking state, summary, and a
  structured save receipt before downstream reliance.
- Local-private retention is default-off and requires explicit user intent.
- Native save uses caller-owned same-parent temporary-file/write/rename
  semantics with replacement detection, cleanup, container digest, package
  identity, and bounded parent-directory durability reporting.
- Native custom-menu dispatch reaches the packaged React app through a closed,
  validated DOM command bridge.
- Source inputs remain non-mutating; established provenance, units,
  diagnostics, report controls, redaction route, professional boundary, and
  claim-state distinctions are preserved.

## Packaged-native terminal proof

The final guarded debug bundle passed the exact native sequence using Computer
Use/Sky with invented preview data:

- load and solve to `MECHANICS_SOLVED`;
- open Report through the native View menu;
- observe private retention off by default, then enable it explicitly;
- cancel the native save sheet with `bytes=0` and no filesystem effect;
- save `desktop-report-run-preview-linear-static-001.opsproj` successfully;
- verify ZIP integrity, package identity, member sizes/hashes, and cleanup;
- save the same basename again through the native replacement-confirmation
  sheet and verify atomic replacement.

The new-save and replacement receipts report `bytes=3189621`,
`durability=durable`, container SHA-256
`4416c53b68a1e71cda0445408ca24d15e56023ab879c435cfd30b86387bb2cb7`,
and package identity
`39628d597b9d09b0f9aa1d9899190f3fd90bb31600495b03bd997317f38f8bff`.
Replacement changed the inode from `160542015` to `160542328`; no
`.openpipestress-report-package-*.tmp` residue remained.

Exact evidence is in:

- `CHECK_native_terminal_save_replace.txt`;
- `CHECK_native_reclassification_and_repairs.txt`;
- `apps/desktop/SMOKE.md`.

## Verification completed before return

- focused desktop App/report/request/redaction/service Vitest: 77/77 passed;
- focused post-repair request/redaction/parity/render Vitest: 15/15 passed;
- Python DEL-08-06 parity: 11/11 passed;
- Rust report-package/PDF/renderer: 28/28 passed;
- focused atomic-save regression: 8/8 passed;
- focused packaged menu bridge: React 1/1 and Rust dispatch 1/1 passed;
- source H4: 2/2 passed;
- production-dist H4: 1/1 passed;
- packaged debug build after all contained repairs: passed;
- full desktop Rust: 65/66, with the sole rule-pack notice failure reproduced
  at the frozen clean base and preserved in `CHECK_clean_base_rule_notice.txt`.
- affected-check selection returned desktop build/test, piping pytest,
  practitioner-harness pytest, harness self-check, and evidence sweep;
- registered desktop production build: passed;
- registered practitioner-harness pytest: passed;
- registered repository self-check: passed;
- registered desktop Vitest first returned one unrelated endpoint-picker
  failure, then passed the immediate focused rerun 1/1 and complete registered
  rerun 28 files / 502 tests; the initial result remains preserved;
- registered piping pytest returned 4 failed / 531 passed; a clean archive of
  exact frozen SHA `8698b0338ac82556fee583dd3f85bb62d0b74f85` reproduced the
  identical four failures with the isolated file at 4 failed / 6 passed because
  the frozen pointer names DAG-008 while the frozen tests hard-code DAG-007;
- claims-language: passed, 267 files;
- path-anchor validation: passed, 947 live surfaces and zero findings;
- loop-receipt validation: passed, frozen through Receipt-44;
- JSON/JSONL, raw-evidence, WRITE_MATRIX containment, and staged/unstaged
  whitespace validation: passed.
- owner-authorized v6 changed only four stale expected DAG paths in
  `tests/test_release_readiness_script.py` from DAG-007 to active DAG-008;
- v6 focused release-readiness pytest: 10/10 passed;
- v6 full piping pytest: 535/535 passed.

## Held gates and residuals

- Broad registered checks pass, including desktop 502/502 on full rerun and
  piping 535/535 after the exact owner-authorized v6 correction.
- The historical frozen-base DAG-007 failures and manager hold remain preserved
  as the basis for v6; they no longer block the pre-sweep gate.
- At this sealed pre-sweep return state, the DEC-025 sweep count is 0 and
  exactly one acceptance-eligible sweep is authorized next. N4 will make no
  filesystem edit after invoking it and will return its status/artifact/hash by
  live message only.
- N5 verification and W3 closeout were not launched by N4.
- The non-repository temporary package is retained transiently for N5
  inspection; N5 or the manager may remove it after independent verification.
- Directory durability is fail-honest: a timeout/error/disconnect or an
  already-active prior worker reports `saved_durability_uncertain` after the
  committed rename. Static symlink checks and same-parent create/rename remain
  best-effort against a malicious concurrent parent or destination swap.

Terminal durable N4 result before the sole sweep: product `IMPLEMENTED`;
verification `PASS_READY_FOR_SINGLE_SWEEP`. N4 does not claim COMMIT-SAFE.

## Candidate v7 replacement implementation — pre-sweep return

N4 completed accepted slices R16-SLICE-01 through R16-SLICE-05 as the sole
serialized implementation owner. The earlier v5/v6 return above remains
historical evidence and is superseded for candidate selection by this v7
section.

- Exact current-session InputManifest construction and analysis-run binding:
  implemented and verified.
- Explicit `linear_stiffness` / `rotational_stiffness` dimension flow through
  the accepted schema/export/report-package boundary: implemented and verified.
- Current-session package-owned copies use private-project/private-only,
  pending, user-local provenance without source mutation.
- TypeScript and Rust package boundaries reject malformed SHA-256 values.
- Option A remains intact: no InputManifest package member and no portable
  replay claim.
- Full desktop Vitest passed 29 files / 510 tests; piping pytest passed
  537/537; practitioner-harness pytest passed 311/311; harness self-check and
  desktop build passed.
- Focused Rust report/package/PDF/renderer/result-export/bridge gates passed
  44/44. The sole extra full-desktop-Rust failure remains the frozen-base
  rule-pack notice assertion recorded in `CHECK_clean_base_rule_notice.txt`.
- Source and production-dist H4 checks passed 4/4.
- Packaged-native cancel, new save, and same-path replacement passed using
  Computer Use against packaged app SHA-256
  `59f281ff17f9d76b6344327f7c668edf1097982c25ef8e283aebef017dddc83a`.
- Both saved receipts reported 3518675 bytes, container SHA-256
  `6a02201b9626156d1833e3a9566c96c1b744ab527f0d24f4c5b30a7caa86ecfd`,
  package identity
  `f23abdf7303673feaa602d4a702676a4098bb924416134f1b0ab7a8b891833a5`,
  and durable parent-directory sync. Replacement reported `replacement=true`;
  the inode changed from 160629227 to 160633120; no package-temp residue
  remained.
- ZIP integrity, manifest member sizes/hashes, InputManifest/result-hash
  distinction, exact SHA syntax, private copied provenance, and explicit
  stiffness dimensions passed after replacement.

Exact v7 evidence:

- `CHECK_v7_pre_native_gates.txt`
- `CHECK_v7_native_terminal_save_replace.txt`
- `apps/desktop/SMOKE.md`

N4 has not invoked the v7 replacement DEC-025 sweep. The manager must validate
this sealed pre-sweep fan-in and containment before releasing that sole
invocation. N4 makes no COMMIT-SAFE claim.

## Candidate v8 three-finding closure — pre-sweep return

N4 implemented only the three owner-adopted v8 findings as sole serialized
implementation owner:

- exact full InputManifest identity derived from solved model token plus
  canonical manifest SHA-256, with wrong-prefix and wrong-model-token negatives;
- result family derived from exact kind and explicit DEL-14-02 source dimension,
  with unit-text inference removed and deceptive-unit/mismatch negatives;
- OS-random `create_new` same-parent temp names through direct
  `getrandom = "0.3"`, with deterministic write/flush/temp-sync/rename failure
  injection proving destination preservation, cleanup, and diagnostic priority.

Focused tests passed: desktop 26/26, Python 14/14, atomic Rust 12/12, and the
combined focused Rust report/package/native union 56/56. Full desktop Vitest
passed 516/516; piping pytest passed 539/539; practitioner-harness pytest passed
311/311; repository self-check, desktop build, source H4 2/2, and
production-dist H4 passed. The extra full desktop Rust run returned 77/78 with
only the reproduced frozen-base rule-pack notice assertion.

Fresh packaged-native cancel/new-save/same-path replacement passed against
packaged executable SHA-256
`cd09000a9655bc706675d623d6436a1611f2ce7aad4f58116023414b9ae228d1`.
Both saves produced 3518303 bytes, container SHA-256
`e0145a9fb7d377034c7876a92336581ecdeb5d737fe223c5b39d560adfc42cc7`,
and package identity
`fb81914e2d9654876eb9421634f1d4ab1e7a14f2fa056b52fe6eb330d19b125a`.
Replacement changed the inode from 160685423 to 160685542; member hashes,
exact InputManifest identity, explicit result semantics, and no-temp cleanup
passed.

Exact v8 evidence:

- `CHECK_v8_pre_native_gates.txt`
- `CHECK_v8_native_terminal_save_replace.txt`
- `apps/desktop/SMOKE.md`

V6 and v7 sweeps remain immutable superseded evidence. N4 has not invoked the
single v8 replacement DEC-025 sweep and awaits manager release after sealed
pre-sweep validation. N4 makes no COMMIT-SAFE claim.

## Candidate v9 verification-only continuation

Candidate v9 authorized verification only and introduced no implementation,
product, test, schema, dependency, decomposition, or Cargo change. The
canonical dirty-product hashes remained exact before and after the full gates:
34 files excluding `apps/desktop/SMOKE.md` at
`0c73787560e06d6d07393f16d8d215f484d6d3fc9801ff9a3f76bae938e04ff4`,
and 35 files including it at
`e7eec87ab4630f8cc22c94de4987515b2fc2b4f284790bd2833da4ab1908d3ae`.

Focused spring-hanger verification passed desktop 15/15, Python 28/28, and
Rust 56/56. A read-only runtime probe proved exact
`constant_load -> hot_load -> cold_load -> installed_load` precedence, exact
force quantity shape, `required_for` reporting plus human review only, and
missing data when no force field exists.

The full union passed desktop Vitest 516/516, piping pytest 539/539,
practitioner-harness pytest 311/311, the 36-manifest Cargo profile, repository
self-check, desktop build, source H4 20/20, and production-dist H4 2/2. The
extra full desktop-native Rust result was 77/78 with only the preserved
frozen-base rule-pack notice assertion.

Fresh packaged-native cancel, new save, and real same-path macOS replacement
passed against packaged executable SHA-256
`49ca370ed7ce54970f0f50356569296777440bbbad87ac69eaf6b93ba8a6d51a`.
Both saves produced 3518303 bytes, container SHA-256
`e0145a9fb7d377034c7876a92336581ecdeb5d737fe223c5b39d560adfc42cc7`,
and package identity
`fb81914e2d9654876eb9421634f1d4ab1e7a14f2fa056b52fe6eb330d19b125a`.
Replacement changed the inode from 160744815 to 160744889; all member hashes
and lengths, exact InputManifest identity, native SH-140 force semantics, and
no-temp cleanup passed.

Exact v9 evidence:

- `CHECK_v9_full_gates.txt`
- `CHECK_v9_native_terminal_save_replace.txt`
- `CHECK_v9_pre_sweep_gate_summary.txt`

V6 through v8 sweep artifacts remain immutable superseded evidence. N4 has not
invoked the one v9 sweep and awaits manager release after terminal static and
containment review. N4 makes no COMMIT-SAFE claim.

Terminal v9 static validation passed claims language (268 files), path anchors
(952 live surfaces), piping receipts (frozen through Receipt-44), R16 JSON
(3 files), N4 raw evidence (21 files), WRITE_MATRIX containment (94 dirty
paths, zero violations), staged and unstaged diff checks, and zero staged
paths. The two unchanged-product hashes remained exact after evidence sealing.
