# RV105: independent review of I78's B0 contract design (documents only)

**Reviewer:** RV105, TASK (Type 2), dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. I made no delegation. I am a fresh instance and wrote none of the design. 2026-10-06 UTC.

**Brief:** `R/BRIEFS/RV105_B0_REVIEW.md`, sha256 `5164c0bc8b17d6e3f3e87ee26e6e99184e15e36271d7b99658a61ef29afb58e8`, verified before reading. I read NUM's root `AGENTS.md` (`f96feb19…`) and `agents/AGENT_TASK.md` (`1a13a5b0…`) first, and the work graph's "T3 current route" section (WG `f6cd3740…`) for the owner-held list.

**The candidate:** `R/I78/b0_contract_01/DESIGN.md`, sha256 `25a07a66944922721c9e56fde34afbab98d10bdf4ca5a0888bf7fdede2f98a0c` (verified; its SHA256SUMS 1/1 OK). It answers `R/BRIEFS/B0_CONTRACT_AND_IDENTITIES.md` (`86bdccd2…`, verified).

**Placeholders:** WT, NUM, P, T, R, RR and VENV as in the dispatch. I also use the design's notation: PP, RE, FK, PY, RS, TS, C1–C3, DN, D2, PLAN, PROBE, QUAL, CORPUS and SCHEMA.

**Basis:** NUM `f12fed9d69`. Against the design's basis `ecb541d63f`, the PP, RE and FK sources, CORPUS and SCHEMA are byte-identical. The only changes are T6S's desktop, schema and test files, plus two RE `tests/` files. So every code citation below holds at both revisions. During the review, NUM moved to `dee7700021` (three records-only commits; nothing outside `P/execution` changed, and RR only gained appended lines), so nothing here is affected. Line numbers are at `f12fed9d69`, and I cite symbols wherever possible. All ten input hashes the design lists match the files I read.

**Method:**
- I read documents and code only.
- I ran read-only Python with VENV against committed files: the probe log, the F-1 dump and CORPUS. The scripts are in `evidence/`.
- I made no cargo, vitest, native or solver jobs, no installs and no Git writes. Git reads used `GIT_OPTIONAL_LOCKS=0`.
- Scratch went to `WT/scratch/rv105_b0_01/`. Nothing went to the system temp directory.
- The host accepted writes into NUM at this records folder.

## Verdict: **FAIL as drafted.** One BLOCKING finding; the repair is local

| Severity | Count |
|---|---|
| BLOCKING | 1 |
| SHOULD-FIX | 3 |
| NOTE | 11 |

**The blocking finding, B-1.** T-4, the design's central rule, defines `not_required` two ways, and the two disagree on a class of case that is real and already in use:
- **The seed-shape definition:** `initial` is a Passed report and `w2` is `not_triggered`.
- **The quality definition:** "the same fact as `solve_quality == checks_passed`".

The class is cases whose ordinary attempt failed with Range and W2 then published a **Passed** report. W6, W-C1 (two-body case B), and the one-body pair are all such cases.
- **Under the seed-shape text,** those cases stay in A. F0-1's defect survives: a selected W2-published Passed case still makes precommit refuse the whole successor. DN §4.3 and RR:2190 are not met.
- **Under the quality text,** which the design's own §1.3 and §1.4 rely on, RS refuses every such `not_required` case. Python and TS admit it. W-C2, as proposed, could therefore not pass precommit in either mode. W-C2 is B1's acceptance witness, D38's pin base and F-1's dense base.

**The repair:**
- key T-4 on the published verdict;
- add RS's `not_required` alignment to B1's three-reader change;
- restate §1.3, §1.4 and F0-2 on that basis.

With B-1 repaired, the design would pass with the SHOULD-FIX and NOTE findings below. Everything else I checked is sound:
- the seven findings;
- the n-case transaction's structure;
- Text B;
- D38's relaxation;
- C3a;
- the caps restatement;
- the deciders.

## Findings

| ID | Sev. | Section | Finding and evidence | Remedy |
|---|---|---|---|---|
| **B-1** | **BLOCKING** | §1.2 T-4; §1.3; §1.4; §0 F0-2; decisions 1, 2, 9 and 11 | **T-4's two definitions of `not_required` disagree on W2-published Passed cases. RS refuses those cases as `not_required`.**<br>**Such cases exist.** A case that is CHECKS_PASSED and also carries `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` had `attempt_err`:<ul><li>PP `needs_source_recovery` = `report_sensitive \|\| attempt_err \|\| load_row_finding`;</li><li>a load-row finding demotes Passed to Sensitive (`formation_guard::demote`);</li><li>so the initial attempt failed, and W2 then published a Passed report.</li></ul>This holds in both modes for W6, two-body B (W-C1) and one-body A and B (`evidence/probe_quality.log`). QUAL's W6 row says the same: "the attempt fails Range, then W2 publishes". The F-1 dump shows the shape for the same two-body model: `initial: structural_failure/range`, `w2: published, 518`. The committed helper `u8_two_body_case_b()` reproduces the probed input's sha `cf688351…` (`evidence/input_shas.log`).<br>**(i) Under the seed-shape text,** these cases are in A. One-body A shows that such a case reaches native Selected (PROBE §4). If it certified, D6b (all three readers) would refuse it at precommit and take the whole n-case successor with it. That is F0-1's own defect, not removed. It also contradicts DN §4.3's trigger ("a `Range` error that W2 scaling could not resolve") and RR:2190 ("W1 is selected only for Sensitive and D-5-routed cases").<br>**(ii) Under the quality text,** the design's §1.3 and §1.4 conclusions hold, but RS refuses `not_required` unless `initial.kind == report && outcome == checks_passed && w2 == not_triggered`. PY's `_g5_ordinary` and TS's `ordinaryAttempts` require only the quality and a null attempt (`evidence/reader_rules.txt`). That is a three-reader divergence F0-4 does not list. W-C2's case B is exactly this shape, so W-C2's must-pass bases and the D38 pin fail RS, and the producer's precommit refuses W-C2 in both modes.<br>**(iii)** So F0-2, and §1.3's two "Changes" rows, hold only under reading (ii). Under T-4 as written, W-C1 and W6 do not change, and the briefed two-case W-C2 is reader-legal | **Re-key T-4:** `not_required` ⇔ `numerical_quality.cases[i].solve_quality == checks_passed` (G-C has already established that every case was attempted). Strike "the same fact … the readers already check the two agree". **Add to B1's G8/G5 alignment:** RS's `not_required` rule aligned to PY and TS (drop the `initial`/`w2` conjuncts; keep the `report` outcome equality where `initial` is a report). Pin it with a producer-solved W2-published `not_required` must-pass (W-C2 case B) and a mutation. It is re-qualified with D38 and F-1. **Restate** §1.3, §1.4, F0-2 and decision 1's rationale on this keying |
| **S-1** | SHOULD-FIX | §1.3 (witness audit); §6 step 7 | **The audit misses QUAL §4's W2 (`witness_w2_cap_maximal`) and W2b (`witness_w2b_cap_maximal_solvable`).** Their ordinary quality is recorded nowhere (QUAL §4; the I65 witness logs). W2b is the only S1 witness of a **full native run at the cap-maximal counts**: a realistic 32-member ring with one anchor and 31 springs, which is plausibly Passed with W2 not triggered. If either is Passed, T-4 (under either reading) turns it into a no-W1 fallback, and the S1 stack evidence loses that path. `registered_g_c_declines_only_unattempted_solves`'s attempted examples (K2a deferred formation, the two `rejected_stress_range` fixtures) would still pass, because the test accepts any `Some(_)`. But its doc line, "a solve that ran but failed still reaches W1", may stop holding for any of them that end Passed after W2 | B1's probe records the ordinary quality of every QUAL §4 witness input and every `attempted_examples` input before T-4 lands, and re-bases any that become `not_required`. Add W2 and W2b to §6 step 7's re-based witnesses where they apply |
| **S-2** | SHOULD-FIX | §1.2 T-7 | **T-7 lets a preparation failure keep a registered source** ("`source_ref: null` unless a valid source was registered"). PY `_g5_stages` requires preparation `completed` ⇔ `source_ref` non-null, and TS `productAttempts` requires a source ⇒ preparation `completed`. Both refuse that shape. The producer already fails closed on it (`serialize_unavailable`: `Untranslated`, `sources[].kernel_source_sha256`). (4b)'s obligation list covers only native-failed shapes | State in T-7 that a preparation-stage failure publishes `source_ref: null` and emits no CaseSource, or that it abandons. Alternatively, add the shape to B1's reader obligation beside (4b) with its own rule and pin |
| **S-3** | SHOULD-FIX | §1.2 T-11 (cumulative snapshots) | **T-11 does not define "that attempt's record point", and "successive attempts' snapshots never decrease" is false in `product_attempts[]` (start) order.** Example: case 1's preparation fails at T-7 (snapshot taken then), while the earlier-started case 0 continues through native and T-9 (snapshot taken later). Then snapshot(0) > snapshot(1). C3 §3 asks the multi-case caller to "establish their actual owners and prevent duplicate accounting"; it asks for no order | Define the record point (for example, the attempt's terminal stage). State monotonicity only in record-point order, or drop the sentence. Say that no reader checks an order among snapshots |
| N-1 | NOTE | T-3 (e) | `ordinary_solve_attempted` (PP `retained_memory.rs`) iterates the **existing** seeds. A run that blocks at case k < c−1 leaves later cases without seeds, and the function returns true. T-3 (e)'s words ("any requested case") are right. If B1 reuses the function unchanged, the seedless cases fall into A and receive notices for W1 work that never ran, against G6 ruling 2(a) | B1 counts requested cases: one seed per case, each with `initial` set |
| N-2 | NOTE | T-4 vs DN §4.3 | DN §4.3 also says the trigger never fires for `Mechanism`, `Asymmetric` or `InvalidInput`. T-4 places every failed attempted solve in A, so those cases get a T-6 notice on the blocked envelope, as D1 does today | State whether B1 keeps D1's behaviour for them or applies DN's exclusion (no notice) |
| N-3 | NOTE | Outcome table; T-11 | `StackReservation` (exact bytes) and the unreachable `PermitUnbound` are unchanged D1 outcomes that the table omits. T-11's "`calls[]` empty / `charged` 0 with no call" branches cannot occur, because T-11 runs only with a selected Run | List them for completeness |
| N-4 | NOTE | T-12; G4 | **Scope of the detail.** The receipt-encoding detail applies only when the abandonment cause is a C1:68 receipt-encoding (or `publication_hash_range`) serializer failure. That is implicit; state it. **Placement of the detail.** When the failing counters belong to an unavailable case, the detail appears on the selected cases' notices. Decision 6 is still sound, but the text should say the detail names the invocation's cause. **Base readers.** T-12 puts c N1 notices in one ordinary envelope, whereas `u3_r2_base_readers_accept_the_unavailable_notice` covers one | Clarify the text, and extend that base-reader test to several notices in B1 |
| N-5 | NOTE | Decision 5; T-11 | `ordinary_value` (PP `retained_wire.rs`) refuses `Untranslated` (`ordinary_attempts[].formation.recovery_finding`) for any seed with R-b′'s `recovery_demoted`. T-11 serializes every case's ordinary attempt. So one R-b′-demoted case, whether attempted or not, abandons every other case's successor | Record it as a known B1 limit, or route a wire member for R-b′ |
| N-6 | NOTE | §2 (4b) and its pin | **`case.source_ref`.** (4b) does not state it. TS requires `c.source_ref === a.source_ref` whenever the attempt has a source. **The test-hook D38 shape.** The existing shape (source withdrawn; the D1 serializer emits `source_ref: null`) is refused by (4b) by design; say so. **The pin's derivation.** Rewriting case C must keep the shared group and builds that the other Runs reference, since D1.5 gives one group | Add the three sentences |
| N-7 | NOTE | §3.4 | CORPUS has 17 bases and no milestone base. The milestone successors exist only as `fixtures/results/retained_precision_milestone_successor_*.json`. Three P2/P3/P4 mutations name "the milestone" bases, and the dense W-C2 base depends on B-1 | B1 adds the milestone bases as D-U6-5 copies, or uses the L = 0 bases (dense L = 0 has a parity row) |
| N-8 | NOTE | §5 item 7; decision 15 | **G8 refuses the route.** 0.3.0 `legacy_pressure_v1` carries `pressure_contract {1.0.0, legacy_pressure_v1}`. All three readers' G8 require `pressure_contract` to be null, and `family_clauses` refuses it under D1.3. So B3's zero-pressure route needs a three-reader G8 widening, not only a D1.3 widening. **Successor lists.** Base tables carry `reserved_inactive_successors`: physics-1 lists `source-blocks-1` and `precision-2`, and load-reference-1 lists `load-reference-source-1`. The retained precedent left preview-physics-1's list unchanged | State both in B3's inheritance |
| N-9 | NOTE | F0-6; decision 12; T-4 | **The rerun collision check** (`evidence/collisions.txt`) finds: the full `0.3.0/physics-retained-1` and `exact_straight_retained_w1a_v2` have 0 hits at both revisions; at `f12fed9d69` the bare `physics-retained-1` occurs once, in a T6S comment (`outputPolicy.ts:5`, not a wire value); C3a's names have no hits except `combination_operand` in the two identifiers named. **The fallback's name.** `NotRequired` already exists as `LegacySeed::NotRequired`, and `not_required` is a `legacy_source` disposition | ROOT's reservation recheck uses the full form. Pick a distinct `W1Fallback` name, for example `NoTriggeredCase` |
| N-10 | NOTE | §6; §9 | **Memory.** The dense margin today is 28,389,922 B (1.81 % of TAV_W, QUAL §3). T-8, T-9 and T-11 keep every attempted case's Run and frozen candidate live until staging. So c ≥ 2 at D1's model caps is unlikely to fit without lowering the model caps. That makes decision 17 (owner-held M) a probable path, not a remote one. **Estimate.** B1's estimate omits B-1's RS change and S-1's witness audit | Say so in §9 and carry a contingency |
| N-11 | NOTE | Citations; §6 step 2 | C2:164 is D6b's quality-binding warrant ("No checks_passed status is inferred from W1 recovery"), not a definition of `not_required`. C1:101 and D2 §4.9.2 define it. DN §4.3 and RR:2190 are the warrant for the trigger. In §6, "case sources" are per attempted case, since only A is prepared | Adjust the citations |

## 1. The findings (§0), checked against the code and records

| Finding | Result | Evidence |
|---|---|---|
| **F0-1** | **TRUE** | **D1 attempts W1 on Passed cases.** `retained_w1` (PP `lib.rs`) has no quality check: coexistence, then late gate, then `w1_case_id`, then the notice reservation, then `prepare_case`. PROBE §2 and §4 and the `I68_ORDINARY` lines show that W6 and two-body B are `CHECKS_PASSED` in both modes and reach Native.<br>**All three readers refuse a selected Passed case.** D6b: RS (`solve_quality` ∈ {sensitive, unresolved, failed}), PY `_g5_ordinary` and TS `ordinaryAttempts`; RR:8118.<br>**The c > 1 consequence.** Precommit is the RS reader, all or nothing, so one such selection would abandon every case.<br>**Real, not hypothetical.** One-body A (Passed) reached native Selected before its proof refused.<br>**But see B-1:** these Passed cases are W2-published, which T-4's text does not handle |
| **F0-2** | **TRUE only under the quality keying (B-1)** | **True part:** case B is `CHECKS_PASSED` in both modes. **But** it is W2-published, so T-4's literal text puts it in A. It would then end Native Ceiling, unavailable, and the briefed two-case W-C2 holds. Under the faithful keying, B is `not_required`, which needs RS's alignment (B-1) |
| **F0-3** | **TRUE** | **The parity row's own lane.** The parity row needs `(DenseScrutiny, Some(formed), w2_publication == None)`, a dense reduction, and `legacy_dense_observation` = `solve_dense` (FK `DENSE_SOLVE_ZERO_PIVOT_GUARD = 1.0e-12`, partial pivoting). Its failure is swallowed (`if let Ok(legacy_dense)`), with no row and no diagnostic.<br>**The mode row's lane.** The mode row's fields come from `solve_preview_reduced_system`'s `legacy` lane: `solve_symmetric_system_from_entries`, with `SPARSE_SOLVE_ZERO_PIVOT_GUARD = 1.0e-12`.<br>**So text A can refuse a faithful base** whenever `solve_dense` alone fails. This is reachable in principle and not witnessed, as the design says |
| **F0-4** | **TRUE** | **Scope:** RS checks the mode and parity rows on selected cases only; TS checks the mode row on every case; PY has no mode or parity check and no `requested_mode` check at all.<br>**Codes:** RS uses `PREPARATION_MISMATCH`; TS uses `INVOCATION_MISMATCH`.<br>**Mode code 3:** TS admits it in sparse, while the producer's `dense_fallback_message` is always `None`.<br>**Also:** B-1 adds a fifth divergence (`not_required`) |
| **F0-5** | **TRUE** | **The kernel.** `RecordedInvocation::solve_cases` (FK `origins.rs`) fails only before the call: capacity, count range, reservations. After that, every source gets a `RecordedCase` with a Run.<br>**The producer.** `solve_native` maps `OriginCapacity`, `RecordedInvocation::new` and `solve_cases` errors to `Origin` with `native: None`.<br>**C2 §3** registers a case source "regardless of final selected/unavailable outcome", and C1 §3 defines its digests over K4SRC and K4STF bytes. PY `_g8` recomputes both through `_native_source_encoding` |
| **F0-6** | **TRUE** | **The reservation.** RR:6960–6970 reserves the CHECKS.json names. CHECKS.json (`289bd974…`) lists both names with `matches: []` at `92790ca0cc`.<br>**My rerun:** 0 hits for the full form at `ecb541d63f` and at `f12fed9d69`. There is one bare-substring comment hit at `f12fed9d69` (N-9) |
| **F0-7** | **TRUE** | **One case counted.** The census sets `primitive_loads` only at `index == 0`; `LOAD_CASES = 1`; `family_clauses` refuses `load_cases.len() != 1`; `late_observations` reads one `CaseLoads`.<br>**The margin.** QUAL §3: dense E_mov,max + R = 0.8929 M, 28,389,922 B under 0.9 M |

## 2. Is T-4 (decision 1) faithful to the accepted design?

**Do DN §4.3, C1:101, C2:164 or D2 §4.9.2 already require the per-case trigger?**
- **Yes, DN §4.3 does.** It says "Trigger, per case: … ended in one of: Sensitive; NumericallyUnresolved …; a Range error that W2 scaling could not resolve."
- **RR:2190 restates it as a ruling:** "W1 is selected only for Sensitive and D-5-routed cases".
- **C1:101 and D2 §4.9.2 define `not_required`** as an ordinary pass. D6b (RR:8118, citing C1:101 and C2:164) makes a selected Passed case reader-illegal.
- **So D1's attempt on Passed inputs is the deviation.** PLAN §1.1 recorded it as a fact ("Preparation checks no ordinary quality"), and U8's W-C1 relied on it. Neither CR nor QUAL states a trigger.
- **The qualification.** The faithful trigger is the **published verdict**. A W2-published Passed case resolved its Range error, so DN does not trigger it. T-4's seed-shape text does trigger it. That is B-1.

**Does T-4 change anything published or public?**
- **No public change.** `permitted_dispatch` is reachable only from D1 Direct calls in the registered dev/test build. The work graph records "No product caller exists", and successors are not public before B8.
- **The only change** is that, in that build, Passed one-case inputs publish the exact ordinary bytes without the N1 notice.

**Is anything owner-held touched?**
- **Public meaning:** no.
- **Observation framing:** no.
- **Native-app witnesses:** no. G10's half runs the milestone, which is Sensitive, on the ordinary route, and B8's witness is not yet defined.
- **PHYS-R4 availability:** not changed. T-4 (re-keyed) stops the dev/test W1 attempt and its notice on PHYS-R4-geometry inputs (W6), as RR:2190 already rules, and leaves PHYS-R4's publication unchanged. Because PHYS-R4 availability is on the owner's list, ROOT should name W6 in the owner notice that decision 1 already proposes.

**Is there a faithful alternative?** The re-keyed T-4 is the faithful form. The alternatives are not faithful:
- **(a) Relax D6b:** contradicts DN §4.3, C1:101 and RR:2190.
- **(b) Attempt every case, then discard a Passed selection:** the case has no legal status. It cannot be `not_required` (C3 requires a null `product_attempt_ref`) or `selected` (D6b), and `unavailable` would need an invented cause.
- **(c) Apply T-4 only for c ≥ 2:** keeps W-C1 and W6 as they are, but makes routing depend on c and leaves D1's deviation in place. I do not recommend it.

**Which committed tests and witnesses change** (under the re-keyed T-4):

| Test or witness | Effect |
|---|---|
| `u8_real_input_fallbacks_append_one_notice`, variant `w_c1_two_body_case_b` | **Changes** (expects Native plus one notice) |
| `witness_w6_force_scaled` (QUAL §4 W6) | **Changes** (expects `Fallback("Native")`) |
| `witness_w2_cap_maximal`, `witness_w2b_cap_maximal_solvable` | **Unknown; change if Passed.** Under either keying (S-1) |
| `registered_g_c_declines_only_unattempted_solves` | Still passes; its doc line may become inaccurate (S-1) |
| The `u3_*` and `u3g2_*` facade tests, W1, W2-deep, W3, W4, W7, headroom | Unchanged (milestone, Sensitive; or exact-selected) |
| `retained_product_tests.rs` (`prepare_observed`, bypassing `retained_w1`) | Unchanged, unless B1 puts the trigger inside preparation |
| **CORPUS** | No entry executes W1. Of the 17 bases, every case is `sensitive` with a `report` initial and `w2` `not_triggered`: all are `selected`, except the unavailable second cases of the two `two_case_*_synthetic` failure bases. No base is Passed or W2-published. The only entries that edit a case to `not_required` are the must-pass `not_required_second_case_checks_passed` and mutation 277 (`g7_not_required_quality_enum_invalid`). Both are reader-only, on a `report`/`not_triggered` ordinary attempt (`evidence/corpus_bases.log`). No CORPUS entry changes; B-1 adds one |

## 3. Are T-1 to T-13 complete and consistent?

The structure matches the sources:
- **C1** (one entry per case; `run: null` only without a schedule; C1:68 abandonment);
- **C2 §3–§5** (sources registered at construction, the batch call and groups, ordinary attempts per case in request order);
- **C3** (`product_attempts[]` in start order, at most one per owner, no retries);
- **DN §4.4's coexistence**, which is T-3 (c) over the whole invocation, and W1 never attempted then;
- **D2 §4.9.2** (a successor needs a selected case);
- **SCHEMA's members.**

**The outcome table** is correct for what it lists. N-3 names the omissions.

**T-11's receipt body** lists every member these sources require for B1, and `combinations` stays `[]`. The cumulative-snapshot paragraph is the right reading of C3 §3, apart from S-3.

**T-12's notice rule** is sound. A selected case loses its successor to the receipt; an unavailable case's own cause does not depend on the receipt. N-4 covers the detail's scope and placement.

**G4** holds by T-11: exactly one SELECTED or UNAVAILABLE diagnostic per selected or unavailable case, none for `not_required`, the legacy disclosure omitted for selected cases under T1 (a), and kept for unavailable ones (D39).

**Cases the text leaves undecided or decides inconsistently:**
- **B-1:** W2-published Passed cases.
- **S-2:** a preparation failure with a registered source.
- **S-3:** the record point and order of the snapshots.
- **N-1:** requested cases without seeds.
- **N-2:** `Mechanism`, `Asymmetric` and `InvalidInput`.
- **N-5:** R-b′-demoted cases.
- **N-6:** `case.source_ref` under (4b).

## 4. Text B, D38's relaxation and C3a

**Text B (§3):** implementable in all three readers and **sound in its keying on `w2`**.
- **The keying is exact.** The producer suppresses the observation exactly when `w2_publication` is `Some`, and `ordinary_w2_published` records the seed `Published` at the same site. G5's D6c already validates `w2`.
- **The other properties hold:**
  - P1: the mode row is 1 or 2, always present for a solved case;
  - P2: at most one parity row (the capture refuses a duplicate);
  - P3: none in sparse.
- **P1 over every case is safe.** A failed case blocks the envelope (`has_blocking` → `blocked_envelope`), and T-6 then falls back.
- **The disclosed limit,** an undetected deletion, is stated honestly.
- **The corpus cases named** need the milestone bases added (N-7). The dense W-C2 base depends on B-1.

**R-D38 (§2):** implementable in all three readers.
- **The relaxation sites are correctly named:** RS `if st["native"] != "not_entered"`; PY `_g5_stages`' `else` branch; TS `(stage.native === 'not_entered') === (a.run_ref === null)`.
- **The producer** keeps refusing S-2, and under T-8 cannot emit the shape beside a selected case. So the pin is rightly labelled synthetic.
- **The B1 obligation** to list later Call or Run assumptions is the right control.
- See N-6 and S-2.

**C3a (§4):** no hidden solve, no fake readiness and no availability exception.
- **No hidden solve.** The operand gets C3 preparation work only: no native stage, Run, Call or proof. The combination's own Run solves the combined source (C1 §5), and "Prepared ordinary operands import none" (C2 §4).
- **No fake readiness.** The result is `prepared` or `refused`, with no `Ready`.
- **No availability exception.** A refusal makes the combination `retained_unavailable` with a typed cause, ordinary rows and its own diagnostic.
- **Hashing.** The preparation hash domain has no cycle (no `source_ref`, work or hash fields).
- **Implementable:** in the producer, and in the readers at G1–G3, G5 and G8 with the schema lift already planned for PR-B2.

## 5. The caps (§6)

- **The restatement per invocation is correct.** The scopes are per invocation, per case and per attempted case; l is counted at index 0 today; G-B's `CaseLoads` is one case; G-C is priced per invocation.
- **No M and no cap value is selected.** Step 5 leaves the values to ROOT within D-7, and step 6 marks M above 3.75 GiB as owner-held.
- One wording fix (N-11) and one realism note (N-10).

## 6. The decisions

| # | Verdict | Reason |
|---|---|---|
| 1 | **DISAGREE (as drafted)** | The principle and the decider (ROOT, inform the owner) are right. T-4's text is keyed on the seed shape, which leaves F0-1's defect live and departs from DN §4.3 and RR:2190 (B-1). Adopt it re-keyed to `solve_quality`, with RS's `not_required` alignment |
| 2 | **AGREE, conditional on B-1** | The three-case W-C2, B1's probe and the stop rule follow only under the re-keyed T-4, and RS must first admit W2-published `not_required`. Re-basing W-C1 and W6 on case C keeps the force-scaled path, since C is range-scaled. Extend the audit to W2 and W2b (S-1) |
| 3 | AGREE | One `CaseBatchCall` per C2 §4. Under D1.5 one stiffness group; the kernel's call is all-or-nothing before Runs |
| 4 | AGREE | C3 as written: owner-bound payloads and no retries. Clarify the snapshots (S-3) |
| 5 | AGREE | C1:68 requires abandonment for unencodable runs. Abandoning on any serializer failure is fail-safe. Record N-5's consequence |
| 6 | AGREE | The notices belong to cases whose W1 work ran. The detail goes only on selections lost to the receipt. Clarify per N-4 |
| 7 | AGREE | D1.5 stays per case; one material basis, which matches all three readers' G8 |
| 8 | AGREE | Text A false-rejects a faithful base when `solve_dense` alone fails (F0-3). Text B matches the producer exactly |
| 9 | AGREE | Every case, RS's code and the stated order; TS drops mode code 3; PY gains the requested-mode check. B-1's `not_required` alignment belongs to the same three-reader change |
| 10 | AGREE | P5 needs a three-language grammar for no reliance. Framing stays owner-held and unchanged |
| 11 | AGREE | The synthetic pin is correctly labelled. The S-2 reading follows C2 §3 and C1 §3. Add N-6's sentences |
| 12 | AGREE | P1 keeps C3's `not_required` and D19/D20 tables intact. The names are clean apart from the two identifier substrings (N-9) |
| 13 | AGREE | A typed cause and a null `call_ref` are legal only for this cause. The owner case is unchanged, so it gets no notice |
| 14 | AGREE | The reservation is confirmed at both revisions. physics-1's table is untouched, as in the retained precedent. B3 names and reserves its exact definition (N-8) |
| 15 | AGREE | The route publishes preview-physics-1 today (`mechanics_producer_for_model`), so its successor is `preview-physics-retained-1`. B3 also needs the G8 widening (N-8) |
| 16 | AGREE | The scopes and method are right. The values are ROOT's within 0.9 M at today's M (N-10) |
| 17 | AGREE | M above 3.75 GiB, or a supported-machine M, is owner-held. It is prepared, not decided |
| 18 | AGREE | R-2 is owner-held for B8. Carried, not decided |
| 19 | AGREE | Native-app witnesses are owner-held. B0 changes neither G10's half nor B8's |
| 20 | AGREE | Every brief item is placed in the unit that owns it. I found no misplacement |

**The deciders match the authoritative owner-held list:**
- dense and lane ceilings;
- PHYS-R4;
- observation framing;
- KF3 and KF2;
- M;
- public meaning;
- native-app witnesses;
- R-2.

**No owner-held item is decided.**

## 7. Effects (§9)

- **Orders.** Nothing changes the breadth order (U8 → B0 → B1 and B6 → PR-B1 → B2 and B3 → B4 if ruled → PR-B2 → B7 → B8) or the owner's F2a order.
- **B1's added work.** B-1's repair adds one RS rule, a PY/TS confirmation and a corpus pair inside B1's existing three-reader change and its single re-qualification. S-1 adds a probe-record item.
- **The estimate.** §9's estimate is honest for what it lists but omits these items and N-10's memory contingency. Decision 17 may be reached in B1, which affects timing, not order.

## 8. For ROOT to rule on, or to put to the owner

**For ROOT:**
1. **B-1.**
   - Re-key T-4 to the published verdict.
   - Add RS's `not_required` alignment, with a corpus pin, to B1.
   - Have §1.3, §1.4 and F0-2 restated (by I78 or in ROOT's selection text) before selecting decisions 1, 2, 9 and 11.
2. **S-1 to S-3:** amend T-7 and T-11, and add the W2 and W2b audit to B1's probe.
3. **N-1 to N-11:** wording, or B1 and B3 scope notes. None gates selection.

**To tell the owner** (as decision 1 already proposes):
- T-4 removes the dev/test W1 attempt and N1 notice for Passed inputs, including W6's PHYS-R4 geometry, consistent with RR:2190.
- B1 may reach decision 17 (M) if multi-case does not fit at today's margin (N-10).

**Nothing owner-held needs a decision now.**

## 9. What I read, and limits

**Read in full:**
- the RV105 and B0 briefs;
- DESIGN.md;
- PROBE.md and its `probe_run2.log` `I68_*` lines;
- the F-1 dump.

**Read in the cited sections:**
- PLAN §0–§2.1;
- DN §4.3, §4.4 and §4.7;
- D2 §4.9.2–§4.9.3;
- C1 §2–§4 (lines 60–110);
- C2 §3–§5;
- C3 §1–§3;
- QUAL §3–§4;
- RR: 1236–1261, 2175–2200, 6945–6992, 7040–7060, 8025–8040, 8105–8125, 8430–8440, 8815–8830, 9095–9130, 12336–12380 and 13290–13365;
- WG "T3 current route";
- CHECKS.json.

**Code read** (at `f12fed9d69`, equal to `ecb541d63f` for these files):
- **PP `lib.rs`:** `W1Fallback`, `permitted_dispatch`, `permitted_run`, `ReservedNotice`, `w1_case_id`, `retained_w1`, `append_integrity_report`, `needs_source_recovery`, `legacy_dense_observation`, `solve_load_case_observed` (W2, OQ5 and the parity sites), `solve_preview_reduced_system`, `append_linear_solver_mode_evidence`, `append_sparse_live_path_evidence`, `mechanics_producer_for_model`, the per-case loop and `has_blocking`, and the R-b′ site.
- **PP `retained_product.rs`:** `OrdinarySeed`, `InitialSeed`, `W2Seed`, the `ordinary_*` recorders, `solver_observations`, `bind_observations` and `solve_native`.
- **PP `retained_wire.rs`:** `bind_preparation`, `serialize_unavailable` and `ordinary_value`'s `recovery_demoted`.
- **PP `retained_memory.rs`:** `caps`, `family_clauses`, `cap_rows`, the census at index 0, `late_observations`, `complete_observations` and `ordinary_solve_attempted`.
- **PP tests:** `retained_facade_tests.rs` (U3, U3g2 and U8), `retained_memory_witness_tests.rs` (W1–W7) and `retained_memory_law_tests.rs` (`cap_maximal`, `attempted_examples`, the G-C test).
- **PP `formation_guard::demote`** and `pressure_runtime::is_exact`.
- **FK:** `RecordedInvocation::solve_cases` and the two pivot guards.
- **The readers:** RS `g8`, `rows_for`, the ordinary checks, D6b and the native stage rule; PY `_g5_ordinary`, `_g5_stages` and `_g8`; TS `ordinaryAttempts`, `productAttempts` and G8's case loop.
- **`source_blocks.rs` `integer`.**
- **The schema maxima** (`source_block_recovery` and `physics_source_recovery`).

**Read-only scripts** (VENV Python, committed inputs only; outputs in `evidence/`):
- `probe_quality.py` classifies the probe inputs (`probe_quality.log`).
- `input_shas.py` recomputes the probe input hashes from the committed helpers' construction (`input_shas.log`). It reproduces the hashes for `two_body_a`, `two_body_b` and the milestone.
- `corpus_bases.py` enumerates CORPUS's bases (`corpus_bases.log`).
- The reader excerpts with blob ids are in `reader_rules.txt`.
- The collision rerun is in `collisions.txt`.
- I also checked the semantic tables' `reserved_inactive_successors` (reported above).

**Limits:**
- I ran nothing.
- That two-body B and the one-body pair are W2-published is inferred from records and code, not printed in a log; W6's is recorded.
- The ordinary quality of W2, W2b and the `attempted_examples` is unknown (S-1).
- I did not audit every reader check for Call or Run assumptions (B1's obligation), or every PP test outside the retained files.
- N-10's fit judgement is reasoning from QUAL §3, not a pricing.
