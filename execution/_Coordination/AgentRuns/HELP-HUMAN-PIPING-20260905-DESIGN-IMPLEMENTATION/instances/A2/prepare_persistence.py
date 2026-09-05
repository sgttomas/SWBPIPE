from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent
source=p/"snapshots/UNITS_V1/source/projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs"
s=source.read_text();sections=[]
for start,end in [("const STORE_SCHEMA_TARGET_VERSION:","fn sqlite_compile_options("),("fn now_unix_seconds()", "fn list_projects(")]:
 lo=s.index(start);hi=s.index(end,lo);sections.append({"start_line":s[:lo].count("\n")+1,"end_line":s[:hi].count("\n"),"sha256":hashlib.sha256(s[lo:hi].encode()).hexdigest(),"code":s[lo:hi]})
scratch=Path("/tmp/piping-a2-persistence-20260905");(scratch/"src").mkdir(parents=True,exist_ok=True)
(scratch/"Cargo.toml").write_text('''[package]
name="piping-a2-persistence"
version="0.1.0"
edition="2021"
[dependencies]
serde_json="1"
rusqlite={version="0.32",features=["bundled"]}
serde={version="1",features=["derive"]}
''')
code="#![allow(dead_code)]\nuse serde::Serialize;\nuse serde_json::{Value,json};\nuse rusqlite::{Connection,params,OptionalExtension};\nuse std::path::Path;\nuse std::time::{SystemTime,UNIX_EPOCH};\n"+"\n".join(x["code"] for x in sections)+(p/"persistence_main.rs").read_text()
(scratch/"src/main.rs").write_text(code)
(p/"results/persistence_extraction.json").write_text(json.dumps({"source_sha256":hashlib.sha256(s.encode()).hexdigest(),"sections":[{k:v for k,v in x.items() if k!="code"} for x in sections],"generated_sha256":hashlib.sha256(code.encode()).hexdigest()},indent=2)+"\n")
