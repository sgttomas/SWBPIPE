# R0 decision package — conventions, grain and scale-out

Prepared by Agent 0 for the owner's R0 ruling (workflow invariant
"Human-calibrated conventions"). Evidence:
- eight calibration ledgers under `R0_CALIBRATION/<DEL>/`;
- the pilot inventory `PILOT_CAPABILITIES.csv`;
- the fresh evidence-only review `R0_REVIEW.md` (verdict **READY WITH NAMED
  AMENDMENTS**, SHA-256
  `a00daaaee450a4985757bde9f385cbc1e89c7bec10c46d2fa0a3afbecda0b5b1`).

Recommendations here are Agent 0's; nothing binds until the owner rules.

## What calibration showed

- **Evidence quality is sound.** All eight ledgers pass structural
  validation. All 576 cited test cases exist in the frozen tree, and every
  substantive finding the reviewer spot-checked reproduced.
- **The rules are not yet fit to scale.**
  - A3 blocked real evidence. The worker who applied it literally could not
    credit an existing parity report, while two others read it loosely.
  - The same situation got different dispositions, causes and tiers from
    different workers.
  - At 102 deliverables, those inconsistencies would turn R4 counts into
    noise.
- **Alignment is overstated in places.** In the sampled `ALIGNED` normative
  rows, 12% are firmly wrong and 27% are wrong or questionable. The main cause
  is "met by construction": the claim holds only because the governed
  behaviour does not exist.
- **The grain carries dead weight.** About 28% of rows carried no audit
  value. 605 units repeat text identical to another deliverable's; the block
  in the rev-0.7 decomposition reference alone appears in 99.
- **The pilot reverse inventory was too narrow to test ownership.** It had no
  capability rows for five of the eight pilots' own implementation.
- **Cause clustering works.** Four causes carry most non-aligned rows, so R4
  can be ruled by class.

## Decisions requested

1. **Adopt the amendment set (Agent 0 recommends yes).** The set comprises:
   - the convention dispositions in `R0_REVIEW.md` §4 (keep, amend or drop,
     with exact text), including:
     - A3 split into authority versus evidence, with a mandatory `git grep`
       discovery step;
     - C6 rules (a)–(f) on vacuous alignment, subject-versus-declaration,
       stale classes, ISSUED, scheduled catch-up, and silence;
     - the `GATE:` evidence token;
     - B2's `COVERS` / `CONSTRAINS` / `UNKEYED` answers;
     - the C5 `RECORD` layer;
   - the four new cause tags and three narrowed ones (§4, C7);
   - extractor v2 with re-issued claim keys (§5): deterministic table-row
     keys, section parenting, and mechanical typing of structural units;
   - a **canonical situation table** authored by Agent 0 under these rules.
     It gives one required disposition, cause, tier and layer per recurring
     situation and per shared-text hash. Workers may depart from it only with
     a written justification, and verifiers check conformance mechanically.
     The table is independently reviewed and shown to the owner at the
     first-wave checkpoint;
   - an R1 reverse inventory rebuilt over the whole product tree, including
     documents and governance, stratified so every deliverable's declared
     paths contribute (§6);
   - the scale-out gate (§7):
     - a first wave of 12 deliverables (two packages, one of them PKG-07),
       verified at double sampling;
     - scale-out only if firm false alignment is at most 5% and no
       shared-situation conflicts remain;
     - otherwise, back to the owner.
2. **Can a verbatim owner record support `ACCEPTED_DIVERGENCE`?** The
   question is whether a hash-bound owner decision in an AgentRuns
   owner-direction or adoption record, naming the exact divergence, counts.
   Example: DEL-11-01's R18 guide-edit exception. **Agent 0 recommends yes.**
   Such rows are flagged and routed to *confirmation*, not decision, at R4.
   This matches the owner's position that merged work carries intent, while
   keeping the owner's final word.
3. **Does DEC-101's "other governance documents take the name at their next
   amendment" reach deliverable Scope of Work files?** One answer decides
   about 100 deliverables. **Agent 0 recommends yes.** Rename residue then
   becomes an accepted divergence until each file's next amendment, under
   C6(e), with the trigger recorded, instead of about 100 separate findings.
4. **ISSUED text overtaken by later rulings (DEL-01-01).** The maintainer
   roster, quorum, release authority and signing were issued as TBD and
   later ruled by DEC-027 and DEC-057/DEC-089. **Agent 0 recommends
   treating this as one `LIFECYCLE_REASSESSMENT_REQUIRED` finding group**,
   which goes to R4 as a single item on the ISSUED change path. The
   alternative is eight separate scope-change rows.

## What does not change

- The calibration ledgers stay as calibration evidence; nothing is patched
  in place.
- All eight pilots are re-encoded in the ordinary waves, carrying the named
  repairs listed in `R0_REVIEW.md` §4.
- Substantive findings stay open for R3/R4 and are not decided by this ruling.
  Among them:
  - a removed verification test for DEL-04-04 REQ-08;
  - the ISSUED governance drift and the licence-wording conflict;
  - no launch path in the CAEPIPE run harness;
  - the unowned `nonlinear_integration` crate;
  - tree and inspector surfaces that have no key;
  - the silent owner of the palette component;
  - the missing `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json`.

## After the ruling

1. Agent 0 records the ruling and amends the conventions to the ruled text.
2. Agent 0 builds extractor v2, the canonical situation table, the evidence
   map and the validator updates.
3. The R0 PR, including the D-73 gate-evidence addendum, gets independent
   review and is merged.
4. R1 builds the whole inventory.
5. The first wave of R2 runs under WORKING_ITEMS managers.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
