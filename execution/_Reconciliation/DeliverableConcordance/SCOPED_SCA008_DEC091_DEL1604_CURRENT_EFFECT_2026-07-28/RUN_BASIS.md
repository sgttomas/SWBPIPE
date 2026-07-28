# Scoped SCA-008 current-effect reconciliation — run basis

**RunID:** `SCOPED_SCA008_DEC091_DEL1604_CURRENT_EFFECT_2026-07-28`

**State:** `FROZEN`

## Activation

| Field | Identity |
|---|---|
| Activation ruling | `execution/_Coordination/_DECISIONS/D-60_sca008_current_effect_reconciliation_activation.md` |
| Activation SHA-256 | `8728813764376e7c19c0760178b3b052eed2c819b5f7ccd24dea2873dddccad9` |
| Activation application commit | `eba66bfe6ef00aa07d9cfac75eee79965218fa43` |
| Durable merge | PR #403 / `21e8e54e1f5648b7d3db29228271aaa8c7d8904f` |
| Frozen source basis | `main@21e8e54e1f5648b7d3db29228271aaa8c7d8904f` |
| Shared method | `docs/DELIVERABLE_CONCORDANCE_METHOD.md` revision 1, SHA-256 `abf3e78fce606c4557d61cdbfbdb7292a3d858838f6526da6b433d1bcd0ef627` |
| Current decomposition | revision 0.11, SHA-256 `82b835b5fd36d0fd337da5b084dbf146caa29c18d0e1ef8f96a06fcfa4363a07` |
| Current DAG | DAG-008 through `execution/_DAG/_LATEST.md`, SHA-256 `46c162ddd2cd4e10e586f0d977f3fa3fc767453b22970f41756f84925349da78` |

D-60 and its register row are present at the durable merge, satisfying the
method's activation-before-dispatch requirement. The examined worktree was
clean before this derivative was created.

## Exact scope

- authority: `DEC-063` and `DEC-091`;
- deliverable: `DEL-16-04`;
- claim identities: `DEL-16-04-REQ-009` and
  `DEL-16-04-DECL-005`;
- closure ceiling:
  `CURRENT_EFFECT_RECONCILED / CLOSED_WITH_RELIANCE_HOLD`.

No other historical claim row receives a fresh disposition.

## Historical package preservation

The 2026-07-11 package is immutable upstream evidence:

`execution/_Reconciliation/DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305/`

| Check | Frozen value |
|---|---|
| Git tree | `79914bd8b4a9bc183994e5a63746ef0f316ed3a2` |
| Tracked members | 294 |
| Ordered `git ls-tree -r` SHA-256 | `c547becd5eecae05cd6a2a5c64708223c136aa3c147ea55399f6b8cb8ce64f02` |
| Historical DEL-16-04 ledger SHA-256 | `514d23f819bf9de8c70c46d21d853d480961831da9588b1eea2dbbddea2a749b` |

The prior package is not edited, repointed, superseded in place, or treated as
current authority.

## Current DEL-16-04 source bindings

| Surface | SHA-256 |
|---|---|
| `_STATUS.md` | `39af5d4b4ebfcf0c0c46b122f29d33f33b0774094be317201ac8a8269572b366` |
| `_CONTEXT.md` | `dfee96be2f6a42034720dc98ab2c095e0b812f06fd449cea74bbd4bebf034a7b` |
| `MEMORY.md` | `f687e95232366012570553723093937bc94d8c2209c3df706e5b2bbce1d4458e` |
| `ScopeOfWork.md` | `6c5ba91d3b7fe6caad2ead5c9b9d3ebf1b697acb303b9078e53ef0a614bc2f1c` |

## Write and effect fence

This run writes only this new immutable sibling derivative. It does not create
or update a reconciliation pointer. It does not edit decisions,
decomposition, SCA, DAG, dependencies, deliverable files, ScopeOfWork,
product, runtime, client, implementation, lifecycle, release, repin,
professional-reliance, estimate, or schedule state.
