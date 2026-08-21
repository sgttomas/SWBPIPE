# N3 return — deferred public G4 derivative export

- Run: `HELP-HUMAN-PIPING-20260821-VOCABULARY-R2`
- Node: `N3`
- Instance: `TASK-VOCAB-R2-N3-G4-EXPORT` (bounded ephemeral Agent 2; no delegation)
- Accepted source basis: `7d5a3f558dfa2e8e902df25fc9a3e813a9ab7048`, the merge commit of PR #599
- Prior deferral: `HELP-HUMAN-PIPING-20260821-SCA009-CI-SUPPORT/HANDOFF_STATE.md` and the `derivative_disposition` in `PIPING-CI-SLOW-OBJECT-HARDENING-20260821.yaml`
- Verdict: `PASS`
- Blockers: none

## Execution

Exact exporter command, run from repository root:

```sh
python3 exports/chirality-app/export_public.py
```

The command was run twice. Each run reported 834 staged files, zero boundary
findings, and rewrote the same tracked manifest/report bytes. No
`--apply-target` argument was supplied and no external target was read or
written.

## Changed derivative inventory

Tracked derivative changes:

- `exports/chirality-app/export-manifest.csv`
  - 8 added lines / 7 removed lines against the accepted basis
  - 834 rows; 99,760 bytes
  - SHA-256 `bfa4040b5bec25059e343a31662c4e94efb2fb22e2c4d0a35a01d7289a0d5abf`
  - adds the accepted Piping G4 tranche manifest and refreshes seven changed
    source hashes/metadata entries, including the governance workflow and G4
    policy-test mirror
- `exports/chirality-app/export-report.md`
  - 2 added lines / 2 removed lines against the accepted basis
  - 593 bytes
  - SHA-256 `12027e3361143e79a1c873382c4013df496877d629393ac42176e4f1633afbd0`
  - manifest count advances 833 to 834 and `docs` count advances 161 to 162

The exporter also regenerated its ignored working staging tree at
`exports/chirality-app/staging/`: 834 files, zero tracked staging paths. That
tree is a rebuildable working derivative and is not part of the commit
inventory.

Node-local evidence added:

- this `N3_RETURN.md`
- `N3_CHECKS.json`

## Validation

- Export boundary scan: `PASS`; zero findings on both runs.
- Deterministic rerun: `PASS`; manifest and report SHA-256 values were
  identical after both invocations.
- Manifest/staging integrity: `PASS`; all 834 rows match staging membership,
  byte size, and SHA-256.
- Deferred mirror currency: `PASS`; staged
  `.github/workflows/governance-harness.yml`,
  `tools/validation/test_validate_instruction_tranche_manifest.py`, and the
  accepted Piping tranche manifest are byte-identical to source.
- Focused G4 policy tests: `PASS`; 47 tests.
- Whole-corpus G4 validation: `PASS`; 40 schema-valid tranche manifests,
  including `PIPING-CI-SLOW-OBJECT-HARDENING-20260821`.
- Scoped `git diff --check`: `PASS` for both tracked derivative files.

## Source/derivative boundary

The authoritative/source hashes recorded before export were identical after
export:

- `.github/workflows/governance-harness.yml` —
  `9e6a9ef814c79f1c35a00d154986bf3ba7174c49450d0e9cc36bca1de8f73079`
- `tools/validation/test_validate_instruction_tranche_manifest.py` —
  `f60566db9034522454cc73c22127d273a79558d1d98725a978e1c8bb5f55b289`
- `docs/governance_harness/tranche_manifests/PIPING-CI-SLOW-OBJECT-HARDENING-20260821.yaml` —
  `2979c44b57d13cfea1cc592adfff5b482d262b0fd8f0a9e6aff15d83b42a7602`
- `exports/chirality-app/export_public.py` —
  `23d51ee7d89db8ce12ed7494ea9ce4714a7b495c99f725689c06ce8b966ea235`

No authority/source, exporter policy, product, deliverable, coverage, receipt,
shared handoff, Git index, branch, commit, push, merge, or external application
surface was changed by N3. The two export files remain derivative evidence and
do not substitute for the accepted PR #599 source snapshot.

## Fan-in disposition

`PASS`; the prior G4 derivative deferral is discharged on the accepted
post-merge basis. N3 is ready for HELP_HUMAN fan-in.
