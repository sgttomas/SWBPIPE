# I27 remaining VR mutations

**All four assigned exact-semantic mutants were detected by the unchanged reference test.** VR-M01, VR-M03, VR-M04 and VR-M06 compiled successfully, failed their intended existing assertion, and each had passing same-filter NONE controls before and after. No variant remains unexecuted in this block. No maintained source, test, criterion, observation or numerical input was changed.

Frozen maintained basis: `81c03849033f3ce745668f581f446530789397b8`. The complete core dependency tree and VR were archived from that revision before compiling. The eight actual NONE executions passed the full193-reference comparison; this block did not run the other six integration tests. Every mutant stopped at the first RF-CHAIN-T-n03-r1e-04 requested assertion, so no later moving assertion or complete mutated roster is claimed.

| Mutant | Intended observed failure |
|---|---|
| VR-M01-OLD-PORT | Requested five-field result was `[10440, 26707, 19853952, 20743723, 20121083]`, expected `[17806, 59937, 18770496, 19774883, 19081795]`. Caller phase/addend assertions passed first. |
| VR-M03-RECORD-COPY | Record3 output phase11:194,446 versus241,417, missing46,971. |
| VR-M04-LATE-RECORD | Late RCM/sparse/summary phases12/13/14:36,480/239,301/11,909 versus83,451/286,272/58,880; each missing46,971. |
| VR-M06-SUMMARY-PREFIX | Summary phase14:58,827 versus58,880, missing53. |

Every retargeted patch has exactly the same added and removed lines as its preserved I24 original. Only hunk positions/context changed for the reviewed helpers. The old port's32 constant declarations and its wide_bytes, model_bytes and estimate bodies independently match historical source40129 byte-for-byte; only the prepared type/result adapters are present. There is no invented zero, broadened fault or compilation-based kill. Detailed interpretation and replay evidence are in [REASONING.md](REASONING.md) and [_run_records](_run_records/COMMANDS.md).

All632 archive files in each of the five source directories match the normal manifest after restoration. Every mutant source was restored after its trial; the last compiled/revalidated artifact and last executed control are normal. Separately preserved mutant binaries remain diagnostic artifacts with explicit hashes. The maintained core and VR trees still match the frozen basis. Runtime ended2026-10-02 at03:42:35 UTC and the Cargo lane was returned to ROOT before sealing.

VR-M02 and VR-M05 were intentionally not repeated; their prior execution and independent RV35 confirmation remain authoritative within their recorded scope. I24's earlier preparation remains unexecuted historical preparation, not a retroactively assigned survivor or kill. This block supplies conditional formula/owner regression evidence, not measured excess, a complete global bound, solver qualification, engineering acceptance or release. ROOT owns integration and programme closure. No V-K seeded matrix, solver/performance/10k run, host/config/guard change, Git/index mutation, delegation or automatic follow-on occurred.
