# Evidence remediation

Parent self-check found absolute ScopePath in initial child REVIEW_RETURN.md line 8. Manager mechanically replaced the checkout prefix with {WORKING_ROOT}, preserving original bytes in PORTABILITY_RAW_EVIDENCE.json. Review content/verdict unchanged; v3 independent PASS remains controlling contract verdict.

Initial runtime ledger faithfully recorded two start/finish attempt pairs using one native session ID; runtime summarizer expects one pair per ID and returned INCOMPLETE. RUNTIME_EVENTS_V2.jsonl is explicitly derived normalization assigning `/attempt-1` and `/attempt-2` logical IDs while preserving event timestamps and actual native session prefix. Both raw ledger and initial summary are retained. RUNTIME_SUMMARY_V2.json is the complete logical-attempt summary and active closeout evidence. No child execution is invented, restarted or suppressed.
