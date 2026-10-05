# RV80 — confirmation of the Rust reader repairs (snapshot 07a)

RV80 is a TASK (Type 2) dispatched by ROOT (HELP_HUMAN) under `BRIEFS/RV78_RV81_CONFIRMATION.md`. ROOT is the return path. RV80 resumes as the reviewer of `reader_review_01`, did not write any of the repairs, and had no descendants.

- **Candidate:** READER `b36739112a48da40cd22a7a3bdf7691bf2ad4404`, built from a `git archive` into `WT/rv80/`. NUM is at `c57496275b`.
- **Under review:** `P/core/reporting/result_export/src/retained_precision.rs` `a0973ae5455ce15a12d4717b49d4af1d3796a24e3023c7d88c0788d02ecf99de`. The test file is `e1ef9da64055…`, and `src/lib.rs` (`375b07313518…`) is unchanged. These equal I63's RETURN_07A.
- **Shared corpus:** 07a `a6fa398731` (15 cases, 236 mutations, 19 must-pass entries). Schema `07951edacf`.
- **Diff reviewed:** `6b607fd01f..b36739112a` for both Rust files: source +483/−116 lines, tests +551 lines.
- **Run window:** 2026-10-03 18:26–18:47 MDT (WT/rv80 deleted at 18:47), inside the 90-minute box. Memory guard PID 5387 was running throughout.
- **Host:** the default toolchain (cargo/rustc 1.97.1), `CARGO_TARGET_DIR=WT/targets/rv80`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `CARGO_NET_OFFLINE=true`, `--locked --offline`. One Cargo job at a time.
  - **Disclosure:** RV80's first mutant run reused review_01's script, which still set `DEVELOPER_DIR`. RV80 killed it within a minute (exit 143) before any result was recorded, restored the source from READER (re-hashed to `a0973ae5`), removed the variable from the script, and reran everything on the default toolchain. No result below comes from that aborted run.
- **No** Git writes, installs, new tooling, or native/solver/DEC-025 jobs. No Python or TypeScript was run. Python lines are cited from `retained_precision.py` at this head, read only.

## Verdict

**PASS** — 0 BLOCKING, 2 SHOULD-FIX, 4 NOTE (new findings).

Every review_01 finding is fixed or superseded by a decision, and each is confirmed by RV80's own probes on this head (S1: PR1, PR5; S2: PR2). The baseline passes 35/35. The exact-rational oracle again finds 0 mismatches in 8,238 vectors (the arithmetic is unchanged). 29 of RV80's 36 mutants are killed; every survivor is explained below.

Two SHOULD-FIX items remain:
- **C1:** one class-1 ATTEMPT predicate reads the running meter, a native-WORK value. Under D3 that turns a meter-chain WORK defect into an ATTEMPT report, unlike Python.
- **C2:** a challenge to D5d's scope. C2:54 binds every reason quantity to the layout, not only stop_rule.

Neither can make a statement eligible.

## Original findings: dispositions

| Finding | Disposition | Evidence on `b36739112a` |
|---|---|---|
| RV80-S1: native error with a selected Run; run_ref unchecked | **Fixed** (D4d). The `native` arm (RS:2149–2159) requires a non-null, nonselected Run with `run_ref` equal to the case Run's id. | PR1 (native, own selected Run) and PR5 (foreign run_ref): G5 PRODUCT_ATTEMPT_MISMATCH, with and without invocation. M19 is equivalent: a selected Run still fails the `refused` arm. M13 survives (NOTE C3). |
| RV80-S2: G3 accepted any unique old member order | **Fixed** (D1). Old, prepared and new ids are `0..len−1` (RS:667–676). | PR2: G3 COVERAGE_MISMATCH with and without invocation (was G8 / Ok). M20 is killed. |
| RV80-N1: native-class order unruled | **Superseded by D3** (ATTEMPT wins; native WORK deferred, RS:813–825, :1356) and implemented. | PR9 (WORK then ATTEMPT) gives ATTEMPT; PR10 (WORK only) gives WORK; M21 (immediate WORK) is killed. One predicate escapes the convention: **C1**. |
| RV80-N2(a): captured_prefix references at G3 | **Superseded by D1.** The member part stays at G3 (RS:677–680); the null source/run and unavailable result moved to G5 (RS:1880–1885). | PR8 (captured_prefix with a run_ref): G5 PRODUCT_ATTEMPT_MISMATCH. M27 is equivalent: the G5 stage and Ready rules give the same code. |
| RV80-N2(b): an empty body inventory under a non-null roster | **Not dispositioned on its subject.** D1's conditional, as withdrawn at checkpoint A, addresses the *member* inventory. Rust still fails G3 here (RS:731–741), as before. Python's G3 (PY:1603–1605) still passes an empty roster over an empty inventory. | PR11: Rust G3 COVERAGE_MISMATCH. Python by reading. See NOTE C4. |
| RV80-N2(c): Run origin owner at G3 | **Superseded by D1** and moved to G5 class 1 (RS:883). G3 keys runs by their owning case (RS:629–632). | M35 is equivalent: the source-owner conjuncts at RS:886–887 give the same ATTEMPT for a single owner move. M07 is killed. |
| RV80-N3: surviving mutants | **Largely fixed** (D13). M07, M08, M12, M14, M15, M16 and M18 are now killed by reader-local or shared tests. M02 and M06 still have no base. M13 now survives for a different reason (C3). | `MUTANTS.json` |
| RV80-N4: `reader_logic` public surface | **Superseded by D14** (keep, marked internal). The doc comment now says "Not a public API" (RS:1570–1576). Three more hooks were added: `schedule_in`, `accounting`, `g7_error`. | Still `#[doc(hidden)]`; they return no Validation and grant nothing. |
| RV80-N5: R3 attempt-wide | **Fixed** (D8 R3′). A cause is checked against its owner's statuses (`fault_owner`, RS:1672–1690), and R4 is added. | M05 (containment dropped) and M29 (R4) are killed. |
| RV80-N6: stale comment | **Fixed** (D15), RS:4088–4090. | — |
| RV80-N7: defensive non-emittable shapes | No change needed; unchanged. | — |
| RV80-N8: 42 versus 43 checklist IDs | **Fixed** (D12). | Ruling text |

## New findings

| ID | Severity | Location | Finding and evidence | Remedy |
|---|---|---|---|---|
| RV80-C1 | SHOULD-FIX | `retained_precision.rs:912–916` | **The group-null idle predicate also requires the running meter, `current >= invocation_limit`.** `current` is the native-WORK meter chain (its equality with `invocation_before` at :898 is a WORK predicate that D3 now defers). So a receipt whose idle run records an exhausted `invocation_before` while the chain is not exhausted reports **ATTEMPT** in Rust. Python decides the same N10 shape on the recorded `invocation_before` (PY:439, 675) and reaches WORK through its chain check (PY:589). The predicate is redundant on every consistent receipt: `g5_schedule` already enforces the recorded value for a group-null Run (RS:1414–1417). **PR14:** an idle F′ run with `invocation_before = 60B` gives ATTEMPT. With only that conjunct removed (M36), PR14 gives WORK, while the shared pin PR15 (`idle_budget_not_exhausted_no_group`) still gives ATTEMPT and the suite passes 35/35. This is a per-check ATTEMPT/WORK mapping difference of the kind D3 asked I62 to tabulate, and it breaks D3's goal of a fixed first code for every dual defect. | ROOT rules which value the N10 predicate reads. RV80 recommends the recorded `invocation_before` (all readers already check it) and dropping the running-meter conjunct, with a shared pin like PR14 expecting G5 WORK_MISMATCH. Under the D18 rule this removal needs ROOT's ruling first. |
| RV80-C2 | SHOULD-FIX (challenge to D5d) | RS:963–970; PY:603 | **D5d binds only `stop_rule` reasons to the Run's layout.** C2:54 says "Every quantity reference is resolved against that run's exact layout before use". C2:22 gives `verification_estimate` and `charge` the same `{quantity,body,kind}` payload, and C2:24 gives `publication_enclosure` the same plus a predicate. Natively all four come from a layout row (`FK/adaptive.rs:4169–4197`; publication enclosure from `prep.layout[index]`, :4714–4720). **PR12:** a rejected p512-ladder candidate whose reason is `verification_estimate` with quantity node 99 and kind `translation` validates `Ok` (estimate kinds are natively force or moment only). PR13 (the same edit as `stop_rule`) gives ATTEMPT. Python checks only `stop_rule` too (PY:603). | Widen D5d to all four quantity-bearing reasons (same body and kind as the layout row; estimate and charge kinds force or moment). Add shared pins at G5 ATTEMPT_MISMATCH. |
| RV80-C3 | NOTE | RS:2152; tests | **M13 survives:** with the `run_ref == run.id` conjunct removed, all 35 tests pass. Every native-error test uses a *selected* Run, which already fails through the nonselected requirement or the `refused` arm. No shared base has an unavailable case with a nonselected Run. So D4d's `run_ref` equality on a nonselected Run is implemented but unpinned. I63's RETURN credits the original M13 kill to PR5, but PR5 uses a selected Run. | Add a reader-local test with an unresolved or refused Run and a foreign `run_ref`, and a shared pin when a nonselected-Run base exists. |
| RV80-C4 | NOTE (challenge) | PY:1603–1605 vs RS:731–741 | **RV80-N2(b) concerned the *body* inventory.** It asked whether a non-null roster may sit on a source with no bodies, not about the member inventory that the checkpoint-A withdrawal addresses. A native citation for the body case exists: `FK/source.rs:497–498` refuses an empty node list (`SourceError::NoNodes`), so no CaseSource has zero bodies, and I57 §1 says "there is no empty-array representation of complete coverage". ROOT's earlier parity rule 2 ("a non-null empty coverage roster fails G3") applies. Rust complies (PR11); Python's G3 passes `[] == [] == []`. | Python adds the non-empty inventory at G3; add a shared pin (PR11's edit; expected G3 COVERAGE_MISMATCH). |
| RV80-C5 | NOTE | RS:1576–1598 | D14 keeps the hidden hooks, and phase 1 added three: `schedule_in`, `accounting`, `g7_error`. All are pure, return no Validation, and cannot reach eligibility. | Recorded for the eventual public-activation review. |
| RV80-C6 | NOTE | `retained_precision.rs` `g5_native` | D3's deferral continues class 1 with saturated or skipped values after a WORK fault (`tw`, the dangling-build `continue`), as settled reading 4 allows. C1 is the only ATTEMPT predicate RV80 found that reads such a value: `current` is the only one, and `amounts` is always pushed. No panic path was found, because the indices are checked before use. | None beyond C1. |

## Repair diff review (`6b607fd01f..b36739112a`)

**Removed or weakened checks:**
- **The G3 extras moved under D1:**
  - the Run origin owner (to G5 class 1, RS:883);
  - captured_prefix source/run/result (to G5, RS:1880–1885).
- **D6a dropped the case-naming requirement** for untyped `diagnostic_refs` (RS:3964); typed references stay strict.
- **D3 changed native WORK from immediate to deferred.**

Each follows its decision's text. No other check was removed. Every other change strengthens a rule:
- **G0:** absent policy, threshold and definition fields now fail.
- **G3:** ids must be `0..len` (RS:667–676), plus the unsourced CaseSource count.
- **New checks:** D4c, the D4d native arm, D5a–D5d, D6b, D8 R1′–R4 and the kernel scope.

The test diff removes only count assertions (178→236, 18→19) and the optional-rehash branch, which D11 now makes mandatory.

**Decisions read against their text:**
- **Implemented as written:**
  - D1 (RS:629–632, 667–690, 883, 1880–1885);
  - D2 (RS:445–500, `g0`: `receipt_version` exactly 1, the policies, thresholds and canonicalization required, both table byte hashes);
  - D3 (RS:813–825, 1356);
  - D4a–e;
  - D5a (RS:940–947), D5b (RS:952–957), D5c (RS:1459–1462) and D5d as worded;
  - D6a, D6b (RS:4047–4054), D6c and D6d;
  - D7;
  - D8 (RS:834–845 kernel scope; RS:1750–1788);
  - D14, D15 and D16 (RS:1016–1021);
  - D17 (`g5_ordinary` before `g5_products`, RS:4101–4102);
  - D18 (RS:3011, positivity unchanged).
- **The exceptions are C1 and C2.**

**Fail-closed:**
- `IMPLEMENTATION_COMPLETE = false` (RS:4090), and `validate`'s eligibility conjunction is unchanged.
- M01 is still killed by three tests.

## Mutation testing (36 single-edit mutants)

`MUTANTS.py` (with WT as its argument) applies M01–M35. M01–M18 are review_01's mutants re-applied, with M05 and M13 re-expressed on the repaired code. M19–M35 target the repairs. M36 was applied by hand with the same command. Each mutant starts from the pristine source (`a0973ae5`) and is restored and re-hashed afterwards.

**Totals:** 29 killed, 7 survived.

| Survivor | Edit | Reason |
|---|---|---|
| M02 | N17: drop `!case_over` | **No base:** a Budget(case) or Budget(invocation) terminal needs ≥20B or ≥60B of work, and the decisions defer it. |
| M06 | Relative class `>=` → `>` | **No base:** no equality row. D13 pinned only the 2^-988 switch. |
| M13 | D4d `run_ref == run.id` dropped | **Genuine gap** (C3): no nonselected-Run test. |
| M19 | D4d nonselected requirement dropped | **Equivalent:** the `refused` arm (RS:2156) still rejects a selected Run with the same code. |
| M27 | D1 captured_prefix G5 references dropped | **Equivalent:** the G5 stage, D4e and Ready rules give the same PRODUCT_ATTEMPT code for each of the three facts. |
| M35 | D1 origin-owner conjunct dropped | **Equivalent for single defects:** RS:886–887 (the source owner equal to the call owner and the case id) gives the same ATTEMPT. |
| M36 | Running-meter conjunct dropped (C1) | **Unpinned,** and redundant on consistent receipts. It changes only the PR14 dual-predicate code. |

**Killed:** M01, M03–M05, M07–M12, M14–M18, M20–M26, M28–M34. That covers fail-closed; I57 feasibility, p512 charge and data facts; R3′ and R4; G3 contiguity; the N10 boundary; the record bound; known differences 4–5; the Budget payload; the G5b floor; G7 detail; D1 ids; the D3 deferral; D5a, D5b and D5d; D6a and D6b; D4c; D8 kernel scope and R2′; D2 `receipt_version`; the D1 unsourced count; and D18 positivity.

## Commands and evidence

| Run | Command (from WT/rv80, default toolchain) | Result |
|---|---|---|
| Baseline | `cargo test --locked --offline --manifest-path WT/rv80/P/core/reporting/result_export/Cargo.toml --test retained_precision_contract -- --test-threads=2` | 35 passed (`BASELINE_TESTS.txt`) |
| Oracle | same, with `--test rv80_oracle` and `RV80_VECTORS` (review_01's generator, seed 80) | 0 mismatches over 8,238 (`ORACLE_RESULT.txt`) |
| Probes | same, with `--test rv80_probes` | PR1–PR15 (`PROBES.json`, `PROBES_HARNESS.rs`) |
| Mutants | `python3 MUTANTS.py WT`, plus M36 by hand | 29 killed, 7 survived (`MUTANTS.json`) |

`rv80_oracle.rs` and `rv80_probes.rs` were added only to RV80's archive copy; they are reviewer instruments. Bulk logs are in `WT/scratch/rv80_reader_confirm/`. WT/rv80 is deleted at the end, and WT/targets/rv80 is kept.

## Files read

| sha256 or identity | File |
|---|---|
| NUM `c57496275b` | R/BRIEFS/RV78_RV81_CONFIRMATION.md |
| NUM `c57496275b` | T3/ROOT_RULINGS_V1.md, from "Reader review RV78–RV81: consolidated ruling and the repair wave" to the end (D1–D18) |
| NUM `c57496275b` | R/I63/review_repair_07/RETURN.md, RETURN_P2.md, RETURN_07A.md |
| a0973ae5455c… / e1ef9da64055… | the Rust source and tests at READER `b36739112a` |
| READER `b36739112a` | P/core/analysis_runs/retained_precision.py (read only: G3 :1589–1608, N10 :434–440, :586–600, :675–706, D5d :603) |
| NUM | FK/adaptive.rs:4169–4197, 4729–4736 (reason payloads); FK/source.rs:492–499 (NoNodes); C2:20–24, 54 |

## Open for ROOT

1. **C1:** the value read by the N10 group-null predicate (recorded `invocation_before`, or the running meter), and a shared pin.
2. **C2:** extend D5d to `verification_estimate`, `charge` and `publication_enclosure`, and pin.
3. **C4:** apply parity rule 2 to the empty body inventory in Python, and pin.
4. **C3:** a nonselected-Run test for D4d's `run_ref` equality.
