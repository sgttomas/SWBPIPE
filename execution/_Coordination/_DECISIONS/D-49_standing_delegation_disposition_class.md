# D-49 — Standing Decision-Latitude Delegation for Disposition-Class Judgments

**Status:** AWAITING_RULING
**Prepared:** 2026-07-17 (session agent, at owner direction)
**Owner:** Ryan Tufts
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` D-49
**Codification on ruling:** `DEC-082` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12

## 1. Context

The D-47/D-48 chain (DEC-080, DEC-081) closed with two residual judgment
items flagged in the D-48 Wave-2 fan-in log
(`execution/_Coordination/AgentRuns/D-48_2026-07-16/RUN_RECORD.md`). Both
resolve deterministically from information already in the repository, yet
under the current per-instance model each would queue as a separate owner
packet. The owner asked whether the majority of remaining owner judgments
can be effectively delegated under a stated coherence criterion.

Convergent precedent: on 2026-07-17 the owner adopted an equivalent
standing delegation for the chirality-app-dev loop as **D-APP-59 S1**
(`projects/chirality-app-dev/execution/_Coordination/_DECISIONS/D-APP-59_RULING_2026-07-17.md`),
using the same criterion phrasing. This packet proposes the piping-loop
counterpart, mirrored on D-APP-59's reach/limits/attribution structure and
bounded by this project's fences and knowledge constraints.

## 2. Owner Direction (Verbatim)

> I wonder if I can effectively delegate the majority of the remaining
> owner judgments.  If there's enough information at hand already that if I
> instruct you to "decide ensuring consistency and coherence in ontology,
> epistemology, praxeology, and axiology of this project" what would you
> say on these two matters at hand?

and, after the agent's assessment of the two matters:

> draft that standing-delegation DEC packet with the exact class definition
> and the two matters above as its first two recorded applications.

These directions authorize preparation of this packet only. No delegation
exists until the owner rules.

## 3. Proposed Standing Delegation (the exact class definition)

**Criterion.** Judgments exercised under this delegation are decided
*ensuring consistency and coherence in ontology, epistemology, praxeology,
and axiology of this project* — i.e., each disposition must (i) keep the
project's categories referring to real, distinct things (ontology); (ii)
preserve or improve what the record can warrant, never manufacturing or
destroying evidence standing (epistemology); (iii) leave future authors and
agents a workable, drift-resistant rule of action (praxeology); and (iv)
apply the project's already-ruled values — prohibition integrity, single
governed vocabulary, truthful attribution, ruled-history immutability —
rather than creating new ones (axiology).

**Class test (gate for every candidate item).** A judgment is
**disposition-class** — inside the delegation — only if ALL hold:

1. **Applies authority, creates none.** Every premise needed to decide it
   is already ruled, codified, or recorded in the repository; the decision
   instantiates existing authority and does not add scope, stage, spend,
   accountability, or new normative content.
2. **Deterministic under the criterion.** The four-lens test yields one
   coherent outcome; if two defensible outcomes survive, the item is not
   disposition-class and returns to the owner.
3. **Reversible and bounded.** The disposition is a recorded, revertible
   repository change (or a recorded no-change) within existing write
   fences; it triggers no ruled gate.

**Reach.** Within that test, the delegation covers the same three judgment
shapes as D-APP-59 S1, instantiated for this loop:

1. **design-shaped judgments** — selection among fence-respecting means,
   wordings, locations, naming, and structure (e.g., which registered
   variant or citation form resolves a lint finding);
2. **evidence-against-criteria judgments** — whether recorded evidence
   satisfies an explicitly stated criterion (e.g., whether a residual is
   dispositioned, whether a check's stated pass condition is met);
3. **hold-as-truthful-gap judgments** — recording that an item correctly
   awaits a future owner act rather than manufacturing closure.

## 4. Limits (not reachable by this or any delegation)

1. adoption, ruling, and direction themselves (K-AUTH-1; D-GOV-04);
2. deliverable lifecycle issuance and transitions through ruled gates
   (F-PIP-3), and stage advancement of the loop;
3. amendment adoption for authority documents (the PRD, `docs/CONTRACT.md`,
   the claims registry's canonical anchors, fence texts) — the agent may
   draft, never adopt;
4. prover activation, tool procurement, reproduction acceptance, and any
   evidence-posture change under DEC-080/D-46;
5. register-gated integrations and promotions (F-PIP-4) and anything
   touching `_DomainEngines/**`;
6. accountability assignments (ResponsibleParty and evidence-owner roles)
   — human self-binding acts;
7. spending, licensing, publication, and external-facing release acts;
8. the hard fences F-PIP-1 through F-PIP-5, ruled-history immutability,
   and the D-44 receipt contract;
9. merge authority — remains per-chain as expressly granted by the owner;
   this delegation grants none.

Any uncertainty about whether an item falls inside the reach is itself
returned to the owner (consequential-amendment rule, root `AGENTS.md`).

## 5. Attribution and Recording

Every judgment exercised under this delegation is recorded as **the
agent's decision under owner-delegated latitude citing `DEC-082`** — never
as an owner selection or ruling. Each exercise names, in the loop receipt
for its tranche, the artifact carrying its four-lens rationale (run record,
packet appendix, or dated note). Truthful attribution is the firm limit of
the decision-latitude model: a delegated disposition that is ever
represented as an owner act is a violation, not a convenience.

## 6. First Two Recorded Applications (executed on an O-A ruling)

**A1 — Ratify the DEL-13-01 prohibition repair.** During D-48 Wave 2,
node W2-SOW-4 repaired an evident pre-existing typo in
`PKG-13/.../DEL-13-01_Design knowledge schema and provenance model/ScopeOfWork.md`:
"Professional-boundary flags require human review and *require false*
software compliance/certification/sealing/approval/authentication claims"
→ "…and *prohibit* software … claims (PRD §21.2)". Four-lens rationale:
the written sentence denoted an obligation that cannot exist in this
project (ontology); the intended meaning is overdetermined by the sibling
rows, PRD §21.2, and F-PIP-2 (epistemology); leaving it would force every
future reader to re-derive the correction or obey a corrupted rule
(praxeology); prohibition integrity outranks textual conservatism on live,
non-ruled surfaces (axiology). **Disposition: repair stands as-is;** the
Wave-2 fan-in log is its rationale artifact. Class test: applies PRD §21.2,
creates nothing; deterministic; already recorded and revertible.

**A2 — Register the evidence-standing sense of "non-authoritative".**
Roughly ten live SOW passages (e.g. DEL-17-01/-05/-09, DEL-15-02/-04,
DEL-04-01) use hyphenated "non-authoritative" as an *evidence-standing
qualifier* on external-run, parsed, or fixture evidence — a source-basis
label meaning "admissible bounded technical context that does not close
source-basis, code-compliance, or acceptance questions." This is
ontologically distinct from the retired product-surface hedge; the
occurrences are lint-clean and semantically correct. **Disposition: leave
the occurrences unedited; add the definitional entry in Appendix A to
`docs/claims_registry.md` §2** so the sense has a governed home and future
authors have a sanctioned rule instead of drift. Class test: applies
DEC-081's single-vocabulary value, creates no new claim category (the
label describes evidence standing, not software authority); deterministic;
a one-paragraph, revertible registry addition inside the DEC-081 mandate.

## 7. Options

- **O-A (recommended):** adopt the standing delegation as defined in §§3–5,
  codify as `DEC-082`, and execute A1 (ratify) and A2 (registry entry) as
  its first two recorded applications.
- **O-B:** adopt the standing delegation only; A1 and A2 return to the
  owner as separate per-instance items.
- **O-C:** defer; the per-instance delegation model continues unchanged.

## 8. Mechanics on an O-A Ruling

1. Append the verbatim ruling to §9; flip the register row to RULED.
2. Codify `DEC-082` in `SOFTWARE_DECOMP.md` §12 (delegation adopted; class
   test, reach, limits, attribution; A1/A2 recorded).
3. Apply Appendix A to `docs/claims_registry.md` §2; claims lint must
   remain VALID over the full live tree.
4. Record the tranche in a loop receipt (parent: Receipt-51) naming this
   packet as the rationale artifact for A1/A2.
5. No lifecycle, stage, release, prover-activation, or
   reproduction-acceptance change. Standard claim fence applies (F-PIP-2;
   claims taxonomy per DEC-081).

## 9. Human Ruling And Disposition

*(reserved for the owner's verbatim ruling)*

---

## Appendix A — Staged registry amendment (applied only on O-A)

Append to `docs/claims_registry.md` §2 (Evidence-status labels):

> `non-authoritative` (evidence-standing qualifier) — marks external-run,
> parsed, or invented-fixture evidence as admissible bounded technical
> context (regression, handoff review) that does not close source-basis,
> code-compliance, or acceptance questions (PRD §21.2). This is a
> source-basis standing label on governance surfaces, distinct from the
> retired product-surface hedge; it never appears on product surfaces.
