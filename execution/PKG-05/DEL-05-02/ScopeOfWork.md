---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-05-02
package_id: PKG-05
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-014]
package_objective_refs: [OBJ-003, OBJ-005]
---

# Scope of Work — DEL-05-02 Load-case algebra engine

## Purpose and Objective Traceability

This deliverable combines the results of solved load cases into the results an engineer assesses. A piping assessment is rarely one case: the expansion range is the difference between an operating case and a sustained case, a three-axis static seismic demand is the square root of the sum of squares, or the absolute sum, of three directional cases, and a design basis asks for weighted sums and envelopes of these. The deliverable defines the arithmetic of each kind of combination, decides when combining results is a valid statement about the model and withholds the combination when it is not, and says how far a combined value can be relied on. Which combinations a design code requires, and with what factors, are the user's: the product ships none.

It serves SOW-014 under OBJ-003 and OBJ-005 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-05, row DEL-05-02: a bounded algebra surface), and PRD §12.6 with FR-SOL-009. Exit criterion 1 of `docs/RELEASE_TARGET_v1.0.md` names it with DEL-05-01 for loads, cases and combinations: among the analyses one route must run are expansion with its basis and three-axis static seismic with SRSS and absolute-sum combination, and the release target judges combination coverage under that criterion. Under criterion 2 each of those analyses passes closed-form or hand-calculation references made by an independent generator.

- **OUT-001** — The load-case algebra: the combination bases and their arithmetic, the rules that withhold a combination, the declared difference basis and the held-status subtrahend, the reliance statement of a combination, and their findings.
- **OUT-002** — The ordinary result reader and its base semantic tables: the one reading of a published result on the pressure-free and exact routes, which classifies the result, gives each row its meaning and the result its standing, so that every product surface says the same thing about the same result, a combination's rows included.

## Deliverable Definition — Ontology

**Terms the deliverable implements.**

- *Combination.* A named result formed from the results of load cases or of other combinations, row by row, on one of five bases. It is never solved. Its operands are named by id; an id resolves to a load case first, then to a combination.
- *Mechanics combination.* Σ fᵢ·xᵢ with finite user factors: the exact sum of the exact products, rounded once to binary64.
- *Result-state subtraction.* Minuend minus subtrahend, two distinct operands.
- *Range envelope.* Over its operands, the minimum, the maximum, the minimum absolute value or the maximum absolute value, as its mode says.
- *Absolute sum.* A = RN(Σ |fᵢ·xᵢ|), the exact sum rounded once.
- *SRSS.* With zᵢ = RN(fᵢ·xᵢ), R = RN(√Σ zᵢ²), the square root of the exact sum of squares, correctly rounded.
- *Unsigned row.* A row of an absolute sum or an SRSS, or of a combination that contains one. It is a magnitude and not an equilibrium state, and results say so.
- *Nested combination.* A combination with a combination as operand. The operand graph is acyclic; evaluation is in dependency order, and among independent combinations in document order.
- *Fixed-load count κ.* How many times a combination carries the loads that do not scale with a case's factors (a constant-effort support's force, an installed reference state). A load case has κ = 1; a mechanics combination Σ fᵢκᵢ; a subtraction κ of the minuend less κ of the subtrahend; an envelope its operands' common κ, otherwise undefined; an absolute sum or SRSS 0 when every operand's κ is exactly 0, otherwise undefined. κ is an exact dyadic rational of at most 4096 bits.
- *Modulus basis and resolved stiffness.* The elastic moduli a case was solved with, and each member's stiffness as solved. A load case selects its moduli by an exact basis reference, which uses the user's entered E and G at that point, or by a solve temperature, which interpolates the user's E, G and expansion coefficient linearly between adjacent entered points.
- *Declared difference basis.* A subtraction the user declares as `operating_minus_sustained`. Its rows read "operating minus sustained difference". The word "expansion" belongs to the user's rule pack.
- *Held-status subtrahend.* A subtraction that sets `subtrahend_support_state` to `minuend_final`: the subtrahend's loads are solved once with every nonlinear support held in the state it ended in for the minuend. What is held is the state (closed or open, stuck or sliding, on or off its seat), not a displacement or a force.
- *Operand standing.* For a subtraction or envelope in a model with nonlinear supports, the published record of each operand's load case, modulus basis and final support states, and whether those states agree between operands.
- *Combination certificate.* For each group of force or moment rows of a mechanics combination or subtraction whose operands are load cases that each carry a bound Bᵢ from their own solve: the bound B_D = RU(Σ |fᵢ|·Bᵢ + ‖v_D − Σ fᵢ·vᵢ‖), and a verdict. The group is *established within its stated bound* when B_D ≤ 10⁻⁹ times the group's magnitude, *not established* when B_D exceeds that or the comparison cannot be made, and *not assessed* when the magnitude is below the floor at which the operand cases are themselves assessed.
- *Base semantic table.* For each ordinary result identity, the reviewed table that gives every row signature its kind, unit, dimension and component, the metadata that identifies the row (location, coordinate system, section basis, sign convention) where the kind has it, and its category and disposition. A result is bound to its table by the table's identifier and SHA-256 digest.
- *Ordinary result reader.* The reader of a published result of an ordinary identity: it classifies the result by its identity and binds it to its table, checks the statement, gives each row its meaning from the table and returns the result's standing.
- *Standing.* What the reader says a result may be used for: numerically eligible, needing recomputation, or unsupported, with the findings that decide it.

**Boundary.** This deliverable defines combinations of solved results. The following belong elsewhere:

- load cases, primitive loads and their generation: DEL-05-01. Solving a case, the held-status solve step itself (DEL-04-07/REQ-012), and publication of every row: DEL-04-07. Final support states and the active-set classifier: DEL-04-04;
- the stress definitions and section bases of the rows combined: DEL-05-03. The bound a load case's own force and moment rows carry from the solve's integrity check: DEL-04-01's code, composed by DEL-04-07. Combinations on the retained-precision routes: DEL-04-08;
- the exact accumulator the sums use: DEL-04-01. The model-document members of a combination: DEL-02-01. The combination editor and its seismic template: DEL-07-03. Result formats: DEL-08-04;
- code-specific combinations, factors, stress categories and limits: the user's rule pack (PKG-06). The rule-pack expression evaluator: DEL-06-02;
- the reader of retained-precision and retained-source statements, their tables, the test verifier that shares no code with the readers, and the comparison of the readers' native and WebAssembly outcomes: DEL-04-08. What each row's value is, and its publication: DEL-04-07, with the stress rows defined by DEL-05-03.

**Not decided.**

- What a combination that includes a hydrostatic test case means. DEL-05-01 withholds it until that is defined.
- Whether the held-status subtrahend is offered on the exact pressure route.

## Completion and Reliance Basis — Epistemology

A combination is a statement about solved results and the user's factors. Its arithmetic is examined against exact rational arithmetic, and its gates against models whose closed-form answer shows why the gate is needed.

### Requirements

- **REQ-001** — User-defined combinations are unit-aware: operands and results carry dimension and unit, and rows are combined only with rows of the same quantity, object and component (SOW-014; PRD §12.6).
- **REQ-002** — A dimensionally incompatible operation is refused with a finding. Nothing is converted or coerced to make it possible (PRD §5.6; `docs/CONTRACT.md` OPS-K-UNIT-1, OPS-K-DATA-2).
- **REQ-003** — Subtraction and ranging of result states are mechanics-result operations. They state and imply no code compliance, and the product gives a subtraction no code name (the decomposition's row; `docs/CONTRACT.md` OPS-K-MECH-2).
- **REQ-004** — Code-specific combinations come from the user: the product bundles no design-code combination, default factor or protected formula. An interface by which a rule pack supplies combinations to the algebra is deferred until after v1.0; until it exists, a user authors combinations in the model (SOW-014; `docs/CONTRACT.md` OPS-K-DATA-1; `docs/RELEASE_TARGET_v1.0.md`, which defers the rule-pack extensions).
- **REQ-005** — The algebra is a closed set of bases with typed operands. It contains no parser of expressions and no way to run arbitrary rules (`docs/CONTRACT.md` OPS-K-RULE-2).
- **REQ-006** — A missing load case, operand or source result, an empty or duplicate operand list, a cycle, a non-finite factor or an unknown basis or mode is reported as a finding that names it. Nothing is defaulted (PRD §5.5; `docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-007** — A combination keeps the mechanics, rule-input and user-rule statuses of its operands, adds that human review is required, and never carries human acceptance (decomposition AB-00-03; PRD §21.2).
- **REQ-008** — Each finding carries code, class, severity, source, affected object, message, remediation and provenance, and a combined row names the rows it was formed from (decomposition AB-00-06).
- **REQ-009** — Evaluation is deterministic and independent of operand order: the same operands and factors give the same rows and findings bit for bit, so every check of it is deterministic (`docs/CONTRACT.md` OPS-K-SOLVER-1).
- **REQ-010** — *Bases.* Mechanics combinations, subtractions, range envelopes, absolute sums and SRSS, and combinations of combinations, are evaluated on both routes by the arithmetic defined above. A value outside the binary64 range is withheld as not representable, never published as an infinity (release target, exit criterion 1; FR-SOL-009).
- **REQ-011** — *Validity gates.* A combination is withheld by name, whole, when its rows would not be a statement about the model:
  - a mechanics combination in a model with a fixed-load source whose κ differs from 1 by more than 2⁻⁴⁶, and an absolute sum or SRSS any of whose operands has κ other than exactly 0;
  - a mechanics combination in a model with nonlinear supports, which requires a solve of the combined loads. The one exception is a single solved case with factor exactly 1 plus one or more terms that are each a combination with κ = 0;
  - a mechanics combination, absolute sum or SRSS whose reached load cases differ in modulus basis or, on the exact route, in any member's resolved stiffness;
  - any combination with a withheld operand. This is tested first.
  Subtractions and envelopes carry no gate of their own (PRD §12.4, §12.6).
- **REQ-012** — *Declared difference basis.* On both routes, a subtraction declared `operating_minus_sustained` is published only when the subtrahend's primitive loads are contained, by content and with multiplicity, in the minuend's (G-1) and the two cases have the same modulus basis (G-2). Its operands are load cases. Each failure is a named refusal (release target, exit criterion 1: expansion with its basis).
- **REQ-013** — *Held-status subtrahend (should).* On request the subtrahend is solved once with every nonlinear support held at the minuend's final state, under the subtrahend's own loads. A support whose held state the subtrahend's loads would release is disclosed and not altered, and a hold that cannot be formed or solved is a named refusal. The held subtrahend's rows are published under the combination, each marked as a held-status operand row with its held standing and evidence, so that no reader takes one for a row of the combination's result. The hold is one function of a source case and a target case, so that another analysis can ask for the same comparison (sustained may take the operating support status, as a Should; PRD §12.6).
- **REQ-014** — *Operand standing.* A subtraction or envelope in a model with nonlinear supports is published with its operands' standing, whatever that standing is. It is never withheld for it (PRD §12.6, §21.2).
- **REQ-015** — *Reliance.* A subtraction or mechanics combination whose operands are load cases carries, for each certificate group, its bound and verdict in the words defined above. A range envelope, an absolute sum, an SRSS, a combination with a combination as operand and a combination with a held-status subtrahend are not assessed, and each says so with its reason. The product never calls a combination certified, and a certificate neither adds to nor removes from a result's numerical eligibility (PRD §12.8).
- **REQ-016** — *Exact-route rows.* On the exact route combination rows pair operand rows by a typed structural key and by section basis; a row kind that has no meaning when combined is not published for a combination; and headlines and member maxima are formed from load cases only (PRD §12.7).
- **REQ-017** — *Readable without the model.* A reader forms every combination row, gate and certificate bound again from the published operand rows, factors and evidence, bit for bit, and refuses a result in which one differs (PRD §12.8; decomposition AB-00-06).
- **CLM-007** — *Load-case modulus selection.* A load case selects its moduli by exact basis reference or by solve temperature, never both. A solve temperature interpolates only between strictly bracketing adjacent entered points, requires a valid entered G at both, records both source points and the method, and blocks at or beyond the range edges without extrapolating or falling back to a base value. The selection is made in DEL-04-07's solve; this deliverable's gates read its outcome.
- **REQ-018** — *Ordinary result reader.* A published result of an ordinary identity is classified by its identity and bound to that route's base semantic table by identifier and digest. Each row is given the kind, unit, dimension, component and identifying metadata its signature has in the table, in the published order, and a source row the table does not define is named. The result's standing is returned with its findings and the route's published limitations. A result whose identity, version, table binding, rows or statement the reader does not admit is refused by name; nothing is defaulted or repaired to read it (PRD §5.5, §5.6, §12.8).
- **REQ-019** — *One product reader.* The ordinary result reader is one implementation, in Rust, built natively and for WebAssembly. The desktop, the headless runner's read command and Python product code each take its verdict, and no other product code classifies an ordinary result or decides its standing; other product code gives a row a meaning only from the table the reader bound, by the row's signature. A second reading exists only as a test verifier outside the product.
- **REQ-020** — *Tables.* Each base semantic table states the meaning of every row signature its route publishes, and changes only as a reviewed input whose new digest the reader binds.

### Acceptance criteria

- **AC-001** — For each basis, on both routes, every published row equals the value computed in exact rational arithmetic from the operands' published rows and the factors, rounded as defined, bit for bit, in any operand order, including nested combinations to depth three and factors that cancel. A value outside the binary64 range is withheld with the not-representable finding.
- **AC-002** — Each gate of REQ-011 withholds its combination with its named finding on a model built to trip it and publishes on the twin that does not: a constant-effort support and a nonzero installed reference for κ, including an operand whose κ is 2⁻⁴⁶; a nonlinear support for the solve requirement, with the exception published; two modulus bases; and a dependant of a withheld operand, withheld whole with the operand named. For a gated model the closed-form result of the combined loads differs from the sum the gate withheld.
- **AC-003** — A declared difference basis whose operands satisfy G-1 and G-2 is published with the row text and its derivation evidence; one that fails either, or names a combination as operand, is refused by name. For a linear model the published difference equals the closed-form response to the loads the operating case has beyond the sustained case.
- **AC-004** — A held-status subtrahend on a model with a gap, a one-way support and a friction support gives the closed-form response of the subtrahend's loads on the structure with those supports in the minuend's final states; a released support is disclosed; each named refusal is raised on the model built for it; and every held subtrahend row published under the combination carries the operand mark with its held standing and evidence.
- **AC-005** — A subtraction and an envelope in a nonlinear model are published with each operand's load case, modulus basis and support states, and with the statement that the states differ where they do.
- **AC-006** — For each certificate group the published bound is not less than the exact difference between the published combination rows and the exact combination of the operands' true values within their bounds; the verdict follows the three classes; a nearly cancelling subtraction is not established; a group below the floor is not assessed; a combination on another basis, with a combination as operand or with a held-status subtrahend is named as not assessed; and no product text says "certified".
- **AC-007** — On the exact route a combination's rows pair only rows of the same key and section basis, no row of a kind without combined meaning appears, and headlines and maxima are unchanged by the presence of combinations.
- **AC-008** — A reader given a result without the model reproduces every combination row, gate decision and certificate bound, and refuses each of a set of altered results: a changed row, factor, operand reference, gate record or bound.
- **AC-009** — An operation on rows of different dimension, a missing operand or source result, an empty or duplicate operand list, an operand cycle, a non-finite factor and an unknown basis or mode are each reported as a finding that names it, and no row is formed from it. A basis or member the route does not evaluate is refused by name.
- **AC-010** — A combination carries its operands' statuses with human review required, and one that would carry human acceptance is refused. The deliverable's sources, tests and fixtures contain no design-code combination, default factor or protected formula and only invented values, and no message or record states or implies approval or compliance.
- **AC-011** — A load case selecting both an exact basis and a solve temperature, a temperature at or beyond the entered range, or a bracketing point without a valid G is blocked with a named finding; an interpolated modulus equals the linear interpolation of the two entered points and names both.
- **AC-012** — On the invented reference system, on the exact route in the desktop, the operating-minus-sustained difference with its declared basis, and the three-axis static seismic combinations by SRSS and by absolute sum, are published and are neither withheld nor refused, and their rows reproduce the system's independent references.
- **AC-013** — For each ordinary identity, every committed result of the base-reader corpus is classified to its route, bound to that route's table by identifier and digest, and given for each row the kind, unit, dimension, component and identifying metadata the table holds for the row's signature, in the published order, with every row the table does not define named; its standing is one of the three, with its findings. A result altered in its identity, its version, its table binding, a row or its statement is refused by name.
- **AC-014** — The desktop, the headless runner's read command and Python product code each return the reader's verdict for the same result text; no other product code classifies an ordinary result or decides its standing, and any other product code that gives a row a meaning takes it from the table the reader bound, by the row's signature.
- **AC-015** — The test verifier reaches the reader's outcome on every statement of the base-reader corpus.
- **AC-016** — Each base semantic table gives each row signature the kind, unit, dimension and identifying metadata of the row its producer publishes under that signature.

## Production and Verification Method — Praxeology

**Method choices that define the algebra.**

- Sums are formed exactly and rounded once. No combination is a running floating-point sum, so no result depends on operand order.
- A combination is derived from published rows only. It reads no solver state, and a reader can form it again.
- κ is carried as an exact rational through the operand graph. The unit rule for a mechanics combination has the tolerance 2⁻⁴⁶; the zero rule for unsigned bases has none, because a tolerance there admits a multiple of the fixed load.
- Gates are tested in a fixed order, a withheld operand first, and a gated combination publishes no rows.
- The certificate is a statement about arithmetic and the operands' own bounds. It takes no part in deciding whether a result is numerically eligible.
- A row means what its route's reviewed table says its signature means. The reader reads the text the producer published and binds the table by digest, so that a result and its meaning cannot drift apart unnoticed.

**How each criterion is examined.**

- **VER-001** — Comparison with a named reference (`validation/references/rt_a4_combinations` and `validation/references/w16_exact_combinations`, whose generators use exact rational arithmetic and import no product code): every basis on both routes, with operands permuted and with nested combinations.
- **VER-002** — Comparison with a named reference (the closed-form models of `validation/references/rt_a4_combinations`, in which the combined loads have a known response): each gate of AC-002 tripped and not tripped, with the finding observed.
- **VER-003** — Comparison with named references (`validation/references/op1b_difference_basis` for the declared basis; `validation/references/op_mf_held_status` for the held-status subtrahend; both closed-form and generated without product code): publication, refusals and disclosures of AC-003 on each route and of AC-004 on the pressure-free route; with an executable check that observes the operand mark, held standing and evidence on every held subtrahend row of AC-004.
- **VER-004** — Comparison with a named reference (`validation/references/combination_certificate`, exact rationals with no product code): bounds and verdicts of AC-006; with an executable check of the standing record of AC-005 and a search of product text for the word the criterion forbids.
- **VER-005** — Executable check: observes the pairing, the absent row kinds and the unchanged headlines of AC-007; reproduction of rows, gates and bounds by the Rust reader and by a separately written verifier that shares no code with the product, and their refusal of each altered result of AC-008.
- **VER-006** — Executable check: observes each finding of AC-009 and AC-011 with the input it names and the absence of rows, the interpolated moduli against the two entered points, the statuses of AC-010 and bit-identical output on repeated evaluation.
- **VER-007** — Judgment by a named role (an independent reviewer who did not write the code): examines the deliverable's sources, tests, fixtures and messages against PRD §20.1 and §21.2 and `docs/claims_registry.md`, with the protected-content scan's result for the same files.
- **VER-008** — Comparison with a named reference (the closed-form and exact-identity references of `validation/references/reference_system`, whose generator imports no product code), with an executable check: observes that the reference system's declared difference and its SRSS and absolute-sum seismic combinations are published on the exact route in the desktop, and compares their rows with the reference.
- **VER-009** — Executable check: observes the reader's classification, table binding, row meanings and standing for every committed result of the base-reader corpus against the table entries for the rows' signatures, and its named refusal of results altered in identity, version, table binding, a row and the statement.
- **VER-010** — Executable check: observes the verdict the desktop, the headless runner's read command and the Python product binding return for the same result texts, and searches product code outside the reader for any other classification, row meaning or standing decision.
- **VER-011** — Comparison with a named reference (the test verifier under `tests/verifiers/`, written separately and sharing no code with the reader): the verifier's outcome on every corpus statement against the reader's.
- **VER-012** — Judgment by a named role (an independent reviewer with knowledge of the result families' mechanics who did not write the tables): examines each base semantic table's entries against the producers' definitions of the rows they publish (DEL-04-07, DEL-05-03).

## Governing Values and Decisions — Axiology

- **AX-001** — *A combination is a statement about the model, or it is withheld.* Better is fewer published combinations that are not the response of the model to any loads. Coverage is never bought by publishing a sum the model's nonlinearity or fixed loads make false.
- **AX-002** — *More of what users combine is evaluated.* Better is fewer named refusals of bases, operands and routes. A refusal is never removed by weakening a gate.
- **AX-003** — *Exact arithmetic, reproducible to the bit.* Better is more combination rows a reader can form again exactly. Speed never justifies an order-dependent sum.
- **AX-004** — *The user's combinations, not a code's.* Where a combination could be given a code name or a default factor, the reading that claims less is the better one. Convenience never justifies a bundled combination.
- **AX-005** — *Reliance stated, not implied.* Better is a tighter stated bound on a combined value. Tightness is never bought by a bound that could lie below the true error, or by wording stronger than the bound supports.
- **AX-006** — *One reading of a result.* Better is fewer places where a result's meaning or standing is decided; a second product reader is a defect. A reading is never made more lenient to admit a result: what the reader cannot classify it refuses by name.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-014 OBJ-003 | REQ-001 REQ-009 REQ-010 | AC-001 | VER-001 | Outside source: exact rational arithmetic in generators that import no product code |
| OUT-001 | SOW-014 OBJ-003 | REQ-011 | AC-002 | VER-002 | Outside source: closed-form response of the combined loads on each gated model, which shows the withheld sum is not that response |
| OUT-001 | SOW-014 OBJ-003 | REQ-003 REQ-012 | AC-003 | VER-003 | Outside source: closed form in `validation/references/op1b_difference_basis`. The row text is the criterion's own, under REQ-003 |
| OUT-001 | SOW-014 OBJ-003 | REQ-013 | AC-004 | VER-003 | Outside source: closed form in `validation/references/op_mf_held_status`, generated without product code. The operand mark is the criterion's own |
| OUT-001 | SOW-014 OBJ-003 | REQ-014 | AC-005 | VER-004 | The criterion states the expected record; the support states are the solve's own output, so the check only guards against change |
| OUT-001 | SOW-014 OBJ-003 | REQ-015 | AC-006 | VER-004 | Outside source: exact rationals in `validation/references/combination_certificate`. No independent grounds for the 1e-9 criterion and the floor beyond their use for the operand cases |
| OUT-001 | SOW-014 OBJ-003 | REQ-016 | AC-007 | VER-005 | Outside source: `validation/references/w16_exact_combinations` for the paired rows; the unchanged headlines compare the product with its own output and only guard against change |
| OUT-001 | SOW-014 OBJ-003 | REQ-017 | AC-008 | VER-005 | A verifier written separately from the product, and altered results whose refusal the criterion states |
| OUT-001 | SOW-014 OBJ-005 | REQ-002 REQ-005 REQ-006 REQ-008 | AC-009 | VER-006 | The criterion states the expected finding for each defect; PRD §5.5 and §5.6 say that none is defaulted |
| OUT-001 | SOW-014 OBJ-005 | REQ-004 REQ-007 | AC-010 | VER-006 VER-007 | PRD §20.1 and §21.2 list what must not appear, and DEL-05-04 states what a derived result may carry |
| OUT-001 | SOW-014 OBJ-003 | CLM-007 | AC-011 | VER-006 | Outside source: linear interpolation between two entered points; the criterion states each block |
| OUT-001 | SOW-014 OBJ-003 | REQ-010 REQ-012 | AC-012 | VER-008 | Exit criterion 1 says what must be published. Outside source: the reference system's independently generated references (`validation/references/reference_system`) |
| OUT-002 | SOW-014 OBJ-003 | REQ-018 | AC-013 | VER-009 | The altered results are constructed, so the criterion states each refusal. The row meanings are compared with the reviewed tables, which only shows the reader applies them; AC-016 carries whether the tables are right. The corpus outcomes are the product's own earlier output and only guard against change |
| OUT-002 | SOW-014 OBJ-003 | REQ-019 | AC-014 | VER-010 | The criterion determines the expected result: one verdict for one result text on every surface |
| OUT-002 | SOW-014 OBJ-003 | REQ-018 | AC-015 | VER-011 | A verifier written separately from the reader. Agreement between two readings shows they are consistent, not that either is right |
| OUT-002 | SOW-014 OBJ-003 | REQ-020 | AC-016 | VER-012 | Judgment of each table against the producers' definitions of their rows, by a reviewer who did not write it. No outside reference fixes a row's meaning; the tables' digests only show that they have not changed since review |
