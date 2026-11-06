# Hanger sizing: governing design

**Status.** This is DEL-04-11's governing design for hanger sizing and the hanger
table (`docs/RELEASE_TARGET_v1.0.md` RT-A11; OC-4(a), owner, 2026-10-28: "Must: size
hangers from the user's private catalogue (invented example records only) and produce a
hanger table."). Agent 0 (HELP_HUMAN) placed the scope in this new deliverable through
SCA-015 on 2026-10-31 (D-1), adopted the method outlined by the lane's implementation
manager as the basis for this text (D-4), decided the hanger library format 1.1.0 (D-2)
and decided that a variable spring is modelled with its cold (preset) load (D-3, §11).
Nothing is built. This design makes no acceptance, issuance or lifecycle statement
(F-PIP-3). SWBPIPE computes mechanics; it does not determine code compliance or the
suitability of any hanger. Standard claim fence applies (claims taxonomy per DEC-081).

**Labels.** **[decided]**: settled here, with the reason. **[open]**: listed in §14 with
its owner.

**Sources.** Every rule below is a mechanics or arithmetic statement written for this
design. No supplier catalogue, supplier selection procedure or standards text is used.
Load variation, travel allowance and the selection rule are inputs the user supplies;
SWBPIPE ships no value for any of them. Catalogue records are the user's own imported
private data (`docs/IP_AND_DATA_BOUNDARY.md`); every catalogue in the repository is
invented.

## 1. What sizing is

- **[decided] A request beside the model.** Hanger sizing is a request that names a
  model, a hanger library and the inputs of §2, like the modal request (DEL-04-09
  Design §1.4). It adds no model-document member (model document 0.9.0 is frozen;
  Agent 0, 2026-10-31). The request is recorded with its result and in the
  analysis-run record.
- **[decided] Results only.** Sized loads, travels and selected records are results
  with their basis (§8). Sizing never writes the model. Applying a selection is the
  person's explicit act through the existing operation route (§9).
- **[decided] The method** (D-4), for the candidate supports the person names:
  1. *Hot-load basis W.* Solve the weight case with every candidate made a rigid
     restraint of its DOF. W is the candidate's reaction.
  2. *Positions.* Solve the weight case (the cold condition) and each operating case with
     every candidate replaced by its force W and no stiffness. A candidate's positions are
     its displacements there, measured from the model's drawn geometry, which is the
     installed position (§6).
  3. *Loads and fit.* For each catalogue record, form the cold and operating loads and
     the load variation, and test the record against its working load range, its travel
     range and the variation limit.
  4. *Selection* among the records that fit, by the person's rule.
  5. *Verification.* Re-solve with the selected records in place and report the loads
     they then carry.

## 2. The request

Every input is required, and none has a default. The engine, the headless runner and the
native entry never supply a value. The desktop pre-fills the values last used in the
project, as it does for the modal request; the request records the values actually used.

| Input | Meaning | Refusal when absent or invalid |
|---|---|---|
| `library` | One imported hanger library in the project's library store, by `library_id`. The request records the library's canonical digest | `HANGER_SIZING_LIBRARY_REQUIRED`, `HANGER_SIZING_LIBRARY_UNAVAILABLE` |
| `candidates` | A non-empty list of support ids, each once, each with its `family_policy`: `variable`, `constant` or `variable_else_constant` | `HANGER_SIZING_CANDIDATES_REQUIRED`, `HANGER_SIZING_CANDIDATE_DUPLICATE`, `HANGER_SIZING_FAMILY_POLICY_REQUIRED` |
| `weight_case` | A load case of the model | `HANGER_SIZING_WEIGHT_CASE_REQUIRED` |
| `weight_support_state` | Which nonlinear-support state W is formed in (§5). Only `own_solve` is admitted until the operating-status machinery of RT-A5 lands | `HANGER_SIZING_WEIGHT_SUPPORT_STATE_REQUIRED`, `HANGER_SIZING_WEIGHT_SUPPORT_STATE_UNSUPPORTED` |
| `design_operating_case` | The load case in which a hot-designed variable spring carries W (§6.1) | `HANGER_SIZING_DESIGN_OPERATING_CASE_REQUIRED` |
| `check_operating_cases` | Further load cases checked against the same setting; may be empty, but must be stated | `HANGER_SIZING_CHECK_OPERATING_CASES_REQUIRED` |
| `load_design` | `hot` or `cold` (§6.1) | `HANGER_SIZING_LOAD_DESIGN_REQUIRED` |
| `variation_limit` | A positive finite ratio; the result states it also as a percentage | `HANGER_SIZING_VARIATION_LIMIT_REQUIRED`, `HANGER_SIZING_VARIATION_LIMIT_INVALID` |
| `variation_reference` | `operating_load` or `design_load` (§6.2) | `HANGER_SIZING_VARIATION_REFERENCE_REQUIRED` |
| `travel_allowance` | A finite length ≥ 0, applied at each end of the travel span (§6.3). Zero is admitted only when entered | `HANGER_SIZING_TRAVEL_ALLOWANCE_REQUIRED`, `HANGER_SIZING_TRAVEL_ALLOWANCE_INVALID` |
| `selection_rule` | `smallest_capacity` or `least_variation` (§6.5) | `HANGER_SIZING_SELECTION_RULE_REQUIRED` |
| `solver_mode` | The solver mode of the caller's own solve of these cases (sparse interactive or dense scrutiny); every derived solve uses it (§4.1) | `HANGER_SIZING_SOLVER_MODE_REQUIRED` |

- **[decided] Case roles.** Every named case is a load case of the model, not a
  combination, and is solvable on the model's route. A case carrying a `test_condition`
  (RT-A6) is refused in either role (`HANGER_SIZING_TEST_CASE_UNSUPPORTED`): a held
  support is not the state a hanger is sized in. The weight case may not also be an
  operating case, and an operating case may be named only once
  (`HANGER_SIZING_CASE_ROLES_OVERLAP`). An unknown case is refused by name
  (`HANGER_SIZING_CASE_UNKNOWN`).
- **[decided] The weight case's content is the person's.** Sizing does not inspect
  whether the weight case holds only weight. The result lists the load categories each
  named case carries as authored, and whether it uses an operating-condition set, whose
  temperature and added weight the solve lowers into the case (§4.1). A case with thermal
  or other loads is then visible as such. Slice 1c lists the authored categories; the
  condition-use flag follows with publication (slice 1d).
- **Quantities** are normalized through DEL-02-02's unit catalog and stored in SI
  (N, m, N/m). The ratio `variation_limit` is dimensionless.

## 3. Candidates

- **[decided] Eligibility.** A candidate is a support of the model that:
  - restrains exactly one DOF, and that DOF is a translation (UX, UY or UZ);
  - carries no skew `direction`;
  - carries no `nonlinear` member and is not a snubber or an anchor;
  - shares its node and DOF with no other support;
  - is referenced by no other model object, such as a friction normal-force source, a
    support-targeted imposed displacement or a boundary association;
  - on an EX load/reference-state document, has in every named case an ordinary support
    state: participation `active_model_device`, with no `boundary_motion`, `base_motion`
    or `device_reference`.

  The support-state entry that every support needs on such a document is the support's
  own participation declaration. It is not a reference that blocks candidacy, and the
  derived documents reconcile it (§4.1). A candidate whose support state moves it, or
  that declares a reference, is refused (`HANGER_SIZING_CANDIDATE_MOVED`), because a
  hanger's travel would then include an imposed motion that sizing does not separate.

  Its present family does not matter: a `vertical_support`, a `spring`, a
  `variable_spring_hanger`, a `spring_hanger` or a `constant_effort_support` placeholder
  is admitted alike, because the derived solves replace it (§4). This is why sizing
  accepts a placeholder that today's validation would block as an incomplete variable
  spring (`SPRING_HANGER_LOAD_MISSING`): a candidate's own hanger data is not read.
  Each failure is refused by name: `HANGER_SIZING_CANDIDATE_UNKNOWN`,
  `HANGER_SIZING_CANDIDATE_DOF_UNSUPPORTED` (none, several or a rotation),
  `HANGER_SIZING_CANDIDATE_SKEW_UNSUPPORTED`, `HANGER_SIZING_CANDIDATE_FAMILY_UNSUPPORTED`,
  `HANGER_SIZING_CANDIDATE_DOF_SHARED`, `HANGER_SIZING_CANDIDATE_REFERENCED`,
  `HANGER_SIZING_CANDIDATE_MOVED`.
- **[decided] Direction.** A candidate's force acts along the positive axis of its DOF,
  the DEC-049 convention constant effort already uses
  (`core/product_physics/src/lib.rs`, `add_constant_effort_support_loads`). Nothing
  infers "up". A candidate whose W is not positive is refused
  (`HANGER_SIZING_LOAD_NOT_POSITIVE`, stating W): the reaction would put the hanger in
  the opposite sense, and sizing never flips a sign.
- **[decided] One refusal refuses the request.** Any refusal of §2 or §3 refuses the
  whole request with every finding listed, because the derived solves replace all
  candidates together. This includes a W that is not positive, found after S_W: the
  force-replacement solves would otherwise be built on a load the hanger cannot carry.
  Every such candidate is named, so that the person can remove it and run again.

## 4. Derived solves

### 4.1 Construction

- **[decided] Modified documents through the normal solve entry.** Each derived solve is
  a modified copy of the stored model document, sent through the same product solve
  entry the model's own cases use (DEL-04-07). It is never assembled directly from a
  built model. The reason: from the load-cases lane's S2 onwards, a case's effective
  loads are resolved at solve time (`operating_conditions::lower`; on EX also the
  `analysis_state.load_sources` ledger), and only the solve entry sees exactly what the
  model's own solve sees (load-cases manager, 2026-10-31).
- **The modifications**, and nothing else:
  - *Rigid candidate* (solve S_W): the candidate's configuration is replaced by a rigid
    linear restraint of its DOF, with its `hanger`, `stiffness` and `nonlinear` members
    removed. **[decided]** (slice 1c) The family becomes `guide`, restrained in exactly
    its DOF. `guide` is the product's family for a rigid restraint of named translational
    DOFs (and its inference for any such set short of six), and both routes admit it. On
    an EX load/reference-state document, its ordinary support-state entry is kept
    unchanged.
  - *Force candidate* (solves S_C and S_Oi): the candidate support is removed, and a
    nodal force of magnitude W along the positive axis of its DOF is added to the solved
    case at its node. On an EX load/reference-state document:
    - the candidate's support-state entry is removed with it, because a support state
      that names no support is refused (`LOAD_STATE_SUPPORT_UNKNOWN`);
    - the force is entered in that case's `analysis_state.load_sources` ledger with
      factor 1, so that it is in the case's declared source ledger.
  - *Selected candidate* (solves V_C and V_Oi, §7): a variable selection becomes family
    `spring` restrained in its DOF, with the record's rate as its stiffness, plus the
    force F_cold. Constant and unfitted candidates are force candidates.
  - Each derived document keeps only the case it solves and no combinations. Added
    primitive ids are in a reserved namespace (`load:hanger-sizing:…`) that is disjoint
    from every existing id; a collision is refused (`HANGER_SIZING_ID_COLLISION`).
- **[decided] Never persisted.** A derived document is never saved, never returned as a
  model, never enters undo history and never passes through the operation route.
- **[decided] Solve settings are the model's.** Each derived solve uses the route and
  settings of the caller's own solve request, and the solver mode the request carries
  (§2). It requests no retained-precision recovery (§10).
- **[decided] Reading the results** (slice 1c). From each solve's published rows:
  - W is the candidate's `support_reaction_component_v2` row in its DOF's force
    component (the support's action on the pipe, positive along the axis);
  - u is the candidate node's `global_nodal_displacement_x|y|z` row;
  - states are the `nonlinear_support_active_set_state_code` rows.

  Each value is converted from its row's own unit through the unit catalog. A missing
  row refuses the request (`HANGER_SIZING_DERIVED_SOLVE_FAILED`), and so does any
  blocking diagnostic of the solve, whose code and message the refusal carries.
- **The exact route** follows in increment 4. Until then a document with a
  `pressure_contract` is refused (`HANGER_SIZING_EXACT_ROUTE_PENDING`), so that no EX
  document is sized before its support-state and ledger handling is tested.

### 4.2 The solves

| Solve | Case | Candidates | Read |
|---|---|---|---|
| S_W | weight case | rigid | W: each candidate's reaction along its DOF |
| S_C | weight case | force W | u_C: each candidate's displacement along its DOF (the cold condition) |
| S_Oi | each operating case i | force W | u_i: each candidate's displacement along its DOF |
| V_C, V_Oi | weight case and each operating case | selected (§7) | the verified loads |

- **[decided] One origin: the installed position.** Every displacement is measured from
  the model's drawn geometry, u = 0, signed along the candidate's positive DOF axis.
  The drawn geometry is the position at which the hanger is installed and set: the pipe
  is erected at its drawn position and the spring is preset there. One law then holds
  everywhere, in sizing, verification, application and the solve's preload (§11):
  a variable spring set to F_cold carries **F(u) = F_cold − k·u**.
  - In a linear model, u_C is zero to rounding at every candidate, because the force W
    equals the rigid reaction.
  - With lift-off, gaps or friction, u_C need not be zero. It is never subtracted, so
    no second origin arises. It enters the loads and the travel span as the cold
    condition's position (§6), and it is published.
- **[decided] All candidates move together.** Each solve replaces every candidate at
  once, as the method requires, so each W, u_C and u_i already includes the effect of the
  other candidates' supports being rigid or replaced by forces.
- **[decided] A failed derived solve refuses the request.** If any derived solve is
  refused, fails to converge or publishes a blocking diagnostic, the request is refused
  (`HANGER_SIZING_DERIVED_SOLVE_FAILED`), carrying the solve's own diagnostics and the
  name of the derived solve. No partial table is published. A singular system after a
  candidate is replaced by a force is reported this way, so the person sees that the
  candidate carried stability, not only load.

## 5. Nonlinear supports (D-4 item 1)

- **[decided] Each derived solve finds its own support states.** Every derived solve is a
  full solve of its case. Lift-off, gaps, one-way supports and friction converge to the
  states that case produces, exactly as in the model's own solve of that case, on PF
  today and on EX once RT-A3 admits them. Friction is the solve's static, history-free
  friction, inherited unchanged.
- **[decided] W is formed in the weight case's own state** (`weight_support_state:
  own_solve`). That is the state the named weight case actually produces. In the hot
  position, a support that has lifted off no longer carries weight, so the hanger near it
  may carry more. Forming W in the design operating case's state is a choice practice
  makes and SWBPIPE will offer: `design_operating_final` solves S_W with each nonlinear
  support held in its converged state from S_O,design. That needs the solve to take a
  support status from another case, which is RT-A5's Should option
  (`subtrahend_support_state: minuend_final`, load-cases lane). Until that lands, the
  input is required and only `own_solve` is admitted, so that the choice is explicit and
  recorded and the request needs no format change when the second value arrives. The
  load-cases lane will provide one product function, a named case solved with its
  nonlinear supports held at another named case's final status. Its `minuend_final`
  subtraction and this request's `design_operating_final` both call it (load-cases manager,
  2026-10-31; DEL-05-02 `Work/operating-case.md` §0). A held status can need a tensile
  reaction at a unilateral support, which is not an admissible state of the weight case.
  When that occurs, the result discloses it as such. [open] O-1.
- **[decided] Positions use each operating case's own state**, and the cold condition uses
  the weight case's own state with the candidates as forces.
- **[decided] Disclosure.** The result publishes each nonlinear support's state in S_W,
  S_C and every S_Oi. A support whose state in S_W differs from its state in
  S_O,design is reported by name (`HANGER_SIZING_SUPPORT_STATE_DIFFERS`, warning). The
  warning states that W was formed in the weight case's state, not the operating state,
  so that the person can judge whether the hot load is representative. A support whose
  state differs between S_W and S_C is reported the same way
  (`HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS`). Equal forces should reproduce the
  rigid equilibrium; a different state means the force replacement found another
  equilibrium.
- **A candidate is never nonlinear** (§3), so no candidate's own state is iterated.

## 6. Loads, fit and selection

The symbols are:
- W, the hot-load basis;
- u_C, the candidate's position in the cold condition (S_C);
- u_i, its position in operating case i (S_Oi), and u_D in the design operating case;
- every position is measured from the installed position, u = 0 (§4.2);
- k, a variable record's rate;
- [F_min, F_max], a record's working load range;
- T, a record's travel range;
- a, the travel allowance;
- λ, the variation limit.

All loads are positive along the candidate's DOF. A variable spring set to F_cold at the
installed position carries F(u) = F_cold − k·u, falling as the pipe moves along the
positive axis.

### 6.1 Variable spring loads

- **The loads.** F_cold is the spring's setting at the installed position, which is
  written to `cold_load` and `installed_load` on application (§9) and consumed by the
  solve (§11). From it:
  - F_C = F_cold − k·u_C, the load in the cold condition;
  - F_i = F_cold − k·u_i, the load in operating case i.
- **Hot design** (`load_design: hot`): the spring carries W in the design operating case,
  F_D = W, so F_cold = W + k·u_D.
- **Cold design** (`load_design: cold`): the spring carries W in the cold condition,
  F_C = W, so F_cold = W + k·u_C.
- **[decided]** These are the only two designs. Each states where the spring carries
  the weight it supports, and practice uses both, so the person chooses. In a linear
  model u_C = 0, and cold design sets F_cold = W.

### 6.2 Load variation

- For each operating case i, the variation is the change of load between the cold
  condition and that case:
  v_i = |F_C − F_i| / R_i = k·|u_i − u_C| / R_i,
  where R_i is the reference load:
  - F_i for `variation_reference: operating_load`;
  - W for `variation_reference: design_load`.
- **[decided]** Both references occur in practice, so the person chooses. They coincide
  for the design case under hot design. A reference load that is not positive (F_i ≤ 0)
  fails the record (test V0 below), because a spring that would carry no load, or
  negative load, cannot be judged by a ratio.

### 6.3 Travel and allowance

- The travel span is the interval from the least to the greatest of {0, u_C, u_1 … u_n}.
  It includes the installed position and the cold condition.
- **[decided] The allowance a is added at each end.** The required travel is
  span + 2a. For a variable record, the load envelope is the loads at the ends of the
  span widened by k·a at each end: [F_cold − k·(max + a), F_cold − k·(min − a)], with
  min and max the ends of the span. Adding the allowance at each end is stated so that
  one number has one meaning; the person enters a.

### 6.4 Fit tests

Records are tested in this order, and the first failure is the record's reason.

**Variable record** (`hanger_type: variable_spring_hanger`):
- E1: the record carries a stiffness, a `working_load_range` and a `travel_range`;
- E2: its stiffness `dof` equals the candidate's DOF;
- V0: F_C > 0 and every F_i > 0;
- L: the load envelope lies within [F_min, F_max];
- T: span + 2a ≤ T;
- V: every v_i ≤ λ.

**Constant record** (`hanger_type: constant_effort_support`):
- E1: the record carries a `working_load_range` and a `travel_range`;
- L: F_min ≤ W ≤ F_max;
- T: span + 2a ≤ T.

Notes on the tests:
- A constant-effort support carries W in every position, so it has no variation test.
  Its load is W, set at installation.
- A record that fails E1 is "not sizable". A library 1.0.0 record always fails E1, with
  the reason "no working load range" (§8.2).
- `movement_limit` is not read by sizing; the result says so.
- A variable record's stiffness always carries a DOF in the library schema. If one
  without a DOF reached the arithmetic, it would fail E2, not E1: a missing DOF does not
  match the candidate's.

### 6.5 Family policy and selection

- `variable` tests only variable records, and `constant` only constant records.
  `variable_else_constant` tests constant records only when no variable record fits, and
  the result says that the fallback was taken.
- Among the fitting records:
  - `smallest_capacity` chooses the smallest F_max, then the smallest max_i v_i, then
    the smallest `hanger_id` in byte order;
  - `least_variation` chooses the smallest max_i v_i, then the smallest F_max, then the
    smallest `hanger_id`.
  - For constant records v is taken as 0, so `least_variation` reduces to capacity, then
    id.
- **[decided]** Both rules occur in practice, so the person chooses; the tie-breaks
  make every selection deterministic.
- **No fit.** The outcome is "no record fits", with the count of records that failed
  each test (§8.1). The candidate is not sized, and nothing is assumed in its place in
  the table.

## 7. Verification pass

- **[decided] Re-solve with the selections in place.** Sizing estimates every load from
  W, u_C and u_i, which were computed with the candidates rigid or replaced by forces. A
  spring's rate redistributes load, so the selections are checked by solving again:
  - V_C, the weight case, and V_Oi, each operating case;
  - each candidate with a selected variable record becomes a spring of rate k on its
    DOF together with a nodal force F_cold along the positive axis. This is exactly the
    law F(u) = F_cold − k·u from the installed position (§4.2), the model the solve
    itself uses once the preload is built (§11);
  - each candidate with a selected constant record becomes a force W;
  - each candidate with no fit stays a force W, as in S_C and S_Oi.
  - These are derived documents as in §4.1. On EX, each candidate keeps its support
    state when it is a spring and loses it when it is a force, and every added force is
    entered in the ledger.
- **[decided] The verification's basis is stated.** The result publishes, per candidate,
  the configuration the verification assumed: a spring of the selected record, a constant
  force, or, for no fit, the force W. When any candidate has no fit, the verification is
  marked as conditional on that assumption. Applying the selected records alone does not
  reproduce it, because an unresolved candidate stays as the person last left it in the
  model (§9).
- **Published, for every candidate:**
  - the verified positions, the verified travel span and the T test with the allowance,
    for every selected family, so that a constant support pushed further by its
    neighbours' springs is caught;
  - for a variable selection, the verified loads F_C^v = F_cold − k·u_C^v and
    F_i^v = F_cold − k·u_i^v, and the verified variations
    v_i^v = |F_C^v − F_i^v| / R_i^v = k·|u_i^v − u_C^v| / R_i^v, with R_i^v = F_i^v for
    the operating-load reference and W for the design-load reference;
  - for a variable selection, F_D^v − W (hot design) or F_C^v − W (cold design), the
    departure from the hot-load basis.

  Where a verified reference load is not positive (F_i^v ≤ 0 under the operating-load
  reference), no ratio is formed for that case. Its variation is published as absent, V0
  reports it, and V is judged on the ratios that are formed, as in §6.2.

  Each failed test, formed with the verified values, is reported by name in the
  candidate's `verification.failed_tests` (§8.3.5). The published document has no
  separate warning code for it:
  - V0, L, T and V for a variable selection;
  - T for a constant selection; its L is unchanged, because its load stays W.
- **[decided] No automatic iteration.** The selection stands as sized, and the
  verification reports how it behaves. Re-sizing after applying a selection is the
  person's choice: the next request then sees the applied springs as candidates or as
  ordinary supports, as the person names them. An automatic loop would change
  selections without a stated convergence rule, and no such rule is offered.

## 8. Library and results

### 8.1 The hanger table and its basis

This section describes the hanger table as it is displayed. The published document
(§8.3) carries the values. A record's name, working load range and travel range come
into the display from the library that the document binds by hash.

**Per candidate**, one row:
- the support, node and DOF, and the family policy;
- the outcome: `selected`, `selected_by_fallback` or `no_record_fits`. A refusal refuses
  the whole request (§3); no row is refused on its own;
- the library id, `hanger_id` and name, and the record's type;
- for a variable record: k, [F_min, F_max] and T;
- W, F_cold (variable) or the constant load W (constant);
- u_C and F_C; for each operating case: u_i, F_i and v_i; the governing case for travel and the one
  for variation;
- the required travel span + 2a;
- the verification's assumed configuration, and the verified positions, travel, loads,
  variations and test outcomes of §7, for every selected family;
- for a candidate with no fit or a fallback: the number of records failing each test,
  E1 to V, for each family tested. A `variable_else_constant` candidate with no fit
  therefore shows both families' counts, which is how the table shows that the fallback
  was tried.

**Per derived solve:** u_C at each candidate, and each nonlinear support's state (§5).

**The basis block** gives:
- every request input (§2);
- the library id and hash;
- the solve request's hash, which identifies the exact model and materials;
- the route;
- for each named case: its role, its authored load categories and whether it uses an
  operating-condition set (§8.3.5).
It also states the installation assumption of §4.2: each spring is installed and set at
the drawn geometry. The table labels F_cold "installed setting (at drawn position)" and
F_C "cold-condition load", so that the two are never read as one where u_C is not zero.
Other installation conventions, such as setting at a settled cold position or with an
erection offset, are not offered. Under them the person reads F_C as the load at the
cold position.

- **[decided] Per-record evaluations are recomputed, not stored.** Selection is a pure
  function of W, u_C, the u_i, the request and the library. The desktop's detail view for one
  candidate calls the same Rust function to list every record with its first failed test,
  so the table's size is independent of the catalogue's size (§10).
- **Publication.** The table is published as its own closed document (§8.3), not under
  a mechanics profile or a 0.3 semantic table. A hanger-sizing analysis-run kind awaits
  the joint analysis-run 0.4.0, which is approved for design only (§8.3).
- **[decided] Readers.** A Rust product reader and a test-only Python verifier, which
  shares no code with the producer. What each establishes, and what it does not, is
  stated in §8.3.
- **CSV.** The hanger table exports to CSV under DEL-08-04's serializer and coverage
  guard (RT-R6), with the desktop work after PR #59 (increment 3).
- **[built, 3c] In the desktop.** The table is shown by the 3b panel, mounted with the
  request form beside the hanger selection. It shows one reply of the native sizing:
  the document, the reader evidence bound to the document's checked hash, and the library
  document the request named, so record details pass 3a's library-hash gate. A refused
  document shows its refusals, and a request that formed no document shows its named
  failure. The result is not recorded or saved.

### 8.3 Publication contract, slice 1d-α (Agent 0, P1–P7, 2026-11-02)

**[decided] Carrier.** A sizing result is published as its own closed, typed document,
not as rows in a mechanics envelope (Agent 0, P1). Modal analysis publishes its own
document; the two share basis and identity terminology, not a carrier.
- **Where it is published:** in 1d-α, only through the headless runner and the
  session.
- **Not a recordable run:** the joint analysis-run 0.4.0 is approved for design, not
  adoption (P2). The design proposal, which is not adopted, is
  `joint-analysis-run-proposal.md`.
- **Not the full publication criteria:** 1d-α does not meet the desktop, report or CSV
  criteria (REQ-007, REQ-012); those come with increment 3.

The schema is `schemas/hanger_sizing_result.schema.json` (JSON Schema 2020-12). Every
object in it is closed. The only maps with dynamic keys are listed in *Maps*, each with
its key and value rules.

#### 8.3.1 Numbers, canonical form and admission

- **The checked profile.** Every hash is the SHA-256 of the `openpipestress_jcs_ijson_v1`
  canonical text, implemented by the product's `canonical_json` and, independently,
  by the verifiers' `canonical_json.py`. That profile admits only:
  - finite numbers;
  - integers within ±(2⁵³ − 1);
  - objects without duplicate keys.
- **Admission before any lossy parse.** The runner reads the input as text and admits
  it through a **path-aware admission layer** owned by the sizing crate, before any
  other parse. The layer parses the text itself and rejects duplicate keys and numbers
  outside the profile. Each finding has:
  - a kind: `invalid_json`, `duplicate_key` or `number_outside_profile`;
  - the JSON path of the offending member.

  The offending value is never reported. The existing `canonical_json_checked_v1_text`
  returns string errors that can quote the value and carry no path, so it is not used
  for diagnostics. A test shows the layer's accept or reject verdict agreeing with
  that function's on the same text across the profile's boundary cases. An input
  outside the profile is refused (`HANGER_SIZING_INPUT_NOT_ADMITTED`), and the
  finding's kind and path make up the refusal's `names`. A
  quantity outside the profile, such as an integral 1e20 N, is refused under this
  code; sizing does not round it. A reader admits a result document by the same
  rule.
- **The verifier's parser.** The Python verifier rejects duplicate keys itself, with an
  `object_pairs_hook` that refuses them, and applies the profile's number rules.
- **Finite outputs.** Every number the engine would publish must be finite and inside
  the profile. Arithmetic that overflows from finite inputs, or any value outside the
  profile, refuses publication (`HANGER_SIZING_RESULT_NOT_PUBLISHABLE`), naming the
  candidate or solve and the member. The engine's own refusal of non-finite derived
  values (`HANGER_SIZING_DERIVED_SOLVE_FAILED`) stands as it is.
- **Hashing derived documents.** A derived document that cannot be put in canonical
  form refuses publication (`HANGER_SIZING_DERIVED_DOCUMENT_NOT_HASHABLE`), naming the
  solve label.
- **Refusals before hashing.** When an input hash cannot be formed, the refused
  document carries that input's `sha256` as `null`. A request that is not admitted
  carries `inputs.sizing_request` as `null`.

#### 8.3.2 The headless entry and its admission

The verb is `openpipestress-runner size-hangers [--input <file>|-] [--output <file>] [--explicit-local-private-intent]`.

- **Its own admission path.** It does not go through `CliInput.request`, a
  `RunnerRequest` that needs operation metadata and an input-manifest reference, and
  it synthesizes no such metadata.
- **What it reuses unchanged:**
  - the runner's reading of `--input` and stdin and its writing of `--output`;
  - the exit statuses: 0 when sized, 1 when refused or blocked, 2 when the invocation
    or input is unusable and no document is formed;
  - the local-private output form and flag (`ControlledExport` and
    `--explicit-local-private-intent`). The result names records of a private
    library, so it is local-private output, sized or refused.
- **Whole-document gate.** `control_local_private` classifies scalars by path tokens,
  and no member of this document matches one, so its walk would leave the document
  ungated. The verb therefore applies the walk's `$.report_package` rule to the whole
  document instead. It makes one decision at `$`:
  - without the flag, `block_export` (`LOCAL_PRIVATE_INTENT_REQUIRED`). No payload
    is emitted, nothing is written to `--output`, and the exit status is 1;
  - with the flag, `warning_only` (`PRIVATE_LOCAL_ALLOWED_WITH_WARNING`), and the
    document is the payload.
- **Byte cap.** The input is limited to 64 MiB. A larger input exits with status 2
  (`HANGER_SIZING_INPUT_TOO_LARGE`) before it is parsed. This limit is the verb's own,
  stated here; it is not an engineering value.
- **Input.** After admission (§8.3.1), the input is exactly
  `{"request": {"model": …, "materials": […]}, "sizing": <sizing request>, "library": <library document>}`.
  Each member is required, and no other member is admitted
  (`HANGER_SIZING_INPUT_SHAPE_INVALID`).
- **The library** must pass the import boundary
  (`library_import_document::validate_library_import`, kind `hanger`, intended
  visibility private) and be accepted. Otherwise the request is refused
  (`HANGER_SIZING_LIBRARY_NOT_ADMITTED`), with the boundary's finding codes and paths
  in the message.
- **The sizing request** must match `$defs.SizingRequest` exactly (§8.3.4). Otherwise
  the request is refused (`HANGER_SIZING_REQUEST_INVALID`), naming each path.
- **`--solver-mode`.** The runner's existing rule accepts it for `solve` only, so
  `size-hangers` refuses it as an unusable invocation (status 2). The request's
  `solver_mode` is the only source of the mode.
- **Identity, not authority.** The output's hashes identify the inputs. They
  authenticate no origin and claim no input manifest (P4).

#### 8.3.3 Identity, status and fixed text

- **Identity members**, with these exact values:
  - `document_kind`: `"openpipestress.hanger_sizing.result"`;
  - `schema_version`: `"1.0.0"`;
  - `semantic_contract_id`: `"openpipestress.result_semantics/hanger-sizing-1"`;
  - `route`: `"pressure_free"`. It is the only route sizing admits; an
    exact-route request is refused (`HANGER_SIZING_EXACT_ROUTE_PENDING`).
- **`limitations`** is this exact array of four strings:
  1. `"Hanger sizing selects records of the user's own library by the user's own tests on the stated cases; it judges no hanger's suitability for a service and determines no code compliance."`
  2. `"Sized means the sizing algorithm completed. A candidate may have no fitting record, and the verification may be conditional; both are stated in the result."`
  3. `"Loads and positions come from derived solves of the bound solve request and inherit every premise of those solves; nonlinear supports take each case's own converged state."`
  4. `"Hanger sizing is published on the pressure-free route only."`
- **`basis_statement`** is this exact array of two strings:
  1. `"Every position is measured from the drawn geometry, taken as the installed position at which each spring is set; F_cold is that setting and F_C the cold-condition load."`
  2. `"Quantities are SI: forces in N, lengths in m, rates in N/m; variations are ratios."`
- **`status`** discriminates two variants:
  - **`"sized"`** means the algorithm completed. It does not mean that every candidate
    fits, and it is no engineering acceptance.
  - **`"refused"`** means no table was formed.

**The sized variant** has exactly these members: the identity members, `limitations`,
`basis_statement`, `status`, `inputs` (every member non-null) and `result`.

**The refused variant** has exactly these members: the identity members,
`limitations`, `basis_statement`, `status`, `inputs` and `refusals`. In it:
- `inputs.sizing_request` is either the admitted request or `null`, and
  `inputs.sizing_request_sha256` is `null` exactly when the request is;
- `inputs.solve_request.sha256` is a hash or `null` (§8.3.1);
- `inputs.library` is `{library_id: string|null, sha256: string|null}`:
  - `library_id` is `null` when the input supplies no `hanger_library.library_id`
    string, or when the input was not admitted, in which case its id is not
    trusted;
  - `sha256` is `null` when the library could not be hashed;
- `refusals` is non-empty, each `{code, names, message}`:
  - `code` is one of the codes this design names;
  - `names` is an array of strings, possibly empty;
  - `message` is a non-empty string.

  Refusals keep the engine's order, which lists every finding together.

#### 8.3.4 Inputs and their binding

`inputs` has exactly these members:
- **`canonical_profile`**: `"openpipestress_jcs_ijson_v1"`.
- **`sizing_request`**: `$defs.SizingRequest`. Every member below is required, with no
  default:
  - `weight_case`: string;
  - `weight_support_state`: `"own_solve"`;
  - `design_operating_case`: string;
  - `check_operating_cases`: array of strings, possibly empty;
  - `candidates`: a non-empty array of `{support: string, family_policy: "variable"|"constant"|"variable_else_constant"}`;
  - `method`: `{load_design: "hot"|"cold", variation_reference: "operating_load"|"design_load", variation_limit: number, travel_allowance_m: number, selection_rule: "smallest_capacity"|"least_variation"}`;
  - `solver_mode`: `"sparse_interactive"|"dense_scrutiny"`.

  The value checks (a positive limit, a non-negative allowance, roles, duplicates) are
  the engine's, and their refusals are the engine's.
- **`sizing_request_sha256`**: the hash of the canonical `sizing_request`.
- **`solve_request`**: `{sha256}`. The solve request `{model, materials}` exactly as
  admitted. Its `materials` array is the collection the solve entry normalizes, so this
  hash binds the material input with the model. The model's cases, both the original
  ones and every case a derived solve keeps, are in it.
- **`library`**: `{library_id, sha256}`. `library_id` is the library document's
  `hanger_library.library_id`, and `sha256` is the hash of the whole document as
  admitted. The record array's order is part of the hash.

A selected record carries `hanger_id` and `record_sha256`, the hash of the whole record
object, the member of `hanger_records` whose `hanger_id` it is. A library whose
records repeat a `hanger_id` is refused by the engine
(`HANGER_SIZING_LIBRARY_RECORD_DUPLICATE`), so the lookup is unique.

#### 8.3.5 The sized result

`result` has exactly these members.
- **`verification_standing`**: `"unconditional"|"conditional"`. It is conditional
  exactly when some candidate's outcome is `no_record_fits`.
- **`cases`**: the named cases in this order: weight, design operating, then the
  checks in request order. Each is `{case, role: "weight"|"operating", load_categories, uses_condition_set}`:
  - `load_categories` is the case's authored primitive-load categories, sorted and
    unique, each a model-document load category;
  - `uses_condition_set` is true when the case carries a `condition_use`, whose
    temperature and added weight the solve lowers into the case (§2). The lowered
    loads themselves are not published.
- **`candidates`**: one per requested candidate, in request order, unique by `support`.
  Each has exactly:
  - `support`, `node`, `dof: "UX"|"UY"|"UZ"`, `family_policy`;
  - `outcome`: `"selected"|"selected_by_fallback"|"no_record_fits"`;
  - `w_n`, `u_cold_m`;
  - `operating`: one `{case, u_m}` per operating case, in operating order;
  - `span_m`: `[min, max]` with min ≤ max, and `required_travel_m`;
  - `failure_counts`: `{variable, constant}`. Each is `null` when that family was not
    tested; otherwise an object with exactly the keys E1, E2, V0, L, T, V (variable) or
    E1, L, T (constant), each a non-negative integer;
  - `selected`: `null` exactly when the outcome is `no_record_fits`; otherwise one of:
    - **variable**:
      `{family: "variable_spring", hanger_id, record_sha256, k_n_per_m, f_cold_n, f_cold_condition_n, envelope_n: [lo, hi], operating: [{case, f_n, variation}]}`,
      with one `operating` entry per operating case and every variation a number;
    - **constant**: `{family: "constant_effort", hanger_id, record_sha256, load_n}`,
      where `load_n` equals `w_n`. It has no per-case loads and no variation;
  - `governing`: `null` for no fit; otherwise `{travel_case, variation_case}`:
    - `travel_case` is the operating case with the largest |u_i − u_C|, ties going to
      the earlier case;
    - `variation_case` is the operating case with the largest v_i for a variable
      selection, ties to the earlier, and `null` for a constant selection;
  - `verification_assumption`: the role the verification solves gave the candidate,
    with its numbers. It is either `{role: "spring", k_n_per_m, f_cold_n}` (a variable
    selection) or `{role: "force", force_n}` with `force_n` equal to `w_n` (a constant
    selection or no fit);
  - `verification`: `null` exactly for no fit; otherwise:
    - variable:
      `{family: "variable_spring", u_cold_m, operating: [{case, u_m, f_n, variation: number|null}], span_m, required_travel_m, t_pass, f_cold_condition_n, departure_n, failed_tests}`.
      `failed_tests` is an ordered subset of `["V0", "L", "T", "V"]`;
    - constant:
      `{family: "constant_effort", u_cold_m, operating: [{case, u_m}], span_m, required_travel_m, t_pass, failed_tests}`.
      `failed_tests` is `[]` or `["T"]`.
- **`solves`**: exactly 2n + 3 entries for n operating cases, in this order: `S_W`,
  `S_C`, `S_O1` … `S_On`, `V_C`, `V_O1` … `V_On`. Each is
  `{label, case, derived_document_sha256, states}`.
- **`disclosures`**: zero, one or two entries. Each is
  `{code: "HANGER_SIZING_SUPPORT_STATE_DIFFERS"|"HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS", names, message}`,
  with `names` non-empty, unique and sorted bytewise. Each code appears at most once.

**Maps.** Each `states` object in `solves` maps a nonlinear support id, a non-empty
string, to `"inactive"|"active"|"sticking"|"sliding"`. It is empty when the model has
no nonlinear support. A state code outside these four refuses publication
(`HANGER_SIZING_RESULT_NOT_PUBLISHABLE`). No other dynamic-key map exists.

**Uniqueness and per-case lists.** Uniqueness holds within each array: candidate
`support`, case `case`, solve `label`.

Each of a candidate's three per-case lists has exactly one entry per operating case,
whose `case` ids equal the operating entries of `result.cases`, in the same order:
- `operating`;
- a variable `selected.operating`;
- `verification.operating` (variable and constant).

Each of these three result lists, where present, has n ≥ 1 entries, because the design operating case is always named. The request's `check_operating_cases` may be empty.

#### 8.3.6 Arithmetic, numerics and comparisons

All arithmetic is binary64, in exactly this order:
- **Hot setting:** F_cold = W + k·u_D. **Cold setting:** F_cold = W + k·u_C.
- **Loads:** F_C = F_cold − k·u_C, and F_i = F_cold − k·u_i.
- **Variation:** v_i = (k·|u_i − u_C|) / R_i.
- **Span:** min and max are taken over 0, u_C, then u_1 … u_n, in that order.
- **Required travel:** (max − min) + 2·a.
- **Envelope:** [F_cold − k·(max + a), F_cold − k·(min − a)].
- **Verified loads:** F^v = F_cold − k·u^v.
- **Departure:** F_D^v − W (hot) or F_C^v − W (cold).
- **Comparisons:** each test is a plain IEEE comparison (`<`, `>`, `<=`), with −0 equal
  to 0. Canonical text writes −0 as 0.

Readers recompute in this order and compare recomputed values with published ones by
numeric equality. Equal operations give equal binary64 results in both languages, so
no tolerance applies.

#### 8.3.7 What a reader establishes

Each standing is reported as passed, failed (with the first difference) or
`not_checked`. No reader states that mechanics are verified.

- **`closure`** (both readers):
  - the document matches the closed schema;
  - the variant and discriminator rules of §8.3.3 and §8.3.5 hold;
  - the cross-field invariants hold: standing against outcomes, `selected` against
    outcome, roles against selections, list cardinalities and orders, and
    uniqueness;
  - the fixed texts are byte-equal.
- **`binding`** (both readers):
  - `sizing_request` hashes to `sizing_request_sha256`;
  - if the library is supplied, it hashes to `library.sha256`, its `library_id`
    matches, and every selected `hanger_id` resolves uniquely to a record whose hash
    is `record_sha256`;
  - if the solve request is supplied, it hashes to `solve_request.sha256`.

  An input not supplied is `not_checked`.
- **`arithmetic`** (both readers; needs the library). From the published W and
  positions, the request and the library records, every value and outcome of §8.3.6
  is recomputed and compared:
  - the loads, variations, spans, envelopes and fit outcomes of every record;
  - the selection, the failure counts and the governing cases;
  - the verification arithmetic and its failed tests.

  This standing is reported as `arithmetic_consistent_with_published_positions`. It
  takes W, every position and every state as published. A document whose positions
  were invented consistently passes it.
- **`derivation`** (Rust reader only; needs the solve request). It rebuilds each
  derived document from the solve request, the sizing request and the published
  numeric roles, and compares its hash with `derived_document_sha256`. The roles are:
  - rigid for `S_W`;
  - a force `w_n` for `S_C` and `S_Oi`;
  - each candidate's `verification_assumption` for `V_C` and `V_Oi`.
- **`reproduction`** (Rust reader only; needs both the solve request and the library,
  each hash-matched). It re-runs the sizing from the bound inputs through the same
  engine and compares the whole result. Every non-numeric member must be equal, and
  every number equal as a binary64 number under §8.3.6's rule, with −0 equal to 0,
  because canonical text writes −0 as 0. This is the only standing that ties
  positions and states to the bound model.
  - A mismatch is reported as failed reproduction.
  - Reproduction is claimed only for the same engine build on the same platform. A
    valid document from another build may fail it.

**Refused documents.** For a refused document:
- `closure` checks the refused variant's rules;
- `binding` checks each non-null hash against its input when the input is supplied,
  and checks `sizing_request_sha256` only when `sizing_request` is non-null;
- `arithmetic`, `derivation` and `reproduction` are `not_checked`.

**Verdict parity.** The Rust reader and the Python verifier must return the same
verdict for `closure`, `binding` and `arithmetic` on every committed document and
mutation. `derivation` and `reproduction` belong to the Rust reader alone.

**Corpus.** `fixtures/results/hanger_sizing_*.json` holds the expected standings. It
includes:
- sized documents for each outcome, both families and a conditional standing;
- a refused document;
- mutations of an outcome, a record id, a count, a closed member, a fixed text and a
  hash;
- a **self-consistent forgery**, whose positions and every dependent value were
  recomputed together. It passes `arithmetic` and fails `reproduction`.

Reproduction of a genuine committed document is not asserted, because the document
may come from another build or platform. For each genuine sized entry, the test
publishes the document afresh from the committed input and checks that reproduction
passes on that fresh document. Every other standing is asserted on the committed
bytes.

#### 8.3.8 Compatibility

- The document is new and changes no existing version.
- Each existing reader's actual handling of the document is tested, not presumed:
  - the mechanics route reader and the result-export derivative refuse or disclose
    it, with their named codes;
  - the analysis-run builders do not accept it.
- Nothing is migrated, and no historical bytes or hashes change.

### 8.2 Hanger library format 1.1.0 (D-2)

DEL-03-02 owns the hanger library schema (`schemas/hanger.schema.yaml`, its CLM-030).
This design defines what sizing needs from it.
- **[decided]** `schema_version` admits `1.0.0` and `1.1.0`. A 1.1.0 record's `hanger` may
  carry `working_load_range: {minimum, maximum}`, each a ForceQuantity with its own
  provenance; no other member is admitted in the range.
- **[decided] Where the rules live** (slice 1b). The schema admits the member under
  either version. The import boundary, the Rust carrier and the Python checker alike,
  applies the rules JSON Schema cannot state:
  - in a 1.0.0 document, the member is refused as an unknown field, with today's finding
    (`IMPORT_HANGER_SCHEMA_INVALID`, "Unknown hanger field.") at its path;
  - a range whose minimum and maximum differ in unit is refused
    (`IMPORT_HANGER_LOAD_RANGE_INVALID`). The boundary converts no unit, and one
    catalogue record states its range in one unit;
  - a range whose minimum is not below its maximum is refused, under the same code. The
    ends are compared as binary64 numbers, the values sizing computes with, so ends that
    are distinct only beyond binary64 precision are refused;
  - the two range rules run only on a range whose own subtree passes the shape check. A
    malformed range gets its shape findings and nothing more.

  This keeps the two bounded schema interpreters on the keywords they already support.
  Their agreement with a full JSON Schema validator holds within the supported numeric
  domain, finite binary64 magnitudes. An integer beyond it, such as 10^400, is refused by
  the interpreters although a full validator admits it, as for every force quantity
  before 1.1.0.
- **[decided] Compatibility.** A 1.0.0 document still imports unchanged, and its records
  are listed by sizing as "not sizable: no working load range". A 1.0.0 document with
  `working_load_range` is refused as an unknown field, as today. A 1.1.0 document with a
  record lacking it imports; that record is not sizable.
- **[decided] Meaning.** `working_load_range` is the range of supported load over which
  the record is to be used. `travel_range` is the record's total working travel.
  `constant_load` in a constant record is not read by sizing: a constant record is
  selected by its range, and the sized load is W.
- **Consumers:**
  - **Changed in slice 1b:** the Rust import boundary
    (`core/library_import/library_import_document`, which embeds the schema), the Python
    provenance checker (`core/library_import/provenance_checker.py`), and the shared
    import cases (`fixtures/hanger/invented_hanger_cases.json`). The cases are read by
    `tests/test_hanger_library_schema.py` and the importer's Rust–Python parity test.
  - **With the desktop work (increment 3):** the library editor's draft documents
    (`libraryImportService.ts`, DEL-07-03), which still create 1.0.0 drafts and have no
    range field until then, and the desktop's typed record (`hangerSelection.ts`). A 1.1.0
    document imported from a file is accepted by the native boundary before then.

  The project library store keeps each document as imported (`upsert_library`), so a
  stored 1.0.0 document is read back as 1.0.0 and needs no migration.
- **Review.** A library format change is persistence. It takes targeted import
  compatibility regressions for 1.0.0 and an independent review of the exact changed
  format and commitment text (Agent 0, D-2; reviewer route as the owner selects).

## 9. Applying a selection

- **[decided] The person's act through the existing route.** The table offers each
  selected record for application. Application builds an operation batch with the
  existing hanger-selection builder (`apps/desktop/src/features/hanger-selection`). The
  person reviews its before and after and accepts it; nothing is applied otherwise.
- **The batch writes**, into existing model members only:
  - the record's family, stiffness, travel range, references and combined provenance,
    as the builder does today;
  - for a variable record, the sized `cold_load` and `installed_load` (both F_cold, the
    setting at the installed position, which is what the solve consumes, §11) and
    `hot_load` (F_D), in place of the record's own load entries. Where u_C is not zero,
    the cold-condition load F_C differs from F_cold. The table shows both, and the model
    stores the setting;
  - for a constant record, `constant_load` = W.
- **Provenance.** Each sized value's source note names the sizing run, its request
  digest and the record, so the model says where every number came from. The record's
  `working_load_range` is not written: the model document has no member for it.
- **The candidate keeps its id, node and DOF.** Application replaces its configuration,
  as the builder does for any support.

## 10. Cost and result volume (D-4 item 2)

- **Solves.** One request runs 2 + n derived solves (S_W, S_C and n operating cases), and
  1 + n verification solves: 2n + 3 in all. Each is one ordinary solve of one case.
- **[decided] Memory.** The solves run one after another. After each, only the values
  of §8.1 are kept: O(candidates × cases) values and O(nonlinear supports × cases)
  states. The solve's full result is dropped before the next solve starts. The peak is
  therefore one solve's peak plus one derived document; sizing adds no concurrent solve.
- **[decided] No retained-precision entry.** Derived solves request ordinary recovery
  only and never enter the retained Direct entry or D1, so the retained route's memory
  registration is unaffected. If increment 1 finds that the solve entry cannot run a case
  without the retained-source response, it reports to Agent 0 and re-prices under
  DEL-04-08's rules before building on it.
- **Time.** The time is the sum of the solves. On the provisional performance targets
  (2,000 nodes: 10 s linear, 60 s nonlinear), a nonlinear model with three operating
  cases needs 9 solves, about 9 minutes at the bound. The run reports progress per
  derived solve and can be cancelled between solves, under DEL-07-07's job contract; a
  cancelled run publishes nothing.
- **Result volume.** The table is O(candidates × operating cases) rows. It does not grow
  with the catalogue (§8.1) or the model's size except through the number of nonlinear
  supports. The performance lane agrees that it is a summary-sized table of the kind its
  result-volume design (option C) publishes by default (performance manager, 2026-10-31).
- **[decided] Where the solves run.** In the desktop, the derived solves run inside the
  solve worker, under the same per-invocation memory budget and cap refusal as any solve,
  not in the app process. No attestation digest is taken for a derived solve: its
  result is never published as a case result (performance manager, 2026-10-31).
- **[decided] One solve call site.** Every derived solve goes through one function in
  the sizing crate (§13). Today each solve pays the full publication (station rows,
  serialized envelope) that sizing then discards, and that publication dominates the
  cost of a large solve. The performance lane's design C will define a minimal
  publication for derived solves: nodal, support and support-state rows only. When it
  lands, that one call site switches to it. [open] O-2.
- **Timing evidence.** When the engine lands, a `hanger-sizing` profile is added to the
  DEL-04-05 rack harness (`core/solver/performance_harness/rack/rack_model.py`). It is a
  linear rack with some rests made variable spring candidates, carrying invented loads,
  plus a runner plan row. It is informative and not a criterion-9 item.

## 11. Variable spring preload (D-3)

**Decision** (Agent 0, 2026-10-31, D-3 (a), under the engineering-judgment and scope
delegations of `loop/LOOP_INIT.md`): a variable spring is modelled as its stiffness plus
its cold (preset) load at the model's reference position, the drawn geometry, which is
the installed position of §4.2. The solve consumes `hanger.cold_load` as a constant force
along the support's stiffness DOF, in every load case except where the support is held
(below), together with the spring's stiffness. The spring then carries
F = F_cold − k·u, or F_cold − k·(u − g) where its base moves by g. That is the same law
sizing uses, so an applied selection reproduces the table's verification in the model's
own cases.

**Reason.** That is the normative model of a set spring. Without it, a sized and applied
spring is a spring from zero load. The model's own sustained and operating cases would
then not reproduce the hanger table's loads, and the sizing result could not be checked
in the model. The question RT-S3 left open (`SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD`,
DEL-04-03 `Work/pf-nonlinear-imposed-motion.md`) is settled this way.

**What changes, built as increment 6 after the engine increments:**
- The force enters through the constant-effort seam: one ledger term per support,
  before the system is reduced, so dense, sparse and nonlinear solves consume it alike.
  The factor-sum gate applies to it in the same way: a combination whose factors would
  count a constant-effort force other than exactly once is withheld. RT-A3 shares the seam
  and is told when the increment starts.
- **Support-action recovery.** The published support force of a variable spring
  includes its preload: F_cold − k·(u − g), not −k·(u − g) alone. Today's recovery forms
  the spring's action from stiffness alone (`core/product_physics/src/lib.rs`, the
  spring support-action rows), so the increment changes the recovery and its readers
  with the assembly. Otherwise the displacements would be right and the reported hanger
  force wrong.
- `SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD` is retired. A new warning,
  `SPRING_HANGER_INSTALLED_COLD_LOAD_DIFFER`, is published when `installed_load` and
  `cold_load` differ. `hot_load` stays review data.
- **Behaviour change.** Documents saved before the change that carry `cold_load` solve
  differently afterwards. The PR and the release-notes material state this explicitly.
- **Evidence:**
  - closed-form references, with the published support forces of the ordinary model
    compared with the sizing verification, not only its displacements;
  - targeted regression on the existing spring-hanger tests, with a held and an unheld
    spring, showing that a held spring carries no preload;
  - re-pricing under DEL-04-08's rules, because it adds a ledger term;
  - an xhigh review.
- **Coordination.** The reference system's RS-VS1 KR-03 follows, and the increment-6 PR
  also changes the reference system's §6 sentence that calls the spring's loads review
  metadata never applied as preload. That wording is agreed with the reference-system
  lane before it merges. The hydrotest's held springs are the load-cases lane's
  (DEL-05-01 `Work/hydrostatic-test.md` §2). In a case with `held_supports:
  springs_and_constant_effort`, a held variable spring is a rigid restraint of its DOF,
  and neither its stiffness nor its preload acts. The seam therefore skips the preload of
  a held support, as it skips a held constant-effort force (load-cases manager,
  2026-10-31). The exact switch follows RT-A6's mechanics design. [open] O-3.
- **Commitment text.** DEL-04-03 records the decision now. Its commitment text is
  changed by increment 6, with that increment's review.

## 12. References (RT-V1)

The references are frozen under `validation/references/hanger_sizing/`. They are
derived by a generator that reads no product code, and refuted by a second TASK before
any product comparison. All values are invented.
- **R-1, rigid reactions:** two- and three-span continuous beams under uniform weight,
  closed form (Euler–Bernoulli). W at the interior supports.
- **R-2, free riser growth:** a vertical riser anchored at its base, with a hanger at its
  top and a horizontal leg whose far end is free vertically. The operating position is
  u = α·ΔT·H, and u_C = 0.
- **R-3, arithmetic:** F_cold, F_C, F_i, v_i, the fit tests and both selection rules
  over an invented catalogue, as exact identities. It covers:
  - hot and cold design, both variation references and the allowance;
  - a nonzero u_C, under both designs, with the one origin of §4.2;
  - multi-case governing, fallback and no fit;
  - F_C ≤ 0, F_i ≤ 0 and a DOF mismatch.
- **R-3b, W ≤ 0:** the whole request is refused, with every such candidate named.
- **R-4, verification pass:** a one-spring beam with a spring of rate k and force F_cold,
  closed form. It gives F_C^v, F^v, the verified variation formed from F_C^v and F_i^v,
  and the load redistribution.
- **R-4b, mixed verification:** a variable and a constant selection on one model, where
  the variable spring's redistribution moves the constant support beyond its travel
  range. The verified T test of the constant selection fails, and its warning is
  published.
- **R-5, lift-off:** a beam on a lift-off support beside a candidate, with the state
  differing between S_W and S_O. Each state and the disclosure are asserted, and the
  values follow the active-set solution in closed form.
- **R-6, preload (increment 6):** the spring model of §11 against the force-plus-spring
  derived solve of R-4. Displacements and published support forces must agree to
  rounding, including a moved spring base and a held spring that carries no preload.
- **R-7, the reference system:** RS-VS1 and RS-CE1 sized from an invented catalogue in
  `fixtures/reference_system/`, recorded as KR-10 once its families solve on EX. Its
  basis is a verified solve.

**Verification tolerance.** The product is verified against R-1 to R-6 at an analytic
tolerance under DEC-026, seeded at 1e-9 relative with stated absolute floors near zero,
measured and only tightened. R-3 is exact for the selection, which is a discrete
outcome.

## 13. Product layout and increments

- **Engine:** its own crate, `core/analysis/hanger_sizing`.
  - **[decided] Outside the solver library.** It changes no file in `core/product_physics`.
    Two reasons: the retained route's pricing carries that crate's sources line by line,
    and the product's public JSON solve entry
    (`run_linear_static_preview_value_with_mode`) already runs the solve-time lowering on
    the ordinary route (§4.1). Derived documents go through that entry from this crate.
  - **Slice 1a, built:** the arithmetic of §6 and §7 as pure functions on W, the positions
    and the SI-normalized records:
    - the request checks that need no solve;
    - the fit tests in order, the selection and the failure counts;
    - the verification arithmetic and its standing.

    The detail view and the readers share `evaluate`. The crate has no product
    dependency.
  - **Slice 1c, built:** `derived.rs` and `library.rs`:
    - the request, the candidate and case checks;
    - derived-document construction and the one solve call site;
    - the sizing run with its verification and disclosures;
    - the library reader, which normalizes through the unit catalog.

    These add the path dependency on `product_physics`, which brings the crate into
    the solver's reachability for CI selection; no `product_physics` file changes. The
    run's results are a typed `SizingRun`.
  - **From slice 1d:** publication. That means the closed result document, the
    readers of §8.3 and the headless `size-hangers` entry (slice 1d-α). The native
    desktop command follows with increment 3.
- **Readers:** `core/analysis/hanger_sizing/src/reader.rs` (the Rust reader; a reader in
  `result_export` would form a dependency cycle through `product_physics`) and
  `tests/verifiers/hanger_sizing_evidence.py` (§8.3).
- **Desktop:** the request form, the hanger table and its detail view, in
  `apps/desktop/src/features/results/` (DEL-07-05). Application goes through
  `features/hanger-selection`.
- **Report:** a "Hanger sizing" section with the request, the basis, the table and the
  disclosures (DEL-08-01).
- **Increments:**
  0. This design and its commitment text.
  1. The engine on PF, with library format 1.1.0, R-1 to R-3, the readers and refusal
     tests.
  2. The verification pass and multi-case sizing, with R-4 and R-5.
  3. The desktop request, table, CSV and application, and the report section.
     - **3a, built:** the desktop's types for the result document and a pure display
       adapter (`features/results/hangerSizingResult.ts`, `hangerSizingView.ts`), tested
       on the committed family fixtures. It gives §8.1's display model: the basis block,
       one row per candidate, the solves, the standing and the disclosures, or a refused
       document's refusals. It keeps every published number and computes nothing. A
       record's details come only from a supplied library whose checked canonical hash
       equals `inputs.library.sha256`, and a record only when its own hash equals
       `record_sha256`; otherwise a named not-shown state. The readers' standings are shown
       only from evidence that carries the document's checked hash; otherwise every standing
       is `not_checked`. It is not a usable desktop feature.
     - **3b, built:** the table component and its CSV through DEL-08-04
       (`features/results/HangerSizingPanel.tsx`, `hangerSizingCsv.ts`), tested on the same
       fixtures. The panel renders 3a's display model read-only: the basis block, one row
       per candidate with the exact F_cold and F_C labels, each candidate's details
       (`<details>`), the solves, the verification standing (a conditional one stated with
       what it means), the disclosures and the reader standings, or a refused document's
       refusals and no table. Values are 3a's published SI texts, with nothing rounded,
       converted or computed. The CSV is kind `hanger-sizing`, with two files:
       `swbpipe-hanger-sizing.csv` has one row per candidate and operating case, and
       `swbpipe-hanger-sizing-solves.csv` has one row per solve and nonlinear support. Its
       numbers are written unrounded under fixed SI header units. Each absent value is a status
       cell carrying the panel's reason. `result_hash` is the document's checked canonical hash.
       The panel is not mounted; 3c mounts it.
     - **3c:** the native sizing job and its command, and mounting the 3b panel.
       - **Part 1, built:** the crate's native size-and-read entry,
         `desktop::size_and_read(raw: &[u8])` (`core/analysis/hanger_sizing/src/desktop.rs`).
         Its input is the raw bytes of exactly the headless input (§8.3.2):
         - over `MAX_INPUT_BYTES` it is refused before anything else
           (`HANGER_SIZING_INPUT_TOO_LARGE`);
         - non-UTF-8 bytes are refused (`HANGER_SIZING_INPUT_NOT_UTF8`).

         Either refusal is a typed error with a code and a message, and forms no
         document. Otherwise the text is admitted once and goes through the headless
         pipeline itself, so the document is what `size_hangers` publishes for the same
         text. The reply is one JSON text:
         `{"document", "evidence": {"document_sha256", "verdict"}}`.
         - `document_sha256` is the document's checked canonical hash.
         - `verdict` is `Verdict::to_json()` of the Rust reader. The reader runs with the
           same input's `request` and `library` supplied.
         - Reproduction is skipped explicitly (`ReadOptions::WITHOUT_REPRODUCTION`) and is
           `not_checked`; `read` and `read_document` still run every standing.
         - A refused document gets closure and binding only (§8.3.7).

         The reply is written so that every number round-trips, and its byte size is
         `reply_len()`. Its tests are `tests/desktop.rs`, on the committed corpus inputs.
       - **Part 2, built:** runs that entry in the desktop's capped worker process as
         worker task 4, returns its reply to the webview, and mounts the panel with the
         reader standings bound to the document. Hanger sizing is then usable in the
         desktop, on PF, and is not recorded (Agent 0's rulings for 3c part 2):
         - **The path** (`apps/desktop/src-tauri/src/hanger_sizing_job.rs`): the model
           checks' capped-worker form. `size_hangers` takes `{runId, inputText}` as a raw
           body, where `inputText` is the headless input's JSON text, and replies the
           entry's reply text. There is no job record, held-result registry, custody,
           persistence or analysis-run record.
         - **Task 4** (`solve_worker.rs`): appended after the MBF render's 3; codes 0 to 3
           are unchanged and unknown codes are refused. It attests nothing (`ATST` status
           0), and a `SizeAndReadError` is the worker's `FAIL` with its code and message.
         - **The reply limit:** 64 MiB (`MAX_DISPLAYABLE_RESULT_BYTES`) on the complete
           reply, checked on the announced length before the result lease is admitted or
           any byte is read. A longer reply ends `HANGER-SIZING-REPLY-TOO-LARGE`, with the
           worker reaped. A larger result would need a custody proposal of its own.
         - **Accounting:** the argument, worker and result leases count in one command
           ledger under the captured invocation budget, as the MBF export's task 3 does:
           raw admission before any model-sized decoding, one ledger bound for the request
           copy, the grant's maximum and the largest result lease, a start wait of the
           model checks' 2 s, the grant held until the worker is reaped, and the result
           lease held while the response is built.
         - **Cancellation:** one current sizing, in a slot of its own. A newer request
           supersedes the one in progress, queued or running. `cancel_hanger_sizing`
           cancels a page's requests through a sequence; it is keyed by `{page, sequence}`,
           so a repeated or late delivery of a cancel or a request cannot reach a newer
           request. The session reset cancels the sizing beside the model check, and the
           webview cancels it when the project session changes. The webview's generation
           guard (`services/hangerSizingService.ts`) never shows a reply that a newer
           request or a retirement has passed. Per-solve progress (§10) is not built.
         - **The form** (`features/hanger-sizing/`) has no pre-set choice in a new project
           and pre-fills nothing (no value is kept between sessions yet). It sends the
           session model with `materials: []`, as the desktop solve does, and the stored
           library document chosen through `openLocalLibrary`.
         - **Tests:** Rust (`hanger_sizing_job_tests.rs`): the committed two-span input
           through the command and a real worker, whose reply is pinned as
           `fixtures/results/hanger_sizing_reply_two_span.json`; the reply limit's
           boundary; admission and ledger refusals; supersession, the keyed cancel and the
           reset. Vitest: the service, the form and the App mount, with that reply shown
           by the panel with its standings bound.
  4. EX derived solves (the ledger entry of §4.1) and the reference system, R-7. This
     needs RT-A3 increments 3 and 4 for the reference system's non-candidate families.
  5. The user guide and theory notes (RT-V4), and any standing refusal entered in
     `docs/RELEASE_LIMITATIONS_v1.0.md` in the PR that makes it standing.
  6. Spring preload (§11), with R-6.

## 14. Open items

| Id | Item | Owner |
|---|---|---|
| O-1 | `weight_support_state: design_operating_final`, through the load-cases lane's held-status solve function (RT-A5 Should, the last of that lane's slices), whose contract that lane sends when drafted | load-cases lane, then this deliverable |
| O-2 | The minimal publication for derived solves (§10), defined in the performance lane's result-volume design C, which awaits Agent 0's approval; sizing switches its one solve call site to it when it lands | performance lane |
| O-3 | The switch that skips a held spring's preload in a hydrotest case (§11), from RT-A6's mechanics design | load-cases lane, with this deliverable |
| O-4 | The invented reference-system catalogue, and the KR-03 and KR-10 updates | reference-system lane, with this deliverable |
| O-5 | Release of a report that carries private-library values from the table: the report-user-release privacy rules for library references apply (FR-REP-003). Confirmed with DEL-08-01 before the report section is built | DEL-08-01 |
| O-6 | A request entry in the embedded agent's capability catalog (RT-E1) | RT-E1 lane |
