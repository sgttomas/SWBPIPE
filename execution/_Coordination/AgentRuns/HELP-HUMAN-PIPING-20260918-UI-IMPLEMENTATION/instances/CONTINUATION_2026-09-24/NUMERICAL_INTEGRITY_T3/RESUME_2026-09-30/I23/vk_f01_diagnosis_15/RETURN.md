# I23 VK-F01 source diagnosis — existing observation route

The already-built normal release `vk_records` can expose the exact recorded per-attempt reasons using its existing `--show=<case id>` option. No new diagnostic code is needed for this distinction. The proposed argument list shows all 30 existing RF-CHAIN records; computation remains the unchanged whole 30-case family, with no parity branch and no `--write`.

`_run_records/COMMAND_PROPOSAL.json` freezes three fresh processes, NONE / VK-F01 / NONE, using the same binary and all 30 explicit show arguments. The current binary SHA256 is `7c477a6bd129a1e2d5dd70c4f7557984615337a155ba8f4b0f0a91f08ddc861e`, matching the completed seeded-faults release build. All 189 source files and their modes match the frozen manifest; relevant sources, oracle, seed, maps, build records and fingerprints are hash-bound in `_run_records/SOURCE_BINDING.json`.

The decisive emitted field is `attempts[*].outcome`, including its full source-defined Debug reason. R7 rejections are distinct from `PublicationEnclosure`; certificate interpretation errors are `Stop(PublicationCertificate {...})`. Both R7 rejection and publication enclosure rejection can end at top-level `Unresolved Ceiling`, explaining why P09's existing output is insufficient. See `_run_records/SOURCE_EXCERPTS.txt` for exact numbered source.

Only the unchanged registered numerical predicate or attributable typed numerical-unavailability discriminator can qualify. Generic certificate-only refusal, work/record drift, an R7 rejection already present under NONE, incomplete reasons or an unrelated failure do not qualify. The whole candidate/verification sequence and matching fresh controls must support the causal interpretation. Existing records do not expose every numeric R7 operand or raw certificate radius; ambiguous attribution remains a gap. No result is inferred.

The example prints failures without enforcing the lane assertions or record equality; exit zero is not a semantic pass. Its stdout contains separate pretty JSON objects plus report text. Retain original streams and inspect emitted records against the unchanged oracle and report pins. No new collector is proposed. ROOT must assign sibling log paths and grant any execution.

P09 remains STOP_UNQUALIFIED with no kill. P10 remains UNRUN. A release observation does not retrospectively reveal debug P09's attempts or replace its pending debug return control. All later runtime stays held pending ROOT disposition.

Actual start: 2026-10-01 18:14:46 UTC. Seven-minute boundary: 18:21:46 UTC. This assignment performs source/metadata reads and additive evidence only; no runtime, source changes, patches/copies, Git/index, tools/framework construction or delegation.

Completed source/evidence preparation: 2026-10-01T18:19:04Z. The exact existing NC-STORED-ASSEMBLY row locators are retained in `_run_records/CASE_ROW_LOCATORS.json`; these are source metadata, not predictions of F01 behavior.
