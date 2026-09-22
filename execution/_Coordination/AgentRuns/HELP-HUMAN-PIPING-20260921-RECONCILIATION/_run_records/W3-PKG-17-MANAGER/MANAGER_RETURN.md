# MANAGER_RETURN — W3 PKG-17

DEL-17-01 PASS forward=f9743d595435f8ebff354752e90f8c044063f709a8be560009f73de39f6dd50e reverse=ea4f399319e9a2f254937756f3b9a471b4ef885bcf3e24260662cc05d31c59d5 rows=96
DEL-17-02 PASS forward=9ddcda4be105a325ce9333e8bef7b4310a8651643e990e6f7731ead13d2f668c reverse=daa4c8424828f7bcc5fec8ef371052615b6acc1c88903af0b3e3fd0c4cccf288 rows=160
DEL-17-03 PASS forward=52d14b389911d238a0344373547cb634b414cb60be747c7bf3e1566a419be70a reverse=88b73fdd3def2741f666a75ee99befd056fdb8c164b5001f4035697895461827 rows=72
DEL-17-04 PASS forward=6d3b3906ecf4eccc8d1f3d04de7937262d72f4bd13b577ed408467b4362ac68d reverse=bcb01884821931fdaccb13885bf96e398a9d768c2185f6485d7d6362d2b4dee3 rows=97
DEL-17-05 PASS forward=39b8be76395157ccefa6f9f0c66edd581017046c097b184bd6fa6eaf048a9587 reverse=3cd8bdc005e399a9ea20484e42a5cd83a59bf6bd84d3947313b34199a9f1422d rows=129
DEL-17-06 PASS forward=e5e7a68099060461034c730e9649170e2a1cf8cfad2b9ade4c9c1fda53324850 reverse=0ca51273fdf2e30fcb2da07a36fd57f365f7e2b512ff3c162549195a15365b23 rows=123
DEL-17-07 PASS forward=b5fce3cdad0804dc14f84fe2960384a547fed5c76129be7fba6ee36dc55a0757 reverse=d85ce1a96ce1fec05bb0ac59a5773b58fb11b23c887d3b3519710a8bbc3e4c0e rows=102
DEL-17-08 PASS forward=490381cfe063811b93f66beef6f97632e85758f6c6504bb367be5fd5e938bcab reverse=cf9a640f511dc20e4cb18ecd023cf012e0664613e18c8b1484db2a63d04aa982 rows=110
DEL-17-09 PASS forward=a47829ee89450e0e4a1bb1881ac6f023bd48b50285c78c768de115059beb4461 reverse=3a38db0132e614db7e5e118d20edb652cd4e44f686d0e898ec307e0c1374c91e rows=91

BATCH FLAGS batch of 9 ledgers: 9 consistency findings (BATCH_PKG-17.txt). Batch-consistency flags are recorded here and are not defects. Each worker's own within-group batch passed, so all 9 flags are cross-group.
- 5 flags: the CONTEXT#architecture-basis-injection rows (same body e8db53739ad57c63). The dispositions differ by group: DEL-17-04 and 17-05 are ALIGNED; DEL-17-07, 17-08 and 17-09 are IMPLEMENTED_DIFFERENTLY / AUTHORITY_UNCLEAR / PROJECT_BASELINE; the majority is STALE_REVIEW_OR_EVIDENCE / BASIS_POINTER_STALE. None carries CANONICAL_DEPARTURE.
- 4 flags: CP-11 PARTIALLY_IMPLEMENTED rows tagged PARTIAL_SLICE where the majority is DEFERRED_BY_RULING: DEL-17-02 CLM-019/REQ-014 and CLM-023/REQ-053, DEL-17-07 CLM-016/REQ-034, and DEL-17-09 CLM-013/REQ-010.
- I made a shell word-splitting error on the first batch invocation, so it failed to run. The rerun is recorded in the same file.

Every validator ran with --reverse, --inventory and --notes-gap and passed with 0 findings. Each forward hash equals the seal hash and the hash the worker reported. The reverse hashes also equal what the workers reported, and each reverse file has 320 rows plus #END. No _scratch files remain. No reruns were needed for defects.

Child agent IDs and worker choices. Mechanism: nested harness-native Agent tool, foreground; model opus, reasoning high (inherited). Worker brief SHA-256 2d793d0a7669618d6eca6f1c8cb66c180ea0edb038a9605d0c0e29556b0db141.
- ab9c074c8a5392451, G1 attempt 1, budget 1. Stopped by API 529 with no output.
- Budget raised to 2, then to 3, by Agent 0. Both raises are recorded in LAUNCHES.jsonl.
- G1 attempt 2, G2 and G3 were launched together under budget 3. All three stopped by 529 with no output:
  - G3 was acaa072c343f59d30;
  - Agent 0 identifies G1 and G2 as a153cf099693426c4 and af123d538d4759193, but the harness returned no ID to me, so I cannot say which is which.
- Choice: no earlier worker was resumed. RUN/WAVES/W3/PKG-17/ did not exist (Agent 0's check, confirmed by mine), so there was no work to keep and nothing to supersede. Per Agent 0's instruction, I launched fresh G1, G2 and G3 with LAUNCH_<g>_R2.md, which adds the scratch-path and early-draft lines.
- Fresh workers, all returned:
  - G1 ac7b47ca7d4965602 (DEL-17-01..03);
  - G2 a2670e354df54fae4 (DEL-17-04..06);
  - G3 ada47f2f6365736c0 (DEL-17-07..09).

Worker-raised items, relayed as reported and not judged; details are in RETURN_G1..G3.md.
- Hash basis. The exporters' "JCS-compatible" hash basis is not RFC 8785 (PROJECT_BASELINE possible defect: DEL-17-02 REQ-007, DEL-17-03 AC-001). G3 flags its own sealed Architecture Basis Injection rows for 17-07 and 17-08 as wrongly saying "JCS-labelled hashing hold". DEL-17-09 CLM-018 is UNKNOWN because of an `rfc8785_jcs` label.
- Missing source. The plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md file is missing at the freeze. The rows citing it use CP-08 with AuthorityNeeded OWNER.
- Possible code defects:
  - MBF `_loss_report` makes MBF-LOSS-REPORT-MISSING unreachable (DEL-17-04);
  - the PCF renderer writes 0 when a coordinate is missing, and the desktop panel treats any section unit other than m as millimetres (DEL-17-07);
  - `write_pcf_export_package` omits `unit_system_disclosure.json` (DEL-17-07);
  - native JSON uses the token `TBD` where the contract uses `tbd` (DEL-17-02 and 17-03).
- IP_DATA at INVARIANT tier. The invented PCF fixture's dimensions match a published table entry (DEL-17-07 REQ-041 and CLM-045, UNKNOWN).
- INVARIANT row. DEL-17-05 CLM-019 is VERIFIED_NOT_VALIDATED.
- Authority conflicts:
  - Rust baseline vs Python/TypeScript code (DEC-009; DEL-17-07..09);
  - the DEL-17-09 REQ-007 taxonomy lists 11 categories but the code implements 9.
- Other items:
  - GUI panels conflict with the SOW GUI exclusions (DEL-17-04 and 17-05);
  - DEC-074 E1/O11/E7 text was not read, because it is in an excluded file;
  - there is frozen-contract rename residue (DEL-17-06).
- No ISSUED rows. No workers reported protected-check rows.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
