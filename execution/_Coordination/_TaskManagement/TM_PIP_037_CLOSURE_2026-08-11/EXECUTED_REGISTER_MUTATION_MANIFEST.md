# Executed register mutation manifest — TM-PIP-037

Status: `EXECUTED EXACTLY — OWNER-RULED CLOSURE`

## Register identities

| Surface | Before SHA-256 | After SHA-256 | Before Git blob | After content blob |
| --- | --- | --- | --- | --- |
| `projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER.csv` | `2175d2c4db7a480cd6ff77b9964d3815ff7558361df3a132838763d49a49ebfe` | `a88fecfdf26f1f984f83c264ff53c4e28f73bfc66efa58caa91d461ac750c200` | `4d19f55bd90dac938b12b970abf4d3729daa0154` | `a23f64e01f13accb9fcffaf191f0f43594dd7c84` |
| `projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER_CLOSED.csv` | `a92c7c7ebceca79a6bfbbf5b1eb94063a6c1099b734b9e26167bf5726556369f` | `c110c052fa2735b31c6889b8fdd7f2898d7a0194fc5bf5fbf703bc9024472192` | `72834422d5308636c6ee7554344cc9c7d0af03d2` | `060006a502a0f47d157e58679b4338f769cf2afe` |

The after content blobs are local `git hash-object` identities, not commit or
merge claims.

## Exact eight-field mutation

Only `TM-PIP-037` changed semantically:

| Field | Before | After |
| --- | --- | --- |
| `Status` | `DEFERRED` | `CLOSED` |
| `Disposition` | empty | `RESOLVED_BY_DECISION` |
| `EvidenceRef` | empty | `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES/OWNER_RULING_2026-08-11.md; validation/benchmarks/nonlinear/release_convergence_policy.dec046.c-b.json; validation/evidence/comparison_measurement/DEL0904_VD_20260811/MANIFEST.json; execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES/R14_QUALIFIED_ACCEPTANCE_RECORD_2026-08-11.md` |
| `EvidenceSha` | empty | `ab17acbd19be9fcce163d7a13bb17dd7d0fbe4d1; 468d6dd4a85525b64989ff520a5f4ff10e7c6e6f; c73d064b2d7fd7a1560e2a47def38df5c4610801; ab2847763043faf8ac3912fc43731e17e111fd7f` |
| `EvidenceQuote` | empty | `DEC CONVERGENCE: C-B; DEC COMPARISON: V-D; R14 REPRODUCTION: O-B.` |
| `LastReviewed` | `2026-08-08` | `2026-08-11` |
| `Closed` | empty | `2026-08-11` |
| `Notes` | frozen historical value | frozen historical value plus `OWNER_CLOSURE_RULING_2026-08-11: attention-row closure only; public comparison-number residual remains open; no lifecycle/release/reliance/page-promotion/GUI/export-CAEPIPE/repair/publication/professional-approval effect; no other row altered.` |

The exact pre-mutation CSV record, including terminal LF, had SHA-256
`8678d162809a909d727551fd79d4931447611794f2f21dc30392f61a2e99a8c6`
and length 2,008 bytes. The exact archived record after mutation, including
terminal LF, has SHA-256
`250f9b2023f23e765436b1113036aec391921df8707a1b8dfdba3ecda2ad50c3`
and length 2,901 bytes.

An independent field comparison proved that `Status`, `Disposition`,
`EvidenceRef`, `EvidenceSha`, `EvidenceQuote`, `LastReviewed`, `Closed`, and
`Notes` are the only changed fields. Every other field, including the complete
historical `Trigger`, is field-identical.

## Storage result

- Before: live 33, archive 7, combined 40.
- After: live 32, archive 8, combined 40.
- `TM-PIP-037` is absent from live and occurs once in the archive as
  `CLOSED / RESOLVED_BY_DECISION`, reviewed and closed `2026-08-11`.
- The original 32 surviving live rows retain exact order and field identity.
- The original seven archive rows retain exact order and field identity; the
  ruled `TM-PIP-037` row is appended eighth.
- Every one of the other 39 combined rows is field-identical to the frozen
  pre-mutation state.
- `TM-PIP-038` and `TM-PIP-039` remain unique, live, and `OPEN`.

Relocation is deterministic storage mechanics only. The owner ruling is the
disposition act.
