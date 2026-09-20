# Prospective worker model update

ROOT relayed the owner's new direction during B3 continuation:

> Replace all Sol workers with Astra models on low reasoning when deploying new instances.  Update the delegation instructions accordingly.

Effective for all new deployments formerly allocated to Sol: use `gpt-6-astra`
with `low` reasoning, including replacement and later shell/canvas/native workers.
This supersedes the high-effort availability fallback for new worker instances.
Manager Astra/high and independent review allocations remain unchanged.

Existing instances may complete their current bounded assignments with their
actual settings preserved: the native worker remains GPT-5.6 Sol/high; the active
shell continuation remains GPT-6 Astra/high. They are not silently relabelled.
No new bounded assignment will be sent to an old Sol instance.

ROOT retains the exact owner record, plan amendment 2 and sealed
`B3_MANAGER_ADDENDUM_2.md` under
`instances/ROOT/CONTINUATION_2026-09-19_CODEX/`, expected SHA256
`f3363e5a2b365af8d0a95cfc1893d80f29ee21dd6aa8aaabd40a63ddb9147ae2`.
The current manager's sealed briefs remain historical; any new brief uses this
prospective allocation. Product scope and gates are unchanged.

The sealed addendum was read from commit `46f052d99` via `git show`; its contents match this prospective allocation.
