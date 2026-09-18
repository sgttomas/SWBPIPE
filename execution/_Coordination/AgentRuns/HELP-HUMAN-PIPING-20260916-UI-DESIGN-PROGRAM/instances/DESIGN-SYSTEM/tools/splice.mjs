// Splices the generated CSS and JSON into the specimen at its two markers (idempotent: re-splices from the pristine markers kept in specimen.src.html).
import fs from "node:fs";
const dest = process.argv[2];
const srcCopy = "specimen.src.html";
let s = fs.existsSync(srcCopy) ? fs.readFileSync(srcCopy, "utf8") : fs.readFileSync(dest, "utf8");
if (!s.includes("/* GENERATED:TOKENS_CSS */") || !s.includes("/* GENERATED:TOKENS_JSON */")) throw new Error("markers missing");
if (!fs.existsSync(srcCopy)) fs.writeFileSync(srcCopy, s);
const css = fs.readFileSync("tokens.css", "utf8").trim();
const json = fs.readFileSync("tokens.embed.json", "utf8").trim();
if (json.includes("</script")) throw new Error("json unsafe");
s = s.replace("/* GENERATED:TOKENS_CSS */", css).replace("/* GENERATED:TOKENS_JSON */", json);
fs.writeFileSync(dest, s);
console.log("spliced into", dest, "| bytes", s.length);
