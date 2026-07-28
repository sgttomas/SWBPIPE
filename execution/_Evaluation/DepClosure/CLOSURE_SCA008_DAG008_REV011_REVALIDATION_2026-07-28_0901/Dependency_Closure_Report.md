# SCA-008 / DAG-008 dependency-closure report

## Finding

SCA-008 and decomposition revision 0.11 are structurally compatible with
accepted DAG-008. DAG-008 remains current by revalidation.

## Basis and identity

- SCA-008 accepted basis:
  `7b0be4d8772a16e5a4774a17988479587d00acca`
- SCA-008 application commit:
  `9b52076701c218f69255afbedcfc52025bd47fa3`
- PR #390 merge commit:
  `380ea2a794588075b83fe8cc0108ab7ce74b6b33`
- examined current basis:
  `404e47c16a88e7ffdc6d1fc5fac61ebb6864211e`
- DAG-008 manifest SHA-256:
  `5cab5b1f4f5be1dd0ac06737e5901c5dda844d0af412f3dc5427fef4d9cb6b60`
- DependencyEdges SHA-256:
  `dde5d2d0601290d085a30833bccb582ff6518ba7dcdd9adebe6e03ba49719005`
- DeliverableNodes SHA-256:
  `6e5050c4e578f6ff9819ee7a11dbb395b3f0a163b4fb0c48e88c3d084d9b0732`
- dag.json SHA-256:
  `f657808890e6e7b663b6ad1be4afe1d63f64b1d8c8fbff156d960132c145e645`
- DAG pointer SHA-256:
  `46c162ddd2cd4e10e586f0d977f3fa3fc767453b22970f41756f84925349da78`

## Structural results

| Measure | Result |
|---|---:|
| Accepted deliverables / DAG nodes | 101 / 101 |
| Local dependency registers | 93 |
| Local / aggregate rows | 1,480 / 1,480 |
| Local status rows | 1,425 ACTIVE / 55 RETIRED |
| Aggregate status rows | 1,395 ACTIVE / 85 RETIRED |
| Accepted aggregate-only duplicate retirements | 30 |
| Unique active directed edges | 972 |
| Orphan endpoints | 0 |
| SCCs | 0 |
| Duplicate active edges | 0 |
| Bidirectional pairs | 0 |
| Hubs at threshold 20 | 20 |
| Topological waves | 15 |
| Canonical findings | 0 |

All 1,480 dependency IDs match between local and aggregate registers. Every
common field other than `Status` and `Notes` matches. The exact 30 status
differences equal the approved duplicate-retirement worklist, and direction-
aware active topology is identical.

## Disposition

`DAG-008 = CURRENT_BY_REVALIDATION`

Keep DAG-008 and `execution/_DAG/_LATEST.md` unchanged. This closes only the
SCA-008 dependency-closure rerun. It does not regenerate dependencies or
authorize a DAG successor.
