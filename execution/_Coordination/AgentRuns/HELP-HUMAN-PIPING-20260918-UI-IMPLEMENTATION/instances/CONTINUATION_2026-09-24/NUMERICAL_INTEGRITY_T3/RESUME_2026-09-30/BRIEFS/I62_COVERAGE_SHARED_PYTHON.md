# I62: summary coverage in the shared schema and corpus, then the Python reader

TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is your return path. You do not delegate.

## Read first

- `REPO_ROOT/AGENTS.md`, `agents/AGENT_TASK.md`, `projects/chirality-piping/AGENTS.md`, and this brief.
- **The selected design:** `R/I57/summary_coverage_01/ADDENDUM.md`, all of it. Sections 1, 4 and 5 are your specification.
- **Its review:** `R/REVIEW_RV76/summary_coverage_01/REVIEW.md`.
- **The rulings:** in `T3/ROOT_RULINGS_V1.md`, read "Summary-coverage representation selected for the successor" and "Resumption by the next ROOT; coverage implementation planned".
- **The previous shared author's handoff:** `R/I58/python_shared_01/RETURN.md`, `SHARED_SNAPSHOT_03.json`, `SOURCE_FREEZE.json` and `CHECKS.json`. Its "Resume exactly here" section applies, except that you implement coverage only (see "Out of scope").
- **The contract basis the corpus already follows:**
  - `R/I32/f2a_wire_c1/WIRE_CONTRACT.md`;
  - `R/I32/f2a_wire_c2/CONTRACT_DELTA.md`;
  - `R/I52/prepared_public_contract_02/C3_DELTA.md` and `DEFINITION.json`;
  - `R/I52/prepared_public_contract_correction_03/ADDENDUM.md`;
  - the 06, 07 and 08 reader-seam addenda under `R/I52/`.
  Read these as you need them.

Record the files you actually read, with their sha256, in your RETURN.

## Paths

- `WT` = the T3 worktree root on the M5 host. Your dispatch prompt gives its absolute path; committed text uses the placeholder.
- **Your worktree:** `READER` = `WT/f2a-readers`, branch `codex/piping-f2a-readers-20261003`. Its head `ae97b7d5c2` is ROOT's unaccepted WIP commit of the frozen reader drafts. Edit only there, and only on your paths.
- `NUM` = `WT/numerics` (records; read from here).
- `P` = projects/chirality-piping
- `T3` = P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3
- `R` = T3/RESUME_2026-09-30

## Objective

Add the selected nullable `summary_coverage` member to the C3 ProofTrace in the shared contract and corpus, publish a frozen shared snapshot, and then make the Python reader check it.

Two later readers depend on your snapshot: Rust (I63) and TypeScript (I64). So the work has two checkpoints.

### Checkpoint A: shared schema, table and corpus (return to ROOT when frozen)

1. **The schema.** In `P/schemas/retained_precision_mp_v2.schema.json`, add the closed member to every non-null C3 ProofTrace, exactly as I57 §1 defines it: `summary_coverage: null | [ {body: U, stop: [bool,bool,bool,bool], has_data: bool} ]`, with no additional properties. Also update `P/schemas/results.v0.3.schema.yaml` if it embeds this shape.
2. **The preview table.** Update the table/contract fixture (`P/fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json`) where it lists ProofTrace members. Leave `DEFINITION.json`'s numerical contents and domain hash unchanged, as I57 §5 requires.
3. **The corpus** (`P/fixtures/results/retained_precision_cases.json`):
   - give both complete synthetic cases a complete `summary_coverage` consistent with their own source layout, extent, E, floor and data blocks, derived as in I57 §2;
   - rehash every scope the change affects. The receipt hash binds the new field; the source-binding and preparation hashes do not; the publication hash is unaffected;
   - add the §5 control table as rehashed first-failure mutations, each with its expected first gate and error code;
   - label positive controls synthetic;
   - keep all thirty existing mutations, with their expected outcomes updated only where the new field genuinely moves them. Say which, and why.
4. **Build method.** Find how I58 built and rehashed corpus 67d5cbcc00 (its records and bulk manifest), and extend that same method. Don't write a new serializer, a generic exact-sum engine or new tooling.
5. **Freeze.** Write `SHARED_SNAPSHOT_04.json` in your evidence folder. It lists every shared file with sha256 and size, the new mutation IDs with expected first failures, and what changed from snapshot 03 and why.
6. **Check.** Run the two retained-precision Python test files (command below). The schema tests must pass. Python reader tests may fail only where they now meet the new field; list each.

Then end your turn with checkpoint A's status. ROOT verifies the snapshot, releases I63 and I64 on it, and resumes you for checkpoint B.

### Checkpoint B: the Python reader (`P/core/analysis_runs/retained_precision.py`)

Implement the I57 §4 checks, in the existing gate order and with its exact error codes:
- **G1:** the closed shape. Exactly the required member; null or an array; closed body objects; four booleans; a boolean has_data.
- **G2:** body U encoding.
- **G3:** for a non-null array, the cardinality and ordered unique body ids equal the source inventory associated through this attempt.
- **G5:** first the existing schedule; then the same source/Run/proof binding and the stage and availability implications from §3's table.
- **G5a,** in this order:
  - the existing summary encodings and ranges, and the native p/P/floor rules;
  - the canonical source layout and extent;
  - the compact-flag Boolean feasibility rule (the sixteen A vectors, with D = false for this C3 scope);
  - the estimate/charge rederivation;
  - the exact summary rosters, items 1–4 in §4;
  - the directly derivable has_data constraints.

**Replace the draft's Cartesian summary-coverage assumption** with these rules. Never derive private facts from final rows. Earlier original checks still win. Keep the public Python API disabled: no eligibility is exposed.

All new §5 controls must produce their expected first failure, and the existing tests must still pass. Report the counts.

## Out of scope

- I58's other remaining audit items (failed-prefix, pre-helper, unavailable, old-Err/new-Ready and maxima-abandon controls; native schedule/cache audits). These follow under a later grant.
- The Rust and TypeScript readers; producer or kernel code; carriers, UI or standing; public activation; M or resources.

## Write fence

Only these READER paths:
- `P/schemas/retained_precision_mp_v2.schema.json`;
- `P/schemas/results.v0.3.schema.yaml`;
- `P/fixtures/results/retained_precision_cases.json`;
- `P/fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json`;
- `P/core/analysis_runs/retained_precision.py`;
- `P/tests/test_retained_precision_contract.py`;
- `P/tests/test_retained_precision_schema.py`.

`P/fixtures/results/retained_precision_prepared_ordinary_v1.json` (the definition) and `P/core/analysis_runs/physics_source.py` are read-only unless you return an exact need first. Anything else is a stop.

## Host and runtime

The memory guard must be running (`pgrep -fl memguard`). The test command, which I58 used, runs from `READER/P`:

```
OPENPIPESTRESS_CHECKED_JSON_BIN=WT/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson \
OPENPIPESTRESS_UNITS_BIN=WT/targets/i52-readers/units/release/openpipestress_units \
VENV/bin/python -m pytest -q \
  tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py
```

- Expand `WT` and `VENV` to the absolute paths in your dispatch prompt. `VENV` is the control checkout's `projects/chirality-piping/.venv`.
- **Both environment variables are required,** or the conftest can launch a hidden Cargo build.
- Put a 1,200-second wall on each run.
- **Never** run Cargo, an install, new tooling, or any solver or native job. I61 runs Cargo beside you in another worktree.

## Time box and stops

- **Checkpoint A:** 75 minutes from your first tool call.
- **Checkpoint B:** 90 minutes from ROOT's resume.
- At each deadline, freeze and report anything unfinished. Don't extend silently.
- **Stop and report** if:
  - the design and the contract basis conflict;
  - a mutation's correct first failure is ambiguous under the existing gate order;
  - you need a path outside the fence;
  - setup takes more than 5 minutes.

## Output

- **No Git writes and no index operations.** Git reads are fine, with `GIT_OPTIONAL_LOCKS=0`. ROOT commits.
- **Evidence:** in `NUM/R/I62/coverage_shared_python_01/`:
  - RETURN.md, short: changed files with sha256; each test command with its result; the mutations with expected and observed first failures; deviations; open items;
  - SHARED_SNAPSHOT_04.json;
  - SHA256SUMS.
  
  Put bulk logs in `WT/scratch/i62_coverage_shared_python_01/`, listed with hash, size and path. Use placeholder paths in committed text.
- **End each checkpoint's turn** with a concise status for ROOT: what changed, test counts, anything unfinished, and anything ROOT must rule on.
