# I45 adapter accounting boundary

The per-call ProductCapture owns AdapterWork. No global or TLS recorder exists.
Its ten checked event counters are, in index order:
source-record loop visits; final-row/evidence/support visits; proof/map writes;
entered fixed validation groups; bytes actually read by the explicit identity
comparison loop; identity/key comparison entries; allocation requests; named
library boundaries; requested identity-copy bytes; observed cumulative Rust
capacity bytes. Prospective overflow latches the typed event fault. Validation groups count their actual entry once; they do not prebook a field
count or claim that later short-circuited fields were read. Counts are
named entered events, not time, LME, a complete instruction count or a resource
permit. A reported capacity observation is not a peak-memory value.

Explicit byte comparisons stop at length mismatch or the first differing byte.
Read credits precede those byte reads. Captured Vec reservations validate Layout
and conversion and record actual capacity times stride. Identity copies retain
requested copy length separately from observed String capacity. Source/count
representation, allocation failure, association failure and lost accounting have
separate typed channels. The full private case decision joins adapter status,
operational scalar status, G5a status, FK spent status, numeric row verdicts and
observable/coverage results; no field can substitute for another.

The known library boundaries are the captured ordinary producer stage, actual
resolver selection capture, identity clones not yet reduced to the fallible-copy
helper, canonical PrimitiveSource construction, the one recorded native call,
fixed expected-id/metadata formatting, and final serde evidence inspection.
The source constructor's sorting/graph/encoding work, allocation implementations,
String/Vec growth and formatter/serde number/key internals remain unqualified
auxiliary work. Source-level boundary counts do not claim their internal cost is
zero or covered by the 20B/60B native limits. Earlier ordinary parse/normalization,
resolver, assembly, solving and rendering remain under their original owners;
the observer does not replay them to measure them. No tariff/admission/profile
or whole-process memory conclusion is made.

Operational ScalarWork separately records each entered add/subtract/multiply/
divide/sqrt and each explicit guard. All 23 successful source operations are
entered by the actual evaluation; failed prefixes retain original numerical
causes alongside lost collection status. G5a's prescribed arithmetic has its own
persistent ScalarWork. FK retains the actual producing native residual/cast/
factor work, final directed arithmetic, exact sums and named visits. Missing
information is not converted into an exact zero amount.
