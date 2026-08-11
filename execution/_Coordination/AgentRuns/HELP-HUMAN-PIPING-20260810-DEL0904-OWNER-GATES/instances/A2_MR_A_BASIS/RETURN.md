# A2-MR-A-BASIS Structured Return

RUN_STATUS: `SUCCESS`

FAN_IN_STATUS: `PASS_WITH_SHARED_WORKTREE_EXTERNAL_DRIFT`

ControlSurface: `FILE`

TaskProfile: `NONE`

TaskSkill: `NONE`

ScopePath: `/private/tmp/chirality-piping-del0904-owner-gates-20260810/projects/chirality-piping/docs/validation_manual/` plus the sealed instance review package

ToolsUsed:

- `read` — governed instructions, accepted packet, candidate/source evidence,
  decomposition/DAG pointers, and validation results.
- `apply_patch` — generator, manual index, validation strategy, and new review
  package files.
- `python3 docs/validation_manual/cases/generate_validation_case_pages.py` —
  deterministic preview, authorized 64-page regeneration, and `--check`.
- read-only Python/hash/diff/Git-status commands for membership, content
  identity, determinism, witness, and containment backchecks.

ToolPolicyCompliance: `PASS`

WriteAuthorization: `ALLOWED_WRITE_TARGETS`

## Result

MR-A Phase A is complete. No review or promotion act was performed.

- Exact corpus: 64 pages — 21 mechanics, 15 stress, 28 nonlinear.
- Exact accepted-PACKET_V2 membership comparison: symmetric difference zero.
- Every page: `DRAFT_EVIDENCE`.
- `MAINTAINER_REVIEWED` pages: zero.
- Review dispositions: zero; all 64 instrument rows say `NOT_RECORDED`, which
  is explicitly defined as a null state rather than a disposition.
- Witness chains: 1 `WITNESS_CHAIN_PRESENT`
  (`STRESS-TP-PHYS-015-CANONICAL-RESULTANT-STRESS-RECOVERY`) and 63
  `WITNESS_CHAIN_MISSING`.
- Promotions and promotion-map entries: zero.

## Applied Changes

- Corrected the generator's superseded runner paragraph to record that PR
  #287 bound the committed `run-benchmark`/`run-regression` payload families,
  without inventing per-page runner coverage; `export-results` remains the
  only structured runner stub.
- Preview-generated all 64 pages outside the repository, then regenerated the
  exact authorized corpus. Exactly 63 pages changed: 20 mechanics, 15 stress,
  28 nonlinear. The DEC-092 page retained its distinct locked/offline
  crate-test-only reproduction text and is byte-identical to base.
- Corrected the two runner-stub statements and current authority paragraph in
  `docs/validation_manual/index.md`.
- Corrected `docs/VALIDATION_STRATEGY.md` to cite revision `0.11` and
  `DAG-009` as current authorities while retaining revision `0.7` as the last
  full strategy-content alignment baseline. Revision `0.10` changed the
  actor-neutral reproduction/external-prover posture, so full 0.11 alignment
  was not asserted without a complete strategy delta review.
- Created a hash-bound 64-row zero-disposition review instrument, basis
  manifest, and substantive changed-path manifest.

## Outputs

- `REVIEW_INSTRUMENT.md` — 64 rows; SHA-256
  `0bc44d02ff8649cfc74275ddbde12e3a449019c4a8928d9aceb772af0eee49d8`;
  calculated Git blob `c87607a47898371d1bb7c9cafebc18f6bf1054cd`.
- `BASIS_MANIFEST.md` — shared-source/current-authority identities and corpus
  aggregate; SHA-256
  `c70e6096e73bf48523dd164c6764a085de872e5fe62feb24f229eaa944e51f04`;
  calculated Git blob `8bded98278b7a20fb294f4c59fda550604a6b592`.
- `CHANGED_PATH_MANIFEST.md` — exact 68-entry hashed substantive manifest
  (66 modified paths plus the two earlier review artifacts), excluding itself
  and the terminal runtime envelope to avoid self/cyclic hashes; SHA-256
  `15e1426e837b05314ece932d4b72d3952b446d11f8e538a2672cd3f631364919`;
  calculated Git blob `e99d75e5146b7da7d8110b585681e92a1848040d`.
- Final page-corpus aggregate SHA-256:
  `e33877bef390f371a009c06e8247b56a2c410ad5fb499a8fc4fadb06165d8b45`.
- Generator SHA-256:
  `d36bb1acf0513c043e230293eaa016ea6b243d1ea6b96f876518f844244a0994`.

## Validation Evidence

- Generator `--check`: `PASS`, 64 pages.
- Fresh final generation to `/private/tmp/chirality-mra-final.zPsED2`:
  mechanics/stress/nonlinear directory comparisons all exit 0.
- Generator/file/index membership: exact; no extra or missing page.
- Accepted V2 case-ID membership: exact 64/64.
- Tier scan: 64 `DRAFT_EVIDENCE`; zero `MAINTAINER_REVIEWED`.
- Stale shared runner diagnostic/prose scan: zero hits across generator, 64
  pages, and index.
- Tracked derivative diff: exactly 66 paths — generator, index, strategy, and
  63 generated pages. DEC-092 has no diff.
- `git diff --check` on all derivative changes: exit 0.
- Candidate-whitespace remediation attempt 2: removed exactly the three
  surplus terminal blank lines identified by manager fan-in, refreshed every
  affected cross-hash, and reran the scoped candidate-whitespace validator to
  `PASS` with no semantic change.
- Aggregate remediation attempt 3: corrected the corpus aggregate to use the
  declared repository-relative UTF-8 path, one NUL byte, the lowercase ASCII
  page SHA-256, and one LF byte per sorted row. The exact reproducible Python
  recipe in `BASIS_MANIFEST.md` yields
  `e33877bef390f371a009c06e8247b56a2c410ad5fb499a8fc4fadb06165d8b45`;
  all 64 individual page hashes and all page bytes remain unchanged.
- Formal witness structural backcheck: JSON parses; every schema-required
  top-level key is present; 11 formulas and 6 outputs; all three referenced
  artifacts exist; canonical JSON SHA-256
  `1af3d494093179bb93435be0e252ba3b84d8d6959a49a3c4aee455b8153518dd`
  matches its deterministic Markdown record. The system Python lacks the
  `jsonschema` dependency, so this is not represented as a fresh full schema
  validator run.
- A2 write containment: `PASS`; every A2 write is within the sealed whitelist.

## Missing

- Fresh full `witness_validator.py` execution was unavailable because the
  system Python lacks `jsonschema`; the committed witness evidence and the
  fresh structural/hash/path backcheck remain recorded without inflation.

## Needs Human Ruling

- None within MR-A Phase A. Any actual page review disposition or promotion
  remains a separate owner/maintainer act.

## Dependency Notes

- Shared-worktree terminal ignored status is not zero: regular file
  `projects/chirality-piping/core/runner/headless/Cargo.lock`, size 10,114,
  SHA-256
  `7a3bd7e0df41a07e5c503aa312734e95fa6625afcd8b12f1f7994bd7a75b2e66`,
  mtime `2026-08-11T09:42:06-0600`. This A2 never invoked Cargo and neither
  created nor touched that out-of-scope path. It is external concurrent drift
  requiring manager attribution/closeout; this A2 did not modify or delete it.
- No dependency cycle was encountered.

## Non-effects

No owner/maintainer disposition, page promotion, promotion map, register or
receipt action, authority mutation, lifecycle/stage/reliance/release effect,
staging, commit, push, PR, merge, reset, clean, or deletion occurred.
