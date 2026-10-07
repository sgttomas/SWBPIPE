# B6: the reader items (corpus and harnesses; before B1's snapshot)

TASK (Type 2), an implementer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance.** Your ID names a records folder and a role, with no memory of earlier sessions. Earlier holders of related roles (I66, I67, I69–I71, RV78, RV94, RV101) left their work in records. Cite them, and assume nothing beyond them.

## Why

B6 is the breadth unit that runs beside B1 (PLAN §2.1: U8 → B0 → B1 and B6 → PR-B1).
- **What it carries:** the reader items routed to wider F2a.
- **Why it goes first:** it is the shared corpus's single writer before B1's snapshot (DESIGN_v2 §7, B6 rows). So its corpus edits land first, and B1 re-pins on top of them.
- **What it must not touch:** the F2a D1 milestone's call graph. TS aligns to the Python and Rust codes (PLAN decision 11). PP and every D1 crate `src`, including the Rust reader `RE/src/retained_precision.rs`, stay unchanged.

## The items

1. **Mutation 277 as a one-entry slice** (RR:12697): `g7_not_required_quality_enum_invalid` is in no slice today. Add the slice in all three harnesses, so that each reader runs it.
2. **N-3's G7 code alignment** (RR:11246; PLAN decision 11). An invalid enum in a `not_required` case's quality is refused at G7 with Python and Rust's `SOURCE_NUMERICAL_CASE_INVALID`, but with TS's `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`.
   - Align TS to `SOURCE_NUMERICAL_CASE_INVALID`. The Rust base validators are unchanged.
   - Remove or narrow the scope clause I67 added to declare the difference (RR:11246), and update the pins in all three languages.
3. **F-U6b-2** (RR:9922; RR:10449; I74 decision 8): a transport validator for the retained successor in Python, the twin of Rust's `for_source_metadata` and TS's `sourceContractTransport`.
   - Python keeps refusing transported successors and successor packages where a declared difference says so. State which declarations change.
   - Use RV92's tampered probes (`R/REVIEW_RV92/`) as the shared refusal set.
4. **RV94 N-5** (RR:11394): Python reads `expected` and never `expected_by_reader.python`. Make Python read its own field where one is present, as Rust and TS do, with a mutation that the corpus kills.
5. **RV78-N1** (RR:9461, D36-tracked). It was deferred "because of its re-pin cascade". The records use the label for more than one thing (compare RR:8069 and RR:8302 with `R/REVIEW_RV78/`), so do this in order:
   - **First, establish exactly what RV78-N1 asks** and what remains open today.
   - **If what remains would re-pin existing corpus entries,** or overlap B1's re-pin (DESIGN_v2 §3.4), **stop and return a checkpoint** with the options before implementing it.
   - Otherwise, implement it.

## Constraints

- **Corpus edits are append-only.** New entries go at the end, so no existing slice moves. Name the new snapshot (after 07l), and record its sha256.
  - **B1 will add** the L = 0-based entries, the D38 pin, the F-1 entries and the W-C2 pins on top of yours.
  - **Don't add** any of B1's entries.
- **Unchanged:** PP, every D1 crate `src` (including RS), the schema, the fixtures, and the T6S files. If an item cannot be done without one of these, stop and return a checkpoint.
- **All three readers pass the full corpus.** Every count change is an added test or entry, and the cross-language declarations stay truthful.

## Where and how

- **Your worktree:** `WT/b6`, branch `codex/piping-t3-b6-20261007` from main `bfb26596bf`, created by ROOT. Commit on it with truthful messages; ROOT pushes. Make no other Git writes.
- **Every cargo goes through `WT/tools/t3_cargo.sh`** (`--locked --offline`). Heavy vitest and pytest runs go under `lockf -k WT/guard/cargo_job.lock`.
  - Other T3 jobs share the lock (ROOT's DEC-025s, I81, I82, RV104). Wait for it, and never kill another job.
  - **Not allowed:** DEC-025, evidence sweeps, native or solver jobs, and installs.
  - **For vitest,** link `node_modules` as ROOT did for `WT/t6-outputs`, and remove the link afterwards. Copy wasm assets only into your worktree's untracked public folder, and remove them afterwards.
- **Scratch** goes in `WT/scratch/<id>_b6/`. Nothing goes to the system temp directory.

## Output

- **The record:** `R/<id>/b6_01/RETURN.md`, with `_run_records/` and SHA256SUMS, placeholder paths only. It contains:
  - the head and commits;
  - each item's change and evidence;
  - the new snapshot's name and sha256;
  - the suites in three languages, base against head;
  - mutants;
  - anything ROOT must rule on.
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/<id>_b6/records/` and say so.
- **Budget:** 5–8 h. Return once, unless item 5 needs a checkpoint.
