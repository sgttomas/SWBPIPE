# Exact Cleanup Backcheck — DEL-09-04 Owner-Gates Application

Date: 2026-08-11

Verdict: `PASS — EXACT CLEANUP COMPLETE; ZERO IGNORED DRIFT`

## Original V-D lockfile

- Path: `projects/chirality-piping/core/runner/headless/Cargo.lock`
- Pre-cleanup type: regular, non-symlink
- Size: 10,114 bytes
- SHA-256: `7a3bd7e0df41a07e5c503aa312734e95fa6625afcd8b12f1f7994bd7a75b2e66`
- Cleanup result: exact file absent; parent retained.

## First-verifier target roots

The separately authorized cleanup child independently verified each root was
a real non-symlink directory, not a mount, with no symlink or special entries,
then removed only the three exact roots.

| Exact root | Regular files | Dirs including root | Regular bytes | Canonical file-list SHA-256 | Result |
| --- | ---: | ---: | ---: | --- | --- |
| `projects/chirality-piping/core/reporting/protected_content_linter/target/` | 277 | 13 | 13,642,088 | `b7b5c922ddf484a00e9e17b60066a88821e676cbca6c746fa631cc030a491147` | absent; parent retained |
| `projects/chirality-piping/core/runner/headless/target/` | 4,283 | 156 | 547,137,823 | `74e7bc0dc91ee5515ffaab8701d7f90a8c2eedfd53d421ad4cfd907b7978168a` | absent; parent retained |
| `projects/chirality-piping/validation/benchmarks/nonlinear/target/` | 1,189 | 37 | 101,489,154 | `30b3ac67409bfd3693a5fecd95fc72d30369e94ac8066fb3ee5473fdda7b38a2` | absent; parent retained |

The canonical file-list hash is the SHA-256 of the sorted stream produced by
`find <root> -type f -print0 | LC_ALL=C sort -z | xargs -0 shasum -a 256`,
then hashed once more with SHA-256. The first-verifier's richer descendant
fingerprints remain preserved in its attempt record; these canonical hashes
are the independent Agent 0 cleanup-gate identities.

## Post-cleanup independent manager checks

- all four exact artifact paths are absent;
- all three target parent directories are present;
- `git ls-files -o -i --exclude-standard` is empty;
- staged path count is zero;
- HEAD remains `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3` on
  `codex/piping-del0904-owner-gates-20260810`;
- the nonignored tracked/untracked state is unchanged by cleanup;
- no broad clean, reset, or other deletion was used.

The cleanup changes no ruled semantic artifact. Fresh independent terminal
verification remains required because the prior verifier was interrupted
before an accepted terminal return.
