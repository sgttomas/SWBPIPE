# F4 blank-EOF formatting change mapping V1

Status: `COMPLETE_EVIDENCE_ONLY`
Scope: five named uncommitted F4 artifacts only
Source/product effect: none

Each normalized file had exactly two terminal LF bytes. The successor removes one blank terminal line and retains exactly one terminal LF. No non-EOF byte changed.

| Portable path | Original bytes / SHA256 | Successor bytes / SHA256 |
|---|---|---|
| `{DELIVERABLE_RUN_RECORD}/CURRENT_NORMAL_EXPECTATIONS_V1.md` | `395` / `fb82b32ea9b79b7c41d3e9c2527ba63e506718b1d0219ac3fad13c8a0326a6d6` | `394` / `540bbe0c408478e3dec6140f647e82531e3a9c50110e7b9e58417ae03ee2bd02` |
| `ACTIVATION.md` | `2611` / `19aca1a0804a5cdd7cedd51c3b3e2e91dae19ff7ee11fa7087c2e796ab532abb` | `2610` / `5bdcf28f0b32405eb04975a6caa816f4b6d41971a24ba9b304ce7e2957b8a866` |
| `CURRENT_NORMAL_EXPECTATIONS_V1.md` | `1722` / `4de46610000a90b9be3166dace956535613f519416a59f394a6095c6324225ae` | `1721` / `cf250869bc6d531a02b8a009d1a2b864ce842d87dacc93b614e911f9c9cf69c7` |
| `WORK_GRAPH_V1.json` | `1556` / `ee459b57655eeb99b0b9d2021a035e7a0bc0e0b45f8af0b14330760447fb9890` | `1555` / `23b0c29005d5585d6e7fd986cc0298467e50d76c555647a2e59871b0c968a7e3` |
| `children/D1/LAUNCH_BRIEF.md` | `2959` / `9c8d4f45f03fe496e8f21d0dda7da4130a99dbd0eafdc64450b826a73dcbb3d1` | `2958` / `3510e58d5f82ad9e2c03f9e319910588f902682ca8c805ff8521898fa2baa8ab` |

The exact pre-normalization bytes are serialized as base64 in `{DELIVERABLE_RUN_RECORD}/formatting_cleanup/SERIALIZED_ORIGINALS_V1.json`, SHA256 `422272f82746a18e034d646042a2df38f21d3d9a986d0d222f54099e5dbd0c7b`. Each decoded member reproduces its recorded original byte count and SHA256.

The existing `INITIAL_PHASE_MANIFEST.json` remains byte-preserved at SHA256 `3366721434a079c00b4df6c0bba5fbd24cba2e0f005b60bf752e530c5cd391fa`; the successor manifest maps its three changed members and the two additional normalized artifacts without rewriting prior evidence.

Structural run evidence beneath `{DELIVERABLE_RUN_RECORD}/diagnosis/**`, including raw logs, the TASK run record, exact commands, and execution records, remains evidence-class provenance and may retain exact host paths. This cleanup did not rewrite or normalize those records.
