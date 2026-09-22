# MANAGER_RETURN — W3 PKG-15

DEL-15-01 PASS forward=25dc9cc513493062de12df103a298158b67dad2d707e1baf988fbcb19b377ec1 reverse=d9d1959f8f46007785c2f2bf0b7563ee5c20995c4052e9713cf9de61dc9dab69 rows=93
DEL-15-02 PASS forward=86d7e398d26bd21afca629325f1be86892db40bce5b5fe5590d8f21a88b3ac73 reverse=203d82b4554524f6111d499ce909b90120521d40179ff3a9bc89e4c525e6867c rows=91
DEL-15-03 PASS forward=daa79746737339137c6a767374bd69097a2cf286d6e9b659ea64903b943f9fc3 reverse=6168bdf653601307ba9c5286c6a728360b024095b193c1de0e4b7a2294e6e16d rows=97
DEL-15-04 PASS forward=474bd64ad670c19c6ed48c46b260322654c8fc8078fa29cfdaaf504c9bc9b7f4 reverse=dc8a8ef8121fcb3adbfb4594e55119d9cc6bd6c2f3572b9208dee4f0fc98300b rows=87

BATCH PASS 0 consistency findings (4 forward ledgers; BATCH_PKG-15.txt).

Children (general-purpose, opus, reasoning "high (inherited)", nested harness-native Agent tool, foreground):
- G1 a31c68c22a2019482: DEL-15-01, DEL-15-02, DEL-15-03, DEL-15-04

Mechanism note: the manager's foreground call to G1 ended on an API 529 with partial output. Agent 0 then resumed the child directly by SendMessage and saved its final return to returns/W3-PKG-15-G1-a31c68c22a2019482_return.md. The manager copied that file verbatim as RETURN_G1.md. The manager did not receive the return through its own call.

Reruns: none. Validators: all 4 pass in single mode with --reverse, --inventory and --notes-gap, with 0 findings each (VALIDATION_DEL-15-0N.txt). Seals: for all 4, the recomputed forward hash equals the SEAL hash and the hash the worker reported. The reverse and notes hashes also match the worker's report. Every #END sentinel is present with the matching count. No _scratch_* files remain.

Worker-disclosed items for Agent 0 and the verifiers (taken from the return; the manager has not judged them):
- Possible defect, DEL-15-03 FG-DEL-15-03-02 (INVARIANT/PROJECT_BASELINE, owner decision): since PR #307 the exporter redacts fields the SOW requires it to preserve. This goes to R4 with PR #307 as context.
- Possible defect, DEL-15-02 FG-DEL-15-02-02 (INVARIANT): when mapping_status or value_kind is missing, the builder silently fills in a default and raises no diagnostic.
- F7 product gaps: no product code calls the core/handoff modules. HandoffPanel output does not follow handoff_package.schema.json (FG-DEL-15-03-01). The desktop prover packet includes unit_policy_evidence, which its schema does not allow (FG-DEL-15-04-01).
- DEL-15-04 R7: a hash-bound acceptance reference cannot be represented (CP-11, deferred by DEC-081; INVARIANT, owner authority needed).
- No ISSUED, protected-check or authority-conflict rows are reported. Selectability is NOT_APPLICABLE.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
