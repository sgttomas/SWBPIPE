# OD7-G1 Piping Candidate Validation

Status: `PASS — CANDIDATE ONLY`

Basis: `main@fb16e32ed60bb4f384cf1e07a83c4a14ff63bbae`

## Results

- D-30 identity: PASS. Commit
  `712df44816cf5253223b449fec0f10b48abd585c` is the first-add commit for
  both D-30 files and an ancestor of the basis. The current ruling SHA-256 is
  `980f6aced0913d3b00b023b350c1265a2d50fa94596850c81c8f30d210ecd368`.
- Exact patch: PASS. `PIPING_C03_C04.patch` applies cleanly and exposes exactly
  the D-30 publication-SHA field, `_LATEST.md` `0.6 → 0.10`, and the separately
  owner-gated `SOFTWARE_DECOMP.md` frontmatter `0.9 → 0.10`.
- SCA-007 authority: PASS. Commit
  `ab0e6cfc4d4871989cf887fdd45e3c2d3168b41f` is an ancestor. Accepted
  SCA-007 `_LATEST`, handoff, action A023/A024, and current decomposition
  revision prose establish revision `0.10`.
- Preserved subject hashes: PASS. D-30 JSON is
  `942266eb00305403493649b7a31870be4073051a7b2fadb18576306860865c68`;
  D-APP-48 JSON is
  `ad10f6e5808754c4acf2e9114f189c892dbec2231a3059d3717e9689e7807040`.
- Bounded mismatch: PASS as expected failure. The combined validator exits
  `1` with exactly `ERROR: consumption source.commitSha mismatch`. This
  establishes record drift, not semantic incompatibility.
- ID scan: PASS. `D-57`, `DEC-090`, and `Receipt-75` are the highest live
  identities; `D-58`, `DEC-091`, and `Receipt-76` remain unreserved candidates.
- Receipt contract: PASS. The live ledger passes. A contained temporary ledger
  with `PROPOSED_PIPING_RECEIPT_76.md` appended also passes the D-44/DEC-075
  validator. An initial invalid Gate-Outcome phrasing was corrected before
  freeze; no authoritative byte was touched.
- Structured artifacts and paths: PASS. JSON and CSV parse; CSV has five data
  rows with uniform six-column width and no blank row. Every candidate source
  exists inside this proposal package and every proposed live path remains
  inside the Piping project root.
- Proposal containment and whitespace: PASS. No tracked or live Piping surface
  changed; every untracked Piping candidate file is inside this package and
  passes no-index whitespace checking.
- Hash freeze: PASS. `ARTIFACT_HASHES.sha256` was generated last, covers every
  other package artifact exactly once, and reproduces end to end.

A PASS is structural evidence only. It does not approve or apply C03/C04/C06.
