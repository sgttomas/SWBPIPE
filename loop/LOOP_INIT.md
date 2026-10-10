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

The following fence text is preserved verbatim from the accepted workplan.
The reset retires administrative lifecycle tracking; owner acceptance, issuance
and release gates and protected baselines remain reserved.

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
- **F-PIP-3 (lifecycle):** deliverable lifecycle transitions follow the register's
  ruled gates; no `CHECKING -> ISSUED` issuance without the owner's gate; the
  currently `ISSUED` baseline is opened only through a human-approved change path.
- **F-PIP-4 (scope-gated integrations):** live external SDK/harness promotion,
  domain-engine bindings, and version-scope promotions stay behind their named
  register rows; tier-0/domain-engine surfaces (`_DomainEngines/**`) belong to their
  own loops — this loop never writes them.

## Knowledge-source constraint (DEC-043)

The accepted decision below is preserved verbatim from SOFTWARE_DECOMP.md.

| Decision | Constraint | Basis | Standing |
|---|---|---|---|
|DEC-043|Knowledge-source reliability constraint for the external engineering corpus `domains/piping-design/` (BM25 + dense retrieval index built 2026-06-18, ~48.2k chunks): its prose / concept / design-guidance content is vetted and reliable, but its extracted **equation artifacts are NOT reliable** — they are unreviewed `pdf2md`/OCR extractions pending the maintainer's manual equation review, which writes a machine-readable per-artifact JSON review status. Therefore: (1) `RESEARCH`/`RESEARCHER` and any retrieval consumer (including a future embedded design agent) may cite piping-design for concepts / terminology / approach but must never present an extracted equation from this corpus as authoritative, and must surface each artifact's review status (cleared vs unverified); (2) piping-design equation artifacts must NOT be used as references for any physics-model build (solver / kernel / analytic-verification), including grounding the Phase-D engineering decisions D-16 / D-18 / D-19 — physics and equations come from the maintainer's vetted sources, not the corpus extractions. Routed into `AGENTS.md` (Knowledge-source reliability section) so agents honor it at runtime.|Human project authority directive on 2026-06-18 during the embedded-agent design session, recorded now that the piping-design retrieval index exists (DEC-042 prep) and `RESEARCHER` could otherwise surface the unreviewed equation artifacts as authoritative.|Accepted; recorded as a standing knowledge-source reliability constraint (no register D-row — not a gated decision packet); routed to `AGENTS.md`; binds `RESEARCH`/`RESEARCHER` and any retrieval consumer; the maintainer's JSON equation-review status is the system of record for per-artifact clearance; no lifecycle, release, professional, certification, sealing, authentication, or code-compliance claim is created.|
