# K8 V3 handoff

K8 V3 is author-complete and writer-frozen at source `779dedb8670625b36af07b89fc5557470e47c50e`. V1 and V2 remain byte-frozen. The successor resolves RK-V2-001 by making validation total over malformed JSON-shaped inputs and resolves RK-V2-002 with raw/optional version normalization, an exact tagged outcome, CLI/Tauri routing, and named validators for both advertised read versions. RK-V2-003 remains closed.

The bounded independent backcheck should verify `MANIFEST_V3.json`, reproduce `build_candidate_v3.py` and `adversarial_tests_v3.py`, rerun V2 adversarial regression, and probe additional malformed nested shapes. It should inspect the exact wire enum and the legacy 0.1 partial/no-upgrade handler contract. Coordination is through `/root` only.

No production source/schema/adapter, public compatibility decision, pressure meaning, register, DAG, or lifecycle record changed.
