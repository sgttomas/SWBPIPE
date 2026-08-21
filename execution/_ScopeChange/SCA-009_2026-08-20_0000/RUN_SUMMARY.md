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

---

# Gate-3 approval and whitespace remediation

The owner ruled Gate 3 `APPROVED` (verbatim in `ACCEPTANCE_RECORD.md`) on
`Amendment_Preview.md` SHA-256 `802c2ce9…` at branch commit `d50e72c4b`,
authorizing application of exactly the six preimage → postimage pairs,
`DEC-094`, and no other surface. After the ruling, the governance
harness's candidate-whitespace check
(`tools/validation/validate_candidate_whitespace.py`, invoked by the
`.github/workflows/governance-harness.yml` "Candidate whitespace" step)
failed on the pushed range with 18 trailing-whitespace findings, all
blank/context lines of the diffs embedded in `Amendment_Preview.md`. The
failure was reproduced locally over `origin/main...HEAD` and remediated in
this commit by stripping trailing whitespace from those 18 lines — a
whitespace-only change; the six pair hashes and every `postimages/` byte
are unchanged. Post-cleanup `Amendment_Preview.md` SHA-256:
`44eaf63ab9de9a4703972acdedc39b65a760dce4f1b2b566134e861512a71eab`. The
ruled-upon commit `d50e72c4b` remains intact in history; this tranche and
the Gate-4 application land as new commits on top.

## Refreshed package integrity (post-remediation, pre-application)

| File | SHA-256 |
|---|---|
| `ACCEPTANCE_RECORD.md` | `5ff23893cf02c879b1ba7ce795b4fc28ce63d1770774f35015df38981ad049dd` |
| `Amendment_Actions.csv` | `07388b81883a0d8758d27b22784fa7420d8fd4be86c8bc8b34c18b106c065901` |
| `Amendment_Preview.md` | `44eaf63ab9de9a4703972acdedc39b65a760dce4f1b2b566134e861512a71eab` |
| `Brief.md` | `ad36f600c0bb796e1029c11f0f70370413c7864e343625da3988f9010bb64ffd` |
| `Decision_Log.md` | `3c479488ca402f6cf2814199f845672c047999596a7e71738b68a8d7781aa195` |
| `Handoff_State.md` | `1021f1a052af0956fe2257e012cbf997b9ab04dd0d3ef8812e98892e1e568e24` |
| `Impact_Assessment.md` | `bfa25d898e65b82012b2a93988432a121d5f2b842a5469cf7d53593a1a2ba6d0` |
| `Impact_Sketch.md` | `7ac40e4e6a94b9276f0bc4a5501e59998c4bbf0db8eb17e51bff538eadfaad77` |
| `Vocabulary_Annex.md` | `f005abbc0d55b047ee7c34628e169aeb3646cd139a9c2c214a33760df4d63c7c` |
| `postimages/ContextBudgetQA.csv` | `84497efb81d4b31b085e016d37a285aed3755be7740a632794e355dad259468d` |
| `postimages/DEL-07-03_CONTEXT.md` | `e69a8d7d832b5af0f15e08c3c242e9d5ace98f26a752ee41fda06855f528ae10` |
| `postimages/DEL-07-03_STATUS.md` | `41039f77e6c0d86dcd013e0c17f09bf51889fbc1aef3a9a68e54d834f8e95596` |
| `postimages/Deliverables.csv` | `f46ea92ab5e5896e86f6b0b8ecbec4e98158886def5d0cf0d7b80d0200f03539` |
| `postimages/SOFTWARE_DECOMP.md` | `a1bf1148b8b96aed83c8a042437b9714d543cb068070662b97397dc700da48a3` |
| `postimages/ScopeLedger.csv` | `97f5a113739e193f07f6d1dfac36f43e08642e12c983f3a3b597a31abe553238` |

**Package SHA-256 (same sorted-lines method, fifteen files):**
`5d19c18461326cc8084b0d523afe31d73985987fad2466e2a8828d8a01e2549a`

All `postimages/` hashes are unchanged from the Gate-3 table — the
approved application targets are byte-identical.

---

# Gate-4 tranche — application of the approved amendment to live surfaces

**Current state:** `GATE 4 APPLIED — GATE 5 VALIDATION AND CLOSURE PENDING`

## Execution

Executed on 2026-08-20 by the Agent 0-dispatched bounded generalist under
the Gate-3 ruling's own authorization ("Apply exactly the six
preimage→postimage pairs recorded there, DEC-094, and no other surface"),
as a new commit on top of the intact ruled-upon commits `d50e72c4b` and the
whitespace-remediation commit. Each approved `postimages/` file was copied
byte-for-byte onto its live path. Nothing else was touched: no pointer
(`_Decomposition/_LATEST.md`, `_ScopeChange/_LATEST.md` unchanged — Gate 5,
pointer-last), no other deliverable folder, no implementation surface.

## Application proof

Live-file SHA-256 after application versus the approved postimage hash
from the `Amendment_Preview.md` table (paths relative to
`projects/chirality-piping/`):

| Live path | Live SHA-256 after application | Approved postimage hash | Match |
|---|---|---|---|
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | `a1bf1148b8b96aed83c8a042437b9714d543cb068070662b97397dc700da48a3` | `a1bf1148…da48a3` | YES |
| `docs/_Registers/ScopeLedger.csv` | `97f5a113739e193f07f6d1dfac36f43e08642e12c983f3a3b597a31abe553238` | `97f5a113…553238` | YES |
| `docs/_Registers/Deliverables.csv` | `f46ea92ab5e5896e86f6b0b8ecbec4e98158886def5d0cf0d7b80d0200f03539` | `f46ea92a…f03539` | YES |
| `docs/_Registers/ContextBudgetQA.csv` | `84497efb81d4b31b085e016d37a285aed3755be7740a632794e355dad259468d` | `84497efb…459468d` | YES |
| `execution/PKG-07_…/DEL-07-03_…/_STATUS.md` | `41039f77e6c0d86dcd013e0c17f09bf51889fbc1aef3a9a68e54d834f8e95596` | `41039f77…e95596` | YES |
| `execution/PKG-07_…/DEL-07-03_…/_CONTEXT.md` | `e69a8d7d832b5af0f15e08c3c242e9d5ace98f26a752ee41fda06855f528ae10` | `e69a8d7d…528ae10` | YES |

## Post-application verification

- Live `SOFTWARE_DECOMP.md`: exactly 1 `DEL-07-09` row, exactly 2
  `SOW-077` rows (the §4 register / §9 ledger pair), exactly 1 `DEC-094`
  row; revision 0.12; telemetry 77/102, `S=9, M=69, L=24, XL=0`.
- Live registers parsed with Python `csv`: `ScopeLedger.csv` 77 data rows,
  `Deliverables.csv` 102, `ContextBudgetQA.csv` 102 — uniform column
  widths (10/10/6), exactly one new target row each.
- `tools/validation/validate_candidate_whitespace.py --base-ref
  origin/main` over the full branch range including the applied files:
  `PASS: candidate whitespace is clean`.
- No `_LATEST.md` in the diff; `git status` showed exactly the six live
  files plus the SCA-009 record updates.

## Package integrity (Gate-4 tranche, final for this commit)

Per-file SHA-256 of every package file except this `RUN_SUMMARY.md`:

| File | SHA-256 |
|---|---|
| `ACCEPTANCE_RECORD.md` | `e43fb752d0927cd935e9d1c0c0df16a7c37c732ac7f530cb9eb51d179b5dae03` |
| `Amendment_Actions.csv` | `07388b81883a0d8758d27b22784fa7420d8fd4be86c8bc8b34c18b106c065901` |
| `Amendment_Preview.md` | `44eaf63ab9de9a4703972acdedc39b65a760dce4f1b2b566134e861512a71eab` |
| `Brief.md` | `ad36f600c0bb796e1029c11f0f70370413c7864e343625da3988f9010bb64ffd` |
| `Decision_Log.md` | `701112eff7a714b0c9972eeb21c042a0d9cf3c3d8ad8b78658d839f0b9160001` |
| `Handoff_State.md` | `41f92f956b61684e0da68978473696be9c0f10530b660140c4f2e3612a8148cd` |
| `Impact_Assessment.md` | `bfa25d898e65b82012b2a93988432a121d5f2b842a5469cf7d53593a1a2ba6d0` |
| `Impact_Sketch.md` | `7ac40e4e6a94b9276f0bc4a5501e59998c4bbf0db8eb17e51bff538eadfaad77` |
| `Vocabulary_Annex.md` | `f005abbc0d55b047ee7c34628e169aeb3646cd139a9c2c214a33760df4d63c7c` |
| `postimages/ContextBudgetQA.csv` | `84497efb81d4b31b085e016d37a285aed3755be7740a632794e355dad259468d` |
| `postimages/DEL-07-03_CONTEXT.md` | `e69a8d7d832b5af0f15e08c3c242e9d5ace98f26a752ee41fda06855f528ae10` |
| `postimages/DEL-07-03_STATUS.md` | `41039f77e6c0d86dcd013e0c17f09bf51889fbc1aef3a9a68e54d834f8e95596` |
| `postimages/Deliverables.csv` | `f46ea92ab5e5896e86f6b0b8ecbec4e98158886def5d0cf0d7b80d0200f03539` |
| `postimages/SOFTWARE_DECOMP.md` | `a1bf1148b8b96aed83c8a042437b9714d543cb068070662b97397dc700da48a3` |
| `postimages/ScopeLedger.csv` | `97f5a113739e193f07f6d1dfac36f43e08642e12c983f3a3b597a31abe553238` |

**Package SHA-256 (same sorted-lines method, fifteen files):**
`ce645fae06dd31952bb722669e7ac864b99e5dacb5e52c09d84a8939fdf95ed9`

(One in-tranche correction before this commit finalized: the Gate-4 edits
had left a blank line at the end of `ACCEPTANCE_RECORD.md`, which the
candidate-whitespace check flags as "new blank line at EOF"; it was
removed and the two affected hashes above reflect the corrected file.)

## Handoff

**NextOwner:** Ryan Tufts (after Gate-5 evidence assembly).
**NextAction:** Gate 5 — pre-change audit baseline, snapshot completion
(Pre/Post_Change_Coverage, supersession artifacts, Propagation_Plan
consolidation), post-change AUDIT_DECOMP, pointer advances pointer-last,
Piping dev-loop coordination notice, owner closure confirmation. The
decomposition is revision 0.12 on this branch while both `_LATEST.md`
pointers still cite the 0.11/SCA-008 state — the expected mid-amendment
posture. Standard claim fence applies (F-PIP-2; DEC-081 claims taxonomy).

---

# Gate-5 tranche — validation and closure candidate

**Current state:** `GATE 5 CANDIDATE — CLOSURE RULING PENDING`

## Execution

Staged on 2026-08-20 by the Agent 0-dispatched bounded generalist on the
owner's direction ("proceed to Gate 5. CI is still running, which is
fine."), as one new commit on top of the Gate-4 application commit
`8d622e4c8`. Pointer advances and the final `CLOSED` state are reserved
for a separate post-ruling closing commit; both `_LATEST.md` files are
untouched by this tranche.

## What was produced

- `Pre_Change_Coverage.json` / `Post_Change_Coverage.json`: synthesized
  deterministic baselines computed from the actual files (SCA-008 schema
  mirrored where deterministically computable; audit-only fields marked
  `NOT_RUN`). Pre at basis `89758a326` (rev 0.11): **76 SOW / 18 PKG /
  101 DEL / 101 context rows / 18 OBJ, `S=9, M=69, L=23, XL=0`**. Post at
  HEAD applied state (rev 0.12): **77 / 18 / 102 / 102 / 18,
  `S=9, M=69, L=24, XL=0`**. Declared == found on both sides;
  forward/reverse/objective coverage 100% both sides; no mismatch against
  the approved deltas.
- `Supersession_Delta.csv` (1 row — the DEL-07-03 R-005/R-006
  ownership-landing re-pointing, old Remaining wording → re-pointed
  wording, per the SCA-008 schema) and `Supersession_Map.csv` (cumulative:
  SCA-005's 3 active rows + SCA-009's 1). New entities DEL-07-09 /
  SOW-077 / DEC-094 are structural additions with no superseded
  predecessor and carry no rows. `Amendment_Actions.csv` row 11 corrected
  `SupersessionBindingPresent NO → YES` to match (disclosed in
  `Validation_Record.md` §10).
- `Propagation_Plan.md`: applied-vs-remaining write scope (closing commit
  = the two pointer advances only) and the downstream-rerun obligation
  table (DAG-008 REBUILD; formal pre/post AUDIT_DECOMP; targeted
  RECONCILIATION refresh; DEL-07-09 Dependencies extract after
  PREPARATION scaffold; PREPARATION scaffold; estimate/schedule
  ADVISORY_STALE) — each with owner instrument and trigger; none executed
  by SCA-009.
- Coordination notice (the one live-surface write of this tranche):
  `execution/_Coordination/NOTICE_2026-08-20_PIPING_SCA-009_DEL-07-09_VOCABULARY_PALETTE.md`
  on the Piping loop's own coordination surface (SCA-009 is
  Piping-internal; no Root/App/Tier-0 notice required). Coordination, not
  authority.
- `Validation_Record.md`: validator inventory (what exists, what ran,
  what is N/A and why) and all deterministic check outputs — candidate
  whitespace, decomposition-register estate (exit 1 with 2090 ERROR
  findings, identical at basis and HEAD, all pre-existing in the
  untouched `Dependencies.csv` family; the exit code is not a pass/fail
  signal for this candidate), DEC-081 claims-language `VALID`, coverage
  computation, ID-uniqueness sweep, CSV parses, six-file application
  re-proof, pointer non-advance.
- `ACCEPTANCE_RECORD.md` Gate-5 section, `Decision_Log.md` rows, and the
  full SCA-008-style `Handoff_State.md` (closure verdict
  `PENDING OWNER RULING`).

## Package integrity (Gate-5 tranche, final for this commit)

Per-file SHA-256 of every package file except this `RUN_SUMMARY.md`:

| File | SHA-256 |
|---|---|
| `ACCEPTANCE_RECORD.md` | `cb2acd242a4d90d033a9488434d34a5262f0353765381651a417216241de1075` |
| `Amendment_Actions.csv` | `1a6df9177205e227b67ac9e2c7cf10b185ec88a323cbd8bfb03781b41232b37c` |
| `Amendment_Preview.md` | `44eaf63ab9de9a4703972acdedc39b65a760dce4f1b2b566134e861512a71eab` |
| `Brief.md` | `ad36f600c0bb796e1029c11f0f70370413c7864e343625da3988f9010bb64ffd` |
| `Decision_Log.md` | `42e0875ea9af837198401f64546233d69a172065f49ba85b77c71d820c149c91` |
| `Handoff_State.md` | `3fdc3203b507d198d2682eed6280d497aeee81b43690228b7f2cf7bf5b19c891` |
| `Impact_Assessment.md` | `bfa25d898e65b82012b2a93988432a121d5f2b842a5469cf7d53593a1a2ba6d0` |
| `Impact_Sketch.md` | `7ac40e4e6a94b9276f0bc4a5501e59998c4bbf0db8eb17e51bff538eadfaad77` |
| `Post_Change_Coverage.json` | `b28a13fbdfcdf89be13e65c72293867388fac1c1c949e3c6d7d6a6b070b9e565` |
| `Pre_Change_Coverage.json` | `664800428731ada0bf70bd0e9179d428991d0901032fd63f91307283d32128c0` |
| `Propagation_Plan.md` | `62abeb91a3dee79c6064075a8f7de025e3b38c7158217d3caa01ef32c82ac93a` |
| `Supersession_Delta.csv` | `d0714e661b03c0236cc33cccf6a2732dd2ac49a240a3776f24c26a1fb2d8ed34` |
| `Supersession_Map.csv` | `29a4e4d96f0f27117a1ceadee3bd0d470ec6668ac58c35180839e33c181fc2cc` |
| `Validation_Record.md` | `adb5f900e929924ea337fd9a56d17570f61ba5f1f740c605c948de9e803ba4d8` |
| `Vocabulary_Annex.md` | `f005abbc0d55b047ee7c34628e169aeb3646cd139a9c2c214a33760df4d63c7c` |
| `postimages/ContextBudgetQA.csv` | `84497efb81d4b31b085e016d37a285aed3755be7740a632794e355dad259468d` |
| `postimages/DEL-07-03_CONTEXT.md` | `e69a8d7d832b5af0f15e08c3c242e9d5ace98f26a752ee41fda06855f528ae10` |
| `postimages/DEL-07-03_STATUS.md` | `41039f77e6c0d86dcd013e0c17f09bf51889fbc1aef3a9a68e54d834f8e95596` |
| `postimages/Deliverables.csv` | `f46ea92ab5e5896e86f6b0b8ecbec4e98158886def5d0cf0d7b80d0200f03539` |
| `postimages/SOFTWARE_DECOMP.md` | `a1bf1148b8b96aed83c8a042437b9714d543cb068070662b97397dc700da48a3` |
| `postimages/ScopeLedger.csv` | `97f5a113739e193f07f6d1dfac36f43e08642e12c983f3a3b597a31abe553238` |

**Package SHA-256 (same sorted-lines method, twenty-one files):**
`0b07defec01a794659e891a0ff792aea615be442673e97b70b3ef453bd32e8fa`

(Amend disclosure: this tranche's commit supersedes the withdrawn
first-form commit `ff91cee32`, which was amended — it was unpushed and no
ruling cites it — to correct an honesty defect found in review: the
register-estate validator's outcome had been recorded as "exit 0
(report-only)" when the tool in fact exited 1 on its 2090 pre-existing
findings, identically at the basis; the exit status had been read from a
`| tail` pipeline. The correction, its details, and the sweep of every
echo of the claim are in `Validation_Record.md` §3; the stale Gate-1-era
first Notes bullet in `Decision_Log.md` was refreshed in the same amend.)

---

# Closing tranche — Gate-5 acceptance, closure, and pointer advance

**Current state:** `CLOSED_FOR_SCOPE_CHANGE_ONLY`

## Execution

Executed on 2026-08-20 by the Agent 0-dispatched bounded generalist under
the owner's Gate-5 closure ruling (verbatim in `ACCEPTANCE_RECORD.md`):
Gate-5 validation `ACCEPTED` on package SHA-256 `0b07defe…` at the intact
ruled-upon commit `a08701951`; amendment `CLOSED`; both pointer advances
and the PR #593 merge authorized. This closing commit lands on top of
`a08701951` and performs exactly the authorized acts.

## What was written

- `execution/_Decomposition/_LATEST.md` → revision 0.12 (existing
  one-line format preserved).
- `execution/_ScopeChange/_LATEST.md` → SCA-009
  (`SCA-009_2026-08-20_0000/`, status accepted, created 2026-08-20;
  SCA-008-format field table with the closure summary; historical-residue
  paragraph: SCA-008 remains immutable and complete, superseded only as
  the active current snapshot). Staged pointer-last.
- `ACCEPTANCE_RECORD.md` (verbatim Gate-5 ruling; state
  `CLOSED_FOR_SCOPE_CHANGE_ONLY`), `Decision_Log.md` (Gate-5
  `ACCEPTED — CLOSED` and pointer-advance rows), `Handoff_State.md`
  (final closure handoff), and this `RUN_SUMMARY.md` (written last).

Nothing else was touched: the two pointer files are the only live-surface
writes; no deliverable folder, register, implementation tree, or
historical snapshot changed in this commit.

## Post-closure obligations

Per `Propagation_Plan.md` and `Handoff_State.md`: PR #593 merge (Agent 0,
post-CI, per the ruling); then DAG-008 rebuild, formal AUDIT_DECOMP
pre/post comparison, targeted RECONCILIATION refresh, PREPARATION
scaffold for DEL-07-09, and the DEL-07-09 `Dependencies.csv` extract —
each owned by its own instrument; none executed by SCA-009.

## Final package integrity

Per-file SHA-256 of every package file except this `RUN_SUMMARY.md`:

| File | SHA-256 |
|---|---|
| `ACCEPTANCE_RECORD.md` | `0e4b789920f9a0f8f6f3776555a44352de051f4542e750aa9bc6e8604a785771` |
| `Amendment_Actions.csv` | `1a6df9177205e227b67ac9e2c7cf10b185ec88a323cbd8bfb03781b41232b37c` |
| `Amendment_Preview.md` | `44eaf63ab9de9a4703972acdedc39b65a760dce4f1b2b566134e861512a71eab` |
| `Brief.md` | `ad36f600c0bb796e1029c11f0f70370413c7864e343625da3988f9010bb64ffd` |
| `Decision_Log.md` | `5179da7e4ab566d7f20d42611119e0b2346e0f82f18ac0378f2ad8afc77ff98d` |
| `Handoff_State.md` | `92df402801ebe6a451f5fb068c56703f70db7f29cadbfae5a5f557ff9e024d7f` |
| `Impact_Assessment.md` | `bfa25d898e65b82012b2a93988432a121d5f2b842a5469cf7d53593a1a2ba6d0` |
| `Impact_Sketch.md` | `7ac40e4e6a94b9276f0bc4a5501e59998c4bbf0db8eb17e51bff538eadfaad77` |
| `Post_Change_Coverage.json` | `b28a13fbdfcdf89be13e65c72293867388fac1c1c949e3c6d7d6a6b070b9e565` |
| `Pre_Change_Coverage.json` | `664800428731ada0bf70bd0e9179d428991d0901032fd63f91307283d32128c0` |
| `Propagation_Plan.md` | `62abeb91a3dee79c6064075a8f7de025e3b38c7158217d3caa01ef32c82ac93a` |
| `Supersession_Delta.csv` | `d0714e661b03c0236cc33cccf6a2732dd2ac49a240a3776f24c26a1fb2d8ed34` |
| `Supersession_Map.csv` | `29a4e4d96f0f27117a1ceadee3bd0d470ec6668ac58c35180839e33c181fc2cc` |
| `Validation_Record.md` | `adb5f900e929924ea337fd9a56d17570f61ba5f1f740c605c948de9e803ba4d8` |
| `Vocabulary_Annex.md` | `f005abbc0d55b047ee7c34628e169aeb3646cd139a9c2c214a33760df4d63c7c` |
| `postimages/ContextBudgetQA.csv` | `84497efb81d4b31b085e016d37a285aed3755be7740a632794e355dad259468d` |
| `postimages/DEL-07-03_CONTEXT.md` | `e69a8d7d832b5af0f15e08c3c242e9d5ace98f26a752ee41fda06855f528ae10` |
| `postimages/DEL-07-03_STATUS.md` | `41039f77e6c0d86dcd013e0c17f09bf51889fbc1aef3a9a68e54d834f8e95596` |
| `postimages/Deliverables.csv` | `f46ea92ab5e5896e86f6b0b8ecbec4e98158886def5d0cf0d7b80d0200f03539` |
| `postimages/SOFTWARE_DECOMP.md` | `a1bf1148b8b96aed83c8a042437b9714d543cb068070662b97397dc700da48a3` |
| `postimages/ScopeLedger.csv` | `97f5a113739e193f07f6d1dfac36f43e08642e12c983f3a3b597a31abe553238` |

**Final package SHA-256 (same sorted-lines method, twenty-one files):**
`f0e3bbdf272e354ffae621c47df3b676c9ac35949e6e95c917031056658e4eba`

The Gate-5 ruled-upon package hash `0b07defe…` identifies the candidate as
ruled at commit `a08701951`; this final hash covers the same files after
the closure transcription (only `ACCEPTANCE_RECORD.md`,
`Decision_Log.md`, and `Handoff_State.md` changed).

## Handoff

**State:** `CLOSED_FOR_SCOPE_CHANGE_ONLY`. **NextOwner:** Agent 0
(HELP_HUMAN) for the authorized PR #593 merge post-CI; thereafter the
owning instruments for the post-closure obligations. Standard claim fence
applies (F-PIP-2; DEC-081 claims taxonomy).

## Handoff

**NextOwner:** Ryan Tufts. **NextAction:** the Gate-5 closure ruling on
this candidate. On acceptance, the closing commit advances
`_Decomposition/_LATEST.md` then `_ScopeChange/_LATEST.md`
(pointer-last), transcribes the ruling, and sets
`CLOSED_FOR_SCOPE_CHANGE_ONLY`. Downstream reruns remain explicit
obligations per `Propagation_Plan.md`. Standard claim fence applies
(F-PIP-2; DEC-081 claims taxonomy).
