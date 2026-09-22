# Accepted-state reconciliation refresh

**PASS — current for the final accepted carrier bytes.** This is the append-only source-binding transition authorized by the actual Group-3 answer in `../OWNER_DECISION.md`, “Accept the audited result and adopt DAG-011”. It does not create another acceptance act or repeat the original R1–R6 census.

The original 598 capability keys, 120 bounded facets, 121 claim links, 13 GUI panel bindings, original dispositions and all residual meanings are unchanged. Capability, claim, GUI, residual and summary outputs are byte-identical to the reviewed applied-state outputs. Only `BOUNDARY_RELATIONS.csv` source hashes changed. `BEFORE_AFTER_COMPARISON.json` binds both versions and the validator permits no other semantic difference.

The adapted copy `build_crosswalk_accepted.py` verifies all 51 final carrier targets against the reviewed Group-3 transformation manifests and the carrier TASK's `../carriers/FINAL_TARGET_MANIFEST.json`. It binds 246 current inputs; 30 original input hashes changed exactly within those accepted transformations. `CURRENT_HOOK_BINDINGS.csv` records 31 resolved current hooks, exact excerpts, line loci and source/excerpt hashes. Status/memory pairs remain among the supplied sources. `SOURCE_BINDING_COMPARISON.json` rejects an unreviewed source change, and `FROZEN_ORIGINALS.json` guards 12 original reconciliation files against mutation.

`BUILDER_ADAPTATION.patch` and `BUILDER_IDENTITIES.json` show the exact adaptation: relocate output/input anchors; replace applied-state hash assertions with accepted-state carrier assertions; identify the actual decision and reviewed commit; compare unchanged mappings/residuals against original output bytes; bind current hook excerpts; and verify original evidence preservation. The original builder, ledgers, crosswalks, returns and source records were not edited.

Actual check:

```text
python3 projects/chirality-piping/execution/_ScopeChange/_PostAcceptanceValidation/SCA-011_20260922T173404Z/reconciliation/build_crosswalk_accepted.py --check
```

`VALIDATION.json` records **PASS**, including six negative probes rejecting missing, duplicate and invalid capability/claim keys. Without `--check`, the copy refreshes only this new evidence home; it does not write canonical carriers or original historical evidence.

This closes the source-bound ownership derivative refresh. The 478 capabilities outside targeted review remain outside it. Implementation, schema acceptance/conformance, native behavior, engineering/professional suitability, operator acceptance, dependency satisfaction and release claims remain at their earlier states. Dependency adoption/quote-stage verification is the separate owning TASK's result. The parent integrates this result into the current-state audit and finalization review; no new human checkpoint follows from this refresh.
