# F4 source implementation activation

RunID: `HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION`
InstanceID: `F4`
Parent: `/root` (`HELP_HUMAN` Agent 0)
Role: `WORKING_ITEMS` Agent 1
PackageID: `PKG-04`
SelectedDeliverable: `DEL-04-04`
SourceBasis: `55df51ac3201456e0f181823e3aefefef47a73bb`
Branch: `codex/piping-physics-ui-implementation-20260908`
ModelContract: requested `gpt-5.6-sol`, high reasoning; actual separate runtime identity field is not exposed

The Owner adopted candidate SHA256 `6866bbf52ef861488277b6a659c7ac0a4e63f42865debb971355db5e553ea142`. Root revalidated the source controls, CHANGE Step 0, 25 prerequisite bindings, and seven factual interfaces, then explicitly released F4 source work. The Rust compile slot is assigned exclusively to F4 until focused work freezes.

Production/test source fence:

- `{WORKING_ROOT}/core/solver/nonlinear_integration/src/lib.rs`
- `{WORKING_ROOT}/core/product_physics/src/lib.rs`, optional test-only regression; no production behavior

Evidence/control fence:

- `{RUN_ROOT}/instances/F4/**`
- `{WORKING_ROOT}/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/**`

Implement the independently reviewed `IMPLEMENTATION_BRIEF_V2.md` current-normal affine coupling exactly. Preserve sliding direction/history, DEC-046 thresholds and outer cap, public result/diagnostic semantics, canonical SI, conversion constants, and failure honesty. No other source, lifecycle, dependency, decision, requirement, or Git write is authorized.

Baseline source bindings before edit: nonlinear integration `fb02a52273637f844c63ac2e1545bc7fbe8ab79b58d7f5e259829e46be4b1439`; product physics `e757b8a51e2c4ae68ac4d6c37620663bf4d698ff03b8d6349b40b484bb591903`. Historical witness hashes before edit: Cargo.lock `ac1abea2355eafb839ee15c8dbde2cb500710b66217106c9b036b59d54461cd4`, Cargo.toml `326677deba0692d4eb2bebbb4c4bd009c805fc0a742197798a072afaf4924558`, main.rs `5b4671282466232902d42daeb6ec12391654381c067c9b9276cc6dbd6253ed2d`.
