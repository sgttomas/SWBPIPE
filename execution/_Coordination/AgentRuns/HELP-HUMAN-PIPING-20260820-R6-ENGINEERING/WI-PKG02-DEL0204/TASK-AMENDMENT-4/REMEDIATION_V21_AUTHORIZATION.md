# V21 remediation write-target amendment

Authorized by: `WI-PKG02-DEL0204` under the original sealed activation write
root `projects/chirality-piping/core/adapters/framework/**`.

The direct provenance validator fix exposed the same hostile unhashable status
membership in `plugin_verification.py::_canonical_provenance`, causing all
composed hostile-status regressions to raise. This is part of the exact V21
finding and the current Amendment 4 node, not a new node or acceptance rule.

Additional allowed write target:

- `{WORKING_ROOT}/core/adapters/framework/plugin_verification.py`

All other objectives, checks, exclusions, and write targets remain unchanged.
Do not mutate caller evidence to avoid the exception; normalize fail-closed
without inventing or clearing provenance.
