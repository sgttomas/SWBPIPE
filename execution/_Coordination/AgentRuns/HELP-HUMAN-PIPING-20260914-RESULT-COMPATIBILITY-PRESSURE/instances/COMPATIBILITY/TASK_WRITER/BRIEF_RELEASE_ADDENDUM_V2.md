# Sealed TASK brief release addendum V2

Root released one compatibility TASK writer through `instances/ROOT/_run_records/COMPATIBILITY_SOURCE_RELEASE.json`, SHA256 `68ec3c0be138eaa85b2ec5cd6ebfd8d371af7e04d9075ab6d219700f77ffe31d`. Read it completely. It controls the refinements below and does not mutate the V1 contract, legacy freeze or V1 brief.

The final exact fence is `../WRITE_SCOPE_RELEASED_V2.json`. Its literal base plus additional paths are the only product/test paths authorized. Evidence remains under `TASK_WRITER/**`.

Apply these release refinements:

1. Preserve legacy instance acceptance, projections and existing produced bytes. Version-specific schema copies may adjust only resolver metadata (`$id` and local `$ref`) needed to avoid collisions and make local exact dispatch work. Record that the active compatibility schema is metadata-adjusted; do not call it a byte-identical active copy. A separate strict abbreviated desktop 0.1 compatibility schema is authorized.
2. Name the manifest checksum as a manifest-seed checksum. It is not a hash of final `manifest.json`. The complete package checksum covers the actual final manifest. Every other JSON member hash covers the exact declared member payload. If a member hash is claimed as file-byte equality, write canonical UTF-8 with no added whitespace/newline. Preserve 0.1 materialization bytes.
3. Reject the largest finite binary64 as an unsafe-integral value under the checked profile. Eligible subnormal/exponent values remain supported. The safe-integral bound stays ±9007199254740991.
4. Supply explicit development/verification setup through the authorized narrow build helper and/or test-session conftest. Runtime adapters do not invoke Cargo, search PATH or fall back. Fresh-checkout full sweep setup must be able to build the private adapter explicitly.
5. Author actual dev and distribution Playwright producer tests in the two released files. Do not run WASM, native, browser or GUI builds/tests until the parent relays a serialized root lease.
6. Result export may consume strict 0.2 row/record checksums and the shared designation. Preserve native/model/input-manifest/public-operation wire versions and global canonicalizer behavior.

Return exact pre/post hashes, complete focused pure checks, held commands for the serialized lease, and any adjacent file request. Do not delegate.
