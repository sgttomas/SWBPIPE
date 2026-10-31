# SWBPIPE v1.0 release limitations

**Status:** companion to `docs/RELEASE_TARGET_v1.0.md`, for exit criterion 3:
"Every remaining refusal in a Must row is listed in the release limitations and
accepted by the owner." This is the single list. Each entry is drafted by the
lane that owns the refusal and links to the deliverable `Work/` text that
explains it. An entry records a refusal and its reason. It does not accept
it: acceptance is the owner's act (F-PIP-3, release target §5), recorded in
the entry's status only when the owner gives it. The release notes'
"Known Limitations And Open Risks" section (`docs/RELEASE_NOTES_TEMPLATE.md`)
is written from the accepted entries.

## Entry form

Each entry is one subsection:

- **Id:** `LIM-nnn`, assigned in order and never reused.
- **Rows:** the `RT-` rows it affects (Must rows; Should rows may be listed
  for completeness).
- **Code:** the refusal or warning code the product emits, if any.
- **What is refused:** in the user's terms.
- **Why:** the engineering or source reason, in one or two sentences.
- **What the user does instead:** the supported alternative, if any.
- **Basis:** the deliverable and `Work/` section that governs it.
- **Status:** `owner's acceptance pending` until the owner rules; then the
  owner's words and date. A refusal later removed by a landed row is marked
  `withdrawn` with the PR, not deleted.

Entries are added in the same PR that makes the refusal standing. The PR names
the entry, and the entry names the PR.

## Entries

### LIM-001: a compensated terminal at a pressure junction

- **Rows:** RT-A3, RT-K2, RT-L1.
- **Code:** `EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED` (blocking).
- **What is refused:** a pressure junction, meaning a node where two or more pressure regions of one load case end, where any of those regions declares its terminal at the node `separately_supported_or_compensated`.
- **Why:** the junction's pressure load is the sum of its terminals' caps. A compensated terminal would remove its member's share from the pipe solve, so the fitting's load would be analysed incomplete without notice.
- **What the user does instead:** declare `transfers_to_wall` at every terminal of the node. Where a branch's thrust really is carried by a separate device, model that device as a connector or support. A compensated terminal at a region's own closure, away from any junction, is admitted.
- **Basis:** DEL-04-07 [`Work/pressure-contract.md`](../execution/PKG-04/DEL-04-07/Work/pressure-contract.md), "Tees and pressure junctions (RT-A3 increment 1)"; RT-A3 increment 1 (PR #<tee-PR>).
- **Status:** owner's acceptance pending.
