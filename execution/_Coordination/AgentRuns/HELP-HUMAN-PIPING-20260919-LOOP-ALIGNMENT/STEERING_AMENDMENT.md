# Stable instructions, state pointers and steering

Owner direction, 2026-09-19, verbatim from the active conversation:

~~~
you should not put specific handoff details into the LOOP instructions.  Those should be managed how the original LOOP architecture handled it: recurrent, stable instructions, plus specific state pointers to orient the agent's discover, plus steering instructions for specific direction priorities.
~~~

SHA-256 without trailing newline: `9963085b01874cf5a37d0e12776ed2006394fc94d8af99efe65386f959f9fdb8`.
Transcript text custody, not transport bytes.

Agent 0's implementation: retain the stable entrypoint's committed standing-plan
selection and separate its recurrent instructions, state discovery and per-run
steering. Remove the dated successor path from LOOP_INIT. State navigation uses
the validated receipt cursor and owning coordination/run records; priorities
come from owner directions and per-run steering. The coordination notice carries
the current UI graph/handoff pointers, outside the recurrent instructions.

This is an owner-directed amendment to the initially accepted draft, not an
adoption-marker transformation. The original draft hashes, initial adoption
checks and initial review remain bound to their original candidate. The follow-up
review covers this amendment and metadata. No test, numerical criterion, product
source, lane head or existing run record changes.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Owner clarification (verbatim)

~~~
in our case, the handoff prompt is the steering (mostly - it also included elements of a loop instruction).
~~~

SHA-256 without trailing newline: `245ae7cb93a9f09f5277369c0f368286b220295c5fef71f629f735d19b8c84f4`.

Agent 0 reading: the handoff prompt is this session's steering and state
navigation; reusable rules belong in the standing instructions. No additional
steering file or separate approval layer is introduced.
