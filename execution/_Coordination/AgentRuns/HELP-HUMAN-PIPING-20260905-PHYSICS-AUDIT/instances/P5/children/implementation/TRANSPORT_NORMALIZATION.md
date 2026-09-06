# Transport normalization V2

Root authorized representational packaging only. Original packet (including manifest, reviewer/author return and every member) is preserved losslessly in `../../transport_archive/IMPLEMENTATION_ORIGINAL_V1/ARCHIVE.json`; decoded original manifest SHAe2038c43862a826ff9bcd3662200f3f290d2c7e243da3507fd56886f7f0a022d. All decoded member hashes verified before changing transport. Original source/return bytes and source hashes are unchanged.

`P5_DIFF.patch.json` stores exact original diff bytes as base64 with original decoded SHA and length. Where original returns/bindings name `P5_DIFF.patch`, decode this file. No whitespace was removed from diff content, no attributes exception, no production mutation. This new candidate manifest binds normalized transport rather than pretending the original file paths remain raw. No source test/review rerun required under root representation-only amendment.
