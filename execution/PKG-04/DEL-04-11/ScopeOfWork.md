---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-11
package_id: PKG-04
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.18
project_scope_refs: [SOW-083]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-11 Hanger sizing

**Basis:**
- decomposition revision 0.18 (SOW-083, OBJ-003);
- PRD §12.4;
- SCA-015 (`execution/_ScopeChange/SCA-015_2026-10-31_HANGER_SIZING/DECISION.md`);
- `docs/RELEASE_TARGET_v1.0.md` RT-A11.

The owner chose OC-4(a) on 2026-10-28: "Must: size hangers from the user's private
catalogue (invented example records only) and produce a hanger table." On 2026-10-31,
Agent 0 (HELP_HUMAN), under the owner's delegation in `loop/LOOP_INIT.md`:
- placed it in this new deliverable;
- adopted the method;
- decided the hanger library format 1.1.0;
- decided that a variable spring is modelled with its cold (preset) load.

These commitments and acceptance criteria are pending an independent review of their
text. Nothing in them is built, and they assert no completion, acceptance or release.
The governing design is [Work/hanger-sizing.md](Work/hanger-sizing.md). Standard claim
fence applies (claims taxonomy per DEC-081).

## Purpose and Objective Traceability

This deliverable sizes variable spring hangers and constant-effort supports from the
user's own imported hanger catalogue and produces a hanger table. It works from a
weight case solved with the hanger locations rigid and from operating cases solved with
the hangers replaced by their loads. Every method choice comes from the user. Sized
values are reported as results with their basis, and the model changes only when the
person applies a selection. It serves OBJ-003 through SOW-083.

- **OUT-001** — The sizing request and its candidates (REQ-001, REQ-002).
- **OUT-002** — The derived solves, with the treatment of nonlinear supports (REQ-003).
- **OUT-003** — Loads, fit, selection and the verification pass (REQ-004, REQ-005).
- **OUT-004** — The hanger table, its basis, readers and CSV export, and application of
  a selection (REQ-006 to REQ-008).
- **OUT-005** — The admitted domain, frozen references, cost bounds, the desktop and
  report surfaces, and the claim boundary (REQ-009 to REQ-013).
- **OUT-006** — Variable spring preload in the solve (REQ-014).

## Deliverable Definition — Ontology

### CLM-001 — The method

For the candidate supports the person names, the sizing runs these steps (Design §1, §4
to §7):
1. **Hot-load basis.** The weight case is solved with every candidate made a rigid
   restraint of its single translational DOF. Each candidate's reaction is its hot-load
   basis W.
2. **Travel.** The weight case and each named operating case are solved with every
   candidate replaced by a force W. Each candidate's travel in an operating case is its
   displacement there minus its displacement in the weight case.
3. **Variable records.** The cold load and the operating loads follow from the record's
   rate, under the person's hot or cold load design. The load variation follows under
   the person's reference load.
4. **Fit.** Each catalogue record is tested against its working load range, its travel
   range with the person's allowance, and the person's variation limit.
5. **Selection.** A fitting record is selected by the person's rule.
6. **Verification.** The selections are re-solved in place and the loads they carry are
   reported.

Every derived solve is a modified copy of the model document, sent through the model's
own solve entry and never persisted.

### CLM-002 — What this deliverable owns, and what it takes as input

**This deliverable owns:**
- the sizing request, its refusals and disclosures;
- the derived-document construction and the solve loop;
- the sizing arithmetic, fit tests and selection;
- the verification pass;
- the hanger table, the hanger-sizing profile, the `hanger-sizing-1` semantic table and
  the hanger-sizing analysis-run kind;
- the Rust reader and its test-only Python verifier;
- the product module `core/product_physics/src/hanger_sizing.rs` and its entries;
- the frozen references under `validation/references/hanger_sizing/`;
- the sizing meaning of the hanger library's `working_load_range`;
- the desktop request form and hanger table, and the report's hanger-sizing section,
  within the results viewer's and report generator's contracts;
- carrying out, in DEL-04-03's support model, the preload decision of REQ-014.

**Declared inputs** (`deliverable.yaml`):
- DEL-04-07: the product solve entries on both routes;
- DEL-04-03: support families, the constant-effort seam and spring assembly;
- DEL-04-04: nonlinear support states;
- DEL-05-01: load cases and their solve-time lowering;
- DEL-02-01: the support and hanger members of the model document;
- DEL-03-02: the hanger library schema;
- DEL-03-07: the library import boundary;
- DEL-02-02: units;
- DEL-00-06: the diagnostics and result-envelope contract;
- DEL-04-08: the retained-memory pricing law;
- DEL-07-05, DEL-07-07, DEL-08-01, DEL-08-04 and DEL-04-05: non-gating.

These deliverables keep their scope.

### CLM-003 — Current reach and sequence

- **Built:** nothing. The hanger records import, and one record can be applied to a
  support as the person's act (`features/hanger-selection`). A variable spring is
  solved as its stiffness only (`SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD`).
- **Sequence** (Design §13):
  1. the engine on PF with library format 1.1.0;
  2. the verification pass and multi-case sizing;
  3. the desktop and report surfaces;
  4. EX and the reference system;
  5. documentation;
  6. spring preload.
- **Later, outside these commitments:**
  - forming W in the design operating case's support state, once the solve can hold
    that state (RT-A5 Should; REQ-003);
  - automatic re-sizing iteration;
  - hangers on skewed or rotational DOFs.

## Completion and Reliance Basis — Epistemology

A sizing result is a numerical statement about the discrete model under the user's
cases, catalogue and method choices. It inherits the premises and limitations of each
solve it uses, and bounds no modelling, input or physical error. A selected record is
the one that meets the user's own tests; SWBPIPE does not judge any hanger's
suitability, and nothing here determines code compliance.

### Request

- **REQ-001** — *Required inputs, no defaults.*
  - The request names:
    - one imported hanger library;
    - the candidate supports, each with its family policy (variable, constant, or
      variable else constant);
    - the weight case and the support state W is formed in;
    - the design operating case and the further operating cases to check (stated, even
      when empty);
    - the load design (hot or cold);
    - the variation limit and its reference load (operating or design load);
    - the travel allowance at each end of the travel span;
    - the selection rule (smallest capacity or least variation).
  - Each absent or invalid input is refused by name. The engine, the headless runner
    and the native entry supply no value. The desktop pre-fills the values last used in
    the project, and the request records the values used.
  - Named cases are load cases, not combinations. A test-condition case is refused, and
    so is a case named in two roles.
  - The request is recorded with the result and in the analysis-run record (Design §2).
- **AC-001** — Maintained tests show, at the product facade, the headless runner and the
  desktop:
  - each refusal for a missing or invalid input;
  - the case-role refusals;
  - no default supplied on any entry.
- **REQ-002** — *Candidates.*
  - A candidate restrains exactly one translational DOF, has no skew direction, is not
    nonlinear, a snubber or an anchor, shares its node and DOF with no other support,
    and is referenced by no other model object.
  - Its present family and hanger data are not read.
  - Its force acts along the positive axis of its DOF (DEC-049's convention).
  - Each failure is refused by name. A candidate whose hot-load basis is not positive is
    reported as refused while the others are sized (Design §3).
- **AC-002** — Each candidate refusal is exercised by a maintained test that asserts its
  code and named support. A placeholder support of each admitted family is sized.

### Solution

- **REQ-003** — *Derived solves.*
  - Each derived solve is a modified copy of the stored model document, sent through
    the product solve entry the model's own cases use. It has the model's route and
    settings and requests no retained-precision recovery.
  - It is never saved, never returned as a model, never in undo history and never
    passed through the operation route.
  - On an EX load/reference-state document, each added force is entered in its case's
    source ledger.
  - Nonlinear supports converge in each derived solve to the states that case produces.
  - The support state W is formed in is a required input. Only the weight case's own
    state is admitted until the solve can hold another case's state.
  - The result publishes each nonlinear support's state in every derived solve. It
    warns by name where a state differs between the weight solve and the design
    operating solve, or between the weight solve and the cold reference.
  - A derived solve that is refused, fails to converge or publishes a blocking
    diagnostic refuses the request, with its own diagnostics. No partial table is
    published (Design §4, §5).
- **AC-003** — Maintained tests show:
  - each kind of derived document, and that the stored model is byte-identical after a
    run;
  - that the solve-time lowering is applied to derived documents;
  - the EX ledger entry;
  - the state disclosures and warnings on a lift-off model;
  - the refusal on a failed derived solve.
- **REQ-004** — *Loads, fit and selection.*
  - **Loads.** Under hot design, a variable spring's cold load is the hot-load basis
    plus its rate times the design case's travel. Under cold design, it is the hot-load
    basis. Each operating load is the cold load minus the rate times that case's travel.
  - **Variation.** Load variation is the difference between the cold and operating loads
    over the chosen reference load.
  - **Variable record fit.** A variable record fits when, in order:
    1. it carries a rate, a working load range and a travel range;
    2. its stiffness DOF matches the candidate;
    3. every operating load is positive;
    4. the load envelope, widened by the rate times the allowance at each end, lies
       within its working load range;
    5. the travel span plus twice the allowance is within its travel range;
    6. every variation is within the limit.
  - **Constant record fit.** A constant record fits when it carries a working load range
    and a travel range, the hot-load basis lies within its range, and the travel test is
    met.
  - **Selection.** It follows the family policy and the selection rule with stated tie
    breaks, so it is deterministic. With no fit, the outcome is "no record fits", with
    the count of records failing each test, and nothing is assumed in its place (Design
    §6).
- **AC-004** — On the frozen references, the product reproduces, within the
  verification tolerance of REQ-010:
  - the rigid reactions;
  - the free-riser travel;
  - every load, variation, fit outcome and selection, the selection exactly.

  This holds over hot and cold design, both reference loads, the allowance,
  multi-case governing, fallback, no fit, non-positive loads and a DOF mismatch.
  Mutation tests show each method choice changing the result as its rule predicts.
  Results are byte-identical on repeated runs.
- **REQ-005** — *Verification pass.*
  - The weight case and each operating case are solved again. Each selected variable
    record is modelled as a spring of its rate with a force equal to its cold load, and
    each constant selection or unfitted candidate as a force equal to its hot-load
    basis.
  - The verified loads, variations and departure from the hot-load basis are published.
  - Each fit test that the verified values fail is warned by name.
  - No selection is changed automatically (Design §7).
- **AC-005** — The one-spring closed-form reference and a lift-off case are reproduced.
  A selection whose verified loads fail a test produces its warning, and the selection
  is unchanged.

### Library, results and application

- **REQ-006** — *Hanger library format 1.1.0.*
  - **Schema.** The hanger library schema (DEL-03-02) admits format 1.1.0. A record may
    carry `working_load_range` (a minimum and a maximum force, each with provenance),
    and the import boundary refuses a minimum not below its maximum by name.
  - **Compatibility.** Format 1.0.0 documents import unchanged. Sizing lists their
    records, and any 1.1.0 record without a range, as "not sizable: no working load
    range". A stored 1.0.0 document needs no migration.
  - **Constant records.** Sizing does not read a constant record's `constant_load`
    (Design §8.2).
- **AC-006** — Import tests show:
  - 1.0.0 documents importing unchanged, and a 1.0.0 document with the new field
    refused;
  - a 1.1.0 document importing;
  - the range refusal;
  - provenance and units preserved;
  - parity between the Rust import boundary and the Python provenance checker;
  - every 1.0.0 record listed as not sizable.

  The persistence change takes an xhigh review.
- **REQ-007** — *The hanger table, its basis and readers.*
  - **The table**, one row per candidate, gives:
    - the outcome and the selected record;
    - the hot-load basis, the cold or constant load, and each case's operating load,
      travel and variation, with the governing cases;
    - the required travel;
    - the verified values;
    - for an unfitted candidate or a fallback, the count of records failing each test.
  - **Derived solves.** The result also gives each candidate's cold-reference
    displacement and each nonlinear support's states.
  - **Basis.** A basis block states every request input, the library id and digest, the
    model revision, the route and each case's load kinds after lowering.
  - **Profile.** Results are published under the hanger-sizing profile and semantic
    table, and the analysis-run record gains the hanger-sizing run kind.
  - **Readers.** A Rust product reader and a test-only Python verifier that shares no
    code with the producer each recompute the loads, variations, fit tests and selection
    from the published values, the request and the library document.
  - **CSV.** The table exports to CSV (Design §8.1).
- **AC-007** — Maintained tests show:
  - the two readers returning the same verdict for every committed result and every
    committed mutation of it;
  - that the Python verifier imports no producer code (an independence test);
  - the table in DEL-08-04's CSV coverage guard.
- **REQ-008** — *Applying a selection.*
  - Applying a selected record is the person's explicit act. It goes through the
    existing hanger-selection operation builder and the one operation route, and is
    applied only on the person's acceptance.
  - The batch writes the record's family, stiffness, travel range, references and
    provenance, and the sized loads, all into existing model members:
    - for a variable record, the cold and installed loads (both the sized cold load) and
      the hot load;
    - for a constant record, the constant load (the hot-load basis).
  - Each sized value's source note names the sizing run, its request digest and the
    record. Sizing itself never writes the model (Design §9).
- **AC-008** — Desktop tests show:
  - the batch's before and after for a variable and a constant selection, with the
    sized values and their source notes;
  - nothing applied without acceptance;
  - the model unchanged by a sizing run.

### Domain, references, cost and surfaces

- **REQ-009** — *Admitted and refused.*
  - Sizing admits what the model's own solve admits, on PF and EX, plus the candidate
    rules of REQ-002.
  - What the solve refuses refuses the request with the solve's code.
  - Each standing refusal in this Must row is listed in
    `docs/RELEASE_LIMITATIONS_v1.0.md` by the change that makes it standing.
- **AC-009** — Each refusal and disclosure is exercised by a maintained test that
  asserts its code and named object, on both routes once EX is admitted.
- **REQ-010** — *References.*
  - **Derivation.** The references are derived and frozen under
    `validation/references/hanger_sizing/` by a TASK that reads no product code, and
    refuted by a second TASK before any product comparison.
  - **Coverage.** They cover:
    - rigid reactions of continuous beams;
    - free riser growth;
    - the sizing arithmetic and selection;
    - the verification pass;
    - a lift-off case;
    - the spring preload;
    - the reference system's hangers, once its families solve on EX.
  - **Values.** All values are invented.
  - **Tolerance.** The product is verified at an analytic tolerance under DEC-026,
    measured and only tightened. Selection outcomes are exact.
- **AC-010** — Each committed reference package has its refutation record. The
  verification tolerance and its measurement are recorded.
- **REQ-011** — *Cost.*
  - A request runs at most 2n + 3 ordinary solves for n operating cases, one after
    another.
  - After each solve, only the values the table needs are kept.
  - Derived solves never enter the retained-precision entry.
  - In the desktop, they run inside the solve worker under its per-invocation memory
    budget, with no attestation digest.
  - They go through one solve call site, so that the minimal publication planned for
    derived solves can replace full publication there.
  - The run reports progress per solve and can be cancelled between solves; a cancelled
    run publishes nothing (Design §10).
- **AC-011** — Maintained tests show:
  - the solve count;
  - that no full result is retained between solves;
  - that the solves run in the worker without an attestation digest;
  - cancellation publishing nothing.

  A hanger-sizing timing profile is added to the DEL-04-05 rack harness.
- **REQ-012** — *Desktop and report.*
  - The desktop takes the request with every input entered by the person. It shows:
    - the hanger table with its basis and disclosures;
    - a detail view for one candidate that lists each record with its first failed test,
      computed by the same function as the selection;
    - an action that applies a selection under REQ-008.
  - The rendered report gains a "Hanger sizing" section: request, basis, table,
    disclosures, refusals and limitations. Library-derived values in it follow the
    report's privacy rules for library references.
  - These surfaces follow the results viewer's and report generator's contracts.
- **AC-012** — Desktop tests (Vitest) cover:
  - the request form, with no pre-set choice in a new project;
  - the table and the detail view;
  - application;
  - the report section.
- **REQ-013** — *Claims.* Product texts state the following, using the claims registry's
  vocabulary (`docs/claims_registry.md`):
  - sizing selects the user's own records by the user's own tests;
  - a result inherits the premises of the solves it uses;
  - it establishes neither the suitability of a hanger nor code compliance.

  No product text states or implies a claim that PRD §21.2 prohibits.
- **AC-013** — The sizing product texts are pinned by maintained tests and reviewed
  against `docs/claims_registry.md`, and GEN-13 passes over them.

### Spring preload

- **REQ-014** — *A variable spring carries its cold load.*
  - On both routes, the solve models a variable spring hanger as its stiffness plus its
    `cold_load`, applied as a constant force along its stiffness DOF in every load case,
    through the constant-effort seam and under the same combination gate. The spring
    carries the cold load minus its rate times the displacement.
  - `SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD` is retired. A warning is published
    where `installed_load` and `cold_load` differ.
  - Documents that carry `cold_load` solve differently after the change. The change and
    the release-notes material state this.
  - DEL-04-03's support model and commitment text change with this requirement (Agent 0,
    D-3, 2026-10-31; Design §11).
- **AC-014** — When built:
  - the preload reference agrees with the force-plus-spring derived solve;
  - targeted regression covers the spring-hanger tests and diagnostics, and the
    combination gate;
  - the retained route is re-priced, with the registration reviewed;
  - the reference system's spring results are updated with that lane.

## Production and Verification Method — Praxeology

The checks named here are planned; they do not exist yet. Each increment registers its
checks in `software-workflow.json` and names them with their head in its pull request.

- **VER-001** — For REQ-001 and REQ-002: refusal tests at the product facade, the
  headless runner and the desktop.
- **VER-002** — For REQ-003: derived-document, lowering, ledger, state-disclosure and
  failure tests, and the stored-model invariance test.
- **VER-003** — For REQ-004 and REQ-005: product tests against the frozen references,
  mutation tests per method choice and a determinism test.
- **VER-004** — For REQ-006: import compatibility and parity tests.
- **VER-005** — For REQ-007: Rust reader tests, the Python verifier with its
  independence test, the shared mutation corpus and the CSV coverage guard.
- **VER-006** — For REQ-008 and REQ-012: Vitest for the request form, table, detail
  view, application and report section.
- **VER-007** — For REQ-009: one maintained test per refusal and disclosure, on each
  admitted route.
- **VER-008** — For REQ-010: the refutation records and the tolerance record.
- **VER-009** — For REQ-011: the solve-count, retention and cancellation tests.
- **VER-010** — For REQ-013: pinned-text tests, GEN-13 and review against
  `docs/claims_registry.md`.
- **VER-011** — For REQ-014: the preload reference, the targeted regressions, and
  DEL-04-08's actual-source D3 run with the AC-004 stack witnesses.

Each numerical increment also gets the independent review that `AGENTS.md` requires.

## Governing Values and Decisions — Axiology

- **AX-001** — *No silent engineering defaults.* Every method choice is entered by the
  user or refused (PRD §5.5). No catalogue, limit, allowance or rule ships.
- **AX-002** — *Results, not edits.* Sizing never writes the model. Applying a selection
  is the person's act through the one operation route.
- **AX-003** — *One solve path.* Derived solves go through the model's own solve entry,
  so they see exactly the loads, states and admissions the model's own solves see.
- **AX-004** — *No relaxation.* References are never altered to make an implementation
  pass.
- **AX-005** — *Independent scrutiny.* Numerical and persistence changes get targeted
  regression checks and independent review by a separate agent.
- **AX-006** — *Private data stays private.* Catalogue records are the user's private
  data. Only invented records enter the repository (`docs/IP_AND_DATA_BOUNDARY.md`).
- **AX-007** — *Held by the owner:* release, acceptance and issuance, risk acceptance and
  holds (F-PIP-3), and declaring a limitation acceptable.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-003 SOW-083 | REQ-001 REQ-002 | AC-001 AC-002 | VER-001 | Refusal and no-default tests on every entry |
| OUT-002 | OBJ-003 SOW-083 | REQ-003 CLM-001 | AC-003 | VER-002 | Derived-document, lowering, ledger and state tests; stored-model invariance |
| OUT-003 | OBJ-003 SOW-083 | REQ-004 REQ-005 | AC-004 AC-005 | VER-003 | Reference, mutation and determinism tests on the candidate head |
| OUT-004 | OBJ-003 SOW-083 | REQ-006 REQ-007 REQ-008 | AC-006 AC-007 AC-008 | VER-004 VER-005 VER-006 | Import compatibility, matching reader verdicts, CSV guard, application tests |
| OUT-005 | OBJ-003 SOW-083 | REQ-009 REQ-010 REQ-011 REQ-012 REQ-013 CLM-002 CLM-003 | AC-009 AC-010 AC-011 AC-012 AC-013 | VER-006 VER-007 VER-008 VER-009 VER-010 | Refusal tests, refutation records, cost tests, desktop tests, pinned texts and claims lint |
| OUT-006 | OBJ-003 SOW-083 | REQ-014 | AC-014 | VER-011 | Preload reference, targeted regressions and D3 re-pricing |
