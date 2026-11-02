---
doc_id: OPS-README
doc_kind: governance.index
status: current_index
created: 2026-04-30
---

# SWBPIPE

SWBPIPE is a code-neutral piping design engine and stress-model authoring
workbench. It computes open, auditable piping mechanics. The responsible
engineer supplies the governing data and decides whether to rely on the results.

Use the desktop to author piping models, organize loads, inspect mechanics
results and diagnostics, and prepare reports and supported downstream handoffs.
Available analyses and export routes have explicit scope and limitations.
SWBPIPE does not establish code compliance or professional approval.

> **Open the mechanics; protect the standards; empower the engineer.**

## Development status

SWBPIPE is under development toward its first publication-ready **v1.0**.
The release baseline is **macOS on Apple Silicon**; Windows development does
not yet establish a qualified Windows release. Source builds and passing CI
are development evidence, not a released or accepted engineering product.

The [v1.0 release target](docs/RELEASE_TARGET_v1.0.md) defines the required
capabilities and exit criteria; it is not a list of features already delivered.
The [release limitations](docs/RELEASE_LIMITATIONS_v1.0.md) distinguish recorded
limitations from those accepted by the owner. Package version strings and older
0.1.0 documentation do not change the v1.0 publication target.

## Start here

- [User guide](docs/user_guide/index.md): model workflows, result interpretation and documented limitations; check each section's stated implementation basis.
- [Build and release guide](docs/BUILD_AND_RELEASE.md): source-build prerequisites, verification and packaging procedures.
- [Validation strategy](docs/VALIDATION_STRATEGY.md) and [theory](docs/theory/centerline_analysis.md): calculation basis and verification approach.
- [Documentation index](docs/README.md): product contracts, developer guides and validation references.

Product commitments are in the [PRD](docs/PRD.md), its companions and the
current deliverable contracts. Source development and passing checks do not
establish an engineering approval or product release.

## Layout

| Path | Contents |
|---|---|
| [core/](core/) | Engineering implementation in Python and Rust |
| [apps/](apps/) | Application and authoring interfaces |
| [api/](api/), [schemas/](schemas/) | Application boundaries and structured formats |
| [tests/](tests/), [fixtures/](fixtures/) | Maintained tests and invented fixtures |
| [validation/](validation/) | Numerical references and qualification support |
| [tools/](tools/) | Build, checking and release-preparation tools |
| [docs/](docs/) | Product basis, guides, theory and validation strategy |
| [execution/](execution/) | ScopeOfWork, Work and dependency conditions |

## Contributing

Read the [repository instructions](AGENTS.md) and
[development loop](loop/LOOP_INIT.md), then locate the affected
`execution/PKG-nn/DEL-nn-nn/` folder. `ScopeOfWork.md` states commitments and
acceptance criteria, `Work/` holds technical detail, and `deliverable.yaml`
states dependency conditions and code/check pointers.

From the SWBPIPE repository root, query the relevant inputs and consumers:

```sh
python3 -m tools.deliverables --project . neighborhood DEL-nn-nn
```

Replace `DEL-nn-nn` with the affected deliverable ID. The
[deliverables tool guide](tools/deliverables/README.md) covers impact,
changed-path and dependency queries. Registered implementation checks are in
[software-workflow.json](software-workflow.json); the loop and
[CI strategy](docs/CI_STRATEGY.md) explain how to select them.

Earlier decomposition registers, lifecycle procedures and agent-run records
remain recoverable through Git and archive tags. Use current deliverable
contracts for forward work.

## License and professional boundaries

See the [MIT License](LICENSE.md), [IP and data boundary](docs/IP_AND_DATA_BOUNDARY.md),
[professional boundary](docs/PROFESSIONAL_BOUNDARY.md) and
[validation strategy](docs/VALIDATION_STRATEGY.md). Product claims retain their
stated scope and qualification limits.
