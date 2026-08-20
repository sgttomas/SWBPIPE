# Frozen Integrated N1 Diff Manifest V4

Base: `57803893d1eb161f395e0574c256dd27920bf1d4`

This complete pre-review freeze supersedes V3 for CHANGE fan-in. It binds all
28 current tranche paths, including the V3 and Review 03 controls and the
stable conditional manager handoff and return. Review 04 must verify every
identity twice, inspect 100% of their content, check semantic consistency and
containment, and return `PASS` only with zero actionable findings. This
manifest and Review 04 runtime records are after-freeze controls.

| SHA-256 | Path |
|---|---|
| `571cc9ec861d109875977f630ea2f4de67cd49fb098a1a025000bda3ebe514a3` | `projects/chirality-piping/apps/desktop/e2e/gui-workflow-validation.spec.ts` |
| `34bba622b5311f2b89739c23b89eef8555dc27c3d836262a1489c1e911118a2f` | `projects/chirality-piping/docs/validation_manual/index.md` |
| `723628f50aeb17ee2938da82eb643252d3aae9511d7a3ca9819b2cdd7bf3c8b9` | `projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/MEMORY.md` |
| `c971dfb0214d17ff578594e67e0f828f8cb0ac83d3e3b3227210cb92d7cd53db` | `projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_STATUS.md` |
| `5d0ebc2a8e5ff2d429527cd7a5662c90b8ad450935b390a61131d54ce7ea65cb` | `projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/TASK_RUN_2026-08-20_0246.md` |
| `8ae512d5cf97a73e4a04c2d3afe2e571c1fadfbc6a56fe3f53daca40abbdacc8` | `projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-08-20_DEL0904_GUI_WORKFLOW_CLOSURE.md` |
| `e8a309d4239ace33431a9bb95b17697d2664c35eed80b9ce864ed57670f91d93` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/ACTIVATION.md` |
| `401c81f6dcebdbd5ff0c7e6046c081fdc4d5da80a78755df21f0b723d295e787` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/FROZEN_DIFF_MANIFEST.md` |
| `6bf4b1cafc82328790f4fe535b54e788e1fed99e281a08228e31e6be248cc00f` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/FROZEN_DIFF_MANIFEST_V2.md` |
| `0d2b410c40e9482062da8fc8845fc79ef083f24489326f489478ba383437822c` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/FROZEN_DIFF_MANIFEST_V3.md` |
| `8e580a67e1b54fa14d222338f24e639b19a9236e918d194f4da6e0fbb07a2fdf` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/HANDOFF_STATE.md` |
| `f8b55e46adf613b6866c1380270a0cf3471b841fb5519ea254cd4162a16ea8d2` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/MANAGER_REGISTERED_CHECKS.json` |
| `a17b8bab0b5580537f9a164d70ab1a2d1814529c85ad1e9aef9ffb7778098aed` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/REMEDIATION.md` |
| `708d16ef5f1a0084eadb2f151a42235bbd1f431817b87ab932f1bfe3b8fa0e71` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/RETURN.md` |
| `038ee295bca5bb32ed1aa4e66f00462b8bb5b9b8c00d24e8664777926a72a1aa` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/WORK_GRAPH.md` |
| `d8487a11846b807703941de4ff41cd9599adb88cc015d9583380e5f5d38e5ed5` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/IMPLEMENTATION/LAUNCH_BRIEF.md` |
| `e3a0595f53a30ec37e829510e2c0dee7df68ef375964ecb2c05538e2f5eba2af` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/IMPLEMENTATION/REGISTERED_CHECKS.json` |
| `72e2bf2631440459dbeb005a42d2bb8c487be3ca4b5d2c467543a937d0b9a567` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/IMPLEMENTATION/RETURN.md` |
| `d2cab489e4e22a2c39b2996470eab5874f1006c7a5875c3d98929bdc4f6e7f0b` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/IMPLEMENTATION/STATUS.json` |
| `0c60f73e06d091718d0702ba6bd511eb4b11e1ec62e72108d83cdc4a39414ac2` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/REVIEW/LAUNCH_BRIEF.md` |
| `b891c1b11aa3722fb9a1cd3de502edc9bc2866c34362e59567537fbca8ee57e7` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/REVIEW/RETURN.md` |
| `ba54b092c91c8de62306dd0d61ee13ed0440cf1e989211ba480005a2bed0d9a9` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/REVIEW/STATUS.json` |
| `994af85332cf54cf4598003387e14803e2559ab39a0157b35298654b8c9cbc77` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/REVIEW_02/LAUNCH_BRIEF.md` |
| `1d47dd2bef860bf72ea5eeb4c25460e5104ae10e50df9c287fe1bf8d7ed79ec7` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/REVIEW_02/RETURN.md` |
| `1cbf94afbb1b46eb0381225cf2d8532a7120daf575f13f648b6459bf1cd7eca8` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/REVIEW_02/STATUS.json` |
| `c33ef45f04c667b1236fd7362c7be8b97bf1f41b6eb1049eb15c2d6d3c243303` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/REVIEW_03/LAUNCH_BRIEF.md` |
| `cdf653ec13b413b87ef86f3ef37de8dc83d89052cc88b656cbed81ecd958da4a` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/REVIEW_03/RETURN.md` |
| `3c3b328f95cb96e5c1dd1397cb74de7a37f431150913ee900786efd6f737cd7c` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE/instances/REVIEW_03/STATUS.json` |
