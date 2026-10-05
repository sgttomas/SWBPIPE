# RV87: independent review of U4 G4 (T16–T19 and the composed admission maximum, l ≤ 128)

**Reviewer:** RV87, TASK (Type 2), dispatched by ROOT. No descendants.
**Candidate:** `R/I65/u4_g4_01/` with `ADDENDUM_L128.md`, at NUM `6796050f47`. Its 92-entry SHA256SUMS verifies OK.
**Source read at:** NUM `6796050f47`. Every core, schema and fixture file there is byte-identical to `a634ac8b53` (U3 grants 1b and 1c). The packet's text-run sites were read at `b1f80234dc`, the packet's arithmetic basis.

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 5 |
| NOTE | 8 |

**The headline.**
- **My implementation reproduces the packet exactly.** RV87's own stdlib model, run under the packet's laws, reproduces every phase and every component of `g4_caps` byte for byte, in both modes, at l = 128 and at l = 192 (`_run_records/compare_packet.out.json`).
- **The corrected maximum still passes.** With the corrections found here, the composed admission maximum at the D1 caps, l ≤ 128, ε = 2 and illustrative strides is:
  - phase **W3**;
  - **3,440,273,816 B = 0.8544 M (sparse)** and **3,459,984,264 B = 0.8593 M (dense)**;
  - **≤ 0.9 M by 183,604,840 B (0.0456 M) sparse and 163,894,392 B (0.0407 M) dense**. The packet stated 197.4 / 177.7 MB;
  - **≤ M by 586.3 / 566.5 MB**.
- **The worst case still passes.** Adding S-2's unpriced text and a 10% stride error together gives 0.8695 M (dense), 123 MB under the rule.
- **No ruling moves.** l ≤ 128 stands. No finding needs an owner decision.

## Findings

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| **S-1** | SHOULD-FIX | `PP/retained_wire.rs:1540`, `:1549`, `:1551–1555`. Packet: PUBLICATION_READER.md §2 table (P2–P4); `_run_records/g4_caps.py:190–195` | **T16 misses a third copy of the run and selection Values.**<br>– `case_v = json!({.."run":run_v,.."selection":selection_v})` deep-copies both, because json!'s `$other:expr` arm is `to_value(&$other)` (serde_json-1.0.149 `macros.rs:278–279`).<br>– The originals are locals of `serialize_selected_from`. They drop only when it returns, which is after `finish()` renders P2–P4.<br>– So at P2 the run and selection exist three times: originals, `case_v`, and the body's json! copy. The packet counts two.<br>– **Missing: RUN.tree() + SELECTION.tree() = 13,252,048 B**, the same at l = 128 and l = 192, because it depends only on n, m, N and P | Add the two trees to P2–P4 in the records and in the expression G5 evaluates. A code-side `drop` is the U1 owner's option, and is not required |
| **S-2** | SHOULD-FIX (route to the T08 owner) | `_run_records/text_args.g4.json` args rule `\.id\b` → `ident`. Sites at `b1f80234dc`: `PP/lib.rs:2728`, `:2730`, `:2738`, `:5292` (×2) and `PP/preview_physics.rs:194`, each with mult 2,115 | **TAV prices six result-id copies at 128 B.**<br>– Each site copies a ResultItem id (`row.id.clone()`, `result.id.clone()`).<br>– The run's own `result_id` class is 1,024 B (longest reached template 872 B), but its rule matches only `result_ref` and `^r\.id$`. Every other `.id` falls to `ident` (128).<br>– **Under-count: ≤ 9,745,920 B in each of TAV_X and TAV_W** (`_run_records/rv87_idclass_scan.l128.out.json`).<br>– With S-1, W3 becomes 0.8568 / 0.8617 M | Extend the result-id rule, or add site_size rows, for result-row id receivers. Rerun TEXT. RV83 and RV84 should be told, since T08 is the repaired item they confirm |
| **S-3** | SHOULD-FIX | `R/I65/u4_g4_01/API_G4.md:18–19` against `PP/retained_memory.rs:376–380` | **API_G4 §1 still has `admit`'s pre-1c signature.**<br>– §1 shows `-> Result<CapturePermit, RetainedAdmissionReport>`.<br>– The code returns `Result<(CapturePermit, RetainedAdmissionReport), RetainedAdmissionReport>`.<br>– ROOT routed recording the new signature to I65 twice: RR "RV85 on U3 grant 1…" (S3) and RR "U6 plan accepted… U3 grant 1c" ("I65 records `admit`'s new signature in API.md §2").<br>– §1 says it replaces API.md §2's code block, but it does not carry the new signature | Replace the signature in §1 |
| **S-4** | SHOULD-FIX | `API_G4.md:59–60` (G-B rows) against API_G4 §1 `LateFacts` (`restrained: &'a [usize]`, `springs: &'a [SpringEntry]`). Hook: `PP/retained_product.rs:3223–3224`. Caller: `PP/lib.rs:5906–5907` | **Two G-B facts cannot be read through the specified hook.**<br>– `restrained_capacity` and `springs_capacity` need `.capacity()`.<br>– The hook and `LateFacts` carry slices, and a slice has no capacity. The caller even rebinds `restrained_dofs` as a slice.<br>– Also, G-C's `late_capture_bytes` source, "`capture.P1_old_source`", is a T11 row label, not a `ProductCapture` field | Either drop the two capacity facts, citing exact construction (e.g. `lib.rs:5906` collects from a TrustedLen map), or specify `&Vec<_>` through the call chain as I61's D-5 edit. Name the G-C late-capture fields |
| **S-5** | SHOULD-FIX | `NOTES_G4.md:81–96` (§5) | **The G5 carry list misses routed items:**<br>– RR "RV85 on U3 grants 1b and 1c", routing **U1 → I65 G5 (optional)**: bind the permit to its invocation, and check linearity structurally, not by text;<br>– the **l = 128 census constant** (RR "U4 G4: … l ≤ 128 adopted"). It is stated only in ADDENDUM §4;<br>– RR "RV83 on U4 G2" B-3/S-5: **a G5 test proves that the compiled identity value carries every key in order** | Add the three items to the G5 carry list |
| N-1 | NOTE | `g4_caps.py:185`, `:182–184`, `:272–274`, `:153–155`. TRANSFER_COMPLETION.md §1 | **Small law corrections, all included in RV87's figure** (combined net effect at W3: +0.51 MB):<br>(a) a `to_value` array is `with_capacity(len)` (`value/ser.rs:233–236`), so the selected-diagnostic push doubles it. Growth is D_env slots, not PushCap(D_env+1)−D_env: +74,752 B at l = 128;<br>(b) the staged clone omits `ResultItem.source_result_refs` Vec backings (≤ 4 · s(String) per row; Text(row) counts content only): +203,040 B;<br>(c) `row_ids` uses `collect::<Option<Vec<_>>>`, which grows by doubling: +47,544 B;<br>(d) BODY's PREPARATION grammar omits `numeric()`'s 10 `e.count` objects and 7 entry slots, plus 4 work counts (`retained_wire.rs:249–252`, `:1264–1290`): +628,768 B per BODY tree;<br>(e) N1 is priced as push headroom (1,069,042 B). The code's `try_reserve_exact(1)` grows by at most one slot, so the exact figure is 894 B of strings plus ≤ 152 B, and the moving term is unchanged | Fold into the G5 expressions |
| N-2 | NOTE | `P/core/reporting/result_export/src/retained_precision.rs:1694–1708`, `:1722–1745` (called at `:914`, `:1750`, `:1804`, `:1822`, `:1846`); `:33–39` (`RowClassification`) | **Two reader costs are unpriced, both far below the peak.**<br>– PUBLICATION_READER §3 says the reader's walkers "do not allocate in proportion to the counts". `objects` and `located` do: `out.push(o)` per object, and `out.push((path.clone(), o))` per object. That is a few MB in V3–V4.<br>– `RowClassification.basis_ref` is a cloned Value object: +1,556,640 B in V4–V6.<br>– Neither is at the maximum, because V2 dominates by about 950 MB | Correct the sentence. Add both terms to V4–V6 |
| N-3 | NOTE | API_G4.md §2 (G-C) | **G-C does not cross-check two string-length atoms.**<br>– It reads no fact for the longest envelope string, L_PUB = 2,549,385, which sizes T16 and T17's per-string temporary and parser scratch.<br>– Nor for diagnostic-id length, L_DIAGID = 2,330, which sizes BODY's `diagnostic_refs`.<br>– Totals do not bound a single string against its atom | Add allocation-free maxima at G-C |
| N-4 | NOTE | `text_budget_W.caps.l128.out.json` rows, e.g. `PP/retained_product.rs:2166` (`return Err(..)`, mult 1,218,240) | **About 0.22 GB of TAV_W is a safe over-count.** It is error-return and eager-argument text inside loops, priced once per iteration. The reader already uses a per-call `site_from` rule for its error paths | An available lever, not a defect |
| N-5 | NOTE | `PP/retained_wire.rs:1545` | **One TAV row is misclassified, harmlessly.** `source.clone()` is inventoried as `clone_text` at 128 B. It is a whole-Value clone, and T16's P1 prices it correctly (SOURCE.tree() × 2) | Reclassify, or leave |
| N-6 | NOTE | COMPOSITION_G4.md §1 (X2); `g4_caps.py:313–314` | **X2's proxy for the receipt is conservative.** X2 prices "the retained receipt" as the W1 BODY tree (about 50 MB). The X envelope carries T25's receipt instead, whose body tree is about 30 MB at the T25 facts | Reword |
| N-7 | NOTE | TRANSFER_COMPLETION.md §7 | **Two citations drift.**<br>– "before `prepare_case` (:3103)": `:3103` is the `ReservedNotice::reserve` call; `prepare_case` is at `:3106`.<br>– `w1_case_id` begins at `:3073` | Fix the citations |
| N-8 | NOTE | `g4_caps.py:51–54` | **ASSUMED s(MechanicsEnvelope) = 640 B is low** against a field sum of about 776 B. It is used only as a stack value in STAGED and inside s(ThreadPacketOutput) = 2,048, which still bounds RV87's estimate of s(output), about 1.4 KB | G5 measures it |

## 1. The composed admission maximum under l ≤ 128

**Method.** `_run_records/rv87_g4.py` is RV87's own stdlib code. It does not import or run the packet's scripts.

**What it takes from the chain.** Only the output numbers of the terms that other reviewers own:
- O and T25: their symbolic forms, evaluated by RV87's evaluator, which reproduces `O_req` and `T25_requested` exactly;
- TAV_X and TAV_W: re-summed from their per-site rows, which match the totals;
- T11–T15;
- the text atoms;
- the ASSUMED strides: the illustrative dict literals, read as data.

**What it re-derives from source at `6796050f47`.**
- the Value laws: `to_value` and `json!` exact, `checked_parse` push-built, the text length;
- the hash route: `domain_hash` plus `canonical_json_checked_v1_text`, `canonical_json/src/lib.rs:39–150`;
- ENV, ENV_S, BODY, SUCC and INVOC;
- T16 P1–P4, T17 V1–V6 and STAGED;
- N1 and T19;
- the phases.

**Phases** (E_mov + R, fraction of M; RV87-corrected):

| Phase | Sparse | Dense |
|---|---|---|
| X1 ordinary span + T25 | 3,202,811,763 (0.7954) | 3,222,522,211 (0.8003) |
| X2 X completion | 1,724,262,601 (0.4282) | 1,743,973,049 (0.4331) |
| W1 ordinary span | 1,828,027,943 (0.4540) | 1,847,738,391 (0.4589) |
| W2 G-B, G-C, T12–T15, N1 | 1,895,965,669 (0.4709) | 1,915,676,117 (0.4758) |
| **W3 T16 + staged copy** | **3,440,273,816 (0.8544)** | **3,459,984,264 (0.8593)** |
| W4 T17 + successor + invocation + statics | 3,419,371,868 (0.8492) | 3,439,082,316 (0.8541) |
| W5 transfer and Direct completion | 2,073,614,818 (0.5150) | 2,093,325,266 (0.5199) |

**W3 against the packet:** +13,765,917 B. Of that, S-1 is +13,252,048 and N-1 is the net remainder. X1, W1 and W2 equal the packet's figures, apart from N1 exactness.

**Phases are complete.**
- Branch X is G-A, then the observed run with T25 inside, then the coexistence return, then the transfer.
- Every W fallback (serializer, staging or precommit) is dominated by W3 or W4, because `publish` only pushes into reserved capacity.

**Which figures rest on ASSUMED strides** (W3, dense):
- **Stride-borne bytes, 311,892,784 B (9.0%):**
  - T16 P2: 146.5 MB, from the s(Value) and Node(String,Value) terms of its five Value trees;
  - O_base: 98.0 MB;
  - T12–T15: 65.0 MB;
  - STAGED: 2.4 MB.
- **Stride-free bytes:** TAV_W (1.51 GB), the hash-route text (≈ 4e, about 757 MB) and the string content.
- **Sensitivity:**

| Strides | Sparse | Dense | Dense margin to 0.9 M |
|---|---|---|---|
| −10% | 0.8470 M | 0.8516 M | — |
| +10% | 0.8618 M | 0.8670 M | 132.7 MB |
| +10%, with S-2 | 0.8643 M | 0.8695 M | 123.0 MB |

- A 10% stride error moves the maximum by about ±31 MB (±0.0077 M).

**At l = 192, for reference.** The corrected figure is 0.9151 / 0.9200 M. That confirms the G4 trip, and the ruling's lever was needed.

## 2. T16, publication (`PP/retained_wire.rs` at `6796050f47`)

**The staged envelope.** `serde_json::to_value(overlaid)` (`:1524`) gives an exact-capacity tree. `successor_envelope` (`:740–773`) then adds:
- `recovery_method` (15 B key, 41 B METHOD) on each of the case's rows;
- identity 64 and profile 31;
- one selected diagnostic: SELECTED_CODE 27 and SELECTED_MESSAGE 272, both within the packet's classes;
- the diagnostics array's doubling push (N-1a).

**The live set at P2** (`finish`, `:1436–1455`): env, the members (parameters), the body's json! copy, LOCALS16, the caller-frame `run_v` and `selection_v` (S-1), and the publication hash route over ENV_S.

**The hash route is checked** against `canonical_json_checked_v1_text`:
- text ≤ max(128, 2e) (`to_vec`, `ser.rs:2217`);
- the checked parse with `seen` key clones;
- the canonical String ≤ max(8, 2e);
- the per-string `to_string` temporary ≤ max(128, 2(2L+2)), with L = 2,549,385 and ε = 2. D1.11 leaves only 2-byte escapes; producer templates' `\n` and `\t` are also 2 bytes;
- the escape scratch ≤ 2L.

**Old and new output growth:** the moving term is the text's last growth, e = 189,281,595 B (RV87's exact strings). The canonical String's last growth is the same size, and only one can be in flight at a time.

**No double count against T25's shared route.**
- T25 is priced only on X, and W3 uses O without T25.
- `serde_json::to_string` inside `domain_hash` is not a TAV site.
- The serializer's TAV share is only `bits` (14,433 × 32 B), `sha_hex`, the selected id, the case id and two short copies. They are transient `format!` Strings whose Value copies are in the trees: a conservative double, not a gap.
- Branch totals are per branch (TAV_W re-sums to 1,511,208,042).

**Grammar spot-checks** against the builders:
- `case_source` (`:896–987`), `selection` (`:1130–1185`), `block_refusal` (`:433–450`), `finish`'s top and `ordinary_value` (`:1412–1434`) all hold;
- PREPARATION and OPERATIONAL are slightly low (N-1d).

## 3. T17, the precommit reader

**Called once** at `PP/lib.rs:3146`, with `&successor` and `json!({"request": raw, ..})`. That is a deep copy of the raw request, 15,782,080 B, dropped at `:3149`.

**`integral_receipt` is borrowed.** `floats()` (`retained_precision.rs:251–289`) is false, because the producer's `safe_integers` (`retained_wire.rs:1310`) refuses any float in the body.

**V2 is the peak** (`:591–603`): the whole-successor clone (172.2 MB), then `remove`, then `hash(publication)` over ENVP, 1,148,567,839 B. Every other stage is ≥ 950 MB lower:
- V1, the receipt hash, 321 MB;
- V5, `project()` plus `for_source`, 177 MB;
- V6, G8, 197 MB.

N-2's walkers and `basis_ref` nodes do not reach V2.

**The statics are counted once and bound by D-6.**
- **13 `OnceLock` statics are reachable:** 3 in `retained_precision.rs:325–351`, 8 in `semantic_contract.rs`, `physics_source.rs:39` and `source_blocks.rs:71`.
- **RV87 parsed each file at `6796050f47`.** Its SHA-256 equals ORIGINS.json, and its Value facts equal the packet's, for 5,397,696 B.
- **First use is W4.** PP's only production call into result_export is the precommit call (`lib.rs:3146`), so W4 and W5 count the statics and nothing earlier does.
- **The two other inputs are unreachable from `validate`:**
  - `load_reference.rs:334`'s transport schema;
  - `physics_evidence.rs:1005`'s per-call YAML parse.

  Both are reached only from the transport-metadata entries.
- **The two `include_bytes!` constants** (`:491–495`) are the same files as two of the 13.
- **So the D-6 extension's 13 hashes suffice for T17.**

## 4. T18 and T19 (U3's code at `6796050f47`)

**D-b, the staged copy.**
- `FrozenCandidate::staged_envelope` (`retained_product.rs:3799–3803`) is a clone plus `apply_prepared_overlay` (`:3769–3783`). The values are written in place, and the Number clones need no heap.
- The two LocatedQuantity clones replace the old ones.
- On a fault, `?` drops the copy inside the function.
- `drop(staged)` comes right after `serialize_frozen` (`lib.rs:3130`).
- **105,778,121 B at l = 192 is confirmed, plus N-1b.**

**N1.**
- `ReservedNotice::reserve` (`lib.rs:3034–3051`) runs after the coexistence and G-B checks. Its call is at `:3103`, and `prepare_case` follows at `:3106`.
- The id is a `format!` of at most 2·(30+ID+12).
- `try_reserve_exact(1)` sits on the typed diagnostics.
- The message reservation is 135 + 35 + 25 + 1 = 196.
- `RECEIPT_ENCODING_DETAIL_MAX` = 25, the length of `work_counter_inconsistent`.
- `publish` (`:3054–3068`) pushes into reserved capacity only.

**Transfer.**
- The success path is `drop(invocation)`, `drop(notice)`, then `(frozen.into_ordinary(), Ok(RetainedSuccessor(successor)))` (`:3149–3153`). These are moves only.
- `into_publication` moves and `successor()` borrows (`:2235–2248`).
- **B-1, one ordinary run, holds:** a spawn failure leaves the slot `Some`, and Domain returns before the observed run.

**T19, completion.**
- `on_reserved_stack` (`:2943–2954`) allocates the scope, Thread, Packet and boxed-closure Arcs. The closure captures `report` by value, about 0.6 KB.
- The output moves through the Packet.
- s(output) = s(RetainedPreviewOutput). Its parts are the envelope, about 776 B; the inline `Option<RetainedAdmissionReport>`, about 560 B; and the retained `Option<Result<…>>`, about 56 B. The total, about 1.4 KB, is within the illustrative 2,048.
- **T19 = 10,240 B holds.**

## 5. Text work: 27 sites spot-checked at `b1f80234dc`

**Each line matches its stated kind and function.**

| Area | Sites |
|---|---|
| retained_wire.rs | `:131`, `:191`, `:763`, `:1340`, `:1453`, `:1545` (N-5) |
| retained_product.rs | `:1246`, `:2166`, `:2173`, `:2296`, `:2332`, `:2351`, `:2383`, `:2404`, `:2846` |
| PP lib.rs | `:2480`, `:2484`, `:2488`, `:2728`, `:2873` |
| result_export | `retained_precision.rs:1738`, `:2622`, `:3250`, `:3280`; `preview_physics_evidence.rs:149`, `:291`, `:385`; `semantic_contract.rs:54` |

**Pricing is conservative, with one exception.** Every site is priced at or above its true size, except the result-id copies (S-2):
- error returns in loops are priced per iteration (N-4);
- constants are priced at the static class;
- `{axis}` is priced at 401 against about 95.

**Every ADDENDUM §5 erratum is confirmed against the outputs:**
- retained_product.rs: 207 sites, 601,040,330 B (of which `from_literal` 7,805,434);
- 2,797 / 2,716 / 1,426;
- `from_literal`: 53, 7,886,920;
- 56 zero-matched headers.

**The l = 128 shares match:** 463,890; 600,843,722; 11,692,824; 264,343,582.

## 6. API_G4.md

**Every listed G-C fact is a borrowed length or capacity read,** allocation-free. The `contract_evidence` census is a stack-only walk.

**The S-6 hook additions compile under NLL as specified:**
- in `prepared_case_source` (`retained_product.rs:3244–3246`), `capture: &*self` sits beside `self.permit.as_ref()`, and the borrow ends before `self.late_refusal = ..`;
- in `permitted_run` (`lib.rs:2999`), `capture: &observer` sits beside `observer.permit()`, and the scrutinee is owned, so the `retained_w1(observer, ..)` arm can move.

**Gaps:** the stale `admit` signature (S-3), two G-B facts that cannot be read through slices (S-4), and no G-C cross-check of the L_PUB and L_DIAGID atoms (N-3).

**Bounds at l = 128:** 2·Text(diag_env) = 137,419,080 and T11 = 166,000 (125,424 + 40,576) match the addendum.

## 7. NOTES_G4.md §5, the G5 carry list

**Present:**
- 1–3 (RV83's G5 notes);
- 4 (F-5, Headless);
- 5–11 (the D1.10/D1.11 census, reader layout witnesses, static hashes, the deepest schema chain, PhaseFact, the N1 append test, the admission constant).

**Missing** (S-5):
- RV85's optional "bind the permit to its invocation; structural linearity";
- the l = 128 census constant (in ADDENDUM §4 only);
- RV83 B-3/S-5's G5 key-order test.

**Also worth listing, not a finding:** I61's S-6 hook edits are a G5 integration dependency (API_G4 §3).

**Checked and confirmed:** RV82-N5 (NOTES §2), and the D-6 lock facts. The PP `Cargo.lock` at `6796050f47` has SHA-256 `4f494db6…475b`, with 37 packages, 22 from the registry, and serde_json 1.0.149. No Cargo file changed from `b1f80234dc` to `6796050f47`.

## Execution record

**Who.** RV87, TASK under ROOT. No delegation.

**Memory guard.** `memguard.sh`, PID 5387, was running at start and at seal.

**Git reads only, all with `GIT_OPTIONAL_LOCKS=0`:** `rev-parse`, `log`, `diff --stat`, `status`, `show`. NUM HEAD moved during the review to `f5d106fb41`, through three records-only commits. No core, schema or fixture file differs from `6796050f47`.

**Not run.** No Cargo, install, new tooling, solver, native or DEC-025 job.

**Writes.**
- `R/REVIEW_RV87/u4_g4_01/`: this REVIEW.md, SHA256SUMS and `_run_records/`.
- `WT/scratch/rv87_u4_g4_01/`:
  - a `git show` copy of six `b1f80234dc` sources;
  - `lead_breakdown.py`. It exec'd the packet's `g4_caps.py` model once, only to locate the dominant sub-terms. No number in this review comes from it.

**Commands** (from `_run_records/`):
- `python3 rv87_g4.py <packet _run_records> <P> 128 rv87_idclass_scan.l128.out.json`;
- the same with `192`;
- `python3 compare_packet.py <packet _run_records>`;
- `python3 rv87_idclass_scan.py <packet _run_records> <P at b1f80234dc> .l128`.

**Not opened:** RV83 and RV84's G4 confirmations, I61's tree, and I66's files.
