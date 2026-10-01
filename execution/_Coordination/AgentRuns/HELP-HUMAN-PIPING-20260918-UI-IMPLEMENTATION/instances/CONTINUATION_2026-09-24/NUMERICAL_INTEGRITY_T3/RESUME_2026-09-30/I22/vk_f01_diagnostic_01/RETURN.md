# F01 release diagnostic — completed for ROOT disposition

Actual block: 2026-10-01 18:33:35–18:43:35 UTC. All three authorized fresh processes launched by18:35:48.208Z and completed before the18:37:31 final check; no build or extra process. Existing normal release binary7c477a6bd129a1e2d5dd70c4f7557984615337a155ba8f4b0f0a91f08ddc861e, opt3/test=false/seeded-faults. Exact commands, explicit faults, prelaunch identity/guard checks, PTY/tool exits and /usr/bin/time -l resources are retained.

| Process | Fault | Launch UTC | Exit | Real seconds | Result |
|---|---|---|---:|---:|---|
| D01_NONE_BEFORE | NONE | 18:34:13.694 | 0 | 0.57 | All30 complete records and fixed report pins match |
| D02_F01 | VK-F01 | 18:34:53.579 | 0 | 0.47 | All30 records retained; new persistent named R7 rejections |
| D03_NONE_RETURN | NONE | 18:35:48.208 | 0 | 0.17 | All30 complete records restored exactly |

Both NONE observations match every complete frozen rf_chain record, including schema, source ID, work/storage and report fields. Both match the fixed2,760 rows/2,700 passes/60 structural zeros/zero failures, all30 selected at128, and82 discriminating/38 non-discriminating controls with empty unexpected lists. JSON analysis retained exact integers and strings; no fingerprint or large integer was round-tripped through JavaScript.

F01 introduced the following R7 rejections at every candidate128/256/512, followed by a solved1024 verification and Ceiling. NONE had128 Accepted and256 Verified for every case:
- All15 torsion cases: StopRule on free root node0 Rx, body0, Rotation.
- Thirteen axial cases: StopRule on free root node0 Ux, body0, Translation.
- RF-CHAIN-A-n03-r1e-04 and r1e-06: VerificationEstimate on root-incident member1, endI, Ux, body0, Force.

CASE_ATTRIBUTION.json binds every case, precision/role/reason, source ID, root spring/free component, original reference row and root-incident member. Historical NC-STORED-ASSEMBLY locator flags remain source metadata, not predictions or new expectations. All30 were examined, including historically non-discriminating controls. No certificate, terminal, budget, source/geometry, or generic VerificationFailed reason appears in these sequences.

The causal interpretation uses the controlled same-source/same-binary/same-argv triplet, restoration of complete NONE records, the source F01 branch that rounds assembled entries then lifts (assemble.rs738–744), the exact layout-mapped R7 reason (adaptive.rs3746–3776), and the R7-false escalation branch before certificate admission (4137–4164). These new, persistent row-specific rejections explain the loss of selection across the full schedule and support the reviewed stored-assembly numerical-unavailability route. This is source-linked release evidence submitted for ROOT's disposition, not automatic fault credit. Missing private numerical operands/radii are not invented.

Original debug P09 remains STOP_UNQUALIFIED. The release records do not retrospectively reveal its attempts. P10 and all later original schedule processes remain UNRUN. No runtime continuation follows.

The native return-control clarification was received after D03 had already run under the original grant, following complete normally returned and attributable D02 records. The committed clarification was received/read18:37:50 UTC and is preserved separately; it does not relabel D03's actual authority or time.

Final check18:37:31 verified all189 source bytes/modes, normal-release binary/raw fingerprint/dependencies and all52 prior runtime01 sealed payloads unchanged. Guard5387 remains live; no owned process remains. No command approached the five-minute checkpoint. No automatic deadline or RSS cap is claimed. Writes are confined to this additive packet and six sibling raw streams in `<wt>/scratch/i23/vk_prep/logs/f01_diagnostic_01`. No source/test/oracle/observation/Git/index edits, new collector/driver, build, delegation or extra model occurred.

Complete raw streams, ordinary decoded records, reports and exact raw hashes are retained. One analysis display was truncated; a compact read of all30 existing records recovered the full case/precision/reason review without a runtime repeat. SHA256SUMS excludes itself. No A1, E_max, W1 or F2a acceptance is claimed.
