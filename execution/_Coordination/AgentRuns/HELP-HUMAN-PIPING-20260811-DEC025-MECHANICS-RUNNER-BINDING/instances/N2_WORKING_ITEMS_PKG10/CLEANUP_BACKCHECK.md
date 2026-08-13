# Cleanup Backcheck — N2 Ignored Cargo Lockfile

Verdict: `PASS — EXACT OWNER-AUTHORIZED CLEANUP; N2 MAY RESUME`

Authority: owner direct approval, 2026-08-12, to verify and delete only the
exact ignored `core/runner/headless/Cargo.lock`, prove containment, and resume
N2 through a fresh remediation author.

## Before deletion

- branch / HEAD:
  `codex/piping-dec025-case-runner-binding-20260811` /
  `f1e311fb7ab1c2a0800b1d32c59445368428dee9`;
- target: `projects/chirality-piping/core/runner/headless/Cargo.lock`;
- regular non-symlink file, mode `-rw-r--r--`, one hard link;
- device `16777232`, inode `33897855`, size `10,114` bytes;
- SHA-256
  `7a3bd7e0df41a07e5c503aa312734e95fa6625afcd8b12f1f7994bd7a75b2e66`;
- ignored by `core/runner/headless/.gitignore:2`;
- it was the sole ignored path; staged inventory was zero.

The complete nonignored inventory was the two authorized production source
files, adopted candidate, and existing managed run root. Pre-cleanup identities
included preserved runner candidate SHA-256
`a86576c6143db678b6437d41f9c8904a548ec2135ff8ecc235cbb456ab57376d`,
accepted N1 source SHA-256
`18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`,
and candidate SHA-256
`c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`.
Every one of the 23 pre-existing managed run files was hashed before deletion.

## Operation and after state

One explicit non-recursive operation deleted only the exact path:

```text
rm -- projects/chirality-piping/core/runner/headless/Cargo.lock
```

Afterward:

- the target is absent and is not a symlink;
- `core/runner/headless` remains a real directory;
- ignored inventory is zero;
- staged inventory is zero;
- the complete nonignored inventory is unchanged;
- runner candidate, accepted N1 source, adopted candidate, and every one of
  the 23 pre-existing managed run-file hashes are byte-identical to the
  recorded before state.

No other path was deleted or modified by cleanup. This record is derivative
cleanup evidence only; it authorizes no production, lifecycle, Git, or
publication effect beyond the separately approved N2 resumption.
