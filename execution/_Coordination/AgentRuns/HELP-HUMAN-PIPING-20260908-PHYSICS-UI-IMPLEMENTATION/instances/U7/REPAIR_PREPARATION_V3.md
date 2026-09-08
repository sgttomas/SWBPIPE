# U7 Repair Preparation V3

Status: `HELD_FOR_RU_TERMINAL_BINDINGS_AND_ROOT_RELEASE`; no product edit or Agent 2 dispatch is authorized. This succeeds `REPAIR_PREPARATION_V2.md` without changing it.

Frozen V1 input remains bound by manifest SHA-256 `9f71f16fe91112a42ed01fc69aee040665c4c1663d3e9d5a02d8df00747f4446`. The V2 continuation and busy/permitted-invalidation findings remain in scope. Root supplied these additional independently reproduced RU results:

1. a single-operation direct Apply publishes when its returned acceptance object is incomplete;
2. malformed create metadata lacking `target.object_type` falsely reserves its `target.ref`;
3. an atomic new-end route permits its new node and pipe to use one identical ID;
4. untouched draft provenance defaults to `user_entered_local_preview`, conflicting with the adopted explicit-entry/no-invented-default contract.

Structural equality probes pass and are not a repair target. Final focused tests and final-cut browser journey also pass; their evidence is preserved rather than treated as closure of the new findings.

The bounded successor must add genuine within-fence regressions that prove:

- single Apply fails closed before model, receipt, retained context, or checkpoint publication when any acceptance evidence required by the terminal RU finding is incomplete;
- receipt and model-basis checks match the exact terminal RU evidence and existing service contract, without inventing fields or changing a service/type file;
- reservation ignores malformed create/connect/insert metadata lacking the required target object type while leaving the original untrusted evidence unchanged;
- a new endpoint ID and its pipe ID must differ before the atomic batch can be reviewed;
- node and pipe provenance start blank, Add remains disabled until explicit provenance is entered, and entered provenance survives exact intent/review/application behavior.

The V2 requirements remain: successful existing-end Continue survives the model update; implicated draft controls are disabled during Add/Apply; actual permitted invalidation paths fail closed; existing exact receipt behavior is proven when the terminal RU packet defines the missing acceptance/model-basis boundary.

All repair work remains within the same nine U7 product paths. No service, type, Toolbar, existing-form, Rust/Tauri, DEL-07-02, schema, default substitution, threshold, lifecycle, dependency, selective-solve, model-slicing, or global-window edit is permitted. RU terminal packet path/hash/finding IDs and root release remain `TBD_PENDING_RU_TERMINAL`; the draft is not launch authority.
