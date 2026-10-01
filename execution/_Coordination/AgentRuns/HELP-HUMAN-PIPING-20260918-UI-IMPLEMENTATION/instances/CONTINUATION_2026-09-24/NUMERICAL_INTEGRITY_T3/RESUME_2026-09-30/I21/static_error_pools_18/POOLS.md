# Two finite parity error-message pools

TASK I21; existing native child of WORKING_ITEMS recovery manager. Actual start 2026-10-01 19:41:35 UTC; deadline 19:56:35 UTC. Frozen production source: 40129a225d73860ac2a53da9a2fa73869df668f3. The grant is the committed I21_STATIC_ERROR_POOLS.md at NUM4f6793b4ed0356d3e731d9327936a4de27bbbd24. This is an additive source result, subject to independent review. format_stream_17 and all earlier seals remain unchanged.

The complete conservative producer union below gives **P_STATIC_STRUCTURAL = 77** and **P_STATIC_FRAME = 41**, in unquoted UTF-8 bytes. These are source-derived upper substitutions for the actual parity caller, not arbitrary bounds on every public caller of these error types.

Source aliases in tables: P = VR/src/parity.rs; F = frame_kernel/src/lib.rs; S = frame_kernel/src/structural.rs; Q = frame_kernel/src/structural/sparse.rs; D = sparse_direct/src/structural.rs. All complete repository paths and original blob hashes are in _run_records/SOURCES.json. Numbered source excerpts preserve the inspected function bodies.

## Actual caller and finite route

P45–102 binary64_model formats errors only from FrameNode::new (P50), FrameSection::new (P56–64), and FrameElement::new (P65–71). Its directional-spring refusal (P78) is a separate fixed String already outside these two static enum-payload cells.

P200–241 forms contributions/allowances, calls assemble_sparse_stiffness with empty users/blocks and SparseAssemblyOptions::new (P204–212), then calls solve_sparse_structural and class. Q484–487 fixes UNSCALED; Q601–613's force-scaled recursion is not taken. P249–250 also formats assemble_global_stiffness if the original dense option is enabled; P287–288 classifies solve_structural_dense. The union includes that dense sibling, preserving both actual parity branches without changing any fixture or option.

contributions/allowances use expect at their existing interfaces; a panic there is not a returned FrameKernelError/StructuralError payload. This packet does not infer that those calls cannot panic and does not substitute these pools for panic/runtime accounting.

## Frame name/detail closure

| Routed producer | Fixed argument sources | Error field / forwarding |
|---|---|---|
| FrameNode::new | F436 coordinates | validate_finite_vector → NonFiniteInput.name |
| FrameSection::new and local_stiffness section revalidation | F460–465 and F713–718: six fixed section labels | validate_positive_finite → NonFiniteInput.name / NonPositiveInput.name |
| FrameProperties::new and local_stiffness length check | F486/F719 length | same positive-finite forwarding |
| FrameElement::new/length/local_x_axis | F574 y_reference; F588/F595 element length | finite-vector name; normalize → DegenerateAxis.detail |
| FrameOrientation::from_x_axis_and_y_reference/new | F500,F529–536: fixed axis/reference labels; F511,F518,F1875: three fixed orientation details | finite-vector, normalize, or direct InvalidOrientation.detail |
| local_stiffness coefficient formation | F739–794: every fixed coefficient/intermediate label | checked_formation_product/quotient → checked_formation_value (F819–850) → NumericalRange.name |
| local/global stiffness validation | F812/F616: computed local/global stiffness | validate_named_finite_slice → NonFiniteInput.name |
| dense and sparse unscaled assembly | F1328/Q675: assembled stiffness | named finite validation / direct NonFiniteInput.name |

F581–613 explicitly connects construction → properties/orientation and global_stiffness → local_stiffness/orientation. F1286–1329 routes dense assembly to the same global_stiffness implementation; Q616–623 does so for sparse assembly. Their node checks only construct index-valued variants, adding no static payload. The empty user/block loops add no alternative implementation. The numerical/matrix helpers used by formation either return numbers/matrices or forward the listed fixed names.

The forwarding helpers F1832–1847, F1855–1877 and F819–850 do not synthesize strings. Their name/detail arguments on these call edges are precisely the constants in LITERALS.json. The numeric model supplies f64/usize/array values, not a static-name parameter. Public force_scaled_value/matrix and unrelated reduction/solve APIs can accept other names, but none is called on this unscaled route.

There are 57 constant argument/constructor occurrences and 47 distinct frame payload literals. Maximum: F518, InvalidOrientation.detail, `local axes must form a right-handed basis`, 41 UTF-8 bytes. Every literal and occurrence, including all coefficient labels, is preserved in LITERAL_TABLE.md/LITERALS.json with its actual variant and forwarding route. No sample is substituted for the complete finite set.

## Structural reason closure

StructuralError's only static text fields are S256 InvalidInput, S257 Range and S264 NumericallyUnresolved.reason. P168–180 selects those payloads using Display interpolation; it does not Debug-format full StructuralError, its direction vectors, or floating fields.

| Route / owner | Complete error-producing closure |
|---|---|
| Sparse entry and prepare | D104–107 → Q1272–1275 prepare_sparse_bound; Q1031–1103 validation, Q1184–1213 contribution sums, Q1299–1479 preparation, Q1488–1513 zero-product fold, Q1519–1618 contribution audit |
| Sparse ordering | D56–75 discards adjacency/order/profile error values with map_err and substitutes sparse adjacency / sparse order / sparse profile |
| Sparse factor and finish | D80–99 → Q1743–1809; checked profile factor and solve are S1994–2055; Q1621–1666 validates prepared storage |
| Refused sparse factor | D94–95 → Q1937–2012 pair witness/energy and Q1908–1929 verdict; all fallible arithmetic uses the shared checked helpers |
| Dense parity sibling | S1930–1933 → S1189–1192 prepare_bound S1266–1383; validation S611–700, contribution audit S800–911, factor S1897–1906/cholesky S1832–1874, refusal witness S2155–2267, finish S1915–1928 |
| Shared completion | S1667–1805 finish_checked_factor, S1562–1662 condition estimate, S1428–1525 original residual, S918–986 intended action; solve callbacks are exactly ProfileFactor::solve or CholeskyFactor::solve (S1811–1829), not caller-supplied closures |
| Shared primitive failures | S561–610 radix/checked arithmetic, Expansion S712–776, exact_radix S785–798, normalized_product/physical_residual S1386–1409, pivot screen S1526–1550 |
| External exact-sum errors | S778–783 sum_range exhaustively replaces all three SumError variants with its three literals; exact-scaled RHS S1235–1264 likewise maps its errors to fixed Range strings |

Each fallible return in this closure either constructs one of the listed constant payloads, forwards one of the listed local errors, maps an external error to a listed constant, or returns a non-text variant. In particular:

- prepare_structural and prepare_sparse_structural set ForceBinding::Legacy (S1192/Q1275); constructed prepared systems set formation=None (S1371/Q1458). S208–218 makes Legacy.assembled()/audit_terms() None. Consequently completion's load-fidelity/formation callbacks at S1724–1749 are not an open producer edge. Their unrelated message trees were not audited.
- The generic condition/completion function signatures alone would permit arbitrary error-producing callbacks. Actual completion calls fix the callbacks at S1927/Q1808 to their private factor solve implementations. The caller cannot inject a different error string at this seam.
- Symmetry evidence BASIS can be copied into an accepted report (S544–548/Q1714–1718), but never becomes a reason/name/detail payload; malformed evidence yields the fixed symmetry provenance label.
- unresolved (S557–558) simply places its borrowed reason in NumericallyUnresolved. Every call in the finite closure supplies a literal; the sole nonliteral first-argument search occurrence is the helper's definition itself. No computed string, returned external str, leaked allocation, concat/include macro or user text reaches these fields on this route.
- The conservative union retains Q1303–1308's fixed binary64-binding rejection even though Legacy makes that branch false here, and retains checks that valid inputs may never fail. It does not assume a failure must occur or remove an original fixture.

The union has 75 constructor/argument occurrences and 55 distinct structural literals. Maximum: S816–819 and Q1206–1209, NumericallyUnresolved.reason, `positive diagonal contribution absorbed by assembly; stabilization unresolved`, 77 UTF-8 bytes. Variant maxima are InvalidInput45, Range34, NumericallyUnresolved77. This is a producer superset, not a data-driven list of observed errors.

## Substitution and ownership consequences

Raw literals remain static borrowed data; their existence creates no new registered heap child. Result enums still retain whatever other variant owners the previous phase ledger already counted. This result only replaces the two text-length parameters.

For structural class, the existing exact prefixes give
`D_class <= max(14+45, 7+34, 23+77, 14) = 100`.
With format_stream_17's unchanged F(D)=max(8,2D), the fresh formatted destination request is ≤200 and destination-plus-active-old grow is ≤300. Fixed-label branches use their original exact literal clone lengths. When the result is copied into a JSON String, preserve the source String plus the new logical-byte child; that is a separate phase alternative, not a second simultaneous grow surcharge. Do not retain a dropped error/solver owner into later emission.

For FrameKernelError, **41 is unquoted UTF-8 length**, not escaped Debug length. All enumerated literals are printable ASCII without quotes/backslashes/control characters, but this packet keeps the previously supplied generic `2+6M` quoted-string envelope rather than reopening formatter proofs. Its substitution is ≤248 quoted bytes; the already-derived full enum formula becomes
`D_frame <= max(87, 32826 + 6*41) = 33072`.
The corresponding fresh destination request is ≤66144; destination plus active old is ≤99216. The large constant is inherited from the explicit f64 default-Debug/i16 overapproximation, not a measured or typical output length and not an arbitrary safety factor. Private request/allocator/runtime and outer consumer owners remain separate.

No request sizes for I23's three private errors were read or inferred. No source14 numerator, prefix, already closed primitive formatter, or pre-cut union work was duplicated. No complete E_max, prelaunch admission, source/build correspondence or whole format_stream17 acceptance follows from this source result.

## Remaining premises

The two named static-message pool cells are proposed closed for the pinned actual parity routes, pending independent review. There is no remaining unknown static producer within the traced returned-error closure. Arbitrary calls to public class/error constructors or generic callback APIs are outside this binding and have no universal static-string bound here. Existing fatal/allocation-failure, panic/expect/indexing, backend and final-build correspondence premises are unchanged interfaces; this tranche does not set their costs to zero.

Read-only text searches over explicit inspected spans and literal/integer arithmetic were used, not Rust/source execution or a Rust parser. One evidence-range read exceeded parity.rs's final line and returned IndexError; its raw failure is preserved, then the bounded source read was repeated with an end-of-file clamp. It did not run product code or alter any source.

All writes are this additive packet. Raw commands, original hashes and numbered source excerpts are under _run_records. No maintained file, Git/index, allocator/toolchain/library or test data was changed; no delegation or runtime experiment occurred. No experiment is running.
