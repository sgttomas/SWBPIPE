# Exact arithmetic performed

Read the original immutable rf_large.jsonl as metadata using Python json;
no production parser, model, graph, encoder or solver was executed. No author
or reviewer arithmetic program was imported or executed.

For each original case: I = UTF-8 bytes of id; K = maximum row-key bytes;
E = maximum row-expected bytes; B = original table shape.B;
geom = 2+23*B; dout = 100+geom; why = max(9,40,75,K+18).
Use the unchanged source-reviewed diagnostic envelope:

    Dfail = max(I+K+E+6+why, I+K+94, I+93, I+56, I+36,
                I+39+dout, I+75, I+20, I+16+dout)
    F(Dfail) = max(8,2*Dfail)
    G(24,4) = 96
    PriorFailures = 96+4*F(Dfail)

Read the two old values and the other two outcome addends from the authenticated
candidate table. Apply the two repairs independently for requested and moving,
take each old/new four-arm maximum, assert equality, and compare all resulting
old/new/delta/max/margin fields with the sealed reviewer DOMINANCE_AND_SUMMARY.
The asserted summary minima are 272896, 99241 and 228723 respectively.

Execution used the supplied VENV Python -B in inline exact integer/file-metadata
commands only. Git immutable reads explicitly set GIT_OPTIONAL_LOCKS=0.
The family content hash, full candidate/review manifest and every payload were
verified. Reads of candidate assemble.py:250–338 were formula inspection only;
METHOD_AND_JOIN.md was read to bind the four shared outcome-slot interfaces.
Raw paths/revisions/content hashes remain in the adjacent provenance records.
