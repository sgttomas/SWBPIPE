# Piping SCA-009 Run Summary — Gate-1 candidate package authoring

**Current state:** `CANDIDATE — GATE 3 NOT APPROVED` (Gate 1 `CONFIRMED`;
Gate 2 `APPROVED` with two owner modifications; see the Gate-2 and Gate-3
tranche sections at the end of this file. The earlier sections are
preserved unchanged as history.)

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

---

# Gate-2 tranche — Gate-1 ruling record and Gate-2 impact assessment

**Current state:** `CANDIDATE — GATE 2 NOT ACCEPTED`

**Current basis:** `main@89758a32634ee6cedbd1dbadf35e3728fb48d2eb` (merge of
PR #591). Gate-1 package merged as PR #592 (merge
`01a8dd4c0aabd4fe1f71bba7201a4345f9e6cfdc`); original durable basis
`7584de0a8d53d69a135c22fe39a78cb4a30b6cb2`.

## Execution

Authored on 2026-08-20 by an Agent 0-dispatched bounded generalist (HELP_HUMAN
coordinating under the owner's explicit direction), in an isolated worktree on
branch `claude/piping-sca-009-gate2-20260820`. This tranche records the
owner's Gate-1 rulings and produces the Gate-2 candidate assessment; it is
not a gate acceptance, triggers no reruns, and touches nothing outside
`SCA-009_2026-08-20_0000/`.

## What was read

- The Agent 0 relay of the in-session owner rulings (verbatim blocks,
  accepted recommendations, tiered backend-coverage inventory, basis facts)
- `agents/AGENT_SCOPE_CHANGE.md` (Gate-2 four-lens requirements, required
  assessment tables, Gate-3 decision-needs contract)
- `SCA-008_2026-07-27_2301/` `ACCEPTANCE_RECORD.md`, `Impact_Assessment.md`,
  and `Decision_Log.md` (verbatim-ruling, ruling-table, and assessment
  format conventions)
- `SOFTWARE_DECOMP.md` rev 0.11: §1, §4 (SOW-020/021 lines 126-127;
  SOW-076 line 182), §5 (OBJ-006 line 194, OBJ-015 line 203), §6 (PKG-07
  line 220, PKG-16 line 229), §7 (DEL-07-01..08 lines 315-322,
  DEL-16-01..04 lines 407-410), §9 (ledger rows 485-486, 534-535, 541),
  §10 (counts and envelope distribution), §12 (DEC-018 line 604, DEC-049
  line 635, DEC-081 line 667)
- `docs/_Registers/` `ScopeLedger.csv` (77 lines = 76 rows),
  `Deliverables.csv` (102 = 101 rows), `ContextBudgetQA.csv` (102 = 101
  rows) — row formats and counts
- `DEL-07-08/Dependencies.csv` (register schema v3.1 format)

## What was verified (backend-inventory anchor spot-checks)

The relay's coverage-inventory citations were spot-verified against the
tree at the current basis; drifted line numbers were corrected. Verified
anchors (paths relative to `projects/chirality-piping/`):

- `core/product_physics/src/lib.rs`: `PreviewModel` :94; `PreviewPipe`
  :140; branch fields :222-238; expansion-joint fields :260-272;
  `SpringHangerInput` :335 (relay ~334); `NonlinearSupportInput` :363
  (relay ~362); support-family collapse to `Guide` :3160-3174;
  `SeismicGenerationInput` :430; `WindExposedSpanInput`/`exposed_spans`
  :463; `compute_pipe_mass_per_length` :4730
- `core/solver/linear_supports/src/lib.rs`: `SupportFamily` enum :18-25
  (incl. LineStop, VerticalSupport)
- `core/model_operations/operation_applier/src/lib.rs`: `FieldKind::
  EntityRef` :190; deferred inspector fields (`Component.kind`,
  `Combination.terms`) :255-256 (relay ~254); `check_kinds` closed
  change-kind set :1606-1629 (19 kinds + `insert_component_symbol`);
  `insert_component_symbol` always-blocked :1678 (relay ~1679);
  combination-basis closed set :3786
- `schemas/model_operation.schema.json`: unresolved change-kind tokens
  `move_geometry`/`reconnect` :338-339; `OperationAuthorType` incl.
  `"agent"` :306-310; `$defs.OperationKind` :407 (unresolved enum tokens
  `move` :411, `reconnect` :414, `constraint` :415, `design_knowledge`
  :418)
- `apps/desktop/src/features/viewport/PipeViewport.tsx`: armed
  object-creation toolbar :590-643 (Node/Pipe/Support/Component/Load;
  relay ~592)
- `apps/desktop/src/App.tsx`: `undoStack`/`redoStack` :359-360
- `apps/desktop/src/services/operationService.ts`: single-engine routing
  through the one applier :9-12
- `apps/desktop/src/features/pcf-export/PcfExportPanel.tsx`:
  `free_end_and_equipment_nozzle_semantics` :391
- `core/library_import/library_import_document/src/lib.rs`: `LibraryKind`
  Material|Section|Component :50-53 (relay path `core/library_import`
  corrected to the `library_import_document` crate)
- snubber: zero hits across `core/`, `apps/`, `schemas/` (confirmed)

## What was produced

- `ACCEPTANCE_RECORD.md` (new): verbatim Gate-1 owner rulings (in-session
  chat, 2026-08-20, Ryan Tufts) with the Gate-1 effect; Gates 2-5 PENDING.
- `Decision_Log.md` (updated): original pending register preserved as
  history; SCA-008-convention ruling table added (D1 `CONFIRMED`; D2
  Option A; D3 two-class vocabulary ruled; D4 single owner DEL-07-09; D5
  fold R-005/R-006 with generator out; D6 SOW-077).
- `Impact_Assessment.md` (new): full Option-A Gate-2 assessment — exact
  candidate entity additions (DEL-07-09 row + M-envelope context row,
  SOW-077 register/ledger rows, OBJ-006/OBJ-015/PKG-07 mappings), topology
  delta 76→77 SOW / 101→102 DEL / 101→102 context rows with all other
  counts unchanged, affected `SOFTWARE_DECOMP.md` sections by name, D5
  fold and DEL-16-04 exclusion, proposed DEL-07-09 depends-on edges
  (DEL-16-01, DEL-07-01, DEL-07-02), derivative-package and
  derivative-surface tables, orphan-risk summary, dev-loop
  write-disjointness, Gate-3 decision needs. Supersedes `Impact_Sketch.md`
  (untouched Gate-1 history).
- `Vocabulary_Annex.md` (new): the D3-ratified two-class vocabulary as
  candidate normative text — NORMATIVE-NOW (Tier 1 + Tier 2 + accepted
  Tier-3) and ROADMAP (deferred), every item with tier provenance,
  verified backend anchor, and implemented-taxonomy binding; DEC-049
  user-imported hanger-library framing; DEC-018 display-toggle units
  framing; sequencing rule; the binding-constraint normative sentence.
- `Handoff_State.md` (updated): `CANDIDATE — GATE 2 NOT ACCEPTED`; next
  owner act is the Gate-2 ruling citing the `Impact_Assessment.md`
  SHA-256; no-effect declaration renewed.
- This `RUN_SUMMARY.md` update (written last; excluded from the package
  hash).

## Package integrity (Gate-2 tranche)

Per-file SHA-256 of every package file except this `RUN_SUMMARY.md`
(computed after all other files were final):

| File | SHA-256 |
|---|---|
| `ACCEPTANCE_RECORD.md` | `6a8ff30add02366e72c99089cd95d1503bbd3da6943769d4c542548e21e6ded0` |
| `Brief.md` | `ad36f600c0bb796e1029c11f0f70370413c7864e343625da3988f9010bb64ffd` (unchanged from Gate 1) |
| `Decision_Log.md` | `580875a4531b6029a1a042d835672fb37da21cbdb9e293e12cd98cd83c4766bd` |
| `Handoff_State.md` | `a5e83e4767045178645ae842a77e924516f0d5f6df6865efe0db9bcb6a1ccd15` |
| `Impact_Assessment.md` | `bfa25d898e65b82012b2a93988432a121d5f2b842a5469cf7d53593a1a2ba6d0` |
| `Impact_Sketch.md` | `7ac40e4e6a94b9276f0bc4a5501e59998c4bbf0db8eb17e51bff538eadfaad77` (unchanged from Gate 1) |
| `Vocabulary_Annex.md` | `08ead1ec5dda6d888ab5ebe849304e0fdc178e8da2ea2f8dacbcbf6807f58d97` |

**Gate-2 package SHA-256 (over the sorted `hash  filename` lines of the
seven files above):**
`e81840568c5b28acbeb0e42f75bc6786ffb6183ac9f1b511c2eac648cfde325d`

Method: `shasum -a 256 ACCEPTANCE_RECORD.md Brief.md Decision_Log.md
Handoff_State.md Impact_Assessment.md Impact_Sketch.md Vocabulary_Annex.md
| sort` (lexicographic sort of the `hash  filename` lines), then SHA-256 of
that sorted list text. `Brief.md` and `Impact_Sketch.md` hashes reproduce
the Gate-1 record exactly, proving the Gate-1 artifacts are untouched.

## Handoff

**NextOwner:** Ryan Tufts. **NextAction:** Gate-2 ruling citing
`Impact_Assessment.md` SHA-256
`bfa25d898e65b82012b2a93988432a121d5f2b842a5469cf7d53593a1a2ba6d0`;
acceptance opens Gate 3 only. No amendment, pointer, decomposition,
lifecycle, dependency, estimate, schedule, release, or Git-state effect
exists beyond this candidate directory's own files. Standard claim fence
applies (F-PIP-2; DEC-081 claims taxonomy).

---

# Gate-3 tranche — Gate-2 approval record and exact amendment candidate

**Current state:** `CANDIDATE — GATE 3 NOT APPROVED`

**Current basis:** `main@89758a32634ee6cedbd1dbadf35e3728fb48d2eb` plus the
intact ruled-upon branch commit
`f5112824f055b3b5584a852dd68923530dc6620b` (no amend; this tranche is a new
commit on top). Single landing PR #593 per the owner's process direction.

## Execution

Authored on 2026-08-20 by an Agent 0-dispatched bounded generalist
(HELP_HUMAN coordinating under the owner's explicit direction), in the same
isolated worktree on branch `claude/piping-sca-009-gate2-20260820`. This
tranche records the Gate-2 approval and stages the exact Gate-3 amendment
candidate; it applies nothing, triggers no reruns, and touches nothing
outside `SCA-009_2026-08-20_0000/`.

## What was read

- The updated Agent 0 relay sections E (Gate-2 ruling verbatim +
  single-PR process direction) and F (accepted implementation-landing
  mapping)
- `SCA-008_2026-07-27_2301/` `Amendment_Preview.md` and
  `Amendment_Actions.csv` (Gate-3 preview and action-register conventions)
- `agents/AGENT_SCOPE_CHANGE.md` Gate-3 requirements and the Amendment
  Actions schema
- Live preimages at the basis: `SOFTWARE_DECOMP.md` (frontmatter, §1
  revision paragraphs, §4/§5/§6/§7/§9 insertion anchors, §10 telemetry,
  §12 decision-log tail, §13 gate posture), `docs/_Registers/`
  `ScopeLedger.csv` / `Deliverables.csv` / `ContextBudgetQA.csv` (formats,
  counts, trailing newlines), and `DEL-07-03/_STATUS.md` / `_CONTEXT.md`
  (full text for the bounded re-pointing)
- `docs/PRD.md` for the SOW-077 SourceRef verification

## What was verified

- **PRD citation:** `docs/PRD.md` is version 0.3 (adopted authority,
  amended 2026-07-16 per D-47/DEC-080/SCA-007); §14 "GUI Requirements"
  (§14.1 Main Interface) supports the SOW-077 GUI scope claim. SourceRef
  `PRD v0.3 §14` is correct; no correction required.
- **Next-free decision ID:** live §12 ends at `DEC-093` (`DEC-092`/
  `DEC-093` landed via other instruments after the Gate-2 assessment), so
  the new row is **`DEC-094`** — verified collision-free.
- **Ruled-upon commit intact:** `f5112824f` remains HEAD's parent basis;
  `Impact_Assessment.md` hash `bfa25d89…` is unchanged.
- **Postimage integrity:** every edit anchor asserted exactly one
  occurrence; postimage counts verified — 77 register + 77 ledger SOW
  rows, 102 deliverable rows, 94 DEC rows in `SOFTWARE_DECOMP.md`;
  78/103/103 lines (header + rows) in the three register postimages;
  envelope distribution `S=9, M=69, L=24, XL=0`.

## What was produced

- `ACCEPTANCE_RECORD.md` (updated): Gate-2 section with both verbatim
  owner direction blocks (single-PR process direction; Gate-2 approval
  "Yes, add the landing column and rule the envelope L. …"), the ruled-on
  assessment hash and commit, and the two owner modifications (envelope L;
  landing column). Gate 2 `APPROVED`; Gate 3 `PENDING`.
- `Decision_Log.md` (updated): process-direction row and Gate-2
  `APPROVED — WITH MODIFICATIONS` row.
- `Vocabulary_Annex.md` (updated): "Implementation lands in" column added
  to every NORMATIVE-NOW and ROADMAP row per the accepted relay-F mapping
  (including the R-006 conditional: DEL-07-02 as inspector forms unless a
  future design act creates a dedicated panel slice), with the landing
  principle recorded in the header; no other item content changed.
- `Amendment_Preview.md` (new): `CANDIDATE — GATE 3 NOT APPROVED`;
  preimage/postimage SHA-256 table and complete exact embedded diffs for
  all six touched surfaces; exact decomposition/register/DEL-07-03 change
  lists; DEC-094 and PRD verification notes; not-touched inventory.
- `postimages/` (new): six byte-exact candidate postimages —
  `SOFTWARE_DECOMP.md` (revision 0.12), `ScopeLedger.csv`,
  `Deliverables.csv`, `ContextBudgetQA.csv`, `DEL-07-03_STATUS.md`,
  `DEL-07-03_CONTEXT.md`.
- `Amendment_Actions.csv` (new): 15 action rows per the SCA-008 schema —
  the decomposition postimage application, SOW-077/DEL-07-09/OBJ/PKG-07/
  DEC-094 entity actions, the three register additions, the bounded
  DEL-07-03 re-pointing, and the Gate-5 rows (snapshot completion, both
  pointer advances, one Piping dev-loop coordination notice; SCA-009 is
  Piping-internal, so no Root/App/Tier-0 notice is required).
- `Handoff_State.md` (updated): `CANDIDATE — GATE 3 NOT APPROVED`; Gate-2
  APPROVED recorded; next owner act is the Gate-3 ruling citing
  `Amendment_Preview.md` SHA-256; zero applied effect renewed.
- This `RUN_SUMMARY.md` update (written last; excluded from the package
  hash).

## Package integrity (Gate-3 tranche)

Per-file SHA-256 of every package file except this `RUN_SUMMARY.md`
(computed after all other files were final; paths relative to the SCA-009
directory):

| File | SHA-256 |
|---|---|
| `ACCEPTANCE_RECORD.md` | `06ab19729420a986ec67993f4b8bed75754db1c09cdfcc0dfe412562da3424bc` |
| `Amendment_Actions.csv` | `07388b81883a0d8758d27b22784fa7420d8fd4be86c8bc8b34c18b106c065901` |
| `Amendment_Preview.md` | `802c2ce92c5a48651f4d06312d4ba26593f0134e3b7c443988e74b79c0e170d4` |
| `Brief.md` | `ad36f600c0bb796e1029c11f0f70370413c7864e343625da3988f9010bb64ffd` (unchanged from Gate 1) |
| `Decision_Log.md` | `d4bb00df938afa2d288a1e0314e736dcfd9213017323b6e84ba25a09f3cf75de` |
| `Handoff_State.md` | `1021f1a052af0956fe2257e012cbf997b9ab04dd0d3ef8812e98892e1e568e24` |
| `Impact_Assessment.md` | `bfa25d898e65b82012b2a93988432a121d5f2b842a5469cf7d53593a1a2ba6d0` (unchanged — Gate-2 ruled-upon artifact) |
| `Impact_Sketch.md` | `7ac40e4e6a94b9276f0bc4a5501e59998c4bbf0db8eb17e51bff538eadfaad77` (unchanged from Gate 1) |
| `Vocabulary_Annex.md` | `f005abbc0d55b047ee7c34628e169aeb3646cd139a9c2c214a33760df4d63c7c` |
| `postimages/ContextBudgetQA.csv` | `84497efb81d4b31b085e016d37a285aed3755be7740a632794e355dad259468d` |
| `postimages/DEL-07-03_CONTEXT.md` | `e69a8d7d832b5af0f15e08c3c242e9d5ace98f26a752ee41fda06855f528ae10` |
| `postimages/DEL-07-03_STATUS.md` | `41039f77e6c0d86dcd013e0c17f09bf51889fbc1aef3a9a68e54d834f8e95596` |
| `postimages/Deliverables.csv` | `f46ea92ab5e5896e86f6b0b8ecbec4e98158886def5d0cf0d7b80d0200f03539` |
| `postimages/SOFTWARE_DECOMP.md` | `a1bf1148b8b96aed83c8a042437b9714d543cb068070662b97397dc700da48a3` |
| `postimages/ScopeLedger.csv` | `97f5a113739e193f07f6d1dfac36f43e08642e12c983f3a3b597a31abe553238` |

**Gate-3 package SHA-256 (over the sorted `hash  filename` lines of the
fifteen files above, filenames relative to the SCA-009 directory):**
`6b1e7080cf4a473db0ccc736088191c00ecc3a017541ee4099451174bf30fa88`

Method: `find . -type f ! -name RUN_SUMMARY.md | sed 's|^\\./||' | sort |
xargs shasum -a 256 | sort` (lexicographic sort of the `hash  filename`
lines), then SHA-256 of that sorted list text.

## Handoff

**NextOwner:** Ryan Tufts. **NextAction:** Gate-3 ruling citing
`Amendment_Preview.md` SHA-256
`802c2ce92c5a48651f4d06312d4ba26593f0134e3b7c443988e74b79c0e170d4`;
approval opens Gate 4 only. Zero applied effect: the live decomposition,
registers, DEL-07-03 metadata, pointers, and all implementation surfaces
are byte-identical to the basis. Standard claim fence applies (F-PIP-2;
DEC-081 claims taxonomy).
