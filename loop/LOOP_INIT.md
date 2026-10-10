# SWBPIPE development entry

SWBPIPE is an analysis-grade piping design engine and stress-model authoring
environment. Work in `projects/chirality-piping`. Start from the affected
`execution/PKG-nn/DEL-nn-nn/`; ScopeOfWork holds commitments, Design holds the
current technical rulings, and `deliverable.yaml` states dependency conditions.
Consult `docs/PRD.md` and its technical companions for the requirements involved.

From the repository root:

```sh
python3 -m tools.deliverables --project projects/chirality-piping neighborhood DEL-nn-nn
```

Use `impact <DEL>` for consumers and `touches <rev|PR:n>` for relevant code
changes. Update current conditions when their meaning changes, not on every run.

## Build and verification

Registered checks are in `software-workflow.json`. From the repository root,
select by project-relative changed paths:

```sh
python3 tools/software_workflow/select_affected_checks.py projects/chirality-piping/software-workflow.json <paths>
```

`docs/CI_STRATEGY.md` describes numerical, persistence, browser and Python
selection. The hosted `harness` result on the exact PR head is the merge check;
DEC-025's blanket local sweep is no longer required. Broad suites run for an
identified need or release preparation. `tools/release/run_evidence_sweep.py`
is optional preparation for macOS release candidates, not a merge gate.

Frozen numerical references are in `validation/references/t3_r1/`. Use the
existing generators and preserve the independent reference basis; do not alter
an oracle to make an implementation pass. Numerical changes require the targeted
checks and independent scrutiny specified in Root `AGENTS.md`.

## Hard boundaries

F-PIP-1 and F-PIP-2 are preserved verbatim from the accepted workplan. The owner
amended F-PIP-3 and F-PIP-4 on 2026-10-10 ("Adopt both F-PIP drafts as written"),
replacing references to the retired register gates and lifecycle states.

- **F-PIP-1 (boundary prohibitions):** local-only operation — no cloud, daemon,
  network, or telemetry features; no repository-default private-data writes;
  user-created models never committed; no protected standards content or private
  project data; invented bundled fixtures only.
- **F-PIP-2 (claims):** no release-readiness, professional approval, certification,
  sealing, authentication, or code-compliance claims. Git closeout is source-control
  hygiene, not lifecycle issuance.
  *Authoring note (DEC-081):* the fence text above is the governing definition and
  is never edited. New artifacts do not restate it as an ad-hoc litany — write
  "Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081)." and use
  `docs/claims_registry.md` for any surface-facing boundary statement.
- **F-PIP-3 (acceptance and issuance):** acceptance, issuance and release of any
  deliverable, package or product baseline are the owner's acts. Agents do not
  record, imply or derive acceptance, issuance or lifecycle standing from completed
  work, checks, reviews or merges. A baseline the owner has issued or protected
  changes only through a change the owner approves.
- **F-PIP-4 (scope-gated integrations):** promoting a live external SDK or harness,
  binding a domain engine, and promoting a version scope each require the owner's
  explicit steer for that scope; ruled decisions that bear on them (for example
  D-22, D-24 and D-30) continue to apply. Tier-0 and domain-engine surfaces
  (`_DomainEngines/**`) belong to their own projects; SWBPIPE work never writes them.

## Knowledge-source constraint (DEC-043)

The owner amended DEC-043 on 2026-10-10 ("Adopt DEC-043 as drafted"); the
original decision remains in `execution/_Decomposition/SOFTWARE_DECOMP.md`.

**DEC-043 (knowledge-source reliability):** in the `domains/piping-design/`
retrieval corpus, prose, concept and design-guidance content is reliable and may
be cited for concepts, terminology and approach. Its extracted equation artifacts
are unreviewed OCR extractions and are not reliable. Any agent, skill (including
`researcher`) or retrieval consumer, including a future embedded design agent:

1. never presents an extracted equation from this corpus as authoritative, and
   shows each artifact's review status (cleared or unverified) from the
   maintainer's per-artifact JSON review, which is the system of record;
2. never uses a corpus equation artifact as a reference for a physics-model build
   (solver, kernel or analytic verification) or to ground an engineering decision.
   Physics and equations come from the maintainer's vetted sources.

This creates no lifecycle, release, professional, certification, sealing,
authentication or code-compliance claim.
