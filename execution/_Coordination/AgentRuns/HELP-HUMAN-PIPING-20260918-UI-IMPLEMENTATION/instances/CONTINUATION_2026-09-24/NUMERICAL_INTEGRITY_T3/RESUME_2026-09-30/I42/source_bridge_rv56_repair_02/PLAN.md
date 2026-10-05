# I42 / RV56 — radius-check failure-prefix repair plan

**Plan only; source and runtime held pending ROOT grant.** Fresh assignment to
the existing TASK `/root/i42_source_bridge`, directly under ROOT HELP_HUMAN.
ROOT identifies clean frozen candidate `4afbac6e613203f93a499b780082b28be37f6c2c`.
First clock witness for this assignment: 2026-10-03 01:04:44 UTC. No Git read,
source mutation, compiler, solver, test or other diagnostic runtime was performed.
The sealed `source_bridge_01` packet is unchanged.

## Diagnosis

RV56's reported reproducer raises the first relative row's radius above its
computed ceiling. ROOT independently confirms that route. Source reading agrees:
`publication_radius_checked` calls `sharper_binary64`, which executes all five
separately rounded operations before its output checks. The later radius test
returns `RadiusClassMismatch`. The view loop propagates that error with `?`
before its subsequent class-based five-operation increment. Earliest accounting
divergence: successful scalar computation followed by rejected radius, recorded
as zero work instead of five. This is confirmed by source trace, not a new run.

## Smallest proposed seam

1. Give the private sharper-allowance helper a spent-return projection containing
   its original `Result<f64, CertificateIssue>` and an exact bounded u8 operation
   count. Keep the legacy `sharper_binary64` signature as the numerical projection
   of that one execution. Preserve input checks, the five statements, their order,
   and every output/error check byte-for-byte in substance. Count is zero before
   input validation and becomes five immediately after the fifth operation,
   before any later fallible validation. Do not infer count from the error kind,
   precharge work or replay the helper.
2. Give the private checked-radius helper the corresponding spent-return seam
   (`Result<Option<SiRadius>, CertificateIssue>` plus u8 count). A local result
   closure preserves the existing validation order while the outer owner retains
   the producing helper's count on every `?` exit. Keep the current checked-radius
   signature as a legacy numerical projection. Earlier identity/class/absence/
   shape/input refusals retain zero; a post-computation excessive radius retains
   five; successful relative validation retains five.
3. In `build_source_bridge_view`, consume that actual count exactly once into
   checked `work.f64_operations` before propagating the radius result. Remove the
   later class-based increment. Preserve an original radius error if accounting
   status is also non-exact; after a successful radius result, non-exact joined
   work must prevent successful view extraction. No numerical value, criterion,
   public reason, policy, solver schedule or bridge theorem changes.

Use small private fixed return records or equally explicit tuples, confined to
`adaptive.rs`; no generic arithmetic service or public export is needed. The
count is bounded to 0 or 5 and enters WorkTotal only at collection. Existing
non-accounting callers discard the count without a second execution.

## Regression evidence after a grant

- Reuse the existing small loaded native fixture and mutate only the first
  relative row radius to the next representable value above its actual ceiling.
  Require `RadiusClassMismatch`, five actual operations in the view/bridge
  refusal prefix, and unchanged ordinary publication.
- Before-computation identity/shape/class/absence and invalid scalar input
  controls must retain zero. A successful relative row must retain five; full
  successful view counts remain unchanged. Check legacy numerical/error parity.
- Seed checked aggregation overflow for success and simultaneous radius refusal;
  require non-E to block success while preserving the original radius cause and
  the actual prefix. Do not claim a seeded accounting fault is naturally reached.
- Run only the new bounded controls, existing source-bridge controls, affected
  legacy private-radius/sharper checks and S11 inventory as ROOT grants. Freeze
  hashes before credited debug/optimized runs and preserve any failed attempts.

Expected source fence: `retained/adaptive.rs` and the existing
`tests/retained_k4/source_bridge_tests.rs`; S11 only if a new scanned disposition
is actually necessary. Record changes and raw evidence only in this new packet.
No other numerical/storage refactor is proposed. RV56 must finish its frozen
candidate review and ROOT must explicitly grant source writes and the runtime
lane before implementation begins.

## Instruction origin and limits

Selected skill: `/Users/ryan/.codex/worktrees/92ea/chirality/.agents/skills/software-defect-diagnosis/SKILL.md`.
Its instruction “Diagnosis does not imply repair authority. Do not edit code
unless the request separately authorizes implementation” agrees with ROOT's
explicit current hold. Root/TASK/Piping context remains the prior supplied
instruction basis; no additional role or workflow was loaded. This plan has no
runtime reproduction result of its own and grants no permission to proceed.

Read-source SHA-256 observations (no Git operation):

- `adaptive.rs`: `cbd78d3e79fe11a5ae3cdd5422d641850980b88aa0dbb7fa318d6a5e7ce8d8bb`
- `source_bridge_tests.rs`: `12bb7ec09844892532de15dd6a051d0aa5c87ceb031f9d6f093e999720681313`
- selected diagnosis skill: `7e423dfd24132c33d3aa8fe6994bf17a1def966c396723f6bd72ac8d2442112b`
