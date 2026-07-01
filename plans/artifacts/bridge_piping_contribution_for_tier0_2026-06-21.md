# Piping-Tier Contribution for the Root Tier-0 Bridge Plan

**Date:** 2026-06-21
**Author:** chirality-piping loop (WORKING_ITEMS persona), domain-specialist seat
**For:** the root `chirality` tier-0 integration agent authoring the `chirality-app-dev` ⇄ `chirality-piping` bridge plan + brief
**Sanction:** harness-independent embedded-agent preparation, ordinary v0.1 prep per `DEC-042` (no R7 adoption, no live binding, no app-dev dependency consumption)
**Status:** CROSS-CHECK INPUT — **not** tier-0 authority

> **Reading order (important).** Read this *after* your own cold pass over the
> tier-0 operative core (`agents/AGENT_DOMAIN_ENGINE.md` PROTOCOL/SPEC/STRUCTURE,
> incl. the OpenPipeStress Example Binding `:709`; the Type-0 parent
> `agents/AGENT_HELPS_HUMANS.md`; `agents/AGENT_EQUATION_AUDIT.md`). This document
> is a specialist cross-check to challenge against, not a lens to read through.
> Every factual claim carries a `file:line` anchor in `projects/chirality-piping/`
> so you can verify it independently rather than trust it.

---

## 0. Why this contribution is shaped differently from app-dev's

The app-dev contribution answers **"is the bridge contractually sound?"** — drift
tables, contract surfaces, fence maps. That is the correct question for the
integration layer, and it is not duplicated here.

This contribution answers a different question that only the domain seat can
answer: **"What must be true for a practicing piping engineer to trust an agent
worker operating through this bridge — and where would that worker confidently
produce wrong engineering?"**

The two are orthogonal and both required. A bridge can be contractually perfect
and still hand a PE a worker they cannot responsibly delegate to. The tier-0
profile and `INTEGRATION_LEVEL` design must satisfy *both* sets of requirements.

The one-line specialist thesis:

> The deterministic engine protects you from **structural** nonsense and from
> **fabricated values**. It is silent on whether *valid* inputs are
> **engineeringly correct**, whether **model premises** are complete, and whether
> anything is **code-compliant**. A trustworthy agent worker is one whose
> autonomy is bounded to where the engine protects you, and whose review burden
> is concentrated exactly where the engine is silent.

---

## 1. The trust boundary, in engineering terms (the keystone — verified)

What the authoritative engine (`core/model_operations/operation_applier`, the
single validate/apply kernel) **guarantees** vs. what it is **silent on**:

| The engine GUARANTEES (trust the green check here) | The engine is SILENT on (human trust must concentrate here) |
|---|---|
| Schema/structural validity of the operation | Whether a schema-valid value is *engineeringly* right (a valid wall thickness can still be wrong) |
| Referential integrity — blocks dangling/missing targets (`core/model_operations/operation_applier/src/lib.rs:9117`) | Whether the *model premises* are complete (a missing thermal case, an omitted support) |
| Unit/dimension correctness and round-trip | Whether the load-combination set satisfies the governing code |
| Provenance carried on every change (`source_note`, before/after) | Whether a support idealization (rigid vs. flexible, gaps, friction) reflects reality |
| **Refusal to invent engineering values** ("values are not invented on the user's behalf", `…/operation_applier/src/lib.rs:1306`) | Whether the result is code-compliant — *no* compliance/certification/approval claim is ever made (`…/operation_applier/src/lib.rs:1136`, `:9482`) |

**Implication for the tier-0 design:** "validation passed" is **necessary, not
sufficient**. The profile must not let a green validation status read as
engineering assurance. This is the single most important thing for the tier-0
agent to encode.

---

## 2. Operation risk-classing — the dimension the `INTEGRATION_LEVEL` ladder lacks

The ladder (`AGENT_DOMAIN_ENGINE.md:168-172`) stages *capability* (read → run →
propose). It does **not** distinguish *engineering stakes within a level*. From
the engineer's seat, the ~21 `change_kind`s (`apps/desktop/src/types.ts:421-441`)
fall into two very different review classes — and a single undifferentiated
"proposal" bucket would mis-set trust:

**Class A — low-judgment, engine-checkable** (the engine's structural validation
covers most of the risk):
- topology/geometry: `create_node`, `delete_node`, `connect_pipe_run`,
  `delete_pipe_run`, node reposition
- scaffolding: `create_load_case`, `create_combination`, `create_combination_term`
- Here an agent worker can be trusted to author freely; review is a sanity check.

**Class B — high-judgment, engine-silent** (the engine validates the *form*, never
the *correctness* — review is irreducible regardless of green checks):
- engineering-value selection: anything setting an **allowable stress, SIF,
  material property, pressure/temperature, support stiffness/gap/friction, or
  load magnitude** via `set_field` / `update_load` / `update_support`
- `create_section`, `create_material`, `create_support` — because their
  engineering *values* are the judgment, even though their *structure* validates
- The system already treats these specially: `completeness_checker` flags an
  **"Invented allowable stress"** and tracks `missing` vs `private_reviewed`
  inputs (`core/rules/completeness_checker/src/lib.rs:585-586`), and the crate
  explicitly "does not provide code-specific defaults" (`…:1-6`).

**Requirement on tier-0:** tag each profile operation with a **risk class /
required-review-depth**, so agent autonomy and human review effort scale to
engineering stakes — not to the capability level alone. (Note: `insert_component_symbol`
is a verified non-applying stub today, `…/operation_applier/src/lib.rs:1300-1306`;
component creation is not yet a real operation regardless of class.)

---

## 3. The physics-grounding rule — `DEC-043` as a trust keystone, not a footnote

The agent worker's single largest credibility risk is confidently asserting a
wrong formula, SIF, or allowable. `DEC-043` already forbids treating the
`domains/piping-design/` corpus's extracted **equations** as authoritative; its
prose/concepts are usable, its equation artifacts are not, until the maintainer's
per-artifact JSON review status clears them (`AGENTS.md` knowledge-source section;
`AGENT_EQUATION_AUDIT.md` is the system of record).

Stated as a trust requirement the tier-0 profile must carry:

> The agent may **operate** the model and **invoke the deterministic, validated
> kernels**. It must **never be the source of engineering physics.** Allowables,
> SIFs, material properties, and governing equations come from the maintainer's
> vetted sources or the engine — never from the LLM's recall or from corpus
> extractions. Every agent-authored engineering value must be traceable to a
> user input, a cited cleared source, or an engine output — never "the model
> produced it."

The substrate to enforce this already exists: the `source_note`/provenance fields
on every `EditorOperationIntent` (`apps/desktop/src/types.ts:408-456`) and the
`missing` / `private_reviewed` input states in `completeness_checker`. The
profile should *require* provenance on Class-B values, not merely allow it.

---

## 4. Results interpretation — the garbage-in-pass-out failure mode

Reading "stress ratio 0.85 — PASS" is L1-trivial and trustworthy *as a read*. But
the dangerous failure mode in pipe stress is **the analysis passing on a wrong
model**: a missing thermal case, an anchor modeled where a guide belongs, a
flexible support idealized as rigid. The result is green; the engineering is
wrong.

So the competence bar for "interpret results" is **not** number-reading — it is
**premise skepticism**: the worker must flag when a PASS is untrustworthy because
the model premises are suspect. The engine supports this posture rather than
fighting it: `rule_check_runner` stamps **"No professional/code-compliance"** on
*every* result and carries `HUMAN_REVIEW_REQUIRED` (`core/rules/rule_check_runner/src/lib.rs:76,139`).

**Requirement on tier-0:** a results-interpretation operation at L1/L2 must
include a **premise-audit obligation** (what model assumptions does this PASS
depend on, and are they justified?), not just a results summary. A worker that
reports green without interrogating premises is a liability, not an asset.

---

## 5. The trust-elevation lifecycle already lives in the schema

The model's `AnalysisStatus` is the engineering-terms version of the propose →
human-accept gate (`schemas/model.schema.yaml:29-40`):

```
MODEL_INCOMPLETE → MECHANICS_SOLVED → RULE_INPUTS_INCOMPLETE
→ USER_RULE_CHECKED → USER_RULE_FAILED → HUMAN_REVIEW_REQUIRED
→ HUMAN_APPROVED_FOR_PROJECT
```

The agent worker operates entirely within and up to **`HUMAN_REVIEW_REQUIRED`**.
Only the engineer elevates to **`HUMAN_APPROVED_FOR_PROJECT`**. The tier-0 profile
should bind its `INTEGRATION_LEVEL`/human-gate map to this existing lifecycle
rather than inventing a parallel one — the reserved human act, in engineering
terms, *is* the `HUMAN_APPROVED_FOR_PROJECT` transition (plus EOR judgment on
Class-B values and code-compliance determination).

---

## 6. What the specialist needs from the tier-0 design (requirements summary)

For a PE to delegate to this worker, the profile + ladder must encode:

1. **Green ≠ assured** — a validation/PASS status must be presented as structural,
   never engineering, assurance (§1).
2. **Operation risk-classing** — Class-A vs Class-B review depth per operation,
   independent of capability level (§2).
3. **Provenance-required on Class-B values** — no agent-authored allowable/SIF/
   material/support/load value without a traceable source (§3).
4. **Premise-audit obligation on results interpretation** (§4).
5. **Reserved human acts in engineering terms** — EOR judgment on Class-B values,
   code-compliance determination, and the `HUMAN_APPROVED_FOR_PROJECT` elevation
   (§5) — i.e., the reserved set is engineering judgment, not only governance
   tokens. (These align with, and are stronger than, the APEGA ceiling in
   `PROFESSIONAL_ENGINEERING.md`.)
6. **Risk-graded `INTEGRATION_LEVEL` staging** — L1 (read) and L2 (run validated
   kernels) are low-risk and may advance early; L3 (`OPERATION_PROPOSAL`) should
   advance **per operation risk class**, not wholesale.

---

## 7. Open questions only the human / tier-0 can resolve (specialist-framed)

- **Data residency for real capability.** An L3 worker that proposes model changes
  *must see the private model and private Class-B inputs* (allowables, SIFs,
  design basis) to be useful — yet piping is no-required-network, private-by-
  default (`docs/SPEC.md:376-377`, `docs/CONTRACT.md:41` OPS-K-PRIV-1). This is
  not a footnote; it is the decision that determines whether L3 is reachable at
  all. Options to rule: local/on-prem inference, redaction, per-session consent.
- **May any Class-A operation ever reach reduced-review autonomy**, or does every
  applied operation require review regardless of class? (Affects how much real
  leverage the worker provides.)
- **Equation-review clearance dependency.** Is the `AGENT_EQUATION_AUDIT` per-
  artifact JSON status populated enough that the worker may cite *anything* from
  the corpus, or is it effectively "concepts only" today (`DEC-043`)?

---

## 8. Verified piping-side facts behind these claims (for independent check)

- Single authoritative mutation kernel, JSON-in/JSON-out, GUI-independent:
  `apps/desktop/src/services/operationService.ts:8-16`;
  `core/model_operations/operation_applier/src/lib.rs:387,397,476,488`.
- Mutation vocabulary (enumerable): `apps/desktop/src/types.ts:408-472`.
- Engine refuses to invent values / no compliance claim:
  `…/operation_applier/src/lib.rs:1136,1306,9482`; referential guard `:9117`.
- Engineering-input completeness + "invented allowable" guard:
  `core/rules/completeness_checker/src/lib.rs:1-6,585-586`.
- Code-check is non-authoritative + human-review-gated:
  `core/rules/rule_check_runner/src/lib.rs:20,76,139`.
- Trust-elevation lifecycle: `schemas/model.schema.yaml:29-40`.
- Headless solve exists (lib-only, needs a thin entrypoint):
  `core/runner/headless/src/lib.rs:655`.
- Governance: `DEC-041/D-22`, `DEC-042`, `DEC-043`, `DEC-037` in
  `execution/_Coordination/_DECISIONS/_REGISTER.md` and
  `execution/_Decomposition/SOFTWARE_DECOMP.md:611-613`.

---

## 9. Companion piping-tier prep (offered, not yet produced)

- **Validate-only / trust-probe spike** — load the shipped `operation_applier`
  wasm and call `validate_operation_json` outside the GUI, including a
  structurally-valid-but-engineeringly-questionable Class-B operation, to
  *empirically* demonstrate §1 (the engine accepts the form, not the
  correctness). Harness-independent, no apply, no network — DEC-042-sanctioned.
- **Surface reconciliation** — map piping's real surfaces against the existing
  OpenPipeStress Example Binding (`AGENT_DOMAIN_ENGINE.md:709`) and Minimal
  Profile Shape (`:669`) into a gap list for the profile draft.

These feed the tier-0 plan; they do not author it.
