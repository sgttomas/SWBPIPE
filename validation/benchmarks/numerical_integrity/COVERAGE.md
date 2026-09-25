# Reference coverage and outstanding probes

| Cases | Frozen independent data | Executable now | Product/gate coverage still required |
|---|---|---|---|
| N01 | EB tip displacement, rotation and root reactions | Generator and consistency checks | Product comparison at protected 1e-9 |
| N02/N03 | Planar axis, null motion, ranks/inertias, rotational spectrum | N02 loaded/unloaded product observer prepared | Gate witness, RX stabilized/RZ wrong companion, spatial rotation |
| N04 | Disconnected geometry, dimension/inertia | Product observer prepared | Per-component diagnostics and null witness |
| N05/N06, NP-A | Intended and exact stored matrices/solutions, exact binary64 hex, ULP sweep, spring action | Scalar comparisons run; product observer prepared | Product 1e-9 physical accuracy; stored vs intended observations; contribution-preserving repair |
| N07 | Invalid K, load, solution, negative vector/energy | Scalar energy check; generic/sparse observer prepared | New typed gate rejects with mapped energy witness |
| N08/N09 | Signed torques, rotations, reactions and slender bending | Precision-loss control run; torque product observer prepared | Full-precision product/native/persistence/export/rule path; N09 bending |
| R01/R02 | Immutable original equation versus wrong candidate/used matrix | Original-equation negative control run | Mutation hooks for dense, sparse, fallback shared gate |
| R03 | Omitted load, zero displacement/work, eta=1 | Negative control run | Production gate kills work-only mutation |
| R04 | Nonzero prescribed map, solution/reactions, omitted shift candidate | Negative control run | Kernel boundary API then M10 product witness |
| R05 | Axial tip ground spring k_s=k/4; omitted spring residual 250 N, eta=1/9 | Negative control run | Sparse-only omission injected into gate |
| R06/R07 | Constrained reactions; zero-load nonunique null state | Scalar reference controls | Product compatibility and no residual-as-stability inference |
| NP-B | Orders 8/32/64/80/128, c=.75 and .750000000001, sparse matrices, exact selected u/load, 3 permutations | Certificate pessimism scalar checks run | Structural API factor/residual/condition observations in all permutations |
| NP-C | Full rigid rank with internal slip; stabilized companion; exact near-collinear rows | Scalar rank-vs-internal control run | Supported product connector/release family, numerical ambiguity classification |
| NP-D | Skew, duplicate lost term, R04/R05/R06 references, range operands, active/inactive contact mode | Data frozen | Production rejection/mutation/range accounting and actual nonlinear selected-state lift-off |

Still not executable gate claims: there is no new structural integrity API at this reference freeze. The source-only observer uses only existing APIs and records results without passing labels. General orthogonal rotation (including (1,2,2)/3), coherent mm/N/MPa versus m/N/Pa, large origin shifts, uniform E/G scaling, support uncertainty, realistic chains/grids, and all unchanged DEC-050/053 observations require candidate execution/integration. This coverage map deliberately retains those open requirements; schema presence does not mean a gate test ran. Product N05/N06 observer DTO validity itself must be observed, not inferred from source authoring.
