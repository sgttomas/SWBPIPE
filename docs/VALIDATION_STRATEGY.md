---
doc_id: OPS-VALIDATION-STRATEGY
doc_kind: governance.validation_strategy
status: draft
created: 2026-04-30
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: implements
    to: SOW-026
  - rel: implements
    to: SOW-027
---

# Verification and Validation Strategy

## 1. Principle

Verification proves that the software solves the stated mechanics problem correctly within declared tolerances. Validation evaluates whether the software workflow is fit for its intended engineering support role. Neither replaces project-specific professional judgment.

This draft's last full content-alignment baseline remains
`execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7`. Current
decomposition authority is revision `0.11`, and current approved graph
coordination authority is `execution/_DAG/DAG-009/`. Revisions `0.8` through
`0.11` add PRD/decomposition currency, including the revision-`0.10`
actor-neutral clean-checkout reproduction criterion and principal
external-prover validation posture; those changes have not received a full
strategy-content alignment review here, so this paragraph does not claim full
revision-`0.11` alignment. `DAG-009` preserves dependency coordination and does not itself
authorize lifecycle changes, release labels, professional approval, legal
clearance, certification, sealing, authentication, or code-compliance claims (PRD §21.2).

This strategy keeps five evidence surfaces separate:

- mechanics verification: benchmark comparison for open mechanics behavior;
- validation evidence: workflow fitness, reproducibility, warnings, and
  limitation evidence;
- user-rule checks: deterministic computation against user-supplied rule-pack
  inputs and definitions;
- release quality evidence: software maturity inputs for future release gates;
- professional reliance: competent human review outside automatic software
  authority.

## 2. Benchmark families

| Family | Examples | Required evidence |
|---|---|---|
| Units and schemas | dimensional conversions, invalid unit rejection, schema completeness | automated tests |
| Frame mechanics | cantilever, fixed-fixed thermal growth, portal frame, inclined member transforms | hand calculations and automated comparisons |
| Piping loads | weight, pressure thrust where modeled, imposed displacement, thermal expansion | hand calculations and regression outputs |
| Stress recovery | axial, bending, torsion, pressure membrane, stress range | hand calculations |
| Nonlinear supports | one-way support, gap, lift-off, friction active state | convergence and state trace |
| Rule packs | invented demo values, missing required inputs, unit mismatch, unsafe expression | evaluator tests |
| Reports | reproducibility, checksum stability, warning inclusion, protected-content lint | report tests |

Where benchmark evidence relies on a hand calculation, the source of record is
the machine-readable hand-calc witness artifact. Human-facing Markdown, MathML,
PDF, or report excerpts are deterministic renderings from that witness for
inspection and review. Automated comparisons, hashes, provenance review, and
repeatability checks must bind to the machine-readable witness, not to a
manually edited rendering.

The initial mechanics benchmark suite for `DEL-09-01` lives under
`validation/benchmarks/mechanics/` with hand-calculation notes under
`validation/hand_calcs/mechanics/`. Its required fixture families are
cantilever, frame, thermal growth, imposed displacement, and stiffness
transform. The fixtures use original invented values and keep final release
tolerances, CI gates, and professional reliance policy as `TBD` until human
approval.

The initial stress recovery benchmark suite for `DEL-09-02` lives under
`validation/benchmarks/stress/` with hand-calculation notes under
`validation/hand_calcs/stress/`. Its required fixture families are axial
normal stress, bending normal stress, torsional shear stress, pressure membrane
stress, and mechanics-only stress range. The fixtures use original invented
values and keep final release tolerances, CI gates, fatigue/allowable
criteria, and professional reliance policy as `TBD` until human approval.

## 3. Validation manual structure

The validation manual skeleton lives at `docs/validation_manual/index.md`. It
is a draft evidence-organization surface for maintainers and reviewers; it is
not an engineering seal, code-compliance ruling, or project-specific reliance
record.

The validation manual shall include these sections:

1. product scope and limitations;
2. solver theory summary;
3. unit and schema verification;
4. element verification;
5. load and stress recovery verification;
6. nonlinear support verification;
7. rule-pack evaluator verification;
8. GUI workflow validation;
9. report reproducibility validation;
10. known limitations and open issues.

Each section must state whether it is verification evidence, workflow
validation evidence, user-rule-check evidence, or professional-reliance context.
Missing evidence, unapproved thresholds, and unresolved source/provenance
questions remain explicit `TBD` items until a governed decision or evidence
record exists.

## 4. Release gate

Release quality evidence is distinct from validation evidence and professional
reliance. Until the release-label vocabulary and final release policy are
settled by the human project authority, validation evidence may only be used as
input to a future release-quality review.

A future release-quality review should record, at minimum:

- mechanics benchmark status for affected solver behavior;
- stress-recovery benchmark status for affected stress/result behavior;
- nonlinear support regression status where nonlinear behavior is affected;
- rule-pack missing-input, unit, provenance, and sandbox test status where rule
  behavior is affected;
- report reproducibility, checksum, warning, and protected-content lint status
  where reports, public examples, or templates are affected;
- known limitations, unresolved risks, and human-maintainer dispositions.

The final benchmark tolerance policy, public benchmark source acceptance
process, release-label policy, GUI validation evidence requirements, and
validation evidence-bundle storage format remain `TBD`.

## 5. Benchmark source rule

Validation examples must be original, public-domain, or permissively licensed.
Protected code examples and commercial software examples are not acceptable
unless written permission is documented.

Each public mechanics benchmark fixture must record source/provenance,
redistribution status, contributor or generator note, assumptions, units,
expected result fields, and review disposition before it is treated as accepted
public validation evidence.

Each public hand-calc witness must be original, public-domain, or permissively
licensed; record units, assumptions, symbolic and numeric steps, expected
results, tolerance basis or `TBD`, provenance, redistribution status, and
review disposition; and generate any human rendering deterministically from the
machine-readable witness artifact.

Each public stress recovery benchmark fixture must follow the same provenance
rule and must remain limited to open mechanics component comparisons. Stress
range fixtures are mechanics differences between result states; they are not
fatigue checks, allowable comparisons, code compliance checks, or professional
approval records.

The public benchmark source acceptance workflow remains `TBD`. Until it is
settled, source/provenance records are review evidence only and must not be
treated as legal clearance or final public benchmark acceptance.

## 6. Evidence-bundle storage

Validation evidence bundles must preserve source artifacts, commands, generated
outputs, hashes or checksums where applicable, provenance, review disposition,
known limitations, and unresolved `TBD` items. The long-term storage location,
package format, retention policy, and release attachment workflow for reviewed
validation evidence bundles remain `TBD`.

GUI validation evidence is also `TBD` until GUI workflow deliverables provide
the relevant surfaces. Future GUI evidence should preserve missing-data
visibility, warning presentation, analysis-status transitions, accessibility or
usability findings where applicable, and screenshots or automated interaction
records without making professional-reliance claims.
