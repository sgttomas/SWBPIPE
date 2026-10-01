# Bounded HELPS_HUMANS assignment — K6c metric/base composition

One question, source-only: what is the smallest defensible estimator composition
that preserves H's existing staged requested-heap metric and VR's existing global
requested-heap metric, all existing H/VR fixture domains and the admission rule,
without pretending a measured runtime baseline is a universal constant?

Read Root AGENTS.md, agents/AGENT_HELPS_HUMANS.md, Piping AGENTS, COMMON,
original TASK_BRIEFS/I21_K6C_IMPLEMENTATION.md and the resumed I21 returns/gaps.
No other full role or reusable workflow is needed. I21 source packets are in
K6C at82cc9fa9d5; A1 source is cb13fcf3ea560c0d07ad9ac02dac78e9d2e06e00.
Read selected estimator/allocator/caller sources via read-only Git as needed,
not all prior response records. Respect COMMON/no Git/index/no delegation.

Consider explicit source-derived caller terms and the existing allocator's
per-run current/peak observations at a well-defined boundary. If a composition
such as max(observed prefix peak, observed live base + proved future envelope)
is defensible for the existing metric, specify the precise boundary, owner
subtractions/overlaps, before/after phases, admission timing and replay binding.
If it is not, explain exactly why. The observation may be a declared input to a
per-run bound; it must not silently become a universal constant or exclude old
prefix/postprocessing phases. Distinguish requested bytes, moved-reallocation
accounting, allocator overhead and RSS. No change of reported metric/domain.

Use I21's existing source owner/capacity work. Do not rederive all math, audit all
stdlib, design a general memory-observer framework, or build/install a host tool.
No Rust, probe, allocator change, solver run, maintained source or contract edit.
Return one compact proposal/tradeoff and exact remaining bindings within30minutes;
state whether any option changes an accepted contract and needs owner disposition.
An unsupported or inconclusive option is a legitimate return. This does not
accept E_max or authorize implementation/measurements. A1 numerical closure
remains the first priority; final K6c measurements still wait for settled A1.

Write only <K6C_WT>/<R>/metric_design_01 and owned <wt>/scratch/k6c-metric-design.
Record actual origins/hashes and reasoning sufficient for independent checking,
without duplicating prior sealed packets. Report to ROOT; no delegation.
