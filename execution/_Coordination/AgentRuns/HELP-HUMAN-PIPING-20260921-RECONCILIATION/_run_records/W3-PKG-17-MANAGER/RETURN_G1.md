DONE DEL-17-01 forward=f9743d595435f8ebff354752e90f8c044063f709a8be560009f73de39f6dd50e reverse=ea4f399319e9a2f254937756f3b9a471b4ef885bcf3e24260662cc05d31c59d5 notes=94bf9158ff7fe7e93b94c61751836e444cab61353af110c532df872cf538e6f0 validator=PASS
DONE DEL-17-02 forward=9ddcda4be105a325ce9333e8bef7b4310a8651643e990e6f7731ead13d2f668c reverse=daa4c8424828f7bcc5fec8ef371052615b6acc1c88903af0b3e3fd0c4cccf288 notes=09f027bbff5034e47a13d46f10ae0f5f9d55e446ba770f63fbc77efc058b0865 validator=PASS
DONE DEL-17-03 forward=52d14b389911d238a0344373547cb634b414cb60be747c7bf3e1566a419be70a reverse=88b73fdd3def2741f666a75ee99befd056fdb8c164b5001f4035697895461827 notes=675bcce302b392f0472e757c54ac6d96debc61cb0960bbba2a7f543cf0942c25 validator=PASS
BATCH PASS 0 findings

- **Disposition counts (328 rows):** 201 ALIGNED, 35 STALE_REVIEW_OR_EVIDENCE, 14 STALE_SETUP_SPECIFICATION, 15 PARTIALLY_IMPLEMENTED, 6 IMPLEMENTED_DIFFERENTLY, 1 DOCUMENTED_UNIMPLEMENTED, 29 COVERED_BY_CHILDREN, 27 NOT_ASSESSED, 0 UNKNOWN. Reverse answers: DEL-17-03 CLAIMED_BY 3 (native schema, fixture, builder); DEL-17-02 COVERS 29; DEL-17-01 COVERS 16; the rest NOT_MINE.
- **Top cause tags:** BASIS_POINTER_STALE 16, RECORD_DRIFT 16, PARTIAL_SLICE 12, REPRESENTATION_MIGRATED 11, POSSIBLE_DEFECT 5.
- **Missing source:** `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` (PLAN-EXPORT-INTEROP) is not in the frozen tree. Commit `349a2ab33` deleted it on 2026-06-03 with no archived copy. DEL-17-01 and DEL-17-02 still cite it, and so does the PCF profile code. These rows use the CP-08 pattern with AuthorityNeeded OWNER.
- **Possible defect, PROJECT_BASELINE, owner decision (DEL-17-02 REQ-007; DEL-17-03 AC-001):** every exporter hashes sorted, compact Python JSON. The project's own code says this is not JCS, but REQ-007 and DEC-028 name a JCS-compatible hash basis. Two related points:
  - DEC-074 E1 narrowed these hash labels to the project-local name. If E1 governs exports, the cause becomes SCOPE_REDIRECTED_BY_RULING. The E1 text is in an excluded July PROPOSED_* file, so I did not read it.
  - The DEL-17-04 MBF package still labels the same bytes `JCS_compatible_json_payload_hash`. That looks like a mislabel, for the verifier.
- **Possible defect, minor:** native JSON uses the loss category token `TBD` where the DEL-17-02 contract and the other four exporters use `tbd` (DEL-17-02 CLM-012, REQ-054, CLM-043; DEL-17-03 REQ-004).
- **TBD-17-01-006 (glTF identity policy):** DEL-17-01 and DEL-17-02 still list it as open, but DEL-17-08 selected a policy for its first profile. I used cause DOC_BEHIND_CODE. DEL-17-08's SOW cites DEC-074 O11/E7 for the selection, which I did not read; if that option ruled it, the cause would be SCOPE_REDIRECTED_BY_RULING.
- **No product caller:** nothing in the app calls the native JSON builder, so DEL-17-03's aligned rows carry `PRODUCT_CALLER: NONE`. No row is ISSUED, protected-check, invariant-tier or authority-conflict; the only PROJECT_BASELINE rows are the two hash-basis rows above.

Output is under `.../RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-17/DEL-17-0{1,2,3}/`, plus the carry-forward notebook `.../WAVES/W3/PKG-17/_WORKER_DEL-17-01_NOTES.md`. Scratch files are deleted.
agentId: ac7b47ca7d4965602 (use SendMessage with to: 'ac7b47ca7d4965602', summary: '<5-10 word recap>' to continue this agent)
<usage>subagent_tokens: 437956
tool_uses: 97
duration_ms: 1686760</usage>
