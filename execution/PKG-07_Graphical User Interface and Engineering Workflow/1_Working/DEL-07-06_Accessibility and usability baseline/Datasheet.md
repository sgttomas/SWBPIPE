# Datasheet: DEL-07-06 Accessibility and usability baseline

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-07-06 |
| Deliverable name | Accessibility and usability baseline |
| Package ID | PKG-07 |
| Package name | Graphical User Interface and Engineering Workflow |
| Type | UX_UI_SLICE |
| Scope coverage | SOW-036 |
| Objective support | OBJ-006 |
| Context envelope | M |
| Setup state | Draft setup artifact for later implementation review |

## Attributes

| Attribute | Baseline value |
|---|---|
| Work surface | GUI workflow and report-facing review surfaces, limited to setup documentation in this run |
| Baseline topics | Keyboard navigation, labels/tooltips, contrast/readability, search/filter, copy/export, undo/redo discoverability, inline validation, warning separation |
| Engineering review focus | Model creation, missing data, results, assumptions, diagnostics, provenance, and human-review boundaries remain visible |
| Architecture basis | Tauri 2 desktop shell, TypeScript/React/Vite GUI, Three.js where viewport-facing, schema-first command/query/job/result envelopes |
| Accessibility conformance target | TBD by human ruling; this setup artifact does not assert a WCAG level |
| Report-facing accessibility target | TBD separately for report preview/export and generated report artifacts |
| Implementation status | No GUI source, tests, schemas, manifests, or report templates changed in this deliverable |

## Conditions

| Condition | Handling |
|---|---|
| Missing solve-required data | Must remain visible as a `SOLVE_BLOCKING` diagnostic rather than being silently defaulted |
| Missing rule-check data | Must remain visible as a `RULE_CHECK_BLOCKING` diagnostic distinct from solve blocking |
| Weak or missing provenance | Must remain visible as provenance or assumption review information where engineering reliance may be affected |
| Result or report review | Must preserve units, warnings, assumptions, provenance, and professional-boundary notices |
| Accessibility evidence artifacts | Screenshots, fixtures, and exported examples must pass protected-content and private-data review before public use |
| Protected standards or private project data | Must not be introduced into public examples, screenshots, tests, reports, or setup artifacts |
| Compliance or certification language | Must not be claimed by the GUI, report, software, or agent output |

## Construction

This setup deliverable defines the future baseline checklist and verification hooks for accessibility and engineering-review usability. It does not select component libraries, implement UI behavior, create automated tests, choose final WCAG conformance, or change report templates.

Future implementation should translate the checklist into GUI and report tests only after the relevant GUI framework choices and human accessibility target are accepted. Any implementation work remains outside this setup session.

## References

| Source | Relevant source slice |
|---|---|
| `_CONTEXT.md` | DEL-07-06 identity, SOW-036, OBJ-006, architecture basis injection |
| `docs/PRD.md` | Sections 6.5, 7, 8, 14, 15, 20, and 21 |
| `docs/SPEC.md` | Sections 1, 7, and 8 |
| `docs/CONTRACT.md` | OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-AUTH-1, OPS-K-IP-1/2/3, OPS-K-PRIV-1/2, OPS-K-AGENT-1..4 |
| `docs/TYPES.md` | Analysis-status vocabulary, provenance labels, report/result boundary |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and report boundary |
| `docs/VALIDATION_STRATEGY.md` | GUI workflow validation and report reproducibility validation |
| `docs/_Decomposition/SOFTWARE_DECOMP.md` | PKG-07, DEL-07-06, AB-00-03, AB-00-05, AB-00-06, AB-00-08, OI-002 |
| `docs/_Registers/Deliverables.csv` | DEL-07-06 register row |
| `docs/_Registers/ScopeLedger.csv` | SOW-036 row |
