# Candidate amendment v3

Detection layer: fresh independent pre-effect refutation N3C.  
Disposition: `BLOCK` v2; no product effect released.

N3C established that active rule-pack metadata is private to
`RuleCheckRunPanel` and not available at the authorized App/report boundary.
V3 chooses the narrower lawful success surface instead of expanding ownership:

- `RULE_INPUTS_INCOMPLETE` sessions emit an honest empty rule-pack reference
  list and preserve that status;
- `USER_RULE_CHECKED` or `USER_RULE_FAILED` sessions stop before assembly with
  `REPORT-PACKAGE-RULE-PACK-BINDING-UNAVAILABLE`;
- no RuleCheckRunPanel path, false empty list, synthetic `TBD`, or new follow-on
  queue item is authorized.

V3 also clarifies that the acceptance test uses invented non-engineering data
through the real current-session code path; no real private data is committed.

