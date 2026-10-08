# RV113's own D38 audit of PY (SR-PY head `11cc14e3e6`), written before reading I91's RETURN and REPAIR_01

R-D38's obligation `[r1: N-6]`: list every check in PY that assumes a prepared source has a Call or a Run; relax each
to (4b), or show it does not apply. Line numbers are PY at the head. "(4b) shape" means: a CaseSource registered and
prepared, the case unavailable with `prepared_product_failure`, the attempt's native stage `failed` with no Run, no
proof, no Call, Group or Build position, and no `execution_order` entry.

| # | Check (PY, head) | Assumes a Call or Run for a prepared source? | Status at the head |
|---|---|---|---|
| 1 | `_g5_stages` 866-875: an entered native stage has `run_ref` and a Run (I1's `else` branch) | **Yes** | **Relaxed** by the new `elif` branch to `_d38_capture_before_run` (843-851), only for `native == failed` with `run_ref` null |
| 2 | `_g5_products` 1025-1026 (D4e): `run_ref` non-null iff the case has a Run, same id and source | No: null `run_ref` with no Run is its own branch | Holds for (4b) unchanged |
| 3 | `_g5_products` 1028-1030 (D4a/D4b): a non-null `source_ref` resolves to a CaseSource owned by the case, same material basis, whose preparation binds this attempt | No | Holds for (4b) unchanged; it is what makes the source "registered beside" the attempt |
| 4 | `_g5_products` 1092-1103 (D4d): the reason from the error kind; `capture` with no Run maps to (`source_unavailable`, `preparation`) | No: the no-Run capture branch predates B1 | Holds unchanged |
| 5 | `_g5_products` 1017-1022 (D19): an unavailable result is carried by `prepared_product_failure` naming this attempt | No | Holds; it duplicates two of the predicate's conjuncts (cause kind, cause attempt) at the same gate and code, so those two conjuncts are reader-equivalent (only the predicate unit test sees them) |
| 6 | `_g5_coverage` 804-806: a complete summary vector needs the attempt's source, Run and selected kernel | Only when `summary_coverage` is non-null | Not applicable: (4b) has `proof` null, and the function returns first |
| 7 | `_g5a_coverage` (1339, 1357): reads `case["run"]` | Only for a selected case, or an unavailable one with a proof and a complete vector | Not applicable to (4b) |
| 8 | `_g5_native_checks` 608-615: each Call position binds its Run, case, source and owner | Run-scoped: it iterates Calls, never sources | A (4b) source in no Call passes; one placed in a Call is refused (m7: G5 ATTEMPT) |
| 9 | `_g5_native_checks` 738: the Calls' `run_refs` are exactly the Runs | Run-scoped | Holds |
| 10 | `_g5_native_checks` 739-742, 748-769: Group sources lie in their Call; Groups are derived from Call positions; each Group's sources share its stiffness | Run/Call-scoped; no rule that every source is in a Group | A (4b) source in no Group passes; one in a Group is refused (m7: G5 ATTEMPT) |
| 11 | `_g5_native_checks` 770-782: Build ids, work and Run cache snapshots | Run-scoped | Holds |
| 12 | `_g5_native_checks` 747 (new): every Build is referenced by its building record | Not an assumption: a new obligation | **Added** (the orphan-Build check): (4b)'s "no Build" conjunct, and Rust's `builds_seen` and TS's `seenBuilds` parity. Without it a Build from the removed Run survives |
| 13 | G3 1800-1802: `execution_order` is a bijection onto the cases with a Run | Run-scoped | Holds; a (4b) case listed there is refused (m4: G3 COVERAGE) |
| 14 | G3 `old_coverage == complete` (D23) and the I57 summary roster | Source- or proof-scoped, no Run | Holds |
| 15 | G8's CaseSource loop: every source re-derived from the invocation | No Run or Call read | Holds: (4b)'s registered source is checked like any other |
| 16 | G5 WORK `_accounting_rules` | Reads the attempt's own work, error and proof (`or {}`) | Not applicable to a Run |

Conclusion: one check assumed a Run (row 1) and is relaxed exactly to (4b). One check was missing and is added (row 12).
The rest are Run- or proof-scoped, or already admit a capture with no Run.
