---
doc_id: OPS-THEORY-HANGER-SIZING
doc_kind: theory.note
status: draft
created: 2026-11-02
updated: 2026-11-02
deliverable_id: DEL-11-03
package_id: PKG-11
subject_deliverable: DEL-04-11
release_target_rows:
  - RT-A11
  - RT-V4
---

# Hanger Sizing

## Purpose and boundary

This note explains how SWBPIPE sizes variable spring hangers and constant-effort
supports from the user's own hanger catalogue. It covers the inputs, the method,
its assumptions and its limitations. It is written from the implemented engine
(`core/analysis/hanger_sizing`) and its governing design
(`execution/PKG-04/DEL-04-11/Work/hanger-sizing.md`); where the two are cited, the
design governs and the code follows it.

Every rule below is a mechanics or arithmetic statement written for SWBPIPE. No
supplier catalogue, supplier selection procedure or standards text is used, and
none ships. The load-variation limit, the travel allowance and the selection rule
are the user's inputs. A selected record is the record that meets the user's own
tests on the user's own model. Sizing does not judge whether a hanger is suitable for
a service, and it does not determine code compliance.

## What is built, and what is not yet

| Capability | State |
|---|---|
| The sizing arithmetic, fit tests, selection and verification arithmetic | Built (slice 1a) |
| Hanger library format 1.1.0, with a per-record working load range | Built (slice 1b) |
| The derived solves on the pressure-free route, the full sizing run and the library reader | Built (slice 1c) |
| A published hanger table, its readers, CSV export and a headless runner entry | Not built (slice 1d) |
| The desktop request form, hanger table, detail view and application of a selection | Not built (increment 3) |
| Sizing on the exact pressure route | Not built (increment 4); a request on that route is refused (`HANGER_SIZING_EXACT_ROUTE_PENDING`) |
| A variable spring carrying its cold load in the model's own solve (preload) | Decided, not built (increment 6) |
| Forming the hot-load basis in the design operating case's support state | Not built; waits for the solve to hold another case's support states |

Today a sizing run produces a typed result inside the engine. Nothing is published
to the desktop or to a report yet, and nothing in this note describes a desktop
screen.

## Inputs

Every input is required. The engine supplies no value for any of them.

- **The library:** one imported hanger library (see *Library records*).
- **The candidates:** support ids, each with its family policy:
  - `variable`: variable spring records only;
  - `constant`: constant-effort records only;
  - `variable_else_constant`: constant-effort records only when no variable record
    fits.
- **The weight case:** a load case of the model.
- **The support state of the weight solve:** only the weight case's own converged
  state is admitted today.
- **The design operating case,** and any further operating cases to check (stated,
  even when there are none).
- **The load design:** hot or cold.
- **The variation limit** λ, a positive ratio, and its reference load: each case's
  operating load, or the hot-load basis.
- **The travel allowance** a, a length of at least zero, added at each end of the
  travel span.
- **The selection rule:** smallest capacity, or least variation.
- **The solver mode** of the user's own solve of these cases.

A named case must be a load case, not a combination. It may not carry a test
condition, because a hydrotest holds its springs, which is not the state a hanger is
sized in. A case may be named in only one role.

The weight case's content is the user's. Sizing does not check that it holds only
weight, and every load it carries enters W. The run returns the load categories each
named case carries as authored, so that a weight case with thermal or other loads can
be seen as such.

## Candidates

A candidate is a support of the model that:
- restrains exactly one translational DOF (UX, UY or UZ);
- has no skew direction;
- is not nonlinear, a snubber or an anchor;
- shares its node and DOF with no other support. "Shares" counts every DOF another
  support acts on: its rigid restraints, its nonlinear DOF, and its stiffness DOF,
  taken from its top-level stiffness or else its hanger's, as the solver takes it;
- is not referenced by another model object: a load targeted at the support, or
  another support's friction normal-reaction source.

The candidate's present family and hanger data are not read, because every derived
solve replaces the candidate. A placeholder support can therefore be sized before
any hanger data exists.

A candidate's force acts along the **positive axis of its DOF**, the convention the
constant-effort support already uses. Nothing infers "up". In a model whose vertical
axis is +Z, a hanger candidate restrains UZ and its force is positive upward.

## The installed position and the spring law

All positions are measured from the model's drawn geometry, u = 0, along the
candidate's positive DOF axis. The drawn geometry is taken as the **installed
position**: the pipe is erected at its drawn position, and the spring is set there.

A variable spring of rate k set to the load F_cold at the installed position carries

  F(u) = F_cold − k·u

at any position u. The same law is used everywhere: in the sizing arithmetic, in the
verification pass, and, once increment 6 is built, in the model's own solve.

**This is an assumption.** Practice also sets springs at a settled cold position or with
an erection offset. SWBPIPE offers neither. Where the cold-condition position u_C is not
zero, the installed setting F_cold and the cold-condition load F_C differ. The engine
returns both, and only F_cold is a setting. Stating the assumption in the published
basis, and labelling the two loads in the hanger table, are planned publication
behaviour (slice 1d).

## The method

### Hot-load basis

The weight case is solved with every candidate made a rigid restraint of its DOF.
Each candidate's reaction along its DOF is its **hot-load basis** W: the load the
hanger must carry where the rigid support was.

A W that is not positive refuses the whole request, naming each such candidate
(`HANGER_SIZING_LOAD_NOT_POSITIVE`). Such a reaction would put the hanger in the
opposite sense, and sizing never flips a sign.

### Positions

The weight case and each operating case are then solved with every candidate
removed and replaced by a nodal force W along its DOF, with no stiffness. The
candidate's displacement there is its position:
- u_C in the weight case, the **cold condition**;
- u_i in each operating case i, and u_D in the design operating case.

In a linear model, u_C is zero at every candidate up to rounding, because the force W
equals the rigid reaction. With lift-off, gaps or friction it need not be. It is
never subtracted: it enters the loads and the travel span as a position in its own
right.

Every candidate is replaced at once, so each W and each position already includes the
other candidates being rigid, or being replaced by forces.

### Derived solves

Each of these solves is a modified copy of the user's own solve request, sent
through the product's ordinary solve entry, the same one the model's own cases use.
Solve-time processing of the case, such as resolving operating-condition sets,
therefore applies exactly as in the model's own solve. A copy keeps only the case it
solves. It is never saved, never returned as a model and never passes through the
operation route, so the stored model is unchanged by a sizing run.

If a derived solve is refused, fails to converge, publishes a blocking diagnostic or
omits a value sizing needs, the whole request is refused, carrying that solve's
diagnostics (`HANGER_SIZING_DERIVED_SOLVE_FAILED`). No partial table is produced. A
candidate whose removal leaves the model without stability is reported this way: it
was carrying stability, not only load.

### Nonlinear supports

Each derived solve is a full solve of its case, and lift-off, gaps, one-way supports
and friction converge to the states that case produces. Friction is the solve's
static, history-free friction. W is formed in the weight case's own state.

Practice also forms W in the design operating case's state: in the hot position, a
support that has lifted off no longer carries weight, so a hanger near it carries
more. SWBPIPE does not do this yet. The result warns, by name, of every nonlinear
support whose state differs between the weight solve and the design operating solve
(`HANGER_SIZING_SUPPORT_STATE_DIFFERS`), so the user can judge whether W represents
the hot condition. It warns in the same way where replacing the candidates by their
forces found another equilibrium in the weight case
(`HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS`).

### Variable spring loads

For a variable record of rate k:
- **Hot design** sets the spring to carry W in the design operating case:
  F_cold = W + k·u_D.
- **Cold design** sets it to carry W in the cold condition: F_cold = W + k·u_C.

From the law, F_C = F_cold − k·u_C, and F_i = F_cold − k·u_i in each operating case.

The **load variation** of case i is the change of load between the cold condition and
that case, over the chosen reference load R_i (F_i, or W):

  v_i = |F_C − F_i| / R_i = k·|u_i − u_C| / R_i.

### Travel span and allowance

The travel span runs from the least to the greatest of {0, u_C, u_1 … u_n}. It
includes the installed position and the cold condition. The required travel is the
span plus 2a, the allowance at each end. For a variable record, the loads at the ends
of the span, widened by k·a at each end, form the load envelope:

  [F_cold − k·(max + a), F_cold − k·(min − a)].

### Fit tests and selection

Records are tested in a fixed order, and a record's first failed test is its reason.

**Variable record:**
- **E1:** it carries a rate, a working load range and a travel range;
- **E2:** its stiffness DOF is the candidate's DOF;
- **V0:** F_C and every F_i are positive;
- **L:** the load envelope lies within the working load range;
- **T:** the required travel is within the travel range;
- **V:** every variation is within λ.

**Constant-effort record:**
- **E1:** it carries a working load range and a travel range;
- **L:** W lies within the range;
- **T:** the required travel is within the travel range.

A constant-effort support carries W in every position, so it has no variation test.

Among the fitting records:
- **smallest capacity** chooses the smallest range maximum, then the smallest
  governing variation, then the smallest `hanger_id` in byte order;
- **least variation** chooses the smallest governing variation, then the smallest range
  maximum, then the smallest `hanger_id`.

A record's governing variation is the largest of its v_i over the operating cases. A
constant-effort record's is zero.

The tie-breaks make every selection deterministic.

**No fit.** When no record fits, the outcome is "no record fits", with the count of
records failing each test in each family tested. Nothing is assumed in the
candidate's place.

### Verification

Sizing estimates loads from positions found with the candidates rigid or replaced by
forces, but a spring's rate redistributes load. The selections are therefore checked
by solving the weight case and every operating case again, with:
- each selected variable record as a spring of its rate plus the force F_cold, which
  is the law F(u) = F_cold − k·u;
- each selected constant-effort record as the force W;
- each candidate with no fit left as the force W.

The verification returns:
- for every selected family, the verified positions, travel span and travel test.
  A constant-effort support pushed further by a neighbouring spring is caught here;
- for a variable selection, the verified loads F_C^v and F_i^v and the verified
  variation |F_C^v − F_i^v| / R_i^v. Where a verified reference load is not positive,
  no ratio is formed; the V0 failure reports it, and V is judged on the ratios that are
  formed;
- the departure from W: F_D^v − W under hot design, F_C^v − W under cold design.

The engine returns each test that the verified values fail, by name. Publishing these
as named warnings is planned publication behaviour (slice 1d). No selection is changed
automatically, because no convergence rule for re-sizing is offered. When any
candidate has no fit, the verification is marked **conditional**. It assumed a
force W at that candidate, and applying the selected records alone does not
reproduce that configuration.

## Library records

A hanger library is the user's own imported private data. Format 1.1.0 adds to each
record an optional working load range, `{minimum, maximum}`. Format 1.0.0 documents
still import, and their records are "not sizable: no working load range". A 1.1.0
record without a range is not sizable either.

The import boundary refuses:
- a range in a 1.0.0 document, as an unknown field;
- a range whose ends are in different units;
- a range whose minimum is not below its maximum, compared as binary64 numbers, the
  values sizing computes with.

Sizing reads each record's quantities into SI through the unit catalog. A quantity
it cannot read is left absent, so the record fails E1. It never takes a value in the
quantity's place.

A constant-effort record is selected by its range; its own `constant_load` is not
read. A record's `movement_limit` is not read.

## Cost

A request with n operating cases runs 2n + 3 ordinary solves, one after another:
- S_W, the weight case with the candidates rigid;
- S_C and the n operating solves, with the candidates replaced by forces;
- the n + 1 verification solves.

Only the values sizing needs are kept after each solve. Each solve currently pays its
full ordinary publication, which the sizing then discards.

## Assumptions and limitations

- **The installed position** is the drawn geometry. Settled-position and
  erection-offset settings are not offered.
- **Candidates** have a single translational DOF, with no skew direction.
- **The weight case's own support state** forms W.
- **The exact pressure route** is refused until increment 4.
- **The model's own solve does not yet apply a variable spring's cold load** (increment
  6). Until then, a spring applied to the model is solved as its rate from zero
  load. The model's own cases therefore do not reproduce the verification's loads,
  and each entered spring load is warned as not applied
  (`SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD`).
- **No automatic re-sizing:** the user re-runs sizing after applying selections.
- **The weight case's content** is not checked: every load it carries enters W.
- **Inherited premises:** a sizing result inherits every premise of the solves it
  uses. It bounds no modelling, input or physical error.

## Evidence

Each set of references below is frozen under `validation/references/hanger_sizing/`.
A generator that reads no product code derived it, and the engine's tests compare
against it:
- **R-1:** rigid reactions of two-span, three-span and unequal-span continuous beams;
- **R-2:** the growth of a free riser;
- **R-3:** the loads, fit tests and selection over an invented catalogue, including:
  - hot and cold design;
  - both reference loads;
  - nonzero cold positions;
  - boundary equality;
  - fallback and no fit;
  - non-positive loads;
- **R-4:** the verification arithmetic, including absent ratios;
- **R-5:** lift-off that opens in the operating case, and lift-off that holds,
  including the support states in every solve.

Selection outcomes, record ids, failure counts and failed tests are compared exactly.
Values are compared at 1e-9 relative, with an absolute allowance:
- in `core/analysis/hanger_sizing/tests/r3_references.rs`, the allowance is 1e-12;
- in `tests/derived_solves.rs`, positions get an allowance of 1e-9 of the case's
  largest expected position, never below 1e-12 m, because a closed-form zero
  cannot be judged by a relative tolerance.

A selected spring's verification is also checked against a closed-form pinned-span
solution in `tests/derived_solves.rs`.

## Source notes

- Governing design: `execution/PKG-04/DEL-04-11/Work/hanger-sizing.md`.
- Commitments: `execution/PKG-04/DEL-04-11/ScopeOfWork.md`.
- Implementation:
  - `core/analysis/hanger_sizing/src/lib.rs` (arithmetic);
  - `src/derived.rs` (derived solves);
  - `src/library.rs` (records).
- Library format: `schemas/hanger.schema.yaml` (DEL-03-02 CLM-030).
- The beam, frame and spring relations used here are classical Euler–Bernoulli
  statics and are stated without external source. The independent references derive
  them in closed form.
