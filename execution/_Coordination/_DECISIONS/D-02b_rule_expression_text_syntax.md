# D-02b — Writable Rule-Expression Text Syntax (Parsing to the Frozen AST)

**Status:** AWAITING_RULING — packet prepared 2026-06-12 at the Phase C2 lead-up, per the `DEC-022` ruling that named this follow-up ("a writable text syntax is deferred to a named follow-up ruling (D-02b) at the Phase C2 editor lead-up", `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 row DEC-022).
**Prepared:** 2026-06-12 by WORKING_ITEMS (Type 1), C2 lead-up decision-preparation subscope.
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-02b.
**Plan basis:** `plans/PLAN_2026-06-10_prd_completion.md` §3 Phase C stage-gate note ("First decision action per the D-14 packet §6: prepare the **D-02b** packet (writable rule-expression text syntax) at the C2 lead-up; C2 AST-based editor work can begin without it per `DEC-022`").
**Parent decision:** D-02 / `DEC-022` (`execution/_Coordination/_DECISIONS/D-02_rule_pack_expression_grammar.md`).
**Epistemic posture:** evidence below is `FACT` with citations; inferences are labeled `ASSUMPTION`; the recommendation is labeled `PROPOSAL`; unknowns stay `TBD`. This packet decides nothing.

---

## 1. Decision statement and scope

**Decide:** whether, when, and in what form rule-pack authors get a **writable text syntax** for rule expressions — a typed-in textual form that parses to the frozen v1.0.0 typed AST — versus continuing with the structured AST composer as the editor's only expression write surface.

**In scope:** adoption timing (now / at a named later trigger / never); the storage posture of any text form (stored in pack documents vs ephemeral editor input); parser placement and security posture; whether a **read-only deterministic text rendering** of ASTs (a pretty-printer with no parser) is permitted before a writable syntax is ruled, and how it must be labeled.

**Out of scope:** the grammar's semantics, function set, versioning, and checksum binding (frozen by `DEC-022`, implemented as grammar v1.0.0 in `TP-C1-GRAMMAR-001`); the rule-pack editor GUI's overall composition (Phase C2 implementation under the coordination loop); any change to the AST as the sole canonical checksum-bound form (all options below preserve it).

**Gating effect:** per the D-02 packet §6 impact map, "the deferred D-02b text-syntax ruling must land before any text box ships" — C2 may build and ship the structured expression composer without this ruling, but no writable expression text input may ship until the human rules here.

---

## 2. Current state evidence

### 2.1 What DEC-022 froze and what it deferred

- `DEC-022` (2026-06-11) froze the in-repo typed expression AST as the sole canonical, checksum-bound grammar: `expression_language: open_pipe_stress_declared_expression`, `declaration_form: declarative_ast`, **no text parser**; `grammar_version` (semver) sits inside the JCS-hashed `rule_pack_checksum` payload (`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 row DEC-022).
- The D-02 packet's Option B (not selected) described the candidate writable-syntax shape: "a small frozen infix text grammar (identifiers, literals-with-units, the same operator/function set, parentheses; no loops/recursion/definitions) with an in-repo parser; the AST remains the sole canonical, checksum-bound form and the text is a stored convenience" (`D-02_rule_pack_expression_grammar.md` §4 Option B).
- The same packet recorded the costs that motivated deferral: "a parser is new, security-reviewable surface in the exact crate whose header promises 'does not parse text'; two representations must be proven equivalent forever (canonicalization burden, round-trip tests); freezing *syntax* as well as semantics doubles the churn-prone surface D-02 exists to pin" (`D-02_rule_pack_expression_grammar.md` §4 Option B, Against).

### 2.2 Grammar v1.0.0 as shipped (the parse target)

- `core/rules/expression_evaluator` implements frozen grammar v1.0.0 (`GRAMMAR_VERSION = "1.0.0"`, `SUPPORTED_GRAMMAR_VERSIONS`, `src/lib.rs`); node set: Literal, VariableRef, Unary (Negate/Abs/Not), Binary (Add/Subtract/Multiply/Divide), Compare (6 operators), Logical (And/Or, eager), Select (eager), Aggregate (Min/Max, n-ary), Interpolate, Lookup (Exact/Step) over user-supplied monotone tables, plus blocking refusal markers UnsupportedForm/UnsafeHostAccess. Power/sqrt are excluded from v1 (named v1.1 candidate per the D-02 packet §5.1).
- The crate header disclaims text parsing: "It does not parse text, execute host-language code, access files, access the network, spawn processes, load plugins, embed protected standards content, or emit professional/code-compliance claims" (`core/rules/expression_evaluator/src/lib.rs:1–6`). **FACT:** any parser must therefore live outside this crate or that constructed property is lost.
- The conformance corpus (`fixtures/rule_expressions/conformance_corpus/`, 69 golden cases) is the freeze artifact; the serialized AST exists as node-tagged JSON objects specified in the corpus README and read today only by the corpus test harness's zero-dependency reader (`core/rules/expression_evaluator/tests/conformance_corpus.rs`). There is no production AST (de)serializer yet.
- **FACT (verbosity datum):** a one-comparison check — actual ≤ allowable — costs roughly ten lines of node-tagged JSON in corpus form (e.g. `fixtures/rule_expressions/conformance_corpus/case_01_accept_logical_and_range_check.json`), versus one short line in any infix text form. Raw pack files are correspondingly hard to hand-review without tooling.

### 2.3 Editor and usability context at the C2 lead-up

- No rule-pack editor or manager UI exists; the workspace IA comment records the rule-pack manager as "intentionally absent: no placeholder dead buttons" (`apps/desktop/src/App.tsx`, WORKSPACE_SECTIONS comment). The C2 structured composer is greenfield work either way — a text box would supplement, not replace, the composer (variable browser, unit checking, table entry, and validation diagnostics are required by PRD §14.5 regardless).
- `DEC-035` (2026-06-12) advanced the target stage to R3 while carrying the **authoring-journey usability finding** as a named blocking residual at the R3 exit review: the human project authority abandoned the TP-MAC-141 packaged-GUI checklist because "the user interface is too cumbersome and opaque" (`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 row DEC-035). **FACT:** usability of authoring surfaces is an active, human-named blocking concern. **FACT:** no usability evidence about a structured expression composer exists yet, because none has been built.
- PRD §14.5 requires the rule-pack editor to provide an "Expression editor" with "Unit checking" and "Validation diagnostics" (`docs/PRD.md:803–818`); it does not commit to text vs structured form. The PRD §12.5 example skeleton shows expressions as opaque placeholder strings ("USER_DEFINED_STRESS_EXPRESSION", `docs/PRD.md:635`) — no text syntax is committed anywhere in the PRD.

### 2.4 Security/sandbox posture constraints

- `OPS-K-RULE-2`: "The expression evaluator is sandboxed; rule packs cannot execute arbitrary code" (`docs/CONTRACT.md`). The schema holds `arbitrary_code_execution_allowed: const false` at formula and payload level (`schemas/rule_pack.schema.yaml`).
- `docs/TYPES.md` glossary hardens the evaluator contract: "evaluates explicit declarative expression trees … does not parse arbitrary text" (`docs/TYPES.md:158`). A writable text syntax does not violate this so long as the parser is a separate component and the evaluator continues to consume only typed ASTs — but the *system-level* "no text parsing" property would no longer hold and the parser becomes permanent security-review surface.

---

## 3. Open questions awaiting ruling

1. **Adoption timing:** now (with C2), at a named later trigger (post-composer usability evidence), or never?
2. **Parser placement (if adopted):** a separate crate (e.g. `core/rules/expression_text_syntax`) parsing to the frozen AST, keeping the evaluator's "does not parse text" header true. Placing the parser inside the evaluator crate would falsify its header and TYPES.md entry. `TBD` pending ruling.
3. **Storage posture (if adopted):** text stored inside pack documents (requires permanent text↔AST equivalence guarantees; checksum still binds only the AST, so stored text can silently drift from the canonical form unless re-rendered on save) vs **ephemeral** editor input that is parsed, converted to AST, and discarded (no document change, no drift surface). `TBD`.
4. **Syntax versioning (if adopted):** does the text syntax ride `grammar_version` (coupling syntax churn to checksum-visible grammar events) or carry its own version (a second frozen surface)? `TBD`.
5. **Read-only rendering:** is a deterministic display-only text rendering of ASTs (no parser; one-way) permitted before/without a writable syntax? It materially improves expression review and diagnostic readability, but it creates a de facto syntax users will expect to type; if later adopted as writable, the rendering becomes retroactively frozen. Should it ship, and under what labeling ("display only — not accepted as input")?

---

## 4. Options

All options preserve the `DEC-022` invariants: the typed AST is the sole canonical, checksum-bound form; the evaluator consumes only typed ASTs; sandbox/determinism properties stay constructed-and-tested; no protected content enters the grammar surface.

### Option O-A — Adopt a writable text syntax now (with C2)

Freeze a small infix text grammar v1 (the D-02 Option B shape: identifiers, literals-with-units, the frozen operator/function set, parentheses; no loops/recursion/definitions) in a new separate parser crate; the C2 editor ships both the structured composer and a text input; parsed text becomes AST immediately; storage posture per §3 Q3.

- For: best authoring ergonomics for non-trivial expressions from day one; directly addresses the human-named "cumbersome and opaque" authoring concern before it recurs in a new surface; raw pack files become hand-reviewable if text is stored.
- Against: every D-02 Option B cost lands now — new permanent security-review surface, forever-equivalence obligations (round-trip corpus), a second frozen surface — and it lands **ahead of any evidence that the composer is insufficient**; slows C2's first shippable slice; the syntax must be frozen in the same tranche that first exercises it, the exact churn risk D-02 was sequenced to avoid.

### Option O-B — Defer with a named re-presentation trigger; no text surface of any kind

C2 ships the structured composer as the only expression surface, with no text rendering anywhere. D-02b is re-presented with evidence after a named trigger: the human's recorded usability assessment of the composer from the R3-exit journey attempt (the TP-MAC-141 successor run), or any earlier recorded human usability finding against the composer.

- For: zero new frozen surface and zero parser security surface until evidence exists; the decision lands later with composer usability data instead of speculation; cheapest C2.
- Against: if the composer proves cumbersome, the finding arrives at the R3 exit review — the most expensive moment to discover it; expression review in raw JSON stays hard (§2.2 verbosity datum) even for reading; diagnostics cannot reference subexpressions in a human-readable form.

### Option O-C — Defer the writable syntax (O-B trigger), but permit a read-only deterministic rendering now

As O-B, plus the C2 composer displays a deterministic, display-only text rendering of the current AST (one-way pretty-printer in the frontend or a small shared helper; no parser anywhere), permanently labeled as display-only and not accepted as input. Validation diagnostics may quote the rendering to locate subexpressions. The rendering's notation is explicitly declared non-frozen and subject to change until a writable syntax is ruled; if D-02b later adopts a writable syntax, the writable grammar starts from the rendering notation where practical.

- For: recovers most of the *review* ergonomics (reading/verifying an authored expression at a glance; readable diagnostics) at near-zero security cost — no parser, no equivalence obligations, no second canonical form; keeps the writable decision evidence-based per O-B; gives the eventual writable ruling a field-tested notation candidate.
- Against: users will predictably attempt to type into what they can read — the label must carry that disappointment; the rendering notation becomes sticky in practice even if declared non-frozen (ASSUMPTION: users screenshot/copy renderings into their own notes, creating informal expectations); a small rendering component must stay in sync with grammar additions (v1.1+).

### Option O-D — Composer-only permanently (close D-02b as "never")

Rule that no writable text syntax will be added; the structured composer is the permanent expression surface; raw pack documents remain machine-oriented.

- For: permanently zero parser surface; one representation forever; simplest security story.
- Against: forecloses an ergonomics remedy before any composer evidence exists; out of step with the PRD §12.5 skeleton's text-expression look; if usability evidence later demands text entry, reopening a "never" ruling is costlier than re-presenting a deferral.

---

## 5. Recommendation — `PROPOSAL`

Adopt **Option O-C**: defer the writable text syntax behind a named evidence trigger, and permit a clearly-labeled read-only deterministic rendering in the C2 composer now.

1. **Writable syntax:** not now. No writable expression text input ships in C2. D-02b is re-presented with evidence at the earlier of: (a) the human's recorded usability assessment of the structured composer during the R3-exit journey run (TP-MAC-141 successor), or (b) any recorded human usability finding against the composer. The re-presented packet inherits §3 Q2–Q4 as its open questions and the O-A "For" case as its baseline.
2. **Read-only rendering:** the C2 expression composer may display a deterministic one-way text rendering of the current AST, labeled in the UI as display-only and not accepted as input, with the notation declared non-frozen. No parser is built anywhere. Validation diagnostics may quote it.
3. **Boundary restated:** the AST remains the sole canonical checksum-bound form; the evaluator crate's "does not parse text" header stays true system-wide (no component parses expression text); grammar v1.0.0 and its corpus are unchanged by this ruling.

Rationale: the composer must be built under any option (PRD §14.5 requires the variable browser, unit checking, and validation diagnostics regardless), so the genuinely deferrable commitment is the parser and its forever-equivalence obligations — exactly the costs D-02 deferred. The one cost of deferral that is *not* speculative is review ergonomics (§2.2 verbosity is a fact today), and O-C recovers that without any parser surface. Committing a frozen writable syntax now (O-A) would freeze notation ahead of the first user-visible composer — repeating, for syntax, the premature-freeze risk the D-02 sequencing deliberately avoided; O-D forecloses with equal absence of evidence in the other direction.

This recommendation is a `PROPOSAL` only. It confers no authority and changes no state.

---

## 6. Downstream impact map

| Surface | Impact of this ruling |
|---|---|
| **Phase C2 editor tranches** | All options keep the structured composer unblocked (already true per `DEC-022`). O-A adds a text-input slice + parser crate to C2 scope; O-C adds a small display-only rendering component; O-B/O-D change nothing in C2 scope. |
| **`core/rules/expression_evaluator`** | Untouched under every option (parser, if ever adopted, lives in a separate crate; §3 Q2). |
| **`schemas/rule_pack.schema.yaml`** | O-B/O-C/O-D: no change (text is never stored). O-A with stored text: optional non-canonical text member + equivalence tooling; checksum semantics unchanged (AST-only payload). |
| **Conformance corpus** | O-A: a text-syntax corpus extension (text → AST golden cases) becomes part of the freeze artifact. Others: unchanged. |
| **Security review surface** | O-A: permanent parser surface, reviewable under OPS-K-RULE-2. O-C: none (one-way rendering only). O-B/O-D: none. |
| **A3 usability lane / R3 exit review** | O-B/O-C place composer usability evidence on the same human journey that already gates R3 exit (TP-MAC-141 successor), so the re-presentation trigger costs no extra gate. |
| **Decision register** | Row D-02b moves to RULED with the selected option; under O-B/O-C the row records the named re-presentation trigger rather than closing the topic. |

---

## 7. Authority and ruling record

Only the **human project authority** rules on D-02b. Agents prepared this packet and may not certify, approve, or adopt it.

Per existing decision practice, the accepted ruling is recorded as a `DEC` entry in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (or its successor register), after which the dispatching persona updates `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-02b from `AWAITING_RULING` to `RULED` with a pointer. This packet does not edit the register's state beyond the preparation pointer and does not resolve the decision.
