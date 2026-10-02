# F01 observation — return-control clarification

For the actual18:33:35–18:43:35 UTC diagnostic block, ROOT clarifies the
already planned D03 NONE control. If D02 completes normally with all30
attributable records and unchanged source/binary/guard bindings, execute that
one return control even when its reason set is mixed or nonqualifying. Record
the qualification stop, complete only the bound return control, then return
all evidence. This permits a restoration check, not fault credit or P10/later
continuation. The existing hard end is unchanged.

Identity mismatch, incomplete/unsupported process output, an initial-control
failure, overrun or another operational failure still stops all remaining
execution. No retry, new command, filter, source, expected value or criterion
is authorized. Preserve the original brief and this prospective clarification.
