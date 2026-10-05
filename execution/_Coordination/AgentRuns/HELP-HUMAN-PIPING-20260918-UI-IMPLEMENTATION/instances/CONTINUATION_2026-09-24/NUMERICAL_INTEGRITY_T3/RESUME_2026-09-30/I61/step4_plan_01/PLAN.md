# I61: plan for handoff step 4, from the accepted readers to the first public milestone

**The shortest sound route has three phases:**
1. Build the real serializer in maintained code, tested through the existing private driver, together with RV77-N4's structural owner binding.
2. Install facade capture behind the existing permit gate.
3. Qualify the memory profile and select M for a bounded first admission domain, then run the milestone, compare it with its independent reference and pass the gates.

**The critical path is memory qualification (U4).** It should start now, in parallel with the serializer. Everything else is days of bounded work.

**The early end-to-end unit is experiment 03.** It is a disposable archive probe in which the actual facade function, behind a test-only permit stub, produces the milestone receipt for the accepted readers, with eligibility off. It tests this phase's consequential premise: that facade capture yields the same certified receipt as the private driver.

This is read-only planning: no code, no Cargo and no Git writes. It ran from 2026-10-04T02:59Z to about 03:10Z, with the memory guard (PID 5387) running. Basis: NUM `b819b902a2` (readers fanned in at `c15e64b756`) and the records cited.

**Abbreviations:**
- P = projects/chirality-piping
- PP = P/core/product_physics/src
- FK = P/core/solver/frame_kernel/src/structural/retained
- R = the RESUME_2026-09-30 record root
- RR = T3/ROOT_RULINGS_V1.md at `b819b902a2`

## 1. What is already settled (do not reopen)

| Settled item | Source |
|---|---|
| The milestone: RF-SKEW-T-CANT-OFF-122-r1e-04 through the captured F2a entry, both modes, M03-INTEGRITY-MP-v2, independent reference agreement, refusal/coexistence controls preserved. The typed entry stays ordinary | HANDOFF_2026-10-03_TO_NEXT_ROOT.md:49–52; R/FIRST_PUBLICATION_PATH.md; RR:6225 |
| W1 work policy 20B per case and 60B per invocation (thresholds, not hard maxima). The 3.75 GiB target is provisional; M and allowance are separate decisions | RR:4938 |
| The admission architecture: separate direct and headless retained entries, census before capture, G-A/G-B/G-C admission, a frozen candidate, then receipt/hash/reader validation, then an infallible transfer | I51/public_producer_admission_05 (accepted, RR:7059); caller barrier and census implemented (RR:7090) |
| The ordinary-memory formulas, at partial scope, with their residual gaps listed. "Do not enable a permit on symbolic or partially priced terms" | RR:7313; handoff:158–160 |
| The wire contracts C1/C2/C3 and completions 06+07+08 | RR:7198; R/I32, R/I52 |
| The summary-coverage representation and the producer seam | RR:7335, RR:7694 |
| The three readers, schema and corpus (07f) are accepted. Eligibility and public activation stay held | RR:8761 |
| T1 = option (a), owner-confirmed; T2 = D31 (G8 admits 0.1.0); T3 = multi-case belongs to wider F2a | RR:8521, RR:8465, RR:8415 |
| The coexistence rule and D-15: exact-block runs first and selects where it can; W1 is attempted only in invocations where exact-block selects nothing | RR:189, RR:229; D1 §4.4 |
| A native-stage failure with no recorded Run keeps its actual cause and never manufactures a wire Run | RR:7262–7264 |
| Real receipts already pass all readers through the private driver (98/98 and 99/99 parity with the certificate) | RR:8557; R/I61/receipt_experiment_02 |
| No new serializer is commissioned "merely to bypass a known boundary"; no new host tooling or guard | handoff:164–166 |

## 2. Units of work, in dependency order

The estimates are agent-hours, author plus repairs. Every unit gets a fresh independent review of its complete frozen diff, with mutants where it has logic, and ROOT verification before fan-in. Under the standing Git authorization ROOT commits; TASKs make no Git writes.

### U1. The real serializer: typed capture, projection and emission

- **Write fence:**
  - PP/retained_receipt.rs, the projection in the C3 seam;
  - a new PP/retained_wire.rs for the JSON projection, if ROOT prefers it separate;
  - PP/retained_product.rs (typed capture fields);
  - PP/lib.rs, only at the ordinary capture sites (G-b, G-l);
  - FK/product_certificate/final_case.rs, only for U2;
  - PP/retained_product_tests.rs.
  
  The in-tree fixtures (definition, semantic table, schema; fanned in at `c15e64b756`) are read-only.
- **The deliverable:** a private, unreachable-in-production function that maps a completed attempt (candidate or refusal) to `retained_precision {body, receipt_sha256}` and the successor envelope. It covers:
  - **G-a,** the envelope transformation: identity, profile, the `recovery_method` token, and the `RETAINED_PRECISION_SELECTED`/`UNAVAILABLE` diagnostic with fixed product text;
  - **G-b,** typed capture of the ordinary `initial`, `w2` and `formation` members at their actual sites;
  - **G-l,** typed capture of the legacy `RecoveryFailure` and its WorkReport at PP/lib.rs:3747–3760. It feeds `legacy_source` and `legacy_source_work` and never parses diagnostic text (C2:160–162);
  - **T1 (a)** emission;
  - **G-c,** D6a attribution, per decision 2 below;
  - **G-d,** a typed `support_indices` map from the existing support build;
  - **G-e,** `not_covered` from the certificate's verdicts;
  - **G-i,** the closed translations for Reason, Stop, Refusal, BridgeError, ProductError, PublicError and CaptureError, with every variant mapped and none left to Debug text;
  - **G-j,** general indexing for one-case invocations; multi-case stays in wider F2a;
  - **A1,** per decision 1;
  - **the hashes,** through the existing `canonical_json_checked_v1_text`;
  - **the definition and table hashes,** bound from the in-tree fixtures.
  
  The experiments' emitter is the reference implementation (R/I61/receipt_experiment_02/_run_records). Its E1 lesson carries over: published-row classes come from the certificate's verdicts.
- **Tests:**
  - the experiment-02 receipts become committed tests through the private driver: byte-stable, both modes;
  - ordinary bytes stay unchanged for every existing fixture, a protected control;
  - mutants for each translation and for T1 (a).
  
  The readers validate in a Python/Rust test lane outside PP, since PP does not depend on result_export today (see decision 5).
- **Cost:** about 10–14 h, in two grants: (a) typed capture (G-b, G-l) plus projection; (b) translations and tests. Review takes about 3 h.
- **Risks:**
  - G-b and G-l touch the ordinary route; the byte-identity controls must hold;
  - the error-translation tables are large;
  - G-h (C1 §2's no-wrap premise) stays open. It is carried to U4, not solved here.

### U2. RV77-N4's structural owner binding (folded into U1's grant, reviewed with it)

- **Write fence:** FK/product_certificate/final_case.rs (`CertifiedProductProof::owner_matches(&RetainedSolve)` over the existing `ProofAnchor::matches_owner`, plus the same on the failure work if a refusal is serialized), and PP/retained_product.rs (make `certificate` private, with accessors).
- **The deliverable:** the serializer refuses with `receipt_failure{association}` unless the proof's anchor matches the selected owner. This replaces custody-only binding; the I61 `I61_FOREIGN_OWNER` assertion flips deliberately.
- **Cost:** about 1–2 h. **Risk:** low.

### U3. Facade capture installation

- **Write fence:**
  - PP/lib.rs: the dispatch at :2201–2229 and `run_linear_static_preview_captured`/`_observed`;
  - PP/retained_product.rs: split `project_candidate` into a FrozenPublicationCandidate and a final consuming commit (I51 design);
  - PP/retained_memory.rs: the permit consumer only, with no profile constructed;
  - P/core/runner/headless, for the headless completion window;
  - PP tests.
- **The deliverable:** with a `CapturePermit`, the direct and headless retained entries install `ProductCapture::prepared_probe()` into the actual single ordinary run. They do not re-run the ordinary solve (I51 public_producer_admission_05 RETURN:7). Then they run preparation, native, proof, serializer and precommit validation, followed by the transfer. Without a permit (always, until U4), behaviour stays exactly today's ordinary route.
- **Also covered:**
  - exact-block bypass, or the coexistence rule;
  - the W1-unavailable fallback with `RETAINED_PRECISION_UNAVAILABLE` and preserved ordinary bytes (C1 §2; I30:98);
  - the receipt-failure route;
  - D9b on a one-case refusal: there is no successor, so the ordinary result is published (T3).
- **Tests:**
  - every existing route stays byte-identical without a permit;
  - fault controls at each new stage;
  - typed entry and shared-Value entries stay ordinary.
- **Cost:** about 10–14 h; review about 3 h.
- **Risk:** the frozen-candidate split and the fallback-byte identity.

### U4. The memory profile and capture permit (the critical path; start in parallel with U1)

- **Write fence:**
  - PP/retained_memory.rs: `RegisteredProfile` and the admission law;
  - FK resource algebra, if the kernel-owned terms move there (I29 plan);
  - records for each derivation.
- **The deliverable:** a registered production profile for a **bounded first admission domain**:
  - one load case, no combinations, preview family, no pressure;
  - counts capped so that the milestone and the post-milestone witnesses (U8) fit.
  
  Every term is priced at the cap. The residual gaps RR:7313 names must be closed for that domain:
  - final build/layout association;
  - nested input capacities and construction;
  - H_formation128;
  - text grammar;
  - deep legacy-exact;
  - stack;
  - whole producer/publication/caller composition.
  
  So must the native context, tail and backing overlap (I53/RV70); the seven `MissingAdmissionTerm`s; and C1 §2's upstream no-wrap premise (G-h; I29 f2a_no_wrap_bound_03 is partial). Then M is selected for that domain and a constructible permit added, with out-of-domain and unknown premises refused to the unchanged ordinary path.
- **Cost:** the largest and least certain: about 20–35 h across 3–5 derivation grants, each with independent derivation review (about 8–12 h in all).
- **Risks:**
  - the native context, tail and backing were left unproved when the vendor investigation hit its limit. The handoff forbids restarting host tooling, so it must close by source/profile argument within the bounded domain. If it cannot, that is a stop for ROOT;
  - stack qualification.
- **This is a ROOT decision** (decision 6). It may touch owner-held machine/ceiling questions (decision 9).

### U5. The independent-reference comparison for the milestone

- **Write fence:** a records/test lane only, reusing I50's named oracle (R/I50/first_publishing_component_02/named_oracle.py, `run_check.py`). It imports no product code and gives exact rational and enclosed bounds.
- **The deliverable:** for both modes, the facade-published successor rows (values, normalized bits, classes and bounds) checked against the oracle's exact references and allowances. Every relative-verified and absolute-verified row must agree within its published class. This is the "independent reference agreement" of the milestone.
- **Cost:** about 2–4 h; review about 1–2 h.
- **Risk:** I50's oracle was written against the ordinary and earlier captured rows. The prepared route's maxima, overlay and support rows must be mapped, not reinterpreted.

### U6. Carriers and standing (FIRST_PUBLICATION_PATH §3: producer, readers and carriers form one public package; S-G1 is atomic with F2a)

- **Write fence:** the result-envelope binding and `derive_document` receipt-copy list (D2 §4.9.7); TypeScript result admission and standing; AnalysisRun and derivative carriers (I30 SOURCE_MAP).
- **The deliverable:** the successor identity is accepted by the carriers, and the receipt travels with the document. Standing comes only from the verified receipt (D1 §5 item 3), and stays `needs_recompute` until U7.
- **Cost:** about 6–10 h; review about 2–3 h.

### U7. The reader eligibility switch-on

- **Write fence:** the three readers' completeness flags (PY `_IMPLEMENTATION_COMPLETE`, RS `IMPLEMENTATION_COMPLETE`, TS `SUMMARY_COVERAGE_COMPLETE`), plus the D36-tracked repairs that are due.
- **Conditions, all required:**
  - U1–U6 accepted;
  - the milestone published through the facade in both modes with reference agreement (U5);
  - RV78's scoped review of the semantic-table fixture and the YAML successor branch closed (RR:8761);
  - the capture (a) Run representation ruled (decision 4);
  - a fresh review of the switch;
  - RV79-N1's independent D37 table in place.
- **Cost:** about 2–4 h; review about 2 h.

### U8. Post-milestone: the deferred producer-solved witnesses (not milestone-gating)

The milestone is p128, single-body, L ≠ 0 and selected. It needs **none** of the deferred witnesses. The native Ceiling row and the L = 0 base (RR:7617, "Snapshot 05b … deferred") need real receipts for other cases through U1/U3. They are wider-F2a and reader-corpus completeness, after the milestone, at about 4–8 h.

### U9. Qualification and the product PR (the standing gates)

- **What runs:** a fresh complete-diff review; hosted and full-SHA CI; the exact-final-head Mac DEC-025; GEN-8; T9; both-entry gates; native Current; pressure refusal with no-pressure success; exact-block coexistence controls; resource evidence. The PR is cut compactly from main under the packaging rule (handoff:176–179).
- **Cost:** about 6–10 h wall, much of it machine time.

**The sequence:**
- U1+U2 run in parallel with U4. Early E2E (§4) runs right after U1(a).
- Then U3 → U5 and U6 in parallel → U7 → U9.
- U8 follows the milestone.

**The total to the milestone:** about 57–91 agent-hours plus about 20–30 h of reviews. The elapsed time is dominated by U4. If U4 closes, the remainder is about 4–6 working sessions.

## 3. Decisions needed, with proposed answers

1. **A1, retained_state_sha256 (ROOT).** Proposed: the raw SHA256 of `RetainedEvidence.retained_state_encoding` (the K4RST bytes), stated in C1 §3's kernel-bytes sentence ("K4LED\x01 ledger bytes and K4RST\x01 state bytes"). No reader checks it; adopt it as written contract.
2. **A2 / D6a, the exact ordinary list (ROOT).** Proposed: `ordinary_attempts[i].diagnostic_refs` lists, once each and in envelope order, exactly the ordinary-route diagnostics whose `affected_refs` contain the case id. It excludes `RETAINED_PRECISION_*` and the T1-omitted legacy disclosure. Invocation-level diagnostics with no affected case (for example `RULE_CHECK_INPUTS_MISSING`) are attributed to no case. Basis: D6a ("the case's actual ordinary diagnostics, each once"); `affected_refs` is the producer's own case attribution. The experiments used this rule, and all readers accepted it.
3. **A4, the fixtures (closed by the fan-in).** NUM now carries the definition, semantic table and schema (`c15e64b756`). The serializer binds them from the tree. The table's scoped review (RV78) still gates U7.
4. **The capture (a) Run representation (ROOT, with the serializer).** Proposed: a `solve_native` failure before any Run is recorded emits `run: null`, `run_ref: null` and `error: {kind: capture, cause}`, with the actual origin or native cause. Basis: RR:7262–7264 ("do not manufacture a wire Run") and C3:167 ("run_ref null iff no native call"). The Rust and Python readers relax "a Run whenever the native stage was entered" to "a Run iff a native call happened". This affects unavailable paths only (D36, tracked).
5. **Precommit reader validation in the facade (ROOT).** The I51 design validates before the transfer. Options:
   - (i) PP takes a path dependency on `result_export` and calls `retained_precision::validate` precommit, refusing with `receipt_failure` on any failure. Its eligibility flag stays false. There is no cycle; result_export depends only on canonical_json and units.
   - (ii) validate only in the test and headless lanes.
   
   Proposed: (i). It is the I51 design and costs one lockfile entry.
6. **The first admission domain and M (ROOT; the W1 ruling keeps M distinct).** Proposed: one case, no combinations, no pressure, preview family, with count caps sized to the milestone plus the U8 witnesses. M is selected after U4's independent qualification, within the provisional 3.75 GiB target. Everything outside the domain takes the unchanged ordinary path.
7. **The early E2E test permit (ROOT).** Proposed: allow a test-only permit stub **only in a disposable archive copy**, as in experiments 01–02. This leaves retained_memory.rs's "no values of this type, including test values" rule unchanged in maintained code. The maintained U3 tests use no permit and prove the no-permit path stays ordinary.
8. **The milestone's independent reference (ROOT).** Proposed: I50's named oracle (exact rational, imports no product code) is the reference for U5.
9. **Owner-reserved items (handoff:194–196; ROOT_CURRENT):** dense and lane ceilings, PHYS-R4 refusal/availability, observation framing, KF3 lambda split, KF2 dense screen. None blocks the milestone case, which is tiny, single-case, with no pressure. But "supported-machine interpretation" of M (RR:4938) is owner-held. If U4's M is presented as a machine statement, ROOT takes it to the owner with the number.

## 4. The early end-to-end unit for this phase (workflow §1): experiment 03

- **The premise it tests:** the actual facade function, with capture installed in the single ordinary run, produces the same certified receipt the private driver produced. That means the same overlay rows, coverage, proof and hashes, apart from what genuinely differs. In particular, `prepare_observed` re-ran the ordinary solve, while the facade will not.
- **The slice:** in a disposable `git archive` of NUM `b819b902a2` (WT/scratch only):
  - wire the dispatch at PP/lib.rs:2201–2229 to install `ProductCapture::prepared_probe()` when a **test-only** permit stub is present;
  - call the experiment-02 emitter, with typed G-l capture in place of message parsing;
  - return the successor envelope.
  
  Call `run_linear_static_preview_value_with_retained_direct` on the unchanged 0.1.0 milestone request in both modes. Validate with the three **accepted** readers at NUM (eligibility off; expected standing `needs_recompute`). Byte-diff against the experiment-02 receipts and explain every difference.
- **Write fence:** WT/scratch/i61_receipt_experiment_03/, WT/targets/i61-receipt/ and NUM/R/I61/receipt_experiment_03/.
- **Cost:** about 3–5 h; no review beyond ROOT verification, because it is disposable.
- **Decisions needed:** 7 (and 2 and 4 as provisional readings).
- **The result** feeds U3's design directly and de-risks it before U4 completes.

## 5. Risks across the plan

- **U4 may not close by source argument** within the bounded domain (native context, tail and backing; stack). Then the milestone waits on ROOT's or the owner's choice. No permit is enabled on partially priced terms.
- **Ordinary-byte identity under G-b and G-l capture** (U1) and under facade installation (U3). Mitigation: protected byte controls on every existing fixture.
- **U5 mapping risk:** prepared-route rows versus I50's ordinary oracle. Map, do not reinterpret.
- **Review load:** the large U1 and U3 diffs. Split them into the grants named above.
