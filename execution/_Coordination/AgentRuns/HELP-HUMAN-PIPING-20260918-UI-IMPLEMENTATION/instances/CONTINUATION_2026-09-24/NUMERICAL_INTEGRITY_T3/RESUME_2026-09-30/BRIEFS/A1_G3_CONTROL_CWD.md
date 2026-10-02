# G3 returned-control working directories

ROOT binds BOTH existing returned-control binaries to the preserved immutable
E1 FK cwd:
<wt>/scratch/i22/mutations_e1/base/projects/chirality-piping/core/solver/frame_kernel.
The library binary remains cd5967f30c1ca9d8957d02126d2639e943435b84f86c5d02c0b21c904af8e00a;
the S11 binary remains ffed9c0f948b771acfd3de71242f38a41dda478839afe7d54de2020f21f9165a.
Their original paths and reviewed build basis remain in the G3 manifest.

ROOT checked the manifest:51 library targets and one S11 integration target.
The FK source/library-test search found no runtime relative-file read on these
selected paths. The two optional external-file reads occur only in the separate
unselected k5_constrained_bodies integration target. S11 uses its23 compiled
include_str sources and the compiled table; its test scans those strings and
does not open relative source files at runtime. A different cwd cannot replace
or alter its compiled source inputs.

Manager rechecks the E1 copy's complete116 original source/test/lock identities,
each bound binary, and RV29's original full-core/S11 include-input provenance.
Do not claim E1 contains the other core crates; its role here is cwd only.
R50's mutant still compiles separately from its full-core copy with its actual
mutated source, and must reach the named S11 site-policy mismatch.

No new baseline, copy, build, overlay, environment relaxation, criterion or
maintained-file change follows. Preserve the preparation's omitted cwd and
record this additive binding in the G3 run. All deadlines, per-build independent
release, exact filters/controls and semantic stop conditions remain unchanged.
