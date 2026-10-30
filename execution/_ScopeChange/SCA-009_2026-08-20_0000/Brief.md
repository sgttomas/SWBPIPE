---
doc_id: PIP-SCA009-BRIEF
doc_kind: scope_change.brief
status: candidate_gate1_not_approved
date: 2026-08-20
amendment_id: SCA-009
decomp_variant: SOFTWARE
durable_basis_commit: 7584de0a8d53d69a135c22fe39a78cb4a30b6cb2
decomposition_revision: "0.11"
---

# Piping SCA-009 brief — interactive model-building operation vocabulary and tool-palette ownership

**State: `CANDIDATE — GATE 1 NOT APPROVED`**

This is a Gate-1 (change intake and validation) candidate package under
`agents/AGENT_SCOPE_CHANGE.md`. No gate has been ruled. Nothing in this
package amends decomposition truth, any deliverable surface, any pointer, or
any downstream state. The active accepted snapshot remains `SCA-008`;
`_LATEST.md` is untouched.

## 1. Change driver — owner request

`CHAT_TRANSCRIPTION — EVIDENCE, NOT RULING`

The owner (Ryan Tufts) initiated this scope-change request in-session on
2026-08-20, verbatim:

> "for the OpenPipeStress app ... we're going to reach a point where all the
> core functionality for piping design and stress analysis is runnable but
> the GUI needs to have an entire set of tools and commands for the human to
> build and modify the model. The agent will of course be able to build the
> model on its own, prompted by the human, but are there any deliverables at
> present that will direct such a user tool palate? I'm just referring to
> what all such existing piping stress analysis applications already have. I
> don't need full parity immediately but all the core functionality should
> exist and be well designed."

And:

> "Can you act as Agent 0 for this work and manage the git coordination?"

Agent 0 (HELP_HUMAN) is executing coordination via dispatched bounded
agents. This transcription is the human-initiated basis required by the
SCOPE_CHANGE "Human-initiated only" invariant. It is evidence of the
request, not a gate ruling.

## 2. Scope question

Does the current decomposition (SOFTWARE_DECOMP.md revision 0.11) contain a
deliverable that (a) enumerates the interactive model-building operation
vocabulary — the command/tool set that incumbent pipe-stress applications
provide for a human to build and modify a piping model — and (b) owns the
organization of the human-facing tool-palette surface that exposes that
vocabulary? If not, which amendment shape closes the gap?

## 3. Evidence-based gap statement

All line references are to
`projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
at revision 0.11, basis commit `7584de0a8d53d69a135c22fe39a78cb4a30b6cb2`,
unless another file is named.

### 3.1 What exists

- **SOW-020** (lines 126 and 485): "The GUI shall provide a 3D centerline
  modeler with model tree and piping component visualization." Mapped to
  PKG-07 / DEL-07-01, DEL-07-02 / OBJ-006.
- **SOW-021** (lines 127 and 486): "The GUI shall provide editors for
  materials, sections, components, load cases, supports, rule packs, and
  private libraries." Mapped to PKG-07 / DEL-07-02, DEL-07-03 / OBJ-006.
- **SOW-060** (lines 166 and 525): GUI state, editing, selection, undo/redo,
  viewport integration, and user workflow **architecture** (PKG-00 /
  DEL-00-05) — "service-command mutation route, and scoped undo/redo";
  architecture basis, not a command inventory.
- **SOW-069** (lines 534, 175): "All GUI and agent edits shall be
  represented as structured model operations that pass schema validation,
  constraint validation, diff preview, and controlled application through
  the model engine." Mapped to PKG-16 / DEL-16-01, DEL-16-02, DEL-16-03.
- **SOW-070** (lines 535, 176): accepted operations preserve history,
  rationale, assumptions, audit metadata. Mapped to DEL-16-03, DEL-16-04.
- **SOW-076** (line 541) / **DEL-07-08** (line 322): design-authoring state
  and comparison workspace — panels, state/run browsers, comparison
  overlays; not a model-building command set.
- **PKG-07** (line 220): "Implements the interactive modeler, editors,
  warning UX, solve-execution UX, and results views."
- **PKG-16** (line 229): "Implements structured model operations,
  validation/diff preview, user acceptance/audit trail, and agent
  rationale/professional-boundary controls."
- **DEL-07-01** (line 315) and its
  `PKG-07_.../1_Working/DEL-07-01_3D viewport and centerline editor/ScopeOfWork.md`
  line 16 (OUT-001): contract covers "command-routed edits" of the
  viewport/centerline surface.
- **DEL-07-02** (line 316) and its `ScopeOfWork.md` line 16 (OUT-001):
  contract covers "command-routed mutations" of the tree/inspector surface.
- **DEL-16-01** (line 407): "Defines add/move/modify/delete/reconnect
  operations for model entities, constraints, loads, supports, and design
  knowledge." Its `ScopeOfWork.md` (lines 53, 152, 266) fixes the operation
  kinds as the **generic type-level enum** `add`, `move`, `modify`,
  `delete`, `reconnect`, `constraint`, `load`, `support`,
  `design_knowledge` (`schemas/model_operation.schema.json`
  `$defs.OperationKind`).
- **OBJ-006** (line 194) and **OBJ-015** (line 203) are the objective homes
  for GUI workflow visibility and controlled model authoring respectively.

### 3.2 What is absent (the gap)

1. **No deliverable enumerates the piping-domain interactive operation
   vocabulary.** DEL-16-01's operation kinds are generic mutation
   categories, not the incumbent-tool command set (e.g., break-pipe /
   insert-in-run, bend insertion with SIF treatment, branch/tee insertion
   with SIF class, spring-hanger selection, node renumbering, run
   copy/rotate/mirror). No SOW row, package scope description, deliverable
   row, or deliverable ScopeOfWork commits any owner to enumerating and
   covering that vocabulary. A whole-file search of `SOFTWARE_DECOMP.md`
   finds no occurrence of "palette", and no "vocabulary" occurrence
   concerns model-building commands (Section 3 Vocabulary Map, line 65 ff.,
   maps product-naming terms only).
2. **No deliverable owns the human tool-palette surface organization.**
   `grep -ri palette` across `execution/**/*.md` returns zero hits outside
   this candidate package. "Toolbar" appears only in implementation run
   records and MEMORY (notably
   `PKG-07_.../DEL-07-06_Accessibility and usability baseline/MEMORY.md`
   lines 78-83: an implemented object-creation toolbar exposing Node, Pipe,
   Support, Component, and Load tools) — i.e., a de-facto partial palette
   exists in the implementation with **no governing decomposition-level
   contract or owning deliverable**.
3. **Adjacent surfaces already record the ownership hole.**
   - `DEL-07-03/_CONTEXT.md` lines 27-29: Context Budget QA **Risk: WATCH**
     — "Confirm scope and split if it expands" (deliverable row line 317:
     "may split later").
   - `DEL-07-03/_STATUS.md ## Remaining`: `DEL-07-03-R-005` (load-case
     editor) and `DEL-07-03-R-006` (support/restraint editor) are preserved
     as "documented GUI absences until an accepted ownership/scope binding
     authorizes" them; `DEL-07-03/ScopeOfWork.md` lines 131-132 and 262
     record the same.
   - `DEL-16-04/_STATUS.md ## Remaining`: the route/support candidate
     "generator currently has no owning deliverable — ownership needs a
     decomposition act".
   - Open issues **OI-012** (line 575, operation granularity) and
     **OI-016** (line 580, agent operation autonomy) touch the operation
     layer but do not cover vocabulary or palette ownership.

**Conclusion (candidate, pending Gate-1 ruling):** the pre-assessed gap is
confirmed against the documents. Command *routing* exists (DEL-07-01/02 +
PKG-16); the command *vocabulary* and the palette *surface organization*
have no owner.

## 4. Candidate reference operation vocabulary

`ASSESSMENT INPUT — NOT YET AUTHORITATIVE.` This list exists so the owner
can rule on coverage; it becomes normative only through the SCOPE_CHANGE
gates. All catalog-shaped data named below (valve/flange weights and
lengths, spring-hanger catalogs) remains user/vendor-supplied private
library data per SOW-009 (line 474), PKG-03 exclusions (line 216), and the
IP boundary; nothing here proposes bundling proprietary catalog content.

1. **Run routing and continuation** — start a run at a node/equipment
   point, route by direction/length or coordinate, continue from an open
   end.
2. **Break / insert-in-run** — split an existing element at a station and
   insert a node or in-line component without re-entering the run.
3. **Bends** — insert bends/elbows with explicit SIF treatment
   (user-entered SIFs and flexibility factors per SOW-007, line 472).
4. **Tees / branch connections** — branch takeoffs with user-entered SIF
   class / reinforcement data (per SOW-008, line 473).
5. **Reducers** — concentric/eccentric transitions between sections.
6. **Valves / flanges as rigid components** — with user/vendor-supplied
   catalog weights, lengths, and centers of gravity (per SOW-009).
7. **Restraint/support set** — anchor, guide, line stop, vertical support,
   spring hanger with catalog selection from private libraries, snubber,
   gaps and friction (support families per SOW-011, nonlinear behavior per
   SOW-012).
8. **Nozzle / equipment boundary conditions** — terminal connections with
   stiffness/displacement boundary data.
9. **Load-case editor with combinations** — primitive cases and unit-aware
   algebra/combinations (SOW-013/SOW-014; note DEL-07-03-R-005 ownership
   absence).
10. **Node renumbering** — controlled renumber/re-sequence with identity
    preservation semantics.
11. **Copy / rotate / mirror of runs** — geometric duplication of model
    subsets with reference remapping.
12. **Unit-system switching** — display/entry unit-system change without
    silent value mutation.
13. **Selection modes** — single, window, chain/run, filter-by-type
    selection consistent with stable selection identity (DEL-07-01
    OUT-001).
14. **Undo/redo as operation inversion** — scoped undo/redo realized as
    inversion/reversal of structured operations, consistent with the
    SCA-001 GUI state basis (SOW-060) and the PKG-16 operation layer.

## 5. Shared routing constraint (both options)

Under either candidate resolution, **the human tool palette and agent
proposals route through the same PKG-16 structured-operation layer**
(SOW-069: all GUI and agent edits are structured model operations;
DEL-16-01 fixes `mutation_route = structured_operations_only` and
`direct_model_mutation_allowed = false`). The vocabulary is a coverage
obligation over that single operation layer. No second mutation route is
proposed, and no change to the PKG-16 acceptance/audit/professional-
boundary controls (DEL-16-02/03/04) is requested by either option.

## 6. Candidate resolutions

Presented neutrally. The choice is reserved to the owner; this package does
not pre-decide.

### Option A — ADD: new deliverable DEL-07-09 "Interactive operation vocabulary and tool palette contract"

One new PKG-07 deliverable (next free ID after DEL-07-08) owning both (a)
the enumerated piping-domain operation vocabulary as a coverage contract
over the PKG-16 operation schema and (b) the tool-palette surface
organization. Requires either a new scope item (next free row: SOW-077) or
an explicit remap extending SOW-020/SOW-021 coverage; OBJ-006 and likely
OBJ-015 mappings; a context row; PREPARATION scaffold.

- **For:** single named owner for the currently unowned question; keeps
  DEL-16-01 bounded to schema mechanics; consistent with the DEL-07-03
  WATCH advice to split rather than expand editor scope; gives the
  DEL-07-03-R-005/R-006 and DEL-16-04-generator ownership residuals a
  candidate landing zone (whether they move is a separate owner decision).
- **Against:** topology change (new deliverable, possibly new SOW row) with
  the attendant DAG/dependency/audit ripple; boundary must be drawn
  carefully against DEL-07-01/02/03 (which own the surfaces the palette
  would organize) or the new deliverable becomes a coordination shim;
  vocabulary (domain semantics) and palette (UX organization) are
  different layers and housing both in one UX_UI_SLICE-type deliverable
  concentrates cross-layer contract risk.

### Option B — MODIFY: amend DEL-16-01 plus DEL-07-01/DEL-07-02

Expand DEL-16-01's ScopeOfWork with a piping-domain vocabulary-coverage
obligation (the schema must express the ratified vocabulary), and amend
DEL-07-01 and DEL-07-02 ScopeOfWork to own the palette surface organization
across their respective surfaces (viewport command set; tree/inspector
command set). No new entities.

- **For:** zero topology change; vocabulary lives beside the operation
  schema that must express it; palette obligations land in the deliverables
  that already own the affected surfaces and "command-routed" language.
- **Against:** amends N existing deliverable contracts (at minimum
  DEL-16-01, DEL-07-01, DEL-07-02; likely a DEL-07-03 boundary note);
  DEL-16-01 is a context-M DATA_MODEL_CHANGE and a vocabulary obligation
  grows it; DEL-07-01 (L) and DEL-07-02 (M) carry active Remaining
  residuals and grow further; runs against the DEL-07-03 WATCH advice
  direction (expanding in place rather than splitting); palette ownership
  splits across two deliverables with no single surface-organization owner.

The owner may also direct a variant (e.g., vocabulary obligation in PKG-16
with a palette-only new deliverable); no variant is developed here.

## 7. Parsed candidate actions (for Gate-1 validation)

| Option | ActionType | EntityType | EntityID | RequestedChange | AffectedSections |
|---|---|---|---|---|---|
| A | ADD | DELIVERABLE | DEL-07-09 (proposed; ID free at rev 0.11) | New PKG-07 deliverable: interactive operation vocabulary + tool palette contract | Deliverables (PKG-07), Scope Ledger, Objectives mapping, Packages (PKG-07 assigned scope), Decision Log |
| A | ADD or MODIFY | SCOPE_ITEM | SOW-077 (proposed) or SOW-020/SOW-021 remap | New scope row or explicit coverage extension | Scope register (section 4), Scope Ledger |
| B | MODIFY | DELIVERABLE | DEL-16-01 | Add piping-domain vocabulary-coverage obligation | DEL-16-01 ScopeOfWork/_CONTEXT; Scope Ledger notes |
| B | MODIFY | DELIVERABLE | DEL-07-01 | Add palette surface-organization ownership (viewport) | DEL-07-01 ScopeOfWork/_CONTEXT |
| B | MODIFY | DELIVERABLE | DEL-07-02 | Add palette surface-organization ownership (tree/inspector) | DEL-07-02 ScopeOfWork/_CONTEXT |

Validation notes: DEL-07-09 does not exist at rev 0.11 (PKG-07 ends at
DEL-07-08, line 322); SOW rows end at SOW-076 (line 541); both proposed IDs
are format-valid and collision-free. No parent partition is removed,
merged, split, or reclassified by either option.

## 8. Decision points reserved to the owner

1. Gate-1 ruling: is the parsed request (Section 7) what the owner intends?
2. Option A vs Option B vs owner-directed variant.
3. Ratification, amendment, or rejection of the Section 4 reference
   vocabulary as the normative coverage list.
4. Palette-surface ownership shape (single owner vs split).
5. Disposition of the adjacent ownership residuals (DEL-07-03-R-005/R-006;
   DEL-16-04 generator ownership): folded into this SCA or left to
   separate acts.
6. If Option A: new SOW row vs SOW-020/SOW-021 remap.

Standard claim fence applies (F-PIP-2; DEC-081 claims taxonomy).
