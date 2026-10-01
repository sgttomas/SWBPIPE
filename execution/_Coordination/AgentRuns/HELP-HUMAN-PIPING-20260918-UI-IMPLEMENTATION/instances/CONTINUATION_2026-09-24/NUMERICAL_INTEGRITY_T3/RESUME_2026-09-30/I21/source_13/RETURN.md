# I21 source13 return — synchronization facade warrants

Same TASK /root/t3_recovery_manager/i21_k6c, parent WORKING_ITEMS /root/t3_recovery_manager. Full grant NUM28c4168ca885f367afff45b54b14eb1e7b6b8600 read. Start2026-10-01 15:32:55 UTC; deadline15:42:55 UTC. Installed Rust1.97.1/aarch64-apple-darwin source only; source12 remains sealed and unchanged.

BRIDGES.md supplies both missing links:

- Public Mutex<()> embeds the previously bound sys::Mutex, inline poison flag and unit data. lock calls that backend; the borrowed guard's Drop updates an inline flag then unlocks. Poisoned results carry the same held guard inline, and ThreadInfo drops them without formatting or unwrap. Its panic-status helper reads a scalar atomic/const TLS Cell with no owned child.
- Public Once embeds sys::Once. new forwards directly; call_once_force keeps a stack Option callback and passes a borrowed adapter to queue.call(true,...). The true flag skips existing-poison propagation; callback allocations remain the already counted source12 identities.

Added registered retained/transient/active-old terms are0 for these two facades on the actual main-thread path. No new lazy child or returned-error heap term appears. Source12's symbolic1028+2*M_mutex+L_info conservative library envelope is unchanged. M_mutex and L_info remain numerically unbound; no layout is inferred. H-I0/C0, checked implementation and final-A1/source-target reconciliation remain open. No complete E_max/admission follows.

LIBRARY_SOURCE.json binds six directly used installed facade/helper/re-export pages, original HTML and decoded hashes, source line anchors and version. Previously supplied source12 helpers/backends are bound in BINDING.json without rewriting them. RAW_COMMANDS.json preserves actual portable commands/results. VERIFICATION.json and SHA256SUMS provide checks and full write inventory. Same RV30 must backcheck this additive correction before dependent reliance; the active review was not consumed as accepted final authority.

All writes are K6C R/I21/source_13; no scratch used. Every Git read used GIT_OPTIONAL_LOCKS=0. No Rust/build/probe/runtime/model/solver/test/measurement or host job, new tool/audit programme, allocator/observer/guard/install/network, maintained source/API/estimate/record/admission/Git/index mutation or delegation. No experiment is running. Return ends this bounded tranche.
