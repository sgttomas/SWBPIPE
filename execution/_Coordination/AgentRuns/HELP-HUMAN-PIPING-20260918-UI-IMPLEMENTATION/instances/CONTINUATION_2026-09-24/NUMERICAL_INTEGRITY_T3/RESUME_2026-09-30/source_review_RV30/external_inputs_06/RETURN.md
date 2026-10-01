# RV30 external_inputs_06 — bounded input binding review

**Binding usable within its stated scope; no blocking finding.** All12 existing raw inputs match source09 and the manifest, and their input metadata matches I23's descriptors. The12 logged K4SRC matches are source-qualified historical witnesses, not a fresh canonical replay.

Independent TASK Type2 RV30, direct native child `/root/rv30_k6c_kernel` of ROOT `/root` HELP_HUMAN Agent0; no delegation. Actual start2026-10-01 15:01:02 UTC; deadline15:11:02 UTC. Completion after checks is recorded in VERIFICATION.json. Same instruction/skill origins and hashes as ../kernel_01/INSTRUCTION_BINDING.json; direct parent follow-up supplies this10-minute additive review scope.

Reviewed I23 packet:
`R/I23/external_inputs_06`,
seal `6891d130f64764a7bfdde7c66fd44db9656bc4ab10cf41af7117fb49ec20674c`.
All five payloads independently verify. I read RETURN, INPUTS, SOURCE_WARRANTS.json and provenance, and independently rehashed all25 source/history origins cited by the warrants. The quoted line ranges exactly match their bound sources. Initial lookup for SOURCE_WARRANTS.md failed because the actual artifact is JSON; the correct sealed JSON was then read.

## Raw identity and hash semantics

I independently read precisely the12 named `<WT>/scratch/i17/b_models/<case>.json` files. They are the exact source09 external-input identity set, with no missing or extra case substituted. Each file:
- exists at its recorded resolved path and is not a symlink;
- has the recorded size/device/inode/mtime and valid UTF-8 ending in newline;
- has SHA256 equal to both source09.model_sha256 and manifest column1;
- is paired with the same expected K4SRC in source09 and manifest column2;
- retains its size/mtime/inode and raw hash after review.

Total raw input bytes independently sum to21,053,750. This is raw file length, not a typed-model, String-capacity, parser or heap bound.

The source warrants correctly distinguish the two hashes. gen_vk_cases.py:108-109 defines compact sorted-key JSON spelling; :452-463 hashes `dumps(adapted)+'\\n'` and writes that model hash beside the separate K4SRC hash; :706-707 writes the same text. cases.rs:330-334 hashes the actual read_to_string bytes. vk_scale.rs:350-370 compares that raw model hash before proceeding. Direct hashing of the existing files removes any need to assume a normalized JSON digest equals the raw-file digest.

K4SRC remains a different typed canonical encoding. vk_scale.rs:383-397 constructs PrimitiveSource, encodes it and separately compares its hash. This review did none of those operations. No raw model was reserialized, generated, copied or altered, and no replacement canonical serializer was written.

## Descriptor backcheck

Using only parsed JSON scalar/array/String metadata, I independently recomputed every numeric/text-child field in each descriptor:
N,m,axis s/d split,r/r_unique,l,t,u,n,conditional f,q,U and raw contribution counts;
distinct loaded DOFs, per-DOF multiplicity maxima/histograms and zero-bit-pattern load counts;
source-ID byte sum/max/histogram;
node/member/spring/omitted/load-ID String-child counts, sums, minima, maxima and length histograms;
total typed-model text bytes and empty aggregate-support child lists.

All fields match. These positions agree with the hash-bound current parse_model/source_parts code at40129, not raw JSON number-spelling lengths. The typed parser retains the identified text groups and turns the hex numeric strings into numeric fields. s=d=0 for these actual12 files is verified input data; it does not narrow the other201 VR entries. u=0 follows this adapter's empty aggregate support vectors and does not remove constraints or physical support semantics.

The metadata can populate the already separately reviewed source09 finite descriptor contract. It does not measure String/Vec capacity, execute a model builder, recompute graph/RCM/profile values, validate all constructor numeric conditions or produce solver expectations. Fields explicitly conditional on valid source remain conditional. No historical heap/time/estimate field entered the descriptors.

BACKCHECK.json contains each recomputed descriptor, raw hash/length, source09/manifest equality and historical witness binding.

## Historical K4SRC witnesses

I independently rehashed all12 named `IMPLEMENTATION/VK/_run_records/b/setup/counts_runs/counts_<case>.jsonl` files. Each has exactly one matching start, counts and counts-only summary row:
- start.model_sha256 and committed_model_sha256 equal the current raw-file hash;
- counts.k4src_sha256 equals the separately expected manifest/source09 K4SRC;
- the logged case identity and nodes/members/springs/constraints/loads/stations/DOFs/free-DOFs/rows match current direct metadata or its explicit conditional arithmetic;
- summary says counts_only=true.

The witnesses retain their historical qualification. The bound VK RETURN:444-451 describes the release archive64470c6ba and8f5d6316… binary/setup that produced those counts-only witnesses, and explicitly names scratch/i17/b_models. KF3 RETURN:331-335 independently records reuse of the12 manifest-matching inputs at its own historical source/binary basis. These are historical source/run statements preserved with their exact file hashes. I did not requalify those old binaries or silently promote their source revision to40129/final A1.

Therefore the packet's status HISTORICAL_CONSUMER_MATCH_PRESERVED_NOT_RERUN is accurate. There is no current raw-file mismatch or missing historical logged canonical match. A current PrimitiveSource validation/encoding replay is still unrun, and a later semantic change to parsing, adaptation, validation or canonical encoding requires its own reconciliation.

## Exact limits and integration consequence

This closes the earlier absence of concrete external12 file locations/raw hashes/byte lengths and input population/String metadata for these exact existing bytes. A later use must still preserve/recheck that identity; paths alone do not pin future file content.

It does not close:
- current/final-A1 constructor or canonical-encoding validation;
- exact fresh graph B/b/z/h/histogram generation;
- source09 checked estimator implementation or acceptance of its capacity formulas;
- parsed JSON/global caller ownership, formatting/I/O/runtime composition;
- full E_max, admission, W1 or numerical/product acceptance.

The raw model identity proof and the historical canonical witness are deliberately separate. No fresh replay is inferred from equality of historical logged hashes.

All writes are additive under R/source_review_RV30/external_inputs_06. No Ruby, Rust, build, test, solver/model/generator, canonical encoder, new tool/framework, Git/index operation, network/install or delegation occurred. Only ordinary existing-file reads, hashing, JSON metadata counting, source reading and review evidence writes were used. No active I21 packet was inspected; all prior seals remain unchanged.

