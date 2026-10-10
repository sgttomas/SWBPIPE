# Pressure contract and T4 integration

## Current pressure boundary

The owner retired `1.0.0/legacy_pressure_v1` product-wide in #1168. It is not accepted even for zero-pressure documents. The flawed computation, its historical bypass and dependent pressure oracles are removed. Formats 0.1.0/0.2.0 remain the pressure-free namespace. Any pressure primitive in a non-exact document, including zero, requires re-authoring (`PRESSURE_MODEL_REAUTHOR_REQUIRED`); authoring uses `OP-PRESSURE-PRIMITIVE-RETIRED`.

Pressure on the current exact route is limited to straight pipe and linear supports, without components, combinations or `equivalent_static`. T4 extends that route; it does not revive the legacy computation. Existing PRD/INTENT pressure requirements remain desired scope, not claims of present capability. T4 rebuilds the curved-pressure and membrane validation removed with the flawed computation.

## Accepted T4 direction

- **D-1:** the L-line (T4-U2) is the first usable path; the corrected joint (T4-U3) proceeds in parallel.
- **D-2:** only realized bends carrying the user's k carry pressure. Geometry-only chord bends are refused on the exact route with a realization message and remain pressure-free.
- **D-3:** steady-flow/transient loads, Bourdon opening, pressure stiffening of k/SIF and ovalization are excluded and disclosed on bend results and the v3 approximation. T4-U9 requires a later owner choice.
- **H-1:** new authoring uses `3.0.0/exact_pressure_v3` and reserved `pressure-1` semantics only for documents that need a family v3 admits and v2 does not (realized bends under pressure, connectors). Straight-only documents continue to be authored as v2 until T3 joins v3 to retained-source recovery (HELP_HUMAN, 2026-10-10). Correct v2 `2.0.0/exact_straight_pressure_v2` remains accepted and byte-identical except the declared p < 0 refusal. **SP-1:** a straight-only v3 case is bit-equal to its v2 twin in its ordinary-route publication. This applies to v3 cases with zero mill tolerance, zero corrosion allowance and the Euler–Bernoulli selection (D-5/D-6, DEL-05-03 Design). Retained-source publications are outside the twin clause (HELP_HUMAN, 2026-10-10). A v3 case that v2 would route to retained-source recovery carries a named diagnostic saying the join is missing.
- **H-2:** retain the pre-cancelled ledger and add member-owned bend `K_b·u_free(ε_p) − c_b` and joint `p(Ae−Ai)` terms. One implementation owns the assembly. T4-U5 must independently compare representation (A), including JR's tie-force double-count control.

## T3 coexistence and B7

Keep PP's Cargo.lock and priced layouts unchanged (`MemberRecord`, `RecoveryRecord`, Preview inputs); connector fields and bend E/ν belong on unpriced component types. Reconcile real shared-file diffs with the preserved B2 work before T4-U1/U3 land, including `s11f_site_test.rs`. T4-U1 must explicitly account for the changed ordinary envelope on a refused retained call and whether curved cases reach the source attempt. The curved numerical requirements are in DEL-04-01's Design.

**H-4:** the summary-key rename, removal (not renaming) of the historical joint row kind, deferred preview-physics wording and any LIMITATIONS change occur together in **B7**, not PR-B2. Until then the key and row kind stay; the retired joint produces no new rows. LIMITATIONS strings are reviewed semantic inputs, so changing them requires re-registration. T4-U4's preview-physics-1 arc-frame unification waits for B7; its other work does not. No product change is authorized merely by migrating these requirements.

**O-10 (HELP_HUMAN, 2026-10-10):**
- TEXT is priced, not zeroed.
- Rule R1 (`edge_zero for_source → validate_pressure_evidence`) is accepted, and the v3 identity text (+23,920 B) is priced.
- The batch-3 rule whose callee `UserStiffnessElement::orientation` is deleted is dropped.
- T3's manager adjudicates L1–L3 and the connector text-argument classification, with an independent check.
- T3 re-runs SQ2 and the TEXT chain on T4's batch heads. Re-registration is approved if M stays 10.5 GiB and the 5 % text-budget rule holds; anything else returns to HELP_HUMAN.

Sources: [NUM frozen rulings: pressure retirement, U3 and U5](https://github.com/sgttomas/chirality/blob/e85d383b64/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md); [T4 frozen rulings: D-1–D-4, H-1–H-4 and SP-1](https://github.com/sgttomas/chirality/blob/f2211ecef6/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/PRESSURE_STRESS_T4/T4_RULINGS.md).

## T4-U1b and T4-U2a as implemented

- **T4-U1b (certified arc loads, R-1):** S11-G certifies consistent load vectors on realized arcs with the proven midpoint–radius enclosure of T4-I9 revision 01 (directed binary64 radii; `Exact { scale: 1.0, scaled_intended: split(m) }`; no FK type, priced-layout or PINNED_RECORD change). Every failed precondition stays CannotBound. The arc load vector uses exact-rational series below the s < ½ switch. T15c's natural refusal stays at guard level: no model in the k window gives an ordinary Passed report, so the end-to-end CannotBound sentence is not shown (R-2's narrowing, ruled by HELP_HUMAN; DEL-04-01 Design).
- **T4-U2a (seam and v3 identity):** `PP/src/exact_admission.rs` classifies every family per contract; v2 keeps its codes and texts; under `3.0.0/exact_pressure_v3` every family not yet admitted is refused with `EXACT_PRESSURE_FAMILY_NOT_ADMITTED`. v3 publishes under `pressure-1` with its own limitations (D-3 exclusions; collapse not assessed) and admits signed pressure. The three readers dispatch on the contract and refuse cross-reads by name. Pressure-1 export is refused by name until T4-U2. The `pressure-1` table is not a reviewed input.
- **SP-1's twin clause** holds on the ordinary route (v2 typed and its v3 twin are bit-equal in every row). v2 documents that select retained-source recovery publish the ordinary route under v3, because pressure-1 has no retained-source counterpart. HELP_HUMAN scoped the clause to ordinary-route publications (H-1 above).
- **Missing-join diagnostic.** On the captured entry, when a model does not join retained-source recovery (`exact_admission::joins_retained_source`), PP adds `EXACT_PRESSURE_V3_RETAINED_SOURCE_NOT_JOINED` to each case that is retained-source eligible and needs recovery: a Sensitive report, a refused attempt or a load-row finding. These are exactly the cases v2's captured entry attempts. The diagnostic is `info`, names the case and has static text. It appears whether the case publishes or is left `MODEL_INCOMPLETE` (n06, which v2 rescues). The typed entry attempts no retained-source recovery under either contract and does not carry it.
