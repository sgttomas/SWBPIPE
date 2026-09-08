# P5-A2-CHECK launch brief — sealed V1

RequestedBy: `/root/pressure_reference` (`WORKING_ITEMS` P5). RunID: `HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION`. ParentInstanceID: `P5`. ChildInstanceID: `P5-A2-CHECK`; actual task identity `/root/pressure_reference/p5_pressure_check`. Role: bounded ephemeral Agent 2 generalist; no delegation. Delegation class: delegated-harness-native; role/non-delegation are instruction+config asserted where the harness does not expose mechanical proof.

Runtime configuration: model `gpt-5.6-sol`, reasoning `high`, `fork_turns=none`. No model exception. This file is the durable exact-scope mirror of the launch prompt; the in-memory sealed prompt was issued before execution and this mirror was persisted immediately after dispatch.

PackageID: `PKG-05`. DeliverableID: `DEL-05-03`. ScopePath: `{WORKING_ROOT}/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module`.

Objective: independently derive and check four pressure-reference cases — free closed, axially restrained closed, separately supported closures/open pipe, and thermal plus pressure — with explicit wall/effective force, endpoint nodal-action versus section-cut signs, cap-area versus mean-radius thin-wall conventions, thick-wall Lamé cross-check, Poisson/material needs, and support load paths. Derive numerical reference values for the existing `Do=.168 m, t=.007 m, L=2 m, E=200 GPa, p=1 MPa` geometry using `nu=.3` only as an explicit candidate assumption. Freeze expectations before inspecting current production output; compare only afterward and preserve manager-selected Option C plus the prior child’s Option A as a limited alternative.

AcceptedBasis: source `779dedb8670625b36af07b89fc5557470e47c50e`; decomposition revision 0.12 / SCA-009 / DAG-010; Receipt 135; frozen parent activation `instances/P5/ACTIVATION_V1.md` and work graph `instances/P5/WORK_GRAPH_V1.json`; accepted E1 pressure preparation for downstream comparison only.

DeclaredReads: root `AGENTS.md`; project `AGENTS.md`; `agents/AGENT_TASK.md`; P5 activation; relevant live source, accepted E1 pressure packet, and primary authoritative engineering publications retrieved directly from publishers. The `domains/piping-design` OCR/equation corpus is prohibited.

AllowedTools: read-only repository and Git inspection, web search/read, and deterministic arithmetic. AllowedWriteTargets: NONE. Dependencies: none. EXCLUSIONS: no file writes, production solve/build, source/test change, threshold, output migration, authority/register/status/pointer edit, external message, sibling message, or delegation.

ExpectedOutputs: a concise terminal return to the manager containing equations, sign conventions, numerical matrix, source URLs/applicability, recommendation, unresolved Owner choices, limitations, actual identity/configuration, non-delegation evidence, and descendant count.

AcceptanceCriteria: all four cases covered; expectations explicitly frozen before current-source comparison; primary sources are publisher-hosted and applicability is limited; force/reference/sign conventions are internally consistent; `nu=.3` remains candidate; no claim of project adoption or blanket pressure closure.

Escalation: any physical adoption, public compatibility choice, cross-package interface change, missing authoritative basis, or scope expansion returns to the manager.
