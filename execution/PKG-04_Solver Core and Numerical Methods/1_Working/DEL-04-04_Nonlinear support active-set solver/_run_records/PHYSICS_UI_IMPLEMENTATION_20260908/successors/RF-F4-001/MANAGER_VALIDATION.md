# RF-F4-001 successor manager validation

Status: `PASS_PENDING_RF_BACKCHECK`

WORKING_ITEMS reviewed the complete V1-to-successor diff. Every changed line is inside the primary `#[cfg(test)] mod tests` block. The production and pre-primary-test region remains SHA-256 `311b4d6657c103982c78d8e86d4efa7dc5d450556802dc3a45d5ce6264bf42ae`; the later computed-finite test module through EOF remains SHA-256 `73463334658edb9ecf09dd48b4b733410e89d1fe95219fb8f485ff3ff7484ea8`.

The successor directly exercises the production branch-selection and affine-solve path. For synthetic prior source reactions `+0.0` and `-0.0`, the affine base normal is exact zero, active direction and coefficient are nonzero, the first row applies zero force, the simultaneously solved second row applies `-1100/41`, and cross-coupling produces current normal `+11/41`. The solver reports the branch inadmissible. The active-set state residual itself is converged and unblocked, while the existing caller expression rejects convergence because branch admissibility is false. The mirrored negative fixture proves direction `-1`, prior source reaction `-1`, zero applied force, exact-zero final current normal, and admissibility in both solve modes.

Manager verification:

- V1-to-successor patch reverse applicability: exit 0;
- base-to-successor patch byte-identical to live diff: exit 0;
- offline locked full nonlinear-integration crate: 29 passed, 0 failed;
- formatting check: exit 0;
- diff and portability checks: pass;
- manager private target cleanup and absence: pass.

The child preserved its initial formatting-only failure and corrected it with the formatter. No production behavior, threshold, algorithm, public type, diagnostic, acceptance criterion, dependency, or frozen V1 artifact changed. The candidate is frozen for RF backcheck and makes no acceptance or release claim.
