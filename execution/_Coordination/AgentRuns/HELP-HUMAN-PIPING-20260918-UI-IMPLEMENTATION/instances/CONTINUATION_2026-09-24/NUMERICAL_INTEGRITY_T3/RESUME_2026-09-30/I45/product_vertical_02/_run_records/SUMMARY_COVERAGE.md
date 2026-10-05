# Native sparse summary coverage

Current native adaptive.rs:2127–2172 constructs verification scales with input-
derived/O9 exclusions and original coupling; :2310–2323 defines magnitude as
max(abs(q_2p), scale). :2360 offers a stop tracker only for nonzero magnitude;
:2395 offers an estimate tracker only for nonzero coupled E; :2443 offers a
charge tracker only for nonzero allowance. At p128/256 that allowance is a
positive multiple of coupled E; at p512 it is a positive multiple of magnitude.
verify.rs:formation_scale and :1144–1160 provide the force/moment estimate and
charge rows for the accepted straight source.

final_case::summary_coverage reads the actual matching P=2p retained state's
Wide zero facts, the same source layout/input-derived flags, coupled positivity,
actual p512 floor, and actual resolution facts. It emits only finite Boolean
required-key sets. It does not publish endpoints or recompute a solve. All native
rows have already passed the checked view and required publication checks.
The returned has_data is derived from the actual SourceBridgeView data flags
and block/body map. Neither missing B nor a cancelled net establishes no data.

PP validates exact membership, uniqueness, body/kind and every value limit
against those required sets. Empty zero-data summaries are not replaced with
fabricated zero entries. B presence must equal the actual body data fact.
The original source-selected registry/Arc/source/cache association and the
native view's ledger/prescription/state/data checks remain required.
