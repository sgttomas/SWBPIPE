# B4 V2 focused recheck

Verdict: REVISE for one remaining integration contract and one attribution-shape clarification. Initial refutation report remains immutable.

C1 CLOSED at design level: explicit allowlisted simulation projection removes per-step acceptance/application flags, model, intermediate model hashes, audit and professional-boundary objects. Top-level result alone owns final acceptance; submitted metadata is explicitly untrusted.

C2 MOSTLY CLOSED at design level: shared backend preflight now explicitly validates author/status/audit/professional controls before replay and offline import. Source preservation is explicit. Residual: EditorOperationSource currently requires source_ref/source_channel/source_role, while V2 only requires source_ref. Declare exact agent source shape: require all three nonempty strings, or define and label the deliberate import-adapter subset. Do not report complete EditorOperationIntent schema validation for malformed or partially typed source fields. No broad canonical schema redesign is requested.

C3 OPEN: specify how UI publication is bound to the same current session snapshot after asynchronous backend return. Require a captured session-generation guard (or equivalent current complete model hash comparison) before any receipt/checkpoint/model publication, reject stale completion with zero changes, and test model replacement/undo during in-flight batch. Alternatively hold every model-changing entry point during apply, not just the apply button. This is a PKG07 integration dependency; Parent may assign it there, but it must be explicit before the integrated interface is accepted.

Core replay remains sound: immutable input plus private model chain and original complete hash preflight allow reusing the existing applier without exposing partially accepted state. The remaining issues do not require a second mutation engine. Hash/source notes in initial RETURN.md remain implementation acceptance requirements, especially preserving the submitted initial claim and not silently rebinding a declared imported basis.

No source edits/tests/delegation. Actual inherited model unavailable. Recheck scope is V2 against initial findings only, not implementation acceptance or D58 adoption.
