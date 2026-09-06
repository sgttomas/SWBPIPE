# V1 lossless evidence transport

The accepted original digest remains `27ba19b9429bff886238db135ffed9f8cb4baa67de0adf8f65e49f503ff0c5a1`. FINAL_MANIFEST.json is physically unchanged and is now an explicitly historical logical manifest. It must be resolved through the complete original archive, rather than expecting the removed raw log at its original physical path.

`transport_archive/ORIGINAL_V1/ARCHIVE.json` preserves all 110 original data members and the original manifest as base64 with per-member SHA256 and byte counts. Decode each `files[path].data` into a fresh empty temporary directory, verify each SHA256, then validate the materialized FINAL_MANIFEST.json against its 110 members. Never materialize over this accepted packet.

Only the raw physical `baseline/cargo.log` was removed. Its exact bytes remain both in the complete archive and in `baseline/encoded/cargo.log.json` (decode `content`). TRANSPORT_MAP.json explicitly maps every historical member and the removed log. FINAL_TRANSPORT_MANIFEST.json binds the current physical evidence presentation. No report, CSV, calculation, test result, source input, or raw execution byte was changed.

The root authorized this transport-only remediation to remove the terminal-blank-line whitespace blocker without introducing a whitespace exception. No source change or test rerun occurred.
