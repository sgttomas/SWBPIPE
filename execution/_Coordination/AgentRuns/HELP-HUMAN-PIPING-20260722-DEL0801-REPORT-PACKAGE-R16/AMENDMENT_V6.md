# Candidate amendment v6

Detection layer: terminal N4 pre-sweep verification against the exact frozen
SHA.  
Disposition: owner-authorized one-file verification-unblock amendment.

The active root DAG pointer at frozen SHA
`8698b0338ac82556fee583dd3f85bb62d0b74f85` resolves DAG-008, while four
assertions in `tests/test_release_readiness_script.py` still require DAG-007.
A clean frozen-SHA archive reproduced the identical failures.

The owner authorized N4 to modify only:

```text
projects/chirality-piping/tests/test_release_readiness_script.py
```

Permitted effect: replace stale DAG-007 assumptions with active DAG-008
expectations. No other test, product, runner, DAG, pointer, lifecycle, release,
or evidence-acceptance change is authorized.

Gates:

1. focused release-readiness tests pass;
2. full piping pytest passes;
3. exactly one acceptance-eligible DEC-025 sweep runs;
4. no filesystem edit follows the sweep;
5. N5 and W3 remain held until terminal sweep evidence is returned.
