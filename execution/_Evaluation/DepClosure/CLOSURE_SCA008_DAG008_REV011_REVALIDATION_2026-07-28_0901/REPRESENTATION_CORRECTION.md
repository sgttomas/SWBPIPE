# Representation correction

**Date:** 2026-07-28
**Status:** APPLIED BEFORE PUBLICATION

The first staged-byte check found CRLF line endings in eight generated CSV
evidence files and Markdown hard-break spaces in the two owner-ruling
postimages that cite this derivative. The semantic audit and all recorded
values were unchanged.

Before publication, the supervising Agent 0 accepted the Piping Agent 1
recommendation to:

1. normalize the derivative snapshot to LF line endings;
2. remove trailing spaces without changing semantic text;
3. regenerate `MANIFEST.sha256`; and
4. rerun the manifest, DAG, receipt, path, and staged-byte checks.

This record is additive. The unpublished pre-correction commit is not an
accepted snapshot and must not be relied upon.
