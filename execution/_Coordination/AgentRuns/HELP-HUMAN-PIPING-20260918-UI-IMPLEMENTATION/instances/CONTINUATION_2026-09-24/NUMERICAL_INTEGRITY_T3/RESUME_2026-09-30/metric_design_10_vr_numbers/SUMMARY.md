# RF-LARGE24 VR caller-number candidate

**Caller components only: not full E_max, admission replay or measurement.**
The JSON table contains each orientation separately, every named requested/move
phase, numeric kernel addends, and the now bound expected-list parser scratch component. Values below are the maximum across AX/ROT for the
same family/size. Public sizes and f64 parse are independently reviewed; final
ordinary-production correspondence remains required.

| Family | Members | Caller-only requested max B | Caller-only move max B | Dominant caller phase |
|---|---:|---:|---:|---|
| CHAIN | 10 | 4,020,127 | 4,020,318 | prefix_before_cut |
| TREE | 10 | 4,020,125 | 4,020,316 | prefix_before_cut |
| CONT | 10 | 4,020,125 | 4,020,316 | prefix_before_cut |
| CHAIN | 100 | 8,367,890 | 9,810,290 | late_sparse_parity |
| TREE | 100 | 8,363,944 | 9,806,344 | late_sparse_parity |
| CONT | 100 | 7,103,863 | 7,915,663 | late_sparse_parity |
| CHAIN | 1,000 | 337,949,334 | 481,973,334 | late_sparse_parity |
| TREE | 1,000 | 338,031,093 | 482,055,093 | late_sparse_parity |
| CONT | 1,000 | 211,884,109 | 292,902,109 | late_sparse_parity |
| CHAIN | 10,000 | 29,394,129,202 | 43,794,369,202 | late_sparse_parity |
| TREE | 10,000 | 29,394,825,566 | 43,795,065,566 | late_sparse_parity |
| CONT | 10,000 | 16,793,318,994 | 24,893,498,994 | late_sparse_parity |

The small-case maximum includes loading/parsing the entire fixed RF-LARGE family,
not only the chosen case. The large sparse maximum uses the proved triangular
profile ceiling. It is deliberately conservative and is not an observed footprint.

All six10,000-member rows have a factor-value subterm alone above the unchanged
4,026,531,840-byte binary half-heap backstop. Hence this upper-bound policy
would defer those VR launches. Neither actual memory excess nor an H staged
deferral follows. The10/100/1000 caller components are below that threshold;
that does **not** establish admission because Kernel_phi and chronological
rho/footprint/projected-RSS calibration are still required. No replay is reported.

The exact kernel joins are in METHOD_AND_JOIN and each row’s kernel_phase_addends.
The source/outcome slot must use VR source capacity and exclude caller owners.
No implicit kernel zero has been inserted into any claimed full bound.

Expected-list scratch is now supplied by the sealed RV28 expected_list_parse_05:
80 retained,120 construction move,0 after parse. Its phase keeps read/path/Jtree
with the surrounding kernel Outcome; the71-byte owned String remains counted
once in Jtree. All final JSON unbound_source_addends are empty for the scoped
caller arithmetic. The five Kernel_phi interfaces are intentionally unjoined.

Scoped failures retain the reviewed normal/handled-I/O and returned typed-error
qualifications. General allocator/backend/expect/panic/foreign-entry or invalidated
input/build premises are not assigned zero. All prior artifacts remain sealed.
