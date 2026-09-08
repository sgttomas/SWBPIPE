# F4-I2 sealed-brief portability amendment

Finding: the originally consumed `children/I2/SEALED_REPAIR_BRIEF.md` contained one absolute implementation-worktree path at line 5.

Disposition: the exact original bytes are serialized losslessly in the DEL successor structural record with original SHA-256 `02eef1f8da9e83fd77009f06a384fadc85fdc09c6c189e35c528dd4036edbca1`. The visible original path now marks that brief superseded. `children/I2/SEALED_REPAIR_BRIEF_V2.md` replaces the host path with `{WORKING_ROOT}` and is the active portable successor.

The child had already consumed the original authorized brief and continued without restart. This amendment changes no objective, source fence, test, model, reasoning level, permission, acceptance condition, or execution result.
