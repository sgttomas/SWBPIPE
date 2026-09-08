# Publication evidence repair — sealed launch brief V1

**Status:** `SEALED_EXECUTION_HELD_PENDING_FINAL_RI_AND_ROOT_RELEASE`
**Parent:** `/root` (`HELP_HUMAN`, Agent 0)
**Construction:** fresh bounded ephemeral Agent 2 generalist assigned publication-evidence repair
**Required model:** `gpt-5.6-sol`, high reasoning, fresh context
**Delegation:** prohibited

## Objective and authority

Repair exactly the 40 evidence files below so the repository's actual path-anchor and candidate-whitespace validators pass while preserving every original byte string and its historical hash. This is representation and portability repair only. Do not change source, tests, validators, `.gitattributes`, policy, manifests or reviews predating this repair, backend or bundle files, Git state, or acceptance criteria. No waiver is authorized.

Execution remains held until final RI terminates and root explicitly releases this brief. When released, write only:

- the correction packet under `instances/PUBLICATION_EVIDENCE_REPAIR/**`;
- the 40 exact logical paths enumerated below; and
- new lossless archive records under each logical file's owning `_run_records/**`.

## Frozen 40-file inventory

The hashes are the current pre-repair SHA-256 values found by read-only runs of `tools/validation/validate_path_anchors.py --json .` and `tools/validation/validate_candidate_whitespace.py --repo-root .`.

### Eight host-path files

```text
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/NATIVE/successors/INVENTED-PROVENANCE-R2/POSTRUN_GUARD_CHECKS.json  2665cc6c4b90873fb5c6e3f7f09796f59c8c587baaa9b5ce4ba849f49abd46cc
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/NATIVE/successors/INVENTED-PROVENANCE-R2/STORE_POST_CLEANUP.json  22a4832a383774a914b71b5997e4ed3bf41891ea1e56de5a89044aaa992a503e
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/NATIVE/successors/INVENTED-PROVENANCE-R2/STORE_PRE_CLEANUP.json  bd35ec1b24c2e0c95c498bc551b01ce6f2b08423a12ee8acdda79d931141af23
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/children/I1/REPAIR_BRIEF_V1.md  61d20df1a043382e57ab6a55bf5e6d5c5a0cb3bca53894586eda98f1eabb99b0
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/children/I1/REPAIR_BRIEF_V2_DRAFT.md  52c6330a87e4266c3c488f979fc88adf383192f2a5a0a1b558b62aeb75c86af3
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/children/I1/REPAIR_BRIEF_V3_DRAFT.md  d2605540a41178c2d8674525e8653fa40ab1c642d80f881182a52baf29deaa9e
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/children/I1/REPAIR_BRIEF_V4.md  5f8b1a712508ffb362eec3c33c5d008dd5fdf8475800014a1c79412cba596bfe
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/children/I1/REPAIR_BRIEF_V5.md  3732433d8673b6136ce6592c2b12ad3c6ed2cb91263a4322688bdb5bb4968856
```

### Thirty-two whitespace files

```text
projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/implementation/CANDIDATE_DIFF.patch  8080a8c242931409863771aba963897a72b055c39aee1d61fd567eee4bb3801b
projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/successors/RF-F4-001/CANDIDATE_DIFF_BASE_TO_SUCCESSOR.patch  ff74ab001f108f36a6e0f4105b4fe40a7a7d784045e4fc0827518074a62d97cc
projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/successors/RF-F4-001/CANDIDATE_DIFF_V1_TO_SUCCESSOR.patch  9bdf1403176e697534c4a373250d4f81930336065b13d7ee79a618e21aa1d09e
projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/U7_FINAL_NINE_PATH.diff  df93c21cb945f6ed5d3e42ecbfd8f4f25ef462ba5c453d76757b36d2b2fa2056
projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/U7_R2_TO_R3.delta  a0ae4a385ed9b76d8fa10974df218525b555732e333d4c8964ac76714b3a21fb
projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/U7_R3_FULL_BASE.diff  6e013cf88417d02c1f1f75d48fac124ba635f8e39b974c6d2cda2477b01b6b10
projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/U7_SUCCESSOR_FULL_BASE.diff  8c01ab323ef27836ecb1e777366b9166adfe28f9948bd990b01441fc2302f30e
projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/U7_V1_TO_SUCCESSOR.delta  09881c4a563bd919a52bd855fc954c798565be9d8817dabfbf36a32793f83863
projects/chirality-piping/execution/_Change/PHYSICS_UI_IMPLEMENTATION_20260908/_run_records/NATIVE_BUILD_V1.log  fd3d048cf7f954d4089eaa32b1a8c6d6db3639293d7916aa571a8bc382403a98
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RF/successors/RF-F4-001-BACKCHECK/_run_records/focused_exact_zero.stdout  3d23fd32a87dc338c2f13a27d6dd381bf4dc9146490b4397b0edf0ab639c7185
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RF/successors/RF-F4-001-BACKCHECK/_run_records/focused_signed_zero.stdout  89f29699ba24bfb312810f563818af26ca34db74cdfffe835b0ee330b65e0e77
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RF/successors/RF-F4-001-BACKCHECK/_run_records/full_crate.stdout  da043ab8840fce49c1c498cbb2b1c885f981ce6796abac69ca1374cc5fa2641a
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RF/successors/RF-F4-001-BACKCHECK/_run_records/rational_witness.stdout  b4f1982a5046d72ff2b8aec16e49be658619af76c1c7ad2ea1fd82bc46e359df
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/_run_records/App-targeted-vitest.txt  b66d79ed37b2ee38f3413a23615d9e43f4464f806a2779767e5320f5b1b6aad8
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/_run_records/repros/vitest-output-attempt2.txt  c3216b46dd3f3d1a0654eedebefdeec16006a34e48ce29304f17d17a50b7431d
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/_run_records/repros/vitest-output.txt  00c9c85ec1269d0f462e5501e6714ee0e904065a11d7f2a87a56dceba8af7ad0
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/_run_records/routeDraft-vitest.txt  e8024e9a37ee3c29c815d4a24ebec3147b56f6d869b0bdf3b59ba0bfc054714d
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/_run_records/typedInspector-vitest.txt  6c50d6f1a36a16b40954af3398a1e1600f834ae77a5a08e3b7c6f1e2ded49155
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-F1-F5-BACKCHECK/_run_records/App-focused-vitest.txt  f8cf12c9cf9e22957129508382d04298e3ba16bd40501c570e4965991d8f2e5b
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-F1-F5-BACKCHECK/_run_records/App-persistence-vitest.txt  bc2a07ec173b9096ca5d355b7d96a12e9774bfbad6801de6b55c24f765c698e7
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-F1-F5-BACKCHECK/_run_records/repros/vitest-output-confirmation.txt  fda1aec191c2f2a426500066e445ccbd471ece4ae75961fd1af15a5aa2ea84df
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-F1-F5-BACKCHECK/_run_records/repros/vitest-output.txt  dfc44f9a0e78fc1228fbc5593da83c3bb762f4757608adfeeb5189261d500982
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-F1-F5-BACKCHECK/_run_records/routeDraft-vitest.txt  f47ce9580176183e0eacd1550caffcd401bea705b84982f3d3b3fd67546618f2
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-F1-F5-BACKCHECK/_run_records/typedInspector-vitest.txt  c1d1993e58603c5bc0fe84c2f3b9d15785f9b43e6ea492cfcc0d176fd58fb5b1
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-R3-BACKCHECK/_run_records/App-focused-vitest.txt  47ae9dc5465c2597b041b12edf5c0fbac330055228bde1579f7adb7a7ca7bc65
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-R3-BACKCHECK/_run_records/repros/vitest-output.txt  47d31517d5ce0295d9bf94a79e74a9093656622f9bf928e705a6459e8e6a2b5c
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-R3-BACKCHECK/_run_records/routeDraft-vitest.txt  39be4d5a700dcd591176e72ece9836ae7666d796456fe5125f90c69756b5c727
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-R3-BACKCHECK/_run_records/typedInspector-vitest.txt  33a3c51ecd9775415ff83ebed99225810b058174126d0c056f1b83e8be651e72
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/ACTIVATION_V1.md  ee804e6aafc13cf40c8b9f8c88d6b0ef95254563197f94468c2bf4e6a6279b91
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/children/I1/LAUNCH_BRIEF.md  2790a1803e4a5af1118bf7d18f73b17cecbe50d2f97c2d998dc45c29b99de2eb
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/children/I1/REPAIR_RETURN_V2.md  0aef9be19b342f376266672df88481f45641d7111287fd6588125ef01fe51af9
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/children/I1/REPAIR_STATUS_V2.md  1083875a4f2b83d0022d85e149c872017cf2a66c6b8bcc34e26d3601f2f1d953
```

The first eight entries in the whitespace list are the complete patch/diff/delta set.

## Required repair method

1. Before changing any logical file, verify its current hash against this brief. Stop on any mismatch.
2. Preserve every exact original as base64 in a JSON record under the owning `_run_records/**`. Each archive JSON must have one final LF and record logical path, original SHA-256, byte length, base64 encoding, decoded SHA-256, and archive purpose. Decode and rehash every archive before continuing.
3. In the eight host-path files, replace machine-local host paths with locally declared portable anchors only. Use `REPO_ROOT={git rev-parse --show-toplevel}`, `WORKING_ROOT={REPO_ROOT}/projects/chirality-piping`, and `USER_APPLICATION_SUPPORT` resolved by the consuming platform. Do not change the represented executable, store identifier, values, or evidence meaning.
4. For the eight patch/diff/delta files, replace the active logical file with a concise portable pointer to its lossless encoded archive. State explicitly that the active representation is a pointer and is not an applicable patch. Do not trim the patch bytes into a syntactically invalid or semantically altered patch.
5. For raw `.log`, `.stdout`, and test-output `.txt` records, remove only surplus blank EOF lines after lossless archival. Record in the correction manifest that the active representation is an EOF-normalized view of raw output. Preserve all non-EOF bytes and failure text.
6. For the remaining nonraw whitespace files, remove only surplus blank EOF lines after lossless archival. No prose or data edit is allowed.
7. Emit one additive correction manifest under `instances/PUBLICATION_EVIDENCE_REPAIR/**`. For every logical file record its original path/hash/length, archive path/hash and decoded hash, successor path/hash/length, transformation class, semantic-equivalence status, and historical-resolution rule. Historical manifests and reviews continue to resolve an old logical member/hash to the decoded archive bytes; the current logical path resolves to the successor hash. Do not claim an older manifest validates directly against a changed active path.

## Verification and return

Rehash the same ten reviewed source members against source aggregate `1180436540fcb8025a5029b16f8d6e019928eeaa6fab74c434772ca6ec78cefe` and the same three bundle members against canonical bundle digest `d54e06c6229163ad27bfb3d3ad27b1383b6cb50383d70316b0b6c83e71bd0b11`; all must remain unchanged. Run both validators exactly as named above and require `PASS`. Validate every JSON, LF ending, absence of surplus EOF blank lines, all 40 successor hashes, every archive hash, and every decoded original hash. Confirm the correction packet and its active references contain no machine-local home paths.

Do not run Git mutations, product tests, builds, full harnesses, native applications, or store access. Return a concise `RETURN.md`, `STATUS.json`, validation evidence, exact 40-file inventory, and correction manifest under the assigned instance root. Report `PASS`, `CHANGES_REQUIRED`, or `BLOCKED`; preserve failures. Root and CHANGE own acceptance and publication backcheck.
