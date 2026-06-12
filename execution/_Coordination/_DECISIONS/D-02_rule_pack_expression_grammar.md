# D-02 — Rule-Pack Expression Grammar Freeze (Operator/Function Set, Conformance Suite)

**Status:** AWAITING_RULING
**Prepared:** 2026-06-11 by TASK (Type 2), requested by WORKING_ITEMS (Type 1), tranche TP-DECIDE-PREP-001 decision-preparation subscope.
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-02.
**Plan basis:** `plans/PLAN_2026-06-10_prd_completion.md` §2 row D-02; §3 Phase C item C1.
**Epistemic posture:** evidence below is `FACT` with citations; inferences are labeled `ASSUMPTION`; the recommendation is labeled `PROPOSAL`; unknowns stay `TBD`. This packet decides nothing.

---

## 1. Decision statement and scope

**Decide:** the frozen expression grammar for user-owned rule packs — the final operator/function set, whether the grammar is a purpose-built restricted language or an embedded existing engine, the versioning/freeze semantics that bind the grammar into rule-pack checksums (FR-016), and the shape of the evaluator conformance suite — so Phase C can implement FR-011's final form without grammar churn after rule packs exist in the wild.

**In scope:** operator/function set (arithmetic, comparison, boolean, conditionals, min/max/abs, interpolation, table lookup); unit-aware semantics under the DEC-018 SI-canonical ruling; canonical representation (expression tree vs text syntax); engine provenance (in-repo vs embedded library) with determinism/sandboxing/IP-boundary implications; grammar version binding into `rule_pack_checksum`; conformance-suite form and coverage.

**Out of scope:** rule-pack schema metadata/provenance/lifecycle contracts (exist: `schemas/rule_pack.schema.yaml`, `core/rules/rule_pack_lifecycle`); required-input completeness blocking (FR-012, exists in `core/rules/completeness_checker`); the rule-pack editor GUI itself (Phase C2); numerical tolerance thresholds (D-04); private-pack encryption default (PRD §24 open question, separate human ruling).

D-02 blocks Phase C item C1 ("Expression grammar freeze (D-02) + evaluator conformance suite over the sandboxed expression-tree evaluator in `core/rules`", `plans/PLAN_2026-06-10_prd_completion.md` §3 C1) and gates the FR-011 final form (`_REGISTER.md` row D-02).

---

## 2. Current state evidence

Citations pinned at repo HEAD `5079a8fa7`. Symbol names are the durable anchors if line numbers drift.

### 2.1 What the evaluator implements today (and what it explicitly defers)

`core/rules/expression_evaluator` is a sandboxed declarative evaluator over **explicit expression trees**; its header disclaims: "It does not parse text, execute host-language code, access files, access the network, spawn processes, load plugins, embed protected standards content, or emit professional/code-compliance claims" (`core/rules/expression_evaluator/src/lib.rs:1–6`). **FACT** — the implemented grammar is:

- `UnaryOperator::Negate` only (`lib.rs:129–131`); `BinaryOperator::{Add, Subtract, Multiply, Divide}` (`lib.rs:134–139`); six `ComparisonOperator`s (`lib.rs:142–149`); `Expression::{Literal, VariableRef, Unary, Binary, Compare}` plus explicit refusal markers `UnsupportedForm` and `UnsafeHostAccess` that always produce blocking findings (`lib.rs:152–175, 413–426`).
- **No** boolean and/or/not, **no** conditional, **no** min/max/abs, **no** interpolation, **no** table lookup, **no** power/root. The PRD-required function set (§2.3) is largely unimplemented.
- Dimension algebra is deliberately deferred in-code: multiplying two dimensional quantities yields `UnsupportedExpressionForm` "multiplication of two dimensional quantities remains TBD" (`lib.rs:537–551`); division producing derived dimensions "remains TBD" (`lib.rs:587–593`). Same-dimension division with matching units yields a dimensionless ratio (`ratio_quantity`, `lib.rs:706`).
- Unit semantics are exact-string equality on `unit_ref` (`quantity_units_match`, `lib.rs:702`); mismatches block (`FindingCode::{UnitMismatch, DimensionMismatch}`, `lib.rs:184–198`). There is **no unit conversion** in the evaluator; `core/units` contains only a README — the D-01/DEC-018 catalog crate is Phase B1 work (`plans/PLAN_2026-06-10_prd_completion.md` §3 B1: "currently empty").
- Status boundary: `AnalysisStatus::HumanApprovedForProject` as an evaluation input is a `StatusBoundaryViolation` (`lib.rs:645`) — the software never consumes or emits human-approval state.
- 17 in-crate `#[test]`s exist; there is no cross-cutting conformance corpus for expressions.

### 2.2 The grammar is a long-declared, deliberately human-gated TBD

- **DEC-012** keeps "rule expression grammar/library" an implementation-level TBD "unless a sealed brief or later human ruling resolves them" (`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 row DEC-012, line 582). **OI-006:** "Rule-pack evaluator must be sandboxed, unit-aware, and declarative; exact expression grammar/library remains TBD" (`SOFTWARE_DECOMP.md:553`).
- DEL-06-02 REQ-06-02-006: "The exact expression grammar, parser, and evaluator library remain TBD and require a future human architecture decision before implementation" (`execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-02_Sandboxed unit-aware expression evaluator/Specification.md:18`; same posture at `:33`, `:48`; `Guidance.md:20, 60` row OI-006).
- The schema encodes the open state as data: `expression_language` enum is only `["TBD", "open_pipe_stress_declared_expression"]` (`schemas/rule_pack.schema.yaml:360–366`); `grammar_status` is `["grammar_not_selected", "future_human_approved_grammar_required", "TBD"]` (`:428–435`); `OpenDecision.topic` includes `expression_grammar` and `evaluator_library` (`:469–483`).

### 2.3 PRD requirements the frozen grammar must satisfy

- FR-011 (Must): "Provide rule-pack schema — Users can define private stress checks, variables, expressions, allowables, and load-combination mappings" (`docs/PRD.md:344`).
- PRD §12.3 fixes the evaluator contract: "Unit-aware. Sandboxed. Deterministic. Non-Turing-complete, if practical. Free of arbitrary filesystem or network access. Capable of basic algebra, conditionals, min/max, interpolation, and table lookup using user-provided tables" (`docs/PRD.md:588–597`).
- FR-016 (Must): reports include the "rule-pack checksum" (`docs/PRD.md:349`); generated reports may reference rule-pack ID/version/checksum (`docs/IP_AND_DATA_BOUNDARY.md:91`). The schema already requires a JCS-canonicalized sha256/sha512 `rule_pack_checksum` over the pack payload (`schemas/rule_pack.schema.yaml:169–264`).
- R3 exit criteria: "User can define a private non-code rule pack and run checks" and "Software blocks pass/fail when required inputs are missing" (`docs/PRD.md` §22.4, `:1220–1221`). The rule-pack editor (§14.5) must provide an "Expression editor" with "Unit checking" (`docs/PRD.md:805–814`).
- PRD §12.5's public example skeleton shows expressions as opaque placeholders ("`expression: "USER_DEFINED_STRESS_EXPRESSION"`", `docs/PRD.md:635`) — no text syntax is committed anywhere.

### 2.4 IP-boundary and sandbox constraints on the grammar

- DEC-001: "mechanics in public core, code-specific values in user/private rule packs" (`SOFTWARE_DECOMP.md` §12 row DEC-001, line 571; `docs/PRD.md:18`). The grammar is public mechanism; **all** standards-derived values enter only as user-supplied bindings/value slots (`docs/IP_AND_DATA_BOUNDARY.md:36–52` may/must-not lists; `:85–87` private-path defaults).
- The schema constitutionally forbids executable payloads: `arbitrary_code_execution_allowed` is `const: false` at both the formula and payload level (`schemas/rule_pack.schema.yaml:367–370, 436–439`); `formula_kind` already names `lookup_reference` and `declaration_form` names `declarative_ast | symbolic_reference | structured_expression` (`:339–359`).
- DEC-018 ruled SI-canonical internal units with a dual display catalog and exact governed conversion constants (`SOFTWARE_DECOMP.md` §12 row DEC-018, line 588). The grammar's unit-aware semantics must compose with that ruling.

### 2.5 In-repo precedent for a blessed conformance corpus

`fixtures/model_operations/contract_corpus/` is a 44-case blessed-reference corpus for the operation seam: every case runs inside the normal test suites, must produce identical semantic outcomes and one canonical hash, and carries a provenance block declaring all values invented (`fixtures/model_operations/contract_corpus/README.md:1–27`). **FACT:** this is the project's established pattern for freezing a contract behind golden cases.

---

## 3. Open questions awaiting ruling

1. **Function-set completeness:** is the PRD §12.3 list (basic algebra, conditionals, min/max, interpolation, table lookup) the frozen v1 set verbatim, or are `power`/`sqrt` included? ASSUMPTION (industry, not repo fact): root-sum-square-style combinations in user checks need a square root; excluding it forces awkward workarounds. `TBD` pending ruling — dimensional exponents complicate `power` (see Q2).
2. **Dimension algebra:** the evaluator's two in-code TBDs (dimensional multiply/divide, §2.1) must be resolved. Full derived-dimension algebra vs an enumerated dimension-product table over the existing closed `Dimension` enum (`lib.rs:13–43`)? The closed enum cannot represent arbitrary products without extension. `TBD`.
3. **Unit semantics under DEC-018:** does the frozen grammar evaluate over **pre-normalized SI-canonical quantities** (evaluator never converts; conversion happens at the Phase B1 catalog boundary), or does it convert internally? Current behavior is exact `unit_ref` string match with no conversion (§2.1). This interacts with B1/B2 sequencing since `core/units` is empty.
4. **Canonical representation:** is the typed expression tree (AST) the sole canonical, checksum-bound form, with any text syntax a non-canonical editor convenience — or is a text grammar itself frozen? The evaluator explicitly does not parse text (§2.1); the schema's `declaration_form` admits both (§2.4).
5. **Checksum binding:** does a `grammar_version` field join the JCS-canonicalized payload hashed by `rule_pack_checksum` (so checksum changes whenever the declared grammar version changes), and what does the evaluator do with an unsupported version — blocking diagnostic per `RULE_EVALUATOR_ERROR`, never silent best-effort? Interaction with the checksum-lifecycle states deferred to DEL-06-04 (`schemas/rule_pack.schema.yaml:214–222`). `TBD`.
6. **Table value structure:** interpolation/lookup tables are user-provided (PRD §12.3) but the schema has no table value shape today — only `lookup_reference` and `source_reference` slot kinds (§2.4). Schema extension is implied; its shape is `TBD`.
7. **Corpus blessing:** who blesses conformance-case expected outputs, and where do they live? The contract-corpus precedent (§2.5) was plan-driven with Rust as reference; expression cases have no second engine. `TBD` proposed in §5.

---

## 4. Options

All options are local-only, deterministic, sandboxed (no filesystem/network/process/plugin access), and value-free — standards content enters only as user bindings, never as grammar or repo content (§2.4).

### Option A — Freeze the in-repo typed expression tree as the canonical grammar, extended to the full PRD §12.3 function set

The existing `Expression` AST (§2.1) is the one canonical, checksum-bound form (`expression_language: open_pipe_stress_declared_expression`, `declaration_form: declarative_ast`). Freeze v1.0.0 adds: boolean `And/Or/Not`; eager `Select(condition, then, else)`; n-ary `Min/Max` and `Abs` over same-dimension, same-unit quantities; piecewise-linear `Interpolate` and exact/step `Lookup` over user-supplied monotone tables (out-of-range = blocking diagnostic, no silent extrapolation); dimension-product algebra resolving the two in-code TBDs per a ruled answer to §3 Q2. No text parser in scope. Conformance suite = blessed golden corpus per §2.5 precedent.

- For: smallest distance from shipped code — extends the crate that already enforces sandbox/determinism/status boundaries (§2.1); AST-as-canonical makes checksum binding and JCS canonicalization trivial; no parser attack/ambiguity surface; trivially non-Turing-complete (no loops/recursion/user functions), satisfying PRD §12.3; grammar churn risk minimized because the frozen surface is exactly the enum set.
- Against: rule-pack authors never see a writable syntax — the Phase C2 editor must build structured expression UI (tree/form composer), which is more GUI work than a text box; JSON-serialized ASTs are verbose for humans reading raw pack files; every future function addition is a grammar version event.

### Option B — Option A's AST core plus a frozen purpose-built text syntax that parses to it

As Option A, plus a small frozen infix text grammar (identifiers, literals-with-units, the same operator/function set, parentheses; no loops/recursion/definitions) with an in-repo parser; the AST remains the sole canonical, checksum-bound form and the text is a stored convenience.

- For: human-writable/reviewable expressions in pack files and the §14.5 expression editor; matches the PRD §12.5 skeleton's text-expression look (§2.3); diff-friendly.
- Against: a parser is new, security-reviewable surface in the exact crate whose header promises "does not parse text" (§2.1); two representations must be proven equivalent forever (canonicalization burden, round-trip tests); freezing *syntax* as well as semantics doubles the churn-prone surface D-02 exists to pin; slows C1.

### Option C — Embed an existing expression-engine library as the grammar

Adopt a third-party expression/scripting engine (vendored, local-only) as both grammar and evaluator; rule packs store its source text; unit-awareness wrapped around it.

- For: full function set immediately; less in-repo evaluator code to maintain; familiar syntax for users (ASSUMPTION: general industry familiarity, not a repo fact).
- Against: the freeze is hostage to upstream grammar evolution — "frozen" semantics now mean pinning a dependency forever or diverging by fork; determinism and sandbox guarantees (no host access, non-Turing-completeness, float behavior) must be audited rather than constructed, against PRD §12.3 (§2.3); unit/dimension semantics and the `AnalysisStatus` boundary (§2.1) must be bolted on outside the engine; DEC-012 deliberately held "grammar/library" for a human ruling precisely because library choice is an IP/security commitment (§2.2); `arbitrary_code_execution_allowed: const false` (§2.4) is hard to demonstrate for a general engine.

### Option D — Minimal freeze: pin exactly today's operator set; defer functions to a later grammar v2

Freeze v1 as the currently shipped set (negate, four arithmetic ops, six comparisons) and ship Phase C on it; min/max/interpolation/lookup/conditionals arrive as a later major version.

- For: zero implementation before freeze; C1 reduces to writing the corpus.
- Against: directly fails PRD §12.3's "Capable of basic algebra, conditionals, min/max, interpolation, and table lookup" (§2.3); realistic user checks (e.g., the larger of two invented ratios against a user limit) are inexpressible, hollowing out the R3 exit criterion (§2.3); guarantees a second freeze after packs exist in the wild — the exact cost the plan's D-02 timing note warns against ("grammar churn after rule packs exist in the wild is costly", `plans/PLAN_2026-06-10_prd_completion.md` §2 row D-02).

---

## 5. Recommendation — `PROPOSAL`

Adopt **Option A now, with Option B's text syntax named as a deferred follow-up decision (suggested D-02b) at the Phase C2 editor lead-up**:

1. **Operator/function set (frozen v1.0.0):** negate; add/subtract/multiply/divide with dimension-product algebra resolved per item 3; the six comparisons; `And/Or/Not`; eager `Select`; n-ary `Min/Max`, `Abs`; piecewise-linear `Interpolate` (monotone user tables, no extrapolation — out-of-range blocks) and exact/step `Lookup`. `Power`/`sqrt` are **excluded from v1 and recorded as a named open candidate for v1.1** unless the human rules them in now (§3 Q1) — exclusion is reversible additively; inclusion is not reversible at all.
2. **Engine:** in-repo purpose-built typed AST (extending `core/rules/expression_evaluator`), not an embedded library — determinism, sandboxing, the status boundary, and `const false` code-execution stay constructed-and-testable properties (§2.1, §2.4). No text parser at freeze; `UnsupportedForm`/`UnsafeHostAccess` remain blocking refusal markers.
3. **Unit semantics:** evaluation operates over SI-canonical quantities per DEC-018; the evaluator itself never converts units — conversion is the Phase B1 catalog's job at the binding boundary, and in-evaluator mismatches keep blocking (§2.1, §3 Q3). Dimension algebra: enumerate the ruled dimension-product table over the existing closed `Dimension` enum, with unrepresentable products yielding a blocking `UnsupportedExpressionForm` rather than silent extension (ASSUMPTION: closed-enum honesty is preferable to open algebra at freeze time; revisit at v2).
4. **Versioning/freeze semantics:** add `grammar_version` (semver) to the pack payload; it sits **inside** the JCS-canonicalized bytes hashed by `rule_pack_checksum`, so the FR-016 reported checksum binds the grammar version (§2.3, §3 Q5). The evaluator declares its supported version set; an undeclared/unsupported version is a blocking `RULE_EVALUATOR_ERROR`-class diagnostic, never best-effort. Minor versions are additive-only; any breaking change requires a new major version and a recorded human ruling. The schema enums flip from `grammar_not_selected`/`TBD` to the named grammar (§2.2).
5. **Conformance suite:** a blessed golden corpus `fixtures/rule_expressions/conformance_corpus/` modeled on the 44-case operation contract corpus (§2.5): one JSON case per file (expression AST + bindings + expected value/findings/statuses), executed by plain `cargo test` in the evaluator crate; coverage floor: every operator/function × at least one accepting case and one blocking case per relevant `FindingCode`, plus unit/dimension cases pinned to the DEC-018 semantics; README provenance block declaring all values invented (obviously synthetic magnitudes only — e.g., a lookup table mapping `{10 → 1.5, 20 → 2.5}` in invented units; never standards constants). The corpus **is** the freeze artifact: a grammar version bump without corresponding corpus extension fails review.
6. **IP boundary:** the grammar ships value-free; interpolation/lookup *mechanisms* are public, table *values* are user value-slots with provenance/redistribution metadata (schema extension for table shapes per §3 Q6 to be specified in the C1 brief), preserving DEC-001 and `docs/IP_AND_DATA_BOUNDARY.md:85–91`.

Rationale: Option A is the only option that satisfies PRD §12.3 verbatim at Phase C cost while keeping every sandbox/determinism/IP property a constructed, in-repo, corpus-tested fact rather than an audited dependency promise (Option C) or a doubled frozen surface (Option B); Option D fails the PRD on its face. OI-006/REQ-06-02-006 and the DEC-012 grammar TBD (§2.2) resolve as: grammar = repo-owned typed AST `open_pipe_stress_declared_expression` v1.0.0; library = none.

This recommendation is a `PROPOSAL` only. It confers no authority and changes no state.

---

## 6. Downstream impact map

| Surface | Impact of this ruling |
|---|---|
| **Phase C1 tranche** | Defines C1's acceptance shape: implement the frozen function set + dimension table in `core/rules/expression_evaluator`, land the conformance corpus, flip the schema grammar enums (`plans/PLAN_2026-06-10_prd_completion.md` §3 C1). |
| **Phase C2 rule-pack editor** | AST-canonical means C2 builds a structured expression composer (PRD §14.5 "Expression editor", `docs/PRD.md:805–814`); the deferred D-02b text-syntax ruling must land before any text box ships. |
| **Phase C4 end-to-end checks** | `USER_RULE_CHECKED`/`USER_RULE_FAILED`/`RULE_INPUTS_INCOMPLETE` flows gain expressible real checks (min/max/interp/lookup) on authored models (`plans` §3 C4). |
| **FR-011 / R3 exit** | FR-011 reaches final form; the §22.4 exit criterion ("define a private non-code rule pack and run checks", `docs/PRD.md:1220`) becomes implementable. |
| **FR-016 reports** | The reported rule-pack checksum becomes grammar-version-binding (§5.4), strengthening reproducibility metadata (PRD §15.3) with no renderer change (D-10/DEC-021 unaffected). |
| **`schemas/rule_pack.schema.yaml`** | `expression_language`/`grammar_status`/`OpenDecision.expression_grammar` enums resolve; table-value structure extension required (§3 Q6); `FormulaDeclarationPayload` gains `grammar_version`. |
| **DEL-06-02 (PKG-06)** | REQ-06-02-006 and OI-006 (§2.2) become resolvable at that deliverable's formal review once ruled. No lifecycle state changed by this packet. |
| **DEC-012 TBD set** | The "rule expression grammar/library" member of the DEC-012 TBD list is retired; other members (CI provider, etc.) remain. |
| **Phase B units (B1/B2)** | §5.3 places conversion at the catalog boundary, so B1's crate becomes a hard dependency of full C1 unit semantics; C1 can land grammar + corpus with exact-match semantics first (current behavior) if B1 lags — sequencing note for the dispatching persona. |
| **D-04 tolerances** | Conformance corpus expected values are exact-match on canonical outputs; any tolerance-bearing comparison cases must wait for or reference the D-04 ruling rather than inventing thresholds. |
| **Completeness checker / lifecycle crates** | No contract change: blocking-on-missing-inputs (FR-012) and checksum lifecycle stay as-is; only the checksum payload contents gain `grammar_version` (§5.4, DEL-06-04 interaction `TBD` per §3 Q5). |

---

## 7. Authority and ruling record

Only the **human project authority** rules on D-02. Agents prepared this packet and may not certify, approve, or adopt it.

Per existing decision practice, the accepted ruling is recorded as a `DEC`/`SCA` entry in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (or its successor register) — D-01, D-08, D-10, and D-13 were recorded this way as `DEC-018`, `DEC-019`, `DEC-021`, and `DEC-020` (`_REGISTER.md` rows; `SOFTWARE_DECOMP.md` §12) — after which the dispatching persona updates `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-02 from `AWAITING_RULING` to `RULED` with a pointer (`_REGISTER.md` header). This packet does not edit the register and does not resolve the decision.
