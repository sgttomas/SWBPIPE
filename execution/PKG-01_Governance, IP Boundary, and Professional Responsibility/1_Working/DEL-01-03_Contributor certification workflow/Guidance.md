# Guidance: DEL-01-03 Contributor certification workflow

## Purpose

This workflow exists to make public contribution intake auditable before data enters the OpenPipeStress public repository. It supports OBJ-002 by separating open mechanics and permissibly redistributable public data from protected standards-body content, proprietary vendor data, private user/project data, and user-supplied code data.

## Principles

1. Evidence first: accept public data only when source, provenance, license or redistribution status, contributor certification, and review disposition are recorded.
2. Stop on protected-content risk: suspected standards text, tables, figures, examples, code-derived formulas, protected dimensional tables, proprietary catalogs, or private project/rule-pack data are blockers, not cleanup tasks.
3. Human authority remains external: maintainers can review repository governance acceptance, but this workflow does not create legal clearance or professional engineering approval.
4. `TBD` is safer than invention: unresolved license, legal, maintainer, release, or redistribution questions stay visible until decided by the appropriate human authority.
5. Local draft only: this setup deliverable describes the future `CONTRIBUTING.md` section and certification template without editing repo-level policy files.

## Considerations

The workflow should be strict enough to prevent accidental public redistribution of protected or private data, but simple enough for contributors to complete before maintainer review. A missing source or license should not be converted into a reviewer guess. A permissive-looking public web page is not automatically a redistribution grant.

Reviewer notes should distinguish factual evidence from assumptions and proposals. When the contribution includes engineering values, reviewer disposition should focus on public-repository acceptability and provenance, not engineering adequacy for project reliance.

## Trade-offs

| Topic | Trade-off | Preferred posture |
|---|---|---|
| Contributor friction | More fields slow contribution intake. | Keep fields minimal but mandatory for public data records. |
| License uncertainty | Fast acceptance may be tempting when data looks useful. | Preserve `TBD` or reject/quarantine until rights are documented. |
| Protected-content detection | Automated checks may miss paraphrased or reformatted protected data. | Use automated gates as evidence aids plus human review. |
| Maintainer authority | Maintainers need a clear merge gate. | Treat maintainer disposition as repository governance only. |

## Examples

### Certification template text, draft

```text
I certify for repository-governance review that this contribution is my original work or is submitted with documented redistribution rights; that I have identified all sources, licenses, and provenance known to me; and that I have not copied protected standards text, tables, figures, examples, proprietary catalog data, private project data, or private rule-pack content into this public contribution except where explicit redistribution rights are documented for maintainer review.
```

This text is a draft workflow artifact, not legal advice and not a final contributor license agreement.

### Review disposition note, draft

```text
Disposition: TBD / accepted / rejected / quarantined
Basis: FACT / ASSUMPTION / PROPOSAL
Evidence reviewed: source_name, source_location, source_license, redistribution_status, protected_content_screen
Limits: Repository governance review only; no engineering approval or legal conclusion.
```

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| C-001 | Exact open-source license and contributor legal mechanism are not selected. | `docs/CONTRACT.md` OPS-K-GOV-1 | `docs/DIRECTIVE.md` section 6 | Specification Standards; Procedure Records | Human project authority decides before repo-level policy issuance. | TBD |
| C-002 | Legal review threshold for accepting public component/material data is unresolved. | SOFTWARE_DECOMP OI-003 | `docs/IP_AND_DATA_BOUNDARY.md` sections 4-5 | Procedure protected-content review and disposition | Treat suspected or uncertain rights as quarantine/reject until human/legal review. | TBD |

## Human Rulings Needed

- Final open-source license and whether a separate contributor license agreement, developer certificate of origin, or project-specific certification text will be used.
- Maintainer/reviewer authority model, quorum, and release-policy linkage.
- Quarantine storage location and access rules for suspected protected/private submissions.
- Whether automated protected-content/provenance lint gates are required before every contribution review or only before release.
