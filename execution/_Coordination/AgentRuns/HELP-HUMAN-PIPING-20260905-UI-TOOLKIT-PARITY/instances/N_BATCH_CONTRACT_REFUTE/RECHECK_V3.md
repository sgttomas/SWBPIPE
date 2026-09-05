# B4 V3 focused recheck

Verdict: PASS_DESIGN_ONLY. Preserved source-shape and renderer-publication residuals are closed at candidate contract level. No remaining design blocker found in this bounded recheck.

Source attribution: V3 requires agent source_ref/source_channel/source_role as nonempty strings and rejects partial/malformed source objects. Optional user source must obey the same shape. Only declared source fields are accepted. This completes the earlier C2/source-shape residual while keeping submitted author/source/rationale unchanged and treating attribution as asserted metadata.

Renderer publication: V3 explicitly requires both an unchanged monotonic model generation and the current authoritative model hash to match the request/response initial hash before publication. Every load/new/undo/redo/edit advances generation. Stale responses produce a diagnostic, no checkpoint and no model overwrite. Successful batch produces one checkpoint. Parent reports N1 accepted ownership of this integration requirement. The guard must be checked at the actual commit boundary, including after any awaited hash recomputation; implementation and race tests must demonstrate that timing.

Initial basis: complete initial_model_hash plus initial_backend_hash remain in the outcome, preserving caller request identity separately from final output hash. V2's allowlisted simulation-only step outcomes and explicit shared preflight remain intact. Internal replay can therefore reuse existing apply_operation over a private evolving model without leaking partial accepted state or replacing original hash/source evidence.

This is candidate-interface acceptance evidence only. It does not authorize source writes, certify implementation/tests, replace the required fresh frozen-source review, or change D58's non-client/runtime status. No source changes, tests, or delegation. Existing initial/V2 records remain immutable. Actual inherited model identifier unavailable. Rerun on design or relevant consumer changes; verify sequential rollback, source/preflight rejection, no nested acceptance, and asynchronous stale-result behavior in implementation.
