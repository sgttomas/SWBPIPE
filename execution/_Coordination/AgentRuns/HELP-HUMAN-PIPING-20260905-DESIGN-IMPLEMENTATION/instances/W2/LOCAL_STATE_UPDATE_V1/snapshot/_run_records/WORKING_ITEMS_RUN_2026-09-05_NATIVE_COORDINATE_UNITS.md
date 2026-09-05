# WORKING_ITEMS — native coordinate units repair

Date: 2026-09-05. Parent HELP_HUMAN; manager W2 WORKING_ITEMS; primary PKG-02 DEL-02-02, SOW-025, OBJ-001/OBJ-012, OUT-001/AC-001/VER-001. Accepted authority: owner direction to implement report/map, root plan and exact source/fixture amendments. Applicable invariants OPS-K-UNIT-1, OPS-K-DATA-2, OPS-K-ID-1 and OPS-K-AGENT-1..4. Root-derived WORKING_ROOT is projects/chirality-piping in the active checkout. Native execution model identifiers unavailable; role/non-delegation evidence instruction+config asserted.

## Bounded result

The product-physics adapter retains project unit metadata and normalizes raw node positions before constructing the numerical model. Missing/invalid declared units block execution; no implicit SI default is introduced. Direction vectors and explicit quantities retain their separate semantics. Normalization operates on the consumed working request, preserving authored inputs and identities. Existing metre results remain stable. The only supporting fixture change declares its already documented metre basis; no numerical input or expected result changes.

## Accepted evidence

All links resolve from this record into the parent run.

- [Implementation brief](../../../../_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/instances/W2/I1/BRIEF.md), with V2 portability and V3 exact fixture amendments in the same directory.
- [W2 frozen return manifest](../../../../_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/instances/W2/snapshots/FINAL_V1/MANIFEST.json), SHA256 `f464f57b592520c4f5c3096c82b5d8992950a1fa2b40be33a17aa85f7acbf945`.
- [Fresh independent R1 review](../../../../_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/instances/W2/R1/RETURN.md): PASS, no actionable findings across 100% frozen two-file diff.
- [Root bounded repair acceptance](../../../../_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/dispositions/ACCEPT_W2_BOUNDED_REPAIR.json).
- [Independent repaired native witness manifest](../../../../_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/instances/A2/_run_records/REPAIRED_UNITS_V1/MANIFEST.json), SHA256 `730e2f7ac617a18d9227f04dd70356e87e3339a9713903592c7f5d3fbb6cdce8`.
- [Root repaired witness acceptance](../../../../_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/dispositions/ACCEPT_A2_REPAIRED_UNITS.json).

Baseline Rust 124/124; final 127/127; two mutation failures establish regression sensitivity. Independent original m/mm/in fixtures now yield exactly equal full envelopes, maximum displacement 0.406023 mm. Claims are bounded verification, not engineering suitability or GUI-transport acceptance. No tests repeated for this documentation-only local-state update. Parent separately reported Piping pytest 1011 PASS; that report does not replace the clean integrated closeout gates.

## State and handoff

Only DEL-02-02 `_STATUS.md`, `MEMORY.md` and this record changed in the supplemental update. Remaining narrows the completed coordinate boundary while preserving broader B2/B3, PDU-037, PDU-015, PDU-025 and PDU-048 obligations. `IN_PROGRESS` remains unchanged. No decomposition, dependencies, scope, catalogue/policy or lifecycle ruling. Immutable I1/R1/FINAL_V1 evidence remains unchanged; this record is derivative implementation evidence. Parent retains integrated review/native build/clean DEC-025, consumer routing and Git closeout. Rerun evidence and review if source changes; this local-state update requires link/hash/paired-state verification only.
