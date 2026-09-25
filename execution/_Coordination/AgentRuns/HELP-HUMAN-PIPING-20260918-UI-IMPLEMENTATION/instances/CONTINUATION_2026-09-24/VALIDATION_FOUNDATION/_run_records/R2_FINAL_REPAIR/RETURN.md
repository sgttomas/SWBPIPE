# Remaining R2 custody consequences — coherent repair

Both independent original false-pass probes were reproduced unchanged in this
separate directory before editing. BEFORE_REPRODUCTION.json retains their
all-required-matched outcomes and exact freeze02 source hashes. The copied
probe source is byte-identical to the independent reviewer script; its natural
HERE output location was moved to avoid overwriting that review's evidence.

The consumer now reads stdout once with its captured byte count and digest under
the parse limit, then passes precisely those immutable bytes to the decoder.
A later file substitution cannot select different compared values, and a grown
archive is rejected by a bounded read. The recorded process stream identity is
kept separate from an archive's observation-time identity.

Reference/criterion copies, retained input/output streams, original bound files
and the retained selection are now checked against the original in-memory
identities at final publication. The final pass covers all processed cases,
including earlier cases that a later process could mutate. Every expected row
stays in the ledger. Interim ledgers cannot publish an all-matched outcome;
final records state the custody observation time and limits. There is no claim
of a filesystem lock, prevention of subsequent tampering, internal solver use,
or build-source attestation. A later consumer must verify hashes again.

REPAIR_CONSEQUENCE_PROBES_02.json retains the fixed results: both original cases
now report not_satisfied. The current focused suite passes31 methods, including
five additional groups covering capture/consumer substitution, bounded growth,
each retained snapshot/stream, cross-case mutation and final-publication timing.
The latter positive control also verifies that the decoder receives bytes with
the actual captured stream digest. Full current command/source/log identities
are in ../gate_checks_05.json and ../gate_checks_05.log.

No supervisor, product, criterion, result schema, transport profile or solver
behavior changed in this repair. It is not a real solver/reference campaign.
GATE_FREEZE_03 binds the four-file gate scope (the supervisor is unchanged) and
complete patch. Independent narrow backcheck is pending through ROOT.
