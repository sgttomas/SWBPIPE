# MANAGER_RETURN — W2 PKG-04

DEL-04-01 PASS forward=cf1f048c31b2b87689f3fedcd1b07d339ebbfaa67ab3b5d58f28d8adf5bfe48d reverse=af3a502209f4fdbb26700b417831d088997c34a8f14499e25a2e1c546ea28bc6 rows=80
DEL-04-02 PASS forward=81a9209c6317569c11962c312aa59e4021274068e8a31929d8d2e87782ff3056 reverse=f0320ee2f393399fadead0d64809f66afacafd0a3d03a1ff22dd665fe6ad9f66 rows=64
DEL-04-03 PASS forward=b46da85362130870f6df6d877d5860e085010da0850ce58be5f9f778c81951a7 reverse=2bb28d95724bb8dc228819504e86c69233680024e66051428e800a0b5cbf0796 rows=71
DEL-04-04 PASS forward=cc63802abc938868694bc586c5534434f99cdf53ffd99754d3d9716ab58692c3 reverse=ccd9b410fbc543281ecde4bb9dc111242a937b9fa37bf8684d62d851260af825 rows=72
DEL-04-05 PASS forward=e3570bd7bc8817a301d2d34d26941500807496c5fe933355f62d2e000e3deb49 reverse=231411bf0448abce51248f94fe0f441044ce0df819bda8c4e2ae24000f73eed3 rows=71
DEL-04-06 PASS forward=8450435e6e1568ffda21f85cef02f571cea593ac46cc7457791787d132964156 reverse=f434eff4b2eb417472b74045a4ffd5ea1ed6a33d3864cf8267c68d7a7049d8bd rows=84

BATCH FAIL 2 consistency findings (6 forward ledgers; not defects, recorded): DEL-04-01 SOW#CLM-014 and SOW#CLM-021 are STALE_REVIEW_OR_EVIDENCE/DOC_BEHIND_CODE where the CP-03 majority (8 rows) is BASIS_POINTER_STALE, and there is no CANONICAL_DEPARTURE. Worker-level batches (per group) both reported PASS 0; the flag appears only when the two groups are run together. No WAVES/W2/RESOLUTIONS.csv existed at run time. Full output: BATCH_PKG-04.txt.

Children (general-purpose, opus, reasoning "high (inherited)", nested harness-native Agent tool, foreground):
- G1 a028feced4c385932: DEL-04-01, DEL-04-02, DEL-04-03
- G2 a5543ab9241a8fc15: DEL-04-04, DEL-04-05, DEL-04-06

Reruns: none. Validators: all 6 pass (single mode, with --reverse, --inventory and --notes-gap; 0 findings). Seals: the recomputed forward hash equals the SEAL hash and the worker-reported hash for all 6.

Worker-disclosed items for Agent 0 and the verifiers (from the returns, not judged by the manager):
- G1 self-reports a sealed-row correction: DEL-04-02 SOW#CLM-010 (ALIGNED) should carry PRODUCT_CALLER: NONE (F7). It is left unedited and recorded in the notes.
- G2 boundary slip: a hash glob printed the G1 SEAL lines and forward hashes. No ledger content was read, and the slip is disclosed in each G2 notes file.
- PROTECTED_CHECK: DEL-04-04 REQ-08 (VERIFICATION_REMOVED, INVARIANT; PR #787 / b43cc00c4).
- UNKEYED ownership: curved_bend (G1) and nonlinear_integration (G2).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
