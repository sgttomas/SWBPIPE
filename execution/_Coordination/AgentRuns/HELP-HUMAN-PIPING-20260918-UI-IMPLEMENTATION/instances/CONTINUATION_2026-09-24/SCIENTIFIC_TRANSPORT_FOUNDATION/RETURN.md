# Scientific transport foundation return

Implemented standalone `openpipestress_jcs_binary64_v1` on reviewed base0a438.
The ten owned source paths are frozen in [SOURCE_FREEZE_04.json](SOURCE_FREEZE_04.json).
Independent complete-candidate review is clear after repair and backcheck of
two related resource defects; see [the independent return](INDEPENDENT_REVIEW/RETURN.md). No Git mutation or product
activation was performed. This return is an implementation handoff, not solver,
WASM/native-application, Current/export/rule or release qualification.

The new Rust authority parses original tokens once to binary64, applies strict
Unicode/duplicate/negative-zero/overflow/nonzero-underflow policy, and preserves
numeric lexemes for explicit schema-owned exact-integer checks before projection.
Safe integer DTO variants are retained while canonical numbers uniformly use
ECMAScript formatting. A separate CLI/protocol profile and explicit Python
text/snapshot helpers preserve the old paths without fallback. New ingress has
bounded document/depth/node/token/path/envelope/batch limits; a cumulative path
construction limit addresses long-key metadata amplification before allocation.

Executed checks, on the frozen source, are retained under `_run_records`:

- `rust_full_04`:24 passed,0 failed/ignored; thirteen new tests plus11 historical.
  Includes all24 finite RFC8785 formatter vectors (negative zero is historical
  rendering only), genuine new-profile admissions/rejections, lexical counters,
  duplicate/Unicode/syntax controls and resource boundaries.
- `python_cross_language_04`:55 passed,0 failed/skipped; includes actual Rust CLI
  calls, strict Python snapshots, old fixture bytes/hashes, real CLI invalid UTF-8
  and profile rejection, plus547 deterministic numeric tokens with matching
  canonical text/bits across Rust native CLI, Python3.13 and Node24.18/V8.
  Fixed seed, midpoint generation and inputs are in the maintained test source.
  Mocked response-failure checks establish adapter refusal only, not Rust behavior.
- Explicit scientific CLI build and Git diff whitespace check passed. The old
  checked CLI/lib also compiled during pytest's existing setup hook; that
  unexpected setup and original cache-permission failure are retained honestly.
  Sixteen initial pretransport tests are a subset, not an additional55.

[RESOURCE_REPAIR.md](RESOURCE_REPAIR.md) preserves the measured finding and
preallocation repair; the first candidate and all earlier results remain.

The old library function bodies, complete old Python helper prefix, old CLI,
Cargo.lock and historical Rust tests remain byte-identical. The build script's
legacy default still selects the old executable; the scientific profile requires
explicit selection. No material/component library or code-rule content is added.

The final six-file design basis, profile selection and two independent review
layers are recoverable at commit `a09931a74e197627105c1287c03957317a297351` on
`codex/piping-pipes-20260924`, with unchanged verified hashes pinned in
[BASIS.json](BASIS.json). A Rust TASK dispatch hit the host thread limit, so no
child executed; the manager implemented locally. ROOT assigned independent
nonimplementing review, now clear for bounded manager fan-in. Actual resource use/commands/versions/binary hashes and
source hashes are in the logs and [ENVIRONMENT_04.json](_run_records/ENVIRONMENT_04.json).

Adoption remains separate: each typed schema must inventory exact count fields;
actual text boundaries must run admission before ordinary parsing; browser strict
snapshot and native/WASM exports need explicit integration; successor invocation
receipts must separate raw text custody from canonical semantic identity and bind
actual mode/model; output-zero projection and SI stress/reader/persistence paths
need new semantics. A serializer pass does not repair numerical accuracy, stored
unit projection, existing Sensitive standing or the original1e160 product refusal.
