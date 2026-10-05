# I62 return: checkpoint C1c (snapshot 05c)

**Basis:** the ruling "Snapshot 05b verified; three non-native must-pass entries retired as 05c" (NUM 1586bca12e).

**Run window:** 2026-10-03T20:59:07Z to the 20:59:59Z freeze. The memory guard (PID 5387) was running. No Git writes, no Cargo, no native job.

## Change

The corpus moves from ea6fe2c757… to **85bff98ea76e5d3433076d12593c91bc49350cc374981db259259282926e3824**.

Three must-pass entries were removed:

| Entry | Reason |
|---|---|
| `cert_failed_after_summary` | Its numeric predicate failure contradicts identical case-0 inputs. |
| `cert_failed_predicate_null_undetectable` | Same contradiction. |
| `old_operational_error_new_ready` | No native trigger is established for a `coefficient_range` refusal on ordinary-magnitude operands. |

`cert_failed_after_summary_accounting` stays.

Everything else is byte-identical (asserted), and nothing was rehashed:
- **Corpus:** all 9 cases, all 121 mutations and the other 16 must-pass entries. The diff is deletion-only: 3 hunks, 227 lines.
- **Other files:** the schema, table, yaml, definition, reader (3b12ca7511) and both test files.

**Final counts:** 9 cases, 121 mutations, 16 must-pass entries.

## Tests

`python_schema_E1`: **164 passed, 0 failed.** That is 05b's 167 minus the 3 must-pass entries the test runs one by one. The input hashes match the final files.

## Note

No Python-only test referenced the removed entries. However, the pre-existing I58 Python-only test `test_old_operational_error_is_retained_independently_of_new_ready` makes the same `coefficient_range` edit on its own. I left it unchanged, as it is outside this grant; ROOT may rule on it.

SHARED_SNAPSHOT_05C.json has the file hashes, the removal reasons, the counts and the bulk listing.
