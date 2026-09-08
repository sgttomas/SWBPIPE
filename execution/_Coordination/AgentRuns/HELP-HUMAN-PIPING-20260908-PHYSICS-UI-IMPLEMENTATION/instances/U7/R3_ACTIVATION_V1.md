# U7-R3 Residual Repair Activation V1

Root released a narrow repair of `RU-BC-F1` and `RU-BC-F2` after reading the full successor backcheck and verifying its 35-member inventory plus five packet hashes. Original `RU-F1`, `RU-F4`, and `RU-F5` are closed and must remain closed. R2's nine source hashes were reverified against successor manifest SHA-256 `72d1cb5f31d7d81e92f05edd6f0b9e8b74c102cc27c898b7139e99644fb529f1` before dispatch.

Backcheck bindings:

- `REVIEW.md`: SHA-256 `54dd95c5f4410974060430221c16bdc3bd83bbb39964ea0622fc64fdd03f8f18`;
- `RETURN.md`: SHA-256 `044f422f3ac5fcb9cfe92f7bcc2e78c79b2ebe6ff93a9c96d836a3a0d1eaf2b4`;
- `VALIDATION.md`: SHA-256 `8345f67d9565403fbf47959f7c8eb24598c975d7ae6b20958c4048a4759f9fa3`;
- `REVIEWED_INVENTORY.sha256`: SHA-256 `f7ff930afeeebdcde1b0ef47ab1d2f98547b2477103b1c1b349fa0777003a9f8`;
- `STATUS.json`: SHA-256 `e52200fc169b55c5599d89e91e90f8d46272c4243688f8176aaaa0f2f33b40f9`.

One bounded Agent 2 (`gpt-5.6-sol`, high reasoning, no delegation) may repair only these residuals within the existing nine paths. Manager remains product-read-only. Native, integration, Rust/WASM builds, service/schema/API changes, and commit/push remain held.
