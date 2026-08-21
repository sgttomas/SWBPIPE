# N2 launch/plan amendment v2 — owner-authorized review remediation

- RequestedBy: owner, relayed by `HELP_HUMAN`
- AppliesTo: `N2_LAUNCH_BRIEF.md`
- Supersedes: the owner stop rule only for the two actionable N2 review findings
- Authorized objective:
  1. prevent viewport and Inspector component forms from queuing the same component target ID;
  2. ensure the component resolver persists canonical trimmed identity/connectivity/reference strings rather than the original whitespace-padded payload.
- Required evidence: directly affected native/Wasm/UI tests and a fresh independent read-only review over 100% of the amended N2 diff.
- Exclusions: no new component kinds; no N4 or Python-environment/policy remediation; no final evidence sweep; no commit/push/PR; no shared closeout-surface write.
