# I27 A_FIRST runtime return

**Both assigned routes completed with their intended semantic results and all four normal controls passing.** A fresh complete core+VR archive at maintained81c03849033f3ce745668f581f446530789397b8 and a newly created target supplied the Cargo-emitted seeded debug adapter. No old executable was reused.

| Process | Exit | Result |
|---|---:|---|
| F10 NONE before |0| Complete original permutation test passed. |
| F10 fault |101| RF-CHAIN-T-n03-r1e-04 canonical-byte assertion failed at adapter.rs73: two535-byte arrays,14 differing positions. |
| F10 NONE after |0| Complete original permutation test passed. |
| UNKNOWN NONE before |0| Complete original permutation test passed. |
| UNKNOWN fault |101| Exact `FK_SEEDED_FAULT: unknown fault id "VK-UNKNOWN"` panic at seeded.rs81:17. |
| UNKNOWN NONE after |0| Complete original permutation test passed. |

All six fresh processes used the exact original filter/arguments and the same new executable. Cargo confirms debug opt0/test=true, VR seeded-faults and FK mutation-controls. Binary SHA256: `86245450e42fa9175215a421a833b046374c6088a51960cb418419a17dee4e50`. All632 source files and22 raw fingerprints were checked before every process and after the block. The unchanged maintained core/VR scope still matches81c038. The last process is NONE on that same seeded artifact; this does not qualify an ordinary or release artifact.

The inline UNKNOWN transcript check initially expected an unquoted ID and stopped before the last control. The recorded panic was already correct: unchanged source uses Debug formatting, and historical P37 prints the same quoted ID. Read-only inspection corrected only that interpretation; ROOT independently confirmed it afterwards. The failed evidence check is preserved under [_run_records](_run_records/CHECK_INTERRUPTION.txt). No fault was rerun, no source/test/criterion changed and no extra runtime process was added. The originally granted final NONE passed.

Runtime ended04:16:52 UTC and the lane was returned to ROOT before sealing, within the original cutoff/deadline. Exact commands/environment, immutable source/plan/brief bindings, successful build stream, binary/features/profile/fingerprints, both full canonical arrays, unabridged logs and final checks are under [_run_records](_run_records/COMMANDS.md).

No assigned process remains unrun. The two fault processes stop at their intended first failure; no complete fault-case loop is inferred. No retained solve,10000-member construction, performance qualification, B/C/D group, overlay/new driver, maintained edit, Git/index mutation or delegation occurred. The remaining14 registered fault IDs and full mechanical matrix remain outside this grant. ROOT owns independent result review and programme closure; no automatic follow-on is running.
