# MR-A Phase-A Basis Manifest

Status: `HASH_BOUND_REVIEW_BASIS — ZERO DISPOSITIONS`

This derivative manifest binds the Phase-A review candidate to exact bytes. It
is not decomposition truth, review acceptance, or a promotion act.

## Corpus aggregate

The corpus aggregate is SHA-256 over the byte sequence formed by sorting every
case-page repository-relative path, then appending for each row
`path + NUL + lowercase-page-sha256 + LF`.

```text
page_count=64
mechanics=21
stress=15
nonlinear=28
corpus_manifest_sha256=e33877bef390f371a009c06e8247b56a2c410ad5fb499a8fc4fadb06165d8b45
```

Reproduce from the repository root with this exact byte recipe:

```python
import hashlib
from pathlib import Path

repo = Path.cwd()
pages = sorted(
    (repo / "projects/chirality-piping/docs/validation_manual/cases").glob("*/*.md")
)
rows = []
for page in pages:
    path = page.relative_to(repo).as_posix().encode("utf-8")
    digest = hashlib.sha256(page.read_bytes()).hexdigest().encode("ascii")
    rows.append(path + bytes([0]) + digest + bytes([10]))
print(hashlib.sha256(b"".join(rows)).hexdigest())
```

The path is repository-relative (it begins `projects/chirality-piping/`), the
separator is one NUL byte, the digest is 64 lowercase ASCII hex characters,
and the row terminator is one LF byte. No literal backslash escape bytes enter
the aggregate.

Expected and observed membership is exactly 64 pages: 21 mechanics, 15 stress,
28 nonlinear. The DEC-092 page remains on its explicit locked/offline crate-test
reproduction text; the other 63 pages receive the corrected shared runner
status paragraph.

## Bound shared identities

| Path | SHA-256 | Git blob |
|---|---|---|
| `docs/validation_manual/cases/generate_validation_case_pages.py` | `d36bb1acf0513c043e230293eaa016ea6b243d1ea6b96f876518f844244a0994` | `9fb1f0d8f85eda268b9cb5b323e784313deaa406` |
| `docs/validation_manual/index.md` | `127e2e9d13b5cd6c9cd194aeed1659e29152c90150f66bbe32971597b10a3ff8` | `48da16892b7083e01b0c1fe83af17c254e74da47` |
| `docs/VALIDATION_STRATEGY.md` | `a9bce923d5ca9656191a4abddad8f9464c2fde6b37c94b3d787e98c00949acda` | `18d58bda65a4c515895bfd9d09eb1c096ca51f81` |
| `validation/benchmarks/mechanics/src/lib.rs` | `8f015dfeeb1fd670065f467335419ab0da8b0c9a4e6d6b3fb565ceea2d6f6a26` | `eb65e53075110995a4ddcd93b4181b15392f91d5` |
| `validation/benchmarks/stress/src/lib.rs` | `ad7239a073f5ca6c161ee3f63642d487414efed6e3176ae86a2af7979b73210c` | `201f5d84d1a666975000d07fb8e21900b88f9807` |
| `validation/benchmarks/nonlinear/src/lib.rs` | `ff3f318224fa2c55392bfa17569182768453b3157cf35fc20d763f54bf02fd12` | `37d19abf27a17c1d1333a81732d0b413e4ca2880` |
| `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-E2-VALMANUAL-001.md` | `b37c85947cb6439a5e005cf8ab8fb6009ab1183ca054f1274f8111b1946e8d28` | `97c07ef9b52fc36e426a503d3c52bfde9a7929a2` |
| `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-08-02_DEC092_TEMPERATURE_G_IMPLEMENTATION.md` | `0a644ac4f8b1ac3c843dee328fdf973a12279a956aab01ae01cfada611a50d1f` | `b9d9fc9e59a8c23ac8bee131892c565304853715` |
| `validation/witness/fixtures/tp_phys_015_section_property_stress_witness.json` | `3b44dff4b2e106926cdb05fe7f6deccbce72e410605b793ce33f71040c0f478b` | `12010995f747002867b8f7dcb320c7fc92d54fce` |
| `validation/hand_calcs/stress/generated/tp_phys_015_section_property_stress_witness.md` | `513a512f9660d6151295b82a20683ee195d95abbf43f997a6e9cb5261ed7faa8` | `5bb21d42dc7b7fa1ff02018b21102213e1070c91` |
| `validation/witness/generated/tp_phys_015_section_property_stress_witness.mathml` | `918489db2f81f8e1874bb9dc66d61ea53363222c908f7cd37003b596842f4b1e` | `c392a2302e3b39696cea8384a21be4b86e37c42e` |
| `fixtures/results/invented/tp_phys_015_section_property_stress_evidence_envelope.json` | `11f5b11964dfa4980efb00e80e041fd31dddfac02a0037902da93834f3952edc` | `9048f97fc339a633d1ad9a460ed4ff9048a73461` |
| `execution/_Decomposition/_LATEST.md` | `fa6f3bcdea5e27f916a607b9bcfe2c3bee445f21dd533b13f9b57f7f38cfc807` | `45de7c70c7c7441d1fcf73f670da8433a1c8e57b` |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | `06419f7e7e399d4732d9e207eb87b17b3a4d4a4ce74a19f9d5fe60f9a342882d` | `a64b02b77248c26d3d17987624131a35a5acbb71` |
| `execution/_DAG/_LATEST.md` | `04f24cd88d16b38d6da00ae2dee32f0387381ee41659de2e352cba07127736e3` | `5441c7127aceecdefe242bef25e5ca9cd5a330b4` |
| `execution/_DAG/DAG-009/APPROVAL_RECORD.md` | `f25526c4e0eec239f5d3464ca4d8e0ab8c9638ebd035bc9aa282def33989337b` | `0fe2817a3937ba690af466ea0509cb77368940e3` |
| `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES/OWNER_RULING_2026-08-11.md` | `5f0f857e5b95284e88506c768c91547b3f02815dfd63747c966d2df7b8775a22` | `ab17acbd19be9fcce163d7a13bb17dd7d0fbe4d1` |
| `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES/instances/A2_MAINTAINER_REVIEW_REMEDIATION/PACKET_V2.md` | `ee08a4af9cfb99ba624a2ab510f2c95d48484172a562f001bb60aa1eac9d8565` | `b712b9f3f00ee45641cec7356e60c711e702028e` |
| `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES/instances/A2_MR_A_BASIS/LAUNCH_BRIEF.md` | `19d57333b4512199ed11e865cb687ae3d5dcaf9f1d9c6e9f9e0b0908005e7236` | `623e944f1ff4962ba0db6ef8026e9b5e8985c814` |
| `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES/instances/A2_MR_A_BASIS/REVIEW_INSTRUMENT.md` | `0bc44d02ff8649cfc74275ddbde12e3a449019c4a8928d9aceb772af0eee49d8` | `c87607a47898371d1bb7c9cafebc18f6bf1054cd` |

The Git-blob values for modified or new files are calculated content identities;
they do not assert that an object was written, staged, or committed.

## Witness structural backcheck

The sole formal witness JSON parsed successfully. Every top-level key required
by `openmath_calculation_witness.schema.json` was present; its 11 formulas and
6 outputs were found; and all three referenced artifacts (result-export
envelope, Markdown rendering, MathML rendering) exist. Canonical JSON SHA-256
using sorted keys and compact separators is
`1af3d494093179bb93435be0e252ba3b84d8d6959a49a3c4aee455b8153518dd`,
matching the deterministic Markdown rendering. The repository validator could
not be invoked with the system Python because its `jsonschema` dependency is
not installed; this manifest therefore records a structural/hash/path
backcheck, not a fresh full schema-validation claim.

## Currency determination

The manual index now cites current accepted decomposition revision `0.11` and
active graph `DAG-009`. The validation strategy retains revision `0.7` as its
last full content-alignment baseline and separately points to revision `0.11`
and `DAG-009`: revision `0.10` introduced actor-neutral reproduction and a
principal external-prover posture, so silently claiming complete revision-0.11
strategy alignment would exceed the evidence.

## Non-effects

No owner/maintainer review result, tier promotion, promotion map, source
acceptance, register action, receipt, lifecycle/reliance/release effect, or Git
action is represented by this manifest.
