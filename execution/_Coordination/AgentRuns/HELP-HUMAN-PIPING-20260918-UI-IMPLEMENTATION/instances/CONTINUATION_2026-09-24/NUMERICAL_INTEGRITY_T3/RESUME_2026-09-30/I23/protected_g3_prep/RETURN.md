# I23 G3 source preparation return

Source preparation complete within the15-minute ceiling. Start
**2026-10-01 13:43:20 UTC**; return boundary **2026-10-01T13:45:48Z**
(148.1 seconds). Runtime ownership is still pending ROOT's explicit transfer;
G1 remains the sole A1 runtime.

Prepared exactly **R01–R52** from immutable40129 and the supplied52-registration
ROOT manifest (SHA256 5194a1b926f01b1bc4372ec6771a76a75e2d80fbeb000a3b93855268113837f6).
There are51 FK-only copies (116 files each) and R50/K4-M31 with full P/core
(560 files). No production fault, test overlay or source change was applied.

Final complete checks: **6,476/6,476 files** match original source hashes/modes
and exact file sets; all regular-file inodes are distinct and nlink1; no links
to maintained files or symlink ancestors. All52 targets and52 log directories
are empty. Logs are siblings under `<wt>/scratch/i22/protected_g3/logs`, never
inside RNN source roots. Previous copies and archives were preserved.

Exact resolved host paths are bound by HOST_PATHS.json in owned scratch and
its hash in CONTAINMENT.json. SOURCE_MANIFESTS.json covers every original
file; SOURCE_BINDINGS.json binds source archive, locks/manifests, test/corpus
hashes and unapplied patches. COPIES.json preserves53 exact reviewed filter
bindings and assertions. ROOT_MANIFEST.json is a byte-identical copy of the
supplied manifest. BASELINE_LINKAGE.json preserves its prior full-FK linkage
as metadata only, pending RV29/ROOT qualification—not new runtime credit.

No source-preparation gaps remain. No Rust/build/test/solver, baseline rerun,
fault application, overlay, maintained edit, Git/index mutation or delegation
occurred. One brief-permitted read-only Git archive exported immutable40129
P/core; its FK subset matched the earlier hash-bound40129 FK archive exactly.
No claim beyond source preparation is made. RUNTIME_BOUNDARY.json contains
only future command/stop proposals; ROOT must transfer and schedule execution.
