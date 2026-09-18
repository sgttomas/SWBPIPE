# D-72 — ruling addendum: item 5, the run plan and pass rule

Status: RULED — explicit in-session owner direction, 2026-09-18. With this addendum all six items of [D-72](D-72_redesigned_product_performance_acceptance_criteria.md) are ruled. Its precedent and form are the `D-66_RULING_ADDENDUM_2026-09-08.md` record beside it and the [D-72 ruling record](D-72_RULING_2026-09-18.md), which this addendum completes and does not change.

## Owner act

Stored transcript timestamp 2026-09-18t13:55:47.429z, 70 bytes, sha-256 of the extracted bytes `5cf6071cf15d7ccf43b91197e7a712019a2bb15c414ba6f777d65bfc48193bc8`; an in-session extraction from the host's stored transcript, not original transport bytes. The owner's words on this packet: "D-72, item 5: S-1, S-2 but not S-3." The whole message is reproduced in the [D-71 ruling addendum](D-71_RULING_ADDENDUM_2026-09-18.md).

## Adopted bounded effect

1. **S-1 accepted; it replaces item 5 as first written.** One run per fixture size (1,000 and 10,000 pipes) per canvas configuration on the reference laptop display. A run whose every gated quantity is at or under 80 % of its limit settles that configuration. If any gated quantity lies above 80 % and at or under 100 % of its limit, that configuration is run twice more and all three runs must meet every limit. Any gated quantity over its limit is a failure; a corrected product is tested as a new candidate. A run the instrument marks invalid is repeated and the invalid run is retained. The 80 % line is frozen as of this ruling. No tolerance, oracle or limit is changed to obtain a result. The picking regression tests pass first.
2. **S-2 accepted.** No fresh baseline cohort is collected on the former interface. The recorded demonstration on the repaired product is the before-picture, with its recorded conditions and limits on comparison. By this owner act, D-70 effect 6's phrase "applied identically to baseline and candidate" no longer requires re-measuring the former interface; the rest of effect 6 stands. This addendum is the record that carries that change; the D-70 ruling record is not edited.
3. **S-3 not accepted.** Items 2 and 3 stand as accepted on the first ruling: the external display is measured by the same method once per size and its result published, and it does not gate; the Dark-theme observation and the device-pixel-ratio-1 observation remain, one run per size each, as observations that do not gate.

Run count in the ordinary case: four gated runs on the laptop display and six observation runs (external display, device pixel ratio 1 and Dark, one per size each), ten in all. Independent review REVIEW-02 found the supplement's comparison "six against thirty" was not like for like: the packet as first written came to thirty-six once its declared observations are counted. The owner ruled with the supplement's figure in view; the direction of the comparison is unaffected, and the figures here govern.

The criteria are now complete and frozen: limits and gated quantities (item 1), reference profile (2), geometry and label populations (3), workloads (4), run plan and pass rule (S-1, S-2), and the list of what is left unfrozen until it exists (6). A later change is a new owner act made before the runs it applies to. Nothing is qualified by this ruling: the redesigned product does not exist yet.

## Unchanged

The original D-70 failures, the successor demonstration and their separate attribution; the independent-usability holds PDU-045 and PDU-046; the WCAG 2.2 AA criteria for touched controls. No implementation, release, lifecycle promotion or minimum-hardware statement follows.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
