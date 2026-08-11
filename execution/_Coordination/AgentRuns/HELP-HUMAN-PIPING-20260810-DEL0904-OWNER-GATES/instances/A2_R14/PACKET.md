# R14 Clean-Checkout Reproduction Acceptance Packet

**Prepared:** 2026-08-10

**Prepared by:** A2-R14, bounded non-delegating Agent 2 under WORKING_ITEMS

**Status:** `OWNER DECISION SUPPORT — NO RULING OR APPLICATION`

**Accepted current-base read:** `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`

**Bundle:** `validation/evidence/reproduction/REPRO_DEL0904_20260720T074714Z_a5235340aae3/`

## 1. Decision requested

Decide whether to accept, decline, defer, or narrowly qualify acceptance of
the committed `INTERNALLY_VERIFIED` R14 reproduction bundle. The evidence
supports acceptance of the bundle as a source-pinned record of the
actor-neutral clean-checkout reproduction at
`a5235340aae3c41cf227f5617e593b268936f6b3`. It does **not** support an
unqualified statement that the current base `c05fe2d6…` has been reproduced:
the controlling brief's explicit rerun triggers have fired since the run.

**Non-binding recommendation:** **O-B — ACCEPT, QUALIFIED TO THE PINNED R14
SOURCE.** Record that the owner accepts this exact bundle as adequate evidence
that P1-P16 passed at `a5235340…` under the pinned procedure, while explicitly
declining to treat it as current-head reproduction evidence. Leave its evidence
label `INTERNALLY_VERIFIED`. A fresh clean-checkout bundle is required before
making a present-base reproduction claim.

This recommendation separates two propositions that the committed evidence
supports differently:

1. **Historic/source-pinned proposition — supported:** the recorded run was a
   clean, offline, actor-neutral execution of the documented procedure at the
   R14 source, and P1-P16 passed.
2. **Current-base proposition — not demonstrated:** the same predicates pass
   at `c05fe2d6…` under today's runner, product-physics, mechanics-suite,
   contract-test, workflow-profile, DAG, and toolchain state.

## 2. Stable execution basis and complete bundle inventory

### 2.1 Source, procedure, brief, profile, and bundle identities

| Item | Stable identity | Finding |
|---|---|---|
| Pinned source commit | `a5235340aae3c41cf227f5617e593b268936f6b3`; tree `071da0894065e465f43ea2204e09dd728e413ae2` | Ancestor of the accepted current base. Commit date `2026-07-20T01:22:10-06:00`. |
| Adopted execution brief | `CB-2026-07-20-DEL-09-04-CLEAN-REPRO-R14-001`; effective-state SHA-256 `702b56012c3d4afe7fe64a1dea4f703beb23571a9abe18f796282ada527539d4`; current Git blob `378cbff2553f48adc19c1b3a05039b1dfa05c0ea` | Adopted under D-52/`DEC-085` as refined by D-54/`DEC-087`; verifier recorded `COMMIT-SAFE`. |
| Documented procedure | `docs/validation_manual/headless_runner_reproduction.md`; SHA-256 `fa714cf44d5c3e8a54ff6e2f6883676b81e01755e2e07d36a5bd118576b299c1`; Git blob `f585f1967a8d0839cb7ec023c4992569530dc9cb` | Source and current-base bytes are identical. |
| Source workflow profile | `software-workflow.json`; SHA-256 `123249634475e87207cd75740dc25e5061c08cc7a1708aa239105b27e30c9c2f`; source blob `ab5768a7383ec119556accb01912c04a85731458` | Pinned by the manifest; current profile differs (currency finding below). |
| Source runner manifest | `core/runner/headless/Cargo.toml`; SHA-256 `803facda9059f0bec58d78f0cff977df8a44835261bd4e6fa31af07a155fb1f0`; source blob `43c81cf7a69cf8493a2c1e66d1f78c705d47b06e` | Path dependencies bind the runner to product physics, result export, canonical JSON, and the mechanics/stress/nonlinear suite crates. |
| Source contract test | `tests/test_headless_runner_contract.py`; SHA-256 `7997562605928c5536470fb31de094f3ccc10cc6ba65e8115e3814a28915f733`; source blob `ba3864e8ec097a3a0ea3311ec36da36bad7d6ef2` | Executed as P12. Current bytes differ. |
| Bundle introduction | commit `4ff617ae123131a1c0152ad8fa42a46fbe1b305d` on parent `a5235340…` | The complete bundle first entered Git here. |
| Bundle Git tree | `3d847390dfa74f8dced090164fb95f31eade83c7` at both introduction and current base | Proves no bundle byte has changed since introduction. |
| Bundle checksum index | `SHA256SUMS.txt` SHA-256 `581f039ffaca00097de75c151dcd13f9403c71076ab06671cd8e94c069f7ec7a`; Git blob `d05cbccdac55888ba848f1e0e7288fe03139799f` | All 74 indexed files independently reverified `OK`; the index is the 75th file. |
| Manifest | SHA-256 `9bd19f9f6a3a8b7730ce3d2acab5347ab1502a81d484bc3bbce02693caf3398d`; Git blob `4bdc36d080b0a8084fefee263a38be09134aed4d` | Declares derivative status (`authoritative_truth: false`), `PASS`, and `INTERNALLY_VERIFIED`. |
| Bundle README | SHA-256 `dc9731166cdbb6cd298e74cdbf282a444c14b75412a3691198388de8aa2e6ff8`; Git blob `f5b56e6163c7504152638dc7d1e608938513100c` | States adoption basis and explicitly says no owner acceptance occurred. |

The bundle contains **75 regular files / 1,761,024 bytes**. The complete
inventory is:

- Ten root records: `README.md`, `SHA256SUMS.txt`, `commands.jsonl`,
  `environment.txt`, `git-status-before.txt`, `git-status-after.txt`,
  `manifest.json`, `procedure.sha256`, `source_commit.txt`, and
  `validation_summary.json`.
- Six check records: `checks/change-scope.json`,
  `checks/evidence-sweep.json`, `checks/sweeps-before.txt`,
  `checks/sweeps-after.txt`, `checks/sweeps-delta.txt`, and
  `checks/witness-compare.json`.
- Eight captured outputs: the three `tp_runner_015_*` outputs and five
  `del1005_*` outputs named in §3 below.
- Seventeen files in each of `stdout/`, `stderr/`, and `exit_codes/`, one for
  each recorded command: `cmd01_generate_tp_runner_015_inputs`,
  `cmd02_scoped_diff_tp_runner_015_inputs`, `cmd03_solve_case1`,
  `cmd04_validate_input_case2`, `cmd05_run_benchmark_case3`,
  `cmd06_generate_del1005_inputs`, `cmd07_scoped_diff_del1005_inputs`,
  `cmd08_benchmark_single_case`, `cmd09_benchmark_multi_case`,
  `cmd10_regression_full_suite`, `cmd11_benchmark_payload_missing`,
  `cmd12_regression_payload_missing`, `cmd13_cargo_test_headless`,
  `cmd14_contract_test`, `cmd15_post_status_untracked_all`,
  `cmd16_post_diff`, and `cmd17_post_diff_cached`.

`SHA256SUMS.txt` is the canonical per-file hash inventory for all 74 other
files. Its successful verification plus the stable Git tree proves zero
missing, extra, or altered bundle files at the accepted current base.

### 2.2 Tool and dependency identity

The bundle's `environment.txt` records:

- Git `2.50.1` at the `command -v` path captured in `environment.txt`;
- Python `3.13.7` at its captured `command -v` path;
- Rust `1.92.0` (`ded5c06cf21d2b93bffd5d884aa6e96934ee4234`), LLVM
  `21.1.3`, at its captured `command -v` path;
- Cargo `1.92.0` (`344c4567c634a25837e3c3476aac08af84cf9203`),
  libgit2 `1.9.1`, at its captured `command -v` path;
- macOS `26.5.2` build `25F84`, Darwin `25.5.0`, arm64.

Every Cargo invocation was offline with `CARGO_NET_OFFLINE=true`, `--offline`,
and a temporary target outside the clone. The runner manifest bound source
path dependencies at `a5235340…`; captured Cargo stderr records the resolved
external package versions (including `serde 1.0.229`, `serde_json 1.0.150`,
`sha2 0.10.9`, and their transitive dependencies). No network, installation,
update, or provisioning occurred. The §4.4 ignored-lockfile contingency was
not used: no `Cargo.lock` was copied into the clone. Consequently, the source
commit, manifest, environment record, and compile logs identify what ran, but
the bundle does not supply a preserved lockfile for a future dependency
resolution. That is a currency/repeatability caveat, not a defect in the
recorded run.

## 3. P1-P16 predicate reconstruction

`validation_summary.json` and `manifest.json` record all sixteen as `PASS`.
The following table ties every result to direct committed witnesses; output
content and hashes were independently inspected, not inferred from the summary
alone.

| Predicate | Result | Direct committed witness and independently checked observation |
|---|---|---|
| P1 — `tp_runner_015` generator determinism | `PASS` | `commands.jsonl` cmd01/cmd02 and their exit files record `0/0`; cmd02 scoped the three named inputs and produced an empty diff. |
| P2 — case 1 solve | `PASS` | `exit_codes/cmd03_*.txt` is 0; `outputs/tp_runner_015_solve.json` has job `COMPLETED`, empty request/result validation diagnostics, 830 result refs, and exactly one `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` occurrence at `mechanics_envelope.diagnostics[4]`, severity `warning`, affected refs `support:CE-120` and `node:N-120`. |
| P3 — case 1 witness relationship | `PASS` | `checks/witness-compare.json`: regenerated SHA-256 `b3cd85af85655eadb827f366457494387ba4b58807fd5608c676958b37168613` differs from historical witness `c406d9c2d8b6e739cd8faf86fcd67ff8f685342f9ee056b5544685a769705188`, exactly as the dated procedure note requires. |
| P4 — case 2 validate-input | `PASS` | cmd04 exit 1; `outputs/tp_runner_015_validation_blocking.json` contains `HEADLESS_RUNNER_LOAD_BASIS_MISSING` in request validation and `runner_result: null`. Its informational witness mismatch is recorded, not treated as a predicate. |
| P5 — case 3 payload-less benchmark | `PASS` | cmd05 exit 1; `outputs/tp_runner_015_benchmark_stub.json` contains blocking `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`. The expected mismatch from the pre-#287 stub witness is recorded in `checks/witness-compare.json`. |
| P6 — `del1005` generator determinism | `PASS` | cmd06/cmd07 exits `0/0`; cmd07 scoped the five named inputs and produced an empty diff. |
| P7 — single mechanics benchmark | `PASS` | cmd08 exit 0; `outputs/del1005_benchmark_single_case.json` records mechanics, requested 1, matched 1, blocked 0, `MECH-TP-PHYS-004-LOAD-TO-RESULTANT` `executed_and_matched`, diagnostics empty; SHA-256 `813a702b…` equals its committed witness. |
| P8 — three stress benchmarks | `PASS` | cmd09 exit 0; `outputs/del1005_benchmark_multi_case.json` records stress 3/3 matched, 0 blocked, all three named cases matched, diagnostics empty; SHA-256 `8feb3d25…` equals its committed witness. |
| P9 — nonlinear whole-suite default | `PASS` | cmd10 exit 0; `outputs/del1005_regression_full_suite.json` records `whole_suite_default_applied: true`, 5/5 matched, 0 blocked, diagnostics empty; SHA-256 `2f89adce…` equals its committed witness. |
| P10 — benchmark payload missing | `PASS` | cmd11 exit 1; `outputs/del1005_benchmark_payload_missing.json` contains blocking `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`; SHA-256 `9596c052…` equals its witness. |
| P11 — regression payload missing | `PASS` | cmd12 exit 1; `outputs/del1005_regression_payload_missing.json` contains blocking `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING`; SHA-256 `61cba4f2…` equals its witness. |
| P12 — documented review checks | `PASS` | cmd13 (`cargo test --offline --manifest-path core/runner/headless/Cargo.toml`) and cmd14 (`python3 tests/test_headless_runner_contract.py`) both exited 0; stdout/stderr and exit files are checksum-bound. |
| P13 — clean-checkout discipline | `PASS` | `git-status-before.txt` proves local-only detached HEAD `a5235340…`, empty porcelain including all untracked files, and clean working/index diffs; `git-status-after.txt` plus cmd15-cmd17 prove the same after execution. |
| P14 — bundle integrity and label | `PASS` | Complete 75-file inventory above; all 74 indexed checksums verify; README and manifest use `INTERNALLY_VERIFIED` only with overall `PASS`, identify derivative status/adoption basis, and state that no owner acceptance occurred. |
| P15 — bounded state update | `PASS` | Commit `4ff617ae…` changes only the governed brief/bundle/sweep/W5/three DEL-09-04 state surfaces. Independent `git show` comparison proves `## Remaining` byte-identical before/after; lifecycle stays `IN_PROGRESS`; one History, one Memory entry, and one run record were added; the commit has no `LOOP_RECEIPTS.md` delta. |
| P16 — closeout checks | `PASS` | `checks/evidence-sweep.json` passes and `sweeps-*.txt` prove exactly one new sweep; `checks/change-scope.json` reports 80 paths and zero violations against seven allowed entries. Manifest records claims/path/receipt/diff/JSON checks PASS. W5 manager closeout additionally records piping pytest 507 passed, harness pytest 311 passed after preserving and curing one portability-only first-attempt failure, and harness self-check PASS; no bundle bytes changed in that cure. |

The seventeen command exits were exactly
`0,0,0,1,1,0,0,0,0,0,1,1,0,0,0,0,0`; every expected/actual pair matches.

## 4. Actor-neutral R6 acceptance semantics

PRD §24 R6 requires that validation examples reproduce from a clean checkout
by following the documented validation-manual procedure with the environment,
tool versions, commands, exit codes, and output hashes recorded. It explicitly
makes the reproduction actor-neutral: a maintainer or an agent may execute it.
Therefore, the fact that a governed agent executed this run is not a deficiency
and no independent-third-party executor is required by this milestone
criterion. PRD §22.1 separately places independent third-party reproduction in
the publication-era credibility lane.

An owner **acceptance of this bundle** would assert only that the owner accepts
the exact committed derivative package as adequate evidence that its pinned
source/procedure run satisfied P1-P16. Under O-B it would be explicitly
source-pinned and historic.

It would **not**:

- relabel the bundle `PROVER_CORRELATED`, `ENGINEER_ACCEPTED`, or
  `MAINTAINER_REVIEWED`; `INTERNALLY_VERIFIED` remains the evidence label;
- constitute the future human acceptance record contemplated by PRD §21.3,
  accept any calculation for professional reliance, or replace responsible
  engineer/project-authority judgment;
- establish external-prover correlation, independent-third-party
  reproduction, or current-base/current-head reproduction;
- accept or promote public-benchmark release tolerances under `DEC-046`;
- promote any case page, supply GUI-workflow validation evidence, or resolve
  the export/CAEPIPE TBD cluster;
- change the D-61 reliance hold, lifecycle, target stage, issuance, release,
  publication, or release-readiness posture;
- by itself close `TM-PIP-037`, whose trigger is conjunctive and also requires
  the separate `DEC-046` value-promotion disposition.

PRD §21.3 requires any future human acceptance record to remain separate from
solver, rule, comparison, and report outputs. This owner-gate record should
therefore be a governance ruling that cites the immutable bundle, not an edit
to the bundle and not a new result label.

## 5. Three-week currency assessment at `c05fe2d6…`

The source commit is approximately three weeks older than the accepted base
(`2026-07-20` to `2026-08-10`). The controlling brief §8 supplies the relevant
currency law.

### 5.1 Historic validity

**No inspected change invalidates or reinterprets the historic run.** The
source commit remains reachable, the complete bundle Git tree is byte-identical
to its introduction, all checksums verify, and all direct predicate witnesses
remain present. Later changes cannot turn the recorded P1-P16 results at
`a5235340…` into failures.

### 5.2 Changes that make the bundle non-current and trigger a fresh run

| Triggered surface | Concrete Git evidence | Effect |
|---|---|---|
| Headless-runner manifest/source/dependencies | Runner tree `d5b2ae2c…` → `c349ee45…`; commits `9457565c2` (redaction controls) and `f82bb28e2` (report-package `export-results` binding); current manifest blob `b2a37123…` adds `open_pipe_stress_report_package`. | Explicit brief §8 runner/manifest/dependency trigger. The procedure's eight commands were not rerun against these bytes in this packet. |
| Product-physics source exercised by solve | Tree `dcef46cd…` → `f609ba62…`; commit `c394365ca` implements DEC-092 temperature-indexed shear modulus and changes `core/product_physics/src/lib.rs`. | Explicit solver-surface trigger. Low-level `core/solver/**` is unchanged, but the directly linked product-physics layer is not. |
| Mechanics suite bound by P7 | Tree `b93b556f…` → `bfed462c…`; `c394365ca` changes inventory 24 → 25 and adds `MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION`. | Explicit bound-suite trigger. The named P7 case bytes/fixture input remain unchanged, but current suite membership changed. |
| Contract test | Source blob `ba3864e8…` → current `a84d089b…`; changed by `9457565c2`, `f82bb28e2`, and `82d64b63d`. | Explicit contract-test trigger; historic P12 does not prove the current test at current source. |
| Workflow profile | Source blob `ab5768a7…` / SHA `123249…` → current blob `71feb5d6…` / SHA `3a6fd86b…`; `82d64b63d` adds pytest xdist arguments. | Explicit root-profile trigger, even though this particular delta changes check execution mechanics rather than the eight case definitions. |
| Applicable DAG pointer | `DAG-007` blob `fdc6d39d…` → `DAG-009` blob `5441c712…`, through `ab7f7c6b1` and `9af422d01`. | Explicit approved-DAG pointer trigger. It does not invalidate the run, but current governance basis differs. |
| Toolchain/environment comparison | Current read-only inventory: Git 2.55.0, Python 3.9.6, Rust/Cargo 1.97.1, LLVM 22.1.6, macOS 26.6 build 25G72 on a different named host; all differ materially from the recorded run environment. | Explicit environment/toolchain comparison trigger. No current run was performed, so no cross-run equivalence is asserted. |

These triggers are sufficient individually under brief §8; together they make
the bundle non-current for a present-base reproduction assertion.

### 5.3 Unchanged surfaces and bounded caveats

- The procedure remains byte-identical (`fa714cf4…`), as does PRD §24/§22
  authority (`docs/PRD.md` blob `d07cc61c…`).
- Both generators, all eight executed input fixtures, and all eight committed
  comparison witnesses are byte-identical between source and current base.
- The stress and nonlinear suite trees are unchanged; the low-level
  `core/solver/**` tree is unchanged.
- DEL-09-04's two `## Remaining` bullets are substantively the same as at the
  source; later history records have accumulated without changing this run's
  meaning.
- The unchanged procedure text now contains dated historical statements that
  are not current product inventory: it says mechanics carries 24 fixtures
  (current suite has 25) and says `export-results` remains the only structured
  stub (the binding landed in `f82bb28e2`). Those statements are dated/contextual
  evidence, but they reinforce why this packet must not call the R14 bundle a
  current-base reproduction.
- No fresh procedure execution was performed for this decision packet. A new
  clean-checkout run would require its own adopted brief/run ID and immutable
  bundle; an ad hoc live command would not replace that governed evidence.

## 6. Owner options

| Option | Ruling | Lawful effect | Consequence |
|---|---|---|---|
| **O-A — ACCEPT** | Accept the exact bundle without an additional currency qualifier beyond its existing source pin. | Records the owner gate as accepted for this source-pinned bundle. All ordinary claim fences still apply. | Risk of later readers over-reading acceptance as current-head evidence; not recommended. |
| **O-B — ACCEPT, QUALIFIED TO PINNED R14 SOURCE** | Accept the exact bundle as adequate evidence of P1-P16 at `a5235340…`, explicitly not current-base reproduction. | Closes the missing owner disposition for the R14 bundle while preserving its `INTERNALLY_VERIFIED` label and requiring a fresh run before any current-head claim. | **Recommended.** Best matches the strong historic evidence and the fired rerun triggers. |
| **O-C — DEFER** | Make no acceptance disposition until a fresh current-base clean-checkout bundle is prepared. | Preserves `TM-PIP-037`'s R14 sub-gate as unresolved. | Most conservative if the owner intends acceptance to carry current-head meaning. |
| **O-D — DECLINE** | Decline the R14 bundle as the accepted reproduction basis. | Records a negative disposition; bundle remains immutable historic evidence. | Appropriate only if the source-pinned evidence is judged inadequate despite P1-P16 and integrity passing. |

## 7. Exact ruling forms

### O-B recommended form

> **R14 REPRODUCTION RULING — ACCEPT, QUALIFIED TO PINNED R14 SOURCE.** I
> accept the immutable bundle
> `REPRO_DEL0904_20260720T074714Z_a5235340aae3` as adequate
> `INTERNALLY_VERIFIED` evidence that the actor-neutral clean-checkout run at
> source commit `a5235340aae3c41cf227f5617e593b268936f6b3`, under procedure
> SHA-256 `fa714cf44d5c3e8a54ff6e2f6883676b81e01755e2e07d36a5bd118576b299c1`,
> satisfied P1-P16. This acceptance is source-pinned: it does not assert
> reproduction at current base `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`,
> because the recorded §8 rerun triggers have fired. The evidence label remains
> `INTERNALLY_VERIFIED`; a fresh governed bundle is required before any
> current-head reproduction claim. Record this ruling verbatim. No tolerance,
> case-page, GUI-evidence, reliance, lifecycle, stage, issuance, release,
> publication, professional-acceptance, or external-prover effect is
> authorized.

### Other exact selections

- `R14 REPRODUCTION RULING — ACCEPT O-A AS PRESENTED.`
- `R14 REPRODUCTION RULING — DEFER O-C PENDING A FRESH CURRENT-BASE CLEAN-CHECKOUT BUNDLE.`
- `R14 REPRODUCTION RULING — DECLINE O-D AS THE ACCEPTED REPRODUCTION BASIS.`

The owner may amend any form. The final text must be preserved verbatim.

## 8. Conditional recording and application mechanism

This packet applies nothing. After an owner ruling:

1. WORKING_ITEMS records the exact owner text in the owner-gates run and cites
   this packet plus the immutable bundle identities above; the bundle itself is
   never edited or relabeled.
2. The DEL-09-04 run record/History may record the governance disposition only
   to the extent the owner authorizes. The current first Remaining bullet does
   not contain the R14 acceptance gate, so R14 acceptance alone authorizes no
   strike of that bullet; its maintainer-review, GUI, and export residuals stay
   governed separately.
3. TASK_MANAGEMENT may dispose `TM-PIP-037` only after fan-in with the separate
   DEC-046 value-promotion ruling, because the registered trigger requires both
   dispositions. This packet supplies only the R14 half.
4. Any present-base rerun is a separate future governed tranche with a new run
   ID and immutable bundle. It is not implicitly authorized by acceptance,
   deferral, or declination here.

## 9. Preserved scope and claim boundary

No code, test, fixture, witness, case, manual, register, lifecycle, release,
reliance, GUI, export/CAEPIPE, tolerance, or evidence-tier surface was modified
or promoted by A2-R14. No procedure was rerun. The D-61 hold and every stated
owner gate remain unchanged pending explicit ruling and the parent manager's
governed fan-in.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
