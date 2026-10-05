# I63 return: Rust reader aligned to snapshot 06d (last alignment before review)

I63 is a TASK (Type 2). ROOT (HELP_HUMAN) granted this work directly in the session. The basis is the rulings "Accounting triggers: only allocator refusals are emittable; F and P rebased as snapshot 06d" and "Snapshot 06d verified; the last reader alignment before review" (NUM `d16fd47d21`). It ran under the same fence, command, target and rules as `BRIEFS/I63_I64_COVERAGE_READERS.md`. I63 had no descendants.

- **Run:** first tool call 2026-10-03T22:37:38Z; freeze about 22:44Z, inside the 45-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, new tooling, solver, native, UI or DEC-025 job; one Cargo job at a time, each under a 1,200 s wall.
- **Basis files:** READER at `f1ff7ebddc`. NUM was at `37b7264e42` when hashed.
- **Paths** use the brief's placeholders.
- **Status:** all 06d checks pass. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`).

## Inputs verified at start

- **Shared files:** all five match SHARED_SNAPSHOT_06D (`becb83842f`). SHA256SUMS_C2_4 and SHA256SUMS_C2_5 verify.

  | File | sha256 prefix |
  |---|---|
  | corpus | `d02701ed6a` |
  | schema | `f943ebd351` |
  | definition | `3e0779a45a` |
  | preview table | `c74742ce6a` |
  | results yaml | `4585a45fcf` |

- **Counts:** 15 cases (F′ and P′ rebased), 178 mutations and 18 must-pass entries.
- **Python reference (read only):** `55736ea65a`.

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before (`3154d5eb5d`) | After |
|---|---|---|
| src/retained_precision.rs | e131a6b9a20f… | bd20dd9a8f888f0a3e81c981db7c02c7cfd48901ddc1ef203914d00be8b19ed0 (156406 B) |
| tests/retained_precision_contract.rs | 10818998018c… | 5cbb6ba4ea47add234b5bf827bfaf1390c4e953a38d73c8d61136a07c14eaf20 (65226 B) |
| src/lib.rs | 375b073135… | unchanged |

### Source changes

1. **R1–R3** (`accounting_rules`, for every product attempt) at G5 WORK_MISMATCH. They are pushed to the deferred work list after the attempt's association and stage checks. They therefore fail after every attempt's association checks and after the P9 typed pass, as in Python's `_accounting_rules`.
   - **R1:** `adapter.fault` must be null, and no object `{kind:"accounting", event}` may appear anywhere in the attempt.
   - **R2:** no object has `lost: true`.
   - **R3:** every `{kind:"work_accounting", fault}` fault set must be contained in the join of the attempt's `{kind:"unavailable", fault}` Count faults and `sticky_status` values. "both" counts as overflow plus inconsistent, and "exact" as empty.
2. **The schedule replay now mirrors Python's `_g5_schedule` record replay exactly.** This closes a gap: Rust previously accepted a rejected attempt whose verification record said `verified`.
   - candidate-record contiguity: a fresh candidate opens the next record;
   - a reused candidate is exactly the prior rejected attempt's completed verification record, with role VtC;
   - verification record = candidate + 1;
   - a failed verification needs a `failed` verification record with the same reason, and a candidate rejected with `verification_failed`;
   - a completed verification record is `verified` when the candidate is accepted and `solved` otherwise, or VtC with a rejected candidate;
   - a candidate without verification is `failed` with a stop;
   - accepted only last, with a completed verification;
   - all records are consumed.
3. **Build state/reason consistency moves to Python's place and code:** per referenced build in the record loop, as G5 WORK. It was a global ATTEMPT pass. Every build is referenced by its building record (the `builds_seen` check), so the accepted set is unchanged; only the code and order match Python now.
4. **Unchanged from 06c:** G7 per language, P5, P7, O2 strict, N5/N8/N9/N10/N17, WorkAccounting rejection, and the harness (`invocation_edits`, `expected_by_reader`).

### Test changes

- the slice tables now assert 178 mutations;
- a new `snapshot_06d_mutation_outcomes` covers 173..178 (G5 WORK 4, G5 ATTEMPT 1);
- must-pass asserts 18 entries.

## Commands and results

Every run used the brief's command and environment variables plus the disclosed `DEVELOPER_DIR`.

| Run | State | Result |
|---|---|---|
| run1 | baseline on 06d (the 06c reader) | 14 passed, 8 failed. The only corpus failures were the four R1–R3 mutations, which were admitted; the idle sibling already gave ATTEMPT at the exhaustion rule. The other failures were count assertions |
| run2 | R1–R3 | 23 passed |
| run3 (**final, full command**) | plus the replay and build-state mirroring | **23 passed, 0 failed** |
| run4 | final bytes, `--nocapture snapshot_0 shared_must_pass` | 8 passed; tables captured |

**Against the bar:**
- **Mutations:** all 178 match this reader's expected first gate and code, using `expected_by_reader.rust` for G7.
- **must_pass:** all 18 validate with the base case's classifications, including the six replaced storage-cause entries and `cert_failed_after_summary_storage`.
- **Cases:** all 15 validate, including F′ and P′.
- **Earlier tests:** all pass.
- **Tables:** `OUTCOMES_06D.json`.
- **No expected outcome looks wrong.**

## Checklist status (Rust, 06d)

Rows not listed are unchanged from `reader_align_06c/RETURN.md`.

| ID | Rust status | Check or evidence |
|---|---|---|
| N1, N3, N4, N6, N7, N13 | checked; the replay now mirrors Python's record/role/outcome replay | 06a/05b bases and mutations; reader-logic tests |
| N10 | checked; plus a discriminating shared control | `idle_budget_not_exhausted_no_group` |
| C1 | checked; build state/reason now WORK per reference, as Python | `failed_build_reason_mismatch`, `cached_failed_slot_rebuilt`, `failed_slot_not_cached` |
| P6 | checked | `maxima_abandoned_separate_failure`; must-pass `values_failed_separate_completion`, `maxima_abandoned`, `aliases_abandoned`, `bind_rows_abandoned` (allocator refusals) |
| P7 | checked (attached entries, every attempt) | `prefix_attached_old_input_unbound`; must-pass `prefix_unattached_old_operand_attested` (on P′) |
| P8 | checked | reason/phase table on F′/P′; must-pass `prefix_captured` |
| P9 | checked; post-certificate observables/G5a must-pass entries retired | `certificate_check_wrong_wrapper`; F′ |
| R1 | **checked (new)** | `adapter_fault_present`, `accounting_cause_without_fault` |
| R2 | **checked (new)** | `scalar_trace_lost_unavailable`; `product_work_only` (Ready) |
| R3 | **checked (new)** | `work_accounting_cause_exact_status` |
| Certificate failure prefix | checked | must-pass `cert_failed_before_summary` (coverage null), `cert_failed_after_summary_storage` (coverage retained) |

## Known differences from Python (`55736ea65a`)

**What was compared.** I compared the G3 and G5 code paths of both readers in a targeted way, plus the parts of G5a, G7 and G8 that I aligned in the earlier grants. This is **not** an exhaustive line-by-line parity proof. On every one of the 178 shared mutations, 18 must-pass entries and 15 cases, the two readers agree.

**The differences found.** Each would show only on an input that no shared entry exercises.

1. **Run-id contiguity and `work.execution_order`.** Rust checks them at **G3 COVERAGE_MISMATCH** (inherited code); Python checks them at **G5 ATTEMPT_MISMATCH** (`_g5_native`).
2. **Operational member indices at G3.**
   - Python requires old/prepared/new members to be exactly `0..len` in order.
   - Rust requires unique old members, with prepared and new as prefixes by member id.
   - A non-contiguous but unique old order passes Rust G3 and fails later; Python fails it at G3.
3. **Old coverage against the source.** For an attempt with a source, Rust requires old members to equal the source's `id_maps` kernel members, at **G3**. Python requires `len(old) == len(id_maps.members)` only when old coverage is complete, at **G5 PRODUCT_ATTEMPT**.
4. **Source preparation back-reference.** Rust requires `source.preparation.attempt_ref == attempt` for every attempt with a source, at G5 PRODUCT_ATTEMPT. Python checks it only on Ready attempts.
5. **The material basis against the ordinary attempt.** Rust also requires `attempt.material_basis_ref == ordinary_attempts[ordinary_attempt_ref].material_basis_ref` (G5 PRODUCT_ATTEMPT). Python doesn't.
6. **P8 reason table edges.**
   - With error kind `native` and a *selected* Run, Python fails (kernel branch, requires unresolved or refused); Rust takes the facade branch.
   - Rust also requires `error.run_ref == run.id` for native errors.
   - Rust requires `run == null && preparation failed` for preparation errors.
   - Python has neither of those two requirements.
7. **Order inside the native G5 class.** Rust checks group basics (ids, non-empty, `first_source_ref`, source stiffness, call membership, unique sources) and build id/work *before* the call loop; Python checks them *after*. Rust also checks the derived cache inside each Run, in a different place relative to the replay. A receipt with two independent native defects of **different codes** (ATTEMPT versus WORK) may therefore report different codes. Single native defects agree.
8. **Rust-only native group checks:** a group's `call` must index an existing call, and its sources must be unique and listed in that call. Python covers these only through its C5 partition.

**These are inherited structural choices,** mostly from I59's G3 and the C3 association pass, and are outside this grant's R1–R3 scope. I did not change them, because each needs a contract reading on which reader is right. The independent review should rule on them, and a shared mutation can then pin each one.

## Host

The Xcode licence is still unaccepted, so every cargo run set `DEVELOPER_DIR=/Library/Developer/CommandLineTools` for the process only. No system setting changed.

## Open

- The differences above, for the review.
- **Deferred, as for Python:** a faithful `prefix_helper_refused`; storage contexts for the seven pin mutations; and the deferred shared bases (Ceiling, L = 0, the source-construction base, exhausted budgets).
- **Not claimed:** acceptance, eligibility, three-reader parity or independent review. I63 ran no Python or TypeScript.

## Files read (sha256)

These are in addition to the earlier I63 RETURNs.

| sha256 | File |
|---|---|
| d562af562ddf057b55a3e790ded7056172ff607ae0dfb87be3a0020da14fb628 | R/I62/coverage_shared_python_01/ACCOUNTING_CAUSES.md (§1 and the class verdicts) |
| 1bfa35c514c302bf16b5cd00f8e34627609c01b4ab5a83dcfa8ea83affbffbea | R/I62/coverage_shared_python_01/RETURN_C2_5.md |
| becb83842f61765f5478efb9deaabaabfaf2726af416e2314f0cb2b9b0d0cbc6 | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_06D.json (files, counts) |
| 8b038e2f3fcebf5b59786bc53d8abdb1fb1352726432acb2306bc56b811d5706 | R/I62/coverage_shared_python_01/SHA256SUMS_C2_5 |
| 1255ce9a4473f82050d4b8f8d2efa622e8215c5a3a29ff288025762f322c452a | T3/ROOT_RULINGS_V1.md (the two 06d sections) |
| 55736ea65aee641fb22288d869632c5ea30f8c016ce56307a974ea598826192f | READER/P/core/analysis_runs/retained_precision.py (`_accounting_rules`, `_objects`, `STATUS_FAULTS`, `_g5_schedule`, `_g5_cache`, `_g5_native`, `_g5_products` and G3; read only) |

## Bulk (WT/scratch/i63_reader_align_06d/)

| sha256 | bytes | file |
|---|---|---|
| 1b10476f4ee3ec6d1a93fecdec1ed35a917dfc35f00cacc90444f26a9ba5e104 | 8088 | I63_06D_DELTA_src.diff |
| adffc6c0fa7c0fc67171f39284cba61e9cff025365bece053e9b35aae99526e9 | 2053 | I63_06D_DELTA_test.diff |
| 63b2949e1e2bd96f27f29551183859e3f5109205104945ed524d981a97377d69 | 4834 | run1_baseline.log |
| c0f3be682147b888b84852e23a4bb4d4f699d4bb7cec6a07763f968a9007c9a6 | 2130 | run2.log |
| 78090027a99e0d6522c8b9705fe8847d4d21ffd15027feed916813a1f6ab855a | 2130 | run3.log (final) |
| 1e694f3512a082080da120db857341215eb35a1c7994238e4a46739c48d7ec3c | 38917 | run4_outcomes.log |
| e131a6b9a20f26dec770977ae2ee891a55ab6d8b42eabe631582b6797e8bd381 | 151402 | before/retained_precision.rs |
| 10818998018c8b5aa7b7700e9d656904f8b5aba3499e07760bf00c24cb3d425f | 64759 | before/retained_precision_contract.rs |
| 375b07313518a0cdd09e317b83b9ca7536acef5c80fd4cd0a94920fd8bf2486d | 66051 | before/lib.rs |
| bd20dd9a8f888f0a3e81c981db7c02c7cfd48901ddc1ef203914d00be8b19ed0 | 156406 | final/retained_precision.rs |
| 5cbb6ba4ea47add234b5bf827bfaf1390c4e953a38d73c8d61136a07c14eaf20 | 65226 | final/retained_precision_contract.rs |
