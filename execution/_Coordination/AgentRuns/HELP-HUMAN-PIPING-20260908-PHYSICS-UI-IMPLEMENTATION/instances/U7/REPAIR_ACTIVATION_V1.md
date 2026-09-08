# U7-R2 Repair Activation V1

Root released bounded repair of `RU-F1` through `RU-F5` after independently verifying the terminal RU packet and its 43-member inventory. The frozen V1 nine-path source hashes were reverified before this activation and remain the exact R2 input.

Terminal bindings:

- `instances/RU/REVIEW.md`: SHA-256 `51a2123ed8ada427eaa34bac2aaccd9e92372499fc738b8663bc2ef7b8b9bbc0`;
- `instances/RU/RETURN.md`: SHA-256 `481edae2ba1fe5a0227cffab231daca0d59086b769f3c55ebdd7bc784a69b69f`;
- `instances/RU/VALIDATION.md`: SHA-256 `c2a199b57196d833997482053ef8024e94ff952e26d906be311de83dd975cb44`;
- `instances/RU/REVIEWED_INVENTORY.sha256`: SHA-256 `7a3f846b51dff2e1c5afec262ad2a000ecfdfa580e3fbbd850a756641ca0b8b4`;
- `instances/RU/STATUS.json`: SHA-256 `0b5761d856c7dbdd3a267774e0edef7be5a763cb5ddec430e675a8e287ac41fa`;
- frozen U7 manifest V1: SHA-256 `9f71f16fe91112a42ed01fc69aee040665c4c1663d3e9d5a02d8df00747f4446`;
- frozen U7 exact diff V1: SHA-256 `df93c21cb945f6ed5d3e42ecbfd8f4f25ef462ba5c453d76757b36d2b2fa2056`.

Authority permits one bounded Agent 2 (`gpt-5.6-sol`, high reasoning, no delegation) to repair only the five terminal findings inside the existing nine source/test paths. U7 manager remains product-read-only. Rust, wasm/native build, native launch, service/schema/API changes, scope changes, and commit/push remain prohibited. Same RU will backcheck the frozen successor after manager validation.
