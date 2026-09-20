# Fidelity supplemental graph recovery

The fidelity reviewer consulted ROOT's changing working graph as supplemental
scope context, after launch. Its already-committed supplemental-inputs.json
recorded SHA2563b61958cb02f6c5549763e3b94bb868e241a39fd12ab658bc03047948babb7eb,
but those intermediate bytes were not separately retained at that time.

ROOT recovered the exact bytes by replaying the actual deterministic graph
updates retained in the active tool conversation against WORK_GRAPH.json at
7d5603ecee1ee941fc696dd1f2d58f6d0de4e3e9. The resulting hash exactly matches the
pre-existing independent input record. fidelity-recovered-WORK_GRAPH.json holds
those bytes; fidelity-graph-recovery.patch records the complete transformation
from the Git basis for independent reconstruction.

This is a post-review recovery, not a snapshot sealed before launch. Original
fidelity inputs, returns and manifests are unchanged. The recovered graph is
historical supplied context, not the current graph or a new authority source.
Future backcheck context uses the frozen candidate graph or retains exact
supplemental bytes before relying on mutable context.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
