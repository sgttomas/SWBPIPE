# Support direction entry (R-6/R-7 S-K1, desktop)

DEL-07-02:18's support-direction part, for skewed restraints (DEL-04-03, R-6/R-7). It serves
RT-G2 and RT-G3. The contract is DEL-04-03 Work `skew-restraints.md`: §2.1, §2.2 (gate order
and codes), §4.2 (range), §6.4 (the DOF token) and "The model, applier and authoring contract
for S-K1". The applier is the authoring lane's (AP-1). This note describes the code; it records
no backcheck or acceptance.

**Engine.** The S-K1 applier (`create_support` writing `direction`) is on main (#140, merge
b2b10c7c6). The creation tests below run against Wasm built from that source.

## 1. The contract the UI serves

- `create_support` writes `direction` exactly as entered: any finite, non-zero 3-vector,
  never normalized. It does so on an explicit `line_stop` or `spring` acting on exactly one
  translational DOF (UX, UY or UZ) with no `nonlinear` member. Axis vectors are stored too,
  and lowered at the solve.
- The engine refuses at authoring, by name. The document rules come first:
  `SUPPORT_DIRECTION_INVALID`, `…_ZERO`, `…_RECORD_UNSUPPORTED` and `…_ROTATIONAL_UNSUPPORTED`.
  Then, for a record they admit, it refuses `SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED` on a
  document with a present, non-null `pressure_contract` (current or retired), and
  `SUPPORT_DIRECTION_RANGE_UNSUPPORTED`. An axis-aligned direction is not a skewed record and
  is never refused for EX. A present `pressure_contract: null` is malformed: the shared
  validator refuses it as `MODEL-SLOT-SHAPE` at `pressure_contract` (Agent 0, 2026-11-05),
  whatever the direction. The desktop adds no rule of its own for any of these.
- The nonlinear-model code, a second stop at a node, redundancy and the node's motion are
  refused at the solve, not at authoring.
- (a) `update_support configuration` replaces only CONFIG (`family`, `restraints`,
  `stiffness`, `hanger`, `nonlinear`, `provenance`), so a stored `direction` survives
  unrelated edits. An edit that would make it inadmissible is refused by name: the direction
  is never written and never dropped.
- (b) On a skewed record, the DOF token only marks it translational, so UX→UY→UZ is admitted.
  A rotational token is refused.
- (c) Agent authors may write `direction` (Agent 0's decision A). This does not relax the
  manual-only k/SIF/stiffness/stiffness-scaling/source rules (LIM-001/O5).
- S-K1 has no edit or remove path for a stored `direction` (`update_support`, `set_field`).

## 2. What is built

**Entry.** The support configuration form's create mode
(`features/support-configuration/SupportConfigurationForm.tsx`) has an optional
**Direction (x, y, z)**: a labelled group of three inputs, `Direction x`, `y` and `z`. It is
never prefilled, and it starts empty when "Create a new support" is ticked from a selected
support. The text is kept as typed. `supportDirection.ts` `parseDirectionEntry` reads it with
the run sheet's typed-number parser (`parseNumber`: no hex, no `Infinity`, no unit text). The
member is written only when all three components are entered, as the typed binary64 values:
JSON's shortest round-trip text, never normalized and with no unit. All three blank means
absent, and the member is not written.

These are refused before queueing, with a message naming the component:
- a partial vector (a blank is never taken as zero);
- a non-number;
- `-0`, which the JSON payload cannot carry apart from `0`, so it would not be written as
  entered.

**Admission is the engine's.** The form does not duplicate the document rules or the skew
admission. A creation that carries a direction is checked, validate-only, by the operation
engine with the current model hash before it is queued (`condition-sets/engineCheckedQueue.tsx`,
the S3b pattern). A blocking finding is shown in place by code, message and references, and
nothing is queued. The group's description states the contract, so an inadmissible family or
DOF is explained there and refused by the engine's own code. A creation without a direction
keeps the form's existing queue unchanged.

**Stored direction.** It is shown read-only, component by component as stored (`-0` shown as
`-0`; a malformed value as its JSON), labelled "edit/remove not supported yet (S-K1)". It
appears in two places:
- the support configuration form's edit mode;
- the inspector's selected-support properties (`model-workspace/modelView.ts`).

No input edits it.

**Preserved by every existing edit path.** No path rebuilds the record or puts `direction` in
its payload. The applier updates the named members only (`rich_authoring.rs` starts from the
stored record).

| Path | Payload | Evidence (test) |
|---|---|---|
| Support configuration form, `update_support configuration` | CONFIG projection only | `supportDirection.test.tsx` (payload); `supportDirectionEngine.test.tsx` (applied: provenance edit and UY→UX keep the bits; guide, UX+UY and RX are refused `…_RECORD_UNSUPPORTED`/`…_ROTATIONAL_UNSUPPORTED`; a nonlinear member is refused `…_RECORD_UNSUPPORTED`) |
| Bulk change, support kind | `supportConfiguration` (CONFIG) | the same two (applied: line stop UY→UZ keeps the bits; → guide is refused by name) |
| Hanger selection, `update_support` | CONFIG members and `provenance_record` | the same two (applied: refused `…_RECORD_UNSUPPORTED`, model unchanged) |
| Support grid cells (`label`, `node`, `restraints`, `provenance`) | the one member | the same two (applied: label, provenance and UY→UZ keep the bits; `UX, UY` is refused by name) |

The inspector's scalar support fields send the same field-path operations as the grid cells.

**Agent route.** Unchanged. No UI added here lets an agent write a person-only modifier.

## 3. Tests

- `supportDirection.test.tsx` (stand-in hash and operation services): 17 tests. They cover
  parsing; refusals; stored display; that empty means absent; the exact payload members; the
  engine call and the in-place code for a refusal; no entry in edit mode; accessibility (the
  labelled group and its description, native inputs in keyboard order before the queue
  button); and each edit path's payload.
- `supportDirectionEngine.test.tsx` (real Wasm):
  - Five preservation and refusal tests over the four paths, which run on main today.
  - Twelve creation tests against the S-K1 applier. They cover:
    - a skewed line stop and a skewed spring created through the form and applied, with the
      stored bits equal to the input;
    - the EX refusal on a current and on a retired contract object, and the range,
      rotational and zero refusals, each shown in place with nothing queued;
    - the record rule before EX: a guide on a current or a retired contract shows
      `…_RECORD_UNSUPPORTED`, never the EX code;
    - an axis-aligned direction on a retired contract, created and stored as entered;
    - a present `pressure_contract: null`: the form shows `MODEL-SLOT-SHAPE` in place, never
      the EX code, and queues nothing;
    - (a) and (b) through `update_support` on an engine-created skewed stop.

  The EX and record-rule cases use shape-valid contract objects; the null case is separate
  (also pinned in `operation_applier/tests/skew_direction_authoring.rs`). Before S-K1 the
  creation tests were skipped, and against the earlier engine they failed (it showed
  `SUPPORT_DIRECTION_SKEW_UNSUPPORTED`), so they are not vacuous.

  The support direction type is one type, `SupportDirection` in `types.ts`, used by the record
  member, this entry and the support load summary (`results/supportLoadSummary.ts`).

## 4. Open

- Engine-checked creation runs only when a direction is entered. A direction-free creation
  could use the same check, but this slice keeps the existing queue.
- The other `create_support` producers do not offer a direction: the inspector's New support
  form, the run sheet, boundary authoring and hanger selection's new support. They never
  write one, so nothing is dropped.
- The support grid has no Direction column; the inspector and the form show it.
- Edit and remove of a stored `direction`, and a viewport skew glyph, are later work.
