# Run basis — RECON_2026-09-21_WHOLE_CORPUS

Whole-corpus deliverable reconciliation of `projects/chirality-piping/`.
Agent 0: HELP_HUMAN (Claude Code, Claude Opus 5). Orchestration record:
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/`.
This file is append-only after its first commit; later facts go in the phase
log at the end.

## Authorization

- D-73, ruled 2026-09-21 (`execution/_Coordination/_DECISIONS/D-73_RULING_2026-09-21.md`,
  `DEC-110`), merged to `main` by PR #837 at
  `e631e078643f317c4ed5d8c3284f13d3db8eecd7` before any reconciliation dispatch.
- Addendum of the same day (`D-73_RULING_ADDENDUM_2026-09-21.md`): no test
  suite is run; gate evidence is cited (see "Verification evidence").
- Phases authorized: **R0–R4 only**. R5 and R6 need a separate owner
  authorization.
- Write and read-only boundary: D-73 packet Item 5a–5d.

## Frozen source state

- Commit `00115c71931bcae79909602d653740d3bb72dfa1` (PR #835 merge).
- Piping tree `38cbfc64be7e55126c90b82a266846815aa52efb`, the same tree object
  as the PR #834 merge `620ff6387b211c774c0de7dabec0a170acdf6017`.
- Evidence checkout: a detached worktree of that commit in Agent 0's session
  scratch directory, created 2026-09-21. Tracked and ignored state were both
  empty at creation. No build, install or test runs in it. Workers read code
  and documents from it; their briefs name its absolute path, which is
  session-local and not recorded here.
- The integration worktree (the repository checkout Agent 0 commits from)
  carries ignored build state dated 2026-09-18, from before this session. It
  is not evidence and nothing reads from it for evidence.
- Material change after the freeze: Piping development is suspended by owner
  direction until this run is done. Each sub-batch boundary diffs the frozen
  Piping tree against `origin/main`; any change marks affected rows
  `STALE_INPUT`.

## Pinned method

| Path | Git blob | SHA-256 |
|---|---|---|
| `workflows/reconciliation/WORKFLOW.md` | `b7aa037d8565956e0bde6612fd29ae4599fe95f3` | `75948a77e7ee3ebd9cd9d8a2089c9aff75ca76b49d87260793e6fbdd152fa380` |
| `workflows/reconciliation/execution.json` | `4774e20015cdaafe8772cbc0f7099802111aab8e` | `35e108bccac694cac738db03c61709e3db1886f84a3ee5545a1bbc5bdd4f3ab2` |
| `workflows/reconciliation/resources/contract.md` | `dd997a40757d1d7d0297c6fcce53de6f9e029737` | `da47475cdf12422416d9d0ce16d2aa6bfb744dfed9731a08cf7ca66a05a6e4d5` |
| `workflows/reconciliation/resources/method.md` | `8bccdfb428891c88e7ee21739c831b6a2c42237e` | `d7b5e22a71a7de72445f8d2718d28b43764f9a7e4f069c674aecae1f33ccb86d` |
| `docs/DELIVERABLE_CONCORDANCE_METHOD.md` | `137209cb37e8d8204a7f2bd78114b4b5753c6c2e` | `abf3e78fce606c4557d61cdbfbdb7292a3d858838f6526da6b433d1bcd0ef627` |
| `projects/chirality-piping/docs/RECONCILIATION_PROFILE.md` | — | `271bd0d0637f061c080ee83931a8c0134bdff8fa354536c09e6a1ca4e71ada73` |

Method extensions (sealed two-pass protocol, cause tags, authority tiers,
extended ledger schema) are candidates in `R0_CALIBRATION/CANDIDATE_CONVENTIONS.md`
and bind only after the owner rules them at the end of R0.

## Authority basis at the frozen state

Git blobs at `00115c719`, paths relative to `projects/chirality-piping/`:

| Authority | Git blob |
|---|---|
| `docs/PRD.md` (SWBPIPE; formerly OpenPipeStress, SCA-010) | `f399024936690ea52db8fd181290897d1a2c260b` |
| `docs/CONTRACT.md` | `44a2ed025b8e02870835c44b00cac50daf073221` |
| `docs/DIRECTIVE.md` | `5966a2b493f178697f5c34cbdbdbd4d2fd654c24` |
| `docs/SPEC.md` | `aa3bc45b53c4139d22b2cdd1c3f16fb5f6e35cd2` |
| `docs/TYPES.md` | `d278ba00506ce15910c4e10e20238e6777949474` |
| `docs/IP_AND_DATA_BOUNDARY.md` | `2ec1d4cecc9ada8583ad763e45a7d3a2ee608299` |
| `docs/PROFESSIONAL_BOUNDARY.md` | `2c4d0d65891e88db151afe45c0f01f26d3f74bfc` |
| `docs/claims_registry.md` | `f776829d6d1c140417a8739b56f099536971389f` |
| `docs/_ScopeChange/_LATEST.md` (names SCA-003) | `fef742ecd925c13cf99e42ce7790788fb56ab9f1` |
| `execution/_ScopeChange/_LATEST.md` (names SCA-010) | `c0c0fe09baf99e78833ec9b95c8b9d857cc80972` |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` (revision 0.12; §12 through `DEC-109`) | `9b6240abdc261e1de8db2145c9db340475f1bb5f` |
| `execution/_Coordination/_DECISIONS/_REGISTER.md` (through D-72) | `89527000027832a4eb12650c2e44decfc1311dc6` |
| `execution/_DAG/_LATEST.md` → `DAG-010` (tree `9077b213ddde46aafa05cf29762ac67f24b27999`) | `b6d1c5f023f5bd9903f3547ad9ae55eefd5d6eb2` |
| `software-workflow.json` | `71feb5d68fc5790b414965c9714456a38f06900b` |
| `AGENTS.md` | `20eec666436d2f76d26495bc3be2eb11278a20f6` |
| `loop/LOOP_INIT.md` | `aa4acd0fbfc217ba8006b1a203bce378d26613e7` |

D-73 and `DEC-110` post-date the freeze; they govern the run and are not
audit inputs. The two scope-change pointers name different amendments; R1's
authority map records how each is used, without resolving precedence.

## Corpus census at the frozen state

- 18 packages, 102 deliverables: 100 `IN_PROGRESS`, 1 `ISSUED` (DEL-01-01),
  1 `OPEN` (DEL-07-09). 151 `## Remaining` items across 65 deliverables
  (`tools/coordination/list_deliverable_status.py`).
- Representations: 93 Scope of Work (`ScopeOfWork.md`), 8 architecture basis
  (`ArchitectureBasis.md`, PKG-00), 1 bespoke (DEL-07-09).
- Claim units issued by `tools/extract_claims.py` (SHA-256
  `34a118efc3ca8493a007051ef35474e754fde992d41d90458d002049ec96d87b`):
  `CLAIM_KEYS.csv`, 7,285 units from 102 deliverables (SHA-256
  `5a451fd8800b21273fbde0c86ae951a16746fad4e1c4a3a8d1980c9ec2518cb1`),
  identical on rerun. Unit grain is a calibration question.

## Verification evidence

Per the D-73 addendum, no suite is run. `GATE_EVIDENCE/` holds copies of the
existing records, blob-verified against their source commit
`44a30112a2d4af21768a9eef822ad1974a3869ec` (branch
`codex/swbpipe-continuation-20260919`, unmerged; listed in
`GATE_EVIDENCE/SOURCE_MANIFEST.tsv`):

- `PR834_CI/`: hosted CI on PR #834 head `339be6c9ee6616dfd60952957ee193f897bd05aa`,
  whose Piping tree is the frozen tree. All 8 checks passed before merge
  (runs `35614512667` governance-harness, `35614512716` Piping Desktop E2E).
- `B4_4_SWEEP_9D55/`: clean local DEC-025 five-surface sweep on
  `9d55bce37a3e683d6a1e026920855270b8929113`. Its Piping tree differs from the
  frozen tree only under `execution/`; all product paths are byte-identical.
  PASS: Python 1,138; desktop unit 1,621; source browser 455 plus 20 existing
  skips; dist browser 53; all cargo commands; production build.

A partial gate run started before the addendum was stopped and discarded; it
is not evidence.

## Concurrent work

- Piping: suspended by the owner until this run is done. The unmerged
  records-only branch `codex/swbpipe-continuation-20260919` (holds
  Receipt-161) and all other sessions' worktrees are untouched.
- App: a separate whole-corpus reconciliation (D-APP-128, PR #836) runs
  concurrently in `projects/chirality-app-dev/`. Disjoint write scope; shared
  host test resources are not used by this run.

## Execution parameters

- Topology: Agent 0 → WORKING_ITEMS managers (from R2) → TASK workers.
- Models: `claude-opus-5`, high reasoning, every role.
- Concurrency: at most 16 live agents including Agent 0; checked against
  `RUN_STATE.jsonl` before each sub-batch.
- Only Agent 0 commits.

## Fences

F-PIP-1 to F-PIP-4; DEC-043 equation-source exclusion; protected checks,
tolerances and oracles never moved; no lifecycle transition, DAG, scope or
deliverable change in R0–R4. Standard claim fence applies (F-PIP-2; claims
taxonomy per DEC-081).

## Phase log (append-only)

- 2026-09-21 — R0 opened. Evidence checkout created; gate evidence copied and
  verified; claim keys issued.
- 2026-09-21 — R0 calibration complete: 8 ledgers and the pilot inventory
  returned and verified; fresh review verdict READY WITH NAMED AMENDMENTS.
- 2026-09-21 — Owner ruled R0 (`R0_CALIBRATION/R0_RULING.md`): amendment set
  adopted; A3a adopted; DEC-101 does not reach deliverable SOWs (rename residue
  is a finding); ISSUED overtaken-by-ruling text is one lifecycle-reassessment
  group. Bound conventions `CONVENTIONS.md`; claim keys re-issued by extractor
  v2 as `CLAIM_KEYS_V2.csv` (12,854 units, 7,257 required); canonical table
  `CANONICAL_SITUATIONS.md` with 811 mechanical assignments; `EVIDENCE_MAP.csv`
  (72 parity records, 5 matching the frozen SOW); validator v2.
- 2026-09-21 — Disclosed Agent 0 departure: `.rNN` table-row keys are optional
  (all-or-none per block) rather than mandatory rows; see `RUN_STATE.jsonl`
  `DISCLOSED_DEPARTURE` for reason and reversal.
- 2026-09-21 — R0 PR review (verdict FINDINGS; return in the orchestration
  record) repaired. Corrections to the preceding entry: extractor v2 now
  bounds unquoted claim blocks (DEL-17-06 CLM-042 restored), hashes block
  bodies without headings (26 in-deliverable duplicates found), and keeps
  12,854 units / 7,257 required (808 pre-typed). Canonical assignments are
  729 (Package Reference compared on all four fields: 20 DRIFT). The evidence
  map lists 501 parity records found by content signature, covering
  88 of 93 SOW deliverables; 9 have a PASS record matching the frozen SOW.
  Current hashes: the latest `BOUND_INPUTS` event in `RUN_STATE.jsonl`.
- 2026-09-21 — Disclosure: the verbatim gate-evidence copies
  (`GATE_EVIDENCE/B4_4_SWEEP_9D55/START.json`, `registered/SWEEP_*.json` and
  the `.log.gz` files) contain absolute local paths from the host that
  produced them. They are left unedited to preserve blob custody.
- 2026-09-21 — Owner answered the R0 PR review's owner items (R0 ruling
  addendum): optional `.rNN` rows confirmed; active code identifiers carrying
  the former name are rename residue (including the four kept on 2026-09-18);
  consistency enforcement replaces per-hash canonical rows. Backcheck findings
  N1–N6 repaired: canonical assignments now 725 (16 Package Reference DRIFT;
  run-together Scope Detail blocks judged normally).
- 2026-09-21 — Disclosure: the verbatim review returns under the orchestration
  record's `returns/` contain absolute scratch paths from the session that
  produced them (harness REVIEW ABS_PATH_IN_UNCLASSIFIED_SURFACE). They are
  kept unedited for custody.
- 2026-09-21 — R1 capability inventory complete: 12 owner-free area
  inventories (`R1_INVENTORY/INV_*.csv`), each structurally checked with full
  file coverage by `tools/check_inventory.py`, merged into
  `IMPLEMENTATION_SURFACES.csv` (598 capabilities). Mechanical indexes and the
  authority map were built earlier in R1.

- 2026-09-21 — R0 PR backcheck 4 findings repaired. B4-2: routing widened
  (PKG-03, PKG-04, PKG-05 add FEATC; PKG-09 adds FEATB and FEATC; PKG-10 adds
  COREC). B4-3: worker routing files no longer mark the cross-area sample and
  are ordered by hash; the mapping moved to `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`
  for verifiers only. B4-1: `check_inventory.py` rejects directory tokens not
  strictly under the project folder (all 12 inventories re-pass). B4-4:
  `merge_inventories.py` and `build_path_hints.py` committed; both reproduce
  the bound files byte for byte. The `.mjs` language label in
  `VERIFICATION_INDEX.csv` (cosmetic) is left as is.
- 2026-09-21 — R2 wave plan and briefs written (`WAVE_PLAN.md`; R2 manager,
  worker and verifier briefs in the orchestration record).
- 2026-09-21 — R0 PR backcheck 5 findings repaired. B4-3 (residual): worker
  routing files drop the Area and Kind columns and carry routing-local IDs
  `RC-<nn>-<NNNN>` in the `CapabilityID` column, because inventory IDs encode
  the area. `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv` maps them back to inventory
  IDs, areas and AREA/SAMPLE; R3 resolves reverse answers through it. Workers
  may never read the manifest, the router, the path hints, the merged
  inventory or `R1_INVENTORY/`. This is a representation choice for the Part D
  reverse file, not a rule change. B5-1: verifier-rerun procedure added to
  `WAVE_PLAN.md`. B5-2: worker rerun clause authorises moving old files into
  `superseded_<n>/`. B5-3: R0 pilots read their calibration reverse file and
  reverse repairs only after sealing. B5-4: the 13-deliverable wave is listed
  for owner confirmation at the wave 1 checkpoint.
- 2026-09-21 — R0 PR review closed: backcheck 6 PASS. Disclosed residual: 276
  of the 521 cross-area sampled rows cite a module path prefix that no area
  row of the same package uses, so they are recognisable from `EntryPoints`.
  This is inherent (workers need the paths to answer); the verifier brief now
  says to weigh the sampled-versus-area comparison accordingly.
- 2026-09-21 — PR #839 CI (harness live baseline, GEN-8) failed: verbatim
  review returns and the sealed wave-1 launch messages sat in AgentRuns paths
  the harness does not classify, and they carry exact machine paths. They were
  moved unchanged (`git mv`) to the structural evidence directory
  `_run_records/` (`_run_records/returns/`, `_run_records/launches/`); earlier
  `RUN_STATE.jsonl` events keep their old `returns/…` paths as history, and a
  `RELOCATE` event records the move. Manager records now go under
  `_run_records/{WAVE}-{PKG}-MANAGER/`. This supersedes the earlier disclosure
  that the returns stay as REVIEW findings.
- 2026-09-21 — R2 wave 1 (PKG-07, PKG-16) ran and was verified at double
  sampling. The gate as written was not met: PKG-16 had 6.3% firm false
  alignment (1 of 16), and one shared-text resolution was unrecorded. The
  pooled rate was 4.4%, and all five firm errors had one cause. The owner
  adopted remedy R-1 to R-6, ruled F7 (implementation with no product
  caller) and F8 (the tier of the gap wins), and chose a second gate wave
  (`WAVES/W1/W1_GATE_RULING.md`; CONVENTIONS Part F). The validator gains
  `--notes-gap` and batch `--resolutions`; the worker, manager and verifier
  briefs carry Part F; `WAVES/W1/RESOLUTIONS.csv` records all 32 wave 1
  disagreements. Gate wave 2 is PKG-00 to PKG-04 at the full cap.
