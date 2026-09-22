# R3 coverage and QA

This report records what the reconciliation covered, how each figure can be
reproduced, and the limits a reader must keep in mind before relying on any
R3 or R4 output. It is written by Agent 0 and is subject to the R3/R4
independent review.

## 1. Corpus coverage

| Measure | Value | Reproduced by |
|---|---|---|
| Deliverables | 102 of 102 (18 packages; DEL-01-01 ISSUED; DEL-07-09 included) | `DELIVERABLE_INVENTORY.csv`, `R3/SYNTHESIS_STATS.md` |
| Claim rows (every issued key, exactly once) | 9,889 (W1 1,318 · W2 2,394 · W3 6,177) | `R3/CORPUS_CLAIMS.csv` |
| Divergent effective rows | 3,299 (W1 420 · W2 773 · W3 2,106) | `CORPUS_CLAIMS.csv` `Divergent = YES` |
| Rows whose values were changed by adopted resolutions | 63 | `ValuesResolved = YES` |
| Adopted resolution rows | 599 over 597 keys (W1 47 · W2 122 · W3 425 · cross-wave 5) | `WAVES/CROSS_WAVE/ALL_WAVES_RESOLUTIONS_COMBINED.csv` |
| Claim rows carrying non-value corrections (applied by judgment, not substituted) | 580 | `OtherCorrections` non-empty |
| Capabilities (reverse inventory) | 598. None were left unrouted. | `R3/CAPABILITY_COVERAGE.csv` |
| Remaining items | 151 across 65 deliverables; 37 deliverables record NONE; matches the inventory for all 102 | `R3/REMAINING_CENSUS.csv` |

**Structural validity.**
- All 102 forward ledgers are sealed, and each seal matches its file.
- All 102 pass single-mode validation. The W2 and W3 ledgers also pass the
  Part F checks.
- All 102 have reverse files and notes.
- Batch consistency passes with 0 findings in two scopes: per wave with that
  wave's resolutions, and over the whole corpus with the combined
  resolutions.

**Verification.**
- Every package had a fresh, evidence-only verifier.
- Every final verdict is ACCEPT WITH CONTESTED ROWS.
- Three deliverables were rerun once each: DEL-03-07, DEL-13-02 and
  DEL-15-02. All three were accepted on re-verification.
- Batch checks never compared rows across packages or waves. A cross-package
  verifier and a cross-wave verifier settled the 12 flags that surfaced there.

## 2. R3 synthesis coverage

**Deterministic layer.**
- `tools/synthesize_r3.py` builds the corpus tables.
- `tools/index_r3_classes.py` builds the class index, class assignments and
  routing-gap measure.
- Both have a `--check` mode that reproduces every output byte for byte from
  the accepted inputs.

**Class partition (T4A–T7).**
- Every divergent row is in exactly one of 55 classes, 3,299 of 3,299. Agent 0
  checked this for each task independently.
- The rows split by route as follows:

| Route | Rows |
|---|---:|
| R5 record repair | 2,099 |
| Owner decision | 444 |
| Code-fix candidate | 375 |
| Review | 295 |
| No action | 58 |
| Scope-change handoff | 17 |
| Engineering authority | 11 |

**Capability views.**
- T1–T3 cover every capability whose status is not OWNED: 296, each exactly
  once.
- The 302 single-owner capabilities were affirmed by their owner's verified
  reverse pass.
- T11 checked the 137 of those that other deliverables cover or constrain.

**Cross-cutting views.**
- T8 covers the seven contested clusters: 176 rows, all resolving to real
  keys.
- T9 covers lifecycle and Remaining: all 45 mismatch and reassessment rows,
  and the census for all 102 deliverables.
- T10 is the July cross-check.
- T11 runs the method checks.
- T12 covers all 666 rows marked `PRODUCT_CALLER: NONE`.

## 3. Limits (read before relying on any figure)

1. **Sampled verification.**
   - Package verifiers sampled rows. They checked some classes at 100% and
     the rest at stated rates.
   - First-pass firm false-alignment rates on the samples were: W1 4.4%, W2
     1.9%, W3 2.1% (10 of 475).
   - Several verifiers found confirmed or suspected errors outside their
     samples: PKG-06, 11, 12, 13 and 15. PKG-12 sampled at 9.5%.
   - ALIGNED rows that were not sampled have not been independently checked.
     R4 decisions that rely on "aligned" should treat them as the worker's
     judgment.
2. **Reverse-pass routing gap.**
   - R1 routed each capability area to a subset of packages. For example,
     SHELL went only to PKG-00, 07, 10 and 12, and VIEW and WSUI went only to
     PKG-00, 07 and 16.
   - Of the 58 owner links that T1–T3 proposed for ROUTING_GAP capabilities,
     39 point to a deliverable whose package was never routed the capability
     (`R3/ROUTING_GAPS.csv`).
   - R3 caught these, as the method intends ("R3 owns the final unmapped
     set"). But R2 never tested the proposed owners' reverse answers for
     them. H1's scope-change items for these capabilities rest on scope text
     and code, not on the owner's own pass.
3. **Batch blind spot.**
   - Batch mode does not compare minted `.sNN` sub-claims across ledgers,
     because they have no body hash.
   - T11 screened for such splits with scripts. Its screens are candidate
     checks, not proofs.
   - Known splits: SR-1, the architecture-basis rows, the injected still-TBD
     lists, and the rename sub-claims.
4. **Resolutions carry more than values.**
   - Only the five value fields are substituted.
   - The other corrections, such as AuthorityNeeded, FindingGroup and
     RemainingWork, appear verbatim in `OtherCorrections` on 580 rows. They
     are applied by judgment in R3/R4.
   - Some findings exist only there. Examples are the unminted DEC-009
     sub-rows for PKG-17 and the unapplied DEL-15-03 tier corrections.
5. **Independence of the DEL-15-02 tier.**
   - The rerun launch message stated the first verifier's tier before the
     worker sealed. The rerun verifier also read that launch.
   - The INVARIANT tier on the DEL-15-02 CLM-005 Units, Missing-values and
     Provenance rows is therefore not independently derived. Everything
     else in that ledger is.
6. **Code reading, not execution.**
   - No R2 or R3 agent ran builds or tests.
   - Behavioural findings come from reading the frozen code and the recorded
     test and gate evidence (`GATE_EVIDENCE/`, `VERIFICATION_INDEX.csv`).
   - Possible defects are candidates until a code-fix brief confirms them
     under a production brief.
7. **"Code is evidence, not authority."**
   - Aligned means the text and the frozen code agree. It does not mean
     either is correct.
   - "Implemented differently" does not say which side should change. The
     R4 packets put that question to the owner.
8. **Classification drift across workers.** The same fact was sometimes
   classified differently: F7 markers, the DEC-009 cause, CONTEXT row
   treatment and CP-10 routing. T8 proposes corpus-wide readings (packet C7),
   and T12 proposes a single F7 ruling. The known lower bound: 98 more
   ALIGNED rows cite unreached engines without the F7 marker.
9. **Process departures.** All are disclosed in `RUN_STATE.jsonl`:
   - The API overload forced two pauses. Agents were resumed rather than
     relaunched.
   - Agent 0 resumed a PKG-11 worker in place of a recorded rerun, and
     launched two G2 workers directly.
   - The convention change under Direction 8 landed mid-W3.
   - Two workers' scratch files collided. No ledger was affected.
   - R3 task T8 ran read-only `git grep` and `git log` in the freeze against
     its brief. It made no writes, and the freeze was checked clean.
10. **July concordance.** It was hidden from R2 workers and used in R3 only as
    a cross-check (T10). It predates the July R5 repairs, and its claim grain
    is about four times coarser. Its figures are not comparable one-for-one.

## 4. Containment

- Through R3, the run wrote only in the run folder and the orchestration
  record.
- The frozen evidence checkout
  (`00115c71931bcae79909602d653740d3bb72dfa1`) stays clean. Its ignored-aware
  porcelain was checked at each phase close.
- No deliverable, code, lifecycle state, DAG or instruction was changed. The
  stale check against `origin/main` found no product change since the freeze.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
