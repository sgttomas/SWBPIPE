---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-11
package_id: PKG-04
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-083]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-11 Hanger sizing

## Purpose and Objective Traceability

This deliverable sizes variable spring hangers and constant-effort supports from the user's own imported hanger catalogue and produces a hanger table. An engineer names the supports to size and the cases to size them for, and gets, for each, the catalogue record that meets the engineer's own load, travel and variation tests, or a statement that none does. It works from a weight case solved with the hanger locations rigid and from operating cases solved with the hangers replaced by their loads. Every method choice comes from the user. Sized values are reported as results with their basis, and the model changes only when the person applies a selection.

It serves SOW-083 under OBJ-003 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-04, row DEL-04-11), and PRD §12.4. In `docs/RELEASE_TARGET_v1.0.md` it is the owning deliverable for hanger sizing. It contributes exit criterion 1 (the reference system's hangers sized on the exact route in the desktop) and criterion 2 (closed-form references frozen in `validation/references/` with an independent generator), with its refusals listed under criterion 3 and its table exported to CSV under criterion 5. The required capability is to size hangers from the user's private catalogue, with invented example records only in the repository, and to produce a hanger table.

- **OUT-001** — The sizing request and its candidates (REQ-001, REQ-002).
- **OUT-002** — The derived solves, with the treatment of nonlinear supports (REQ-003).
- **OUT-003** — Loads, fit, selection and the verification pass (REQ-004, REQ-005).
- **OUT-004** — The hanger table, its basis, readers and CSV export, and application of a selection (REQ-006 to REQ-008).
- **OUT-005** — The admitted domain, frozen references, cost bounds, the desktop and report surfaces, and the claim boundary (REQ-009 to REQ-013).
- **OUT-006** — Variable spring preload in the solve (REQ-014).

## Deliverable Definition — Ontology

### CLM-001 — The method

For the candidate supports the person names, the sizing runs these steps:
1. **Hot-load basis.** The weight case is solved with every candidate made a rigid restraint of its single translational DOF. Each candidate's reaction is its hot-load basis W.
2. **Positions.** The weight case (the cold condition) and each named operating case are solved with every candidate replaced by a force W. Each candidate's positions are its displacements there, measured from the drawn geometry, which is the installed position.
3. **Variable records.** A variable spring set to its cold load at the installed position carries that load minus its rate times its displacement, the one law used throughout.
   - The cold load is chosen so that the spring carries W in the design operating case (hot design) or in the cold condition (cold design).
   - The load variation is the change of load between the cold condition and each operating case, over the person's reference load.
4. **Fit.** Each catalogue record is tested against its working load range, its travel range with the person's allowance, and the person's variation limit.
5. **Selection.** A fitting record is selected by the person's rule.
6. **Verification.** The selections are re-solved in place and the loads they carry are reported.

Every derived solve is a modified copy of the model document, sent through the model's own solve entry and never persisted.

### CLM-002 — What this deliverable owns, and what it takes as input

**This deliverable owns:**
- the sizing request, its refusals and disclosures;
- the derived-document construction and the solve loop;
- the sizing arithmetic, fit tests and selection;
- the verification pass;
- the hanger table, the closed hanger-sizing result document and its schema, and the hanger-sizing analysis-run kind;
- the product's reader of that document and its independent test verifier;
- the sizing engine and its headless and desktop entries;
- the frozen references under `validation/references/hanger_sizing/`;
- the sizing meaning of the hanger library's `working_load_range`;
- the desktop request form and hanger table, and the report's hanger-sizing section, within the results viewer's and report generator's contracts;
- the rest of the variable spring's preload law (REQ-014): held springs, recovery, diagnostics and the combination gate. DEL-04-03/REQ-014 states the spring-plus-cold-load model.

**Inputs from neighbours** (`deliverable.yaml`): DEL-04-07 (the product solve entries on both routes), DEL-04-03 (support families, the constant-effort seam and spring assembly), DEL-04-04 (nonlinear support states), DEL-05-01 (load cases and their solve-time lowering), DEL-02-01 (the support and hanger members of the model document), DEL-03-02 (the hanger library schema), DEL-03-07 (the library import boundary), DEL-02-02 (units), DEL-00-06 (the diagnostics contract), DEL-14-02 (the analysis-run record), DEL-04-08 (the retained-memory pricing law), and DEL-07-05, DEL-07-07, DEL-08-01, DEL-08-04, DEL-10-05 and DEL-04-05 (the viewer, solve worker, report, CSV, headless runner and timing harness). These deliverables keep their scope.

### CLM-003 — What is outside these commitments

- Forming W in the design operating case's support state.
- Automatic re-sizing iteration.
- Hangers on skewed or rotational DOFs.
- Any judgment of a hanger's suitability, and any shipped catalogue, limit, allowance or rule.

## Completion and Reliance Basis — Epistemology

A sizing result is a numerical statement about the discrete model under the user's cases, catalogue and method choices. It inherits the premises and limitations of each solve it uses, and bounds no modelling, input or physical error. A selected record is the one that meets the user's own tests; SWBPIPE does not judge any hanger's suitability, and nothing here determines code compliance.

**Undecided.**

- Whether a sizing run is recorded as an analysis run is not decided. The hanger-sizing analysis-run kind awaits adoption in the analysis-run record, and DEL-14-02 leaves the question open.
- Whether the report's privacy rules for library references cover the hanger section's library-derived values is not confirmed with DEL-08-01.

**Selected and sequenced.** SWBPIPE's Agent 0 selected the hanger implementation lane (2026-11-12), and no hold stands on it. The spring preload in the solve (REQ-014) follows the change that makes pressure elongation a user option on the exact route (DEL-04-07), because both change the same solve sources and every change to them is priced again (DEL-04-08/REQ-004).

### Request

- **REQ-001** — *Required inputs, no defaults.*
  - The request names:
    - one imported hanger library;
    - the candidate supports, each with its family policy (variable, constant, or variable else constant);
    - the weight case and the support state W is formed in;
    - the design operating case and the further operating cases to check (stated, even when empty);
    - the load design (hot or cold);
    - the variation limit and its reference load (each case's operating load, or the design load, which is the hot-load basis W);
    - the travel allowance at each end of the travel span;
    - the selection rule (smallest capacity or least variation);
    - the solver mode of the caller's own solve, which every derived solve uses.
  - Each absent or invalid input is refused by name. The engine, the headless runner and the native entry supply no value. The desktop pre-fills the values last used in the project, and the request records the values used.
  - Named cases are load cases, not combinations. A test-condition case is refused, and so is a case named in two roles.
  - The request is recorded with the result.
- **AC-001** — Maintained tests show, at the product facade, the headless runner and the desktop:
  - each refusal for a missing or invalid input;
  - the case-role refusals;
  - no default supplied on any entry.
- **REQ-002** — *Candidates.*
  - A candidate acts on exactly one translational DOF: its single rigid restraint, or, for a variable spring hanger with no rigid restraint, the DOF its stiffness declares. It has no skew direction, is not nonlinear, a snubber or an anchor, shares its node and DOF with no other support, and is referenced by no other model object.
  - Apart from that DOF, its present family and hanger data are not read.
  - On an EX load/reference-state document, it has in every named case an ordinary support state: active, with no imposed motion or device reference. That entry is the support's own declaration, not a blocking reference.
  - Its force acts along the positive axis of its DOF.
  - Each failure is refused by name, and refuses the request. A hot-load basis that is not positive also refuses the request, naming every such candidate.
- **AC-002** — Each candidate refusal, the moved-candidate refusal on EX and the non-positive hot-load refusal are exercised by maintained tests that assert the code and the named supports. A placeholder support of each admitted family is sized, and so is an authored variable spring hanger with no rigid restraint.

### Solution

- **REQ-003** — *Derived solves.*
  - Each derived solve is a modified copy of the stored model document, sent through the product solve entry the model's own cases use. It has the model's route and settings and requests no retained-precision recovery.
  - It is never saved, never returned as a model, never in undo history and never passed through the operation route.
  - On an EX load/reference-state document, a candidate made rigid keeps its support state, and a candidate replaced by a force loses its support state with the support. Each added force is entered in its case's source ledger.
  - Nonlinear supports converge in each derived solve to the states that case produces.
  - The support state W is formed in is a required input. Only the weight case's own state is admitted.
  - The result publishes each nonlinear support's state in every derived solve. It warns by name where a state differs between the weight solve and the design operating solve, or between the weight solve and the cold reference.
  - A derived solve that is refused, fails to converge or publishes a blocking diagnostic refuses the request, with its own diagnostics. No partial table is published.
- **AC-003** — Maintained tests show:
  - each kind of derived document, and that the stored model is byte-identical after a run;
  - that the solve-time lowering is applied to derived documents;
  - on EX, the support-state reconciliation for rigid, force and spring candidates, and the ledger entry, with each derived document admitted by the solve;
  - the state disclosures and warnings on a lift-off model;
  - the refusal on a failed derived solve.
- **REQ-004** — *Loads, fit and selection.*
  - **One origin.** Every position is measured from the drawn geometry, the installed position. A variable spring set to its cold load F_cold carries F_cold − k·u at position u, in sizing, verification, application and the solve.
  - **Loads.** Under hot design, F_cold is chosen so that the spring carries the hot-load basis in the design operating case. Under cold design, it is chosen so that the spring carries it in the cold condition. The cold-condition and operating loads follow from the law.
  - **Variation.** Load variation is the difference between the cold-condition load and each operating load, over the chosen reference load.
  - **Variable record fit.** A variable record fits when, in order:
    1. it carries a rate, a working load range and a travel range;
    2. its stiffness DOF matches the candidate;
    3. the cold-condition load and every operating load are positive;
    4. the loads over the travel span, widened by the rate times the allowance at each end, lie within its working load range;
    5. the travel span (from the least to the greatest position over the installed position, the cold condition and every operating case) plus twice the allowance is within its travel range;
    6. every variation is within the limit.
  - **Constant record fit.** A constant record fits when it carries a working load range and a travel range, the hot-load basis lies within its range, and the travel test is met.
  - **Selection.** It follows the family policy and the selection rule, and is deterministic. Smallest capacity takes the smallest maximum working load, then the smallest largest variation, then the smallest record id in byte order. Least variation takes the smallest largest variation, then the smallest maximum working load, then the smallest record id. A constant record's variation counts as zero. Variable-else-constant tests constant records only when no variable record fits, and the result says the fallback was taken. The first test a record fails, in the order above, is its stated reason. With no fit, the outcome is "no record fits", with the count of records failing each test, and nothing is assumed in its place.
- **AC-004** — On the frozen references, the product reproduces, within the verification tolerance of REQ-010:
  - the rigid reactions;
  - the free-riser travel;
  - every load, variation, fit outcome and selection, the selection exactly.

  This holds over hot and cold design, both reference loads, the allowance, a cold-condition position that is not zero, multi-case governing, fallback, no fit, non-positive loads and a DOF mismatch. Mutation tests show each method choice changing the result as its rule predicts. Results are byte-identical on repeated runs.
- **REQ-005** — *Verification pass.*
  - **Solves.** The weight case and each operating case are solved again with:
    - each selected variable record as a spring of its rate with a force equal to its cold load, the same law as REQ-004;
    - each constant selection as a force equal to its hot-load basis;
    - each unfitted candidate as a force equal to its hot-load basis.
  - **Basis.** The result states the configuration assumed for each candidate. When a candidate has no fit, the verification is marked as conditional on that assumption, which applying the selected records alone does not reproduce.
  - **Every selected family.** The verified positions, travel span and travel test are published for every selected family.
  - **Variable selections.** The verified cold-condition and operating loads are published, with:
    - the verified variations, each the difference between the verified cold-condition load and that case's verified operating load, over the reference load;
    - the departure from the hot-load basis.
  - **Warnings.** Each fit test that the verified values fail is warned by name: the travel test for a constant selection, and the load, travel and variation tests for a variable one.
  - No selection is changed automatically.
- **AC-005** — The following are reproduced:
  - the one-spring closed-form reference, with its verified variation formed from both verified loads;
  - a mixed variable and constant reference, in which the constant support's verified travel fails its test and warns;
  - a lift-off case.

  A selection whose verified values fail a test produces its warning, and the selection is unchanged. The conditional marking appears when a candidate has no fit.

### Library, results and application

- **REQ-006** — *The sizing meaning of the hanger library.*
  - **Working load range.** Sizing reads a record's `working_load_range`, which the hanger library format 1.1.0 carries (DEL-03-02, DEL-03-07). A record without one, in either library format, is listed as "not sizable: no working load range".
  - **Constant records.** Sizing does not read a constant record's `constant_load` or any record's `movement_limit`, and the result says so.
- **AC-006** — Every record without a working load range, a 1.0.0 record included, is listed as not sizable; a record with one is tested against it; and the result states that `constant_load` and `movement_limit` were not read.
- **REQ-007** — *The hanger table, its basis and readers.*
  - **The table**, one row per candidate, gives:
    - the outcome and the selected record;
    - the hot-load basis, the cold or constant load, and each case's operating load, travel and variation, with the governing cases;
    - the required travel;
    - the verified values;
    - for an unfitted candidate or a fallback, the count of records failing each test.
  - **Derived solves.** The result also gives each candidate's cold-reference displacement and each nonlinear support's states.
  - **Basis.** A basis block states:
    - every request input;
    - the library id and hash;
    - the solve request's hash, which identifies the exact model and materials;
    - the route;
    - each named case's role, its authored load categories and whether it uses an operating-condition set.

    It also states the installation assumption, that each spring is set at the drawn geometry. The table labels the installed setting separately from the cold-condition load.
  - **Document.** Results are published as a closed hanger-sizing result document. It embeds the sizing request, binds the solve request and library by hash, and has two statuses: `sized`, meaning the algorithm completed (never acceptance, and always with each outcome and the verification standing), and `refused`.
  - **Recording.** Until the analysis-run record adopts the hanger-sizing run kind (see Undecided), a sizing result is not a recordable run; the request it embeds is its record.
  - **Readers.** A Rust product reader and a test-only Python verifier that shares no code with the producer. Each establishes the document's closure, its binding to the inputs supplied, and arithmetic consistent with the published positions. The Rust reader alone can establish reproduction from the bound inputs. No reader states that the published positions are the model's mechanics without reproducing them.
  - **CSV.** The table exports to CSV through DEL-08-04's serializer.
- **AC-007** — Maintained tests show:
  - the two readers returning the same closure, binding and arithmetic verdicts for every committed result and mutation (derivation and reproduction are the Rust reader's alone);
  - a self-consistent forgery of the positions that passes the arithmetic standing and fails reproduction;
  - that the Python verifier imports no producer code (an independence test);
  - each existing reader's actual named refusal or disclosure of the document, as tested;
  - the table in DEL-08-04's CSV coverage guard.
- **REQ-008** — *Applying a selection.*
  - Applying a selected record is the person's explicit act. It goes through the existing hanger-selection operation builder and the one operation route, and is applied only on the person's acceptance. The selections of one sizing run are applied together, as one batch on one acceptance that one undo reverses; a candidate with no fit is left unchanged and named. A selection is offered for application only while the model and the stored library are the ones the result binds.
  - The batch writes the record's family, stiffness, travel range, references and provenance, and the sized loads, all into existing model members:
    - for a variable record, the cold and installed loads (both the sized cold load) and the hot load;
    - for a constant record, the constant load (the hot-load basis).
  - Each sized value's source note names the sizing run, its request digest and the record; one note may serve all the sized values of one selection. Sizing itself never writes the model.
- **AC-008** — Desktop tests show:
  - the batch's before and after for a variable and a constant selection, with the sized values and their source notes;
  - nothing applied without acceptance;
  - one run's selections applied as one batch, with one undo restoring the model;
  - no selection offered when the model or the stored library differs from what the result binds;
  - the model unchanged by a sizing run.

### Domain, references, cost and surfaces

- **REQ-009** — *Admitted and refused.*
  - Sizing admits what the model's own solve admits, on PF and EX, plus the candidate rules of REQ-002.
  - What the solve refuses refuses the request with the solve's code.
  - Each standing refusal is listed in `docs/RELEASE_LIMITATIONS_v1.0.md`.
- **AC-009** — Each refusal and disclosure is exercised by a maintained test that asserts its code and named object, on both routes.
- **REQ-010** — *References.*
  - **Independence.** The references are frozen under `validation/references/hanger_sizing/`, each derived, with its generator, from this scope without reading product code; the generator imports none.
  - **Coverage.** They cover:
    - rigid reactions of continuous beams;
    - free riser growth;
    - the sizing arithmetic and selection;
    - the verification pass;
    - a lift-off case;
    - the spring preload;
    - the reference system's hangers on EX.
  - **Values.** All values are invented.
  - **Tolerance.** Values agree with the references to 1e-9 relative with an absolute floor (the governed analytic class); selections and fit outcomes agree exactly. The tolerance is only tightened, and a reference is never altered to make an implementation pass.
- **AC-010** — The product agrees with every frozen reference package at the stated tolerance, and each generator imports no product code.
- **REQ-011** — *Cost.*
  - A request runs at most 2n + 3 ordinary solves for n operating cases, one after another.
  - After each solve, only the values the table needs are kept.
  - Derived solves never enter the retained-precision entry.
  - In the desktop, a sizing run stays within the solve worker's per-invocation memory budget.
  - The run reports progress per solve and can be cancelled between solves; a cancelled run publishes nothing.
- **AC-011** — Maintained tests show:
  - the solve count;
  - that no full result is retained between solves;
  - that a desktop sizing run stays within the per-invocation memory budget;
  - cancellation publishing nothing.

  The DEL-04-05 rack harness carries a hanger-sizing timing profile.
- **REQ-012** — *Desktop and report.*
  - The desktop takes the request with every input entered by the person. It shows:
    - the hanger table with its basis and disclosures;
    - a detail view for one candidate that lists each record with its first failed test, computed by the same function as the selection;
    - an action that applies a selection under REQ-008.
  - The rendered report gains a "Hanger sizing" section: request, basis, table, disclosures, refusals and limitations. Library-derived values in it follow the report's privacy rules for library references.
  - These surfaces follow the results viewer's and report generator's contracts.
- **AC-012** — Desktop tests cover:
  - the request form, with no pre-set choice in a new project;
  - the table and the detail view;
  - application;
  - the report section.
- **REQ-013** — *Claims.* Product texts state the following, using the claims registry's vocabulary (`docs/claims_registry.md`):
  - sizing selects the user's own records by the user's own tests;
  - a result inherits the premises of the solves it uses;
  - it establishes neither the suitability of a hanger nor code compliance.

  No product text states or implies a claim that PRD §21.2 prohibits.
- **AC-013** — Every sizing product text states REQ-013's three statements and none of the claims PRD §21.2 prohibits.

### Spring preload

- **REQ-014** — *A variable spring carries its cold load.*
  - **Preload.** On both routes, the solve models a variable spring hanger as DEL-04-03/REQ-014 states: its stiffness plus its `cold_load` as a constant force along its stiffness DOF. The cold load acts in every load case, through the constant-effort seam and under the same combination gate. The spring carries the cold load minus its rate times its displacement from the drawn geometry, relative to its base where the base moves.
  - **Held springs.** In a case whose test condition holds springs, a held spring (DEL-05-01's held support) is a rigid restraint of its DOF, and neither its stiffness nor its preload acts.
  - **Recovery.** The spring's published support force includes the preload. A case that applies the cold load also publishes the cold load itself, separately from that support force, as the support's applied force, as a consumed constant-effort support's force is published (DEL-04-07/CLM-004). A spring whose cold load a case does not apply, a held spring included, publishes no applied force in that case, and the result states that it was not applied and why.
  - **Diagnostics.** No solve warns that a spring hanger's load is not applied as preload (`SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD` has no place). A warning is published where `installed_load` and `cold_load` differ.
  - **Behaviour change.** A document that carries `cold_load` solves differently with the preload than without it, and the release notes say so.
  - DEL-04-03/REQ-014 states the model; the rest of the law is stated here.
- **AC-014** — Maintained checks show:
  - the preload reference agrees with the force-plus-spring derived solve, in displacements, in published support forces and in the published applied force, including a moved base;
  - targeted regression covers the spring-hanger tests and diagnostics, the combination gate, and a held and an unheld spring, with no preload on the held one;
  - the retained route's memory profile priced with the preload term, under the memory-pricing law (DEL-04-08/REQ-004);
  - the reference system's spring results including the preload.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- *One solve path.* Every derived solve is a modified copy of the model document sent through the product's own solve entry; sizing assembles no system of its own.
- *One spring law.* F_cold − k·u from the drawn geometry is the only law, in sizing, verification, application and the solve.
- *One fit function.* The selection and the detail view compute each record's first failed test with the same function.
- *A closed result document.* The result embeds its request and binds its inputs by hash, so a reader can check it without the producer.
- *Results, not edits.* Sizing never writes the model; application is a separate act through the operation route.
- *Independent references.* A reference is written without reading product code and is refuted by a second agent that reads none, before any product comparison.

**How each criterion is examined.**

- **VER-001** — Executable check: observes each request, case-role and candidate refusal with its code and named supports, and the absence of any default, at the product entry, the headless runner and the desktop.
- **VER-002** — Executable check: observes each kind of derived document, the solve-time lowering applied to it, the ledger entry and support-state reconciliation on EX, the state disclosures and warnings, the refusal on a failed derived solve, and the stored model byte-identical after a run.
- **VER-003** — Comparison with a named reference: the product against the frozen closed-form references in `validation/references/hanger_sizing`, at the verification tolerance and with selections exact, with a mutation of each method choice and repeat-run byte identity.
- **VER-004** — Executable check: observes records with and without a working load range, in both library formats, listed and tested as AC-006 states, and the result's statement of the members not read.
- **VER-005** — Executable check: observes the product reader and the test verifier returning the same verdicts on every committed result and mutation, the forged-positions case failing reproduction, the verifier importing no producer code, and the table's presence in the CSV coverage guard.
- **VER-006** — Executable check: observes the request form, the table, the detail view, the application batch with nothing applied without acceptance, and the report section, in the desktop.
- **VER-007** — Executable check: observes each refusal and disclosure with its code and named object, on both routes.
- **VER-008** — Comparison with named references, with an executable check: the product against every frozen reference package at the stated tolerance; the check scans each generator's imports, and each reference package is derived and refuted under the independent-references method choice above.
- **VER-009** — Executable check: observes the solve count, that no full result is kept between solves, the worker's peak footprint against the per-invocation budget, and that a cancelled run publishes nothing; the timing profile is measured through the performance harness.
- **VER-010** — Executable check with judgment by a reviewer who did not write the text: the check observes the pinned product texts and runs the repository's check of claims vocabulary over them; the reviewer examines the texts against `docs/claims_registry.md`.
- **VER-011** — Comparison with a named reference, with an executable check: the solve against the preload reference in `validation/references/hanger_sizing`; the check observes agreement with the force-plus-spring derived solve, the held and unheld spring, the combination gate, and the actual-source pricing audit with its stack witnesses (DEL-04-08/AC-004).

## Governing Values and Decisions — Axiology

- **AX-001** — *No silent engineering defaults.* Fewer method values supplied by the product is better; the measure is the number of method inputs any entry supplies, and its target is zero (PRD §5.5). No catalogue, limit, allowance or rule ships, and ease of use never justifies a supplied value.
- **AX-002** — *Results, not edits.* The model is written only by the person's applied selection through the one operation route. A faster path from a sizing result to the model never justifies a write the person did not accept.
- **AX-003** — *One solve path.* Better is closer agreement between derived solves and the model's own solves, measured by identical loads, states and admissions, at fewer and cheaper derived solves. A lighter publication may reduce what is kept, but never changes those loads, states or admissions, and never becomes a second assembly.
- **AX-006** — *Private data stays private.* Catalogue records are the user's private data. Only invented records enter the repository (`docs/IP_AND_DATA_BOUNDARY.md`), whatever a real record would add to a test.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-003 SOW-083 | REQ-001 | AC-001 | VER-001 | The refusal list in REQ-001 (SOW-083; PRD §5.5 and §12.4) |
| OUT-001 | OBJ-003 SOW-083 | REQ-002 | AC-002 | VER-001 | The refusal list in REQ-002 (SOW-083; PRD §12.4) |
| OUT-002 | OBJ-003 SOW-083 | REQ-003 CLM-001 | AC-003 | VER-002 | The criterion states the expected derived documents and refusals. The stored model's byte identity after a run compares the product with its own earlier bytes and only guards against change |
| OUT-003 | OBJ-003 SOW-083 | REQ-004 | AC-004 | VER-003 | Outside source: the frozen closed-form references in `validation/references/hanger_sizing` (continuous-beam reactions by three moments, free riser growth, the sizing arithmetic as exact identities), from generators that import no product code; the mutation outcomes follow the rules REQ-004 states. Repeat-run identity only guards against change |
| OUT-003 | OBJ-003 SOW-083 | REQ-005 | AC-005 | VER-003 | Outside source: the one-spring closed form and the mixed and lift-off references in `validation/references/hanger_sizing`, from generators that import no product code |
| OUT-004 | OBJ-003 SOW-083 | REQ-006 | AC-006 | VER-004 | The criterion states the expected listing; the range's shape is DEL-03-02's |
| OUT-004 | OBJ-003 SOW-083 | REQ-007 | AC-007 | VER-005 | Outside source: a Python verifier that shares no producer code. Existing readers' handling of the document and the committed documents are the product's own earlier output and only guard against change |
| OUT-004 | OBJ-003 SOW-083 | REQ-008 | AC-008 | VER-006 | The criterion states the batch's expected before and after; SOW-083 requires that the model change only by the person's act |
| OUT-005 | OBJ-003 SOW-083 | REQ-009 CLM-002 CLM-003 | AC-009 | VER-007 | The refusals REQ-009 names and release target exit criteria 1 and 3 |
| OUT-005 | OBJ-003 SOW-083 | REQ-010 | AC-010 | VER-008 | Outside source: the frozen reference packages, derived without reading product code, from generators that import none (release target exit criterion 2), at REQ-010's stated tolerance |
| OUT-005 | OBJ-003 SOW-083 | REQ-011 | AC-011 | VER-009 | The criterion states the expected solve count and behaviour, within the memory budget |
| OUT-005 | OBJ-003 SOW-083 | REQ-012 | AC-012 | VER-006 | PRD §5.5 for the absence of a pre-set choice; the criterion states the expected behaviour of the surfaces |
| OUT-005 | OBJ-003 SOW-083 | REQ-013 | AC-013 | VER-010 | Judgment of the texts against PRD §21.2 and `docs/claims_registry.md` |
| OUT-006 | OBJ-003 SOW-083 | REQ-014 | AC-014 | VER-011 | Outside source: the closed-form preload reference (force plus spring) in `validation/references/hanger_sizing`, from a generator that imports no product code. Agreement with the force-plus-spring derived solve is the product's own output and only guards against change |
