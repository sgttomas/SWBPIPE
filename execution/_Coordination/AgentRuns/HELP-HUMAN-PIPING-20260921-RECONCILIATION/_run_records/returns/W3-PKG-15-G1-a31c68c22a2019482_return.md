# Return — W3 PKG-15 G1 worker a31c68c22a2019482 (resumed by Agent 0), verbatim

Received by Agent 0 during the second 529 pause, after four resumes. It is
relayed verbatim to the PKG-15 manager (aa623d46f548e1a44) when the manager is
resumed.

---- WORKER RETURN (verbatim) ----
DONE DEL-15-01 forward=25dc9cc513493062de12df103a298158b67dad2d707e1baf988fbcb19b377ec1 reverse=d9d1959f8f46007785c2f2bf0b7563ee5c20995c4052e9713cf9de61dc9dab69 notes=f5bac28d92b13ebdee6bfa18ae9625e0f30a30997a3594b389f966be5359fa6f validator=PASS
DONE DEL-15-02 forward=86d7e398d26bd21afca629325f1be86892db40bce5b5fe5590d8f21a88b3ac73 reverse=203d82b4554524f6111d499ce909b90120521d40179ff3a9bc89e4c525e6867c notes=9a02ace7e183f800695ac87f56f50e576d0d462aaffabbd7c5d951470a4706bd validator=PASS
DONE DEL-15-03 forward=daa79746737339137c6a767374bd69097a2cf286d6e9b659ea64903b943f9fc3 reverse=6168bdf653601307ba9c5286c6a728360b024095b193c1de0e4b7a2294e6e16d notes=4c47dea7dfae7769c1e2fcf3eaaae3964c3a2d700a2be5770f0ec928d7ee7f03 validator=PASS
DONE DEL-15-04 forward=474bd64ad670c19c6ed48c46b260322654c8fc8078fa29cfdaaf504c9bc9b7f4 reverse=dc8a8ef8121fcb3adbfb4594e55119d9cc6bd6c2f3572b9208dee4f0fc98300b notes=930744761256037d087518a019e237ab8e6855464c550e767828f1cea87490a4 validator=PASS
BATCH PASS 0 findings

- Seals and scratch: before resuming, I checked the DEL-15-01 seal against the file on disk and it matched. I did not touch that file. All four seal hashes match their forward files, and every _scratch_* file is deleted.
- Dispositions (368 forward rows):
  - 188 ALIGNED, 67 STALE_REVIEW_OR_EVIDENCE, 27 IMPLEMENTED_DIFFERENTLY, 15 PARTIALLY_IMPLEMENTED
  - 7 STALE_SETUP_SPECIFICATION, 5 DOCUMENTED_UNIMPLEMENTED, 23 COVERED_BY_CHILDREN, 36 NOT_ASSESSED
  - No UNKNOWN rows.
  - Reverse answers: 7 CLAIMED_BY, 2 PARTIAL, 1,151 NOT_MINE.
- Top cause tags: BASIS_POINTER_STALE 40, POSSIBLE_DEFECT 15, PARTIAL_SLICE 14, then EVIDENCE_OVERTAKEN, RECORD_DRIFT and DOC_BEHIND_CODE at 8 each.
- Possible defect, DEL-15-03 (FG-DEL-15-03-02; owner decision, INVARIANT/PROJECT_BASELINE, MEDIUM): since PR #307 the exporter replaces almost everything in its output with [REDACTED]. That includes the model hash, unit values, entity IDs, references, assumptions, warnings and even professional-boundary values, for invented public data. The scope of work requires these to be preserved. A merged PR can't amend that on its own, so this goes to R4 with PR #307 as context.
- Possible defect, DEL-15-02 (FG-DEL-15-02-02; INVARIANT, MEDIUM, from reading the code only): when a mapping record omits mapping_status, the target-mapping builder silently fills in mapped. When it omits value_kind, the builder fills in metadata, which skips the unit-metadata check. No diagnostic is raised in either case.
- Product gap (F7): none of the core/handoff Python modules is called from product code.
  - The desktop handoff export (HandoffPanel) doesn't follow handoff_package.schema.json, so no product path produces schema-compliant handoff packages (SOW-074; FG-DEL-15-03-01).
  - The desktop external-prover packet adds unit_policy_evidence, which its strict schema doesn't allow (FG-DEL-15-04-01).
- Other items for the verifier:
  - DEL-15-04 R7: a hash-bound human-acceptance reference can't be represented. This is CP-11, deferred by DEC-081, marked INVARIANT with owner authority needed.
  - DEL-15-02 CT-001 still shows "human ruling TBD", but the v3.1 normalization in the dependency mirror and the approved DEL-007 type-system rectification have overtaken it.
  - Rename residue (CP-04) is recorded once on each Scope of Work surface row, including the active openpipestress.org schema $id values.
- No ISSUED, protected-check or authority-conflict rows. Selectability is NOT_APPLICABLE throughout.

Files are in RUN/WAVES/W3/PKG-15/: one folder per deliverable holding the forward ledger, seal, reverse file and notes, plus _WORKER_DEL-15-01_NOTES.md. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
---- END ----
