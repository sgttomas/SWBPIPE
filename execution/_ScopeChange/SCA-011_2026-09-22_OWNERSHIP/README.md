# SCA-011 historical helper sources: not runnable

The 22 Python files under this directory are accepted historical sources, kept
byte-identical as fidelity evidence for this scope change. `ACCEPTED_MANIFEST.csv`
binds their bytes, and the frozen and applied review manifests bind them too.
They are not runnable and must not be invoked, imported, path-loaded or read and
compiled, directly or through one another. They depend on former monorepo tools
and on the historical states they were written against.

`tools/validation/execution_boundary.py` holds their exact paths and SHA256
values as fidelity-only exceptions: a changed byte, mode or link fails it, and
it rejects any maintained code that would execute them. Any future replay is a
separate undertaking outside `execution/`, with explicitly named historical
inputs.
