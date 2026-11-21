---
name: swbpipe-overview
description: Introduce SWBPIPE, its development status and documentation.
metadata:
  kind: explanation
  surface: reference
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
The [release limitations](docs/RELEASE_LIMITATIONS_v1.0.md) record each
remaining refusal and whether Agent 0 has declared it acceptable. Package version strings and older
0.1.0 documentation do not change the v1.0 publication target.

**Known numerical defect on development main:** the solver's integrity check
rests on an inverse-norm bound the product does not verify, so no error bound
and no nonlinear support state is established for any solve. The desktop and
the result reader's verdict state this as not verified and not confirmed; a
result document's own tokens, and the file outputs, do not yet carry that
status. Exit criterion 18 is not met;
[LIM-026](docs/RELEASE_LIMITATIONS_v1.0.md#lim-026--swbpipe-verifies-no-error-bound-for-a-solve-every-results-integrity-check-reads-not-verified)
names the maintained false-pass and false-state witness and what remains. The
solver's tokens are not independent assurance.

A separate [structured quantity-edit defect, LIM-027](docs/RELEASE_LIMITATIONS_v1.0.md#lim-027--a-bare-structured-quantity-edit-can-store-its-number-beside-the-old-unit),
can store a bare entered number beside the old unit and publish a result for
that unintended physical input. The entry identifies the executed witness and
which authoring paths it does and does not establish.

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

Read the [repository instructions](AGENTS.md), then locate the affected
`execution/PKG-nn/DEL-nn-nn/` folder. `ScopeOfWork.md` states commitments and
acceptance criteria, and `deliverable.yaml` states dependency conditions and
code/check pointers.

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

See the [MIT License](LICENSE), [IP and data boundary](docs/IP_AND_DATA_BOUNDARY.md)
and [validation strategy](docs/VALIDATION_STRATEGY.md). Product claims retain their
stated scope and qualification limits.
