# I23 — authenticate the two already installed serializer packages

ROOT grants existing I23 one 10-minute read-only evidence task. Source11's
V-LIB gap is specifically the identity of local serde_json1.0.151 and
serde_core1.0.229 source against the pinned Cargo.lock packages. The absent
.cargo-checksum.json files must remain recorded as absent.

Read source11/BINDING.json and the locked package checksums. Inspect only the
ordinary existing Cargo registry cache/source roots for those exact packages.
If their cached .crate archives exist, hash each archive against Cargo.lock,
read its members without extracting over the source, and compare the actual
installed package files and the seven directly cited source/manifest pages.
Distinguish package authentication, local-file equality and enabled-feature
evidence; none implies a new build. Record any generated marker or extra file
separately rather than silently treating it as authenticated package content.

No Cargo/Rust/build/test, fetch/network/install, package restoration, cache
mutation, permission/configuration change, new tool/framework or dependency
audit. If the cached archives or required members are absent/mismatching,
return that concrete gap and stop. This is an existing-file provenance check,
not permission to engineer around a missing tool or dependency.

Write only K6C R/I23/serde_binding_07 and own scratch evidence if needed, with
portable origins, exact raw hashes, comparison results, RETURN and seal. Do
not copy raw package trees or change source11. No Git/index operations or
delegation. Return within the actual 10-minute boundary; no E_max, serializer
allocation theorem, runtime or merge acceptance follows.
