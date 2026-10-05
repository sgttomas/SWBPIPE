# RV89: independent review of U4 G5 part 1 (the admission law in code)

**Reviewer:** RV89, TASK (Type 2), dispatched directly by ROOT. No descendants.
**Candidate:** `1e323058f3` on `codex/piping-f2a-memory-20261004`, against base `8abb5274a9`. Both were taken by `git archive` into WT/rv89/, and WT/f2a-memory's working tree was not used.
**Basis read:** the brief `BRIEFS/RV89_U4_G5_REVIEW.md`; I65's `R/I65/u4_g5_01/` (RETURN, IMPLEMENTATION, R4_CALLGRAPH and the run records); DOMAIN.md, BUILD.md, G2_AMENDMENTS.md, API_G4.md, NOTES_G4.md §3 and §5, ADDENDUM_L128.md, and G4's ORIGINS.json; and every U4 ruling from "U4 plan: decisions…" through "U4 G5 part 1 verified…" (RR:8865–9729).
**Oracles:** RV89's own. These are the sweep, the probes, the mutants and the standalone build-script runs below. I65's tests were only run, never used as the reference.

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 1 |
| NOTE | 6 |

**The headline.**
- **No permit can exist, and nothing published changes.**
  - `REGISTERED_PROFILES` is `&[]`. `admission()` is the only `CapturePermit` construction, and it requires both no refusal and an index that `REGISTERED_PROFILES.get` resolves.
  - The bound is `Unpriced`, and both gates' unpriced bounds are 0, so a hypothetical registration would still be refused twice more.
  - RV89's own sweep is **byte-identical to base**: 71 inputs × 2 modes × 5 routes, 4,022 lines including every public admission-report field.
  - The PP, runner/headless and result_export outcome sets equal base, apart from I65's 23 added tests.
- **D1.0–D1.11 match DOMAIN.md as amended, in order, with the ruled refusal kinds.** RV89's own boundary inputs (cap and cap + 1 for 20 facts, plus the D1.10 and D1.11 edges) all behave as the derivation says.
- **D-6 behaves as designed.**
  - The self-contained SHA-256 agrees with `sha2` and with the NIST vectors on 3,270 inputs, up to 16 MiB.
  - The escaping round-trips for every byte and every byte pair, and hostile values cannot forge a key.
  - The build script never fails, under 12 crafted environments.
  - The 14 reviewed-input hashes equal G4's record.
- **Mutants.** I65's 84 reproduce exactly: 83 killed and 1 equivalent survivor. RV89 added 24 of its own: 17 are killed and 7 survive I65's suite.
  - Six of the seven survivors are killed by RV89's probes, which pass on the candidate. So the code is right and only the committed tests are thin.
  - The seventh survivor is a build-script edge, which the standalone runs check.
  - None of the mutants was killed only by a compile error.

## Findings

| ID | Sev | Where (candidate) | Evidence | Remedy |
|---|---|---|---|---|
| **S-1** | SHOULD-FIX | `PP/src/retained_memory.rs:716` (D1.3 expansion laws), `:808` (Σ restraint capacity row), `:361` (request materials in the typed walk), `:351` (temperature-point ids). Tests: `PP/src/retained_memory_law_tests.rs` | **Four weakenings of the D1 predicate survive the committed tests.** These are RV89's mutants V03, V06, V07 and V08 (`evidence/mutants_rv89.jsonl`):<br>– **V03** changes D1.3's `any(law not Absent)` to `all(…)`. A request with two or more materials, only one of which authors `expansion_laws`, would then pass D1.3. I65's test uses the one-material milestone, where `any` and `all` agree;<br>– **V06** makes `RestraintCapacityTotal` read the per-support maximum, so Σ capacities above 192 with every single capacity ≤ 192 would pass;<br>– **V07** drops `request.materials`, and **V08** drops temperature-point ids, from the nested typed walk. Their Strings then leave the typed length and capacity rows.<br>The production code is correct: RV89's probes pass on the candidate and kill all four (`evidence/mutants_rv89_survivors_with_probes.jsonl`). Control 3 of I65's brief asks for "mutants for every D1 clause" | In part 2, add committed tests for:<br>– a multi-material request with one authored law, in both the model and the request lists;<br>– Σ restraint capacity > 192 with every support ≤ 192 (for example, one support's restraints `reserve_exact(100)`: total 292);<br>– a 129-byte, or over-capacity, String in a request-level material and in a temperature-point id.<br>Re-run V03, V06, V07 and V08. Test-only: no published byte changes |
| N-1 | NOTE | `retained_memory_law_tests.rs:379`; `:256–261` | **Two of I65's 83 kills are text pins, not behaviour.** "D1.8 deletion" is killed by `include_str!(…).contains("return refuse(C::Members, F::Components);")`, and "admission ignores the refusal" by the structural source-text assertions. I65's own harness lists both as equivalent today (D1.4 refuses first; nothing is registered). The accurate tally is 81 behavioural kills, 2 text-pin kills and 1 equivalent survivor | Record the tally that way in part 2's mutant table. No code change |
| N-2 | NOTE | `retained_memory.rs:1143–1145` (`Bytes::get`); `:1180–1183` (`longest_string`'s diagnostic arm); `PP/build.rs:52–60` (`field`) | **Three smaller survivors of the committed tests:**<br>– **V17:** an overflowed gate sum reads 0 instead of saturating. This is fail-open in principle, though unreachable in practice on 64-bit capacities;<br>– **V24:** G-C's longest string ignores `affected_refs`;<br>– **V19:** build.rs accepts a duplicated `rustc -vV` field.<br>RV89's probes kill V17 and V24, and the candidate passes them. V19 is not observable in-crate; RV89's standalone run D shows the candidate emits `v1;unavailable` for a duplicated field | Optionally add the two in-crate tests in part 2 (saturation of `Bytes`, and a long `affected_refs` or `source`). V19 needs no test |
| N-3 | NOTE | `retained_memory.rs:919–928` (`READER_LAYOUTS`); I65 RETURN, carry item 6 | **Carry item 6 is only partly done.** NOTES_G4.md §5 item 6 asks for layout witnesses of `Validation`, `ValidationError`, `RowClassification` **and "the reader's set and map entry tuples"**. `READER_LAYOUTS` covers the first three plus `AccuracyClass`, but no entry tuple, and RETURN marks the item "done".<br>The tuples in `result_export/src/retained_precision.rs` are std compositions: `(u64,&str)`, `(String,String)`, `(String,u64)`, `((usize,usize),Vec<usize>)` and `(&str,u64)`. The D-6 compiler identity pins their layouts, so the residual risk is small | Part 2 either records their size and alignment beside `READER_LAYOUTS`, or states in the record that the identity pins them |
| N-4 | NOTE | `retained_memory.rs:966–974` (`bindings_hold`) against `:958–961` (`identity_match`) | **The reviewed-input comparison has no `unavailable` guard.**<br>– `identity_match` refuses `v1;unavailable` explicitly.<br>– `bindings_hold` compares the reviewed-input text byte for byte only.<br>– A build that cannot read an input still emits a valid identity, with `<path>=unavailable` in the reviewed record (`evidence/build_rs/run_bs.out`, cases H and I).<br>If G6 ever registered such a text, a later build with the same unreadable inputs would match. Today this is unreachable (nothing is registered) | A one-line refusal of any compiled reviewed-input text containing `=unavailable`, inside U4's fence, or a G6 registration check. ROOT's choice |
| N-5 | NOTE | `retained_memory.rs:1440–1452` | **The public report's derived `Debug` and `PartialEq` now include the private `law`.**<br>– Nothing serializes `RetainedAdmissionReport`: it has no `Serialize`.<br>– No production code formats or compares it. runner/headless only holds it (`runner/headless/src/lib.rs:751–757`).<br>– RV89's sweep compares each public field separately, byte-identical to base | None now. Do not let a later consumer publish the report's `Debug` text |
| N-6 | NOTE | `R/I65/u4_g5_01/R4_CALLGRAPH.md` §2 | **One R-4 sentence is inaccurate, but the totals hold.**<br>– §2 says load_ledger.rs:131 `push` "carries no text". The R-4 text run (`r4/text_budget.caps.r4.out.json`) has an `into_text` site there: line 133, 128 B, multiplicity 915,466,156 → 915,000,428, with `req` = 163,840 B both before and after.<br>– So TEXT's total (2,044,161,940 B) and the claim "TEXT unchanged" hold.<br>– Also, `function_multiplicity` gains 10 entries, which the record does not mention: eight PP functions already reachable at G4 (six `typed_trace`, `summary_coverage` and `project`) and two of the newly reachable final_case functions. None carries a text site; `sites_with_positive_multiplicity` is 1,426 in both runs | Correct the sentence in part 2's record |

## 1. No permit can exist, and nothing published changes

**By reading** (`retained_memory.rs`):
- `RegisteredProfile` (:933–943) has private fields, no constructor and no `Default`. `static REGISTERED_PROFILES: &[RegisteredProfile] = &[]` (:946).
- `CapturePermit { _profile }` is built in exactly one place, `admission()` (:1499–1507): `(None, Some(profile))` from `report.law.registered.and_then(|i| REGISTERED_PROFILES.get(i))`.
- Only `retained_memory` and its child modules can name the private fields: `build_identity`, `law_tests` and `tests`. A search of all of them finds no `RegisteredProfile {…}` or `CapturePermit {…}` literal outside the definitions and that one construction. **There is no test permit.**
- **There is no `unsafe`** (nor `transmute`, `static mut` or `MaybeUninit`) in any added line of the diff.
- **Fail-closed in depth:**
  - `cap_priced_maximum` returns `Err(Unpriced)` (:1014–1016);
  - the gate bounds marked `UNPRICED` are 0 (:1267). RV89 observed G-B refusing at `LateObservationBytes` (2,358 / 2,949 B > 0) and G-C at `EnvelopeResultCapacity` (128 > 0) on the milestone, in both modes (`evidence/outcomes/cand_probe_printed.txt`).
- **The fence holds.** The diff touches exactly the 7 files listed in the brief. The only `lib.rs` and `retained_product.rs` changes are the two authorized struct-construction edits (`CompleteFacts { …, capture: &observer }` at lib.rs:3006, and `LateFacts{…,capture:&*self}` at retained_product.rs:3245). `Cargo.lock` is unchanged, and no dependency or build-dependency is added.

**RV89's sweep** (`evidence/zz_rv89_sweep.rs`, `evidence/inputs/`, `evidence/sweep.tsv`):
- **The inputs:** 71, of which 69 parse and 2 are invalid (parse refusals).
  - 33 fixture requests and models: every product_preview `*.request.json`, the source_blocks and ui sets, the dec092, torsion and invented models, and PP's exact-pressure request. The non-deterministic `rejected_stress_range` pair is excluded (RR "U6a verified", F1).
  - 38 RV89 variants: the milestone across D1 clauses, D1.10 and D1.11 edges, the text, key, depth and value limits, and an RV89 cap-maximal request with seven cap + 1 variants.
- **The routes:** R1 the ordinary Value entry; R2 the typed entry; R3 retained Direct (envelope, `successor()`, every public report field, `into_parts`); R4 retained Headless (the same); R5 `into_publication`.
- **The result:** 4,022 lines, and **the base and candidate files are byte-identical** (sha256 `e1677d736f60e6a1b63e8eccc2e2fc9b0265187b507d6767e266a62eed6d0a3d` for both).
  - Every report has `profile: Missing`, `allowance: Unselected` and `successor: None`, and every publication is `Ordinary`.
  - Eight reports (deep and over-limit inputs) have `census_complete: false`, identically at base.
- **The new field `law` is private and not serialized:** `RetainedAdmissionReport` derives no `Serialize` (see N-5).

**The suites** (default toolchain rustc 1.97.1, `--locked --offline`; `evidence/outcomes/`):

| Suite | Base | Candidate | Difference |
|---|---|---|---|
| PP (lib and every integration target) | 661 passed, 1 failed, 2 ignored | 684 passed, 1 failed, 2 ignored | The 23 `law_tests` only. The failure is the known Mac `t13_committed_fallback_uz_is_byte_identical` in both. The ignored count includes RV89's sweep, which is ignored by design |
| runner/headless | 85 passed, 2 failed | 85 passed, 2 failed | none. Both failures are at base too |
| result_export | 149 passed | 149 passed | none |

The lib warnings are 8 at base and 8 in the candidate.

## 2. D1.0–D1.11 against DOMAIN.md (l ≤ 128, no sections, D1.10, D1.11)

**By reading** (`domain_clauses` :869–876, `law_order` :1511–1522):
- **The order:** D1.0 (`caller_clause`), D1.1 (`build_status`), then D1.2 (census: raw, raw text, typed DOF, nested walk, and Headless roots when present), D1.3–D1.8 (`family_clauses`), D1.9 (the first 45 cap rows), D1.10 (`provenance_clause`), D1.11 (the last row), then the bound. This is DOMAIN.md's numbering order.
- **Each clause is exact**, as DOMAIN.md §1 with G2_AMENDMENTS §2 and §3 and ADDENDUM_L128 §4 state it:
  - D1.3: schema ∈ {0.1.0, 0.2.0}; no pressure contract; reference configurations Absent; every material expansion law Absent; no request expansion laws; no sections; no `section_ref`;
  - D1.4: one case; no combinations or components;
  - D1.5: the five case fields;
  - D1.6: no hanger or nonlinear support, and family None or one of the five exact strings;
  - D1.7: Node target; dimension force or moment;
  - D1.8: re-states D1.4.
- **The cap table matches** DOMAIN §2 and the amendments row by row:
  - n, m, g ≤ 32 with capacities;
  - Σr ≤ 192, plus each and Σ restraint capacities ≤ 192;
  - s ≤ 192; l ≤ 128 with capacity;
  - materials ≤ 4 each with capacity; temperature points ≤ 16 with capacity;
  - load_cases capacity ≤ 1;
  - sections, components, combinations and request-expansion capacities 0; material-expansion capacity ≤ 4;
  - typed and raw text, and raw keys, ≤ 128;
  - raw values ≤ 16,384, depth ≤ 16 (root at 0, 17 levels: N-12);
  - raw totals 65,536; raw capacities 32,768 / 131,072 / 131,072;
  - digest ≤ 128;
  - the `project.units` Value within the raw caps;
  - control bytes = 0.
- **The refusal kinds:** D1.0 → `caller`; D1.1, D1.2, D1.9, D1.11 and the bound → `resource_admission`; D1.3–D1.8 and D1.10 → `source_family`. This is DOMAIN §3 as amended, and RR "U4 G4" (D1.10 and D1.11).
- **Headless is refused at D1.0**, before the build status. The domain verdict is still recorded privately.
- **The values the gates and caps rely on reproduce from G4's l = 128 records:** D_env 9,360; Text(diag_env) 68,709,540; Text(row) 11,474; Text(err) 16,384 (`text_closure.caps.l128.json`); P_final = 7n + 51m + 8g + 3 = 2,115; and the PREVIEW facts (`ordinary_caps.py` `value_tree` arguments).

**RV89's own boundary inputs, through the actual G-A** (`evidence/rv89_probe_tests.rs`; all pass on the candidate):
- **The cap-maximal request** (RV89's own construction, not I65's helper) is inside D1 in both modes. Every count sits at its cap: n = m = g = 32, r = 192, s = 32, l = 128, 4 + 4 materials, 16 points and a 128-byte identifier.
- **Cap and cap + 1, each refusing with the exact `Cap { fact, observed, cap }`:**
  - nodes, members and supports (33);
  - Σ restraints (193), with a single support at 192 admitted;
  - loads (129); model and request materials (5); temperature points in the request list (17);
  - typed text at 128/129 bytes in five different typed owners (node provenance, load category, a support's stiffness unit, a request-level temperature-point id, `analysis_status.rule_check`), also 64 × "é" (128 B) admitted and 65 × "é" (130 B) refused, so the cap is in bytes, not characters;
  - raw-only text (129) and key (129);
  - raw depth 16/17;
  - raw string bytes 65,536/65,537 and raw key bytes 65,536/65,537, each built to land exactly on the total;
  - typed capacities after the parse: load cases 2, primitive loads 129, Σ restraint capacity 292 with every single capacity ≤ 192, a single restraint capacity 193, a String capacity 129, sections capacity 1;
  - units depth 16/17.
- **D1.10:**
  - `{`, `{}`, ` {…}` and `   {` refuse as `Family(D1.10, ObjectProvenance)` on the last load (index 127);
  - so do `\t{`, `\n{`, `\r{` and `\x0c{`: these are ASCII whitespace, and D1.10 precedes D1.11;
  - `\x0b{` passes D1.10, because VT is not ASCII whitespace in Rust, and is refused at D1.11;
  - `x{`, NBSP`{`, `[{}]`, `"{"`, empty, a single space and `}{` stay inside D1.
  
  JSON's whitespace set is a subset of Rust's ASCII whitespace, so every provenance that serde_json would parse as an object is refused.
- **D1.11:** a control byte in a key, 0x7F in a key, 0x7F/0x1F/0x00 in a value (observed 3) and 0x1F inside `project.units` are all refused. Space, U+0080, U+0085, U+009F and "é" are not.
- **The order across clauses:**
  - D1.2 (raw depth 80) before D1.3;
  - D1.3 (schema "0.3.0") before D1.9 (nodes 33);
  - D1.9 (nodes 33) before D1.10;
  - D1.10 before D1.11.
  
  Exact strings: `"spring"` is admitted, while `"Spring"` and `"0.1.0 "` are refused.

## 3. The census extensions

- **They are allocation-free.** This was checked with the PP/tests counting-allocator pattern: a per-thread `#[global_allocator]` in RV89's test module, with a positive control showing it counts.
  - **Zero allocations** for the whole G-A (`assess`, Direct and Headless) on six inputs in both modes: the milestone, RV89's cap-maximal request, an out-of-namespace request, an object provenance, a 0x7F key, and a raw depth of 80.
  - The same for `raw_text_census`, `nested_typed_census` and `borrowed_request_census`.
- **The nested walk matches T03's roster** (RESIDUALS.md T03, with sections removed by S-4) against the type definitions at PP/lib.rs:170–770:
  - every String of `MaterialInput` and `MaterialTemperaturePointInput` in both material lists;
  - `schema_version`, `document_kind`, `project.id` and `project.units`;
  - the three status strings;
  - node `id` and `provenance`;
  - pipe `id`, `from`, `to`, `material`, `section_ref`, `provenance` and the 7 section Quantity units;
  - support `id`, `node`, `family`, `provenance`, the restraint Vec and Strings, and `stiffness.dof` and its unit;
  - case `id` and `provenance`, and each load's `id`, `category`, `direction`, `dimension`, `provenance`, target String and magnitude unit.
  
  The owners it does not read (hanger, nonlinear, pressure, sections, components, combinations, equivalent-static, modulus basis and analysis state) are exactly those that D1.3–D1.6 refuse.
- **The typed capacity caps read actual capacities.** RV89 changed capacities after the parse with `reserve_exact` and `with_capacity`, and each was refused on its own row (§2).

## 4. D-6

- **The build script never fails the build.** RV89 compiled build.rs standalone with rustc, writing to scratch, and ran it under 12 environments (`evidence/build_rs/`). Every run exits 0:
  - **A** normal: the identity equals the cargo-built one. `target.env=` is empty, which is a value;
  - **B** no `$RUSTC`, **C** `$RUSTC` failing, **D** a duplicated field, **D2** an empty field, **E** `CARGO_CFG_TARGET_ENV` unset, **G** a non-UTF-8 variable: each gives `v1;unavailable`;
  - **D3** a rustc field containing `;`, `=`, space and `%`, and **F** `CARGO_ENCODED_RUSTFLAGS` containing 0x1F, `=`, `;`, newline and `%`: each is escaped (`1.97.1%3Bevil%3D1`; `-C%1Fopt-level%3D3%3Bx%0Ay%25`). No key is forged;
  - **H** no `CARGO_MANIFEST_DIR` and **I** the wrong directory: the identity is still valid, and each reviewed input reads `unavailable` (N-4);
  - **K** `CARGO_CFG_DEBUG_ASSERTIONS` absent: `debug_assertions=false`;
  - **J** stdout closed: exit 0.
- **The escaping round-trips** for all 256 single bytes and all 65,536 byte pairs. The output is pure 0x21–0x7E with no `;` or `=`.
  - In 400 random and hostile 16-value identities (`;target=evil`, `\ncargo:rustc-env=X=1`, `%3B`, `é;π=` and others), decoding returns exactly the values, with exactly 16 `;` and 16 `=`.
  - A cargo `rustc-env=` split at the first `=` keeps the whole text.
  - An extra key, a reordered key, a lowercase escape or a `v2` prefix fails to decode.
- **`option_env!` absence means Stale.** `identity_match(None, …)` with something registered is `Stale`; with nothing registered, every compiled value is `Missing`. An exact match is required, so a prefix or a trailing space is `Stale`. `build_status()` is `Missing` on this build.
- **The reviewed inputs.**
  - RV89's `shasum -a 256` of the PP lock and the 13 statics equals G4's ORIGINS.json `statics` and the lock `4f494db6…475b`, in REVIEWED_INPUTS order.
  - The compiled `OPS_RETAINED_REVIEWED_INPUTS` (`cand_probe_printed.txt`) carries the same 14 hashes.
  - That these 13 are the only static inputs `validate` reaches is RV87's finding; RV89 did not re-derive the set.
- **The self-contained SHA-256 against `sha2`, on 3,270 inputs. All agree:**
  - the four NIST short vectors (empty, `abc`, the 448- and 896-bit messages) and the one-million-`a` vector, as literal digests;
  - every length from 0 to 1,100 with random content;
  - lengths 55, 56, 57, 63, 64, 65, 119, 120, 127, 128 and 129, at offsets 0, 64, 128, 4,096 and 65,536, with 0x00, 0xFF and 0x80 fill;
  - 2,000 random lengths up to 20,000;
  - 1 MiB, 3 MiB + 7, 5,000,003 B and 16 MiB + 55 random inputs.
- **The layout witnesses never become a compile error.** `LAYOUT_WITNESSES` (:897–907) is a `const bool` and `READER_LAYOUTS` (:920–928) is a `const` array, with no `const` assertion or `static_assertions` anywhere. A false witness is `Stale` through `bindings_hold`, never a build failure. On this build, `LAYOUT_WITNESSES` holds.
- **The stated residual, unchanged:** the reviewed-lock record hashes PP's own `Cargo.lock`, not the lock that governs a downstream workspace (BUILD.md §2.3; NOTES_G4 §3 item 4).

## 5. The gates

- **`check_late` and `check_complete` allocate nothing.** Their whole bodies (`late_observations`/`complete_observations` + `phase_caps()` + `check_phase`) were run under the counting allocator on the milestone's actual owners (BuiltModel, the observer and the envelope), in both modes: 0 allocations.
- **`PhaseRefusal` carries `{gate, fact, observed, cap}`:** for example `{Complete, EnvelopeResultCapacity, 128, 0}` and `{Late, LateObservationBytes, 2358, 0}`.
- **G-C bounds the longest string at 2,599,962 B and the longest diagnostic id at 2,330 B.** RV89 isolated each row by lifting only the unpriced bounds:
  - a diagnostic message or a result `entity_ref` of 2,599,962 B passes, and 2,599,963 B refuses with exactly `{Complete, EnvelopeMaxStringBytes, 2599963, 2599962}`;
  - a diagnostic id of 2,330 B passes, and 2,331 B refuses with `{Complete, DiagnosticIdMaxBytes, 2331, 2330}`;
  - long `affected_refs` and `source` strings are read too.
  
  The milestone's own longest string is 12,401 B.
- **The two authorized struct edits are the only changes** to `lib.rs` and `retained_product.rs` (§1).

## 6. The bound arithmetic

`bound_admits` (:1018–1025) uses `checked_add`, and admits when `required ≤ threshold`. RV89's checks, with R = 64 MiB and M = 4,026,531,840:

| Input | Result |
|---|---|
| M−1 | `Ok(M−1)` |
| M | `Ok(M)` |
| M+1 | `Exceeds { M+1, M }` |
| u64 overflow | `Overflow` |
| `0, 0, 0` | `Ok(0)` |
| `1, 0, 0` | `Exceeds` |

`cap_priced_maximum` is `Unpriced` in both modes. With a registered index and an in-domain request, `law_order` still refuses with `Bound(Unpriced)`.

## 7. Mutants

All mutants were run in WT/rv89/mut, with sources restored from a pristine `git archive` before each one, using `cargo test --lib retained_memory`, one job at a time. **The unmutated baseline passes, 28 of 28.**

| Set | Run | Killed | Survived | Compile-only |
|---|---|---|---|---|
| I65's 84 (`mutants_g5.py`, unchanged) | 84 | 83 | 1, "build_status ignores bindings", equivalent by decision 7 | 0 |
| RV89's 24 (`evidence/mutants_rv89.py`) | 24 | 17 | 7 | 0 |
| RV89's 7 survivors, re-run with RV89's probes mounted | 7 | 6 | 1 (V19, build.rs only) | 0 |

- **I65's re-run equals its record mutant for mutant** (`evidence/mutants_i65_rerun.jsonl`). Two of its kills are text pins (N-1).
- **RV89's 24 cover:**
  - D1 clauses weakened: V01 (D1.10 trims spaces only), V02 (D1.6 trims the family), V03, V22 (domain after the bound);
  - caps off by one: V04 (depth 17), V05 (text 129), V21 (the last D1.9 row dropped);
  - wrong facts: V06, V07, V08, V09 (D1.11 misses keys);
  - the identity escaping: V10 (`%` passes through), V11 (lowercase hex);
  - the lock-hash comparison: V12 (presence only), V13 (build.rs hashes the lock for every input);
  - gate facts ignored or misread: V14, V15, V20, V23, V24, V17;
  - the bound comparison inverted: V16;
  - the SHA-256 padding edge: V18 (56);
  - the build script: V19.
- **Survivors of the committed tests:** V03, V06, V07 and V08 (S-1), and V17, V19 and V24 (N-2).

## 8. R4_CALLGRAPH.md (spot check; RV83 confirms R-4 in detail)

- **The stated remaining limit holds as a description.** RV89 read two of the cited true-but-dropped calls at `b1f80234dc`:
  - `let folded_force = folded_force.finish(n)…` then `folded_force.values()` (source_recovery.rs:754–760);
  - `let e = pos_lift(e)?` then `e.cmp_value(` (product_certificate.rs:417–424).
  
  Both are same-body rebindings to a different type, exactly the class §3 names.
- **TEXT and the recursion inventory are unchanged, as claimed.** From `r4/r4_compare.out.json` and the G4 and R-4 text runs:
  - TAV is 2,044,161,940 B in both, complete in both;
  - `sites_with_positive_multiplicity` is 1,426 in both;
  - the only changed row is load_ledger.rs:131 (multiplicity only; `req` unchanged);
  - explicit cycles are 22/22 and implicit 40/40, with the same components;
  - the functions reachable from the Direct root go 2,698 → 2,705, none lost.
  
  One sentence is inaccurate (N-6).

## Execution record

- **Who.** RV89, TASK (Type 2) under ROOT. No descendants.
- **When.** About 03:08–03:55 MDT, 2026-10-04 (about 50 minutes of the 3.5-hour box).
- **Memory guard.** `memguard.sh`, PID 5387, was running. Every cargo run checked it first.
- **Cargo.** The default toolchain (rustc 1.97.1, commit `8bab26f4f68e`), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, and `TMPDIR` in WT/scratch/rv89_u4_g5/tmp. One cargo job at a time; the one standalone rustc compile of build.rs ran alone.
  - **Targets:** WT/targets/rv89/{base, cand, base-runner, cand-runner, base-rx, cand-rx, mut}.
- **Copies.** WT/rv89/{base, cand} (full `git archive`), WT/rv89/mut and WT/rv89/pristine (subset archives), and WT/rv89/pristine_probe (three files). RV89's sweep test was added only to the base and cand copies, and the probe module only to the cand copy and the mutant copy.
- **Not run.** No Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`), no installs, no new tooling, and no native, solver-at-scale or DEC-025 jobs. Nothing was written to the system temp directory.
- **Writes.** Only R/REVIEW_RV89/u4_g5_01/, WT/rv89/, WT/targets/rv89/ and WT/scratch/rv89_u4_g5/. The evidence copies here have machine paths replaced by `WT`.
- **Evidence** (`evidence/`): the sweep test, its inputs (with `inputs_manifest.sha256`) and its output `sweep.tsv`; the probe module; both mutant sets and the survivor re-run; the standalone build-script runs; the run scripts; and the suite outcomes and summaries.
