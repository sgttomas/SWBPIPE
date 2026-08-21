# N1 DAG basis amendment — live-current successor correction

- RequestedBy: `HELP_HUMAN`
- AppliedBy: `PROJECT-SETUP-SCA009-INSTRUMENTS`
- Date: 2026-08-21
- Classification: `AMEND` N1 in place; non-consequential current-authority correction; no new node, objective, acceptance criterion, or topology change.
- Frozen-plan wording: rebuild the SCA-009 obligation identified historically as `_DAG/DAG-008`.
- Live basis discovery: at branch basis `b1876a5e0f0083e10c0c18255cd92ed0079b63a2`, `execution/_DAG/_LATEST.md` names `DAG-009` as `approved_active_graph_authority` and explicitly names DAG-008 as superseded.
- Immutability rule: DAG-008 and DAG-009 are immutable accepted snapshots. Neither may be rewritten.
- Amended action: materialize `DAG-010` as the immutable successor to live approved DAG-009, retaining exactly the owner-directed additive SCA-009 graph delta: one DEL-07-09 node carrying SOW-077 and exactly three new upstream execution edges from DEL-07-09 to DEL-16-01, DEL-07-01, and DEL-07-02. Preserve every existing DAG-009 node and edge row byte-for-field.
- Pointer rule: advance `execution/_DAG/_LATEST.md` to DAG-010 only after strict canonical audit, delta validation, schema checks, topology/cycle validation, and artifact assembly pass.
- Authority: HELP_HUMAN disposition `AMEND` communicated in-session after the live-pointer discrepancy was surfaced.

All other N1 bounds remain unchanged: no product code, schedule/estimate recomputation, lifecycle promotion, commit, push, or PR.
