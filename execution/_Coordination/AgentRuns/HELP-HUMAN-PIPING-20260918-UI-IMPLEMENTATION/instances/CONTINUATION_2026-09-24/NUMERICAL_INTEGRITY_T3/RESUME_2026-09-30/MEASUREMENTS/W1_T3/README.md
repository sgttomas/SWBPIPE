# W1-T3 raw delta preservation

ROOT preserves191 new T3 files plus the complete108-row journal. All192 payloads
in delta_records.tar.gz match the original raw inventory byte-for-byte. The
journal retains the exact313009-byte T1/T2 prefix from the immutable T2 snapshot.
Earlier raw files remain in their canonical T1/T2 archives and input packets.

Archive paths are relative to the measurement root. Container metadata is
normalized; original time provenance remains in the sealed author packet.
This immutable snapshot is preserved before any future working-journal append.
No payload was rewritten, no prior packet copied, and no measurement acceptance
or next-tier authority follows from preservation.
