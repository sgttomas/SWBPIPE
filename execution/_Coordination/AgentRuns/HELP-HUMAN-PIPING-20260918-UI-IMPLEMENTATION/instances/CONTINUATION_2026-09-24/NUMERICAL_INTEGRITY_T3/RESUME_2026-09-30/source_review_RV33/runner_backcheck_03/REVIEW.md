# RV33 runner hold-order backcheck

**RV33-H1 is closed on exact candidate
`10315a8167c47f43aa41402beb88ed2c70e62cf2`.** No new actionable finding was
identified in this H-owned backcheck, including the shared eligibility helper's
VR consequences. The four-file repair is suitable for fan-in within this scope;
RV32 retains its separately assigned VR review.

Base is `9086964a1fb656a76cda6d1002d8594efa636fdc`. H means
`projects/chirality-piping/core/solver/performance_harness`; V means
`projects/chirality-piping/validation/benchmarks/numerical_robustness`.
Only H/runner/k6_runner.py, its test, V/runner/vk_scale_runner.py and its test
changed in core/validation. Rust is unchanged, so no Cargo build was performed.
The complete four-file diff, exact postimages and frozen Python dependencies are
preserved under `_run_records`.

## Repair and consequences

The pure H eligibility helper (`H/runner/k6_runner.py:496–516`) evaluates named
refusals, the caller's conditional reason, and recorded ascent. It reads no
numeric counts and returns permission to bind rather than numeric admission.
`run_tier` now checks that result before `bind_w1_launch_counts`, numeric admission
or normal launch. Its prior already-recorded skip and tier no-op baseline remain.
The original CHAIN100/no-CHAIN10 trigger now records a deferred ascent row with no
model-specific launch, no numeric admission call, and no freshly bound estimate.

Prior error, timeout and heap-abort attempts still satisfy recorded ascent;
None/not_run do not. The existing family filter, same-model/mode identity and
require_ascent behavior remain. Conditional/named holds also avoid both model
prepasses and normal launches. The shared VR wrapper preserves its first100-member
exception and10000 approval condition before binding. These guards use actual
records and policy fields; no fake estimate/count object is needed by the guard.

Direct admission remains a separate pure interface with legacy priority:
H checks named refusal, missing counts/estimate, then ascent with the populated
estimate; VR checks missing counts, binary backstop, approval, then H admission.
The factored H result dictionary retains its keys and values. Plan uses those
same direct paths and remains process-free. Preflight held-run records intentionally
lack a newly bound estimate, as ROOT specified; this is the reviewed difference
from legacy run_tier diagnostics, not an accidental loss of pure admission data.

After eligibility passes, the existing binding still precedes all numeric
predicates and supplies the admission/recorded ratio denominator. Binding failures
retain their exception behavior and have no stale fallback. H's counts prepass
keeps the normal launch's heap cap; VR's counts cap remains512MiB and its normal
cap is unchanged. Arguments, resource/time limits, no-op handling and numeric
thresholds were not modified. The added permitted-run tests check
bind → numeric admission → normal launch, matching model/context, caps and the
recorded denominator.

## Independent verification

The I24 seal
`49bf3e38ff68144cc49fa918a84fa5dfdcfe0eab2c021b4d42a197f5ad181ab6`
and all20 payloads verify. Each of the four archived candidate postimage hashes
matches the author. All38 frozen dependency/observation files remain unchanged.

All executions used the requested existing project virtual-environment Python;
its exact path/version, arguments, environment overrides and raw outputs are in
`CHECK_COMMANDS.json` and the logs. The author's use of system Python was
separately disclosed; it is not relied on as the independent interpreter witness.

- **Prior H1 process-free reproduction passed.** It uses the same CHAIN100 fixture,
  archived repaired module and mocked launch/baseline functions, with assertions
  changed to the repaired outcome. The adaptation diff is preserved. There are
  zero model-specific launches; the row is `not_run` for ascent and its estimate
  is None.
- **54 H and15 VR runner tests passed** under the requested interpreter. Held-run
  tests verify the permitted tier baseline and zero model-specific processes;
  positive controls retain failed/timeout predecessor semantics and rebound
  denominators/caps. H's ordinary live-limit tests only use their existing small
  Python helpers, not K6/VR model runs.
- **13,968 H and1,512 VR independent pure admission comparisons matched** the
  frozen908 implementations, with zero dictionary differences. Cases cover every
  scheduled row, absent/missing/small/half-cap/over-cap estimates, ascent/approval
  settings, and absent/None/not_run/ok/error/timeout/abort predecessors.
- **Four plan comparisons matched**, covering H absent/committed counts and VR
  both approval settings. Existing Python schedule metadata helpers were used;
  no product count/solver, model-specific subprocess, or performance run occurred.

No remaining blocker is identified for this repair. This closes the confirmed
ordering defect; it does not reopen or claim acceptance of unrelated H/VR math,
Rust execution qualification, full E_max, chronological admission/recalibration,
measurements, engineering outcome or release. Earlier sealed packets and source
archives were left untouched. No maintained/index edits, delegation, new tools,
heavy work or automatic continuation occurred. All Git reads set
GIT_OPTIONAL_LOCKS=0.

`_run_records/SHA256SUMS` seals this packet; its paths resolve from this packet root.
