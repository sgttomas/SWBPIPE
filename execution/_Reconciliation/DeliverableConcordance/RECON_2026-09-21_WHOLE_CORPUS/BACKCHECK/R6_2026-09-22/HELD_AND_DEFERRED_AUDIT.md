# Held, deferred and unchanged populations

`CLAIM_DISPOSITIONS.csv` is the complete 9,889-key accounting, preserving the
original disposition, class, route, packet, source subject and repair proposal.
`H4_ACCOUNTING.csv` accounts for all 2,234 proposed tranche rows, including
52 non-claim items; `CAPABILITY_ACCOUNTING.csv` covers all 598 capabilities.
The original `#END` sentinels are not data rows. These views have different
denominators and must not be summed as if they were independent claims.

DIRECT_REPAIRED describes the identified wording/record defect. It does not
assert whole-deliverable completion. PARTIAL_REPAIR preserves mixed statements
whose untouched portion still needs a decision, implementation or review.
CHANGED_BINDING_RESIDUAL_RETAINED marks a changed enclosing source hash without
semantic closure of the child finding. REPAIRED_RECORD_RESIDUAL_PRESERVED marks
a corrected Remaining premise whose actual open action or authority remains.

HELD_UNCHANGED means no repair was executed for that exact source key in this
bounded tranche. It is not an invented new owner hold. The original holder and
proposal remain visible; the per-row subject and class-specific reason explain
what further evidence or semantic work is needed. Where prior decisions already
settle intent, their application is recorded in the packet analysis without
claiming a missing carrier or implementation has been repaired.

Substantial residual classes are deliberately heterogeneous:

- T5A setup/current-state prose can contain durable requirements, exclusions
  and partial implementation scope. It cannot be globally deleted merely
  because code is now present.
- T5B Still-TBD carriers mix resolved local choices with public interfaces,
  privacy/legal/reviewer authority, engineering tolerances and unselected
  runtime subjects. DEC-012's sealed-brief route narrows this population, but
  does not collectively discharge it.
- T4B four-document residue includes actual operative references, migration
  provenance and old checker/results. Current references were repaired where
  their meaning was established; historical checker output cannot be rewritten
  to pretend it evaluated SoW.
- Formal content/validation and protected-content reviews retain their own
  scope, source state and holder. Hash changes neither erase historical PASS
  nor manufacture present approval.
- Scope/ownership and implementation residuals keep their H1/H2/B/C routes.
  Existing DEC-044 and annex ownership decisions are applied as follow-through;
  proposed new ownership remains a concrete scope-change undertaking.

The issued DEL-01-01 population is unchanged, not silently absorbed by record
normalization. Engine/runtime, canonical hashing, units, schema conformance,
diagnostics, protected tolerances, private data, independent validation,
external activation and professional acceptance retain their actual governing
constraints. No code, schema, test, fixture, protected criterion, lifecycle or
public-release action executes from a row marked held/deferred here.

The supplemental required-pin and snapshot/review-status tranche is accounted
separately from the initial reviewed source. Its hashes and delta review must
cover the final candidate; the initial review alone cannot cover it.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
