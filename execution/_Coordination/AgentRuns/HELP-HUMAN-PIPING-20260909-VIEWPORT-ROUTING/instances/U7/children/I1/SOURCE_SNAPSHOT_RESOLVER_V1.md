# U7/I1 source snapshot resolver V1

- Archive: `SOURCE_SNAPSHOT_V1.tar.gz.b64`
- Archive encoding: single-line Base64 of a gzip-compressed tar archive
- Encoded bytes: `177497`
- Encoded SHA-256: `c4a94940582e96e7d6bed834e6bcf397d6383dd9addae669708fe8ab21cbf91a`
- Members: the exact six project-relative source/test paths listed in `TERMINAL_RETURN_V1.md`

Restore into an empty inspection directory on macOS:

```text
base64 -D -i SOURCE_SNAPSHOT_V1.tar.gz.b64 | tar -xzf -
```

This is a current-source snapshot, not a basis diff. Root/CHANGE owns the authoritative basis-HEAD diff because I1's sealed brief prohibited Git commands.
