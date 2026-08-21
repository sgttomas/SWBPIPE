# Piping SCA-009 Gate 5 — validation record

**State:** `GATE 5 CANDIDATE — CLOSURE RULING PENDING`

All commands were run in the SCA-009 worktree (branch
`claude/piping-sca-009-gate2-20260820`, basis
`main@89758a32634ee6cedbd1dbadf35e3728fb48d2eb`) on 2026-08-20, from the
repository root unless noted. Every check below is deterministic and
read-only against the surfaces it validates.

## 1. Validator inventory and applicability

Root inventory: `tools/validation/` (per its README and file listing).
Piping-local inventory: `projects/chirality-piping/tools/validation/`.

| Validator | Applicable? | Ran | Outcome |
|---|---|---|---|
| `tools/validation/validate_candidate_whitespace.py` | YES (committed-range whitespace gate run by `governance-harness.yml` "Candidate whitespace") | YES (see §2) | `PASS` over `origin/main...HEAD` including the new notice and snapshot members |
| `tools/validation/validate_decomposition_registers.py` | YES (decomposition companion-register estate) | YES (see §3) | exit 1 with 2090 ERROR findings (DRB-006 ×1388, EVQ-006 ×701, EVQ-004 ×1) — identical at basis `89758a326` and at HEAD, all pre-existing, zero attributable to SCA-009 (proof in §3); the tool exits nonzero on any error regardless of provenance, so its exit code is not a pass/fail signal for this candidate |
| `tools/validation/validate_claims_language.py` | YES (DEC-081 piping claims-language taxonomy, scans piping surfaces incl. this snapshot and the new notice) | YES (see §4) | `VALID` — 269 files scanned, 0 findings, exit 0 |
| `tools/validation/validate_dependencies_schema.py` (and piping-local copy) | Subsumed — single-file v3.1 `Dependencies.csv` schema check, delegated to by `validate_decomposition_registers.py`; SCA-009 touched no `Dependencies.csv` and DEL-07-09 has none yet | via §3 | covered by §3 |
| `tools/validation/validate_scope_change_packet.py` | NO — validates the PKG-00 "Scope Change Consumable Packet" artifact class, not `_ScopeChange` SCA snapshots | no | N/A |
| `projects/chirality-piping/tools/validation/validate_architecture_basis.py` | NO — SCA-009 does not amend the SCA-001 architecture basis | no | N/A |
| `tools/validation/validate_agent_instructions.py`, root G0–G4 validators, loop-receipt validators, DOMAIN validators, semantic/skill validators | NO — their governed surfaces (agents/, root governance state, loop receipts, DOMAIN decompositions, skills) are untouched by SCA-009 | no | N/A |

No deterministic validator exists for the ScopeLedger/Deliverables/
ContextBudgetQA companion registers as such (the register validator's
families target the `Dependencies.csv` estate) or for `_ScopeChange`
snapshot completeness; those surfaces are covered by the §5–§8 checks
below and by the downstream formal AUDIT_DECOMP obligation recorded in
`Propagation_Plan.md`.

## 2. Candidate whitespace (committed range)

Command:

    python3 tools/validation/validate_candidate_whitespace.py --base-ref origin/main

Protocol: the validator checks the committed range, so it is run
immediately after this tranche's commit is created; under the
amend-until-clean protocol used throughout SCA-009, this record is final
only in a commit whose range check printed:

    PASS: candidate whitespace is clean (untracked binary/symlink paths safely skipped: 0).

Pre-commit staged equivalent, run over the fully staged tranche:

    git diff --cached --check   → exit 0 (no findings)

## 3. Decomposition companion registers (non-mutating; exit 1 on any error)

Command:

    python3 tools/validation/validate_decomposition_registers.py projects/chirality-piping/execution

Outcome: **exit 1** with 2090 ERROR findings. The tool exits nonzero
whenever `error_count > 0` (`main()`: `if report["error_count"]: return
1`), regardless of whether the findings predate the candidate, so its
exit code is not a pass/fail signal for this candidate. Summary:
`ERROR DRB-006 ×1388` (DependencyID prefix vs FromDeliverableID),
`ERROR EVQ-006 ×701` (EvidenceFile path does not resolve),
`ERROR EVQ-004 ×1` (placeholder locus); 0 warnings. The identical run in
a clean worktree at basis `89758a32634ee6cedbd1dbadf35e3728fb48d2eb`
produced the identical summary (2090 ERROR / 0 WARNING) and the identical
exit 1. **Zero findings attributable to SCA-009**, by construction:
every finding is in the per-deliverable `Dependencies.csv` estate, and

    git diff --name-only origin/main...HEAD -- "*Dependencies.csv"   → 0 files

— SCA-009 modified no file of that family (DEL-07-09 has no
`Dependencies.csv` yet; its extraction is a recorded downstream
obligation). The findings are pre-existing at the basis and belong to the
loop's own register-repair backlog, not to this amendment.

Correction disclosure: an earlier revision of this record stated
"exit 0 (report-only)". That was false — the exit status had been
captured after a `| tail` pipeline, measuring `tail` instead of the
validator. The finding counts and the pre-existing attribution were and
remain correct; only the exit-code characterization is corrected, here
and in the §1 inventory row and the `ACCEPTANCE_RECORD.md` Gate-5
validation summary.

## 4. Claims language (DEC-081)

Command:

    python3 tools/validation/validate_claims_language.py

Outcome:

    VALID claims-language surfaces: 269 files scanned; DEC-081 registry taxonomy satisfied   (exit 0)

Scan includes the applied live surfaces, this snapshot's files, and the
new coordination notice.

## 5. Coverage computation (pre/post; computed, not transcribed)

Command: `python3 build_coverage.py` (script retained in the session
scratchpad; parses the decomposition sections and the three companion
registers — §4/§5/§6/§7/§9/§10 row counts, register row counts, envelope
distribution from `ContextBudgetQA.csv`, and mapping-based coverage
percentages).

- PRE at `89758a326` (rev 0.11): SOW 76 | PKG 18 | DEL 101 | context rows
  101 | OBJ 18 | envelopes `S=9, M=69, L=23, XL=0` | declared==found:
  True | forward/reverse/objective coverage all 100.0%.
- POST at HEAD applied state (rev 0.12): SOW 77 | PKG 18 | DEL 102 |
  context rows 102 | OBJ 18 | envelopes `S=9, M=69, L=24, XL=0` |
  declared==found: True | forward/reverse/objective coverage all 100.0%.

Both match the Gate-2/Gate-3 expected deltas exactly; no mismatch.
Results written to `Pre_Change_Coverage.json` /
`Post_Change_Coverage.json` (synthesized deterministic baselines; the
formal AUDIT_DECOMP pre/post comparison remains a downstream obligation).

## 6. ID-uniqueness sweep (live surfaces)

Commands: `grep -c '^|DEL-07-09|' / '^|SOW-077|' / '^|DEC-094|'` on the
live `SOFTWARE_DECOMP.md`; `grep -c` of the register row keys.

- `DEL-07-09`: exactly 1 decomposition deliverable row; exactly 1 row in
  `Deliverables.csv`; exactly 1 row in `ContextBudgetQA.csv`.
- `SOW-077`: exactly 2 decomposition rows (the §4 register / §9 ledger
  pair); exactly 1 row in `ScopeLedger.csv`.
- `DEC-094`: exactly 1 decomposition decision row.
- No occurrence of a stray `DEL-07-10` in the registers (the R-006
  conditional's hypothetical future slice is not minted).

## 7. CSV parses

`python3` `csv` parses, uniform column widths, data-row counts:

| File | Rows | Widths |
|---|---|---|
| live `ScopeLedger.csv` | 77 | {10} |
| live `Deliverables.csv` | 102 | {10} |
| live `ContextBudgetQA.csv` | 102 | {6} |
| `Supersession_Delta.csv` | 1 | {13} |
| `Supersession_Map.csv` | 4 | {13} |
| `Amendment_Actions.csv` | 15 | {9} |

## 8. Six-file application re-proof

`shasum -a 256` of the six live surfaces at this tranche, equal to the
approved postimage hashes from the `Amendment_Preview.md` table (order:
decomposition, ScopeLedger, Deliverables, ContextBudgetQA, DEL-07-03
status, DEL-07-03 context):

    a1bf1148b8b96aed83c8a042437b9714d543cb068070662b97397dc700da48a3
    97f5a113739e193f07f6d1dfac36f43e08642e12c983f3a3b597a31abe553238
    f46ea92ab5e5896e86f6b0b8ecbec4e98158886def5d0cf0d7b80d0200f03539
    84497efb81d4b31b085e016d37a285aed3755be7740a632794e355dad259468d
    41039f77e6c0d86dcd013e0c17f09bf51889fbc1aef3a9a68e54d834f8e95596
    e69a8d7d832b5af0f15e08c3c242e9d5ace98f26a752ee41fda06855f528ae10

All six match. The applied bytes are exactly the Gate-3-approved bytes.

## 9. Pointer non-advance

`git diff --cached --name-only -- '**/_LATEST.md'` → 0 paths.
`_Decomposition/_LATEST.md` and `_ScopeChange/_LATEST.md` are untouched by
this tranche; both still cite the 0.11/SCA-008 state (expected
mid-amendment posture; advance is the post-ruling closing commit,
pointer-last).

## 10. Register-consistency note

`Amendment_Actions.csv` row 11 (`DEL-07-03-RESIDUAL-REPOINT`) was corrected
`SupersessionBindingPresent NO → YES` in this tranche: the schema requires
`YES` while a corresponding `Supersession_Delta.csv` row survives, and this
tranche mints that row. The correction is disclosed here and in
`ACCEPTANCE_RECORD.md`; the Gate-3 ruling target (the six
preimage→postimage pairs) is unaffected.
