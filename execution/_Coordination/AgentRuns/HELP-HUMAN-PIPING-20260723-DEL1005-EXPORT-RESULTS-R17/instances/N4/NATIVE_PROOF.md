---
doc_id: R17-DEL1005-N4-NATIVE-PROOF
doc_kind: coordination.native_proof
status: pass
created: 2026-07-23
---

# Native proof

This run-local proof uses invented fixtures only and makes no release,
professional-reliance, or public-data claim.

## Native runner

The final debug `openpipestress-runner` was executed directly with the adopted
invented success input.

| Case | Exit | Container bytes | SHA-256 | ZIP members | `$.report_package` decisions/findings |
|---|---:|---:|---|---:|---:|
| tracked invented success | 0 | 29,126 | `8abef0f82ce8be9a1ce5b0c427d66951fdf81f984b0854e524cbe6209bc062ab` | 6 | 1 / 1 |
| transient 3,200,000-character padding | 0 | 3,229,152 | `1c575aa2ac35c8504f08f78f1ee4f0b99c6ac5f6599270f53970c257057eafc7` | 6 | 1 / 1 |

For both cases the computed container SHA-256 equals the runner-reported hash,
the ZIP opens successfully, and its member list equals the bounded projection.
Decision/finding cardinality is constant at native size.

The focused subprocess contract also proves:

- deterministic repeated success bytes;
- exact member byte lengths and SHA-256 hashes;
- the sole `--output` artifact is the caller-named structured JSON file;
- missing payload, binding mismatch, producer block, and absent intent write no
  output file;
- invalid wire input preserves the report-package diagnostic code;
- producer-blocked package bytes are withheld.

## Packaged macOS desktop regression

Built command:

```text
npm run tauri --workspace apps/desktop -- build --debug
```

Packaged executable:
`apps/desktop/src-tauri/target/debug/bundle/macos/OpenPipeStress Technical Preview.app/Contents/MacOS/openpipestress-desktop`

- executable SHA-256:
  `b37e49c403985ec9261123ba2da6ff49cf65f0ba3d2e6b66f6d1fe1c12c90fd4`;
- executable size: 41,880,568 bytes;
- fresh mechanics run reached `MECHANICS_SOLVED`;
- no-intent save returned `REPORT-PACKAGE-REDACTION-BLOCKED`;
- native picker cancel returned `REPORT-PACKAGE-SAVE-CANCELLED`, zero bytes,
  `replacement=false`, and durability `not_applicable`;
- new save produced 3,518,303 bytes with container SHA-256
  `e0145a9fb7d377034c7876a92336581ecdeb5d737fe223c5b39d560adfc42cc7`;
- package identity:
  `fb81914e2d9654876eb9421634f1d4ab1e7a14f2fa056b52fe6eb330d19b125a`;
- same-path native replacement returned `replacement=true`, preserved exact
  bytes/hash/identity, and changed inode from `161244912` to `161244949`;
- no `.openpipestress-report-package-*.tmp` residue remained.

The native ZIP passed integrity checking and contained exactly these six
manifest-matched members:

| Member | Bytes | SHA-256 |
|---|---:|---|
| `package_manifest.json` | 3,938 | `fb81914e2d9654876eb9421634f1d4ab1e7a14f2fa056b52fe6eb330d19b125a` |
| `calculation_report.html` | 391,774 | `716a559d4a7180d4d05d0d37acf37186195080b40ab316cfff31d1fbfa3094bb` |
| `calculation_report.pdf` | 982,777 | `e88e7b2e3d9a2a4c9b6e8417c15759d7ae685c67f6dd1820298e281fe80d9d68` |
| `audit_manifest.json` | 1,176 | `80d0b2f19103e3d970bc2b6a58c311ae7cea227b4c61c4b387ca7bcbd594e480` |
| result-export envelope | 1,127,925 | `05bae94667e049fc079da786f03c3312d13d788e4e37c0ea346de1e51662b209` |
| state/comparison/handoff | 1,009,753 | `83d68cc553eb1eb098a674798905702ecd82263c62f13bfe72e083661858a40e` |

One extra new-save package was created during the native picker sequence; it
had the same valid bytes/hash. It is isolated outside the repository. The
packaged desktop test process was terminated after proof completion.
