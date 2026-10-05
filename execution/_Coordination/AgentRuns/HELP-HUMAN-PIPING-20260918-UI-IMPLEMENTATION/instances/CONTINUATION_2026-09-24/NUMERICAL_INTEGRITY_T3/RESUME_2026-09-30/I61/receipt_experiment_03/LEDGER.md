# I61 receipt experiment 03: ledger

**There are no reader mismatches, no receipt-byte differences and no contract tensions on the milestone path.**
- The facade-produced milestone receipts (sparse and dense) pass G0–G8 in all three accepted readers at NUM `dca3b65b3d`.
- They are byte-identical to the experiment-02 receipts.

## Basis changes since experiment 02

| Item | Experiment 02 | Experiment 03 | Effect |
|---|---|---|---|
| Producer | NUM `83732c5677` (core identical to `c817a86cb1`) | NUM `dca3b65b3d` | none on the receipt bytes |
| Readers | READER `abcb16fd27` drafts | the accepted readers at NUM. Against the READER drafts: py +36/−8 lines, rs +80/−15, ts +32/−3; the cases fixture +342. The schema and the definition fixture are identical. | none: per-row classes identical, all gates pass |
| Route | private driver: `PreparedCase::prepare_observed` | the facade `run_linear_static_preview_value_with_retained_direct`, with the test-only permit stub (decision 7, archive only) | none on the bytes; see RETURN, "explained" |
| G-l | values parsed from the diagnostic message | typed `LegacyCapture` from the `Err(failure)` arm; the message is a cross-check only | none on the bytes; the 4 mutants are killed |
| Refusal receipt | emitted as a G0–G2 check | not emitted (T3 / D9b) | — |

## Producer gaps

- **Closed in the archive, carried into U1:**
  - **G-l:** typed capture works, and its values equal the disclosure's.
- **Open, unchanged:**
  - **G-a:** the envelope transformation and its text;
  - **G-b:** `initial`, `w2` and `formation` are still read back from the envelope;
  - **G-c:** A2 is now ruled as decision 2;
  - **G-d:** `support_indices`;
  - **G-e:** `not_covered`;
  - **G-g:** A1 is ruled as decision 1, but no reader checks it;
  - **G-h:** the no-wrap premise, carried to U4;
  - **G-i:** the translations;
  - **G-j:** multi-case;
  - **G-k:** owner binding (U2).
- **Closed by the fan-in:**
  - **G-f:** the fixtures are in-tree.

## New items

- **R-U1-1 (a contract reading for U1; it does not block this experiment).**
  - **The issue:** C2:160 names the dispositions `not_eligible`, `not_required`, `declined_without_attempt` and `unavailable` without mapping them to producer sites.
  - **Where they come from:** the producer distinguishes them at PP/lib.rs :3700–3760:
    - `source_eligible`;
    - `needs_source_recovery`;
    - the formation-guard and range-formation declines, both with `WorkReport{0,0,0}`;
    - the actual attempt.
  - **What the experiment does:** it maps only `unavailable` (attempted) and refuses anything else as `UNMAPPED`. The `gl_unattempted` mutant demonstrates that refusal.
  - **The proposed table** is in RETURN under the U1 brief.
  - **A conflation to correct:** experiment 02's "no disclosure means `not_eligible`" conflated `not_eligible` and `not_required`.
- **U3 design inputs D-a to D-d (not contract readings):**
  - D-a: the successor-bearing output type;
  - D-b: the fallback copy of the ordinary envelope, which counts toward M;
  - D-c: SF-1 and the exact budget sit outside the gated domain;
  - D-d: the permit is consulted after the census and before the single run.

## Known platform failure (not this slice)

`s11g_tests::t13_committed_fallback_uz_is_byte_identical` fails identically at base and with the experiment bytes. It is the known Mac platform failure recorded at HANDOFF_2026-09-30_AUDIT_PAUSE:120. It was not touched.
