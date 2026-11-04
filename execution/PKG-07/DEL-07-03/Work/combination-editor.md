# Combination editor: unsigned bases, typed nested operands and the seismic template

RT-A4 and RT-A9 on the desktop, serving RT-G5 and RT-G2. The governing text is DEL-05-02 Work
`absolute-sum-srss-combinations.md` §2 (members), §3, §5.1 (κ), §5.2 (nonlinear supports, OQ-1)
and §6 (refusals); the applier and the solve are as built there (§10). This note describes the
implementation; it records no backcheck or acceptance.

Sources: `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx`,
`combinationAuthoring.ts`, `seismicTemplate.ts`, `SeismicTemplateSection.tsx`.

## 1. Create combination

- **Nothing is chosen for the person** (review r1 of #114). Every operand, factor and envelope
  mode in the create editor, in a new term row, in add-term on an existing combination and
  in the seismic template starts blank, and blank means not chosen. Queue stays disabled and
  the preview names what is missing, for example "term 1: not chosen: its load case",
  "not chosen: the minuend" or "not entered: the factor". This removes defaults main's
  editor had: the first load case with factor 1 as the first mechanics term; the first and
  second cases as minuend and subtrahend; the first case as envelope operand, with mode
  `max_abs`; and, in add-term, the first unused case with factor 1. An operand that leaves
  the model is cleared, never replaced.
- **Bases.** `mechanics`, `result_state_subtraction`, `range_envelope`, `absolute_sum`, `srss`
  (the applier's closed set), in the create editor and the basis edit.
- **Terms** (`mechanics`, `absolute_sum`, `srss`). An ordered list of term rows. Each row has:
  - an operand kind, **Load case** or **Combination**;
  - an operand picker of that kind, and a factor;
  - Up, Down and Remove. The last term cannot be removed.

  A load-case term writes exactly `{"load_case": id, "factor": f}`; a combination term
  writes exactly `{"combination": id, "factor": f}`, in the order shown. Changing a row's
  kind clears its operand. Nothing else is written on these bases: no `minuend_id`,
  `subtrahend_id`, `operand_ids`, `mode` or `difference_basis`. Queueing needs at least one
  term, a chosen operand and an entered finite factor in each, and no repeated operand (same
  kind and id). The first problem is
  named in the preview line, for example "term 2: repeats load case …".
- **Bare references** (`minuend_id`, `subtrahend_id`, `operand_ids`). The pickers offer load
  cases and combinations, labelled by kind. The editor shows each reference's resolved
  target: the load case when one has the id, otherwise the combination (§2, r1 F3). A
  combination whose id a load case also has is not offered to a bare reference, since the
  reference would name the load case. It can be named through a combination term. The
  stored list shows the resolution of each stored reference, and no reference is rewritten.
- **Operating minus sustained** (`difference_basis`): offers load cases only. The reason is
  shown as `LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE`.
- **Known gates, stated in place.**
  - On an unsigned draft: the row text (unsigned, "not an equilibrium state", formed from
    rows with nonlinear supports) and the §5.1 κ gate with both codes.
  - The load cases the draft names directly (κ = 1).
  - The model's constant-effort supports, by id. Whether the solve consumes one is the
    solve's decision.
  - On a `mechanics` draft in a model with nonlinear supports: the OQ-1 rule and
    `NONLINEAR_COMBINATION_REQUIRES_SOLVE`.

  The desktop computes no κ. It states κ only where §5.1 defines it outright: 1 for a load
  case and 0 for a load case minus a load case.

## 2. Create combination term

The term editor has the same typed operand choice. In the combination picker, some options
are disabled, each with its reason in its label:

- the target itself: self-reference, `LOAD_COMBINATION_OPERAND_CYCLE`;
- every combination that already reaches the target through its terms or its bare
  references that resolve to combinations: it would close a cycle, and the path is shown;
- a combination already a term of the target.

## 3. Editing stored records

There is no whole-record combination update in the applier. Edits are therefore the existing
field-level operations: basis (`update_load` `basis`), term factor (`terms.N.factor`) and
term creation. Each writes only its own member, so untouched members, their values and
references are kept. The basis editor lists stored members it does not edit as "Retained as
stored", for example `minuend_id`, `difference_basis`, `subtrahend_support_state` and
`notes`. The engine returns its applied document with sorted keys. Key order is the engine's.

## 3a. Deleting combinations and terms

Both delete buttons work on every closed basis, on the applier contract of #115 (DEL-05-02
Work `absolute-sum-srss-combinations.md` §10.5). Sources: `load-cases/combinationDelete.ts`
and `LoadCaseManagerPanel.tsx`.

- **The `before` value comes only from the stored record or term.** It is never coerced to
  the flat display.
  - `delete_combination`: the historical display `"<id>; <label>; basis=<basis>;
    terms=<lc> x <factor>; …"` (or `terms=none`) when the stored `terms` is an array of flat
    `{load_case: non-empty string, factor: finite number}` terms, empty included, whatever
    the basis. Factors are written as the engine's `display_number` writes them: an integer
    below 10^15 in magnitude, otherwise Rust's plain shortest decimal, for example
    `0.0000001`. Otherwise, with no `terms` or any non-flat term, the before is the stored
    record's canonical JSON, from the wasm engine's `canonicalJsonString`.
  - `delete_combination_term` (`terms.<i>`): `"<lc> x <factor>"` for a flat term, otherwise
    the stored term's canonical JSON.
  - The preview shows the before value. A canonical value is computed by the engine, so the
    preview says so until it is ready.
- **Checked before queueing.** Each delete is checked by the operation engine
  (validate-only, the shared engine-checked queue) and queued for Review/Apply only when
  nothing blocks it. A refusal is listed in place by code, message and references, and
  nothing is queued.
- **References, before the attempt.** When other combinations name the selected one, the
  editor lists every path, as the engine does: `<id>.terms.<i>` (any factor, 0 included),
  `<id>.minuend_id`, `<id>.subtrahend_id` and `<id>.operand_ids.<i>`. It says the engine
  refuses the delete while they do (`OP-COMBINATION-DELETE-REFERENCED`) and that nothing is
  deleted with it. For a three-axis static seismic template set it says "delete top-down:
  the unsigned combination first". Otherwise it names the referencing combinations to
  delete or change first. The button stays enabled, so an attempt shows the engine's
  refusal by name. Nothing cascades: the editor never deletes a referencer.
- **Undo.** Session Undo restores the document exactly from its checkpoint. The engine's
  inverse is `create_combination` at `combinations[i]`, or `create_combination_term` at
  `terms.<i>`, with the stored record's or term's own JSON. A record no create authors (a
  termless mechanics record, or one with `minuend_final`) deletes, but create refuses its
  restore by name. Session Undo still restores it.

## 4. Three-axis static seismic template

§5.1's "(base + EQx) − base" is a subtraction of two load-case states, each with κ = 1.

- **Choices.** The person chooses a base case and an explicit, complete axis-state case for
  X, Y and Z: the base's actions plus that axis's excitation. All four cases are distinct.
  The person also chooses `srss` or `absolute_sum`; no basis is preselected. Four free
  combination ids are proposed (`combination:SEISMIC-X-n` … `combination:SEISMIC-n`) and
  may be changed. An id that exists, or that a load case has, is refused in place.
- **Factors.** The person enters each difference's factor in the unsigned combination; none
  is proposed.
- **Writes.** Through `create_combination`, in order:
  - one `result_state_subtraction` per axis, axis state minus base, κ = 0, with
    `terms: []`, and no `difference_basis` (its gate G-1 is a thermal comparison) or
    `subtrahend_support_state`;
  - then the unsigned combination over the three differences, as typed combination
    terms with the entered factors.

  `minuend_final` is never offered.
- **Preview before queueing.** It explains the base and axis states, κ = 0 per difference,
  and why this is not "base case + EQ-only case". It lists the four records and their
  exact JSON. It states the §5.1 unsigned-operand gate, naming both codes and the model's
  constant-effort supports, and §5.2: the operands are derived differences, not an
  equilibrium load state, and the model's nonlinear supports are named with OQ-1. It also
  states the boundary: no load composition, no gravity or excitation value, and no code,
  spectrum or modal claim.
- **Check and queue.** The operation engine validates the four creates as one sequence
  (`validateOperationBatch`, validate-only, on the current model and hash). Its findings
  are listed in place by code (`EngineFeedback`), and a blocking finding queues nothing.
  Otherwise the four are queued, in order, as single changes for Review/Apply. The preview
  says: apply them together with "Review all queued changes as one batch" (G9) to apply all
  four or none; applied singly, each applies on its own, so one already applied stays
  applied if a later one is refused. Applied before the differences, the unsigned
  combination is refused by name (`OP-COMBINATION-TERM-LOAD-NOT-FOUND`).
- **A draft changed during the check.** The inputs are disabled while the engine checks.
  If the draft still changes before the check returns, the check is invalidated and
  nothing is queued; the person is told to check again. What is queued is always the draft
  the preview shows.
- **Axis-state cases.** The template creates no load case. Its note directs the person to
  Create load case and to the case's own editors. "Go to Create load case" scrolls to that
  editor, focuses its ID field and says so in a status line. The case's load/reference
  state offers "Copy analysis state from another case", the existing reviewed copy. The
  excitation stays person-entered.

## 5. Operation ids

Every `operation_id` and `change_id` the load manager builds (load case, primitive,
combination, term, factor, basis and delete operations, and the template) carries each id
through `operationIdToken`. That is a reversible escape: a character in `[A-Za-z0-9_.:-]` is
kept, and any other character, `~` included, becomes `~XX` per UTF-8 byte. So distinct raw ids
give distinct tokens. For example, `combination:EQ/X` gives `combination:EQ~2FX`, distinct from
`combination:EQ-X`. An id of safe characters is its own token, so existing operation ids are
unchanged. Target ids and payload values are never encoded.

**Remaining limit.** The primitive-load operations join two tokens, the case's and the
primitive's, with `-`, as before (`<case>-<primitive>-primitive`), and `-` is a safe
character. Two different pairs can therefore still join to the same text. Changing that join
would change every published primitive operation id: the e2e smoke and `SMOKE.md` record
them. It is left as it was.

## 6. Evidence

Every test runs the real wasm engine. Models are built in test code from the committed
`fixtures/product_preview/invented_demo_model.json` (0.9.0, with nonlinear supports and a
constant-effort support), with invented axis-state cases. No fixture file was added.

- `combinationEditor.engine.test.tsx` (12):
  - the end-to-end path: two differences and an SRSS over them authored in the panel and
    applied by the engine;
  - typed terms, factors and reordering;
  - a repeated operand refused in place;
  - resolved targets and shadowing;
  - the operating comparison's load-case-only choice, and the engine refusing
    `…_NOT_LOAD_CASE` by name;
  - self-reference and a cycle prevented with reasons, and the engine refusing a cycle, a
    self-reference and an unknown operand by name;
  - since #115, a term naming a combination is deletable, and a referenced
    combination lists its referencing path;
  - the template's payloads, preview, gates (consuming constant-effort variant, nonlinear
    supports) and engine acceptance;
  - the out-of-order refusal;
  - the OQ-1 mechanics combination;
  - `absolute_sum`, and a taken id;
  - field-level edits of a nested record;
  - review r1 of #114: a basis queued without operands queues nothing and names what is
    missing (every basis), a new row and add-term start blank, the template needs each
    factor entered; a template draft changed during a pending check queues nothing; distinct
    raw ids give distinct operation ids and pass batch preflight; "Go to Create load case"
    focuses the ID field and shows its status line.
- `combinationDelete.engine.test.tsx` (7):
  - `display_number` and the historical display;
  - deletes through the panel and the engine: an SRSS of combinations (canonical), a
    subtraction with `terms: []` (historical), a subtraction without `terms` and an
    envelope without `terms` (canonical), and a flat mechanics record with factors 1.5 and
    1e-7 (historical);
  - a term naming a combination (canonical) and a flat term;
  - the referenced refusal: every path (a factor-0 term, a minuend, an envelope operand and
    an SRSS term) shown before the attempt, then the engine's refusal by name with nothing
    queued;
  - the seismic template set deleted top-down back to the original document;
  - the inverse restores at `combinations[i]` and `terms.0`;
  - the termless mechanics and `minuend_final` restores refused by name, with session Undo
    restoring exactly.
- `combinationEditor.session.test.tsx` (3): the session route:
  - the four changes applied singly in order, the out-of-order apply refused by name, and
    Undo restoring each document;
  - batch review applying all four, with one Undo;
  - a saved, reopened, edited, saved and reopened nested SRSS.
- `combinationAuthoring.test.ts` (4): reference resolution, cycle paths through bare
  references, term shapes and moves.

**Limit.** No browser-reachable solve exists. The solve is a native job, and the wasm engine
exposes operations, document validation and pre-formation model checks. So the κ and
nonlinear gates are stated by the editor, not observed in these tests. They are judged by the
engine's own tests (`core/product_physics/tests/rt_a4_combinations.rs` V-6, V-9), and a
native solve reports them in its diagnostics.
