# I97 B2-C revision 01: RV118's amendments, RV115's notes and I98's witnesses

TASK (Type 2), I97, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC.

**The brief:** `R/BRIEFS/B2C_REVISION_01.md`, sha256 `2afb59046e2b96c3e11b8474d53038e8584f879c95e9992ce5619965b03f2b74`, verified before reading.

**The basis, verified before reading:**

| Record | sha256 |
|---|---|
| My `R/I97/b2_c_01/CONTRACT.md` | `165cd4b1c0b5ed3f83f9230435c0a3ab2284211c90908daa647610bba81d9d28` |
| RV118's `R/REVIEW_RV118/b2_c_01/REVIEW.md` | `fe640ca5fb312440a8766762bcd0cf10b49997b3c37ef9a0f6924fda77989b0a` |
| RV115's `R/REVIEW_RV115/b2_kd_01/ADDENDUM_02.md` | `7cd7a1b4ed2e2295f6b696315c441301b7faeffe7561d8db58acc32442e68bdf` |
| I98's `R/I98/b2_w_probe_01/PROBE.md` | `e224899a51aa0bee6cedebb678d581b69a6ab0eb77d566270676080706a1a51f` |

The specification is RR's three rulings:
- "RV118 (RV-C) accepts B2-C with amendments; B2-C ruled; C-1 to C-16 selected; I97 revises";
- "RV115 (RV-K) accepts DEF-C's numerical content";
- "I98's B2-W verified; …".

**What this record does.**
- **Sealed files are untouched:** CONTRACT.md, `SHA256SUMS`, `statics/` and `_run_records/`.
- **The new files:**
  - `statics/r1/` holds the two revised statics;
  - `_run_records/r1/` holds the revision's scripts, outputs and commands;
  - `SHA256SUMS.revision_01` lists them and this file.
- **How this revision relates to CONTRACT.md.** Where a section below says it replaces or amends a CONTRACT.md section, that text supersedes the contract's. Every part of CONTRACT.md it does not name stands as written, with C-1 to C-16 as RR selected them.

**Notation** is CONTRACT.md's. Code citations:
- **B1:** at `603e238517`. The B1 sites cited in PP `retained_product.rs` and `lib.rs` are unchanged at the `b1` head `03f55e7178` (`git diff --stat` is empty for both).
- **Readers:** at `b1-r` `6e3e4fe219`, `b1-p` `2843a59a16` and `b1-t` `6fa6a64658`.
- **NUM:** at `ba7bbae589`. Its maintained tree (`P/fixtures`, `P/schemas`, `P/core`, `P/tests`, `P/apps`) equals CONTRACT.md's basis `cebff253d6`, checked by `git diff --quiet`.

## 0. In brief

| Item | Amendment | Section |
|---|---|---|
| S-4 (a) | DEF-C's combination `displacement_magnitude` takes DEF-O's support-magnitude pattern: binary64 hypot(hypot(x,y),z) of the published components, certified. G7's 64ε guard then holds by construction, with a 16× margin across reader libraries | §1.1 |
| S-4 (b) | The combination's observables stage is defined: G7's `combination_magnitudes` guard and the support guard on its own frozen rows. Any failure is that combination's `facade_certificate` | §1.2 |
| S-3 | R-COMB-1 classes quantity rows `not_covered`. A `combination_modulus_basis_record` stays `non_quantity` | §2 |
| N-7 | PTABLE's `accuracy_classification.scope` states R-COMB-1 | §2 |
| S-1 | §2.2 gains the three T-2 capture hooks and `observables_view` at T-9 | §3.1 |
| S-2 | T-6′ follows the producer's layout, with records after every combination's rows. All 18 in-domain shapes hold, `[range(A), 2·A]` included | §3.2 |
| N-1, N-9, C-4 (N-2), N-5 | Any rebuild refusal leads to `CombinationCustody`. The freeze's maps and member facts come from operand 0's slot. C-4 is recorded as an amendment of C3a rule 4. G5 refuses an `origin` capture error | §3.3 |
| N-3, N-11, N-4 (m50), N-12, D6b | The reader branch points are listed. G3 checks the order of operand preparations and of operand-prepared sources. m50 stays at G5. A DEF-C id on an OperandPreparation is refused first at G1. D6b is case-only | §4 |
| N-6, N-8, N-10, N-14 | `formation_warrant` is not read by any code. J1's reader edits are constant lists only. The census reads combination strings, with the cap rows placed before `ControlBytes`. The `count_range` pair is reserved | §4.6 |
| S-5, the witnesses | W-CB1 is 1·A + 0.5·B, with A + B kept as a cancellation pin (W-CB1z). W-CB2 is `r7_cb2`, W-CB3 is `r7_cb3_v1`, W-CB4 splits into 4a and 4b, and there is a new c = 1 base. m28 is rebased | §5.1 |
| S-6 | 07o's rehash rule: seven steps in dependency order, with what each harness adds | §5.2 |
| 07o | Mutation rows corrected (m10–m12, m14, m18–m20, m28, m33, m36, m58–m61), and m65–m69 added | §5.3 |
| NB-1, NB-3 | Wording in DEF-C | §6 |
| N-15 | The estimates restated | §8 |

**The statics** (`_run_records/r1/b2c_statics_r1.py`, run twice, byte-identical):

| Static | sha256 | Status |
|---|---|---|
| **DEF-C r1, raw** (`statics/r1/retained_precision_prepared_combination_v1.json`, 10,863 B) | `6467c733ff5060afd3040172a5df91b12c228555254ae6d33d40555e06f30c03` | was `03d40598…` |
| **DEF-C r1, H(`retained_precision_formation_v1`)** | `0c43cf427b35d35e291e42382d242bca3762b61344b705744127c7c9b4372b6f` | was `9adf5178…` |
| **PTABLE r1** (`statics/r1/semantic_contract_v0_3_preview_physics_retained_1.json`, 53,719 B) | `791c0a0d9a06ce4effac5914b40872fa808ce3208dfe6c2c6d51359d6ff41573` | was `863738f1…` |
| `SCHEMA_B2.diff` | `ff3b88956ff399a50116b637fe2a9bf046271a54f2b842000ea85701672600d3` | **unchanged** (v0 `statics/`) |
| **The J1 SCHEMA text** | `abf3225ca431342dd785072a1baad7715b7e8c19afd15b5777feebf06d48669e` | **unchanged** (v0 `statics/`) |
| `SCHEMA_J1.diff`; B2-only SCHEMA | `f674370d…25d3`; `841b2c5d…b2da` | unchanged |

- **Why SCHEMA does not change.** It carries no definition or table hash (checked), and every amendment here is a reader rule or a static's text. N-12's refusal is already SCHEMA's (§4.4).
- **J1's set** is therefore `statics/r1/` (DEF-C, PTABLE) plus the v0 SCHEMA text and diff in `statics/`.
- **J1's REVIEWED_INPUTS hashes** become SCHEMA `abf3225c…`, PTABLE `791c0a0d…` and DEF-C `6467c733…`. The PTABLE cascade (RV114 §6's 12 files, constants only) is `c74742ce…` → `791c0a0d…`.
- **The alternative domain's H** (R-10's comparison) is now `82fccc74…`. C-15 stands.

## 1. S-4: the combination's magnitudes

### 1.1 (a) The displacement-magnitude recipe (replaces DEF-C `rows.displacement_magnitude`; CONTRACT §3)

**DEF-C r1's text, verbatim:** "finish the same node's frozen global_nodal_displacement_x/y/z rows; binary64 hypot(hypot(x,y),z) of their frozen raw values (mm), finite nonnegative and canonical +0, SI by the projection's mm rule; still certify dual physical norm and the unchanged 64-epsilon combination magnitude guard (base G7 combination_magnitudes), which the observables stage checks; not H of a lane norm hull".

It is DEF-O's `support_magnitude` pattern ("binary64 hypot(hypot(x,y),z), finite nonnegative and canonical +0; still certify dual physical norm and unchanged 64-epsilon component guard"), applied to a node's displacement. DEF-C's `support_magnitude` stays DEF-O's, byte for byte. So all three magnitude kinds that G7 checks on a combination are formed from the published components.

**Why G7's base guard holds by construction.** The guard is RE `combination_magnitudes` with `guarded` (`preview_physics_evidence.rs`), PY `_consistent_norm`, and TS's equivalent. It requires |p − r| ≤ 64ε·max(|p|, MIN_POSITIVE), where p is the published magnitude and r = hypot(hypot(x,y),z) of the same combination's published component values.
1. **The same inputs.** The recipe reads the frozen raw values that staging writes to the component rows' `value` (unit mm), the very bits the guard reads.
   - R-7 gives exactly one row per node and component in the combination's block, so the guard's `find`, which binds the last matching row, finds that row.
   - The guard and the recipe both use `value` alone. Rows publish no SI member.
2. **The same formula; only the library may differ.** If each hypot call is faithful (within 1 ulp, with an exact zero staying +0), then p and r each lie within about 2 ulps of the exact norm. So |p − r| ≤ 4ε|p| for normal results, and at most 4 subnormal ulps below MIN_POSITIVE. The guard allows 64ε|p|, or 64 subnormal ulps: **a 16× margin.**
   - **Checked** (`_run_records/r1/b2c_checks_r1.out.json` §s4_guard): 4,013 component triples, adversarial and random across the subnormal, normal and large ranges, signed zeros included. All 81 pairs of 1-ulp-perturbed evaluations per triple pass the guard. The largest |p − r| is 0.0625 of the allowance, and nested `math.hypot` is within 0.87ε of the exact norm (120 decimal digits).
   - RS uses the same Rust `f64::hypot` as the producer, so on one platform its recomputation equals p exactly. PY and TS fall under the bound above.
3. **Finite only.** A nonfinite hypot refuses the row. That is the combination's `facade_certificate`, never an abandonment.

**The ordinary route already forms magnitudes this way.**
- PP `append_combined_vector_magnitude` (`lib.rs`) publishes `vector[0].hypot(vector[1]).hypot(vector[2])` of the combined components, and `preview_physics.rs` does the same for support magnitudes.
- So a combination's magnitude rows have one formation rule on both routes, and `retained_unavailable` and `ordinary` rows keep passing G7 as today.
- Case displacement magnitudes keep DEF-O's hull recipe; G7 never checks them against components (RV118 S-4).

**The certificate's coverage.**
- The displacement-magnitude rows stay in DEF-C's `rows.coverage`: n of the 7n + 50m + 8g rows.
- Each row is certified as every row is: "direct error certificate of the actual frozen raw and SI values against both physical readouts" (`acceptance.final`, unchanged). A magnitude row's physical readout is each lane's Euclidean norm at that node (the dual physical norm).
- The bound: ‖x̂‖ − ‖x‖ ≤ ‖x̂ − x‖ plus two roundings, so the published magnitude carries the components' certified error, with a √3 factor on absolute floors and at most 2 ulps more.
- Whether a given row meets the predicates is the certificate's decision. A refusal is the combination's `facade_certificate` (decision 7).
- RR assigns RV115 (RV-K) to confirm this numerical content.

### 1.2 (b) The combination's observables stage (new; amends CONTRACT §2.4 (iii) 4)

**DEF-C r1's `stages.observables`, verbatim:** "base G7 combination_magnitudes on the frozen rows: one support-action row per support and component, and each displacement, force and moment magnitude within 64 epsilon relative of binary64 hypot(hypot(x,y),z) of the same combination's published components; the case support coverage and guard; no preview case evidence, maximum or headline check; a failure is this combination's facade_certificate".

**The stage, on the combination's own block** (its contiguous run, §3.2), after the certificate and before G5a, in this order:
1. **The adapter:** `require`, then one `LibraryBoundary` event, as B1's case `observables_view`.
2. **The gate entry:** the combination's `combination_gates` entry is `withheld: false`, which is T-10a's consistency check.
3. **Supports,** for each model support:
   - exactly six `support_reaction_component_v2` rows naming it, each component once;
   - exactly one force-magnitude and one moment-magnitude row;
   - each magnitude within 64ε of hypot(hypot) of its three components. That is B1's case support guard, the same code, and with item 4 it is G7's `combination_magnitudes`.
   - Every support-action row is unique per (support, component), G7's `COMBINATION_SUPPORT_DUPLICATE`.
4. **Displacements,** for each model node: exactly one `displacement_magnitude` row and one row each of `global_nodal_displacement_{x,y,z}`, with the magnitude within 64ε of hypot(hypot(x,y),z).
5. **Nothing case-only:** the block has no maximum or intensified row, and the stage makes no `preview_cases` evidence read, maximum-midpoint check or headline-alias check (DEF-C `stages.aliases`: headlines cover load cases only).

**Its events** are counted in the CombinationAttempt's adapter trace as B1's case stage counts them (`RowVisit`, `ValidationEntry`). At z = 0 there is no combination, so nothing changes.

**On failure:**
- the stage `observables` is `failed` and `g5a` is `not_entered`;
- `result` is `{kind: "unavailable", error: {kind: "observable", cause: CaptureError}}`;
- CONTRACT §4's reason table then gives (`facade_certificate`, `facade`) with the Run selected.

It is a per-combination outcome. The successor is not abandoned.

**Consequences:**
- **CONTRACT §2.4 (iii) 4 now reads:** "the combination's own freeze over its own row block: DEF-C's dual-readout proof, the certificate with KD §5.2's coverage rule, the observables stage (REVISION_01 §1.2) and G5a".
- **CONTRACT §10.1's G7 row is replaced by:** "Unchanged base validator. The projection removes `recovery_method`; T0R's gate evidence and `combination_magnitudes` run as today on every mechanics and subtraction combination. For a `retained_selected` combination, the guard holds by construction under DEF-C's magnitude recipes (§1.1), and its observables stage has already checked it. So a precommit G7 refusal can come only from a producer defect after the freeze, which abandons as any precommit refusal does. Ordinary and unavailable combinations keep the ordinary route's rows."

## 2. S-3 and N-7: R-COMB-1's rows, and PTABLE (amends CONTRACT §5 and §7)

**S-3, the rule.**
- **What the reader appends.** For every row of an R-COMB-1 combination, with combinations in authored order and rows in publication order, R-COMB-1 (R) appends one classification:
  - `not_covered` when the reader's row kind is a quantity kind;
  - **`non_quantity`** when it is `non_quantity`.
- **Which rows are records.** The only non-quantity kind a combination publishes is `combination_modulus_basis_record`, one per operand of a subtraction or range combination.
  - All three readers class it `non_quantity`: RS `row_kind` (`b1-r` `retained_precision.rs:2493`), PY `NONQUANTITY` (`b1-p` `:1149`) and TS `NONQUANTITY` (`b1-t` `retainedPrecision.ts:107`).
  - DN §4.1.6.1 closes that table.
- **The entry's other members** (`normalized_bits` and `scale_bits` null) follow a selected case's row of the same class.
- **Downstream:** D-U6-2's `class_disclosure` attaches nothing to a `non_quantity` row, and T6S withholds no witness for one, so neither has a code change.
- **The pins:** the must-pass entries of W-CB4a, W-CB4b and `b2_c1_range_mechanics` (§5.1).

**N-7: PTABLE's one revision states R-COMB-1.** `accuracy_classification.scope` in PTABLE r1, verbatim: "registered ordinary-prepared source, and registered prepared mechanics combinations (RP-PREPARED-COMBINATION-DUAL-v1: their own solve on the combined exact ledger, at least one operand selected); no exact-profile, subtraction or range extension; R-COMB-1: each quantity row of a retained_unavailable combination, and of an ordinary combination whose expression names a load case that is not not_required, is not_covered and withheld from binding with its value unchanged; record rows stay non_quantity".

**PTABLE r1 against the v0 draft:** only `accuracy_classification.scope` and `product_formation_definitions` (DEF-C's new H) change. `formation_warrant` and `receipt_bindings` are byte-equal to the v0 draft's, and `receipt_bindings` still equals XTABLE's. Against main, it is still three members changed and one added, and only `scope` changes within `accuracy_classification`.

## 3. The transaction (amends CONTRACT §2)

### 3.1 S-1: four more B1 sites (CONTRACT §2.2 gains these rows)

**At B1, each of these refuses any model combination.** Today none fires, because T-4's `w1_case_ids` returns `Domain` first. Once T-4 admits combinations, the three hooks would make every combination invocation in D1.4 a T-6 custody fallback with notices, and `observables_view` would fail every case freeze (RV118 S-1). CONTRACT §2.1's "T-2 Unchanged" is corrected to "**T-2′**: the three hooks below".

| Site on `b1` | Today | B2's extension | At z = 0 |
|---|---|---|---|
| **T-2′:** `ProductCapture::normalized` (`retained_product.rs:474–508`) | `!model.combinations.is_empty()` gives "outside private ordinary no-component/no-combination scope" | The conjunct becomes "the model's combinations are outside D1.4", by the same predicate as T-4's re-check (one shared function; suggested `w1_combinations_admitted`). T-1 already admitted D1.4, so it never fires in a W1 run. The component, pressure and load-state conjuncts are unchanged | The same predicate |
| **T-2′:** `prepared_case_seen` (`:3469–3475`) | The same conjunct gives "prepared case/no-combination source scope" | As above | The same |
| **T-2′:** `prepared_case_source` (`:3504–3517`) | The same conjunct gives "prepared late hook scope/presence" | As above | The same |
| **T-9′ (observables):** `observables_view` (`:2254–2272`) | `combination_gates` must be empty ("excluded combination gates") | **Shape and consistency only:** exactly one entry per model combination, in authored order, each closed over `{combination_id, withheld, reason}`; `combination_id` equal to the model's (adapter `same`); `withheld: false` ⇔ `reason: null`; `withheld: true` ⇒ `reason` is one of the three gate codes. A withheld entry is not a case-freeze failure: T-10a gives that combination `base_withheld`. One `ValidationEntry` per entry. The rest of the case's observables read its own block (T-9′'s scope); at c = 1 that keeps the support count at six, where `WHOLE` would also count the combinations' support rows | The array is empty: no entry and no new event |

**No adapter event is added by lifting the hooks.** No hook is entered for a combination: the ordinary route forms combination rows algebraically after the case loop (`append_combination_results`), with no solve.

**For T-9′'s gate check I chose "shape and consistency"** over RV118's other option, "every entry `withheld: false`". The stricter option would make a gate trigger fail every case freeze, which couples the cases' availability to a combination's disposition. Under D1.5 and D1.6 either option is unreachable.

### 3.2 S-2: T-6′ follows the producer's layout (replaces CONTRACT §2.2's T-6′ row)

| Site | B2's extension | At z = 0 |
|---|---|---|
| **T-6′:** `prepared_custody` → `bind_case_rows` (c ≥ 2); `case_scope` (c = 1) | The case blocks are B1's, ending at the first row whose basis `ref_type` is `combination`. **Every later row has basis `combination` and names a model combination.** **Each mechanics combination's rows form one contiguous run**, which is its freeze scope. Subtraction and range rows, including their trailing `combination_modulus_basis_record` rows, bind to their entries by basis id, with no contiguity (G3 (c) asks none). Each case's scope is its own block, at c = 1 too. A violation is a custody failure (T-6: `Preparation`, then notices) | Identical: no combination row exists |

**The corrected premise.**
- The producer appends, after the case rows, each combination's rows in authored order. `preview_physics.rs` `append_combination_results` iterates `model.combinations`, pushing that combination's base-id rows and then its support actions.
- Then `append_combination_modulus_basis_records` (`lib.rs:2802`, `:10019`) appends one record per operand of every subtraction and range combination, again in authored order, **after every combination's rows**.
- In-domain, nothing follows: D1.4 admits no component, and D1.6 refuses hangers (`retained_memory.rs`, `support.hanger`), so the component and spring-hanger appends add no row.

**Why no in-domain authored order falls back for layout:**
1. After the case rows, every row the producer appends in-domain is a combination row or a record, both with basis `combination`.
2. A mechanics combination has no record, and its rows are pushed in one iteration, so they are contiguous.

**Checked on a model of that append order** (`b2c_checks_r1.out.json` §s2_layout).
- **The 18 in-domain shapes** are:
  - c = 1 with z = 1 or 2, and c = 2 with z = 1;
  - each combination mechanics with distinct cases, mechanics with a repeated case (C-1, rowless), range over each operand set, or subtraction.
- **Revision 01's rule holds for all 18.**
- **v0's rule failed for two,** both at c = 1 with `range(A)` authored first: `[range(A), 2·A]` and `[range(A), range(A)]`.
  - For **c = 1, `[range(A), 2·A]`**, the layout is `[A rows][range(A) rows][2·A rows][range(A)'s record]`. v0 refused it at custody. Revision 01 admits it, and 2·A's freeze scope is its own contiguous run.
  - 07o pins it as `b2_c1_range_mechanics` (§5.1).

### 3.3 N-1, N-9, C-4 (N-2) and N-5

**N-1 (amends CONTRACT §2.4 (i) and §2.6): any rebuild refusal abandons the successor through `W1Fallback::CombinationCustody`,** whatever its cause.
- This includes a case Run that left no CasePrep: an `InvocationEntry` Run (meter exhausted or a work-accounting fault) or a `GroupPreparation` refusal. FK `solve_cases_projected` leaves no prep for either.
- Both are unreachable with a retained combination in D1.4 (RV118 N-1). The source table's "kernel prep was built" now reads "the batch registered its CaseSource".
- G5's `operand_source_unavailable` rule stays keyed on `ledger_unavailable`.

**N-9: the freeze's inputs** (amends CONTRACT §2.4 (iii) 4 and §10.1's G5a–G5c row). `begin_prepared_product` for a combination takes **operand 0's slot**:
- **the maps:** the id maps of operand 0's CaseSource;
- **the member facts Φ** (`ProductMemberFacts`: A, Ẑ, J and c, with the prepared D, t_eff and material operands) from operand 0's preparation record:
  - its case `ProductAttempt`'s preparation when operand 0 is a selected or unavailable case;
  - its `OperandPreparation` when operand 0 is `not_required`.
- **Why this slot:**
  - an operand with a source always has a completed preparation;
  - R-8 makes every operand's facts equal, so the choice is canonical;
  - G5a–G5c read the same slot, "the representative source's maps and sections".

**C-4 is recorded as an amendment of DESIGN §4.2's C3a rule 4** (RV118 N-2; RR ruling 5).
- **Rule 4's sentence** "A null `call_ref` is legal only with this cause" (`operand_preparation_failure`) now reads: "A null `call_ref` is legal only with cause `operand_preparation_failure` or `operand_source_unavailable`; `operand_source_unavailable` is decided first, at T-10b(i), before any preparation is requested."
- **It supersedes KD §2.1's mapping** of a ledger-refusing rebuild to `operand_preparation_failure`, which named a record that an unavailable case does not have.

**N-5: G5 (products class) refuses any CaptureError `{kind: "origin"}`**, with code `RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH`:
- within a `CombinationAttempt`: `result.error`'s `capture`, `observable` or `abandoned` cause;
- within an `OperandPreparation`: `result.error.capture`.

Readers thereby check RV115's N-5, that a combination's origin refusal never became a capture error. SCHEMA admits these shapes, as it does on a case attempt; checked under both validators (`b2c_checks_r1.out.json` §n12_n5_schema). It stays a reader rule, as RR ruled. Pinned by m66 and m67.

## 4. Gate placement (amends CONTRACT §1 C3a-7, §8 and §10.1)

### 4.1 N-3: the existing conjuncts that must branch (replaces §10.1's "Three existing conjuncts are widened")

Each conjunct below is written for the case batch. Each must branch for combination attempts, Runs and Calls, not merely be appended to, and each branch must be today's predicate at z = 0 (R5's census over 07n). RS by symbol, at `b1-r`:

| # | RS site | Today | The branch |
|---|---|---|---|
| 1 | `g3`, the attempt bijection | `refs` from cases' `product_attempt_ref`; `refs == 0..len(product_attempts)` | `refs` also takes every combination entry's non-null `product_attempt_ref`; the bijection covers both kinds (CONTRACT G3 (e)) |
| 2 | `g3`, Runs and `execution_order` | `runs` from cases; keys 0..len; `execution_order == runs.values()` | The cases' Runs, then the combination entries' Runs as `{kind: "combination", index}` (G3 (f)) |
| 3 | `g3`, the per-attempt owner lookup | `at(cases, owner_ref.index)` and `rows_for(source, case)`; member association with the source's `id_maps`; the `captured_prefix` and unsourced complete-inventory checks | For owner kind `combination`: the entry in `combinations[]` and its rows. `projection_outcomes` index its own rows. `summary_coverage` is checked against operand 0's CaseSource `body_membership` (a CombinationSource has none). The member-association, captured-prefix and complete-inventory checks are for case attempts only: a CombinationAttempt has no `operational` or `preparation` members |
| 4 | `g3`, the sources loop | `owner.kind == "case"` | Also the CombinationSource branch, and the operand-prepared CaseSources (G3 (g), (i)) |
| 5 | `g3`, the row basis | `ref_type == "load_case"` | A case or a combination entry (G3 (c)) |
| 6 | `g4` | A `RETAINED_PRECISION_*` diagnostic names exactly one case | One case or one combination |
| 7 | `g5_native`, the Run list | Runs from cases, indexed by Run id | The cases' and the combination entries' Runs |
| 8 | `g5_native`, per Call | `run_refs.len == source_refs.len == owner_refs.len` | For a mechanics Call: one owner; `source_refs` and `run_refs` both empty (`pre_source_refusal`) or both one (`runs`) |
| 9 | `g5_native`, the origin | Owner kind `case`; `owner_ref == {kind: case, index: ci}`; the case's `source_ref`; `src.owner.case_index` | For a mechanics Call: owner `{kind: "combination", index}`, the entry's `source_ref`, and the CombinationSource's `owner.combination_index` |
| 10 | `g5_native`, groups and caches | A Group's sources and stiffness; the cache from the group | A CombinationGroup: `imports`, `cache_before` = the imports (CONTRACT G5 native class) |
| 11 | `g5_products`, the stage and reason tables | Case attempts | CombinationAttempts: no preparation stage, and CONTRACT §4's reason table |
| 12 | `g8`, the invocation | `combinations` absent or `[]` | Removed; G8's expression equality takes its place |

**PY and TS mirror each site:**
- **PY:** `_validate_draft`'s G3 block (`b1-p` `:1821` owner kind `case`, `:1846` `execution_order`), `_g5_native` and `_g5_native_checks`, `_g5_products`, and G8's `combinations` check (`:1503`).
- **TS:** `coverage` (`b1-t` `:268` `execution_order`), `nativeClass`, and G8's `absentOrEmpty('combinations')` (`:1143`).

### 4.2 N-11: G3 checks two more orders (adds G3 (h) and (i))

- **(h)** `operand_preparations[]` is in first-need order (C3a-1). **Within B2's domain it is vacuous:** there is at most one record, because z ≥ 1 means c ≤ 2, and with one case selected at most one case is `not_required`.
- **(i)** The operand-prepared CaseSources sit after the batch's case sources and before the first CombinationSource, in record order (CONTRACT §2.7's `sources[]`).
- Both fail with `RETAINED_PRECISION_COVERAGE_MISMATCH`. m68 pins (i).

### 4.3 N-4: m50, m36 and `b2_base_withheld` at J1

**m50 stays a G5 failure.** C3a-7's G3 (d), and G3 (g)'s "named by exactly one … operand preparation", read a record's `source_ref` **only when the record is prepared** (`result.kind == "prepared"`). A refused record's non-null `source_ref` is G5's: `stage == "completed"` ⇔ `prepared` ⇔ `source_ref` non-null. This follows D22, which leaves a case attempt's unresolved source to G5. So m50 reaches G5 `PRODUCT_ATTEMPT_MISMATCH`, as C3a mutation 3 intends.

**m36** adds the entry's `RETAINED_PRECISION_UNAVAILABLE` diagnostic too (§5.3), so G4 passes and G5's disposition rule fires.

**At J1, before B2's readers,** CONTRACT §10.3's second group, the bases with a combination entry but no CombinationAttempt, passes G1 and fails at G3, which refuses a non-`load_case` row. **The exception is `b2_base_withheld`:** it has no combination row, so it passes G3 and first fails at **G8 `INVOCATION_MISMATCH`** (bound), because the invocation's model has a combination. All three readers refuse that at G8 (RS `:3518`, PY `:1503`, TS `:1143`; TS's SR-TS repair 01 moved its refusal there).

### 4.4 N-12: which gate refuses DEF-C's id on an OperandPreparation

**The schema gate, G1, in all three readers.** `OperandPreparation.definition_id` is `const` DEF-O's id in the J1 SCHEMA, so DEF-C's id is refused there. G0's row 9 checks only that the id is one of the table's: DEF-C's id is in PTABLE r1's `product_formation_definitions`, so G0 passes (checked, §n12_n5_schema). The texts are aligned:
- **CONTRACT §1 C3a-7's G0 row** now reads "`definition_id` is an id in PTABLE's `product_formation_definitions` (§8 row 9)".
- **Its G1 row** gains "`definition_id` is `const` DEF-O's id (SCHEMA): DEF-C's id is refused here".
- **CONTRACT §8 row 9** is unchanged, with the note "An OperandPreparation carrying DEF-C's id passes row 9 and is refused by G1's `const` (m65), as a case attempt carrying it is (m8)".

### 4.5 D6b is case-only (I98; RR ruling 3)

D6b (G5, ordinary class: a `selected` case's ordinary quality must be `sensitive`, `unresolved` or `failed`) applies to case entries only.
- **There is no combination analogue.** No reader refuses a `retained_selected` combination for being ordinarily `checks_passed`.
- **A combination has no ordinary quality to test.** It has no ordinary attempt and no `numerical_quality.cases` entry, and T-4 never puts it in A.
- **This includes W-CB1z,** whose combined ledger cancels exactly: its ordinary rows are all 0 (I98 §2.5). It is `retained_selected` by the disposition rule alone.
- The proxy's D6b refusal in I98 is a property of its one-case shape.

### 4.6 N-6, N-8, N-10 and N-14

**N-6 (amends CONTRACT §7).**
- **No code reads `formation_warrant`.** A `git grep` at NUM and on the four B1 branches finds only PTABLE itself (RV118 N-6), and §8 adds no read.
- It is hash-bound documentation.
- PTABLE's list of two warrant objects differs in shape from XTABLE's single object. C-8 stands, and the shape has no behavioural effect.

**N-8 (amends CONTRACT §11).** J1's RS edit is **the constant list only**: `g0`'s `json!([{DEF-O}])` gains `{DEF-C id, 0c43cf42…}`.
- I apply the same reasoning to TS's `header`.
- Packaging DEF-C and checking its file's H (§8 row 4's second half) moves to B2's RS and TS work.
- PP's `u1_constants_bound_to_in_tree_fixtures` and PY's `TABLE_HASH` are as §11 states.
- J1 stays mechanical.

**N-10 (amends CONTRACT §9; also to B2-A's brief, as RR rules).**
- **The census.** B1's typed census skips combination owners by design (`retained_memory.rs:291–297`), because D1.4 refused them. With z ≤ 2 admitted, B2-A also censuses every combination's strings for the typed text rows and the priced string counts, as a case's are:
  - the combination id;
  - the term case ids;
  - range operand ids;
  - minuend and subtrahend ids;
  - basis, mode and provenance text.
- **Where the rows go.** The six new cap rows sit **before `ControlBytes`**, which stays the last row: `domain_clauses` checks `rows[..CAP_ROWS − 1]` and then `rows[CAP_ROWS − 1..]` (`:893–899`).

**N-14 (amends CONTRACT §13).** The `CombinationReason` branch **`{space: "combination", tag: "count_range", name}`** is a new wire value (FK `CasePrep::combination`'s count-range refusal), and is reserved as the pair.
- The same tag and member shape exist in spaces `stop` and `unresolved` (SCHEMA `Stop`, `Unresolved`), with the same meaning.
- `count_range` is also a kind or tag in CaptureError, OriginError and SourceError.
- `"name"` is a common member (`b2c_collisions_r1.out.txt`).

## 5. 07o (amends CONTRACT §10.2–§10.3)

### 5.1 The witnesses and bases (S-5; I98; replaces CONTRACT §10.2's table)

Input hashes are I98's (PROBE §8; RR ruling 1).

| Base | Content | Expected | Provenance label |
|---|---|---|---|
| **W-CB1** ×2 modes | SW's A and B, both selected, combination **1·A + 0.5·B** (`r7_cb1_halfb.json`, `7af8c049…`) | `retained_selected`, non-degenerate | producer-solved (proxy `cb1_a_halfb_canonical.json` publishes; Rust PASS) |
| **W-CB1z** ×1 (sparse) | The same cases, **A + B** (`r7_cb1.json`, `0c346f49…`) | `retained_selected`, every combination row exactly 0 | producer-solved; **labelled "cancellation pin"**: the combined ledger keeps exact cancellation (RV115 NB-3) |
| **W-CB2** ×2 | Two-body A selected and B `not_required` (one OperandPreparation), A + B (`r7_cb2.json`, `ce52d528…`) | `retained_unavailable`, `combination_unresolved`, phase `kernel` | producer-solved (predicted by case C, which reproduces I81) |
| **W-CB3** ×2 | **v1:** the L = 0 base, A the milestone's moments (selected), B a 1 N `global_y` force on the restrained isolated N2 (`not_required`), A + B (`r7_cb3_v1.json`, `76bb9831…`) | `retained_selected` with one OperandPreparation | producer-solved (I98: B alone `not_required`; the proxy publishes) |
| **W-CB4a** ×2 | W-CB3's two cases with **A − B** (c = 2, z = 1, C_eq = 3) | `ordinary`; R-COMB-1: quantity rows `not_covered`, its two records `non_quantity` | producer-solved |
| **W-CB4b** ×2 | W-CB3's two cases with **range(A, B)**, `max_abs` (C_eq = 3) | The same | producer-solved |
| **W-CB5** ×2 | W-CB3's two cases with **2·B** (C_eq = 3) | `ordinary`, base rows, no appended class | producer-solved |
| **`b2_c1_range_mechanics`** ×2 (new) | The milestone (c = 1), combinations **`[range(case), 2·case]`** in that order (z = 2, C_eq = 3) | range: `ordinary`, R-COMB-1 (and its record `non_quantity`); 2·case: `retained_selected`. It pins S-2's layout | producer-solved (predicted: 2·case's ledger is the case's scaled exactly by 2) |
| `b2_base_withheld` | A gated mechanics combination | `base_withheld` | "synthetic: not producer-emittable in B2's domain" |
| `b2_pre_source_refusal` | A Call with `operands_differ` | `retained_unavailable` | "synthetic: not producer-emittable in B2's domain" |
| `b2_operand_preparation_failure` | **W-CB3** with its operand preparation refused | `retained_unavailable`, `operand_preparation_failure` | hook-produced (`fail_operand_preparation`), labelled |
| `b2_operand_source_unavailable` | W-CB1 with case B's T-7 preparation failed | `retained_unavailable`, `operand_source_unavailable {operand_index: 1}` | hook-produced (B1's `preparation_of_case`), labelled |

**There are 19 bases:** 15 producer-solved, 2 synthetic and 2 hook-produced.
- **Changes from v0:**
  - W-CB4 split into 4a and 4b, each in domain (RV118 S-5);
  - W-CB1 rebased (RR ruling 1), with W-CB1z added;
  - `b2_c1_range_mechanics` added;
  - the operand-preparation failure base moved to W-CB3, the probe-qualified operand preparation. I98 §4 did not observe W-CB2's operand-preparation path.
- **Why W-CB1z is in 07o.** The readers meet an all-zero combination there (S = 0 in G5b and G5c's absolute bound for S = 0), an edge no other base reaches. One mode suffices for the readers. B2-P pins A + B in both modes as a producer pin.
- **If B2-P finds W-CB2's B is not `not_required`,** W-CB2 keeps its role (an unresolved combination) without an operand preparation, and SC2 restates its must-pass entry under decision 20. After §5.3's moves, no mutation depends on W-CB2's operand preparation.

**Must-pass:** one per base, with `expected_classifications` covering the combination rows, by G5c or by R-COMB-1 with S-3.
- **Expected standing `eligible`:** where every case is `selected` or an ordinarily-eligible `not_required` case. That is every base except `b2_operand_source_unavailable`, and W-CB2 only when B qualifies. `b2_operand_preparation_failure` makes no case unavailable.
- **`needs_recompute`:** `b2_operand_source_unavailable`, whose case B is `unavailable`.

### 5.2 S-6: 07o's rehash rule (replaces "rehash all" for 07o; SC2's three harnesses)

**The rule, in order.** Each step reads only what earlier steps wrote. A reference that is not a strict index (07e's rule), or does not resolve, is skipped and left to the reader.

| Step | Recomputes | Depends on | New in 07o |
|---|---|---|---|
| 1 | Each CaseSource's `preparation.sha256` through `attempt_ref`, when every member of that attempt is prepared: H(`retained_precision_preparation_v1`, payload) | — | No (07e step 1) |
| 2 | Each CaseSource's `preparation.sha256` through `operand_preparation_ref`, when that record is `prepared` and every member is prepared: H(`retained_precision_operand_preparation_v1`, `{definition_id, definition_sha256 (DEF-O's H), owner_ref, ordinary_attempt_ref, material_basis_ref, purpose, members}`) (C3a-5) | — | **Yes** |
| 3 | Each `selected` case's `source_identity_sha256`: H(`retained_precision_source_mp_v2`, its CaseSource without `index`) | 1, 2 | No (07e step 2) |
| 4 | Each CombinationSource's `operands[].source_identity_sha256`: the identity of the CaseSource at its `source_ref` | 1, 2 | **Yes** |
| 5 | Each `retained_selected` combination's `source_identity_sha256`: H(`retained_precision_source_mp_v2`, its CombinationSource without `index`) | 4 | **Yes** |
| 6 | `publication_sha256` | — | No |
| 7 | `receipt_sha256` | 1–6 | No |

**Checked on a synthetic B2 body** built from CORPUS `two_case_synthetic` (`b2c_checks_r1.out.json` §s6_rehash), with the generator's canonical hash. The point is the dependency order, not reader parity.
- One pass of this order is a fixed point and leaves every hash relation true.
- After an operand-preparation edit (an m27-like edit), one pass leaves every relation true.
- 07e's rule leaves the operand preparation's hash false.
- A wrong order (5 before 4 before 2) leaves an operand identity false.

**Which designed first failures need it.** Without steps 2, 4 and 5, these stop at G1 instead of their designed gate:
- m27, m28, m29, m30, m64 and m65 (operand preparations);
- m39, m40, m62, m63 and m68 (CombinationSources and their operands).

**`after_rehash` for m10–m12.** Rehash would recompute what they edit, so they become `after_rehash` edits: 07b's D24 semantics, applied literally after the rule with no further hashing.
- Their designed first failure is G1 `RECEIPT_MISMATCH`. RS's G1 checks the receipt hash before any inner hash (`g1`), so the corpus pins gate and code, as 07b's hash-integrity pins already do.
- **Reader-local unit tests (three per reader)** pin the inner conjuncts by editing each inner hash and re-sealing the publication and receipt hashes.

**What SC2's three harnesses add:**
- **PY** (`tests/test_retained_precision_contract.py` `apply_mutation`):
  - Today it raises `KeyError` on any B2 base: a CombinationSource has no `preparation`, and an operand-prepared CaseSource's `preparation` has no `attempt_ref`. It reads `source["preparation"]["attempt_ref"]` unguarded.
  - It needs guarded reads (`.get`), steps 2, 4 and 5, and the operand-preparation payload with `purpose`: a reader helper beside `_preparation_payload`, or the harness's own.
- **RS** (`tests/retained_precision_contract.rs` `rehash`): steps 2, 4 and 5. Its `index(&s["preparation"]["attempt_ref"])` already skips both new source shapes, because a missing key indexes as null.
- **TS** (`retainedPrecision.test.ts` `rehash`): steps 2, 4 and 5. Its `if (s.preparation)` and `rehashRef` already skip both.
- **All three:**
  - the corpus file's `format_rule` text gains steps 2, 4 and 5 (snapshot 07o);
  - each lane's 07e index-rule test (RS `rehash_index_rule_07e`, PY `test_rehash_index_rule_07e`, and TS's tests of `rehashRef`) gains `operand_preparation_ref`, an operand's `source_ref` and a combination entry's `source_ref`;
  - step 2 uses the same DEF-O H constant as step 1 (RS `rp::DEFINITION_HASH`, PY `DEFINITION_HASH`, TS's literal).

### 5.3 The mutation table's changes

**Rows changed** (CONTRACT §10.3; every other row stands):

| # | Base | Edit | Designed first failure | Today |
|---|---|---|---|---|
| m2, m12, m27, m29, m30, m51, m64 | **W-CB3** (was W-CB2) | As v0 | As v0 | G0 |
| m10, m11, m12 | W-CB1, W-CB1, W-CB3 | The same edits, **as `after_rehash` edits** (§5.2) | G1 `RECEIPT_MISMATCH` | G0 |
| m14, m20, m33, m58 | **W-CB4a** | As v0 | As v0 | G1 |
| m18 | **`b2_c1_range_mechanics`** | The two entries swapped | G3 `COVERAGE_MISMATCH` | **G0** (it has a CombinationAttempt) |
| m19 | **`b2_c1_range_mechanics`** | One entry removed | G3 | **G0** |
| m28 | **W-CB5** | An OperandPreparation for B added (its operand-prepared CaseSource after the case sources), `requested_by: [0]`, naming the `ordinary` 2·B (C3a mutation 2) | G3 `COVERAGE_MISMATCH` | G1 |
| m36 | **W-CB4a** | The subtraction entry recast as `retained_unavailable` (`operand_source_unavailable`, every reference null) **and its `RETAINED_PRECISION_UNAVAILABLE` diagnostic added** (`affected_refs` = [its id], `diagnostic_ref` = that id) | G5 `ATTEMPT_MISMATCH` (disposition) | G1 |
| m48, m50 | `b2_operand_preparation_failure` (now on W-CB3) | As v0. m50 keeps G5 (§4.3) | G5 `PRODUCT_ATTEMPT_MISMATCH` | G1 |
| m59, m61 | **W-CB4b** | As v0 | G8 `INVOCATION_MISMATCH` | G1 |
| m60 | **W-CB4a** | As v0 | G8 `INVOCATION_MISMATCH` | G1 |

**New rows:**

| # | Base | Edit | Designed first failure | Today |
|---|---|---|---|---|
| m65 | W-CB3 | The operand preparation's `definition_id` → DEF-C's id (N-12) | G1 `RECEIPT_MISMATCH` (G0 row 9 passes) | G0 |
| m66 | `b2_operand_preparation_failure` | The refused record's `error.capture` → `{kind: "origin", cause: {kind: "missing_selected_origin", operand: 1}}` (N-5) | G5 `PRODUCT_ATTEMPT_MISMATCH` | G1 |
| m67 | W-CB2 | The combination attempt's error → `{kind: "capture", cause: {kind: "origin", …}}` (N-5) | G5 `PRODUCT_ATTEMPT_MISMATCH` | G0 |
| m68 | W-CB3 | The operand-prepared CaseSource moved after the CombinationSource (indices and references renumbered) (N-11 (i)) | G3 `COVERAGE_MISMATCH` | G0 |
| m69 | W-CB1 | One combination `displacement_magnitude` value × (1 + 2⁻⁴⁰), at a node that does not carry the combination's largest magnitude (S-4) | G7, the base code `COMBINATION_MAGNITUDE` as each reader names it | G0 |

- **How "Today" was found.** It is read from code. Every base with a CombinationAttempt stops at G0 today, because every attempt must carry DEF-O's id. The others stop at G1 (`combinations` `maxItems 0`, or `operand_preparations` an unknown member).
- **What SC2 does.** It fixes the actual first failures from the readers and declares any difference (decision 20). For m69 in particular, SC2 confirms that no G5 scale or class check fires first, since the edited row is not the combination's maximum.
- **The total** is 69 mutations.

## 6. RV115's NB-1 and NB-3: wording in DEF-C

**NB-1, `scope.operand_equality`, verbatim:** "equal K4STF bytes, layout, stations and supports (kernel OperandsDiffer) and equal prepared section facts across all operands and the combination: material basis, the selected material operands of every member bit for bit, normalized D, effective wall and the prepared A,I,J,Z,c bits of every member, as the operands' preparations and CaseSource section_terms state them".

**NB-3, `lanes.loads`, verbatim:** "each loaded DOF's exact combined net N_g (the owner's K4LED: every operand term's exact product c_i*v_ij, summed exactly per DOF) enclosed outward at 1024 bits as [RD1024(N_g),RU1024(N_g)], in free residual rows and constrained reaction offsets; prescribed coordinates exact zero; data flags from each individual product (c_i!=0 and v_ij!=0), never from a net, so an exactly cancelled net stays data; the enclosure never takes an operand's individual terms, a binary64 product or a rounded net".

**DEF-C r1 against v0's DEF-C:** exactly four paths change, `lanes.loads`, `rows.displacement_magnitude`, `scope.operand_equality` and `stages.observables` (new), as the generator lists. In addition:
- the bytes are their own canonical form, ASCII only, with no float;
- `inherits.operand_definition.sha256` is still H(DEF-O) `a7ed7ca0…`;
- `support_magnitude` equals DEF-O's;
- DEF-O and its six pins are unchanged.

## 7. Names (amends CONTRACT §13; `_run_records/r1/b2c_collisions_r1.out.txt`, at NUM `ba7bbae589`)

- **Reserved,** added by N-14: the pair `{space: "combination", tag: "count_range"}` with member `name` (§4.6).
- **07o base ids, 0 hits each:** `b2_c1_range_mechanics`, `W-CB4a`, `W-CB4b`, `W-CB1z`, `b2_base_withheld`, `b2_pre_source_refusal`, `b2_operand_preparation_failure` and `b2_operand_source_unavailable`.
- **Suggested internal names (lanes P and A), 0 hits each, no reservation needed:** `w1_combinations_admitted` and `combination_observables`.
- **Everything else** in CONTRACT §13 stands.

## 8. Estimates, restated (N-15; replaces CONTRACT §14's refined column)

Agent hours, without repair rounds.

| Slice | v0 | **r1** | What moves it |
|---|---|---|---|
| B2-A | 5–8 | **5.5–8.5** | N-10: the combination-string census, and the rows before `ControlBytes` |
| B2-P | 26–38 | **28–40** | S-1: three hooks and the observables gate check (1–2). S-4: the recipe and the combination observables stage (0.5–1). S-2: the layout rule (0–0.5). The W-CB1z, `b2_c1_range_mechanics` and W-CB4a/4b pins (0.5–1) |
| B2 readers | 39–55 | **41–58** (RS 17–24, PY 11.5–16, TS 12.5–18) | N-3's branches, N-5, N-11, S-3's record class, and the three inner-hash unit tests |
| SC2, B2's part | 7–10 | **9–13** | S-6's rehash in three harnesses (1–2); 19 bases and 69 mutations (0.5–1) |
| J1's package | 1.5–2.5 | **1–2** | N-8: constant lists only in RS and TS |

**Net:** about +6–8 h on v0's refined rows, and about +18–27 h on PLAN's. Review adds RV115's confirmation of S-4 (a) and RV118's confirmation of this revision, about 1–2 h. B2-K is unchanged.

## 9. For ROOT

1. **The confirmations RR orders:**
   - **RV115 (RV-K)** confirms S-4 (a)'s numerical content: DEF-C r1, H `0c43cf42…`, §1.1.
   - **RV118 (RV-C)** confirms this revision. B2-C is then final for J1.
2. **Choices I made within the rulings,** each reversible at review:
   - T-9′'s gate check is shape and consistency, not "all `withheld: false`" (§3.1);
   - W-CB1z is in 07o, sparse only (§5.1);
   - the new base `b2_c1_range_mechanics` serves m18, m19 and S-2's pin;
   - m28 is rebased on in-domain W-CB5 with an added record, not on a labelled out-of-domain synthetic base;
   - m50 is kept at G5 by restricting G3 (d) and (g) to prepared records (§4.3);
   - N-8's trim is extended to TS's J1 edit (§4.6);
   - the operand-preparation mutations and hook base move to W-CB3 (§5.1);
   - `statics/r1/` holds only the two changed statics, and the SCHEMA text stays v0's.
3. **Basis drift.**
   - NUM moved to `ba7bbae589` (records only; the maintained tree equals `cebff253d6`'s).
   - The B1 heads moved: `b1` to `03f55e7178`, `b1-r` to `6e3e4fe219`, `b1-p` to `2843a59a16` and `b1-t` to `6fa6a64658`. The B1 producer sites cited here are unchanged from `603e238517`.
   - I3's nodal-term-ordinal fix (RR) does not touch B2's contract: a CombinationSource has no `nodal_terms` (C2 §3), and its operands' CaseSources follow the fix as cases do.
4. **Nothing here needs an owner decision.** Public meaning before B8 is unchanged.

## 10. Execution record and limits

**Executed** with VENV (Python 3.13.14), `PYTHONDONTWRITEBYTECODE=1`, `-B`, `TMPDIR` in scratch and Git reads with `GIT_OPTIONAL_LOCKS=0`. `_run_records/r1/RUN_R1.md` has the commands with placeholders.
- **S1r1:** `b2c_statics_r1.py`, twice, byte-identical. It imports v0's sealed `b2c_statics.py` and I96's generators unchanged, and its controls reproduce all five v0 statics byte for byte.
- **S2r1:** `b2c_checks_r1.py`, twice, byte-identical.
- **S3r1:** `b2c_collisions_r1.sh`, twice, identical.
- **A control:** v0's sealed `b2c_checks.py`, rerun once at the current NUM, reproduces its sealed output byte for byte.
- **Writes:** only scratch, `statics/r1/`, `_run_records/r1/`, this file and `SHA256SUMS.revision_01`. No cargo, native job, install or Git write.

**Read:**
- the brief; the four basis records (in full); RR's three rulings and the B1 alignment notes they cite;
- my CONTRACT.md and v0 scripts;
- **code:**
  - PP `retained_product.rs` (`normalized`, `prepared_case_seen`, `prepared_case_source`, `observables_view`, the support guard);
  - PP `lib.rs` (the combination appends, `append_combined_vector_magnitude`, `append_combination_modulus_basis_records`) and `preview_physics.rs` (`append_combination_results`, the gate entries);
  - PP `retained_memory.rs` (`CAP_ROWS`, `domain_clauses`, the census note, D1.6);
  - RE `preview_physics_evidence.rs` (`guarded`, `combination_magnitudes`);
  - RS `g1`, `g3`, `g5_native` and `g5c`;
  - PY's and TS's mirrored sites, `NONQUANTITY` and the G8 combination checks;
  - the three corpus harnesses' rehash;
  - SCHEMA's `OperandPreparation`, `CombinationAttempt`, `PublicError`, `CaptureError`, `OriginError` and `count_range` branches.

**Limits:**
- **Nothing was compiled or run in Rust or TS.** S-1, S-2 and N-3 are code readings.
- **S-2's check is on a model of the producer's append order,** which I read from the code. No committed envelope has a subtraction or range record.
- **S-4 (a)'s guard check models faithful hypot implementations.** Its certificate coverage is an argument for RV115 to confirm, not a run.
- **The rehash check uses the generator's canonical hash,** not the readers' JCS adapter.
