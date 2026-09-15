# TASK review repair V10 — checked CLI Cargo target

Continue the existing sole TASK / Type 2 writer under WORKING_ITEMS, gpt-5.6-sol / high. Do not delegate.

Consume briefs/COMPATIBILITY_REVIEW_REPAIR_V10.md (SHA-256 aec01b36f60f672edff6108fc30fdc1454a397079f1f24d562941b635aaebb68) and this exact scope. Preserve the frozen V10 source and evidence.

Edit exactly projects/chirality-piping/core/serialization/canonical_json/Cargo.toml. Add an explicit named binary target for src/bin/openpipestress_jcs_ijson.rs with required-features = ["checked-cli"]. Do not modify the binary source, features, dependencies, library, lock file, or any other product path.

The active native lease prohibits builds, Cargo tests, browser execution and native execution. Perform static inspection only. Record the pre-repair Cargo.toml hash, the static expectation that ordinary feature-off target discovery would otherwise compile a crate whose crate-level cfg removes main, the post-repair source hash, and the exact pending verification commands. Do not claim an observed E0601.

Write run evidence only under instances/COMPATIBILITY/REVIEW_REPAIR_V10/TASK_WRITER/**. Return the minimal source patch and wait for Root's verification lease. No Git.
