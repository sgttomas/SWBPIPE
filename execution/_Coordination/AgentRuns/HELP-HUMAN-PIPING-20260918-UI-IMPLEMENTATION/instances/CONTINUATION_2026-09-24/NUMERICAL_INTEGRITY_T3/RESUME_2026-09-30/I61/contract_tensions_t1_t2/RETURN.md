# I61: analysis of contract tensions T1 and T2

**Recommendations:**
- **T1:** option (a). The successor suppresses the legacy source-unavailable diagnostic on a retained-*selected* case and carries the legacy attempt as typed receipt data. It implements text in the ROOT-selected designs (D1 §5 ("Interface note for D2") item 11; D2 §4.9.3 G4, "the S13 rule from the start"). It still changes what a user sees in the diagnostics panel for the milestone case, so by ROOT's criterion it goes to the owner for confirmation.
- **T2:** admit model 0.1.0 at G8. It is a reader false-reject with no recorded basis. The producer treats 0.1.0 and 0.2.0 identically on this route, and the 0.1.0 and 0.2.0 receipts differ only in the invocation digest. No owner decision is needed, and the milestone request stays unchanged.

This was read-only analysis: no code, no Cargo and no Git writes. Basis: NUM `19065f4828` and READER `a894d9d0ba` (and `b36739112a` for the experiment). It ran from 2026-10-04T01:10Z to about 01:20Z, with the memory guard (PID 5387) running.

**Abbreviations:**
- P = projects/chirality-piping
- PP = P/core/product_physics/src
- R = the RESUME_2026-09-30 record root
- T3 = NUMERICAL_INTEGRITY_T3
- PY / RS / TS = the readers at READER `a894d9d0ba`

## T1: G4 versus C2's legacy disclosure

### 1. The texts and their order

The first-commit times in NUM give this order. The designs D1 and D2 come before all of these and are what the contracts realize.
- **D1** (T3/DESIGN_NUMERICS/DESIGN.md), the accepted numerical design.
  - **§4.4 coexistence rule (revision 5, R4-2):** "While exact-block selection is retained for a domain, W1 is not attempted in any invocation in which exact-block selects a case … An invocation in which exact-block selects no case may attempt W1 … So one envelope never carries both `SOURCE_BLOCK_RECOVERY_SELECTED` and `RETAINED_PRECISION_SELECTED`, which keeps D2's G4."
  - **§5 item 11:** "Selected-UNAVAILABLE alignment. Under option A, fresh solves no longer emit `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` beside a selected case."
- **D2** (T3/DESIGN_STANDING/DESIGN.md), the standing design.
  - **§4.9.3 G4:** "No `RETAINED_PRECISION_UNAVAILABLE` or `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` names a selected case (the S13 rule from the start)."
  - **§4.4 (S-A):** "An `UNAVAILABLE` on a selected case contradicts the receipt and the `SELECTED` diagnostic … The alternative, relaxing joined S13, would weaken a T1-qualified reader in three languages."
- **C1** (R/I32/f2a_wire_c1/WIRE_CONTRACT.md, 2026-10-02 14:04). Its G4 row (:147) says "no source-unavailable naming a retained-selected case". C1’s G8 row (:154) also says "no 0.4 extension".
- **C2** (R/I32/f2a_wire_c2/CONTRACT_DELTA.md, 14:35, by the same I32 author). C2 is later and more specific, but only about the **ordinary-attempt fields**; it does not amend G4. At :160:
  - `legacy_source={disposition:not_eligible|not_required|declined_without_attempt|unavailable, diagnostic_ref:null|DiagnosticRef, work_ref:null|U}`;
  - "Exact-source Selected cannot appear in a W1 successor due coexistence";
  - "the original diagnostic_ref preserves original source failure disclosure";
  - "Exact recovery error internals remain in the private audit".
- **C3** (R/I52/prepared_public_contract_02/C3_DELTA.md, 10-03 07:07) and its later seams 06, 07 and 08 inherit C1 and C2 unchanged on this point. The G4 seam 08 only reallocates raw-row token checks.

**What C2:160 means for a retained-selected case.** C2 applies `legacy_source` to every ordinary attempt, one per requested case, selected or not. Its coexistence sentence removes `selected` from the dispositions: if exact-block selects anything, there is no successor at all. So for any W1-selected case where the legacy method was eligible and actually ran, `unavailable` is the **only** possible disposition. C2 therefore did contemplate `unavailable` on retained-selected cases. But C2's `diagnostic_ref` is **nullable**, and C2 does not say the diagnostic must remain in a successor envelope. Its "preserves original disclosure" sentence describes what a non-null reference points to.

**The real conflict is narrower than it first looks.** D1 item 11 assumed exact-block was already retired (option A, F2b). During F2a, exact-block still runs first, under coexistence, D-15 and I30's LegacyPrefix. So the ordinary route really does emit the diagnostic before W1 is attempted. The designs did not specify what the successor does with it. I30's routing is the nearest text (R/I30/f2a_routing_02/ROUTING.md:48–49): "For successful W1 replacement, build a separately admitted diagnostic projection with ordinary failure evidence intact; only its selected-method presentation changes." The source-method disclosure is part of the selected-method presentation, so I30 is consistent with option (a).

### 2. Every consumer of SOURCE_BLOCK_RECOVERY_UNAVAILABLE, and the effect of each option

| Consumer | Where | Today, on the milestone case | (a) suppress in a successor | (b) amend G4 | (c) decline by routing |
|---|---|---|---|---|---|
| Producer emitter | PP/lib.rs:3753–3758, the attempt's Err arm. It is the single emitter (D2 §3.4) | Emits it: "did not produce a selected response: RecoveryFailure{source closure … not a signed permutation … charged 46628}" | Ordinary and fallback bytes are unchanged. Only the successor envelope omits it for a selected case | Unchanged | The legacy attempt is not made, so ordinary bytes change (see 3) |
| Retained-precision readers | PY:1631, RS:801, TS:268 (C1 G4) | First failure at G4 | Pass with the G4 rule unchanged. The experiment's probe B passes G4–G7 | All three readers relax G4, against D2's "from the start" | Pass |
| Load-reference (joined) readers | `load_reference.rs:42, 596–618` (`JOIN_SELECTED_UNAVAILABLE_DIAGNOSTIC`); `load_reference_evidence.py:40, 416`; `loadReferenceEvidence.ts:36, 397` | They read only 0.4.0 load-reference envelopes. The milestone (preview family) never reaches them | No change. 0.4.0 load-state never enters W1 in F2a (I30 §3 table, "0.4.0 load-state route → original path"). The F3 successor would then match the joined S13 rule | No change now. At F3, a load-reference successor would contradict the joined S13 rule, or force relaxing a T1-qualified reader | No change now. At F3 it changes T1's joined behaviour |
| Source-blocks and physics-source readers | `source_blocks.rs`/`.py`, `physics_source.*` | They do not read the code (D2 §3.4). Their closed field lists refuse a `retained_precision` member, so they never see a successor | No change | No change | No change |
| Product tests on ordinary and fallback bytes | `s11g_tests.rs:905–918, 2134, 2530`; `load_state_*_tests.rs`; `source_budget_tests.rs:67–72`; `tests/f1b_w2_runtime.rs:1096`; `tests/load_reference_state_runtime_extension.rs:2120`; headless `load_reference_route_tests.rs:830`, `load_reference_cli.rs:524`; committed fixtures `load_reference_fallback_uz-*.raw.json`, `load_reference_*mutations.json` | They pin its presence on ordinary, fallback and joined routes | Untouched: the successor is a separate envelope | Untouched | Affected wherever the declined route runs. The committed fallback bytes must stay identical (I30:98) |
| Desktop UI | `features/diagnostics/DiagnosticsPanel.tsx:25`, plus other panels merging `result.diagnostics` (App.tsx:1543, Export/Design/Headless panels) | The user sees an info row, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE: The bounded retained-source method did not produce a selected response: …" | On a successor result the user sees `RETAINED_PRECISION_SELECTED` and **not** the source-method row. The legacy attempt is disclosed only in the receipt (`legacy_source` + `legacy_source_work`: stage, helper_stage, charged, rejected, limit). The failure text is not shown | The user sees both "selected" and "did not produce a selected response" for the same case | The source-method row disappears from ordinary results too, for W1-eligible cases |
| Report and standing | Reports carry `result.diagnostics` | As in the UI | As in the UI. Standing comes from the receipt (D1 §5 item 3) | Both diagnostics are reported | As in the UI |
| I30 projection | `ROUTING.md:42–49, 86–98` | The LegacyPrefix runs unchanged. The fallback must restore the original prefix byte for byte | Matches "only its selected-method presentation changes" | Keeps the whole prefix in the successor | Conflicts with LegacyPrefix and with "exact fallback bytes remain mandatory" |

### 3. Does option (c) change protected ordinary-route bytes? Yes

- **It breaks the coexistence order.** While exact-block is retained (D-15, R4-2), it must run first: an invocation it selects "publishes exactly as today", and W1 may not replace it (D-15 ordering, gate condition 3). Whether exact-block will select is known only by attempting it; the milestone fails at source closure, a property of the attempt. A routing decline before the attempt would therefore replace exact-block wherever it would have selected. That is the regression D-15 forbids.
- **It changes the fallback bytes.** C1 §2 and I30:98 require the fallback (W1 unavailable) to return "original ordinary base … preserve its original diagnostic prefix", and that "exact fallback bytes remain mandatory". Declining the legacy attempt changes those bytes: the diagnostic and the 46,628 units of legacy work.
- **It loses disclosure.** The actual legacy work and its stage would no longer exist to disclose.

### 4. Recommendation for T1: option (a)

**The producer obligation, applied in the successor transaction only.** When a case is published as retained-selected:
- the successor envelope's diagnostics omit any `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` whose affected_refs name that case;
- `ordinary_attempts[i].legacy_source = {disposition:"unavailable", diagnostic_ref:null, work_ref:k}`, where `legacy_source_work[k] = {case_index, stage:"source closure", helper_stage:"source_closure", charged:46628, rejected:0, limit:4000000, settlement:"booked"}` is copied from the actual WorkReport (C2:160–162);
- `ordinary_attempts[i].diagnostic_refs` does not list the removed diagnostic;
- the publication hash covers the projected successor envelope.

Unselected and fallback publications keep today's bytes. A retained-*unavailable* case in a multi-case successor (wider F2a) keeps the diagnostic, with `diagnostic_ref` naming it, because G4 forbids it only on selected cases.

**The reader rule: no relaxation; everything stays as it is:**
- C1 G4 (PY:1631, RS:801, TS:268);
- C2 O4 (a non-null `legacy_source.diagnostic_ref` resolves);
- D6d (`work_ref` resolves to the same case's `legacy_source_work` entry).

One optional pin: a selected case whose `legacy_source.disposition` is `unavailable` must have a null `diagnostic_ref`. This follows from G4 plus O4 if O4 also checks the code; record it as a shared pin rather than a new gate.

**User-visible semantics.**
- **What changes:** compared with today's ordinary publication of this case, the successor's diagnostics panel drops the source-method info row and shows `RETAINED_PRECISION_SELECTED`. The legacy attempt's work and stage move into the receipt; the error text stays in the private audit (C2).
- **Does it need the owner?** It realizes ROOT-selected design text (D1 item 11, D2 G4 and §4.4), not a new meaning. But by ROOT's stated criterion ("if the chosen reading changes what users see, the owner decides"), it should go to the owner as a confirmation, with (b) as the alternative.

**Why not (b).**
- It reverses D2's explicit "from the start" choice and D2 §4.4's tightening direction.
- It changes all three readers.
- It shows a user "selected" beside "did not produce a selected response".
- It creates an F3 inconsistency with the joined S13 rule.

## T2: the 0.2.0/0.3.0 model-schema admission at G8

**Where the rule came from:**
- **In Git:** the clause first appears, in all three readers at once, in READER commit `ae97b7d5c2` ("WIP (unaccepted): reader drafts frozen at the 2026-10-03 handoff", 2026-10-03T13:31). That is the pre-handoff reader drafts listed in R/HANDOFF_2026-10-03/READER_STATE.json. It survives unchanged to `a894d9d0ba`, at PY:1366, RS:3330 (3278 at `b36739112a`) and TS:1077.
- **In the records:** no brief or return in R/I52, I58, I59, I60, I62, I63, I64, R/BRIEFS or the handoff mentions it (`git grep` for the version pair).
- **In the contracts:** C1 G8 (:154) says only "no 0.4 extension". C3 G8 (:308) says "Existing raw invocation/mode/project/material/order/family/pressure checks".
- **The "existing" preview-family check has no model-version restriction.** `analysis_runs/source_blocks.py:270–303` checks project id, lists, cases and mode only. Only the exact-pressure physics-source reader requires 0.3.0 (`physics_source.py:303`, `physics_source.rs:681`), for its pressure contract.
- **Conclusion:** I found no recorded basis. The clause looks like an over-tight encoding of "no 0.4 extension".

**What the producer accepts on this route:**
- **Direct solves** accept exactly 0.1.0, 0.2.0, 0.3.0 and 0.4.0 (PP/pressure_runtime.rs:112–160).
- **0.1.0 and 0.2.0 share one arm.** In that arm a pressure contract is a problem (:113–118).
- **The legacy source-blocks namespace is "model0.1/0.2 without pressure contract or regions"** (PP/source_recovery.rs:604).
- **0.3.0 requires a pressure contract** (:128–133), which the retained preview route and the reader's G8 (`not pressure_contract`) both exclude.
- **0.4.0 is the load-state route** (case_state/mod.rs:20, 40), which I30 sends to the original path, with no W1.
- **Net effect:** the reader's current set {0.2.0, 0.3.0}, intersected with "no pressure contract", **admits only 0.2.0**. The 0.3.0 member is unreachable.

**Does 0.1.0 differ in anything a receipt binds? No, only the invocation digest.** The experiment's probe B receipts (0.1.0 request) and probe C receipts (the same request at 0.2.0) differ in exactly three places, in both modes:
- `body.invocation.value`;
- `receipt_sha256`;
- the request's own `schema_version`.

The envelope, `publication_sha256`, every source, run, selection and attempt field, and every classification are identical. The PROGRESS and RETURN of R/I61/receipt_experiment_01 hold the receipt hashes.

**Recommendation for T2: admit 0.1.0.**
- **The exact reader rule:** in G8, replace the version clause with `schema_version ∈ {"0.1.0","0.2.0","0.3.0"}`. That keeps 0.4.0 excluded per C1's "no 0.4 extension". All other G8 checks are unchanged: `not pressure_contract`, `not combinations`, `not components`, materials, sections, maps and digest.
- **Equivalently,** state the rule as `schema_version != "0.4.0"` within the producer's accepted set.
- **The producer obligation:** none. The milestone request stays at 0.1.0, so its digest and identity are unchanged.
- **The owner:** not needed. This is a reader false-reject repair consistent with C1 and C3, with no meaning change.
- **The cost:** one shared corpus pin, a 0.1.0 invocation that passes G8, plus a 0.4.0 negative that fails at G8.

## Status for ROOT

- **T1:** recommend (a), with the obligation and rules above. User-visible (one info row moves into the receipt), so take it to the owner for confirmation; (b) is the alternative, and (c) is excluded by coexistence, D-15 and the fallback-byte rule.
- **T2:** recommend admitting 0.1.0 at G8, with the basis above. ROOT can rule; no owner decision.
- With both adopted, the experiment's probe C result applies directly. The milestone receipts in both modes would pass G0–G8 in all three readers, with the reader closure condition's real-receipt half met, after a rerun on the accepted reader head.
