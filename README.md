---
doc_id: OPS-README
doc_kind: governance.index
status: current_index
created: 2026-04-30
---

# SWBPIPE

SWBPIPE is a code-neutral piping design engine and stress-model authoring
workbench. It computes open, auditable piping mechanics. The responsible
engineer supplies the governing data and accepts engineering reliance.

> **Open the mechanics; protect the standards; empower the engineer.**

## Start here

- [Development entry](loop/LOOP_INIT.md): current work, project fences and check selection.
- [Documentation index](docs/README.md): product contracts, user/developer guides and validation references.
- [Build and release instructions](docs/BUILD_AND_RELEASE.md): source setup, checks and packaging.
- [Root agent instructions](../../AGENTS.md): shared repository roles and working conventions.

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
| [execution/](execution/) | ScopeOfWork, Design and dependency conditions |

## Work on a deliverable

Enter through the [development loop](loop/LOOP_INIT.md) and locate the affected
`execution/PKG-nn/DEL-nn-nn/` folder. `ScopeOfWork.md` states commitments and
acceptance criteria, `Work/` holds technical detail, and `deliverable.yaml`
states dependency conditions and code/check pointers.

From the Chirality repository root, query the relevant inputs and consumers:

```sh
python3 -m tools.deliverables --project projects/chirality-piping neighborhood DEL-nn-nn
```

Replace `DEL-nn-nn` with the affected deliverable ID. The
[deliverables tool guide](../../tools/deliverables/README.md) covers impact,
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
