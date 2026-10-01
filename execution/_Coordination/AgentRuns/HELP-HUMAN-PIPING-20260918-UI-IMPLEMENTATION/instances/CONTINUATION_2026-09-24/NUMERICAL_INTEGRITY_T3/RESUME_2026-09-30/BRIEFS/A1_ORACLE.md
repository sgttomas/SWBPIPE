# Independent A1 exact oracle — TASK, reports directly to ROOT

Read agents/AGENT_TASK.md and COMMON.md. You did not author the audit, probe or
response. Fresh context is deliberate. No delegation. Source base is
d01ad98a754698631f927709d08284c272de85e8, unchanged numerically from #1064.

Writes: R/oracle/** in A1 worktree, and owned <wt>/scratch/a1-oracle/** only.
No Git/index mutations, Rust work, solver edits, accepted fixture changes or tools.
Read audit finding/math and response scope handoff, then source-model definitions
from the response probe/matrix. Do not import its comparator or copy its expected
answers. Independently derive expected mathematical values from the synthetic
primitive source and decide binary64 intervals/classes/range with exact rationals.

Initial output CHECKPOINT_0.md plus a small independent Python oracle if feasible:
state equation/input interpretation, scope coverage, exact rounding method,
published claim predicate, and a machine-readable input/output interface.
Ground expectations before inspecting the old B01/B02 truth fields. Old results
may be checked afterwards, explicitly recording that order. Standard-library
Python work is authorized; no compilation or solver experiment.

Check the audit arithmetic independently, including L and 1/L amplification,
zero published scale, threshold crossings, O9 row membership and direct rounding.
Do not substitute R1's looser benchmark criterion for the row's published claim.
Give I22 concrete interface requirements through ROOT. No full design acceptance
is requested yet; identify what source reachability and design proof still need.
