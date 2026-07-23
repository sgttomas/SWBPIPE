# Candidate amendment v5

Detection layer: fresh independent pre-effect refutation N3E.  
Disposition: `BLOCK` v4; no product effect released.

V4's path-based writer could not support an absolute containment claim against
concurrent malicious filesystem swaps. V5 does not expand implementation or
dependency scope. It narrows the claim:

- static symlink parent/destination preflight remains fail-closed;
- temp creation remains same-parent and `create_new`;
- the threat model is an ordinary non-adversarial local user;
- race-free directory-handle/no-follow containment is not claimed;
- the TOCTOU residual is mandatory in UI/receipt/tests/run record/handoff as
  `path_containment=best_effort_non_adversarial`.

