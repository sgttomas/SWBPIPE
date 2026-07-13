# D-41 R5 T3 supporting record — telemetry off-by-default

**Date:** 2026-07-12
**Decision:** DEC-074, O7 before E5
**Row:** PDU-028
**Lifecycle:** IN_PROGRESS (unchanged)

## Bounded evidence

- DEL-10-02's deny-only adapter declaration gate rejects disabled privacy, report, sandbox, and private-transmission controls and never dispatches while the runtime is unselected.
- Focused verification at the owning DEL-10-02 seam: `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -p no:cacheprovider tests/test_adapter_framework_contract.py -q` → `17 passed`.

## Residual and fences

- Adapter approval/allowlist remains unselected. Plugin, adapter, import/export, report, and private-library runtime paths are not bound to the telemetry guard; PDU-043's documented absence remains intact.
- This supporting record is negative evidence at adapter declaration admission only. It is not whole-product privacy/security assurance, telemetry runtime closure, a lifecycle transition, review, dependency/DAG/register change, decomposition change, or ISSUED evidence.
- Cargo copy-out was not applicable because no Rust source or crate was changed for this seam.
