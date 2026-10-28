# SCA-014 — Response-spectrum seismic analysis

Status: **DECIDED** by the owner on 2026-10-28 and executed in place. Its
changed commitment text is pending the independent review that
`loop/LOOP_INIT.md` requires.

The change re-enters, for **response-spectrum seismic analysis only**, the v0.1
FR-024 dynamics scope that DEC-078 (D-12, owner 2026-07-15) deferred. Snubbers
come with it, as dynamic-only restraints, which lifts PRD §7 non-goal 15 for
that use. DEC-078 required that "any future dynamics scope re-enters only
through a new governed scope change with its own human decision packet"; this
is that change, as SCA-013 was for modal analysis.

## Authority

The owner's rule (owner, 2026-10-28, `loop/LOOP_INIT.md`):

> "I said you should only develop a response spectrum capability for seismic analysis if that's a functionality that CAEPIPE has, otherwise you can go with only static for now."

**The condition is met.** The vendor's public manual describes
response-spectrum analysis with mode and directional combination and a
missing-mass correction. This cites what the public manual documents; it evaluates nothing:
- <https://docs.sstcae.com/tech_manual/dynamic_analysis.htm>
- <https://docs.sstcae.com/users_manual/spectrums.htm>
- <https://docs.sstcae.com/users_manual/load_cases.htm>

**The owner's decision** (owner, 2026-10-28) chose option (a) of open choice
OC-3, as drafted:

> "(a) Must: user-entered spectra, modal and directional combination, a missing-mass correction, and snubbers as a dynamic-only restraint. This reverses DEC-078 and the last paragraph of PRD §12.9 for response spectrum only, and lifts PRD §7 item 15."

**HELP_HUMAN's adjudication** (2026-10-28, under the owner's delegation of
2026-10-27): uniform support motion is Must for v1.0, and independent
(multi-level) support motion is Should (`docs/RELEASE_TARGET_v1.0.md` RT-A8).

## The amendment

| Surface | Before | After |
|---|---|---|
| `docs/PRD.md` §7 item 15 | Model snubbers. | Snubbers, except as dynamic-only restraints in response-spectrum analysis (§12.9) |
| `docs/PRD.md` §12.9 last paragraph | Response-spectrum, harmonic and time-history analysis are not required. | Response-spectrum seismic analysis on the modal basis is required for v1.0; harmonic and time-history analysis are not required |
| `docs/PRD.md` §23, MVP exclusions | response-spectrum and time-history modules are not prioritized | response spectrum prioritized by the v1.0 target; time history not |
| `docs/PRD.md` header and §2 amendment note | — | amended 2026-10-28; names SCA-014 |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | DEC-078 keeps response spectrum deferred; OI-009 lists it; SOW-081 note says it stays deferred | DEC-078 amended in place to record the re-entry; OI-009 narrowed to the modules still deferred; SOW-081 note updated |
| `execution/PKG-04/DEL-04-09/ScopeOfWork.md` AX-006 | Response spectrum stays deferred under DEC-078 | Response spectrum re-entered by SCA-014 and not yet assigned to a deliverable |
| `docs/COMPLETENESS_MAP.md` FR-SOL-016 row | response spectrum stays deferred | response spectrum re-entered by SCA-014; unassigned, not built |
| `execution/_ScopeChange/_LATEST.md` | selects SCA-013 | selects SCA-014 |

The amendment adds no scope item or Deliverable. Both are added by a later
decomposition revision when the numerical lane scopes the build: either a new
PKG-04 Deliverable or an extension of DEL-04-09, which is HELP_HUMAN's choice
under the delegation. Until then RT-A8 records "new" as its owner.
`execution/_Coordination/_DECISIONS/D-12_fr024_fr025_disposition.md` is
historical and unchanged.

## What enters scope, and what does not

**In scope:** seismic response-spectrum analysis of the linear model on DEL-04-09's modal basis:
- user-entered spectra only, with no spectrum library shipped;
- modal combination and directional combination;
- a missing-mass correction;
- snubbers as dynamic-only restraints.

The methods, their benchmarks and the treatment of nonlinear supports are set
when the build is scoped.

**Not in this amendment:** time history (survey F3), harmonic analysis, force
spectrum and every other dynamic module. They stay deferred under DEC-078.

**Unchanged:** every other scope item, Deliverable mapping, dependency meaning
and lifecycle state. No lifecycle, release-readiness, professional,
certification or code-compliance claim follows.

## Method

The `scope-change` workflow's grouped checkpoints are not used, as with SCA-012
and SCA-013. The change is one decided scope statement and its governed text,
edited in place under the repository's current record rule (AGENTS.md: a record
needs a concrete consumer; decisions edit the text they govern).
`_ScopeChange/_LATEST.md` now selects this amendment.

Standard claim fence applies (claims taxonomy per DEC-081).
