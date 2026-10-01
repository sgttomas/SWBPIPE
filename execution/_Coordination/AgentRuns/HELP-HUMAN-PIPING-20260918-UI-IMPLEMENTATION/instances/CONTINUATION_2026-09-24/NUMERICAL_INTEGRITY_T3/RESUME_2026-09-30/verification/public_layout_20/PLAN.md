# Public type fact diagnostic20

Prepared exactly ten public type expressions requested by the H/VR numerical
assembly. Source uses actual public Row and Matrix12 types from the already
reviewed libraries; standard types are named directly. Only type_name/sizeof/
alignof and stdout are executed. No input/model/solver/allocator experiment
is called, no private struct is mirrored, and no maintained file changes.

Before runtime, an independent source/plan review checks exact expressions,
source/artifact binding and permitted invocation. ROOT owns the single compile
and run in a fresh scratch directory under the existing guard. Final output
and raw bindings receive the same reviewer backcheck. No fact is claimed yet.
