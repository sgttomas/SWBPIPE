# PLAN AMENDMENT 001 — Exact DEC-025 Sweep Output

- RunID: `HELP-HUMAN-PIPING-20260719-SYSTEMATIC-PORTABILITY-REPAIR-R10`
- Plan version: `2`
- Disposition: `AMEND`
- Consequentiality: `NON_CONSEQUENTIAL_FENCE_CORRECTION`
- Applies to: `TASK-SYSTEMATIC-PORTABILITY-AUTHOR-R10-01`
- Authority: HELP_HUMAN manager disposition implementing the explicit owner-approved requirement for one five-surface sweep

## Basis

The frozen plan requires one DEC-025 five-surface sweep. The original author
brief accidentally omitted the deterministic sweep output directory from its
write fence. The author correctly refused the out-of-fence write and preserved
that refusal as run evidence.

## Amendment

Add exactly this one output class to `AllowedWriteTargets`:

- exactly one newly generated R10 sweep JSON at `projects/chirality-piping/validation/evidence/sweeps/SWEEP_*.json`

No other validation/evidence path is authorized. The child must acknowledge
this amendment, execute the sweep exactly once, validate the resulting JSON
and five-surface result, and terminalize. All other brief terms remain exact.
