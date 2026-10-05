# RV80 — independent review of the Rust retained-precision reader

RV80 is a TASK (Type 2) independent reviewer dispatched by ROOT (HELP_HUMAN) under
`BRIEFS/RV78_RV81_READER_REVIEW.md`. ROOT is the return path. RV80 did not write
any of the code under review, had no descendants, and did not use the authors'
tests as oracles.

- **Candidate:** READER = `WT/f2a-readers`, branch `codex/piping-f2a-readers-20261003`, head `6b607fd01f9819a3b6526dd9fde02cd3bc4db586`, built from a `git archive` into `WT/rv80/`.
- **Under review:** `P/core/reporting/result_export/src/retained_precision.rs` (sha256 `bd20dd9a8f888f0a3e81c981db7c02c7cfd48901ddc1ef203914d00be8b19ed0`), its wiring in `src/lib.rs` (`375b07313518…`, adds only `pub mod retained_precision;` at :1664) and `tests/retained_precision_contract.rs` (`5cbb6ba4ea47…`). All three equal I63's 06d RETURN.
- **Shared inputs (RV78's):** corpus `d02701ed6a` (15 cases, 178 mutations, 18 must-pass), schema `f943ebd351`.
- **Run window:** 2026-10-03 16:47–17:09 MDT (WT/rv80 deleted at 17:09), inside the two-hour box. Memory guard PID 5387 running throughout.
- **Host:** cargo/rustc 1.97.1. Every Cargo run set `DEVELOPER_DIR=/Library/Developer/CommandLineTools` for the process only (the Xcode licence is unaccepted; ROOT interim ruling), plus `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `CARGO_NET_OFFLINE=true`, `CARGO_TARGET_DIR=WT/targets/rv80`, `--locked --offline`. One Cargo job at a time. This is local review evidence, not gate evidence.
- **No** Git writes or index operations, installs, new tooling, or native/solver/DEC-025 jobs. No Python or TypeScript was run; Python behaviour below is read from `retained_precision.py` `55736ea65a` (read only) and is RV79's to confirm.

## Verdict

**PASS** — 0 BLOCKING, 2 SHOULD-FIX, 8 NOTE.

The reader runs every gate G0→G8 in contract order, keeps eligibility impossible
while `IMPLEMENTATION_COMPLETE` is false, and its arithmetic helpers agree with an
independent exact-rational oracle on all 8,238 vectors. The two SHOULD-FIX items
are acceptances of receipts the contract calls invalid, both confined to
unavailable product attempts (which can never make a statement eligible). They
should be fixed and pinned by shared mutations **before** the reader is accepted.
This verdict is not acceptance, eligibility or parity certification.

## Findings

| ID | Severity | Location | Finding and evidence | Remedy |
|---|---|---|---|---|
| RV80-S1 | SHOULD-FIX | `retained_precision.rs:1885–1907` | The S06 reason table sends `PublicFailure::native` with a **selected** Run to the facade arm (`_ =>` at :1903), which accepts it as `facade_certificate/facade` and never checks `native.run_ref`. S06:38: "native selected is invalid for this error"; S06:33: every Run reference resolves to the same Run. **Probe PR1:** F′ case 1 error → `{kind:"native",run_ref:1}` (its own selected Run), fully rehashed → `Ok` with and without invocation. **Probe PR5:** `run_ref:0` (case 0's Run) → `Ok`. Mutant M13 (delete the run_ref check at :1894) survives the suite. I63's RETURN difference 6 ("Rust also requires `error.run_ref == run.id` for native errors") holds only on the nonselected branch. | Give `native` its own arm: require a non-null, nonselected Run and `run_ref == run.id`, else G5 PRODUCT_ATTEMPT_MISMATCH. Pin with two shared mutations (native+selected; native with a foreign run_ref), expected G5 PRODUCT_ATTEMPT_MISMATCH in every reader. |
| RV80-S2 | SHOULD-FIX | `retained_precision.rs:648–661` | G3 accepts any **unique** old member order, and prepared/new as prefixes of it by id, instead of the native member order. F1:78–84 (inventory "in the producer's native member order"; "Arrays never skip an entered evaluation"), F1:101, F1:108 and F1:130 ("G3 checks the declared complete/prefix inventory and ordered helper/new overlap"); C3:124 and C3:302. Sourced attempts are safe (:671–680 binds old to the source's member map), so this affects unsourced (unavailable) attempts only. **Probe PR2:** P′ attempt 1 (complete old inventory, no source) with old member id 1 in a one-member model → G8 PREPARATION_MISMATCH with invocation, and `Ok` without invocation. Python rejects at G3 (`retained_precision.py:1424`). | Require old, prepared and new member ids to equal `0..len` in order at G3 (COVERAGE_MISMATCH), as Python does. Pin with a shared mutation (gapped or permuted old ids on P′). |
| RV80-N1 | NOTE (ruling) | `:798–814` vs the call loop `:815–1215` | Known difference 7. Inside the native G5 class, Rust checks group basics and build ids/work before the per-call replay; Python checks them after. With an ATTEMPT defect and a WORK defect in different places, the first codes differ. The contract fixes only the class order (C1:148; C3:293–304) and "ascending attempt/member/lane/row index", so neither reader is wrong. | ROOT rules one order and pins it with a dual-defect shared mutation. RV80 prefers Rust's order (the arrays the replay dereferences are validated first), but either is contract-consistent. |
| RV80-N2 | NOTE | `:662–670`, `:707–721`, `:609–615` | Three G3 divergences from Python that are **not** in I63's list. (a) `captured_prefix` additionally requires `source_ref`/`run_ref` null and an unavailable result at G3. F1:97 states these facts, but C3:304 places "C3 run/source … references" in G5; Python reaches them in G5. (b) A non-null coverage roster over an empty source inventory fails Rust's G3 (`!inventory.is_empty()`); Python's G3 (`:1435`) passes it; RV80 did not trace where Python fails it. (c) Run origin owner and unique run ids at G3, which is the same family as difference 1 and consistent with C1:146. | ROOT rules (a) (RV80 reads C3:304 as G5 PRODUCT_ATTEMPT_MISMATCH for the reference part) and confirms (b) (I57 §1 says a complete source has at least one body). Pin both. |
| RV80-N3 | NOTE | tests | 10 of RV80's 18 mutants survive the suite (see Mutation testing). The survivors are known deferred families (M02 N17 scope, M08 N10 boundary, M16 Budget terminal) or unpinned reader-only rules (M07 G3 run-id contiguity, M12 the record-bound rule on unavailable attempts, M13–M15 known differences 4–6, M18 G7 detail separation, M06 the relative-class boundary). Probe PR3 shows that M12's rule works on the unmutated reader (G5a SCALE_MISMATCH) but nothing pins it. | For the shared corpus (RV78): F′ case 1 record bound → null (G5a SCALE); the S1/S2 pins; pins for differences 1, 4 and 5; a G7 base error that carries detail; a relative-class equality row. |
| RV80-N4 | NOTE | `:1476–1487` | `#[doc(hidden)] pub mod reader_logic` exposes `schedule` and `ordinary` partial-gate entry points. They return `Result<(), _>` and grant no Validation or eligibility, but they widen the public crate surface (compare RV81's `accountingRules`). | Put them behind a test-only cargo feature before any public activation. |
| RV80-N5 | NOTE | `:1531–1545` | R3 checks each `work_accounting{fault}` against the **attempt-wide** join of unavailable-Count faults and `sticky_status`, not "its owning trace's status" as ROOT's 06d ruling words it. Python does the same, per I63. The owning trace is not generally identifiable from the public cause, so this is a necessary-condition narrowing. | ROOT records the narrowing, or specifies how an owning trace is identified. |
| RV80-N6 | NOTE | `:3811–3814` | Stale comment: it says the coverage checks are "against shared snapshot 04 only" and that eligibility waits for snapshot 05. The constant is correct; only the text is stale. | Update the comment when the hold rationale next changes. |
| RV80-N7 | NOTE | `:1429–1430` | The replay rejects two native shapes that keep an Accepted last attempt with a non-selected terminal: the pair-identity refusal (`FK/adaptive.rs:4819–4827`, which also leaves the record Verified) and `CertifiedBoundUnencodable` (`:4836–4843`). Neither is emittable: the first is a defensive invariant check, and the second needs an infinite bound that Bits cannot encode (C1:92). No false rejection of an emittable shape was found. | None; recorded for the audit trail. |
| RV80-N8 | NOTE | READER_AUDIT_PLAN Part 1 | The checklist has 43 rows (N1–N17, C1–C6, O1–O5, P1–P11, W1–W4), while the plan and the rulings say 42. | Clerical. |

## 1. Gate order and first-failure codes

- **Top level** (`validate`, `:3818–3865`): G0, G1, G2 (`encoding`), G3, G4, then G5 (native, ordinary, products), G5a (`numeric_cases` and `g5a`), G5b, G5c, G6, G7 (base validator on the projection) and G8 (only with an invocation). That matches C1 §6 and C3:293–295. Each gate runs over **all** cases before the next gate, so the order is gate-major across cases, as ruled. `numeric_cases` raises G5a for selected cases before `g5a` visits unavailable cases, but both raise the same G5a SCALE_MISMATCH.
- **G5's internal classes:** native (`g5_native`: ATTEMPT and WORK), then ordinary (`g5_ordinary`, ATTEMPT), then C3 association and typed checks (`g5_products`, PRODUCT_ATTEMPT, with the P9 pass at `:1919–1936` after every attempt), then the deferred C3 WORK list (`:1937–1939`, R1–R3 included). This matches C3:304 and Python's ruled order. P2 and P3 checks are interleaved per attempt, but they share PRODUCT_ATTEMPT_MISMATCH, so this cannot change a first-failure code. The one unruled order is inside the native class (RV80-N1).
- **G7:** `base_error` (`:1266–1277`) reports the leading `[A-Z][A-Z0-9_]*` token as the bare code and keeps any remaining text as `detail`. That follows the 06b settlement (each language keeps its own base code). The Rust base errors start with an uppercase code (`preview_physics_evidence.rs:90,291`). The only shared G7 entry gives a bare `SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS`, so detail separation is unpinned (M18).
- **Schema at G1:** `shape` deliberately ignores `pattern`, `minimum` and `maximum`. In this schema those keywords occur only on the U, I32, Bits, NonnegativeBits and Hash encodings, and `encoding()` enforces them at G2 through `x-rp-encoding`, as C1's G1/G2 split requires. All 287 object schemas are closed, and `combinations` is `maxItems:0`.

## 2. G5 checklist: I63's status table

RV80 checked every ID against the code (line references are to `retained_precision.rs`).

- **Confirmed:**
  - native schedule: N1–N4 (`:837–843`, `:1339–1410`); N5 (`:1431–1434` with `terminal_of` `:1447–1472`, which equals `FK/adaptive.rs:4349–4378`); N6 and N7; N8 (`:1440–1441`); N9 (`:1307–1309`, `:1435–1438`); N10 (`:1148–1154`, `:1319–1324`); N11 (`:1250–1258`, no control); N12 (G1 schema); N13 and N14; N15 (`:1180–1194`); N16 (`:904–918`); N17 (`:1199–1211`);
  - cache, build, call and group: C1–C6;
  - ordinary: O1–O5;
  - typed errors and stages: P1–P7, P9–P11;
  - work: W1–W3; W4 is attested.
- **Native readings checked:**
  - **The replay against the native ladder** (`FK/adaptive.rs:4519–4800`). A failed verification-pass is terminal, and natively it can only carry a non-escalating stop: `terminal()` is unreachable for escalating stops, `:4377`. So Rust's classification of failed verifications by escalation is faithful.
  - **The N10/N11 idle shapes against the native pre-schedule returns** (`:4994–5075`).
  - **The N17 rule against `StageGuard::test`** (`:276–286`). It applies case before invocation on the same `used`, so an invocation-scope stop implies the run's case charge did not exceed Lc.
- **Disputed:** **P8 is only partial.** The reason/phase table admits a native error with a selected Run, and does not check run_ref there (RV80-S1). I63's table lists P8 as "checked".
- **Coverage of the checks:** N10, N17, the Budget terminal translation and the record-bound rule on unavailable attempts are implemented but unpinned (M02, M08, M16, M12), as I63 states for the deferred families.

## 3. Arithmetic (independent exact-rational oracle)

`ORACLE_GEN.py` builds 8,238 vectors with `fractions.Fraction` and explicit directed rounding, deliberately concentrated on subnormal, boundary and overflow operands. No reader function is used to compute the expected results.

| Function | Oracle | Vectors | Mismatches |
|---|---|---:|---:|
| `upward_product(a,b)` | RU(a·b); error above the largest finite, or on a negative or non-finite operand | 1,828 | 0 |
| `upward_small_sum(b0,r)` | RU(b0 + r + 2^-1074) | 1,824 | 0 |
| `absolute_bound(n,S)` | C1:158 and native `row_bound` (`FK/adaptive.rs:424–452`): 0 at S=0; RU(RU(2^-64 S) + RU(2^-53 \|n\|) + 2^-1074) for 0<S<2^-988; otherwise RU(2^-64 S) | 1,576 | 0 |
| `phi_512(e)` | RU(2^-438 e) | 1,510 | 0 |
| `e_hat([fo,mo],L)` | E at L=0; otherwise [max(fo, RN(mo/L)), max(mo, RN(L·fo))] | 1,500 | 0 |

`ORACLE_HARNESS.rs` is RV80's own test, added to RV80's copy only. It printed "mismatches 0" and passed.

**Read against native code:**
- `e_hat` and `phi_512` mirror `FK/verify.rs:323–334, 367–376` and `next_up`.
- `body_extent` (`:2128–2152`) reproduces `FK/adaptive.rs:321–336`, including the `(dx²+dy²)+dz²` order.
- The relative-class predicate (`:2830`) equals `relative_class` (`FK/adaptive.rs:467–475`).
- Feasibility (`:2423–2436`) and the estimate/charge rederivation (`:2438–2443`) equal `summary_coverage_data` (`FK/final_case.rs:1461–1532`) with D=false. D=false holds because `canonical_layout` (`:2226–2320`) marks only constrained displacement rows input-derived, and requires every constraint to be +0.

**Other native mirrors read:**
- The G5a resolution zero, sanity and lower tests (`:2620–2697`) follow the producer's own G5a (`PP/retained_product.rs:2597–2752`): the same extent coupling, the 1+2^-40 upper factor, the 2^-59/2^-60 thresholds and the operation order.
- `verify_native_source_hashes` (`:3580–3678`) re-encodes K4SRC and K4STF byte for byte as `FK/source.rs:776–888` does (little-endian u32 counts, f64 bits, an empty directional-spring list).
- The G8 interpolation `l + f·(h − l)` matches `PP/lib.rs:9329–9334`.

**Checked counters:**
- `sum` (`:270–276`) is checked u64 addition, capped at 2^53−1 and failing as WORK.
- `uint` (`:230–242`) refuses fractions, −0 and anything above 2^53−1.
- Fragment differences use `checked_sub` (`:1111–1121`).

## 4. Coverage (I57)

- **G3** (`:700–721`): a non-null roster has exactly one entry per source body, 0..n−1, and is non-empty (parity rule 2).
- **G5** (`g5_coverage`, `:1573–1613`): Ready, a completed certificate or a passed G5a requires non-null coverage. Non-null coverage requires:
  - the same source and Run;
  - a selected Run;
  - two completed lanes in order;
  - completed proof_start, projection, maxima, values and aliases;
  - an entered certificate.
  
  This is I57 §3. M11 is killed.
- **G5a** (`coverage_g5a`, `:2337–2523`):
  - native p and the 2p record;
  - the canonical layout rebuilt from the source maps, with equality required (parity rule 1);
  - the 16-vector feasibility rule, with the floor ORed after coupling (M03 killed);
  - estimate rederivation, and charge = stop at p512 (M04 killed);
  - exact stop, estimate and charge rosters, and the B roster (item 4);
  - the record bound: one entry per body, non-null iff has_data (parity rule 3; PR3);
  - theta = +0 on no-data bodies;
  - data_blocks;
  - the two direct data facts (M09 killed).
- **Unavailable attempts with complete coverage** (`g5a`, `:2527–2556`): Φ = `phi_512(e_hat(E,L))` comes from the Run's verification record at p512, and no Selection roster is applied. This is I57 §4 "Unavailable attempts".
- **Floor equality** at G5b (`:2715–2721`), with exact bits (M17 killed).

## 5. Fail-closed behaviour and public API

- **Eligibility is held.** `numerical_eligible` (`:3853–3858`) is `IMPLEMENTATION_COMPLETE && invocation present && MECHANICS_SOLVED && every case selected or not_required`, and the constant is `false` (`:3815`). Setting it true is caught: M01 is killed by three tests, and every one of the 15 shared cases would otherwise be eligible.
- **No early success.** `validate` has no return path before G7, and G8 runs whenever eligibility could depend on it.
- **Transport metadata is never eligible.** `validate_transport_metadata` (`:3868–3886`) runs G0–G2 plus the base metadata checks, as C1:162 requires. It labels base metadata failures as gate "G2".
- **Public surface:**
  - `validate` and `validate_transport_metadata`;
  - the five arithmetic helpers;
  - the constants and result types;
  - the hidden `reader_logic` module (RV80-N4).
- **Nothing else in the crate dispatches to this module.** `lib.rs` only declares it. No other `semantic_contract` path recognises the retained contract id.

## 6. Mutation testing

`MUTANTS.py` applies each single-edit mutant to a pristine copy of `retained_precision.rs` in `WT/rv80`, runs the candidate's own command, then restores the file and checks its sha256 (`bd20dd9a…`). Per-run logs are in `WT/scratch/rv80_reader_review/mutants/`.

| ID | Edit | Result |
|---|---|---|
| M01 | `IMPLEMENTATION_COMPLETE = true` | **killed** (3 tests) |
| M02 | N17: drop `!case_over` from the invocation-scope rule | survived (deferred base) |
| M03 | Feasibility: positive force floor not ORed | **killed** |
| M04 | p512 charge = estimate | **killed** |
| M05 | R3 containment dropped | **killed** |
| M06 | Relative class `>=` → `>` | survived (no equality row) |
| M07 | G3 run-id contiguity dropped | survived (difference 1 unpinned) |
| M08 | N10 idle exhaustion `>=` → `>` | survived (deferred base) |
| M09 | Data fact: nonzero free nodal term → has_data dropped | **killed** |
| M10 | Small bound: the +2^-1074 term dropped | **killed** |
| M11 | Ready/certificate/G5a passed with null coverage allowed | **killed** |
| M12 | Record bound non-null iff has_data dropped | survived (unpinned; PR3) |
| M13 | P8 native run_ref check deleted | survived (RV80-S1) |
| M14 | C3:165 ordinary material basis check deleted | survived (difference 5 unpinned) |
| M15 | C3:147 preparation back-reference deleted | survived (difference 4 unpinned) |
| M16 | `terminal()` Budget payload dropped | survived (no Budget-terminal base) |
| M17 | G5b p512 floor equality removed | **killed** |
| M18 | G7 detail folded into the code | survived (no G7 entry with detail) |

**Totals:** 8 killed, 10 survived. Every survivor maps to a disclosed deferral or to a pin recommended in RV80-N3.

## Named questions: I63's eight differences from Python

Probes PR1–PR7 (`PROBES_HARNESS.rs`, outcomes in `PROBES.json`) edit a shared base and rehash every affected digest exactly as the candidate's harness does.

| # | Rust behaviour | Contract | Reader RV80 believes right |
|---|---|---|---|
| 1 | Run-id contiguity and `execution_order` at **G3 COVERAGE** (`:609–626`); call `run_refs` concatenation at G5 (`:1216`) | C1:146 (G3: "execution-order bijection"); C2:117 ("each nested run.id equals its execution-order position"); C2:131 | **Rust.** Python's G5 placement contradicts C1's G3 row. Pin it (M07 survives). |
| 2 | G3 accepts unique old ids, with prepared/new as prefixes by id | F1:78–84, 101, 108, 130; C3:124, C3:302 | **Python** (0..len at G3). RV80-S2; PR2. |
| 3 | Sourced attempt: old ids = the source's member map at **G3** (`:671–680`) | F1:101 ("old ids exactly equal the full member inventory"), F1:130 (G3); F1 table: a source exists only in the Complete-M row (F1:95) | **Rust:** exact equality at G3. Python's length-only check at G5 is weaker and at the wrong gate. |
| 4 | `source.preparation.attempt_ref == attempt` for every sourced attempt (`:1654–1656`) | C3:146–148 (every constructed source); S06:33 | **Rust.** PR6 → G5 PRODUCT_ATTEMPT. Python checks only Ready attempts. Pin it (M15). |
| 5 | `attempt.material_basis_ref == ordinary.material_basis_ref` (`:1637–1639`) | C3:165–166 ("agrees with owner, ordinary attempt and material basis"); checklist P1 | **Rust.** PR7 → G5 PRODUCT_ATTEMPT. Without it the defect surfaces only at G8. Pin it (M14). |
| 6 | P8 edges: (a) native error with a selected Run takes the facade branch; (b) native run_ref = Run id on the nonselected branch; (c) preparation error needs Run null and preparation failed | S06:33, 37, 38 | Split: **Python** on (a), where S06:38 makes it invalid (RV80-S1; PR1, PR5). **Rust** on (b) and (c), from S06:37–38, which Python lacks. |
| 7 | Inside the native class, group/build structure is checked before the per-call replay | C1:148 and C3:293–304 give only the class order | **Neither**; the contract is silent. ROOT ruling plus a dual-defect pin (RV80-N1). |
| 8 | Group `call` must index an existing call; group sources unique and listed in that call (`:798–811`) | C2:135 (call-local groups, `call` reference), C2:143; checklist C5 | **Rust.** These are necessary reference-resolution facts. Python's C5 partition covers the in-range case only, per I63; RV79 should confirm whether a dangling `call` reaches a failure there. Pin it. |

## Basis read (sha256)

| sha256 | File |
|---|---|
| 48af25ae74b0352bf38c7bda52725b1c3b1233ee20c583358402462b6b0d600d | R/BRIEFS/RV78_RV81_READER_REVIEW.md |
| cc2541604ff9df35221264dd77030156693c649913b154e26a1884396a0b8e44 | T3/ROOT_RULINGS_V1.md (from "Resumption by the next ROOT; coverage implementation planned" to the end) |
| c8ab2318457bd3897e207888af80f54f1a889071bf630ff1815939d922a567e3 | R/I32/f2a_wire_c1/WIRE_CONTRACT.md (C1) |
| 923da0b97eb5becad7e7c1373c5a6f362568dc28ac8eab029c9170677c890869 | R/I32/f2a_wire_c2/CONTRACT_DELTA.md (C2 §4–5) |
| fd00d2c1e8f4f7e2077f304560a63830e2b7b61cb5b1d47aff994c7874ed292e | R/I52/prepared_public_contract_02/C3_DELTA.md (C3) |
| 6b8ebea5b83c3033211df144f69add8fcb753855cae570ec66afdd6684ddbf36 | R/I52/prepared_public_contract_correction_03/ADDENDUM.md (F1) |
| 031b2a150545df4d80da8d8078b956e98760e0df757a75f98bd39b914dec42ab | R/I52/reader_contract_seams_06/ADDENDUM.md (S06 §1) |
| 845d5258bf6ec61734242e8be958bbf544a13af3c6d6d8871d7f536f0b44ad59 | R/I57/summary_coverage_01/ADDENDUM.md |
| ee3cc5918cec2a789116702fd4bdbce78dd0a243dde388b21b2b6828954a7790 | R/I62/coverage_shared_python_01/READER_AUDIT_PLAN.md (Part 1) |
| 9ece528f917dc9741fb56a89b24b35fd3027828fe35c1321c29b8b074fa8212b | R/I63/reader_align_06d/RETURN.md |
| 70694a3c2065315fe4ff419359a2f92593e9da8fb4ec3bf5f5dd39eaf20ed6da | R/I63/reader_align_06c/RETURN.md (status rows) |
| 2238ac8438c608dd1b58a38e0ed2945ab31b4540f8150c930f6f39f2b235f9a7 | R/I63/reader_audit_06a/RETURN.md (status table) |
| 6a2fc382bf8cae0502c41030da8ac9bc0cfe1f1b80b7aceac60ff7e40f345eda | NUM P/core/solver/frame_kernel/src/structural/retained/adaptive.rs |
| 66022cc78bb5779d021a35e33fafc7cc7bc2d64723a5d0fdaf3a48cb6fea795e | NUM …/retained/verify.rs |
| 3986919726e962b52eaa730eb4cc76311fb79ff6dee6e075aa75b4a4d639def7 | NUM …/retained/product_certificate/final_case.rs |
| 55736ea65aee641fb22288d869632c5ea30f8c016ce56307a974ea598826192f | READER P/core/analysis_runs/retained_precision.py (G3 only, read only) |
| d77574b804d4cfc94f8dbcac6ea91f2408e4123a5244fd80a75df2a42b9302b5 | R/I59/rust_reader_01/RETURN.md (I59's account; its remaining obligation 5 is this review) |
| 9956c08421eebb44ee2663a790cf3ac882452f5119f6470cf6c5dcc386c3f2b8 | NUM …/retained/source.rs (:776–888, the K4SRC/K4STF encoders, read for G8) |
| d07383fc026e61e494a2b0a307271eaf2eb0329c333da95533f4b39c5d52af1a | NUM P/core/product_physics/src/retained_product.rs (:2535–2752, the producer G5a summary checks) |
| 4fff1a331c754f666a0ddff2438418e02c90f19ec21d1a234cb65ed55985f594 | NUM P/core/product_physics/src/lib.rs (:9208–9345, the preview E/G interpolation) |

## Commands and evidence

| Run | Command (from WT/rv80, with the environment above) | Result |
|---|---|---|
| Baseline | `cargo test --locked --offline --manifest-path WT/rv80/P/core/reporting/result_export/Cargo.toml --test retained_precision_contract -- --test-threads=2` | 23 passed, 0 failed (`BASELINE_TESTS.txt`) |
| Oracle | same, with `--test rv80_oracle` and `RV80_VECTORS=WT/scratch/rv80_reader_review/oracle_vectors.json` | 1 passed, 0 mismatches over 8,238 vectors (`ORACLE_RESULT.txt`) |
| Mutants | `python3 MUTANTS.py WT` (the baseline command, once per mutant) | 8 killed, 10 survived (`MUTANTS.json`); the source was restored to `bd20dd9a…` after each mutant |
| Probes | same, with `--test rv80_probes` | PR1–PR7 (`PROBES.json`) |

`ORACLE_HARNESS.rs` and `PROBES_HARNESS.rs` were added only to RV80's archive copy, as `tests/rv80_oracle.rs` and `tests/rv80_probes.rs`. They are reviewer instruments, not candidate changes. Bulk logs are in `WT/scratch/rv80_reader_review/`; WT/rv80 was deleted at the end and WT/targets/rv80 kept.

## Open for ROOT

1. **Rule on the eight differences** (table above). RV80 reads 1, 3, 4, 5 and 8 for Rust; 2 and 6(a) for Python (the SHOULD-FIX items); and 7 as unruled contract.
2. **Rule on the unlisted G3 divergences** in RV80-N2.
3. **Record R3's attempt-wide narrowing** (RV80-N5).
4. **Route the shared pins in RV80-N3 to the corpus owner,** so that every rule above gets a shared mutation before acceptance.
