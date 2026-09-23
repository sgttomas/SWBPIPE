# Receiving-document patch preparation

`PROPOSED_RECEIVING_AMENDMENTS.patch` is an **unapplied** unified diff for the
non-issued receiving documents named in `SEMANTIC_DECISION_PACKET.md`. Its
13 preimage files and SHA-256 values are in `PATCH_TARGETS.csv`. The first
draft was built at `10e9a0d21507484636392c14f07a1602414458ab`; this
version was regenerated after rebasing over PRs #859/#860 and after the
DEL-05-02 completion backcheck, with the separately directed DEL-17-06
historical banners already present. `git apply --check` passes on the current
target preimages. The patch does not
touch any `_STATUS.md`, register, issued deliverable or product code.

The human has approved conditional group treatment and selected historical
status for the DEL-17-06 semantic derivatives; `OWNER_DIRECTION.md` binds
those replies. Authorization for the listed non-issued receiving-document
amendments is deferred at `TM-PIP-043`. Do not apply this patch merely because it parses
or because a source row has a proposed destination. Verify the exact owner
decision, source bytes, form constraints and each surviving row meaning first.
After any approved amendment, recheck the destination before removing its
source bullet. Eighteen independently verified empty `NONE` headings and two
DEL-05-02 bounded/fulfilled bullets (four census keys) were removed under
the separate conditional group approval; 219 account rows remain held. The
earlier proposed DEL-09-04 DEC-092 regeneration hunk was withdrawn after
checking the actual 2026-08-09 completion evidence in
`DEL0502_DEC092_COMPLETION_BACKCHECK.md`. This patch does not apply or
justify any additional removal.

The separate `DEL1706_HISTORICAL_MARKING.patch` records the applied minimal
two-file historical marking chosen by the owner. It prepends a banner to
`_SEMANTIC.md` and `_SEMANTIC_LENSING.md`; their complete dated bodies remain
byte-identical and in order. This does not regenerate either artifact or
remove the compound `DEL-17-06:3` status bullet. The proposed Scope of Work
current-reference note and fixture-specific verification clause remain in
the unapplied receiving-document patch.

Some proposals cannot yet be rendered as an executable final change:

- **ISSUED DEL-01-01:** D-74's MIT consequence needs its own formal-change
  act, exact target revision and issuance procedure. The line-level comparison
  is in `ISSUED_DEL0101_FORMAL_CHANGE_INVENTORY.md`; no issued-file hunk is in
  either patch.
- **DEL-05-03 connector treatment:** the present straight-pipe stress
  recovery contract does not identify the connector owner. Retain this
  compound source bullet until an owning decomposition/scope decision names
  its receiving contract; no guessed connector amendment is drafted.
- **DEL-17-06 dependencies:** the pending patch only corrects the current
  DAG-011 navigation assertion while labeling dated row counts. A complete
  source-faithful `_DEPENDENCIES.md` refresh requires the owning dependency
  workflow to compare `Dependencies.csv` and approved DAG-011. Do not treat
  the two-line navigation correction as that workflow's completed result.
- **Other ordinary-scope candidates:** the decision packet and row account
  deliberately leave every unverified destination held. This patch covers
  the specifically listed receiving amendments; it is not a blanket transfer
  of the remaining census.

The historical 2026-05-18 `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` are now
marked for no current use per the owner's choice, so their alternative
regeneration proposal is superseded and no regeneration patch is prepared.
