# MANAGER_RETURN — W3 PKG-12

DEL-12-01 PASS forward=e2019839dc83a518b2b69c0d284bc8fcac101231563093200a4f7da8a61b8f53 reverse=6f7e73d894b2dca7b8957e7e39100c6d40225a4838ca0a7e5c5eae61d263f8c4 rows=138
DEL-12-02 PASS forward=b24c5042add02c82d111d58b20ec4c08c2ca11bfa14bb6105289919ed1a29f12 reverse=11182de2cd40078e8fd43adf29a9041c374813634d03bae87e41ae291034fde9 rows=147
DEL-12-03 PASS forward=b7869d18ce4f3733e780ec80e8af262cf133b98d4e9049c5e25a5f7a1eac8a0f reverse=1ba6ede41e8679fe3e54363a1cbfb8f4fe3cdab2b853bd079d05571ca523c7f4 rows=98
DEL-12-04 PASS forward=85e41a75ee4bcf89a44ddabad82dcdbe51cae7fdaecd27bf48ec07f4baff6f09 reverse=90e3524abaa45af73c337bc65e9f736634195f155a51936068e5e6ef95a07f35 rows=113
DEL-12-05 PASS forward=fc28af0d272e443dc798f6dae53dee17e9d4ab9dc08f8c0cce595b65f762dd57 reverse=cc88f4df296dabcbb2c542242141c995ae1db702b4bb662e93cdb208df246a39 rows=158

BATCH PASS 0 consistency findings (5 forward ledgers). The worker-level batches also passed with 0 findings. No WAVES/W3/RESOLUTIONS.csv existed at run time. Full output: BATCH_PKG-12.txt.

Children (general-purpose, opus, reasoning "high (inherited)", nested harness-native Agent tool, foreground; 1 live at most, within budget 1; G2 was launched only after G1 returned):
- G1 a90621afa63129ffb: DEL-12-01, DEL-12-02, DEL-12-03. A 529 interrupted it after DEL-12-01 was sealed and while DEL-12-02 was unsealed. The manager resumed the same agent by SendMessage, and Agent 0 also resumed it directly. It was not a rerun. Agent 0 relayed the final return. Records: RETURN_G1_INTERRUPTED_1.md and RETURN_G1.md.
- G2 a49be139655083c23: DEL-12-04, DEL-12-05. LAUNCH_G2.md was revised before launch to add the scratch-path clause and a 529-resume note. The revision is logged in LAUNCHES.jsonl with the old and new hashes.

Reruns: none. Validators: all 5 pass in single mode, run with --reverse, --inventory and --notes-gap (0 findings each). Seals: for all 5, the recomputed forward hash equals the SEAL hash and the worker-reported hash. Sentinels and notes files are present for all 5, and no _scratch_* files remain.

Items the workers disclosed for Agent 0 and the verifiers (taken from the returns; the manager did not judge them):
- R4 owner items:
  - DEL-12-01 FG-DEL-12-01-01 (7 rows) and DEL-12-02 REXC-OI-010: the desktop project store location (Tauri app-local data dir) was settled in code with no ruling located, while SPEC §4.4 is still TBD.
  - DEL-12-02 REXC-CON-002: the proposal is implemented, but the ruling column is still TBD.
  - DEL-12-05 FG-DEL-12-05-02 (MEDIUM confidence): under DEC-051, model providers can receive private data with no guard, which conflicts with the SOW's disclosure theme. The worker recorded it as ruling-redirected, not AUTHORITY_CONFLICT.
- INVARIANT rows, none a known violation:
  - DEL-12-01 LFSP-REQ-008 and -010;
  - DEL-12-02 CLM-004, REXC-REQ-010 and -012;
  - DEL-12-03 TEL-REQ-009, -010 and TEL-TEST-006. The plugin clauses hold only because no plugin runtime exists (CP-11).
  - DEL-12-04 CLM-004.r04, .r05, CLM-007.r05 and CLM-011.r01, .r06, .r10. The quarantine taxonomy is held under PDU-034.
- UNKNOWN rows: DEL-12-04 CF-001 and CF-002.
- The verifier must see:
  - The sealed DEL-12-04 notes on CLM-004.r01, CLM-011.r01 and CLM-027 wrongly say no product flow exists. `save_local_library` in apps/desktop/src-tauri/src/lib.rs refuses quarantined imports. The worker would keep those rows PARTIALLY_IMPLEMENTED and corrected this in its notes, not in the ledger.
- Record errors for R3:
  - The DEL-12-04 SOW claims telemetry-exclusion tests that do not exist (FG-DEL-12-04-01). Its helper has no product caller.
  - DEL-12-05 threat_model.md is stale: review triggers fired twice, and the redaction workflow is still TBD (CLM-036).
  - DEL-12-02 RF-001 and RF-002 are OPEN but missing from its Remaining section, and its SOW still says runtime integration is TBD.
- CP-04 rename residue is on the DEL-12-04 and DEL-12-05 SOW surfaces, threat_model.md, and the `openpipestress.*` document_kind identifiers.
- The workers report no protected-check, ISSUED or AUTHORITY_CONFLICT rows.
- Boundary disclosure (G1): before the scratch-path instruction, G1 wrote scratch files to the shared session scratchpad. Its `_scratch_lib.py` there was overwritten by a PKG-15 worker's file of the same name, and G1 left that file untouched. G1 says it deleted all its own scratch files and modified no file it does not own. G2 reports writing nothing to the shared scratchpad or the freeze.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
