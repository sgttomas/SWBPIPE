# RV118 (RV-C): independent review of B2-C, I97's B2 contract (documents only)

TASK (Type 2), RV118, holding RV-C for B2, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I am a fresh instance, wrote none of the contract, and made no delegation. 2026-10-07 UTC.

**The brief:** `R/BRIEFS/RV118_RVC_CONTRACT_REVIEW.md`, sha256 `1fd7e33483b6481382bfd50bf4bcc0035386d4756739cfce6e02763a1414e4f8`, verified before reading. I read NUM's `AGENTS.md` (`f96feb19…`) and `agents/AGENT_TASK.md` (`1a13a5b0…`) first.

**The candidate:** `R/I97/b2_c_01/CONTRACT.md`, sha256 `165cd4b1c0b5ed3f83f9230435c0a3ab2284211c90908daa647610bba81d9d28` (verified); its SHA256SUMS 13 of 13 OK. The brief it answered, `R/BRIEFS/B2C_CONTRACT.md`, is `a604a2c3…` as cited.

**Method.** Documents and code reading at NUM and on B1's four branches (read-only), plus read-only Python with VENV in `WT/scratch/rv118_rvc/`: I97's three scripts, unchanged, and two scripts of my own (`evidence/`). No cargo, native, solver or test job, no install, no Git write; Git reads used `GIT_OPTIONAL_LOCKS=0`. NUM moved from `96e52b5e62` to `670a5144dd` during the review (records only; its maintained tree equals the dispatch commit's at both). `evidence/RUN.md` has the commands and `evidence/checks.txt` the hashes and code sites.

**Notation** is the contract's: WT, NUM, P, T, R, RR, VENV, PP, FK, RE, RS, PY, TS, C1–C3, SC1, DN, D2, PTABLE, DEF-O, DEF-C, SCHEMA, CORPUS, XTABLE, PLAN, REV, KD, B3D, c, z, C_eq, A, h. "§n" is CONTRACT.md's section; "b1:" cites the `b1` worktree at `603e238517`.

## Verdict

**ACCEPT WITH AMENDMENTS. 0 BLOCKING, 6 SHOULD-FIX, 15 NOTE.**

The contract is careful and mostly right, and its load-bearing claims reproduce:
- **The statics rebuild byte for byte** (two runs), and my own canonicalizer gives DEF-C's H `9adf5178…` with DEF-O's `a7ed7ca0…` as the control. DEF-O's bytes and its six pins are unchanged. `receipt_bindings` equals XTABLE's both as JCS and as text.
- **The SCHEMA checks reproduce exactly** (I97's output byte-identical), and 81 instances of my own (31 positive, 50 negative) agree under `jsonschema` and PY's walker, with every new `oneOf` disjoint from its sibling.
- **T-10a and T-10b sit correctly** after B1's T-10 and before T-11. The four operand-source kinds follow C3a rule 1 and S-2, and the failure split follows decision 7 with R-1's abandonment. RV115's N-5 is stated as asked.
- **C-3's union is the right form,** and it keeps B3b's `definition_id` hunk applicable unchanged.
- **J1's three extra edits are necessary,** and with REV §1.4's they are sufficient as far as a repository-wide search finds.

Six amendments are needed. None reopens the design:
- **S-1:** §2.2 misses four B1 sites on the W1 path that refuse any model combination: three capture hooks at T-2 and the observables check at T-9. With §2.2's list alone, every combination invocation falls back at T-6.
- **S-2:** T-6′'s row-block rule contradicts the producer, which appends subtraction and range modulus records after every combination's rows.
- **S-3:** R-COMB-1's "every row" would class those modulus records `not_covered`, against DN's closed `non_quantity` table.
- **S-4:** G7's base validator checks every combination's displacement magnitude against its own components within 64ε. That is a check cases never meet, and DEF-C's inherited recipe does not guarantee it. A failure would abandon the whole successor at precommit, not just the combination. **This one may change DEF-C's bytes, so ROOT should rule on it before J1.**
- **S-5:** 07o's W-CB4 (and m28's base) is outside D1.4: two cases plus two combinations is C_eq = 4.
- **S-6:** 07o's designed first failures depend on a "rehash all" that the shared corpus format does not define for B2's derived hashes.

## Findings

| ID | Sev. | Where | Finding and evidence | Required change |
|---|---|---|---|---|
| **S-1** | SHOULD-FIX | §2.1 (T-2 "Unchanged"; T-9), §2.2, §14 | **§2.2's list of B1 code that B2 must extend is incomplete.** Four B1 sites on the facade's W1 path refuse any model combination (`checks.txt` §4):<br>• `ProductCapture::normalized` (T-2; b1 `retained_product.rs:500–507`): "outside private ordinary no-component/no-combination scope". It sets the capture's error, which `prepared_custody` returns at T-6;<br>• `prepared_case_seen` (T-2 early hook; `:3474–3475`): "prepared case/no-combination source scope";<br>• `prepared_case_source` (T-2 late hook; `:3516–3517`): "prepared late hook scope/presence";<br>• `observables_view` (T-9; `:2254–2272`): `combination_gates` must be empty ("excluded combination gates"), but the ordinary producer writes one gate entry per model combination.<br>Today `w1_case_ids` returns `Domain` first, so these never fire. Once T-4's re-check admits combinations, the first three make every combination invocation in D1.4 a T-6 custody fallback, with notices. Even with them lifted, the fourth makes every case freeze fail at observables. So §2.1's "T-2 Unchanged" is not true of B1's code. | **Add rows T-2′ and T-9′ (observables) to §2.2:**<br>• lift the three capture-scope refusals to "within D1.4";<br>• replace the observables gate check with a stated rule, for example one entry per model combination, each `withheld: false` (D1.5 and D1.6 exclude every gate trigger), or shape and consistency only;<br>• each identical at z = 0, with no new adapter event.<br>Add about 1–2 h to B2-P (§14). |
| **S-2** | SHOULD-FIX | §2.2 T-6′ ("The ordinary producer always appends this way") | **Combination rows are not always one contiguous block per combination.** After `preview_physics::append_combination_results`, PP's ordinary run calls `append_combination_modulus_basis_records` unconditionally (NUM PP `lib.rs:2790–2802`, `:10019–`). It appends one `combination_modulus_basis_record` row (basis `combination`) per operand of every subtraction and range combination, after every combination's rows. Validation admits a one-operand range (`validation.rs:2086–`). So in-domain shapes such as c = 1, z = 2 with `range(A)` authored before `2·A` give `[A][range rows][2·A rows][range record]`. T-6′ as written then fails custody and falls back with notices. This is fail-safe, but it loses availability, and the stated premise is false. | **T-6′:** the case blocks end at the first combination-basis row, and every later row names a model combination. Only each mechanics combination's rows need to be one contiguous block (its freeze scope); subtraction and range rows, including their trailing records, bind to their entries by basis id. G3 (c) already asks for no contiguity, so no reader text changes. Correct the premise sentence. |
| **S-3** | SHOULD-FIX | §5 (R-COMB-1 (R)), §10.2 must-pass `expected_classifications` | **R-COMB-1 (R) appends `not_covered` for every row of a covered combination, including its `combination_modulus_basis_record` rows.** DN §4.1.6.1 closes `non_quantity` as exactly four kinds, this record among them ("never bound to a rule, never in a receipt list"). All three readers' row-kind tables map it to `non_quantity` (b1-r RS `row_kind`, `:2463–2470`). D-U6-2's `class_disclosure` would attach the not-covered disclosure ("no verified accuracy for this quantity kind") to a presence record, and T6S would withhold a witness for it. | State that R-COMB-1 applies to quantity rows, and that record rows of those combinations are classed `non_quantity` (or get no appended class; choose one). Pin it in the W-CB4 must-pass entries. |
| **S-4** | SHOULD-FIX (rule before J1) | §3 `rows`; §4; §10.1 G7 ("so retained values pass it"); §2.4 (iii) 4 | **G7's base validator checks every mechanics or subtraction combination's displacement magnitude against hypot(hypot(x,y),z) of the same combination's published components, within 64ε relative.** The checks are RE `combination_magnitudes` (`preview_physics_evidence.rs:721–772`, guard `:154`), PY `_consistent_norm` (`:385–404`) and TS (`:100`). **Case displacement magnitudes are never checked this way;** only case support magnitudes are (`:574`, `:351`).<br>DEF-C inherits DEF-O's `displacement_magnitude` recipe: "H of hull of independent lane norm functionals, not norm of published components". DEF-O's `support_magnitude` recipe exists precisely to satisfy this guard: "binary64 hypot … unchanged 64-epsilon component guard".<br>Where each row's hull is a few ulps wide, the two values agree well inside 64ε. Where the K/G law gap at a node is amplified by cancellation (a small net displacement made of large member contributions, as combinations with opposing factors produce), the independently projected magnitude can differ from hypot of the projected components by more than 64ε.<br>If that happens, precommit G7 refuses, and **the whole successor is abandoned, including every selected case.** Decision 7 means a facade failure to cost only the combination. The contract argues only that G7 never reads operand rows. It also leaves the combination's observables stage undefined: B1's case observables read the case's `preview_cases` evidence, which a combination does not have. | **(b), at least, with no change to the statics:** define the combination freeze's observables stage. It runs G7's `combination_magnitudes` guard (displacement and support magnitudes against components, 64ε) on the frozen values, plus the case checks that apply to a combination (the support guard). A failure is then per-combination `facade_certificate`. Correct §10.1's G7 sentence.<br>**(a), optional, before J1 only:** DEF-C's `displacement_magnitude` takes DEF-O's support-magnitude pattern: publish binary64 hypot(hypot(x̂,ŷ),ẑ) of the frozen components, and certify the dual physical norm and the guard. That changes DEF-C's bytes and H, PTABLE's hash and B2-K's recipe, so RV115 should confirm it. RV115's NB-1 and NB-3 wording can ride the same regeneration. |
| **S-5** | SHOULD-FIX | §10.2 (W-CB4); §10.3 m14, m18–m20, m33, m36, m58–m61; m28 | **W-CB4 is outside D1.4.** "A − B and range(A, B) over a selected A" has c = 2 and z = 2, so C_eq = 4. The producer refuses it at G-A, with ordinary bytes and no successor, so "producer-solved ×2" cannot be built. m28's "W-CB2 + an ordinary combination" is also C_eq = 4. | **Split W-CB4 into two in-domain bases:** A − B, and range(A, B), each with c = 2 and z = 1 (16 bases).<br>Give m18 and m19, which need two entries, an in-domain base with c = 1 and z = 2, for example `[2·A, range(A)]` in that order (S-2). Or label them synthetic.<br>Label m28's base "synthetic: outside D1.4". |
| **S-6** | SHOULD-FIX | §10.3 ("rehash all unless noted"); SC2 | **"Rehash all" is undefined for B2's derived hashes.** The shared format, as PY `apply_mutation` (`test_retained_precision_contract.py:101–127`), RS `rehash` and TS implement it, recomputes only:<br>• CaseSource `preparation.sha256` through `attempt_ref`;<br>• selected cases' `source_identity_sha256`;<br>• the publication and receipt hashes.<br>PY's raises `KeyError` on an operand-prepared CaseSource. Several designed first failures (m27, m39, m40, m62–m64, and any edit to a CaseSource used as an operand) need the C3a-5 hashes, the CombinationSource operand identities and the combination's `source_identity_sha256` recomputed. Without that they stop at G1. Conversely, if rehash does recompute them, m10–m12 must be post-rehash edits, or rehash undoes them. | **State 07o's rehash scope and order:**<br>1. operand-preparation `sha256` (C3a-5, with DEF-O's table-bound H);<br>2. every CaseSource identity used by a CombinationSource operand;<br>3. `CombinationSource.operands[].source_identity_sha256`;<br>4. a `retained_selected` entry's `source_identity_sha256`;<br>5. the publication and receipt hashes.<br>Mark m10–m12 as post-rehash edits, and add the harness work (three lanes) to SC2's estimate. |
| N-1 | NOTE | §2.4 (i), §2.6 | **The operand-source table equates "kernel prep was built" with "Run not refused `ledger_unavailable`".** FK's `solve_cases_projected` also leaves no CasePrep for an `InvocationEntry` Run (meter exhausted or a work-accounting fault; `origin.group` null) and a `GroupPreparation` refusal (FKR `adaptive.rs:4981–`). A rebuild refusal there is not "where the batch built the prep". Both are unreachable with a retained combination in D1.4: the first needs at least 40B of overshoot or a counter fault, and the second leaves no selected case under one stiffness. | Say "any rebuild refusal abandons the successor (`CombinationCustody`)". G5's `operand_source_unavailable` rule stays keyed on `ledger_unavailable` |
| N-2 | NOTE | §2.4 (iii) 1, C-4 | **C-4 amends DESIGN §4.2 C3a rule 4**, which says "A null `call_ref` is legal only with this cause". It also supersedes KD §2.1's mapping of a ledger-refusing rebuild to `operand_preparation_failure`, which named a record that does not exist for an unavailable case. The contract's choice is the better one. | ROOT records the amendment with C-4 |
| N-3 | NOTE | §10.1 ("Three existing conjuncts are widened") | **The list undercounts.** RS `g3` also has, before the row check:<br>• the bijection of product attempts with case references (`refs == 0..len`, b1-r `:702–707`);<br>• the Runs/`execution_order` equality (`:708–711`);<br>• the per-attempt owner lookup into `cases[]` (`:776–781`).<br>`g5_native` assumes the case batch, and PY and TS mirror both. Each must branch for combination attempts, Runs and Calls, not just be appended. | Name them, each equal to today's predicate at z = 0 (R5's census) |
| N-4 | NOTE | §10.3 | **Three designed first failures need a correction:**<br>• **m50:** C3a-7's own G3 (d) ("`source_ref` non-null resolves to a CaseSource … whose `operand_preparation_ref` is this id") fires before G5 for a refused record with a source. Either restrict (d) to prepared records, or expect G3.<br>• **m36:** must also add the entry's `RETAINED_PRECISION_UNAVAILABLE` diagnostic, or G4 fires first.<br>• **At J1, `b2_base_withheld`** has no combination rows, so it passes G3 and first fails at G8 (`combinations` non-empty), not G3 | Correct the rows |
| N-5 | NOTE | §2.6 (RV115's N-5); §6.1 | **RV115's N-5 is a producer obligation only.** The J1 schema admits `CaptureError {kind: origin}` inside a `CombinationAttempt`'s `capture` error and an `OperandPreparation`'s `preparation.capture` (checked). | Optional: G5 refuses an `origin` capture error on a combination attempt or an operand preparation, so readers can check RV115's N-5 |
| N-6 | NOTE | §7, C-8 | **No code reads `formation_warrant`.** A `git grep` at NUM and on the four B1 branches finds only PTABLE itself, and §8 adds no read. It is hash-bound documentation. PTABLE's list and XTABLE's single object differ in shape. | State both facts in §7 |
| N-7 | NOTE | §5, §7 | **R-COMB-1 is not stated in PTABLE.** Under (R), the rule lives only in the contract and reader code. PTABLE's `accuracy_classification` is the hashed statement of coverage, and this is its one revision. | ROOT's choice: extend `accuracy_classification.scope` (or add a member) to state R-COMB-1 now. Any later change costs a cascade and a re-registration |
| N-8 | NOTE | §11 | **J1's RS edit needs only the constant list.** Packaging DEF-C and checking its H (§8 row 4) is B2's RS work, and can wait, which keeps J1 mechanical. Otherwise §11 and REV §1.4 are sufficient (search in `checks.txt` §4). | Optional |
| N-9 | NOTE | §2.4 (iii) 4, §10.1 G5a–G5c | **The combination freeze's inputs are not named.** These are the maps and the member facts Φ passed to `begin_prepared_product`. G5a–G5c read "the representative source's maps and sections". | State that the freeze uses operand 0's slot (its case attempt or OperandPreparation) |
| N-10 | NOTE | §9 | **B1's typed census skips combination owners** by design (b1 `retained_memory.rs:291–297`). With z ≤ 2 admitted, B2-A must also census combination strings (ids, term and operand ids, basis, mode and provenance) for the typed text rows and the priced string counts. The six new rows must also sit before `ControlBytes`, because `domain_clauses` splits at `CAP_ROWS − 1`. | Add both to §9 |
| N-11 | NOTE | §1 C3a-7 G3; §2.7 | **G3 does not check the order of `operand_preparations[]`** (first need), nor where the operand-prepared CaseSources sit in `sources[]`. | Check both, or say they are not checked |
| N-12 | NOTE | §1 C3a-7 G0, §8 row 9 | **The two texts disagree on which gate refuses an OperandPreparation carrying DEF-C's id.** C3a-7's G0 row says the id "equals DEF-O's id"; §8 row 9 says only "is an id in the table", which leaves it to G1's `const`. | Pick one, for all three readers |
| N-13 | NOTE | §2.3 rule 1, §10.1 G5 | **No reader rederives a `base_withheld` reason from the model.** G7 checks the gate evidence's consistency, not its truth. In B2's domain every `base_withheld` entry is therefore necessarily forged. This is fail-safe, since the rows are withheld. | Optional: G8 refuses `base_withheld` within D1.4, or rederives the T0R gate |
| N-14 | NOTE | §13 | **`count_range` in space `combination` is a new wire value,** not in C2 §2, and so is its member `name`. The table omits both. | Add them to the reservation table |
| N-15 | NOTE | §14 | **The estimates are honest as reading estimates,** but S-1, S-4 (b), S-5 and S-6 add about 3–6 h: B2-P +1–2, readers +1–2 (rehash harness, records' class), SC2 +1–2 and B2-A +0.5. | Restate |

## 1. The transaction (§2)

**Placement.** T-10a and T-10b sit after B1's T-10 check and before T-11's staging, as B1's `w1_transaction` makes possible:
- the selected bits are computed at b1 `lib.rs:3310`, after T-10 and before staging;
- the invocation stays in the capture after `freeze` (`capture.native_invocation` is restored);
- each case slot keeps its prepared `PrimitiveSource` (`native_call` moves the sources back "whatever it returned") and its Run.

**Every z = 0 byte is unchanged,** provided each extension is the identity at z = 0. That includes the adapter's event counts, which are receipt bytes (T-6′ at c = 1 must add no binding pass when z = 0). `for_invocation(&[n], &[], 0) ≡ for_calls(&[n], &[])` is KD's K-02 with RV115's N-6.

**The four operand-source kinds,** against C3a rule 1 and S-2:

| Kind | Contract | Basis | Verdict |
|---|---|---|---|
| `selected` (public, after T-9) | Its `RetainedSolve`, imports allowed | C3a rule 1; KD I4 | Correct. The custody check holds: `GroupCache` has no interior mutability, and the freeze borrows the solve immutably, so the cache still matches its finish snapshot |
| `unavailable` with a CaseSource, prep built (incl. a kernel-selected, freeze-failed case, R-11) | Rebuilt `PreparedCaseSource`, K4SRC-checked, no import | S-2 (a); KD I7; R-11 | Correct, with N-1's wording |
| `unavailable` with no CaseSource, or Run refused `ledger_unavailable` | `operand_source_unavailable`, no Call, no preparation request | C3a rule 1 ("no second preparation"); C-4 | Correct. It improves on KD §2.1 (N-2) |
| `not_required` | Its one shared OperandPreparation's registered source | C3a; S-2 (b) | Correct |

**The first selected operand's group** (S-2 (c)) is stated, with operand 0 as the representative (§2.5).

**The failure split** (decision 7) is complete:
- per-combination for pre-source refusals, operand-source and operand-preparation failures, a non-selected Run and facade refusals;
- the whole successor for every `OriginError`, `OriginRefusal` (`MissingSelectedOrigin`, R-1), `PreparedCaseSource::new` refusal and rebuild refusal (N-1).

`W1Fallback::CombinationCustody` publishes plain notices, because `receipt_encoding_detail` applies only to serializer causes (b1 `lib.rs` `ReservedNotices::publish`).

**RV115's N-5 is stated as RV115 asked,** with one caveat: the J1 schema still admits an `origin` capture error on a combination attempt (my N-5). A T-8′ capacity error is B1's T-8 call failure. No case can then be selected, so no receipt is published.

**§2.2's completeness.** Every site it lists is real, and I confirmed each in code:
- `bind_case_rows` refuses at the first combination row;
- `case_scope` gives `WHOLE` at c = 1;
- `native_call` uses `for_calls(&[count], &[])`;
- `stage_headlines` ranges over all rows of a headline kind;
- `serialize_cases_with` refuses at `:1779` and `:1808`;
- `invocation_arrays` writes `case_batch` and checks every Call's after against `charged`;
- `run_value` hard-codes owner kind `case`.

The list misses the four capture and observables refusals of **S-1**, and T-6′'s premise is wrong (**S-2**). I found no other B1 path on the W1 facade that mis-handles a combination row, Call or headline. Two sites in the scan are left out on purpose: `source_eligible` and the private one-case `serialize_selected` are off that path.

## 2. DEF-C and `CombinationAttempt` (§3, §4)

**Regeneration.** `b2c_statics.py`, run twice, rebuilds all six outputs byte for byte, equal to the record (`checks.txt` §2).

**The hash.** My own JCS canonicalizer uses UTF-16 key order and imports nothing. It gives:
- H(`retained_precision_formation_v1`, DEF-C) = `9adf5178…1731`, with DEF-O's `a7ed7ca0…0349` as the control;
- R-10's alternative domain: `562cbe14…487d`.

DEF-C's raw bytes are its own JCS form. `inherits.operand_definition.sha256` equals H(DEF-O).

**DEF-O is unchanged:** raw `3e0779a4…` and its six maintained pins. DEF-C's member delta against DEF-O is exactly §3's table (`rv118_hashes.out.json`). RV115 has since accepted its numerical content (ADDENDUM_02).

**The one exception is S-4:** DEF-C keeps a displacement-magnitude recipe that the G7 guard on combinations does not tolerate by construction.

**C-3, the union, is a faithful reading of REV §1.1 item 3. I recommend it.**
- **The intent of item 3** is that a combination carries a product attempt with owner kind `combination`, no preparation stage and DEF-C's id, which the schema admits. The union does exactly that, and keeps both shapes closed.
- **Widening `ProductAttempt` instead** needs either fabricated `ordinary_attempt_ref`, `preparation` and `operational` values, or nullable ones. Nullable members would widen every case attempt's type domain and the readers' stage tables.
- **REV's letter on `OperandPreparation`** ("becomes an enum") was overtaken by RV116 N-8 and by sense. An operand preparation is always a DEF-O preparation, so `const` is right.
- **Composition with B3b.** With the union, B3b's `SCHEMA_ENUM.diff` enum hunk applies unchanged to B2's text. Only the title and `$comment` hunk conflicts, as §6.2 says, and NA-1's one coherent text resolves it (`checks.txt` §3). Under REV's letter the enum hunk itself would conflict.
- **Disjointness.** `ProductAttempt` and `CombinationAttempt` are disjoint under both validators (`rv118_instances.out.json`).
- **Pairing.** G0 row 9 and G1's `const` together tie each kind of attempt to its definition.

## 3. R-COMB-1 (§5)

**(R) needs no output-code change:**
- `retained_row_classes` maps the reader's `classifications` by result id;
- `class_disclosure` discloses any `NotCovered` row whatever its kind;
- T6S's withheld-witness path takes classes from the reader (I74 §4.3).

**Every input is receipt-bound:** the disposition, the expression, `cases[].status` and `result_ids`, all under the receipt hash.

**The per-case summary is unaffected,** because `classification_summary_from` filters by case id. That reinforces C-9.

**I agree with (R),** with S-3 (records are `non_quantity`) and N-7 (state the rule in PTABLE).

**The owner note names DN §4.2 correctly.** DN §4.2's row lists "T0R-admitted `mechanics`, subtraction, range" under W1a "at the facade". In F2a, subtraction and range rows over a Sensitive case, and rows of unavailable combinations, are withheld from binding, and the owner was told this (RR "B2/B3 R1 …").

**D-U6-2's text** ("no verified accuracy for this quantity kind") misstates the cause for these rows. It is correctly routed to S-I2. A changed disclosure meaning goes back to ROOT under I74 §4.3, and it must be settled before B8.

## 4. SCHEMA (§6)

**`b2c_checks.py`** gives an output byte-identical to the record:
- meta-validation passes;
- the J1 vocabulary is main's;
- 0 verdict changes over 17 bases, 28 must-pass entries, 294 mutations and 6 successors, under `jsonschema` and PY's `_shape`;
- 29 instances and the R-7 row count reproduce.

**The three readers' walkers are generic** over `$ref`, `oneOf`, `const`, `enum`, `minItems` and `maxItems`, and `oneOf` inside `items` works in RS, PY and TS. So the J1 text needs no walker change.

**My own instances:** 81 across `CombinationExpression`, `Combination` (all four branches), `CombinationReason`, `CombinationSource`, `MechanicsCombinationCall`, `CombinationGroup`, `CombinationAttempt`, `OperandPreparation`, the CaseSource third branch, `UnavailableCause`'s additions and B3b's enum on `ProductAttempt`. Every verdict is as expected and equal under both validators.

**`oneOf` disjointness** holds for every new pair: each `Combination` instance matches only its own branch, and `CombinationReason` is disjoint from `Reason`.

**Body level:**
- a body with one of each new member validates;
- `operand_preparations: []` is refused;
- `combinations` given as id strings is refused.

**Left to the reader gates, correctly:** a repeated-case mechanics expression (C-1's G5 rule) and an `origin` capture error (my N-5).

## 5. PTABLE (§7)

**Members:** three changed (`product_formation_definitions`, `accuracy_classification`, `formation_warrant`) and one added (`receipt_bindings`). Within `accuracy_classification`, only `scope` changes.

**The scope text** is accurate: own solve on the combined exact ledger, at least one operand selected; no exact-profile, subtraction or range extension.

**Both `product_formation_definitions` hashes recompute.**

**`receipt_bindings`** equals XTABLE's as JCS and as indent-2 text, with the same key order.

**The inherited and base members are unchanged and still true,** including `supported_profile_limitations`.

**C-8, a list of two warrant objects, is the right shape.** Each object is self-describing by `definition_id`, and the DEF-O object is byte-equal to today's. **G0 does not read it:** no code anywhere reads `formation_warrant` (N-6), so its shape has no behavioural effect. The difference from XTABLE's object should be stated in §7.

## 6. G0 (§8) and gate placement (§10)

**G0.** The table rows keep today's checks first:
- the definitions comparison sits where RS and TS have it;
- the cross-check (row 6) sits between today's table hash and the body checks. It is "inserted" rather than "appended", but it reads only the table, so no 07n first failure can move.

My N-12 must be reconciled. §8's eleven reader-local unit tests are the right way to test a table change; a corpus entry cannot change a table (REV §2).

**Gate order across the readers.** The new checks are placed consistently in all three. N-3 lists existing conjuncts that must branch, not merely be appended.

**07o's 14 bases.** W-CB4 is out of domain (S-5). `b2_base_withheld`'s must-pass is buildable, because no reader rederives gate truth (N-13).

**The 64 mutations.** Each designed failure is right on its gate rules, with these exceptions:
- m50, m36 and the J1 prediction for `b2_base_withheld` (N-4);
- the hash-dependent rows (S-6);
- the W-CB4 rows (S-5).

**"Today's readers stop every `CombinationAttempt` base at G0."** I confirmed this in RS `g0`, PY `_validate_draft` and TS `header`: every attempt must carry DEF-O's id. **It is the right designed failure for a reader that does not know DEF-C.** The receipt names a formation outside that reader's table, so "unsupported contract" at G0 is the honest refusal, and once B2's readers land, DEF-C passes G0. The other bases fail at G1 today (`maxItems 0`). At J1 they reach G3, except `b2_base_withheld`, which reaches G8 (N-4).

## 7. D1.4 and the cap rows (§9)

**The limits** are PLAN decision 10 with REV N-4. They match I82's study, PLAN §3.3 and RV114's pricing:
- C_eq ≤ 3 is S3's price;
- h = 3 at c < 3 is at most 5,981,880 B over S3 (RV114 N-3).

**CAP_ROWS 47 → 53 is right:** Combinations, CaseEquivalents, CombinationTerms, CombinationTermsCapacity, RangeOperands and RangeOperandsCapacity. `CombinationsCapacity` goes from cap 0 to 2.

**C-9 is necessary.** The producer and the readers match by bare id:
- `case_diagnostic_refs_into`;
- `classification_summary_from`;
- RS `g3`'s `ids.contains(ref_id)`;
- diagnostic ids.

And `validation.rs` checks duplicates per entity type only.

**G-C's two bounds hold.** Each combination's rows are fewer than a case's at the caps. The records that carry error text (case attempts, operand preparations and combination attempts) number at most C_eq.

**REV N-4 (Li = 3·Lc) is correct.** There are at most three Runs, Li can bind only within earlier Runs' recorded overshoot, and a pre-source refusal makes no Run.

**Add N-10:** the combination strings in the census, and where the new rows sit.

## 8. J1's three extra edits (§11)

**They are necessary:**
- RS `g0` compares PTABLE's list with a one-entry constant (`:532`);
- TS `header` does the same, under `FORMATION_MISMATCH` (`:187`);
- PP's `u1_constants_bound_to_in_tree_fixtures` asserts it (`:378`).

PY compares no list, so its `TABLE_HASH` suffices.

**They are sufficient,** with REV §1.4's constants, re-pin and law tests. A repository-wide search finds no other reader of PTABLE's changed members: no code reads `formation_warrant` or `receipt_bindings`, and every other user of the file reads its rows, path or hash. The SCHEMA tests that read content hold on the J1 text.

**REVIEWED_INPUTS 14 → 17 is right:** DEF-C plus B3b's two. N-8 suggests keeping RS's J1 edit to the constant list.

## 9. Decisions C-1 to C-16 (§12)

| # | Verdict | Reason |
|---|---|---|
| C-1 | **AGREE** | Validation admits repeated mechanics terms (`validation.rs:1996–`). The ordinary algebra blocks them (`DuplicateOperand`), so no component, magnitude or support row is published (`LOAD_COMBINATION_VECTOR_INVALID`, no row). Retaining one would buy a Run and then fail KD's coverage rule. Refusing repeats in D1.4 would cost the whole invocation. DEF-C excludes `repeated_operand_cases` |
| C-2 | **AGREE** | DN §4.2, C3a rule 4 and C2 §4 use `combination_unresolved`. The code covers a ceiling, a budget stop and a refusal alike; the Run's `kernel_terminal` stays authoritative, and G5's reason table binds the mapping. Dropping C2 §2's three Run-terminal tags is stated |
| C-3 | **AGREE** | §2 above. I recommend the union |
| C-4 | **AGREE** | It is unambiguous, has no Call and takes precedence over a preparation failure. Record it as an amendment of C3a rule 4 (N-2) |
| C-5 | **AGREE** | It keeps T-1 to T-13 intact, and the meter chain is the Calls' alone |
| C-6 | **AGREE** | "Empty if none" would change every B1 and c = 1 receipt byte (PLAN risk 5). `minItems 1` is checked |
| C-7 | **AGREE** | §3 above, with S-3 and N-7 |
| C-8 | **AGREE** | §5 above, with N-6 |
| C-9 | **AGREE** | §7 above. An overlapping model falls back with ordinary bytes, losing availability only |
| C-10 | **AGREE** | Exact counts are unknown at T-8. Declared maxima satisfy SF-1's check, because the batch comes first |
| C-11 | **AGREE** | C2 §4: "no duplicate cache_inputs payload is serialized" |
| C-12 | **AGREE** | All three readers already rebuild K4SRC (`checks.txt` §4), so K4CMB is a concatenation. K4LED recomputation stays optional, as a case's ledger is attestation today. FK's oracle checks it |
| C-13 | **AGREE** | Collision-free under C-9; a serializer collision abandons |
| C-14 | **AGREE** | A per-combination summary would be a T6S-visible shape change |
| C-15 | **AGREE** | Ruled R-10. I reproduced both H values |
| C-16 | **AGREE** | An invariant failure has no honest per-combination wire tag. Readers could enforce RV115's N-5 (my N-5) |

**The collision log** reproduces: every line after the header is equal to I97's at NUM's current HEAD. Every name proposed for reservation has 0 hits outside `P/execution`. N-14 adds `count_range` in space `combination`.

**Public meaning.** Nothing in B2-C changes public meaning before B8, since successors have no product caller. It touches no owner-held choice:
- dense and lane ceilings;
- PHYS-R4;
- observation framing;
- KF2 and KF3;
- the supported machine or M above 12 GiB;
- the native-app witnesses;
- R-2.

R-COMB-1 stays as the owner was told. D-U6-2's text for combination rows is S-I2's, and must be settled before B8.

## 10. The estimates (§14)

**Honest as reading estimates,** and the refinements over PLAN are explained. S-1, S-4 (b), S-5 and S-6 add about 3–6 h (N-15).

## For ROOT

**Rule on:**
1. **S-4,** before J1's statics are applied:
   - (b) alone, with no static change, which I recommend at minimum; or
   - (a) as well, which changes DEF-C and therefore PTABLE's hash. Ask RV115 to confirm (a)'s certification first, and fold RV115's NB-1 and NB-3 into the same regeneration.
2. **S-3's choice:** records classed `non_quantity`, or no appended class.
3. **N-7:** whether PTABLE's one revision states R-COMB-1.
4. **N-12:** the gate for an OperandPreparation carrying DEF-C's id.
5. **C-1 to C-16** (all AGREE), with C-4 recorded as an amendment of C3a rule 4 (N-2).

**I97 revises the contract for S-1, S-2, S-3, S-5, S-6 and the NOTEs it takes.** The statics change only if (a) or N-7 is adopted. I hold RV-C for B2's statics at J1 and can confirm the revision.

## Execution record, what I read, and limits

**Executed:**
- I97's `b2c_statics.py` (twice) and `b2c_checks.py` (twice), unchanged, from scratch copies; I96's generators were imported from verified scratch copies;
- `b2c_collisions.sh` (once);
- my `rv118_hashes.py` (twice) and `rv118_instances.py` (twice);
- `patch` on scratch copies.

All with VENV, `PYTHONDONTWRITEBYTECODE=1`, `-B`, and `TMPDIR` in scratch. `evidence/RUN.md` records one correction in my own instances.

**Read:**
- the brief; CONTRACT.md in full, with its scripts;
- PLAN (§0–§2, §5–§6) and REV in full; RV114's review;
- KD in full; RV115's review (verdict, findings, §1.4, §2, §3, §7) and ADDENDUM_02 (as committed during this review);
- DESIGN_v2 §1.2, §4 and §8; C1 §2, §5 and §6; C2 §2–§4;
- DN (combination passages and §4.1.6.1);
- RR from "I93's B2/B3 plan returned …" to the end.

**Code read:**
- FK `origins.rs`, `combine.rs`, `adaptive.rs` (`solve_cases_projected`, `CasePrep::combination`, `GroupCache`) and `source.rs`;
- PP `preview_physics.rs` (combination rows, gates), `lib.rs` (the combination append, the modulus records, the vector magnitude, `CombinationExpression`) and `validation.rs` (combinations);
- the load-case algebra;
- RE `preview_physics_evidence.rs`, `semantic_contract.rs` and `derivative.rs`; PY's and TS's base validators;
- the three readers' G0 and G1 at NUM, and RS `g3`, `g4`, `reason_table` and `row_kind` on `b1-r`;
- on `b1`: PP `lib.rs` (`w1_case_ids`, `retained_w1`, `w1_transaction`, `ReservedNotices`), `retained_product.rs` (capture hooks, `case_scope`, custody, `bind_case_rows`, `native_call`, `freeze`, `freeze_case`, `observables_view`, `staged_envelope`, `stage_headlines`), `retained_wire.rs` (`serialize_cases_with`, `invocation_arrays`, `run_value`, D6a) and `retained_memory.rs` (caps, family clauses, cap rows, census);
- the corpus harnesses' rehash in all three languages.

**Limits:**
- **Nothing was compiled or run in Rust or TS.** S-1, S-2 and S-4 are code readings. S-4's magnitude disagreement is a mechanism, not an observed failure: how often it occurs depends on hull widths that only B2-K and B2-P's pins will show.
- **The schema checks are shape checks,** not the readers' full G0–G8.
- **B1 may still move at I3.** Every B1 site here is cited by symbol and line at `603e238517`.
