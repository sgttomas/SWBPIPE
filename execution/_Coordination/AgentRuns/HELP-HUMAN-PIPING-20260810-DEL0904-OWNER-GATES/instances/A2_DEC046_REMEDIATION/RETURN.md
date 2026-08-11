# STRUCTURED RETURN — A2-DEC046-REMEDIATION

## Status

`PASS`

`PACKET_V2.md` is owner-ready. It preserves the verified convergence evidence
and exact historic 13-case census while repairing the failed owner interface:
DEC-046 convergence and DEC-026 verification are separate top-level gates;
the comparison gate now has three concrete evidence-derived numeric options,
an explicit measurement-only option, and defer/decline.

## Identity and parentage

| field | value |
| --- | --- |
| RunID | `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES` |
| ParentInstanceID | `WORKING_ITEMS/Agent1/working_items_del0904_owner_gates_prepare` |
| ChildInstanceID | `A2-DEC046-REMEDIATION` |
| Agent form | fresh non-delegating ephemeral Agent 2 generalist, attempt 2 |
| exact branch | `codex/piping-del0904-owner-gates-20260810` |
| exact HEAD | `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3` |

## Outputs

| path | SHA-256 |
| --- | --- |
| `instances/A2_DEC046_REMEDIATION/PACKET_V2.md` | `1265e843c2c33eaa915f26cba5b75b72e811b7bf2bcc280bb90bcb02ecc5178c` |
| `instances/A2_DEC046_REMEDIATION/RETURN.md` | self-referential file hash must be computed by the parent after this final write |

## Owner interface returned

- **Gate C (DEC-046 only):** exact C-A/C-B/C-C convergence tables retain
  changed-support threshold `0 count` and use caps `2/2/2/2/3`, `4`, or `6`.
  C-D defers. C-B is the non-binding recommendation and is supported by all
  22 accepted assembled observations.
- **Gate V (distinct DEC-026-derived public comparison policy):**
  - V-A is the exact measured boundary per analytic kind and sparse
    subcriterion;
  - V-B is the same envelope with an expressly owner-selected 10× headroom;
  - V-C projects the ruled `1.0e-9` relative seed plus the internal absolute
    `1.0e-9` precedent into unit-bearing pairs, and preserves the current
    narrow sparse limits;
  - V-D authorizes only a current capture/measurement/design return; and
  - V-E defers/declines.
- **Overall recommendation:** C-B plus V-D (or V-E if no more preparation is
  wanted). This does not prevent the owner from selecting V-A/V-B/V-C; their
  exact values and caveats are fully stated.

## Deterministic recomputation checks

| check | result |
| --- | --- |
| exact HEAD | `PASS` — equals sealed base |
| original remediation dependencies read | `PASS` — plan amendment V2, original packet/return, failed verifier return |
| mechanics source population | `PASS` — 24 requested, 11 matched, 13 blocked, 91 compared, 109 unobserved recorded values |
| strict-exact mechanics boundary | `PASS` — 78/91 values; 7/11 complete cases; 13 nonzero values in exactly PHYS-004/005/006/007 |
| mechanics per-kind maxima/floors | `PASS` — recomputed from every observed value, including moment zero-reference floor `1.7763568394002505e-15 N-m` |
| V-A mechanics admission | `PASS` — 91/91 values, 11/11 cases |
| V-B mechanics admission | `PASS` — 91/91 values, 11/11 cases |
| V-C mechanics admission | `PASS` — 91/91 values, 11/11 cases |
| stress population | `PASS` — 11/11 exact values, 3/3 cases |
| nonlinear regression population | `PASS` — five exact residual values and five exact categorical case projections |
| assembled convergence population | `PASS` — 9 seed + 13 multi-support; 21 at most iteration 2 and cascade at iteration 3 |
| sparse maxima | `PASS` — relative `7.060341894958857e-11`, absolute solution `5.342535303043405e-10`, residual `1.0058283805847168e-7`, repeat `0`, pivots `0` |
| V-A/V-B/V-C sparse admissions | `PASS` — 9/9 for each option on its declared subcriteria |
| blocked-ID set | `PASS` — packet table/source symmetric difference empty; 13/13 primary implementation |
| blocked option effect | `PASS` — every C/V choice compares 0/109 values and unblocks 0/13 cases |
| source object identities | `PASS` — all 14 packet Git blobs resolve at the accepted base |
| packet required interface | `PASS` — separate C/V gates, concrete options, formula, finite/zero/unit semantics, suite matrices, recommendation, ruling form, application boundaries, 24/25 caveat, fences |
| packet whitespace check | `PASS` |

All numerical checks used read-only parsing of committed JSON and no evidence
rerun. No test, generator, build, or repository-writing script was invoked.

## Explicit inferences, headroom choices, and TBDs

1. The result-comparison formula
   `abs(o-r) <= max(atol_kind, rtol_kind*abs(r))` is proposed application
   algebra for the DEC-026 pair/floor structure; it is not the current helper.
2. V-A assigns zero absolute floors to exact observed kinds without a zero
   reference only as an explicitly labeled exact-boundary inference. It does
   not prove near-zero robustness.
3. V-B's multiplier `10` is disclosed owner-selected headroom, not statistics,
   uncertainty analysis, professional guidance, or measured necessity. Exact
   zero members remain zero.
4. V-C's unit-bearing `1.0e-9` absolute members are a policy proposal derived
   from the crate-internal absolute epsilon; that epsilon expressly is not
   existing public policy. Extending V-C to blocked-only force-per-length and
   mass-per-length is labeled unmeasured inference.
5. V-A/V-B keep force-per-length and mass-per-length relative/absolute members
   `TBD` because the committed runner capture has no observation for them.
6. The sparse absolute solution delta is recorded in heterogeneous “solution
   DOF units,” not a unit-normalized per-kind floor. V-C's current sparse
   precedent has no absolute sparse/dense solution limit; it remains
   `TBD / not gated` and the omission is part of any V-C choice.
7. A current 25-fixture run likely has a fourteenth DEC-092 block because the
   runner binding is unchanged. This is an inference, not a committed run
   result; the exact census remains the historic 24-fixture/13-block capture.
8. C-B's one-to-two-iteration headroom is a recommendation from the accepted
   validation values; C-C's extra iterations are unsupported by current
   observed need.
9. Current-population admission counts do not distinguish V-A/V-B/V-C. Their
   materially distinct values govern future envelopes; no statistical
   generalization is claimed.

## Containment and attestation

- I read only committed sources and the immutable run-history inputs at the
  exact sealed base.
- I wrote only
  `instances/A2_DEC046_REMEDIATION/PACKET_V2.md` and
  `instances/A2_DEC046_REMEDIATION/RETURN.md` via `apply_patch`.
- I did not alter the original author outputs, verifier return, plan,
  amendment, status, project truth, evidence, code, fixtures, case pages,
  manual, register, receipt, lifecycle, decision, or Git state.
- I did not delegate or spawn another agent.
- I did not stage, commit, fetch, push, merge, rebase, reset, clean, delete,
  use network/external systems, rerun evidence, or perform a destructive
  action.
- The packet stops before every owner ruling and application gate.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
