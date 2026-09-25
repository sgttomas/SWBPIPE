# Explicit historical constructor version guard — narrow review delta

The explicit v0.2 historical constructor previously checked precision-header
presence but did not check its received source schema version. Thus a header-free
future version could enter historical row interpretation, while default dispatch
correctly rejected it. This is the source-level P2 finding relayed by ROOT; this
TASK did not execute a baseline probe.

The v0.2 branch now requires an actual string exactly0.1.0 or0.2.0 before row
interpretation or hashing. Unknown, missing and nonstring values raise
ANALYSIS_LEGACY_SOURCE_VERSION_UNSUPPORTED. The existing header-presence downgrade
guard stays first and unchanged: producer, numerical_quality or formulation_basis
is forbidden even when its value is null, false or empty. Default dispatch,
precision/Current gates, schemas and semantic tables are unchanged.

SOURCE_FREEZE.json/SOURCE.diff pin only compatibility.py and its existing test
file relative to REPO_ROOT (the clean numerical checkout). The prior110-file
freeze/diff is untouched. Only the two needed source preimages are copied under
_run_records; no full-source after copies or alternate corpus was created.

## Prepared focused controls

The new test names share the selector explicit_legacy_constructor:

- rejects_unknown_or_nonstring_source_version: future/unknown/malformed strings,
  null, boolean, integers/floats and list/object values;
- rejects_missing_source_version;
- rejects_header_presence_even_when_empty: both known versions, each reserved
  header, with null/false/empty-object/empty-list values;
- refuses_actual_current_precision_fixture: actual sparse and dense raw0.2
  producer fixtures cannot be downgraded, and their source/file bytes stay intact;
- known_versions_preserve_real_hashes_and_semantics: both supported historical
  versions use the real checked canonical adapter for received/row/record hashes,
  verify the record, preserve source bytes and historical semantics, and keep the
  known0.2 language-neutral fixture byte-value record identity. Distinct source
  versions retain distinct received/record hashes rather than being relabelled.

No hash_fn is injected. The positive control requires the actual
openpipestress_jcs_ijson executable through the existing adapter protocol1.0.0 /
openpipestress_jcs_ijson_v1. The runtime never builds it or falls back to Python.
Manager should set OPENPIPESTRESS_CHECKED_JSON_BIN to its qualified existing
checked-JSON binary (or use the adapter's explicit built default under
core/serialization/canonical_json/target/checked-json/release). If unavailable,
the existing tools/serialization/build_checked_json.py is the explicit build
entrypoint, to run only under a separately scheduled manager build lane.

Focused command from WORKING_ROOT, under that adapter environment:
`python -m pytest tests/test_analysis_run_compatibility.py -q -k explicit_legacy_constructor`
Then run the complete same test file and any owning full-review checks. No tests,
Python runtime probe, canonical adapter call, Cargo/build or Git operation was
performed here. No passing or compatibility-hash preservation result is asserted
until those real-adapter checks run.

Actual author: TASK /root/solver_manager/evidence_primitives under
/root/solver_manager, native followup allocation Astra/low; no descendants.
