# Owner decision — intensified stress from user-supplied SIFs (2026-09-26)

HELP_HUMAN (ROOT) asked the owner a question raised by the T0R design (`DEFAULT_ROUTE_DESIGN/DESIGN.md`). Should the product ever compute an intensified stress from user-supplied stress-intensification factors, for example `hypot(ii·Mi, io·Mo)/Z`, or leave all intensification to user rule packs? The owner replied:

> Regarding SIFs you can use derivative formulas incorporating the users input.

## ROOT application

- The product may publish intensified stress quantities derived by formula from the user's own SIF input (ii, io and any other user-supplied factors). Each quantity must name the formula it uses and the inputs it consumed.
- The factors themselves stay user-supplied. Agents do not populate SIF values, flexibility factors or component libraries, and the product supplies no default SIF where the user gives none.
- Code rules stay user-supplied: allowables, stress categories and code-specific acceptance. A derived intensified stress is a physical-plus-user-factor quantity, not a code-compliance verdict.
- Intensified quantities must not pass through linear combination algebra where that is invalid. Ranges and combinations of intensified stress follow the owner's rule once T6 defines them. The M08 finding that SIF×k rows enter combinations remains a defect to remove.
- This settles the open question before T4 designs M08. T0R may use it where its design can do so correctly on the ordinary route. It changes no other scope, hold or protected criterion.
