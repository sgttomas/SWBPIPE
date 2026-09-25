# Practical target acquisition contract

The chosen source snapshot is [Code_Aster commit7558f1d06088bd93d8324e314912bf0e6df7801f](https://gitlab.com/codeaster/src/-/commit/7558f1d06088bd93d8324e314912bf0e6df7801f), observed through a direct public API response dated2026-09-23. It is a source commit, not an installed/qualified Code_Aster release. Every inspected raw source URL in ACQUISITION.json includes that commit. Do not execute a future installed release against this snapshot and label the result as the source commit without recording compatibility/build evidence.

Manuals are the [v17 stable tree](https://code-aster.org/doc/v17/) and [v18 development tree](https://codeaster.gitlab.io/doc/docaster/), as labelled by the acquired switcher. Six corresponding article bodies were compared; equality does not supply the hidden documentation-build SHA. Retain URL, version label, retrieval time, HTML hash, section and any original figure used in a target mapping. If a manual is updated, add a new source revision rather than replace the existing reference silently.

## Declared file inputs available for private inspection

| Variant | Export's external file inputs, in addition to its `.comm` | Availability and meaning |
|---|---|---|
| SSLL106A/B/C/D | `ssll106a.mmed`, logical unit20 | Retrieved binary HDF5/MED bytes, not a Git LFS pointer. Group/topology decode remains to be checked. |
| SSLL106E | `ssll106e.mail`, unit20; `ssll106e.datg`, unit16 | Retrieved ASCII mesh and legacy mesh-generation input. The inspected command reads the mesh and converts SEG3→SEG4; no external mesher was run here. Keep the export's declared auxiliary file even if current command inspection does not consume it. |
| SSLL106F | `ssll106f.mail`, unit20 | Retrieved single-SEG4 ASCII mesh. This is a different discretization from E. |
| SSLL101A | `ssll101a.mmed`, unit20 | Retrieved binary HDF5/MED bytes. Need decoded groups, coordinates and connectivity before equivalent-model construction. |
| SSLL101C | `ssll101c.msup`, unit19; second command`ssll101c.com1` | Retrieved IDEAS text mesh plus continuation command. The continuation is an issue-regression field/case check; it is not another physical Hovgaard oracle. |
| SSLL101D | `ssll101c.msup`, unit19 | Same declared source mesh as C, different Fourier formulation. |
| SSLL101E | `ssll101e.mmed`, unit20 | Retrieved HDF5/MED input; command converts/supports its SEG4 model and adds an elastic alternate-solver thermal comparison. |

All ten `.export` and `.comm` files and all their declared `F` dependencies were acquired; per-file hashes, sizes, units and formats are recorded in [ACQUISITION.json](ACQUISITION.json). Static read/include scanning found no additional external data-file dependency beyond those declared and the actual Code_Aster runtime. This is **declared data-file closure**, not proof that a runtime/environment is installed, runnable, compatible or licensed for the planned distribution. MED group interpretation is not established merely by an HDF5 signature. No `INCLUDE`, command deck or mesh generator was executed.

`ssll101b.comm`/`.export` returned404 at the pin; current manual navigation names A/C/D/E. Treat B as unavailable, not as a missing mandatory file of one of the four acquired variants. Attempts to locate a public documentation Git project under two guessed API namespaces also returned404; no exact doc source commit was recovered. Web-tool cache misses for some manual pages were resolved by direct HTTP into the private folder. A cached source-main API result was stale relative to direct access; subsequent source/mesh retrieval used only the direct commit pin.

## Assertion-level admission record

Before a candidate becomes a scoring target, its implementing TASK must produce one small record containing:

1. Actual source commit/file/line or manual-version/section/figure plus byte hash; exact selected case/model and all dependency hashes. Record manual-versus-command choices and unresolved contradictions. Do not silently pick200 for the thermal mismatch or repair the pressure expression.
2. Problem inputs and reference configuration: geometry/topology, exact effective section properties and their purpose, E/nu/G relation, constant-alpha definition and explicit temperature/reference datum, physical load sources, supports and enriched constraints. Distinguish the water-filled Hovgaard effective-density fixture from a separately applied fluid-weight source.
3. Assertion lineage: analytical expression, published external numerical value, same-engine comparison (`AUTRE_ASTER`), regression value (`VALE_CALC`), or nonphysical field/regression assertion. Preserve both reference and regression values when a source carries both. Source label alone is not independent verification. [ASSERTION_LINEAGE.json](ASSERTION_LINEAGE.json) is a locator/count aid, not an accepted oracle table.
4. Exact output mapping: entity/node/group, point/subpoint, frame, direction/cut sign, unit/dimension, strain convention and pressure wall/effective meaning. Retain unsupported outputs as unsupported. A centerline beam result cannot stand in for radial/ovalization DOFs or a through-thickness point.
5. Independent expected-value check using matched assumptions and adequate arithmetic, without importing the production implementation as oracle. Keep the source's published precision/uncertainty separate from the project comparison/error budget. No2% or upstream1% field becomes a replacement for protected1e-9 checks.
6. If an external executable is later used: actual software release/build, source/runtime compatibility, settings, complete inputs, outputs, warnings and execution provenance. A published table-only comparison stays labelled that way. Runtime acquisition/execution remains separately authorized programme work; none occurred here.

A claimed partial subset has an explicit finite required denominator. Required but missing assertions cannot disappear into a success rate. The whole external case remains incomplete while excluded loads, dynamic checks, enriched fields, source conflicts or independent reference checks remain unresolved. This does not block a separately defined project-original straight-static profile.

## Rights and retention

Inspected `.comm`/`.com1` and assertion-catalog files explicitly carry GPL-3.0-or-later notices; the pinned root LICENSE contains GPLv3 text. Their availability is useful, but this packet makes no general license conclusion for every mesh, figure, PDF or generated document. Rendered documentation displays EDF copyright. Public readability is not a blanket redistribution permission. Record each asset's actual terms before vendoring, packaging or publishing it; none was copied into source by this assignment.

Private retrieval is for inspection only. Tracked output contains this original analysis, metadata/hashes and bounded source locators, not the external decks, meshes or manual copies. Benchmark-specific data may be independently authored into a later authorized fixture with correct provenance and rights handling; it must not become an operational materials/component library or code-rule pack. Original Piping reference work stays at its owning records and is not replaced by these sources.
