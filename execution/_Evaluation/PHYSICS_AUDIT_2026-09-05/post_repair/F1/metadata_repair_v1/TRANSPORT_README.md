# Metadata repair evidence transport

MANIFEST.json remains the exact original30-member logical manifest (SHA3fc67be3aa81585a58986f55ecb090527f64220ee3dda20ced538cd156eb9058). Only physical storage of FROZEN_DIFF.patch changed: its single-space unified-diff context lines are required original patch bytes but trigger repository whitespace checks. FROZEN_DIFF.patch.transport.json stores those exact bytes as base64, with original SHA and size. TRANSPORT_MAPPING.json resolves the logical member to this physical wrapper. All other original logical members remain at their original paths and unchanged.

To materialize without writing repository evidence, run from this packet directory:

```python
from pathlib import Path
import base64, hashlib, json
packet = json.loads(Path("FROZEN_DIFF.patch.transport.json").read_text())
raw = base64.b64decode(packet["data"], validate=True)
assert len(raw) == packet["bytes"]
assert hashlib.sha256(raw).hexdigest() == packet["sha256"]
Path("/tmp/piping-f1-metadata-review.patch").write_bytes(raw)
```

PHYSICAL_TRANSPORT_MANIFEST.json binds the physical files. This is lossless evidence packaging only: no content, numerical, source or test edits; no repeated behavioral tests or whitespace waivers. Original logical manifest remains authoritative for its evidence snapshot; transport mapping is a derivative storage index, not new audit acceptance. Root transport-equivalence review remains required.
