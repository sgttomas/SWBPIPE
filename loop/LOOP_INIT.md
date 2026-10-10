# Piping development entry

Work in `projects/chirality-piping` relative to the repository root. Read Root
`AGENTS.md`, the assigned role, `docs/PRD.md` and the affected deliverables under
`execution/PKG-*/1_Working/DEL-*/`. ScopeOfWork and Design hold commitments;
`Dependencies.csv` holds needs until the dependency migration. Follow only the
relevant neighbourhood. Code and checks are located through
`software-workflow.json`; use `coordinated-knowledge-work` for coordination.
The current objective comes from the init prompt and subsequent owner steering.
Edit current source documents when decisions or dependency conditions change;
the PR is the change record. No routine progress or lifecycle files are updated.
Hosted checks cover the exact merge head: selected desktop build, behavioural
tests and core browser journey; selected Cargo numerical oracles and Python
product contracts. Broad browser matrices run nightly or on demand. Once the
replacement required checks are applied and demonstrated, this rule supersedes
DEC-025's blanket local merge sweep. `tools/release/run_evidence_sweep.py` remains
optional preparation for macOS release candidates; release is an owner act.

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
