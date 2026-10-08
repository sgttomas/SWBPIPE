# I97 B2-C: the B2 contract (combinations, mixed and preparation-only operands)

TASK (Type 2), I97, a designer for ROOT (HELP_HUMAN, Agent 0), who is the return path. I am a fresh instance and made no delegation. 2026-10-07 UTC.

**The brief:** `R/BRIEFS/B2C_CONTRACT.md`, sha256 `a604a2c3611ac3212bb0e5ce72d3e9ee66dcf97c3dc00226ff2f42df0ac24c6a`, verified before reading. I read NUM's `AGENTS.md` and `agents/AGENT_TASK.md` first.

**What I did.** Documents and code reading, plus read-only Python with VENV on committed bytes: a statics generator and a checks script of my own (`_run_records/`), each run twice with byte-identical outputs, and one read-only `git grep` collision script. No cargo, no native, solver or test job, no install, no Git write. Git reads used `GIT_OPTIONAL_LOCKS=0`. Scratch and `TMPDIR` were `WT/scratch/i97_b2_c/` only; nothing went to the system temp directory. **This is contract text for RV-C's review and ROOT's selection. It installs nothing and reserves no name.**

**Notation.** WT, NUM, P, T, R, RR and VENV as in the dispatch. PP, FK, RS, PY, TS, C1–C3, SC1, DN, D2, PTABLE, DEF-O, SCHEMA and CORPUS as `R/I93/b2b3_plan_01/PLAN.md` defines them (FK = `P/core/solver/frame_kernel/src/structural/retained`). Also:
- **PLAN** = that PLAN.md; **REV** = its `REVISION_01.md`; **DESIGN** = `R/I78/b0_contract_01/DESIGN_v2.md` (B0); **KD** = `R/I94/b2_kd_01/DESIGN.md` (B2-KD, as accepted with RV115's amendments); **B3D** = `R/I96/b3_d_01/DESIGN.md` with its `REVISION_01.md`; **XTABLE** = B3D's `statics/r1/semantic_contract_v0_3_physics_retained_1.json`.
- **DEF-C** = the combination formation definition this contract drafts, `RP-PREPARED-COMBINATION-DUAL-v1` (suggested name), at `P/fixtures/results/retained_precision_prepared_combination_v1.json`.
- **c** = load cases, **z** = combinations of any basis, **C_eq** = c + z, **A** = the attempted cases (T-4), **h** = a mechanics combination's terms (repeats counted).
- "Selected" means a case's public status `selected` after T-9 (B1's `AttemptEnd::Frozen`); "kernel-selected" means only that its Run ended `selected`.
- "Retained combination" means a combination whose disposition is `retained_selected` or `retained_unavailable`.

**Basis.**
- **Code:** NUM `cebff253d6` (read throughout; its maintained tree outside `P/execution` equals the dispatch commit `6aa2878872`'s, checked by `git diff --quiet`; only records moved). B1, read-only: `b1` `603e238517`, `b1-r` `b5cb7faaeb`, `b1-p` `11cc14e3e6`, `b1-t` `7e47e51b5d`. Code is cited by symbol.
- **Records** (sha256 in §15): PLAN, REV, RV114's review, DESIGN whole, C1 §1–§6, C2 §1–§7, C3 §1–§6, SC1 whole, KD whole, RV115's review, B3D with its revision and statics, RV116's review and addendum, RV109's round-2 review, DN §4.2, D2 §4.9.4, I74 PLAN §4.3.
- **RR**, by heading: "B0 selected on DESIGN_v2; C3a's names reserved; …", "B2/B3 R1: …", "I93's REVISION_01 accepted; …", "RV115 (RV-K) accepts B2-KD with amendments; R-1 to R-11 ruled", "RV116 (RV-D) accepts B3-D with amendments; …", "RV116 confirms B3-D's revision 01; B3-D is final for J1", "NUM absorbs main with #1109's RV58 fixture repair; …", "RV109 passes SP in RV-P round 2; SF-1 and SF-2 go to I3's pinning step; B2-C dispatched as I97", and "RV117 passes #1111; …" (its screen rule for host names).

## 0. Findings in brief

1. **Where B2 sits.** B2's work is two new steps, **T-10a** (dispositions) and **T-10b** (operand sources, operand preparations, then per combination its Call, Run and freeze), between B1's T-10 check and T-11's staging (§2.1). T-1 to T-13 keep their meaning, and every z = 0 byte is unchanged.
2. **Four B1 code sites assume no combination, so B2 must extend them for z ≥ 1 only** (§2.2). These are B1's code, not its contract; at z = 0 each extension is the identity.
   - **T-6:** at c ≥ 2, `bind_case_rows` refuses any envelope whose rows end with combination rows ("case row block"), so every combination-bearing invocation would fall back at custody. At c = 1 the case's scope is `CaseScope::WHOLE`, which would put combination rows into the selected case's row binding.
   - **T-8:** `native_call` declares `OriginCapacity::for_calls(&[n], &[])`, which reserves no combination Call, operand or prepared registration.
   - **T-11, headlines:** `stage_headlines` ranges over every staged row of a headline kind. A combination's `displacement_magnitude` row could then become the headline, which the base readers' G7 refuses (RE `preview_physics_evidence.rs` `headline`: "Combinations are outside headline scope"), so the successor would be abandoned at precommit.
   - **T-11, serializer:** `serialize_cases_with` refuses combinations and more than one Call, and `invocation_arrays` writes every Call as `case_batch`.
3. **A mechanics combination that names a load case twice publishes no rows** on the ordinary route. Validation admits it (`validate_mechanics_combination` has no duplicate check), but `evaluate_values` returns nothing because the load-case algebra blocks a duplicate operand (`evaluate_linear_combination`, `DuplicateOperand`). Retaining it would buy a kernel run and then a certain facade failure. **Recommended (decision C-1): such a combination is `ordinary`**, one conjunct added to decision 5's trigger. With C_eq ≤ 3 and z ≥ 1, c ≤ 2, so a retained combination has h ≤ 2.
4. **Combination ids and load-case ids are separate id spaces** in the model (`validation.rs` checks duplicates per entity). But the producer and the three readers match rows and diagnostics by the bare id (`selected_case_envelope`, D6a, every reader's G4). **Recommended (C-9): D1.4 requires them disjoint;** an overlap falls back to the ordinary route.
5. **Unavailability codes for a combination (C-2).** `combination_unresolved` for every failure without a selected Run, in phase `preparation` (no Run) or `kernel` (a Run not selected), as DN §4.2 and PLAN's W-CB2 expect. `facade_certificate` after a selected Run, as for cases. A third no-Call cause is needed beside C3a's: **`operand_source_unavailable`**, for an operand case that has no usable source (C-4).
6. **The combination's product attempt is its own closed `$def`, `CombinationAttempt`** (C-3). It exists exactly when the combination has a Run. It carries no ordinary attempt, preparation or operational member, because a combination has none. So `product_attempts[]` admits `ProductAttempt` (cases) or `CombinationAttempt`. ProductAttempt's `definition_id` keeps B3b's change only, and B3b's `SCHEMA_ENUM.diff` composes unchanged (§6.2). REV §1.1 item 3's letter ("ProductAttempt.definition_id holds the combination id") is met in substance by the union; ROOT confirms.
7. **`operand_preparations` must be absent when empty (C-6).** C3a says "empty if none". A required empty member would change every c = 1 and B1 receipt byte, which PLAN risk 5 makes a stop.
8. **R-COMB-1 is best reader-derived (C-7).** The readers already hold every input. Their validation classes those rows `not_covered`, and D-U6-2's disclosure and T6S's withheld-witness path then follow with no output-code change (§5).
9. **J1's package needs three edits beyond REV §1.4's constants** (§11), or J1's own acceptance fails:
   - RS's and TS's G0 compare PTABLE's `product_formation_definitions` with a one-entry constant (RS `g0`; TS `header`). After PTABLE gains DEF-C, every successor fails precommit at G0 `FORMATION_MISMATCH`, so the c = 1 and B1 pins break.
   - PP `retained_wire_tests.rs` `u1_constants_bound_to_in_tree_fixtures` asserts the same one-entry list.
10. **R-7 is confirmed on committed bytes** (`_run_records/b2c_checks.out.json` §r7). The preview-physics-1 fixture `preview_physics_unicode_ids_sparse.json` has n = 3, m = 2 and g = 1, and its mechanics combination has **129 = 7n + 50m + 8g** rows: 20 stress rows per member over the five sites `end_i`, `end_j`, `quarter_1`, `midspan` and `quarter_3`, and 30 action rows per member. It has no maximum, intensified, mode, parity or modulus record, and its rows follow the case rows contiguously.
11. **The draft statics** (generator run twice, byte-identical; §6–§7):

    | Static | sha256 |
    |---|---|
    | DEF-C, raw (9,944 B) | `03d40598be82a5df8867bb9a279b212b3acd53ba5f51ff85b9635e6cefaa69cc` |
    | DEF-C, H(`retained_precision_formation_v1`, DEF-C) | `9adf5178c7c1d5b81de340c2f0f605396e717a9264b59430df8d2a1306f21731` |
    | PTABLE revised (53,462 B) | `863738f1aa81c7bdbf87c716b68e690c6104d7d25416df8ed4c28caece44d0e1` |
    | SCHEMA with B2's change only | `841b2c5d77a81c9165b789eb7946a7c0ff347c3580338c848d97db403189b2da` |
    | `SCHEMA_B2.diff` | `ff3b88956ff399a50116b637fe2a9bf046271a54f2b842000ea85701672600d3` |
    | **The merged J1 SCHEMA text** (211,243 B) | `abf3225ca431342dd785072a1baad7715b7e8c19afd15b5777feebf06d48669e` |
    | `SCHEMA_J1.diff` (against main) | `f674370d0c62f2fa8993f90b0b1828697b22598ed7214d64653c3ca94e125fd1` |

    - **Controls:** DEF-O's H reproduces (`a7ed7ca0…`). PTABLE rebuilds from preview-physics-1. B3b's patched SCHEMA reproduces I96's `0d5bb812…`.
    - **The checks:** the J1 text is a valid 2020-12 schema in main's keyword vocabulary. All 17 CORPUS bases, 28 must-pass entries, 294 mutations and 6 committed successor documents keep their G1 shape verdict under main's SCHEMA and under the J1 text, by `jsonschema` and by PY's own G1 walker.
12. **Estimates** (§14): B2-A 5–8 h, B2-P 26–38 h, B2's readers 39–55 h, and B2's part of SC2 7–10 h. That is about +12–19 h on PLAN's corresponding rows.

## 1. C3a's completions (brief item 1; PLAN §1.2.2)

DESIGN §4.2's P1 text stands as selected, with ROOT's reserved spellings. B2-C adds only what P1 left open.

**C3a-1. Placement.** Operand preparations run at **T-10b(ii)**, after T-10a's dispositions and T-10b(i)'s operand-source check, and before the first `MechanicsCombinationCall`.
- **Which:** one for each `not_required` case that is a term of a retained combination not already decided at T-10b(i) (`operand_source_unavailable`, §2.4).
- **How many:** at most one per owner case, shared by every combination that needs it.
- **Order:** the order of first need: combinations in authored order, and within each, terms in authored order. That is the records' `id` order in `operand_preparations[]` (actual start order).
- **`requested_by`:** the ascending indices of exactly those combinations whose terms name the owner case.

**C3a-2. What a preparation does.**
- **C3's preparation stage on the owner case,** from the case-source inputs that T-2's capture already holds per case. B2-P reuses `prepare_attempt`'s per-case machinery without a `ProductAttempt` and without any native stage.
- **On success:**
  - PP builds the kernel prep, `PreparedCaseSource::new(source)` (KD §1.2), from the prepared `PrimitiveSource`;
  - it registers it with `RecordedInvocation::register_prepared_source`. That is one `SourceOrigin` with owner `NativeOwner::Case(next native ordinal)`, no Call, no Run and no meter change (KD I6);
  - PP maps that native ordinal to the owner's request index;
  - the receipt gains that case's `CaseSource`, with `preparation: {operand_preparation_ref, sha256}`, and the record's `source_ref` names it.
- **If `PreparedCaseSource::new` refuses** a source that C3's preparation completed, or the registration refuses (`OriginError`), that is not a record state. It is a whole-successor abandonment (§2.6).

**C3a-3. Record point.** The record's **terminal stage**: the snapshot of the one shared capture is taken when its one preparation stage completes or fails, as T-11's attempt snapshots (DESIGN T-11 S-3). It is not ordered against any other snapshot.

**C3a-4. Work.** C3 preparation work exactly as it ran: `preparation.members[].work`, the conversions, `operational` and `adapter`. It is outside LME and outside every budget (P1 rule 2). Neither the preparation nor its registration touches the meter.

**C3a-5. The preparation hash.**
- The CaseSource's `preparation.sha256` is H(`retained_precision_operand_preparation_v1`, payload).
- **The payload is** `{definition_id, definition_sha256, owner_ref, ordinary_attempt_ref, material_basis_ref, purpose, members}`. `definition_sha256` is DEF-O's table-bound H, the route's definition (RR "RV116 (RV-D) accepts B3-D…", ruling 1, S-1), and `members` is C3 §2's member projection.
- No `source_ref`, work or hash field enters it, so there is no cycle.

**C3a-6. Refusal.**
- `result: {kind: "refused", error}` with `error.kind == "preparation"` (PublicError's preparation branch), `stage: "failed"`, `source_ref: null`, and no CaseSource.
- Each combination in `requested_by` becomes `retained_unavailable` with `operand_preparation_failure` (§2.4 T-10b(iii)), unless an earlier term already decided it. The owner case gets nothing (P1 rule 4).

**C3a-7. Gates and codes** (each new check runs after the gate's existing checks, so 07n's first failures cannot move):

| Gate | Check | Code |
|---|---|---|
| G0 | `definition_id` is in PTABLE's `product_formation_definitions` and equals DEF-O's id (§8) | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| G1 | Closed shape; `preparation.sha256` recomputed under `retained_precision_operand_preparation_v1` | `RETAINED_PRECISION_RECEIPT_MISMATCH` |
| G2 | U, Bits, Count and enum encodings | `RETAINED_PRECISION_ENCODING_MISMATCH` |
| G3 | The member is absent or non-empty; `id` = position; owner `not_required`; at most one per owner; `requested_by` ascending and unique, each naming a retained combination whose terms include the owner and whose cause is not `operand_source_unavailable`; conversely, every `not_required` term case of such a combination has exactly one record listing it; `source_ref` non-null resolves to a CaseSource owned by the owner whose `preparation.operand_preparation_ref` is this id, and every such CaseSource is named by exactly one record | `RETAINED_PRECISION_COVERAGE_MISMATCH` |
| G5 | `stage == "completed"` ⇔ `result.kind == "prepared"` ⇔ `source_ref` non-null; a refused record has `error.kind == "preparation"`; C3's preparation-stage member-prefix rules (complete members, or a successful prefix then at most one refused member) | `RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH` |
| G5 | C3's preparation-work status, conversion-prefix and count equations | `RETAINED_PRECISION_WORK_MISMATCH` |
| G8 | C3's old and new tuple equalities and the preparation binding, against the invocation (D, t, E, G for the owner case), then R-8's operand equality (§2.5) | `RETAINED_PRECISION_PREPARATION_MISMATCH` |

C3a's four corpus mutations are §10.3's m27, m28, m45 and m50.

## 2. The combination transaction: T-10a and T-10b (brief item 2)

### 2.1 Where each step sits in B1's confirmed T-1 to T-13

| Step | B1, as RV109 confirmed it at `603e238517` | B2 |
|---|---|---|
| T-1 | G-A admission | D1.4 widened (§9). A refusal keeps the ordinary route |
| T-2 | One ordinary run; one capture | Unchanged. The ordinary run already appends each combination's rows after the case rows, and its gate evidence (`contract_evidence.combination_gates`) |
| T-3 | Invocation gates (a)–(e) | Unchanged. (c) never fires with a combination (PP `source_eligible`) |
| T-4 | A, by the published verdict | Unchanged. Combinations are never in A and never get a notice. The domain re-check (`w1_case_ids`) admits combinations within D1.4 and keeps refusing outside it (`Domain`) |
| T-5 | Notices reserved per case in A | Unchanged |
| T-6 | Custody once | **T-6′** for z ≥ 1: the row blocks (§2.2) |
| T-7 | One product attempt per case in A | Unchanged |
| T-8 | One `CaseBatchCall` | **T-8′:** the capacity declaration (§2.2). The call itself is unchanged |
| T-9 | One freeze per selected Run | **T-9′:** at c = 1 with z ≥ 1, the case's scope is its own block |
| T-10 | No selected case: T-12 | Unchanged. A retained combination needs a selected case, so T-10 still decides |
| **T-10a** (new) | — | Each model combination's disposition (§2.3) |
| **T-10b** (new) | — | (i) operand sources; (ii) operand preparations and registrations; (iii) per retained combination in authored order: its Call, its source, Group and Run, then its freeze (§2.4) |
| T-11 | Staging, serializer, precommit, transfer | **T-11′:** combination overlays and diagnostics after the cases'; headlines over load-case rows only; the receipt's combination members (§2.7) |
| T-12 | The fallback publication | Unchanged, with one more abandonment cause (§2.6) |
| T-13 | One ordinary run through G-C | Unchanged |

B1's `selected` bits (T-12's detail placement) are computed after T-10, before T-10a, as today. Combinations never set them.

### 2.2 B1 code that B2 extends, for z ≥ 1 only

| Site on `b1` | Today | B2's extension | At z = 0 |
|---|---|---|---|
| **T-6′:** `prepared_custody` → `bind_case_rows` (c ≥ 2); `case_scope` (c = 1 gives `CaseScope::WHOLE`) | Every row must belong to a case block; the last block runs to the envelope's end | The case blocks end at the first row whose basis `ref_type` is `combination`. After them, the rows form one contiguous block per combination, in authored order, each naming a model combination id; a combination with no rows has an empty block. No case row follows a combination row. Each case's scope is its own block, at c = 1 too. A violation is a custody failure (T-6: `Preparation`, then notices). The ordinary producer always appends this way (`append_combination_results` after the case loop) | Identical: no combination row exists, so each block and scope is today's |
| **T-8′:** `native_call` | `OriginCapacity::for_calls(&[n], &[])` | `OriginCapacity::for_invocation(&[n], &operands, p)` (KD §1.2). `operands` holds h for each mechanics combination, in authored order, that the gates do not withhold, whose terms name distinct cases, and that has a term in the batch. `p` is the number of distinct `not_required` cases among those combinations' terms. These are declared maxima, not counts: a combination later found `ordinary` or decided before its Call leaves its reservation unused. SF-1's run-capacity check then holds, because the batch comes first | `for_invocation(&[n], &[], 0)` ≡ `for_calls(&[n], &[])` byte for byte (KD I2, K-02) |
| **T-9′:** `freeze` → `with_case` → `freeze_case` | `CaseScope::WHOLE` at c = 1 | As T-6′: the case's own block | Identical |
| **T-11′, headlines:** `staged_envelope` → `stage_headlines` (c ≥ 2) | Every staged row of kind `displacement_magnitude` or `pipe_elastic_normal_stress_maximum_v2` | Load-case rows only (basis `ref_type` `load_case`), as the ordinary `maximum_across_cases` and the base readers' `headline` do | Identical: every row is a case row |
| **T-11′, serializer:** `serialize_cases_with`; `invocation_arrays` | Refuses `combinations != 0`, more than one Call, and runs or sources beyond the batch; writes `kind: "case_batch"` and `result: {kind: "runs"}` for every Call; checks every Call's `invocation_after` against `charged` | As §2.7: one entry per model combination; the Calls' own kinds and results; the meter chain (C2 §4) | Identical bytes: `combinations: []`, one Call, no new member |
| **T-4:** `w1_case_ids` | `combinations != 0` gives `Domain` | D1.4's re-check (§9): c, z, C_eq, h, range operands, id disjointness; components 0 | Identical |

### 2.3 T-10a: dispositions (decisions 5 and 20; C1 §5; SC1 §2)

For each model combination, in authored order, after T-10's check:

| # | Condition | Disposition | Reachable in B2's domain? |
|---|---|---|---|
| 1 | Its gate evidence entry is `withheld: true` (the T0R mechanics gates; `combination_gates`) | `base_withheld`, `reason` = the gate's code, `result_ids` empty | **No.** D1.5 and D1.6 exclude every gate's trigger (nonlinear supports, constant effort, mixed modulus bases). Synthetic corpus base only |
| 2 | Basis `mechanics`, terms name **distinct** load cases, and at least one term's case is selected | **Retained** (decided at T-10b) | Yes |
| 3 | Anything else: subtraction, range, mechanics with no selected operand, or mechanics with a repeated case (§0 item 3, C-1) | `ordinary`, `reason: "no_retained_mechanics"` | Yes (W-CB4, W-CB5) |

- An `ordinary` or `base_withheld` combination gets no W1 work, no Call, no Run, no source, no attempt and no diagnostic.
- A combination whose expression the ordinary route could not resolve (gate entry `withheld: false`, no rows) is unreachable in a solved model (`CombinationExpression::resolve`'s note). If it ever appears, rule 2 or 3 applies. A retained one then fails its freeze coverage (`facade_certificate`), which is fail-safe.

### 2.4 T-10b: operand sources, operand preparations, Calls, Runs and freezes

**(i) Operand sources** (decision 6 with S-2; KD §2). For each retained combination, terms in authored order:

| Term's case | Operand | Recorded API (KD §1.2) |
|---|---|---|
| `selected` | Its `RetainedSolve`; cache imports allowed | `RecordedOperand::Selected(&solve)` |
| `unavailable` with a CaseSource, whose kernel prep was built (its Run is not `refused` with `ledger_unavailable`). This includes a kernel-selected case whose freeze failed (R-11) | A `PreparedCaseSource` rebuilt from PP's retained prepared `PrimitiveSource` for that case, once per case and shared by every combination; no import | `RecordedOperand::Prepared { source: its batch-registered source id, prepared }` |
| `unavailable` with no CaseSource (T-7 preparation failure), or whose Run refused `ledger_unavailable` (its kernel prep refused, so a rebuild must refuse again) | **None.** The combination is `retained_unavailable` with cause `operand_source_unavailable {operand_index}`, naming the first such term. It requests no operand preparation and makes no Call (C-4) | — |
| `not_required` | The case's operand preparation's source (T-10b(ii)) | `RecordedOperand::Prepared { source: its registered id, prepared }` |

A rebuilt source's identity is checked by the kernel (KD I7: full K4SRC bytes against the registered origin). A rebuild that refuses where the batch built the prep, or whose identity differs, is an invariant failure. The kernel reports it as `OriginRefusal(MissingSelectedOrigin)` (R-1), which abandons the successor.

**(ii) Operand preparations** (C3a-1 to C3a-6 above).

**(iii) Per retained combination, in authored order:**
1. **Decided without a Call.** If (i) gave `operand_source_unavailable`, or some `not_required` term's preparation was refused (the first such term in authored order gives `operand_preparation_failure`), the combination is `retained_unavailable` with `call_ref`, `run`, `source_ref` and `product_attempt_ref` all null.
2. **Otherwise, one `MechanicsCombinationCall`** through `RecordedInvocation::solve_combination_sources(operands, CaseLimit(20,000,000,000))`, on the invocation's one meter (60,000,000,000; C1 §2). Its kernel checks run in KD §1.4's order: capacity → `NoOperands` → `NestedCombination` → `OperandsDiffer` → `NoSelectedOperand` → custody (R-1) → combined preparation → Run.
   - **`Ok(PreSourceRefusal)`:** the Call is recorded with result `pre_source_refusal`. The combination is `retained_unavailable` with its `call_ref`, and `run`, `source_ref` and `product_attempt_ref` null. Its cause is the Call's typed reason.
   - **`Ok(OriginRefusal)` or `Err(OriginError)`:** abandonment (§2.6).
   - **`Ok(WithRun)`:** the kernel has registered the `CombinationSource` and recorded its Group and one Run. The combination's `CombinationAttempt` starts here (§4), with native `completed` if the Run is selected and `failed` otherwise.
3. **Run not selected:** `retained_unavailable`, `combination_unresolved`, phase `kernel` (§2.6).
4. **Run selected: the combination's own freeze**, over its own row block: DEF-C's dual-readout proof, the certificate with KD §5.2's coverage rule, observables and G5a, on the combination's own selected owner (`product_owner` gives `NativeOwner::Combination`, KD §1.5).
   - **Passing:** `retained_selected`.
   - **Refused:** `retained_unavailable`, `facade_certificate`, phase `facade`.

Then the next combination. The freeze uses no meter (facade work is outside LME), so the meter chain is the Calls' alone.

### 2.5 The Call, `CombinationSource`, Group, imports and Run (C2 §3–§4; KD)

- **`MechanicsCombinationCall`** (C2 §4): `{id, kind: "mechanics_combination", owner_refs: [{kind: "combination", index}], requested_operands: [{source_ref, factor}], source_refs, run_refs, invocation_before, invocation_after, result}`.
  - `requested_operands` keeps the authored terms in order, repeats included (there are none in a retained combination; C-1), each `factor` as its binary64 bits.
  - `source_ref` is the operand's source:
    - a `selected` or `unavailable` case's `source_ref`;
    - a `not_required` case's operand-preparation source.
  - `result` is `{kind: "runs"}`, with `source_refs = [the CombinationSource]` and `run_refs = [its Run]`, or `{kind: "pre_source_refusal", stage, reason}`, with both arrays empty and `invocation_after == invocation_before`.
  - **The pre-source reasons** (`CombinationReason`):
    - stage `operand_validation`: `no_operands`, `nested_combination`, `operands_differ` and **`no_selected_operand`** — four reasons (RV115 N-4; R-3);
    - stage `combined_preparation`: `ledger_unavailable {error: LedgerError}` and `count_range {name}`. FK's `CasePrep::combination` returns both; C2 §2 named only the first.
  - **Not used in B2:** C2 §2's three Run-terminal tags (`combination_unresolved`, `unresolved`, `refused`). The recorded path puts the Run's own native reason in its `kernel_terminal`.
- **`CombinationSource`** (C2 §3 exactly): `{index, owner: {kind: "combination", combination_index, combination_id}, kernel_source_sha256, ledger_sha256, stiffness_sha256, representative_source_ref, operands: [{case_index, factor, source_ref, source_identity_sha256}]}`.
  - `representative_source_ref` = `operands[0].source_ref`. Operand 0 is the representative whatever its kind (KD I5).
  - `kernel_source_sha256` = sha256 of K4CMB: `"K4CMB\x01"`, then u32le(h), then for each operand in order u64le(factor bits), u32le(length of its K4SRC) and its K4SRC bytes (FK `CasePrep::combination`, `source.rs` `put_u32`/`put_u64`).
  - `ledger_sha256` = sha256(K4LED). It equals the selected combination's `selection.ledger_sha256`.
  - `stiffness_sha256` equals every operand CaseSource's.
- **Group** (C2 §4): a new, call-local Group whose `first_source_ref` and `source_refs` are the CombinationSource, with `preparation: {kind: "ready"}`.
  - **The kernel prep:** the kernel runs on the **first selected operand's GroupPrep** (S-2 (c); KD I4). Operand 0 stays the representative.
  - **`imports`:** `[{operand_index, selected_run, slot, build}]`, in fixed slot order, for each slot the first occupied one among the **selected** operands in authored order (`GroupCache::merged`).
  - **Prepared operands import nothing,** including a kernel-selected, freeze-failed one (R-11).
  - The Run's `cache_before` is exactly the imported set. The combination's own new builds stay in its own group, and nothing is written back into an operand.
- **The Run:** `origin = {call, position: 0, group, source_ref: the CombinationSource, owner_ref: {kind: "combination", index: authored index}}`.
  - Its records, attempts, charges and cache inventories follow C1/C2 as a case Run's do.
  - **Ordinal mapping:** FK names the owner `NativeOwner::Combination(k)`, where k is the k-th mechanics Call, refused Calls included. PP maps k to the authored index. FK names a prepared registration `NativeOwner::Case(j)` with j at or above the batch length, and PP maps j to the owner's request index.
- **R-8, operand equality** (DEF-C `scope.operand_equality`; G8). Under D1.5 it holds by construction; a reader still checks it.
  - every operand's CaseSource has the same `material_basis_ref` and `section_terms` (every member, including `geometry`) as the representative's;
  - the combination's `selection.section_terms` equal the representative's, projected to the Selection members;
  - each operand's preparation record (its case `ProductAttempt` or its `OperandPreparation`) is bound to its own CaseSource by C3's G8 equalities. So the prepared D, t_eff, A, I, J, Z and c bits are equal across operands transitively.

### 2.6 Failures (decision 7) and N-5

| Failure | Result | Notices |
|---|---|---|
| An operand with no usable source (T-10b(i)) | This combination: `retained_unavailable`, `combination_unresolved`, phase `preparation`, cause `operand_source_unavailable {operand_index}` | None |
| A `not_required` operand's preparation refused | This combination: the same, cause `operand_preparation_failure {operand_preparation_ref}` | None |
| `pre_source_refusal` | This combination: the same, cause the Call's `CombinationReason` | None |
| The Run is not selected (any terminal, the ceiling included) | This combination: `combination_unresolved`, phase `kernel`, cause `prepared_product_failure {product_attempt_ref: its CombinationAttempt}` | None |
| The freeze refuses after a selected Run (certificate, values, observables, G5a, or a capture error) | This combination: `facade_certificate`, phase `facade`, the same cause | None |
| **Any `OriginError`** from `register_prepared_source` or `solve_combination_sources`, or an `OriginRefusal` (`MissingSelectedOrigin`, R-1); a `PreparedCaseSource::new` refusal after a completed operand preparation; a rebuild refusal where the batch built the prep | **The whole successor is abandoned** (T-12). New internal cause `W1Fallback::CombinationCustody` | One per case in A; plain text (C1:68's detail is only for serializer causes) |
| A staging fault, any serializer failure, precommit | Abandoned, as B1 (DESIGN decision 5) | As B1 |

**N-5 (R-1's condition), stated as contract text.** A combination's origin refusal never becomes a capture error. No `OriginError` from a combination's custody, `MissingSelectedOrigin` included, is ever wrapped as `CaptureError::Origin` or serialized in any receipt member. Each abandons the successor through `W1Fallback::CombinationCustody`. The only `CaptureError::Origin` a receipt can carry stays B1's T-8 call failure (D38's representation). A reader therefore never meets a combination's origin refusal.

T-12 is unchanged. Notices are reserved and published only for the cases in A, never for a combination (decision 7). An abandonment at T-10b publishes them, as any abandonment after T-8 does.

### 2.7 Staging, diagnostics, the receipt and `execution_order` (T-11′)

**Staging** (one copy of the ordinary envelope, as B1):
1. The case overlays, in request order (B1).
2. For each `retained_selected` combination, in authored order: its frozen row values on its own block, and `recovery_method` on each of its rows. A combination has no maxima patch and no evidence patch.
3. At c ≥ 2, the headlines over **load-case rows only** (§2.2).
4. A staging fault abandons the successor.

**Diagnostics** (the serializer, after the case diagnostics, which keep B1's order):
- **Order and form:** for each retained combination in authored order, one diagnostic `{id, code, severity: "info", message, source: "core/product_physics", affected_refs: [combination id]}`.
- **`retained_selected`:** id `diagnostic:retained-precision:<combination id>:selected`, code `RETAINED_PRECISION_SELECTED`, message **COMBINATION_SELECTED_MESSAGE** = "Retained-precision recovery (contribution_preserving_multiprecision_v1) is selected for this load combination, from its own solve and certificate. Its published rows carry recovery_method; the retained_precision receipt binds their certified classes, the native attempts and work, and its operands' sources."
- **`retained_unavailable`:** id `diagnostic:retained-precision:<combination id>:unavailable`, code `RETAINED_PRECISION_UNAVAILABLE`, message **COMBINATION_UNAVAILABLE_MESSAGE** = "Retained-precision recovery is unavailable for this load combination. Its published rows keep their ordinary values and diagnostics and are withheld from rule binding; the retained_precision receipt records the actual attempt and its typed cause."
- **No diagnostic** for `ordinary` or `base_withheld` (C1 G4's analogue).
- **A collision** of either id with an existing diagnostic is a serializer refusal (`Association`), which abandons the successor, as `selected_case_envelope` does for cases. D1.4's id disjointness rules out a collision with a case's notice or diagnostic.

**The receipt body:**
- **`combinations[]`:** one entry per model combination, in authored order (§6.1's `Combination`).
  - `basis_ref = {ref_type: "combination", ref_id}`.
  - `expression` from the invocation:
    - mechanics: terms with `case_id` and factor bits, in authored order;
    - subtraction: minuend, then subtrahend;
    - range: `operand_ids` sorted as the producer sorts them (Rust `String` order, that is UTF-8 byte order; TS must not use JavaScript's UTF-16 default sort), and `mode`.
  - `result_ids`: the ids of its rows, in publication order.
  - `diagnostic_refs`: D6a's rule for the combination id (the envelope's diagnostics whose `affected_refs` name it, once each, in envelope order, excluding `RETAINED_PRECISION_*`).
  - The members of its disposition's branch.
- **`operand_preparations[]`:** present only when non-empty (C-6), in start order.
- **`sources[]`:** registration order. First the batch's case sources (B1), then the operand-prepared case sources in registration order, then each `CombinationSource` in Call order.
- **`calls[]`:** the `CaseBatchCall` (id 0), then each `MechanicsCombinationCall` in Call order. Ids are consecutive, and every Call's `invocation_before` equals the previous Call's `invocation_after`.
- **`groups[]`** and **`builds[]`:** as recorded.
- **`product_attempts[]`:** actual start order, so the case attempts (T-7) come first, then the `CombinationAttempt`s in Call order.
- **`work.execution_order`:** every Run in actual order. First the batch's case Runs, then the combination Runs, each `{kind: "combination", index: authored index}`.
- **`work.charged`:** the last Call's `invocation_after`, which is also the last Run's when that Call made one.
- **`material_bases[]`, `ordinary_attempts[]`, `cases[]` and `legacy_source_work[]`:** unchanged; combinations add nothing to them.

**Hashes.**
- A `retained_selected` combination's `source_identity_sha256` = H(`retained_precision_source_mp_v2`, its CombinationSource without `index`).
- Each `CombinationSource.operands[i].source_identity_sha256` equals the recomputed identity of the CaseSource at its `source_ref` (C2 §3).
- The publication and receipt hashes are B1's.

### 2.8 The outcome table, B2's rows added to DESIGN T-12's

| Outcome | Published | Notices |
|---|---|---|
| D1.4 refusal (T-1), or the domain re-check (`Domain`) | Ordinary bytes, exactly | None |
| T-10b abandonment (`CombinationCustody`) | Ordinary bytes, then notices | One per case in A |
| At least one case selected, T-11 completes, any combination dispositions | The successor | None (a retained combination carries its own diagnostic) |

## 3. DEF-C: the combination formation definition (brief item 3; decision 28; REV §1.1 item 1)

**The draft** is `statics/retained_precision_prepared_combination_v1.json`. Like DEF-O, it is compact, sorted-key, ASCII-only JSON with no float and no trailing newline, so its bytes are its canonical form.

| | |
|---|---|
| Raw sha256 (9,944 B) | `03d40598be82a5df8867bb9a279b212b3acd53ba5f51ff85b9635e6cefaa69cc` |
| **H(`retained_precision_formation_v1`, DEF-C)** | `9adf5178c7c1d5b81de340c2f0f605396e717a9264b59430df8d2a1306f21731` |
| H under a new domain `retained_precision_combination_formation_v1` (R-10's alternative, for comparison only) | `562cbe14b9920bfdf3aa33bd4d7a04e223315ba3e16d321afb5630956f35487d` |
| Collision check (§13) | 0 hits outside `P/execution` for the id, the file stem and the alternative domain |

**DEF-O's bytes are unchanged** (raw `3e0779a4…`, H `a7ed7ca0…`). Its six maintained pins (RV114 `checks.txt` §4) do not move.

**R-10: DEF-C reuses `retained_precision_formation_v1` as its hash domain** (recommended, as KD and RV115 did). A domain names an object type, and a definition object is a definition whatever it defines. The ids differ, so no two definitions can share an H. A new domain would add a name and a reader constant for no gain.

**What DEF-C binds, member by member against DEF-O** (the generator lists every changed path; KD §4's content, R-7 and R-8):

| Member | DEF-C |
|---|---|
| `id`, `version` | `RP-PREPARED-COMBINATION-DUAL-v1`, 1 |
| `inherits` | DEF-O's eight values, plus `operand_definition: {id: RP-PREPARED-ORDINARY-DUAL-v1, sha256: a7ed7ca0…}` |
| `scope.owners` | `mechanics_combination` |
| `scope.trigger` | T0R-admitted mechanics, distinct operand cases, at least one selected after T-9; the group is the first selected operand's and the representative is operand 0's |
| `scope.loads` | `combined_exact_ledger` |
| `scope.prescriptions` | `exact_positive_zero` (every operand term) |
| `scope.operand_sources` | Selected: its solve, with imports. Unavailable with a source: rebuilt, K4SRC-checked, no import. `not_required`: its one shared OperandPreparation source, no import. No operand is solved, mutated or re-prepared |
| **`scope.operand_equality`** (R-8) | Equal K4STF, layout, stations and supports (the kernel's `OperandsDiffer`), **and** equal prepared section facts across the operands and the combination: material basis, normalized D, effective wall, and prepared A, I, J, Z, c of every member. The kernel checks only K primitives: t_eff and Z are not in K4STF |
| `scope.excludes` | DEF-O's list minus `prepared_combinations`, plus `result_state_subtraction`, `range_envelope`, `nested_combinations`, `repeated_operand_cases` (C-1), `combinations_without_selected_operand`, `combination_maxima`, `intensified_rows` and `nonzero_operand_prescriptions`. `exact_profile_prepared_formation` stays |
| `scope.materials`, `supports`, `source`, `entry`, `requires`, `scope_limit` | DEF-O's |
| `preparation` | `{combination: not_entered, operands: …}`: each operand source is a DEF-O prepared source, through a case attempt (`retained_precision_preparation_v1`) or an OperandPreparation (`retained_precision_operand_preparation_v1`) |
| `stages` (new) | Entered: native, proof_start, projection, maxima, values, aliases, certificate, observables, g5a. Not entered: preparation. Maxima and aliases complete empty: there is no maximum row, and headlines cover load cases only |
| `lanes` | DEF-O's, with `loads` (new): each loaded DOF's exact combined net N_g (K4LED) enclosed outward at 1024 bits in free residual rows and constrained reaction offsets. `admitted_k` reads the representative's members. `owner` is the combination's recorded selected run, its K4CMB source and ledger, its cache (imports plus its own builds) and one non-reused anchor |
| `precision`, `projection` | DEF-O's |
| `rows` | DEF-O's recipes for component, component_stress, displacement_magnitude and support_magnitude. `prescribed`: combined prescription exactly +0. `maximum`: not published. `ancillary`: none. `coverage` (new, R-7): 7n + 50m + 8g; slots 0–19 per member, slot 20 empty; no record; a missing, duplicated or extra row fails the whole proof |
| `acceptance` | DEF-O's, plus `composition`: no operand radius, row, state, verification or certificate enters the proof; the rows are not the factor-weighted sum of the operands' published rows (DESIGN §4.2's disclosed property) |
| `hash_domains` | definition `retained_precision_formation_v1` (R-10); publication, receipt and source as DEF-O; no preparation domain |
| `trust.G8` | The expression equal to the invocation's; operand CaseSources rederived as cases; K4CMB recomputed and bound to `kernel_source_sha256`; one common K4STF; `operand_equality`; `ledger_sha256` bound as attestation |
| `work` | DEF-O's, without preparation. Combined-net enclosures are counted as `add` entries (KD §3.7), and the native meter is the combination's own Run under its own 20B case limit |

**R-7 confirmed** (§0 item 10; KD §5.2 and RV115 §1.4 agree). The coverage text is KD's rule verbatim in substance. S-14's stop stays for any later change of PP's combination publication.

## 4. The combination's product attempt (brief item 4; REV §1.1 item 2)

**Owner kind `combination`. Its own closed `$def`, `CombinationAttempt`:**

```text
CombinationAttempt = {
  id:U, definition_id:"RP-PREPARED-COMBINATION-DUAL-v1",
  owner_ref:{kind:"combination", index:U},          // authored combination index
  material_basis_ref:U,                              // the operands' one basis
  source_ref:U, run_ref:U,                           // its CombinationSource and its Run
  result: {kind:"ready"} | {kind:"unavailable", error:PublicError},
  stages:{native, proof_start, projection, maxima, values, aliases, certificate, observables, g5a},  // no preparation
  proof:null|ProofTrace, adapter:AdapterTrace, overlay_work:ScalarTrace, g5a_work:ScalarTrace }
```

- **When it exists.** Exactly when the combination's Call returned a Run (`WithRun`). That is the C3 rule "null iff no attempt entered", with the attempt entering at its native stage. So `product_attempt_ref` is null exactly when `run` is null. A pre-source refusal, an operand preparation failure and an unavailable operand source have no attempt (C-3).
- **Stages** (DEF-C `stages`).
  - native: `completed` iff the Run is selected, otherwise `failed`, with error `{kind: "native", run_ref}` and every later stage `not_entered`.
  - The proof stages follow C3's rules, transitions as entered (C3 §2: "Stage values must come from entered/returned transitions").
  - There is no preparation stage: preparation belongs to the operands' case attempts or OperandPreparations.
- **Order in `product_attempts[]`:** actual start order (its Call), so after every case attempt, and in authored order of the combinations that reached a Run.
- **`product_attempt_ref`** on `retained_selected` (non-null) and `retained_unavailable` (null or not) entries; not a member of `ordinary` or `base_withheld`.
- **Why a separate `$def` rather than widening `ProductAttempt`** (C-3).
  - A combination has no ordinary attempt, no preparation and no operational re-evaluation. `ProductAttempt` requires `ordinary_attempt_ref`, `preparation` and `operational`, and any value there would be fabricated.
  - The union keeps both shapes closed. B3b's `SCHEMA_ENUM.diff` then applies to `ProductAttempt` unchanged.
- **Its record point** is its terminal stage (native for a non-selected Run, the last proof stage otherwise), and its snapshot is of the one shared capture (B1 T-11).
- **The reason table** for a `retained_unavailable` entry with an attempt, applied by the attempt's own error (the combination analogue of D4d):
  - error `native` → (`combination_unresolved`, `kernel`), with the Run not selected and `run_ref` its id;
  - error `capture` with a non-selected Run → the same;
  - any other error → (`facade_certificate`, `facade`), with the Run selected.
  - Error `preparation` is refused.

## 5. R-COMB-1's representation (brief item 5; decision 8; N-6)

**The rule (decision 8, unchanged in substance).** These rows are `not_covered`, withheld from binding with D-U6-2's disclosure, and their values are unchanged:
- every row of an `ordinary` combination whose expression references a case that is not `not_required`;
- every row of a `retained_unavailable` combination.

`base_withheld` has no rows. The rows of an `ordinary` combination over `not_required` cases only keep their base standing. Envelope standing stays D2 §4.9.4's: an `unavailable` case still makes it `needs_recompute`, and a combination never does.

| | **(R) Reader-derived** (recommended) | (M) A receipt member |
|---|---|---|
| Representation | None on the wire. Each reader's validation appends, after the selected cases' and the `retained_selected` combinations' G5c classes, one `{result_id, basis_ref, normalized_bits, scale_bits: null, class: not_covered}` per R-COMB-1 row: combinations in authored order, rows in publication order | `ordinary` and `retained_unavailable` entries gain `row_class: "not_covered" | "base"`; readers check it against the rule (G5, `ATTEMPT_MISMATCH`) and then use it |
| Inputs | Disposition, expression and `cases[].status`: all bound by the receipt hash, all already read by G3/G5 | The same, restated |
| D-U6-2 (derivatives) | `retained_row_classes` → `class_disclosure` gives `retained_precision_not_covered` for those rows; no code change | Same, once readers return it |
| T6S | Its withheld-witness path takes classes from the reader (I74 §4.3: "takes classes only from the reader"); no code change | Same |
| Transport (G0–G2 only) | No raw rows, so no classes; never eligible anyway | The member travels, but transport is never eligible, so nothing is gained |
| Cost | Readers only: about 1 h per language, plus the must-pass `expected_classifications` | Schema, producer, readers and two more mutations |

**Recommendation: (R).** Both inputs are in the receipt; the consumers already take classes from the reader; a stored copy would be one more thing to forge consistently and one more check. I93 leaned the same way (REV N-6).

**Owner note (names DN §4.2).** DN §4.2's coverage table lists combinations, "T0R-admitted `mechanics`, subtraction, range", under W1a, "at the facade: exact expansion over the retained states, rounded once". C1 §5 and SC1 §2 then kept subtraction and range on their base contract. In F2a:
- rows of subtraction and range combinations that reference a Sensitive (non-`not_required`) case, and rows of unavailable combinations, are withheld from binding as `not_covered`;
- only mechanics combinations with a selected operand get their own certificate.

RR "B2/B3 R1…" already informed the owner of this, and successors are not public before B8.

**Two notes for ROOT:**
- **The per-case classification summary is unchanged** (RS `classification_summary_from`, PY, TS `retainedPrecisionStanding.ts`). Combination rows are disclosed per row, not counted in a per-case line. Adding per-combination summary entries would change a T6S-visible shape, which I74 §4.3 returns to ROOT. I recommend not doing it in B2 (C-14).
- **D-U6-2's not-covered text** reads "no verified accuracy for this quantity kind". For an R-COMB-1 row the cause is the combination's operands, not the kind. The text is a shared constant whose change S-I2 already owns (I74 §4.3). I recommend leaving it in B2 and noting it for S-I2.

## 6. SCHEMA's `$defs` (brief item 6)

### 6.1 B2's change (`statics/SCHEMA_B2.diff`, against main's SCHEMA `07951eda…`)

Every change is additive: a new `$def`, or a new branch in a `oneOf`. Nothing that an existing receipt uses changes, and §6.3 checks that every existing receipt keeps its verdict.

| Where | Change | Basis |
|---|---|---|
| `title`, `$comment` | "Standalone prepared C1/C2/C3 receipt"; one coherent comment for the ordinary and combination formations (§6.2 gives the J1 text) | RV116 N-8 |
| `Body.combinations` | `items: {$ref: Combination}`; `minItems 0`/`maxItems 0` removed | PLAN §1.2.2 |
| `Body.operand_preparations` (new, optional) | `[OperandPreparation]`, `minItems 1`: absent when empty (C-6) | C3a; PLAN risk 5 |
| `Body.sources.items` | `oneOf [CaseSource, CombinationSource]` | REV §1.1 item 3; C2 §3 |
| `Body.calls.items` | `oneOf [Call, MechanicsCombinationCall]` | C2 §4 |
| `Body.groups.items` | `oneOf [Group, CombinationGroup]` (the case group unchanged; the combination group adds required `imports`) | C2 §4 |
| `Body.product_attempts.items` | `oneOf [ProductAttempt, CombinationAttempt]` | REV §1.1 items 2–3; C-3 |
| `CaseSource.preparation` | a third branch, `{operand_preparation_ref, sha256}` | C3a |
| `UnavailableCause` | two branches before `Reason`: `{kind: "operand_preparation_failure", operand_preparation_ref}` and `{kind: "operand_source_unavailable", operand_index}`; and `CombinationReason` after it | C3a rule 4; C-4; C2 §4 |
| new `Combination` | `oneOf` four closed branches over the common members `{basis_ref (ref_type const combination), expression, disposition, result_ids, diagnostic_refs}`: **`retained_selected`** + `method, call_ref, product_attempt_ref, run, source_ref, source_identity_sha256, selection`; **`retained_unavailable`** + `call_ref, product_attempt_ref, reason{code, phase, cause}, diagnostic_ref, run, source_ref` (`call_ref`, `product_attempt_ref`, `run` and `source_ref` each `null|…`); **`ordinary`** + `reason: "no_retained_mechanics"`; **`base_withheld`** + `reason` ∈ the three gate codes, `result_ids` `maxItems 0` | C1 §5; C2 §4 |
| new `CombinationExpression` | `mechanics {terms: [{case_id, factor}] minItems 1}`, `result_state_subtraction {minuend_id, subtrahend_id}`, `range_envelope {operand_ids minItems 1, mode ∈ min, max, min_abs, max_abs}` | C1 §5 |
| new `CombinationReason` | `space: "combination"`; tags `no_operands`, `nested_combination`, `operands_differ`, `no_selected_operand`; `ledger_unavailable {error}`; `count_range {name}` | C2 §2 and §4; RV115 N-4 |
| new `CombinationSource` | C2 §3's members exactly; `operands minItems 1` | C2 §3 |
| new `MechanicsCombinationCall` | C2 §4's members; `owner_refs` exactly one `{kind: "combination", index}`; `source_refs` and `run_refs` at most one; `result` `runs` or `pre_source_refusal {stage, reason}` | C2 §4 |
| new `CombinationGroup` | `Group`'s members plus required `imports: [{operand_index, selected_run, slot, build}]` | C2 §4 |
| new `CombinationAttempt` | §4 | REV §1.1 item 2 |
| new `OperandPreparation` | DESIGN §4.2's P1 members; `definition_id` const DEF-O's id (not the exact id: RV116 N-8); `requested_by minItems 1`; `result` `prepared` or `refused` with PublicError's preparation branch; `adapter` and `operational` copied from `ProductAttempt` (the latter with its `old_coverage`) | C3a; RV116 N-8 |

**Not added (C-11):** C1 §5's `operand_cases` and `cache_inputs`. C2 superseded both:
- the operand identities live in `CombinationSource.operands` and the statuses in `cases[]`;
- "C1 cache_inputs is now a derived compatibility view …; no duplicate cache_inputs payload is serialized" (C2 §4), and the imports are `CombinationGroup.imports`.

The schema type domain stays wider than the emitted domain, for example a `Owner`-typed field, or `UnavailableCause` admitting a `CombinationReason` on a case. G3, G5 and G8 enforce the emitted domain, as SCHEMA's `$comment` states (RV78-N8).

### 6.2 The merged J1 text (NA-1; `statics/retained_precision_mp_v2.schema.json`, `statics/SCHEMA_J1.diff`)

**B3b's `SCHEMA_ENUM.diff` is B3b's whole SCHEMA change:** `ProductAttempt.definition_id` becomes the enum [ordinary, exact], plus N-8's `title` and `$comment`. B2's change touches neither `ProductAttempt` nor its `definition_id`, so the only textual overlap is the `title` and `$comment` lines.

**The merged text** is produced by applying I96's own `schema_r1` (imported unchanged from `b3d_statics_r1.py`) to main, then B2's `$defs`, then one coherent text:
- **`title`:** "Standalone prepared C1/C2/C3 receipt" (B3b's, which B2 also uses).
- **`$comment` prefix:** "Prepared ordinary and exact case formations, selected by ProductAttempt.definition_id, and the prepared combination formation (CombinationAttempt, RP-PREPARED-COMBINATION-DUAL-v1) with its operand preparations (OperandPreparation, ordinary only); the exact route's constraints are reader gates (G0, G5b, G8), and combination dispositions, operand sources and row coverage are reader gates (G3, G5, G8), not schema branches." The comment's remaining sentences (encoding versus shape, emitted domain, D9b) are unchanged.

**Composition, checked by the generator:**
- the J1 text equals B2's text with B3b's `definition_id` enum and the merged `$comment`;
- it differs from B2's `$defs` only in `ProductAttempt`, and from B3b's only in B2's `$defs`.

**Across the merged text, the definition ids are:**
- `ProductAttempt.definition_id` ∈ {ordinary, exact};
- `CombinationAttempt.definition_id` = combination;
- `OperandPreparation.definition_id` = ordinary.

So every receipt attempt admits exactly one of the three, and B3b's exact id never appears on an operand preparation (N-8).

### 6.3 Checks (`_run_records/b2c_checks.out.json`)

- **Meta-validation:** `jsonschema` (Draft 2020-12) `check_schema` passes for main's SCHEMA and the J1 text.
- **Vocabulary:** the J1 text uses only main's keywords: `$ref`, `oneOf`, `const`, `enum`, `type`, `properties`, `required`, `additionalProperties`, `items`, `minItems`, `maxItems`, `minLength`, `minimum`, `maximum`, `pattern` and `x-rp-encoding`. PY's walker (`_shape`, `_encoding`) knows exactly these; RS's and TS's mirror it.
- **Existing tests that read SCHEMA's content, emulated on the J1 text:** every object is closed with `required` within `properties` (PY `test_all_object_definitions_are_closed`). The 400-byte window after `"const": "unavailable_precondition"` still names `caller`, `source_family` and `resource_admission` (PP `refusal_kinds_are_the_schema_preconditions`), because B2's new causes come after `facade_failure`.
- **Backward compatibility,** with both `jsonschema` and PY's own `_shape` (loaded from NUM's committed reader, its schema loader pointed at each text):

  | Population | Under main | Under J1 | Changed |
  |---|---|---|---|
  | 17 CORPUS bases | all valid (both) | all valid (both) | 0 |
  | 28 must-pass entries (edits applied) | valid | valid | 0 |
  | 294 mutations (edits applied) | 263 valid, 28 invalid, 2 valid for PY only, 1 with the receipt removed | identical | **0** |
  | 6 committed successor documents (milestone, L = 0, derivatives) | valid | valid | 0 |

  The two PY-only valid mutations are `coverage_body_fraction` and `coverage_body_negative`, expected at G2: PY defers integer encoding to G2, which `jsonschema` checks as shape. The difference is the same under both texts.
- **29 hand-built instances of the new `$defs`** (13 positive, 16 negative), plus four body-level cases, give the same verdict under `jsonschema` and PY's walker, as designed. Among them:
  - a combination group read as a case `Group` is refused (closed);
  - a mechanics call read as a `Call` is refused;
  - `operand_preparations: []` is refused;
  - an OperandPreparation with the exact id is refused;
  - a case attempt carrying DEF-C's id is refused.

## 7. PTABLE's one revision (brief item 7; REV §2; RR "RV116 confirms B3-D's revision 01…", ruling 3)

**The draft:** `statics/semantic_contract_v0_3_preview_physics_retained_1.json`, sha256 `863738f1aa81c7bdbf87c716b68e690c6104d7d25416df8ed4c28caece44d0e1` (53,462 B; PTABLE's own serialization, indent 2, ASCII, one trailing newline).

| Member | Change |
|---|---|
| `accuracy_classification.scope` | "registered ordinary-prepared source, and registered prepared mechanics combinations (RP-PREPARED-COMBINATION-DUAL-v1: their own solve on the combined exact ledger, at least one operand selected); no exact-profile, subtraction or range extension". It was "… only; no exact-profile or prepared-combination extension". No other `accuracy_classification` key changes |
| `product_formation_definitions` | `[{DEF-O id, a7ed7ca0…}, {DEF-C id, 9adf5178…}]` |
| `formation_warrant` | A list of two warrant objects, each shaped exactly as XTABLE's single one: DEF-O's (its bytes unchanged) and DEF-C's (the same text with DEF-C's id) (C-8) |
| `receipt_bindings` (new; B3D-8) | **XTABLE's object, copied from XTABLE's bytes:** `canonicalization`, `method`, `projection_policy`, `work {case_limit, invocation_limit}`, `work_policy`. Equal to XTABLE's both canonically and as indented text (generator controls) |

**Unchanged** (generator: `members_changed` = these three, `members_added` = `receipt_bindings`): the contract id, profile, rows, inherited hash `ae55503d…` and every other member. That includes `combination_policy`, `supported_profile_limitations` (still true: "Mechanics combinations are withheld for …; no combination maxima") and `reserved_inactive_successors`.

**The hash cascade** (constants only; RV114 §6's 12 files): `c74742ce…` → `863738f1…`. J1 also needs the three non-hash edits of §11.

## 8. G0 per N-12 and decision 31, in all three readers (brief item 8)

**The table read.** G0 reads the reader's packaged PTABLE: RS `TABLE_BYTES`, PY `ROOT/fixtures/results/…`, TS's JSON import. The order below keeps today's checks first and appends the new ones, so no 07n first failure moves.

| # | Check | Code |
|---|---|---|
| 1–3 | Today's identity, schema/component version and table-identity checks | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| 4 | **Definitions:** the table's `product_formation_definitions` equals the constant list [{DEF-O id, H}, {DEF-C id, H}] in that order, and each packaged definition file's H(`retained_precision_formation_v1`) equals its constant (DEF-C packaged as DEF-O is: RS `include_str!`, PY a file read, TS an import) | `RETAINED_PRECISION_FORMATION_MISMATCH` |
| 5 | Today's table hash and inherited hash | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| 6 (new) | **Cross-check:** the table's `receipt_bindings` equals the constant object exactly (keys, strings, integers by value), its `receipt_policy` equals the `policy` constant, and its `accuracy_classification.policy` equals the `facade_policy` constant | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| 7 | The body exists (today's) | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| 8 | **The bound receipt members against the table:** `receipt_version` = 1; `policy` = table `receipt_policy`; `facade_policy` = table `accuracy_classification.policy`; `projection_policy`, `work_policy` and `canonicalization` = the table's `receipt_bindings`; `work.case_limit` and `work.invocation_limit` = its `work`, by value. Today these compare with constants; after 6 the table and the constants agree, so each is one check | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| 9 | **Every `product_attempts[]` and `operand_preparations[]` `definition_id`** is an id in the table's `product_formation_definitions` (today: equal to DEF-O's id) | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |

**The constants kept as a cross-check (N-12):**
- RS's G0 literals (`policy`, `projection_policy`, `work_policy`, `facade_policy`, `canonicalization`, `case_limit`, `invocation_limit`) and `METHOD`;
- `DEFINITION_ID`/`DEFINITION_HASH`, plus a combination pair (suggested `COMBINATION_DEFINITION_ID`/`COMBINATION_DEFINITION_HASH`);
- PY's equivalents in `_validate_draft` and its module constants;
- TS's in `header`, with `PREPARED_DEFINITION_ID`/`PREPARED_DEFINITION_HASH` and a combination pair.

**What G0 does not check:** which owner kind carries which definition. A case attempt with DEF-C's id passes G0, because DEF-C is in the table, and fails G1's shape (§10.3 m8). The method token stays G1's schema `const` and G6's row check; G0 checks only the table's `method` against `METHOD` (row 6).

**The tests.**
- **07o:** one mutation per bound receipt member, each refused at G0 in all three readers (§10.3 m3–m7). Today's constants already refuse each, so these mutations pin the behaviour across the change.
- **A reader-local unit test per language.** G0's table-dependent checks take the table as an internal parameter: production passes the packaged copy, and the test passes a test-only copy. Then:
  - one test per `receipt_bindings` member and one each for `receipt_policy` and `accuracy_classification.policy` (8 in all): the table changed, the receipt unchanged, expecting G0 `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`;
  - one with `product_formation_definitions` reordered and one with DEF-C's H changed: G0 `RETAINED_PRECISION_FORMATION_MISMATCH`;
  - one positive control on the packaged table.

  A corpus entry cannot change a table (REV §2).

## 9. D1.4's text and the cap rows (brief item 9; decision 10; N-4)

**D1.4 (B2), contract text.** The model has 1 ≤ c ≤ 3 load cases and z ≥ 0 load combinations of any basis, with **C_eq = c + z ≤ 3**. Every mechanics combination has **at most 3 terms**, repeats counted, and every range envelope **at most 3 operand ids**. **No combination id equals a load-case id** (C-9). There are no components. D1.5–D1.8 are unchanged, and apply to every case. A combination has no loads of its own, so G-B is unchanged.

**Where each part is enforced** (lane A, `PP/retained_memory.rs` outside the generated block):

| Part | Site | Change |
|---|---|---|
| 1 ≤ c ≤ 3; no components | `family_clauses` D1.4 | Unchanged |
| z ≠ 0 refused | `family_clauses` D1.4 (`FamilyFact::Combinations`) | **Removed** |
| Combination ids disjoint from load-case ids | `family_clauses` D1.4 | **New** family fact (suggested `CombinationIds`) |
| z ≤ 2 | `cap_rows` D1.9 | **New** row `Combinations` (cap `COMBINATIONS` = 2: c ≥ 1 and C_eq ≤ 3) |
| Typed capacity of `combinations` | `cap_rows` | `CombinationsCapacity`: cap **0 → 2** |
| c + z ≤ 3 | `cap_rows` | **New** row `CaseEquivalents` (cap `CASE_EQUIVALENTS` = 3) |
| Terms of each mechanics combination | `cap_rows` | **New** `CombinationTerms` and `CombinationTermsCapacity` (cap `TERMS` = 3; maxima over combinations, as `Loads` is over cases) |
| Operand ids of each range envelope | `cap_rows` | **New** `RangeOperands` and `RangeOperandsCapacity` (cap 3) |
| Rows | `CAP_ROWS` | 47 → **53** |
| The census | `DomainFacts` | The typed census gains each combination's term and range-operand counts and capacities |
| The domain re-check | PP `w1_case_ids` (T-4) | The same predicate; a failure is `Domain` |
| G-C | `phase_caps` | `EnvelopeResults ≤ C_eq·P_final` (numerically today's 3·P_final) and `RetainedErrorTextBytes ≤ C_eq·(3m + 1)·Text(err)` (PLAN §1.2.4) |
| Out-of-domain oracles | the law tests and the runner's literal and tie test | Re-based to C_eq + 1 (for example c = 3 with z = 1, and c = 1 with z = 3), h = 4 and 4 range operands, as PLAN_v2's SF-2 did |

With z ≥ 1, c ≤ 2. So a combination's distinct cases, and a range's distinct operands, are at most 2, and the term and operand caps bind only on repeated or unknown ids, which validation blocks or makes rowless (§0 item 3). They are kept because they bound the typed arrays that admission prices before validation, and they cost nothing. Subtraction has exactly two operands and needs no row.

**N-4's reason that C1's invocation limit stays non-binding** (REV N-4; RV114 N-4). C1 §2 gives each Run, case or combination, its own Lc = 20,000,000,000 and the invocation Li = 60,000,000,000 = 3·Lc. With C_eq ≤ 3 there are at most three Runs: at most c case Runs in the batch and at most z combination Runs, and a pre-source refusal makes none. So Li can stop a Run before that Run's own Lc only if earlier Runs overshot their own Lc. Thresholds admit a recorded overshoot (C1 §2), and then only within the sum of those overshoots: the third Run's invocation room is at least Lc − (o₁ + o₂). There is no fourth Run to be invocation-exhausted. A fourth case-equivalent would need either that exhaustion or a new work policy, which PTABLE's `receipt_bindings` now binds.

## 10. Gate and code placement, and 07o (brief item 10; decision 20)

### 10.1 Every new check, by gate (all three readers, identical order and codes; each appended after the gate's existing checks)

**Three existing conjuncts are widened rather than appended.**
- G3's row-basis check: a load case only, becoming a case or a combination entry.
- G4's "a `RETAINED_PRECISION_*` diagnostic names exactly one case", becoming one case or combination.
- G8's refusal of any model combination, which is removed.

At z = 0 each is the same predicate as today, so no 07n outcome can change. R5's census over 07n checks this in each lane.

| Gate | New checks, in order | Code |
|---|---|---|
| G0 | §8 rows 4, 6, 8 and 9 | as §8 |
| G1 | Shapes (§6.1). A `retained_selected` combination's `source_identity_sha256`; each `CombinationSource.operands[].source_identity_sha256` = the recomputed identity at its `source_ref`; operand-preparation hashes (C3a-5) | `RETAINED_PRECISION_RECEIPT_MISMATCH` |
| G2 | Encodings of the new members (U, Bits, Hash) | `RETAINED_PRECISION_ENCODING_MISMATCH` |
| G3 | (a) One entry per gate-evidence entry, in the same order and with the same ids (no invocation needed). (b) Case ids and combination ids pairwise distinct (one id set). (c) Every row's basis names a case (`load_case`) or a combination entry (`combination`); a combination's `result_ids` are exactly its rows, in publication order; the sets are disjoint and exhaustive. (d) C3a's coverage (C3a-7). (e) `product_attempts[]`: case attempts first, then combination attempts; every non-null `product_attempt_ref` (case or combination) names a distinct attempt whose owner is that entry; a combination attempt exists exactly for the entries with a non-null `run`. (f) `execution_order`: the case Runs, then the combination Runs in Call order, each `{kind, index}` naming the entry whose `run.id` is its position. (g) `sources[]`: each CaseSource is named by exactly one case or one operand preparation; each CombinationSource by exactly one combination entry | `RETAINED_PRECISION_COVERAGE_MISMATCH` |
| G4 | One `RETAINED_PRECISION_SELECTED` per `retained_selected` and one `RETAINED_PRECISION_UNAVAILABLE` per `retained_unavailable` combination, each with `affected_refs` exactly `[combination id]`; none naming an `ordinary` or `base_withheld` combination; an unavailable combination's `diagnostic_ref` is its diagnostic's id; every `RETAINED_PRECISION_*` diagnostic names exactly one case or combination | `RETAINED_PRECISION_DIAGNOSTIC_MISMATCH` |
| G5, ordinary class | **The disposition rule:** `base_withheld` ⇔ the gate entry is withheld, with an equal `reason`; retained ⇔ mechanics with distinct term cases, at least one `selected`, and not withheld; otherwise `ordinary`. A `base_withheld` expression is mechanics. Each combination's `diagnostic_refs` follow D6a's rule | `RETAINED_PRECISION_ATTEMPT_MISMATCH` |
| G5, native class | Call ids consecutive; `calls[0]` is the case batch, then the mechanics Calls in authored order of the entries with a non-null `call_ref`, each naming its entry. `requested_operands` = the expression's terms (factor bits; `source_ref` per §2.5). `runs` ⇔ one source and one Run, `pre_source_refusal` ⇔ none. The combination Run's origin. The `CombinationSource` structure: `case_index` and factor per term, `representative_source_ref` = `operands[0].source_ref`, each operand's CaseSource owned by its case, `stiffness_sha256` equal to every operand's, `ledger_sha256` = `selection.ledger_sha256` when selected. A combination Call's Group is a `CombinationGroup` and a batch's is a `Group`. Imports: only from `selected` operands (the term's case `selected`, `selected_run` its Run), the first occupied slot in authored order, each build existing and backward, `cache_before` = the imports, no other cross-group reuse | `RETAINED_PRECISION_ATTEMPT_MISMATCH` |
| G5, native class (deferred WORK) | The meter chain (each Call's before = the previous after; `charged` = the last Call's after); `pre_source_refusal` with after = before; the combination Run's charge equations as a case Run's | `RETAINED_PRECISION_WORK_MISMATCH` |
| G5, products | `retained_unavailable` reason table: no-Call causes ⇔ `call_ref`, `run`, `source_ref` and `product_attempt_ref` all null. `operand_source_unavailable`: the named term is the first whose case is `unavailable` with no CaseSource, or with a Run refused `ledger_unavailable`. `operand_preparation_failure`: the named record is refused and owned by the first `not_required` term with a refused record, and no term qualifies for `operand_source_unavailable`. A `CombinationReason` cause ⇔ the Call's `pre_source_refusal` reason, with `call_ref` non-null and the rest null. With a Run: §4's reason table. Combination attempts: stages, result and checks as C3 without a preparation stage; Ready ⇔ `retained_selected`. Operand preparations: C3a-7 | `RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH` (work equations: `WORK_MISMATCH`) |
| G5a–G5c | A `retained_selected` combination is one more numeric owner: its rows, its own `selection` (S\* from its own rows), the representative source's maps and sections, and its own prescribed DOFs. Classes as a case's | The adopted scale, section, class and input-DOF codes |
| G5c, R-COMB-1 (R) | The appended `not_covered` classifications (§5). No failure code: a derivation, not a check | — |
| G6 | `recovery_method` exactly on the rows of `retained_selected` combinations; on no other combination row | `RETAINED_PRECISION_ROW_METHOD_MISMATCH` |
| G7 | Unchanged base validator: the projection removes `recovery_method` from every row; T0R's gate evidence and the combination magnitude checks (RE `combination_magnitudes`) run as today. The base validator never recomputes a combination's values from its operands' rows, so retained values pass it | Existing base codes |
| G8, invocation | Model combinations are admitted (today all three readers refuse any). The entries equal the invocation's `model.combinations` in order and id. Each expression equals its model combination: basis to kind; terms' `load_case` and factor bits (the JSON number parsed to binary64), authored order, repeats; subtraction minuend and subtrahend; range ids sorted in UTF-8 byte order; mode | `RETAINED_PRECISION_INVOCATION_MISMATCH` |
| G8, preparation | K4CMB recomputed from the operands' recomputed K4SRC bytes and factor bits, equal to `kernel_source_sha256`; R-8's operand equality (§2.5); the combination attempt's `material_basis_ref` = every operand case's; C3a's preparation binding | `RETAINED_PRECISION_PREPARATION_MISMATCH` |

**Not recomputed by any reader:** `ledger_sha256`, which would need exact integer sums of c·v in three languages. It is bound as attestation, as a case's `selection.ledger_sha256` is today (C-12; KD §4 made this optional). The FK oracle (KD §6, with RV115 SF-3's 54-bit product) checks K4LED independently.

### 10.2 07o: bases and must-pass entries

**Bases** (one writer, append-only after 07n):

| Base | Content | Provenance label |
|---|---|---|
| W-CB1 ×2 modes | A + B, both cases selected → `retained_selected` | producer-solved |
| W-CB2 ×2 | A selected, B `not_required` (one OperandPreparation), A + B at Ceiling → `retained_unavailable`, `combination_unresolved`, phase `kernel` | producer-solved (predicted by W-C2's case C; B2-W) |
| W-CB3 ×2 | A mixed combination with a prepared operand → `retained_selected` | producer-solved if the probe selects; otherwise a labelled synthetic (PLAN §6 item 7) |
| W-CB4 ×2 | A − B and range(A, B) over a selected A → `ordinary`, R-COMB-1 rows | producer-solved |
| W-CB5 ×2 | 2·B with B `not_required`, beside a selected A → `ordinary`, base rows | producer-solved |
| `b2_base_withheld` | A gated mechanics combination | "synthetic: not producer-emittable in B2's domain" |
| `b2_pre_source_refusal` | A Call with `operands_differ` | "synthetic: not producer-emittable in B2's domain" (a test hook may also emit it) |
| `b2_operand_preparation_failure` | W-CB2 with its operand preparation refused | hook-produced (`fail_operand_preparation`), labelled |
| `b2_operand_source_unavailable` | A combination over a case whose T-7 failed | hook-produced (B1's `preparation_of_case`), labelled |

**Must-pass:** one per base, with `expected_classifications` including the combination rows, by G5c or R-COMB-1. Expected standing: `eligible` where every case is `selected` or ordinarily-eligible `not_required` (W-CB1, W-CB3, W-CB4, W-CB5 and W-CB2 when B qualifies); `needs_recompute` for the hook bases with an unavailable case.

### 10.3 07o mutations, with their designed first failure

"Designed" is the contract's gate and code, identical in all three readers once B2's readers land; SC2 establishes the actual ones. "Today" is what NUM's and `b1-r`/`b1-p`/`b1-t`'s readers do with main's SCHEMA, read from their code:
- **a base with a `CombinationAttempt`** (W-CB1, W-CB2, W-CB3) fails first at **G0 `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`** in all three, because every attempt's `definition_id` must equal DEF-O's (RS `g0`, PY `_validate_draft`, TS `header`);
- **any other base with a combination entry** fails at **G1 `RECEIPT_MISMATCH`** (`combinations` has `maxItems 0`), and so does `operand_preparations` (an unknown member);
- **a mutation of an existing base** is as listed.

At J1, before B2's readers, the first group still fails at G0, and the second passes G1 and fails at G3, which refuses a non-`load_case` row (RS `g3`, PY `_validate_draft`, TS `coverage`).

| # | Base | Edit (rehash all unless noted) | Designed first failure | Today |
|---|---|---|---|---|
| m1 | W-CB1 | combination attempt's `definition_id` → an unknown id | G0 `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` | G0 |
| m2 | W-CB2 | operand preparation's `definition_id` → an unknown id | G0 `…UNSUPPORTED` | G0 |
| m3–m7 | `ordinary_prepared_synthetic` | `projection_policy`, `work_policy`, `canonicalization`, `work.case_limit`, `work.invocation_limit` each changed | G0 `…UNSUPPORTED` | **G0 `…UNSUPPORTED`, all three** |
| m8 | W-CB1 | a case attempt's `definition_id` → DEF-C's id | G1 `RECEIPT_MISMATCH` | G0 |
| m9 | W-CB1 | the combination attempt's `definition_id` → DEF-O's id | G1 | G1 (every attempt then carries DEF-O's id, so G0 passes) |
| m10 | W-CB1 | the combination's `source_identity_sha256` changed, body rehashed | G1 | G0 |
| m11 | W-CB1 | `CombinationSource.operands[1].source_identity_sha256` changed | G1 | G0 |
| m12 | W-CB2 | the operand-prepared CaseSource's `preparation.sha256` made under `retained_precision_preparation_v1` | G1 | G0 |
| m13 | `two_case_synthetic` | `operand_preparations: []` added | G1 | **G1, all three** (unknown member) |
| m14 | W-CB4 | an `ordinary` entry gains `run: null` | G1 | G1 |
| m15 | `b2_base_withheld` | `result_ids` gains a row id | G1 | G1 |
| m16 | W-CB1 | a term's factor bits in uppercase hex | G2 `ENCODING_MISMATCH` | G0 |
| m17 | W-CB1 | a requested operand's factor → `7ff8000000000000` | G2 | G0 |
| m18 | W-CB4 | the two entries swapped | G3 `COVERAGE_MISMATCH` | G1 |
| m19 | W-CB4 | one entry removed | G3 | G1 |
| m20 | W-CB4 | a combination renamed to a case id throughout (rows, evidence, diagnostics) | G3 | G1 |
| m21 | W-CB1 | `result_ids` loses a row | G3 | G0 |
| m22 | W-CB1 | `result_ids` gains a case row | G3 | G0 |
| m23 | W-CB1 | two `result_ids` swapped | G3 | G0 |
| m24 | W-CB1 | `execution_order` loses the combination Run | G3 | G0 |
| m25 | W-CB1 | an `execution_order` combination index changed | G3 | G0 |
| m26 | W-CB1 | the combination attempt moved before the case attempts (ids renumbered) | G3 | G0 |
| m27 | W-CB2 | the operand preparation's owner → the selected case (C3a mutation 1) | G3 | G0 |
| m28 | W-CB2 + an ordinary combination | `requested_by` names the ordinary combination (C3a mutation 2) | G3 | G0 |
| m29 | W-CB2 | the operand preparation removed (its source too) | G3 | G0 |
| m30 | W-CB2 | the operand preparation duplicated | G3 | G0 |
| m31 | W-CB1 | the combination's selected diagnostic removed | G4 `DIAGNOSTIC_MISMATCH` | G0 |
| m32 | W-CB2 | `diagnostic_ref` → another id | G4 | G0 |
| m33 | W-CB4 | a `RETAINED_PRECISION_SELECTED` naming the ordinary combination added | G4 | G1 |
| m34 | W-CB1 | the combination diagnostic's `affected_refs` → [combination, case] | G4 | G0 |
| m35 | W-CB5 | a term of the `ordinary` combination → the selected case A (expression only) | G5 `ATTEMPT_MISMATCH` (disposition) | G1 |
| m36 | W-CB4 | the subtraction entry recast as `retained_unavailable` (`operand_source_unavailable`) | G5 `ATTEMPT_MISMATCH` | G1 |
| m37 | `b2_base_withheld` | `reason` → another gate code | G5 `ATTEMPT_MISMATCH` | G1 |
| m38 | W-CB1 | `requested_operands` swapped | G5 `ATTEMPT_MISMATCH` | G0 |
| m39 | W-CB1 | `representative_source_ref` → operand 1's | G5 `ATTEMPT_MISMATCH` | G0 |
| m40 | W-CB1 | `operands[0].case_index` → the other case | G5 `ATTEMPT_MISMATCH` | G0 |
| m41 | W-CB1 | an import taken from operand 1 where operand 0 had the slot | G5 `ATTEMPT_MISMATCH` | G0 |
| m42 | W-CB1 | the combination group's `imports` removed (it then parses as a case group) | G5 `ATTEMPT_MISMATCH` | G0 |
| m43 | W-CB1 | the combination Call's `invocation_before` ≠ the batch's after | G5 `WORK_MISMATCH` | G0 |
| m44 | W-CB1 | `work.charged` = the batch Call's after | G5 `WORK_MISMATCH` | G0 |
| m45 | W-CB3 | the group imports a slot from the prepared operand (C3a mutation 4) | G5 `ATTEMPT_MISMATCH` | G0 |
| m46 | `b2_pre_source_refusal` | `run_refs` gains a Run id | G5 `ATTEMPT_MISMATCH` | G1 |
| m47 | `b2_pre_source_refusal` | the Call's after ≠ before | G5 `WORK_MISMATCH` | G1 |
| m48 | `b2_operand_preparation_failure` | `call_ref` null with a `CombinationReason` cause | G5 `PRODUCT_ATTEMPT_MISMATCH` | G1 |
| m49 | `b2_operand_source_unavailable` | `operand_index` → the selected term | G5 `PRODUCT_ATTEMPT_MISMATCH` | G1 |
| m50 | `b2_operand_preparation_failure` | the refused record gets a `source_ref` (C3a mutation 3) | G5 `PRODUCT_ATTEMPT_MISMATCH` | G1 |
| m51 | W-CB2 | the operand preparation's `stage` → `failed` while `prepared` | G5 `PRODUCT_ATTEMPT_MISMATCH` | G0 |
| m52 | W-CB2 | the combination attempt's native → `completed` (the Run is unresolved) | G5 `PRODUCT_ATTEMPT_MISMATCH` | G0 |
| m53 | W-CB2 | `reason.code` → `kernel_unresolved` | G5 `PRODUCT_ATTEMPT_MISMATCH` | G0 |
| m54 | W-CB1 | the combination's body-scale translation bits off by one ulp | G5b scale code | G0 |
| m55 | W-CB1 | a combination row id added to `selection.not_covered` | G5c `CLASSIFICATION_MISMATCH` | G0 |
| m56 | W-CB2 | `recovery_method` on a combination row | G6 `ROW_METHOD_MISMATCH` | G0 |
| m57 | W-CB1 | `recovery_method` removed from a combination row | G6 | G0 |
| m58 | W-CB4 | `recovery_method` on an ordinary combination row | G6 | G1 |
| m59 | W-CB4 | the range `mode` changed | G8 `INVOCATION_MISMATCH` | G1 |
| m60 | W-CB4 | the subtraction's minuend and subtrahend swapped (rows untouched) | G8 `INVOCATION_MISMATCH` | G1 |
| m61 | W-CB4 | the range `operand_ids` reversed | G8 `INVOCATION_MISMATCH` | G1 |
| m62 | W-CB1 | `kernel_source_sha256` replaced (identities resealed) | G8 `PREPARATION_MISMATCH` | G0 |
| m63 | W-CB3 | one operand CaseSource's `effective_wall` changed (identities resealed) | G8 `PREPARATION_MISMATCH` | G0 |
| m64 | W-CB2 | the operand preparation's `old_facts` D changed | G8 `PREPARATION_MISMATCH` | G0 |

Every row needs its base. Mutations on W-CB3 move to a labelled synthetic base if the probe does not select (PLAN §6 item 7). Where a mutation reaches a different reader-shared first failure, SC2 fixes it from the readers and declares any difference to ROOT (decision 20).

### 10.4 Reader-local unit tests, and T6S (decision 21; N-8)

- **Each reader:** §8's G0 table tests (11 per language); the reason-table rows for combinations; K4CMB recomputation against FK's test vectors where a lane has them.
- **T6S, tests only (N-8):** the TS lane adds tests on a combination successor in `StressNeutralExportPanel.test.tsx` (`basisReference` for combination rows; their withheld witnesses) and `analysisRunCompatibility.test.ts` (`sourceBasisReference`), and the T6S suites run in the full suite.
  - There is no source edit.
  - More rows carry D-U6-2's existing codes, with no new code or text, so no disclosure meaning changes. A changed meaning would go back to ROOT (I74 §4.3).

## 11. J1's package: three more edits than REV §1.4 (for ROOT)

REV §1.4 item 5 limits J1's reader edits to "PTABLE's hash cascade, constants only, in the 12 files". These also bind PTABLE's content and fail once DEF-C joins `product_formation_definitions`:

| File (owner lane) | Site | What fails without it | J1's edit (mechanical) |
|---|---|---|---|
| RS `RE/src/retained_precision.rs` | `g0`: `table()["product_formation_definitions"] == json!([{DEF-O}])` | **Every successor's precommit G0 `FORMATION_MISMATCH`:** every c = 1 and B1 pin falls back, which is J1's own acceptance | The constant list gains DEF-C's id and H, and DEF-C's file is packaged and its H checked as DEF-O's is (§8 row 4) |
| TS `retainedPrecision.ts` | `header`: `same(table.product_formation_definitions, [one])` | Every TS suite case at G0 | The same |
| PP `retained_wire_tests.rs` | `u1_constants_bound_to_in_tree_fixtures` | That test | Its list assertion gains DEF-C |

PY does not compare the list (only DEF-O's H and the table hash), so its `TABLE_HASH` suffices.

**Also confirmed for J1** (checks §6.3): `refusal_kinds_are_the_schema_preconditions` and `test_all_object_definitions_are_closed` hold on the J1 text, and every existing receipt validates. **J1 also adds** DEF-C to `REVIEWED_INPUTS` (14 → 17 with B3b's two), with new hashes SCHEMA `abf3225c…`, PTABLE `863738f1…` and DEF-C `03d40598…`.

## 12. Decisions (brief item 11)

None is owner-held. Each is ROOT's.

| # | Decision | Recommendation | Alternatives |
|---|---|---|---|
| C-1 | The retained trigger (decision 5, one conjunct added) | **Mechanics, gates pass, distinct term cases, ≥ 1 selected.** A repeated-case mechanics combination is `ordinary`: the ordinary route publishes none of its rows | (a) Retain it anyway: a kernel run, then a certain coverage failure. (b) Refuse repeats in D1.4: the whole invocation loses W1 |
| C-2 | Combination unavailability codes | **`combination_unresolved`** (phase `preparation` without a Run, `kernel` with a non-selected Run); **`facade_certificate`** after a selected Run; cause `prepared_product_failure` naming the CombinationAttempt when a Run exists | Mirror cases exactly (`kernel_unresolved`, `kernel_refused`), against DN §4.2's and PLAN W-CB2's `combination_unresolved` |
| C-3 | The combination's product attempt | **A separate closed `CombinationAttempt`**, existing iff a Run exists; `product_attempts[]` a union | Widen `ProductAttempt` (REV §1.1 item 3's letter): it would need fabricated `ordinary_attempt_ref`, `preparation` and `operational` values, or nullable members on the case shape |
| C-4 | An operand case with no usable source | **New cause `operand_source_unavailable {operand_index}`**, no Call, checked before any preparation is requested (so it takes precedence over `operand_preparation_failure`) | Reuse `prepared_product_failure` naming the operand's attempt (ambiguous with the combination's own); or abandon the successor (an availability loss for one combination) |
| C-5 | Where B2's steps sit | **T-10a and T-10b after T-10, with the z ≥ 1 extensions of §2.2** | Freeze all Calls' Runs after the last Call: same meter, larger live set |
| C-6 | `operand_preparations` when empty | **Absent** (`minItems 1` when present), so no B1 or c = 1 byte changes | C3a's "empty if none": every existing receipt's bytes change, a PLAN risk 5 stop |
| C-7 | R-COMB-1's representation | **Reader-derived** (§5) | A receipt member `row_class` |
| C-8 | `formation_warrant` with two definitions | **A list of two XTABLE-shaped warrant objects** | One object with `definition_ids: [..]`; or a second member `combination_formation_warrant` |
| C-9 | Case and combination ids | **Disjoint, by D1.4** (an overlap falls back) | Qualify diagnostic ids and every match by `ref_type` in the producer and three readers |
| C-10 | T-8's capacity | **Declared maxima from the gate-admitted, distinct-case mechanics combinations with a term in the batch, and their `not_required` cases** | Exact counts are not known at T-8; a second `RecordedInvocation` would split the one meter |
| C-11 | C1 §5's `operand_cases` and `cache_inputs` | **Not serialized** (C2 §3–§4 supersede them) | Serialize the derived views: duplicates to forge consistently |
| C-12 | Reader recomputation | **K4CMB recomputed** (concatenation only); **K4LED bound as attestation** | Recompute K4LED in three languages: exact integer sums of c·v up to about 4,200 bits; estimated +2–4 h per reader |
| C-13 | Combination diagnostics | **Case ids' pattern with the combination id; the two messages of §2.7; after the case diagnostics, in authored order** | A separate pattern (`…:combination:<id>:…`), unnecessary under C-9 |
| C-14 | The classification summary | **Per case, unchanged** | Per-combination entries: a T6S-visible shape change, and more reader files |
| C-15 | DEF-C's hash domain (R-10) | **Reuse `retained_precision_formation_v1`** | A new domain (H `562cbe14…` shown for comparison) |
| C-16 | A combination's origin refusal (N-5) and the new abandonment cases | **Whole-successor abandonment via `W1Fallback::CombinationCustody`**; never a capture error | Map to a per-combination unavailability: it would need a wire tag for an invariant failure |

**Unchanged and restated:** decisions 5–10 as PLAN and REV word them (with C-1); 20 (07o); 21 (T6S tests only); 28–31 as selected.

## 13. Collision log and the names to reserve (brief item 11)

`_run_records/b2c_collisions.out.txt`: `git grep -l -F` at NUM `cebff253d6`, outside `P/execution`. Wire values are searched as quoted JSON strings.

| Name | Kind | Hits | Note |
|---|---|---|---|
| **`RP-PREPARED-COMBINATION-DUAL-v1`** | definition id | 0 | **Reserve** |
| **`retained_precision_prepared_combination_v1`** (`.json`) | file | 0 | **Reserve** |
| `retained_precision_formation_v1` | hash domain | (DEF-O's) | Reused (C-15) |
| `receipt_bindings` | table member | 0 | Already ruled at B3D-8; PTABLE uses it identically |
| **`"operand_source_unavailable"`** | cause kind | 0 | **Reserve** (C-4) |
| **`"no_selected_operand"`** | `CombinationReason` tag | 0 | **Reserve** (R-3's wire tag) |
| `"mechanics_combination"`, `"pre_source_refusal"`, `"operand_validation"`, `"combined_preparation"`, `"retained_selected"`, `"retained_unavailable"`, `"base_withheld"`, `"no_retained_mechanics"`, `"combination_unresolved"` | C1/C2 wire values | 0 each | **Reserve** (C1/C2 proposed them; none was ever reserved). `mechanics_combination` and `combination_unresolved` occur only as identifier substrings or FK comments |
| `"requested_operands"`, `"representative_source_ref"`, `"combination_index"`, `"call_ref"`, `"operand_preparation_ref"`, `"selected_run"` | C1/C2/C3a members | 0 each | **Reserve** |
| `"combination_id"`, `"result_ids"`, `"disposition"`, `"expression"`, `"operand_index"`, `"imports"`, `"purpose"`, `"minuend_id"`, `"subtrahend_id"`, `"operand_ids"`, `"terms"`, `"factor"`, `"ledger_sha256"` | members | in use elsewhere | Same meaning or unrelated documents: `combination_id` is the gate evidence's key; `operand_index` is in SCHEMA's `facade_failure`; the model's own `minuend_id`, `subtrahend_id`, `operand_ids`, `terms` and `factor`; `ledger_sha256` is `Selection`'s. No conflicting wire meaning |
| `"result_state_subtraction"`, `"range_envelope"`, `"mechanics"` | expression kinds | in use | The model's own basis values, same meaning |
| `C3a`'s spellings (`operand_preparations`, `OperandPreparation`, `operand_preparation_failure`, `retained_precision_operand_preparation_v1`, `combination_operand`) | — | 0 (`combination_operand` as before) | Already reserved (RR "B0 selected …") |
| `CombinationAttempt`, `CombinationGroup`, `CombinationSource`, `MechanicsCombinationCall` | `$def` names | 0 | Reserve with SCHEMA |
| `Combination`, `CombinationExpression`, `CombinationReason` | `$def` names | `Combination`: common word; the other two are a private PP enum and FK's enum | Schema-internal names, not wire values; `CombinationReason` deliberately names FK's enum it translates. ROOT may prefer other spellings |
| `CombinationCustody`, `CaseEquivalents`, `CombinationTerms`, `RangeOperands`, `CombinationIds`, `COMBINATION_SELECTED_MESSAGE`, `COMBINATION_UNAVAILABLE_MESSAGE`, `COMBINATION_DEFINITION_ID`, `COMBINATION_DEFINITION_HASH` | internal Rust/PY/TS names (suggested) | 0 each | Lane-internal; no reservation needed |
| `retained_precision_combination_formation_v1` | alternative domain | 0 | Only if C-15's alternative is chosen |

## 14. Estimates (brief item 12; refining PLAN §2.2 and REV §7)

Agent hours, without repair rounds. B2-K is unchanged at 19–30 h (RR "RV115 …").

| Slice | PLAN | Refined | What moves it |
|---|---|---|---|
| **B2-A** | 4–7 | **5–8** | 6 new cap rows and a capacity re-cap; the census's term and operand counts; the id-disjointness clause; G-C's two facts; out-of-domain oracles at C_eq + 1, h = 4 and 4 range operands; law tests and mutants |
| **B2-P** | 20–30 | **26–38** | T-6′/T-9′ row blocks and scopes (1–2); T-8′ capacity (0.5); T-10a with gate evidence and C-1 (1–2); operand preparations with registration (3–5); operand sources, rebuilds and the ordinal mapping (2–3); Calls, Runs and the combination freeze scope (4–6); the serializer's members and meter chain (5–7); staging, headlines and diagnostics (1–2); hooks (1–2); W-CB1 to W-CB5 pins in both modes (3–5); mutants (3–4) |
| **B2 readers** | 33–47 | **39–55** | RS 16–23, PY 11–15, TS 12–17. Each: G0's table read and its 11 unit tests (2–3); G1 hashes (1); G3 (2–3); G4 (1); G5's native class for combination Calls, Groups, imports and the meter chain (3–4); the disposition rule (1); C3a's and the combination attempts' G5 (2–3); G5a–G5c's combination owner (2–3); G6 (0.5); G8's expression, K4CMB, R-8 and C3a binding (3–4); R-COMB-1 (1); the census over 07n and the D38-style audit (1–2). PY and TS less where RS's text transfers |
| **SC2, B2's part** | in SC2's 8–12 | **7–10** of a revised SC2 of 10–14 | 14 bases and must-pass entries (10 producer-solved, 2 synthetic, 2 hook-produced); 64 mutations with first failures from three readers |
| J1's package (I-A) | 1–2 | **1.5–2.5** | §11's three edits added |

**Net against PLAN's rows:** about +12–19 h agent. Review is about +1–2 h for RV-R2, which now has more checks per gate.

## 15. What I read, execution record and limits

**Read (sha256):**

| Input | sha256 |
|---|---|
| The brief | `a604a2c3…4c6a` (full hash above) |
| `AGENTS.md`; `agents/AGENT_TASK.md` | `f96feb19…3977`; `1a13a5b0…c8fb7` |
| PLAN; REV | `e1147dbd…238a`; `63abb73f…bba0c` |
| RV114 REVIEW | `bc6918ce…a9ee6` |
| DESIGN (whole) | `5933b90b…1114` |
| C1 (whole); C2 (whole); C3 (whole); SC1 (whole) | `c8ab2318…67e3`; `923da0b9…0869`; `fd00d2c1…292e`; `28bc4dd6…4054` |
| KD (whole) | `4e8c33a4…bed3` |
| RV115 REVIEW (verdict, findings, §1.4, §3, §7) | `0f7ab77d…be93c` |
| B3D DESIGN; REVISION_01; RV116 REVIEW; ADDENDUM_01 | `ad7942f6…b69b`; `6f5b1a6d…e542`; `001b7a32…f51d`; `e10ddc52…8ddf` |
| B3D's `b3d_statics.py`; `b3d_statics_r1.py`; XTABLE; `SCHEMA_ENUM.diff` (r1) | `513a8b82…266c`; `dcd8e20d…1eb3`; `c4987e87…0a3d`; `b1597c7b…f39e` |
| RV109 round 2 REVIEW (§0–§2) | `206fd360…a43bc` |
| DN (§4.2) | `fb62ef4a…7a74` |
| D2 (§4.9.4) | `993f5f3a…4c8d` |
| I74 PLAN (§4.3) | `0350c918…2ed9` |
| RR (the sections named under Basis) | `71f1de09…697a` (whole file, as read) |
| PTABLE; DEF-O; SCHEMA; CORPUS (at NUM) | `c74742ce…c6a8`; `3e0779a4…e296`; `07951eda…b61c`; `c21112fd…6807` |

**Code read.**
- **At NUM:**
  - FK `origins.rs` (`OriginCapacity`, `RecordedInvocation::{new, solve_cases, solve_combination}`, `CallResult`, `NativeOwner`) and `combine.rs` (`CombinationReason`, `validate_operands`, `prepare_operands`);
  - FK `adaptive.rs` `CasePrep::combination` (K4CMB), `ledger.rs` (`LedgerRefusal`) and the exact sum's `SumError`;
  - PP `preview_physics.rs` (`append_combination_results`, `evaluate_values`), `lib.rs` (`CombinationExpression`, `maximum_across_cases`), `validation.rs` (the combination validators, `detect_duplicate_ids`), `retained_wire_tests.rs` (`u1_constants_bound_to_in_tree_fixtures`) and `retained_memory_law_tests.rs` (`refusal_kinds_are_the_schema_preconditions`);
  - the load-case algebra's `evaluate_linear_combination`;
  - RE `preview_physics_evidence.rs` (§9 combination rows, `headline`, `combination_magnitudes`), `semantic_contract.rs` (`retained_row_classes`, `retained_standing_from`, `classification_summary_from`) and `derivative.rs` (`class_disclosure`);
  - PY `tests/test_retained_precision_schema.py`.
- **On `b1` (`603e238517`):** PP `lib.rs` (`W1Fallback`, `ReservedNotices`, `CaseSet`, `w1_case_ids`, `retained_w1`, `w1_transaction`); `retained_product.rs` (`AttemptEnd`, `PreparedCases::{native, freeze, staged_envelope, selected_attempts}`, `prepared_custody`, `bind_case_rows`, `case_scope`, `CaseScope`, `prepare_cases`, `native_call`, `stage_headlines`); `retained_wire.rs` (`serialize_cases_with`, `serialize_attempt`, `invocation_arrays`, `run_value`, `selected_case_envelope`, `legacy_source`, `finish`, `bind_preparation`); `retained_memory.rs` (`caps`, `family_clauses`, `cap_rows`, `FamilyFact`, `CapFact`).
- **The readers:** RS on `b1-r` (`g0`, `g1`, `g3`, `g4`, `g5_native`, `reason_table`, `d38_capture_before_run`, `g8`, `validate`); PY on `b1-p` (`_shape`, `_encoding`, `_native_source_encoding`, `_validate_draft`); TS on `b1-t` (`header`, `integrity`, `coverage`).

**Executed** (Python 3.13.14 with VENV, `PYTHONDONTWRITEBYTECODE=1`, `TMPDIR` in scratch; `_run_records/RUN.md` has the commands with placeholders):
- `b2c_statics.py`: two runs into scratch, all six outputs byte-identical. The four statics and the two diffs in `statics/` are those bytes, and `b2c_statics.out.json` is that run's report.
- `b2c_checks.py`: two runs, outputs byte-identical. It loads NUM's committed PY reader file (no repository write) only to use its G1 walker.
- `b2c_collisions.sh`: two runs, identical.
- **A disclosed slip:** one plain-text substitution in my own scratch copy of `b2c_checks.py` (its argument list) was made with the host's `python3` rather than VENV, before any check ran. No record, static or check output was produced by it.

**Limits.**
- **Nothing was compiled or run in Rust or TS.** Every statement about producer or reader behaviour is read from code. In particular:
  - the B1 sites of §2.2, and the duplicate-term rowlessness of §0 item 3, are code readings. B2-P's pins establish them;
  - the "today" first failures of §10.3 are readings of the readers' code and SCHEMA's `maxItems 0`, not runs. SC2 establishes the actual ones.
- **The schema checks are shape checks only** (`jsonschema` and PY's walker), not the readers' full G0–G8.
- **B1 is still moving** (I3's pins). The contract cites B1 by symbol; a later B1 change to a cited site is for ROOT to re-read before J0.
- **The draft statics follow my recommendations** (C-1, C-3, C-6, C-8, C-15). A different ruling changes them; the generator regenerates every hash.
- **The record's screen.** The merged SCHEMA text and the two diffs contain SCHEMA's own `$id`, whose host label is the repository's schema namespace ending in the dot-local suffix. It is main's existing identifier, not a machine name, so ROOT's dot-local screen will report it there. No other hit is expected (§ "Screen" in `_run_records/RUN.md`).
