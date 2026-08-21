# Piping SCA-009 Acceptance Record

**Amendment:** `SCA-009`

**State:** `GATE 5 CANDIDATE — CLOSURE RULING PENDING`

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

## Gate 2 — verbatim owner rulings

`APPROVED` — ruled in-session, in chat, on 2026-08-20. Actor: Ryan Tufts.
Transcribed verbatim from the Agent 0 relay.

Process direction (after the Gate-2 tranche had been opened as its own PR):

> "we should only be doing on PR for when all of this lands, not at each
> gate."

Effect: PR #593 is the single landing PR for SCA-009; gates accumulate as
commits on branch `claude/piping-sca-009-gate2-20260820` with one merge
after Gate 5. (The Gate-1 package had already merged via PR #592 before
this direction was given.)

Gate-2 ruling (after the envelope M-vs-L rationale and the
implementation-landing discussion):

> "Yes, add the landing column and rule the envelope L.  On that, Gate 2 is
> approved and you may proceed."

Transcription note: the double spacing is preserved exactly as relayed.

### Gate-2 effect

The ruling was made on `Impact_Assessment.md` SHA-256
`bfa25d898e65b82012b2a93988432a121d5f2b842a5469cf7d53593a1a2ba6d0` at
branch commit `f5112824f055b3b5584a852dd68923530dc6620b`; that ruled-upon
commit remains intact and all later work lands as new commits on top of it.
Gate 2 is `APPROVED` **with two owner modifications**:

1. **Context envelope = `L`** for `DEL-07-09`, overriding the assessment's
   `M` proposal. The expected post-change envelope distribution is
   therefore `S=9, M=69, L=24, XL=0` (not the assessment's
   `S=9, M=70, L=23, XL=0`). All other assessment content stands as
   accepted.
2. **Implementation-landing column.** `Vocabulary_Annex.md` gains an
   "Implementation lands in" column on every NORMATIVE-NOW and ROADMAP
   row, carrying the accepted mapping (principle: `DEL-07-09` never
   dispatches implementation; its coverage ledger routes each vocabulary
   item to the owning deliverable(s)).

This approval opens Gate 3 only. It approves no amendment application,
propagation, implementation, dependency, estimate, schedule, pointer,
lifecycle, release, or Git action.

## Gate 3 — verbatim owner ruling

`APPROVED` — ruled in-session, in chat, on 2026-08-20. Actor: Ryan Tufts.
Transcribed verbatim from the Agent 0 relay:

> "I APPROVE Piping SCA-009 Gate 3 using Amendment_Preview SHA-256
> 802c2ce92c5a48651f4d06312d4ba26593f0134e3b7c443988e74b79c0e170d4 at
> branch commit d50e72c4b. Apply exactly the six preimage→postimage pairs
> recorded there, DEC-094, and no other surface. This opens Gate 4 only."

### Whitespace-remediation note

The ruling's cited `Amendment_Preview.md` SHA-256 `802c2ce9…` refers to the
version at branch commit `d50e72c4bd4b8c58fd5ddcc9637d077806dfc55c`, which
remains immutable in history. After the ruling, the governance harness's
candidate-whitespace check (`tools/validation/
validate_candidate_whitespace.py`, run by the `governance-harness.yml`
"Candidate whitespace" step) flagged 18 trailing-whitespace lines in that
file — all blank/context lines of the embedded diffs, an artifact of
inlining `diff -u` output. This commit applies a whitespace-only cleanup
(trailing spaces stripped on those 18 lines; no other character changed).
The operative approved content — the six preimage → postimage pair
SHA-256 values and the `postimages/` files themselves, which are the
byte-exact application targets — is byte-identical before and after the
cleanup. Post-cleanup `Amendment_Preview.md` SHA-256:
`44eaf63ab9de9a4703972acdedc39b65a760dce4f1b2b566134e861512a71eab`.

Gate-4 application executes exactly the six approved pairs by their
recorded postimage hashes; the ruling's authorization is unaffected by the
cleanup.

## Gate 4 — application

`APPLIED` (2026-08-20, executed by the Agent 0-dispatched bounded
generalist under the Gate-3 ruling's own authorization: "Apply exactly the
six preimage→postimage pairs recorded there, DEC-094, and no other
surface."). Each of the six approved `postimages/` files was copied
byte-for-byte onto its live path, and each live file's SHA-256 was proven
equal to the approved postimage hash from the `Amendment_Preview.md` table
(proof table in `RUN_SUMMARY.md`, Gate-4 tranche section). The live
decomposition is now revision 0.12 on this branch. **No pointer moved**:
`_Decomposition/_LATEST.md` and `_ScopeChange/_LATEST.md` are untouched
per the pointer-last rule — pointer advances, snapshot completion, and
validation are Gate 5.

## Gate 5 — validation and closure candidate

`CANDIDATE — CLOSURE RULING PENDING` (staged 2026-08-20 on the owner's
"proceed to Gate 5" direction). This tranche completed the snapshot
artifact set and the deterministic validation pass; the pointer advance
and the final `CLOSED` state are explicitly reserved for a separate
closing commit made only after the owner's Gate-5 closure ruling.

Validation summary (full commands and outputs in `Validation_Record.md`):

- Pre/post coverage computed from the actual files (not transcribed):
  pre at basis `89758a326` = 76 SOW / 18 PKG / 101 DEL / 101 context rows
  / 18 OBJ, `S=9, M=69, L=23, XL=0`; post at HEAD applied state = 77 /
  18 / 102 / 102 / 18, `S=9, M=69, L=24, XL=0`; declared == found on both
  sides; forward/reverse/objective coverage 100% both sides — exactly the
  approved additive delta.
- Six-file application re-proof: all live hashes equal the approved
  postimage hashes.
- ID-uniqueness sweep: `DEL-07-09` / `SOW-077` / `DEC-094` each appear
  exactly where expected and nowhere else; no stray `DEL-07-10`.
- Validators: candidate whitespace `PASS` over the full branch range
  (incl. the new notice); DEC-081 claims-language `VALID` (269 files, 0
  findings); decomposition-register estate validator **exit 1** with 2090
  ERROR findings — identical at basis `89758a326` and at HEAD, all
  pre-existing in the untouched `Dependencies.csv` family (0 such files
  changed in the range), zero attributable to SCA-009; the tool exits
  nonzero on any error regardless of provenance, so its exit code is not
  a pass/fail signal for this candidate (`Validation_Record.md` §3).
- Supersession artifacts minted: `Supersession_Delta.csv` (one binding —
  the DEL-07-03 R-005/R-006 ownership-landing re-pointing) and the
  cumulative `Supersession_Map.csv` (SCA-005's three active rows +
  SCA-009's one). `Amendment_Actions.csv` row 11 corrected
  `SupersessionBindingPresent NO → YES` to match (disclosed;
  `Validation_Record.md` §10).
- Coordination: one notice routed on the Piping loop's own surface
  (`execution/_Coordination/NOTICE_2026-08-20_PIPING_SCA-009_DEL-07-09_VOCABULARY_PALETTE.md`);
  SCA-009 is Piping-internal, so no Root/App/Tier-0 notice is required.
- Pointers untouched: both `_LATEST.md` files still cite the
  0.11/SCA-008 state (expected mid-amendment posture, pointer-last).

The formal pre/post AUDIT_DECOMP comparison, DAG-008 rebuild, targeted
RECONCILIATION refresh, DEL-07-09 dependency extraction, and PREPARATION
scaffold remain explicit downstream obligations (`Propagation_Plan.md`);
none is executed by SCA-009.

**Awaiting owner act:** the Gate-5 closure ruling. On acceptance, the
closing commit advances `_Decomposition/_LATEST.md` then
`_ScopeChange/_LATEST.md` (pointer-last), transcribes the ruling here, and
sets the final `CLOSED_FOR_SCOPE_CHANGE_ONLY` state.
