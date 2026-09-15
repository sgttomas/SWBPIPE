# UI writer checkpoint 1 evidence

This package binds the checkpoint 1 qualification evidence to source commit
`0121e1ba7acd2b0e8429f07846042cd016c03aea`, whose parent is activation commit
`107911e35e7f26123ab283ed33b8bcfb936c026b`.

`SOURCE_HASHES.sha256` records the 12 exact source-file hashes. Its complete
stream has SHA-256
`e1fcfdbe31d9d21ce964c956b4f8509c09e39472887a5b882ba1c22bf429d637`.
`RUNTIME_RESULTS.txt` is the retained checkpoint transcript copied from
`/tmp/ui-checkpoint1-runtime-results.txt`. The temporary jsdom-only Vitest
configuration is retained as `vitest.checkpoint1.config.ts`.

The authorized sequence ran from `projects/chirality-piping`:

1. `npm ci --ignore-scripts` — exit 0.
2. `npm exec --workspace apps/desktop tsc -- -b --pretty false` — initial exit
   1 for a missing local `midpoint`, then exit 0 after repair, and final exit 0
   after the selection repair.
3. The registered five-file Vitest command — exit 1 before loading tests
   because the required prebuilt WASM asset was absent. No WASM build ran.
4. The first temporary-config invocation — exit 1 because the config import
   could not resolve from `/tmp`.
5. The corrected jsdom-only invocation — 18/19 tests, then 19/19 tests after
   the repeat-singleton selection repair.
6. `git diff --check` — exit 0.

The final supporting results were TypeScript exit 0 and five focused files / 19
tests passing under the retained temporary config. The registered setup result
remains explicitly limited by `WASM-ENGINE-ASSET-ABSENT`; browser, build,
native, and broad test execution were outside the lease. The lease was
explicitly released before the checkpoint commit.

`SHA256SUMS` covers every other file in this directory. This is the sole
canonical checkpoint 1 qualification package.
