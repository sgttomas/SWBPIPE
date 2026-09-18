// Splices the generated blocks into the specimen and the document in place, between markers, so that
// re-running after a token change is idempotent.
//   node splice.mjs <gen dir> <specimen.html> <DESIGN_SYSTEM_V1.md>
// Specimen markers:  /* GENERATED:TOKENS_CSS:BEGIN */ … /* GENERATED:TOKENS_CSS:END */
//                    <script id="tokens" type="application/json">…</script>
//                    <script id="pairs" type="application/json">…</script>
// Document markers:  <!-- GENERATED:COLOUR_TABLES:BEGIN --> … <!-- GENERATED:COLOUR_TABLES:END -->
//                    <!-- GENERATED:CONTRAST_TABLE:BEGIN --> … <!-- GENERATED:CONTRAST_TABLE:END -->
//                    <!-- GENERATED:LABEL_TABLE:BEGIN --> … <!-- GENERATED:LABEL_TABLE:END -->   (V1.2)
import fs from "node:fs";
import path from "node:path";
const [gen, spec, doc] = process.argv.slice(2);
const read = (f) => fs.readFileSync(path.join(gen, f), "utf8").trim();
function between(s, begin, end, body) {
  const i = s.indexOf(begin), j = s.indexOf(end, i + begin.length);
  if (i < 0 || j < 0) throw new Error("marker missing: " + begin);
  return s.slice(0, i + begin.length) + "\n" + body + "\n" + s.slice(j);
}
if (spec) {
  let s = fs.readFileSync(spec, "utf8");
  const json = read("tokens.embed.json"), pairs = read("pairs.embed.json");
  if (json.includes("</script") || pairs.includes("</script")) throw new Error("json unsafe");
  s = between(s, "/* GENERATED:TOKENS_CSS:BEGIN */", "/* GENERATED:TOKENS_CSS:END */", read("tokens.css"));
  s = s.replace(/(<script id="tokens" type="application\/json">)[\s\S]*?(<\/script>)/, `$1${json}$2`);
  s = s.replace(/(<script id="pairs" type="application\/json">)[\s\S]*?(<\/script>)/, `$1${pairs}$2`);
  fs.writeFileSync(spec, s);
  console.log("spliced specimen", spec, "| bytes", s.length);
}
if (doc) {
  let d = fs.readFileSync(doc, "utf8");
  d = between(d, "<!-- GENERATED:COLOUR_TABLES:BEGIN -->", "<!-- GENERATED:COLOUR_TABLES:END -->", read("colour_tables.md"));
  d = between(d, "<!-- GENERATED:CONTRAST_TABLE:BEGIN -->", "<!-- GENERATED:CONTRAST_TABLE:END -->", read("contrast_table.md"));
  d = between(d, "<!-- GENERATED:LABEL_TABLE:BEGIN -->", "<!-- GENERATED:LABEL_TABLE:END -->", read("label_table.md"));
  fs.writeFileSync(doc, d);
  console.log("spliced document", doc, "| bytes", d.length);
}
