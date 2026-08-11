# A2-R14 Structured Return

- **Status:** `PASS`
- **RunID:** `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES`
- **Parent:** `WORKING_ITEMS/Agent1/working_items_del0904_owner_gates_prepare`
- **Child:** `A2-R14`
- **Objective:** prepare the owner acceptance packet for immutable bundle
  `REPRO_DEL0904_20260720T074714Z_a5235340aae3`
- **Accepted base checked:** `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`

## Outputs

| File | SHA-256 | Disposition |
|---|---|---|
| `instances/A2_R14/PACKET.md` | `8218b4566c4e3d4476ee7f29db4b7e96ae8740c32273bc9b7c4102b86172335d` | Complete owner-ready packet; non-authoritative decision support only. |
| `instances/A2_R14/RETURN.md` | self-hash intentionally not embedded because that would be recursive | This structured return; parent can compute final byte hash during fan-in. |

## Result

- All P1-P16 predicate results independently reconstruct as `PASS` from
  direct committed command, output, exit, comparison, cleanliness, state, and
  check witnesses.
- Bundle integrity independently passes: 75 files; 74 indexed entries; every
  `SHA256SUMS.txt` entry verifies; bundle tree
  `3d847390dfa74f8dced090164fb95f31eade83c7` is unchanged from introduction
  commit `4ff617ae123131a1c0152ad8fa42a46fbe1b305d` through the accepted base.
- The actor-neutral PRD §24 R6 criterion permits maintainer- or agent-executed
  reproduction; the governed agent executor is not a criterion deficiency.
- **Currency finding:** no later change invalidates the historic run, but the
  bundle is not current-head evidence. Explicit brief §8 triggers fired in the
  headless runner/manifest/dependency surface, product-physics source,
  mechanics suite, contract test, workflow profile, approved DAG pointer, and
  toolchain/environment. The procedure, PRD, both generators, all eight input
  fixtures, all eight witnesses, stress/nonlinear suites, and low-level solver
  tree remain byte-identical.
- **Recommendation:** qualified `ACCEPT` at pinned source `a5235340…`; leave
  evidence label `INTERNALLY_VERIFIED`; require a new governed clean-checkout
  bundle before any current-base reproduction claim.

## Sources and checks

Read and cross-checked:

- the complete immutable bundle, its 74-entry checksum index, manifest,
  README, command stream, environment record, all eight outputs, all 17 exit
  records, witness comparisons, cleanliness records, and check records;
- adopted brief
  `CB-2026-07-20-DEL-09-04-CLEAN-REPRO-R14-001` in full, including predicates,
  bundle contract, exclusions, and §8 rerun triggers;
- W5 run record and manager return, including the preserved portability-only
  first-attempt harness failure and successful cure/recheck;
- PRD §§21.1-21.3, §22.1, and §24 R6; `docs/claims_registry.md`; the validation
  manual's evidence-state definitions and reproduction slice;
- current DEL-09-04 status and `TM-PIP-037` trigger;
- read-only Git ancestry, introduction commit, blobs/trees/hashes, exact
  source-to-base diffs, relevant path histories, and current tool inventory.

Specific independent checks included:

- exact HEAD equality to the sealed base and source-commit ancestry;
- successful `shasum -a 256 -c SHA256SUMS.txt` over all 74 entries;
- introduction/current bundle-tree identity;
- direct JSON inspection of every P2-P11 output, including exactly one P2
  warning occurrence and the P7/P8/P9 case counts/statuses;
- all 17 expected/actual exit pairs;
- P15 parent/commit Remaining-section byte comparison and no receipt delta;
- exact blob comparison for the two generators, eight inputs, and eight
  witnesses;
- source/current tree comparison for runner, product physics, suite crates,
  solver, DAG pointer, contract test, profile, procedure, and PRD;
- source/current environment and toolchain comparison.

No procedure was rerun; currency conclusions derive from committed identity
and diff evidence, not an ad hoc ungoverned run.

## Ambiguities and disposition

One semantic ambiguity is material but not blocking: “accept the R14 bundle”
can mean accepting the truthful source-pinned historic evidence, or it can be
misread as asserting current-head reproduction. The packet resolves this by
presenting distinct options and recommending an exact qualified acceptance
form. A second distinction is explicit: owner governance acceptance of this
bundle is not the `ENGINEER_ACCEPTED` evidence label or the future formal
human-acceptance workflow contemplated by PRD §21.3.

No predicate witness is missing. No acceptance-law ambiguity prevents a
defensible owner ruling when the source pin and currency qualification are
stated.

## Containment and execution attestations

- I did not delegate, spawn, or use a child agent.
- I wrote only `instances/A2_R14/PACKET.md` and
  `instances/A2_R14/RETURN.md`, the two paths permitted by the sealed brief.
- I did not modify the launch brief, status file, bundle, manual, evidence,
  register, lifecycle, decision, case, fixture, witness, source, test, profile,
  DAG, or any other path.
- I performed no Git write, stage, commit, push, fetch, merge, reset, clean,
  branch, worktree, external, network, or destructive action.
- All reads and validations were non-writing. No temporary repository output
  was produced.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
