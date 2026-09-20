Public signals distinguished tested commits from cancellations:

- Same-value Return and pointer commits emitted action notifications.
- Escape and outside-click cancellations did not.
- End-tracking preceded commit actions.

Production bridging remains unproven; the original Escape-keydown failure did not reproduce. Evidence sealed in `public-menu-signal-probe/RETURN.md`, SHA256 `7d2cfac5e2810d22bfd37e9ab82ae3acf65f63f1dae92338b34f081f8f641943`.

PID19891 stopped and verified absent. UI slot released.
