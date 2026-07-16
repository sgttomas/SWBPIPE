# D-44 — Piping Loop Receipt Contract and Enforcement

**Status:** PROPOSAL — AWAITING_RULING

**Date prepared:** 2026-07-15

**Prepared by:** untyped direct-human session applying HELPS_HUMANS and
`docs/WORKFLOW_COMPONENT_STANDARD.md`

## Decision statement

Select the durable receipt structure and enforcement mechanism for the
chirality-piping development loop. The decision must preserve the loop-level
facts that cannot be reconstructed from governed artifacts or Git while
preventing the receipt surface from becoming a shadow history, evidence, or
status system.

This packet changes no loop instruction, tool, validator, receipt rule, or
historical receipt. It is a candidate governed record under K-AUTH-1 and
F-PIP-5. Implementation remains blocked until the owner rules, the ruling is
SHA-bound, and the ruling record plus register update land on `main`.

## Governing basis and evidence

- Root `AGENTS.md`, Governance Integration Rules and Multi-Agent
  Orchestration: claim status must survive relays; actual orchestration state
  belongs under `execution/_Coordination/AgentRuns/<RunID>/`; handoffs record
  blockers and rerun requirements.
- `docs/CONTRACT.md` §§1.2, 1.9, and 1.10: K-AUTH-1, K-AUTH-2, K-PROV-1,
  K-CLAIM-1, and K-WRITE-2.
- `docs/WORKFLOW_COMPONENT_STANDARD.md` §§3.4, 5, 6, 12, and 15:
  deterministic validation belongs in a tested tool; evidence is not
  approval; compatibility and retirement require explicit contracts.
- `projects/chirality-piping/loop/LOOP_INIT.md` §§2–6 and 8: receipts are
  handoff context rather than authority; the live tree wins; owner gates and
  receipt closeout are mandatory.
- `projects/chirality-piping/loop/WORKPLAN_2026-07-10_piping_loop.md`, protocol
  steps 0, 3, and 5 plus F-PIP-5: Step 0 starts from the newest applicable
  receipt, gate outcomes are recorded, and no new standing surface is created
  without owner ruling.
- `projects/chirality-piping/loop/LOOP_RECEIPTS.md`, Rules 1, 3, 5, and 6:
  receipts are pointers rather than narratives; measurement counts live
  elsewhere; gate outcomes include no-ops; receipts are roughly 6–12 lines.
- Observed drift in that ledger: Receipts 35–40 occupy 28, 29, 26, 21, 15,
  and 15 physical lines respectively and restate exact test counts,
  per-PDU dispositions, and containment measurements already owned by their
  closeout artifacts. Receipts 41–42 returned to 8 and 7 lines. This is
  evidence that the existing prose rules can be followed, but do not
  deterministically prevent drift.
- Cursor evidence: the ledger last changed at commit
  `9b8b6c91966fdd4aab1988ef422176239cc11651` (Receipt 42), while the packet
  branch was opened from
  `4e8ab5b0c20fca4ac8ed50b7e1d3da9072323ec5`. That gap is not itself
  staleness: it is the discovery interval the next loop must examine.

## Requirements

R1. Preserve verbatim chat-only owner directions that have no governed home,
with claim status showing that the entry is a transcription/evidence record
and not itself a ruling.

R2. Preserve gate outcomes, including lawful no-ops and their rationale, so a
later loop can distinguish `not attempted` from `examined and parked`.

R3. Preserve stale-map deltas without rewriting the dated map or duplicating
the live source that disproves it.

R4. Give Step 0 an exact discovery cursor that distinguishes repository state
already examined from later state requiring rediscovery.

R5. Keep implementation history, measurements, deliverable state, decision
state, and orchestration details in their owning artifacts; receipts point to
those artifacts and do not reproduce them.

R6. Make all objectively checkable receipt constraints deterministic, tested,
and part of the mandatory closeout gate. Semantic judgment still decides
whether a statement is genuinely non-reconstructible.

R7. Preserve all historical receipts byte-for-byte and retain their declared
derivative/non-authoritative status.

R8. Require a receipt whenever a loop reaches an owner gate and stops, even if
the session creates no implementation artifact.

R9. Keep claim authority calibrated: validation proves structural conformance,
not the truth or authority of a transcribed direction or observation.

## Common cursor contract

Both implementation options use the following exact cursor fields:

```text
Receipt-ID: Receipt-<positive decimal integer>
Examined-Through: <full 40-character lowercase Git commit SHA>
Parent-Receipt: Receipt-<positive decimal integer>
```

- `Examined-Through` is the commit returned by `git rev-parse
  HEAD^{commit}` at the end of mandatory Step 0 and before the session's first
  mutation. It is an acknowledgment boundary, not a closeout commit.
- `Parent-Receipt` is the receipt actually used as the session's handoff basis.
  It need not be the physically preceding receipt: concurrent sessions may
  lawfully share a parent.
- The first receipt under the enforced contract names the final legacy receipt
  as its parent. Receipt identifiers are unique within this loop.
- A validator must confirm field syntax, referenced-parent existence, commit
  resolution, and that `Examined-Through` is an ancestor of the commit being
  validated. It must not rewrite a cursor after concurrent work lands; the
  resulting Git gap remains work for the next Step 0.

## Common minimal-content contract

Every new receipt contains the cursor fields and one `Gate-Outcome` record.
It may additionally contain only:

- a verbatim owner direction not already held by a governed artifact, labeled
  `CHAT_TRANSCRIPTION — EVIDENCE, NOT RULING`;
- a one-record stale-map delta with a pointer to the disagreeing live source;
- artifact, decision, PR, commit, test-plan, or AgentRuns pointers;
- a pass/fail-only check summary without measurements; and
- model attribution only when required by the loop convention and not already
  recoverable from the pointed-to AgentRuns record.

Status vocabulary is allowed where needed in `Gate-Outcome` or in a cited
delta. Verbatim owner text is preserved even when it contains a pattern that
would otherwise be prohibited. Recoverable implementation narratives,
per-item dispositions, exact test counts, evidence measurements, tables,
nested headings, and duplicated orchestration detail are inadmissible.

The implemented format must have a deterministic size bound. The proposed
default is at most 12 top-level receipt records and 4,096 UTF-8 bytes per
receipt, excluding its heading. The owner may amend those two values in the
ruling without changing the option selected.

## Options

### O-A — Retain the append-only ledger and enforce the minimal contract
(recommended)

Keep `projects/chirality-piping/loop/LOOP_RECEIPTS.md` as the single ordered
handoff log. Freeze the complete pre-contract prefix byte-for-byte and append
only receipts conforming to the common cursor and content contracts.

Implement a deterministic receipt validator with positive and negative tests.
The validator must:

1. verify the pinned SHA-256 of the frozen pre-contract ledger prefix;
2. parse only the versioned receipts following that prefix and refuse an
   ambiguous boundary;
3. enforce unique receipt identities, exact cursor fields, parent existence,
   commit resolution/ancestry, and a mandatory gate outcome;
4. permit concurrent receipts to share a parent without pretending the later
   physical entry examined the earlier sibling's work;
5. enforce the size bound and reject objective duplicate-content signatures,
   including exact test-count summaries outside verbatim owner text;
6. permit status words only in schema-appropriate records rather than banning
   them globally;
7. report structural PASS/FAIL without deciding whether prose is semantically
   non-reconstructible; and
8. operate read-only, fail non-zero on invalid required input or contract
   violations, and make no repository edits.

Register the validator as a deterministic tool and integrate it into the
repo-wide practitioner-harness `self-check` when chirality-piping is in scope,
as well as naming it explicitly in the piping loop closeout sequence. Update
`LOOP_INIT.md`, the standing workplan, and the ledger rules atomically. Do not
alter historical receipt text.

**Tradeoff:** preserves the simplest ordered handoff and avoids a new pointer
surface, but the ledger remains a shared append target. Concurrent branches
may still require ordinary append-only integration.

### O-B — Freeze the ledger and move to immutable per-run receipts

Freeze `LOOP_RECEIPTS.md` as a historical derivative and create a governed
receipt directory containing one immutable receipt per run plus a canonical
`_LATEST.md` pointer. Apply the same common cursor/content contract and
deterministic validation used by O-A.

Concurrency semantics:

1. a receipt filename carries a collision-resistant run identity and is never
   edited after acceptance;
2. multiple receipts may share one parent and preserve their original
   `Examined-Through` values;
3. `_LATEST.md` means the most recently merged canonical handoff, not the
   receipt with the greatest implied knowledge;
4. Step 0 follows `_LATEST.md`, then replays Git from that receipt's
   `Examined-Through` SHA, so concurrent work the receipt did not examine is
   rediscovered rather than silently acknowledged; and
5. concurrent `_LATEST.md` writes are serialized by the integration owner;
   an unresolved pointer conflict blocks that PR rather than rewriting an
   accepted receipt.

If O-A is selected, O-B remains a documented but unselected reserve and gains
no implementation authority. Evidence sufficient to return O-B for a later
owner ruling is either: (a) a structurally valid ledger state for which the
validator cannot identify an unambiguous applicable cursor lineage; or (b)
repeated shared-file conflicts, evidenced in at least two independent PRs,
that cannot be resolved by append-only integration without changing already
validated receipt bytes.

**Tradeoff:** isolates receipt writes and makes accepted receipt immutability
direct, but introduces a new standing directory, mutable pointer, pointer
serialization rule, and additional discovery logic.

### O-C — Retain the current ledger and prose-only rules

Make no protocol or tooling change. Continue relying on the existing fixed
rules and human review.

**Tradeoff:** zero implementation cost, but leaves the already observed
objective drift without a deterministic closeout barrier.

## Evaluation

- The receipt function remains necessary: chat-only direction, attempted-but-
  parked gate outcomes, stale-map deltas, and an acknowledgment cursor are not
  fully reconstructible from Git or deliverable state.
- The recent ledger demonstrates content-rule drift, not a proven failure of
  the append-only architecture. Later concise receipts demonstrate that the
  file can still carry minimal entries.
- Restating the existing rules without tooling would not address the observed
  enforcement gap. Per-run files alone would not prevent oversized or
  duplicative receipts either.
- O-A adds deterministic enforcement while retaining the smaller surface and
  existing Step 0 model. O-B is justified only if a validated monolith later
  produces cursor ambiguity or irreducible shared-write conflicts.

## Recommendation

Adopt O-A with the common cursor and minimal-content contracts above. This is
a non-binding recommendation. It addresses the demonstrated failure—lack of
mechanical enforcement—without replacing a ledger architecture that has not
yet demonstrated an operational failure. Preserve O-B in this packet as the
named reserve; do not treat it as conditionally adopted unless the owner says
so expressly.

## Validation criteria before implementation

Implementation is not valid unless all of the following hold:

1. the owner-selected option is recorded in a SHA-bound ruling, the D-44 row
   is `RULED`, and both have landed on `main` before implementation starts;
2. exact write scope is recorded, including any root `tools/**` and
   practitioner-harness files authorized by the ruling;
3. historical ledger bytes remain unchanged and the frozen-prefix identity is
   tested;
4. positive fixtures cover a minimal ordinary receipt, a gate-stop receipt,
   a chat transcription, and concurrent sibling cursors;
5. negative fixtures cover a missing/invalid SHA, missing/nonexistent parent,
   duplicate ID, missing gate outcome, oversized receipt, duplicated test
   counts, ambiguous parse boundary, and altered historical prefix;
6. the standalone validator and practitioner-harness self-check both fail on
   invalid fixtures and pass on valid fixtures;
7. the loop instructions, tool registry, validator behavior, and tests agree
   on one schema and one closeout sequence; and
8. full closeout checks pass without exact measurements being copied into the
   receipt.

## On-ruling mechanism

### If O-A is selected

1. Record the owner's verbatim ruling in `D-44_RULING_2026-07-XX.md`, bind it
   to the ruling commit per K-AUTH-2, update D-44 to `RULED`, and codify the
   result as the next free `DEC-XXX` in `SOFTWARE_DECOMP.md` §12.
2. Merge that ruling-only change to `main`; no validator or instruction change
   executes before this point.
3. Open a fresh implementation branch. Pin the ledger prefix through the last
   pre-contract receipt, implement and register the deterministic validator,
   integrate it with self-check, update the three loop surfaces atomically,
   and add the first versioned receipt.
4. Run the packet's validation criteria and complete the implementation
   through an unmerged PR for owner review.

### If O-B is selected

Follow the same ruling-first sequence, then freeze the ledger, create the
per-run receipt and pointer surfaces, implement their validator and
concurrency rules, update all loop callers atomically, and complete the
implementation through an unmerged PR.

### If O-C is selected

Record and codify the hold. Make no validator or loop-surface implementation
change.

## Explicit non-authorizations

Preparing, committing, or merging this packet does not select an option,
authorize validator implementation, change the receipt schema, amend the
standing loop protocol, alter any historical receipt, approve any owner-
direction transcription as a ruling, change deliverable lifecycle, or merge
an implementation PR.
