# Piping SCA-009 Run Summary — Gate-1 candidate package authoring

**Current state:** `CANDIDATE — GATE 1 NOT APPROVED`

**Basis commit:** `7584de0a8d53d69a135c22fe39a78cb4a30b6cb2` (main)

**Decomposition basis:** `SOFTWARE_DECOMP.md` revision 0.11

## Execution

Authored by an Agent 0-dispatched bounded generalist under owner direction
(owner request of 2026-08-20, transcribed in `Brief.md` Section 1 as
`CHAT_TRANSCRIPTION — EVIDENCE, NOT RULING`), in an isolated worktree on
branch `claude/piping-sca-009-gate1-20260820`. This run is candidate-package
authoring only: it is not a SCOPE_CHANGE gate execution, triggers no
reruns, and touches nothing outside `SCA-009_2026-08-20_0000/`.

## What was read

- `agents/AGENT_SCOPE_CHANGE.md` (protocol; Gate-1 vs Gate-2 boundary,
  snapshot layout, invariants)
- `execution/_ScopeChange/SCA-008_2026-07-27_2301/` (Brief, Decision_Log,
  Handoff_State, RUN_SUMMARY, Impact_Assessment conventions; state labels;
  hash discipline) and `_ScopeChange/_LATEST.md`
- `execution/_Decomposition/SOFTWARE_DECOMP.md` rev 0.11 — PKG-07/PKG-16/
  PKG-13/PKG-03/PKG-05 package rows; deliverable tables (DEL-07-01..08,
  DEL-16-01..04); scope register and Scope Ledger rows SOW-020/021/060/
  069/070/076; OBJ-006/OBJ-015; Vocabulary Map; OI-012/OI-016; plus
  whole-file searches for palette / toolbar / command / vocabulary
- Deliverable-local surfaces: ScopeOfWork.md and `_STATUS.md ## Remaining`
  of DEL-07-01, DEL-07-02, DEL-07-03, DEL-16-01, DEL-16-02, DEL-16-03,
  DEL-16-04; `DEL-07-03/_CONTEXT.md` (WATCH risk); DEL-07-06 MEMORY/run
  records (implemented object-creation toolbar evidence)
- Project-wide grep across `execution/**/*.md` for palette/toolbar
  coverage (gap verification)

## What was produced

Exactly one new candidate directory,
`execution/_ScopeChange/SCA-009_2026-08-20_0000/`, containing `Brief.md`
(owner request, evidence-based gap statement with exact line citations,
candidate reference operation vocabulary as assessment input, neutral
Option A/B analysis, owner-reserved decision points), `Impact_Sketch.md`
(preliminary; explicitly not a Gate-2 impact assessment), `Decision_Log.md`
(all items `PENDING`; Gate 1 first), `Handoff_State.md`
(`CANDIDATE — GATE 1 NOT APPROVED`; no-effect declaration), and this
`RUN_SUMMARY.md`.

Gap finding: verified. Command routing exists (DEL-07-01/02 OUT-001;
PKG-16 operation layer), but no deliverable enumerates the piping-domain
interactive model-building operation vocabulary and none owns the
tool-palette surface organization; adjacent surfaces (DEL-07-03 R-005/
R-006, DEL-16-04 generator residual, DEL-07-03 WATCH) independently record
the ownership hole.

## Package integrity

Per-file SHA-256 (computed before this file was written; this file is
excluded from the package hash):

| File | SHA-256 |
|---|---|
| `Brief.md` | `ad36f600c0bb796e1029c11f0f70370413c7864e343625da3988f9010bb64ffd` |
| `Impact_Sketch.md` | `7ac40e4e6a94b9276f0bc4a5501e59998c4bbf0db8eb17e51bff538eadfaad77` |
| `Decision_Log.md` | `3be90ef043df0799288397baa875195574b25f1fe15023fa204a602a33562c4c` |
| `Handoff_State.md` | `22a32707efdff85b9988488e5792e4263789cac1e786dd1d3d36151f1e0b7c64` |

**Package SHA-256 (covering the four files above):**
`2458c1dce9b175330c8b28a4a0e4647988213539ee4edb8d27f7ae74e0e9adc0`

Method: `shasum -a 256 Brief.md Impact_Sketch.md Decision_Log.md
Handoff_State.md | sort` (lexicographic sort of the `hash  filename`
lines), then SHA-256 of that sorted list text.

## Handoff

**NextOwner:** Ryan Tufts. **NextAction:** Gate-1 ruling per
`Decision_Log.md` D1. No amendment, pointer, decomposition, lifecycle,
dependency, estimate, schedule, release, or Git-state effect exists beyond
adding this candidate directory. Standard claim fence applies (F-PIP-2;
DEC-081 claims taxonomy).
