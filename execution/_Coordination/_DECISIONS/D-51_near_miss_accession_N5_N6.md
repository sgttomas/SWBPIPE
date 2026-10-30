# D-51 — Near-miss corpus accession: N5 (gate bypass) and N6 (repeal by misclassification)

**Status:** RULED (O-A — 2026-07-18)
**Date staged:** 2026-07-18
**Prepared by:** piping loop session agent (Claude Fable 5) — PROPOSAL only;
adoption is the owner's act (K-AUTH-1; D-GOV-04; DEC-082 limit 3)

## 1. Purpose

Accede two candidates into the local near-miss corpus bound by `DEC-083`
(D-50 §4 S1 local bindings, currently N1–N4, supersede-never-edit), and
record the cross-corpus numbering map offered by chirality-app-dev. The
corpus is part of the delegation instrument's local bindings, so accession
amends the instrument: this packet fails the class-test limits screen by
construction and stops at the owner's ruling gate.

## 2. Basis (evidence, not ruling)

- **Owner direction, 2026-07-18 (in-session, verbatim):** "The
  D-APP-61-authorized NM-5 offer is now durable on `main` through PR #273
  (`5b8e37780576f99e281342f06ad19f670e51a570`). Process it through piping's
  governed accession path using this mapping: app-dev NM-4 ↔ piping N5;
  app-dev NM-5 ↔ piping N6. No direction is being issued on canonical model
  doctrine, D-APP-62, or further instruction cleanup. Pursue the offer as
  far as live authority permits and stop at any required owner gate."
  (CHAT_TRANSCRIPTION — EVIDENCE, NOT RULING.)
- **The offer record:** `projects/chirality-app-dev/execution/_Coordination/AgentRuns/D-APP-61_INSTRUCTION_SEPARATION_GATEWAY_2026-07-18/NM5_PIPING_OFFER_2026-07-18.md`
  (merged in PR #273), authorized by D-APP-61's piping-hold and one-time-port
  terms, twice COMMIT-SAFE-verified on the app-dev side. The offer performs
  no piping write and expressly preserves piping's separate accession
  authority.
- **N5 provenance (piping-native):** the Receipt-53 tranche incident of
  2026-07-17, disclosed the same day on PR #265: a `;` in a command chain
  allowed commit and force-push to proceed past a failed receipt-validator
  gate; corrected pre-merge and disclosed as a candidate near-miss. App-dev
  imported this incident as its NM-4.
- **N6 provenance (cross-project):** the instruction-separation run record
  (`execution/_Coordination/AgentRuns/INSTRUCTION-SEPARATION-20260717/RUN_RECORD.md`,
  section "N6 candidate — repeal by misclassification") and app-dev NM-5
  ("self-authored verification tested intent, not ruled-record protection"),
  the app-dev-side facet of the same PR #268-era failure.

## 3. Proposed accessions (corpus entries as they would stand)

- **N5 — mandatory-gate bypass via command chaining.** A deterministic
  closeout gate (the D-44 receipt validator) failed, but a `;`-chained
  command sequence let the subsequent commit and push execute anyway. The
  gate outcome was correct; the sequencing made it non-binding. Rule taught:
  a validation gate that does not halt subsequent state-changing commands is
  not a gate — sequencing is part of the gate. Run each mandatory gate so
  that its failure stops the chain (separate invocation, or short-circuit
  conjunction only; never `;`). Cross-corpus: app-dev NM-4.
- **N6 — repeal by misclassification.** In the PR #268-era
  instruction-separation work, the owner-revised (2026-07-12) model-agnostic
  subagent capability-assignment and attribution convention was classified
  as "canonical agent doctrine" and deleted, although no complete canonical
  successor existed anywhere — deduplication was in fact repeal of an
  owner-ruled instruction, which fails class test (a): repeal changes
  normative content and is an owner act. The deletion existed only in
  unmerged PR #268 and would have been silent repeal had that PR merged;
  main never lost the convention. Owner review closed PR #268 unmerged, and
  the D-APP-61 ruling preserved the rule and re-homed it verbatim (M3-C:
  app-dev `AGENTS.md`, "Runtime capability convention"). The same
  misclassification pattern in piping PR #269's initial commit was caught in
  owner-mediated cross-project review and resolved by owner direction before
  merge (run-record N6 candidate). Standing correction (imported with
  app-dev NM-5): a verifier's refutation claims must be enumeration-derived
  — every governed artifact touched by a diff generates claims about that
  artifact's applicable protections; the tranche's description of its own
  intent cannot substitute for that enumeration. Cross-corpus: app-dev NM-5.

## 4. Cross-corpus numbering map (per the offer, recorded to prevent misjoins)

- app-dev **NM-4** ↔ piping **N5** (command-chaining mandatory-gate bypass);
- app-dev **NM-5** ↔ piping **N6** (repeal by misclassification /
  self-authored verification scoped to intent).

The numbers are local accession identifiers, not shared identities.

## 5. Supersession discipline

On accession, the current corpus enumeration becomes N1–N6: N1–N4 exactly as
bound in D-50 §4 S1 (by reference, unchanged), plus §3 above. No ruled
history is edited: the D-49 and D-50 packets, the `DEC-082`/`DEC-083` rows,
all receipts, and the run record's candidate sections remain byte-stable;
the run record's "N6 candidate" section becomes superseded-as-candidate by
this packet's accession, never edited (block item 6). Shared-Block v1 is not
touched; its hash and both adopting records are unaffected.

## 6. Class test on this packet (adversarial)

Fast-reject limits screen: accession extends the instrument's local-binding
enumeration — authority-document adoption (DEC-082 limit 3: draft yes, adopt
never). Owner-class immediately; no criterion analysis is reached.
Attempted failure mode (recorded per method binding): "the entries already
exist as durable candidates, so accession is a mere evidence record inside
the delegation." This fails because candidacy and corpus membership differ
precisely in adoption: N1–N4 each entered the corpus through an owner ruling
(D-49, D-50), and corpus entries bind future class-test screening in a way
candidates do not.

## 7. S5 cross-project instrument review

This packet amends the instrument, so `DEC-083` S5 applies: the sibling
project's agent reviews this packet, every relay owner-mediated, before or
at the ruling. The app-dev offer already reviewed the substance and the
numbering map (twice COMMIT-SAFE on their side), so the S5 relay of this
packet text can be lightweight; the owner mediates it as they see fit.

## 8. Options

- **O-A (recommended):** accede N5 and N6 as §3, with the §4 map; codify as
  `DEC-084`; register row → RULED.
- **O-B:** accede either entry alone (each is separable; the §4 map row for
  a deferred entry is recorded as offered-not-acceded).
- **O-C:** defer; both candidates remain durable where they are.

On ruling, execution follows the D-49/D-50 pattern: verbatim ruling
transcription with canonical SHA-256 binding appended in §9; `DEC-084`
codification row; register row updated; receipt (Receipt-55) written at
ruling execution with this packet as the rationale artifact. No lifecycle,
stage, release, prover-activation, or reproduction-acceptance change.
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 9. Ruling

Ruled by the owner (Ryan Tufts, in-session, 2026-07-18), after the DEC-083
S5 owner-mediated sibling review returned support for O-A and its
conditional provenance correction was applied and re-verified
(COMMIT-SAFE, 14 checks). Verbatim, between markers:

<!-- BEGIN RULING D-51 -->
I Rule "O-A"
<!-- END RULING D-51 -->

Canonical SHA-256 (UTF-8 bytes between the marker lines, excluding the
marker lines and the newline delimiters adjacent to them):
`26743be581639738e653f8b0caacb2e28aaaa583b52b57c6e04e6e263317b826` —
recomputed and verified after writing this section. Note: this hash equals
the D-49 §9 ruling hash because the canonical ruling text is identical;
the two rulings are distinct owner acts, distinguished by their packets,
register rows, and dates — the binding is per-packet by construction.

Executed as: N5 and N6 acceded per §3 (as corrected per the S5 review);
corpus enumeration now N1–N6 per §5; cross-corpus map per §4 recorded;
codified as `DEC-084`; register row → RULED; Receipt-55 written with this
packet as the rationale artifact. Merge of the staging PR remains the
owner's per-chain grant.
