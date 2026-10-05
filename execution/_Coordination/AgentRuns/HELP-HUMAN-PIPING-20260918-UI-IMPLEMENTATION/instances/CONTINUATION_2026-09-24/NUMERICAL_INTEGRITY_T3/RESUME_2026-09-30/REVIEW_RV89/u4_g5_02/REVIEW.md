# RV89: independent review of U4 G5 part 2 (the in-build profile, gate caps, witnesses and challenge)

**Reviewer:** RV89, TASK (Type 2), dispatched directly by ROOT. No descendants. This continues the part-1 review (`R/REVIEW_RV89/u4_g5_01/`) with the same context.
**Candidate:** `cba3e9fda7` on `codex/piping-f2a-memory-20261004`, on top of the part-1 commit `1e323058f3`, which I reviewed. The base for published bytes is `8abb5274a9`. Both trees come from `git archive`; WT/f2a-memory's working tree was not used.
**Basis read:**
- I65's part-2 records: `R/I65/u4_g5_01/part2/RETURN.md`, `IMPLEMENTATION_PART2.md` and `_run_records/` (the profile tree, the chain outputs, the printed record, the generator and the mutant harness);
- the G4 l ≤ 128 records (COMPOSITION_G4.md, ADDENDUM_L128.md, API_G4.md, PUBLICATION_READER.md, G4's chain outputs);
- STACK_PLAN.md, STACK_INVENTORY.md and I54's BOUND.md;
- RR "U4 G5 part 2 verified and committed; in-build maximum 0.889 M; G6 granted", and ROOT's message.

**Oracles:** RV89's own. These are my sweep, my independent evaluation of the profile, my probes (including a deep-input witness I built myself), my mutants, and the reconciliation against G4. I65's tests were only run.

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 3 |
| NOTE | 6 |

**The headline.**
- **Still no permit, and nothing published changes.**
  - `REGISTERED_PROFILES` is `&[]`.
  - `priced_maximum` returns `Unpriced` whenever the Estimate count is not 0. I checked counts 1, 41, 42 and 1000; `ESTIMATES` is 42. `cap_priced_maximum` is `Unpriced` in both modes.
  - My part-1 sweep (71 inputs × 2 modes × 5 routes, every public admission-report field) is **byte-identical to base**. Base and candidate both hash to sha256 `e1677d73…0a3d`, the same file as in part 1.
- **The profile reproduces independently, exactly.**
  - All 47 generated forms and all 244 illustrative atom values transcribe I65's chain (`profile_tree.json`) term for term.
  - My own reading of the phase law (COMPOSITION_G4.md §1, plus RV84 C-N3: T19 and the statics in every phase) reproduces all 7 phases in both modes, byte for byte, against the printed record.
  - My own build prints the same 244 in-build atom values, and 31 of them recompute independently with my own `size_of` and node-law code.
  - **W3 is the maximum: 0.884342 M sparse and 0.889237 M dense, under 0.9 M by 63,048,886 B and 43,338,438 B**, and under M by 465.7 MB and 446.0 MB.
- **Against G4's l ≤ 128 record**, W3's +73,277,126 B, at the chain's illustrative strides, decomposes exactly into the routed corrections:
  - +53,656,672 B TAV_W (RV87 S-2 and the rebase);
  - +15,066,778 B T16 P2 (S-1 13,252,048 B, N-1(c) 47,544 B, and the BODY grammar and rebase);
  - +213,888 B STAGED (N-1(b) 203,040 B);
  - −1,068,148 B NOTICE (N-1(e));
  - +5,397,696 B statics and +10,240 B T19 (C-N3).
  
  The rest of the move to 0.889 M is in-build strides.
- **The witnesses, the challenge, FK and SR.** All 8 witnesses pass, each run as its own process; their outcomes match I65's. The challenge reproduces I65's peaks byte for byte. FK `--lib` (480/0/1) and SR (48) are identical to base.
- **Mutants.** I re-ran all 148 of I65's, and the result equals I65's record mutant for mutant: 146 killed, and the 2 recorded survivors (one equivalent by decision 7, and V19 in build.rs). I added 12 of my own on the profile and the gates: 10 are killed and 2 survive.
  - Q08 is equivalent in this build: `Shared<4,4>` and `Shared<4,8>` are both 752 B.
  - **Q10 is a real gap:** a maximum that skips X1 and X2 survives (S-3).
  - None was killed only by a compile error.

## Findings

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| **S-1** | SHOULD-FIX | Text run `part2/_run_records/text_p2/text_budget.caps.out.json`. Sites at `1e323058f3`: `PP/src/source_receipt.rs:1016`; `PP/src/source_receipt/rows.rs:533`, `:590`, `:592`; `PP/src/lib.rs:5592` | **RV87 S-2's repair is incomplete: five reached result-id copies are still priced in the 128-byte identifier class.**<br>– S-2's new rule matches the receiver spelling `^(row\|result)\.id$`.<br>– These sites copy the same `ResultItem` ids under other names:<br>  `accounted.insert(r.result_id.clone())` (mult 4,230);<br>  `binding.result_id.clone()` ×2 (1,760 each);<br>  `primary[&function].id.clone()` (960);<br>  `matched[0].id.clone()` (1,760), where `matched` comes from `rows: &[ResultItem]`.<br>– **Under-count: (1,024 − 128) × 10,470 = 9,381,120 B.**<br>– All five run only when legacy exact recovery selects (`finalize_for`, `rows::bind`, and `source_row_bindings` behind `selected_source.take()`), so the real effect is on X1: 0.8218 → about 0.8241 M dense. **W3 and the 0.9 M margin are unaffected.**<br>– lib.rs:5592 is also counted, spuriously, in TAV_W at 128 B, because the graph is flow-insensitive. That is conservative.<br>– I scanned every reached PP function for an unpriced id clone. The one hit, lib.rs:5561 `row.id = ids[&row.id].clone()`, is priced structurally in O (I54 BOUND "Source row qualification": "replacing already cloned id … old/new child pair") | Classify result ids by type, not spelling: `ResultItem.id` and the fields copied from it, `RowTreatment`/`Projection`/`FunctionalRowBinding` `.result_id`. Re-run TEXT and regenerate the profile before G6 registers it. RV87, which confirms S-2, should check these five |
| **S-2** | SHOULD-FIX | `PP/src/retained_memory_witness_tests.rs` (`witness_w2_cap_maximal`, `witness_w2b_cap_maximal_solvable`) against STACK_INVENTORY.md §3, row W2 | **The committed W2 does not drive the chain STACK_INVENTORY assigns it.**<br>– STACK_INVENTORY W2 is to exercise "the deepest raw Value (Deserialize, flatten buffering, Value Clone and Drop, canonical rendering of the raw request), maximal counts, maximal escaping", with "every string with a quote or backslash".<br>– The committed W2 puts a quote and backslash in **one** string, though its comment says "every string". It ends at `Fallback(Preparation)` in 0.04 s. W2b ends at `Fallback(Candidate)`.<br>– Neither asserts its outcome, and neither reaches the serializer or the precommit reader with the depth-16 raw Value. Only the milestone, at raw depth 7, publishes on the witness stack.<br>– **RV89's own witness closes the gap as evidence.** I took the milestone, appended a quote and a backslash to every provenance string, and added a raw depth-16 value. It stays inside D1 (raw depth 16) and **publishes a successor at 4 MiB (R/16) and at 1 MiB, in both modes** (`evidence/p2/probe_output.txt`, RV89_DEEP_WITNESS) | Commit a publishing deep-input witness like RV89's (W2's publishing half), and assert W2's and W2b's outcomes. G6 records it per identity |
| N-1 | NOTE | `part2/_run_records/profile_record_p2.txt`; RV89's `evidence/p2/profile_check.out` | **The 43 MB dense margin is thin against text revisions, not against strides.**<br>– At W3 (dense), 2.84 GB (0.705 M) is layout-free form constants, and a further 0.107 GB is Text atoms. The stride-dependent in-build terms total about 0.38 GB.<br>– The dominant terms are TAV_W (1.565 GB, 0.389 M), T16 P2 (1.268 GB, 0.315 M), O (0.292 GB), T16's moving text (0.189 GB) and the staged copy (0.095 GB).<br>– With every stride now in-build, the margin's original purpose (ROOT's margin rule: "absorbs stride differences in the build") is discharged.<br>– What remains is derivation risk:<br>  a 2.8% error in TAV_W, or 3.9% in T16 P2's constant, would consume the 43 MB;<br>  this part's own S-2 correction moved W3 by +53.1 MB, and the SR node stride by about +63 MB.<br>– ≤ M itself keeps 446 MB (11.1%) | No code change. ROOT may want G6 to state a text error budget, or to hold the phase-aware lever (COMPOSITION_G4 §5, about −0.085 M) in reserve |
| N-2 | NOTE | `retained_memory.rs`, `ATOM_BINDINGS` and the 42 Estimate atoms (IMPLEMENTATION_PART2.md §8) | **The 42 Estimates are plausibly bounded.**<br>– They appear only in T12–T15 and O (`SupportVector`), and weigh 6,053,120 B in W3.<br>– The heaviest are `EnclosureE` 1.25 MB, `LazyE` 1.14 MB, `TableE` 1.00 MB and `ProductRow` 0.39 MB.<br>– For any single Estimate to consume the dense margin, its stride would have to be at least 35× too small. The break-even strides are: EnclosureE 10,884 B against 304 B (35.8×); LazyE 86,877 B against 2,232 B (38.9×); TableE 2,131 B against 48 B (44.4×); ProductRow 111×. Every other Estimate needs at least 166× | G6 closes all 42, as I65 states. The bound stays `Unpriced` until then |
| N-3 | NOTE | `PP/tests/retained_memory_challenge.rs` | **The challenge measures what it claims, with three stated limits:**<br>– the input Value is cloned before the baseline, so it is outside the measured window;<br>– realloc is counted as net growth, so moving bytes are not counted;<br>– it covers only the ordinary span (W1), because no permit exists.<br>My own permitted-path challenge, through the private driver with old and new backings counted together on realloc, peaks at 2,274,596 / 2,280,959 B (milestone, successor) and 13,799,790 / 13,807,663 B (my large D1 input, Candidate fallback). That is ≤ 0.065% and ≤ 0.395% of the W3 maximum | None. It remains a challenge, as stated |
| N-4 | NOTE | RETURN.md "Decisions for ROOT" 1 | **The pinned in-build record is appropriate.**<br>– `profile_in_build_record` and `challenge_bounds_are_the_profile` are test-only. They bind this identity's layouts, which matches G6's per-identity registration.<br>– Only `ProductCapture` has `cfg(test)` fields, and it is not an atom, so the test build's atom values equal production's. My production-independent recomputation agrees.<br>– No CI job runs these PP tests, so the pin bites only when someone runs them | Accept. G6 regenerates and re-pins per identity |
| N-5 | NOTE | `retained_memory.rs:1769` (`s(Shared<4>)`); I65's P2 mutant table | **Two mutant notes.**<br>– My Q08 (`Shared<4>` reading only `Shared<4,4>`) survives because both instantiations are 752 B in this build, so it is equivalent here. Taking the maximum stays the right binding for other builds.<br>– Four of I65's 40 P2 kills come only from the pinned record (`profile_in_build_record`), as I65 states. That is legitimate for an identity-pinned record, and it makes the pin's acceptance (N-4) load-bearing for those mutants | None |
| **S-3** | SHOULD-FIX (test-only) | `retained_memory.rs:2254–2266` (`profile::maximum`) | **A maximum that skips the X phases survives the committed tests.** My Q10 changes `while i < PHASES` to `while i < 5`, so X1 and X2 are dropped from the admission maximum. It **survives**, because W3 is today's maximum and the tests pin each phase's value and the maximum's value, but never a case where X1 or X2 is largest. X1 is 0.822 M; a T25 correction of 0.07 M would make it the maximum, and the bug would then under-report silently | Add a pure test of `maximum` over synthetic phase arrays in which each of the 7 phases in turn is largest, including ties and an overflowed phase |
| N-6 | NOTE | RETURN.md §RV85 U1 | **RV85 U1 is still not done.** This is unchanged from part 1, and the permit's binding to its invocation remains text- and structure-checked. I65's reason (lib.rs is outside D-5, and a compile-only assertion is unkillable) is accurate | ROOT or G6 to decide whether it is wanted before registration |

## 1. Still no permit, and nothing published changes

- **By reading** (`retained_memory.rs`):
  - `REGISTERED_PROFILES` is unchanged, `&[]`, and `admission()` is unchanged.
  - `cap_priced_maximum(mode)` = `priced_maximum(profile::ESTIMATES, mode)`, which returns `Err(Unpriced)` for any non-zero count (:1025–1034).
  - `ESTIMATES` is a `const` count of `Binding::Estimate` atoms; it is 42.
- **By probe** (`rv89p2_no_permit_and_unpriced`):
  - `priced_maximum(e, mode)` is `Unpriced` for e = 1, 41, 42 and 1000, in both modes;
  - `build_status()` is `Missing`;
  - the binding counts are 190 InBuild, 6 SourceUpper, 6 Text and 42 Estimate.
- **The sweep:** 4,022 lines, byte-identical between base `8abb5274a9` and `cba3e9fda7` (`e1677d73…0a3d`).
- **The suites** (default toolchain, `--locked --offline`):

| Suite | Base | Candidate | Difference |
|---|---|---|---|
| PP (lib and every integration target) | 661 / 1 / 2 | 694 / 1 / 10 | Additions only: part 1's 23 and part 2's 9 law tests, the challenge, and the 8 ignored witnesses. Nothing removed or renamed. The failure is the Mac t13 in both |
| runner/headless | 85 / 2 | 85 / 2 | identical |
| FK `--lib` | 480 / 0 / 1 | 480 / 0 / 1 | identical |
| SR | 48 | 48 | identical |

- **The part-2 test diff removes nothing.** Two assertions are replaced, each with a stronger one:
  - the milestone's G-C facts are now all checked ≤ their caps, with no UNPRICED exemption;
  - 2·Text(diag_env) is now tied to the profile atom.

## 2. The profile against the derivations

**The transcription, all 47 forms** (`evidence/p2/profile_check.py`).
- I parse the generated `FORMS`, `ATOM_NAMES` and `ATOM_ASSUMED` from the Rust source and compare each with I65's `profile_tree.json`.
- **All 47 forms match term for term, and all 244 illustrative atom values match.** The 6 Text atoms are in `text_atoms`.
- This extends I65's own check, `profile_transcribes_the_python_chain_exactly`, which compares only the maximum.

**The phase law, my reading.** I compose the phases from COMPOSITION_G4.md §1 (X1, X2, W1–W5 and their moving candidates), with RV84 C-N3(a) and (b): T19 and the reader statics in every phase.
- My composition reproduces all 7 phases' requested and moving bytes **exactly** against the printed record, in both modes.
- The generated `phases_sparse` and `phases_dense` therefore implement that law.
- **No phase is missing a term.** Every part COMPOSITION_G4 lists is present: O, T25, TAV_X, TAV_W, T11, T12–T15, T16, the staged copy, T17, the successor, the invocation, the statics, the N1 reserve, T19 and the moving candidates.
  - T18.3, a 48-B move, and the `W1Fallback` move are correctly absent.
  - T07's moving transient is in W1 only, which is right: it dies in the ordinary span.

**Reconciliation, form by form, against G4's l ≤ 128 record**, at the chain's illustrative strides (`evidence/p2/reconcile.out`). Of the 47 forms, 33 have a G4 counterpart. The rest are new or split forms. **I checked all 47:**
- **Unchanged from G4** (so tied to the derivations RV84 and RV87 reproduced byte for byte): O_base (2), T11, T12–T15 (sum unchanged), STATICS, INVOC, T19, HELPER_moving, TXT_moving, T17_V3, T17_moving_invocation, T25_S1, S2, S3, S5.
- **Changed, each by a routed correction:**

| Form | Δ (illustrative) | Attribution, checked |
|---|---|---|
| TAV_W / TAV_X | +53,656,672 / +49,355,862 | **RV87 S-2.** 15 result-id sites moved to the 1,024-B class (×8), plus carry 3's `collect_string` (+16,080 B). The rebase is +520,336 in W. The +71,898,576 TAV total is verified site by site against `text_budget.caps.rebase.out.json`. The 10 extra sites I65 found are genuine: RV89 read rows.rs:294 and :608, preview_physics.rs:517 and :690, and retained_product.rs:2239 and :2434. See S-1 for the five sites the rule still misses |
| T16_P2 | +15,066,778 | **S-1:** 14,248 nodes, 8,272 Values and +4,268,002 B of constant, which includes S-1's 13,252,048 B (RUN.tree() + SELECTION.tree()). **N-1(c):** exactly +1,981 Strings = 47,544 B, RV87's figure. The rest is the BODY grammar (N-1(d)) and the rebased envelope |
| T16_P3 / P4 | +16,489,992 / +15,280,926 | S-1 applies to P2–P4, as RV87 asked |
| STAGED | +213,888 | **N-1(b)** 8,460 Strings (4 per row × 2,115) = 203,040 B, exactly RV87's figure; D_env +1; rebased text +10,696 B |
| NOTICE | −1,068,148 | **N-1(e)**, grant 1b's exact reservation: 718 + s(Diagnostic) + s(String). This equals part 1's `NOTICE_RESERVE_BYTES` expression |
| NOTICE_moving | (new) | D_env × s(Diagnostic): the N1 reserve's old backing (COMPOSITION_G4's moving candidate) |
| T17 V1, V4, V5, V6 | +1.84, +4.95, +5.60, +4.97 MB | **N-2:** the per-object walkers and the `basis_ref` clones in V4–V6, plus the BODY grammar in V1. The maximum is still V2 |
| T17_V2_hash / T17_output | split | `RowClassification` × 4,096 and `Validation` move to `T17_output`, which is added to the stage maximum. That is conservative |
| T25_S4 | +1,572,864 | **RV84 C-N2:** the per-term `dof` numbers. It does not set T25's maximum, which matches RV84's "absorbed" |
| T25_I1–I3 | +427,934 / +13,366 / +12,424 | **C-N1(b):** L_BODY = max(L_ROW, 2,330), plus rebased text. I1 sets T25 |
| SUCC | +642,818 | N-1(a), the doubled diagnostics push, and the BODY grammar. It is not in the maximum |
| BODY (X2) | (new) | **C-N3(c):** RECEIPT_X is replaced by the BODY tree, as RV87 N-6 suggested |
| T16/T17/T25 moving | +21,576 each | rebased publication text |

**The W3 decomposition is exact.** G4's W3 requested 3,170,117,330 B becomes part 2's 3,243,394,456 B at illustrative strides. The difference, +73,277,126 B, equals TAV_W + T16 + STAGED + NOTICE + statics + T19 as listed in the headline, to the byte. The in-build strides then add +61,023,169 B (sparse), mostly SR's `Node`: 184 B against an assumed 64 B, × 524,288, for +62.9 MB.

**Are the in-build atoms the right types?** I read all 244 bindings against the type names and the source.
- **Correct as mapped:**
  - `s(ThreadPacketOutput)` is the `Option<thread::Result<Option<Result<RetainedPreviewOutput, String>>>>` that the scoped thread's closure returns (lib.rs:2928–2931, :2945);
  - `s(RowBinding)` = `ProductFinalRow` (bind_rows' element, retained_product.rs:1746);
  - `s((&str,&T))` = `(&str, &u8)`: every `&T` at its sites is a thin reference to a sized type (`&PreviewSupport`, `&MaterialInput`), so it is 24;
  - `Shared<4>` takes the larger of the two L = 4 instantiations (4,4) and (4,8), while `VerifyShared<4>` uses (4,8) only. This is correct: `VerifySlot`s exist only for (4,8), (8,16) and (16,16) (adaptive.rs:3924–3926);
  - `ArcCasePrep` = 2·usize + `CasePrep` (the Arc header, for alignment ≤ 8);
  - the node atoms use BUILD.md §4's law at in-build K and V layouts. `Node(String,BTreeMap)` uses `BTreeMap<String,String>`, and every BTreeMap is 24 B.
- **Independent recomputation** (`rv89p2_in_build_atoms`): 31 atoms, recomputed with my own `size_of` and my own node law, match the generated values. These include s(Value), s(String), ResultItem, Diagnostic, MechanicsEnvelope, the request types, OrdinarySeed, CaptureError, the reader types, FrameNode/FrameElement, SpringEntry, ThreadPacketOutput and five node atoms.
- **No `cfg(test)` layout skew.** `ProductCapture` is the only PP type with `cfg(test)` fields or variants, and it is no atom. FK is compiled without `cfg(test)` in PP's test build.

**The 6 SourceUpper atoms are sound** (my field reading of the source; probe-checked):
- `Projection` (source_receipt.rs:582: 4 String, 3 &str, 4 f64, [f64;2]) = 192;
- `RowTreatment` (:597: String, &str, Option<String>, Option<&str>, Vec<String>) = 104;
- `Derived` (rows.rs:324: ResultItem, &str, Vec<String>) = 336.
- A field sum with each field padded to the maximum alignment bounds any repr(Rust) layout: sorting by alignment, Rust's size is up(Σsize, A) ≤ Σ up(size_i, A).
- `Node(String,RowTreatment)` is the node law at that value size, which is monotone.
- `Option<FormationRecord>` (load_ledger.rs:101: Formation, f64, bool) adds one aligned slot for the tag.
- `(Content, Content)` ≤ 64: serde_core 1.0.228 private/content.rs:10–39. Content's largest payloads are 24 B (String, Vec, Seq, Map), so Content is ≤ 32, and the `Option` FlatMap buffering fits in the niche.

## 3. The in-build maximum, reproduced, and how sensitive it is

**Reproduced.** From the printed record's 244 in-build atoms and the 47 forms, by my composition: sparse W3 E_mov = 3,493,720,906 B, + R = 3,560,829,770 B = **0.884342 M**; dense 3,513,431,354 B, + R = 3,580,540,218 B = **0.889237 M**. The margins below 0.9 M are **63,048,886 B and 43,338,438 B**. My own build's atom print is identical.

**The per-phase picture (dense, + R):**

| Phase | Fraction of M |
|---|---|
| W1 | 0.4813 |
| W2 | 0.5056 |
| **W3** | **0.8892** |
| W4 | 0.8827 |
| W5 | 0.5484 |
| X1 | 0.8218 |
| X2 | 0.4544 |

W4 is 26.4 MB under W3.

**What dominates W3 (dense):**

| Term | Bytes | Fraction of M |
|---|---|---|
| TAV_W | 1,564,864,714 | 0.389 |
| T16 P2 | 1,268,332,585 | 0.315 |
| O | 291,873,278 | 0.073 |
| T16 moving | 189,303,281 | 0.047 |
| STAGED | 95,455,410 | 0.024 |
| T14 | 54,563,728 | 0.014 |
| T13 | 41,504,510 | 0.010 |
| the rest | < 6 MB | — |

**Sensitivity, per term.** In-build strides are exact for 190 atoms, so they no longer carry uncertainty.

| Term | W3 coefficient | Effect |
|---|---|---|
| Node(String,Value) | 227,301 | +1 B per node → +227 KB |
| SR Node | 524,288 | 96.5 MB |
| s(Value) | 616,454 | 19.7 MB |
| Text(row) | 8,302 | 95.3 MB |

A uniform +10% on every layout atom would add 48.5 MB, which exceeds the dense margin, but layouts are now measured, not assumed. The live risk is in the layout-free text constants; see N-1.

## 4. The 42 Estimate atoms (6,053,120 B)

Every Estimate sits in T12–T15, except `SupportVector`, which is in O. Their W3 coefficients range from 1 to 20,800. With in-build values (`evidence/p2/profile_check.out`, last section), the largest weights are:
- `EnclosureE` 304 × 4,096;
- `LazyE` 2,232 × 512;
- `TableE` 48 × 20,800;
- `ProductRow` 96 × 4,096;
- `FinalRowConversion`, `LawE`, `ProductValue` and `ProjectionOutcome` at 0.26 MB each.

The stride at which a single Estimate would consume the dense margin is at least 35.8× its design-record value (EnclosureE; then LazyE at 38.9× and TableE at 44.4×). For all the others it is above 100×. None is plausibly that far off. Most are hash-map entry strides and small records of a few words. **They cannot be closed in G5 and need not be: the bound stays `Unpriced` until G6 replaces them.**

## 5. My part-1 findings

- **S-1 (V03, V06, V07, V08) and N-2 (V17, V24):** killed by I65's new tests (`d1_3_refuses_one_authored_law_among_several_materials`, `restraint_capacity_total_is_the_sum_over_supports`, `typed_walk_reads_request_materials_and_temperature_point_ids`, `gate_sums_saturate_and_the_longest_string_reads_every_diagnostic_field`). My re-run of I65's RV set confirms it: 23 of 24 are killed. V03, V06, V07, V08, V17 and V24 are each killed by the new tests, and V12 is killed at its re-anchored line. V19, in build.rs, survives as expected.
- **N-4:** `bindings_hold` (:968–980) now refuses any compiled reviewed-input text with a field ending `=unavailable` (`split(';')`, no allocation). An encoded path cannot contain a raw `=` or `;`, so no real path can end that way. `an_unreadable_reviewed_input_never_binds` covers each of the 14 inputs. I65's two N-4 mutants ("unavailable inputs bind" and "only the first input checked") are both killed.
- **N-1, N-3, N-5 and N-6:** recorded as I65 states. N-6 is an erratum in IMPLEMENTATION_PART2.md §9, which correctly lists the 10 `function_multiplicity` entries.

## 6. The witnesses and the challenge

**Witness inputs against STACK_PLAN.md / STACK_INVENTORY.md §3.** Each was run as its own `cargo test … --ignored --exact --test-threads=1` process (`evidence/p2/witnesses.txt`). All 8 pass, and every outcome equals I65's:

| Witness | Input | Outcome | Fits the plan? |
|---|---|---|---|
| W1 | milestone, both modes | Successor | yes |
| W2 | the cap-maximal D1 input + depth-16 raw + one quoted string | Fallback(Preparation) in 0.04 s | **partly**: S-2 |
| W2b | W2, solvable | Fallback(Candidate) after the native run, 41.6 s | a useful addition; asserts nothing |
| W3 | source_blocks n05; **inside D1** (1 case, 1 load; RV89 probe) | ExactSelected (T25 runs on the reserved stack) | yes |
| W4 | the milestone with the captured diameter zeroed | Fallback(Preparation) | yes |
| W5 | the dense halves | as above | yes |
| W6 | PHYS-R4 cantilever (`force_scaled=true`) | Fallback(Native) | yes: the force-scaled re-entry |
| W7 | U3's five faults, carried onto the reserved thread | Native; Serializer(Encoding); Staging; Precommit G8; Precommit G1 | yes |
| Headroom | W1 at 1 MiB | Successor | yes |

**RV89's deep publishing witness** (S-2) passes at 4 MiB and 1 MiB in both modes (successor 113,733 / 114,894 B).

**The challenge** (`tests/retained_memory_challenge.rs`). It is its own binary with a live-bytes peak allocator. The peaks reproduce I65's exactly: 533,234 / 540,831 B (milestone) and 13,191,619 / 13,199,237 B (its large input). It measures what it says, with N-3's limits. `challenge_bounds_are_the_profile` ties its literals to the in-build W1 phase.

**The priced gates on real owners** (my probe; a permit is still impossible, so the gate functions are called directly):
- **G-C** with the part-2 caps admits the milestone and my large in-domain input in both modes, with 0 allocations;
- **the G-B late cap** = T11 − T11_late_capture = 117,696 B;
- **the G-C observation cap** = T11 = 155,712 B;
- **the seed cap** = 11,968 B;
- `push_capacity(h)` equals the real `Vec` growth for every h from 1 to 20,000 (u64), and from 1 to 12,000 (Diagnostic).

## 7. The FK and SR exports

- **FK's `retained_resource.rs`** is 98 `size_of` constants, `FORMATION_ALIGN` and `MAX_ALIGN` in a `const` block, plus one `pub mod` line in structural.rs. **SR** gains one `pub const NODE_STRIDE`.
- There is no function body, no allocation, and no visibility change to any type: Node stays private.
- **FK `--lib`** is 480 / 0 / 1 and **SR** is 48 / 0 / 0, with outcome sets identical to base.

## 8. Mutants

All mutants were run in WT/rv89/mut2. Before each one, the sources were restored from a pristine `git archive` of `cba3e9fda7`, and then `cargo test --lib retained_memory` was run, one job at a time.

| Set | Run | Killed | Survived | Compile-only |
|---|---|---|---|---|
| I65 P1 (part 1's 84, re-anchored) | 84 | 83 | 1: "build_status ignores bindings", equivalent by decision 7 | 0 |
| I65 RV (RV89's 24 from part 1) | 24 | 23 | 1: V19, build.rs, not observable in-crate; RV89's standalone run D covers it | 0 |
| I65 P2 (part 2's 40) | 40 | 40 | 0 | 0 |
| **RV89 Q01–Q12** (`evidence/p2/mutants_rv89_p2.py`) | 12 | 10 | 2: Q08, equivalent here (N-5); **Q10** (S-3) | 0 |

- **I65's 148 match its record exactly** (`evidence/p2/mutants_i65_p2_rerun.jsonl`).
- **My 12, and what kills them:**

| Mutant | Result | Killed by |
|---|---|---|
| Q01: W3 (sparse) drops the staged copy | KILLED | `profile_in_build_record`, `profile_transcribes_the_python_chain_exactly` |
| Q02: W4 (dense) drops the invocation Value | KILLED | `profile_in_build_record` |
| Q03: T25 drops the carried case | KILLED | `profile_in_build_record` |
| Q04: s(Value) bound to String's size | KILLED | `challenge_bounds_are_the_profile`, `profile_in_build_record` |
| Q05: ESTIMATES counts SourceUpper | KILLED | `profile_laws_hold_in_this_build` |
| Q06: the Estimate gate exempts today's 42 | KILLED | `law_order_reports_the_first_failing_clause`, `profile_laws_hold_in_this_build` |
| Q07: node law with 11 edges | KILLED | `challenge_bounds_are_the_profile`, `profile_in_build_record`, `profile_laws_hold_in_this_build` |
| Q08: Shared<4> drops (4,8) | survived | equivalent in this build (752 = 752) |
| Q09: SR NODE_STRIDE of the wrong type | KILLED | `challenge_bounds_are_the_profile`, `profile_in_build_record` |
| Q10: the maximum skips X1 and X2 | **survived** | S-3 |
| Q11: G-C result-capacity cap is the length cap | KILLED | `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` |
| Q12: G-B late cap is all of T11 | KILLED | `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` |

## Execution record

- **Who.** RV89, TASK (Type 2) under ROOT. No descendants.
- **When.** 2026-10-04, about 05:21–06:15 MDT (about 55 minutes of the 3.5-hour box).
- **Memory guard.** `memguard.sh` PID 5387 was running, and every cargo run and mutant checked it first.
- **Cargo.** The default toolchain (rustc 1.97.1), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` (`--test-threads=1` for the witnesses, the challenge and the probes), and `TMPDIR` in WT/scratch/rv89_u4_g5/tmp. One cargo job at a time.
- **Targets:** WT/targets/rv89/{base, cand, fk-base, fk-cand, sr-base, sr-cand, cand-runner, mut2}.
- **Copies.**
  - WT/rv89/base2 and cand2 are full `git archive`s.
  - WT/rv89/mut2 and pristine2 are core, schemas and fixtures archives.
  - RV89's sweep test was added only to base2 and cand2, and the probe module only to cand2, after the witnesses and the challenge ran.
- **Not run.** No Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`), no installs, no new tooling, and no native, solver-at-scale or DEC-025 jobs. Nothing was written to the system temp directory.
- **Writes.** Only R/REVIEW_RV89/u4_g5_02/, WT/rv89/, WT/targets/rv89/ and WT/scratch/rv89_u4_g5/. The evidence copies here have machine paths replaced by `WT`.
