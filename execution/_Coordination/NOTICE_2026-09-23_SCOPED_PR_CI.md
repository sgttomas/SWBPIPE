# Notice — automatic scoped PR checks

Tranche `ROOT-SCOPED-PR-CI-20260923` implements the owner-selected policy:
automatic checks from the complete PR diff, with explicit full coverage on demand.
See Root `docs/CI_SELECTION.md` for the routing table and commands.

App/PEC GitHub jobs reuse the existing affected-check selector with a hosted
profile. Root packaged instruction changes receive App bundle construction and
integrity checks; product and shared Runtime changes retain their full consumer
suites. Project records do not start product suites. Unknown inputs and routing
policy changes select full coverage. Piping retains its source planner while
recognizing Root agent/workflow packages and project AGENTS as non-product inputs.

The governance harness and existing stable check names remain. Manual workflow
dispatch requests full coverage for the selected workflow/candidate. Selection
artifacts retain the base, head, changed paths and reasons. Failed or cancelled
required jobs cannot produce a passing aggregate; not-applicable product checks
state that no product tests ran.

Local project check profiles, accepted native/empirical evidence obligations,
DEC-025 sweeps, review, scope, lifecycle and release boundaries remain governed
by their existing sources. This notice neither repins an authority corpus nor
changes PEC's loop/work selection.
