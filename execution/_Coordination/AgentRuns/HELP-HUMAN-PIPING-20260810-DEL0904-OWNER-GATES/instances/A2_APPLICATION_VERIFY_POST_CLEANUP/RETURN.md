# Structured Return — A2-APPLICATION-VERIFY-POST-CLEANUP

## Terminal verdict

`HOLD — RULED APPLICATION SEMANTICS PASS; SEALED-BRIEF PATH-ANCHOR VALIDATION FAILS`

The complete C-B / V-D / O-B / MR-A application and the post-cleanup
repository state independently reconcile. Terminal fan-in cannot be reported
as `PASS`, however, because the project path-anchor validator finds two
machine-local absolute paths in this verifier's own live sealed launch brief.
The verifier did not repair or rewrite its controlling brief.

## Identity and parentage

| Field | Value |
| --- | --- |
| RunID | `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES` |
| ParentInstanceID | `WORKING_ITEMS-A1-APPLICATION` |
| ChildInstanceID | `A2-APPLICATION-VERIFY-POST-CLEANUP` |
| Agent form | fresh non-delegating ephemeral Agent 2 verifier |
| Branch | `codex/piping-del0904-owner-gates-20260810` |
| Accepted basis / HEAD | `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3` |
| Locally observed `origin/main` | `912e3a8c9c07e9b8359093f63feace1c7c9f4776` |
| Current-main caveat | No fetch was authorized. The locally observed tracking ref is newer than the accepted basis, but its Piping tree and relevant root instruction files have zero delta from that basis. |

## Bounded-task shell report

RUN_STATUS: `FAILED`

ControlSurface: `FILE`

TaskProfile: `NONE`

TaskSkill: `NONE`

ScopePath:
`projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES/instances/A2_APPLICATION_VERIFY_POST_CLEANUP`

ToolsUsed:

- shell `git`
- shell `jq`
- shell `shasum`
- shell `find`
- shell `diff` / `cmp`
- shell `rg`
- Python `-B` standard-library checks and repository read-only validators
- runtime `apply_patch` for this return and status only

ToolPolicyCompliance: `PASS` — Cargo, Task Management federation, network,
dependency installation, and mutation-capable test suites were not invoked.
Python used `PYTHONDONTWRITEBYTECODE=1` and `-B`; every generated comparison
and temporary output was directed to the unique external root
`/private/tmp/del0904-postcleanup-verify.aZn0hN`.

WriteAuthorization: `ALLOWED_WRITE_TARGETS` — only this `RETURN.md` and this
instance's `STATUS.json` were deliberately written.

Outputs:

- this `RETURN.md`
- this instance's `STATUS.json`

MISSING:

- a path-anchor-clean sealed verifier launch record and a terminal verifier
  run governed by that record;
- commit-bound DEC-025 execution, which is lawfully post-commit and was not
  eligible in this uncommitted worktree;
- a fresh full Piping pytest pass in an interpreter with `jsonschema`; the
  frozen author evidence and safe structural checks were not inflated into a
  new full-suite claim.

NEEDS_HUMAN_RULING:

- none on C-B, V-D, O-B, or MR-A semantics;
- orchestration must decide whether to issue a new tokenized sealed brief and
  rerun terminal verification. This verifier cannot amend its own brief.

DEPENDENCY_NOTES:

- no semantic dependency cycle was found;
- owner selection of a public comparison number, page-specific review or
  promotion, TM-PIP-037 treatment, receipt, Git closeout, and merge remain
  separate governed acts.

## Requirements matrix

| Requirement | Result | Independent evidence |
| --- | --- | --- |
| Exact authority and frozen preparation | `PASS` | Owner ruling SHA-256 `5f0f857e5b95284e88506c768c91547b3f02815dfd63747c966d2df7b8775a22`; exact four owner lines present. `PACKAGE_RETURN.md` is `01855ffbaee369e9321ae16972af56183fb6fada66a99a7aa9680c73a8bc906f`; DEC, R14, and MR V2 packets are `1265e843c2c33eaa915f26cba5b75b72e811b7bf2bcc280bb90bcb02ecc5178c`, `8218b4566c4e3d4476ee7f29db4b7e96ae8740c32273bc9b7c4102b86172335d`, and `ee08a4af9cfb99ba624a2ab510f2c95d48484172a562f001bb60aa1eac9d8565`. |
| Frozen child returns | `PASS` | C-B `651a4a41374d63ef74592aa41b0f5a95712ee42d2fdb5d2c0c7bfc5594d2aeae`; V-D `8bd4d4acfc02c20d36fdf144fb49f891e45a5705b1615315c05cc67048eb73e2`; MR-A `23bc35e9750479a19684828402350d3b6f1037e3799748c3015a84a236f697b9`. The interrupted prior verifier was read only as attempt evidence and was not treated as terminal verification. |
| C-B policy | `PASS` | New record SHA-256 `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`, blob `468d6dd4a85525b64989ff520a5f4ff10e7c6e6f`; exact five-class order, zero-count relative fields and floors, cap four, inclusive final changed-support zero, policy-not-release and non-comparison fences. |
| C-B history and compatibility | `PASS` | All 12 pre-existing direct nonlinear JSON records equal their accepted-base blobs; all 13 current JSON files parse. Frozen focused consumer evidence is 8/8. No Cargo was rerun by this verifier. |
| V-D integrity and identity | `PASS` | All nine `SHA256SUMS.txt` rows pass; checksum index `2253707ec212069b77a4cd6416a08d028efb98c45887623b5ec3a0617d90daae`; manifest `a08a738634155b01be83a04f2777bfcbbb131246b934ebda24eaf9ad4860f385`; embedded raw suite object equals the standalone projection byte-for-byte after canonical JSON rendering. |
| V-D census and characterization | `PASS` | 25 requested, 11 matched, 0 mismatched, 14 blocked; 206 recorded, 91 observed, 115 unobserved. All 14 blocks are `implementation_binding`; 13 preserve the July-20 membership and DEC-092 is the fourteenth with six values. Twelve per-kind rows preserve measured floors/TBDs; sparse design is unit-normalized and selects no public numeric value or repair. |
| O-B and immutable R14 | `PASS` | Acceptance record `c110ba0649ed91bf624622f6f2deed8ec7a9787a26fd79870c6741e3c6a81f2b`; bundle tree `3d847390dfa74f8dced090164fb95f31eade83c7`; checksum index `581f039ffaca00097de75c151dcd13f9403c71076ab06671cd8e94c069f7ec7a`; 74/74 entries pass. Source remains `a5235340aae3c41cf227f5617e593b268936f6b3`, P1-P16 remain PASS, label remains `INTERNALLY_VERIFIED`, and no current-head reproduction is asserted. |
| MR-A deterministic corpus | `PASS` | Generator `--check` reports 64; a fresh external generation compares exactly. Membership is 21 mechanics / 15 stress / 28 nonlinear. Exactly 63 pages differ from base and the DEC-092 blob remains `edcb6e07de0d369a190f04318fd89162772b3f45`. |
| MR-A tier/review/witness posture | `PASS` | All 64 pages contain `DRAFT_EVIDENCE`; zero contain `MAINTAINER_REVIEWED`. Instrument SHA-256 `0bc44d02ff8649cfc74275ddbde12e3a449019c4a8928d9aceb772af0eee49d8` has 64 unique rows, 64 `NOT_RECORDED`, one `WITNESS_CHAIN_PRESENT`, 63 `WITNESS_CHAIN_MISSING`, and zero promotions. |
| MR-A reproducible basis | `PASS` | The documented repository-relative `path + NUL + lowercase SHA-256 + LF` recipe independently yields `e33877bef390f371a009c06e8247b56a2c410ad5fb499a8fc4fadb06165d8b45`. Basis and changed-path manifests retain final hashes `c70e6096e73bf48523dd164c6764a085de872e5fe62feb24f229eaa944e51f04` and `15e1426e837b05314ece932d4b72d3952b446d11f8e538a2672cd3f631364919`. |
| DEL-09-04 state | `PASS` | Lifecycle remains `IN_PROGRESS`; Last Updated is 2026-08-11. Exactly two Remaining bullets exist; the first is byte-identical to base (SHA-256 `09a9ae45caffc2ee4c0581155f0428ce308eb4e3a1958eccd03e3b04f7cd0978`) and only the second records C-B while keeping public result-comparison numeric selection open. Memory/history/run-record cleanup/currentness text is truthful. |
| Cleanup | `PASS` | `CLEANUP_BACKCHECK.md` SHA-256 `1a745171923f504f4e19cf6e332ffb1b58d24adac578237a1be826b34197f49c` records the exact lock and three target-root cleanup. All four paths are absent, target parents are retained, baseline and pre-return ignored inventories are zero. |
| Delta containment and protected non-effects | `PASS` | Before writing this return: 143 status entries, all 143 within the declared application/run scopes; zero staged paths. No Task Management register, receipt, claims registry, PRD, decomposition, DAG, GUI, export/CAEPIPE, reliance, lifecycle/release, runner/data/fixture/binding, or other forbidden path changed. Candidate whitespace validator passes. Claims-language validator passes 269 files. Receipt validator passes with receipts unchanged. |
| Live path-anchor validation | `FAIL` | `validate_path_anchors.py --text .` scans 1,283 live surfaces and reports exactly two findings, both in `instances/A2_APPLICATION_VERIFY_POST_CLEANUP/LAUNCH_BRIEF.md`: line 13 contains `/private/tmp/chirality-piping-del0904-owner-gates-20260810`; line 36 contains `/private/tmp`. This is the terminal HOLD reason. |
| DEC-025 | `DEFERRED_TO REQUIRED POST-COMMIT GATE` | The validation/docs/execution delta triggers the commit-bound sweep workflow. The current dirty, uncommitted state is not eligible; this is not a pre-commit semantic defect. No sweep was invoked. |
| Side-effect backcheck before return | `PASS` | Repository status and tracked-diff name lists are byte-identical to verifier baseline; HEAD/branch unchanged; ignored count zero; staged count zero. |

## Tested commands and safeguards

- `jq -e` exact C-B assertions; `jq empty` over all nonlinear JSON records.
- `shasum -a 256 -c` for V-D and R14 checksum indexes.
- Canonical `jq -S` raw/projection comparison and direct V-D analysis queries.
- `python3 -B generate_validation_case_pages.py --check` and fresh generation
  to the unique external temporary root, followed by recursive diff.
- Python `-B` reproduction of the MR-A corpus aggregate.
- Read-only Git blob/tree/status/diff comparisons, candidate-whitespace,
  claims-language, path-anchor, and receipt validators.
- No pytest command was run because repository harness paths can invoke
  Cargo transitively. Frozen focused-test evidence was retained with this
  explicit caveat. The system Python lacks the optional `jsonschema`
  dependency needed for the full witness/Piping suite; no installation was
  attempted.
- The generator emitted macOS developer-path cache warnings, but generated
  output remained outside the repository and the repository status/ignored
  inventory did not change.

## Remaining human and workflow gates

1. Resolve the path-anchor defect through a new governed launch instrument;
   do not rewrite this completed sealed brief in place.
2. Select or defer later V-D public comparison values under a separate owner
   ruling.
3. Conduct page-specific maintainer review and any promotion separately.
4. Treat TM-PIP-037 only through TASK_MANAGEMENT under separate authority.
5. Run commit-bound DEC-025, receipt, stage/commit/push/PR/merge only under
   their separate gates.

## Attestation

- I did not delegate.
- I did not invoke Cargo, Task Management federation, network access, or a
  mutation-capable test suite.
- I did not repair any artifact, delete anything, stage, commit, fetch, push,
  open a PR, merge, rebase, reset, or clean.
- I wrote only the two authorized verifier records and returned `HOLD` on the
  first terminal governance defect.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
