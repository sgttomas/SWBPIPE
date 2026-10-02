# RV29 F17 diagnostic result review 36

**The separate release observation establishes named certificate prevention after R7, with zero CLASS credit. P30's actual debug return passes.** P29 remains unqualified. An owner decision is still required to adopt a different F17 validation route; this review grants no amendment, runtime, V-K/A1 acceptance or merge.

Independent TASK review began 2026-10-01 20:56:04 UTC, ten-minute boundary 21:06:04. The diagnostic is bound to committed A1 `0158df21a13de7df138f0f7274339ab0a71dac81`; numerical source remains 40129 and observation basis 942572. I used the exact source/control-flow and protected-quantity basis sealed in review35, then independently compared the actual records. No runtime/probe, source/test/oracle/criterion changes, Git/index writes or delegation occurred.

Committed evidence and controls

A fresh candidate archive contains all 23 child and five manager payloads, verified against child seal `d5b60c2048f745c4cb2614a625d30a020c7acc7a1da0675f50f6eb9146b55a00` and manager seal `7c3c78c7981613d3e51969c6862804627b17edc4a37f347f2eb8c54d740242de`. The archive and checks are preserved under `_run_records/`.

I independently parsed each original stdout stream with Python's standard JSON decoder and exact integers, obtaining all 36 records from each of D01 NONE, D02 F17 and D03 NONE. Both complete NONE record arrays equal every field of the frozen RF-SKEW observations and each other. Their stdout bytes are also identical. All case/source identities, exact u64::MAX limits, report/control pins and record counts agree. Both controls restore 36 selections at128, 1,080 rows: 982 passes, 96 structural zeros, two not-covered, zero failures; controls105/33 and empty unexpected lists. All 108 case invocation totals close against the existing own/exact-sum/shared/verification-shared components. This arithmetic is a closure check, not adoption of newly observed work as an expected oracle.

Actual fault sequences

Every one of the 36 D02 records has this complete sequence:

| Precision | Role | Actual outcome |
|---:|---|---|
| 128 | Candidate | Rejected PublicationEnclosure, PublicRelative |
| 256 | VerificationThenCandidate | Rejected PublicationEnclosure, PublicRelative |
| 512 | VerificationThenCandidate | Rejected PublicationEnclosure, PublicRelative |
| 1024 | Verification | Solved |

Thus there are108 named rejections and36 terminal Ceilings, no selection and no CLASS line. Every rejecting row has body0. The seven actual first-rejection groups are displacement node0/Rz9cases; member1/I/Uy6; member1/I/Ux6; displacement node1/Ux3; displacement node0/Uz6; member1/I/Rx3; displacement node1/Rx3. Within each case, quantity/body/kind/predicate stay identical across its three candidate rejections. Full exact sequences, identities and all field values remain available; no case or conflicting reason was omitted. There is no R7 rejection, arithmetic/budget/malformed-certificate/verification-failure prefix in these records. Verification at1024 is recorded as Solved, not incorrectly relabeled Verified.

The source reviewed in35 calls `certify_publication` only after R7 `Ok(true)` (`adaptive.rs:4068–4081`), and records a named `PublicationEnclosure` only from the new exact predicate rejection (`4112–4134`). PublicRelative occurs only in the RelativeVerified branch (`2997–3061`), while F17 forces that class for eligible finite non-input-derived rows (`451–458`, `3328–3338`). These facts, unchanged source, fresh fault selection and exact restoring controls establish the observed prevention mechanism. They do not turn the result into the original CLASS witness.

Protected row distinction

For RF-SKEW-T-CANT-AX-345-r1e-12 and RF-SKEW-T-CANT-AX-122-r1e-12, the original protected comparison is `tw.M1`, backed by member1/endJ/Rx, body0, Moment. Both cases actually reject member1/endI/Ux, body0, Force at all three candidate precisions. Quantity and kind differ; only body matches. An earlier force row prevents whole-case publication. There is no observed class, exact H or executed predicate for either protected twist row. Existing records do not expose those private draft fields. This limitation is material and preserved; no new private trace is needed to describe what the observation does establish.

Provenance and timing

All six original streams match recorded hashes and full portable copies. All189 physical archive files match the committed preparation manifest bytes/modes, and candidate comparison shows the complete three source scopes unchanged from942572. Every cited source35 blob remains identical. The actual normal-release binary, original example fingerprint and three dependency fingerprints match their recorded hashes and exact integer JSON; profiles/features retain opt3, non-test, seeded-faults, FK mutation-controls and empty rustflags. No new build is claimed.

The exact36-selector RF-SKEW-only argv, cwd, fresh NONE/F17/NONE environment, cleared flags/wrappers, installed1.97.1, offline/incremental-zero/two-thread settings, guard source hash/PID5387 snapshots and ROOT/manager release were independently checked. ROOT grant bytes match NUM79428ed6. Release20:46:58 precedes launches20:48:35.285,20:49:34.693,20:50:41.210. Raw real times are0.07,0.23,0.07seconds. Child completion check20:52:19 and manager final check20:53:49 precede the original combined20:56:11 end. The prior disclosed three-second preparation sealing overrun remains historical; no reset or extension is inferred. Recorded process/guard snapshots are not continuous monitoring.

P30 limited independent backcheck

After its final seals arrived, I verified all seven child and five manager payloads: `0fc0ac466e124b1ca913598d754ede055575142aeb67ebf72396a5fac221b2dc` and `c0b67ccaa09b8144a166d497dde29245be05ca870244359c62bb3c4368e0bb9c`. These are ROOT-supplied sealed local packets, separately copied and bound; they are not represented as part of the earlier diagnostic commit archive.

P30 is exactly the original debug `rf_skew` NONE filter, one passing test, exit0,1.31seconds real. Original streams/hashes/copies, lane binary/original fingerprint/dependencies, argv/cwd/environment, guard, NUM457cc6 grant, manager release and timestamps match. Launch20:57:15.141 and observed pass20:58:23 occur within the separately granted20:55:31–21:00:31 block. The unchanged lane test reaches all class/not-covered, tally, outcome, comparison-control, exception and byte-exact stored-record assertions. It restores36selected,1,080rows/982passes/96zeros/2notcovered and105/33controls. P30 is now closed as an actual return; it supplies no retrospective cause or CLASS credit for P29. P31–P53 remain uncredited/unrun in this review.

Owner proposal assessment

I read NUM `16a65a7ef3153276367ecd3d86d02dc12061cc39`, `OWNER_DECISION_F17_2026-10-01.md` in full (hash recorded in raw evidence). Its proposed F17-only alternative is supported as an explicit owner decision grounded in the above observation. Its line “whenever a mutated case publishes” needs the announced narrow clarification: actual CLASS evidence concerns original registered not-covered comparison rows, outside the three prescribed InputDerived exceptions, when publication reaches those comparisons. It must not impose a new requirement that every published case/row independently fails. The concrete wording in OWNER_DECISION_TEXT.md supplies that clarification.

A prospective extension to the remaining originally registered F17 family, RF-CANCEL, can be stated now. This is deliberately broader than review35's SKEW-only recommendation and is only a proposed owner-selected validation rule. The warrant is the same unchanged F17 classification site, same R7-before-certificate sequence and same protected class-correspondence harness. It is not a prediction of RF-CANCEL's outcome and gives no unrun credit. Each actual reachable CLASS witness must still match original protected comparisons; each withheld case claimed under the alternative needs its own controlled, source-linked full sequence and named relative predicate, normal restoration, complete disclosed outcomes and independent review.

Eligible named relative predicates are PublicRelative, SharperExact and SharperBinary64, each tied to the reviewed relative-class branch. This run observed only PublicRelative. Neither AbsoluteBound, generic Ceiling/FLOOR/record/work drift, malformed certificate, terminal/budget/arithmetic/verification failure nor a different fault qualifies automatically. Mixed or pre-existing rejection sequences must be explained against controls; an unexplained cause or failed control remains a hold. Any source/corpus change reopens affected source and evidence warrants. The rule is confined to the original A1 F17 families and frozen source basis, not future faults or product qualification.

No protected unmutated51-row list, three prescribed exceptions, reference, tolerance, class/record check or solver contract changes. No certificate bypass, fabricated row/certificate, retirement of F17 or completion of remaining gates follows. The owner's alternative is to retain the CLASS-only criterion and hold affected acceptance. Independent work may continue within separate authority; this review does not choose for the owner.
