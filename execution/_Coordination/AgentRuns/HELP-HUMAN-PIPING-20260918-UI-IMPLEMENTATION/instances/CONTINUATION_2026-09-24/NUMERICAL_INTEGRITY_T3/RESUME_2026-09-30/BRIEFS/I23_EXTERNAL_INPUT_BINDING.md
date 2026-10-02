# I23 — bind the existing external VR model files

ROOT assigns existing TASK I23 one 15-minute read-only evidence task, separate
from I21's active VR derivation. Purpose: close source_09's concrete external
file identity/size inputs without a solver or generation run.

Read K6C R/I21/source_09/FIXTURE_INPUTS.json and its RETURN/source binding;
exactly12 embedded_model=false rows name the required RF-LARGE cases. The
historical VK RETURN's scale section points to <wt>/scratch/i17/b_models;
KF3 RETURN binds these files to VR/cases/large_models.sha256. Follow those
specific references first; use bounded filename discovery, not a host scan.

For each existing file, record actual bytes/SHA256, expected manifest identity,
and actual model population/string lengths needed by the current input
descriptor. Distinguish raw-file hash from any canonical model or K4SRC hash:
inspect the named producer/consumer hash rule before claiming equality. Use
exact integer/string parsing and existing source-defined verification only;
do not invent a replacement serializer or infer missing bytes from a digest.
Name absent/mismatching inputs or an unavailable canonical binding explicitly.

No Rust/build/test/solver/model generation, large numeric solve, new helper
tool/framework, probe, network/install, old-file mutation, copying/pruning of
raw archives or expectation change. Read existing files only. Write a concise
additive INPUTS.json/RETURN/provenance/seal under K6C R/I23/external_inputs_06
and owned scratch if needed. Paths in canonical evidence are portable; original
physical paths remain recoverable through <wt>. Preserve all prior seals.

Return by the actual 15-minute boundary, even with concrete missing inputs.
This is file/descriptor binding only, not an E_max proof or new numerical
measurement. No Git/index operations or Type2 delegation. ROOT owns integration;
I21 may consume the sealed result through the manager.
