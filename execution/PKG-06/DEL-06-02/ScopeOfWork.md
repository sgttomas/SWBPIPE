---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-06-02
package_id: PKG-06
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-045]
package_objective_refs: [OBJ-005]
---

# Scope of Work — DEL-06-02 Sandboxed unit-aware expression evaluator

## Purpose and Objective Traceability

A user's formula is computed safely and with its units. The expressions in a rule pack are data. Evaluating one cannot run code, read a file or reach the network. Every quantity in it carries a dimension and a unit, and an operation on incompatible quantities yields a finding and no number. The same inputs give the same value and the same findings, in the same order, every time. The result this deliverable contributes is that a user's own criteria can be applied to solver results without the product executing anything a pack's author wrote.

It serves SOW-045 under OBJ-005 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-06, row DEL-06-02), and `docs/PRD.md` §17.3, FR-RULE-002, §5.6, §10.6 step 3 and §22.4, with `docs/CONTRACT.md` OPS-K-RULE-2 and OPS-K-UNIT-1, `docs/SPEC.md` §7 and threat STM-007 of `docs/security/threat_model.md`. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The expression grammar and its evaluator: the typed expression tree with its encoding in a pack document, evaluation over bound quantities, the static type check of an expression, and the conformance corpus that fixes the grammar.

## Deliverable Definition — Ontology

**Terms the evaluator implements.** The grammar and its function set are those of DEC-022 (`execution/_Coordination/_DECISIONS/D-02_rule_pack_expression_grammar.md`), with the dimension table and unit-reference composition of DEC-031 and the text-syntax decision DEC-037.

- *Expression.* A typed tree. The tree is the only canonical form of a formula and is bound by the pack's checksum. There is no text syntax and no parser anywhere in the product. A one-way rendering of a tree as text may be displayed, labelled as display only and not accepted as input.
- *Grammar version.* A semantic version a pack declares. The evaluator supports a stated set of versions and blocks every other; it never evaluates on a best-effort basis. A minor version only adds; a breaking change is a new major version. The conformance corpus is the statement of a version: a grammar change without a corpus extension and a version change is not a grammar change.
- *Function set.* Numeric literal and variable reference; add, subtract, multiply, divide, negate and absolute value; the six comparisons; and, or and not; the conditional `select`; `min` and `max` over any number of operands; piecewise-linear `interpolate`, and `lookup` in exact or step mode, over a user-supplied table with strictly increasing arguments. Power and square root are not in the grammar.
- *Quantity.* A value with a dimension and a unit reference. The grammar's dimensions are a closed set.
- *Dimension algebra.* Addition, subtraction, comparison, `min`, `max` and the two branches of `select` require the same dimension and the same unit reference. A dimensionless factor or divisor leaves the other operand's dimension and unit reference unchanged. A quotient of two quantities of the same dimension and the same unit reference is dimensionless, with reference `ratio`. Any other product or quotient of dimensions resolves only through an enumerated table of eighteen mechanics relations, such as force times length is moment and stress times section modulus is moment. A product or quotient that the table does not hold, or that it resolves ambiguously, is a blocking finding. The unit reference of a product is its factors' references joined in sorted order, of a quotient its numerator's over its denominator's, and of a dimensionless result `ratio`.
- *No conversion.* The evaluator converts no unit. A bound value reaches it already in its declared unit; the conversion is made at binding, through the unit catalog (DEL-06-03).
- *Eager evaluation.* Every operand, and both branches of `select`, are evaluated in a fixed order, so a blocking finding in a part the value does not need still blocks.
- *Tables.* An argument outside a table's range blocks, with no extrapolation and no clamping. An exact lookup that matches no row blocks.
- *Finding.* A code from a closed set, the identifier of the subject, and a message. The codes cover an unsafe construct, an unsupported form, a missing variable, a duplicate binding, an invalid reference, a missing required value, a value that is not finite, division by zero, missing unit metadata, a unit mismatch, a dimension mismatch, a type mismatch, a status-boundary violation, an unsupported grammar version, a malformed table, a table argument out of range and a table key not found.
- *Static type check.* The check of an expression from its variables' declared dimensions and units alone, with no values. It applies the evaluator's own rules and reports the findings that depend only on types and on the pack's literals. A variable declared without complete unit metadata is unknown, and no finding is reported against an unknown operand.
- *Status boundary.* An evaluation result carries analysis statuses and can never carry human approval.

**Undecided.**

- Whether a pack may declare a unit that is not in the unit catalog, bound only where the entered unit string is identical. DEL-02-02 commits that an unknown unit is refused at the rule-input boundary.
- Whether the catalog's dimensionless symbol and the grammar's `ratio` reference denote one unit.
- Whether a rule comparison takes a tolerance. Comparisons are exact in binary64.
- What a rule check does with a solver value that is published as an interval.
- Whether a validation finding may quote the display-only rendering of an expression, as DEC-037 permits. AC-009 excludes it, under threat STM-007.

**Boundary.** This deliverable evaluates and type-checks expressions. The pack format, and where a pack declares its variables, are DEL-06-01's. Binding data to variables, converting bound values to declared units, the outcome of a check and how a finding appears in a run result are DEL-06-03's. The binding of the grammar version into the pack checksum is DEL-06-04's. The expression composer is DEL-07-03's, and unit definitions and conversions are DEL-02-02's. Evaluation of expressions on behalf of a plugin or adapter is outside v1.0, as `docs/RELEASE_TARGET_v1.0.md` states for rule packs (also cited as DEL-06-02:3).

## Completion and Reliance Basis — Epistemology

A value the evaluator returns is the user's formula applied to the user's data. It establishes nothing about whether the formula or the data is right.

### Requirements

Requirement numbers 001 to 011 are those of the earlier `REQ-06-02-0nn` series.

- **REQ-001** — The evaluator is sandboxed and incapable of arbitrary code execution.
- **REQ-002** — The evaluator is declarative: a rule pack defines expressions and checks, not programs. The grammar has no loop, recursion or user-defined function, so the work of an evaluation is bounded by the size of the expression.
- **REQ-003** — All expression inputs, intermediate quantities, comparisons and outputs are unit-aware and dimensionally checked.
- **REQ-004** — A missing variable, a missing required value, a unit mismatch, an invalid reference and an unsupported expression form each produce an explicit finding and never a default.
- **REQ-005** — A variable binds only to a declared rule-pack variable, a solver result field or a user-supplied design-basis input.
- **REQ-006** — The grammar is the declarative typed tree defined above, within its sandbox boundary. A change to the grammar, an extension included, is made only by the owner's decision, as a new grammar version.
- **REQ-007** — Public artifacts of the evaluator include no protected standards text, table, code-derived formula, material allowable, SIF or flexibility data, proprietary vendor data, owner standard or private rule pack.
- **REQ-008** — An evaluation result is never represented as code compliance, certification, sealing, approval or professional reliance (PRD §21.2).
- **REQ-009** — Evaluation keeps mechanics solved, user-rule checked, incomplete data and human approval as separate states.
- **REQ-010** — No adapter or plugin reaches evaluation around the sandbox, the unit checks, the provenance checks, the diagnostics or the data boundary.
- **REQ-011** — The evaluator is examined for rejection of unsafe expressions, dimension mismatch, missing bindings, deterministic findings, the protected-content boundary of its public examples, and bypass attempts by a plugin or adapter. The last is examined with the plugin and adapter runtime, after v1.0 (also cited as DEL-06-02:1).
- **REQ-012** — Evaluation is deterministic: the same expression, bindings and grammar version give the same value and the same findings in the same order (PRD §17.3).
- **REQ-013** — The units of every check's formula are checked when a pack is validated, without values, so that a check that can never evaluate is reported to its author before it is run (PRD §10.6 step 3; FR-RULE-002).
- **REQ-014** — A finding never discloses the private content of a pack: no magnitude, table row or rendered formula (threat STM-007).

### Acceptance criteria

- **AC-001** — No expression causes the evaluator to read or write a file, open a network connection, start a process, load code or evaluate host-language text. A node of a kind outside the grammar, each unsafe request form included, yields its named finding and no value. The evaluator's module has no dependency or call that performs such access.
- **AC-002** — For each function of the grammar, evaluation on invented inputs gives the value its definition gives, exactly, where the inputs and the result are exactly representable in binary64: the arithmetic and comparison operators, the logical operators, both outcomes of `select`, `min` and `max` of one and of several operands, interpolation at a row and between rows, and lookup in each mode.
- **AC-003** — Each operation on compatible operands gives the stated dimension and unit reference, for every relation of the dimension table in each direction, and a dimensionless scaling and a same-dimension quotient give the stated dimension and reference. An addition, subtraction, comparison, `min`, `max` or pair of `select` branches on operands of different dimension or unit reference, a same-dimension quotient of different unit references, a product or quotient outside the table, an ambiguous quotient, and a quantity without unit metadata give their named findings and no value.
- **AC-004** — A missing variable, a duplicate binding, an empty reference, a missing required value, an input or intermediate result that is not finite, a zero divisor, a malformed table, a table argument out of range and an exact lookup with no matching row each give their named finding and no value. Nothing is defaulted, extrapolated or clamped, and a blocking operand blocks even where the selected value does not use it.
- **AC-005** — The same expression, bindings and grammar version give a bit-identical value and the same findings in the same order on every run and on every supported platform.
- **AC-006** — A pack whose grammar version is missing, malformed or unsupported is blocked with a named finding and evaluates nothing. The conformance corpus has, for every function of the grammar, an accepting case and a blocking case for each finding code that function can raise, and cases for the unit and dimension rules; every case reproduces.
- **AC-007** — Every corpus expression decodes from its pack encoding to a tree and encodes back to the same document. An expression with an unknown node or operator is refused with a named finding and its path in the pack.
- **AC-008** — For every corpus case, each expected finding that depends only on types and literals is reported by the static type check with the same code and subject; on a case whose expected findings are all of that kind, or that has none, the static findings equal the expected ones. Validation of a pack reports, for each check, whether its formula and its limit are consistent in dimension and unit with the declared inputs and slots, and reports a check that would block on units at every run.
- **AC-009** — No finding from evaluation or from validation contains a magnitude, a table row, a summary of a formula's payload or a rendering of an expression; its message names only codes, subject identifiers, paths in the pack, and dimension and unit tokens.
- **AC-010** — No evaluation result carries human approval, compliance, certification or sealing, and an input status of human approval is not passed through as one.
- **AC-011** — The conformance corpus and every other public fixture of the evaluator hold invented values only, and none holds protected or private content.
- **AC-012** — Evaluation has two callers in the product, the validation of a pack and the rule-check run, and no import, export, adapter or plugin path evaluates an expression.

## Production and Verification Method — Praxeology

**Method choices that define the evaluator.**

- The evaluator is purpose-built and in the repository. It takes a tree and bindings in memory and has no access to anything else.
- Dimensions are a closed set and their products an enumerated table. A relation that is not in the table is refused; the set is never extended silently.
- The static type check shares the evaluator's rules for products, quotients, composed units and unit equality, and it visits every operand because it has no values to stop at.
- The corpus holds invented values chosen to be exact in binary64, and compares values exactly and findings in order.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-012, executable check with judgment by a reviewer who did not write the evaluator: observes the evaluator module's dependency set and its source for file, network, process and code-loading interfaces, the modules that depend on it, and the finding for each unsafe form; the reviewer examines the result against threat STM-007.
- **VER-002** — AC-002, AC-003, AC-004 and AC-006, comparison with a named reference: the conformance corpus, each case's expected value and findings recomputed from the grammar's definitions by a reference that imports no evaluator code; with an executable check of the corpus's coverage of each function with each finding code it can raise, and of the unit and dimension rules.
- **VER-003** — AC-005, executable check: evaluates every corpus case repeatedly and on each supported platform and compares the value's bits and the findings' order.
- **VER-004** — AC-007 and AC-008, executable check: round-trips every corpus expression through the pack encoding, compares the static findings with each case's expectations, and observes the unit findings of constructed packs.
- **VER-005** — AC-009 and AC-010, executable check: validates and evaluates constructed packs that carry sentinel magnitudes, table rows and private strings, and observes every serialized finding, run result and status.
- **VER-006** — AC-011, judgment by a reviewer who did not write the corpus, with the protected-content scan: examines every corpus case and fixture and its provenance statement.

## Governing Values and Decisions — Axiology

- **AX-001** — *Sandbox before expressiveness.* Better is a surface small enough to be read and audited whole: fewer node kinds and no escape hatch. A function is never added by weakening the absence of code and of input or output, and a new function enters only as a new grammar version (REQ-006).
- **AX-002** — *Block, do not guess.* Better is fewer evaluations that return a number where an input, a unit or a table range was in doubt; the target is none. A value is never produced by defaulting, converting silently, extrapolating or clamping.
- **AX-003** — *The same answer every time.* Better is more of an evaluation's output that is fixed by its inputs, finding order included. Determinism is never traded for speed, as by stopping at the first operand that decides a value.
- **AX-004** — *Say what is wrong, not what the rule is.* Better is a finding that names its subject and the dimensions and units involved more exactly. It never does so by echoing a value, a table row or a formula.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-045 OBJ-005 | REQ-001 REQ-002 REQ-006 | AC-001 | VER-001 | PRD §17.3 and `docs/CONTRACT.md` OPS-K-RULE-2 state what must not happen. The comparison is the module's dependency manifest and source, read by a check that imports no evaluator code |
| OUT-001 | SOW-045 OBJ-005 | REQ-003 REQ-006 | AC-002 | VER-002 | Outside source: closed form, the arithmetic, ordering, linear interpolation and table lookup the grammar defines, on invented values exact in binary64 and recomputed without evaluator code. The function set is the grammar's |
| OUT-001 | SOW-045 OBJ-005 | REQ-003 | AC-003 | VER-002 | DEC-022 and DEC-031 fix the dimension table and the composition of unit references. Outside source for the table's relations: dimensional analysis |
| OUT-001 | SOW-045 OBJ-005 | REQ-004 REQ-005 | AC-004 | VER-002 | The inputs are constructed, so the criterion states the expected finding (PRD §5.5; the table rules above) |
| OUT-001 | SOW-045 OBJ-005 | REQ-012 | AC-005 | VER-003 | The criterion fixes the expected result: equality between runs and platforms |
| OUT-001 | SOW-045 OBJ-005 | REQ-006 | AC-006 | VER-002 | The blocked pack is constructed, so the criterion states the expected finding. The corpus's coverage is counted against the grammar's function set and finding codes, the floor the criterion states |
| OUT-001 | SOW-045 OBJ-005 | REQ-005 REQ-006 | AC-007 | VER-004 | The criterion fixes the expected result: the document read back equals the document written. The encoding is the corpus's |
| OUT-001 | SOW-045 OBJ-005 | REQ-003 REQ-013 | AC-008 | VER-004 | The corpus expectations, which rest on the grounds of AC-002 to AC-004; the agreement of static and evaluated findings is fixed by the criterion |
| OUT-001 | SOW-045 OBJ-005 | REQ-014 | AC-009 | VER-005 | Threat STM-007 and PRD §19.4 and §20.3 state what must not appear. The sentinel content is constructed |
| OUT-001 | SOW-045 OBJ-005 | REQ-008 REQ-009 | AC-010 | VER-005 | PRD §21.2 and the status vocabulary of DEL-02-03 |
| OUT-001 | SOW-045 OBJ-005 | REQ-007 REQ-011 | AC-011 | VER-006 | PRD §20.1. The reviewer compares against sources the product did not write |
| OUT-001 | SOW-045 OBJ-005 | REQ-010 REQ-011 | AC-012 | VER-001 | The decomposition's AB-00-07, no bypass. The comparison is the repository's module graph |
