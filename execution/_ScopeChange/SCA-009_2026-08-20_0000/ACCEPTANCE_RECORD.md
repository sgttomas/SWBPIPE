# Piping SCA-009 Acceptance Record

**Amendment:** `SCA-009`

**State:** `GATE 1 CONFIRMED — GATE 2 PENDING`

## Gate 1 — verbatim owner rulings

The Gate-1 candidate package (package SHA-256
`2458c1dce9b175330c8b28a4a0e4647988213539ee4edb8d27f7ae74e0e9adc0`,
covering `Brief.md`, `Impact_Sketch.md`, `Decision_Log.md`,
`Handoff_State.md`; original durable basis
`7584de0a8d53d69a135c22fe39a78cb4a30b6cb2`) was merged to `main` as PR #592,
merge commit `01a8dd4c0aabd4fe1f71bba7201a4345f9e6cfdc`. The owner ruled
in-session, in chat, on 2026-08-20. Actor: Ryan Tufts. The rulings were
relayed for durable transcription by Agent 0 (HELP_HUMAN); the blocks below
are transcribed verbatim from that relay.

Ruling block 1 (after the Gate-1 package was presented inline):

> "D1 is confirmed.  Let's discuss D3 further, to ensure comprehensive
> coverage for what's already built in the backend, or what ought to be for
> \"core functionality coverage\".  D2, D4, D5, D6 I accept all
> recommendations."

Ruling block 2 (after the tiered backend-coverage inventory was presented):

> "I agree with you on the Tier 3 stable [sic — table], and the sequence of
> development you recommended focusing on normative now and new capability
> separately and afterwards.  Cold spring and cut-short are rarely used and
> I'm comfortable adding that later rather than now."

Merge/proceed direction (same session):

> "But before you proceed, merge PR #591 because PR #591 has also landed and
> is mergable after (in the App session)."

Transcription notes (following the SCA-008 exact-preservation convention):
the double spaces, the backslash-escaped inner quotation marks in block 1,
and the bracketed `[sic — table]` annotation in block 2 are preserved
exactly as they appear in the Agent 0 relay; the `[sic — table]` bracket is
a relay annotation marking the owner's typed word "stable", not owner text.
PR #591 (merge `89758a32634ee6cedbd1dbadf35e3728fb48d2eb`, now `main`) was
merged per the owner's direction before this Gate-2 tranche was authored.

### Gate-1 effect

- **D1 — Gate 1: `CONFIRMED`.** The parsed change request (Gate-1
  `Brief.md` Section 7) is confirmed as the owner's intent. Gate 2 is open.
- **D2, D4, D5, D6 — accepted as recommended** ("I accept all
  recommendations"), binding the recommendations presented to the owner:
  - **D2 = Option A**: ADD `DEL-07-09` "Interactive operation vocabulary
    and tool palette contract" (new PKG-07 deliverable).
  - **D4** = single palette-surface owner (`DEL-07-09`).
  - **D5** = fold the `DEL-07-03-R-005`/`R-006` (load-case, support/
    restraint editor) landing into `DEL-07-09`; leave the `DEL-16-04`
    route/support candidate-generator ownership to a separate act.
  - **D6** = new scope row `SOW-077` (not a SOW-020/SOW-021 remap).
- **D3 — ruled after the tiered-coverage discussion** (ruling block 2): the
  two-class vocabulary is ratified — class **NORMATIVE-NOW** (Tier 1 +
  Tier 2 + the accepted Tier-3 items) and class **ROADMAP** (deferred,
  including node renumbering, snubbers, and cold spring / cut-short —
  owner: rarely used, add later), with the recommended sequencing
  (normative-now wiring of existing backend capability first; net-new
  backend capability separately and afterwards) and the binding constraint
  that the vocabulary binds to the implemented operation taxonomy, not the
  broader schema `OperationKind` enum. The ruled content is carried as
  candidate normative text in `Vocabulary_Annex.md`.

This confirmation opens Gate 2 only. It approves no impact assessment,
amendment, propagation, implementation, dependency, estimate, schedule,
pointer, lifecycle, release, or Git action.

## Gate 2 — impact assessment acceptance

`PENDING` — awaiting the owner's Gate-2 ruling on `Impact_Assessment.md`
(cited by its SHA-256).

## Gate 3 — amendment approval

`PENDING` — blocked by Gate 2.

## Gate 4 — propagation plan approval

`PENDING` — blocked by Gate 3.

## Gate 5 — execute and validate

`PENDING` — blocked by Gate 4.
