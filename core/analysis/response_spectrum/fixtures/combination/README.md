# Combination contract fixtures (DEL-04-10 unit S4)

Each fixture is an input document (`*.input.json`, `openpipestress.spectrum_combination.input/v1`)
and the output document the combinations give for it (`*.output.json`,
`openpipestress.spectrum_combination.output/v1`). The documents are the combination unit's own
interchange: the contract that a producer of signed contributions and a later document unit and
its readers can build on. They are not `spectrum-1` documents and not spectrum solve results.

- **The inputs** are frozen references' published signed contributions, modal metadata and rigid
  responses, each as its binary64. A mutated input changes one stated thing. Each input's
  `provenance` says which.
- **The outputs** are the product's own earlier output and only guard against change. The
  combined magnitudes are checked against the references' values in
  `tests/f2_combinations.rs`.

| Fixture | Source | Shows |
|---|---|---|
| `c4_base` | discrete C4-BASE | CQC on a close pair with opposite-sign contributions, rigid response by absolute sum, 100-40-40 over X and Y |
| `c4_mutated_sign` | C4-BASE, one sign reversed | CQC of N3.uy under X changes; Y is unchanged |
| `c4_refused_cqc_zero_damping` | C4-BASE, ζ = 0 | `SPECTRUM_CQC_DAMPING_UNSUPPORTED` |
| `c10_w3_straddling_cluster` | discrete C10-W3 | clusters of unequal frequencies, the first straddling f_r and periodic as a whole; signed cluster sums |
| `c10_w3_refused_cluster_split` | C10-W3, cluster 1 split | mode 2 now forms a rigid cluster: `SPECTRUM_CONTRIBUTION_NOT_PERIODIC` |
| `c10_w3_refused_rigid_premise` | C10-W3, wrong premise | `SPECTRUM_RIGID_RESPONSE_PREMISE_MISMATCH` |
| `c9_y_member_axes` | discrete C9-Y, member-axis end forces | rows in member axes (`P1.i.Fx_m`) |

`tests/contract_fixtures.rs` rebuilds every fixture and fails when a committed one differs. After a
deliberate change, regenerate the fixtures with the command below and review the diff.

```sh
COMBINATION_FIXTURES_BLESS=1 cargo test --manifest-path core/analysis/response_spectrum/Cargo.toml --test contract_fixtures
```
