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

- **[decided] Case roles.** Every named case is a load case of the model, not a
  combination, and is solvable on the model's route. A case carrying a `test_condition`
  (RT-A6) is refused in either role (`HANGER_SIZING_TEST_CASE_UNSUPPORTED`): a held
  support is not the state a hanger is sized in. The weight case may not also be an
  operating case, and an operating case may be named only once
  (`HANGER_SIZING_CASE_ROLES_OVERLAP`). An unknown case is refused by name
  (`HANGER_SIZING_CASE_UNKNOWN`).
- **[decided] The weight case's content is the person's.** Sizing does not inspect
  whether the weight case holds only weight. The result lists the load kinds the case
  carries, after solve-time lowering (§4.2), so that a case with thermal or other loads
  is visible as such.
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
    removed. Increment 1 fixes the family token from those both routes admit as a
    rigid single-DOF restraint. On an EX load/reference-state document, its ordinary
    support-state entry is kept unchanged.
  - *Force candidate* (solves S_C and S_Oi): the candidate support is removed, and a
    nodal force of magnitude W along the positive axis of its DOF is added to the solved
    case at its node. On an EX load/reference-state document:
    - the candidate's support-state entry is removed with it, because a support state
      that names no support is refused (`LOAD_STATE_SUPPORT_UNKNOWN`);
    - the force is entered in that case's `analysis_state.load_sources` ledger with
      factor 1, so that it is in the case's declared source ledger.
  - *Selected candidate* (solves V_C and V_Oi, §7): see §7.
  - Each derived document keeps only the case it solves and no combinations. Added
    primitive ids are in a reserved namespace (`load:hanger-sizing:…`) that is disjoint
    from every existing id; a collision is refused (`HANGER_SIZING_ID_COLLISION`).
- **[decided] Never persisted.** A derived document is never saved, never returned as a
  model, never enters undo history and never passes through the operation route.
- **[decided] Solve settings are the model's.** Each derived solve uses the route, solver
  mode and settings the model's own solve of that case would use. It requests no
  retained-precision recovery (§10).

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

  Each failed test, formed with the verified values, is reported by name
  (`HANGER_SIZING_VERIFICATION_TEST_FAILED`, warning):
  - V0, L, T and V for a variable selection;
  - T for a constant selection; its L is unchanged, because its load stays W.
- **[decided] No automatic iteration.** The selection stands as sized, and the
  verification reports how it behaves. Re-sizing after applying a selection is the
  person's choice: the next request then sees the applied springs as candidates or as
  ordinary supports, as the person names them. An automatic loop would change
  selections without a stated convergence rule, and no such rule is offered.

## 8. Library and results

### 8.1 The hanger table and its basis

**Per candidate**, one row:
- the support, node and DOF, and the family policy;
- the outcome: `selected`, `selected_by_fallback`, `no_record_fits` or `refused`, with the
  refusal;
- the library id, `hanger_id` and name, and the record's type;
- for a variable record: k, [F_min, F_max] and T;
- W, F_cold (variable) or the constant load W (constant);
- u_C and F_C; for each operating case: u_i, F_i and v_i; the governing case for travel and the one
  for variation;
- the required travel span + 2a;
- the verification's assumed configuration, and the verified positions, travel, loads,
  variations and test outcomes of §7, for every selected family;
- for a candidate with no fit or a fallback: the number of records failing each test,
  E1 to V.

**Per derived solve:** u_C at each candidate, and each nonlinear support's state (§5).

**The basis block** gives every request input (§2), the library id and digest, the model
revision hash, the route, the case ids and the load kinds of each case after lowering.

- **[decided] Per-record evaluations are recomputed, not stored.** Selection is a pure
  function of W, u_C, the u_i, the request and the library. The desktop's detail view for one
  candidate calls the same Rust function to list every record with its first failed test,
  so the table's size is independent of the catalogue's size (§10).
- **Publication.** Results are published under the profile
  `product_preview_hanger_sizing_v1`, with fixed limitation text, and the semantic table
  `openpipestress.result_semantics/0.3.0/hanger-sizing-1`. The analysis-run record
  gains a hanger-sizing run kind that carries the request. These join the result
  envelope under DEL-00-06's contract and its one-version rule.
- **[decided] Readers.** A Rust product reader and a test-only Python verifier, which
  shares no code with the producer, each recompute from the published W, u_C, u_i, the
  request and the library document:
  - F_cold, F_i and v_i;
  - every fit test of the selected record;
  - the selection.

  They do not re-run a solve.
- **CSV.** The hanger table exports to CSV under DEL-08-04's serializer and coverage
  guard (RT-R6).

### 8.2 Hanger library format 1.1.0 (D-2)

DEL-03-02 owns the hanger library schema (`schemas/hanger.schema.yaml`, its CLM-030).
This design defines what sizing needs from it.
- **[decided]** `schema_version` admits `1.0.0` and `1.1.0`. A 1.1.0 record's `hanger` may
  carry `working_load_range: {minimum, maximum}`, each a ForceQuantity with its own
  provenance. The import boundary refuses minimum ≥ maximum by name
  (`IMPORT_HANGER_LOAD_RANGE_INVALID`), because JSON Schema cannot compare the two.
- **[decided] Compatibility.** A 1.0.0 document still imports unchanged, and its records
  are listed by sizing as "not sizable: no working load range". A 1.0.0 document with
  `working_load_range` is refused as an unknown field, as today. A 1.1.0 document with a
  record lacking it imports; that record is not sizable.
- **[decided] Meaning.** `working_load_range` is the range of supported load over which
  the record is to be used. `travel_range` is the record's total working travel.
  `constant_load` in a constant record is not read by sizing: a constant record is
  selected by its range, and the sized load is W.
- **Consumers to change together:**
  - the Rust import boundary (`core/library_import/library_import_document`, which
    embeds the schema);
  - the Python provenance checker (`core/library_import/provenance_checker.py`);
  - the desktop library editor's draft documents (`libraryImportService.ts`, DEL-07-03);
  - the desktop's typed record (`hangerSelection.ts`);
  - the import tests (`tests/test_hanger_library_schema.py`,
    `tests/test_library_import_provenance.py`, the importer's parity tests).

  The project library store keeps each document as imported (`upsert_library`), so a
  stored 1.0.0 document is read back as 1.0.0 and needs no migration.
- **Review.** A library format change is persistence: it takes an xhigh review and
  import compatibility tests for 1.0.0 (Agent 0, D-2).

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
  `hanger_sizing.rs`. Today each solve pays the full publication (station rows,
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

- **Engine:** `core/product_physics/src/hanger_sizing.rs` and `hanger_sizing/**`:
  - the request DTO and refusals;
  - candidate checks;
  - derived-document construction;
  - the solve loop;
  - the arithmetic and selection, as one pure function shared with the detail view;
  - the result rows.

  It has a headless runner entry and a native desktop command.
- **Readers:** `core/reporting/result_export` (the Rust reader) and
  `tests/verifiers/hanger_sizing_evidence.py`.
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
