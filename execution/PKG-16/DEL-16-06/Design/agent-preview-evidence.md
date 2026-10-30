# Agent preview evidence

An agent route consuming preview/browser results must distinguish fixture
fallback from native execution. If native invocation fails, substituted fixture
output is not evidence of authoritative agent-run success; the route must fail
closed rather than report fixture output as native success.

This preserves the existing conditional requirement. It does not assert that
the historical fallback defect is present in current production.

Source: TM-PIP-033 and its owner ruling, plus section E-06 of
`execution/_Coordination/COORDINATION_RESPONSE_2026-08-02_PIPING_RUNTIME_SURFACE_NEEDS.md`,
at repository commit `fa927aac8c0e3be6bfa8f7ce8ec49daa6e112ee4`.
