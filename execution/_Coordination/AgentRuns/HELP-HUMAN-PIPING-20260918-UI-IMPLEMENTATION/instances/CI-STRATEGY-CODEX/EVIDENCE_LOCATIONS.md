# CI evidence locations

Raw generated CI outputs and indexes now live under `_run_records/`, retaining
all original bytes and the earlier directory structure. `EVIDENCE_LOCATIONS.json`
maps every prior relative path to its current path and SHA256. The original paths
remain available at revision73fc5fd6. The two sealed RETURN.md files are unchanged;
read their historical pointers through this map. This fixes raw absolute-path
classification without normalizing or changing empirical evidence. No product,
workflow, selector, assertion or test behavior changes.
