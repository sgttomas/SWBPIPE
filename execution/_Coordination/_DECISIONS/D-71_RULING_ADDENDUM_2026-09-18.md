# D-71 — ruling addendum: item 7, the Checked mark

Status: RULED — explicit in-session owner direction, 2026-09-18. With this addendum all nine items of [D-71](D-71_interface_program_governed_text_decisions.md) are ruled. Its precedent and form are the `D-66_RULING_ADDENDUM_2026-09-08.md` record beside it and the [D-71 ruling record](D-71_RULING_2026-09-18.md), which this addendum completes and does not change.

## Owner act

The first ruling left item 7 open at the owner's request. ROOT put option B's implications to the owner in session (summarised in the ruling record, "Item 7"). The owner answered, stored transcript timestamp 2026-09-18T13:55:47.429Z, 70 bytes, SHA-256 of the extracted bytes `5cf6071cf15d7ccf43b91197e7a712019a2bb15c414ba6f777d65bfc48193bc8`; an in-session extraction from the host's stored transcript, not original transport bytes:

> D-71, item 7: A.
> D-72, item 5: S-1, S-2 but not S-3.
> I accept SCA-010.

## Adopted bounded effect

**Item 7 — option A, a classification.** The Checked mark is a PRD §16.3 tag: human-authored, per row (set on one row or many), recording who and when and a hash of the row's content; shown stale when the content changes; set and cleared by the engineer only; carried in the project's interface state and not in the model payload; never emitted in any status; never written to an export or a run record; never rendered in a report's fixed sections or its review/signoff block. It is not the PRD §21.3 human acceptance record: `ENGINEER_ACCEPTED` stays reserved and `PB-TBD-002` is not triggered. Its words: the control "Check"; the mark "Checked"; the tooltip "Checked by name · date time · bound to this row's content · not a software status"; stale "the row changed since · Check again or Clear"; the filter "Unchecked rows". A formal acceptance of a whole analysis on the Review page remains possible later as its own owner act; this ruling neither creates nor forecloses it. Codified as `DEC-104`.

## Unchanged

Everything the ruling record lists as unchanged. No implementation is authorized; the mark's storage is operations-map gap G-08, work for an owner-authorized tranche.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
