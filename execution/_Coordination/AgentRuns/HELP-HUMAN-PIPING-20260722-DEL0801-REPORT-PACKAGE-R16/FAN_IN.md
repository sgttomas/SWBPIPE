# R16 pre-effect fan-in

## Reconnaissance

- N1 `SUCCESS`: existing producer, payload types, contracts, redaction gap,
  desktop bridge gap, and checks mapped at `instances/N1/RETURN.md`.
- N2 `SUCCESS`: dual-menu route, missing picker/save capability, atomic-save
  semantics, H4 limitations, and checks mapped at `instances/N2/RETURN.md`.
- Product files remained unchanged; the two returns are disjoint evidence.

## Manager disposition

The candidate freezes the four reconnaissance unknowns as follows:

1. explicit command-local owned DTO mapping; no core producer/schema change;
2. v1 `public_report` was refuted; v2 uses the existing `local_private`
   explicit-intent contract with evidence retained outside the package;
3. backend-only Tauri dialog plugin with no frontend filesystem grant;
4. macOS-bounded atomic replacement, reject symlinks, pre-rename rollback,
   and truthful post-rename durability-uncertain status.

N3B refuted v1 on incomplete/TBD DEL-08-04 preview inputs, contradictory public
lossless routing, insufficient DEL-08-06 validation, and absent packaged-native
evidence. V2 instead maps actual session truth directly, requires Python/TS
DEL-08-06 parity, uses explicit-intent local-private control, and adds the
packaged-macOS smoke gate. N3C refuted v2 because active rule-pack metadata is
private to RuleCheckRunPanel and absent from the authorized App/report state.
V3 keeps the fence bounded: `RULE_INPUTS_INCOMPLETE` sessions may proceed with
an honest empty rule-pack list; `USER_RULE_CHECKED`/`USER_RULE_FAILED` sessions
block before assembly with a stable diagnostic. The exact candidate remains a
proposal pending fresh refutation and owner adoption. N4 is held.

N3D refuted v3 because a non-null active-pack run may itself be
`RULE_INPUTS_INCOMPLETE`, and because current report/analysis metadata still
carried invented/public labels plus a public-profile ID. V4 allows empty refs
only for null aggregate + solve-time incomplete status, blocks every non-null
aggregate, binds solver identity to existing product-physics component APIs,
uses a truthful local-private profile, and corrects current-session provenance
only in the owned report/package projections. N4 remains held pending fresh
refutation and owner adoption.

N3E refuted v4's absolute containment wording because the path-based design
cannot prevent adversarial concurrent parent/destination swaps. V5 retains
static symlink preflight and same-parent temp/rename, but explicitly narrows the
boundary to a non-adversarial local user and makes the TOCTOU residual visible
in the UI/receipt/run record/handoff. No race-free containment claim or hidden
hardening queue remains. N4 remains held pending fresh refutation and adoption.

N3F independently returned `COMMIT-SAFE` for v5. Pre-effect fan-in is complete.
N4 remains held solely on the owner adoption act quoted by the candidate.

## N4 implementation fan-in

The owner adopted v5 and authorized the exact `WRITE_MATRIX.csv`, with N4 as
the sole serialized implementation owner. N4 returned `BLOCKED`, not
`IMPLEMENTED` or `COMMIT-SAFE`.

Accepted provisional evidence before the blocker:

- focused desktop Vitest: 77/77 passed;
- Python DEL-08-06 parity: 11/11 passed;
- Rust report-package/PDF/renderer: 28/28 passed;
- focused atomic-save tests: 4/4 passed;
- source H4: 2/2 passed;
- production-dist H4: 1/1 passed;
- packaged debug application build: passed;
- full desktop Rust: 65/66, with its sole rule-pack notice failure reproduced
  unchanged from the frozen clean base.

The mandatory packaged-native cancel/save/replace and bridge end-to-end gate
could not be exercised. Sky enumerated native menu items but could not activate
even the known-visible `File -> List Local Projects` command through direct AX
or keyboard interaction. The explicit System Events fallback exited 1 with
assistive-access error `-1719`. This establishes a verification-environment
blocker, not a product native-menu defect.

HELP_HUMAN independently attempted the dedicated Mac control path and observed
the same condition: the packaged app and its native File/View items were
visible, but menu activation produced no React state change. The Mac then
locked, and the control runtime requires a manual human unlock. This
cross-check preserves the blocker classification without converting it into a
product-defect claim.

Manager disposition:

- all N4 product changes remain provisional and unaccepted;
- broad registered checks were not begun;
- DEC-025 sweep count is exactly zero;
- N5 is not eligible and was not dispatched;
- W3 is not eligible and no DEL-08-01 status, memory, or run-record closeout
  was performed;
- DEL-10-05 remains undispatched.

Exact blocker evidence is preserved in `instances/N4/RETURN.md` and
`instances/N4/CHECK_packaged_native_blocker.txt`. Resume requires an authorized
native-menu actuator for the packaged process, followed by the full native
default-off intent, cancel, save, replace, cleanup, receipt, and digest
sequence before broad gates or the single acceptance-eligible DEC-025 sweep.

## N4 resumed implementation and terminal pre-sweep fan-in

The earlier verification-environment classification was superseded after the
human unlocked the Mac. HELP_HUMAN proved the same AX actuator executed built-in
macOS menu items while custom app menu items produced no React state change.
N4 resumed as the sole serialized implementation owner and repaired four
contained defects within the adopted write matrix:

1. custom native menu commands were dropped because the packaged capability
   set did not provide the frontend Tauri event subscription;
2. the spring-hanger desktop adapter emitted a quantity and `required_for`
   vocabulary that did not match the accepted report-sections contract;
3. the blocking save dialog ran on the main thread rather than the async
   command runtime required by `tauri-plugin-dialog`;
4. post-rename parent-directory durability could hang indefinitely, so it is
   now bounded, fail-honest, and guarded against worker accumulation.

The final guarded packaged bundle passed native load/solve, View > Report,
default-off private intent, explicit enable, cancellation/no-effect, new save,
and same-path replacement. Both saves produced identical deterministic bytes
and hashes; replacement changed the inode and left no temporary sidecar.
Archive integrity, package identity, and all six member sizes/hashes passed.

Broad fan-in:

- registered desktop build: `PASS`;
- registered desktop test initial run: one non-reproducible failure; focused
  rerun and full rerun passed 28 files / 502 tests;
- practitioner harness pytest: `PASS`;
- repository self-check: `PASS`;
- claims, paths, receipts, JSON/JSONL, raw evidence, containment, and diff
  validators: `PASS`;
- piping pytest: 531/535 passed; all four failures are in
  `tests/test_release_readiness_script.py` and hard-code DAG-007 although the
  exact frozen SHA already points `_LATEST.md` to accepted DAG-008;
- a clean archive of frozen SHA `8698b0338ac82556fee583dd3f85bb62d0b74f85`
  reproduced the identical four failures (six tests in that file passed).

Manager disposition: the frozen-base mismatch is outside the adopted N4 write
matrix, but it does not release the acceptance-eligible sweep. Candidate §7
requires piping pytest, and the DEC-025 sweep would deterministically fail the
same four tests. N4 therefore sealed product status `IMPLEMENTED`, verification
status `BLOCKED_BEFORE_SWEEP`, and sweep count zero. N5 and W3 remain held.

Unblock by separately authorizing the DAG-008 expectation correction in
`tests/test_release_readiness_script.py`, or explicitly amending the acceptance
rule. Then rerun full piping pytest and, if green with no further terminal-state
change, run exactly one acceptance-eligible DEC-025 sweep.

## V6 sweep, fresh N5 block, and accepted v7 remediation fan-in

The owner authorized v6's one-file DAG-008 expectation correction. Focused
release-readiness tests passed 10/10 and full piping passed 535/535. The sole v6
DEC-025 sweep ran once and passed, then fresh read-only N5 returned `BLOCK` on
four semantic findings: a result hash mislabeled as input-manifest evidence,
stiffness quantities assigned force/moment dimensions, invented-public
classifications copied into current-private records, and insufficient SHA-256
shape validation. W3 remained held.

SOFTWARE_DECOMP produced run-local derived plan
`R16-SOFTWARE-DECOMP-REMEDIATION-PLAN-001`. The owner accepted it without
canonical decomposition publication, adopted v7, expanded the exact matrix,
kept N4 as sole serialized owner, and authorized one replacement sweep after
green gates/native proof.

N4 completed serialized slices 01–05:

- DEL-08-02 deterministic current-session InputManifest/ref/exact hash;
- DEL-14-02 manifest binding and explicit source result dimensions;
- DEL-08-04 additive `linear_stiffness`/`rotational_stiffness` schema and Rust
  exporter support;
- DEL-08-01 actual manifest/dimension consumption, private package-owned
  projection, and exact lowercase 64-hex validation at TS/Rust boundaries;
- focused/full/browser/native verification.

Terminal v7 evidence:

- desktop Vitest 510/510, piping pytest 537/537, practitioner harness 311/311,
  self-check, production build, H4 4/4, and focused reporting Rust 44/44 pass;
- extra full desktop Rust 73/74 retains only the exact frozen-base unrelated
  rule-pack notice wording residual; no excluded rule-pack path changed;
- fresh native cancel/new-save/same-path replacement passed; replacement is
  durable, changed inode `160629227` to `160633120`, preserved exact container
  SHA/package identity, and left no temp residue;
- post-replacement package proves InputManifest/result-hash distinction,
  private copied provenance, explicit stiffness dimensions, exact SHA values,
  six-member schema preservation, and no portable-replay claim;
- claims 268, path anchors 950/0, receipt, JSON, raw evidence, diff checks, and
  exact 84-path/42-row WRITE_MATRIX containment pass with zero violations.

Manager independently reproduced the 84-path zero-violation containment result,
verified the immutable v6 sweep hash, and confirmed replacement sweep count is
zero. All pre-sweep records are now sealed. The v6 sweep remains immutable
superseded evidence. N4 is released to invoke exactly one v7 replacement
DEC-025 sweep and may make no filesystem edit afterward. Fresh N5 and W3 remain
held until the replacement sweep returns terminal evidence.

## V7 replacement sweep, fresh N5 block, and v8 closure fan-in

The v7 replacement DEC-025 sweep ran exactly once and passed. Fresh read-only
N5 confirmed all four prior semantic defects closed, then returned `BLOCK` on
three narrower implementation findings: suffix-only InputManifest ref
validation, remaining unit-text family inference, and predictable temp names
with incomplete pre-rename fault evidence.

The owner authorized sealed v8 without decomposition/schema expansion. N4
closed only those findings:

- exact model-token/digest InputManifest ref equality in TypeScript/Python,
  with wrong-prefix/wrong-model valid-digest negatives;
- kind/component plus explicit-dimension result-family mapping, with no unit/id
  family branches and deceptive-unit/mismatch tests;
- direct `getrandom = "0.3"` root dependency (already transitively locked),
  OS-random create-new temp tokens, and injected write/flush/temp-sync/rename/
  cleanup tests proving destination preservation, cleanup, and primary error
  retention.

V8 gates pass: desktop 516/516, piping 539/539, harness 311/311 plus
self-check, source/dist H4 4/4, build/type, focused Rust union 56/56, and fresh
packaged-native cancel/new-save/replacement. Replacement is durable, changed
inode `160685423` to `160685542`, preserves deterministic container/package
hashes, and leaves no temp residue. The getrandom lock delta is exactly the
direct desktop-root dependency on already locked `getrandom 0.3.4`.

Static validation passes: claims 268, path anchors 951/0, receipt, JSON/raw
evidence, diff checks, staged zero. Manager independently verified exact 89
dirty paths against 42 matrix rows with zero violations and confirmed exactly
the two immutable superseded v6/v7 sweep artifacts with unchanged hashes.

All v8 pre-sweep records are sealed. V8 sweep invocation count is zero. N4 is
released to invoke exactly one v8 replacement DEC-025 sweep and may make no
filesystem edit afterward. Fresh N5 and W3 remain held pending its terminal
result.

## V8 fresh N5 authority block and v9 ratification fan-in

The v8 replacement DEC-025 sweep ran exactly once and passed. Fresh read-only
N5 independently verified all three v8 implementation findings closed, then
returned `BLOCK` solely because WRITE_MATRIX row 6 did not authorize the
existing spring-hanger semantic changes in
`apps/desktop/src/features/report/renderableReportInput.ts`.

The owner adopted a sealed v9 authority amendment limited to that existing
behavior:

- select spring-hanger force using
  `constant_load -> hot_load -> cold_load -> installed_load`;
- emit `{magnitude, unit, dimension: "force"}`;
- limit `required_for` to `reporting` and `human_review`;
- report missing data when no force quantity exists.

No source, schema, decomposition, or other semantic change was authorized.
The amendment was materialized before any v9 gate. N4 made no product or test
change: the exact 34-file product/test hash remains
`0c73787560e06d6d07393f16d8d215f484d6d3fc9801ff9a3f76bae938e04ff4`,
and the 35-file hash including unchanged `SMOKE.md` remains
`e7eec87ab4630f8cc22c94de4987515b2fc2b4f284790bd2833da4ab1908d3ae`.

N4 repeated the affected, full, and native gates against the ratified scope:

- focused report/render adapter Vitest 15/15, Python 28/28, Rust 56/56;
- in-memory spring-hanger precedence, force-shape, `required_for`, and
  missing-data assertions all pass;
- desktop 516/516, piping 539/539, practitioner harness 311/311 plus
  repository self-check, release Cargo across all 36 manifests, source H4
  20/20, dist H4 2/2, and production build/type all pass;
- extra full desktop Rust is 77/78 with only the exact frozen-base,
  unmodified rule-pack notice wording residual;
- fresh packaged-native cancel, new save, and same-path replacement pass with
  durable replacement, inode change, deterministic container/package hashes,
  six-member integrity, SH-140 `hot_load=390 N`/force binding, and no temporary
  residue.

Terminal pre-sweep validation passes: claims 268, path anchors 952/0,
Receipt-44, R16 JSON, raw N4 evidence, diff checks, staged zero, and exact
94-dirty-path containment against all 42 WRITE_MATRIX rows with zero
violations. Manager independently reproduced that containment result and
confirmed exactly three immutable prior sweep artifacts (v6, v7, v8) with
unchanged hashes.

All v9 pre-sweep records are sealed. The v9 replacement sweep invocation count
is zero. N4 is released to invoke exactly one v9 replacement DEC-025 sweep and
may make no filesystem edit afterward. Fresh N5 and W3 remain held pending the
terminal sweep result.

## V9 terminal sweep, fresh N5, W3, and final manager fan-in

N4 invoked exactly one v9 replacement DEC-025 sweep and made no filesystem
edit, repair, stage, or commit afterward. The terminal artifact is
`validation/evidence/sweeps/SWEEP_20260723T054036Z_8698b0338ac8-dirty.json`,
SHA-256
`9f876f998e6f52ce4473efa72879f5f48a2ed900c6b4249539ef1e8ae2af7374`.
It is `openpipestress.evidence_sweep_summary` schema version 2 with overall
status `pass`. Cargo crate sweep, Python pytest, desktop Vitest, source/dist
Playwright, and desktop production build all passed. Runtime was 314.395
seconds.

Manager post-sweep checks independently confirmed:

- exactly four R16 sweep artifacts with their recorded hashes;
- exact 95 post-sweep dirty leaf paths, all under the working root and covered
  by the 42-row matrix;
- staged zero and both diff checks passing;
- no product, control, or evidence file newer than the v9 sweep at the N5
  dispatch boundary.

Fresh read-only N5-v9 returned `COMMIT-SAFE`. Its independent return confirms:

1. the owner act, v9 amendment, and WRITE_MATRIX row 6 authorize exactly the
   existing spring-hanger behavior and no further semantic expansion;
2. the 34-file and 35-file unchanged product hashes reproduce exactly;
3. all prior manifest, dimension, private-projection, exact-hash, and atomic
   save findings remain closed;
4. the v9 sweep and all prior sweep hashes reproduce exactly;
5. the retained native package passes six-member integrity, member hashes,
   manifest binding, stiffness dimensions, current-private provenance,
   SH-140 `390 N` force rendering, no-portable-replay posture, and zero
   temporary residue;
6. runner, DEL-10-05, DAG, lifecycle, status, memory, and W3 paths were
   untouched at its verification boundary.

N5 released only DEL-08-01 W3 closeout. W3 then changed exactly:

- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/_STATUS.md`;
- the same deliverable's `MEMORY.md`;
- the same deliverable's
  `_run_records/WORKING_ITEMS_RUN_2026-07-22_DEL0801_REPORT_PACKAGE_R16.md`.

W3 preserved `IN_PROGRESS`, removed only the evidenced report-package seam
residual, and retained the component-provenance hardening and `.opsproj`
compatibility-window/versioning residuals. It made no other change.

Manager post-W3 fan-in passed: claims-language 268, path anchors 952/0,
Receipt-44, practitioner self-check, both diff checks, staged zero, and exact
98-path containment against the 42-row matrix with zero violations. Exactly
the three W3 paths were newer than the v9 sweep before final durable
message-return and handoff projections.

Final WORKING_ITEMS verdict: `ACCEPTED_FOR_CHANGE_CLOSEOUT`.

Derivative-package status: this R16 run root is coordination/evidence derived
from accepted DAG-008, sealed candidate v9, live code, and deliverable-local
truth. It does not replace decomposition, dependency, lifecycle, release,
issuance, or publication authority.

Rerun posture:

- no implementation rerun is required if CHANGE closes the exact validated
  state;
- any product/test/schema/Cargo change requires the appropriate full gate
  union, a new replacement sweep, and fresh N5;
- any closeout-record change requires renewed claims, paths, receipt,
  containment, and diff checks.

Remaining residuals are the two DEL-08-01 `Remaining` rows, current-session
integrity without portable replay, bounded fail-honest parent-directory
durability, and the unchanged frozen-base rule-pack assertion. No package
blocker remains. No lifecycle, release, issuance, runner, DEL-10/DEL-10-05,
DAG, dependency, decomposition, professional-approval, certification, sealing,
authentication, or code-compliance effect is made.
