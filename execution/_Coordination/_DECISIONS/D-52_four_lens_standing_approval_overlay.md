# D-52 — Four-Lens Standing Approval Overlay for Resolvable Owner Gates

**Status:** OWNER-DIRECTED; `S5_COMMIT_SAFE`;
`READY_FOR_DURABLE_LANDING`; operational effect held until landing
**Date:** 2026-07-18
**Owner:** Ryan Tufts
**Prepared by:** HELPS_HUMANS for HELP_HUMAN
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` D-52
**Codification:** `DEC-085` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12
**Managed run:** `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260718-STANDING-APPROVAL-R2/`

## 1. Purpose and Current Effect

This record formalizes the owner's current direction as a project-local
standing approval for the class of owner-gated matters that can be resolved
to one outcome by the project's ontology, epistemology, praxeology, and
axiology. It also applies the proposed contract to the pending DEL-09-04
candidate brief.

The owner act and every later agent classification remain distinct:

- the **owner act** is the conditional class-level standing approval and
  adoption rule quoted in §2 and formalized here;
- the **agent act** is limited to `CLASSIFY_ELIGIBLE`,
  `ACTIVATE_OWNER_STANDING_APPROVAL`, `HOLD`, or `INELIGIBLE` under that rule;
- adoption of a governed brief remains strictly the human act. An agent never
  adopts or rules, and activation is never represented as an agent adoption or
  as an owner case-specific selection.

The actual `DEC-083` owner-mediated sibling-project S5 review returned
`COMMIT-SAFE` with no correction required, after the fresh local adversarial
verifier returned `COMMIT-SAFE`. The combined owner approval and agent
classification are therefore `READY_FOR_DURABLE_LANDING`. No downstream
DEL-09-04 execution is released until this owner-authored standing approval
plus the agent classification are committed and published in the governed
record as required by D-GOV-04 and K-AUTH-2.

## 2. Owner Direction (Verbatim)

<!-- BEGIN OWNER DIRECTION VERBATIM D-52 -->
Can you resolve this yourself using the heuristic of consistency and coherence within the project as a whole, in terms of ontology, epistemology, praxeology, and axiology?  Do so if possible and carry that heuristic as a standing approval on matters that can have their owner-gated rulings resolved that way.
<!-- END OWNER DIRECTION VERBATIM D-52 -->

Canonical SHA-256 of the UTF-8 text between the markers, excluding marker
lines and adjacent delimiter newlines:
`a31a551af14b10381087cb9150f3ef6e9a1339d866d4e1b8a221e68cca36f7b2`.

This is the owner's conditional class-level standing approval: for every case
that satisfies the contract below, the owner adopts the uniquely determined
outcome. It is not a case-specific owner selection; the owner authors the
adoption rule, and the agent may only establish and record whether its
conditions are satisfied.

## 3. Authority and Relationship to DEC-082 / DEC-083 / DEC-084

This is **Standing-Approval Overlay v1**, a piping-local layer on the existing
delegation instrument:

1. Shared-Block v1 in D-50/`DEC-083` is not edited. Its canonical bytes and
   SHA-256 remain unchanged.
2. D-49/`DEC-082`, D-50/`DEC-083`, D-51/`DEC-084`, and near-miss entries N1–N6
   remain immutable ruled history.
3. Shared-Block v1 continues to govern the disposition-class lane. This
   overlay adds a separate **standing-approval lane** for a narrowly bounded
   class of owner-gated matters. It does not retroactively reclassify prior
   decisions or delete any near miss.
4. The overlay does not relax D-49 limit 1 or the ratified
   `WORKFLOW_COMPONENT_STANDARD.md` §5.1 rule that governed-brief adoption is
   human-owned. When a matter passes this contract, the owner standing approval
   supplies the conditional adoption. The agent only classifies eligibility
   and activates the already-authored owner rule; it does not author an owner
   ruling, direction, adoption, or case-specific approval.
5. All other D-49 limits remain fast-reject evidence unless this record names
   a narrower controlling boundary. In particular, new authority or normative
   content, professional reliance, lifecycle/stage/release acts, external
   commitments, and destructive/integration authority stay nondelegable.

The root authority chain remains controlling: `DIRECTIVE.md` §§2–3;
`CONTRACT.md` K-AUTH-1/K-AUTH-2/K-BIND-1/K-SEAL-1/K-CONFLICT-1/K-CLAIM-1;
`WORKFLOW_COMPONENT_STANDARD.md` §§5 and 8; D-GOV-04; root/project
`AGENTS.md`; and the piping fences F-PIP-1 through F-PIP-5.

## 4. Operational Standing-Approval Contract

### 4.1 Fast-reject boundary

A candidate matter is **not eligible** when any outcome depends on, performs,
or changes any of the following:

1. irreducible owner preference, personal identity, competence, or a choice
   for which two defensible project-coherent outcomes remain;
2. personal, professional, fiduciary, contractual, safety, hazard,
   residual-risk, code-compliance, legal, or regulatory accountability;
3. a conflict/contradiction ruling where the accepted authority chain does not
   itself determine the result;
4. scope inclusion/exclusion/boundary change, new project/product/lifecycle
   acceptance criteria (or run criteria not derived from accepted sources),
   new normative content, a governing-code/design-basis change, or an exception
   or override to an objective governance block;
5. `CHECKING`/`ISSUED` transition, issuance, stage or milestone advancement,
   release or reliance acceptance, professional authentication, reproduction
   acceptance, external-prover correlation acceptance, or evidence-posture
   promotion;
6. third-party consent or authority, procurement, spending, licensing, legal
   instrument adoption, a financial/binding commitment, publication, public
   release, external communication, networked service use, or external-system
   mutation;
7. merge/integration authority changing an accepted baseline, force or
   history-rewriting Git, destructive/irreversible action, or cleanup whose
   exact recovery cannot be proven;
8. protected, proprietary, confidential, or private data exposure, or any
   action prohibited by OPS-K-IP-1..3, OPS-K-PRIV-1..2, or F-PIP-1/F-PIP-4;
9. evidence unavailable in the live project, an unresolved `TBD`, a stale or
   unaccepted basis, an unverifiable external fact, or a claim stronger than
   its warrant;
10. protected domain-engine paths, prover/tool activation, account/secrets
    handling, ResponsibleParty/evidence-owner assignment, or any other
    higher-order human boundary in root/project governance.

The limits screen runs first. A hit returns the matter to the owner without
four-lens balancing. Ambiguity is a hit. This preserves the error asymmetry in
Shared-Block v1: over-referral costs attention; over-deciding converts human
authority.

### 4.2 Eligibility gates

Only a matter passing the limits screen receives the full test. It is
**standing-approval eligible** only when all gates pass:

1. **Live-tree sufficiency.** Every outcome-determining premise is present in
   current, accepted project artifacts opened in the live tree. Derivative
   summaries and prior tasking are maps, never authority.
2. **Existing-authority application.** The classification applies recorded
   authority, scope, criteria, and values; it creates none.
3. **Four-lens convergence.** Ontology, epistemology, praxeology, and axiology
   each support the same bounded outcome. A lens may constrain but may not be
   omitted or balanced away.
4. **Single surviving outcome.** An adversarial search leaves exactly one
   defensible outcome. Rejection or deferral is not a second defensible outcome
   merely because inaction is physically possible; it must have positive
   support in the accepted project purpose, evidence, or risk record.
5. **Bounded reversibility.** The immediate effect is local, finite,
   reviewable, and reversible before any nondelegable gate. Later execution,
   if any, has its own sealed brief, exact writes, failure posture, and gates.
6. **Truthful attribution.** The record names the owner standing approval and
   the agent classification/activation separately. It states that adoption is
   the owner's act under the standing rule, never an agent act, while recording
   `OwnerCaseSelection=NONE` because no separate case-specific choice occurred.
7. **Rationale artifact.** The exercise records sources, limits-screen result,
   four-lens reasoning, the attempted failure mode, classification, effect, and
   preserved gates in an approved rationale home.
8. **Independent refutation before effect.** A fresh-context verifier receives
   enumeration-derived claims to refute and returns only `COMMIT-SAFE` or
   `BLOCK`. `BLOCK`, missing S5 review where applicable, or missing durable
   human-approval binding prevents effect.

### 4.3 Four-lens test

- **Ontology — what exists and what kind of act is proposed?** Confirm stable
  identities, authority classes, lifecycle classes, evidence classes, and
  boundaries. Do not collapse proposal, approval, execution, evidence,
  acceptance, issuance, or professional reliance into one thing.
- **Epistemology — what does the record warrant?** Cite accepted sources,
  surface gaps/conflicts, keep claims within warrant, and refuse if unavailable
  evidence or owner preference is needed.
- **Praxeology — what rule of action follows?** Select the outcome that gives
  future operators a bounded, executable, fail-closed, drift-resistant path
  under existing workflow and write fences.
- **Axiology — what ruled values govern?** Apply public welfare, evidence over
  plausibility, human responsibility, prohibition integrity, privacy/IP,
  truthful attribution, reversibility, and progress toward adopted project
  purpose; create no new value hierarchy.

### 4.4 Eligible judgment shapes

Subject to every gate above, the overlay may cover:

- conditional owner adoption of a bounded internal candidate brief whose scope,
  acceptance basis, risks, writes, and exclusions are already fixed and which
  stops before every nondelegable act;
- classification against explicit criteria that does not constitute
  lifecycle, release, reproduction, prover, professional, or external-reliance
  acceptance;
- selection among equivalent internal, fence-respecting means, wording,
  naming, location, sequencing, or structure when one outcome survives;
- a truthful `HOLD`/`INELIGIBLE` classification when the record warrants no
  action.

Eligibility is case-by-case. No artifact type is eligible merely because it
appears in this list.

### 4.5 Attribution and effect record

Every exercise uses this form:

```text
OwnerStandingApproval: DEC-085 / D-52 §2
AgentClassification: <CLASSIFY_ELIGIBLE | HOLD | INELIGIBLE>
RuleActivation: <ACTIVATE_OWNER_STANDING_APPROVAL | NOT_ACTIVATED>
ClassifiedBy: <agent role and run/instance ID>
AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL
OwnerCaseSelection: NONE
RationaleArtifact: <path>
IndependentVerifier: <COMMIT-SAFE path | BLOCK path>
EffectStatus: <HELD | READY_FOR_DURABLE_LANDING | EFFECTIVE>
PreservedGates: <named gates>
```

`EFFECTIVE` requires the human standing-approval record to be durable and
SHA-bound, every applicable independent review to pass, and the downstream
brief to cite that record. The owner standing rule performs any adoption;
classification and activation do not. Git closeout transports the record; it
is not itself approval.

## 5. Current Application — CB-2026-07-18-DEL-09-04-CLEAN-REPRO-001

### 5.1 Live basis opened

- candidate brief at
  `execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md`;
- its prior managed planning run
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260718-DEL0904-R1/`;
- DEL-09-04 `_STATUS.md` (IN_PROGRESS; actor-neutral clean-checkout item open);
- `DEC-080`/D-47 (the reproduction is ordinary, actor-neutral loop work;
  evidence home fixed; no reproduction acceptance);
- D-46 supersession note (prior acceptance-protocol question preserved as
  history; no acceptance work proceeds under D-46);
- approved DAG-007 and all eight active DEL-09-04 execution-upstream rows as
  `SATISFIED`, as revalidated by the prior run;
- current loop workplan and structurally valid Receipt-55 cursor.

### 5.2 Limits screen

`PASS`. Adoption of this brief authorizes only a later local evidence run. It
does not accept a reproduction result, promote evidence posture, activate a
prover, change code/scope/criteria, move lifecycle or stage, publish, spend,
merge, access protected data, or perform an external act. The brief expressly
forbids those actions and fails closed on missing offline prerequisites.

### 5.3 Four-lens convergence

- **Ontology:** the pending object is an internal execution brief, not the
  reproduction evidence, its acceptance, a lifecycle transition, or a release.
  DEL-09-04 and the exact Remaining item already exist under DEC-080; the
  brief creates no new project entity or scope.
- **Epistemology:** the brief claims no run result. It requires a clean pinned
  checkout, recorded environment/argv/exit codes/hashes, JSON predicates,
  immutable derivative-bundle identity, and truthful `PASS`/`FAIL`/`BLOCKED`
  status. A later run cannot close scope on weak or missing evidence.
- **Praxeology:** the brief is executable, local-only, offline/no-install,
  exact-write-fenced, rerunnable by new immutable run ID, and fail-closed. It
  advances the widest lawful DEL-09-04 item selected by the adopted loop plan
  while preserving every repair and owner gate.
- **Axiology:** adoption advances evidence over plausibility and the adopted
  PRD purpose while preserving privacy/IP, public-welfare/professional
  boundaries, truthful attribution, F-PIP-1..5, and the DEC-081 claim fence.

All four lenses classify the brief **ELIGIBLE FOR ACTIVATION OF THE OWNER'S
CONDITIONAL STANDING APPROVAL FOR LATER BOUNDED EXECUTION**.

### 5.4 Adversarial failure attempt and single-outcome result

Attempted failure mode: classify any reproduction-related act as prohibited by
D-49 limit 4 or D-46. The attempt fails because the candidate performs only
actor-neutral internal reproduction under DEC-080; D-46's acceptance question
is superseded and the candidate excludes reproduction acceptance,
`PROVER_CORRELATED`, `ENGINEER_ACCEPTED`, and every external/publication act.

Attempted alternative: defer despite no defect. It does not survive: the live
loop directs the widest lawful selectable tranche, DEC-080 expressly made this
item ordinary agent-executable work, all upstream dependencies are satisfied,
and no contrary risk/evidence record supports deferral. Rejection likewise has
no positive basis in the live tree.

**Classification:** `STANDING_APPROVAL_ELIGIBLE`.

**Agent classification:** `CLASSIFY_ELIGIBLE`.

**Rule activation:** `ACTIVATE_OWNER_STANDING_APPROVAL`; the resulting
conditional adoption is the owner's act under §2, not an agent adoption.

**Owner case selection:** none; no separate case-specific owner choice is
claimed.

**Effect status:** `READY_FOR_DURABLE_LANDING — OPERATIONAL EFFECT HELD UNTIL
K-AUTH-2 / D-GOV-04 BINDING`.
No execution is released by this pre-landing state.

## 6. Workplan Amendment Classification

The overlay changes loop Step 2 only: it adds the standing-approval
eligibility/classification lane after the DEC-082/Shared-Block triage. It does
not change Steps 0, 1, 3, 4, or 5; Step 3 still stops for owner acts. For an
eligible case, the agent only records eligibility and activates the owner's
already-recorded conditional standing approval; adoption remains the owner's
act.

Under the `DEC-083` strict convention, this is the second post-re-mint lineage
entry and touches one of six protocol steps. A one-step in-place surgical
amendment with an inline `*(DEC-085)*` marker is therefore lawful. A re-mint is
not required. The prior owner-directed Step-4 instruction-separation amendment
remains unchanged.

## 7. Independent Review State

### 7.1 Local adversarial verifier

The verifier receives enumeration-derived claims covering every changed
governed artifact and is instructed solely to refute: owner/agent attribution;
root and project nondelegable boundaries; overlay compatibility with
DEC-082/083/084; fast-reject completeness; current-candidate eligibility and
single-outcome result; workplan amendment threshold; write containment; and
the execution hold. The returned verdict is recorded only after the verifier
actually returns it.

**First return:** `BLOCK`, preserved at
`instances/STANDING-APPROVAL-LOCAL-VERIFIER-01/RETURN.md`. It correctly found
that the initial draft's `AgentDisposition=ADOPT` attribution violated the
human-owned governed-brief adoption boundary. That model has been removed.

**Fresh post-rework verifier:** `COMMIT-SAFE`; see
`instances/STANDING-APPROVAL-LOCAL-VERIFIER-02/RETURN.md`. The return confirms
human-owned adoption, classification-only agent authority, current-candidate
eligibility, preserved limits, lawful Step-2 surgery, and the execution hold.

### 7.2 DEC-083 S5 sibling-project review

**Status:** `COMMIT-SAFE — NO CORRECTION REQUIRED`.

The actual owner-mediated sibling review received this packet, the diff, and
these refutation questions:

1. Does the overlay leave Shared-Block v1 byte-identical and avoid silently
   amending app-dev's instrument?
2. Does conditional class-level owner approval plus agent eligibility
   classification/activation preserve the §5.1 human-adoption boundary,
   K-AUTH-1/D-GOV-04 attribution, and D-GOV-04/K-AUTH-2 durable binding?
3. Are any nondelegable decisions missing from §4.1?
4. Does the DEL-09-04 application accidentally perform reproduction
   acceptance, evidence-posture promotion, lifecycle/stage/release action, or
   a case-specific owner attribution?
5. Is the one-step workplan surgery lawful under DEC-083, and does it preserve
   Step 3's owner-act stop?

The actual independent sibling return was relayed by HELP_HUMAN and is recorded
at `AgentRuns/HELP-HUMAN-PIPING-20260718-STANDING-APPROVAL-R2/S5_REVIEW_RETURN.md`.
It found: authority/attribution passes; the fast-reject boundary is complete;
DEL-09-04 has exactly one bounded eligible result and excludes acceptance and
lifecycle effects; the workplan edit is a lawful second Step-2 lineage entry
with Step 3 unchanged; Shared-Block v1 remains byte-identical at 5,108 bytes
and SHA-256
`76438ab0e00dc70e5f6db751a32d0ff07b681c7b7fb12eeda338157c5ebe7668`;
and no execution, receipt, lifecycle, release, prover, merge, or external
effect occurred.

## 8. Activation, Failure, and Supersession

Post-rework local verification and actual S5 fan-in are both `COMMIT-SAFE`.
The record is ready for durable landing without another owner choice: §2 is
already the owner direction establishing the standing approval. Operational
effect remains held until the landing supplies K-AUTH-2/D-GOV-04 binding.

The overlay is project-local and versioned. A future amendment creates a new
decision record; this record and prior near misses are never edited after
landing. Shared-Block v1 changes only through its existing paired-owner-act
rule. No receipt is appended in this pre-landing tranche, and no DEL-09-04
reproduction, product change, lifecycle edit, stage change, release action,
prover action, or evidence write occurs.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
