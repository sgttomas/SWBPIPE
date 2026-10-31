# SCA-015 — Hanger sizing from the user's private catalogue

Status: **DECIDED**, executed in place. The scope is the owner's decision of 2026-10-28,
and Agent 0 (HELP_HUMAN) placed it on 2026-10-31. The changed commitment text is pending
the independent review that `loop/LOOP_INIT.md` requires.

## Authority

**The owner's decision** (owner, 2026-10-28) chose option (a) of open choice OC-4, as
drafted:

> "Must: size hangers from the user's private catalogue (invented example records only) and produce a hanger table."

The release target records it as RT-A11 (`docs/RELEASE_TARGET_v1.0.md` §1 and §2) and in
exit criterion 1's analyses. On the same date, DEC-049's "no catalog sizing" was amended
in place to admit sizing from the user's private catalogue, with no catalogue data
shipped (`execution/_Decomposition/SOFTWARE_DECOMP.md`).

**Agent 0's decisions** (2026-10-31), under the owner's delegation of 2026-10-27
(`loop/LOOP_INIT.md`), on the lane implementation manager's proposal:
- **D-1, placement:** a new PKG-04 Deliverable, DEL-04-11 "Hanger sizing", following
  DEL-04-09 and DEL-04-10.
- **D-2, library format:** hanger library format 1.1.0 adds an optional per-record
  `working_load_range`, which sizing requires; 1.0.0 documents still import.
- **D-3, spring preload:** a variable spring is modelled as its stiffness plus its cold
  (preset) load, which settles RT-S3's open question. It is built after the sizing
  engine.
- **D-4, method:** the method outlined (Design §1) is adopted as the basis for the
  design text.

## The amendment

| Surface | Before | After |
|---|---|---|
| `docs/PRD.md` §12.4 | Support types only | Adds the hanger-sizing requirement |
| `docs/PRD.md` header | Amendments through 2026-10-28 | Adds 2026-10-31 (§12.4, hanger sizing) and names SCA-015 |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | Revision 0.17 | Revision 0.18 adds SOW-083 and DEL-04-11 |
| `execution/PKG-04/DEL-04-11/` | — | ScopeOfWork, governing design `Work/hanger-sizing.md`, declarations |
| `execution/PKG-03/DEL-03-02/ScopeOfWork.md` CLM-030 | Hanger schema contract | Adds format 1.1.0 and its `working_load_range` |
| `execution/PKG-04/DEL-04-03/Work/pf-nonlinear-imposed-motion.md` | Preload limitation, to be lifted by RT-S3 and RT-A11 | Records D-3 and its reason; the limitation stands until DEL-04-11's preload increment |
| `docs/RELEASE_TARGET_v1.0.md` RT-A11 | Owner "new" | Owner DEL-04-11 |
| `docs/COMPLETENESS_MAP.md` | No row | A §12.4 hanger-sizing row, not built |
| `execution/_ScopeChange/_LATEST.md` | Selects SCA-014 | Selects SCA-015 |

## What enters scope, and what does not

**In scope:** sizing variable spring hangers and constant-effort supports from the
user's own imported hanger catalogue:
- a hanger table of sized loads, travels and selected records, reported as results with
  their basis;
- the person's explicit application of a selection through the existing operation route;
- the hanger library format 1.1.0;
- the variable spring's cold-load preload in the solve.

Every method choice is a required user input.

**Not in this amendment:**
- any catalogue, load-variation limit, travel allowance or selection rule shipped with
  the product;
- automatic re-sizing iteration;
- hangers on skewed or rotational DOFs;
- any judgment of a hanger's suitability or code compliance.

**Unchanged:** every other scope item, Deliverable mapping, dependency meaning and
lifecycle state. No lifecycle, release-readiness, professional, certification or
code-compliance claim follows.

## Method

As with SCA-012 to SCA-014, the `scope-change` workflow's grouped checkpoints are not
used. The change is one decided scope statement and its governed text, edited in place
under the repository's current record rule (AGENTS.md: a record needs a concrete
consumer; decisions edit the text they govern). `_ScopeChange/_LATEST.md` now selects
this amendment.

Standard claim fence applies (claims taxonomy per DEC-081).
