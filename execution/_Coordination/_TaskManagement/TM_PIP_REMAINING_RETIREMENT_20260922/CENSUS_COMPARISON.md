# Pinned census comparison

Compared only the authorized R5 continuation census, `REMAINING_WORK_CENSUS.csv` (SHA-256 `646470f87ea916b939bdbf1d66877ae8181fcd61bd1922b27f63900b30e70551`), with the 106 current Piping DEL `_STATUS.md` Remaining sections at basis `b3e2ce4ec74e01d6f393fc0bc069699bb079df91`.

All 239 census records resolve to the same 106 current status files; every one of the 106 current file hashes matches the census `SourceSHA256`. There is no source-byte drift. The census has 219 non-NONE rows while the 106 current Remaining sections contain 215 live bullets. Other than the two cardinality exceptions below, per-deliverable row/bullet counts match. `DEL-01-01/NONE` correctly corresponds to an empty Remaining section.

| Deliverable | Census rows | Current bullets | Finding |
|---|---:|---:|---|
| DEL-05-02 | 5 | 3 | Census items 3–5 are text fragments that split one current bullet; preserve their original identities together as one entry. |
| DEL-12-01 | 4 | 2 | Census items 1–4 are fragments outside the canonical Remaining section (CSV physical lines 163–166). The same hash-matching status file has two live bullets omitted by extraction: LFSP-REQ-011 runtime private-path resolution/review mapping, and human dispositions for RF-001/RF-002. Preserve the four fragment rows and carry both real obligations as current entries. |

These are census extraction/cardinality findings, not fulfillment, closure, or authorization to remove source entries. The mechanical row-level comparison and current section text are retained in ignored `.candidates/TM_PIP_REMAINING_RETIREMENT_20260922/census_comparison.json` for rebuildable inspection. No register or `_STATUS.md` content was changed.
