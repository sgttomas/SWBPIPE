# Manager's proposed dispositions of the V1 review

T3 WORKING_ITEMS manager, 2026-09-26. Input: [REVIEW/RETURN.md](../REVIEW/RETURN.md), verdict **BLOCKING** (V1-B1 in D1; D2 has none). These are proposals for ROOT. Items marked **ROOT rules** need a ruling before the designers revise; the rest are technical corrections the designers make directly, subject to V1's backcheck.

No owner decision is needed. No protected predicate or frozen reference changes.

## Rulings requested from ROOT

| # | Finding | Question | Manager's recommendation |
|---|---|---|---|
| R-1 | V1-B1 | Accept the blocking finding and its required change | **Accept.** D1 forms every source-level linear sum exactly from its binary64 inputs (the existing `Expansion` machinery) and rounds once to p, or detects absorption and escalates. Combinations are formed exactly over retained states, or put under the stop rule. D1 adds V1's check L as a negative control and withdraws the §3.1 claim "never accepted a failing candidate". V1 backchecks. |
| R-2 | V1-S11 | Main already has a silent-wrong path: nodal loads are summed in binary64 (`primitive_loads/src/lib.rs:1405` via `PP:1810`) and the intended-action audit sees only the sum (`FK/structural.rs:513-532`). A load absorbed and cancelled on one DOF publishes a zero response as Passed and Current. Is it T3 scope, and how soon? | **Add it to T3 as an M03 intended-action item, and to the graph's T3 row.** D1's revision gives it its own small slice: audit load contributions exactly. A failed audit makes the case not Passed, which today's standing already withholds from Current, and which triggers W1 once W1 exists. `primitive_loads` and `FK` are disjoint from T1; the `PP:1810` wiring waits for T1's merge. Schedule it as the first facade slice after the merge, ahead of the W1 wiring, because it closes a silent-wrong path cheaply. The trigger needs load magnitudes on one DOF that differ by more than about 2^53 and cancel, so it is contrived, but it is silent. |
| R-3 | V1-S1, Q1 | (a) When does `load-reference-source-1` stop being fresh? (b) Does historical physics-source-1 keep its eligibility once no fresh solve emits it? | **(a)** It stays fresh until D1's F3 (the W1b facade slice) covers 0.4.0 load states, then retires under the shared gate (R-4). Joined results stay `needs_recompute` until then, which is safe. Split D2's S-E: build **S-E1** (the resolved-case re-derivation core) with F3, because it is also the standing input for D1's 0.4.0 successor. Build **S-E2** (joined standing wiring, route test, carriers) only if F3 will not land within T3. **(b) Yes.** Invocation-bound physics-source-1 has no known defect under physics-1 semantics. It stays readable and eligible through its existing reader; "no longer emitted fresh" is separate from "admitted as Current". |
| R-4 | V1-S2 | One retirement gate for D1's F2 and D2's S-F | **Accept V1's four-condition gate:** coverage (every committed `fixtures/product_preview/{source_blocks,physics_source}` request solved fresh, both modes, inside the successor method); budgets (D-8 selected from measurement); three-language standing for the successor identities (R-5); value agreement with the exact-block oracle. **D1 owns the gate's definition; D2 cites it.** |
| R-5 | V1-S3 | Who designs the readers for D1's successor identities? | **D2**, in its revision: receipt validation, the standing basis (`checks_passed` against a verified receipt), fresh sets and schema slices, and cross-language parity. It uses D1's receipt (§5) as the interface. |
| R-6 | V1-S5 (with D2 §4.6.2) | Invocation capture hashes the raw request with the checked profile (`PP:1239-1240` → `source_receipt.rs:64-73`), so any request containing a finite \|x\| ≥ 2^53 is refused before solving on every route, and on T1 too. It fails closed. Who owns the capture boundary? | **T3 (M34 range).** It is a solve-entry range defect, not a T6 carrier. D2's revision designs the capture-hash fix (a binary64-safe profile or encoding at capture), in `source_receipt*` and `PP`, after T1's merge, and rewrites its PR-5 expectations to the capture refusal. T6 still adopts binary64 in canonical result carriers and persistence (DD-8). Add it to the graph's T3 row. |
| R-7 | V1-N8, D2 DD-7 | Historical all-selected source-blocks-1 at retirement: (i) keep Current with the notice and the summary rule-binding refusal (T0R's R-2 exception), or (ii) historical-only, like precision-1 | **(i).** Its only defect is the conservative abs-sum summary, which rule binding already refuses, and re-solving produces the successor anyway. (ii) removes Current from correct results with no gain in safety. D2 recommended (ii); the choice is ROOT's. |

## Technical corrections (designers make them; V1 backchecks)

**D1:**
- **V1-S4:** name the receipt's canonical profile, and encode fields that can exceed 2^53 (such as "pivot margin minimum") as bit strings, as `source_receipt.rs:36-38` does. Say what a per-case hashing failure does.
- **V1-S7:** keep `force_scale_exponent` out of the `Debug`-published report so current envelopes stay bit-identical, and make the b-selection rule unambiguous, including the subnormal case.
- **V1-S8:** state the guarantee the stop rule implies (1e-9 only for |q| ≥ about 5.4e-11·S\*). Add a weak-coupling control. Either adopt the per-member option for weakly coupled bodies, or require VP-ROBUST to check that the references' zero scales are at least about 5.4e-11·S\*. The manager passes the latter to R1 and V2.
- **V1-S9:** add the targeted hard-case classes and the seeded `Fraction` differential to the arithmetic test plan.
- **V1-N1:** relabel probe column (C) or replace it with the contribution-exact results.
- **V1-N2:** consider triggering W1 when the negative-energy witness exists only against represented entries.
- **V1-N4:** serialize K5 after K2 (both write `SA`).
- **V1-N5:** name an owning tranche for components, releases and equivalent-static, or record them as open.
- **V1-N6:** state that the `RLIMIT_AS` guard is Linux-only, or use a macOS-effective guard.
- **V1-N3:** no change; P1 checks the Passed boundary.

**D2:**
- **V1-S6:** replace the host-relative one-ulp rule with a host-independent check: re-derive exp at higher precision in the reader and accept the recorded value only if it is one of the two binary64 values bracketing the exact result, with the argument exact. The same outcome then holds in every language and on every host.
- **V1-S10:** serialize S-C after S-A (both write `physics_source.rs` and `physics_source.py`).
- **V1-N7:** reword the scan claim; T1's fallback raws carry the diagnostic.
- Apply R-3 to R-7 as ROOT rules.

## After the revisions

V1 backchecks both revisions (narrow, same reviewer). R1's references and V2 proceed independently. P1 runs after V2 and the host release, and should also exercise V1's check L and the S11 load-absorption case on main.
