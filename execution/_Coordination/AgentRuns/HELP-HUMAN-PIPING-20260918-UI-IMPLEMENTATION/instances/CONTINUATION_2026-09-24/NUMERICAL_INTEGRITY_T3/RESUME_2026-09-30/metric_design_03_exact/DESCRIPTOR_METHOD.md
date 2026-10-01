# Descriptor extraction and replay

Use GIT_OPTIONAL_LOCKS=0 git show40129:PATH for the ten family files listed
in DESCRIPTORS.json.inputs. Require the source09 hash for each byte stream.
Read existing JSON lines; never generate/solve a model. Parse each rows entry
(key,expected,class,optional scale); chosen scale is String row[3] when present,
otherwise case.scales[class], matching cases.rs137-140,229-234. Resolve every
value-control key to that case's existing row and add its value string.

The one-off VENV Python metadata pass used this full ASCII grammar:
  ([+-]?)([0-9]*)(?:\.([0-9]*))?(?:[eE]([+-]?[0-9]+))?\Z
Require a nonempty concatenation of integer/fraction digits. Record total digit
count d including leading zeros, fraction length f, explicit exponent e or0,
p10=e-f and whether every digit is0. Check e and p10 fit i64 and all strings
match. The original temporary dictionary keyed exact text identified9522 unique
strings. No binary64 parsing, numerical predicate, Nat implementation or solver
was executed. This is inspection of existing inputs, not an oracle result.

Collapse exact strings to memory shapes(d,p10,zero). Sign does not reduce any
branch bound. Group row pairs(expected shape,chosen-scale shape) and control
triples(observed shape,expected shape,chosen-scale shape) within each case.
Retain multiplicity, first row/control index and key, case ID, file and line.
The set of213 case IDs was checked equal to source09.vr_rows; grouped totals
are27752 rows and5085 value-control entries. All10 source file hashes match.
The compact259-shape representation avoids duplicating every reference value;
replay the same deterministic scan to validate all grouped occurrences.

DESCRIPTOR_SUMMARY retains the scan's maxima, unique-text count and final
compact file size. No prior sealed packet was modified. ARITHMETIC contains
source-derived integer bounds, not measured capacities or expected numerical
results.
