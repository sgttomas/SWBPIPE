# Integrated review attempt 8

Frozen range: `357a58b56726feba49507534159c3fbc4656b818..3662143bc9558d2da32e74e068768ba81edc0a74`
Verdict: `FAIL`

The reviewer inspected all 36 paths and the complete range; ancestry, scope,
diff integrity, and registered tool-policy compliance passed. N1 and N3 had no
finding. N2 verified an input manifest internally but compared the separately
supplied report model only by project ID, allowing a stale or different model
snapshot with the same ID to be rendered while package evidence attested the
manifest's model payload.

Disposition: closed by N2 Amendment 3 and commit
`561d6d601102657607748af1de3f07c11d98eb59`. Production now requires canonical
hash equality between the supplied model and verified manifest model payload
before rendering or assembly, reuses the accepted hash, rejects same-ID payload
substitution, and accepts canonically equivalent reordered keys. Focused Vitest
8/8, desktop build/typecheck, Cargo 19/19 plus docs/fmt, containment, and diff
checks passed; fresh full-N2 review attempt 4 passed with zero findings over the
four-file original-basis diff. A fresh complete integrated review is required.
