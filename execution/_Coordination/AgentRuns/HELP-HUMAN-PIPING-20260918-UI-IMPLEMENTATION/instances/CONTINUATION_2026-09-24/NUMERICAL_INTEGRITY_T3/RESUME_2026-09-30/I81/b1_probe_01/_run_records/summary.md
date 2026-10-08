## Direct entry (probe log probe_run2.log)
| Input | Mode | Mechanics | Published verdict | Seed `initial` | Seed `w2` | Legacy | W1 runs | W1 outcome | Native | Notices | Bytes | T-4 class |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| u8 first_load_only | sparse | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | unavailable(stage=source closure) | yes | Err(Candidate) | Selected | 1 | with_notice | A |
| u8 two_body_case_a | sparse | MECHANICS_SOLVED | sensitive | structural_failure/range | published b=518 | unavailable(stage=source closure) | yes | Ok(successor) | Selected | 0 | successor | A |
| u8 two_body_case_b (W-C1) | sparse | MECHANICS_SOLVED | checks_passed | structural_failure/range | published b=518 | unavailable(stage=source closure) | yes | Err(Native) | Unresolved reason=Ceiling | 1 | with_notice | not_required |
| u8 l0_isolated_node | sparse | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | unavailable(stage=source closure) | yes | Ok(successor) | Selected | 0 | successor | A |
| u8 first_load_only | dense | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | unavailable(stage=source closure) | yes | Err(Candidate) | Selected | 1 | with_notice | A |
| u8 two_body_case_a | dense | MECHANICS_SOLVED | sensitive | structural_failure/range | published b=518 | unavailable(stage=source closure) | yes | Err(Precommit { gate: "G8", code: "RETAINED_PRECISION_PREPARATION_MISMATCH" }) | Selected | 1 | with_notice | A |
| u8 two_body_case_b (W-C1) | dense | MECHANICS_SOLVED | checks_passed | structural_failure/range | published b=518 | unavailable(stage=source closure) | yes | Err(Native) | Unresolved reason=Ceiling | 1 | with_notice | not_required |
| u8 l0_isolated_node | dense | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | unavailable(stage=source closure) | yes | Ok(successor) | Selected | 0 | successor | A |
| attempted: milestone | sparse | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | unavailable(stage=source closure) | yes | Ok(successor) | Selected | 0 | successor | A |
| attempted: failed attempt | sparse | MODEL_INCOMPLETE | unresolved | structural_failure/numerically_unresolved | not_triggered | unavailable(stage=source closure) | yes | Err(Preparation) | - | 1 | with_notice | A |
| attempted: deferred formation (K2a partial underflow) | sparse | MECHANICS_SOLVED | sensitive | formation_failure (NumericalRange { name: "12EIy/L^3: (12*E)*Iy" }) | published b=898 | declined_without_attempt | yes | Err(Candidate) | Selected | 1 | with_notice | A |
| attempted: rejected_stress_range sparse | sparse | MODEL_INCOMPLETE | sensitive | report/sensitive | not_triggered | exact_selected | yes | Err(Preparation) | - | 1 | with_notice | A |
| attempted: rejected_stress_range dense | sparse | MODEL_INCOMPLETE | sensitive | report/sensitive | not_triggered | exact_selected | yes | Err(Preparation) | - | 1 | with_notice | A |
| attempted: milestone | dense | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | unavailable(stage=source closure) | yes | Ok(successor) | Selected | 0 | successor | A |
| attempted: failed attempt | dense | MODEL_INCOMPLETE | unresolved | structural_failure/numerically_unresolved | not_triggered | unavailable(stage=source closure) | yes | Err(Preparation) | - | 1 | with_notice | A |
| attempted: deferred formation (K2a partial underflow) | dense | MECHANICS_SOLVED | sensitive | formation_failure (NumericalRange { name: "12EIy/L^3: (12*E)*Iy" }) | published b=898 | declined_without_attempt | yes | Err(Candidate) | Selected | 1 | with_notice | A |
| attempted: rejected_stress_range sparse | dense | MODEL_INCOMPLETE | sensitive | report/sensitive | not_triggered | exact_selected | yes | Err(Preparation) | - | 1 | with_notice | A |
| attempted: rejected_stress_range dense | dense | MODEL_INCOMPLETE | sensitive | report/sensitive | not_triggered | exact_selected | yes | Err(Preparation) | - | 1 | with_notice | A |
| milestone (W1, W4, W7, headroom; attempted 'milestone') | sparse | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | unavailable(stage=source closure) | yes | Ok(successor) | Selected | 0 | successor | A |
| W2 cap_maximal | sparse | MODEL_INCOMPLETE | not_assessed | (no seed) | - | - | no | Err(CompleteGate(PhaseRefusal { gate: Complete, fact: OrdinarySolveNotAttempted, observed: 1, cap: 0 })) | - | 0 | plain | no seed |
| W2-deep | sparse | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | unavailable(stage=source closure) | yes | Ok(successor) | Selected | 0 | successor | A |
| W2b cap_maximal_solvable | sparse | MECHANICS_SOLVED | checks_passed | report/checks_passed | not_triggered | not_required | yes | Err(Candidate) | Selected | 1 | with_notice | not_required |
| W3 n05 | sparse | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | exact_selected | no | Err(Coexistence) | - | 0 | plain | T-3(c) coexistence (before T-4; no W1) |
| W6 force_scaled | sparse | MECHANICS_SOLVED | checks_passed | structural_failure/range | published b=536 | unavailable(stage=source closure) | yes | Err(Native) | Unresolved reason=Ceiling | 1 | with_notice | not_required |
| milestone (W1, W4, W7, headroom; attempted 'milestone') | dense | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | unavailable(stage=source closure) | yes | Ok(successor) | Selected | 0 | successor | A |
| W2 cap_maximal | dense | MODEL_INCOMPLETE | not_assessed | (no seed) | - | - | no | Err(CompleteGate(PhaseRefusal { gate: Complete, fact: OrdinarySolveNotAttempted, observed: 1, cap: 0 })) | - | 0 | plain | no seed |
| W2-deep | dense | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | unavailable(stage=source closure) | yes | Ok(successor) | Selected | 0 | successor | A |
| W2b cap_maximal_solvable | dense | MECHANICS_SOLVED | checks_passed | report/checks_passed | not_triggered | not_required | yes | Err(Candidate) | Selected | 1 | with_notice | not_required |
| W3 n05 | dense | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | exact_selected | no | Err(Coexistence) | - | 0 | plain | T-3(c) coexistence (before T-4; no W1) |
| W6 force_scaled | dense | MECHANICS_SOLVED | checks_passed | structural_failure/range | published b=536 | unavailable(stage=source closure) | yes | Err(Native) | Unresolved reason=Ceiling | 1 | with_notice | not_required |
| case_c | sparse | MECHANICS_SOLVED | sensitive | structural_failure/range | published b=518 | unavailable(stage=source closure) | yes | Err(Native) | Unresolved reason=Ceiling | 1 | with_notice | A |
| case_c | dense | MECHANICS_SOLVED | sensitive | structural_failure/range | published b=518 | unavailable(stage=source closure) | yes | Err(Native) | Unresolved reason=Ceiling | 1 | with_notice | A |

## Detail lines
- u8 first_load_only sparse_interactive cand: Proof(ProductProofFailure { failure: ProductFailure { cause: Predicate { row: 80, predicate: SharperExact } },
- u8 two_body_case_a sparse_interactive reader: PASS
- u8 l0_isolated_node sparse_interactive reader: PASS
- u8 first_load_only dense_scrutiny cand: Proof(ProductProofFailure { failure: ProductFailure { cause: Predicate { row: 81, predicate: SharperExact } },
- u8 two_body_case_a dense_scrutiny precommit: ValidationError { gate: "G8", code: "RETAINED_PRECISION_PREPARATION_MISMATCH", detail: None }
- u8 l0_isolated_node dense_scrutiny reader: PASS
- attempted: milestone sparse_interactive reader: PASS
- attempted: failed attempt sparse_interactive prep: error=Some(Association("prepared case custody/permit")) preparation_error=None
- attempted: deferred formation (K2a partial underflow) sparse_interactive cand: Proof(ProductProofFailure { failure: ProductFailure { cause: Predicate { row: 81, predicate: SharperExact } },
- attempted: rejected_stress_range sparse sparse_interactive prep: error=Some(Association("missing produced mode")) preparation_error=None
- attempted: rejected_stress_range dense sparse_interactive prep: error=Some(Association("missing produced mode")) preparation_error=None
- attempted: milestone dense_scrutiny reader: PASS
- attempted: failed attempt dense_scrutiny prep: error=Some(Association("prepared case custody/permit")) preparation_error=None
- attempted: deferred formation (K2a partial underflow) dense_scrutiny cand: Proof(ProductProofFailure { failure: ProductFailure { cause: Predicate { row: 81, predicate: SharperExact } },
- attempted: rejected_stress_range sparse dense_scrutiny prep: error=Some(Association("missing produced mode")) preparation_error=None
- attempted: rejected_stress_range dense dense_scrutiny prep: error=Some(Association("missing produced mode")) preparation_error=None
- milestone (W1, W4, W7, headroom; attempted 'milestone') sparse_interactive reader: PASS
- W2-deep sparse_interactive reader: PASS
- W2b cap_maximal_solvable sparse_interactive cand: Proof(ProductProofFailure { failure: ProductFailure { cause: Predicate { row: 7, predicate: SharperExact } }, 
- W3 n05 sparse_interactive: counts are not ONE_RUN_THROUGH_G_C
- milestone (W1, W4, W7, headroom; attempted 'milestone') dense_scrutiny reader: PASS
- W2-deep dense_scrutiny reader: PASS
- W2b cap_maximal_solvable dense_scrutiny cand: Proof(ProductProofFailure { failure: ProductFailure { cause: Predicate { row: 8, predicate: SharperExact } }, 
- W3 n05 dense_scrutiny: counts are not ONE_RUN_THROUGH_G_C

## Committed witnesses (witness log witness_run.log)
| Witness | Stack | Ran | Mechanics | Verdict | Seed `initial` | Seed `w2` | T-4 class (of the input) |
|---|---|---|---|---|---|---|---|
| W1@1MiB SparseInteractive | 1048576 | Successor | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W1@1MiB DenseScrutiny | 1048576 | Successor | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W1 SparseInteractive | 4194304 | Successor | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W1 DenseScrutiny | 4194304 | Successor | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W2 SparseInteractive | 4194304 | Fallback("Preparation") | MODEL_INCOMPLETE | not_assessed |  |  | no seed |
| W2 DenseScrutiny | 4194304 | Fallback("Preparation") | MODEL_INCOMPLETE | not_assessed |  |  | no seed |
| W2-deep SparseInteractive | 4194304 | Successor | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W2-deep SparseInteractive | 1048576 | Successor | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W2-deep DenseScrutiny | 4194304 | Successor | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W2-deep DenseScrutiny | 1048576 | Successor | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W2b SparseInteractive | 4194304 | Fallback("Candidate") | MECHANICS_SOLVED | checks_passed | report/checks_passed | not_triggered | not_required |
| W2b DenseScrutiny | 4194304 | Fallback("Candidate") | MECHANICS_SOLVED | checks_passed | report/checks_passed | not_triggered | not_required |
| W3 SparseInteractive | 4194304 | ExactSelected | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | T-3(c) coexistence (before T-4; no W1) |
| W3 DenseScrutiny | 4194304 | ExactSelected | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | T-3(c) coexistence (before T-4; no W1) |
| W4 SparseInteractive | 4194304 | Fallback("Preparation") | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W4 DenseScrutiny | 4194304 | Fallback("Preparation") | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W6 SparseInteractive | 4194304 | Fallback("Native") | MECHANICS_SOLVED | checks_passed | structural_failure/range | published b=536 | not_required |
| W6 DenseScrutiny | 4194304 | Fallback("Native") | MECHANICS_SOLVED | checks_passed | structural_failure/range | published b=536 | not_required |
| W7 native | 4194304 | Fallback("Native") | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W7 serializer | 4194304 | Fallback("Serializer(ReceiptFailure { check: Encoding, field_path: \"cases[].run.invocation_after\" })") | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W7 staging | 4194304 | Fallback("Staging(StagingFault(\"pipe_stress_extrema[]\"))") | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W7 precommit binding | 4194304 | Fallback("Precommit { gate: \"G8\", code: \"RETAINED_PRECISION_INVOCATION_MISMATCH\" }") | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
| W7 precommit corruption | 4194304 | Fallback("Precommit { gate: \"G1\", code: \"RETAINED_PRECISION_RECEIPT_MISMATCH\" }") | MECHANICS_SOLVED | sensitive | report/sensitive | not_triggered | A |
