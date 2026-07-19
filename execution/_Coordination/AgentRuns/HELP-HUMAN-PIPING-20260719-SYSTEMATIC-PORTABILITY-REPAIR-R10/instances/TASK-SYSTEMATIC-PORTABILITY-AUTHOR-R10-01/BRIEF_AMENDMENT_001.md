# TASK Brief Amendment 001 — Exact DEC-025 Sweep Output

- InstanceID: `TASK-SYSTEMATIC-PORTABILITY-AUTHOR-R10-01`
- Parent amendment: `PLAN_AMENDMENT_001.md`
- Disposition: `AMEND`
- Existing brief terms: unchanged except for the exact addition below

Add to `AllowedWriteTargets`:

- exactly one newly generated R10 sweep JSON at `projects/chirality-piping/validation/evidence/sweeps/SWEEP_*.json`

No other validation/evidence path is authorized. Acknowledge this amendment in
the terminal return, execute the sweep exactly once, and validate its JSON and
five-surface result.
