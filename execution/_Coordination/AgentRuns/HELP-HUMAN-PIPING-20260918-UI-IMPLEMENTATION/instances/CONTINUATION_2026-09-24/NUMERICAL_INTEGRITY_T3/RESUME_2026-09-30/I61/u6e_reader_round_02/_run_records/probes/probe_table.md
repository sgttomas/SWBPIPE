| probe | expected | py 07g | rs 07g | ts 07g | py 07h | rs 07h | ts 07h |
|---|---|---|---|---|---|---|---|
| s1:affected_refs_string_listed | G5/RP_ATTEMPT_MISMATCH | G7/SPP_EVIDENCE_INVALID | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH |
| s1:affected_refs_string_unlisted | G7 (per-reader code) | G5/RP_ATTEMPT_MISMATCH | G7/SPP_ARRAY_INVALID | G7/SPP_EVIDENCE_INVALID | G7/SPP_EVIDENCE_INVALID | G7/SPP_ARRAY_INVALID | G7/SPP_EVIDENCE_INVALID |
| s1:affected_refs_number_listed | G5/RP_ATTEMPT_MISMATCH | G5/RP_PRODUCT_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH |
| s1:affected_refs_number_unlisted | G7 (per-reader code) | G5/RP_PRODUCT_ATTEMPT_MISMATCH | G7/SPP_ARRAY_INVALID | G7/SPP_EVIDENCE_INVALID | G7/SPP_EVIDENCE_INVALID | G7/SPP_ARRAY_INVALID | G7/SPP_EVIDENCE_INVALID |
| s1:affected_refs_object_listed | G5/RP_ATTEMPT_MISMATCH | G7/SPP_EVIDENCE_INVALID | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH |
| s1:affected_refs_object_unlisted | G7 (per-reader code) | G5/RP_ATTEMPT_MISMATCH | G7/SPP_ARRAY_INVALID | G7/SPP_EVIDENCE_INVALID | G7/SPP_EVIDENCE_INVALID | G7/SPP_ARRAY_INVALID | G7/SPP_EVIDENCE_INVALID |
| s1:typed_integrity_affected_refs_string | G5/RP_ATTEMPT_MISMATCH | G7/SPP_EVIDENCE_INVALID | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH |
| n1:second_case_swapped | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH |
| n1:second_case_relaxed_form | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH |
| n2:envelope_reordered_exact | pass | pass [66, 1, 6, 1] | pass [66, 1, 6, 1] | pass [66, 1, 6, 1] | pass [66, 1, 6, 1] | pass [66, 1, 6, 1] | pass [66, 1, 6, 1] |
| n2:envelope_reordered_strict_prefix | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH |
| n2:corpus_base_truncated_last | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH | G5/RP_ATTEMPT_MISMATCH |
| milestone:sparse_interactive:unmodified | pass | pass [25, 69, 3, 1] | pass [25, 69, 3, 1] | pass [25, 69, 3, 1] | pass [25, 69, 3, 1] | pass [25, 69, 3, 1] | pass [25, 69, 3, 1] |
| milestone:dense_scrutiny:unmodified | pass | pass [25, 69, 3, 2] | pass [25, 69, 3, 2] | pass [25, 69, 3, 2] | pass [25, 69, 3, 2] | pass [25, 69, 3, 2] | pass [25, 69, 3, 2] |

expectation misses (07h readers): []
07h cross-reader gate divergences: []
07h cross-reader code divergences: ['s1:affected_refs_string_unlisted', 's1:affected_refs_number_unlisted', 's1:affected_refs_object_unlisted']
