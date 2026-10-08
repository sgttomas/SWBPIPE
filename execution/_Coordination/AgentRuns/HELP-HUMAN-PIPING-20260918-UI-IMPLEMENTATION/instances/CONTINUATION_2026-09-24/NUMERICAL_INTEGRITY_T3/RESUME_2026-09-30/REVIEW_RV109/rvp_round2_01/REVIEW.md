# RV109 (RV-P, B1 round 2): independent review of B1's SP slice (the n-case transaction)

TASK (Type 2), RV109, role RV-P for all of B1, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. I wrote none of this change. 2026-10-07 UTC.

**Brief (verified before work):** `R/BRIEFS/RV109_RVP_ROUND2.md`, sha256 `06cfb671e544a07228394701d1640764cbea89eaac2b7b087b965bb1be6bce04`. My earlier records continue: round 1 with ADDENDUM_01, and the early read at R3′ (`R/REVIEW_RV109/rvp_r3p_read_01/NOTES.md`, `7d312cf0…`).

**Basis read** (sha256 verified):
- DESIGN_v2 §1.1–§1.4, §2, §3.3 and decisions 5, 6 and 21 (`R/I78/b0_contract_01/DESIGN_v2.md`, `5933b90b…`);
- PLAN_v2 §1, §2.2 and §5 (`R/I84/b1_plan_01/PLAN_v2.md`, `c85786b7…`);
- PROBE §2–§4 (`R/I81/b1_probe_01/PROBE.md`, `3e32726d…`);
- RR (read at `1569976c…`): R3′; the early-read dispositions; I89/SA's ruling 2 (the seam saturates); RV112's N-3 and N-4; I2; "SP returned before I3; the headline rule accepted; RV-P round 2 dispatched";
- the readers' headline code: RE `src/preview_physics_evidence.rs` (`headline`, `governs`), RE `src/retained_precision.rs` (`validate`, `project`), PY `core/analysis_runs/preview_physics_evidence.py` (§6), TS `previewPhysicsEvidence.ts` (§6; read, not run);
- **after forming my own view:** I85's `R/I85/b1_sp_01/RETURN.md` (`f5c4797a…`), its `wc2_pins.json`, and from `post_i3.py` only the W-C2 document id (`w_c2_<mode>`), to reproduce the document hashes.

**Placeholders:** `WT`, `NUM`, `T`, `R`, `RR`, `P`, `PP` (= `P/core/product_physics/src`), `PP-tests`, `RE` as in the dispatch. `S` = my scratch `WT/scratch/rv109_rvp_01/`. `E` = this folder's `evidence/`. My archive copies (all deleted): `I1` = `262bd687f0`; `HEAD` = `603e238517`; `MUT` = `603e238517` for mutants; **`I3M` = the scratch merge:** `603e238517` with SR-RS's three RE files from `b5cb7faaeb` (`retained_precision.rs`, `source_blocks.rs`, `tests/retained_precision_contract.rs`; `E/i3/overlay.sha256`). SP's head leaves RE untouched since I1, SR-RS's base, so the overlay is the whole merge.

**Limits kept.**
- **Cargo:** every job through `WT/tools/t3_cargo.sh` under `WT/guard/cargo_job.lock`, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `TMPDIR=S/tmp`, fresh targets `WT/targets/rv109-r2-*`. `RUSTFLAGS` unset except `--cfg=rv109_stale` for Stale. **79 jobs** (`E/cargo_jobs_rv109_r2.log`). No test binary ran outside cargo. I shared the lock with I88, I94, I95 and RV113, and killed no job.
- **Waits:** one bounded wait at a time, each ending on its result file or when the job's process was gone. No wait of mine is running.
- **No Git writes** (reads with `GIT_OPTIONAL_LOCKS=0`); no DEC-025; no installs; nothing in the system temp directory.
- **Probe and mutant code went only into my copies.** The records use placeholder paths only; no folder is named `build`.

## 0. Verdict

**PASS** at `603e238517`.

| BLOCKING | SHOULD-FIX | NOTE |
|---|---|---|
| 0 | 2 | 4 |

**The n-case transaction is right.**
- T-1 to T-13 match DESIGN_v2's text and outcome table, and C1–C3 where they apply.
- **c = 1 is byte-identical to I1:** 106 rows of my probe, and adapter counts at every W1 stage boundary on 100 rows.
- **W-C2's receipt, re-derived independently,** holds in both modes and in six reorderings (up to 31 checks per row, none failing).
- **The headline rule is the one the base readers' G7 requires.** RS and PY refuse every alternative I built, and W-C2's stress headline moves from case A's row to case C's.
- **I85's I3 front-run reproduces exactly** on my scratch merge: all six pins, and the one expected change.

**Both SHOULD-FIX findings are test- and records-only,** and fit I3's W-C2 pinning step:
- **SF-1:** N-16 has a record-level difference that I85 reported as none.
- **SF-2:** no committed test has a selected case that is not first, or two selected cases. Six of my mutants survive there.

## 1. Findings

| ID | Severity | Path | Evidence | Remedy |
|---|---|---|---|---|
| **SF-1** | SHOULD-FIX | I85 RETURN §3.3 ("N-16: no difference … no finding"); PP `retained_facade_tests.rs` `b1_sp_w_c2_transaction_outcomes_and_ordinal_mapping` (its N-16 block, `outcome_facts`) | **The batch's second Run is not identical to that case's one-case Run.** It differs in exactly two `AttemptRecord` fields:<br>• `shared_built_here` (its p128 and p256 records);<br>• `verification_shared_built_here` (p256).<br>Both are `false` in the batch and `true` alone. This holds in every two-Run batch I ran at the head (12 rows: 6 orders × 2 modes), whichever case runs second: C in (A, B, C), A in (C, B, A) (`E/wc2/n16_record_diff.txt`).<br>**The cause** is C2 §4's group-build sharing. D1.5 gives one stiffness, so one group: the first Run builds s128, s256 and v256, and the second reuses them. W-C2's receipt shows it: builds 0–2 originate in run 0 (A), and C's p128/p256 records reference builds 0 and 1 (§2 item 2).<br>**Everything else is identical:** terminals and reasons, precisions, roles, outcomes, residual bases, corrections, margins, rcond, and every work and stage counter (own, shared, verification, K4, storage, gate). C's one-case ladder is PROBE §4's: 128 Candidate StopRule(Uy) → 256 VtC StopRule(EndAction Ux) → 512 VtC Charge → 1024 Verification Solved.<br>**Why the test misses it:** it compares only (terminal, record count, selected precision), so it cannot see a record-level difference. PLAN_v2 N-16 and R8 say a difference inside the one `CaseBatchCall` "is recorded with its cause and returned to ROOT as a finding. It is not absorbed into the pin." I3's W-C2 pins would absorb these two flags | **Records and test, before I3's pins:**<br>(a) I85 records the difference and its cause in the I3 append, for ROOT's ruling;<br>(b) the N-16 assertion compares every record field except the two build-provenance flags, and asserts those directly: the first Run builds, a later Run reuses.<br>**ROOT rules** whether these flags are within "outcomes". I recommend yes: they are C2 §4's expected sharing, and the receipt records them |
| **SF-2** | SHOULD-FIX | PP `retained_facade_tests.rs`: the W-C2 tests, and I3's successor pins (I85 RETURN §7) | **No committed test has a selected case that is not first, or two selected cases.**<br>• In every test the one selected case is first in A and first in the batch.<br>• Its request index equals its ordinal and its attempt id. The exception is (B, A), which before I3 asserts no Run owner.<br>**Six of my mutants on the selected-case serializer paths and the headline rule survive** the whole PP `--lib` suite (`E/mutants/summary_a.md`):<br>• **M15:** the selected diagnostics in reverse request order;<br>• **M16:** a selected case's `preparation.attempt_ref` is 0;<br>• **M17:** its product-attempt id is 0;<br>• **M19:** its Run owner is named by batch ordinal;<br>• **M28:** a headline tie ignores the case id;<br>• **M32:** the displacement headline is not restaged.<br>**On my I3 scratch merge** (`E/mutants/summary_i3.txt`):<br>• the accepted reader refuses five of them: M16 at G1, M17 at G3, M19 at G5, M28 and M32 at G7. That is a fail-safe abandonment of a publishable successor, not a wrong publication;<br>• **it accepts M15:** reader-legal, but contrary to T-11's "for each selected case, in request order".<br>These are variants of PLAN_v2 §2.2's own acceptance list: the staging order, `attempt_ref`, and the ordinal mapping | **Test-only, at I3 with the W-C2 pins.** Also pin the successors of **(C, B, A)** and **(A, A2)**, where A2 is a copy of A with its load ids suffixed:<br>• both pass the accepted reader on my merge, in both modes (`E/i3/wc2_i3_summary.txt`);<br>• together they kill all six, as shown on the merge.<br>Before I3, the same facts can be asserted on the captured successor instead |
| N-1 | NOTE | PP `retained_wire.rs` `case_source`; `retained_product.rs` `native_call` | **Mutant M11 survives the `--lib` suite.** It moves the call's sources back to the wrong slots after the call.<br>• **Its effect:** an unavailable case's `CaseSource` is built from another case's source, while its `kernel_source_sha256` is its own Run's origin.<br>• **Nothing in the producer checks this.** `case_source`'s doc states the precondition ("must equal the registered identity bytes' owner"). At c = 1 the source is borrowed in place; at c ≥ 2 it moves out for the call and back.<br>• **The head moves them back correctly:** my W-C2 rows, and G8 passing on the merge.<br>• **Before I3,** precommit's G5 refusal of B masks the mutant.<br>• **On my merge,** the reader refuses it at G8 `PREPARATION_MISMATCH` wherever the swap moves an unavailable case's source: (A, B, C), (C, B, A) and (B, C, A), both modes. I3's W-C2 pin will also catch it | **Optional:** a producer-side check that binds a non-selected case's slot source to its Run's origin identity. A wrong source would then be a typed `Association` refusal, not a G8 abandonment |
| N-2 | NOTE | My mutants | **Four survivors need no pin:**<br>• **M46** (the domain re-check admits c = C + 1) is equivalent: `CaseSet::push` refuses past C. This is I85's S8a, and my round-1 S16;<br>• **M26** (a diagnostic naming several cases is referenced by the first only) is equivalent on every input I found. No ordinary diagnostic names two case ids (W-C2's do not);<br>• **M31** (the ruled rule's "only where the ordinary has a headline") is unreachable on admitted inputs. A missing stress headline needs arc members, and D1 refuses components;<br>• **M43** (`facade_certificate` coded as the kernel's) has no known input with a T-9 refusal beside a selected case. The readers' D4d table would refuse a wrong code at precommit | Record only |
| N-3 | NOTE | The registered Direct entry at the head (I2 + SP), before I3 | **Since I2, the registered Direct entry publishes c ≥ 2 successors** whenever no case is `not_required`; the pre-I3 reader accepts them:<br>• I86's cap-maximal three-case input (`i3_c1_three_case.json`; Σ loads = 384 = L, admitted) publishes a 6.65 MB successor in both modes. Receipts: `45d0cdda…` sparse, `0134f210…` dense;<br>• W-C2's (A, C) publishes in sparse.<br>They are unpinned, and not SP's to pin (`E/probe/probe_compare.txt`) | For SQ's S1 and G6 runs at c = 3 |
| N-4 | NOTE | PLAN_v2 §1 (records) | **PLAN_v2 §1 says s11f "scans PP/lib.rs and PP/retained_product.rs with rules 1–6 and rule 8".** Rule 8's `RULE8_FILES` has `PP/lib.rs` but not `retained_product.rs`.<br>• SP adds no rule-8 shape to `lib.rs`.<br>• In `retained_product.rs` it adds one `.fold(` (`selected_attempts`, a bitwise OR, not an accumulation), which rule 8 would count if it scanned that file | **Records:** correct the plan's sentence, or add the file to `RULE8_FILES` with its table rows (ROOT's choice) |

## 2. The review, item by item

### Item 1: T-1 to T-13 against DESIGN_v2, its outcome table, and C1–C3

I read every SP line of `lib.rs`, `retained_product.rs`, `retained_receipt.rs` and `retained_wire.rs`. (I read `retained_product.rs` with `-w` for the moved freeze body.)

- **T-1 and T-3:** unchanged. `retained_w1`'s order is coexistence, G-B, the domain re-check (`w1_case_ids`), T-4, then T-5.
- **T-2:** one `ProductCapture`.
  - `per_case_capture!` lists 29 per-case fields, swapped by `with_case`. They include the observations, the late capture, `error`, `observable_error`, `verdicts`, `numeric_*`, `g5a_*`, `work`, `native` and `scope_rows`.
  - The adapter, the seeds and `native_invocation` are the invocation's.
- **T-5:** `ReservedNotices::reserve` is called with A's ids. It refuses a base collision or a collision between notices, then reserves |A| slots and one maximal message per case. At c = 1 the order of operations is the old one.
- **T-6, custody once** (`prepared_custody`):
  - prior errors, parked first (R3P-3);
  - the preconditions, now including `native_invocation` and a parked `native`;
  - `cases_seen == requested`, and A in range and strictly increasing (R3P-2);
  - the one-case binder at c = 1; `bind_observations_by_case` and `bind_case_rows` at c ≥ 2.
  - A failure gives (`Preparation`, 0) and then the notices, as the outcome table says.
- **T-7:** `prepare_attempt`, per case in A in request order, with ids in start order.
  - A failure keeps its error in its slot and takes its snapshot at preparation.
  - It has no source, so it is not submitted. The serializer gives it `source_unavailable`/`preparation`, `run` and `source_ref` null, and no `CaseSource`.
  - `bind_preparation` names the attempt's own index.
- **T-8:** one `native_call`.
  - `for_calls(&[n], &[])`, the 60e9 invocation limit, and the 20e9 case limit.
  - The batch is the attempts ending `Prepared`, in request order (R3P-4).
  - The kernel names each case by its batch ordinal (`Case(k)`); the serializer maps ordinals to request indices.
  - A call failure before any Run fails every submitted case, so T-10 gives `Native`.
- **T-9:** `freeze`, per `Selected` attempt in request order, through `with_case`.
  - Each runs in its case's own `CaseScope`: its row block, its `preview_cases[]` index, and qualified ids after the first case.
  - All of them run against the one untouched ordinary owner. My check "ordinary untouched" holds on every row.
  - A refusal gives `Candidate` and then `facade_certificate`.
- **T-10:** with nothing frozen, the cause is the furthest stage reached. The selected bits are computed before staging.
- **T-11, staging:** one clone, then each frozen overlay in request order, then `stage_headlines` at c ≥ 2 (item 4).
- **T-11, the serializer:**
  - the identity once;
  - then each selected case's token, its omitted legacy disclosure and its selected diagnostic, in request order;
  - then the unavailable cases' diagnostics.
  - `cases[]`: `not_required` is checked against `checks_passed`.
  - one material basis; one call, with its groups and builds as recorded;
  - `execution_order` from `inv.runs()`;
  - `charged` checked against the last Run's and every call's `invocation_after` (C1 §4);
  - D6a in one pass.
  - Any serializer refusal abandons; R-b′ is kept; precommit runs with the actual invocation.
- **T-12:** |A| notices, in request order, from the reserved space. C1:68's detail goes only on bit-k notices, where k is the k-th case in A.
- **T-13:** `ONE_RUN_THROUGH_G_C` is pinned on Direct. My document probe on I3M also shows `runs 1, complete_gates 1`.

### Item 2: W-C2's receipt, re-derived independently (`E/wc2/`, `E/probe/zz_rv109_rev.head.rs`)

**My own construction**, from DESIGN_v2 §1.4's words: input sha256 `ea5c13b1…`, which is I85's. The transaction ran stage by stage on the head's API.

**Expected values come from the attempts, not the serializer.** Up to 31 checks per row (31 on each three-case row), in both modes and in the orders (A,B,C), (C,B,A), (B,A,C), (B,C,A), (A,C), (C,A) and (A,B). **Every check passes** (`E/wc2/wc2_head_summary.txt`).

| Fact (sparse; dense the same except the noted counts) | Value |
|---|---|
| Verdicts and A | A Sensitive, B ChecksPassed, C Sensitive; A = {0, 2} |
| Runs | run 0 = request 0 (A), kernel owner `Case(0)`, selected p128, 2 records; run 1 = request 2 (C), owner `Case(1)`, unresolved Ceiling, 4 records |
| `cases[]` | A `selected`, source 0, attempt 0; B `not_required` (4 members, attempt ref null); C `unavailable`: `kernel_unresolved`, phase `kernel`, cause attempt 1, terminal `{unresolved, {space: unresolved, tag: ceiling}}`, source 1 |
| Ordinals → requests | `calls[0].owner_refs` = [case 0, case 2]; `execution_order` = [case 0, case 2]; each `cases[i].run.origin.owner_ref` = i; `sources[j]` = (j, owner request, its id, the attempt's `attempt_ref`). Also checked for A = {1, 2}, {0, 1} and {0} at the head, and {0, 1, 2} on I3M |
| Work | A: before 0, increment 2,899,623; C: before 2,899,623, increment 10,530,055, after 13,429,678 = **`charged`** = `calls[0].invocation_after` |
| Groups and builds | one group (stiffness `8d929aaa…`), `source_refs` [0, 1]. 7 builds: s128, s256 and v256 from run 0; s512, v512, s1024 and v1024 from run 1. **C's p128 and p256 records reference builds 0 and 1:** the sharing |
| Snapshots (T-11 record point) | A's = the counts after T-9 `[792, 6030, 1567, 10418, 216920, 9499, 178, 476, 3424, 63866]`. C's = the counts right after the call `[790, 1539, 671, 3341, 21508, 2905, 163, 127, 3345, 20995]`, taken before A's freeze. Dense differs only in elements 4, 8 and 9 (0-based) of each. Each `product_attempts[k].adapter.counts` equals the trace's snapshot |
| Staging | the tail is A's selected diagnostic, then C's unavailable one. The token is on A's rows only. Every unselected row is byte-identical to the ordinary row, and selected rows differ only in value and token. One material basis, `case_indices` [0, 1, 2] |
| Precommit before I3 | G5 `RETAINED_PRECISION_ATTEMPT_MISMATCH` (B's `not_required`), both modes; selected bits 0b01 |

**N-16: the batch Runs against the one-case runs** (`E/wc2/n16_record_diff.txt`). Each case alone, through the head's c = 1 transaction:
- **A:** selected, p128, 2 records;
- **B:** ChecksPassed, so `NoTriggeredCase` (its standalone Run is Ceiling);
- **C:** Ceiling, with PROBE §4's ladder exactly, in both modes.

The batch's first Run equals its one-case Run in every record field. The second differs only in the two build-provenance flags (**SF-1**).

**I3's front-run, reproduced on I3M** (`E/i3/`):
- **PP registered `--lib` on I3M:** 567 ok, 6 failed, 11 ignored.
  - The failures are t13 and five SP tests that pin "before SR-RS".
  - Four now see the successor. **The T-7-on-C test now ends at `Precommit { G8, PREPARATION_MISMATCH }`:** the one expected change, as I85 states.
  - Every c = 1 pin passes.
- **RE on I3M:** carriers 17/17, and SR-RS's contract test 70/70.
- **All six of I85's pins, reproduced exactly** through the actual registered Direct entry (`runs 1, complete_gates 1`) and the private driver (`E/i3/i3_docs.log`):

| Mode | Document sha256 | Receipt sha256 | Published bytes sha256 |
|---|---|---|---|
| sparse | `7922e3e5278d0d87dc5faf79dfbc1f2a384899e97df306cc742355cdacdb6269` | `cccb9664e1c58f0582348df3348d8b6e0b0941bcb0294a5b351a4d092ed18886` | `c7a1859330e9e36e18f5572838dbad72ea251a817241acc6968f8f83b70170fa` |
| dense | `f2800bd4f2b4c90217918a6e1287f98305a1b6b07893295b5790f387c075d3a3` | `612e23ca4b90604b3d2351fd7465d2e3cefb0f3fbb36bdea39efaa82a68bc07a` | `a77c010b4ae7ffa9c535c31305b8a91fcc3c05e8a512075fa5b4694dbae3062c` |

- **On I3M, the accepted reader passes W-C2 and all ten of my other orders, in both modes.** These include two selected cases: (A, A2), (A, C, A2), (A2, B, A) and (C, A, A2). Every reviewer check passes (`E/i3/wc2_i3_summary.txt`).

### Item 3: c = 1 byte identity (`E/probe/`)

**My round-1 probe on I1 and the head:** 61 inputs × 2 modes. It records:
- the plain bytes, and Direct's published bytes, cause and notices;
- the private driver's envelope and successor;
- the native-fault sentinel, and the seam.

**All 106 c = 1 rows are identical** in every recorded field. The 16 differing rows are all c ≥ 2 (`E/probe/probe_compare.txt`):
- `i3_c1_three_case` publishes (N-3);
- the four multicase source-block fixtures are now admitted and end at `Coexistence`, with exact bytes;
- the two load-reference fixtures move from `Domain` to `NoTriggeredCase` on the driver;
- `invented_dec092`'s refusal family is now `Combinations` (SA's).

**Adapter counts at every W1 stage boundary**, 100 c = 1 rows, through each revision's production transaction (`E/probe/stage_compare.txt`):
- the stages are I1's `prepare_case`, `solve_native`, `freeze_candidate`, `staged_envelope` and `serialize_frozen`, against the head's `prepare_cases(…, 1, &[0])`, `native`, `freeze`, `staged_envelope` and `serialize_cases`;
- the counts after the run, preparation, native, the candidate, staging and serialization are **identical on all 100 rows**, as are the outcomes, the terminal snapshot, the staged summary, the successor sha256 and the reader's verdict;
- the one exception is the 56 custody-failure rows, where the head has no attempt and so no snapshot. My shim records none, and neither path publishes a receipt.

**The committed c = 1 pins, both modes, read on Direct at the head:**
- milestone receipts `efc1a39b…` and `3e26499f…`;
- L = 0 receipts `c00cbe76…` and `0b4250c8…`, published bytes `9b425066…` and `5d84fce6…`.

These equal `PINNED` and `U8_L0_PINNED`, and the pin tests pass.

### Item 4: the headline rule (RETURN §8, as ruled) (`E/headline/`, `E/wc2/`)

**What the readers require:**
- RS: `headline` and `governs` (G7, through `for_source` on the projection);
- PY: `governing = min(key = (−value, case id, entity_ref))`;
- TS: a reduce with code-point comparison.

All three require **the governing row over all load-case rows of the kind: greatest value, then smaller case id, then smaller location.** `stage_headlines` implements exactly that, with `&str` byte order (= code points), and only where the ordinary envelope has the headline.

**Shown on every W-C2 row:**
- My own projection of each staged successor passes RS's G7 (`for_source`) and PY's base reader.
- **Each alternative fails wherever it differs from the rule:**
  - keeping the ordinary headline, the first frozen case's alias, and the selected rows only: RS `HEADLINE_BINDING`, PY "does not govern";
  - null (PY only): PY refuses it. RS's equivalent code is `HEADLINE_PRESENCE`.

**W-C2's stress headline moves from A's row to C's,** in both modes:
- `result:elastic-maximum:6:case-a:2:M1` → `…case-c:2:M1`, at the same value: 2.5172202259989904e-08 Pa sparse, 4.619475839070268e-08 dense.
- A's and C's M1 rows tie in the ordinary run, and A wins on case id. A's overlay value is 8.170865639376687e-92, so C's unchanged row then governs.
- The displacement headline stays case B's row (282.942… mm).

**The tie-break by case id is exercised in my (A, A2) input:**
- A's and A2's M1 rows tie at 8.17e-92, and `case-a` governs;
- the readers accept it on I3M.

**The committed test exercises ties only where id order equals request order** (B and C at node-section-b). See SF-2 (M28).

### Item 5: the early read's notes, and RV112 N-4

| Note | Closed by (code; killed mutant or test) |
|---|---|
| R3P-1 | `into_single` and the one-case facade path are gone. **Killed:** S18 and M08; test `b1_sp_r3p_1_…`. My probe runs (A, B), (A, C), (C, A), (B, A, C) and (B, C, A) on the attempted case's own slot and source |
| R3P-2 | A is validated in custody before any attempt. **Killed:** M36; test `b1_sp_r3p_2_3_…` |
| R3P-3 | Prior errors are taken parked-first. **Killed:** M37 |
| R3P-4 | The batch is the attempts ending `Prepared`. **Killed:** B_R3P4 (every slot submitted) |
| R3P-5 | The detail is placed by bit. **Killed:** M02, M03, M04, M05 |
| R3P-6 | `grant2.rs`'s docs now name the case |
| R3P-7 | Per-case presence, the parked late capture, and parked native. **Killed:** B_S3, B_S4, S7 |
| R3P-8 | **S13, S14 and S20 are now killed** (by `b1_sp_t5_…`, the T-12 test, and the W-C2 outcomes test) |
| R3P-9 | One custody prelude: `prepare_owned_case` → `prepare_cases(…, 1, &[0])`. One removed line: the prior-cause path's `trace.costs.record::<Option<CaptureError>>()`. **It is unobservable:** trace costs are not receipt bytes, and only `I51_PRIOR_CAUSE`'s print shows them (I85 §9.6 discloses it) |
| RV112 N-4 | `prepared_case_source` returns at once when `late_refusal` is set. Test `b1_sp_first_g_b_refusal_stands_…`: total 3, only A's late hook ran, Direct gives `LateGate` and exact bytes. **At c = 1 this is unchanged:** one late hook, so no prior refusal |

### Item 6: multi-case coexistence and the seam's saturation

- **n05 with a second case** is pinned: `Coexistence`, exact bytes, no notice, the diagnostics' capacity unchanged, and `{runs: 1, complete_gates: 0}` in both modes.
- **My probe adds four committed multicase source-block fixtures:** Direct, admitted, `Coexistence`, exact bytes, both modes.
- **The seam:** `saturating_add` before G-B, and no adapter event. Test `b1_sp_seam_overflow_…` checks (`CaseLoadsTotal`, u64::MAX, 384) in both modes. SA's G-B reads `late_loads_total` as SP writes it.
- **SA's other reads match SP's writes:**
  - `retained_error_text` reads the own and parked slots' `error` and `observable_error`;
  - `ordinary_solve_attempted` reads the invocation's seeds against `requested_cases`;
  - `capture_bytes` reads the shared adapter.

### Item 7: the fault tests

I read `b1_sp_w_c2_transaction_faults_and_abandonment`, `…_through_retained_w1_publishes_t12`, `b1_sp_t6_…`, `b1_sp_t6_t7_…` and `b1_sp_t5_…`. **Each fault case asserts:**
- the cause and the selected bits;
- the ordinary owner untouched;
- every armed fault fired.

**What they cover:**
- custody (Preparation, 0);
- preparation on C (successor without C's source) and on A (Native, 0);
- the call failure (Native, 0);
- staging (0b01);
- all eight serializer checks (0b01);
- precommit G1 (0b01);
- R-b′ (`Untranslated … recovery_finding`, 0b01).

Through `retained_w1`, the T-12 bytes have the detail on A's notice only.

**My mutants of decision 5's set are all killed:** M47 custody, M41 call failure, M48 no case selected, M49 staging, M51 precommit, M52 R-b′, and M54 the T-10 order.

### Item 8: the guards

- **s11f:** 11/11, including rule 8. No `lib.rs` hunk of SP adds `+=`, `-=`, `.sum(` or `fold(` (my grep of SP's added lines; see N-4 for `retained_product.rs`).
- **PP-tests' admission guard:** 5/5. The runner's admission test: 3/3, the same as at I1. RE's carriers: 17/17.
- **The one changed expected text** is `u3_n9_single_parse_custody`: `serialize_frozen(&frozen, &staged, capture)` → `serialize_cases(&mut prepared, &staged, capture)`.
  - **The reason holds:** the serializer still takes the parse's borrowed half.
  - The permitted section now also covers `w1_transaction`, under the same forbidden list.
  - `borrowed_raw()` is still read exactly twice: `w1_case_ids` and precommit.
- **`u1_serializer_reads_no_legacy_work_field`** passes. `serialize_cases` and its helpers sit before the first `#[cfg(test)]`, and none of the forbidden reads occurs (my check).

### Item 9: weakening (every removed line of `98a77c716e..603e238517` in SP's 8 files)

**`lib.rs`:**
- The one-notice `ReservedNotice::reserve`/`publish` became `ReservedNotices`, with the same checks.
- `w1_case_id` became `w1_case_ids`.
- The one-case body of `retained_w1` became `w1_transaction`, with the same order and causes at c = 1.
- The hooks now take `PreparedCases`.

**`retained_product.rs`:**
- **The old custody prelude:** its checks all survive in `prepared_custody`. The only exception is the trace-cost line (item 5).
- `freeze_candidate`'s body moved into `freeze_case`.
- `FrozenCandidate`'s staging, fallback and trace accessors moved to `PreparedCases` and `CaseAttempt`.
- `apply_prepared_overlay` gained a scope.

**`retained_wire.rs`:**
- `serialize_frozen`, and `SelectedCandidate` for `FrozenCandidate`, are removed.
- `material_basis`, `product_attempt` and `bind_preparation` are parameterized.
- `ordinary_value`'s diagnostic loop moved to `case_diagnostic_refs`.

**The tests:**
- `native.as_ref()` → `native_pair()` is mechanical. In `assert!(….native_pair().is_none())` it is equivalent at c = 1, where `native_call` sets both halves together.
- `i51_c0`'s "multiple cases" is restated: the refusal moves from the late hook to custody (`prepared case count`), and is still asserted.
- `u3_r2` gains W-C2's two notices.
- **No assertion is removed or loosened.**

### Item 10: mutants (`E/mutants/`)

**55 of my own**, each an exact edit of a fresh `MUT` copy, then PP's whole `--lib` suite, registered. A pristine control on the same target shows only t13 failing. The sources were restored and checked against `603e238517` (`restored_src*.sha256`). **I wrote them before reading I85's return,** with my own edits. Some test the same PLAN_v2 §2.2 items as I85's 29. The R3′ S-mutants are mine from the early read; the brief asks for S13, S14 and S20 again.

| Group | Mutants | Result |
|---|---|---|
| R3′'s S3, S4, S7, S9, S10, S13, S14, S18, S20 | 9 | **9 killed** |
| T-12 (detail placement, bit order, notice order, \|A\| reservation: S13) | M02 to M05, M07, B_M53 | **6 killed** |
| T-10 and decision 5 (each member) | M06, M47, M48, M49, M51, M52, B_M54 | **7 killed** |
| Domain re-check (c = C + 1) | M46 | **equivalent** (N-2); one combination is round 1's S15 |
| T-4 and T-7 (A, not every case) | M08 | killed |
| T-8 (Runs to cases, selection test, call failure, source move-back, R3P-4) | M09, M10, M41, M11, B_R3P4 | **4 killed; M11 survives** (N-1; on I3M the reader catches it at G8) |
| T-11 record point | M12 (C's snapshot after T-9), M13 (A's before its proof) | **2 killed** |
| T-9 scope, T-6 order and A | M33, M34, M36, M37 | **4 killed** |
| The headline rule | M27, M29 killed; **M28, M31, M32 survive** | M28 and M32 → SF-2; M31 → N-2 |
| T-11 staging order | M14 (unavailable first) and M38 (`not_required` gets a notice) killed; **M15 (selected reversed) survives** | SF-2 |
| `attempt_ref`, ids, ordinal mapping | M18, M20, M21, M22, M23, M24, M25 killed; **M16, M17, M19 survive** | SF-2 |
| D6a, D4d | **M26, M43 survive** | N-2 |

**Totals:**
- **44 killed by assertions.** One of them, S13, is also caught by `publish`'s test-build allocation assert; its first failure is t5's capacity assertion.
- **11 survive:**
  - two equivalent: M46, and M26 on every input I found;
  - two unreachable or with no input: M31 and M43;
  - M11, caught after I3;
  - six for SF-2.

**On I3M, the seven re-runnable survivors** (`E/mutants/summary_i3.txt`):
- the reader refuses M11, M16, M17, M19, M28 and M32 on at least one of my orders;
- the reader accepts M15; only my staging check catches it.

### Item 11: the suites against I1, test by test (`E/suites/`)

| Suite | I1 | Head | Differences |
|---|---|---|---|
| PP registered, all targets | 712 ok, 1 failed (t13), 11 ignored | 735 ok, 1 failed (t13), 11 ignored | only **+15 `b1_sp_*`** and **+8 `b1_sa_*`**, each ok |
| PP Stale (`--lib`) | 549 / 1 / 11 | 572 / 1 / 11 | the same 23 additions. **Stale equals registered** outcome for outcome, at both revisions |
| Runner (headless) | 85 ok, 2 failed | identical | 0. The two load-reference failures are I1's own |
| Witnesses (`--ignored`) | 10/10 | 10/10 | output identical after normalizing line numbers and timings (`witness_normalized_diff.txt`) |
| RE `retained_precision_carriers` | 17/17 | 17/17 | 0 |

t13 and the two runner tests fail identically at I1 and the head, as they did in round 1.

## 3. The PR-head ledger, extended (`E/ledger/`)

**Every hunk of `98a77c716e..603e238517` in SP's eight files is SP's, and is reviewed here.**
- **115 default (−U3) hunks.** Each is attributed by `git blame` of its added lines inside the range, and by each commit's own removed lines, to SP's commits: `56c5579f07`, `7458527ff7`, `89222942ee`, `59393e2d43`, `0ec3651a36`, `8db7906ee7`, `c17340d50b` and `603e238517`.
- **Neither merge touches SP's files.** I1's absorption of main `2007709549` brings the readers, records and rules. I2 (`eca6c00a72`) brings SA's `retained_memory.rs`, its law tests and the runner's admission test, and RV112 reviewed those.
- With round 1 (`47a3bdfcf5..a8e719f5b4`) and ADDENDUM_01 (`..98a77c716e`), **every ST and SP hunk on the branch up to `603e238517` is now reviewed by RV-P.**
- **Outside RV-P's ledger:** main's changes absorbed at I1 (their own PRs, #1106 and #1107), and SA's hunks merged at I2 (RV112).

| # | File (PP) | Hunk (`98a77c716e` → `603e238517`) | Context | +/− | SP commit(s) | Reviewed |
|---|---|---|---|---|---|---|
| 1 | `lib.rs` | `-2994,7 +2994,8` | fn permitted_run( | +2/−1 | `603e238517` | RV109 r2 |
| 2 | `lib.rs` | `-3039,19 +3040,13` | fn receipt_encoding_detail(check: retained_wire::ReceiptC… | +5/−11 | `56c5579f07` | RV109 r2 |
| 3 | `lib.rs` | `-3066,39 +3061,111` | impl ReservedNotice { | +91/−19 | `56c5579f07`, `7458527ff7` | RV109 r2 |
| 4 | `lib.rs` | `-3156,7 +3223,8` | fn dn_trigger_excluded(seed: &retained_product::OrdinaryS… | +2/−1 | `56c5579f07` | RV109 r2 |
| 5 | `lib.rs` | `-3173,49 +3241,89` | fn retained_w1( | +60/−20 | `56c5579f07`, `7458527ff7` | RV109 r2 |
| 6 | `lib.rs` | `-3226,13 +3334,11` | fn retained_w1( | +3/−5 | `7458527ff7` | RV109 r2 |
| 7 | `lib.rs` | `-3249,6 +3355,8` | pub(crate) mod retained_tests_hooks { | +2/−0 | `56c5579f07` | RV109 r2 |
| 8 | `lib.rs` | `-3332,20 +3440,24` | pub(crate) mod retained_tests_hooks { | +9/−5 | `7458527ff7`, `89222942ee` | RV109 r2 |
| 9 | `lib.rs` | `-3355,6 +3467,8` | pub(crate) mod retained_tests_hooks { | +2/−0 | `7458527ff7` | RV109 r2 |
| 10 | `retained_product.rs` | `-64,7 +64,7` | pub(super) struct SolverObservations { | +1/−1 | `7458527ff7` | RV109 r2 |
| 11 | `retained_product.rs` | `-128,7 +128,14` | pub(super) struct ProductCapture { | +8/−1 | `7458527ff7` | RV109 r2 |
| 12 | `retained_product.rs` | `-147,8 +154,140` | pub(super) struct ProductCapture { | +133/−1 | `56c5579f07`, `7458527ff7` | RV109 r2 |
| 13 | `retained_product.rs` | `-687,7 +826,10` | impl ProductCapture { | +4/−1 | `56c5579f07` | RV109 r2 |
| 14 | `retained_product.rs` | `-704,6 +846,21` | impl ProductCapture { | +15/−0 | `56c5579f07` | RV109 r2 |
| 15 | `retained_product.rs` | `-716,7 +873,14` | impl ProductCapture { | +8/−1 | `56c5579f07` | RV109 r2 |
| 16 | `retained_product.rs` | `-864,6 +1028,11` | impl ProductCapture { | +5/−0 | `7458527ff7` | RV109 r2 |
| 17 | `retained_product.rs` | `-886,7 +1055,7` | impl ProductCapture { | +1/−1 | `7458527ff7` | RV109 r2 |
| 18 | `retained_product.rs` | `-897,12 +1066,13` | impl ProductCapture { | +2/−1 | `7458527ff7` | RV109 r2 |
| 19 | `retained_product.rs` | `-911,12 +1081,13` | impl ProductCapture { | +2/−1 | `7458527ff7` | RV109 r2 |
| 20 | `retained_product.rs` | `-926,6 +1097,89` | impl ProductCapture { | +83/−0 | `56c5579f07` | RV109 r2 |
| 21 | `retained_product.rs` | `-1293,7 +1547,8` | impl ProductCapture { | +2/−1 | `56c5579f07` | RV109 r2 |
| 22 | `retained_product.rs` | `-1674,11 +1929,15` | impl ProductCapture { | +6/−2 | `56c5579f07` | RV109 r2 |
| 23 | `retained_product.rs` | `-1713,7 +1972,8` | impl ProductCapture { | +2/−1 | `7458527ff7` | RV109 r2 |
| 24 | `retained_product.rs` | `-1742,18 +2002,23` | impl ProductCapture { | +8/−3 | `7458527ff7` | RV109 r2 |
| 25 | `retained_product.rs` | `-1767,7 +2032,7` | impl ProductCapture { | +1/−1 | `7458527ff7` | RV109 r2 |
| 26 | `retained_product.rs` | `-1964,7 +2229,7` | impl ProductCapture { | +1/−1 | `7458527ff7` | RV109 r2 |
| 27 | `retained_product.rs` | `-1984,7 +2249,7` | impl ProductCapture { | +1/−1 | `7458527ff7` | RV109 r2 |
| 28 | `retained_product.rs` | `-2008,10 +2273,10` | impl ProductCapture { | +2/−2 | `7458527ff7` | RV109 r2 |
| 29 | `retained_product.rs` | `-2217,6 +2482,7` | fn validate_final_metadata( | +1/−0 | `7458527ff7` | RV109 r2 |
| 30 | `retained_product.rs` | `-2433,6 +2699,14` | fn validate_final_metadata( | +8/−0 | `7458527ff7` | RV109 r2 |
| 31 | `retained_product.rs` | `-3194,10 +3468,13` | impl ProductCapture { | +6/−3 | `56c5579f07` | RV109 r2 |
| 32 | `retained_product.rs` | `-3227,15 +3504,18` | impl ProductCapture { | +6/−3 | `56c5579f07`, `603e238517` | RV109 r2 |
| 33 | `retained_product.rs` | `-3247,11 +3527,10` | impl ProductCapture { | +4/−5 | `56c5579f07`, `7458527ff7`, `59393e2d43`, `0ec3651a36` | RV109 r2 |
| 34 | `retained_product.rs` | `-3307,6 +3586,360` | pub(super) struct PreparedCaseFailure { | +354/−0 | `56c5579f07`, `7458527ff7`, `0ec3651a36`, `8db7906ee7` | RV109 r2 |
| 35 | `retained_product.rs` | `-3316,131 +3949,136` | impl ProductCapture { | +126/−121 | `56c5579f07`, `7458527ff7`, `0ec3651a36`, `8db7906ee7` | RV109 r2 |
| 36 | `retained_product.rs` | `-3463,17 +4101,12` | impl PreparedCase { | +4/−9 | `7458527ff7` | RV109 r2 |
| 37 | `retained_product.rs` | `-3490,16 +4123,35` | struct PreparedPayload { values:k::FrozenProductValues, m… | +24/−5 | `7458527ff7` | RV109 r2 |
| 38 | `retained_product.rs` | `-3514,11 +4166,11` | impl<'a> ProductCaseView<'a> { | +4/−4 | `7458527ff7` | RV109 r2 |
| 39 | `retained_product.rs` | `-3549,7 +4201,7` | impl PreparedCandidateRefusal { | +1/−1 | `7458527ff7` | RV109 r2 |
| 40 | `retained_product.rs` | `-3583,11 +4235,12` | impl ProductCapture { | +3/−2 | `7458527ff7` | RV109 r2 |
| 41 | `retained_product.rs` | `-3638,15 +4291,15` | impl ProductCapture { | +4/−4 | `7458527ff7` | RV109 r2 |
| 42 | `retained_product.rs` | `-3656,68 +4309,74` | impl PreparedCase { | +50/−44 | `7458527ff7` | RV109 r2 |
| 43 | `retained_product.rs` | `-3725,62 +4384,80` | impl PreparedCase { | +48/−30 | `7458527ff7` | RV109 r2 |
| 44 | `retained_product.rs` | `-3788,52 +4465,55` | fn apply_prepared_overlay(envelope:&mut MechanicsEnvelope… | +31/−28 | `7458527ff7` | RV109 r2 |
| 45 | `retained_receipt.rs` | `-123,7 +123,7` | fn summary_coverage<'a>(trace:&PreparedTrace,capture:&'a … | +1/−1 | `7458527ff7` | RV109 r2 |
| 46 | `retained_receipt.rs` | `-174,7 +174,7` | pub(super) fn project<'a>(trace:&'a PreparedTrace,capture… | +1/−1 | `7458527ff7` | RV109 r2 |
| 47 | `retained_receipt.rs` | `-191,7 +191,7` | pub(super) fn project<'a>(trace:&'a PreparedTrace,capture… | +1/−1 | `7458527ff7` | RV109 r2 |
| 48 | `retained_wire.rs` | `-738,9 +738,18` | fn range_trigger(e: &Enc, trigger: &RangeTrigger) -> Value { | +10/−1 | `7458527ff7` | RV109 r2 |
| 49 | `retained_wire.rs` | `-854,12 +863,16` | fn ordinary_members(e: &Enc, seed: &rp::OrdinarySeed) -> … | +9/−5 | `7458527ff7` | RV109 r2 |
| 50 | `retained_wire.rs` | `-886,7 +899,7` | fn material_basis(e: &Enc, capture: &rp::ProductCapture, … | +1/−1 | `7458527ff7` | RV109 r2 |
| 51 | `retained_wire.rs` | `-895,6 +908,8` | fn material_basis(e: &Enc, capture: &rp::ProductCapture, … | +2/−0 | `7458527ff7` | RV109 r2 |
| 52 | `retained_wire.rs` | `-964,7 +979,7` | fn case_source(e: &Enc, capture: &rp::ProductCapture, sou… | +1/−1 | `7458527ff7` | RV109 r2 |
| 53 | `retained_wire.rs` | `-1212,7 +1227,7` | fn public_error(e: &Enc, f: &rr::FailureRef<'_>, capture:… | +1/−1 | `7458527ff7` | RV109 r2 |
| 54 | `retained_wire.rs` | `-1261,7 +1276,8` | fn proof_trace(e: &Enc, view: &rr::PreparedAttemptView<'_… | +2/−1 | `7458527ff7` | RV109 r2 |
| 55 | `retained_wire.rs` | `-1292,7 +1308,7` | fn product_attempt(e: &Enc, view: &rr::PreparedAttemptVie… | +1/−1 | `7458527ff7` | RV109 r2 |
| 56 | `retained_wire.rs` | `-1364,6 +1380,8` | fn kernel_outcome<'a>(e: &Enc, outcome: &'a k::ExecutionO… | +2/−0 | `7458527ff7` | RV109 r2 |
| 57 | `retained_wire.rs` | `-1379,10 +1397,21` | fn run_value(e: &Enc, run: &k::RunOrigins, records: &[k::… | +14/−3 | `7458527ff7` | RV109 r2 |
| 58 | `retained_wire.rs` | `-1401,16 +1430,29` | fn invocation_arrays(e: &Enc, inv: &k::RecordedInvocation… | +14/−1 | `7458527ff7` | RV109 r2 |
| 59 | `retained_wire.rs` | `-1421,28 +1463,44` | fn ordinary_value(e: &Enc, env: &Value, case_id: &str, ca… | +26/−10 | `7458527ff7` | RV109 r2 |
| 60 | `retained_wire.rs` | `-1453,10 +1511,11` | fn finish(e: Enc, mut env: Value, invocation: &source_rec… | +4/−3 | `56c5579f07` | RV109 r2 |
| 61 | `retained_wire.rs` | `-1469,13 +1528,6` | pub(super) fn serialize_selected(candidate: &rp::PrivateP… | +0/−7 | `7458527ff7` | RV109 r2 |
| 62 | `retained_wire.rs` | `-1489,13 +1541,6` | impl SelectedCandidate for rp::PrivatePreparedCandidate { | +0/−7 | `7458527ff7` | RV109 r2 |
| 63 | `retained_wire.rs` | `-1506,7 +1551,7` | fn serialize_selected_from(candidate: &impl SelectedCandi… | +1/−1 | `7458527ff7` | RV109 r2 |
| 64 | `retained_wire.rs` | `-1539,8 +1584,8` | fn serialize_selected_from(candidate: &impl SelectedCandi… | +2/−2 | `56c5579f07`, `7458527ff7` | RV109 r2 |
| 65 | `retained_wire.rs` | `-1550,10 +1595,10` | fn serialize_selected_from(candidate: &impl SelectedCandi… | +4/−4 | `7458527ff7` | RV109 r2 |
| 66 | `retained_wire.rs` | `-1575,9 +1620,13` | pub(super) const UNAVAILABLE_MESSAGE: &str = "Retained-pr… | +6/−2 | `7458527ff7` | RV109 r2 |
| 67 | `retained_wire.rs` | `-1608,7 +1657,7` | pub(super) fn serialize_unavailable(refused: Refused<'_>,… | +1/−1 | `7458527ff7` | RV109 r2 |
| 68 | `retained_wire.rs` | `-1633,8 +1682,8` | pub(super) fn serialize_unavailable(refused: Refused<'_>,… | +2/−2 | `7458527ff7` | RV109 r2 |
| 69 | `retained_wire.rs` | `-1648,7 +1697,7` | pub(super) fn serialize_unavailable(refused: Refused<'_>,… | +1/−1 | `7458527ff7` | RV109 r2 |
| 70 | `retained_wire.rs` | `-1668,18 +1717,292` | pub(super) fn serialize_unavailable(refused: Refused<'_>,… | +278/−4 | `56c5579f07`, `7458527ff7` | RV109 r2 |
| 71 | `retained_wire.rs` | `-1979,7 +2302,8` | pub(super) fn test_after_conserved(before: u64, increment… | +2/−1 | `7458527ff7` | RV109 r2 |
| 72 | `retained_facade_tests.rs` | `-406,7 +406,9` | fn u3_n9_single_parse_custody() { | +3/−1 | `7458527ff7` | RV109 r2 |
| 73 | `retained_facade_tests.rs` | `-482,6 +484,11` | fn u3_capture_permit_is_linear() { | +5/−0 | `7458527ff7` | RV109 r2 |
| 74 | `retained_facade_tests.rs` | `-495,8 +502,10` | fn u3_r2_base_readers_accept_the_unavailable_notice() { | +3/−1 | `7458527ff7` | RV109 r2 |
| 75 | `retained_facade_tests.rs` | `-506,7 +515,11` | fn u3_r2_base_readers_accept_the_unavailable_notice() { | +5/−1 | `7458527ff7` | RV109 r2 |
| 76 | `retained_facade_tests.rs` | `-1325,3 +1338,718` | fn b1_t4_retained_w1_applies_decision_21_and_keeps_a_seed… | +715/−0 | `56c5579f07`, `7458527ff7`, `89222942ee`, `0ec3651a36`, `c17340d50b`, `603e238517` | RV109 r2 |
| 77 | `retained_product_tests.rs` | `-134,7 +134,7` | fn i50_dump(e: &MechanicsEnvelope, o: &ProductCapture, mo… | +1/−1 | `7458527ff7` | RV109 r2 |
| 78 | `retained_product_tests.rs` | `-194,7 +194,7` | fn i50_actual_named_case_both_modes_complete_private_verd… | +1/−1 | `7458527ff7` | RV109 r2 |
| 79 | `retained_product_tests.rs` | `-443,7 +443,7` | fn i50_actual_support_bijections_and_accounting_prefixes() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 80 | `retained_product_tests.rs` | `-619,7 +619,7` | fn i50_support_coverage_native_non_aliasing_and_g5a() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 81 | `retained_product_tests.rs` | `-914,7 +914,7` | fn i50_observation_custody_presence_fields_and_failure_pr… | +1/−1 | `7458527ff7` | RV109 r2 |
| 82 | `retained_product_tests.rs` | `-1048,7 +1048,7` | fn actual_ordinary_zero_loaded_capture_and_final_verdict() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 83 | `retained_product_tests.rs` | `-1140,7 +1140,7` | fn actual_ordinary_zero_loaded_capture_and_final_verdict() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 84 | `retained_product_tests.rs` | `-1442,7 +1442,7` | fn adapter_byte_prefix_count_range_and_loss_are_typed() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 85 | `retained_product_tests.rs` | `-1470,7 +1470,7` | fn actual_sparse_zero_and_cancelled_summary_coverage_is_n… | +1/−1 | `7458527ff7` | RV109 r2 |
| 86 | `retained_product_tests.rs` | `-1486,7 +1486,7` | fn actual_sparse_zero_and_cancelled_summary_coverage_is_n… | +1/−1 | `7458527ff7` | RV109 r2 |
| 87 | `retained_product_tests.rs` | `-1603,7 +1603,7` | fn rv60_fixed_mode_sign_refuses_in_isolated_synthetic_zer… | +1/−1 | `7458527ff7` | RV109 r2 |
| 88 | `retained_product_tests.rs` | `-1798,7 +1798,7` | fn i47_actual_selected_material_zero_loaded_capture_and_f… | +1/−1 | `7458527ff7` | RV109 r2 |
| 89 | `retained_product_tests.rs` | `-2008,7 +2008,7` | fn i47_actual_selection_and_resolver_validity_controls() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 90 | `retained_product_tests.rs` | `-2025,7 +2025,7` | fn i47_modulus_record_closed_binding_and_independent_pres… | +1/−1 | `7458527ff7` | RV109 r2 |
| 91 | `retained_product_tests.rs` | `-2170,7 +2170,7` | fn i47_modulus_record_closed_binding_and_independent_pres… | +1/−1 | `7458527ff7` | RV109 r2 |
| 92 | `retained_product_tests.rs` | `-2348,7 +2348,7` | fn i47_successful_aggregate_capture_sticky_errors_and_wor… | +1/−1 | `7458527ff7` | RV109 r2 |
| 93 | `retained_product_tests.rs` | `-2393,7 +2393,7` | fn i51_first_prepared_native_both_modes() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 94 | `retained_product_tests.rs` | `-2405,7 +2405,7` | fn i51_first_prepared_native_both_modes() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 95 | `retained_product_tests.rs` | `-2463,7 +2463,7` | fn i51_c0_isolated_late_hook_custody_and_prefixes() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 96 | `retained_product_tests.rs` | `-2495,7 +2495,7` | fn i51_c0_isolated_late_hook_custody_and_prefixes() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 97 | `retained_product_tests.rs` | `-2503,9 +2503,11` | fn i51_c0_isolated_late_hook_custody_and_prefixes() { | +4/−2 | `56c5579f07` | RV109 r2 |
| 98 | `retained_product_tests.rs` | `-2515,11 +2517,21` | fn i51_c0_isolated_late_hook_custody_and_prefixes() { | +11/−1 | `56c5579f07`, `7458527ff7` | RV109 r2 |
| 99 | `retained_product_tests.rs` | `-2531,7 +2543,7` | fn i51_c0_isolated_late_hook_custody_and_prefixes() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 100 | `retained_product_tests.rs` | `-2568,7 +2580,7` | fn i51_c0_isolated_guard_accounting_boundaries() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 101 | `retained_product_tests.rs` | `-2620,11 +2632,11` | fn i51_actual_exact_pressure_selection_suppresses_all_pre… | +2/−2 | `7458527ff7` | RV109 r2 |
| 102 | `retained_product_tests.rs` | `-2642,7 +2654,7` | fn i51_ready_for_controls()->(MechanicsEnvelope,retained_… | +1/−1 | `7458527ff7` | RV109 r2 |
| 103 | `retained_product_tests.rs` | `-2670,7 +2682,7` | fn i51_frozen_owner_values_and_numeric_refusal_controls() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 104 | `retained_product_tests.rs` | `-2919,7 +2931,7` | fn prepared_trace_prior_owned_cause_precedes_sticky_adapt… | +1/−1 | `7458527ff7` | RV109 r2 |
| 105 | `retained_product_tests.rs` | `-3055,7 +3067,7` | fn i61_fk_certificate_failure(variant:usize)->(retained_p… | +1/−1 | `7458527ff7` | RV109 r2 |
| 106 | `retained_wire_tests.rs` | `-174,7 +174,7` | fn u2_foreign_owner_refused() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 107 | `retained_wire_tests.rs` | `-234,7 +234,7` | fn u1_stage_slots_require_an_exact_status() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 108 | `retained_wire_tests.rs` | `-518,7 +518,7` | fn u1g2_d38_failure_before_any_run() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 109 | `retained_wire_tests.rs` | `-574,7 +574,7` | fn u2_failure_path_foreign_owner_refused() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 110 | `retained_wire_tests.rs` | `-594,7 +594,7` | fn u1g2_failed_verification_keeps_its_reason() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 111 | `retained_wire_tests.rs` | `-705,7 +705,7` | fn u1g2_run_and_body_charge_conservation_negatives() { | +1/−1 | `7458527ff7` | RV109 r2 |
| 112 | `retained_tests_hooks/grant2.rs` | `-14,9 +14,10` | use super::{arm, consume, Armed}; | +3/−2 | `7458527ff7` | RV109 r2 |
| 113 | `retained_tests_hooks/grant2.rs` | `-27,11 +28,23` | pub(crate) struct Counts { pub(crate) runs: usize, pub(cr… | +13/−1 | `7458527ff7` | RV109 r2 |
| 114 | `retained_tests_hooks/grant2.rs` | `-48,10 +61,12` | pub(super) fn merge(a: &mut Armed, faults: &Armed) { | +4/−2 | `56c5579f07` | RV109 r2 |
| 115 | `retained_tests_hooks/grant2.rs` | `-64,10 +79,23` | pub(crate) fn fail_next_late_gate() { arm(/a/ a.late_gate… | +14/−1 | `56c5579f07`, `0ec3651a36` | RV109 r2 |

**Carried open:** SF-1 and SF-2 (test- and records-only), for I3. Later rounds extend this ledger with I3, SR, SC and SQ.

## 4. For ROOT

1. **SF-1, N-16:** rule whether the two build-provenance flags are within N-16's "outcomes". I recommend yes, as C2 §4's sharing, recorded with that cause. Either way, the difference belongs in the records before I3's pins absorb it.
2. **SF-2:** I recommend adding the (C, B, A) and (A, A2) pins at I3, beside W-C2's. Both pass the accepted reader on my scratch merge, in both modes.
3. **The headline rule is confirmed as ruled.** No erratum is needed beyond the change-record line ROOT already directed.
4. **N-4** is a records correction to PLAN_v2 §1, or a guard extension. ROOT's choice.
5. **No stop fired.** No reader, FK, schema or c = 1 byte changed. I85's other claims check out: c = 1 identity, W-C2 before I3, the ordinal mapping, coexistence, the fault tests, the seam, the suites, the guards, and the I3 front-run (pins and the one change). The exception is N-16 (SF-1).

## 5. Host, commands and cleanup

- **Copies:** `git archive` of `262bd687f0` and `603e238517` (twice: `HEAD` and `MUT`), and of `603e238517` plus `git show b5cb7faaeb:<3 RE files>` for `I3M`, all from `WT/b1` with `GIT_OPTIONAL_LOCKS=0`.
- **Jobs** (scripts in `E/tools/`, each job through `runjob.sh` → `WT/tools/t3_cargo.sh`):
  - `r2_phase1.sh`: the suites at I1 and the head;
  - `r2_probe.sh`: the probe, with shims `zz_rv109_rev.{i1,head}.rs`;
  - `r2_i3.sh` and `r2_i3b.sh`: the I3 front-run and its document pins;
  - `r2_mutants.sh` and `r2_mutants_b.sh`: batches A and B (`sp2_mutants.py`);
  - `r2_i3_mut.sh`: the survivors on I3M.
- **Analysis** (system `python3`, read-only): `suite_diff.py`, `stage_compare2.py`, `wc2_summary.py`, `mutant_summary.py`, `ledger.py`, `ledger_table.py` and `sanitize.py`. The PY base reader ran in-process from `HEAD`'s sources.
- **Cleanup:** `WT/rv109/{i1,head,i3,mut}` and the eleven targets `WT/targets/rv109-r2-*` were deleted at 22:29 UTC, after the last job ended at 22:25 UTC. My scratch `S` (logs, probe outputs, the W-C2 documents, mutant logs) is kept. No process of mine is running.
- **Time:** about 2.5 h of agent time (20:25–22:45 UTC), against 6–9 h.

## 6. Limits

- **The TS reader was read, not run.** Its §6 headline reducer has the same order as RS and PY.
- **I3M is my reconstruction of I3,** not ROOT's merge. I85's `post_i3.py` and fixtures were not applied. Its pins were reproduced from my own document construction.
- **N-16 compares the kernel's records** through `AttemptRecord`'s manual `Debug` form, which prints every field, including all work and stage counters; `work_status` is printed only when it is not exact. A one-case run of C publishes no receipt, so receipt-level fields (build references, Run work) were compared only inside the batch receipt.
- **The probe ran in the registered dev/test build with its prints.** It establishes outcomes and bytes, not S1 stack margins or memory, which are SQ's.

## 7. Records

- `REVIEW.md` (this file) and `SHA256SUMS`.
- `evidence/suites/`: filtered logs of every suite, the test-by-test differences, and the witness diff.
- `evidence/probe/`: the probe and both shims, the four JSONL outputs, `probe_compare.txt`, `stage_compare.txt`, the filtered run lines, and `fixture_files.txt`.
- `evidence/wc2/`: the W-C2 summary at the head, `n16_record_diff.txt`, and the head's W-C2 document hashes.
- `evidence/headline/`: the PY base reader on every staged successor and its alternatives.
- `evidence/i3/`: the overlay hashes, the merge's PP, RE and document-pin logs, its W-C2 summary, and hashes.
- `evidence/mutants/`: the lists, the batch A and B summaries, each mutant's filtered log, `summary_i3.txt` and `i3_merge/`.
- `evidence/ledger/ledger_sp.md` (the generated 115 rows), `evidence/tools/`, and `evidence/cargo_jobs_rv109_r2.log`.
