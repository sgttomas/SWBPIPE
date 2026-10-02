# W1-T1 raw measurement preservation

ROOT preserves the complete T1 raw record bytes before any later tier can append
to the planned chronological working journal. records.tar.gz contains exactly the
412 files bound by I26/t1_measurements_06/_run_records/RAW_RECORDS_MANIFEST.json.
Each extracted payload was checked against that manifest; no payload was sanitized,
rewritten or regenerated. Raw machine context remains in its evidence role.
The archive container uses normalized metadata; original filesystem-time evidence
is in the sealed author packet and is not inferred from tar timestamps.

This immutable archive is the recoverable T1 snapshot. The original scratch root
remains preserved and is the planned live working stream for separately granted
later tiers; its future journal prefixes must be judged against the matching
immutable tier snapshot. No subsequent tier or acceptance follows from preservation.
The author packet and all its seals remain unchanged.
