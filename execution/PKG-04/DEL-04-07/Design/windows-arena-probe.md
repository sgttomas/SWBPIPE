# Windows arena investigation

Maturity: experimental, incomplete; hosted native execution and independent
review pending. This standalone fixture exercises documented `VirtualAlloc`
reserve/commit and `VirtualFree` decommit/release, explicit admission before
new pages, fixed in-arena metadata, retained pages, alignment, zeroing, moving
reallocation and injected errors. Its actual child fixture uses two native
processes in a creation-time Job with restricted inherited handles, and keeps
its reservation through cancellation until observed exit. It is a controlled
arena API, not a production allocator or a connection to the solver or host.
A reserve-only address range is not a full-budget lease. Results report scripted
costs, not qualified performance. Real OS commit failure and production
availability profiles are not claimed.

The ledger holds future liability separately from actual committed bytes and
uses zero residency credit; physical and commit headroom are independent inputs.
A child retries an injected headroom refusal within one absolute 2-second
window and follows an injected post-admission commit failure with a realloc
retry under that same window. A separate case terminates a child blocked on a
grow grant and holds the liability through observed exit. These cases still
require hosted Windows execution and independent scrutiny; they do not exercise
a real operating-system commitment failure or prove every cancellation race.
Main/global allocator and native startup costs still require a separate audit; this probe proves no production source
path or complete worker bound.

Pending solver work at `9b73234e50f5ac151ad9f8a80c4c961c5a488170` adds a
provisional pre-case nonlinear dense guard for both modes. Its checked estimate
is 88·n_global² + 16·n_free² with a fixed floor(0.9 × 12 GiB) = 11,596,411,699
bytes ceiling, about 1,759 all-free nodes; overflow refuses. It is a component
estimate, not a whole-worker bound, and does not scale with a larger desktop
budget. It is pending and is not adopted or imported here; reconcile when it
lands.

The DEL-04-07 ScopeOfWork and dependency conditions are unchanged by this finite
investigation. Windows retained registration remains **Stale** and R64 MiB is
**unqualified for Windows**. DEL-07-11 and DEL-10-04 obligations are not
discharged. The existing budget, wait, headroom, confirmed-exit and result
custody semantics and numerical-mode behavior are not changed.
